// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

use burn::tensor::{backend::Backend, Int, Tensor};

/// The Hodge-Dirac topological flow engine.
/// Computes physical diffusion (heat, fluid, stress) across the Cellular Sheaf.
pub struct TopologicalLaplacian;

impl TopologicalLaplacian {
    /// Computes the discrete scalar Laplacian across the 1-skeleton (Graph).
    /// Mathematically: \Delta x = B_1 ((1 - d) \odot B_1^T x)
    /// This guarantees absolute conservation of mass and energy while supporting continuous topological fracture.
    ///
    /// # Arguments
    /// * `x` - The scalar features on the nodes. Shape: `[Batch, N_nodes, Features]`
    /// * `edges_b1` - The boundary matrix mapping nodes to edges. Shape: `[2, E_edges]`
    /// * `damage` - Continuous fracture damage scalar on the nodes `[0.0 (pristine), 1.0 (broken)]`. Shape: `[Batch, N_nodes, 1]`
    pub fn scalar_laplacian<B: Backend>(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let batch_size = x.dims()[0];
        let num_edges = edges_b1.dims()[1];
        let features = x.dims()[2];

        // 1. Extract the Source and Target node indices
        let src_indices = edges_b1
            .clone()
            .slice([0..1])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);

        let tgt_indices = edges_b1
            .clone()
            .slice([1..2])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);

        // 2. Gather values from nodes to the edges
        let x_src = x.clone().gather(1, src_indices.clone());
        let x_tgt = x.clone().gather(1, tgt_indices.clone());

        // 3. Compute continuous Edge Damage (average of connected nodes)
        // Expand the scalar damage to match the feature dimension for element-wise multiplication
        let damage_src = damage.clone().gather(1, src_indices.clone());
        let damage_tgt = damage.clone().gather(1, tgt_indices.clone());
        let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);

        // The fracture coefficient: 1.0 means perfectly connected, 0.0 means completely severed.
        let flow_coefficient = Tensor::<B, 3>::ones_like(&edge_damage).sub(edge_damage);

        // 4. Compute topological gradient across the edge, masked by the continuous fracture state
        let raw_flow = x_tgt.sub(x_src);
        let edge_flow = raw_flow.mul(flow_coefficient);

        // 5. Scatter the flow back to the nodes (sum reduction) for divergence (Laplacian \Delta = d^* d).
        // **Do not** concatenate `[src‖tgt]` into one `scatter` on large graphs: Burn 0.13 autograd can
        // then mis-shape backward buffers (`[…,2E]` vs `[…,N]`, e.g. Striatus `40×40×4`). Two scatters
        // from separate zero templates (then `add`) match the same sum while each scatter’s index axis
        // stays length `E` — distinct from chained `scatter().scatter()` on one tensor, which failed
        // historically on this codebase.
        let to_src = Tensor::<B, 3>::zeros_like(&x).scatter(1, src_indices, edge_flow.clone());
        let to_tgt = Tensor::<B, 3>::zeros_like(&x).scatter(1, tgt_indices, edge_flow.neg());
        to_src.add(to_tgt)
    }

    /// Fused scalar Laplacian: same numerics as [`Self::scalar_laplacian`] with fewer intermediate
    /// tensor clones on the gather/scatter path (THMC spike — Burn API unchanged).
    pub fn scalar_laplacian_fused<B: Backend>(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let batch_size = x.dims()[0];
        let num_edges = edges_b1.dims()[1];
        let features = x.dims()[2];
        let n_nodes = x.dims()[1];

        let src_row = edges_b1.clone().slice([0..1]);
        let tgt_row = edges_b1.slice([1..2]);
        let src_indices = src_row
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);
        let tgt_indices = tgt_row
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);

        let x_src = x.clone().gather(1, src_indices.clone());
        let x_tgt = x.gather(1, tgt_indices.clone());

        let damage_src = damage.clone().gather(1, src_indices.clone());
        let damage_tgt = damage.gather(1, tgt_indices.clone());
        let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);
        let flow_coefficient = Tensor::<B, 3>::ones_like(&edge_damage).sub(edge_damage);

        let raw_flow = x_tgt.sub(x_src.clone());
        let edge_flow = raw_flow.mul(flow_coefficient);

        let zeros = Tensor::<B, 3>::zeros([batch_size, n_nodes, features], &x_src.device());
        let to_src = zeros.clone().scatter(1, src_indices, edge_flow.clone());
        let to_tgt = zeros.scatter(1, tgt_indices, edge_flow.neg());
        to_src.add(to_tgt)
    }

    /// Positive diagonal \(d_i = \sum_{j\sim i} w_{ij}\) of \(-\mathcal{L}\), where \(\mathcal{L}\) is the
    /// operator returned by [`Self::scalar_laplacian`] with the same `damage` mask (\(w_{ij} = 1-\bar d_{ij}\)).
    ///
    /// For each edge, \(w\) is accumulated at both endpoints — matching \((-\mathcal{L})_{ii}\) in the
    /// primal scatter convention used by [`Self::scalar_laplacian`].
    pub fn scalar_laplacian_neg_opposite_diag<B: Backend>(
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let batch_size = damage.dims()[0];
        let features = damage.dims()[2];
        let num_edges = edges_b1.dims()[1];

        let src_indices = edges_b1
            .clone()
            .slice([0..1])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);
        let tgt_indices = edges_b1
            .clone()
            .slice([1..2])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);

        let damage_src = damage.clone().gather(1, src_indices.clone());
        let damage_tgt = damage.clone().gather(1, tgt_indices.clone());
        let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);
        let w = Tensor::<B, 3>::ones_like(&edge_damage).sub(edge_damage);

        let to_src = Tensor::<B, 3>::zeros_like(&damage).scatter(1, src_indices, w.clone());
        let to_tgt = Tensor::<B, 3>::zeros_like(&damage).scatter(1, tgt_indices, w);
        to_src.add(to_tgt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn chain_graph_edges(
        n_nodes: usize,
        device: &<B as burn::tensor::backend::Backend>::Device,
    ) -> Tensor<B, 2, Int> {
        let ne = n_nodes.saturating_sub(1);
        let mut e = Vec::with_capacity(ne * 2);
        for i in 0..ne {
            e.push(i as i64);
        }
        for i in 0..ne {
            e.push((i + 1) as i64);
        }
        let flat: Vec<f32> = e.iter().map(|&x| x as f32).collect();
        Tensor::<B, 1>::from_data(Data::new(flat, [e.len()].into()), device)
            .reshape([2, ne])
            .int()
    }

    #[test]
    fn scalar_laplacian_fused_matches_original() {
        let device = Default::default();
        let n = 5usize;
        let x_data: Vec<f32> = (0..n).map(|i| (i as f32 + 1.0) * 0.1).collect();
        let x =
            Tensor::<B, 1>::from_data(Data::new(x_data, [n].into()), &device).reshape([1, n, 1]);
        let dmg = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_graph_edges(n, &device);

        let lap = TopologicalLaplacian::scalar_laplacian(x.clone(), edges.clone(), dmg.clone());
        let fused = TopologicalLaplacian::scalar_laplacian_fused(x, edges, dmg);

        let a = lap.into_data().value;
        let b = fused.into_data().value;
        let max_delta = a
            .iter()
            .zip(b.iter())
            .map(|(u, v)| (u - v).abs())
            .fold(0.0_f32, f32::max);
        assert!(
            max_delta < 1e-5,
            "fused vs original Laplacian max |Δ| = {max_delta}"
        );
    }
}

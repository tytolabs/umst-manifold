// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Discrete scalar Laplacian on the primal 1-skeleton (Hodge–Dirac / graph flow).
//!
//! # Honest boundary (W29-056)
//!
//! [`TopologicalLaplacian`] lands gather/scatter diffusion with continuous damage masking,
//! fused path parity, and Jacobi diagonal helper. Unit fences: zero row-sum at `damage ≡ 0`,
//! diagonal consistency vs \((-\mathcal{L})_{ii}\), and full severance at `damage ≡ 1`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — Laplacian honest fence bundle.
pub const W29_LAPLACIAN_DEEPEN_CELL: &str = "W29-056-LAPLACIAN";

/// Honest posture tag — primal scalar Laplacian research lane.
pub const LAPLACIAN_POSTURE_TAG: &str = "honest-primal-scalar-laplacian-research-lane";

/// Honest physics posture — conservation unit tests pass; does not certify fleet physics GREEN.
pub const LAPLACIAN_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by Laplacian module alone.
pub const LAPLACIAN_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const LAPLACIAN_MASTER: bool = false;

/// Scalar / fused / neg-diag contracts landed in this module.
pub const LAPLACIAN_SCALAR_CONTRACTS_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const LAPLACIAN_HONEST_FENCE: &str =
    "scalar_laplacian_landed=true fused_parity_wired=true neg_opposite_diag_wired=true zero_row_sum_at_damage0=true production_wired=false physics_green=false master=false";

/// Compile-time fence — production/master/physics GREEN flip not authorized.
const _: () = assert!(!LAPLACIAN_PHYSICS_GREEN);
const _: () = assert!(!LAPLACIAN_PRODUCTION_WIRED);
const _: () = assert!(!LAPLACIAN_MASTER);

/// Typed probe for Laplacian posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaplacianPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub scalar_contracts_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the topological Laplacian.
#[must_use]
pub fn laplacian_honest_posture_bundle() -> LaplacianPostureProbe {
    LaplacianPostureProbe {
        physics_green: LAPLACIAN_PHYSICS_GREEN,
        production_wired: LAPLACIAN_PRODUCTION_WIRED,
        master: LAPLACIAN_MASTER,
        scalar_contracts_landed: LAPLACIAN_SCALAR_CONTRACTS_LANDED,
        honest_fence: LAPLACIAN_HONEST_FENCE,
        posture_tag: LAPLACIAN_POSTURE_TAG,
        deepen_cell: W29_LAPLACIAN_DEEPEN_CELL,
    }
}

/// Laplacian landed with production/master composition honestly open.
#[must_use]
pub fn laplacian_posture_honest(probe: &LaplacianPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.scalar_contracts_landed
        && probe.honest_fence.contains("scalar_laplacian_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.deepen_cell == W29_LAPLACIAN_DEEPEN_CELL
}

/// Validate Laplacian honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_laplacian_honesty() -> Result<(), &'static str> {
    let probe = laplacian_honest_posture_bundle();
    if !laplacian_posture_honest(&probe) {
        return Err("laplacian_probe failed honesty predicate");
    }
    Ok(())
}

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{Field, HumidityField, StepEntryDamageMask, TemperatureField};

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
        let damage_features = damage.dims()[2];

        // 1. Extract the Source and Target node indices (feature-width matches gather operand)
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

        let src_indices_dmg = edges_b1
            .clone()
            .slice([0..1])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, damage_features]);

        let tgt_indices_dmg = edges_b1
            .clone()
            .slice([1..2])
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, damage_features]);

        // 2. Gather values from nodes to the edges
        let x_src = x.clone().gather(1, src_indices.clone());
        let x_tgt = x.clone().gather(1, tgt_indices.clone());

        // 3. Compute continuous Edge Damage (average of connected nodes)
        // Damage stays `[B,E,1]` (or damage_features); broadcast onto feature flow.
        let damage_src = damage.clone().gather(1, src_indices_dmg);
        let damage_tgt = damage.clone().gather(1, tgt_indices_dmg);
        let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);

        // The fracture coefficient: 1.0 means perfectly connected, 0.0 means completely severed.
        let flow_coefficient = Tensor::<B, 3>::ones_like(&edge_damage)
            .sub(edge_damage)
            .expand([batch_size, num_edges, features]);

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

    /// Typed Laplacian on temperature — operand swaps are compile errors (FP P3.2).
    #[inline]
    #[must_use]
    pub fn scalar_laplacian_temperature<B: Backend>(
        t: &TemperatureField<B>,
        damage_mask: &StepEntryDamageMask<B>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> TemperatureField<B> {
        Field::new(Self::scalar_laplacian(
            t.as_tensor().clone(),
            edges_b1,
            damage_mask.as_tensor().clone(),
        ))
    }

    /// Typed Laplacian on humidity — operand swaps are compile errors (FP P3.2).
    #[inline]
    #[must_use]
    pub fn scalar_laplacian_humidity<B: Backend>(
        h: &HumidityField<B>,
        damage_mask: &StepEntryDamageMask<B>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> HumidityField<B> {
        Field::new(Self::scalar_laplacian(
            h.as_tensor().clone(),
            edges_b1,
            damage_mask.as_tensor().clone(),
        ))
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
        let damage_features = damage.dims()[2];
        let n_nodes = x.dims()[1];

        let src_row = edges_b1.clone().slice([0..1]);
        let tgt_row = edges_b1.slice([1..2]);
        let src_indices = src_row
            .clone()
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);
        let tgt_indices = tgt_row
            .clone()
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, features]);
        let src_indices_dmg = src_row
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, damage_features]);
        let tgt_indices_dmg = tgt_row
            .reshape([1, num_edges, 1])
            .expand([batch_size, num_edges, damage_features]);

        let x_src = x.clone().gather(1, src_indices.clone());
        let x_tgt = x.gather(1, tgt_indices.clone());

        let damage_src = damage.clone().gather(1, src_indices_dmg);
        let damage_tgt = damage.gather(1, tgt_indices_dmg);
        let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);
        let flow_coefficient = Tensor::<B, 3>::ones_like(&edge_damage)
            .sub(edge_damage)
            .expand([batch_size, num_edges, features]);

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
    fn laplacian_honest_posture_refuses_green_and_production() {
        let probe = laplacian_honest_posture_bundle();
        assert!(laplacian_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert_eq!(probe.deepen_cell, W29_LAPLACIAN_DEEPEN_CELL);
        validate_laplacian_honesty().expect("honesty validation must pass");
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

    /// With `damage ≡ 0`, \(\mathcal{L}\) has zero row-sum (discrete mass conservation).
    #[test]
    fn scalar_laplacian_zero_row_sum_at_damage_zero() {
        let device = Default::default();
        let n = 4usize;
        let x_data: Vec<f32> = vec![1.0, -2.0, 0.5, 3.25];
        let x =
            Tensor::<B, 1>::from_data(Data::new(x_data, [n].into()), &device).reshape([1, n, 1]);
        let dmg = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_graph_edges(n, &device);
        let lap = TopologicalLaplacian::scalar_laplacian(x, edges, dmg);
        let sum: f32 = lap.into_data().value.iter().sum();
        assert!(
            sum.abs() < 1e-5,
            "damage≡0 Laplacian must conserve mass; row-sum = {sum}"
        );
    }

    /// Constant field is in the kernel when damage is pristine.
    #[test]
    fn scalar_laplacian_constant_field_is_kernel_at_damage_zero() {
        let device = Default::default();
        let n = 5usize;
        let x = Tensor::<B, 3>::from_data(Data::new(vec![2.5_f32; n], [1, n, 1].into()), &device);
        let dmg = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_graph_edges(n, &device);
        let lap = TopologicalLaplacian::scalar_laplacian(x, edges, dmg);
        for v in lap.into_data().value {
            assert!(
                v.abs() < 1e-5,
                "constant field → Laplacian zero; got {v}"
            );
        }
    }

    /// Diagonal helper matches \((-\mathcal{L})_{ii}\) via basis probes on a pristine chain.
    #[test]
    fn scalar_laplacian_neg_opposite_diag_matches_basis_probe() {
        let device = Default::default();
        let n = 4usize;
        let dmg = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_graph_edges(n, &device);
        let diag = TopologicalLaplacian::scalar_laplacian_neg_opposite_diag(
            edges.clone(),
            dmg.clone(),
        )
        .into_data()
        .value;

        for i in 0..n {
            let mut e = vec![0.0_f32; n];
            e[i] = 1.0;
            let x =
                Tensor::<B, 1>::from_data(Data::new(e, [n].into()), &device).reshape([1, n, 1]);
            let lx = TopologicalLaplacian::scalar_laplacian(x, edges.clone(), dmg.clone())
                .into_data()
                .value;
            let neg_l_ii = -lx[i];
            assert!(
                (diag[i] - neg_l_ii).abs() < 1e-5,
                "diag[{i}]={} vs (-L)_ii={neg_l_ii}",
                diag[i]
            );
        }
    }

    /// Full nodal damage severs all edges → Laplacian is identically zero.
    #[test]
    fn scalar_laplacian_full_damage_severs_flow() {
        let device = Default::default();
        let n = 4usize;
        let x_data: Vec<f32> = vec![0.0, 1.0, -1.0, 4.0];
        let x =
            Tensor::<B, 1>::from_data(Data::new(x_data, [n].into()), &device).reshape([1, n, 1]);
        let dmg = Tensor::<B, 3>::ones([1, n, 1], &device);
        let edges = chain_graph_edges(n, &device);
        let lap = TopologicalLaplacian::scalar_laplacian(x, edges, dmg);
        for v in lap.into_data().value {
            assert!(
                v.abs() < 1e-5,
                "damage≡1 must zero all edge flow; got {v}"
            );
        }
    }
}

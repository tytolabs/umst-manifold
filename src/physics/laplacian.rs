// SPDX-FileCopyrightText: 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy, and Studio Tyto
// SPDX-License-Identifier: Apache-2.0

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

        // 5. Scatter the flow back to the nodes to compute divergence (Laplacian \Delta = d^* d)
        let mut out = Tensor::<B, 3>::zeros_like(&x);

        // Add flow to the source node
        out = out.scatter_add(1, src_indices, edge_flow.clone());
        // Subtract flow from target node (strict thermodynamic conservation!)
        out = out.scatter_add(1, tgt_indices, edge_flow.neg());

        out
    }
}

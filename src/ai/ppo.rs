// SPDX-FileCopyrightText: 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy, and Studio Tyto
// SPDX-License-Identifier: Apache-2.0

use crate::ai::cbf::ThermodynamicCBF;
use crate::core::traits::{IScienceCartridge, PhysicalResult};
use burn::tensor::{backend::Backend, Tensor};

/// The Gateway interface for Thermodynamic Topology Optimization.
/// It wraps the physical Cartridges and enforces the Thermodynamic CBF.
pub struct ManifoldGateway<B: Backend, C: IScienceCartridge<B>> {
    pub cartridge: C,
    pub cbf: ThermodynamicCBF,
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend, C: IScienceCartridge<B>> ManifoldGateway<B, C> {
    pub fn new(cartridge: C, temperature_k: f64, initial_credit: f64) -> Self {
        Self {
            cartridge,
            cbf: ThermodynamicCBF::new(temperature_k, initial_credit),
            _backend: std::marker::PhantomData,
        }
    }

    /// Evaluates a proposed topology state.
    /// This runs the full Cartridge functor pass and gates the result through the CBF.
    ///
    /// # Arguments
    /// * `raw_state` - The proposed UMST Cellular Sheaf
    /// * `info_gain` - The calculated mutual information resolved by this step.
    ///
    /// # Returns
    /// * Ok(VerifiedUMST, Reward) - The mathematically secured state and the backprop reward.
    /// * Err(String) - If the state violates the Clausius-Duhem Thermodynamic gate.
    pub fn evaluate_topology_step(
        &mut self,
        raw_state: crate::core::tensors::UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
    ) -> Result<
        (
            crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
            Tensor<B, 1>,
        ),
        String,
    > {
        // 1. Execute the physics simulation across the topological Cellular Sheaf
        let physical_result: PhysicalResult<B> = self.cartridge.compute_topology(&raw_state);

        // Keep metrics in Sparse Space [Batch, N_active_voxels]
        let free_energy = physical_result.free_energy.clone();
        let dissipation = physical_result.dissipation.clone();
        let cost = physical_result.cost.clone();

        // 2. Validate against the Thermodynamic Control Barrier Function
        // Sum dissipation across all voxels for the CBF macro check
        let d_int = dissipation.clone().sum_dim(1).squeeze(1);

        match self.cbf.verify_tensor_update::<B>(d_int, info_gain.clone()) {
            Ok(erasure_cost) => {
                // Spatial Reward = (Alpha * Performance) - (Beta * Dissipation) - (Gamma * CO2) - Erasure Cost
                let alpha = 1.0_f32;
                let beta = 0.5_f32;
                let gamma = 2.0_f32;

                let performance = free_energy.mul_scalar(alpha);
                let penalty = dissipation.mul_scalar(beta).add(cost.mul_scalar(gamma));

                // The erasure cost is paid uniformly across the topology
                let final_spatial_reward = performance.sub(penalty).sub_scalar(erasure_cost as f32);

                // Construct the mathematically secured tensor
                let verified_state = crate::core::tensors::VerifiedUMST::new(raw_state);

                // Flatten the spatial reward to a single scalar [Batch] for the policy gradient (Adjoint Method target)
                let total_reward = final_spatial_reward.sum_dim(1).squeeze(1);

                Ok((verified_state, total_reward))
            }
            Err(e) => {
                // The CBF rejected the state
                Err(format!("Transition Rejected by CBF: {}", e))
            }
        }
    }
}

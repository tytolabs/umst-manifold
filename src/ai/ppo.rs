// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! PPO gateway and reward wiring.
//!
//! Nodal diagnostics: [`crate::core::emergence::nodal_defect_tensor`],
//! [`crate::core::emergence::combine_nodal_for_reward`]; grid hotspots:
//! [`crate::core::emergence::EmergenceMonitor`].
//!
//! Optional structural-margin shaping: [`ManifoldGateway::zeta`] scales a per-batch
//! **mean** of [`PhysicalResult::safety_margin`](crate::core::traits::PhysicalResult::safety_margin)
//! added to the scalar reward. Default **ζ = 0** keeps the legacy reward and leaves the
//! thermodynamic CBF gate unchanged.
//!
//! With the **`information_density`** crate feature, [`ManifoldGateway::eta`] adds
//! **η · mean(information_density)** from [`PhysicalResult::information_density`](crate::core::traits::PhysicalResult::information_density)
//! the same way. Default **η = 0** preserves the reward without that term.

use crate::ai::cbf::ThermodynamicCBF;
use crate::core::traits::{IScienceCartridge, PhysicalResult};
use burn::tensor::{backend::Backend, Tensor};

/// The Gateway interface for Thermodynamic Topology Optimization.
/// It wraps the physical Cartridges and enforces the Thermodynamic CBF.
pub struct ManifoldGateway<B: Backend, C: IScienceCartridge<B>> {
    pub cartridge: C,
    pub cbf: ThermodynamicCBF,
    /// Safety-margin reward weight **ζ** (dimensionless). When non-zero, the scalar reward adds
    /// `ζ * mean_voxels(safety_margin)` per batch row (positive ζ rewards higher structural margin).
    /// **Default 0** in [`Self::new`] — no effect on CBF admissibility checks or legacy reward.
    pub zeta: f32,
    /// Information-density reward weight **η** (dimensionless). With the **`information_density`**
    /// feature enabled, when non-zero the scalar reward adds `η * mean_voxels(information_density)` per
    /// batch row (same reduction pattern as [`Self::zeta`] on `safety_margin`).
    /// **Default 0** in [`Self::new`]; ignored when the feature is off (field is not compiled into
    /// [`PhysicalResult`](crate::core::traits::PhysicalResult)).
    pub eta: f32,
    _backend: std::marker::PhantomData<B>,
}

impl<B: Backend, C: IScienceCartridge<B>> ManifoldGateway<B, C> {
    pub fn new(cartridge: C, temperature_k: f64, initial_credit: f64) -> Self {
        Self {
            cartridge,
            cbf: ThermodynamicCBF::new(temperature_k, initial_credit),
            zeta: 0.0_f32,
            eta: 0.0_f32,
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
    /// * Ok(VerifiedUMST, Reward) - The mathematically secured state and the per-batch scalar reward
    ///   (spatial thermodynamic terms plus **ζ · mean(safety_margin)** when [`ManifoldGateway::zeta`] ≠ 0,
    ///   and with **`information_density`**, **η · mean(information_density)** when [`ManifoldGateway::eta`] ≠ 0).
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
                let mut total_reward = final_spatial_reward.sum_dim(1).squeeze(1);
                if self.zeta != 0.0_f32 {
                    let margin_mean = physical_result.safety_margin.clone().mean_dim(1).squeeze(1);
                    total_reward = total_reward.add(margin_mean.mul_scalar(self.zeta));
                }
                #[cfg(feature = "information_density")]
                if self.eta != 0.0_f32 {
                    let info_mean = physical_result
                        .information_density
                        .clone()
                        .mean_dim(1)
                        .squeeze(1);
                    total_reward = total_reward.add(info_mean.mul_scalar(self.eta));
                }

                Ok((verified_state, total_reward))
            }
            Err(e) => {
                // The CBF rejected the state
                Err(format!("Transition Rejected by CBF: {}", e))
            }
        }
    }
}

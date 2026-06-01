// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use burn::tensor::{backend::Backend, Tensor};
use num_traits::ToPrimitive;

/// Control Barrier Function (CBF) for Thermodynamic Admissibility.
/// This enforces the Clausius-Duhem inequality and Landauer erasure costs.
///
/// It acts as a Natural Transformation between the Topology State and the physical
/// limits defined by global thermodynamic bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct ThermodynamicCBF {
    /// Local temperature of the physical system (Kelvin)
    pub temperature_k: f64,
    /// Available energy budget for the PPO agent, sourced from the external thermodynamic budget source.
    pub available_credit_joules: f64,
    /// Converts the **batch sum** of integrated dissipation `d_int` (cartridge-native `f32` units from
    /// [`PhysicalResult::dissipation`](crate::core::traits::PhysicalResult::dissipation) reductions)
    /// into additional joule-equivalent entropy production for the Clausius–Duhem check in
    /// [`Self::verify_tensor_update`]. Defaults to `1.0` (identity bridge); set per cartridge when
    /// dissipation is calibrated to SI. See `docs/DISSIPATION_CBF_AUDIT.md`.
    pub k_phys_dint_to_joules: f64,
}

impl ThermodynamicCBF {
    pub fn new(temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self {
            temperature_k,
            available_credit_joules: initial_credit_joules,
            k_phys_dint_to_joules: 1.0,
        }
    }

    /// Calculates the Landauer erasure cost required to resolve `bits_of_uncertainty`.
    /// Cost = k_B * T * ln(2) * N_bits
    pub fn calculate_landauer_cost(&self, bits_of_uncertainty: f64) -> f64 {
        crate::constants::landauer_bit_energy_joules(self.temperature_k) * bits_of_uncertainty
    }

    /// Evaluates if a proposed topology update from the RL agent is thermodynamically permissible.
    ///
    /// # Arguments
    /// * `entropy_production_joules` - The macroscopic entropy produced by the material state transition.
    /// * `bits_resolved` - The mutual information gained by the PPO agent choosing this topology mutation.
    ///
    /// # Returns
    /// * `Ok(cost_in_joules)` if the agent has enough energy credit and the transition satisfies Clausius-Duhem.
    /// * `Err` if the agent attempts an unphysical or unfunded topological change.
    pub fn verify_and_deduct_update(
        &mut self,
        entropy_production_joules: f64,
        bits_resolved: f64,
    ) -> Result<f64, String> {
        // 1. Calculate the minimum thermodynamic cost of this computation
        let erasure_cost = self.calculate_landauer_cost(bits_resolved);

        // 2. Check Global Thermodynamic Limits (Economic/Computational Bound)

        if erasure_cost > self.available_credit_joules {
            let avail = self.available_credit_joules;
            return Err(format!(
                "REJECTED: Insufficient Global Energy Credit. Required {erasure_cost} J, Available {avail} J.",
            ));
        }

        // 3. Clausius-Duhem Inequality (Physical Bound)
        // The physical entropy produced must be strictly non-negative when accounting for Landauer erasure.
        let generalized_entropy = entropy_production_joules - erasure_cost;
        if generalized_entropy < 0.0 {
            return Err(format!(
                "REJECTED: Clausius-Duhem Violation. Generalized entropy {generalized_entropy} < 0.",
            ));
        }

        // 4. Deduct the exact erasure cost from the agent's energy pool
        self.available_credit_joules -= erasure_cost;

        Ok(erasure_cost)
    }

    /// Processes a full batch gradient step to ensure gradient descent directions
    /// do not push the topology into a physically invalid state.
    ///
    /// **Scalar / host barrier:** sums `info_gain` and **`d_int`** then performs deliberate
    /// `.into_scalar()` syncs for the [`crate::ai::ppo::ManifoldGateway`] stack — total resolved bits
    /// and batch-summed dissipation become `f64` for Landauer cost, material entropy bookkeeping, and
    /// credit deduction. Prefer keeping other solver paths free of per-iteration `.into_scalar()` unless
    /// a kernel truly needs a host scalar for control flow.
    pub fn verify_tensor_update<B: Backend>(
        &mut self,
        d_int: Tensor<B, 1>,
        info_gain: Tensor<B, 1>,
    ) -> Result<f64, String> {
        // Compute precise bits resolved via tensor sum reduction
        // info_gain contains the mutual information per batch element
        let sum_bits_tensor = info_gain.sum();
        let total_bits_resolved = sum_bits_tensor.into_scalar().to_f64().unwrap_or(0.0);

        // Batch-summed integrated dissipation (gateway convention: one CBF scalar per topology step).
        let d_sum = d_int.sum().into_scalar().to_f64().unwrap_or(0.0);
        let d_sum_nonneg = d_sum.max(0.0);
        let dissipation_entropy_joules = self.k_phys_dint_to_joules * d_sum_nonneg;

        // Landauer floor (informational branch) plus irreversible material entropy from `d_int`.
        // The 5% margin remains a small conservative cushion on the Landauer term alone.
        let landauer_floor = self.calculate_landauer_cost(total_bits_resolved) * 1.05;
        let entropy_production = landauer_floor + dissipation_entropy_joules;

        self.verify_and_deduct_update(entropy_production, total_bits_resolved)
    }
}

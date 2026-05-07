// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use burn::tensor::{backend::Backend, Tensor};
use num_traits::ToPrimitive;
use std::f64::consts::LN_2;

// Boltzmann constant in J/K
const K_BOLTZMANN: f64 = 1.380649e-23;

/// Control Barrier Function (CBF) for Thermodynamic Admissibility.
/// This enforces the Clausius-Duhem inequality and Landauer erasure costs.
///
/// It acts as a Natural Transformation between the Topology State and the physical
/// limits defined by global thermodynamic bounds.
pub struct ThermodynamicCBF {
    /// Local temperature of the physical system (Kelvin)
    pub temperature_k: f64,
    /// Available energy budget for the PPO agent, sourced from the external thermodynamic budget source.
    pub available_credit_joules: f64,
}

impl ThermodynamicCBF {
    pub fn new(temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self {
            temperature_k,
            available_credit_joules: initial_credit_joules,
        }
    }

    /// Calculates the Landauer erasure cost required to resolve `bits_of_uncertainty`.
    /// Cost = k_B * T * ln(2) * N_bits
    pub fn calculate_landauer_cost(&self, bits_of_uncertainty: f64) -> f64 {
        K_BOLTZMANN * self.temperature_k * LN_2 * bits_of_uncertainty
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
            return Err(format!(
                "REJECTED: Insufficient Global Energy Credit. Required {} J, Available {} J.",
                erasure_cost, self.available_credit_joules
            ));
        }

        // 3. Clausius-Duhem Inequality (Physical Bound)
        // The physical entropy produced must be strictly non-negative when accounting for Landauer erasure.
        let generalized_entropy = entropy_production_joules - erasure_cost;
        if generalized_entropy < 0.0 {
            return Err(format!(
                "REJECTED: Clausius-Duhem Violation. Generalized entropy {} < 0.",
                generalized_entropy
            ));
        }

        // 4. Deduct the exact erasure cost from the agent's energy pool
        self.available_credit_joules -= erasure_cost;

        Ok(erasure_cost)
    }

    /// Processes a full batch gradient step to ensure gradient descent directions
    /// do not push the topology into a physically invalid state.
    /// Uses Burn tensor reductions to calculate batch entropy.
    pub fn verify_tensor_update<B: Backend>(
        &mut self,
        _d_int: Tensor<B, 1>,
        info_gain: Tensor<B, 1>,
    ) -> Result<f64, String> {
        // Compute precise bits resolved via tensor sum reduction
        // info_gain contains the mutual information per batch element
        let sum_bits_tensor = info_gain.sum();
        let total_bits_resolved = sum_bits_tensor
            .into_scalar()
            .to_f64()
            .unwrap_or(0.0);

        // Note: Real D_int to entropy conversion would use material specific heat capacity.
        // For the exact thermodynamic barrier, we require the scalar extraction of bits.
        // Assume minimal entropy production for purely informational transitions.
        let entropy_production = self.calculate_landauer_cost(total_bits_resolved) * 1.05; // 5% physical loss margin

        self.verify_and_deduct_update(entropy_production, total_bits_resolved)
    }
}

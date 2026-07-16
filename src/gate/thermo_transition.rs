// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Clausius–Duhem admissibility for bulk reaction-extent transitions (prototype thermodynamic gate).
//!
//! Ported from `umst-prototype/.../thermodynamic_filter.rs` — **wasm-free** manifold build.

use super::transition_proposal::{transition_outcome, ThermodynamicStateSnapshot};
use crate::core::material_transition::{MaterialTransitionParams, SubstrateMaterialParams};

/// Result of thermodynamic admissibility check
#[derive(Clone, Debug)]
pub struct AdmissibilityResult {
    pub accepted: bool,
    /// `D_int` value (W/m³) — Clausius–Duhem dissipation surrogate.
    pub dissipation: f64,
    pub mass_conserved: bool,
    pub energy_positive: bool,
}

impl AdmissibilityResult {
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.accepted
    }

    #[must_use]
    pub fn rejection_reason_code(&self) -> &'static str {
        super::verdict::AdmissibilityVerdict::from_transition_conjuncts(
            self.accepted,
            self.mass_conserved,
            self.energy_positive,
        )
        .as_str()
    }
}

/// Thermodynamic state for admissibility checking (`f64`, host units).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThermodynamicState {
    pub density: f64,         // kg/m³
    pub temperature: f64,     // K
    pub free_energy: f64,     // Helmholtz ψ (J/kg)
    pub entropy: f64,         // η (J/kg·K)
    pub reaction_extent: f64, // α (0-1)
    pub strength: f64,        // f_c (MPa)
}

impl ThermodynamicState {
    #[must_use]
    pub fn new() -> Self {
        ThermodynamicState {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.0,
            reaction_extent: 0.0,
            strength: 0.0,
        }
    }

    /// Create state from mix parameters using substrate-neutral defaults.
    #[must_use]
    pub fn from_mix(w_c: f64, alpha: f64, temp: f64) -> Self {
        Self::from_mix_with_params(w_c, alpha, temp, &SubstrateMaterialParams)
    }

    #[must_use]
    pub fn from_mix_with_params(
        w_c: f64,
        alpha: f64,
        temp: f64,
        params: &impl MaterialTransitionParams,
    ) -> Self {
        Self::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            params.default_intrinsic_strength_mpa(),
            params,
        )
    }

    /// Create state from mix parameters using explicit intrinsic strength (MPa).
    #[must_use]
    pub fn from_mix_calibrated(w_c: f64, alpha: f64, temp: f64, s_intrinsic: f64) -> Self {
        Self::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            s_intrinsic,
            &SubstrateMaterialParams,
        )
    }

    #[must_use]
    pub fn from_mix_calibrated_with_params(
        w_c: f64,
        alpha: f64,
        temp: f64,
        s_intrinsic: f64,
        params: &impl MaterialTransitionParams,
    ) -> Self {
        let q_reaction = params.reaction_enthalpy_j_per_kg();
        let x = 0.68 * alpha / (0.32 * alpha + w_c + 1e-6);
        let fc = s_intrinsic * x.powi(3);
        let psi = -q_reaction * alpha;

        ThermodynamicState {
            density: 2400.0 - 400.0 * w_c,
            temperature: temp,
            free_energy: psi,
            entropy: alpha * 0.1,
            reaction_extent: alpha,
            strength: fc,
        }
    }
}

impl Default for ThermodynamicState {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
fn thermodynamic_state_snapshot(state: &ThermodynamicState) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: state.density,
        temperature: state.temperature,
        free_energy: state.free_energy,
        entropy: state.entropy,
        reaction_extent: state.reaction_extent,
        strength: state.strength,
    }
}

/// Pure transition evaluator: `(old, new, dt, ε) → outcome` with no gate telemetry.
///
/// FP SSOT; [`ThermodynamicGate::check_transition`] wraps this with accept/reject counters.
#[must_use]
pub fn thermo_gate_transition_outcome(
    old_state: &ThermodynamicState,
    new_state: &ThermodynamicState,
    dt_s: f64,
    tolerance: f64,
) -> AdmissibilityResult {
    let outcome = transition_outcome(
        &thermodynamic_state_snapshot(old_state),
        &thermodynamic_state_snapshot(new_state),
        dt_s,
        tolerance,
    );
    AdmissibilityResult {
        accepted: outcome.accepted,
        dissipation: outcome.dissipation,
        mass_conserved: outcome.mass_conserved,
        energy_positive: outcome.energy_positive,
    }
}

/// Thermodynamic gate (formerly `ThermodynamicFilter`).
#[derive(Clone, Debug)]
pub struct ThermodynamicGate {
    tolerance: f64,
    rejections: u64,
    acceptances: u64,
}

impl Default for ThermodynamicGate {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermodynamicGate {
    #[must_use]
    pub fn new() -> Self {
        ThermodynamicGate {
            tolerance: 1e-6,
            rejections: 0,
            acceptances: 0,
        }
    }

    #[must_use]
    pub fn with_tolerance(tolerance: f64) -> Self {
        ThermodynamicGate {
            tolerance,
            rejections: 0,
            acceptances: 0,
        }
    }

    /// Substantive transition check ported from Algorithm 1 in the UMST prototypes.
    #[must_use]
    pub fn transition_proposal_admissible(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> bool {
        self.check_transition(old_state, new_state, dt_s).accepted
    }

    /// Full transition evaluation with accounting statistics (telemetry wrapper).
    pub fn check_transition(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> AdmissibilityResult {
        let result = thermo_gate_transition_outcome(old_state, new_state, dt_s, self.tolerance);
        if result.accepted {
            self.acceptances += 1;
        } else {
            self.rejections += 1;
        }
        result
    }

    #[must_use]
    pub fn stats_summary(&self) -> String {
        let total = self.acceptances + self.rejections;
        if total == 0 {
            return "No transitions checked".to_string();
        }
        let rate = self.acceptances as f64 / total as f64 * 100.0;
        format!(
            "Accepted: {}, Rejected: {}, Rate: {:.1}%",
            self.acceptances, self.rejections, rate
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::material_transition::SubstrateMaterialParams;

    #[test]
    fn pure_outcome_matches_telemetry_path_without_counters() {
        let old =
            ThermodynamicState::from_mix_with_params(0.5, 0.4, 293.0, &SubstrateMaterialParams);
        let new =
            ThermodynamicState::from_mix_with_params(0.5, 0.65, 293.0, &SubstrateMaterialParams);
        let dt = 86_400.0;
        let pure = thermo_gate_transition_outcome(&old, &new, dt, 1e-6);
        let mut gate = ThermodynamicGate::new();
        let telemetry = gate.check_transition(&old, &new, dt);
        assert_eq!(pure.accepted, telemetry.accepted);
        assert_eq!(pure.mass_conserved, telemetry.mass_conserved);
        assert_eq!(pure.energy_positive, telemetry.energy_positive);
        assert!((pure.dissipation - telemetry.dissipation).abs() < 1e-12);
        assert!(gate.stats_summary().contains("Accepted: 1"));
    }
}

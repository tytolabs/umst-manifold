// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Clausius–Duhem admissibility for cement hydration transitions (prototype thermodynamic gate).
//!
//! Ported from `umst-prototype/.../thermodynamic_filter.rs` — **wasm-free** manifold build.

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
        use super::verdict::AdmissibilityVerdict;
        if self.accepted {
            AdmissibilityVerdict::ACCEPTED
        } else if !self.mass_conserved {
            AdmissibilityVerdict::MASS_VIOLATION
        } else if !self.energy_positive {
            AdmissibilityVerdict::NEGATIVE_DISSIPATION
        } else {
            AdmissibilityVerdict::UNKNOWN
        }
    }
}

/// Thermodynamic state for admissibility checking (`f64`, host units).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThermodynamicState {
    pub density: f64,          // kg/m³
    pub temperature: f64,      // K
    pub free_energy: f64,      // Helmholtz ψ (J/kg)
    pub entropy: f64,          // η (J/kg·K)
    pub hydration_degree: f64, // α (0-1)
    pub strength: f64,         // f_c (MPa)
}

const S_INT_DEFAULT: f64 = 240.0;
const Q_HYDRATION: f64 = 450.0;

impl ThermodynamicState {
    #[must_use]
    pub fn new() -> Self {
        ThermodynamicState {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.0,
            hydration_degree: 0.0,
            strength: 0.0,
        }
    }

    /// Create state from mix parameters using default intrinsic strength (240 MPa).
    #[must_use]
    pub fn from_mix(w_c: f64, alpha: f64, temp: f64) -> Self {
        Self::from_mix_calibrated(w_c, alpha, temp, S_INT_DEFAULT)
    }

    #[must_use]
    pub fn from_mix_calibrated(w_c: f64, alpha: f64, temp: f64, s_intrinsic: f64) -> Self {
        let x = 0.68 * alpha / (0.32 * alpha + w_c + 1e-6);
        let fc = s_intrinsic * x.powi(3);
        let psi = -Q_HYDRATION * alpha;

        ThermodynamicState {
            density: 2400.0 - 400.0 * w_c,
            temperature: temp,
            free_energy: psi,
            entropy: alpha * 0.1,
            hydration_degree: alpha,
            strength: fc,
        }
    }
}

impl Default for ThermodynamicState {
    fn default() -> Self {
        Self::new()
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

    /// `mix_proposal` — substantive transition check ported from Algorithm 1 in the UMST prototypes.
    #[must_use]
    pub fn mix_proposal_admissible(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> bool {
        self.check_transition(old_state, new_state, dt_s).accepted
    }

    /// Full transition evaluation with accounting statistics.
    pub fn check_transition(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> AdmissibilityResult {
        let mass_conserved = (new_state.density - old_state.density).abs() < 100.0;

        let rho = (old_state.density + new_state.density) / 2.0;
        let psi_dot = (new_state.free_energy - old_state.free_energy) / (dt_s + 1e-10);
        let d_int = -rho * psi_dot;

        let strength_valid = new_state.strength >= old_state.strength - self.tolerance;

        let energy_positive = d_int >= -self.tolerance && strength_valid;
        let accepted = mass_conserved && energy_positive;

        if accepted {
            self.acceptances += 1;
        } else {
            self.rejections += 1;
        }

        AdmissibilityResult {
            accepted,
            dissipation: d_int,
            mass_conserved,
            energy_positive,
        }
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

    #[test]
    fn forward_hydration_admissible() {
        let mut gate = ThermodynamicGate::new();
        let old = ThermodynamicState::from_mix(0.5, 0.3, 293.0);
        let new = ThermodynamicState::from_mix(0.5, 0.5, 293.0);
        let r = gate.check_transition(&old, &new, 3600.0);
        assert!(r.accepted);
        assert!(r.dissipation > 0.0);
    }
}

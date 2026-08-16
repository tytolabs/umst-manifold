// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Phase 0b — canonical **Core** `gate<R>` predicate (blueprint §7 0b · §17.3 · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! **Universal conjuncts only:** Mass Conservation + Clausius–Duhem in open-system form
//! `𝒟 − P_input ≥ 0` (`P_input = 0` default for passive matter).
//!
//! Material-specific conjuncts (strength monotonicity, hydration / reaction-extent irreversibility)
//! live in [`super::material_gate`] and in the cartridge's constitutive response — **not** here.
//!
//! The legacy host cluster [`super::transition_proposal::transition_outcome`] composes Core +
//! Material for backward-compatible parity until Phase 0d routes all callers through one surface.

/// Mass jump band (kg/m³) — mirrors umst-math `GATE_MASS_TOLERANCE_KG_M3` / registry.
pub const GATE_MASS_TOLERANCE_KG_M3: f64 = 100.0;

/// Scalar constitutive response consumed by Core `gate<R>` (shape mirrors `umst-cartridge-api` §A1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarConstitutiveResponse {
    /// Internal dissipation 𝒟 (host cluster uses `D_int = −ρ ψ̇`).
    pub dissipation: f64,
    /// External power input `P_input`; zero for passive systems (default).
    pub power_input: f64,
}

impl ScalarConstitutiveResponse {
    /// Passive matter: `P_input = 0`.
    #[must_use]
    pub const fn passive(dissipation: f64) -> Self {
        Self {
            dissipation,
            power_input: 0.0,
        }
    }

    /// Open-system net dissipation `𝒟 − P_input`.
    #[must_use]
    pub fn net_dissipation(self) -> f64 {
        self.dissipation - self.power_input
    }
}

/// Outcome of the Core gate — universal conjuncts only.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoreGateOutcome {
    pub accepted: bool,
    pub mass_conserved: bool,
    pub clausius_duhem: bool,
    pub dissipation: f64,
    pub power_input: f64,
    pub net_dissipation: f64,
}

/// Pure Core gate: Mass Conservation ∧ Clausius–Duhem (`𝒟 − P_input ≥ −ε`).
#[must_use]
pub fn core_gate(
    response: &ScalarConstitutiveResponse,
    mass_conserved: bool,
    tolerance: f64,
) -> CoreGateOutcome {
    let net_dissipation = response.net_dissipation();
    let clausius_duhem = net_dissipation >= -tolerance;
    let accepted = mass_conserved && clausius_duhem;
    CoreGateOutcome {
        accepted,
        mass_conserved,
        clausius_duhem,
        dissipation: response.dissipation,
        power_input: response.power_input,
        net_dissipation,
    }
}

/// Canonical alias for blueprint `gate<R>(ConstitutiveResponse) -> Verdict` (scalar `R` today).
#[must_use]
pub fn gate(
    response: &ScalarConstitutiveResponse,
    mass_conserved: bool,
    tolerance: f64,
) -> CoreGateOutcome {
    core_gate(response, mass_conserved, tolerance)
}

/// Mass-conservation witness from a density jump.
#[must_use]
pub fn mass_conserved_between_densities(old_density: f64, new_density: f64) -> bool {
    (new_density - old_density).abs() < GATE_MASS_TOLERANCE_KG_M3
}

/// Lift a thermodynamic transition step into a scalar constitutive response.
#[must_use]
pub fn scalar_response_from_transition(
    old_density: f64,
    new_density: f64,
    old_free_energy: f64,
    new_free_energy: f64,
    dt: f64,
    power_input: f64,
) -> ScalarConstitutiveResponse {
    let rho = (old_density + new_density) / 2.0;
    let psi_dot = (new_free_energy - old_free_energy) / (dt + 1e-10);
    let d_int = -rho * psi_dot;
    ScalarConstitutiveResponse {
        dissipation: d_int,
        power_input,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_gate_accepts_mass_and_cd_with_zero_power_input() {
        let response = ScalarConstitutiveResponse::passive(1.0);
        let outcome = core_gate(&response, true, 1e-6);
        assert!(outcome.mass_conserved);
        assert!(outcome.clausius_duhem);
        assert!(outcome.accepted);
        assert_eq!(outcome.power_input, 0.0);
    }

    #[test]
    fn core_gate_open_system_subtracts_power_input() {
        let response = ScalarConstitutiveResponse {
            dissipation: 5.0,
            power_input: 3.0,
        };
        let outcome = core_gate(&response, true, 1e-6);
        assert!((outcome.net_dissipation - 2.0).abs() < 1e-12);
        assert!(outcome.clausius_duhem);
        assert!(outcome.accepted);
    }

    #[test]
    fn core_gate_rejects_negative_net_dissipation() {
        let response = ScalarConstitutiveResponse {
            dissipation: 1.0,
            power_input: 5.0,
        };
        let outcome = core_gate(&response, true, 1e-6);
        assert!(!outcome.clausius_duhem);
        assert!(!outcome.accepted);
    }

    #[test]
    fn core_gate_rejects_mass_violation_independently_of_cd() {
        let response = ScalarConstitutiveResponse::passive(10.0);
        let outcome = core_gate(&response, false, 1e-6);
        assert!(!outcome.mass_conserved);
        assert!(outcome.clausius_duhem);
        assert!(!outcome.accepted);
    }
}

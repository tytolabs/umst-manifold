// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Manifold shim — SSOT in `umst-gate` (P2.0).
pub use umst_gate::core_gate::{
    core_gate, gate, mass_conserved_between_densities, scalar_response_from_transition,
    AdmissibilityResponse, CoreGateOutcome, ScalarConstitutiveResponse, GATE_MASS_TOLERANCE_KG_M3,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::{ThermodynamicStateSnapshot, TRANSITION_TOLERANCE};
    use crate::gate::verdict::{ConjunctVerdict, GateRejectReason};
    use umst_cartridge_api::{ConstitutiveResponse, ScalarAlgebra};

    /// Registry anchor — `umst-math` / `umst-dec` bulk density jump band (kg/m³).
    const REGISTRY_MASS_BAND_KG_M3: f64 = 100.0;

    #[test]
    fn manifold_shim_ssot_mass_tolerance_matches_registry() {
        assert!(
            (GATE_MASS_TOLERANCE_KG_M3 - REGISTRY_MASS_BAND_KG_M3).abs() < f64::EPSILON,
            "manifold shim must re-export canonical GATE_MASS_TOLERANCE_KG_M3"
        );
    }

    #[test]
    fn mass_conserved_between_densities_respects_registry_band() {
        // Host density from `from_mix_calibrated(0.45, …)` — see transition_proposal census.
        let rho = 2220.0;
        assert!(mass_conserved_between_densities(rho, rho));
        assert!(mass_conserved_between_densities(rho, rho + 50.0));
        assert!(!mass_conserved_between_densities(
            rho,
            rho + REGISTRY_MASS_BAND_KG_M3
        ));
    }

    #[test]
    fn core_gate_accepts_phase0b_calibrated_transition_via_scalar_lift() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.35;
        new.free_energy = old.free_energy - 100.0;

        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            0.0,
        );
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!(outcome.is_accepted());
        assert!(outcome.is_clausius_duhem());
        assert!(outcome.is_mass_conserved());
        assert!(outcome.power_input.abs() < 1e-12);
        assert!(outcome.dissipation > 0.0);
    }

    #[test]
    fn core_gate_open_system_subtracts_power_input_phase0b_fixture() {
        let response = ScalarConstitutiveResponse {
            dissipation: 10.0,
            power_input: 4.0,
        };
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!((outcome.net_dissipation - 6.0).abs() < 1e-12);
        assert!(outcome.is_accepted());
    }

    #[test]
    fn core_gate_rejects_negative_net_dissipation() {
        let response = ScalarConstitutiveResponse {
            dissipation: 1.0,
            power_input: 5.0,
        };
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
        assert!(!outcome.is_clausius_duhem());
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn core_gate_rejects_mass_violation_independently_of_cd() {
        let response = ScalarConstitutiveResponse::passive(10.0);
        let outcome = core_gate(&response, false, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
        assert!(!outcome.is_mass_conserved());
        assert!(outcome.is_clausius_duhem());
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn core_gate_tolerance_accepts_at_negative_epsilon_boundary() {
        let tol = TRANSITION_TOLERANCE;
        let response = ScalarConstitutiveResponse::passive(-tol);
        let outcome = core_gate(&response, true, tol);
        assert!(outcome.is_clausius_duhem());
        assert!(outcome.is_accepted());
    }

    #[test]
    fn core_gate_tolerance_rejects_below_negative_epsilon() {
        let tol = TRANSITION_TOLERANCE;
        let response = ScalarConstitutiveResponse::passive(-tol - 1e-9);
        let outcome = core_gate(&response, true, tol);
        assert!(!outcome.is_clausius_duhem());
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn gate_alias_matches_core_gate_on_scalar_response() {
        let response = ScalarConstitutiveResponse::passive(8.5);
        let via_core = core_gate(&response, true, TRANSITION_TOLERANCE);
        let via_gate = gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(via_core, via_gate);
        assert_eq!(via_gate.conjunct_verdict(), ConjunctVerdict::Accepted);
    }

    #[test]
    fn core_gate_idempotent_on_equilibrated_response() {
        let response = ScalarConstitutiveResponse::passive(0.0);
        let first = core_gate(&response, true, TRANSITION_TOLERANCE);
        let second = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(first, second, "gate<R> must not drift on re-application");
        assert!(first.is_accepted());
    }

    #[test]
    fn gate_accepts_constitutive_response_from_cartridge_api() {
        let response = ConstitutiveResponse::passive(0.0, 8.0, 0.0);
        let mut open = response;
        open.power_input = 2.0;
        let outcome = gate(&open, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!((outcome.net_dissipation - 6.0).abs() < 1e-12);
    }

    #[test]
    fn scalar_response_from_transition_honors_power_input_leg() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
        let power_input = 3.0;
        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            power_input,
        );
        assert!((response.power_input - power_input).abs() < 1e-12);
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.power_input, power_input);
        assert_eq!(outcome.net_dissipation, response.dissipation - power_input);
    }

    #[test]
    fn admissibility_response_trait_net_dissipation_via_scalar() {
        let response = ScalarConstitutiveResponse {
            dissipation: 5.0,
            power_input: 3.0,
        };
        assert!((AdmissibilityResponse::net_dissipation(&response) - 2.0).abs() < 1e-12);
        assert!((response.net_dissipation() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn core_gate_mass_violation_precedes_negative_dissipation() {
        let response = ScalarConstitutiveResponse {
            dissipation: 1.0,
            power_input: 5.0,
        };
        let outcome = core_gate(&response, false, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
        assert!(!outcome.is_mass_conserved());
        assert!(!outcome.is_accepted());
        // Verdict short-circuits on mass; CD witness is not NegativeDissipation reason.
        assert!(outcome.is_clausius_duhem());
    }

    #[test]
    fn mass_conserved_between_densities_symmetric_registry_band() {
        let rho = 2220.0;
        assert!(mass_conserved_between_densities(rho, rho - 50.0));
        assert!(!mass_conserved_between_densities(
            rho,
            rho - REGISTRY_MASS_BAND_KG_M3
        ));
    }

    #[test]
    fn core_gate_accepts_zero_net_dissipation_open_system() {
        let response = ScalarConstitutiveResponse {
            dissipation: 7.5,
            power_input: 7.5,
        };
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!(outcome.net_dissipation.abs() < 1e-12);
        assert!(outcome.is_clausius_duhem());
    }

    #[test]
    fn core_gate_outcome_copies_response_legs() {
        let response = ScalarConstitutiveResponse {
            dissipation: 12.0,
            power_input: 3.5,
        };
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(outcome.dissipation, response.dissipation);
        assert_eq!(outcome.power_input, response.power_input);
        assert_eq!(outcome.net_dissipation, response.net_dissipation());
    }

    #[test]
    fn gate_alias_idempotent_on_open_system_response() {
        let response = ScalarConstitutiveResponse {
            dissipation: 10.0,
            power_input: 4.0,
        };
        let first = gate(&response, true, TRANSITION_TOLERANCE);
        let second = gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(first, second);
        assert_eq!(first.conjunct_verdict(), ConjunctVerdict::Accepted);
    }

    #[test]
    fn scalar_response_from_transition_d_int_phase0b_alpha_ramp() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.35;
        new.free_energy = old.free_energy - 80.0;
        let dt = 1.0;
        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            dt,
            0.0,
        );
        let rho_mid = (old.density + new.density) / 2.0;
        let psi_dot = (new.free_energy - old.free_energy) / (dt + 1e-10);
        let expected_d_int = -rho_mid * psi_dot;
        assert!((response.dissipation - expected_d_int).abs() < 1e-9);
        assert!(mass_conserved_between_densities(old.density, new.density));
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(outcome.dissipation > 0.0);
    }

    #[test]
    fn scalar_response_from_transition_rejects_cd_on_free_energy_gain() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.free_energy = old.free_energy + 50.0;
        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            0.0,
        );
        assert!(response.dissipation < 0.0);
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn w8e14_core_gate_passive_response_accepts_at_zero_net() {
        let response = ScalarConstitutiveResponse::passive(0.0);
        let outcome = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(outcome.net_dissipation.abs() < 1e-12);
    }
}

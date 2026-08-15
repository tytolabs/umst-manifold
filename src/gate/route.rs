// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0d — canonical gate routing surface (blueprint §7 0d · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! **Single entry for admissibility:** [`core_gate`] (Mass + CD) ∧ consumer
//! [`evaluate_material_conjuncts`] composed via
//! [`super::transition_proposal::transition_outcome`]. Every compute/consume site delegates here —
//! no second predicate.

use super::core_gate::{
    core_gate, mass_conserved_between_densities, scalar_response_from_transition, CoreGateOutcome,
};
use super::material_gate::{MaterialGateOutcome, MaterialTransitionWitness};
use super::thermo_transition::{thermo_gate_transition_outcome, ThermodynamicState};
use super::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome,
    TRANSITION_TOLERANCE,
};
use umst_cartridge_concrete::evaluate_material_conjuncts;

/// Canonical snapshot transition — composes Core + Material conjuncts (parity SSOT).
#[must_use]
pub fn canonical_transition_outcome(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt_s: f64,
) -> ThermodynamicTransitionOutcome {
    transition_outcome(old_state, new_state, dt_s, TRANSITION_TOLERANCE)
}

/// Canonical snapshot admissibility bool.
#[must_use]
pub fn canonical_transition_admissible(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt_s: f64,
) -> bool {
    canonical_transition_outcome(old_state, new_state, dt_s).is_accepted()
}

/// Host [`ThermodynamicState`] variant (cartridge `manifest-bridge` path).
#[must_use]
pub fn canonical_thermo_transition_admissible(
    old_state: &ThermodynamicState,
    new_state: &ThermodynamicState,
    dt_s: f64,
) -> bool {
    thermo_gate_transition_outcome(old_state, new_state, dt_s, TRANSITION_TOLERANCE).is_accepted()
}

/// Core-only margin for cold/tensor alignment (Mass + CD via [`core_gate`]).
#[must_use]
pub fn canonical_core_gate_outcome(
    old_density: f64,
    new_density: f64,
    old_free_energy: f64,
    new_free_energy: f64,
    dt_s: f64,
    power_input: f64,
) -> CoreGateOutcome {
    let mass_conserved = mass_conserved_between_densities(old_density, new_density);
    let response = scalar_response_from_transition(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
        power_input,
    );
    core_gate(&response, mass_conserved, TRANSITION_TOLERANCE)
}

/// Material conjunct witness routed through canonical tolerance.
#[must_use]
pub fn canonical_material_gate_outcome(
    old_strength: f64,
    new_strength: f64,
    old_reaction_extent: f64,
    new_reaction_extent: f64,
) -> MaterialGateOutcome {
    evaluate_material_conjuncts(
        &MaterialTransitionWitness {
            old_strength,
            new_strength,
            old_reaction_extent,
            new_reaction_extent,
        },
        TRANSITION_TOLERANCE,
    )
}

/// Strength upper-bound conjunct (HTTP manifest / C-ABI parity).
#[must_use]
pub fn canonical_strength_upper_bound_admissible(strength_mpa: f64, max_strength_mpa: f64) -> bool {
    strength_mpa.is_finite() && max_strength_mpa.is_finite() && strength_mpa <= max_strength_mpa
}

#[cfg(test)]
mod tests {
    use super::super::core_gate::core_gate;
    use super::super::material_gate::MaterialTransitionWitness;
    use super::super::verdict::{AdmissibilityVerdict, ConjunctVerdict, GateRejectReason};
    use super::*;
    use crate::core::material_transition::SubstrateMaterialParams;
    use umst_cartridge_concrete::evaluate_material_conjuncts;

    #[test]
    fn route_delegates_to_transition_outcome() {
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let routed = canonical_transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0);
        let direct = transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0, TRANSITION_TOLERANCE);
        assert_eq!(routed, direct);
    }

    #[test]
    fn canonical_transition_admissible_matches_outcome_accepted() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        assert_eq!(
            canonical_transition_admissible(&old, &new, dt),
            canonical_transition_outcome(&old, &new, dt).is_accepted()
        );
        assert!(canonical_transition_admissible(&old, &new, dt));
    }

    #[test]
    fn canonical_transition_admissible_rejects_reaction_extent_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.1;
        let dt = 1.0;
        assert!(!canonical_transition_admissible(&old, &new, dt));
        let outcome = canonical_transition_outcome(&old, &new, dt);
        assert!(!outcome.is_reaction_extent_irreversible());
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn canonical_transition_admissible_rejects_malformed_dt() {
        let idle = ThermodynamicStateSnapshot::new_idle();
        assert!(!canonical_transition_admissible(&idle, &idle, -1.0));
        assert_eq!(
            canonical_transition_outcome(&idle, &idle, -1.0).verdict,
            ConjunctVerdict::Rejected(GateRejectReason::MalformedInput)
        );
    }

    #[test]
    fn canonical_thermo_transition_admissible_delegates_to_thermo_gate() {
        let old =
            ThermodynamicState::from_mix_with_params(0.5, 0.4, 293.0, &SubstrateMaterialParams);
        let new =
            ThermodynamicState::from_mix_with_params(0.5, 0.65, 293.0, &SubstrateMaterialParams);
        let dt = 86_400.0;
        assert_eq!(
            canonical_thermo_transition_admissible(&old, &new, dt),
            thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE).is_accepted()
        );
        assert!(canonical_thermo_transition_admissible(&old, &new, dt));
    }

    #[test]
    fn canonical_thermo_transition_admissible_matches_snapshot_route() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicState::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let thermo = canonical_thermo_transition_admissible(&old, &new, dt);
        let snapshot_old = ThermodynamicStateSnapshot {
            density: old.density,
            temperature: old.temperature,
            free_energy: old.free_energy,
            entropy: old.entropy,
            reaction_extent: old.reaction_extent,
            strength: old.strength,
        };
        let snapshot_new = ThermodynamicStateSnapshot {
            density: new.density,
            temperature: new.temperature,
            free_energy: new.free_energy,
            entropy: new.entropy,
            reaction_extent: new.reaction_extent,
            strength: new.strength,
        };
        assert_eq!(
            thermo,
            canonical_transition_admissible(&snapshot_old, &snapshot_new, dt)
        );
    }

    #[test]
    fn canonical_core_gate_outcome_delegates_to_core_gate_phase0b_fixture() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.35;
        new.free_energy = old.free_energy - 100.0;
        let dt = 1.0;
        let power_input = 0.0;
        let routed = canonical_core_gate_outcome(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            dt,
            power_input,
        );
        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            dt,
            power_input,
        );
        let direct = core_gate(
            &response,
            mass_conserved_between_densities(old.density, new.density),
            TRANSITION_TOLERANCE,
        );
        assert_eq!(routed, direct);
        assert_eq!(routed.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!(routed.is_accepted());
    }

    #[test]
    fn canonical_core_gate_outcome_rejects_mass_violation() {
        let rho = 2220.0;
        let violated = rho + 200.0;
        let routed = canonical_core_gate_outcome(rho, violated, 0.0, -1.0, 1.0, 0.0);
        assert_eq!(
            routed.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
        assert!(!routed.is_accepted());
        assert!(!mass_conserved_between_densities(rho, violated));
    }

    #[test]
    fn canonical_core_gate_outcome_honors_open_system_power_input() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
        let power_input = 3.0;
        let routed = canonical_core_gate_outcome(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            power_input,
        );
        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            power_input,
        );
        let direct = core_gate(
            &response,
            mass_conserved_between_densities(old.density, new.density),
            TRANSITION_TOLERANCE,
        );
        assert_eq!(routed, direct);
        assert_eq!(routed.power_input, power_input);
        assert_eq!(routed.net_dissipation, response.dissipation - power_input);
    }

    #[test]
    fn canonical_material_gate_outcome_delegates_to_evaluate_material_conjuncts() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let routed = canonical_material_gate_outcome(
            old.strength,
            new.strength,
            old.reaction_extent,
            new.reaction_extent,
        );
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let direct = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert_eq!(routed, direct);
        assert!(routed.is_accepted());
    }

    #[test]
    fn canonical_material_gate_outcome_rejects_strength_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.9, 293.15, 80.0);
        let new_strength = 10.0;
        assert!(old.strength > new_strength);
        let routed = canonical_material_gate_outcome(
            old.strength,
            new_strength,
            old.reaction_extent,
            old.reaction_extent,
        );
        assert!(!routed.is_accepted());
        assert!(!routed.is_strength_monotonic());
    }

    #[test]
    fn canonical_strength_upper_bound_admissible_finite_cases() {
        assert!(canonical_strength_upper_bound_admissible(30.0, 80.0));
        assert!(canonical_strength_upper_bound_admissible(80.0, 80.0));
        assert!(!canonical_strength_upper_bound_admissible(81.0, 80.0));
    }

    #[test]
    fn canonical_strength_upper_bound_rejects_non_finite() {
        assert!(!canonical_strength_upper_bound_admissible(f64::NAN, 80.0));
        assert!(!canonical_strength_upper_bound_admissible(
            30.0,
            f64::INFINITY
        ));
        assert!(!canonical_strength_upper_bound_admissible(
            f64::INFINITY,
            80.0
        ));
    }

    #[test]
    fn canonical_transition_outcome_idle_self_transition_accepted() {
        let idle = ThermodynamicStateSnapshot::new_idle();
        let dt = 1.0;
        let outcome = canonical_transition_outcome(&idle, &idle, dt);
        assert!(outcome.is_accepted());
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        assert!(outcome.dissipation.is_finite());
        assert_eq!(
            canonical_transition_admissible(&idle, &idle, dt),
            outcome.is_accepted()
        );
    }

    #[test]
    fn canonical_transition_outcome_rejects_zero_dt() {
        let idle = ThermodynamicStateSnapshot::new_idle();
        assert!(!canonical_transition_admissible(&idle, &idle, 0.0));
        assert_eq!(
            canonical_transition_outcome(&idle, &idle, 0.0).verdict,
            ConjunctVerdict::Rejected(GateRejectReason::MalformedInput)
        );
    }

    #[test]
    fn canonical_transition_outcome_rejects_strength_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.9, 293.15, 80.0);
        let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.95, 293.15, 80.0);
        new.strength = 10.0;
        assert!(old.strength > new.strength);
        let dt = 1.0;
        let outcome = canonical_transition_outcome(&old, &new, dt);
        assert!(!outcome.is_accepted());
        assert!(!canonical_transition_admissible(&old, &new, dt));
    }

    #[test]
    fn canonical_transition_outcome_conjunct_verdict_matches_rest_ladder() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let outcome = canonical_transition_outcome(&old, &new, 1.0);
        assert_eq!(
            outcome.conjunct_verdict().is_accepted(),
            outcome.is_accepted()
        );
        assert_eq!(outcome.verdict(), AdmissibilityVerdict::Accepted);
        assert!(outcome.is_mass_conserved());
        assert!(outcome.is_energy_positive());
        assert!(outcome.is_reaction_extent_irreversible());
    }

    #[test]
    fn canonical_core_gate_outcome_rejects_excessive_power_input() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
        let power_input = 1.0e6;
        let routed = canonical_core_gate_outcome(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            power_input,
        );
        assert!(!routed.is_accepted());
        assert_eq!(
            routed.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
        assert!(!routed.is_clausius_duhem());
    }

    #[test]
    fn canonical_core_gate_outcome_accepts_equilibrated_identity() {
        let snap = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let routed = canonical_core_gate_outcome(
            snap.density,
            snap.density,
            snap.free_energy,
            snap.free_energy,
            1.0,
            0.0,
        );
        assert!(routed.is_accepted());
        assert!(routed.is_mass_conserved());
        assert!(routed.is_clausius_duhem());
        assert!(routed.dissipation.abs() < 1e-12);
    }

    #[test]
    fn canonical_material_gate_outcome_rejects_reaction_extent_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let routed = canonical_material_gate_outcome(
            old.strength,
            old.strength + 2.0,
            old.reaction_extent,
            0.1,
        );
        assert!(!routed.is_accepted());
        assert!(!routed.is_reaction_extent_irreversible());
        assert_eq!(
            routed.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::ReactionExtentRegression)
        );
    }

    #[test]
    fn canonical_thermo_transition_admissible_rejects_malformed_dt() {
        let idle = ThermodynamicState::new();
        assert!(!canonical_thermo_transition_admissible(&idle, &idle, -1.0));
        assert!(!canonical_thermo_transition_admissible(&idle, &idle, 0.0));
    }

    #[test]
    fn canonical_thermo_transition_admissible_rejects_extent_regression() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old.clone();
        new.reaction_extent = 0.1;
        let dt = 1.0;
        assert!(!canonical_thermo_transition_admissible(&old, &new, dt));
        assert_eq!(
            canonical_thermo_transition_admissible(&old, &new, dt),
            thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE).is_accepted()
        );
    }

    #[test]
    fn canonical_route_honors_transition_tolerance_constant() {
        assert_eq!(TRANSITION_TOLERANCE, 1e-6);
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let routed = canonical_transition_outcome(&old, &new, dt);
        let direct = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert_eq!(routed, direct);
    }

    #[test]
    fn canonical_strength_upper_bound_admits_negative_strength_below_bound() {
        assert!(canonical_strength_upper_bound_admissible(-1.0, 80.0));
        assert!(!canonical_strength_upper_bound_admissible(81.0, 80.0));
    }

    #[test]
    fn w8e14_canonical_transition_outcome_identity_is_admitted() {
        let idle = ThermodynamicStateSnapshot::new_idle();
        let outcome = canonical_transition_outcome(&idle, &idle, 1.0);
        assert!(outcome.is_accepted());
    }
}

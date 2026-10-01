// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Manifold shim — SSOT wire types in `umst-gate` (P2.0); predicate SSOT in
//! `umst_cartridge_concrete::evaluate_material_conjuncts` (B4 @ Z20).
pub use umst_gate::material_gate::{MaterialGateOutcome, MaterialTransitionWitness};

#[cfg(test)]
mod tests {
use crate::gate::transition_proposal::{
    CENSUS_BINDER_LIQUID_RATIO, CENSUS_DT_ONE_HOUR_S, CENSUS_FREE_ENERGY_DROP_J,
    CENSUS_GOLDEN_IDENTITY_ENTROPY, CENSUS_GOLDEN_IDENTITY_FREE_ENERGY_J,
    CENSUS_GOLDEN_IDENTITY_REACTION_EXTENT, CENSUS_GOLDEN_IDENTITY_STRENGTH_MPA,
    CENSUS_GOLDEN_MASS_REJECT_ENTROPY, CENSUS_GOLDEN_MASS_REJECT_STRENGTH_MPA,
    CENSUS_GOLDEN_NEGATIVE_DISSIPATION_ENTROPY, CENSUS_GOLDEN_NEGATIVE_DISSIPATION_FREE_ENERGY_J,
    CENSUS_GOLDEN_NEGATIVE_DISSIPATION_FREE_ENERGY_SPIKE_J,
    CENSUS_GOLDEN_NEGATIVE_DISSIPATION_STRENGTH_MPA, CENSUS_HTTP_ADMISSIBILITY_REL_MARGIN,
    CENSUS_HTTP_AGE_DAYS, CENSUS_HTTP_AIR_VOID_FRACTION, CENSUS_HTTP_CONSTITUENT_PRIMARY_KG,
    CENSUS_HTTP_PREDICTED_STRENGTH_MPA, CENSUS_HTTP_TEMPERATURE_C, CENSUS_HTTP_WATER_KG,
    CENSUS_INTRINSIC_STRENGTH_MPA, CENSUS_MASS_REJECT_DENSITY_KG_M3,
    CENSUS_MASS_VIOLATION_DELTA_KG_M3, CENSUS_MIX_CALIBRATED_DENSITY_KG_M3,
    CENSUS_NEGATIVE_DISSIPATION_DENSITY_KG_M3, CENSUS_OPEN_SYSTEM_POWER_W,
    CENSUS_REACTION_EXTENT_HIGH, CENSUS_REACTION_EXTENT_INCREMENT, CENSUS_REACTION_EXTENT_LOW,
    CENSUS_REACTION_EXTENT_MID, CENSUS_REACTION_EXTENT_NEAR_COMPLETE, CENSUS_REACTION_EXTENT_OVER,
    CENSUS_REACTION_EXTENT_REGRESSION, CENSUS_STRENGTH_INTRINSIC_MPA, CENSUS_STRENGTH_MPA_MID,
    CENSUS_STRENGTH_OPEN_SYSTEM_MPA, CENSUS_STRENGTH_REGRESSION_MPA,
    MIX_CALIBRATION_REFERENCE_TEMPERATURE_K,
};

    use super::*;
    use crate::gate::core_gate::{core_gate, scalar_response_from_transition};
    use crate::gate::route::canonical_material_gate_outcome;
    use crate::gate::transition_proposal::{
        transition_outcome, ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
    };
    use crate::gate::verdict::{ConjunctVerdict, GateRejectReason};
    use umst_cartridge_concrete::evaluate_material_conjuncts;

    #[test]
    fn manifold_shim_reexports_material_wire_types() {
        let witness = MaterialTransitionWitness {
            old_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value,
            new_strength: 42.0,
            old_reaction_extent: CENSUS_REACTION_EXTENT_LOW.value,
            new_reaction_extent: CENSUS_REACTION_EXTENT_MID.value,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
    }

    #[test]
    fn material_gate_accepts_phase0b_calibrated_transition() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_LOW.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_MID.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_STRENGTH_MPA_MID.value);
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(outcome.is_strength_monotonic());
        assert!(outcome.is_reaction_extent_irreversible());
        assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
    }

    #[test]
    fn material_gate_rejects_strength_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_NEAR_COMPLETE.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_STRENGTH_INTRINSIC_MPA.value);
        let new_strength = CENSUS_STRENGTH_REGRESSION_MPA.value;
        assert!(old.strength > new_strength);
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: old.reaction_extent + 0.05,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert!(!outcome.is_strength_monotonic());
        assert!(outcome.is_reaction_extent_irreversible());
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
        );
    }

    #[test]
    fn material_gate_rejects_reaction_extent_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_HIGH.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let mut new = old;
        new.reaction_extent = CENSUS_REACTION_EXTENT_REGRESSION.value;
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: old.strength + 2.0,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert!(outcome.is_strength_monotonic());
        assert!(!outcome.is_reaction_extent_irreversible());
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::ReactionExtentRegression)
        );
    }

    #[test]
    fn material_gate_strength_regression_takes_priority_over_reaction_extent() {
        let witness = MaterialTransitionWitness {
            old_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value,
            new_strength: 30.0,
            old_reaction_extent: 0.5,
            new_reaction_extent: 0.1,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression),
            "from_material short-circuits on strength before reaction extent"
        );
        assert!(!outcome.is_strength_monotonic());
        assert!(
            outcome.is_reaction_extent_irreversible(),
            "reaction-extent witness stays true unless verdict is ReactionExtentRegression"
        );
    }

    #[test]
    fn material_gate_tolerance_accepts_at_negative_epsilon_boundary() {
        let tol = TRANSITION_TOLERANCE;
        let witness = MaterialTransitionWitness {
            old_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value,
            new_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value - tol,
            old_reaction_extent: 0.5,
            new_reaction_extent: 0.5 - tol,
        };
        let outcome = evaluate_material_conjuncts(&witness, tol);
        assert!(outcome.is_accepted());
        assert!(outcome.is_strength_monotonic());
        assert!(outcome.is_reaction_extent_irreversible());
    }

    #[test]
    fn material_gate_tolerance_rejects_below_negative_epsilon() {
        let tol = TRANSITION_TOLERANCE;
        let witness = MaterialTransitionWitness {
            old_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value,
            new_strength: CENSUS_INTRINSIC_STRENGTH_MPA.value - tol - 1e-9,
            old_reaction_extent: 0.5,
            new_reaction_extent: 0.5,
        };
        let outcome = evaluate_material_conjuncts(&witness, tol);
        assert!(!outcome.is_accepted());
        assert!(!outcome.is_strength_monotonic());
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
        );
    }

    #[test]
    fn material_strength_failure_is_not_core_failure() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_HIGH.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_OVER.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_STRENGTH_OPEN_SYSTEM_MPA.value);
        new.free_energy = old.free_energy - 50.0;

        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            0.0,
        );
        let core = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert!(
            core.is_accepted(),
            "strength regression must not fail Core gate"
        );

        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let material = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(!material.is_strength_monotonic());
        assert!(!material.is_accepted());

        let composed = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert!(
            !composed.is_accepted(),
            "composed cluster rejects strength regression"
        );
        assert!(core.is_accepted(), "Core alone still accepts");
    }

    #[test]
    fn material_reaction_failure_is_not_core_failure() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_HIGH.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let mut new = old;
        new.reaction_extent = CENSUS_REACTION_EXTENT_REGRESSION.value;
        new.free_energy = old.free_energy - 50.0;

        let response = scalar_response_from_transition(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            1.0,
            0.0,
        );
        let core = core_gate(&response, true, TRANSITION_TOLERANCE);
        assert!(
            core.is_accepted(),
            "reaction-extent regression must not fail Core gate when mass+CD hold"
        );

        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let material = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(!material.is_reaction_extent_irreversible());
        assert!(!material.is_accepted());

        let composed = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert!(!composed.is_accepted());
        assert!(core.is_accepted());
    }

    #[test]
    fn canonical_material_gate_outcome_delegates_to_consumer_ssot() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_LOW.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_MID.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_STRENGTH_MPA_MID.value);
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
    fn material_gate_idempotent_on_equilibrated_witness() {
        let state = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_HIGH.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let witness = MaterialTransitionWitness {
            old_strength: state.strength,
            new_strength: state.strength,
            old_reaction_extent: state.reaction_extent,
            new_reaction_extent: state.reaction_extent,
        };
        let first = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        let second = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert_eq!(
            first, second,
            "evaluate_material_conjuncts must not drift on re-application"
        );
        assert!(first.is_accepted());
    }

    #[test]
    fn material_gate_idempotent_on_admissible_transition() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_LOW.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let mut new = old;
        new.reaction_extent = CENSUS_REACTION_EXTENT_MID.value;
        new.strength = 42.0;
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let first = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        let second = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert_eq!(first, second);
        assert!(first.is_accepted());
    }

    #[test]
    fn w8e14_material_gate_strength_regression_rejects() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(CENSUS_BINDER_LIQUID_RATIO.value, CENSUS_REACTION_EXTENT_HIGH.value, MIX_CALIBRATION_REFERENCE_TEMPERATURE_K, CENSUS_INTRINSIC_STRENGTH_MPA.value);
        let mut new = old;
        new.strength = old.strength - 1.0;
        let witness = MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        };
        let outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
    }
}

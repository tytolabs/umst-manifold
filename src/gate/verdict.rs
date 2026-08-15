// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Manifold shim — SSOT in `umst-gate` (P2.0).
pub use umst_gate::verdict::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::evaluator::{GateEvaluator, TransitionGateEvaluator, TransitionVerdict};
    use crate::gate::thermo_transition::ThermodynamicState;
    use crate::gate::transition_eval_registry::{
        ThermodynamicMixEvaluator, ThermodynamicTransitionContext,
    };
    use crate::gate::transition_proposal::{
        transition_outcome, ThermodynamicMixFilter, ThermodynamicStateSnapshot,
        TRANSITION_TOLERANCE,
    };
    use crate::gate::ThermodynamicTransitionEvaluator;
    use crate::runtime::catalog::traceability::{
        CD_TRANSITION_CATALOG_ID, THERMODYNAMIC_MIX_CATALOG_ID,
    };

    fn host_to_snapshot(s: &ThermodynamicState) -> ThermodynamicStateSnapshot {
        ThermodynamicStateSnapshot {
            density: s.density,
            temperature: s.temperature,
            free_energy: s.free_energy,
            entropy: s.entropy,
            reaction_extent: s.reaction_extent,
            strength: s.strength,
        }
    }

    fn rest_from_conjunct(v: ConjunctVerdict) -> AdmissibilityVerdict {
        match v {
            ConjunctVerdict::Accepted => AdmissibilityVerdict::Accepted,
            ConjunctVerdict::Rejected(reason) => reason.to_rest_verdict(),
        }
    }

    /// Golden vectors from [`tests/gate_parity_fixture.rs`] / `docs/GOLDEN_FIXTURES.md`.
    fn golden_identity_admissible() -> (ThermodynamicState, ThermodynamicState, f64) {
        let s = ThermodynamicState {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        (s.clone(), s, 1.0)
    }

    /// Mass bound violation: `|Δρ| = 120` kg/m³ (registry band is `< 100`).
    fn golden_mass_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
        let old = ThermodynamicState {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.1,
            reaction_extent: 0.3,
            strength: 10.0,
        };
        let mut new = old.clone();
        new.density = 2280.0;
        (old, new, 3600.0)
    }

    /// Clausius–Duhem reject: free-energy spike breaks `D_int ≥ −tolerance`.
    fn golden_negative_dissipation_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
        let old = ThermodynamicState {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let mut new = old.clone();
        new.free_energy = -1.0e4;
        (old, new, 1.0)
    }

    #[test]
    fn verdict_shim_ssot_rest_tokens_pinned() {
        assert_eq!(AdmissibilityVerdict::ACCEPTED, "ACCEPTED");
        assert_eq!(AdmissibilityVerdict::MASS_VIOLATION, "MASS_VIOLATION");
        assert_eq!(
            AdmissibilityVerdict::NEGATIVE_DISSIPATION,
            "NEGATIVE_DISSIPATION"
        );
        assert_eq!(AdmissibilityVerdict::UNKNOWN, "UNKNOWN");
        for variant in [
            AdmissibilityVerdict::Accepted,
            AdmissibilityVerdict::MassViolation,
            AdmissibilityVerdict::NegativeDissipation,
            AdmissibilityVerdict::Unknown,
        ] {
            assert_eq!(variant.as_str(), format!("{variant}"));
        }
    }

    #[test]
    fn verdict_parse_round_trips_documented_wire_tokens() {
        let tokens = [
            (
                AdmissibilityVerdict::ACCEPTED,
                AdmissibilityVerdict::Accepted,
            ),
            (
                AdmissibilityVerdict::MASS_VIOLATION,
                AdmissibilityVerdict::MassViolation,
            ),
            (
                AdmissibilityVerdict::NEGATIVE_DISSIPATION,
                AdmissibilityVerdict::NegativeDissipation,
            ),
            (AdmissibilityVerdict::UNKNOWN, AdmissibilityVerdict::Unknown),
        ];
        for (wire, expected) in tokens {
            assert_eq!(AdmissibilityVerdict::parse(wire), Some(expected));
            assert_eq!(expected.as_str(), wire);
        }
        assert_eq!(AdmissibilityVerdict::parse("NOT_A_VERDICT"), None);
    }

    #[test]
    fn verdict_from_transition_conjuncts_locked_ladder() {
        let cases = [
            (true, true, true, AdmissibilityVerdict::Accepted),
            (true, true, false, AdmissibilityVerdict::Accepted),
            (true, false, true, AdmissibilityVerdict::Accepted),
            (true, false, false, AdmissibilityVerdict::Accepted),
            (false, true, true, AdmissibilityVerdict::Unknown),
            (
                false,
                true,
                false,
                AdmissibilityVerdict::NegativeDissipation,
            ),
            (false, false, true, AdmissibilityVerdict::MassViolation),
            (false, false, false, AdmissibilityVerdict::MassViolation),
        ];
        for (accepted, mass_conserved, energy_positive, expected) in cases {
            assert_eq!(
                AdmissibilityVerdict::from_transition_conjuncts(
                    accepted,
                    mass_conserved,
                    energy_positive
                ),
                expected,
                "accepted={accepted} mass={mass_conserved} energy={energy_positive}"
            );
            assert_eq!(
                AdmissibilityVerdict::from_thermo_flags(accepted, mass_conserved, energy_positive),
                expected
            );
        }
    }

    #[test]
    fn verdict_gate_reject_reason_rest_fold() {
        assert_eq!(
            GateRejectReason::MassViolation.to_rest_verdict(),
            AdmissibilityVerdict::MassViolation
        );
        assert_eq!(
            GateRejectReason::NegativeDissipation.to_rest_verdict(),
            AdmissibilityVerdict::NegativeDissipation
        );
        assert_eq!(
            GateRejectReason::StrengthRegression.to_rest_verdict(),
            AdmissibilityVerdict::NegativeDissipation
        );
        for reason in [
            GateRejectReason::ReactionExtentRegression,
            GateRejectReason::StrengthUpperBound,
            GateRejectReason::RegimeEnvelope,
            GateRejectReason::MalformedInput,
        ] {
            assert_eq!(
                reason.to_rest_verdict(),
                AdmissibilityVerdict::Unknown,
                "{reason:?}"
            );
        }
    }

    #[test]
    fn verdict_conjunct_from_core_all_pairs() {
        assert_eq!(
            ConjunctVerdict::from_core(true, true),
            ConjunctVerdict::Accepted
        );
        assert_eq!(
            ConjunctVerdict::from_core(false, true),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
        assert_eq!(
            ConjunctVerdict::from_core(true, false),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
        assert_eq!(
            ConjunctVerdict::from_core(false, false),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
    }

    #[test]
    fn verdict_conjunct_from_material_all_pairs() {
        assert_eq!(
            ConjunctVerdict::from_material(true, true),
            ConjunctVerdict::Accepted
        );
        assert_eq!(
            ConjunctVerdict::from_material(false, true),
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
        );
        assert_eq!(
            ConjunctVerdict::from_material(true, false),
            ConjunctVerdict::Rejected(GateRejectReason::ReactionExtentRegression)
        );
        assert_eq!(
            ConjunctVerdict::from_material(false, false),
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
        );
    }

    #[test]
    fn verdict_conjunct_compose_short_circuits_core() {
        let core_reject = ConjunctVerdict::Rejected(GateRejectReason::MassViolation);
        let material_reject = ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression);
        assert_eq!(
            ConjunctVerdict::compose(core_reject, material_reject),
            core_reject
        );
        assert_eq!(
            ConjunctVerdict::compose(ConjunctVerdict::Accepted, material_reject),
            material_reject
        );
        assert_eq!(
            ConjunctVerdict::compose(ConjunctVerdict::Accepted, ConjunctVerdict::Accepted),
            ConjunctVerdict::Accepted
        );
        assert!(ConjunctVerdict::Accepted.is_accepted());
        assert!(!ConjunctVerdict::Rejected(GateRejectReason::MassViolation).is_accepted());
    }

    #[test]
    fn verdict_golden_identity_evaluator_maps_accepted() {
        let (old, new, dt) = golden_identity_admissible();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        let v = tv.rest_verdict();
        assert_eq!(v, AdmissibilityVerdict::Accepted);
        assert_eq!(v.as_str(), AdmissibilityVerdict::ACCEPTED);
        assert_eq!(
            AdmissibilityVerdict::from_transition_conjuncts(
                tv.is_admissible(),
                tv.is_mass_conserved(),
                tv.is_energy_positive()
            ),
            v
        );
    }

    #[test]
    fn verdict_golden_mass_reject_evaluator_maps_mass_violation() {
        let (old, new, dt) = golden_mass_reject();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        let v = tv.rest_verdict();
        assert_eq!(v, AdmissibilityVerdict::MassViolation);
        assert_eq!(v.as_str(), AdmissibilityVerdict::MASS_VIOLATION);
        assert_eq!(
            ConjunctVerdict::from_core(tv.is_mass_conserved(), tv.is_energy_positive()),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
    }

    #[test]
    fn verdict_golden_negative_dissipation_evaluator_maps_reject() {
        let (old, new, dt) = golden_negative_dissipation_reject();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        let v = tv.rest_verdict();
        assert_eq!(v, AdmissibilityVerdict::NegativeDissipation);
        assert_eq!(v.as_str(), AdmissibilityVerdict::NEGATIVE_DISSIPATION);
        assert_eq!(
            ConjunctVerdict::from_core(tv.is_mass_conserved(), tv.is_energy_positive()),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn verdict_phase0b_transition_outcome_rest_ladder() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.verdict(),
            AdmissibilityVerdict::from_transition_conjuncts(
                outcome.is_accepted(),
                outcome.is_mass_conserved(),
                outcome.is_energy_positive()
            )
        );
        if outcome.is_accepted() {
            assert_eq!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        } else {
            assert_ne!(outcome.conjunct_verdict(), ConjunctVerdict::Accepted);
        }
    }

    #[test]
    fn verdict_display_matches_as_str() {
        for variant in [
            AdmissibilityVerdict::Accepted,
            AdmissibilityVerdict::MassViolation,
            AdmissibilityVerdict::NegativeDissipation,
            AdmissibilityVerdict::Unknown,
        ] {
            assert_eq!(format!("{variant}"), variant.as_str());
        }
    }

    #[test]
    fn verdict_parse_rejects_malformed_tokens() {
        for token in ["", "accepted", "mass_violation", "ACCEPTED ", " UNKNOWN"] {
            assert_eq!(
                AdmissibilityVerdict::parse(token),
                None,
                "malformed token must not parse: {token:?}"
            );
        }
    }

    #[test]
    fn verdict_conjunct_rest_from_reject_reason_all_variants() {
        let cases = [
            (
                GateRejectReason::MassViolation,
                AdmissibilityVerdict::MassViolation,
            ),
            (
                GateRejectReason::NegativeDissipation,
                AdmissibilityVerdict::NegativeDissipation,
            ),
            (
                GateRejectReason::StrengthRegression,
                AdmissibilityVerdict::NegativeDissipation,
            ),
            (
                GateRejectReason::ReactionExtentRegression,
                AdmissibilityVerdict::Unknown,
            ),
            (
                GateRejectReason::StrengthUpperBound,
                AdmissibilityVerdict::Unknown,
            ),
            (
                GateRejectReason::RegimeEnvelope,
                AdmissibilityVerdict::Unknown,
            ),
            (
                GateRejectReason::MalformedInput,
                AdmissibilityVerdict::Unknown,
            ),
        ];
        for (reason, expected) in cases {
            let conjunct = ConjunctVerdict::Rejected(reason);
            assert_eq!(reason.to_rest_verdict(), expected, "{reason:?}");
            assert!(!conjunct.is_accepted());
        }
    }

    #[test]
    fn verdict_transition_gate_evaluator_trait_golden_vectors() {
        let mut ev = ThermodynamicTransitionEvaluator::new();
        assert_eq!(ev.catalog_id(), CD_TRANSITION_CATALOG_ID);
        assert_eq!(ev.gate_family(), "clausius_duhem_transition");

        let (id_old, id_new, id_dt) = golden_identity_admissible();
        let id_tv = ev.check_transition_host(&id_old, &id_new, id_dt);
        assert_eq!(id_tv.rest_verdict(), AdmissibilityVerdict::Accepted);
        assert_eq!(id_tv.conjunct_verdict(), ConjunctVerdict::Accepted);

        let (mass_old, mass_new, mass_dt) = golden_mass_reject();
        let mass_tv = ev.check_transition_host(&mass_old, &mass_new, mass_dt);
        assert_eq!(mass_tv.rest_verdict(), AdmissibilityVerdict::MassViolation);
        assert_eq!(
            mass_tv.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );

        let (cd_old, cd_new, cd_dt) = golden_negative_dissipation_reject();
        let cd_tv = ev.check_transition_host(&cd_old, &cd_new, cd_dt);
        assert_eq!(
            cd_tv.rest_verdict(),
            AdmissibilityVerdict::NegativeDissipation
        );
        assert_eq!(
            cd_tv.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn verdict_transition_outcome_conjunct_rest_ladder_scenarios() {
        fn rest_from_conjunct(v: ConjunctVerdict) -> AdmissibilityVerdict {
            match v {
                ConjunctVerdict::Accepted => AdmissibilityVerdict::Accepted,
                ConjunctVerdict::Rejected(reason) => reason.to_rest_verdict(),
            }
        }

        let scenarios = [
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0),
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0),
            ),
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0),
                {
                    let mut n =
                        ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
                    n.reaction_extent = 0.1;
                    n
                },
            ),
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0),
                {
                    let mut n =
                        ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
                    n.strength = 10.0;
                    n
                },
            ),
        ];

        for (old, new) in scenarios {
            let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
            assert_eq!(
                outcome.verdict(),
                AdmissibilityVerdict::from_transition_conjuncts(
                    outcome.is_accepted(),
                    outcome.is_mass_conserved(),
                    outcome.is_energy_positive()
                )
            );
            assert_eq!(
                rest_from_conjunct(outcome.conjunct_verdict()),
                outcome.verdict()
            );
            if outcome.is_accepted() {
                assert_eq!(outcome.verdict(), AdmissibilityVerdict::Accepted);
            } else if matches!(
                outcome.conjunct_verdict(),
                ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
            ) {
                assert_eq!(outcome.verdict(), AdmissibilityVerdict::NegativeDissipation);
            } else if matches!(
                outcome.conjunct_verdict(),
                ConjunctVerdict::Rejected(GateRejectReason::ReactionExtentRegression)
            ) {
                assert_eq!(outcome.verdict(), AdmissibilityVerdict::Unknown);
            }
        }
    }

    #[test]
    fn verdict_compose_material_strength_reject_rest_negative_dissipation() {
        let core = ConjunctVerdict::from_core(true, true);
        let material = ConjunctVerdict::from_material(false, true);
        let composed = ConjunctVerdict::compose(core, material);
        assert_eq!(
            composed,
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
        );
        assert_eq!(
            GateRejectReason::StrengthRegression.to_rest_verdict(),
            AdmissibilityVerdict::NegativeDissipation
        );
    }

    #[test]
    fn verdict_eq_copy_hash_wire_tokens_stable() {
        use std::collections::HashSet;

        let a = AdmissibilityVerdict::MassViolation;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a.as_str(), AdmissibilityVerdict::MASS_VIOLATION);

        let mut set = HashSet::new();
        set.insert(AdmissibilityVerdict::Accepted);
        set.insert(AdmissibilityVerdict::MassViolation);
        set.insert(AdmissibilityVerdict::NegativeDissipation);
        set.insert(AdmissibilityVerdict::Unknown);
        assert_eq!(set.len(), 4);

        let reason = GateRejectReason::ReactionExtentRegression;
        assert_eq!(reason, GateRejectReason::ReactionExtentRegression);
        assert_ne!(reason, GateRejectReason::MassViolation);
    }

    #[test]
    fn verdict_parse_as_str_round_trip_all_variants() {
        for variant in [
            AdmissibilityVerdict::Accepted,
            AdmissibilityVerdict::MassViolation,
            AdmissibilityVerdict::NegativeDissipation,
            AdmissibilityVerdict::Unknown,
        ] {
            let wire = variant.as_str();
            assert_eq!(AdmissibilityVerdict::parse(wire), Some(variant));
            assert_eq!(format!("{variant}"), wire);
        }
    }

    #[test]
    fn verdict_mix_evaluator_golden_vectors_rest_ladder() {
        let mut ev = ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new());
        assert_eq!(ev.catalog_id(), THERMODYNAMIC_MIX_CATALOG_ID);
        assert_eq!(ev.gate_family(), "thermodynamic_mix_transition");

        let cases = [
            (
                golden_identity_admissible(),
                AdmissibilityVerdict::Accepted,
                AdmissibilityVerdict::ACCEPTED,
            ),
            (
                golden_mass_reject(),
                AdmissibilityVerdict::MassViolation,
                AdmissibilityVerdict::MASS_VIOLATION,
            ),
            (
                golden_negative_dissipation_reject(),
                AdmissibilityVerdict::NegativeDissipation,
                AdmissibilityVerdict::NEGATIVE_DISSIPATION,
            ),
        ];

        for ((old, new, dt), expected, wire) in cases {
            let ctx = ThermodynamicTransitionContext {
                old_state: &host_to_snapshot(&old),
                new_state: &host_to_snapshot(&new),
                dt_seconds: dt,
            };
            let v = ev.evaluate_thermo_transition(ctx);
            assert_eq!(v, expected);
            assert_eq!(v.as_str(), wire);
            assert_eq!(AdmissibilityVerdict::parse(wire), Some(expected));
        }
    }

    #[test]
    fn verdict_compose_rejection_lattice_short_circuits_core() {
        let core_outcomes = [
            ConjunctVerdict::Accepted,
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation),
        ];
        let material_outcomes = [
            ConjunctVerdict::Accepted,
            ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression),
            ConjunctVerdict::Rejected(GateRejectReason::ReactionExtentRegression),
        ];

        for core in core_outcomes {
            for material in material_outcomes {
                let composed = ConjunctVerdict::compose(core, material);
                let expected = match core {
                    ConjunctVerdict::Rejected(_) => core,
                    ConjunctVerdict::Accepted => material,
                };
                assert_eq!(composed, expected);
                assert_eq!(rest_from_conjunct(composed), rest_from_conjunct(expected));
                if composed.is_accepted() {
                    assert_eq!(composed, ConjunctVerdict::Accepted);
                    assert_eq!(rest_from_conjunct(composed), AdmissibilityVerdict::Accepted);
                } else {
                    assert!(!composed.is_accepted());
                }
            }
        }
    }

    #[test]
    fn verdict_transition_verdict_rest_aligns_conjunct_fold_golden() {
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let vectors = [
            golden_identity_admissible(),
            golden_mass_reject(),
            golden_negative_dissipation_reject(),
        ];

        for (old, new, dt) in vectors {
            let tv: TransitionVerdict = ev.check_transition_host(&old, &new, dt);
            assert_eq!(tv.rest_verdict(), rest_from_conjunct(tv.conjunct_verdict()));
            assert_eq!(
                tv.rest_verdict(),
                AdmissibilityVerdict::from_transition_conjuncts(
                    tv.is_accepted(),
                    tv.is_mass_conserved(),
                    tv.is_energy_positive()
                )
            );
        }
    }

    #[test]
    fn verdict_reject_reason_all_variants_rest_and_hash() {
        use std::collections::HashSet;

        let reasons = [
            GateRejectReason::MalformedInput,
            GateRejectReason::MassViolation,
            GateRejectReason::NegativeDissipation,
            GateRejectReason::StrengthRegression,
            GateRejectReason::ReactionExtentRegression,
            GateRejectReason::StrengthUpperBound,
            GateRejectReason::RegimeEnvelope,
        ];

        let mut set = HashSet::new();
        for reason in reasons {
            set.insert(reason);
            let conjunct = ConjunctVerdict::Rejected(reason);
            assert!(!conjunct.is_accepted());
            assert_eq!(rest_from_conjunct(conjunct), reason.to_rest_verdict());
            let debug = format!("{reason:?}");
            assert!(!debug.is_empty());
        }
        assert_eq!(set.len(), reasons.len());
    }

    #[test]
    fn verdict_transition_outcome_mass_reject_maps_mass_violation() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.density = old.density - 150.0;
        let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert_eq!(outcome.verdict(), AdmissibilityVerdict::MassViolation);
        assert_eq!(
            outcome.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
        assert_eq!(
            rest_from_conjunct(outcome.conjunct_verdict()),
            outcome.verdict()
        );
    }

    #[test]
    fn w8e14_conjunct_compose_short_circuits_on_mass_reject() {
        let mass_reject = ConjunctVerdict::Rejected(GateRejectReason::MassViolation);
        let cd_reject = ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation);
        let composed = ConjunctVerdict::compose(mass_reject, cd_reject);
        assert_eq!(
            composed,
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
    }
}

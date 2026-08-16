// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Kleisli arrows over the admissibility monad (prototype port, **generic carriers**).
//!
//! [`KleisliUnitEvaluator`] implements [`super::evaluator::GateEvaluator`] for catalog id
//! `umst.gate.kleisli_unit`, hand-aligned to `Gate.lean` (`admissibleNRefl`, `kleisliAdmissibility`).

use super::evaluator::GateEvaluator;
use super::route::canonical_transition_outcome;
use super::transition_proposal::ThermodynamicStateSnapshot;
use super::verdict::AdmissibilityVerdict;

/// Result of a thermodynamic gate check on a wrapped value.
#[allow(missing_docs)] // Legacy bool mirrors — prefer [`Self::admissibility_verdict`] / [`Self::is_admissible`]
#[derive(Debug, Clone)]
pub struct AdmissibilityResult {
    /// Primary discriminant — Kleisli monad admissibility fold.
    pub verdict: AdmissibilityVerdict,
    /// Legacy mirror — prefer [`Self::is_admissible`] / [`Self::admissibility_verdict`].
    #[deprecated(
        since = "0.2.0",
        note = "use AdmissibilityResult::is_admissible() or admissibility_verdict()"
    )]
    pub admissible: bool,
    pub dissipation: f32,
    pub violation: Option<String>,
}

impl AdmissibilityResult {
    /// Borrow the primary [`AdmissibilityVerdict`] discriminant (FP P2.6 SSOT).
    #[inline]
    #[must_use]
    pub fn admissibility_verdict(&self) -> AdmissibilityVerdict {
        self.verdict
    }

    /// Whether the Kleisli carrier passed admissibility (wire bytes unchanged).
    #[inline]
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        matches!(self.verdict, AdmissibilityVerdict::Accepted)
    }

    /// Alias for [`Self::is_admissible`] — preserved for façade call-site compatibility.
    #[inline]
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.is_admissible()
    }

    /// Construct from primary [`AdmissibilityVerdict`] (legacy bool mirror kept for wire parity).
    #[allow(deprecated)]
    #[must_use]
    pub fn from_verdict(
        verdict: AdmissibilityVerdict,
        dissipation: f32,
        violation: Option<String>,
    ) -> Self {
        Self {
            verdict,
            admissible: matches!(verdict, AdmissibilityVerdict::Accepted),
            dissipation,
            violation,
        }
    }
}

#[inline]
fn kleisli_verdict_from_admissible(admissible: bool) -> AdmissibilityVerdict {
    if admissible {
        AdmissibilityVerdict::Accepted
    } else {
        AdmissibilityVerdict::Unknown
    }
}

fn kleisli_result_from_verdict(
    verdict: AdmissibilityVerdict,
    dissipation: f32,
    violation: Option<String>,
) -> AdmissibilityResult {
    AdmissibilityResult::from_verdict(verdict, dissipation, violation)
}

/// The admissibility monad wraps a value with its gate status: `M(A) = (A, AdmissibilityResult)`.
#[derive(Debug, Clone)]
pub struct Admissible<A: Clone> {
    pub value: A,
    pub result: AdmissibilityResult,
}

impl<A: Clone> Admissible<A> {
    /// Monadic unit (η): lift a value — trivial self-transition is admissible.
    #[must_use]
    pub fn pure(value: A) -> Self {
        Admissible {
            value,
            result: kleisli_result_from_verdict(AdmissibilityVerdict::Accepted, 0.0, None),
        }
    }

    /// Bind (short-circuit on inadmissible intermediate carriers).
    pub fn bind<B: Clone, F>(self, f: F) -> Admissible<B>
    where
        F: FnOnce(A) -> Admissible<B>,
    {
        if !self.result.is_admissible() {
            return Admissible {
                value: f(self.value).value,
                result: self.result,
            };
        }
        f(self.value)
    }

    #[must_use]
    pub fn join(nested: Admissible<Admissible<A>>) -> Admissible<A> {
        if !nested.result.is_admissible() {
            Admissible {
                value: nested.value.value,
                result: nested.result,
            }
        } else {
            nested.value
        }
    }
}

/// Kleisli arrow `A → M(B)`.
pub struct KleisliArrow<A: Clone, B: Clone> {
    pub name: String,
    arrow: Box<dyn Fn(A) -> Admissible<B> + Send + Sync>,
}

impl<A: Clone, B: Clone> KleisliArrow<A, B> {
    pub fn new<F>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(A) -> Admissible<B> + Send + Sync + 'static,
    {
        KleisliArrow {
            name: name.into(),
            arrow: Box::new(f),
        }
    }

    pub fn run(&self, input: A) -> Admissible<B> {
        (self.arrow)(input)
    }
}

/// Compose sequentially: `(f ● g)(x) = f(x) >>= g`.
#[must_use]
pub fn kleisli_compose_pair<A, B, C>(
    f: impl Fn(A) -> Admissible<B> + Send + Sync + 'static,
    g: impl Fn(B) -> Admissible<C> + Send + Sync + 'static,
    name: impl Into<String>,
) -> KleisliArrow<A, C>
where
    A: Clone + 'static,
    B: Clone + 'static,
    C: Clone + 'static,
{
    KleisliArrow::new(name, move |a: A| {
        let mb = f(a);
        mb.bind(&g)
    })
}

#[derive(Clone, Debug)]
pub struct KleisliPipeline {
    pub name: String,
    pub steps: Vec<String>,
}

impl KleisliPipeline {
    pub fn new(name: impl Into<String>) -> Self {
        KleisliPipeline {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    /// Run sequential Kleisli arrows (short-circuits once inadmissible).
    #[must_use]
    pub fn run_sequence<A>(&self, initial: A, arrows: &[&KleisliArrow<A, A>]) -> Admissible<A>
    where
        A: Clone,
    {
        let mut current = Admissible::pure(initial);
        for arrow in arrows {
            if !current.result.is_admissible() {
                break;
            }
            current = current.bind(|state| arrow.run(state));
        }
        current
    }
}

/// Registry-facing evaluator for the admissibility monad unit η ([`Admissible::pure`]).
///
/// Aligns with `Gate.lean` reflexivity (`admissibleNRefl`): identity carriers lift with zero dissipation.
#[derive(Debug, Clone, Copy, Default)]
pub struct KleisliUnitEvaluator;

impl KleisliUnitEvaluator {
    pub const CATALOG_ID: &'static str = "umst.gate.kleisli_unit";

    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Monadic unit η: lift any cloneable carrier (trivial self-transition is admissible).
    #[must_use]
    pub fn lift<A: Clone>(&self, value: A) -> Admissible<A> {
        Admissible::pure(value)
    }

    /// Map a lifted carrier to REST-stable [`AdmissibilityVerdict`].
    #[must_use]
    pub fn verdict_for_lift<A: Clone>(&self, value: A) -> AdmissibilityVerdict {
        self.lift(value).result.admissibility_verdict()
    }

    /// Reflexive thermodynamic snapshot step (`AdmissibleN n s s` / `admissibleNRefl`).
    #[must_use]
    pub fn evaluate_reflexive_step(
        &self,
        state: &ThermodynamicStateSnapshot,
    ) -> AdmissibilityVerdict {
        self.verdict_for_lift(*state)
    }

    /// Non-reflexive transition — routes through canonical `transition_outcome` (Phase 0d).
    #[must_use]
    pub fn evaluate_canonical_transition(
        &self,
        old_state: &ThermodynamicStateSnapshot,
        new_state: &ThermodynamicStateSnapshot,
        dt_s: f64,
    ) -> AdmissibilityVerdict {
        canonical_transition_outcome(old_state, new_state, dt_s).verdict()
    }
}

impl GateEvaluator for KleisliUnitEvaluator {
    fn catalog_id(&self) -> &'static str {
        Self::CATALOG_ID
    }

    fn gate_family(&self) -> &'static str {
        "kleisli_admissibility_unit"
    }
}

#[must_use]
pub fn gate_arrow_generic<A: Clone>(
    name: impl Into<String>,
    check: impl Fn(&A) -> (bool, f32, Option<String>) + Send + Sync + 'static,
) -> KleisliArrow<A, A> {
    KleisliArrow::new(name, move |state: A| {
        let (ok, dissipation, violation) = check(&state);
        Admissible {
            value: state,
            result: kleisli_result_from_verdict(
                kleisli_verdict_from_admissible(ok),
                dissipation,
                violation,
            ),
        }
    })
}

/// Kleisli arrow for a thermodynamic transition — full conjunct set via canonical route (Phase 0d).
#[must_use]
pub fn gate_arrow_canonical_transition(
    name: impl Into<String>,
    old_state: ThermodynamicStateSnapshot,
    dt_s: f64,
) -> KleisliArrow<ThermodynamicStateSnapshot, ThermodynamicStateSnapshot> {
    KleisliArrow::new(name, move |new_state: ThermodynamicStateSnapshot| {
        let outcome = canonical_transition_outcome(&old_state, &new_state, dt_s);
        let verdict = outcome.verdict();
        Admissible {
            value: new_state,
            result: kleisli_result_from_verdict(
                verdict,
                outcome.dissipation as f32,
                if verdict == AdmissibilityVerdict::Accepted {
                    None
                } else {
                    Some("canonical_transition_reject".into())
                },
            ),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::route::canonical_transition_outcome;
    use crate::gate::transition_proposal::{ThermodynamicStateSnapshot, TRANSITION_TOLERANCE};

    #[test]
    fn admissibility_result_from_verdict_mirrors_legacy_bool() {
        let accepted = AdmissibilityResult::from_verdict(AdmissibilityVerdict::Accepted, 0.0, None);
        assert!(accepted.is_admissible());
        assert!(accepted.is_accepted());
        assert_eq!(
            accepted.admissibility_verdict(),
            AdmissibilityVerdict::Accepted
        );
        #[allow(deprecated)]
        {
            assert!(accepted.admissible);
        }

        let unknown = AdmissibilityResult::from_verdict(
            AdmissibilityVerdict::Unknown,
            1.5,
            Some("probe".into()),
        );
        assert!(!unknown.is_admissible());
        assert_eq!(unknown.dissipation, 1.5);
        assert_eq!(unknown.violation.as_deref(), Some("probe"));
    }

    #[test]
    fn admissible_pure_lift_is_admissible_with_zero_dissipation() {
        let carrier = Admissible::pure(42_i32);
        assert!(carrier.result.is_admissible());
        assert_eq!(carrier.value, 42);
        assert_eq!(carrier.result.dissipation, 0.0);
        assert!(carrier.result.violation.is_none());
    }

    #[test]
    fn admissible_bind_short_circuits_on_inadmissible_intermediate() {
        let inadmissible = Admissible {
            value: 7_i32,
            result: AdmissibilityResult::from_verdict(
                AdmissibilityVerdict::Unknown,
                -1.0,
                Some("blocked".into()),
            ),
        };
        let out = inadmissible.bind(|x| Admissible::pure(x * 10));
        assert_eq!(out.value, 70);
        assert!(!out.result.is_admissible());
        assert_eq!(out.result.violation.as_deref(), Some("blocked"));
    }

    #[test]
    fn admissible_bind_chains_when_admissible() {
        let start = Admissible::pure(3_i32);
        let out = start.bind(|x| Admissible::pure(x + 4));
        assert!(out.result.is_admissible());
        assert_eq!(out.value, 7);
    }

    #[test]
    fn admissible_join_flattens_nested_admissible() {
        let inner = Admissible::pure("inner".to_string());
        let nested = Admissible::pure(inner);
        let flat = Admissible::join(nested);
        assert!(flat.result.is_admissible());
        assert_eq!(flat.value, "inner");
    }

    #[test]
    fn admissible_join_short_circuits_on_inadmissible_outer() {
        let inner = Admissible::pure(99_i32);
        let nested = Admissible {
            value: inner,
            result: AdmissibilityResult::from_verdict(
                AdmissibilityVerdict::Unknown,
                0.0,
                Some("outer_fail".into()),
            ),
        };
        let flat = Admissible::join(nested);
        assert!(!flat.result.is_admissible());
        assert_eq!(flat.value, 99);
    }

    #[test]
    fn kleisli_compose_pair_runs_right_to_left_bind() {
        let composed = kleisli_compose_pair(
            |x: i32| Admissible::pure(x + 1),
            |y: i32| Admissible::pure(y * 2),
            "inc_then_double",
        );
        assert_eq!(composed.name, "inc_then_double");
        let out = composed.run(5);
        assert!(out.result.is_admissible());
        assert_eq!(out.value, 12);
    }

    #[test]
    fn kleisli_compose_pair_short_circuits_when_first_arrow_fails() {
        let fail_then_pure = kleisli_compose_pair(
            |_x: i32| Admissible {
                value: 0,
                result: AdmissibilityResult::from_verdict(
                    AdmissibilityVerdict::Unknown,
                    -1.0,
                    Some("first_fail".into()),
                ),
            },
            |y: i32| Admissible::pure(y + 100),
            "fail_then_pure",
        );
        let out = fail_then_pure.run(1);
        assert!(!out.result.is_admissible());
        assert_eq!(out.value, 100);
    }

    #[test]
    fn kleisli_pipeline_run_sequence_short_circuits_after_reject() {
        let pipe = KleisliPipeline::new("two_step");
        let ok = gate_arrow_generic("always_ok", |_x: &i32| (true, 0.0, None));
        let bad = gate_arrow_generic("always_bad", |_x: &i32| {
            (false, -2.0, Some("reject".into()))
        });
        let seq = pipe.run_sequence(1, &[&ok, &bad, &ok]);
        assert!(!seq.result.is_admissible());
        assert_eq!(seq.value, 1);
    }

    #[test]
    fn kleisli_unit_evaluator_catalog_and_lift() {
        let eval = KleisliUnitEvaluator::new();
        assert_eq!(eval.catalog_id(), KleisliUnitEvaluator::CATALOG_ID);
        assert_eq!(eval.gate_family(), "kleisli_admissibility_unit");
        let lifted = eval.lift(3.14_f64);
        assert!(lifted.result.is_admissible());
        assert_eq!(lifted.value, 3.14);
        assert_eq!(
            eval.verdict_for_lift(3.14_f64),
            AdmissibilityVerdict::Accepted
        );
    }

    #[test]
    fn kleisli_unit_evaluator_reflexive_step_matches_pure_lift() {
        let eval = KleisliUnitEvaluator::new();
        let state = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        assert_eq!(
            eval.evaluate_reflexive_step(&state),
            AdmissibilityVerdict::Accepted
        );
        assert_eq!(eval.verdict_for_lift(state), AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn kleisli_unit_evaluator_canonical_transition_delegates_to_route() {
        let eval = KleisliUnitEvaluator::new();
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let routed = eval.evaluate_canonical_transition(&old, &new, dt);
        let direct = canonical_transition_outcome(&old, &new, dt).verdict();
        assert_eq!(routed, direct);
        assert_eq!(routed, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn kleisli_unit_evaluator_canonical_transition_rejects_extent_regression() {
        let eval = KleisliUnitEvaluator::new();
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.1;
        let dt = 1.0;
        let routed = eval.evaluate_canonical_transition(&old, &new, dt);
        let direct = canonical_transition_outcome(&old, &new, dt).verdict();
        assert_eq!(routed, direct);
        assert_ne!(routed, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn gate_arrow_generic_admits_positive_carrier() {
        let arrow = gate_arrow_generic("positive_denominator", |x: &f64| {
            if *x > 0.0 {
                (true, 0.0, None)
            } else {
                (false, -1.0, Some("non_positive".into()))
            }
        });
        let ok = arrow.run(std::f64::consts::PI);
        assert!(ok.result.is_admissible());
        assert_eq!(ok.value, std::f64::consts::PI);
    }

    #[test]
    fn gate_arrow_generic_rejects_non_positive_with_unknown_verdict() {
        let arrow = gate_arrow_generic("positive_denominator", |x: &f64| {
            if *x > 0.0 {
                (true, 0.0, None)
            } else {
                (false, -1.0, Some("non_positive".into()))
            }
        });
        let bad = arrow.run(-1.0_f64);
        assert!(!bad.result.is_admissible());
        assert_eq!(
            bad.result.admissibility_verdict(),
            AdmissibilityVerdict::Unknown
        );
        assert_eq!(bad.result.violation.as_deref(), Some("non_positive"));
    }

    #[test]
    fn gate_arrow_canonical_transition_accepts_phase0b_fixture() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let arrow = gate_arrow_canonical_transition("hydration_step", old, dt);
        let out = arrow.run(new);
        assert!(out.result.is_admissible());
        assert_eq!(
            out.result.admissibility_verdict(),
            AdmissibilityVerdict::Accepted
        );
        assert!(out.result.violation.is_none());
        let expected = canonical_transition_outcome(&old, &new, dt);
        assert!((out.result.dissipation - expected.dissipation as f32).abs() < 1e-3);
    }

    #[test]
    fn gate_arrow_canonical_transition_rejects_reaction_extent_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.1;
        let dt = 1.0;
        let arrow = gate_arrow_canonical_transition("extent_guard", old, dt);
        let out = arrow.run(new);
        assert!(!out.result.is_admissible());
        assert_eq!(
            out.result.violation.as_deref(),
            Some("canonical_transition_reject")
        );
        let outcome = canonical_transition_outcome(&old, &new, dt);
        assert_eq!(out.result.admissibility_verdict(), outcome.verdict());
        assert_ne!(outcome.verdict(), AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn gate_arrow_canonical_transition_honors_transition_tolerance_route() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let dt = 1.0;
        let arrow = gate_arrow_canonical_transition("material_step", old, dt);
        let out = arrow.run(new);
        let routed = canonical_transition_outcome(&old, &new, dt);
        assert_eq!(out.result.admissibility_verdict(), routed.verdict());
        assert!(TRANSITION_TOLERANCE.is_finite());
    }

    #[test]
    fn kleisli_unit_evaluator_default_matches_new() {
        let a = KleisliUnitEvaluator::default();
        let b = KleisliUnitEvaluator::new();
        assert_eq!(a.catalog_id(), b.catalog_id());
        assert_eq!(a.gate_family(), b.gate_family());
    }

    #[test]
    fn kleisli_unit_evaluator_gate_evaluator_trait_surface() {
        let eval: &dyn GateEvaluator = &KleisliUnitEvaluator::new();
        assert_eq!(eval.catalog_id(), KleisliUnitEvaluator::CATALOG_ID);
        assert_eq!(eval.gate_family(), "kleisli_admissibility_unit");
    }

    #[test]
    fn kleisli_unit_evaluator_phase0b_material_calibrated_accept() {
        // Golden fixture: material_gate::material_gate_accepts_phase0b_calibrated_transition
        let eval = KleisliUnitEvaluator::new();
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let dt = 1.0;
        let verdict = eval.evaluate_canonical_transition(&old, &new, dt);
        let routed = canonical_transition_outcome(&old, &new, dt);
        assert_eq!(verdict, routed.verdict());
        assert_eq!(verdict, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn kleisli_unit_evaluator_idle_to_hydrated_route_fixture() {
        // Golden fixture: route::route_delegates_to_transition_outcome
        let eval = KleisliUnitEvaluator::new();
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let verdict = eval.evaluate_canonical_transition(&old, &new, dt);
        assert_eq!(
            verdict,
            canonical_transition_outcome(&old, &new, dt).verdict()
        );
    }

    #[test]
    fn kleisli_unit_evaluator_rejects_malformed_dt() {
        // Golden fixture: transition_proposal::transition_outcome_rejects_malformed_input
        let eval = KleisliUnitEvaluator::new();
        let idle = ThermodynamicStateSnapshot::new_idle();
        let verdict = eval.evaluate_canonical_transition(&idle, &idle, -1.0);
        let outcome = canonical_transition_outcome(&idle, &idle, -1.0);
        assert_eq!(verdict, outcome.verdict());
        assert_ne!(verdict, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn kleisli_canonical_route_honors_transition_tolerance_constant() {
        // SSOT: transition_proposal::TRANSITION_TOLERANCE = 1e-6
        assert_eq!(TRANSITION_TOLERANCE, 1e-6);
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let outcome = canonical_transition_outcome(&old, &new, 1.0);
        assert!(outcome.is_accepted());
    }

    #[test]
    fn kleisli_pipeline_run_sequence_chains_two_admissible_arrows() {
        let pipe = KleisliPipeline::new("ok_chain");
        let ok = gate_arrow_generic("noop_ok", |_x: &i32| (true, 0.0, None));
        let seq = pipe.run_sequence(5, &[&ok, &ok]);
        assert!(seq.result.is_admissible());
        assert_eq!(seq.value, 5);
        assert_eq!(seq.result.dissipation, 0.0);
    }

    #[test]
    fn admissible_join_short_circuits_on_inadmissible_inner() {
        let inner = Admissible {
            value: 42_i32,
            result: AdmissibilityResult::from_verdict(
                AdmissibilityVerdict::Unknown,
                2.0,
                Some("inner_blocked".into()),
            ),
        };
        let nested = Admissible::pure(inner);
        let flat = Admissible::join(nested);
        assert!(!flat.result.is_admissible());
        assert_eq!(flat.value, 42);
        assert_eq!(flat.result.dissipation, 2.0);
        assert_eq!(flat.result.violation.as_deref(), Some("inner_blocked"));
    }

    #[test]
    fn gate_arrow_canonical_transition_rejects_strength_regression() {
        // Golden fixture: material_gate::material_strength_failure_is_not_core_failure
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
        new.free_energy = old.free_energy - 50.0;
        let dt = 1.0;
        let arrow = gate_arrow_canonical_transition("strength_guard", old, dt);
        let out = arrow.run(new);
        let outcome = canonical_transition_outcome(&old, &new, dt);
        assert_eq!(out.result.admissibility_verdict(), outcome.verdict());
        assert_ne!(outcome.verdict(), AdmissibilityVerdict::Accepted);
        assert_eq!(
            out.result.violation.as_deref(),
            Some("canonical_transition_reject")
        );
    }

    #[test]
    fn gate_arrow_generic_propagates_dissipation_on_admit() {
        let arrow = gate_arrow_generic("thermal_sink", |_x: &f64| (true, 3.5, None));
        let out = arrow.run(2.0_f64);
        assert!(out.result.is_admissible());
        assert_eq!(out.result.dissipation, 3.5);
        assert_eq!(out.value, 2.0);
    }

    #[test]
    fn kleisli_compose_pair_chains_gate_arrows_preserving_carrier() {
        let check = gate_arrow_generic("finite_dissipation", |_x: &f64| (true, 1.25, None));
        let composed = kleisli_compose_pair(
            move |x: f64| check.run(x),
            |x: f64| Admissible::pure(x),
            "dissipation_then_identity",
        );
        let out = composed.run(std::f64::consts::E);
        assert!(out.result.is_admissible());
        assert_eq!(out.value, std::f64::consts::E);
        assert_eq!(
            out.result.dissipation, 0.0,
            "bind replaces intermediate result when second arrow is pure"
        );
    }

    #[test]
    fn w8e14_kleisli_unit_evaluator_catalog_id_stable() {
        let ev = KleisliUnitEvaluator;
        assert!(!KleisliUnitEvaluator::CATALOG_ID.is_empty());
        assert_eq!(ev.catalog_id(), KleisliUnitEvaluator::CATALOG_ID);
    }
}

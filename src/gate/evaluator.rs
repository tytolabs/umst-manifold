// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use super::thermo_transition::{ThermodynamicGate, ThermodynamicState};
use super::verdict::ConjunctVerdict;
use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

/// Stable multi-gate façade (`catalog_id`s in **`docs/GateUnificationSpec.md`**).
pub trait GateEvaluator {
    #[must_use]
    fn catalog_id(&self) -> &'static str;

    #[must_use]
    fn gate_family(&self) -> &'static str;
}

#[allow(missing_docs)] // Legacy bool mirrors — prefer [`Self::conjunct_verdict`] / [`Self::is_accepted`]
#[derive(Clone, Debug)]
pub struct TransitionVerdict {
    /// Primary discriminant — core ∧ material conjunct cluster.
    pub verdict: ConjunctVerdict,
    /// Legacy mirror of [`ConjunctVerdict::is_accepted`] — prefer [`Self::is_accepted`].
    #[deprecated(
        since = "0.2.0",
        note = "use TransitionVerdict::is_accepted() or verdict.is_accepted()"
    )]
    pub admissible: bool,
    pub dissipation_w_m3: f64,
    /// Legacy core conjunct witness — prefer [`CoreGateOutcome`] via open-system route.
    #[deprecated(
        since = "0.2.0",
        note = "use CoreGateOutcome::mass_conserved or verdict reject reason"
    )]
    pub mass_conserved: bool,
    /// Legacy CD ∧ strength fold — prefer [`Self::rest_verdict`] / [`ConjunctVerdict`].
    #[deprecated(
        since = "0.2.0",
        note = "use rest_verdict() or ConjunctVerdict reject reason"
    )]
    pub energy_positive: bool,
}

impl TransitionVerdict {
    /// Borrow the primary [`ConjunctVerdict`] discriminant (FP P2.4 SSOT).
    #[inline]
    #[must_use]
    pub fn conjunct_verdict(&self) -> ConjunctVerdict {
        self.verdict
    }

    /// Whether the composed transition cluster accepted (wire bytes unchanged).
    #[inline]
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.verdict.is_accepted()
    }

    /// Alias for [`Self::is_accepted`] — preserved for façade call-site compatibility.
    #[inline]
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.is_accepted()
    }

    /// Legacy mass-balance conjunct witness (unchanged semantics).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_mass_conserved(&self) -> bool {
        self.mass_conserved
    }

    /// Legacy CD ∧ strength fold witness (unchanged semantics — not Core-only CD).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_energy_positive(&self) -> bool {
        self.energy_positive
    }

    /// REST-stable verdict via locked transition conjunct ladder (legacy `energy_positive` fold).
    #[allow(deprecated)]
    #[must_use]
    pub fn rest_verdict(&self) -> super::verdict::AdmissibilityVerdict {
        super::verdict::AdmissibilityVerdict::from_transition_conjuncts(
            self.admissible,
            self.mass_conserved,
            self.energy_positive,
        )
    }
}

/// Host `f64` transition gate (prototype `ThermodynamicFilter`).
pub trait TransitionGateEvaluator: GateEvaluator {
    fn check_transition_host(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> TransitionVerdict;
}

#[derive(Debug, Clone, Default)]
pub struct ThermodynamicTransitionEvaluator {
    inner: ThermodynamicGate,
}

impl ThermodynamicTransitionEvaluator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: ThermodynamicGate::new(),
        }
    }
}

impl GateEvaluator for ThermodynamicTransitionEvaluator {
    fn catalog_id(&self) -> &'static str {
        CD_TRANSITION_CATALOG_ID
    }

    fn gate_family(&self) -> &'static str {
        "clausius_duhem_transition"
    }
}

impl TransitionGateEvaluator for ThermodynamicTransitionEvaluator {
    #[allow(deprecated)]
    fn check_transition_host(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> TransitionVerdict {
        let r = self.inner.check_transition(old_state, new_state, dt_s);
        TransitionVerdict {
            verdict: r.conjunct_verdict(),
            admissible: r.is_accepted(),
            dissipation_w_m3: r.dissipation,
            mass_conserved: r.is_mass_conserved(),
            energy_positive: r.is_energy_positive(),
        }
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::core::material_transition::SubstrateMaterialParams;
    use crate::gate::verdict::{AdmissibilityVerdict, ConjunctVerdict, GateRejectReason};
    use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

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
    fn evaluator_catalog_surface_stable() {
        let ev = ThermodynamicTransitionEvaluator::new();
        assert_eq!(ev.catalog_id(), CD_TRANSITION_CATALOG_ID);
        assert_eq!(ev.catalog_id(), "umst.gate.cd_transition");
        assert_eq!(ev.gate_family(), "clausius_duhem_transition");
    }

    #[test]
    fn evaluator_default_new_equivalent() {
        let from_new = ThermodynamicTransitionEvaluator::new();
        let from_default = ThermodynamicTransitionEvaluator::default();
        assert_eq!(from_new.catalog_id(), from_default.catalog_id());
        assert_eq!(from_new.gate_family(), from_default.gate_family());
    }

    #[test]
    fn transition_verdict_is_admissible_alias_is_accepted() {
        let (old, new, dt) = golden_identity_admissible();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert_eq!(tv.is_admissible(), tv.is_accepted());
        assert_eq!(tv.conjunct_verdict(), tv.verdict);
        assert!(tv.is_accepted());
        assert!(tv.is_admissible());
    }

    #[test]
    fn transition_verdict_legacy_fields_mirror_accepted_cluster() {
        let (old, new, dt) = golden_identity_admissible();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert_eq!(tv.admissible, tv.is_accepted());
        assert_eq!(tv.mass_conserved, tv.is_mass_conserved());
        assert_eq!(tv.energy_positive, tv.is_energy_positive());
        assert_eq!(tv.verdict, ConjunctVerdict::Accepted);
    }

    #[test]
    fn evaluator_golden_identity_admissible() {
        let (old, new, dt) = golden_identity_admissible();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert!(tv.is_accepted());
        assert_eq!(tv.rest_verdict(), AdmissibilityVerdict::Accepted);
        assert_eq!(tv.rest_verdict().as_str(), AdmissibilityVerdict::ACCEPTED);
        assert!(tv.is_mass_conserved());
        assert!(tv.is_energy_positive());
        assert!(tv.dissipation_w_m3.abs() < 1e-12);
    }

    #[test]
    fn evaluator_golden_mass_reject() {
        let (old, new, dt) = golden_mass_reject();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert!(!tv.is_accepted());
        assert_eq!(tv.rest_verdict(), AdmissibilityVerdict::MassViolation);
        assert_eq!(
            tv.rest_verdict().as_str(),
            AdmissibilityVerdict::MASS_VIOLATION
        );
        assert!(!tv.is_mass_conserved());
        assert_eq!(
            tv.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::MassViolation)
        );
    }

    #[test]
    fn evaluator_golden_negative_dissipation_reject() {
        let (old, new, dt) = golden_negative_dissipation_reject();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert!(!tv.is_accepted());
        assert_eq!(tv.rest_verdict(), AdmissibilityVerdict::NegativeDissipation);
        assert_eq!(
            tv.rest_verdict().as_str(),
            AdmissibilityVerdict::NEGATIVE_DISSIPATION
        );
        assert!(tv.is_mass_conserved());
        assert!(!tv.is_energy_positive());
        assert_eq!(
            tv.conjunct_verdict(),
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn evaluator_parity_inner_gate_dissipation_and_verdict() {
        let old =
            ThermodynamicState::from_mix_with_params(0.5, 0.4, 293.0, &SubstrateMaterialParams);
        let new =
            ThermodynamicState::from_mix_with_params(0.5, 0.65, 293.0, &SubstrateMaterialParams);
        let dt = 86_400.0;
        let mut gate = ThermodynamicGate::new();
        let inner = gate.check_transition(&old, &new, dt);
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        assert_eq!(tv.is_accepted(), inner.is_accepted());
        assert_eq!(tv.is_mass_conserved(), inner.is_mass_conserved());
        assert_eq!(tv.is_energy_positive(), inner.is_energy_positive());
        assert!((tv.dissipation_w_m3 - inner.dissipation).abs() < 1e-12);
        assert_eq!(tv.conjunct_verdict(), inner.conjunct_verdict());
        assert_eq!(
            tv.rest_verdict(),
            AdmissibilityVerdict::from_transition_conjuncts(
                tv.is_accepted(),
                tv.is_mass_conserved(),
                tv.is_energy_positive()
            )
        );
    }

    #[test]
    fn transition_verdict_rest_verdict_locked_ladder() {
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
        for (admissible, mass_conserved, energy_positive, expected) in cases {
            let tv = TransitionVerdict {
                verdict: ConjunctVerdict::from_core(mass_conserved, energy_positive),
                admissible,
                dissipation_w_m3: 0.0,
                mass_conserved,
                energy_positive,
            };
            assert_eq!(
                tv.rest_verdict(),
                expected,
                "admissible={admissible} mass={mass_conserved} energy={energy_positive}"
            );
        }
    }

    #[test]
    fn evaluator_mix_calibrated_phase0b_accepted() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicState::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, 1.0);
        assert!(tv.is_accepted());
        assert_eq!(tv.rest_verdict(), AdmissibilityVerdict::Accepted);
        assert!(tv.dissipation_w_m3.is_finite());
    }

    #[test]
    fn transition_verdict_clone_debug_round_trip() {
        let (old, new, dt) = golden_identity_admissible();
        let mut ev = ThermodynamicTransitionEvaluator::new();
        let tv = ev.check_transition_host(&old, &new, dt);
        let cloned = tv.clone();
        assert_eq!(cloned.is_accepted(), tv.is_accepted());
        assert_eq!(cloned.dissipation_w_m3, tv.dissipation_w_m3);
        assert_eq!(cloned.conjunct_verdict(), tv.conjunct_verdict());
        let debug = format!("{tv:?}");
        assert!(debug.contains("TransitionVerdict"));
        let ev_debug = format!("{ev:?}");
        assert!(ev_debug.contains("ThermodynamicTransitionEvaluator"));
    }

    #[test]
    fn w8e14_evaluator_catalog_id_matches_cd_transition() {
        let ev = ThermodynamicTransitionEvaluator::new();
        assert_eq!(ev.catalog_id(), CD_TRANSITION_CATALOG_ID);
        assert_eq!(ev.gate_family(), "clausius_duhem_transition");
    }
}

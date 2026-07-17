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
            mass_conserved: r.mass_conserved,
            energy_positive: r.energy_positive,
        }
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use super::thermo_transition::{ThermodynamicGate, ThermodynamicState};
use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

/// Stable multi-gate façade (`catalog_id`s in **`docs/GateUnificationSpec.md`**).
pub trait GateEvaluator {
    #[must_use]
    fn catalog_id(&self) -> &'static str;

    #[must_use]
    fn gate_family(&self) -> &'static str;
}

#[derive(Clone, Debug)]
pub struct TransitionVerdict {
    pub admissible: bool,
    pub dissipation_w_m3: f64,
    pub mass_conserved: bool,
    pub energy_positive: bool,
}

impl TransitionVerdict {
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
    fn check_transition_host(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> TransitionVerdict {
        let r = self.inner.check_transition(old_state, new_state, dt_s);
        TransitionVerdict {
            admissible: r.accepted,
            dissipation_w_m3: r.dissipation,
            mass_conserved: r.mass_conserved,
            energy_positive: r.energy_positive,
        }
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Stateful transition evaluator + trivial registry façade (fixture tests / Kleisli integration).

use super::evaluator::GateEvaluator;
use super::kleisli::KleisliUnitEvaluator;
use super::transition_proposal::{ThermodynamicStateSnapshot, TransitionFilter};
use super::verdict::AdmissibilityVerdict;
use crate::runtime::catalog::traceability::THERMODYNAMIC_MIX_CATALOG_ID;

#[derive(Clone, Copy, Debug)]
pub struct ThermodynamicTransitionContext<'a> {
    pub old_state: &'a ThermodynamicStateSnapshot,
    pub new_state: &'a ThermodynamicStateSnapshot,
    pub dt_seconds: f64,
}

/// Wraps [`TransitionFilter`] and maps outcomes to REST-stable [`AdmissibilityVerdict`].
#[derive(Debug)]
pub struct TransitionEvaluator {
    pub filter: TransitionFilter,
}

impl TransitionEvaluator {
    #[must_use]
    pub fn new(filter: TransitionFilter) -> Self {
        Self { filter }
    }

    #[must_use]
    pub fn evaluate_thermo_transition(
        &mut self,
        ctx: ThermodynamicTransitionContext<'_>,
    ) -> AdmissibilityVerdict {
        let r = self
            .filter
            .check_transition(ctx.old_state, ctx.new_state, ctx.dt_seconds);
        r.verdict()
    }
}

impl GateEvaluator for TransitionEvaluator {
    fn catalog_id(&self) -> &'static str {
        THERMODYNAMIC_MIX_CATALOG_ID
    }

    fn gate_family(&self) -> &'static str {
        "thermodynamic_mix_transition"
    }
}

/// Holds registered gate evaluators keyed by [`GateEvaluator::catalog_id`].
#[derive(Debug, Default)]
pub struct GateEvaluatorRegistry {
    thermodynamic_mix: Option<TransitionEvaluator>,
    kleisli_unit: Option<KleisliUnitEvaluator>,
}

impl GateEvaluatorRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, ev: TransitionEvaluator) {
        self.thermodynamic_mix = Some(ev);
    }

    pub fn register_kleisli(&mut self, ev: KleisliUnitEvaluator) {
        self.kleisli_unit = Some(ev);
    }

    pub fn evaluate_mut(
        &mut self,
        catalog_id: &str,
        ctx: ThermodynamicTransitionContext<'_>,
    ) -> Option<AdmissibilityVerdict> {
        if catalog_id == KleisliUnitEvaluator::CATALOG_ID {
            return self
                .kleisli_unit
                .as_ref()
                .map(|ev| ev.evaluate_reflexive_step(ctx.new_state));
        }
        if catalog_id != THERMODYNAMIC_MIX_CATALOG_ID {
            return None;
        }
        Some(
            self.thermodynamic_mix
                .as_mut()?
                .evaluate_thermo_transition(ctx),
        )
    }

    /// Route by `catalog_id` without transition context (monadic unit ping).
    #[must_use]
    pub fn evaluate_kleisli_unit(&self, catalog_id: &str) -> Option<AdmissibilityVerdict> {
        if catalog_id != KleisliUnitEvaluator::CATALOG_ID {
            return None;
        }
        self.kleisli_unit
            .as_ref()
            .map(|ev| ev.evaluate_reflexive_step(&ThermodynamicStateSnapshot::new_idle()))
    }
}

#[deprecated(note = "renamed to TransitionEvaluator")]
pub type ThermodynamicMixEvaluator = TransitionEvaluator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::kleisli::KleisliUnitEvaluator;
    use crate::gate::transition_proposal::{
        ThermodynamicStateSnapshot, TransitionFilter,
    };

    #[test]
    fn w8e14_registry_routes_thermodynamic_mix_catalog() {
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = old;
        let ctx = ThermodynamicTransitionContext {
            old_state: &old,
            new_state: &new,
            dt_seconds: 1.0,
        };
        let mut reg = GateEvaluatorRegistry::new();
        reg.register(TransitionEvaluator::new(TransitionFilter::default()));
        let verdict = reg
            .evaluate_mut(THERMODYNAMIC_MIX_CATALOG_ID, ctx)
            .expect("registered evaluator");
        assert_eq!(verdict, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn w8e14_registry_unknown_catalog_returns_none() {
        let old = ThermodynamicStateSnapshot::new_idle();
        let ctx = ThermodynamicTransitionContext {
            old_state: &old,
            new_state: &old,
            dt_seconds: 1.0,
        };
        let mut reg = GateEvaluatorRegistry::new();
        assert!(reg.evaluate_mut("unknown-catalog", ctx).is_none());
    }

    #[test]
    fn w8e14_registry_kleisli_unit_ping_without_context() {
        let mut reg = GateEvaluatorRegistry::new();
        reg.register_kleisli(KleisliUnitEvaluator);
        let verdict = reg
            .evaluate_kleisli_unit(KleisliUnitEvaluator::CATALOG_ID)
            .expect("kleisli unit registered");
        assert_eq!(verdict, AdmissibilityVerdict::Accepted);
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Golden vectors for the gate façade: stable [`GateEvaluator`] catalog identity plus host transition
//! parity mapped to [`AdmissibilityVerdict`] tokens (same predicate split as [`ThermodynamicGate`]).
//! [`ThermodynamicMixEvaluator`] repeats the scalar mix path via [`ThermodynamicStateSnapshot`].
//! Hand-constructed states — no network.

use umst_manifold::gate::{
    AdmissibilityVerdict, GateEvaluator, ThermodynamicMixEvaluator, ThermodynamicMixFilter,
    ThermodynamicState, ThermodynamicStateSnapshot, ThermodynamicTransitionContext,
    ThermodynamicTransitionEvaluator, TransitionGateEvaluator, TransitionVerdict,
};

fn verdict_from_transition(tv: &TransitionVerdict) -> AdmissibilityVerdict {
    if tv.admissible {
        AdmissibilityVerdict::Accepted
    } else if !tv.mass_conserved {
        AdmissibilityVerdict::MassViolation
    } else if !tv.energy_positive {
        AdmissibilityVerdict::NegativeDissipation
    } else {
        AdmissibilityVerdict::Unknown
    }
}

fn host_to_snapshot(s: &ThermodynamicState) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: s.density,
        temperature: s.temperature,
        free_energy: s.free_energy,
        entropy: s.entropy,
        hydration_degree: s.hydration_degree,
        strength: s.strength,
    }
}

/// Trivial admissible transition: identical bulk state over `dt` ⇒ zero intrinsic dissipation rate.
fn golden_identity_admissible() -> (ThermodynamicState, ThermodynamicState, f64) {
    let s = ThermodynamicState {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -1.35e5,
        entropy: 0.05,
        hydration_degree: 0.42,
        strength: 12.7,
    };
    (s.clone(), s, 1.0)
}

/// Mass bound violation: [`ThermodynamicGate`] uses `|Δρ| < 100` as the discrete guard.
fn golden_mass_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
    let old = ThermodynamicState {
        density: 2400.0,
        temperature: 293.0,
        free_energy: 0.0,
        entropy: 0.1,
        hydration_degree: 0.3,
        strength: 10.0,
    };
    let mut new = old.clone();
    new.density = 2280.0; // |Δρ| = 120 ≥ 100
    (old, new, 3600.0)
}

/// Thermodynamic inconsistency: `D_int = −ρ ψ̇` must stay ≥ −tolerance; spike `ψ` upward to force reject.
fn golden_negative_dissipation_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
    let old = ThermodynamicState {
        density: 2200.0,
        temperature: 300.0,
        free_energy: -2.0e5,
        entropy: 0.2,
        hydration_degree: 0.5,
        strength: 20.0,
    };
    let mut new = old.clone();
    new.free_energy = -1.0e4;
    (old, new, 1.0)
}

#[test]
fn gate_evaluator_catalog_surface_stable() {
    let ge = ThermodynamicTransitionEvaluator::new();
    assert_eq!(ge.catalog_id(), "umst.gate.cd_transition");
    assert_eq!(ge.gate_family(), "clausius_duhem_transition");
}

#[test]
fn gate_evaluator_golden_identity_accepted() {
    let (old, new, dt) = golden_identity_admissible();
    let mut ev = ThermodynamicTransitionEvaluator::new();
    let tv = ev.check_transition_host(&old, &new, dt);
    assert!(tv.admissible);
    let v = verdict_from_transition(&tv);
    assert_eq!(v, AdmissibilityVerdict::Accepted);
    assert_eq!(v.as_str(), AdmissibilityVerdict::ACCEPTED);
}

#[test]
fn gate_evaluator_golden_mass_violation() {
    let (old, new, dt) = golden_mass_reject();
    let mut ev = ThermodynamicTransitionEvaluator::new();
    let tv = ev.check_transition_host(&old, &new, dt);
    assert!(!tv.admissible);
    let v = verdict_from_transition(&tv);
    assert_eq!(v, AdmissibilityVerdict::MassViolation);
    assert_eq!(v.as_str(), AdmissibilityVerdict::MASS_VIOLATION);
}

#[test]
fn gate_evaluator_golden_negative_dissipation() {
    let (old, new, dt) = golden_negative_dissipation_reject();
    let mut ev = ThermodynamicTransitionEvaluator::new();
    let tv = ev.check_transition_host(&old, &new, dt);
    assert!(!tv.admissible);
    let v = verdict_from_transition(&tv);
    assert_eq!(v, AdmissibilityVerdict::NegativeDissipation);
    assert_eq!(v.as_str(), AdmissibilityVerdict::NEGATIVE_DISSIPATION);
}

#[test]
fn mix_gate_evaluator_catalog_surface_stable() {
    let ge = ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new());
    assert_eq!(ge.catalog_id(), "thermodynamic_mix");
    assert_eq!(ge.gate_family(), "thermodynamic_mix_transition");
}

#[test]
fn mix_gate_evaluator_golden_identity_accepted() {
    let (old, new, dt) = golden_identity_admissible();
    let old_s = host_to_snapshot(&old);
    let new_s = host_to_snapshot(&new);
    let mut ev = ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new());
    let ctx = ThermodynamicTransitionContext {
        old_state: &old_s,
        new_state: &new_s,
        dt_seconds: dt,
    };
    let v = ev.evaluate_thermo_transition(ctx);
    assert_eq!(v, AdmissibilityVerdict::Accepted);
}

#[test]
fn mix_gate_evaluator_golden_mass_violation() {
    let (old, new, dt) = golden_mass_reject();
    let old_s = host_to_snapshot(&old);
    let new_s = host_to_snapshot(&new);
    let mut ev = ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new());
    let ctx = ThermodynamicTransitionContext {
        old_state: &old_s,
        new_state: &new_s,
        dt_seconds: dt,
    };
    let v = ev.evaluate_thermo_transition(ctx);
    assert_eq!(v, AdmissibilityVerdict::MassViolation);
}

#[test]
fn mix_gate_evaluator_golden_negative_dissipation() {
    let (old, new, dt) = golden_negative_dissipation_reject();
    let old_s = host_to_snapshot(&old);
    let new_s = host_to_snapshot(&new);
    let mut ev = ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new());
    let ctx = ThermodynamicTransitionContext {
        old_state: &old_s,
        new_state: &new_s,
        dt_seconds: dt,
    };
    let v = ev.evaluate_thermo_transition(ctx);
    assert_eq!(v, AdmissibilityVerdict::NegativeDissipation);
}

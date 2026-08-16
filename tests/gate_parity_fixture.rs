// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Golden vectors for the gate façade: stable [`GateEvaluator`] catalog identity plus host transition
//! parity mapped to [`AdmissibilityVerdict`] tokens (same predicate split as [`ThermodynamicGate`]).
//! [`ThermodynamicMixEvaluator`] repeats the scalar mix path via [`ThermodynamicStateSnapshot`].
//! Hand-constructed states — no network.

use umst_manifold::gate::{
    AdmissibilityVerdict, GateEvaluator, ThermodynamicMixEvaluator, ThermodynamicMixFilter,
    ThermodynamicState, ThermodynamicStateSnapshot, ThermodynamicTransitionContext,
    ThermodynamicTransitionEvaluator, TransitionGateEvaluator,
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

/// Trivial admissible transition: identical bulk state over `dt` ⇒ zero intrinsic dissipation rate.
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

/// Mass bound violation: [`ThermodynamicGate`] uses `|Δρ| < 100` as the discrete guard.
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
        reaction_extent: 0.5,
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
    assert!(tv.is_admissible());
    let v = tv.rest_verdict();
    assert_eq!(v, AdmissibilityVerdict::Accepted);
    assert_eq!(v.as_str(), AdmissibilityVerdict::ACCEPTED);
}

#[test]
fn gate_evaluator_golden_mass_violation() {
    let (old, new, dt) = golden_mass_reject();
    let mut ev = ThermodynamicTransitionEvaluator::new();
    let tv = ev.check_transition_host(&old, &new, dt);
    assert!(!tv.is_admissible());
    let v = tv.rest_verdict();
    assert_eq!(v, AdmissibilityVerdict::MassViolation);
    assert_eq!(v.as_str(), AdmissibilityVerdict::MASS_VIOLATION);
}

#[test]
fn gate_evaluator_golden_negative_dissipation() {
    let (old, new, dt) = golden_negative_dissipation_reject();
    let mut ev = ThermodynamicTransitionEvaluator::new();
    let tv = ev.check_transition_host(&old, &new, dt);
    assert!(!tv.is_admissible());
    let v = tv.rest_verdict();
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

/// Cartridge-style mix-calibrated lift (concrete SSOT scalars) — parity with host CD cartridge.
#[test]
fn gate_parity_concrete_mix_calibrated_snapshots() {
    use umst_manifold::core::SubstrateMaterialParams;
    use umst_manifold::gate::transition_proposal::ThermodynamicStateSnapshot;
    use umst_manifold::runtime::gate::evidence::AdmissibilityToken;
    use umst_manifold::runtime::gate::{CdTransitionCartridge, GateCartridge};

    const CEMENT_S_INTRINSIC_MPA: f64 = 240.0;

    let old = ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
        0.45,
        0.30,
        293.15,
        CEMENT_S_INTRINSIC_MPA,
        &SubstrateMaterialParams,
    );
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    assert!(
        old.strength > 0.0,
        "mix-calibrated lift must carry non-zero strength at cement SSOT"
    );
}

/// Mix-calibrated cement lift (240 MPa) must agree with host CD on admissible identity step.
#[test]
fn gate_parity_cement_ssot_matches_host_cd() {
    use umst_manifold::core::SubstrateMaterialParams;
    use umst_manifold::gate::transition_proposal::ThermodynamicStateSnapshot;
    use umst_manifold::runtime::gate::evidence::AdmissibilityToken;
    use umst_manifold::runtime::gate::{CdTransitionCartridge, GateCartridge};

    let old = ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
        0.45,
        0.30,
        293.15,
        240.0,
        &SubstrateMaterialParams,
    );
    let new = old;
    let cd = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    assert_eq!(cd.admissibility, AdmissibilityToken::Admissible);
    assert_eq!(cd.catalog_id, "umst.gate.cd_transition");
}

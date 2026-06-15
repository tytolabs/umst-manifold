// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Chaos Crucible POC: fail-closed gate under synthetic pathology (robustness, not antifragile).

use umst_manifold::gate::{ThermodynamicMixFilter, ThermodynamicStateSnapshot};
use umst_manifold::{PrecisionLane, SolveReport};

#[test]
fn nan_free_energy_rejects_and_preserves_old_state() {
    let mut filter = ThermodynamicMixFilter::new();
    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let old_copy = old;
    let mut new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
    new.free_energy = f64::NAN;

    let outcome = filter.check_transition(&old, &new, 3600.0);
    assert!(!outcome.accepted, "NaN free_energy must reject");
    assert_eq!(old.density, old_copy.density);
    assert_eq!(old.free_energy, old_copy.free_energy);
    assert_eq!(old.hydration_degree, old_copy.hydration_degree);
}

#[test]
fn garbage_temperature_rejects() {
    let mut filter = ThermodynamicMixFilter::new();
    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let mut new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
    new.temperature = -1.0;

    let outcome = filter.check_transition(&old, &new, 3600.0);
    assert!(!outcome.accepted, "negative temperature must reject");
}

#[test]
fn infinity_free_energy_rejects() {
    let mut filter = ThermodynamicMixFilter::new();
    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let mut new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
    new.free_energy = f64::INFINITY;

    let outcome = filter.check_transition(&old, &new, 3600.0);
    assert!(!outcome.accepted, "infinite free_energy must reject");
}

#[test]
fn clean_forward_hydration_still_accepts() {
    let mut filter = ThermodynamicMixFilter::new();
    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);

    let outcome = filter.check_transition(&old, &new, 3600.0);
    assert!(outcome.accepted, "clean forward step must accept");
    assert!(outcome.dissipation > 0.0);
}

#[test]
fn non_converged_solve_report_rejected_by_honesty_hook() {
    let report = SolveReport {
        iterations: 2000,
        rel_residual: 1.0,
        stiffness_scale: 1.0,
        e_ref: 30e9,
        dx_char: 0.1,
        rel_tol: 1e-6,
        lane: PrecisionLane::HostKrylov,
    };
    assert!(!report.converged());
    assert!(report.gate_reject_non_converged_solve());
}

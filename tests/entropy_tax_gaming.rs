// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Entropy Tax POC: gaming move raises joules excess over Landauer floor.

use umst_manifold::gate::{ThermodynamicMixFilter, ThermodynamicStateSnapshot};
use umst_manifold::{entropy_tax_j, PrecisionLane, SolveReport};

#[test]
fn tax_gaming_exceeds_tax_honest_high_iteration_low_progress() {
    let t_k = 293.0;

    let honest_report = SolveReport {
        iterations: 10,
        rel_residual: 1e-7,
        stiffness_scale: 1.0,
        e_ref: 30e9,
        dx_char: 0.1,
        rel_tol: 1e-6,
        lane: PrecisionLane::F32BurnBarPcg,
    };
    assert!(honest_report.converged());

    let mut filter = ThermodynamicMixFilter::new();
    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, t_k);
    let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, t_k);
    let honest_outcome = filter.check_transition(&old, &new, 3600.0);
    assert!(honest_outcome.accepted);
    let tax_honest = honest_report.entropy_tax_j(honest_outcome.dissipation, t_k);

    let gaming_report = SolveReport {
        iterations: 5000,
        rel_residual: 0.5,
        rel_tol: 1e-6,
        ..honest_report
    };
    assert!(!gaming_report.converged());
    let tax_gaming = gaming_report.entropy_tax_j(honest_outcome.dissipation * 2.0, t_k);

    eprintln!("tax_honest={tax_honest:.6} J tax_gaming={tax_gaming:.6} J");
    assert!(
        tax_gaming > tax_honest,
        "gaming tax {tax_gaming} must exceed honest {tax_honest}"
    );
}

#[test]
fn tax_gaming_exceeds_tax_honest_straight_through_biased_dissipation() {
    let t_k = 293.0;
    let delta_mi_bits = 100.0_f64;
    let landauer_floor = entropy_tax_j(0.0, t_k, delta_mi_bits).abs();

    let honest_dissipation = landauer_floor * 1.5;
    let tax_honest = entropy_tax_j(honest_dissipation, t_k, delta_mi_bits);
    assert!(tax_honest > 0.0, "honest path slightly above floor");

    let gaming_dissipation = landauer_floor * 50.0;
    let tax_gaming = entropy_tax_j(gaming_dissipation, t_k, delta_mi_bits);

    eprintln!("tax_honest={tax_honest:.6} J tax_gaming={tax_gaming:.6} J");
    assert!(tax_gaming > tax_honest);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0d — single gate routing surface (blueprint §7 0d).

use umst_manifold::gate::{
    canonical_transition_admissible, canonical_transition_outcome, route,
    KleisliUnitEvaluator, ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
    GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX,
};

#[test]
fn phase0d_route_module_exports_canonical_surface() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    let dt = 28.0 * 24.0 * 3600.0;
    assert!(canonical_transition_admissible(&old, &new, dt));
    let _ = route::canonical_core_gate_outcome(2400.0, 2400.0, 0.0, -1.0, dt, 0.0);
}

#[test]
fn phase0d_kleisli_routes_canonical_transition() {
    let eval = KleisliUnitEvaluator::new();
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    let verdict = eval.evaluate_canonical_transition(&old, &new, 28.0 * 24.0 * 3600.0);
    assert_eq!(verdict, umst_manifold::gate::AdmissibilityVerdict::Accepted);
}

#[test]
fn phase0d_parity_digest_unchanged() {
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "d5608148e29eeabd83935988699d08ce1233c3e87f2cd217d658e0c71c7a841e"
    );
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "d5608148e29eeabd");
}

#[test]
fn phase0d_open_deltas_cleared_for_routing() {
    use umst_manifold::gate::OPEN_RECONCILIATION_DELTAS;
    let ids: Vec<_> = OPEN_RECONCILIATION_DELTAS.iter().map(|d| d.id).collect();
    assert!(!ids.contains(&"http_shim_strength_only"));
    assert!(!ids.contains(&"kleisli_partial_cd"));
    assert!(!ids.contains(&"mcp_composite_physics_path"));
    assert!(
        !ids.contains(&"cbf_open_system_extension"),
        "0e clears cbf_open_system_extension — see phase0e_open_system_spike"
    );
}

#[test]
fn phase0d_transition_outcome_matches_route() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    let dt = 28.0 * 24.0 * 3600.0;
    let routed = canonical_transition_outcome(&old, &new, dt);
    let direct = umst_manifold::gate::transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    assert_eq!(routed, direct);
}

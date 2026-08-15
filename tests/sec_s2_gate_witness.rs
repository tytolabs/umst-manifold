// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S2 gate runtime witness — manifold TrustGatePolicy refuse-path census on cold-edge evidence.

use umst_manifold::runtime::gate::{
    gate_trust_refuse_census, manifold_gate_sec_s2_ceremony_closed,
    manifold_s2_all_refuse_paths_probed, manifold_verify_trust_gate_policy_pins,
    sec_s2_gate_factor_table, sec_s2_gate_manifold_probe, sec_s2_gate_transition_evidence_probe,
    sec_s2_gate_wire_matrix, sec_s2_p1941_k2_honest, sec_s2_p1941_k2_probe,
    sec_s2_production_wired, validate_sec_s2_gate_honesty, FLEET_P1941_K2_JOB_ID,
    MANIFOLD_SEC_S2_GATE_WIRE_HOPS, S2_FACTOR_ROW_COUNT, S2_GREEN_CLAIM_BLOCKED,
};

#[test]
fn sec_s2_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s2_gate_transition_evidence_probe());
}

#[test]
fn sec_s2_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s2_ceremony_closed());
    assert!(manifold_s2_all_refuse_paths_probed());
    assert!(manifold_verify_trust_gate_policy_pins());
    assert!(!sec_s2_production_wired());
    assert!(S2_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_s2_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S2_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S2_GATE_WIRE_HOPS.len(), 7);
}

#[test]
fn sec_s2_p1941_k2_fleet_probe_honest() {
    let probe = sec_s2_p1941_k2_probe();
    assert_eq!(probe.k2_job_id, FLEET_P1941_K2_JOB_ID);
    assert!(probe.ceremony_closed);
    assert!(sec_s2_p1941_k2_honest());
}

#[test]
fn sec_s2_gate_census_validate_and_matrix() {
    validate_sec_s2_gate_honesty().expect("honest SEC-S2 gate census");
    let census = gate_trust_refuse_census();
    assert_eq!(census.wire_hop_wired_count, 5);
    assert_eq!(census.factor_row_count, S2_FACTOR_ROW_COUNT);
    let probe = sec_s2_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_s2_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
    let factors = sec_s2_gate_factor_table();
    assert!(factors.contains("expected_gate_exit=BLOCKED"));
}

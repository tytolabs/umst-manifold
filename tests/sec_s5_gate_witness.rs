// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S5 gate runtime witness — manifold synthetic consensus census on cold-edge evidence.

use umst_manifold::runtime::gate::{
    gate_synthetic_consensus_census, manifold_gate_sec_s5_ceremony_closed,
    manifold_s5_all_scenarios_probed, manifold_verify_s5_consensus_algebra_roundtrip,
    sec_s5_gate_factor_table, sec_s5_gate_manifold_probe, sec_s5_gate_transition_evidence_probe,
    sec_s5_gate_wire_matrix, sec_s5_p1812_i2_honest, sec_s5_p1812_i2_probe, sec_s5_production_wired,
    validate_sec_s5_gate_honesty, FLEET_P1812_I2_JOB_ID, LN0_PROOF_WIRED_HONEST,
    LIVE_FANOUT_WIRED_HONEST, MANIFOLD_SEC_S5_GATE_WIRE_HOPS, S5_GREEN_CLAIM_BLOCKED,
};

#[test]
fn sec_s5_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s5_gate_transition_evidence_probe());
}

#[test]
fn sec_s5_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s5_ceremony_closed());
    assert!(manifold_s5_all_scenarios_probed());
    assert!(manifold_verify_s5_consensus_algebra_roundtrip());
    assert!(!sec_s5_production_wired());
    assert!(!LN0_PROOF_WIRED_HONEST);
    assert!(!LIVE_FANOUT_WIRED_HONEST);
    assert!(S5_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_s5_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S5_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S5_GATE_WIRE_HOPS.len(), 7);
}

#[test]
fn sec_s5_p1812_i2_fleet_probe_honest() {
    let probe = sec_s5_p1812_i2_probe();
    assert_eq!(probe.i2_job_id, FLEET_P1812_I2_JOB_ID);
    assert!(probe.ceremony_closed);
    assert!(sec_s5_p1812_i2_honest());
}

#[test]
fn sec_s5_gate_census_validate_and_matrix() {
    validate_sec_s5_gate_honesty().expect("honest SEC-S5 gate census");
    let census = gate_synthetic_consensus_census();
    assert_eq!(census.wire_hop_wired_count, 5);
    let probe = sec_s5_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_s5_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
    let factors = sec_s5_gate_factor_table();
    assert!(factors.contains("expected_gate_exit=BLOCKED"));
}

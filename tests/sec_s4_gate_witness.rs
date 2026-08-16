// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! SEC-S4 gate runtime witness — manifold side-channel scrub census on cold-edge evidence.

use umst_manifold::runtime::gate::{
    gate_side_channel_scrub_census, manifold_gate_sec_s4_ceremony_closed,
    manifold_ls5_all_k_v1_probed, manifold_verify_scrub_roundtrip, sec_s4_gate_manifold_probe,
    sec_s4_gate_transition_evidence_probe, sec_s4_gate_wire_matrix, sec_s4_p1800_h3_honest,
    sec_s4_p1800_h3_probe, sec_s4_production_wired, validate_sec_s4_gate_honesty,
    FLEET_P1800_H3_JOB_ID, L_S5_PROOF_WIRED_HONEST, MANIFOLD_SEC_S4_GATE_WIRE_HOPS,
};

#[test]
fn sec_s4_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s4_gate_transition_evidence_probe());
}

#[test]
fn sec_s4_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s4_ceremony_closed());
    assert!(manifold_ls5_all_k_v1_probed());
    assert!(manifold_verify_scrub_roundtrip());
    assert!(!sec_s4_production_wired());
    assert!(!L_S5_PROOF_WIRED_HONEST);
}

#[test]
fn sec_s4_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S4_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S4_GATE_WIRE_HOPS.len(), 7);
}

#[test]
fn sec_s4_p1800_h3_fleet_probe_honest() {
    let probe = sec_s4_p1800_h3_probe();
    assert_eq!(probe.h3_job_id, FLEET_P1800_H3_JOB_ID);
    assert!(probe.ceremony_closed);
    assert!(sec_s4_p1800_h3_honest());
}

#[test]
fn sec_s4_gate_census_validate_and_matrix() {
    validate_sec_s4_gate_honesty().expect("honest SEC-S4 gate census");
    let census = gate_side_channel_scrub_census();
    assert_eq!(census.wire_hop_wired_count, 5);
    let probe = sec_s4_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_s4_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S3 gate runtime witness — manifold palette/ledger census on cold-edge evidence.

use umst_manifold::runtime::gate::{
    gate_palette_ledger_census, gate_transition_evidence_probe, manifold_gate_sec_s3_ceremony_closed,
    sec_s3_gate_manifold_probe, sec_s3_gate_wire_matrix, sec_s3_p1606_c5_honest, sec_s3_p1606_c5_probe,
    sec_s3_production_wired, session_ledger_wired, validate_sec_s3_gate_honesty,
    FLEET_P1606_C5_JOB_ID, MANIFOLD_SEC_S3_GATE_WIRE_HOPS, PALETTE_PERSISTED_HONEST,
};

#[test]
fn sec_s3_gate_transition_evidence_wired_on_manifold() {
    assert!(gate_transition_evidence_probe());
}

#[test]
fn sec_s3_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s3_ceremony_closed());
    assert!(!session_ledger_wired());
    assert!(!sec_s3_production_wired());
    assert!(!PALETTE_PERSISTED_HONEST);
}

#[test]
fn sec_s3_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S3_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S3_GATE_WIRE_HOPS.len(), 7);
}

#[test]
fn sec_s3_p1606_c5_fleet_probe_honest() {
    let probe = sec_s3_p1606_c5_probe();
    assert_eq!(probe.c5_job_id, FLEET_P1606_C5_JOB_ID);
    assert!(probe.ceremony_closed);
    assert!(sec_s3_p1606_c5_honest());
}

#[test]
fn sec_s3_gate_census_validate_and_matrix() {
    validate_sec_s3_gate_honesty().expect("honest SEC-S3 gate census");
    let census = gate_palette_ledger_census();
    assert_eq!(census.wire_hop_wired_count, 5);
    let probe = sec_s3_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_s3_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
}

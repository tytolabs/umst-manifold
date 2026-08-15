// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S7 gate runtime witness — manifold fed-trust migration census on cold-edge evidence.

use umst_manifold::runtime::gate::sec_s7::{
    manifold_s7_migrate_queue_residue_pins_verified, migration_complete_measured,
    sec_s7_accel_ac08_honest, sec_s7_accel_ac08_probe, sec_s7_migrate_queue_table,
    ACCEL_2030_AC08_JOB_ID,
};
use umst_manifold::runtime::gate::{
    gate_fed_trust_migration_census, manifold_gate_sec_s7_ceremony_closed,
    manifold_s7_all_migrate_surfaces_probed, manifold_verify_migration_inventory_census,
    sec_s7_gate_factor_table, sec_s7_gate_manifold_probe, sec_s7_gate_transition_evidence_probe,
    sec_s7_gate_wire_matrix, sec_s7_p1931_j2_honest, sec_s7_p1931_j2_probe,
    sec_s7_production_wired, validate_sec_s7_gate_honesty, FLEET_P1931_J2_JOB_ID,
    MANIFOLD_SEC_S7_GATE_WIRE_HOPS, MIGRATION_COMPLETE_HONEST, S7_GREEN_CLAIM_BLOCKED,
    S_FED_TRUST_PARTIAL_HONEST, S_FED_TRUST_PRODUCTION_WIRED_HONEST,
};

#[test]
fn sec_s7_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s7_gate_transition_evidence_probe());
}

#[test]
fn sec_s7_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s7_ceremony_closed());
    assert!(manifold_s7_all_migrate_surfaces_probed());
    assert!(manifold_verify_migration_inventory_census());
    assert!(!sec_s7_production_wired());
    assert!(!MIGRATION_COMPLETE_HONEST);
    assert!(S7_GREEN_CLAIM_BLOCKED);
    assert!(S_FED_TRUST_PARTIAL_HONEST);
    assert!(!S_FED_TRUST_PRODUCTION_WIRED_HONEST);
}

#[test]
fn sec_s7_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S7_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S7_GATE_WIRE_HOPS.len(), 7);
}

#[test]
fn sec_s7_p1931_j2_fleet_probe_honest() {
    let probe = sec_s7_p1931_j2_probe();
    assert_eq!(probe.j2_job_id, FLEET_P1931_J2_JOB_ID);
    assert!(probe.ceremony_closed);
    assert!(sec_s7_p1931_j2_honest());
}

#[test]
fn sec_s7_accel_ac08_migrate_queue_deepen_honest() {
    assert!(sec_s7_accel_ac08_honest());
    let probe = sec_s7_accel_ac08_probe();
    assert_eq!(probe.ac08_job_id, ACCEL_2030_AC08_JOB_ID);
    assert!(probe.migrate_queue_table_residue_pinned);
    assert!(!migration_complete_measured());
    let table = sec_s7_migrate_queue_table();
    assert!(table.contains("residue_id=R-classical-wrap-gateway"));
    assert!(manifold_s7_migrate_queue_residue_pins_verified());
}

#[test]
fn sec_s7_gate_census_validate_and_matrix() {
    validate_sec_s7_gate_honesty().expect("honest SEC-S7 gate census");
    let census = gate_fed_trust_migration_census();
    assert_eq!(census.wire_hop_wired_count, 5);
    let probe = sec_s7_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_s7_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
    let factors = sec_s7_gate_factor_table();
    assert!(factors.contains("expected_gate_exit=BLOCKED"));
}

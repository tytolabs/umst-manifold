// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S1 gate runtime witness — manifold trust-gate census on cold-edge evidence.
//!
//! FLEET-COMPOSER ACCEL-K **AC352** · verify-only · 0 gateway writers.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC352.md`.
//! Absorbs AC28 (`FLEET_ACCEL2_AC28_JOB_ID`) · Y79 · H51 · J33 · G72 · Z48 prior receipts.

use umst_manifold::runtime::gate::{
    collect_sec_s1_gate_factor_rows, gate_trust_census, manifold_gate_sec_s1_ceremony_closed,
    manifold_s1_all_factors_probed, manifold_s1_factor_coverage_probes,
    manifold_verify_trust_gate_s1_pins, sec_s1_accel_ac28_honest, sec_s1_accel_ac28_probe,
    sec_s1_gate_factor_table, sec_s1_gate_manifold_probe, sec_s1_gate_transition_evidence_probe,
    sec_s1_gate_wire_matrix, sec_s1_production_wired, sec_s1_session_ledger_next_hop,
    sec_s1_session_ledger_wired, validate_sec_s1_gate_honesty, FLEET_ACCEL2_AC28_JOB_ID,
    FLEET_ACCEL2_AC28_RECEIPT_PATH, MANIFOLD_SEC_S1_GATE_WIRE_HOPS, S1_FACTOR_IDS,
    S1_FACTOR_ROW_COUNT, S1_GREEN_CLAIM_BLOCKED, SEC_S1_BOARD_SLICE_ID, SEC_S1_EXPECTED_GATE_EXIT,
};

/// FLEET-COMPOSER ACCEL-K AC352 agent job id.
pub const FLEET_ACCEL2_AC352_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC352";

/// AC352 receipt path — SSOT for this pass.
pub const COMPOSER_ACCEL2_AC352_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC352.md";

/// Fleet verify command (scratch target dir).
pub const AC352_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac352-sec-s1 cargo test -p umst-manifold sec_s1 -- --nocapture";

#[test]
fn sec_s1_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s1_gate_transition_evidence_probe());
}

#[test]
fn sec_s1_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s1_ceremony_closed());
    assert!(manifold_s1_all_factors_probed());
    assert!(manifold_verify_trust_gate_s1_pins());
    assert!(!sec_s1_production_wired());
    assert!(!sec_s1_session_ledger_wired());
    assert!(S1_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_s1_gate_wire_hops_five_of_seven_wired() {
    let wired = MANIFOLD_SEC_S1_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 5);
    assert_eq!(MANIFOLD_SEC_S1_GATE_WIRE_HOPS.len(), 7);
    assert!(MANIFOLD_SEC_S1_GATE_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("trust_wrap_wired") && !h.wired));
    assert!(MANIFOLD_SEC_S1_GATE_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("trust_gate_production_wired") && !h.wired));
}

#[test]
fn sec_s1_factor_coverage_six_of_six() {
    assert_eq!(S1_FACTOR_IDS.len(), S1_FACTOR_ROW_COUNT);
    let probes = manifold_s1_factor_coverage_probes();
    assert_eq!(probes.len(), S1_FACTOR_ROW_COUNT);
    assert!(probes.iter().all(|p| p.probe_hit));
}

#[test]
fn sec_s1_accel_ac28_fleet_probe_honest() {
    let probe = sec_s1_accel_ac28_probe();
    assert_eq!(probe.ac28_job_id, FLEET_ACCEL2_AC28_JOB_ID);
    assert!(probe.prior_y79_absorbed);
    assert!(probe.prior_h51_absorbed);
    assert!(probe.prior_j33_absorbed);
    assert!(probe.prior_g72_absorbed);
    assert!(probe.prior_z48_absorbed);
    assert!(probe.trust_gate_deepen_matrix_verified);
    assert!(probe.ceremony_closed);
    assert!(sec_s1_accel_ac28_honest());
}

#[test]
fn sec_s1_gate_census_validate_and_matrix() {
    validate_sec_s1_gate_honesty().expect("honest SEC-S1 gate census");
    let census = gate_trust_census();
    assert_eq!(census.board_slice_id, SEC_S1_BOARD_SLICE_ID);
    assert_eq!(census.factor_row_count, S1_FACTOR_ROW_COUNT);
    assert_eq!(census.wire_hop_wired_count, 5);
    assert!(census.s1_all_factors_probed);
    assert!(census.s1_green_claim_blocked);
    assert!(!census.production_wired);
    assert!(!census.session_ledger_wired);
    let probe = sec_s1_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    assert_eq!(probe.wire_hop_wired_count, 5);
    let matrix = sec_s1_gate_wire_matrix();
    assert!(matrix.contains("wired=5/7"));
    assert!(matrix.contains("production_wired=false"));
    let factors = sec_s1_gate_factor_table();
    assert!(factors.contains("expected_gate_exit=BLOCKED"));
    assert!(factors.contains("scert_credit=BLOCKED"));
    let rows = collect_sec_s1_gate_factor_rows();
    assert_eq!(rows.len(), S1_FACTOR_ROW_COUNT);
    assert!(rows.iter().all(|r| r.probe_wired));
    assert!(rows.iter().all(|r| !r.acceptance_credit));
}

#[test]
fn fleet_accel2_ac352_sec_s1_trust_gate_honest() {
    assert_eq!(SEC_S1_BOARD_SLICE_ID, "SEC-S1");
    assert_eq!(SEC_S1_EXPECTED_GATE_EXIT, "BLOCKED");
    assert_eq!(FLEET_ACCEL2_AC352_JOB_ID, "FLEET-COMPOSER-ACCEL2-AC352");
    assert!(COMPOSER_ACCEL2_AC352_RECEIPT_PATH.contains("AC352"));
    assert!(AC352_VERIFY_COMMAND.contains("umst-accel2-ac352-sec-s1"));
    assert!(FLEET_ACCEL2_AC28_RECEIPT_PATH.contains("AC28"));
    assert!(manifold_gate_sec_s1_ceremony_closed());
    assert!(sec_s1_accel_ac28_honest());
    assert!(!sec_s1_production_wired());
    assert!(!sec_s1_session_ledger_wired());
    assert!(sec_s1_session_ledger_next_hop().contains("session_ledger_wired"));
}

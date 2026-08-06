// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-GW-AUDIT gate runtime witness — manifold admit-audit census on cold-edge evidence.
//!
//! FLEET-COMPOSER ACCEL-K **AC354** · verify-only · 0 gateway writers.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC354.md`.
//! Absorbs AC31 (`FLEET_ACCEL2_AC31_JOB_ID`) · Y51 · Z87 · Z08 · P1710-F1 prior receipts.

use umst_manifold::runtime::gate::{
    gate_admit_audit_census, manifold_gate_sec_gw_audit_ceremony_closed,
    manifold_gw_audit_all_stamp_paths_probed, manifold_gw_audit_stamp_legs_complete,
    manifold_verify_upstream_gw_wrap_delegate, sec_gw_audit_accel2_ac31_honest,
    sec_gw_audit_accel2_ac31_probe, sec_gw_audit_manifold_probe,
    sec_gw_audit_production_wired, sec_gw_audit_trust_chain_next_hop,
    sec_gw_audit_wire_matrix, validate_sec_gw_audit_honesty,
    ADMIT_STAMP_PATH_COUNT, FLEET_ACCEL2_AC31_JOB_ID, GW_AUDIT_GREEN_CLAIM_BLOCKED,
    MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS, MANIFOLD_GW_AUDIT_STAMP_LEGS,
    MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS, SEC_GW_AUDIT_BOARD_SLICE_ID,
};

/// FLEET-COMPOSER ACCEL-K AC354 agent job id.
pub const FLEET_ACCEL2_AC354_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC354";

/// AC354 receipt path — SSOT for this pass.
pub const COMPOSER_ACCEL2_AC354_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC354.md";

/// Fleet verify command (scratch target dir).
pub const AC354_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac354-gw-audit cargo test -p umst-manifold sec_gw_audit -- --nocapture";

#[test]
fn sec_gw_audit_gate_transition_evidence_wired_on_manifold() {
    let probe = sec_gw_audit_manifold_probe();
    assert!(probe.gate_evidence_wired);
}

#[test]
fn sec_gw_audit_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_gw_audit_ceremony_closed());
    assert!(manifold_gw_audit_all_stamp_paths_probed());
    assert!(manifold_gw_audit_stamp_legs_complete());
    assert!(manifold_verify_upstream_gw_wrap_delegate());
    assert!(!sec_gw_audit_production_wired());
    assert!(GW_AUDIT_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_gw_audit_gate_wire_hops_six_of_eight_wired() {
    let wired = MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 6);
    assert_eq!(MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS.len(), 8);
}

#[test]
fn sec_gw_audit_admit_stamp_paths_four_of_four() {
    assert_eq!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS.len(), ADMIT_STAMP_PATH_COUNT);
    assert!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS.iter().all(|p| p.census_hit));
    assert!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS
        .iter()
        .any(|p| p.path_id == "material_mcp_delegate"));
}

#[test]
fn sec_gw_audit_stamp_legs_two_of_two() {
    assert_eq!(MANIFOLD_GW_AUDIT_STAMP_LEGS.len(), 2);
    assert!(manifold_gw_audit_stamp_legs_complete());
}

#[test]
fn sec_gw_audit_accel2_ac31_fleet_probe_honest() {
    let probe = sec_gw_audit_accel2_ac31_probe();
    assert_eq!(probe.ac31_job_id, FLEET_ACCEL2_AC31_JOB_ID);
    assert!(probe.prior_y51_absorbed);
    assert!(probe.prior_z87_absorbed);
    assert!(probe.ceremony_closed);
    assert!(sec_gw_audit_accel2_ac31_honest());
}

#[test]
fn sec_gw_audit_gate_census_validate_and_matrix() {
    validate_sec_gw_audit_honesty().expect("honest SEC-GW-AUDIT gate census");
    let census = gate_admit_audit_census();
    assert_eq!(census.board_slice_id, SEC_GW_AUDIT_BOARD_SLICE_ID);
    assert_eq!(census.wire_hop_wired_count, 6);
    assert_eq!(census.admit_stamp_path_count, ADMIT_STAMP_PATH_COUNT);
    assert_eq!(census.delegate_residual_path_count, 0);
    let probe = sec_gw_audit_manifold_probe();
    assert!(probe.ceremony_closed);
    let matrix = sec_gw_audit_wire_matrix();
    assert!(matrix.contains("wired=6/8"));
    assert!(matrix.contains("production_wired=false"));
    assert!(matrix.contains("delegate_residuals=0"));
}

#[test]
fn fleet_accel2_ac354_sec_gw_audit_admit_audit_honest() {
    assert_eq!(SEC_GW_AUDIT_BOARD_SLICE_ID, "SEC-GW-AUDIT");
    assert!(COMPOSER_ACCEL2_AC354_RECEIPT_PATH.contains("AC354"));
    assert!(AC354_VERIFY_COMMAND.contains("umst-accel2-ac354-gw-audit"));
    assert!(manifold_gate_sec_gw_audit_ceremony_closed());
    assert!(sec_gw_audit_accel2_ac31_honest());
    assert!(!sec_gw_audit_production_wired());
    assert!(sec_gw_audit_trust_chain_next_hop().contains("resolve_admit_warrant_from_env"));
    assert_eq!(FLEET_ACCEL2_AC354_JOB_ID, "FLEET-COMPOSER-ACCEL2-AC354");
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! SEC-S6 gate runtime witness — manifold HCOM prov gateway fence census on cold-edge evidence.
//!
//! FLEET-COMPOSER ACCEL-L **AC376** · verify-only · 0 gateway writers.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC376.md`.
//! Absorbs AC33 (`ACCEL_B_2050_AC33_JOB_ID`) · Z126 · H55 · J34 · AGAP-2033 prior receipts.

use umst_manifold::runtime::gate::{
    collect_sec_s6_gate_factor_rows, gate_hcom_prov_gateway_fence_census,
    manifold_gate_sec_s6_ceremony_closed, manifold_hcom_prov_gw_fence_hops_verified,
    manifold_s6_inspect_delegate_verified, manifold_scert_upstream_slots_verified,
    sec_s6_accel_ac33_honest, sec_s6_accel_ac33_probe, sec_s6_gate_factor_table,
    sec_s6_gate_manifold_probe, sec_s6_gate_transition_evidence_probe, sec_s6_gate_wire_matrix,
    sec_s6_hcom_prov_fence_table, sec_s6_hcom_prov_gw_next_hop, sec_s6_production_wired,
    sec_s6_scert_upstream_table, validate_sec_s6_gate_honesty, ACCEL_AC33_RECEIPT_PATH,
    ACCEL_B_2050_AC33_JOB_ID, HCOM_PROV_GW_FENCE_HOPS, HCOM_PROV_GW_WIRE_HOP_COUNT,
    LIVE_ATTESTATION_WIRED_HONEST, MANIFOLD_SEC_S6_GATE_WIRE_HOPS, S6_GREEN_CLAIM_BLOCKED,
    S6_INSPECT_FACTOR_COUNT, SCERT_EXIT_NOT_WIRED, SCERT_UPSTREAM_SLOTS, SEC_S6_BOARD_SLICE_ID,
    SEC_S6_EXPECTED_GATE_EXIT,
};

/// FLEET-COMPOSER ACCEL-L AC376 agent job id.
pub const FLEET_ACCEL2_AC376_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC376";

/// AC376 receipt path — SSOT for this pass.
pub const COMPOSER_ACCEL2_AC376_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC376.md";

/// Fleet verify command (scratch target dir).
pub const AC376_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac376-sec-s6 cargo test -p umst-manifold sec_s6 -- --nocapture";

#[test]
fn sec_s6_gate_transition_evidence_wired_on_manifold() {
    assert!(sec_s6_gate_transition_evidence_probe());
}

#[test]
fn sec_s6_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_gate_sec_s6_ceremony_closed());
    assert!(manifold_hcom_prov_gw_fence_hops_verified());
    assert!(manifold_scert_upstream_slots_verified());
    assert!(manifold_s6_inspect_delegate_verified());
    assert!(!sec_s6_production_wired());
    assert!(!LIVE_ATTESTATION_WIRED_HONEST);
    assert!(S6_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_s6_gate_wire_hops_six_of_seven_wired() {
    let wired = MANIFOLD_SEC_S6_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 6);
    assert_eq!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len(), 7);
    assert!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("s6_inspect") && h.wired));
    assert!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("hcom_prov_gw_production_wired") && !h.wired));
}

#[test]
fn sec_s6_accel_ac33_fleet_probe_honest() {
    let probe = sec_s6_accel_ac33_probe();
    assert_eq!(probe.ac33_job_id, ACCEL_B_2050_AC33_JOB_ID);
    assert!(probe.prior_2033_absorbed);
    assert!(probe.prior_z126_absorbed);
    assert!(probe.prior_h55_absorbed);
    assert!(probe.hcom_prov_fence_table_residue_pinned);
    assert!(probe.ceremony_closed);
    assert!(sec_s6_accel_ac33_honest());
}

#[test]
fn sec_s6_hcom_prov_fence_and_scert_upstream_honest() {
    assert_eq!(HCOM_PROV_GW_FENCE_HOPS.len(), HCOM_PROV_GW_WIRE_HOP_COUNT);
    assert_eq!(SCERT_UPSTREAM_SLOTS.len(), 4);
    assert_eq!(S6_INSPECT_FACTOR_COUNT, 6);
    assert_eq!(SCERT_EXIT_NOT_WIRED, 2);
    let fence_table = sec_s6_hcom_prov_fence_table();
    assert!(fence_table.contains("enforce_hcom_prov_semantic_admit"));
    assert!(fence_table.contains("verified=true"));
    let upstream_table = sec_s6_scert_upstream_table();
    assert!(upstream_table.contains("upstream_green=0/4"));
    assert!(upstream_table.contains("H-Arc"));
    assert!(upstream_table.contains("M-Arc"));
}

#[test]
fn sec_s6_gate_census_validate_and_matrix() {
    validate_sec_s6_gate_honesty().expect("honest SEC-S6 gate census");
    let census = gate_hcom_prov_gateway_fence_census();
    assert_eq!(census.board_slice_id, SEC_S6_BOARD_SLICE_ID);
    assert_eq!(census.wire_hop_wired_count, 6);
    assert!(census.hcom_prov_fence_hops_verified);
    assert!(census.scert_upstream_slots_verified);
    assert!(census.s6_inspect_delegate_verified);
    assert!(!census.live_attestation_wired);
    assert!(census.s6_green_claim_blocked);
    assert!(!census.production_wired);
    let probe = sec_s6_gate_manifold_probe();
    assert!(probe.ceremony_closed);
    assert_eq!(probe.wire_hop_wired_count, 6);
    let matrix = sec_s6_gate_wire_matrix();
    assert!(matrix.contains("wired=6/7"));
    assert!(matrix.contains("production_wired=false"));
    let factors = sec_s6_gate_factor_table();
    assert!(factors.contains("expected_gate_exit=BLOCKED"));
    assert!(factors.contains("scert_credit=BLOCKED"));
    let rows = collect_sec_s6_gate_factor_rows();
    assert_eq!(rows.len(), 6);
    assert!(rows.iter().all(|r| r.probe_wired));
    assert!(rows.iter().all(|r| !r.acceptance_credit));
}

#[test]
fn fleet_accel2_ac376_sec_s6_gate_witness_honest() {
    assert_eq!(SEC_S6_BOARD_SLICE_ID, "SEC-S6");
    assert_eq!(SEC_S6_EXPECTED_GATE_EXIT, "BLOCKED");
    assert_eq!(FLEET_ACCEL2_AC376_JOB_ID, "FLEET-COMPOSER-ACCEL2-AC376");
    assert!(COMPOSER_ACCEL2_AC376_RECEIPT_PATH.contains("AC376"));
    assert!(AC376_VERIFY_COMMAND.contains("umst-accel2-ac376-sec-s6"));
    assert!(ACCEL_AC33_RECEIPT_PATH.contains("AC33"));
    assert!(manifold_gate_sec_s6_ceremony_closed());
    assert!(sec_s6_accel_ac33_honest());
    assert!(!sec_s6_production_wired());
    assert_eq!(
        sec_s6_hcom_prov_gw_next_hop(),
        "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_production_wired"
    );
}

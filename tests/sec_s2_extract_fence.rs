// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SEC-S2 extract production fence integration witness — manifold `SEC-TRUST-EXTRACT` census.
//!
//! FLEET-COMPOSER ACCEL-K **AC353** · verify-only · 0 trust/gateway writers.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC353.md`.
//! Absorbs AC29 (`FLEET_ACCEL_AC29_JOB_ID`) · K2 · G73 · H52 · X48 prior receipts.

use umst_manifold::runtime::gate::{
    manifold_gate_sec_s2_ceremony_closed, manifold_s2_extract_fence_facets_verified,
    sec_s2_accel_ac29_honest, sec_s2_accel_ac29_probe, sec_s2_extract_fence_wired_count,
    sec_s2_extract_production_fence_matrix, sec_s2_extract_production_fence_next_hop,
    sec_s2_production_wired, sec_s2_trust_extract_production_wired, validate_sec_s2_gate_honesty,
    EXTRACT_SSOT, FLEET_ACCEL_AC29_JOB_ID, FLEET_ACCEL_AC29_RECEIPT_PATH,
    MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS, S2_EXTRACT_FENCE_FACET_COUNT,
    S2_EXTRACT_FENCE_FACET_IDS, S2_EXTRACT_FENCE_WIRED_COUNT, S2_GREEN_CLAIM_BLOCKED,
    SEC_S2_BOARD_SLICE_ID, TRUST_ADT_SSOT, UCRS_WIRE_PARITY_TEST,
};

/// FLEET-COMPOSER ACCEL-K AC353 agent job id.
pub const FLEET_ACCEL2_AC353_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC353";

/// AC353 receipt path — SSOT for this pass.
pub const COMPOSER_ACCEL2_AC353_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC353.md";

/// Fleet verify command (scratch target dir).
pub const AC353_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac353-sec-s2 cargo test sec_s2_extract_fence -- --nocapture";

#[test]
fn sec_s2_extract_fence_board_slice_and_ssot_pins() {
    assert_eq!(SEC_S2_BOARD_SLICE_ID, "SEC-S2");
    assert!(EXTRACT_SSOT.contains("sec_ecosystem_extract.rs"));
    assert!(TRUST_ADT_SSOT.contains("crypto/trust.rs"));
    assert!(UCRS_WIRE_PARITY_TEST.contains("s1_trust_ucrs_wire_parity.rs"));
}

#[test]
fn sec_s2_extract_fence_facet_inventory_seven_facets() {
    assert_eq!(
        MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS.len(),
        S2_EXTRACT_FENCE_FACET_COUNT
    );
    assert_eq!(S2_EXTRACT_FENCE_FACET_IDS.len(), S2_EXTRACT_FENCE_FACET_COUNT);
    for facet_id in S2_EXTRACT_FENCE_FACET_IDS {
        assert!(MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
            .iter()
            .any(|f| f.facet == *facet_id));
    }
}

#[test]
fn sec_s2_extract_fence_facets_five_of_seven_wired() {
    let wired = MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count();
    assert_eq!(wired, S2_EXTRACT_FENCE_WIRED_COUNT);
    assert_eq!(sec_s2_extract_fence_wired_count(), S2_EXTRACT_FENCE_WIRED_COUNT);
    assert!(manifold_s2_extract_fence_facets_verified());
}

#[test]
fn sec_s2_extract_fence_residue_facets_session_ledger_and_production() {
    let session = MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
        .iter()
        .find(|f| f.facet == "session_ledger")
        .expect("session_ledger facet");
    let production = MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS
        .iter()
        .find(|f| f.facet == "production_wired")
        .expect("production_wired facet");
    assert!(!session.wired);
    assert!(!production.wired);
    assert_eq!(session.owning_slice, "SEC-S3");
    assert_eq!(production.owning_slice, "SEC-GW-WRAP");
}

#[test]
fn sec_s2_extract_production_fence_matrix_honest_posture() {
    let matrix = sec_s2_extract_production_fence_matrix();
    assert!(matrix.contains("SEC-S2 extract production fence"));
    assert!(matrix.contains("facets_wired=5/7"));
    assert!(matrix.contains("trust_extract_production_wired=false"));
    assert!(matrix.contains("session_ledger_wired=false"));
    assert!(matrix.contains("s1_green_claimed=false"));
    assert!(matrix.contains("core_adt_ssot"));
    assert!(matrix.contains("production_wired wired=false"));
}

#[test]
fn sec_s2_extract_production_fence_next_hop_session_ledger() {
    assert_eq!(
        sec_s2_extract_production_fence_next_hop(),
        "umst-foundations/crates/umst-trust/src/sec_ecosystem_extract.rs:session_ledger_wired"
    );
}

#[test]
fn sec_s2_extract_production_wired_honest_false() {
    assert!(!sec_s2_trust_extract_production_wired());
    assert!(!sec_s2_production_wired());
    assert!(S2_GREEN_CLAIM_BLOCKED);
}

#[test]
fn sec_s2_accel_ac29_extract_fence_fleet_probe_honest() {
    let probe = sec_s2_accel_ac29_probe();
    assert_eq!(probe.ac29_job_id, FLEET_ACCEL_AC29_JOB_ID);
    assert!(probe.prior_k2_absorbed);
    assert!(probe.prior_g73_absorbed);
    assert!(probe.prior_h52_absorbed);
    assert!(probe.prior_x48_absorbed);
    assert!(probe.extract_fence_matrix_verified);
    assert!(probe.ceremony_closed);
    assert!(!probe.production_wired);
    assert!(probe.trust_extract_production_wired_honest_false);
    assert_eq!(probe.extract_wired_facet_count, S2_EXTRACT_FENCE_WIRED_COUNT);
    assert!(sec_s2_accel_ac29_honest());
    assert!(!sec_s2_trust_extract_production_wired());
}

#[test]
fn sec_s2_extract_fence_validate_gate_honesty_residue() {
    validate_sec_s2_gate_honesty().expect("honest SEC-S2 gate census includes extract fence");
    assert!(manifold_gate_sec_s2_ceremony_closed());
    assert!(manifold_s2_extract_fence_facets_verified());
}

#[test]
fn fleet_accel2_ac353_sec_s2_extract_fence_honest() {
    assert_eq!(SEC_S2_BOARD_SLICE_ID, "SEC-S2");
    assert!(COMPOSER_ACCEL2_AC353_RECEIPT_PATH.contains("AC353"));
    assert!(AC353_VERIFY_COMMAND.contains("umst-accel2-ac353-sec-s2"));
    assert!(FLEET_ACCEL_AC29_RECEIPT_PATH.contains("AC29"));
    assert!(manifold_gate_sec_s2_ceremony_closed());
    assert!(manifold_s2_extract_fence_facets_verified());
    assert!(sec_s2_accel_ac29_honest());
    assert!(!sec_s2_trust_extract_production_wired());
    assert_eq!(FLEET_ACCEL2_AC353_JOB_ID, "FLEET-COMPOSER-ACCEL2-AC353");
}

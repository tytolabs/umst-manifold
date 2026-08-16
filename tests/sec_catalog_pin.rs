// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! SEC-CATALOG pin runtime witness — manifold catalog lock ceremony on cold-edge evidence.
//!
//! FLEET-COMPOSER ACCEL-L **AC384** · verify-only · 0 gateway writers.
//! Receipt SSOT: `outputs/.tmp/COMPOSER_ACCEL2_AC384.md`.
//! Absorbs AC30 (`FLEET_ACCEL2_AC30_RECEIPT_PATH`) · P1542 A2 · Z39 prior receipts.

use umst_manifold::runtime::catalog::catalog_pin::{
    catalog_pin_accel_ac30_honest, catalog_pin_accel_ac30_probe, FLEET_ACCEL2_AC30_JOB_ID,
    FLEET_ACCEL2_AC30_RECEIPT_PATH,
};
use umst_manifold::runtime::catalog::sec_catalog_pin::FLEET_Z39_RECEIPT_PATH;
use umst_manifold::runtime::catalog::{
    bundled_catalog_lock_json, catalog_lock_bundle_sha256_hex, catalog_pin_manifold_probe,
    catalog_pin_manifold_wired, catalog_pin_production_wired, lock_bundle_content_address_hex,
    lock_upstream_catalog_digest_hex, manifold_catalog_pin_ceremony_closed, pin_witness_ok,
    resolve_catalog_digest, sec_catalog_pin_p1542_a2_honest, sec_catalog_pin_p1542_a2_probe,
    verify_ssot_catalog_digest_hex, CatalogDigestAttachMode, CATALOG_PIN_BOARD_SLICE_ID,
    EXPECTED_MODULE_COUNT, EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX, FLEET_P1542_A2_JOB_ID,
    FLEET_P1542_A2_RECEIPT_PATH, JOB_ID, MANIFOLD_CATALOG_PIN_WIRE_HOPS,
};

/// FLEET-COMPOSER ACCEL-L AC384 agent job id.
pub const FLEET_ACCEL2_AC384_JOB_ID: &str = "FLEET-COMPOSER-ACCEL2-AC384";

/// AC384 receipt path — SSOT for this pass.
pub const COMPOSER_ACCEL2_AC384_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC384.md";

/// Fleet verify command (scratch target dir).
pub const AC384_VERIFY_COMMAND: &str =
    "CARGO_TARGET_DIR=/tmp/umst-accel2-ac384-seccat cargo test -p umst-manifold sec_catalog_pin -- --nocapture";

#[test]
fn sec_catalog_pin_digest_matches_ssot() {
    assert_eq!(
        lock_upstream_catalog_digest_hex(),
        EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX
    );
    verify_ssot_catalog_digest_hex().expect("SSOT digest hex");
    pin_witness_ok().expect("pin_witness_ok");
}

#[test]
fn sec_catalog_pin_module_count_is_129() {
    let json = bundled_catalog_lock_json();
    assert!(json.contains(&format!("\"module_count\": {EXPECTED_MODULE_COUNT}")));
}

#[test]
fn sec_catalog_pin_lock_bundle_content_address_matches_build_pin() {
    assert_eq!(
        lock_bundle_content_address_hex(),
        catalog_lock_bundle_sha256_hex(),
        "bundled lock SHA-256 must match build.rs UMST_CATALOG_LOCK_SHA256_HEX"
    );
}

#[test]
fn sec_catalog_pin_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_catalog_pin_ceremony_closed());
    assert!(catalog_pin_manifold_wired());
    assert!(!catalog_pin_production_wired());
    assert!(resolve_catalog_digest(CatalogDigestAttachMode::FromBuildLock).is_none());
    assert!(resolve_catalog_digest(CatalogDigestAttachMode::Unattached).is_none());
}

#[test]
fn sec_catalog_pin_wire_hops_four_of_five_wired() {
    let wired = MANIFOLD_CATALOG_PIN_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    assert_eq!(wired, 4);
    assert_eq!(MANIFOLD_CATALOG_PIN_WIRE_HOPS.len(), 5);
    assert!(MANIFOLD_CATALOG_PIN_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("umst-gateway") && !h.wired));
    assert!(MANIFOLD_CATALOG_PIN_WIRE_HOPS
        .iter()
        .any(|h| h.surface.contains("pin_witness_ok") && h.wired));
}

#[test]
fn sec_catalog_pin_accel_ac30_fleet_probe_honest() {
    let probe = catalog_pin_accel_ac30_probe();
    assert_eq!(probe.ac30_job_id, FLEET_ACCEL2_AC30_JOB_ID);
    assert!(probe.prior_p1542_a2_absorbed);
    assert!(probe.prior_z39_absorbed);
    assert!(probe.prior_y50_absorbed);
    assert!(probe.prior_agap_2350_absorbed);
    assert!(probe.ceremony_closed);
    assert!(probe.probe.lock_witness_ok);
    assert!(!probe.gateway_production_wired);
    assert!(catalog_pin_accel_ac30_honest());
}

#[test]
fn sec_catalog_pin_p1542_a2_fleet_probe_honest() {
    let probe = sec_catalog_pin_p1542_a2_probe();
    assert_eq!(probe.a2_job_id, FLEET_P1542_A2_JOB_ID);
    assert!(probe.z39_trust_absorbed);
    assert!(probe.ceremony_closed);
    assert!(probe.probe.ssot_digest_ok);
    assert!(probe.probe.lock_witness_ok);
    assert!(!probe.production_wired);
    assert!(probe.attach_none);
    assert!(sec_catalog_pin_p1542_a2_honest());
}

#[test]
fn sec_catalog_pin_manifold_probe_census() {
    let probe = catalog_pin_manifold_probe();
    assert!(probe.ssot_digest_ok);
    assert!(probe.lock_witness_ok);
    assert!(probe.sha3_catalog_adopted);
    assert!(probe.manifold_adopt_wired);
    assert_eq!(probe.wire_hop_wired_count, 4);
    assert!(!probe.production_wired);
    assert!(!probe.catalog_digest_attached);
}

#[test]
fn fleet_accel2_ac384_sec_catalog_pin_honest() {
    assert_eq!(CATALOG_PIN_BOARD_SLICE_ID, "SEC-CATALOG-PIN");
    assert_eq!(JOB_ID, "AGAP-2350-SEC-CATALOG-PIN");
    assert_eq!(FLEET_ACCEL2_AC384_JOB_ID, "FLEET-COMPOSER-ACCEL2-AC384");
    assert!(COMPOSER_ACCEL2_AC384_RECEIPT_PATH.contains("AC384"));
    assert!(AC384_VERIFY_COMMAND.contains("umst-accel2-ac384-seccat"));
    assert!(FLEET_ACCEL2_AC30_RECEIPT_PATH.contains("AC30"));
    assert!(FLEET_P1542_A2_RECEIPT_PATH.contains("P1542_A2"));
    assert!(FLEET_Z39_RECEIPT_PATH.contains("Z39_1015"));
    assert!(manifold_catalog_pin_ceremony_closed());
    assert!(catalog_pin_accel_ac30_honest());
    assert!(sec_catalog_pin_p1542_a2_honest());
    assert!(!catalog_pin_production_wired());
}

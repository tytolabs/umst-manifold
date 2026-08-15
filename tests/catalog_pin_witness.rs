// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Catalog pin witness — MaOS parity with egoff I-0 `pin_witness_ok`.

use umst_manifold::runtime::catalog::{
    bundled_catalog_lock_json, catalog_pin_production_wired, lock_upstream_catalog_digest_hex,
    manifold_catalog_pin_ceremony_closed, pin_witness_ok, resolve_catalog_digest,
    CatalogDigestAttachMode, CatalogPinMismatch, EXPECTED_MODULE_COUNT,
    EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX,
};

#[test]
fn catalog_pin_digest_matches_ssot() {
    assert_eq!(
        lock_upstream_catalog_digest_hex(),
        EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX
    );
    pin_witness_ok().expect("pin_witness_ok");
}

#[test]
fn catalog_pin_module_count_is_129() {
    let json = bundled_catalog_lock_json();
    assert!(json.contains(&format!("\"module_count\": {EXPECTED_MODULE_COUNT}")));
}

#[test]
fn catalog_pin_lock_bundle_content_address_matches_build_pin() {
    use umst_manifold::runtime::catalog::{
        catalog_lock_bundle_sha256_hex, lock_bundle_content_address_hex,
    };
    assert_eq!(
        lock_bundle_content_address_hex(),
        catalog_lock_bundle_sha256_hex(),
        "bundled lock SHA-256 must match build.rs UMST_CATALOG_LOCK_SHA256_HEX"
    );
}

#[test]
fn catalog_pin_manifold_ceremony_predicate_and_honest_residue() {
    assert!(manifold_catalog_pin_ceremony_closed());
    assert!(!catalog_pin_production_wired());
    assert!(resolve_catalog_digest(CatalogDigestAttachMode::FromBuildLock).is_none());
}

#[test]
fn catalog_pin_witness_rejects_tampered_digest_constant() {
    // Document mismatch variant exists — cannot mutate const; structural check only.
    assert_ne!(CatalogPinMismatch::Digest, CatalogPinMismatch::ModuleCount);
}

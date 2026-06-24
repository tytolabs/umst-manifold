// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use super::{
    catalog_lock_bundle_sha256_bytes, catalog_lock_bundle_sha256_hex, catalog_lock_quickcheck,
    lock_upstream_catalog_digest_hex, witness_catalog_quickcheck_ok, CatalogLock, WitnessCatalog,
    WitnessRecord, ENV_WITNESS_CATALOG_PATH, WITNESS_CATALOG_EMBEDDED_LEN,
    WITNESS_CATALOG_EMBEDDED_SHA256_HEX,
};
use std::sync::Mutex;

/// Serialize env mutations touching [`ENV_WITNESS_CATALOG_PATH`].
static CATALOG_ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn embedded_witness_catalog_parses() {
    let cat = WitnessCatalog::from_embedded().expect("embedded witness envelope parses");
    assert_eq!(cat.version, 1);
    assert!(cat.witnesses.iter().any(|w| w.id.contains("builtin")));
}

#[test]
fn lock_bundle_bytes_match_hex_digest() {
    let hex = catalog_lock_bundle_sha256_hex();
    let bytes = catalog_lock_bundle_sha256_bytes();
    assert_eq!(hex.len(), 64);
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hi = chunk[0];
        let lo = chunk[1];
        let nibble = |c: u8| match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => panic!("expected lowercase hex from build.rs"),
        };
        assert_eq!(bytes[i], (nibble(hi) << 4) | nibble(lo));
    }
}

#[test]
fn bundled_lock_matches_build_digest_semantics() {
    let lock = CatalogLock::from_bundled().expect("bundled catalog.lock.json parses");
    assert!(
        lock.composed_digest_hex().is_some(),
        "expected composed or upstream digest in pinned lock"
    );
    assert!(catalog_lock_quickcheck(&lock), "v2 dual-pin invariants");
    assert!(
        lock.version >= 2 && !lock.fiber_pins.is_empty(),
        "production lock should be v2 with fiber_pins"
    );
    let upstream = lock.upstream_catalog_digest_hex.as_deref().unwrap();
    let composed = lock.composed_catalog_digest_hex.as_deref().unwrap();
    assert_eq!(
        upstream, composed,
        "composed_catalog_digest_hex must equal upstream_catalog_digest_hex"
    );
    assert_eq!(
        lock_upstream_catalog_digest_hex(),
        upstream,
        "build.rs must emit UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX from lock JSON"
    );
    assert_eq!(lock.module_count, Some(122));
    assert!(
        catalog_lock_bundle_sha256_hex().len() == 64,
        "expected 64-char hex fingerprint for lock-bundle digest",
    );
}

#[test]
fn v1_monolith_lock_quickcheck_backward_compat() {
    let v1 = CatalogLock {
        version: 1,
        role: "manifold_runtime_lock".into(),
        upstream_repo: Some("umst-formal-double-slit".into()),
        upstream_catalog_digest_hex: Some(
            "ef0ed071fc82bf8ebc8971aeee8d142b4b54e15583f0c575d942cb237474d1dc".into(),
        ),
        module_count: Some(119),
        composition_rule: None,
        composed_catalog_digest_hex: None,
        fiber_pins: vec![],
    };
    assert!(
        catalog_lock_quickcheck(&v1),
        "v1 monolith without fiber_pins"
    );
}

#[test]
fn v2_dual_pin_per_fiber_digests_present() {
    let lock = CatalogLock::from_bundled().expect("bundled lock");
    assert_eq!(lock.fiber_pins.len(), 3);
    let repos: Vec<_> = lock.fiber_pins.iter().map(|p| p.repo.as_str()).collect();
    assert!(repos.contains(&"umst-formal-double-slit"));
    assert!(repos.contains(&"umst-formal"));
    assert!(repos.contains(&"umst-ucrs"));
    for pin in &lock.fiber_pins {
        assert_eq!(pin.catalog_digest_hex.len(), 64);
    }
}

#[test]
fn composed_digest_covers_non_preview_fibers() {
    let lock = CatalogLock::from_bundled().expect("bundled lock");
    let non_preview: Vec<_> = lock
        .fiber_pins
        .iter()
        .filter(|pin| {
            let role = pin.lock_role.as_deref().unwrap_or("").to_ascii_lowercase();
            !role.contains("preview") && !role.contains("track_f")
        })
        .collect();
    assert!(
        !non_preview.is_empty(),
        "production lock should have non-preview fiber pins"
    );
    let composed = lock
        .composed_catalog_digest_hex
        .as_deref()
        .or(lock.upstream_catalog_digest_hex.as_deref())
        .expect("composed or upstream digest");
    assert_eq!(composed.len(), 64);
    assert_eq!(
        lock.composed_catalog_digest_hex.as_deref(),
        lock.upstream_catalog_digest_hex.as_deref(),
        "composed_catalog_digest_hex must equal upstream for v2 dual-pin"
    );
}

#[test]
fn witness_quickcheck_reports_coherent_bundle() {
    assert!(witness_catalog_quickcheck_ok());
}

#[test]
fn embedded_constant_surface_matches_helpers() {
    assert_eq!(
        WitnessCatalog::embedded_bundle_sha256_hex(),
        WITNESS_CATALOG_EMBEDDED_SHA256_HEX
    );
    assert_eq!(WitnessCatalog::embedded_len(), WITNESS_CATALOG_EMBEDDED_LEN);
}

#[test]
fn embedded_digest_is_lower_hex_64_chars() {
    let h = WITNESS_CATALOG_EMBEDDED_SHA256_HEX;
    assert_eq!(h.len(), 64);
    assert!(h.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')));
}

#[test]
fn rejects_invalid_json() {
    let err = serde_json::from_slice::<WitnessCatalog>(b"not-json").unwrap_err();
    assert!(
        matches!(err.classify(), serde_json::error::Category::Syntax),
        "{err:?}"
    );
}

#[test]
fn roundtrip_via_vec() {
    let cat = WitnessCatalog {
        version: 1,
        witnesses: vec![WitnessRecord {
            id: "test.roundtrip".into(),
            description: Some("q".into()),
        }],
    };
    let js = serde_json::to_vec(&cat).unwrap();
    let back: WitnessCatalog = serde_json::from_slice(&js).unwrap();
    assert_eq!(cat, back);
}

#[test]
fn catalog_path_override_reads_file() {
    let _guard = CATALOG_ENV_LOCK.lock().expect("catalog env mutex");
    let path = std::env::temp_dir().join(format!(
        "witness_catalog_umst_manifold_override_{}.json",
        std::process::id()
    ));
    std::fs::write(
        &path,
        br#"{"version":1,"witnesses":[{"id":"from.temp.file","description":null}]}"#,
    )
    .expect("write temp catalog");

    std::env::set_var(ENV_WITNESS_CATALOG_PATH, &path);
    let loaded = WitnessCatalog::load_default().expect("UMST_CATALOG_PATH load");
    assert!(loaded.witnesses.iter().any(|w| w.id == "from.temp.file"));

    std::env::remove_var(ENV_WITNESS_CATALOG_PATH);
    std::fs::remove_file(path).ok();
}

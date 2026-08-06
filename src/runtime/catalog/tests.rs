// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use super::{
    bundled_catalog_lock_json, bundled_semantic_witness_section_present,
    catalog_lock_bundle_sha256_bytes, catalog_lock_bundle_sha256_hex, catalog_lock_quickcheck,
    is_preview_fiber_pin, lock_bundle_content_address_hex, lock_upstream_catalog_digest_bytes,
    lock_upstream_catalog_digest_hex, lookup_bundled_semantic_cold_witness,
    semantic_witness_section_quickcheck, witness_catalog_quickcheck_ok, CatalogFiberPin,
    CatalogLoadError, CatalogLock, WitnessCatalog, WitnessRecord, CATALOG_SCHEMA_STUB_REVISION,
    DEFAULT_SEMANTIC_COLD_WITNESS_ID, ENV_WITNESS_CATALOG_PATH, EXPECTED_MODULE_COUNT,
    EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX, WITNESS_CATALOG_EMBEDDED_LEN,
    WITNESS_CATALOG_EMBEDDED_SHA256_HEX,
};
use std::path::Path;
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
    let upstream = lock
        .upstream_catalog_digest_hex
        .as_deref()
        .expect("bundled v2 lock must carry upstream_catalog_digest_hex");
    let composed = lock
        .composed_catalog_digest_hex
        .as_deref()
        .expect("bundled v2 lock must carry composed_catalog_digest_hex");
    assert_eq!(
        upstream, composed,
        "composed_catalog_digest_hex must equal upstream_catalog_digest_hex"
    );
    assert_eq!(
        lock_upstream_catalog_digest_hex(),
        upstream,
        "build.rs must emit UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX from lock JSON"
    );
    assert_eq!(lock.module_count, Some(129));
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
        composed_primary_fiber_fingerprint_hex: None,
        fiber_pins: vec![],
        semantic_witnesses: None,
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
fn ucrs_preview_fiber_excluded_from_composed_digest() {
    let lock = CatalogLock::from_bundled().expect("bundled lock");
    let ucrs = lock
        .fiber_pins
        .iter()
        .find(|pin| pin.repo == "umst-ucrs")
        .expect("umst-ucrs preview fiber pin");
    assert!(
        is_preview_fiber_pin(ucrs),
        "umst-ucrs lock_role must mark preview (preview or track_f substring)"
    );
    let non_preview: Vec<_> = lock
        .fiber_pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin(pin))
        .collect();
    assert_eq!(
        non_preview.len(),
        2,
        "composed digest covers primary fibers only"
    );
    assert!(
        !non_preview.iter().any(|pin| pin.repo == "umst-ucrs"),
        "umst-ucrs must not contribute to composed_catalog_digest_hex"
    );
    assert!(
        ucrs.commit_stamp.is_none(),
        "commit_stamp is optional until witnessed commit egress"
    );
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
fn catalog_pin_manifold_ceremony_closed_on_bundled_lock() {
    use super::sec_catalog_pin::{
        catalog_pin_production_wired, manifold_catalog_pin_ceremony_closed,
        resolve_catalog_digest, CatalogDigestAttachMode,
    };
    assert!(manifold_catalog_pin_ceremony_closed());
    assert!(!catalog_pin_production_wired());
    assert!(resolve_catalog_digest(CatalogDigestAttachMode::Unattached).is_none());
}

#[test]
fn pin_witness_ok_passes_on_bundled_lock() {
    use super::catalog_pin::pin_witness_ok;
    pin_witness_ok().expect("pin_witness_ok on bundled lock");
}

#[test]
fn catalog_sha3_pin_witness_roundtrip() {
    use super::catalog_pin::catalog_sha3_pin_witness_ok;
    use umst_algebra::crypto::hash::{digest_hex, HashPolicy};
    let preimage = b"manifold-catalog-export";
    let hex = digest_hex(HashPolicy::Sha3Catalog, preimage).expect("digest_hex");
    catalog_sha3_pin_witness_ok(&hex, preimage).expect("sha3 catalog pin");
}

#[test]
fn composed_fiber_fingerprint_guard_holds_on_bundled_lock() {
    use super::catalog_pin::composed_fiber_fingerprint_guard_holds;
    let lock = CatalogLock::from_bundled().expect("bundled lock");
    assert!(
        composed_fiber_fingerprint_guard_holds(&lock),
        "composed_primary_fiber_fingerprint_hex must match non-preview fiber SHA-256"
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
    let js = serde_json::to_vec(&cat).expect("WitnessCatalog roundtrip serializes to JSON");
    let back: WitnessCatalog =
        serde_json::from_slice(&js).expect("WitnessCatalog roundtrip deserializes from JSON");
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

#[test]
fn load_default_without_override_uses_embedded() {
    let _guard = CATALOG_ENV_LOCK.lock().expect("catalog env mutex");
    std::env::remove_var(ENV_WITNESS_CATALOG_PATH);
    let loaded = WitnessCatalog::load_default().expect("embedded default");
    let embedded = WitnessCatalog::from_embedded().expect("from_embedded");
    assert_eq!(loaded, embedded);
}

#[test]
fn from_path_missing_file_is_io_error() {
    let missing = Path::new("/tmp/umst_manifold_catalog_missing_does_not_exist.json");
    let err = WitnessCatalog::from_path(missing).expect_err("missing path");
    assert!(
        matches!(err, CatalogLoadError::Io(_)),
        "expected Io CatalogLoadError, got {err}"
    );
}

#[test]
fn upstream_digest_bytes_match_hex() {
    let hex = lock_upstream_catalog_digest_hex();
    let bytes = lock_upstream_catalog_digest_bytes();
    assert_eq!(hex.len(), 64);
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let nibble = |c: u8| match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => panic!("expected lowercase hex digest"),
        };
        assert_eq!(bytes[i], (nibble(chunk[0]) << 4) | nibble(chunk[1]));
    }
}

#[test]
fn bundled_lock_json_parses_identically() {
    let from_str: CatalogLock =
        serde_json::from_str(bundled_catalog_lock_json()).expect("bundled JSON");
    let from_api = CatalogLock::from_bundled().expect("from_bundled");
    assert_eq!(from_str, from_api);
}

#[test]
fn schema_stub_revision_is_v1() {
    assert_eq!(CATALOG_SCHEMA_STUB_REVISION, "catalog.schema.stub.v1");
}

#[test]
fn expected_pin_constants_match_bundled_lock() {
    let lock = CatalogLock::from_bundled().expect("bundled lock");
    assert_eq!(lock.module_count, Some(EXPECTED_MODULE_COUNT));
    assert_eq!(
        lock.upstream_catalog_digest_hex.as_deref(),
        Some(EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX)
    );
    assert_eq!(
        lock_upstream_catalog_digest_hex(),
        EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX
    );
}

#[test]
fn lock_bundle_content_address_matches_build_hex() {
    assert_eq!(
        lock_bundle_content_address_hex(),
        catalog_lock_bundle_sha256_hex()
    );
}

#[test]
fn preview_fiber_role_heuristics() {
    let preview = CatalogFiberPin {
        repo: "umst-ucrs".into(),
        catalog_digest_hex: "a".repeat(64),
        module_count: 1,
        lock_role: Some("preview_fiber".into()),
        catalog_path: None,
        commit_stamp: None,
    };
    let track_f = CatalogFiberPin {
        lock_role: Some("Track_F_audit".into()),
        ..preview.clone()
    };
    let primary = CatalogFiberPin {
        lock_role: Some("primary".into()),
        ..preview.clone()
    };
    assert!(is_preview_fiber_pin(&preview));
    assert!(is_preview_fiber_pin(&track_f));
    assert!(!is_preview_fiber_pin(&primary));
}

#[test]
fn catalog_lock_quickcheck_refuses_honest_fences() {
    let good = CatalogLock::from_bundled().expect("bundled");
    assert!(catalog_lock_quickcheck(&good));

    let mut bad_role = good.clone();
    bad_role.role = "not_manifold_runtime_lock".into();
    assert!(!catalog_lock_quickcheck(&bad_role));

    let mut mismatch = good.clone();
    mismatch.composed_catalog_digest_hex = Some("b".repeat(64));
    assert!(!catalog_lock_quickcheck(&mismatch));

    let mut empty_repo = good.clone();
    empty_repo.fiber_pins[0].repo.clear();
    assert!(!catalog_lock_quickcheck(&empty_repo));

    let mut bad_version = good.clone();
    bad_version.version = 0;
    assert!(!catalog_lock_quickcheck(&bad_version));
}

#[test]
fn bundled_semantic_witness_surface_present_and_lookupable() {
    assert!(bundled_semantic_witness_section_present());
    let lock = CatalogLock::from_bundled().expect("bundled");
    let section = lock
        .semantic_witnesses
        .as_ref()
        .expect("semantic_witnesses section on bundled lock");
    assert!(semantic_witness_section_quickcheck(section));
    assert!(
        lookup_bundled_semantic_cold_witness(DEFAULT_SEMANTIC_COLD_WITNESS_ID).is_some(),
        "default cold witness must resolve from bundled lock"
    );
    assert!(lookup_bundled_semantic_cold_witness("nonexistent.witness.id").is_none());
}

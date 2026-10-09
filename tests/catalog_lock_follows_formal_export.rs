// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `artifacts/catalog.lock.json` pins the pinned formal export `artifacts/upstream_catalog.json`
//! (umst-formal-double-slit merged catalog at the `.umst-pins.toml` SHA): digest, module rows and edge rows.
//! Both files are read from disk here, and the compiled `FORMAL_CATALOG_*` / `EXPECTED_*` constants must
//! equal what the files say, so neither side can carry a stale literal.

use std::fs;
use std::path::PathBuf;

use umst_manifold::runtime::catalog::{
    traceability::TRACEABILITY_R0_MODULE_COUNT, EXPECTED_MODULE_COUNT,
    EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX, FORMAL_CATALOG_DIGEST_HEX, FORMAL_CATALOG_MODULE_COUNT,
    FORMAL_CATALOG_MODULE_GRAPH_EDGE_COUNT,
};

fn read_json(rel: &str) -> serde_json::Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn rows(doc: &serde_json::Value, key: &str) -> u64 {
    doc.get(key)
        .and_then(|v| v.as_array())
        .map(|a| a.len() as u64)
        .unwrap_or_else(|| panic!("formal export has no {key} array"))
}

fn field<'a>(doc: &'a serde_json::Value, key: &str) -> &'a serde_json::Value {
    doc.get(key)
        .unwrap_or_else(|| panic!("catalog.lock.json has no {key}"))
}

#[test]
fn catalog_lock_pins_the_formal_export_on_disk() {
    let lock = read_json("artifacts/catalog.lock.json");
    let export = read_json("artifacts/upstream_catalog.json");
    let export_digest = export
        .get("digest")
        .and_then(|v| v.as_str())
        .expect("formal export has a digest");

    assert_eq!(
        field(&lock, "upstream_catalog_digest_hex").as_str(),
        Some(export_digest)
    );
    assert_eq!(
        field(&lock, "composed_catalog_digest_hex").as_str(),
        Some(export_digest)
    );
    assert_eq!(
        field(&lock, "module_count").as_u64(),
        Some(rows(&export, "modules"))
    );
    assert_eq!(
        field(&lock, "module_graph_edge_count").as_u64(),
        Some(rows(&export, "module_graph_edges"))
    );
}

#[test]
fn compiled_catalog_pin_constants_follow_the_files_on_disk() {
    let lock = read_json("artifacts/catalog.lock.json");
    let export = read_json("artifacts/upstream_catalog.json");

    assert_eq!(
        export.get("digest").and_then(|v| v.as_str()),
        Some(FORMAL_CATALOG_DIGEST_HEX)
    );
    assert_eq!(
        rows(&export, "modules"),
        u64::from(FORMAL_CATALOG_MODULE_COUNT)
    );
    assert_eq!(
        rows(&export, "module_graph_edges"),
        u64::from(FORMAL_CATALOG_MODULE_GRAPH_EDGE_COUNT)
    );

    assert_eq!(
        field(&lock, "upstream_catalog_digest_hex").as_str(),
        Some(EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX)
    );
    assert_eq!(
        field(&lock, "module_count").as_u64(),
        Some(u64::from(EXPECTED_MODULE_COUNT))
    );
    assert_eq!(
        field(&lock, "module_count").as_u64(),
        Some(TRACEABILITY_R0_MODULE_COUNT as u64)
    );
}

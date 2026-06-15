// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Incremental catalog drift: lock pins `module_graph_edge_count` for the unified export DAG.

use std::fs;
use std::path::PathBuf;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn catalog_lock_pins_module_graph_edge_count_for_unified_export() {
    let lock_path = manifest_dir().join("artifacts/catalog.lock.json");
    let lock_raw = fs::read_to_string(&lock_path).unwrap_or_else(|e| {
        panic!("read catalog.lock.json at {}: {e}", lock_path.display());
    });
    let lock: serde_json::Value =
        serde_json::from_str(&lock_raw).expect("catalog.lock.json must be valid JSON");
    let pinned = lock
        .get("module_graph_edge_count")
        .and_then(|v| v.as_u64())
        .expect("catalog.lock.json must declare module_graph_edge_count");
    assert_eq!(
        pinned, 329,
        "bump module_graph_edge_count after Lean import-graph churn (regen export first)"
    );
}

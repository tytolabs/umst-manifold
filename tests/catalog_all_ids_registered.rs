// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Every Lean `catalog.json` module id is wired to a runtime `catalog_id`, witness id, or
//! [`umst_manifold::runtime::catalog::traceability::ALLOW_UNUSED_CATALOG_IDS`].
//!
//! Every [`GateEvaluator`] registry `catalog_id` is backed by a wired Lean module or
//! [`ALLOW_UNUSED_GATE_CATALOG_IDS`].
//!
//! See `docs/CATALOG_TRACEABILITY.md`.

use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use umst_manifold::runtime::catalog::{
    traceability::{
        ALLOW_UNUSED_CATALOG_IDS, ALLOW_UNUSED_GATE_CATALOG_IDS, CATALOG_MODULE_WIRED,
        DEFAULT_UPSTREAM_CATALOG_JSON, GATE_REGISTRY_CATALOG_IDS,
        GATE_UNIFICATION_SPEC_CATALOG_IDS, RUNTIME_EXTRA_GATE_CATALOG_IDS,
    },
    WitnessCatalog,
};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn resolve_upstream_catalog_json() -> PathBuf {
    if let Ok(p) = std::env::var("UMST_LEAN_CATALOG_JSON") {
        return PathBuf::from(p);
    }
    manifest_dir().join(DEFAULT_UPSTREAM_CATALOG_JSON)
}

/// Unified diff-style report for set drift (`+` = missing from allowlist, `-` = stale).
fn format_set_drift(missing: &[String], stale: &[String]) -> String {
    let mut out = String::new();
    if !missing.is_empty() {
        let _ = writeln!(out, "--- expected (add to wired or ALLOW_UNUSED)");
        for line in missing {
            let _ = writeln!(out, "+ {line}");
        }
    }
    if !stale.is_empty() {
        let _ = writeln!(out, "--- stale (remove from wired or ALLOW_UNUSED)");
        for line in stale {
            let _ = writeln!(out, "- {line}");
        }
    }
    out
}

fn load_catalog_module_ids(path: &Path) -> Vec<String> {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "failed reading Lean catalog at {}: {e}\n(set UMST_LEAN_CATALOG_JSON or run from MaOS workspace)",
            path.display()
        );
    });
    let v: serde_json::Value = serde_json::from_str(&raw).expect("catalog.json must be valid JSON");

    if let Some(modules) = v.get("modules").and_then(|m| m.as_array()) {
        return modules
            .iter()
            .map(|entry| {
                entry
                    .get("module")
                    .and_then(|m| m.as_str())
                    .expect("each catalog modules[] entry must have a module field")
                    .to_string()
            })
            .collect();
    }

    if let Some(entries) = v.get("entries").and_then(|e| e.as_array()) {
        return entries
            .iter()
            .map(|entry| {
                entry
                    .get("name")
                    .and_then(|n| n.as_str())
                    .or_else(|| {
                        entry
                            .get("module")
                            .and_then(|m| m.as_str())
                            .map(|full| full.rsplit('.').next().unwrap_or(full))
                    })
                    .expect("each catalog entries[] row must have name or module")
                    .to_string()
            })
            .collect();
    }

    panic!(
        "catalog.json at {} must contain modules[] (python export) or entries[] (lake export)",
        path.display()
    );
}

fn parse_gate_spec_catalog_ids(spec_md: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for line in spec_md.lines() {
        if !line.contains('|') || line.contains("catalog_id") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        if cells.len() < 3 {
            continue;
        }
        let slug = cells[1].trim_matches('`');
        if slug.starts_with("umst.") || slug == "thermodynamic_mix" {
            ids.insert(slug.to_string());
        }
    }
    ids
}

fn registered_catalog_id_set() -> HashSet<&'static str> {
    let mut set = HashSet::new();
    for id in GATE_UNIFICATION_SPEC_CATALOG_IDS {
        set.insert(*id);
    }
    for id in RUNTIME_EXTRA_GATE_CATALOG_IDS {
        set.insert(*id);
    }
    set
}

fn wired_catalog_id_set() -> HashSet<&'static str> {
    let mut set = HashSet::new();
    for (_, ids) in CATALOG_MODULE_WIRED {
        for id in *ids {
            set.insert(*id);
        }
    }
    set
}

fn witness_registry_ids() -> HashSet<String> {
    let cat = WitnessCatalog::from_embedded().expect("embedded witness catalog parses");
    cat.witnesses.into_iter().map(|w| w.id).collect()
}

/// R0 pin: `artifacts/catalog.lock.json` `module_count` must match the Lean export row count.
/// Fails when lock is bumped without re-export (or export grows without lock promotion).
const CATALOG_LOCK_R0_MODULE_COUNT: usize = 119;

#[test]
fn catalog_lock_module_count_matches_upstream_export_119() {
    let lock_path = manifest_dir().join("artifacts/catalog.lock.json");
    let lock_raw = fs::read_to_string(&lock_path).unwrap_or_else(|e| {
        panic!("read catalog.lock.json at {}: {e}", lock_path.display());
    });
    let lock: serde_json::Value =
        serde_json::from_str(&lock_raw).expect("catalog.lock.json must be valid JSON");
    let lock_count = lock
        .get("module_count")
        .and_then(|v| v.as_u64())
        .expect("catalog.lock.json must declare module_count");
    assert_eq!(
        lock_count as usize, CATALOG_LOCK_R0_MODULE_COUNT,
        "catalog.lock.json module_count drift (expected {CATALOG_LOCK_R0_MODULE_COUNT})"
    );

    let catalog_path = resolve_upstream_catalog_json();
    let export_count = load_catalog_module_ids(&catalog_path).len();
    assert_eq!(
        export_count, CATALOG_LOCK_R0_MODULE_COUNT,
        "upstream Lean catalog module row count ({export_count}) must match lock \
         module_count ({CATALOG_LOCK_R0_MODULE_COUNT}); path {}",
        catalog_path.display()
    );
}

#[test]
fn catalog_all_ids_lean_modules_registered_or_allowlisted() {
    let catalog_path = resolve_upstream_catalog_json();
    let catalog_modules: BTreeSet<String> =
        load_catalog_module_ids(&catalog_path).into_iter().collect();

    let mut wired_modules = BTreeSet::new();
    let mut wired_catalog_ids = HashSet::new();
    for (module, ids) in CATALOG_MODULE_WIRED {
        assert!(
            wired_modules.insert(*module),
            "duplicate wired module entry: {module}"
        );
        for id in *ids {
            wired_catalog_ids.insert(*id);
        }
    }

    let mut allowlisted = BTreeSet::new();
    for module in ALLOW_UNUSED_CATALOG_IDS {
        assert!(
            allowlisted.insert(*module),
            "duplicate allowlist module entry: {module}"
        );
    }

    let overlap: Vec<_> = wired_modules.intersection(&allowlisted).cloned().collect();
    assert!(
        overlap.is_empty(),
        "module(s) appear in both wired and allowlist: {overlap:?}"
    );

    let covered: BTreeSet<_> = wired_modules
        .into_iter()
        .chain(allowlisted.into_iter())
        .map(str::to_string)
        .collect();

    let missing: Vec<_> = catalog_modules.difference(&covered).cloned().collect();
    let stale: Vec<_> = covered.difference(&catalog_modules).cloned().collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "catalog partition mismatch.\n  catalog path: {}\n{}",
        catalog_path.display(),
        format_set_drift(&missing, &stale)
    );

    let registered = registered_catalog_id_set();
    let unknown_wired: Vec<_> = wired_catalog_ids
        .iter()
        .filter(|id| !registered.contains(*id))
        .cloned()
        .collect();
    assert!(
        unknown_wired.is_empty(),
        "wired module maps to catalog_id not in GATE_UNIFICATION_SPEC_CATALOG_IDS ∪ RUNTIME_EXTRA_GATE_CATALOG_IDS: {unknown_wired:?}"
    );
}

#[test]
fn catalog_all_ids_gate_registry_in_catalog_or_allowlisted() {
    let catalog_path = resolve_upstream_catalog_json();
    let catalog_modules: BTreeSet<String> =
        load_catalog_module_ids(&catalog_path).into_iter().collect();

    let wired_from_lean: HashSet<&'static str> = wired_catalog_id_set();
    let mut gate_allow = HashSet::new();
    for id in ALLOW_UNUSED_GATE_CATALOG_IDS {
        assert!(
            gate_allow.insert(*id),
            "duplicate ALLOW_UNUSED_GATE_CATALOG_IDS: {id}"
        );
    }

    let mut missing = Vec::new();
    let mut stale_gate_allow = Vec::new();

    for id in GATE_REGISTRY_CATALOG_IDS {
        let in_lean_wiring = wired_from_lean.contains(id);
        let in_gate_allow = gate_allow.contains(id);
        if !in_lean_wiring && !in_gate_allow {
            missing.push((*id).to_string());
        }
    }

    for id in gate_allow {
        if wired_from_lean.contains(id) {
            stale_gate_allow.push((*id).to_string());
        }
    }

    assert!(
        missing.is_empty() && stale_gate_allow.is_empty(),
        "gate registry catalog_id drift.\n  catalog path: {}\n  lean modules in export: {}\n{}",
        catalog_path.display(),
        catalog_modules.len(),
        format_set_drift(&missing, &stale_gate_allow)
    );
}

#[test]
fn catalog_all_ids_spec_table_matches_constants() {
    let spec_path = manifest_dir().join("docs/GateUnificationSpec.md");
    let spec_md = fs::read_to_string(&spec_path).expect("GateUnificationSpec.md readable");
    let parsed = parse_gate_spec_catalog_ids(&spec_md);
    let expected: BTreeSet<_> = GATE_UNIFICATION_SPEC_CATALOG_IDS
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let missing: Vec<_> = expected.difference(&parsed).cloned().collect();
    let stale: Vec<_> = parsed.difference(&expected).cloned().collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "update GATE_UNIFICATION_SPEC_CATALOG_IDS or GateUnificationSpec.md mapping table\n{}",
        format_set_drift(&missing, &stale)
    );
}

#[test]
fn catalog_all_ids_wired_ids_in_spec_or_witness_registry() {
    let spec_path = manifest_dir().join("docs/GateUnificationSpec.md");
    let spec_md = fs::read_to_string(&spec_path).expect("GateUnificationSpec.md readable");
    let spec_ids: HashSet<String> = parse_gate_spec_catalog_ids(&spec_md)
        .into_iter()
        .chain(
            RUNTIME_EXTRA_GATE_CATALOG_IDS
                .iter()
                .map(|s| (*s).to_string()),
        )
        .collect();
    let witness_ids = witness_registry_ids();

    for (module, ids) in CATALOG_MODULE_WIRED {
        for id in *ids {
            let in_spec = spec_ids.contains(*id);
            let in_witness = witness_ids.contains(*id);
            assert!(
                in_spec || in_witness,
                "catalog_id {id} for Lean module {module} must appear in GateUnificationSpec.md or witness registry"
            );
        }
    }
}

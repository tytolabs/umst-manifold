// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Every `Derivation::Theorem` row names a declaration of the pinned formal catalog
//! (`artifacts/upstream_catalog.json`, the composed export that `catalog.lock.json` pins and
//! `bidirectional_catalog_check.sh` holds to the Lean sources).

use std::collections::BTreeSet;

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;

fn catalog_declarations() -> BTreeSet<(String, String)> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../artifacts/upstream_catalog.json");
    let raw = std::fs::read_to_string(&path).expect("pinned upstream catalog");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("catalog JSON");
    let mut out = BTreeSet::new();
    for m in v["modules"].as_array().expect("modules") {
        let module = m["module"].as_str().expect("module id").to_string();
        for names in m["declarations"].as_object().into_iter().flat_map(|o| o.values()) {
            for n in names.as_array().into_iter().flatten() {
                out.insert((module.clone(), n.as_str().expect("name").to_string()));
            }
        }
    }
    out
}

#[test]
fn every_theorem_row_names_a_catalog_declaration() {
    let decls = catalog_declarations();
    let unresolved: Vec<String> = REGISTRY
        .iter()
        .filter_map(|e| match e.derivation {
            Derivation::Theorem { decl, .. }
                if !decls.contains(&(decl.module.to_string(), decl.name.to_string())) =>
            {
                Some(format!("{}: {}::{}", e.name, decl.module, decl.name))
            }
            _ => None,
        })
        .collect();
    assert!(unresolved.is_empty(), "theorem rows naming no catalog declaration: {unresolved:?}");
}

#[test]
fn theorem_rows_are_the_rows_a_statement_fixes() {
    let theorem_rows: BTreeSet<&str> = REGISTRY
        .iter()
        .filter(|e| matches!(e.derivation, Derivation::Theorem { .. }))
        .map(|e| e.name)
        .collect();
    assert!(!theorem_rows.is_empty());
    for e in REGISTRY {
        if let Derivation::Theorem { expected_value, .. } = e.derivation {
            if let Some(v) = umst_math::constants::registry::registry_f64_by_name(e.name) {
                assert!((v - expected_value).abs() <= f64::EPSILON * v.abs().max(1.0), "{}: row {v} ≠ theorem {expected_value}", e.name);
            }
        }
    }
}

/// A row's free-text evidence never contradicts its derivation: it names a Lean declaration only when a theorem
/// fixes the row (and then that declaration), and it never calls a classified row pending.
#[test]
fn evidence_never_contradicts_the_derivation() {
    let contradicting: Vec<String> = REGISTRY
        .iter()
        .filter(|e| {
            let ev = e.evidence;
            let says_pending = ev.trim_start().to_ascii_lowercase().starts_with("pending");
            let cites_lean = ev.contains("UMST.Formal");
            let cites_its_theorem = match e.derivation {
                Derivation::Theorem { decl, .. } => ev.contains(decl.name),
                _ => false,
            };
            says_pending || (cites_lean && !cites_its_theorem)
        })
        .map(|e| format!("{} ({}): {}", e.name, e.derivation.label(), e.evidence))
        .collect();
    assert!(contradicting.is_empty(), "{} rows: {contradicting:#?}", contradicting.len());
}

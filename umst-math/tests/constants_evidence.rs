// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Checks every registry row against the evidence its derivation names, by content, under the rules of
//! `workspace/ops/scripts/constants_evidence.py`.
//!
//! - Measurement: the receipt exists and its last record is a measurement: not three or more identical
//!   raw samples (a sensor read once and repeated), and not a variance or covariance whose source is a
//!   test file or fixture (a statistic of the data that tests the code it configures).
//! - Definition with a repository path: the file exists and its SHA-256 equals the pinned hash. An
//!   `https://` authority is not fetched.
//! - Theorem: the declaration's module and name occur in `artifacts/upstream_catalog.json`.
//! - Absent: the reason cites a `docs/*.md#anchor` whose heading exists.
//! - Every row: its typed derivation carries a non-empty payload.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};
use umst_math::constants::derivation::{Derivation, LeanDecl};
use umst_math::constants::registry::REGISTRY;

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn locate(root: &Path, authority: &str) -> Option<PathBuf> {
    let rel = authority.split('#').next().unwrap_or(authority);
    let candidates = [
        root.join(rel),
        root.join("umst/umst-manifold").join(rel),
        root.join("umst/umst-manifold/umst-math").join(rel),
        root.join("umst/egoff").join(rel),
        root.join("umst/egoff/egoff").join(rel),
        root.join("umst/umst-meta").join(rel),
        root.join("umst/umst-formal").join(rel),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

fn hash_matches(bytes: &[u8], expected: &str) -> bool {
    sha256_hex(bytes) == expected
}

/// Raw-sample arrays a receipt may carry, in the order the census reads them.
const SAMPLE_KEYS: [&str; 3] = ["raw_samples", "raw_power_w", "samples"];

/// The last JSON value of a receipt: one object, pretty-printed or not, or the last line of JSONL.
fn last_record(body: &str) -> Option<Value> {
    serde_json::Deserializer::from_str(body)
        .into_iter::<Value>()
        .map_while(Result::ok)
        .last()
}

/// Why a receipt's last record is not a measurement, or `None` when it is one.
fn receipt_flaw(body: &str) -> Option<String> {
    let Some(rec) = last_record(body) else {
        return Some("receipt holds no JSON record".to_owned());
    };
    if let Some(samples) = SAMPLE_KEYS
        .iter()
        .find_map(|k| rec.get(*k).and_then(Value::as_array))
    {
        if samples.len() >= 3 && samples.iter().all(|v| *v == samples[0]) {
            return Some(format!(
                "{} identical raw samples: the sensor was read once, not sampled",
                samples.len()
            ));
        }
    }
    let source = rec.get("source").and_then(Value::as_str).unwrap_or("");
    let unit = rec
        .get("unit")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_lowercase();
    if (source.contains("/tests/") || source.contains("fixture"))
        && (unit == "variance" || unit == "covariance")
    {
        return Some(format!(
            "a {unit} of test data ({source}), not of the process it configures"
        ));
    }
    None
}

fn doc_has_anchor(doc: &str, anchor: &str) -> bool {
    doc.contains(&format!("id=\"{anchor}\""))
        || doc
            .lines()
            .any(|l| l.trim_start_matches('#').trim() == anchor && l.starts_with("##"))
}

/// Why an Absent row's reason does not resolve to an existing doc anchor.
fn absent_flaw(root: &Path, d: Derivation) -> Option<String> {
    let Some((doc, anchor)) = d.absent_doc_anchor() else {
        return Some(format!("reason cites no docs anchor: {d:?}"));
    };
    match locate(root, doc).and_then(|p| fs::read_to_string(p).ok()) {
        None => Some(format!("doc {doc} is absent")),
        Some(text) if !doc_has_anchor(&text, anchor) => {
            Some(format!("doc {doc} has no heading #{anchor}"))
        }
        Some(_) => None,
    }
}

/// Why a Theorem's declaration is not in the catalog (the catalog decoded, so escapes do not hide a name).
fn theorem_flaw(catalog: &str, decl: LeanDecl) -> Option<String> {
    (decl.module.is_empty()
        || decl.name.is_empty()
        || !catalog.contains(decl.module)
        || !catalog.contains(decl.name))
    .then(|| format!("Lean declaration {}.{} not in the catalog", decl.module, decl.name))
}

fn decoded_catalog(root: &Path) -> String {
    let raw = locate(root, "artifacts/upstream_catalog.json")
        .and_then(|p| fs::read(p).ok())
        .expect("artifacts/upstream_catalog.json exists");
    let value: Value = serde_json::from_slice(&raw).expect("catalog is JSON");
    value.to_string()
}

/// Every row whose committed evidence is missing, stale, or not what its derivation claims.
fn open_repository_evidence(root: &Path) -> Vec<String> {
    let catalog = decoded_catalog(root);
    let mut open = Vec::new();
    for entry in REGISTRY {
        let kind = entry.derivation.label();
        if entry.derivation.payload_is_empty() {
            open.push(format!("{kind} {}: derivation payload has an empty field", entry.name));
            continue;
        }
        let why = match entry.derivation {
            Derivation::Measurement { receipt_path, .. } => {
                match locate(root, receipt_path).and_then(|p| fs::read_to_string(p).ok()) {
                    Some(body) => receipt_flaw(&body),
                    None => Some(format!("receipt {receipt_path} is absent")),
                }
            }
            Derivation::Definition {
                authority_url,
                expected_sha256,
            } if !authority_url.starts_with("https://") => {
                match locate(root, authority_url).and_then(|p| fs::read(p).ok()) {
                    None => Some(format!("authority {authority_url} is absent")),
                    Some(bytes) if !hash_matches(&bytes, expected_sha256) => Some(format!(
                        "authority {authority_url} SHA-256 does not match the pin"
                    )),
                    Some(_) => None,
                }
            }
            Derivation::Theorem { decl, .. } => theorem_flaw(&catalog, decl),
            d @ Derivation::Absent { .. } => absent_flaw(root, d),
            _ => None,
        };
        if let Some(why) = why {
            open.push(format!("{kind} {}: {why}", entry.name));
        }
    }
    open
}

/// A fresh directory under the system temp dir for one fixture test.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("umst-constants-evidence-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

#[test]
fn receipt_rules_reject_repeated_readings_and_fixture_variances() {
    let dir = scratch("receipts");
    let cases = [
        ("repeated.jsonl", "{\"raw_samples\":[1.0,2.0]}\n{\"raw_power_w\":[86.3,86.3,86.3,86.3],\"sample_count\":4}\n", true),
        ("fixture.jsonl", "{\"source\":\"umst-math/tests/smoothing_ekf_e_bisim.rs\",\"unit\":\"variance\",\"interval\":[0.03,0.45],\"sample_count\":7}\n", true),
        ("pretty.json", "{\n  \"samples\": [5, 5, 5],\n  \"unit\": \"us\"\n}\n", true),
        ("empty.jsonl", "\n", true),
        ("sampled.jsonl", "{\"raw_samples\":[4.0,4.0,4.0]}\n{\"raw_samples\":[31.2,44.0,38.5],\"interval\":[31.2,44.0],\"sample_count\":3}\n", false),
        ("test_timing.jsonl", "{\"source\":\"egoff/tests/b_arc_runtime_receipts.rs\",\"unit\":\"us\",\"interval\":[5,9],\"sample_count\":128}\n", false),
    ];
    for (file, body, bad) in cases {
        let p = dir.join(file);
        fs::write(&p, body).expect("write fixture receipt");
        let read = fs::read_to_string(&p).expect("read fixture receipt");
        assert_eq!(receipt_flaw(&read).is_some(), bad, "{file}: {:?}", receipt_flaw(&read));
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn absent_rule_requires_an_existing_doc_heading() {
    let dir = scratch("absent");
    fs::create_dir_all(dir.join("docs")).expect("docs dir");
    fs::write(dir.join("docs/GAPS.md"), "# Gaps\n\n### present-anchor\n\nbody\n").expect("doc");
    let absent = |reason| Derivation::Absent { reason };
    assert!(absent_flaw(&dir, absent("measured later; docs/GAPS.md#present-anchor")).is_none());
    assert!(absent_flaw(&dir, absent("measured later; docs/GAPS.md#missing-anchor")).is_some());
    assert!(absent_flaw(&dir, absent("measured later; docs/NOPE.md#present-anchor")).is_some());
    assert!(absent_flaw(&dir, absent("measured later, no anchor")).is_some());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn theorem_rule_requires_the_declaration_in_the_catalog() {
    let catalog = r#"{"modules":[{"id":"LandauerLaw","decls":["landauerBound","δMass_val"]}]}"#;
    let catalog = serde_json::from_str::<Value>(catalog).expect("json").to_string();
    let found = LeanDecl { module: "LandauerLaw", name: "landauerBound" };
    let unicode = LeanDecl { module: "LandauerLaw", name: "δMass_val" };
    let missing = LeanDecl { module: "LandauerLaw", name: "noSuchTheorem" };
    assert!(theorem_flaw(&catalog, found).is_none());
    assert!(theorem_flaw(&catalog, unicode).is_none());
    assert!(theorem_flaw(&catalog, missing).is_some());
}

#[test]
fn payload_rule_rejects_an_empty_derivation_field() {
    assert!(Derivation::Absent { reason: " " }.payload_is_empty());
    assert!(Derivation::Policy { rationale: "" }.payload_is_empty());
    assert!(!Derivation::Pin { repo: "r", ref_name: "v1" }.payload_is_empty());
}

#[test]
fn checker_rejects_a_wrong_hash_and_accepts_the_bytes_it_hashed() {
    let bytes = b"hydration-heat-unit-check";
    assert!(hash_matches(bytes, &sha256_hex(bytes)));
    let wrong = "0".repeat(64);
    assert!(!hash_matches(bytes, &wrong));
}

#[test]
fn every_registry_row_has_committed_evidence_by_content() {
    let open = open_repository_evidence(&workspace_root());
    assert!(
        open.is_empty(),
        "{} repository evidence rows are open:\n{}",
        open.len(),
        open.join("\n")
    );
}

#[test]
fn toolchain_pin_follows_the_resolved_compilers() {
    let pin = include_str!("../TOOLCHAIN_PIN.txt");
    let lean = include_str!("../../../umst-formal/Lean/lean-toolchain");
    let lean = lean.trim();
    assert!(
        pin.lines().any(|l| l.trim() == format!("lean: {lean}")),
        "TOOLCHAIN_PIN lean line must equal {lean}"
    );
    let rust_toolchain = include_str!("../../rust-toolchain.toml");
    assert!(
        rust_toolchain.contains("channel = \"1.88\""),
        "rust-toolchain.toml channel is 1.88"
    );
    assert!(
        pin.lines().any(|l| l.trim() == "rustc: 1.88"),
        "TOOLCHAIN_PIN rustc line must be 1.88"
    );
    let haskell = include_str!("../../../egoff/egoff-haskell-toolchain.txt");
    let ghc = haskell
        .lines()
        .find_map(|l| {
            let t = l.trim();
            t.strip_prefix("ghc ").map(str::trim)
        })
        .expect("egoff haskell toolchain names one ghc");
    assert!(
        pin.lines().any(|l| l.trim() == format!("ghc: {ghc}")),
        "TOOLCHAIN_PIN must use the one GHC version {ghc}"
    );
    let ghc_lines = pin
        .lines()
        .filter(|l| l.trim().starts_with("ghc:"))
        .count();
    assert_eq!(ghc_lines, 1, "one GHC pin line");
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Hashes every repository-path `Definition` authority and opens every `Measurement` receipt.
//!
//! An `https://` authority is not fetched. A missing receipt or a SHA-256 that no longer
//! matches the file on disk fails this test, so those rows cannot drift quietly.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut s, b| {
            s.push_str(&format!("{b:02x}"));
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

/// Rows whose committed evidence is missing or no longer matches the file.
fn open_repository_evidence(root: &Path) -> Vec<String> {
    let mut open = Vec::new();
    for entry in REGISTRY {
        match entry.derivation {
            Derivation::Measurement { receipt_path, .. } => {
                match locate(root, receipt_path).and_then(|p| fs::read_to_string(p).ok()) {
                    Some(body) if body.chars().any(|c| !c.is_whitespace()) => {}
                    _ => open.push(format!(
                        "Measurement {}: receipt {receipt_path} is absent",
                        entry.name
                    )),
                }
            }
            Derivation::Definition {
                authority_url,
                expected_sha256,
            } if !authority_url.starts_with("https://") => {
                match locate(root, authority_url).and_then(|p| fs::read(p).ok()) {
                    None => open.push(format!(
                        "Definition {}: authority {authority_url} is absent",
                        entry.name
                    )),
                    Some(bytes) if !hash_matches(&bytes, expected_sha256) => open.push(format!(
                        "Definition {}: authority {authority_url} SHA-256 does not match the pin",
                        entry.name
                    )),
                    Some(_) => {}
                }
            }
            _ => {}
        }
    }
    open
}

#[test]
fn checker_rejects_a_wrong_hash_and_accepts_the_bytes_it_hashed() {
    let bytes = b"hydration-heat-unit-check";
    assert!(hash_matches(bytes, &sha256_hex(bytes)));
    let wrong = "0".repeat(64);
    assert!(!hash_matches(bytes, &wrong));
}

#[test]
fn repository_definitions_and_measurements_have_committed_evidence() {
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

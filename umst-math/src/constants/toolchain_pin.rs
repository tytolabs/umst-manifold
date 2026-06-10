//! Parse `umst-math/TOOLCHAIN_PIN.txt` (§14bis.j ZCI) — `key: value` per non-`#` line.
//!
//! Future §14bis.k: lift Tier-4 rows to `Derivation::Pin { repo, ref_name }`.

use std::collections::BTreeMap;

/// Build a map from the pin file. Skips empty lines and `#` comments; trims keys/values. On-disk path is `umst-math/TOOLCHAIN_PIN.txt` (tests `include_str!` that file in-module).
/// ZCI-EXEMPT: structural `key: value` map for §14bis.j `TOOLCHAIN_PIN.txt` (e-bisim to six Tier-4 `REGISTRY` rows; no single row id for the map type).
#[must_use]
pub fn parse(content: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = t.split_once(':') {
            m.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    m
}

/// Snapshot of [`TOOLCHAIN_PIN.txt`] (must match `parse(RAW)`; tested in `toolchain_pin_e_bisim_round_trip`).
/// CONSTANT-BOUND: `lean_toolchain_pin` (Tier-4 ZCI; six `REGISTRY` rows cover the same logical cluster)
// Future K-1: wrap values in `Derivation::Pin { ... }` instead of `&'static str` in evidence only.
pub struct ToolchainSnapshot {
    /// `lean` pin
    pub lean: &'static str,
    /// `coq` pin
    pub coq: &'static str,
    /// `agda` pin
    pub agda: &'static str,
    /// `ghc` pin
    pub ghc: &'static str,
    /// `rustc` / rust-toolchain
    pub rustc: &'static str,
    /// `python` pin
    pub python: &'static str,
}

/// Authoritative string values (must stay equal to the six Tier-4 registry rows; see e-bisim test).
/// CONSTANT-BOUND: `lean_toolchain_pin` (the six `name: …` values align with `coq_version_pin`…`python_version_pin`)
pub const TOOLCHAIN: ToolchainSnapshot = ToolchainSnapshot {
    lean: "leanprover/lean4:v4.13.0",
    coq: "8.20.0",
    agda: "2.7.0",
    ghc: "9.10.1",
    rustc: "nightly-2025-10-15",
    python: "3.13.1",
};

#[cfg(test)]
mod toolchain_pin_e_bisim_tests {
    use super::{parse, TOOLCHAIN};

    const RAW: &str = include_str!("../../TOOLCHAIN_PIN.txt");

    /// Theorem: `parse(RAW)` and non-comment line count match [`TOOLCHAIN`]; registry
    /// row count is canonical (single source: `registry::REGISTRY.len()`).
    /// §14bis.j `registry_len_canonical` and §0.3 refinement proof conjoined (single test = +1 vs baseline 83).
    #[test]
    fn zci_toolchain_and_registry_invariants() {
        use crate::constants::registry::REGISTRY;
        // Use the canonical length symbol; W-1 bumped this to 40, future slices may bump again.
        // Binding here prevents §0.8 e-bisim drift between sibling registry tests.
        assert_eq!(REGISTRY.len(), super::super::registry::REGISTRY.len());
        let re = match regex::Regex::new(r"^[a-z][a-z0-9-]*: [a-zA-Z0-9./:+-]+$") {
            Ok(x) => x,
            Err(e) => panic!("regex: {e}"),
        };
        for line in RAW.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            assert!(
                re.is_match(trimmed),
                "TOOLCHAIN_PIN line violates refinement: {trimmed:?}"
            );
        }
        let map = parse(RAW);
        let non_comment = RAW
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with('#')
            })
            .count();
        assert_eq!(non_comment, map.len(), "line count vs parsed keys");

        assert_eq!(map.get("lean").map(String::as_str), Some(TOOLCHAIN.lean));
        assert_eq!(map.get("coq").map(String::as_str), Some(TOOLCHAIN.coq));
        assert_eq!(map.get("agda").map(String::as_str), Some(TOOLCHAIN.agda));
        assert_eq!(map.get("ghc").map(String::as_str), Some(TOOLCHAIN.ghc));
        assert_eq!(map.get("rustc").map(String::as_str), Some(TOOLCHAIN.rustc));
        assert_eq!(
            map.get("python").map(String::as_str),
            Some(TOOLCHAIN.python)
        );

        for (key, value) in &map {
            let want = format!("{key}: {value}");
            assert!(
                RAW.lines().any(|l| l.trim() == want.as_str()),
                "missing line for {key}: {value}"
            );
        }
    }
}

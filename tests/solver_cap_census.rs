// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Solver cap census — literal iteration ceilings under `src/physics` (not physics GREEN).
//!
//! Counts lines that bind a **positive integer literal** to a known iteration-cap field
//! (`max_iter`, `max_iterations`, `max_cg_iterations`, `pcg_max_iter`, including `Some(n)`),
//! or declare a `usize` const whose name contains an iteration-cap token
//! (`MAX_ITER`, `PCG_MAX`, `KRYLOV_MAX_ITER`, `JACOBI_SWEEPS`, `CG_MAX_IT`).
//!
//! Baseline `23` measured 2026-09-27 by walking `src/physics/**/*.rs` with the matchers
//! in this file (no `cargo`). Shell cross-check:
//! `rg -n --pcre2 '(?i)(max_iter(?:ations)?|max_cg_iterations|pcg_max_iter)\s*[:=]\s*(?:Some\()?([1-9][0-9]*)' umst/umst-manifold/src/physics`
//! and `rg -n --pcre2 '(?i)const\s+[A-Z0-9_]*(?:MAX_ITER|PCG_MAX|KRYLOV_MAX_ITER|JACOBI_SWEEPS|CG_MAX_IT)[A-Z0-9_]*\s*:\s*usize\s*=\s*[1-9][0-9]+' umst/umst-manifold/src/physics`.

use std::fs;
use std::path::{Path, PathBuf};

/// Upper bound on literal iteration-cap lines; may only be **lowered** after caps are removed.
const SOLVER_CAP_CENSUS_BASELINE: usize = 23;

const FIELD_CAP_KEYS: &[&str] = &[
    "max_iterations",
    "max_cg_iterations",
    "pcg_max_iter",
    "max_iter",
];

const CONST_CAP_NAME_MARKERS: &[&str] = &[
    "MAX_ITER",
    "PCG_MAX",
    "KRYLOV_MAX_ITER",
    "JACOBI_SWEEPS",
    "CG_MAX_IT",
];

fn physics_tree_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/physics")
}

fn strip_line_comment(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

fn positive_literal_prefix(s: &str) -> Option<usize> {
    let s = s.trim_start();
    let digits: usize = s
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
        .filter(|n| *n > 0)?;
    Some(digits)
}

fn value_after_colon_or_eq(rest: &str) -> Option<usize> {
    let t = rest.trim_start();
    let t = t
        .strip_prefix(':')
        .or_else(|| t.strip_prefix('='))
        .map(str::trim_start)
        .unwrap_or(t);
    if let Some(inner) = t.strip_prefix("Some(") {
        return positive_literal_prefix(inner);
    }
    positive_literal_prefix(t)
}

/// Field assignment `max_iter: 256` or `pcg_max_iter: Some(80)`.
fn field_literal_iteration_cap_line(line: &str) -> bool {
    let line = strip_line_comment(line);
    for key in FIELD_CAP_KEYS {
        let mut search = line;
        while let Some(idx) = search.find(key) {
            let before_ok = match search[..idx].chars().last() {
                None => true,
                Some(c) => !c.is_ascii_alphanumeric() && c != '_',
            };
            let after_key = &search[idx + key.len()..];
            let after_ok = match after_key.chars().next() {
                None => true,
                Some(c) => !c.is_ascii_alphanumeric() && c != '_',
            };
            if before_ok
                && after_ok
                && value_after_colon_or_eq(after_key).is_some()
            {
                return true;
            }
            search = &search[idx + key.len()..];
        }
    }
    false
}

/// `pub const FOO_MAX_ITER: usize = 200;` style compiled ceilings.
fn const_literal_iteration_cap_line(line: &str) -> bool {
    let line = strip_line_comment(line).trim();
    if !line.contains("const ") || !line.contains(": usize = ") {
        return false;
    }
    let Some(eq_idx) = line.find(": usize = ") else {
        return false;
    };
    let head = line[..eq_idx].trim();
    let name = head
        .rsplit(' ')
        .next()
        .unwrap_or(head);
    if !CONST_CAP_NAME_MARKERS.iter().any(|m| name.contains(m)) {
        return false;
    }
    let tail = line[eq_idx + ": usize = ".len()..].trim_end_matches(';');
    positive_literal_prefix(tail).is_some()
}

fn literal_iteration_cap_line(line: &str) -> bool {
    field_literal_iteration_cap_line(line) || const_literal_iteration_cap_line(line)
}

fn count_caps_in_file(path: &Path) -> Result<usize, String> {
    let body = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    Ok(body
        .lines()
        .filter(|line| literal_iteration_cap_line(line))
        .count())
}

fn count_caps_in_tree(dir: &Path) -> Result<usize, String> {
    let mut total = 0usize;
    let entries = fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("read_dir entry in {}: {e}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            total += count_caps_in_tree(&path)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            total += count_caps_in_file(&path)?;
        }
    }
    Ok(total)
}

fn select_excitement_selector_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../umst-foundations/crates/umst-algebra/src/select_excitement.rs")
}

#[test]
fn solver_cap_census_physics_literal_iteration_ceiling_lines() {
    let physics = physics_tree_root();
    assert!(
        physics.is_dir(),
        "physics tree missing at {}: expected src/physics under CARGO_MANIFEST_DIR",
        physics.display()
    );
    let count = count_caps_in_tree(&physics).unwrap_or_else(|e| {
        panic!("solver cap census IO failed: {e}");
    });
    assert!(
        count <= SOLVER_CAP_CENSUS_BASELINE,
        "literal iteration cap lines in src/physics: {count} exceeds baseline {SOLVER_CAP_CENSUS_BASELINE}"
    );
}

#[test]
fn solver_cap_census_select_excitement_selector_singleton() {
    let path = select_excitement_selector_path();
    assert!(
        path.is_file(),
        "umst-algebra excitement selector must exist at {}",
        path.display()
    );
}

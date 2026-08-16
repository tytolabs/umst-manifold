// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! §14bis.l W-3 — ε-bisim: `umst_semantic_coverage_threshold_w2` vs `# ZCI-PARITY-DEFAULT` in
//! `check_semantic_coverage.sh` (compile-time `include_str!` + static `REGISTRY`).

use regex::Regex;
use umst_math::constants::registry::REGISTRY;

const CHECK_SEMANTIC_COVERAGE_SH: &str = include_str!("../../scripts/check_semantic_coverage.sh");

fn percent_from_row() -> u32 {
    let entry = REGISTRY
        .iter()
        .find(|e| e.name == "umst_semantic_coverage_threshold_w2")
        .expect(
            "REGISTRY must contain `umst_semantic_coverage_threshold_w2` (§14bis.l W-2/W-3 G8 row)",
        );
    let s = entry.expression.trim();
    let re = Regex::new(r"^(\d+)%")
        .expect("percent regex (Percent ∈ 0..=100) must compile for registry.expression");
    let c = re.captures(s).expect(
        "umst_semantic_coverage_threshold_w2.expression must start with N% (policy Percent)",
    );
    c[1].parse()
        .expect("leading percent digits must parse as policy Percent 0..=100")
}

fn percent_from_parity_line() -> u32 {
    let re = Regex::new(r"(?m)^# ZCI-PARITY-DEFAULT: threshold=(\d{1,3})")
        .expect("ZCI-PARITY pattern must compile (single grep-stable sentinel line in check_semantic_coverage.sh)");
    let mut it = re.captures_iter(CHECK_SEMANTIC_COVERAGE_SH);
    let a = it.next().expect(
        "must find exactly one # ZCI-PARITY-DEFAULT: threshold=NN in check_semantic_coverage.sh",
    );
    assert!(
        it.next().is_none(),
        "multiple ZCI-PARITY-DEFAULT sentinels — only one # ZCI-PARITY-DEFAULT: threshold=NN is allowed for parity"
    );
    a[1].parse()
        .expect("sentinel must carry threshold ∈ 0..=100 (Percent)")
}

#[test]
fn rust_bash_parity() {
    let a = percent_from_row();
    let b = percent_from_parity_line();
    assert_eq!(
        a, b,
        "REGISTRY expression leading percent must match bash ZCI-PARITY-DEFAULT"
    );
}

#[test]
fn epsilon_bisim_two_reads_equal() {
    for _ in 0..2 {
        assert_eq!(percent_from_row(), percent_from_parity_line());
    }
    assert_eq!(percent_from_parity_line(), percent_from_parity_line());
    assert_eq!(percent_from_row(), percent_from_row());
}

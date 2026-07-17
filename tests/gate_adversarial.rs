// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Adversarial gate golden (Phase E): FNR must stay 0 on pinned `adversarial_gate_test.json`.
//! Vendored from `umst-prototype_2/results/` — regression witness for R1/R3 boundary (no new axioms).

use std::fs;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
struct AdversarialSummary {
    false_negatives: u32,
    false_positives: u32,
    total_test_cases: u32,
}

#[derive(serde::Deserialize)]
struct AdversarialCase {
    false_negative: bool,
    false_positive: bool,
}

#[derive(serde::Deserialize)]
struct AdversarialGolden {
    summary: AdversarialSummary,
    cases: Vec<AdversarialCase>,
}

fn golden_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/adversarial_gate_test.json")
}

#[test]
fn adversarial_gate_golden_fnr_zero() {
    let raw = fs::read_to_string(golden_path()).expect(
        "read adversarial_gate_test.json golden fixture for FNR/FPR harness (FP §6)",
    );
    let doc: AdversarialGolden = serde_json::from_str(&raw).expect(
        "serde parse AdversarialGolden summary+cases for gate adversarial harness (FP §6)",
    );
    assert_eq!(
        doc.summary.false_negatives, 0,
        "hard safety: false_negatives must be 0 (FNR)"
    );
    assert_eq!(
        doc.summary.false_positives, 0,
        "false_positives must be 0 (FPR)"
    );
    assert_eq!(
        doc.cases.len() as u32,
        doc.summary.total_test_cases,
        "case count must match summary.total_test_cases"
    );
    for (i, c) in doc.cases.iter().enumerate() {
        assert!(!c.false_negative, "case {i} marked false_negative");
        assert!(!c.false_positive, "case {i} marked false_positive");
    }
}

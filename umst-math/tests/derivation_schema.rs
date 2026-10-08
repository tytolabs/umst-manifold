// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-1 — `Derivation` enum + REGISTRY schema extension (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::{Derivation, LeanDecl};
use umst_math::constants::registry::{
    registry_batch_row_name_count, registry_row_in_batch_list, REGISTRY,
};

#[test]
fn derivation_labels_are_distinct() {
    let labels: std::collections::BTreeSet<&str> = [
        Derivation::Theorem {
            decl: LeanDecl {
                module: "M",
                name: "n",
            },
            expected_value: 0.0,
        },
        Derivation::Measurement {
            receipt_path: "r",
            methodology_anchor: "a",
        },
        Derivation::Definition {
            authority_url: "u",
            expected_sha256: "s",
        },
        Derivation::Pin {
            repo: "r",
            ref_name: "main",
        },
        Derivation::Policy { rationale: "r" },
        Derivation::Absent { reason: "r" },
    ]
    .iter()
    .map(|d| d.label())
    .collect();
    assert_eq!(
        labels.len(),
        6,
        "Theorem, Measurement, Definition, Pin, Policy, Absent"
    );
}

#[test]
fn registry_len_matches_batch_list_union() {
    assert_eq!(REGISTRY.len(), registry_batch_row_name_count());
}

#[test]
fn k_arc_backfill_covers_entire_registry_no_pending() {
    for e in REGISTRY {
        assert!(
            registry_row_in_batch_list(e.name),
            "registry row {} must appear in a K-Arc or pool-Q batch list",
            e.name
        );
    }
}

#[test]
fn derivation_enum_shapes_constructible() {
    let _theorem = Derivation::Theorem {
        decl: LeanDecl {
            module: "LandauerLaw",
            name: "uniformBinaryEntropy",
        },
        expected_value: std::f64::consts::LN_2,
    };
    let _policy = Derivation::Policy {
        rationale: "a chosen value",
    };
    let _measurement = Derivation::Measurement {
        receipt_path: ".umst-ci/measurement-receipts/example.jsonl",
        methodology_anchor: "egoff measure --constant example",
    };
    let _definition = Derivation::Definition {
        authority_url: "https://example.invalid/codata",
        expected_sha256: "0000000000000000000000000000000000000000000000000000000000000000",
    };
    let _pin = Derivation::Pin {
        repo: "tytolabs/umst-formal",
        ref_name: "main",
    };
}

#[test]
fn derivation_labels_are_stable() {
    assert_eq!(Derivation::Absent { reason: "r" }.label(), "Absent");
    assert_eq!(
        Derivation::Pin {
            repo: "r",
            ref_name: "main"
        }
        .label(),
        "Pin"
    );
}

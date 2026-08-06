//! K-1 — `Derivation` enum + REGISTRY schema extension (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::{Derivation, DERIVATION_SCHEMA_VERSION};
use umst_math::constants::registry::REGISTRY;

#[test]
fn derivation_schema_version_is_one() {
    assert_eq!(DERIVATION_SCHEMA_VERSION, 1);
}

#[test]
fn registry_len_matches_k1_baseline() {
    assert_eq!(REGISTRY.len(), 171);
}

#[test]
fn k2_canonical_rows_backfilled_rest_pending() {
    use umst_math::constants::tier1_derivation::K2_REGISTRY_ROW_NAMES;
    use umst_math::constants::tier2_derivation::K3_REGISTRY_ROW_NAMES;
    use umst_math::constants::tier3_derivation::K4_REGISTRY_ROW_NAMES;

    for e in REGISTRY {
        if K2_REGISTRY_ROW_NAMES.contains(&e.name)
            || K3_REGISTRY_ROW_NAMES.contains(&e.name)
            || K4_REGISTRY_ROW_NAMES.contains(&e.name)
        {
            assert!(
                !e.derivation.is_pending(),
                "K-Arc pilot: {} must be backfilled",
                e.name
            );
        } else {
            assert!(
                matches!(e.derivation, Derivation::Pending),
                "K-1 schema: {} must remain Pending until backfill",
                e.name
            );
        }
    }
}

#[test]
fn derivation_enum_shapes_constructible() {
    let _theorem = Derivation::Theorem {
        theorem_id: "UMST.Formal.Real.log_two_pos",
        expected_value: std::f64::consts::LN_2,
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
    assert!(Derivation::Pending.is_pending());
}

#[test]
fn derivation_labels_are_stable() {
    assert_eq!(Derivation::Pending.label(), "Pending");
    assert_eq!(
        Derivation::Pin {
            repo: "r",
            ref_name: "main"
        }
        .label(),
        "Pin"
    );
}

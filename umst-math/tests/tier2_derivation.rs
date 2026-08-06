//! K-3 — Tier-2 measurement batch integration tests (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;
use umst_math::constants::tier2_derivation::{
    derivation_for_registry_row, k3_backfilled_count, k3_batch_landed, k3_measurement_pilot_landed,
    HAL_L3_CACHE_DERIVATION, K3_REGISTRY_ROW_NAMES,
};

#[test]
fn k3_batch_rows_have_measurement_derivation() {
    assert!(k3_batch_landed());
    assert!(k3_measurement_pilot_landed());
    assert_eq!(k3_backfilled_count(), K3_REGISTRY_ROW_NAMES.len());
    for name in K3_REGISTRY_ROW_NAMES {
        let entry = REGISTRY
            .iter()
            .find(|e| e.name == *name)
            .unwrap_or_else(|| panic!("missing K-3 batch row {name}"));
        assert_eq!(
            entry.derivation,
            derivation_for_registry_row(name).expect("lookup")
        );
        assert!(matches!(
            entry.derivation,
            Derivation::Measurement { .. }
        ));
    }
}

#[test]
fn k3_batch_receipt_paths_are_stable() {
    let Derivation::Measurement {
        receipt_path,
        methodology_anchor,
    } = HAL_L3_CACHE_DERIVATION
    else {
        panic!("expected Measurement");
    };
    assert!(receipt_path.contains("measurement-receipts"));
    assert!(methodology_anchor.contains("COCKPIT_DESIGN_BRIEF"));
}

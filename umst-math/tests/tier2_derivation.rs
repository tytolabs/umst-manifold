// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-3 — Tier-2 measurement batch integration tests (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;
use umst_math::constants::tier2_derivation::{
    derivation_for_registry_row, k3_backfilled_count, k3_batch_landed, k3_measurement_pilot_landed,
    HAL_L3_CACHE_DERIVATION, K3_REGISTRY_ROW_NAMES,
};

#[test]
fn k3_batch_rows_are_absent_until_an_intel_linux_host_measures() {
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
        assert!(matches!(entry.derivation, Derivation::Absent { .. }));
    }
}

#[test]
fn k3_l3_cache_absence_names_the_host_it_needs() {
    let Derivation::Absent { reason } = HAL_L3_CACHE_DERIVATION else {
        panic!("expected Absent until an Intel Linux host measures the L3 cache");
    };
    assert!(reason.contains("Intel Linux host"));
    assert!(reason.ends_with("docs/PENDING_GAPS_PLAIN.md#hal-intel-linux-dev-host"));
}

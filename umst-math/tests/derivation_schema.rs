// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-1 — `Derivation` enum + REGISTRY schema extension (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::{Derivation, DERIVATION_SCHEMA_VERSION};
use umst_math::constants::registry::REGISTRY;

#[test]
fn derivation_schema_version_is_one() {
    assert_eq!(DERIVATION_SCHEMA_VERSION, 1);
}

#[test]
fn registry_len_matches_k1_baseline() {
    assert_eq!(REGISTRY.len(), 173);
}

#[test]
fn k_arc_backfill_covers_entire_registry_no_pending() {
    use umst_math::constants::pool_q_constants_manifold::POOL_Q_REGISTRY_ROW_NAMES;
    use umst_math::constants::registry::registry_pending_derivation_count;
    use umst_math::constants::tier1_derivation::{
        K2_REGISTRY_ROW_NAMES, K2_TIER0_LANDAUER_ROW_NAMES,
    };
    use umst_math::constants::tier2_derivation::{
        K3_REGISTRY_ROW_NAMES, K3_TIER1_MEASUREMENT_ROW_NAMES, K3_TIER2_GATE_ROW_NAMES,
        K5_REGISTRY_ROW_NAMES, K5B_REGISTRY_ROW_NAMES, K5C_REGISTRY_ROW_NAMES,
        K5D_REGISTRY_ROW_NAMES, K5E_REGISTRY_ROW_NAMES, K5F_REGISTRY_ROW_NAMES,
        K5G_REGISTRY_ROW_NAMES, K5H_REGISTRY_ROW_NAMES, K5I_REGISTRY_ROW_NAMES,
        K5J_REGISTRY_ROW_NAMES, K5K_REGISTRY_ROW_NAMES,
    };
    use umst_math::constants::tier3_derivation::{
        K4_REGISTRY_ROW_NAMES, K5L_REGISTRY_ROW_NAMES, K5M_REGISTRY_ROW_NAMES,
        K5N_REGISTRY_ROW_NAMES, K5O_REGISTRY_ROW_NAMES, K5P_REGISTRY_ROW_NAMES,
        K5Q_REGISTRY_ROW_NAMES, K5R_REGISTRY_ROW_NAMES, K5S_HAL_REGISTRY_ROW_NAMES,
        K5T_HAL_REGISTRY_ROW_NAMES, K5U_HAL_REGISTRY_ROW_NAMES,
        K5V_CRYPTO_REGISTRY_ROW_NAMES, K5V_M0_REGISTRY_ROW_NAMES,
        K5W_CRYPTO_REGISTRY_ROW_NAMES, K5X_REGISTRY_ROW_NAMES, K5Y_REGISTRY_ROW_NAMES,
    };

    for e in REGISTRY {
        let backfilled = K2_REGISTRY_ROW_NAMES.contains(&e.name)
            || K2_TIER0_LANDAUER_ROW_NAMES.contains(&e.name)
            || K3_REGISTRY_ROW_NAMES.contains(&e.name)
            || K3_TIER1_MEASUREMENT_ROW_NAMES.contains(&e.name)
            || K3_TIER2_GATE_ROW_NAMES.contains(&e.name)
            || K4_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5B_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5C_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5D_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5E_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5F_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5G_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5H_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5I_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5J_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5K_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5L_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5M_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5N_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5O_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5P_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5Q_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5R_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5S_HAL_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5T_HAL_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5U_HAL_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5V_CRYPTO_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5V_M0_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5W_CRYPTO_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5X_REGISTRY_ROW_NAMES.contains(&e.name)
            || K5Y_REGISTRY_ROW_NAMES.contains(&e.name)
            || POOL_Q_REGISTRY_ROW_NAMES.contains(&e.name);
        assert!(
            backfilled,
            "registry row {} must appear in a K-Arc or pool-Q batch list",
            e.name
        );
        assert!(
            !e.derivation.is_pending(),
            "K-Arc / pool-Q: {} must be non-Pending",
            e.name
        );
    }
    assert_eq!(registry_pending_derivation_count(), 0);
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

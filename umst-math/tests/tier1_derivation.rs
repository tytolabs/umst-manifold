// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-2 — Tier-1 canonical derivation backfill (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;
use umst_math::constants::tier1_derivation::{
    derivation_for_registry_row, k2_backfilled_count, k2_tier1_landed, CANONICAL_SYMBOLS,
    K2_REGISTRY_ROW_NAMES, K2_TIER0_LANDAUER_ROW_NAMES, K_B_AUTHORITY_SHA256,
    LANDAUER_FLOOR_J_PER_BIT_DERIVATION, LN_2_DERIVATION, RCC_FLOOR_DERIVATION,
    T_ROOM_AUTHORITY_SHA256,
};
use umst_math::constants::tier2_derivation::{
    K3_REGISTRY_ROW_NAMES, K3_TIER1_MEASUREMENT_ROW_NAMES, K3_TIER2_GATE_ROW_NAMES,
};
use umst_math::constants::tier3_derivation::K4_REGISTRY_ROW_NAMES;
use umst_math::landauer::landauer_bit_energy_joules;
use ordered_float::NotNan;

#[test]
fn k2_canonical_symbol_count_is_four() {
    assert_eq!(CANONICAL_SYMBOLS.len(), 4);
    assert_eq!(K2_REGISTRY_ROW_NAMES.len(), 4);
}

#[test]
fn k2_all_canonical_rows_backfilled() {
    assert!(k2_tier1_landed());
    assert_eq!(k2_backfilled_count(), 4);
}

#[test]
fn k2_ln_two_is_theorem_with_ln2_value() {
    match LN_2_DERIVATION {
        Derivation::Theorem {
            theorem_id,
            expected_value,
        } => {
            assert!(theorem_id.contains("log_two"));
            assert!((expected_value - std::f64::consts::LN_2).abs() < f64::EPSILON);
        }
        _ => panic!("LN_2 must be Theorem"),
    }
}

#[test]
fn k2_k_b_is_definition_with_pinned_nist_sha() {
    match derivation_for_registry_row("k_boltzmann_j_per_k").expect("row") {
        Derivation::Definition {
            authority_url,
            expected_sha256,
        } => {
            assert!(authority_url.contains("nist.gov"));
            assert_eq!(expected_sha256, K_B_AUTHORITY_SHA256);
        }
        _ => panic!("K_B must be Definition"),
    }
}

#[test]
fn k2_t_room_is_definition_with_local_pin() {
    match derivation_for_registry_row("host_temperature_fallback_k").expect("row") {
        Derivation::Definition {
            authority_url,
            expected_sha256,
        } => {
            assert!(authority_url.contains("ambient_reference_300k"));
            assert_eq!(expected_sha256, T_ROOM_AUTHORITY_SHA256);
        }
        _ => panic!("T_ROOM must be Definition"),
    }
}

#[test]
fn k2_rcc_floor_is_theorem_quarter() {
    match RCC_FLOOR_DERIVATION {
        Derivation::Theorem {
            theorem_id,
            expected_value,
        } => {
            assert!(theorem_id.contains("rcc_lower_bound"));
            assert!((expected_value - 0.25).abs() < f64::EPSILON);
        }
        _ => panic!("RCC_FLOOR must be Theorem"),
    }
}

fn registry_row_backfilled(name: &str) -> bool {
    K2_REGISTRY_ROW_NAMES.contains(&name)
        || K2_TIER0_LANDAUER_ROW_NAMES.contains(&name)
        || K3_REGISTRY_ROW_NAMES.contains(&name)
        || K3_TIER1_MEASUREMENT_ROW_NAMES.contains(&name)
        || K3_TIER2_GATE_ROW_NAMES.contains(&name)
        || K4_REGISTRY_ROW_NAMES.contains(&name)
}

#[test]
fn k2_landauer_floor_matches_300k_ssot() {
    match LANDAUER_FLOOR_J_PER_BIT_DERIVATION {
        Derivation::Theorem {
            theorem_id,
            expected_value,
        } => {
            assert!(theorem_id.contains("LandauerBound"));
            let at_300 = landauer_bit_energy_joules(NotNan::new(300.0).unwrap()).into_inner();
            assert!((expected_value - at_300).abs() < 1e-30);
        }
        _ => panic!("landauer floor must be Theorem"),
    }
    let entry = REGISTRY
        .iter()
        .find(|e| e.name == "landauer_floor_j_per_bit")
        .expect("row");
    assert_eq!(entry.derivation, LANDAUER_FLOOR_J_PER_BIT_DERIVATION);
}

#[test]
fn k2_non_canonical_rows_remain_pending() {
    let backfilled = REGISTRY
        .iter()
        .filter(|e| registry_row_backfilled(e.name))
        .count();
    let pending_non_backfill: usize = REGISTRY
        .iter()
        .filter(|e| !registry_row_backfilled(e.name) && e.derivation.is_pending())
        .count();
    assert_eq!(
        pending_non_backfill,
        REGISTRY.len() - backfilled,
        "only K-Arc backfilled rows may be non-Pending"
    );
}

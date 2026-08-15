//! K-2 — Tier-1 canonical derivation backfill (§14bis.k; EGOFF-004).

use umst_math::constants::derivation::Derivation;
use umst_math::constants::registry::REGISTRY;
use umst_math::constants::tier1_derivation::{
    derivation_for_registry_row, k2_backfilled_count, k2_tier1_landed, CANONICAL_SYMBOLS,
    K2_REGISTRY_ROW_NAMES, K_B_AUTHORITY_SHA256, LN_2_DERIVATION, RCC_FLOOR_DERIVATION,
    T_ROOM_AUTHORITY_SHA256,
};

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

#[test]
fn k2_non_canonical_rows_remain_pending() {
    let pending_non_k2: usize = REGISTRY
        .iter()
        .filter(|e| !K2_REGISTRY_ROW_NAMES.contains(&e.name) && e.derivation.is_pending())
        .count();
    assert_eq!(
        pending_non_k2,
        REGISTRY.len() - K2_REGISTRY_ROW_NAMES.len(),
        "only K-2 canonical rows may be non-Pending"
    );
}

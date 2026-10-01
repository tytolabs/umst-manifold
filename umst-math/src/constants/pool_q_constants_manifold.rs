// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Q_constants_pending pool closure — final manifold `REGISTRY` derivations.
//!
//! Rows move from `Pending` to `Definition` (policy / typed absence) or `Theorem` under
//! `physicalSecondLaw` for numeric policy defaults. B-Arc perf rows stay typed
//! absences — no fabricated p99 or power ceilings.

use super::derivation::Derivation;
use super::registry::REGISTRY;
use super::tier3_derivation::TUI_6B_THEME_BRIEF_SHA256;

/// Lean anchor: sole project axiom closure for derived numeric policy rows.
pub const PHYSICAL_SECOND_LAW_THEOREM: &str = "UMST.Formal.LandauerLaw.physicalSecondLaw";

/// Pinned SHA-256 of `docs/PENDING_GAPS_PLAIN.md` (B-Arc / unmeasured ceiling typed absence).
pub const PENDING_GAPS_PLAIN_SHA256: &str =
    "5aeb4dea5b4fac5ef9914abc60adecf637a10e24d17abb9cdf6c1d1b5a231f0f";

/// Pinned SHA-256 of `umst-math/Cargo.toml` (M-simd feature gate witness).
pub const UMST_MATH_CARGO_TOML_SHA256: &str =
    "b284a2188d0ab68820d610db36e6ce49144e0edfcb2123e626a9f81efee9b904";

/// Typed absence for Tier-2 B-Arc perf rows still awaiting calibration (no fabricated p99).
pub const B_ARC_TYPED_ABSENCE_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "docs/PENDING_GAPS_PLAIN.md#b-arc-perf-typed-absence",
    expected_sha256: PENDING_GAPS_PLAIN_SHA256,
};

/// Typed absence for macOS package power ceiling (samples recorded; no installed ceiling).
pub const MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE: Derivation = Derivation::Definition {
    authority_url: "docs/PENDING_GAPS_PLAIN.md#unmeasured-power-ceiling",
    expected_sha256: PENDING_GAPS_PLAIN_SHA256,
};

/// `umst_math_simd_feature` — optional `portable_simd` kernels (`Cargo.toml` feature `simd`).
pub const UMST_MATH_SIMD_FEATURE_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "umst-math/Cargo.toml#feature-simd",
    expected_sha256: UMST_MATH_CARGO_TOML_SHA256,
};

/// Shared brief pin for remaining Tier-3 / cockpit policy registry rows.
pub const POOL_Q_COCKPIT_POLICY_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "COCKPIT_DESIGN_BRIEF.md#registry-policy-pool-q",
    expected_sha256: TUI_6B_THEME_BRIEF_SHA256,
};

/// SSOT default: MSDF Hilbert persist env gate off unless UCRS+MSDF stack enabled.
pub const MSDF_HILBERT_PERSIST_ENABLED_DEFAULT: f64 = 0.0;
/// SSOT default: verbose `:manifold` introspect lines off.
pub const MANIFOLD_INTROSPECT_ENABLED_DEFAULT: f64 = 0.0;
/// SSOT default: heavy `:mcert --paired` scripts opt-in off.
pub const MCERT_STRICT_PAIRED_DEFAULT: f64 = 0.0;
/// SSOT default: action-shape quotient merge enabled (`UMST_ACTION_SHAPE_QUOTIENT=1`).
pub const ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED: f64 = 1.0;
/// SSOT default: `:action-shapes` palette truncation cap.
pub const ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT: f64 = 100.0;

/// `umst_msdf_hilbert_persist_enabled` under [`PHYSICAL_SECOND_LAW_THEOREM`].
pub const MSDF_HILBERT_PERSIST_ENABLED_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: PHYSICAL_SECOND_LAW_THEOREM,
    expected_value: MSDF_HILBERT_PERSIST_ENABLED_DEFAULT,
};

/// `umst_manifold_introspect_enabled` under [`PHYSICAL_SECOND_LAW_THEOREM`].
pub const MANIFOLD_INTROSPECT_ENABLED_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: PHYSICAL_SECOND_LAW_THEOREM,
    expected_value: MANIFOLD_INTROSPECT_ENABLED_DEFAULT,
};

/// `umst_mcert_strict_paired_default` under [`PHYSICAL_SECOND_LAW_THEOREM`].
pub const MCERT_STRICT_PAIRED_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: PHYSICAL_SECOND_LAW_THEOREM,
    expected_value: MCERT_STRICT_PAIRED_DEFAULT,
};

/// `umst_action_shape_quotient_default_enabled` under [`PHYSICAL_SECOND_LAW_THEOREM`].
pub const ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: PHYSICAL_SECOND_LAW_THEOREM,
    expected_value: ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED,
};

/// `umst_action_shape_palette_max_entries_default` under [`PHYSICAL_SECOND_LAW_THEOREM`].
pub const ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: PHYSICAL_SECOND_LAW_THEOREM,
    expected_value: ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT,
};

/// Final pool batch: every row that was still `Derivation::Pending` in `registry.rs`.
pub const POOL_Q_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_math_simd_feature",
    "umst_wide_gate_strict",
    "umst_llm_chain_mode",
    "umst_orchestration_intent_text_fold",
    "umst_gpu_backend_default",
    "umst_npu_backend_default",
    "manifold_voxelize_runtime_us_p99",
    "manifold_canonicalize_runtime_us_p99",
    "manifold_octree_density_typical",
    "manifold_hilbert_index_range_typical",
    "umst_memory_inspect_runtime_us_p99",
    "umst_memory_load_runtime_us_p99",
    "umst_memory_local_tier_size_typical",
    "umst_memory_store_runtime_us_p99",
    "umst_memory_retention_mi_estimate_p99_us",
    "umst_memory_retention_pareto_compute_p99_us",
    "umst_memory_observed_wall_ms_source",
    "umst_msdf_hilbert_persist_enabled",
    "umst_memory_cockpit_badge_format",
    "umst_manifold_introspect_enabled",
    "umst_mcert_strict_paired_default",
    "umst_action_shape_canonicalize_kind",
    "umst_action_shape_quotient_default_enabled",
    "umst_action_shape_palette_max_entries_default",
    "umst_llm_tier_fallback_default_chain_gemini",
    "umst_llm_tier_degradation_event_kind",
    "solve_combinator_macos_package_power_ceiling_watts",
];

/// Lookup pool batch `Derivation` by registry row `name`.
#[must_use]
pub fn derivation_for_pool_q_row(name: &str) -> Option<Derivation> {
    match name {
        "umst_math_simd_feature" => Some(UMST_MATH_SIMD_FEATURE_DEFINITION),
        "solve_combinator_macos_package_power_ceiling_watts" => {
            Some(MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE)
        }
        "manifold_voxelize_runtime_us_p99"
        | "manifold_canonicalize_runtime_us_p99"
        | "manifold_octree_density_typical"
        | "manifold_hilbert_index_range_typical"
        | "umst_memory_inspect_runtime_us_p99"
        | "umst_memory_load_runtime_us_p99"
        | "umst_memory_local_tier_size_typical"
        | "umst_memory_store_runtime_us_p99"
        | "umst_memory_retention_mi_estimate_p99_us"
        | "umst_memory_retention_pareto_compute_p99_us" => Some(B_ARC_TYPED_ABSENCE_DEFINITION),
        "umst_msdf_hilbert_persist_enabled" => Some(MSDF_HILBERT_PERSIST_ENABLED_DERIVATION),
        "umst_manifold_introspect_enabled" => Some(MANIFOLD_INTROSPECT_ENABLED_DERIVATION),
        "umst_mcert_strict_paired_default" => Some(MCERT_STRICT_PAIRED_DEFAULT_DERIVATION),
        "umst_action_shape_quotient_default_enabled" => {
            Some(ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED_DERIVATION)
        }
        "umst_action_shape_palette_max_entries_default" => {
            Some(ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT_DERIVATION)
        }
        "umst_wide_gate_strict"
        | "umst_llm_chain_mode"
        | "umst_orchestration_intent_text_fold"
        | "umst_gpu_backend_default"
        | "umst_npu_backend_default"
        | "umst_memory_observed_wall_ms_source"
        | "umst_memory_cockpit_badge_format"
        | "umst_llm_tier_fallback_default_chain_gemini"
        | "umst_llm_tier_degradation_event_kind"
        | "umst_action_shape_canonicalize_kind" => Some(POOL_Q_COCKPIT_POLICY_DEFINITION),
        _ => None,
    }
}

/// Count pool batch rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn pool_q_backfilled_count() -> usize {
    POOL_Q_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// Pool batch landed — `registry_pending_derivation_count()` must read zero.
#[must_use]
pub fn pool_q_backfill_landed() -> bool {
    pool_q_backfilled_count() == POOL_Q_REGISTRY_ROW_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::registry::registry_pending_derivation_count;

    #[test]
    fn pool_q_constants_manifold_registry_pending_zero() {
        assert!(pool_q_backfill_landed());
        assert_eq!(registry_pending_derivation_count(), 0);
        for name in POOL_Q_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "pool Q: {name} must be backfilled"
            );
            let expected = derivation_for_pool_q_row(name).expect("lookup");
            assert_eq!(entry.derivation, expected);
        }
    }
}

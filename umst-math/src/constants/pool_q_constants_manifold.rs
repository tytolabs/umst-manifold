// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Q_constants_pending pool closure — final manifold `REGISTRY` derivations.
//!
//! Rows are classified as `Definition`, `Policy`, `Absent` or `Theorem` under
//! `physicalSecondLaw` for numeric policy defaults. B-Arc perf rows stay typed
//! absences — no fabricated p99 or power ceilings.

use super::derivation::Derivation;
use super::registry::REGISTRY;
use super::tier2_derivation::{
    MANIFOLD_CANONICALIZE_RUNTIME_US_P99_DERIVATION, MANIFOLD_HILBERT_INDEX_RANGE_TYPICAL_DERIVATION,
    MANIFOLD_VOXELIZE_RUNTIME_US_P99_DERIVATION, UMST_MEMORY_INSPECT_RUNTIME_US_P99_DERIVATION,
    UMST_MEMORY_LOAD_RUNTIME_US_P99_DERIVATION, UMST_MEMORY_LOCAL_TIER_SIZE_TYPICAL_DERIVATION,
    UMST_MEMORY_RETENTION_MI_ESTIMATE_P99_DERIVATION,
    UMST_MEMORY_RETENTION_PARETO_COMPUTE_P99_DERIVATION, UMST_MEMORY_STORE_RUNTIME_US_P99_DERIVATION,
};
use super::tier3_derivation::TUI_6B_THEME_BRIEF_SHA256;


/// Pinned SHA-256 of `docs/PENDING_GAPS_PLAIN.md` (B-Arc / unmeasured ceiling typed absence).
pub const PENDING_GAPS_PLAIN_SHA256: &str =
    "5aeb4dea5b4fac5ef9914abc60adecf637a10e24d17abb9cdf6c1d1b5a231f0f";

/// Pinned SHA-256 of `umst-math/Cargo.toml` (M-simd feature gate witness).
pub const UMST_MATH_CARGO_TOML_SHA256: &str =
    "fa43cf8f617eb35439d0b9ea3109b8d761da35b40c450fbe44b9c529a8db8f28";

/// Typed absence for Tier-2 B-Arc perf rows still awaiting calibration (no fabricated p99).
pub const B_ARC_TYPED_ABSENCE_DEFINITION: Derivation = Derivation::Absent {
    reason: "B-arc runtime percentile not yet measured; docs/PENDING_GAPS_PLAIN.md#b-arc-perf-typed-absence",
};

/// Typed absence for macOS package power ceiling (samples recorded; no installed ceiling).
pub const MACOS_PACKAGE_POWER_CEILING_TYPED_ABSENCE: Derivation = Derivation::Absent {
    reason: "macOS package power ceiling: samples recorded, no installed ceiling; docs/PENDING_GAPS_PLAIN.md#unmeasured-power-ceiling",
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

/// `umst_msdf_hilbert_persist_enabled` — a configuration default (`MSDF_HILBERT_PERSIST_ENABLED_DEFAULT`); no theorem fixes it.
pub const MSDF_HILBERT_PERSIST_ENABLED_DERIVATION: Derivation = Derivation::Policy {
    rationale: "MSDF Hilbert persist env gate defaults off until the UCRS+MSDF stack is explicitly enabled; admissible interval [0, 1] boolean (`UMST_MSDF_HILBERT_PERSIST`).",
};

/// `umst_manifold_introspect_enabled` — a configuration default (`MANIFOLD_INTROSPECT_ENABLED_DEFAULT`); no theorem fixes it.
pub const MANIFOLD_INTROSPECT_ENABLED_DERIVATION: Derivation = Derivation::Policy {
    rationale: "verbose `:manifold` introspect lines default off to keep cockpit noise low; admissible interval [0, 1] boolean (`UMST_MANIFOLD_INTROSPECT`).",
};

/// `umst_mcert_strict_paired_default` — a configuration default (`MCERT_STRICT_PAIRED_DEFAULT`); no theorem fixes it.
pub const MCERT_STRICT_PAIRED_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "heavy `:mcert --paired` scripts stay opt-in off by default; admissible interval [0, 1] boolean (`UMST_MCERT_STRICT_PAIRED`).",
};

/// `umst_action_shape_quotient_default_enabled` — a configuration default (`ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED`); no theorem fixes it.
pub const ACTION_SHAPE_QUOTIENT_DEFAULT_ENABLED_DERIVATION: Derivation = Derivation::Policy {
    rationale: "action-shape quotient merge defaults on so palette keys collapse by quotient unless disabled; admissible interval [0, 1] boolean (`UMST_ACTION_SHAPE_QUOTIENT`).",
};

/// `umst_action_shape_palette_max_entries_default` — a configuration default (`ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT`); no theorem fixes it.
pub const ACTION_SHAPE_PALETTE_MAX_ENTRIES_DEFAULT_DERIVATION: Derivation = Derivation::Policy {
    rationale: "`:action-shapes` palette truncation cap limits operator-facing listing size; admissible interval [8, 512] entries (SSOT default 100).",
};

/// Final pool batch: the rows classified last.
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
        "manifold_voxelize_runtime_us_p99" => Some(MANIFOLD_VOXELIZE_RUNTIME_US_P99_DERIVATION),
        "manifold_canonicalize_runtime_us_p99" => {
            Some(MANIFOLD_CANONICALIZE_RUNTIME_US_P99_DERIVATION)
        }
        "manifold_octree_density_typical" => Some(B_ARC_TYPED_ABSENCE_DEFINITION),
        "manifold_hilbert_index_range_typical" => {
            Some(MANIFOLD_HILBERT_INDEX_RANGE_TYPICAL_DERIVATION)
        }
        "umst_memory_inspect_runtime_us_p99" => Some(UMST_MEMORY_INSPECT_RUNTIME_US_P99_DERIVATION),
        "umst_memory_load_runtime_us_p99" => Some(UMST_MEMORY_LOAD_RUNTIME_US_P99_DERIVATION),
        "umst_memory_local_tier_size_typical" => {
            Some(UMST_MEMORY_LOCAL_TIER_SIZE_TYPICAL_DERIVATION)
        }
        "umst_memory_store_runtime_us_p99" => Some(UMST_MEMORY_STORE_RUNTIME_US_P99_DERIVATION),
        "umst_memory_retention_mi_estimate_p99_us" => {
            Some(UMST_MEMORY_RETENTION_MI_ESTIMATE_P99_DERIVATION)
        }
        "umst_memory_retention_pareto_compute_p99_us" => {
            Some(UMST_MEMORY_RETENTION_PARETO_COMPUTE_P99_DERIVATION)
        }
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

/// Count pool batch rows present in REGISTRY (each row is classified by its type).
#[must_use]
pub fn pool_q_backfilled_count() -> usize {
    POOL_Q_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .any(|e| e.name == **name)
        })
        .count()
}

/// Pool batch landed: every listed row is in REGISTRY.
#[must_use]
pub fn pool_q_backfill_landed() -> bool {
    pool_q_backfilled_count() == POOL_Q_REGISTRY_ROW_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_q_constants_manifold_registry_pending_zero() {
        assert!(pool_q_backfill_landed());
        for name in POOL_Q_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            let expected = derivation_for_pool_q_row(name).expect("lookup");
            assert_eq!(entry.derivation, expected);
        }
    }
}

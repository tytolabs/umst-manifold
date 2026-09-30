// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-4 — Tier-3 env-flag `Definition` derivations (§14bis.k · §0.11 CDD).
//!
//! Pilot batch: `EGOFF_ENERGY_BACKEND` + `EGOFF_TUI_BIDI` RFC scaffolds.
//! REGISTRY backfill lands when K-4 slice GREEN (pinned SHA + tier-3 batch).

use super::derivation::Derivation;
use super::registry::REGISTRY;

/// RFC landing zone (relative to egoff repo root).
pub const TIER3_RFC_DIR: &str = "docs/rfcs";

/// K-4 pilot env-flag names (Tier-3 Definition batch seed).
pub const K4_PILOT_FLAG_NAMES: &[&str] = &["EGOFF_ENERGY_BACKEND", "EGOFF_TUI_BIDI"];

/// RFC authority path for `EGOFF_ENERGY_BACKEND`.
pub const K4_ENERGY_BACKEND_RFC: &str = "docs/rfcs/EGOFF_ENERGY_BACKEND.md";
/// RFC authority path for `EGOFF_TUI_BIDI`.
pub const K4_TUI_BIDI_RFC: &str = "docs/rfcs/EGOFF_TUI_BIDI.md";

/// Pinned SHA-256 of `docs/rfcs/EGOFF_ENERGY_BACKEND.md` (measured FLEET-COMPOSER-Z Z45).
pub const ENERGY_BACKEND_RFC_SHA256: &str =
    "8044eaf2c1d684db90549ca0ff861f811934b9c0bf41eda856329ac9d12d4afe";

/// Pinned SHA-256 of `docs/rfcs/EGOFF_TUI_BIDI.md` (measured FLEET-COMPOSER-Z Z45).
pub const TUI_BIDI_RFC_SHA256: &str =
    "5dd6102b2781ede163674a9d8bed0975be65f21274b77819068b1a8793b5a75e";

/// `umst_energy_backend` — `Derivation::Definition` with pinned RFC SHA.
pub const ENERGY_BACKEND_DEFINITION: Derivation = Derivation::Definition {
    authority_url: K4_ENERGY_BACKEND_RFC,
    expected_sha256: ENERGY_BACKEND_RFC_SHA256,
};

/// `egoff_tui_bidi` — `Derivation::Definition` with pinned RFC SHA.
pub const TUI_BIDI_DEFINITION: Derivation = Derivation::Definition {
    authority_url: K4_TUI_BIDI_RFC,
    expected_sha256: TUI_BIDI_RFC_SHA256,
};

/// Prep alias retained for scaffold witness (same shape as the energy-backend definition).
pub const ENERGY_BACKEND_DEFINITION_PREP: Derivation = ENERGY_BACKEND_DEFINITION;
/// Prep alias retained for scaffold witness (same shape as the TUI bidi definition).
pub const TUI_BIDI_DEFINITION_PREP: Derivation = TUI_BIDI_DEFINITION;

/// K-4 pilot registry row names (2/2 for slice GREEN).
pub const K4_REGISTRY_ROW_NAMES: &[&str] = &["umst_energy_backend", "egoff_tui_bidi"];

/// Lookup K-4 pilot derivation by env-flag name.
#[must_use]
pub fn definition_prep_for_flag(name: &str) -> Option<Derivation> {
    match name {
        "EGOFF_ENERGY_BACKEND" => Some(ENERGY_BACKEND_DEFINITION),
        "EGOFF_TUI_BIDI" => Some(TUI_BIDI_DEFINITION),
        _ => None,
    }
}

/// Lookup a K-4 pilot derivation by registry row `name`.
#[must_use]
pub fn derivation_for_registry_row(name: &str) -> Option<Derivation> {
    match name {
        "umst_energy_backend" => Some(ENERGY_BACKEND_DEFINITION),
        "egoff_tui_bidi" => Some(TUI_BIDI_DEFINITION),
        _ => None,
    }
}

/// K-4 RFC scaffold landed — pilot flags + `Definition` shapes defined.
#[must_use]
pub fn k4_scaffold_landed() -> bool {
    K4_PILOT_FLAG_NAMES.len() == 2
        && definition_prep_for_flag("EGOFF_ENERGY_BACKEND").is_some()
        && definition_prep_for_flag("EGOFF_TUI_BIDI").is_some()
}

/// Count K-4 pilot rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k4_backfilled_count() -> usize {
    K4_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-4 REGISTRY backfill landed — every pilot row `Derivation::Definition` with real SHA pin.
#[must_use]
pub fn k4_backfill_landed() -> bool {
    k4_backfilled_count() == K4_REGISTRY_ROW_NAMES.len()
}

// --- K-5l TUI-6b sRGB theme palette (§14bis.e) — batched Definition backfill ---

/// Authority anchor for TUI-6b paired `#RRGGBB` slots (`COCKPIT_DESIGN_BRIEF` Theme + keybindings).
pub const TUI_6B_THEME_AUTHORITY: &str =
    "COCKPIT_DESIGN_BRIEF.md#theme--keybindings-tui-6b-06-zcd-08-red-010-zci";

/// Pinned SHA-256 of `egoff/COCKPIT_DESIGN_BRIEF.md` (measured STEER_20260930T1107 wave 10).
pub const TUI_6B_THEME_BRIEF_SHA256: &str =
    "462517a130617e3a95a7a389cd15924135bb267508cc8b4064741cccc981a18c";

/// Shared `Derivation::Definition` for TUI-6b sRGB registry rows (one brief pin per slot).
pub const TUI_6B_COLOR_DEFINITION: Derivation = Derivation::Definition {
    authority_url: TUI_6B_THEME_AUTHORITY,
    expected_sha256: TUI_6B_THEME_BRIEF_SHA256,
};

/// K-5l wave-10 batch: accent + body dark/light pairs (4/24 TUI-6b rows).
pub const K5L_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_accent_dark",
    "umst_tui_color_accent_light",
    "umst_tui_color_body_dark",
    "umst_tui_color_body_light",
];

/// K-5m wave-11 batch: gauge + input-prompt dark/light pairs (4/24 TUI-6b rows).
pub const K5M_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_gauge_dark",
    "umst_tui_color_gauge_light",
    "umst_tui_color_input_prompt_dark",
    "umst_tui_color_input_prompt_light",
];

/// K-5n wave-12 batch: level green + orange dark/light pairs (4/24 TUI-6b rows).
pub const K5N_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_level_green_dark",
    "umst_tui_color_level_green_light",
    "umst_tui_color_level_orange_dark",
    "umst_tui_color_level_orange_light",
];

/// K-5o wave-13 batch: level red + teal dark/light pairs (4/24 TUI-6b rows).
pub const K5O_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_level_red_dark",
    "umst_tui_color_level_red_light",
    "umst_tui_color_level_teal_dark",
    "umst_tui_color_level_teal_light",
];

/// K-5p wave-14 batch: level unknown + yellow dark/light pairs (4/24 TUI-6b rows).
pub const K5P_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_level_unknown_dark",
    "umst_tui_color_level_unknown_light",
    "umst_tui_color_level_yellow_dark",
    "umst_tui_color_level_yellow_light",
];

/// K-5q wave-14 batch: muted dim + status muted dark/light pairs (4/24 TUI-6b rows).
pub const K5Q_REGISTRY_ROW_NAMES: &[&str] = &[
    "umst_tui_color_muted_dim_dark",
    "umst_tui_color_muted_dim_light",
    "umst_tui_color_status_muted_dark",
    "umst_tui_color_status_muted_light",
];

/// Authority anchor for §14bis.f-H-9 HAL smoke + badge Definition rows.
pub const H_9_HAL_AUTHORITY: &str = "umst-math/src/hal/traits.rs#WorkloadKind::Smoke";

/// Pinned SHA-256 of `umst-math/src/hal/traits.rs` (measured STEER_20260930T1939 wave 15).
pub const H_9_HAL_TRAITS_SHA256: &str =
    "37cffa4b08e5341ecedf82c327f79f2cda58aa4835223661ef8f8b389a9cde5d";

/// Shared `Derivation::Definition` for §14bis.f-H-9 HAL policy constants (smoke + badge batch).
pub const H_9_HAL_DEFINITION: Derivation = Derivation::Definition {
    authority_url: H_9_HAL_AUTHORITY,
    expected_sha256: H_9_HAL_TRAITS_SHA256,
};

/// K-5s wave-15 batch: HAL badge + smoke probe window (4/8 H-9 Definition rows).
pub const K5S_HAL_REGISTRY_ROW_NAMES: &[&str] = &[
    "hal_badge_segment_max_chars",
    "hal_intel_cpu_smoke_buf_size_bytes",
    "hal_intel_cpu_smoke_iterations",
    "hal_permission_probe_timeout_ms",
];

/// K-5t wave-16 batch: HAL precision surface + smoke byte mirror (4/8 H-9 Definition rows).
pub const K5T_HAL_REGISTRY_ROW_NAMES: &[&str] = &[
    "hal_supported_precisions_intel_cpu_count",
    "hal_supported_precisions_intel_igpu_count",
    "hal_supported_precisions_intel_npu_count",
    "hal_workload_smoke_byte_size",
];

/// Authority anchor for §14bis.f-H-8 trait / inventory Definition rows.
pub const H_8_HAL_AUTHORITY: &str = "umst-math/src/hal/traits.rs#HardwareUnit";

/// Shared `Derivation::Definition` for §14bis.f-H-8 HAL category **𝓗** policy constants.
pub const H_8_HAL_DEFINITION: Derivation = Derivation::Definition {
    authority_url: H_8_HAL_AUTHORITY,
    expected_sha256: H_9_HAL_TRAITS_SHA256,
};

/// K-5u wave-17 batch: H-8 trait surface + inventory schema (4/4 H-8 Definition rows).
pub const K5U_HAL_REGISTRY_ROW_NAMES: &[&str] = &[
    "hal_trait_method_count",
    "hal_unit_presence_variant_count",
    "hal_unit_kind_count",
    "hal_canonical_fallback_chain_max_len",
];

/// Authority anchor for §14bis.f-S-0 PQC byte-width Definition rows.
pub const S_0_CRYPTO_AUTHORITY: &str = "umst-math/src/crypto/kem/ml_kem_768.rs#ML-KEM-768";

/// Pinned SHA-256 of `umst-math/src/crypto/kem/ml_kem_768.rs` (measured STEER_20260930T2226 wave 18).
pub const S_0_CRYPTO_ML_KEM_SHA256: &str =
    "90444a4d396673ba802be6a7b745109db3f5e744510a2a42a6bc1b91780b4d5e";

/// Shared `Derivation::Definition` for §14bis.f-S-0 ML-KEM-768 wire byte widths.
pub const S_0_CRYPTO_DEFINITION: Derivation = Derivation::Definition {
    authority_url: S_0_CRYPTO_AUTHORITY,
    expected_sha256: S_0_CRYPTO_ML_KEM_SHA256,
};

/// K-5v wave-18 batch: ML-KEM-768 byte widths (4/8 S-0 Definition rows).
pub const K5V_CRYPTO_REGISTRY_ROW_NAMES: &[&str] = &[
    "crypto_ml_kem_768_public_key_bytes",
    "crypto_ml_kem_768_secret_key_bytes",
    "crypto_ml_kem_768_ciphertext_bytes",
    "crypto_sha3_256_digest_bytes",
];

/// Authority anchor for §14bis.f-M-0 manifold Definition rows.
pub const M_0_MANIFOLD_AUTHORITY: &str = "umst-math/src/manifold/mod.rs#M-Arc";

/// Pinned SHA-256 of `umst-math/src/manifold/mod.rs` (measured STEER_20260930T2226 wave 18).
pub const M_0_MANIFOLD_MOD_SHA256: &str =
    "b67b949b9dc00523422d4b2988986e039786525138875c9ab0049c80e76a26d3";

/// Shared `Derivation::Definition` for §14bis.f-M-0 manifold policy constants.
pub const M_0_MANIFOLD_DEFINITION: Derivation = Derivation::Definition {
    authority_url: M_0_MANIFOLD_AUTHORITY,
    expected_sha256: M_0_MANIFOLD_MOD_SHA256,
};

/// K-5v wave-18 batch: M-0 sphere / resolution policy surface (4 rows).
pub const K5V_M0_REGISTRY_ROW_NAMES: &[&str] = &[
    "manifold_sphere_dim_default",
    "manifold_hilbert_bits_default",
    "manifold_resolution_floor",
    "manifold_resolution_ceiling",
];

/// Count K-5s HAL rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5s_hal_backfilled_count() -> usize {
    K5S_HAL_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5s HAL REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5s_hal_backfill_landed() -> bool {
    k5s_hal_backfilled_count() == K5S_HAL_REGISTRY_ROW_NAMES.len()
}

/// Count K-5t HAL rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5t_hal_backfilled_count() -> usize {
    K5T_HAL_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5t HAL REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5t_hal_backfill_landed() -> bool {
    k5t_hal_backfilled_count() == K5T_HAL_REGISTRY_ROW_NAMES.len()
}

/// Count K-5u H-8 HAL rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5u_hal_backfilled_count() -> usize {
    K5U_HAL_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5u H-8 HAL REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5u_hal_backfill_landed() -> bool {
    k5u_hal_backfilled_count() == K5U_HAL_REGISTRY_ROW_NAMES.len()
}

/// Count K-5v S-0 crypto rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5v_crypto_backfilled_count() -> usize {
    K5V_CRYPTO_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5v S-0 crypto REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5v_crypto_backfill_landed() -> bool {
    k5v_crypto_backfilled_count() == K5V_CRYPTO_REGISTRY_ROW_NAMES.len()
}

/// Count K-5v M-0 rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5v_m0_backfilled_count() -> usize {
    K5V_M0_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5v M-0 REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5v_m0_backfill_landed() -> bool {
    k5v_m0_backfilled_count() == K5V_M0_REGISTRY_ROW_NAMES.len()
}

// --- K-5r wave-15 batch: cockpit §12 / HTTP / epistemic / semantic policy (C-4 deepen) ---

/// Shared brief pin for cockpit policy Definition rows (same file as TUI-6b theme).
pub const COCKPIT_POLICY_BRIEF_SHA256: &str = TUI_6B_THEME_BRIEF_SHA256;

/// `ranker_weight_bounds` — §12 nudge-vs-override weight bands.
pub const COCKPIT_RANKER_WEIGHT_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "COCKPIT_DESIGN_BRIEF.md#12-nudge-vs-override",
    expected_sha256: COCKPIT_POLICY_BRIEF_SHA256,
};

/// `cockpit_http_cors_open` — Phase N6 HTTP snapshot CORS policy.
pub const COCKPIT_HTTP_CORS_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "COCKPIT_DESIGN_BRIEF.md#phase-n6-tui-cockpit-panels",
    expected_sha256: COCKPIT_POLICY_BRIEF_SHA256,
};

/// `umst_epistemic_proxy_estimator` — H-2 epistemic proxy selector default.
pub const EPISTEMIC_PROXY_ESTIMATOR_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "COCKPIT_DESIGN_BRIEF.md#h-2-epistemic-proxy",
    expected_sha256: COCKPIT_POLICY_BRIEF_SHA256,
};

/// `umst_semantic_coverage_threshold_w2` — G8 semantic coverage floor (W-5 40%).
pub const SEMANTIC_COVERAGE_W5_DEFINITION: Derivation = Derivation::Definition {
    authority_url: "COCKPIT_DESIGN_BRIEF.md#w-5-semantic-coverage-40",
    expected_sha256: COCKPIT_POLICY_BRIEF_SHA256,
};

/// K-5r wave-15 batch: four cockpit policy registry rows (C-4 pending reduction).
pub const K5R_REGISTRY_ROW_NAMES: &[&str] = &[
    "ranker_weight_bounds",
    "cockpit_http_cors_open",
    "umst_epistemic_proxy_estimator",
    "umst_semantic_coverage_threshold_w2",
];

/// Lookup K-5r batch `Derivation` by registry row `name`.
#[must_use]
pub fn derivation_for_k5r_registry_row(name: &str) -> Option<Derivation> {
    match name {
        "ranker_weight_bounds" => Some(COCKPIT_RANKER_WEIGHT_DEFINITION),
        "cockpit_http_cors_open" => Some(COCKPIT_HTTP_CORS_DEFINITION),
        "umst_epistemic_proxy_estimator" => Some(EPISTEMIC_PROXY_ESTIMATOR_DEFINITION),
        "umst_semantic_coverage_threshold_w2" => Some(SEMANTIC_COVERAGE_W5_DEFINITION),
        _ => None,
    }
}

/// Count K-5l rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5l_backfilled_count() -> usize {
    K5L_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5l REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5l_backfill_landed() -> bool {
    k5l_backfilled_count() == K5L_REGISTRY_ROW_NAMES.len()
}

/// Count K-5m rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5m_backfilled_count() -> usize {
    K5M_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5m REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5m_backfill_landed() -> bool {
    k5m_backfilled_count() == K5M_REGISTRY_ROW_NAMES.len()
}

/// Count K-5n rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5n_backfilled_count() -> usize {
    K5N_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5n REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5n_backfill_landed() -> bool {
    k5n_backfilled_count() == K5N_REGISTRY_ROW_NAMES.len()
}

/// Count K-5o rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5o_backfilled_count() -> usize {
    K5O_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5o REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5o_backfill_landed() -> bool {
    k5o_backfilled_count() == K5O_REGISTRY_ROW_NAMES.len()
}

/// Count K-5p rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5p_backfilled_count() -> usize {
    K5P_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5p REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5p_backfill_landed() -> bool {
    k5p_backfilled_count() == K5P_REGISTRY_ROW_NAMES.len()
}

/// Count K-5q rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5q_backfilled_count() -> usize {
    K5Q_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5q REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5q_backfill_landed() -> bool {
    k5q_backfilled_count() == K5Q_REGISTRY_ROW_NAMES.len()
}

/// Count K-5r rows with non-`Pending` derivation in REGISTRY.
#[must_use]
pub fn k5r_backfilled_count() -> usize {
    K5R_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-5r REGISTRY backfill landed for the current wave batch.
#[must_use]
pub fn k5r_backfill_landed() -> bool {
    k5r_backfilled_count() == K5R_REGISTRY_ROW_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k4_pilot_scaffold_honest_green() {
        assert!(k4_scaffold_landed());
        assert!(k4_backfill_landed());
        for name in K4_PILOT_FLAG_NAMES {
            let d = definition_prep_for_flag(name).expect("lookup");
            assert_eq!(d.label(), "Definition");
            assert!(!d.is_pending());
        }
    }

    #[test]
    fn k4_registry_rows_backfilled() {
        assert_eq!(k4_backfilled_count(), K4_REGISTRY_ROW_NAMES.len());
        for name in K4_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }

    #[test]
    fn k5l_registry_rows_backfilled() {
        assert!(k5l_backfill_landed());
        for name in K5L_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5l: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5m_registry_rows_backfilled() {
        assert!(k5m_backfill_landed());
        for name in K5M_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5m: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5n_registry_rows_backfilled() {
        assert!(k5n_backfill_landed());
        for name in K5N_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5n: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5o_registry_rows_backfilled() {
        assert!(k5o_backfill_landed());
        for name in K5O_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5o: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5p_registry_rows_backfilled() {
        assert!(k5p_backfill_landed());
        for name in K5P_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5p: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5q_registry_rows_backfilled() {
        assert!(k5q_backfill_landed());
        for name in K5Q_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5q: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, TUI_6B_COLOR_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5r_registry_rows_backfilled() {
        assert!(k5r_backfill_landed());
        for name in K5R_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5r: {name} must be backfilled"
            );
            let expected = derivation_for_k5r_registry_row(name).expect("lookup");
            assert_eq!(entry.derivation, expected);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5s_hal_registry_rows_backfilled() {
        assert!(k5s_hal_backfill_landed());
        for name in K5S_HAL_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5s HAL: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, H_9_HAL_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5t_hal_registry_rows_backfilled() {
        assert!(k5t_hal_backfill_landed());
        for name in K5T_HAL_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5t HAL: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, H_9_HAL_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5u_hal_registry_rows_backfilled() {
        assert!(k5u_hal_backfill_landed());
        for name in K5U_HAL_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5u H-8 HAL: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, H_8_HAL_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5v_m0_registry_rows_backfilled() {
        assert!(k5v_m0_backfill_landed());
        for name in K5V_M0_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5v M-0: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, M_0_MANIFOLD_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }

    #[test]
    fn k5v_crypto_registry_rows_backfilled() {
        assert!(k5v_crypto_backfill_landed());
        for name in K5V_CRYPTO_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .expect("registry row");
            assert!(
                !entry.derivation.is_pending(),
                "K-5v S-0 crypto: {name} must be backfilled"
            );
            assert_eq!(entry.derivation, S_0_CRYPTO_DEFINITION);
            assert_eq!(entry.derivation.label(), "Definition");
        }
    }
}

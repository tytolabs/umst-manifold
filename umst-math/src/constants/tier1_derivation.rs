// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! K-2 — Tier-1 canonical constant derivations (§14bis.k · §0.11 CDD).
//!
//! Backfills the four SSOT anchors cited in `egoffplan §14bis.k` K-2:
//! `LN_2`, `K_B`, `T_ROOM`, `RCC_FLOOR` → matching [`super::registry::REGISTRY`] rows.

use super::derivation::Derivation;
use super::registry::REGISTRY;

/// CODATA 2018 k page mirror (pinned snapshot under `authority-pins/`).
pub const K_B_AUTHORITY_URL: &str = "https://physics.nist.gov/cgi-bin/cuu/Value?k";

/// Pinned SHA-256 of [`authority-pins/codata2018_k_b.txt`](../../authority-pins/codata2018_k_b.txt).
pub const K_B_AUTHORITY_SHA256: &str =
    "fe430ef2d67dd311ee94374ed1fb1096a560c4d50851aa8a250da563eaece607";

/// Operator ambient anchor pin (local authority doc).
pub const T_ROOM_AUTHORITY_URL: &str = "umst-math/authority-pins/ambient_reference_300k.txt";

/// Pinned SHA-256 of [`authority-pins/ambient_reference_300k.txt`](../../authority-pins/ambient_reference_300k.txt).
pub const T_ROOM_AUTHORITY_SHA256: &str =
    "f310bf142186d5f4b34aa15bbc4d9556a90c9fc6144926451ada856b375402f7";

/// `LN_2` — theorem-derived via Mathlib / UMST ln(2) chain.
pub const LN_2_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Real.log_two_pos",
    expected_value: std::f64::consts::LN_2,
};

/// `K_B` — CODATA authority definition (NIST CUU mirror).
pub const K_B_DERIVATION: Derivation = Derivation::Definition {
    authority_url: K_B_AUTHORITY_URL,
    expected_sha256: K_B_AUTHORITY_SHA256,
};

/// `T_ROOM` — operator ambient reference anchor (300 K cockpit fallback).
pub const T_ROOM_DERIVATION: Derivation = Derivation::Definition {
    authority_url: T_ROOM_AUTHORITY_URL,
    expected_sha256: T_ROOM_AUTHORITY_SHA256,
};

/// `RCC_FLOOR` — theorem-derived residual-coherence lower bound.
pub const RCC_FLOOR_DERIVATION: Derivation = Derivation::Theorem {
    theorem_id: "UMST.Formal.Convergence::rcc_lower_bound",
    expected_value: 0.25,
};

/// Canonical K-2 symbolic ids (operator / egoffplan vocabulary).
pub const CANONICAL_SYMBOLS: &[&str] = &["LN_2", "K_B", "T_ROOM", "RCC_FLOOR"];

/// [`REGISTRY`] `name`s backfilled in K-2.
pub const K2_REGISTRY_ROW_NAMES: &[&str] = &[
    "ln_two_eta_cog_denominator",
    "k_boltzmann_j_per_k",
    "host_temperature_fallback_k",
    "rcc_floor_residual_coherence",
];

/// Lookup a K-2 backfilled derivation by registry row `name`.
#[must_use]
pub fn derivation_for_registry_row(name: &str) -> Option<Derivation> {
    match name {
        "ln_two_eta_cog_denominator" => Some(LN_2_DERIVATION),
        "k_boltzmann_j_per_k" => Some(K_B_DERIVATION),
        "host_temperature_fallback_k" => Some(T_ROOM_DERIVATION),
        "rcc_floor_residual_coherence" => Some(RCC_FLOOR_DERIVATION),
        _ => None,
    }
}

/// Count K-2 canonical rows with non-`Pending` derivation.
#[must_use]
pub fn k2_backfilled_count() -> usize {
    K2_REGISTRY_ROW_NAMES
        .iter()
        .filter(|name| {
            REGISTRY
                .iter()
                .find(|e| e.name == **name)
                .is_some_and(|e| !e.derivation.is_pending())
        })
        .count()
}

/// K-2 landed witness — all four canonical Tier-1 rows backfilled.
#[must_use]
pub fn k2_tier1_landed() -> bool {
    k2_backfilled_count() == K2_REGISTRY_ROW_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k2_canonical_derivations_non_pending() {
        assert!(!LN_2_DERIVATION.is_pending());
        assert!(!K_B_DERIVATION.is_pending());
        assert!(!T_ROOM_DERIVATION.is_pending());
        assert!(!RCC_FLOOR_DERIVATION.is_pending());
    }

    #[test]
    fn k2_registry_rows_backfilled() {
        assert!(k2_tier1_landed());
        assert_eq!(k2_backfilled_count(), 4);
        for name in K2_REGISTRY_ROW_NAMES {
            let entry = REGISTRY
                .iter()
                .find(|e| e.name == *name)
                .unwrap_or_else(|| panic!("missing K-2 row {name}"));
            assert!(
                !entry.derivation.is_pending(),
                "K-2: {name} must be backfilled"
            );
            assert_eq!(
                entry.derivation,
                derivation_for_registry_row(name).expect("lookup")
            );
        }
    }
}

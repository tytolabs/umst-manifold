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

/// Prep alias retained for scaffold witness (same shape as GREEN).
pub const ENERGY_BACKEND_DEFINITION_PREP: Derivation = ENERGY_BACKEND_DEFINITION;
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
}

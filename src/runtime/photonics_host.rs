// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host / CLI IO boundary for photonics DEC patch policy.
//!
//! [`crate::physics::solvers::PhotonicsDecPatchConfig`] is pure config with **no `std::env`**
//! reads in `src/physics/**`. Binaries, cartridge runners, and future `umst-cli` subcommands
//! should call [`photonics_dec_patch_config_from_env`] (or the pure parsers) here and inject
//! the result into [`crate::physics::solvers::PhotonicsSolver::dec_patch_config`].

#[cfg(feature = "photonics")]
use crate::physics::solvers::{
    DecPatchCsrInnerMode, PhotonicsDecPatchConfig, PhotonicsSolver,
};

/// Parse `UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV` (absent → `false`).
///
/// Only the literal `"1"` forces Krylov / skips dense Gauss–Jordan fallback — matches the
/// pre-`bb40684` physics-core env contract.
#[cfg(feature = "photonics")]
#[must_use]
pub fn parse_force_krylov_env(value: Option<&str>) -> bool {
    matches!(value, Some("1"))
}

/// Parse `UMST_PHOTONICS_DEC_PATCH_CSR_INNER` (absent → [`DecPatchCsrInnerMode::Auto`]).
#[cfg(feature = "photonics")]
#[must_use]
pub fn parse_csr_inner_env(value: Option<&str>) -> DecPatchCsrInnerMode {
    match value {
        Some(s) if s == "0" || s.eq_ignore_ascii_case("off") || s.eq_ignore_ascii_case("false") => {
            DecPatchCsrInnerMode::Off
        }
        Some(s)
            if s == "1"
                || s.eq_ignore_ascii_case("on")
                || s.eq_ignore_ascii_case("true")
                || s.eq_ignore_ascii_case("force") =>
        {
            DecPatchCsrInnerMode::On
        }
        _ => DecPatchCsrInnerMode::Auto,
    }
}

/// Read `UMST_PHOTONICS_DEC_PATCH_*` from the process environment.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_dec_patch_config_from_env() -> PhotonicsDecPatchConfig {
    PhotonicsDecPatchConfig {
        force_krylov: parse_force_krylov_env(
            std::env::var("UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV")
                .ok()
                .as_deref(),
        ),
        csr_inner: parse_csr_inner_env(
            std::env::var("UMST_PHOTONICS_DEC_PATCH_CSR_INNER")
                .ok()
                .as_deref(),
        ),
    }
}

/// Construct a [`PhotonicsSolver`] with host-injected DEC patch policy.
#[cfg(feature = "photonics")]
#[must_use]
pub fn photonics_solver_from_env(frequency_hz: f32) -> PhotonicsSolver {
    PhotonicsSolver {
        frequency_hz,
        dec_patch_config: photonics_dec_patch_config_from_env(),
    }
}

#[cfg(all(test, feature = "photonics"))]
mod tests {
    use super::*;

    #[test]
    fn force_krylov_only_literal_one() {
        assert!(!parse_force_krylov_env(None));
        assert!(!parse_force_krylov_env(Some("0")));
        assert!(!parse_force_krylov_env(Some("true")));
        assert!(parse_force_krylov_env(Some("1")));
    }

    #[test]
    fn csr_inner_env_aliases_match_legacy_physics() {
        assert_eq!(parse_csr_inner_env(None), DecPatchCsrInnerMode::Auto);
        assert_eq!(parse_csr_inner_env(Some("auto")), DecPatchCsrInnerMode::Auto);
        assert_eq!(parse_csr_inner_env(Some("OFF")), DecPatchCsrInnerMode::Off);
        assert_eq!(parse_csr_inner_env(Some("0")), DecPatchCsrInnerMode::Off);
        assert_eq!(parse_csr_inner_env(Some("false")), DecPatchCsrInnerMode::Off);
        assert_eq!(parse_csr_inner_env(Some("on")), DecPatchCsrInnerMode::On);
        assert_eq!(parse_csr_inner_env(Some("1")), DecPatchCsrInnerMode::On);
        assert_eq!(parse_csr_inner_env(Some("force")), DecPatchCsrInnerMode::On);
    }

    #[test]
    fn from_env_defaults_without_vars() {
        let cfg = photonics_dec_patch_config_from_env();
        assert!(!cfg.force_krylov);
        assert_eq!(cfg.csr_inner, DecPatchCsrInnerMode::Auto);
    }
}

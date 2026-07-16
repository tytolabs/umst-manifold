// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host / CLI IO boundary for PPO constraint slack weights.
//!
//! [`crate::ai::ppo::ManifoldGateway`] keeps **no `std::env`** reads in `src/ai/**`.
//! Binaries, cartridge runners, and future `umst-cli` subcommands should call
//! [`ppo_constraint_weights_from_env`] (or the pure parsers) here and inject via
//! [`crate::ai::ppo::ManifoldGateway::with_constraint_weights`].

/// Epistemic / Kleisli constraint slack weights parsed at the IO boundary.
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpoConstraintWeights {
    /// Clausius–Duhem slack weight **λ_cd**.
    pub lambda_cd: f32,
    /// Landauer erasure slack weight **λ_landauer**.
    pub lambda_landauer: f32,
}

#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
impl Default for PpoConstraintWeights {
    fn default() -> Self {
        Self {
            lambda_cd: 0.0_f32,
            lambda_landauer: 0.0_f32,
        }
    }
}

/// Parse `UMST_LAMBDA_CD` (absent / invalid → `0.0`).
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
#[must_use]
pub fn parse_lambda_cd_env(value: Option<&str>) -> f32 {
    value.and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0_f32)
}

/// Parse `UMST_LAMBDA_LANDAUER` (absent / invalid → `0.0`).
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
#[must_use]
pub fn parse_lambda_landauer_env(value: Option<&str>) -> f32 {
    value.and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0_f32)
}

/// Read `UMST_LAMBDA_CD` / `UMST_LAMBDA_LANDAUER` from the process environment.
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
#[must_use]
pub fn ppo_constraint_weights_from_env() -> PpoConstraintWeights {
    PpoConstraintWeights {
        lambda_cd: parse_lambda_cd_env(
            std::env::var("UMST_LAMBDA_CD").ok().as_deref(),
        ),
        lambda_landauer: parse_lambda_landauer_env(
            std::env::var("UMST_LAMBDA_LANDAUER").ok().as_deref(),
        ),
    }
}

#[cfg(all(test, any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
mod tests {
    use super::*;

    #[test]
    fn lambda_cd_absent_or_invalid_defaults_zero() {
        assert_eq!(parse_lambda_cd_env(None), 0.0);
        assert_eq!(parse_lambda_cd_env(Some("")), 0.0);
        assert_eq!(parse_lambda_cd_env(Some("nope")), 0.0);
    }

    #[test]
    fn lambda_cd_parses_finite_float() {
        assert_eq!(parse_lambda_cd_env(Some("0.5")), 0.5);
    }

    #[test]
    fn lambda_landauer_absent_defaults_zero() {
        assert_eq!(parse_lambda_landauer_env(None), 0.0);
    }

    #[test]
    fn lambda_landauer_parses_finite_float() {
        assert_eq!(parse_lambda_landauer_env(Some("1.25")), 1.25);
    }
}

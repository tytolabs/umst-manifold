// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! IO-adjacent and gateway boundary error types (FP manifesto §4).
//!
//! Distinct from [`crate::physics::error::PhysicsError`]: these surface at the UMST writeback /
//! policy gateway without routing filesystem or catalog IO through the physics core.

use core::fmt;

/// Failures from [`crate::ai::cbf::ThermodynamicCBF`] admissibility checks.
#[derive(Clone, Debug, PartialEq)]
pub enum CbfReject {
    /// Landauer erasure cost exceeds the agent's remaining energy credit.
    InsufficientGlobalEnergyCredit {
        required_j: f64,
        available_j: f64,
    },
    /// Clausius–Duhem inequality violated after Landauer debit.
    ClausiusDuhemViolation { generalized_entropy: f64 },
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl fmt::Display for CbfReject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CbfReject::InsufficientGlobalEnergyCredit {
                required_j,
                available_j,
            } => write!(
                f,
                "REJECTED: Insufficient Global Energy Credit. Required {required_j} J, Available {available_j} J."
            ),
            CbfReject::ClausiusDuhemViolation {
                generalized_entropy,
            } => write!(
                f,
                "REJECTED: Clausius-Duhem Violation. Generalized entropy {generalized_entropy} < 0."
            ),
            CbfReject::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<String> for CbfReject {
    fn from(detail: String) -> Self {
        CbfReject::LegacyDetail { detail }
    }
}

/// Failures reading Lean `catalog.json` for traceability partition (FP §4 IO boundary).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogIoError {
    /// Filesystem read of `catalog.json` failed.
    Read { detail: String },
    /// JSON parse of `catalog.json` failed.
    Json { detail: String },
    /// Top-level `modules` array absent or not an array.
    MissingModulesArray,
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl fmt::Display for CatalogIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CatalogIoError::Read { detail } | CatalogIoError::Json { detail } => f.write_str(detail),
            CatalogIoError::MissingModulesArray => {
                f.write_str("catalog.json missing modules array")
            }
            CatalogIoError::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<std::io::Error> for CatalogIoError {
    fn from(err: std::io::Error) -> Self {
        CatalogIoError::Read {
            detail: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for CatalogIoError {
    fn from(err: serde_json::Error) -> Self {
        CatalogIoError::Json {
            detail: err.to_string(),
        }
    }
}

impl From<String> for CatalogIoError {
    fn from(detail: String) -> Self {
        CatalogIoError::LegacyDetail { detail }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbf_insufficient_credit_display_preserves_legacy_wording() {
        let err = CbfReject::InsufficientGlobalEnergyCredit {
            required_j: 1.5,
            available_j: 0.25,
        };
        assert_eq!(
            err.to_string(),
            "REJECTED: Insufficient Global Energy Credit. Required 1.5 J, Available 0.25 J."
        );
    }

    #[test]
    fn cbf_clausius_duhem_display_preserves_legacy_wording() {
        let err = CbfReject::ClausiusDuhemViolation {
            generalized_entropy: -0.01,
        };
        assert_eq!(
            err.to_string(),
            "REJECTED: Clausius-Duhem Violation. Generalized entropy -0.01 < 0."
        );
    }

    #[test]
    fn catalog_io_missing_modules_display_preserves_legacy_wording() {
        assert_eq!(
            CatalogIoError::MissingModulesArray.to_string(),
            "catalog.json missing modules array"
        );
    }

    #[test]
    fn catalog_io_from_string_shim_round_trip() {
        let err = CatalogIoError::from("legacy catalog detail".to_string());
        assert_eq!(err.to_string(), "legacy catalog detail");
    }
}

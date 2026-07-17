// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! IO-adjacent and gateway boundary error types (FP manifesto §4).
//!
//! Distinct from [`crate::physics::error::PhysicsError`]: these surface at the UMST writeback /
//! policy gateway without routing filesystem or catalog IO through the physics core.

use core::fmt;

use crate::core::dec_typestate::DecTypestateError;

/// Failures from [`crate::core::apply_physics::apply_physics_to_umst`] UMST writeback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyPhysicsError {
    /// DEC typestate witness rejected on the live UMST bundle.
    DecTypestate {
        context: &'static str,
        source: DecTypestateError,
    },
    /// `scalar_features` width cannot index the damage channel.
    ScalarFeaturesTooSmallForDamage {
        width: usize,
        required_index: usize,
    },
    /// Sparse damage tensor node count disagrees with UMST layout.
    DamageWidthMismatch {
        damage_width: usize,
        umst_nodes: usize,
    },
    /// `scalar_features` width cannot index the temperature channel.
    ScalarFeaturesTooSmallForTemperature {
        width: usize,
        required_index: usize,
    },
    /// Temperature delta tensor node count disagrees with UMST layout.
    TemperatureWidthMismatch {
        delta_width: usize,
        umst_nodes: usize,
    },
    /// Legacy string shim for callers still bridging `Err(String)`.
    LegacyDetail { detail: String },
}

impl fmt::Display for ApplyPhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyPhysicsError::DecTypestate { context, source } => {
                write!(f, "apply_physics_to_umst: {context}: {source:?}")
            }
            ApplyPhysicsError::ScalarFeaturesTooSmallForDamage {
                width,
                required_index,
            } => write!(
                f,
                "apply_physics_to_umst: scalar_features width {width} too small for SCALAR_DAMAGE={required_index}"
            ),
            ApplyPhysicsError::DamageWidthMismatch {
                damage_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: damage width {damage_width} != UMST nodes {umst_nodes}"
            ),
            ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature {
                width,
                required_index,
            } => write!(
                f,
                "apply_physics_to_umst: scalar_features width {width} too small for SCALAR_TEMPERATURE={required_index}"
            ),
            ApplyPhysicsError::TemperatureWidthMismatch {
                delta_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: temperature_delta width {delta_width} != UMST nodes {umst_nodes}"
            ),
            ApplyPhysicsError::LegacyDetail { detail } => f.write_str(detail),
        }
    }
}

impl From<String> for ApplyPhysicsError {
    fn from(detail: String) -> Self {
        ApplyPhysicsError::LegacyDetail { detail }
    }
}

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
    fn apply_physics_dec_typestate_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::DecTypestate {
            context: "invalid SCALAR_DAMAGE channel",
            source: DecTypestateError::ScalarChannelOutOfRange {
                index: 99,
                channel_count: 8,
            },
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: invalid SCALAR_DAMAGE channel: ScalarChannelOutOfRange { index: 99, channel_count: 8 }"
        );
    }

    #[test]
    fn apply_physics_damage_width_display_preserves_legacy_wording() {
        let err = ApplyPhysicsError::DamageWidthMismatch {
            damage_width: 3,
            umst_nodes: 5,
        };
        assert_eq!(
            err.to_string(),
            "apply_physics_to_umst: damage width 3 != UMST nodes 5"
        );
    }

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

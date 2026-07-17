// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! IO-adjacent and gateway boundary error types (FP manifesto §4).
//!
//! Distinct from [`crate::physics::error::PhysicsError`]: these surface at the UMST writeback /
//! policy gateway without routing filesystem or catalog IO through the physics core.

use core::fmt;

use crate::core::dec_typestate::DecTypestateError;

/// Failures from [`crate::core::apply_physics::apply_physics_to_umst`] UMST writeback.
#[derive(Clone, Debug, PartialEq)]
pub enum ApplyPhysicsError {
    DecTypestate {
        context: &'static str,
        source: DecTypestateError,
    },
    ScalarFeaturesTooSmallForDamage {
        width: usize,
        required_index: usize,
    },
    DamageWidthMismatch {
        damage_width: usize,
        umst_nodes: usize,
    },
    ScalarFeaturesTooSmallForTemperature {
        width: usize,
        required_index: usize,
    },
    TemperatureWidthMismatch {
        delta_width: usize,
        umst_nodes: usize,
    },
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
            ApplyPhysicsError::ScalarFeaturesTooSmallForTemperature {
                width,
                required_index,
            } => write!(
                f,
                "apply_physics_to_umst: scalar_features width {width} too small for SCALAR_TEMPERATURE={required_index}"
            ),
            ApplyPhysicsError::DamageWidthMismatch {
                damage_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: damage width {damage_width} != UMST nodes {umst_nodes}"
            ),
            ApplyPhysicsError::TemperatureWidthMismatch {
                delta_width,
                umst_nodes,
            } => write!(
                f,
                "apply_physics_to_umst: temperature_delta width {delta_width} != UMST nodes {umst_nodes}"
            ),
        }
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
}

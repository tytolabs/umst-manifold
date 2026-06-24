// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured rejection reasons at the manifold policy gateway.
//!
//! CBF rejections carry [`LANDAUER_CBF_CATALOG_ID`] for telemetry alignment with
//! `docs/GateUnificationSpec.md`.
//!
//! The legacy [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`] surface keeps
//! [`String`] errors for backward compatibility; use [`FormalReject`] via
//! [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step_formal`] when you need
//! machine-readable witnesses.

pub use crate::runtime::catalog::traceability::LANDAUER_CBF_CATALOG_ID;

#[cfg(feature = "formal-witness")]
fn format_catalog_digest_hex(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    use std::fmt::Write as _;
    for b in bytes {
        write!(&mut s, "{b:02x}").expect("write to pre-allocated String");
    }
    s
}

/// Machine-readable rejection at the manifold gateway boundary.
///
/// The thermodynamic barrier variant preserves the wording expected from the legacy
/// `Err(String)` path; optional catalog hashing is gated by the **`formal-witness`**
/// crate feature.
#[derive(Clone, Eq, PartialEq)]
pub enum FormalReject {
    /// DEC typestate staging bundle rejected the proposed UMST layout before physics.
    DecTypestateStaging { detail: String },
    /// Clausius–Duhem / Landauer bookkeeping rejected the proposed transition (`ThermodynamicCBF`).
    ThermodynamicControlBarrier {
        catalog_id: &'static str,
        detail: String,
    },
    /// Runtime material catalog/schema digest disagree between gateway expectation and UMST carrier.
    #[cfg(feature = "formal-witness")]
    CatalogSchemaDigestMismatch {
        expected: [u8; 32],
        observed: [u8; 32],
    },
}

impl core::fmt::Debug for FormalReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FormalReject::DecTypestateStaging { detail } => f
                .debug_struct("DecTypestateStaging")
                .field("detail", detail)
                .finish(),
            FormalReject::ThermodynamicControlBarrier { catalog_id, detail } => f
                .debug_struct("ThermodynamicControlBarrier")
                .field("catalog_id", catalog_id)
                .field("detail", detail)
                .finish(),
            #[cfg(feature = "formal-witness")]
            FormalReject::CatalogSchemaDigestMismatch { expected, observed } => f
                .debug_struct("CatalogSchemaDigestMismatch")
                .field("expected", &format_catalog_digest_hex(expected))
                .field("observed", &format_catalog_digest_hex(observed))
                .finish(),
        }
    }
}

impl FormalReject {
    /// Stable gate / witness slug for telemetry (see `GateUnificationSpec.md`).
    pub fn catalog_id(&self) -> &'static str {
        match self {
            Self::DecTypestateStaging { .. } => "umst.gate.dec_typestate",
            Self::ThermodynamicControlBarrier { catalog_id, .. } => catalog_id,
            #[cfg(feature = "formal-witness")]
            Self::CatalogSchemaDigestMismatch { .. } => "umst.formal.catalog_lock",
        }
    }
}

impl core::fmt::Display for FormalReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FormalReject::DecTypestateStaging { detail } => {
                write!(f, "DEC typestate staging reject [umst.gate.dec_typestate]: {detail}")
            }
            FormalReject::ThermodynamicControlBarrier { catalog_id, detail } => {
                write!(f, "Transition Rejected by CBF [{catalog_id}]: {detail}")
            }
            #[cfg(feature = "formal-witness")]
            FormalReject::CatalogSchemaDigestMismatch { expected, observed } => write!(
                f,
                "Catalog schema digest mismatch [umst.formal.catalog_lock]: expected {}, observed {}",
                format_catalog_digest_hex(expected),
                format_catalog_digest_hex(observed),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "formal-witness")]
    #[test]
    fn catalog_digest_mismatch_carries_catalog_lock_slug() {
        let rej = FormalReject::CatalogSchemaDigestMismatch {
            expected: [1u8; 32],
            observed: [2u8; 32],
        };
        assert_eq!(rej.catalog_id(), "umst.formal.catalog_lock");
        assert!(
            rej.to_string().contains("umst.formal.catalog_lock"),
            "Display must embed catalog_id"
        );
    }

    #[test]
    fn cbf_reject_carries_landauer_catalog_id() {
        let rej = FormalReject::ThermodynamicControlBarrier {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            detail: "insufficient dissipation".into(),
        };
        assert_eq!(rej.catalog_id(), "umst.gate.landauer_cbf");
        assert!(
            rej.to_string().contains("umst.gate.landauer_cbf"),
            "Display must embed catalog_id for telemetry parsers"
        );
    }
}

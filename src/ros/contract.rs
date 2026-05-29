// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Serializable payloads shared with ROS 2 bridges. Enable the `serde` crate feature for derives.

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GateDecisionPayload {
    /// Should align bundle policy with [`crate::manifest::UmstManifest::compiled_catalog_lock_bundle_sha256_hex`] in strict deployments.
    pub catalog_hash: [u8; 32],
    pub gate_lane_id: u32,
    pub admitted: bool,
    /// Unitless or normalized margin used for logging / ranking (bridge-defined).
    pub residual_margin: f64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MixProposalPayload {
    pub catalog_hash: [u8; 32],
    pub mix_epoch: u64,
    /// Secondary digest (e.g. proposal content id); bridge-defined.
    pub proposal_digest: [u8; 32],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryFramePayload {
    pub catalog_hash: [u8; 32],
    pub frame_seq: u64,
    pub wall_time_ns: u128,
}

/// Human-readable SHA-256 fingerprint of the verbatim `artifacts/catalog.lock.json`
/// (build-time env `UMST_CATALOG_LOCK_SHA256_HEX`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UmstCatalogHeader {
    pub catalog_bundle_sha256_hex: String,
}

/// DDS/JSON-friendly gate acknowledgement (string catalog lane / hex bundle).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct UmstGateAckV1 {
    pub schema_tag: String,
    pub catalog_hash: String,
    pub gate_catalog_id: String,
    pub admissible: bool,
}

impl UmstGateAckV1 {
    #[must_use]
    pub fn new(
        catalog_hash: impl Into<String>,
        gate_catalog_id: impl Into<String>,
        admissible: bool,
    ) -> Self {
        UmstGateAckV1 {
            schema_tag: "umst.gate_ack.v1".into(),
            catalog_hash: catalog_hash.into(),
            gate_catalog_id: gate_catalog_id.into(),
            admissible,
        }
    }
}

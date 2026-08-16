// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Serializable payloads shared with ROS 2 bridges. Enable the `serde` crate feature for derives.
//!
//! Honest fence: these are **bridge DTOs only** — shape / schema validation for DDS/JSON
//! interchange. Passing well-formedness does **not** attest physics GREEN, PRODUCTION_WIRED,
//! MASTER, or OP-5.

/// SHA-256 hex digest length (64 lowercase/uppercase ASCII hex digits).
pub const SHA256_HEX_LEN: usize = 64;

/// Canonical schema tag for [`UmstGateAckV1`].
pub const UMST_GATE_ACK_V1_SCHEMA_TAG: &str = "umst.gate_ack.v1";

/// True iff `hex` is exactly 64 ASCII hex digits (SHA-256 fingerprint shape).
#[must_use]
pub fn is_sha256_hex(hex: &str) -> bool {
    hex.len() == SHA256_HEX_LEN && hex.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Explicit non-claim: ROS contract DTOs are not a production wire attestation.
#[must_use]
pub const fn ros_contract_production_wired() -> bool {
    false
}

/// Explicit non-claim: DTO well-formedness ≠ physics GREEN / MASTER / OP-5.
#[must_use]
pub const fn ros_contract_physics_green() -> bool {
    false
}

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

/// Shape fence for [`GateDecisionPayload`] (finite margin only; hash is fixed-width).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecisionWellFormedError {
    ResidualMarginNotFinite,
}

impl GateDecisionPayload {
    #[must_use]
    pub fn new(
        catalog_hash: [u8; 32],
        gate_lane_id: u32,
        admitted: bool,
        residual_margin: f64,
    ) -> Self {
        Self {
            catalog_hash,
            gate_lane_id,
            admitted,
            residual_margin,
        }
    }

    /// Bridge shape check — not an admissibility / GREEN attestation.
    pub fn check_well_formed(&self) -> Result<(), GateDecisionWellFormedError> {
        if !self.residual_margin.is_finite() {
            return Err(GateDecisionWellFormedError::ResidualMarginNotFinite);
        }
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MixProposalPayload {
    pub catalog_hash: [u8; 32],
    pub mix_epoch: u64,
    /// Secondary digest (e.g. proposal content id); bridge-defined.
    pub proposal_digest: [u8; 32],
}

impl MixProposalPayload {
    #[must_use]
    pub fn new(catalog_hash: [u8; 32], mix_epoch: u64, proposal_digest: [u8; 32]) -> Self {
        Self {
            catalog_hash,
            mix_epoch,
            proposal_digest,
        }
    }

    /// Fixed-width digests are always shape-valid; kept for symmetric fence API.
    pub fn check_well_formed(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryFramePayload {
    pub catalog_hash: [u8; 32],
    pub frame_seq: u64,
    pub wall_time_ns: u128,
}

impl TelemetryFramePayload {
    #[must_use]
    pub fn new(catalog_hash: [u8; 32], frame_seq: u64, wall_time_ns: u128) -> Self {
        Self {
            catalog_hash,
            frame_seq,
            wall_time_ns,
        }
    }

    /// Fixed-width catalog digest is always shape-valid; kept for symmetric fence API.
    pub fn check_well_formed(&self) -> Result<(), ()> {
        Ok(())
    }
}

/// Human-readable SHA-256 fingerprint of the verbatim `artifacts/catalog.lock.json`
/// (build-time env `UMST_CATALOG_LOCK_SHA256_HEX`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UmstCatalogHeader {
    pub catalog_bundle_sha256_hex: String,
}

/// Shape fence for [`UmstCatalogHeader`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogHeaderWellFormedError {
    BundleSha256HexInvalid,
}

impl UmstCatalogHeader {
    #[must_use]
    pub fn new(catalog_bundle_sha256_hex: impl Into<String>) -> Self {
        Self {
            catalog_bundle_sha256_hex: catalog_bundle_sha256_hex.into(),
        }
    }

    /// Requires 64 ASCII hex digits — fingerprint shape only, not lock-file equality.
    pub fn check_well_formed(&self) -> Result<(), CatalogHeaderWellFormedError> {
        if !is_sha256_hex(&self.catalog_bundle_sha256_hex) {
            return Err(CatalogHeaderWellFormedError::BundleSha256HexInvalid);
        }
        Ok(())
    }
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

/// Shape / schema fence for [`UmstGateAckV1`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateAckWellFormedError {
    SchemaTagMismatch,
    CatalogHashNotSha256Hex,
    GateCatalogIdEmpty,
}

impl UmstGateAckV1 {
    pub const SCHEMA_TAG: &'static str = UMST_GATE_ACK_V1_SCHEMA_TAG;

    #[must_use]
    pub fn new(
        catalog_hash: impl Into<String>,
        gate_catalog_id: impl Into<String>,
        admissible: bool,
    ) -> Self {
        UmstGateAckV1 {
            schema_tag: Self::SCHEMA_TAG.into(),
            catalog_hash: catalog_hash.into(),
            gate_catalog_id: gate_catalog_id.into(),
            admissible,
        }
    }

    /// Schema tag + SHA-256 hex catalog fingerprint + non-empty gate id.
    /// `admissible` is a bridge field — this check does **not** re-run the physics gate.
    pub fn check_well_formed(&self) -> Result<(), GateAckWellFormedError> {
        if self.schema_tag != Self::SCHEMA_TAG {
            return Err(GateAckWellFormedError::SchemaTagMismatch);
        }
        if !is_sha256_hex(&self.catalog_hash) {
            return Err(GateAckWellFormedError::CatalogHashNotSha256Hex);
        }
        if self.gate_catalog_id.is_empty() {
            return Err(GateAckWellFormedError::GateCatalogIdEmpty);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hash(byte: u8) -> [u8; 32] {
        let mut h = [0_u8; 32];
        h[0] = byte;
        h[31] = byte.wrapping_add(1);
        h
    }

    fn sample_sha256_hex(nibble: u8) -> String {
        let c = char::from_digit(u32::from(nibble & 0x0f), 16).unwrap_or('0');
        std::iter::repeat(c).take(SHA256_HEX_LEN).collect()
    }

    #[test]
    fn contract_sha256_hex_fence() {
        assert!(is_sha256_hex(&sample_sha256_hex(0xa)));
        assert!(!is_sha256_hex("deadbeef"));
        assert!(!is_sha256_hex(&format!("{}g", "0".repeat(63))));
    }

    #[test]
    fn contract_honest_non_claims() {
        assert!(!ros_contract_production_wired());
        assert!(!ros_contract_physics_green());
    }

    #[test]
    fn contract_gate_decision_margin_fence() {
        let ok = GateDecisionPayload::new(sample_hash(1), 7, true, 1.0e-6);
        assert!(ok.check_well_formed().is_ok());
        let bad = GateDecisionPayload::new(sample_hash(1), 7, false, f64::NAN);
        assert_eq!(
            bad.check_well_formed(),
            Err(GateDecisionWellFormedError::ResidualMarginNotFinite)
        );
    }

    #[test]
    fn contract_mix_and_telemetry_constructors() {
        let mix = MixProposalPayload::new(sample_hash(2), 9, sample_hash(3));
        assert!(mix.check_well_formed().is_ok());
        let tel = TelemetryFramePayload::new(sample_hash(4), 128, 1_700_000_000_000u128);
        assert!(tel.check_well_formed().is_ok());
        assert_eq!(tel.frame_seq, 128);
    }

    #[test]
    fn contract_catalog_header_hex_fence() {
        let ok = UmstCatalogHeader::new(sample_sha256_hex(0xb));
        assert!(ok.check_well_formed().is_ok());
        let bad = UmstCatalogHeader::new("not-a-digest");
        assert_eq!(
            bad.check_well_formed(),
            Err(CatalogHeaderWellFormedError::BundleSha256HexInvalid)
        );
    }

    #[test]
    fn contract_gate_ack_schema_and_hex_fences() {
        let ok = UmstGateAckV1::new(sample_sha256_hex(0xc), "gate.lane.demo", true);
        assert_eq!(ok.schema_tag, UMST_GATE_ACK_V1_SCHEMA_TAG);
        assert!(ok.check_well_formed().is_ok());

        let mut bad_tag = ok.clone();
        bad_tag.schema_tag = "umst.gate_ack.v0".into();
        assert_eq!(
            bad_tag.check_well_formed(),
            Err(GateAckWellFormedError::SchemaTagMismatch)
        );

        let bad_hash = UmstGateAckV1::new("short", "gate.lane.demo", false);
        assert_eq!(
            bad_hash.check_well_formed(),
            Err(GateAckWellFormedError::CatalogHashNotSha256Hex)
        );

        let bad_id = UmstGateAckV1::new(sample_sha256_hex(0xd), "", true);
        assert_eq!(
            bad_id.check_well_formed(),
            Err(GateAckWellFormedError::GateCatalogIdEmpty)
        );
    }
}

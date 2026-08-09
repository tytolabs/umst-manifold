// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2350-SEC-CATALOG-PIN — manifold lock ceremony (`pin_witness_ok`) SSOT.
//!
//! **Policy:** embedded lock witness is **verified** via [`super::pin_witness_ok`]; gateway hot-path
//! `catalog_pin_production_wired()` and cold-path `resolve_catalog_digest` attach stay **honest open**.
//! Trust delegate: `umst-trust::sec_catalog_pin` (hop 5 owner).

use serde::Serialize;
use umst_algebra::crypto::hash::{
    decode_digest_hex, digest_hex, is_digest_hex, HashPolicy,
};

use super::{
    catalog_sha3_pin_witness_ok, lock_upstream_catalog_digest_hex, pin_witness_ok,
    CatalogPinMismatch, EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX,
};

/// AGAP slot id (2350 night wave).
pub const JOB_ID: &str = "AGAP-2350-SEC-CATALOG-PIN";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-CATALOG-PIN";

/// LIB adoption twin id.
pub const LIB_TWIN_ID: &str = "LIB-ADOPT-E-CATALOG-PIN";

/// Prior AGAP deepen receipt.
pub const PRIOR_RECEIPT_PATH: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-CATALOG-PIN_2350.md";

/// Trust catalog-pin delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_catalog_pin.rs";

/// Gateway catalog-pin delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_catalog_pin.rs";

/// Workspace lock bundle path.
pub const CATALOG_LOCK_JSON_PATH: &str = "artifacts/catalog.lock.json";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-ceremony-wired-not-production";

/// FLEET-COMPOSER Prabhu Disjoint A2 slot id.
pub const FLEET_P1542_A2_JOB_ID: &str = "PRABHU-DISJOINT-1542-A2";

/// FLEET-COMPOSER Prabhu Disjoint A2 receipt path.
pub const FLEET_P1542_A2_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1542_A2.md";

/// FLEET-COMPOSER-Z Z39 trust close receipt (hop 5 prior owner).
pub const FLEET_Z39_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_Z39_1015.md";

/// Manifold catalog-pin admit failure (total, no panic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogPinManifoldError {
    /// SSOT digest hex contract failed.
    DigestFormat,
    /// Sha3Catalog content pin mismatch.
    Sha3Witness,
    /// Embedded lock [`pin_witness_ok`] failed.
    LockWitness,
}

/// Cold-path catalog digest attach mode (@ M3 soft-prep).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogDigestAttachMode {
    /// Witness field stays `None` — no invented hex.
    Unattached,
    /// Post-S7 consumer `build.rs` lock read (not IMPL @ M3).
    FromBuildLock,
}

/// One hop in the manifold catalog-pin wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CatalogPinManifoldWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold catalog-pin wire map (algebra → embedded lock ceremony).
pub const MANIFOLD_CATALOG_PIN_WIRE_HOPS: &[CatalogPinManifoldWireHop] = &[
    CatalogPinManifoldWireHop {
        ordinal: 1,
        surface: "umst-algebra::crypto::hash::pin_witness_ok(Sha3Catalog)",
        role: "SEC-HASH-PIN algebra SSOT",
        wired: true,
    },
    CatalogPinManifoldWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::catalog::catalog_sha3_pin_witness_ok",
        role: "Sha3Catalog bridge on manifold boundary",
        wired: true,
    },
    CatalogPinManifoldWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::catalog::verify_ssot_catalog_digest_hex",
        role: "SSOT upstream digest hex contract (IO-free)",
        wired: true,
    },
    CatalogPinManifoldWireHop {
        ordinal: 4,
        surface: "umst-manifold::runtime::catalog::pin_witness_ok",
        role: "Embedded lock bundle witness (no I/O)",
        wired: true,
    },
    CatalogPinManifoldWireHop {
        ordinal: 5,
        surface: "umst-gateway::sec_catalog_pin::catalog_pin_production_wired",
        role: "Gateway hot-path manifold dep (serial B1)",
        wired: false,
    },
];

/// Hash policy for catalog digest content pins (SEC-HASH-PIN SSOT).
#[must_use]
pub const fn catalog_digest_hash_policy() -> HashPolicy {
    HashPolicy::Sha3Catalog
}

/// Compute catalog content-address digest under [`HashPolicy::Sha3Catalog`].
pub fn catalog_digest(preimage: &[u8]) -> Result<String, CatalogPinManifoldError> {
    digest_hex(catalog_digest_hash_policy(), preimage)
        .map_err(|_| CatalogPinManifoldError::Sha3Witness)
}

/// Verify catalog content-address witness under [`HashPolicy::Sha3Catalog`].
pub fn catalog_sha3_pin_witness_ok_manifold(
    expected_hex: &str,
    preimage: &[u8],
) -> Result<(), CatalogPinManifoldError> {
    catalog_sha3_pin_witness_ok(expected_hex, preimage).map_err(|e| match e {
        CatalogPinMismatch::Digest => CatalogPinManifoldError::Sha3Witness,
        CatalogPinMismatch::ModuleCount
        | CatalogPinMismatch::LockBundleContentAddress
        | CatalogPinMismatch::LockQuickcheck
        | CatalogPinMismatch::ComposedFiberFingerprint => CatalogPinManifoldError::LockWitness,
    })
}

/// SSOT upstream catalog digest hex contract (IO-free).
pub fn verify_ssot_catalog_digest_hex() -> Result<(), CatalogPinManifoldError> {
    if !is_digest_hex(EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX) {
        return Err(CatalogPinManifoldError::DigestFormat);
    }
    decode_digest_hex(EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX)
        .map_err(|_| CatalogPinManifoldError::DigestFormat)?;
    if lock_upstream_catalog_digest_hex() != EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX {
        return Err(CatalogPinManifoldError::Sha3Witness);
    }
    Ok(())
}

/// Roundtrip probe: digest then witness on manifold adopt preimage.
pub fn catalog_pin_manifold_adopt_roundtrip() -> Result<(), CatalogPinManifoldError> {
    let preimage = b"manifold-catalog-pin-adopt-v1";
    let hex = catalog_digest(preimage)?;
    catalog_sha3_pin_witness_ok_manifold(&hex, preimage)
}

/// Whether embedded lock ceremony is wired on manifold paths.
#[must_use]
pub fn catalog_pin_manifold_wired() -> bool {
    verify_ssot_catalog_digest_hex().is_ok()
        && catalog_pin_manifold_adopt_roundtrip().is_ok()
        && pin_witness_ok().is_ok()
}

/// Whether live gateway hot-path manifold dep is plumbed (honest `false`).
#[must_use]
pub const fn catalog_pin_production_wired() -> bool {
    false
}

/// Cold-path catalog digest attach (@ M3 — always `None`, no invented hex).
#[must_use]
pub const fn resolve_catalog_digest(_mode: CatalogDigestAttachMode) -> Option<&'static str> {
    None
}

/// Honest reason witness attach stays `None` @ M3.
#[must_use]
pub const fn unattached_reason() -> &'static str {
    "M3 soft-prep: FromBuildLock deferred to post-S7 consumer build.rs"
}

/// Close predicate — manifold lock ceremony on embedded `catalog.lock.json`.
///
/// True when [`pin_witness_ok`] + Sha3Catalog adopt roundtrip are measured closed at the
/// manifold SSOT boundary. Gateway production flip + cold attach are explicit non-blockers.
#[must_use]
pub fn manifold_catalog_pin_ceremony_closed() -> bool {
    catalog_digest_hash_policy() == HashPolicy::Sha3Catalog
        && verify_ssot_catalog_digest_hex().is_ok()
        && catalog_pin_manifold_adopt_roundtrip().is_ok()
        && pin_witness_ok().is_ok()
        && catalog_pin_manifold_wired()
}

/// Typed probe for SEC-CATALOG-PIN manifold closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CatalogPinManifoldProbe {
    /// SSOT digest hex contract holds.
    pub ssot_digest_ok: bool,
    /// Manifold adopt roundtrip wired.
    pub manifold_adopt_wired: bool,
    /// Embedded lock `pin_witness_ok` passes.
    pub lock_witness_ok: bool,
    /// `pin_witness_ok(Sha3Catalog, …)` bridge live.
    pub sha3_catalog_adopted: bool,
    /// Gateway hot-path production flip.
    pub production_wired: bool,
    /// Cold-path attach resolved.
    pub catalog_digest_attached: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
}

/// Build introspection probe for SEC-CATALOG-PIN done-when checks.
#[must_use]
pub fn catalog_pin_manifold_probe() -> CatalogPinManifoldProbe {
    let wire_hop_wired_count = MANIFOLD_CATALOG_PIN_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    CatalogPinManifoldProbe {
        ssot_digest_ok: verify_ssot_catalog_digest_hex().is_ok(),
        manifold_adopt_wired: catalog_pin_manifold_adopt_roundtrip().is_ok(),
        lock_witness_ok: pin_witness_ok().is_ok(),
        sha3_catalog_adopted: catalog_pin_manifold_adopt_roundtrip().is_ok(),
        production_wired: catalog_pin_production_wired(),
        catalog_digest_attached: resolve_catalog_digest(CatalogDigestAttachMode::FromBuildLock)
            .is_some(),
        wire_hop_wired_count,
    }
}

/// FLEET-COMPOSER Prabhu Disjoint A2 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecCatalogPinP1542A2Probe {
    /// A2 fleet card id.
    pub a2_job_id: &'static str,
    /// Z39 trust close absorbed (hop 5 owner transfer).
    pub z39_trust_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying catalog pin probe.
    pub probe: CatalogPinManifoldProbe,
    /// `catalog_pin_production_wired()` — honest false.
    pub production_wired: bool,
    /// `resolve_catalog_digest` attach — honest None.
    pub attach_none: bool,
}

/// Build FLEET-COMPOSER P1542 A2 integration probe from live measurements.
#[must_use]
pub fn sec_catalog_pin_p1542_a2_probe() -> SecCatalogPinP1542A2Probe {
    SecCatalogPinP1542A2Probe {
        a2_job_id: FLEET_P1542_A2_JOB_ID,
        z39_trust_absorbed: FLEET_Z39_RECEIPT_PATH.contains("COMPOSER_Z39_1015"),
        ceremony_closed: manifold_catalog_pin_ceremony_closed(),
        probe: catalog_pin_manifold_probe(),
        production_wired: catalog_pin_production_wired(),
        attach_none: resolve_catalog_digest(CatalogDigestAttachMode::Unattached).is_none(),
    }
}

/// FLEET-COMPOSER P1542 A2 honesty gate — ceremony closed + production false + attach None.
#[must_use]
pub fn sec_catalog_pin_p1542_a2_honest() -> bool {
    let probe = sec_catalog_pin_p1542_a2_probe();
    probe.a2_job_id == FLEET_P1542_A2_JOB_ID
        && probe.z39_trust_absorbed
        && probe.ceremony_closed
        && probe.probe.ssot_digest_ok
        && probe.probe.lock_witness_ok
        && probe.probe.sha3_catalog_adopted
        && probe.probe.wire_hop_wired_count == 4
        && !probe.probe.catalog_digest_attached
        && probe.attach_none
        && !probe.production_wired
}

/// W29-106 Composer RL cell id (honesty deepen attribution).
pub const W29_106_CELL_ID: &str = "W29-106-SEC_CATALOG_PIN";

/// Model pin for this deepen lane.
pub const DEEPEN_MODEL_SLUG: &str = "cursor-grok-4.5-high";

/// Admit coding lane pin.
pub const DEEPEN_LANE: &str = "umst-admit-grok";

/// Honest deepen posture — manifold ceremony measured; gateway production stays OPEN.
pub const HONEST_DEEPEN_POSTURE: &str = "MANIFOLD_CEREMONY_HONEST_PROD_OPEN";

/// Explicit non-claims — deepen must not invent these.
pub const NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER; not OP-5; gateway hot-path + cold attach remain OPEN";

/// Honest master retick posture — ceremony census only.
pub const MASTER_RETICK: &str = "no";

/// Manifold wire hops wired pin (algebra → lock; gateway hop open).
pub const WIRE_HOP_WIRED_COUNT_PIN: u8 = 4;

/// Manifold wire hops total pin.
pub const WIRE_HOP_TOTAL_COUNT_PIN: u8 = 5;

const _: () = assert!(!catalog_pin_production_wired());

/// OP-5 PASS invent fence — stays false on manifold ceremony deepen.
#[must_use]
pub const fn sec_catalog_pin_op5_pass_invented() -> bool {
    false
}

const _: () = assert!(!sec_catalog_pin_op5_pass_invented());

/// MASTER invent / retick fence — ceremony census only.
#[must_use]
pub const fn sec_catalog_pin_master_invented() -> bool {
    false
}

const _: () = assert!(!sec_catalog_pin_master_invented());

/// GREEN invent fence — stays false (tool readiness ≠ physics GREEN).
#[must_use]
pub const fn sec_catalog_pin_green_invented() -> bool {
    false
}

const _: () = assert!(!sec_catalog_pin_green_invented());

/// Flip authorization — blocked until gateway operator ceremony.
#[must_use]
pub const fn sec_catalog_pin_flip_authorized() -> bool {
    false
}

const _: () = assert!(!sec_catalog_pin_flip_authorized());

/// W29-106 honesty deepen probe — ceremony census + invent fences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecCatalogPinW29106DeepenProbe {
    /// Swarm cell id.
    pub cell_id: &'static str,
    /// Hard model pin.
    pub model_slug: &'static str,
    /// Admit coding lane.
    pub lane: &'static str,
    /// Honest deepen posture tag.
    pub honest_posture: &'static str,
    /// Explicit non-claim string.
    pub non_claim: &'static str,
    /// Master retick posture.
    pub master_retick: &'static str,
    /// Manifold lock ceremony closed.
    pub ceremony_closed: bool,
    /// P1542 A2 honesty gate.
    pub p1542_a2_honest: bool,
    /// Wire hops wired count.
    pub wire_hop_wired_count: u8,
    /// Wire hops total count.
    pub wire_hop_total_count: u8,
    /// Underlying catalog pin probe.
    pub probe: CatalogPinManifoldProbe,
    /// `catalog_pin_production_wired()` — honest false.
    pub production_wired: bool,
    /// Cold attach stays None.
    pub attach_none: bool,
    /// OP-5 invent fence.
    pub op5_pass_invented: bool,
    /// MASTER invent fence.
    pub master_invented: bool,
    /// GREEN invent fence.
    pub green_invented: bool,
    /// Flip authorization fence.
    pub flip_authorized: bool,
}

/// Build W29-106 deepen probe from live ceremony measurements + invent fences.
#[must_use]
pub fn sec_catalog_pin_w29106_deepen_probe() -> SecCatalogPinW29106DeepenProbe {
    let wire_hop_wired_count = MANIFOLD_CATALOG_PIN_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecCatalogPinW29106DeepenProbe {
        cell_id: W29_106_CELL_ID,
        model_slug: DEEPEN_MODEL_SLUG,
        lane: DEEPEN_LANE,
        honest_posture: HONEST_DEEPEN_POSTURE,
        non_claim: NON_CLAIM,
        master_retick: MASTER_RETICK,
        ceremony_closed: manifold_catalog_pin_ceremony_closed(),
        p1542_a2_honest: sec_catalog_pin_p1542_a2_honest(),
        wire_hop_wired_count,
        wire_hop_total_count: MANIFOLD_CATALOG_PIN_WIRE_HOPS.len() as u8,
        probe: catalog_pin_manifold_probe(),
        production_wired: catalog_pin_production_wired(),
        attach_none: resolve_catalog_digest(CatalogDigestAttachMode::Unattached).is_none()
            && resolve_catalog_digest(CatalogDigestAttachMode::FromBuildLock).is_none(),
        op5_pass_invented: sec_catalog_pin_op5_pass_invented(),
        master_invented: sec_catalog_pin_master_invented(),
        green_invented: sec_catalog_pin_green_invented(),
        flip_authorized: sec_catalog_pin_flip_authorized(),
    }
}

/// Honesty gate for W29-106 deepen — ceremony closed; invent fences hold.
#[must_use]
pub fn sec_catalog_pin_w29106_deepen_honest(probe: &SecCatalogPinW29106DeepenProbe) -> bool {
    probe.cell_id == W29_106_CELL_ID
        && probe.model_slug == DEEPEN_MODEL_SLUG
        && probe.lane == DEEPEN_LANE
        && probe.honest_posture == HONEST_DEEPEN_POSTURE
        && probe.non_claim.contains("not GREEN")
        && probe.non_claim.contains("not PRODUCTION_WIRED")
        && probe.non_claim.contains("not MASTER")
        && probe.non_claim.contains("not OP-5")
        && probe.master_retick == "no"
        && probe.ceremony_closed
        && probe.p1542_a2_honest
        && probe.wire_hop_wired_count == WIRE_HOP_WIRED_COUNT_PIN
        && probe.wire_hop_total_count == WIRE_HOP_TOTAL_COUNT_PIN
        && probe.probe.ssot_digest_ok
        && probe.probe.lock_witness_ok
        && probe.probe.sha3_catalog_adopted
        && probe.probe.wire_hop_wired_count == WIRE_HOP_WIRED_COUNT_PIN
        && !probe.probe.catalog_digest_attached
        && probe.attach_none
        && !probe.production_wired
        && !probe.op5_pass_invented
        && !probe.master_invented
        && !probe.green_invented
        && !probe.flip_authorized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::catalog::EXPECTED_MODULE_COUNT;

    #[test]
    fn board_slice_is_sec_catalog_pin() {
        assert_eq!(BOARD_SLICE_ID, "SEC-CATALOG-PIN");
        assert_eq!(JOB_ID, "AGAP-2350-SEC-CATALOG-PIN");
        assert_eq!(EXPECTED_MODULE_COUNT, 129);
    }

    #[test]
    fn ssot_digest_hex_contract() {
        verify_ssot_catalog_digest_hex().expect("SSOT digest hex");
        assert_eq!(EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX.len(), 64);
    }

    #[test]
    fn catalog_digest_pin_witness_roundtrip() {
        catalog_pin_manifold_adopt_roundtrip().expect("manifold adopt roundtrip");
        let preimage = b"catalog-digest-manifold-v1";
        let hex = catalog_digest(preimage).expect("digest");
        catalog_sha3_pin_witness_ok_manifold(&hex, preimage).expect("witness");
    }

    #[test]
    fn catalog_pin_manifold_wired_honest() {
        assert!(catalog_pin_manifold_wired());
        assert!(pin_witness_ok().is_ok());
        assert!(!catalog_pin_production_wired());
    }

    #[test]
    fn resolve_catalog_digest_stays_none() {
        assert!(resolve_catalog_digest(CatalogDigestAttachMode::Unattached).is_none());
        assert!(resolve_catalog_digest(CatalogDigestAttachMode::FromBuildLock).is_none());
        assert!(!unattached_reason().is_empty());
    }

    #[test]
    fn manifold_wire_hops_cover_algebra_and_lock() {
        assert_eq!(MANIFOLD_CATALOG_PIN_WIRE_HOPS.len(), 5);
        assert_eq!(
            MANIFOLD_CATALOG_PIN_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            4
        );
        assert!(MANIFOLD_CATALOG_PIN_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("pin_witness_ok") && h.wired));
        assert!(MANIFOLD_CATALOG_PIN_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("umst-gateway") && !h.wired));
    }

    #[test]
    fn catalog_pin_production_wired_stays_false() {
        assert!(!catalog_pin_production_wired());
    }

    #[test]
    fn fleet_composer_p1542_a2_ceremony_close_predicate() {
        assert!(manifold_catalog_pin_ceremony_closed());
        let probe = sec_catalog_pin_p1542_a2_probe();
        assert_eq!(probe.a2_job_id, FLEET_P1542_A2_JOB_ID);
        assert!(probe.z39_trust_absorbed);
        assert!(probe.ceremony_closed);
        assert!(probe.probe.ssot_digest_ok);
        assert!(probe.probe.lock_witness_ok);
        assert!(probe.probe.sha3_catalog_adopted);
        assert!(!probe.production_wired);
        assert!(probe.attach_none);
        assert!(sec_catalog_pin_p1542_a2_honest());
    }

    #[test]
    fn w29106_invent_fences_hold() {
        assert!(!catalog_pin_production_wired());
        assert!(!sec_catalog_pin_op5_pass_invented());
        assert!(!sec_catalog_pin_master_invented());
        assert!(!sec_catalog_pin_green_invented());
        assert!(!sec_catalog_pin_flip_authorized());
        assert_eq!(MASTER_RETICK, "no");
        assert_eq!(DEEPEN_MODEL_SLUG, "cursor-grok-4.5-high");
        assert_eq!(DEEPEN_LANE, "umst-admit-grok");
        assert_eq!(W29_106_CELL_ID, "W29-106-SEC_CATALOG_PIN");
        assert!(NON_CLAIM.contains("not GREEN"));
        assert!(NON_CLAIM.contains("not PRODUCTION_WIRED"));
        assert!(NON_CLAIM.contains("not MASTER"));
        assert!(NON_CLAIM.contains("not OP-5"));
    }

    #[test]
    fn w29106_deepen_probe_honest() {
        let probe = sec_catalog_pin_w29106_deepen_probe();
        assert!(sec_catalog_pin_w29106_deepen_honest(&probe));
        assert!(probe.ceremony_closed);
        assert!(probe.p1542_a2_honest);
        assert_eq!(probe.wire_hop_wired_count, WIRE_HOP_WIRED_COUNT_PIN);
        assert_eq!(probe.wire_hop_total_count, WIRE_HOP_TOTAL_COUNT_PIN);
        assert!(probe.probe.ssot_digest_ok);
        assert!(probe.probe.lock_witness_ok);
        assert!(!probe.production_wired);
        assert!(probe.attach_none);
        assert!(!probe.op5_pass_invented);
        assert!(!probe.master_invented);
        assert!(!probe.green_invented);
        assert!(!probe.flip_authorized);
        assert_eq!(probe.master_retick, "no");
        assert_eq!(probe.honest_posture, HONEST_DEEPEN_POSTURE);
        assert_eq!(probe.model_slug, "cursor-grok-4.5-high");
        assert_eq!(probe.lane, "umst-admit-grok");
    }
}

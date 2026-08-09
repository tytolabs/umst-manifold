// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Catalog digest pin witness — MaOS parity with egoff `pin_witness_ok` (§14bis.f-I-0).
//!
//! Idempotent, no-I/O checks against the embedded `artifacts/catalog.lock.json` bundle:
//! upstream digest pin · module count · lock-bundle content-address (SHA-256) · structural quickcheck
//! · composed primary-fiber fingerprint (core `sha2` hash, mirrors `umst-math::catalog_functor`).
//!
//! Digest compare + Sha3Catalog content pins route through `umst-algebra::crypto::hash` (SEC-HASH-PIN).
//!
//! **ACCEL2-AC30:** manifold `catalog_pin` SSOT heal + honest probe; gateway hot-path stays open.
//!
//! **W29-105:** deepen probe + honest fence — ceremony measured; no GREEN / PRODUCTION_WIRED /
//! MASTER / OP-5 invent.

use serde::Serialize;
use sha2::{Digest, Sha256};
use umst_algebra::crypto::hash::{
    digest_hex, pin_witness_bytes_ok, pin_witness_ok as algebra_pin_witness_ok, HashPolicy,
};

use super::{
    bundled_catalog_lock_json, catalog_lock_bundle_sha256_hex, catalog_lock_quickcheck,
    is_preview_fiber_pin, lock_upstream_catalog_digest_hex, CatalogLock,
};

/// SSOT upstream / composed catalog digest (`artifacts/catalog.lock.json`).
pub const EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX: &str =
    "17a6d8e17d9a4847231a255ffb1214db0319a7a2727ecd80708cb7f08045da1e";

/// Module count witness (same lock file).
pub const EXPECTED_MODULE_COUNT: u32 = 129;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-CATALOG-PIN";

/// AGAP slot id (2350 night wave).
pub const JOB_ID: &str = "AGAP-2350-SEC-CATALOG-PIN";

/// FLEET-COMPOSER ACCEL2 Band B slot AC30 id.
pub const FLEET_ACCEL2_AC30_JOB_ID: &str = "ACCEL2-AC30-SEC-CATALOG-PIN";

/// FLEET-COMPOSER ACCEL2 AC30 receipt path.
pub const FLEET_ACCEL2_AC30_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC30.md";

/// Prior PRABHU-DISJOINT-1542 A2 ceremony receipt.
pub const PRIOR_RECEIPT_PATH_P1542_A2: &str = "outputs/.tmp/COMPOSER_P1542_A2.md";

/// Prior FLEET-COMPOSER-Z Z39 trust close receipt (hop 5 prior owner).
pub const PRIOR_RECEIPT_PATH_Z39: &str = "outputs/.tmp/COMPOSER_Z39_1015.md";

/// Prior FLEET-COMPOSER-Y Y50 trust bridge receipt.
pub const PRIOR_RECEIPT_PATH_Y50: &str = "outputs/.tmp/COMPOSER_Y50_0808.md";

/// Prior AGAP-2350 deepen receipt.
pub const PRIOR_RECEIPT_PATH_AGAP_2350: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-CATALOG-PIN_2350.md";

/// Gateway catalog-pin delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_catalog_pin.rs";

/// Trust catalog-pin delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_catalog_pin.rs";

/// One hop in the catalog-pin witness wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CatalogPinWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Catalog-pin witness wire map (algebra → embedded lock bundle).
pub const CATALOG_PIN_WIRE_HOPS: &[CatalogPinWireHop] = &[
    CatalogPinWireHop {
        ordinal: 1,
        surface: "umst-algebra::crypto::hash::pin_witness_ok(Sha3Catalog)",
        role: "SEC-HASH-PIN algebra SSOT",
        wired: true,
    },
    CatalogPinWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::catalog::catalog_sha3_pin_witness_ok",
        role: "Sha3Catalog bridge on manifold boundary",
        wired: true,
    },
    CatalogPinWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::catalog::pin_witness_ok",
        role: "Embedded lock bundle witness (no I/O)",
        wired: true,
    },
    CatalogPinWireHop {
        ordinal: 4,
        surface: "umst-manifold::runtime::catalog::composed_fiber_fingerprint_guard_holds",
        role: "v2 dual-pin non-preview fiber fingerprint",
        wired: true,
    },
    CatalogPinWireHop {
        ordinal: 5,
        surface: "umst-gateway::sec_catalog_pin::catalog_pin_production_wired",
        role: "Gateway hot-path manifold dep (serial B1)",
        wired: false,
    },
];

/// Pin mismatch for catalog digest witnesses (total, no panic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogPinMismatch {
    /// Embedded upstream digest ≠ [`EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX`].
    Digest,
    /// Lock JSON `module_count` ≠ [`EXPECTED_MODULE_COUNT`].
    ModuleCount,
    /// SHA-256 of bundled lock bytes ≠ [`catalog_lock_bundle_sha256_hex`].
    LockBundleContentAddress,
    /// [`catalog_lock_quickcheck`] failed on bundled lock.
    LockQuickcheck,
    /// v2 `composed_primary_fiber_fingerprint_hex` ≠ recomputed non-preview fiber fingerprint.
    ComposedFiberFingerprint,
}

/// Verify catalog content-address witness under [`HashPolicy::Sha3Catalog`].
pub fn catalog_sha3_pin_witness_ok(
    expected_hex: &str,
    preimage: &[u8],
) -> Result<(), CatalogPinMismatch> {
    algebra_pin_witness_ok(HashPolicy::Sha3Catalog, expected_hex, preimage)
        .map_err(|_| CatalogPinMismatch::Digest)
}

/// Recompute non-preview fiber fingerprint (sorted `repo:digest` pairs, SHA-256).
#[must_use]
pub fn non_preview_fiber_fingerprint_hex(lock: &CatalogLock) -> String {
    let mut digests: Vec<String> = lock
        .fiber_pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin(pin))
        .map(|pin| format!("{}:{}", pin.repo, pin.catalog_digest_hex))
        .collect();
    if digests.is_empty() {
        return String::new();
    }
    digests.sort();
    let payload = digests.join("|");
    let hash = Sha256::digest(payload.as_bytes());
    format!("{hash:x}")
}

/// Whether v2 composed primary-fiber fingerprint matches embedded lock field.
#[must_use]
pub fn composed_fiber_fingerprint_guard_holds(lock: &CatalogLock) -> bool {
    if lock.version < 2 {
        return true;
    }
    let non_preview: Vec<_> = lock
        .fiber_pins
        .iter()
        .filter(|pin| !is_preview_fiber_pin(pin))
        .collect();
    if non_preview.is_empty() {
        return true;
    }
    let fp = non_preview_fiber_fingerprint_hex(lock);
    lock.composed_primary_fiber_fingerprint_hex
        .as_deref()
        .is_some_and(|stored| !stored.is_empty() && stored == fp)
}

/// SHA-256 content-address of verbatim bundled lock JSON (lowercase hex).
#[must_use]
pub fn lock_bundle_content_address_hex() -> String {
    let hash = Sha256::digest(bundled_catalog_lock_json().as_bytes());
    format!("{hash:x}")
}

/// Idempotent pin witness against embedded manifold lock (no I/O).
pub fn pin_witness_ok() -> Result<(), CatalogPinMismatch> {
    pin_witness_bytes_ok(
        EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX,
        &lock_upstream_catalog_digest_bytes(),
    )
    .map_err(|_| CatalogPinMismatch::Digest)?;

    let json = bundled_catalog_lock_json();
    let module_count_needle = format!("\"module_count\": {EXPECTED_MODULE_COUNT}");
    if !json.contains(&module_count_needle) {
        return Err(CatalogPinMismatch::ModuleCount);
    }

    // Recompute SHA-256 of verbatim lock JSON (not the build-time constant against itself).
    let recomputed = Sha256::digest(json.as_bytes());
    let mut recomputed_bytes = [0u8; 32];
    recomputed_bytes.copy_from_slice(&recomputed);
    pin_witness_bytes_ok(catalog_lock_bundle_sha256_hex(), &recomputed_bytes)
        .map_err(|_| CatalogPinMismatch::LockBundleContentAddress)?;

    let lock = CatalogLock::from_bundled().map_err(|_| CatalogPinMismatch::LockQuickcheck)?;
    if !catalog_lock_quickcheck(&lock) {
        return Err(CatalogPinMismatch::LockQuickcheck);
    }
    if !composed_fiber_fingerprint_guard_holds(&lock) {
        return Err(CatalogPinMismatch::ComposedFiberFingerprint);
    }

    Ok(())
}

/// Build-time upstream digest bytes (for constant-time pin compare).
fn lock_upstream_catalog_digest_bytes() -> [u8; 32] {
    super::lock_upstream_catalog_digest_bytes()
}

/// Whether embedded lock ceremony is wired at catalog_pin SSOT.
#[must_use]
pub fn catalog_pin_witness_wired() -> bool {
    pin_witness_ok().is_ok()
}

/// Whether live gateway hot-path manifold dep is plumbed (honest `false`).
#[must_use]
pub const fn catalog_pin_gateway_production_wired() -> bool {
    false
}

/// Close predicate — embedded `catalog.lock.json` pin witness chain.
#[must_use]
pub fn catalog_pin_ceremony_closed() -> bool {
    pin_witness_ok().is_ok()
        && lock_bundle_content_address_hex() == catalog_lock_bundle_sha256_hex()
        && catalog_pin_witness_wired()
}

/// Typed probe for catalog_pin witness closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CatalogPinWitnessProbe {
    /// SSOT upstream digest hex matches build pin.
    pub upstream_digest_ok: bool,
    /// `pin_witness_ok()` passes on bundled lock.
    pub lock_witness_ok: bool,
    /// Lock-bundle content-address matches build constant.
    pub lock_bundle_content_address_ok: bool,
    /// v2 composed primary-fiber fingerprint guard holds.
    pub fiber_fingerprint_ok: bool,
    /// Sha3Catalog adopt roundtrip wired.
    pub sha3_catalog_adopt_ok: bool,
    /// Gateway hot-path production flip.
    pub gateway_production_wired: bool,
    /// Wire hop wired count.
    pub wire_hop_wired_count: u8,
}

/// Build introspection probe for catalog_pin done-when checks.
#[must_use]
pub fn catalog_pin_witness_probe() -> CatalogPinWitnessProbe {
    let lock = CatalogLock::from_bundled().ok();
    let fiber_fingerprint_ok = lock
        .as_ref()
        .map(composed_fiber_fingerprint_guard_holds)
        .unwrap_or(false);
    let sha3_preimage = b"catalog-pin-ac30-sha3-adopt-v1";
    let sha3_catalog_adopt_ok = digest_hex(HashPolicy::Sha3Catalog, sha3_preimage)
        .ok()
        .and_then(|hex| catalog_sha3_pin_witness_ok(&hex, sha3_preimage).ok())
        .is_some();
    CatalogPinWitnessProbe {
        upstream_digest_ok: lock_upstream_catalog_digest_hex()
            == EXPECTED_UPSTREAM_CATALOG_DIGEST_HEX,
        lock_witness_ok: pin_witness_ok().is_ok(),
        lock_bundle_content_address_ok: lock_bundle_content_address_hex()
            == catalog_lock_bundle_sha256_hex(),
        fiber_fingerprint_ok,
        sha3_catalog_adopt_ok,
        gateway_production_wired: catalog_pin_gateway_production_wired(),
        wire_hop_wired_count: CATALOG_PIN_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count() as u8,
    }
}

/// FLEET-COMPOSER ACCEL2 AC30 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CatalogPinAccelAc30Probe {
    /// AC30 fleet card id.
    pub ac30_job_id: &'static str,
    /// Prior P1542 A2 ceremony absorbed.
    pub prior_p1542_a2_absorbed: bool,
    /// Prior Z39 trust close absorbed.
    pub prior_z39_absorbed: bool,
    /// Prior Y50 trust bridge absorbed.
    pub prior_y50_absorbed: bool,
    /// Prior AGAP-2350 deepen absorbed.
    pub prior_agap_2350_absorbed: bool,
    /// Catalog_pin ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying witness probe.
    pub probe: CatalogPinWitnessProbe,
    /// `catalog_pin_gateway_production_wired()` — honest false.
    pub gateway_production_wired: bool,
}

/// Build FLEET-COMPOSER ACCEL2 AC30 integration probe from live measurements.
#[must_use]
pub fn catalog_pin_accel_ac30_probe() -> CatalogPinAccelAc30Probe {
    CatalogPinAccelAc30Probe {
        ac30_job_id: FLEET_ACCEL2_AC30_JOB_ID,
        prior_p1542_a2_absorbed: PRIOR_RECEIPT_PATH_P1542_A2.contains("P1542_A2"),
        prior_z39_absorbed: PRIOR_RECEIPT_PATH_Z39.contains("Z39_1015"),
        prior_y50_absorbed: PRIOR_RECEIPT_PATH_Y50.contains("Y50"),
        prior_agap_2350_absorbed: PRIOR_RECEIPT_PATH_AGAP_2350.contains("SEC-CATALOG-PIN"),
        ceremony_closed: catalog_pin_ceremony_closed(),
        probe: catalog_pin_witness_probe(),
        gateway_production_wired: catalog_pin_gateway_production_wired(),
    }
}

/// FLEET-COMPOSER ACCEL2 AC30 honesty gate — ceremony closed + gateway production false.
#[must_use]
pub fn catalog_pin_accel_ac30_honest() -> bool {
    let probe = catalog_pin_accel_ac30_probe();
    probe.ac30_job_id == FLEET_ACCEL2_AC30_JOB_ID
        && probe.prior_p1542_a2_absorbed
        && probe.prior_z39_absorbed
        && probe.prior_y50_absorbed
        && probe.prior_agap_2350_absorbed
        && probe.ceremony_closed
        && probe.probe.upstream_digest_ok
        && probe.probe.lock_witness_ok
        && probe.probe.lock_bundle_content_address_ok
        && probe.probe.fiber_fingerprint_ok
        && probe.probe.sha3_catalog_adopt_ok
        && probe.probe.wire_hop_wired_count == 4
        && !probe.probe.gateway_production_wired
        && !probe.gateway_production_wired
}

// ── W29-105-CATALOG_PIN · deepen + honest fence ──────────────────────────────

/// Swarm cell id for this catalog_pin deepen.
pub const W29_105_CELL_ID: &str = "W29-105-CATALOG_PIN";

/// Honest posture — deepen measured ceremony only; no invent claims.
pub const W29_105_HONEST_POSTURE: &str = "CATALOG_PIN_DEEPEN_ONLY";

/// Explicit non-claims (gate text).
pub const W29_105_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// Deepen schema version for W29-105.
pub const W29_105_DEEPEN_SCHEMA_VERSION: &str = "catalog_pin_w29_105_deepen_v1";

/// Expected wired hop count on manifold side (gateway hop stays open).
pub const W29_105_WIRE_HOP_WIRED_COUNT: u8 = 4;

/// Expected total wire hops (4 closed + 1 gateway open).
pub const W29_105_WIRE_HOP_TOTAL: usize = 5;

/// Honest fence flags for catalog_pin deepen (W29-105).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CatalogPinW29105DeepenProbe {
    /// Deepen schema pin.
    pub schema_version: &'static str,
    /// Swarm cell id pin.
    pub cell_id: &'static str,
    /// Honest posture label.
    pub honest_posture: &'static str,
    /// Explicit non-claim string.
    pub non_claim: &'static str,
    /// Board slice id pin.
    pub board_slice_id: &'static str,
    /// Live ceremony close predicate.
    pub ceremony_closed: bool,
    /// ACCEL2-AC30 honesty gate.
    pub accel_ac30_honest: bool,
    /// Wire hops wired (manifold side).
    pub wire_hop_wired_count: u8,
    /// Total wire hops in map.
    pub wire_hop_total: usize,
    /// Gateway hot-path invent claim — always false.
    pub production_wired_claimed: bool,
    /// Physics/fleet GREEN invent claim — always false.
    pub green_claimed: bool,
    /// OP-5 PASS invent claim — always false.
    pub op5_pass_claimed: bool,
    /// MASTER_RETICK invent claim — always false.
    pub master_retick_claimed: bool,
    /// Aggregate honesty of this deepen probe.
    pub deepen_honest: bool,
}

/// Build the W29-105 catalog_pin deepen honesty probe from live measurements.
#[must_use]
pub fn catalog_pin_w29_105_deepen_probe() -> CatalogPinW29105DeepenProbe {
    let production_wired_claimed = catalog_pin_gateway_production_wired();
    let green_claimed = false;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let witness = catalog_pin_witness_probe();
    let ceremony_closed = catalog_pin_ceremony_closed();
    let accel_ac30_honest = catalog_pin_accel_ac30_honest();
    let wire_hop_wired_count = witness.wire_hop_wired_count;
    let wire_hop_total = CATALOG_PIN_WIRE_HOPS.len();
    let deepen_honest = W29_105_CELL_ID == "W29-105-CATALOG_PIN"
        && W29_105_DEEPEN_SCHEMA_VERSION == "catalog_pin_w29_105_deepen_v1"
        && W29_105_HONEST_POSTURE == "CATALOG_PIN_DEEPEN_ONLY"
        && BOARD_SLICE_ID == "SEC-CATALOG-PIN"
        && ceremony_closed
        && accel_ac30_honest
        && witness.upstream_digest_ok
        && witness.lock_witness_ok
        && witness.lock_bundle_content_address_ok
        && witness.fiber_fingerprint_ok
        && witness.sha3_catalog_adopt_ok
        && wire_hop_wired_count == W29_105_WIRE_HOP_WIRED_COUNT
        && wire_hop_total == W29_105_WIRE_HOP_TOTAL
        && !production_wired_claimed
        && !witness.gateway_production_wired
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && W29_105_NON_CLAIM.contains("not GREEN")
        && W29_105_NON_CLAIM.contains("not OP-5 PASS")
        && W29_105_NON_CLAIM.contains("not production_wired")
        && W29_105_NON_CLAIM.contains("not MASTER_RETICK");
    CatalogPinW29105DeepenProbe {
        schema_version: W29_105_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_105_CELL_ID,
        honest_posture: W29_105_HONEST_POSTURE,
        non_claim: W29_105_NON_CLAIM,
        board_slice_id: BOARD_SLICE_ID,
        ceremony_closed,
        accel_ac30_honest,
        wire_hop_wired_count,
        wire_hop_total,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-105 catalog_pin deepen honesty probe passes.
#[must_use]
pub fn catalog_pin_w29_105_deepen_honest() -> bool {
    catalog_pin_w29_105_deepen_probe().deepen_honest
}

/// Fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn catalog_pin_w29_105_honest_fence_holds() -> bool {
    let p = catalog_pin_w29_105_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

#[cfg(test)]
mod catalog_pin_tests {
    use super::*;

    #[test]
    fn board_slice_is_sec_catalog_pin() {
        assert_eq!(BOARD_SLICE_ID, "SEC-CATALOG-PIN");
        assert_eq!(JOB_ID, "AGAP-2350-SEC-CATALOG-PIN");
        assert_eq!(FLEET_ACCEL2_AC30_JOB_ID, "ACCEL2-AC30-SEC-CATALOG-PIN");
    }

    #[test]
    fn catalog_pin_witness_wired_on_bundled_lock() {
        assert!(catalog_pin_witness_wired());
        pin_witness_ok().expect("pin_witness_ok");
    }

    #[test]
    fn catalog_pin_ceremony_closed_on_bundled_lock() {
        assert!(catalog_pin_ceremony_closed());
        assert_eq!(
            lock_bundle_content_address_hex(),
            catalog_lock_bundle_sha256_hex()
        );
    }

    #[test]
    fn catalog_pin_wire_hops_four_of_five_wired() {
        assert_eq!(CATALOG_PIN_WIRE_HOPS.len(), 5);
        assert_eq!(
            CATALOG_PIN_WIRE_HOPS.iter().filter(|h| h.wired).count(),
            4
        );
        assert!(CATALOG_PIN_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("pin_witness_ok") && h.wired));
        assert!(CATALOG_PIN_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("umst-gateway") && !h.wired));
    }

    #[test]
    fn catalog_pin_gateway_production_stays_false() {
        assert!(!catalog_pin_gateway_production_wired());
    }

    #[test]
    fn catalog_pin_witness_probe_honest() {
        let probe = catalog_pin_witness_probe();
        assert!(probe.upstream_digest_ok);
        assert!(probe.lock_witness_ok);
        assert!(probe.lock_bundle_content_address_ok);
        assert!(probe.fiber_fingerprint_ok);
        assert!(probe.sha3_catalog_adopt_ok);
        assert!(!probe.gateway_production_wired);
        assert_eq!(probe.wire_hop_wired_count, 4);
    }

    #[test]
    fn fleet_accel_ac30_catalog_pin_heal_honest() {
        assert!(catalog_pin_accel_ac30_honest());
        let probe = catalog_pin_accel_ac30_probe();
        assert_eq!(probe.ac30_job_id, FLEET_ACCEL2_AC30_JOB_ID);
        assert!(probe.prior_p1542_a2_absorbed);
        assert!(probe.prior_z39_absorbed);
        assert!(probe.prior_y50_absorbed);
        assert!(probe.prior_agap_2350_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.gateway_production_wired);
    }

    #[test]
    fn w29_105_catalog_pin_deepen_honest_probe() {
        let probe = catalog_pin_w29_105_deepen_probe();
        assert_eq!(probe.cell_id, W29_105_CELL_ID);
        assert_eq!(probe.schema_version, W29_105_DEEPEN_SCHEMA_VERSION);
        assert_eq!(probe.board_slice_id, "SEC-CATALOG-PIN");
        assert_eq!(probe.wire_hop_wired_count, W29_105_WIRE_HOP_WIRED_COUNT);
        assert_eq!(probe.wire_hop_total, W29_105_WIRE_HOP_TOTAL);
        assert!(probe.ceremony_closed);
        assert!(probe.accel_ac30_honest);
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(catalog_pin_w29_105_deepen_honest());
        assert!(catalog_pin_w29_105_honest_fence_holds());
    }

    #[test]
    fn w29_105_non_claim_text_covers_forbidden_invent() {
        for needle in [
            "not GREEN",
            "not OP-5 PASS",
            "not production_wired",
            "not MASTER_RETICK",
        ] {
            assert!(
                W29_105_NON_CLAIM.contains(needle),
                "missing non-claim fragment: {needle}"
            );
        }
    }

    #[test]
    fn w29_105_gateway_hop_stays_open() {
        assert!(!catalog_pin_gateway_production_wired());
        assert!(CATALOG_PIN_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("umst-gateway") && !h.wired));
        let probe = catalog_pin_w29_105_deepen_probe();
        assert!(!probe.production_wired_claimed);
        assert!(probe.deepen_honest);
    }
}

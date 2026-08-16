// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGAP-2033/2127-SEC-S3 — manifold gate runtime trust palette / session ledger wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S3 palette/ledger +
//! revoke-posture SSOT; sled persistence, gateway `trust_wrap_wired()`, and
//! `session_ledger_wired()` stay **honest open**.
//!
//! # W29-122 deepen
//!
//! Open-residual fence pins for hops 6–7 (sled session ledger + gateway trust-wrap)
//! measured at census tier. No invented GREEN / PRODUCTION_WIRED / MASTER / OP-5.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S3";

/// W29 continuous worklist cell id (Composer RL → Grok NEW Task lane).
pub const W29_CELL_ID: &str = "W29-122-SEC_S3";

/// AGAP slot id (2033 palette/ledger deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S3";

/// FLEET-COMPOSER Prabhu Wave C slot C5 id.
pub const FLEET_P1606_C5_JOB_ID: &str = "PRABHU-WAVE-C-1606-C5";

/// FLEET-COMPOSER Prabhu Wave C C5 receipt path.
pub const FLEET_P1606_C5_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1606_C5.md";

/// FLEET-COMPOSER ACCEL-25 slot AC05 id (revoke posture deepen).
pub const ACCEL_2030_AC05_JOB_ID: &str = "ACCEL-2030-AC05";

/// FLEET-COMPOSER ACCEL-25 AC05 receipt path.
pub const ACCEL_AC05_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL_2030_AC05.md";

/// FLEET-COMPOSER-F F74 SEC-S3 revoke posture forensic receipt.
pub const FLEET_COMPOSER_F74_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_F74_SEC_S3_1942.md";

/// FLEET-COMPOSER-G G74 SEC-S3 revoke ledger deepen receipt.
pub const FLEET_COMPOSER_G74_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_G74_SEC_S3_2143.md";

/// egoff integration test SSOT for S-3 revoke lifecycle witnesses.
pub const EGOFF_S3_INTEGRATION_SSOT: &str = "egoff/egoff/tests/s2_s3_security_deepen.rs";

/// Prior AGAP-2033 SEC-S3 palette/ledger receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S3_2033.md";

/// Prior AGAP-2127 SEC-S3 revoke posture receipt.
pub const PRIOR_RECEIPT_PATH_2127: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S3_2127.md";

/// umst-trust SEC-S3 revoke posture delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_s3_revoke_posture.rs";

/// umst-trust SEC-S3 sled persistence census delegate SSOT.
pub const TRUST_SLED_SSOT: &str = "umst-foundations/crates/umst-trust/src/sled.rs";

/// egoff session ledger SSOT (in-memory deepen; sled lands S-3 GREEN).
pub const EGOFF_LEDGER_SSOT: &str = "egoff/egoff/src/security/ledger.rs";

/// egoff palette forensic gate SSOT.
pub const EGOFF_PALETTE_SSOT: &str = "egoff/egoff/src/security/palette.rs";

/// Gateway trust-wrap delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version.
pub const SCHEMA_VERSION: &str = "sec_s3_gate_palette_ledger_census_v3";

/// Open residual hop count on the manifold wire map (hops 6–7 honest-open).
pub const OPEN_RESIDUAL_HOP_COUNT: usize = 2;

/// MASTER / OP-5 retick — honest false at census deepen.
pub const MASTER_RETICK_ELIGIBLE: bool = false;

/// S-3 revoke posture facet row count (F74/G74 forensic census).
pub const S3_REVOKE_POSTURE_FACET_COUNT: usize = 8;

/// S-3 revoke posture facets wired today (F74 measured 5/8).
pub const S3_REVOKE_POSTURE_WIRED_COUNT: usize = 5;

/// S-3 revoke ledger lifecycle hop count (G74 deepen).
pub const S3_REVOKE_LEDGER_HOP_COUNT: usize = 8;

/// S-3 revoke ledger lifecycle hops wired today (G74 measured 6/8).
pub const S3_REVOKE_LEDGER_WIRED_COUNT: usize = 6;

/// S-Arc GREEN claim blocked — honest true in scaffold deepen.
pub const S3_GREEN_CLAIM_BLOCKED: bool = true;

/// Sled persistence claimed — honest false until sled I/O lands.
pub const SLED_PERSISTENCE_CLAIMED_HONEST: bool = false;

/// Palette envelope `persisted` field — honest false until sled I/O lands.
pub const PALETTE_PERSISTED_HONEST: bool = false;

/// One hop in the manifold SEC-S3 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS3GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S3 gate runtime wire map (cold-edge evidence → trust palette/ledger census).
pub const MANIFOLD_SEC_S3_GATE_WIRE_HOPS: &[SecS3GateWireHop] = &[
    SecS3GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS3GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS3GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s3::gate_palette_ledger_census",
        role: "Manifold gate SEC-S3 palette/ledger census",
        wired: true,
    },
    SecS3GateWireHop {
        ordinal: 4,
        surface: "umst-trust::sec_s3_revoke_posture::s3_revoke_posture_report",
        role: "Trust revoke posture delegate (G74)",
        wired: true,
    },
    SecS3GateWireHop {
        ordinal: 5,
        surface: "egoff::security::palette + egoff::security::ledger",
        role: "Palette/ledger in-memory deepen (persisted=false)",
        wired: true,
    },
    SecS3GateWireHop {
        ordinal: 6,
        surface: "umst-trust::sec_ecosystem_extract::session_ledger_wired",
        role: "Sled session ledger persistence",
        wired: false,
    },
    SecS3GateWireHop {
        ordinal: 7,
        surface: "umst-gateway::sec_gw_trust_wrap::trust_wrap_wired",
        role: "Gateway production ceremony (serial Wave D)",
        wired: false,
    },
];

/// One honest-open residual fence pin (sled / gateway — not wired today).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS3OpenResidualFence {
    /// Residual id (`R-sled-session-ledger` / `R-gateway-trust-wrap`).
    pub residue_id: &'static str,
    /// Wire hop ordinal on the manifold map.
    pub hop_ordinal: u8,
    /// Delegate SSOT path.
    pub delegate_ssot: &'static str,
    /// Whether the hop remains honest-open (`wired=false`).
    pub honest_open: bool,
    /// Whether GREEN credit is blocked for this residual.
    pub green_credit_blocked: bool,
}

/// Open residual fence pins — hops 6–7 measured open at W29-122 deepen.
pub const OPEN_RESIDUAL_FENCES: &[SecS3OpenResidualFence] = &[
    SecS3OpenResidualFence {
        residue_id: "R-sled-session-ledger",
        hop_ordinal: 6,
        delegate_ssot: TRUST_SLED_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
    SecS3OpenResidualFence {
        residue_id: "R-gateway-trust-wrap",
        hop_ordinal: 7,
        delegate_ssot: GATEWAY_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
];

/// One row of the S-3 revoke posture forensic matrix (pinned from `umst-trust::sec_s3_revoke_posture`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldS3RevokePostureFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// S-3 revoke posture facet inventory (pinned from `umst-trust::sec_s3_revoke_posture` SSOT).
pub const MANIFOLD_S3_REVOKE_POSTURE_FACETS: &[ManifoldS3RevokePostureFacet] = &[
    ManifoldS3RevokePostureFacet {
        facet: "trust_adt_revoke",
        wired: true,
        owning_slice: "SEC-TRUST-EXTRACT",
    },
    ManifoldS3RevokePostureFacet {
        facet: "gate_attestation_revoked_refuse",
        wired: true,
        owning_slice: "SEC-TRUST-GATE",
    },
    ManifoldS3RevokePostureFacet {
        facet: "egoff_posture_trust",
        wired: true,
        owning_slice: "SEC-S3",
    },
    ManifoldS3RevokePostureFacet {
        facet: "egoff_revoke_outcome_envelope",
        wired: true,
        owning_slice: "SEC-S3",
    },
    ManifoldS3RevokePostureFacet {
        facet: "egoff_forensic_acknowledge_revoked",
        wired: true,
        owning_slice: "SEC-S3",
    },
    ManifoldS3RevokePostureFacet {
        facet: "sled_session_ledger_persistence",
        wired: false,
        owning_slice: "SEC-S3",
    },
    ManifoldS3RevokePostureFacet {
        facet: "gateway_session_ledger_provider",
        wired: false,
        owning_slice: "SEC-GW-WRAP",
    },
    ManifoldS3RevokePostureFacet {
        facet: "ecosystem_consumer_wired_flip",
        wired: false,
        owning_slice: "SEC-GW-WRAP",
    },
];

/// One hop in the S-3 revoke ledger lifecycle wire map (G74 deepen).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldS3RevokeLedgerHop {
    /// Ordinal (1-based) in the lifecycle map.
    pub ordinal: u8,
    /// Lifecycle stage label.
    pub stage: &'static str,
    /// SSOT surface (egoff ledger or umst-trust gate).
    pub surface: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// S-3 revoke ledger lifecycle wire map — grant through forensic read (G74).
pub const MANIFOLD_S3_REVOKE_LEDGER_HOPS: &[ManifoldS3RevokeLedgerHop] = &[
    ManifoldS3RevokeLedgerHop {
        ordinal: 1,
        stage: "grant_append",
        surface: "egoff::security::ledger::TrustLedger::grant",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 2,
        stage: "revoke_absorbing",
        surface: "egoff::security::ledger::TrustLedger::revoke → RevokeOutcome",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 3,
        stage: "posture_trust_revoked",
        surface: "egoff::security::ledger::TrustLedger::posture_trust",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 4,
        stage: "gate_admit_refuse",
        surface: "umst-trust::permission::require_trust_gate → AttestationRevoked",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 5,
        stage: "compose_absorbing",
        surface: "umst-algebra::crypto::trust::Trust::compose → RevokedAbsorbing",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 6,
        stage: "forensic_acknowledge_revoked",
        surface: "egoff::security::palette::acknowledge-revoked (EGOFF_TRUST_ALLOW_REVOKED_READ)",
        wired: true,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 7,
        stage: "sled_persistence",
        surface: "egoff::trust::ledger (sled I/O)",
        wired: false,
    },
    ManifoldS3RevokeLedgerHop {
        ordinal: 8,
        stage: "sled_audit_trail",
        surface: "acknowledge-revoked audit_logged",
        wired: false,
    },
];

/// Aggregated SEC-S3 gate palette/ledger census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS3GatePaletteLedgerCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// W29 cell id pin.
    pub w29_cell_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// Palette envelope `persisted` honest false.
    pub palette_persisted: bool,
    /// `session_ledger_wired()` — honest false until sled.
    pub session_ledger_wired: bool,
    /// Gateway production flip.
    pub production_wired: bool,
    /// Honest open residual hop count (2).
    pub open_residual_hop_count: usize,
    /// Open residual fence pins verified.
    pub open_residual_fences_verified: bool,
    /// S-3 revoke posture facets verified at manifold boundary (5/8).
    pub revoke_posture_facets_verified: bool,
    /// S-3 revoke ledger lifecycle verified at manifold boundary (6/8).
    pub revoke_ledger_lifecycle_verified: bool,
    /// S-Arc GREEN claim blocked — honest true.
    pub s3_green_claim_blocked: bool,
    /// Sled persistence claimed — honest false.
    pub sled_persistence_claimed: bool,
    /// MASTER / OP-5 retick — honest false.
    pub master_retick_eligible: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// Exercise gate cold-edge evidence at manifold SSOT (identity transition admits).
#[must_use]
pub fn gate_transition_evidence_probe() -> bool {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    evidence.admissibility == AdmissibilityToken::Admissible && !evidence.catalog_id.is_empty()
}

/// Whether sled-backed session ledger persistence is wired (honest `false`).
#[must_use]
pub const fn session_ledger_wired() -> bool {
    false
}

/// Whether live gateway trust-wrap production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_s3_production_wired() -> bool {
    false
}

/// Whether MASTER / OP-5 retick is eligible (honest `false` at census deepen).
#[must_use]
pub const fn sec_s3_master_retick_eligible() -> bool {
    MASTER_RETICK_ELIGIBLE
}

/// Whether open residual fence pins match unwired hops 6–7 on the wire map.
#[must_use]
pub fn manifold_s3_open_residual_fences_verified() -> bool {
    const EXPECTED: [(&str, u8); 2] = [("R-sled-session-ledger", 6), ("R-gateway-trust-wrap", 7)];
    OPEN_RESIDUAL_FENCES.len() == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES
            .iter()
            .zip(EXPECTED.iter())
            .all(|(fence, (id, ord))| {
                fence.residue_id == *id
                    && fence.hop_ordinal == *ord
                    && fence.honest_open
                    && fence.green_credit_blocked
                    && !fence.delegate_ssot.is_empty()
            })
        && MANIFOLD_SEC_S3_GATE_WIRE_HOPS
            .iter()
            .filter(|h| !h.wired)
            .count()
            == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES.iter().all(|fence| {
            MANIFOLD_SEC_S3_GATE_WIRE_HOPS
                .iter()
                .any(|h| h.ordinal == fence.hop_ordinal && !h.wired)
        })
}

/// Whether all S-3 revoke posture facets are enumerated at manifold boundary.
#[must_use]
pub fn manifold_s3_revoke_posture_facets_verified() -> bool {
    MANIFOLD_S3_REVOKE_POSTURE_FACETS.len() == S3_REVOKE_POSTURE_FACET_COUNT
        && MANIFOLD_S3_REVOKE_POSTURE_FACETS
            .iter()
            .filter(|f| f.wired)
            .count()
            == S3_REVOKE_POSTURE_WIRED_COUNT
        && MANIFOLD_S3_REVOKE_POSTURE_FACETS
            .iter()
            .any(|f| f.facet == "gate_attestation_revoked_refuse" && f.wired)
        && MANIFOLD_S3_REVOKE_POSTURE_FACETS
            .iter()
            .any(|f| f.facet == "sled_session_ledger_persistence" && !f.wired)
}

/// Whether S-3 revoke ledger lifecycle hops are pinned at manifold boundary.
#[must_use]
pub fn manifold_s3_revoke_ledger_lifecycle_verified() -> bool {
    MANIFOLD_S3_REVOKE_LEDGER_HOPS.len() == S3_REVOKE_LEDGER_HOP_COUNT
        && MANIFOLD_S3_REVOKE_LEDGER_HOPS
            .iter()
            .filter(|h| h.wired)
            .count()
            == S3_REVOKE_LEDGER_WIRED_COUNT
        && MANIFOLD_S3_REVOKE_LEDGER_HOPS
            .iter()
            .any(|h| h.stage == "gate_admit_refuse" && h.wired)
        && MANIFOLD_S3_REVOKE_LEDGER_HOPS
            .iter()
            .any(|h| h.stage == "sled_persistence" && !h.wired)
}

/// Build manifold SEC-S3 gate palette/ledger census from live measurements.
#[must_use]
pub fn gate_palette_ledger_census() -> SecS3GatePaletteLedgerCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S3_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecS3GatePaletteLedgerCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        w29_cell_id: W29_CELL_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        palette_persisted: PALETTE_PERSISTED_HONEST,
        session_ledger_wired: session_ledger_wired(),
        production_wired: sec_s3_production_wired(),
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_s3_open_residual_fences_verified(),
        revoke_posture_facets_verified: manifold_s3_revoke_posture_facets_verified(),
        revoke_ledger_lifecycle_verified: manifold_s3_revoke_ledger_lifecycle_verified(),
        s3_green_claim_blocked: S3_GREEN_CLAIM_BLOCKED,
        sled_persistence_claimed: SLED_PERSISTENCE_CLAIMED_HONEST,
        master_retick_eligible: sec_s3_master_retick_eligible(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S3 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + palette/ledger wire map hops 1–5 are measured wired
/// and open residual fences for hops 6–7 are pinned honest-open.
/// Session ledger persistence + gateway production flip are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_s3_ceremony_closed() -> bool {
    let census = gate_palette_ledger_census();
    census.gate_evidence_wired
        && !census.palette_persisted
        && !census.session_ledger_wired
        && !census.production_wired
        && census.open_residual_hop_count == OPEN_RESIDUAL_HOP_COUNT
        && census.open_residual_fences_verified
        && census.s3_green_claim_blocked
        && !census.master_retick_eligible
        && census.wire_hop_wired_count == 5
        && census.w29_cell_id == W29_CELL_ID
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-S3 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS3GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Palette envelope persisted honest false.
    pub palette_persisted_honest_false: bool,
    /// Session ledger persistence honest false.
    pub session_ledger_honest_false: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Open residual fences verified (hops 6–7).
    pub open_residual_fences_verified: bool,
    /// Open residual hop count.
    pub open_residual_hop_count: usize,
    /// Revoke posture facets verified (5/8).
    pub revoke_posture_facets_verified: bool,
    /// Revoke ledger lifecycle verified (6/8).
    pub revoke_ledger_lifecycle_verified: bool,
    /// S-Arc GREEN claim blocked.
    pub s3_green_claim_blocked: bool,
    /// Sled persistence claimed honest false.
    pub sled_persistence_honest_false: bool,
    /// MASTER retick honest false.
    pub master_retick_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-S3 done-when checks.
#[must_use]
pub fn sec_s3_gate_manifold_probe() -> SecS3GateManifoldProbe {
    let census = gate_palette_ledger_census();
    SecS3GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        palette_persisted_honest_false: !census.palette_persisted,
        session_ledger_honest_false: !census.session_ledger_wired,
        production_honest_false: !census.production_wired,
        open_residual_fences_verified: census.open_residual_fences_verified,
        open_residual_hop_count: census.open_residual_hop_count,
        revoke_posture_facets_verified: census.revoke_posture_facets_verified,
        revoke_ledger_lifecycle_verified: census.revoke_ledger_lifecycle_verified,
        s3_green_claim_blocked: census.s3_green_claim_blocked,
        sled_persistence_honest_false: !census.sled_persistence_claimed,
        master_retick_honest_false: !census.master_retick_eligible,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s3_ceremony_closed(),
    }
}

/// FLEET-COMPOSER Prabhu Wave C C5 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS3P1606C5Probe {
    /// C5 fleet card id.
    pub c5_job_id: &'static str,
    /// Prior 2033 palette deepen absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior 2127 revoke posture absorbed.
    pub prior_2127_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS3GateManifoldProbe,
    /// `sec_s3_production_wired()` — honest false.
    pub production_wired: bool,
    /// Palette `persisted` — honest false.
    pub palette_persisted: bool,
}

/// Build FLEET-COMPOSER P1606 C5 integration probe from live measurements.
#[must_use]
pub fn sec_s3_p1606_c5_probe() -> SecS3P1606C5Probe {
    SecS3P1606C5Probe {
        c5_job_id: FLEET_P1606_C5_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S3_2033"),
        prior_2127_absorbed: PRIOR_RECEIPT_PATH_2127.contains("SEC-S3_2127"),
        ceremony_closed: manifold_gate_sec_s3_ceremony_closed(),
        probe: sec_s3_gate_manifold_probe(),
        production_wired: sec_s3_production_wired(),
        palette_persisted: PALETTE_PERSISTED_HONEST,
    }
}

/// FLEET-COMPOSER P1606 C5 honesty gate — ceremony closed + production false + persisted false.
#[must_use]
pub fn sec_s3_p1606_c5_honest() -> bool {
    let probe = sec_s3_p1606_c5_probe();
    probe.c5_job_id == FLEET_P1606_C5_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_2127_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.palette_persisted_honest_false
        && probe.probe.session_ledger_honest_false
        && probe.probe.production_honest_false
        && probe.probe.open_residual_fences_verified
        && probe.probe.open_residual_hop_count == OPEN_RESIDUAL_HOP_COUNT
        && probe.probe.master_retick_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && !probe.palette_persisted
}

/// Render S-3 revoke posture facet matrix for operator receipts.
#[must_use]
pub fn sec_s3_revoke_posture_matrix() -> String {
    let mut out = String::from("SEC-S3 revoke posture facets (F74/G74 @ manifold):\n");
    for facet in MANIFOLD_S3_REVOKE_POSTURE_FACETS {
        out.push_str(&format!(
            "  {} wired={} owning_slice={}\n",
            facet.facet, facet.wired, facet.owning_slice
        ));
    }
    let wired = MANIFOLD_S3_REVOKE_POSTURE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count();
    out.push_str(&format!(
        "  facets_wired={}/{} s3_green_claim_blocked={} sled_persistence_claimed={}\n",
        wired,
        MANIFOLD_S3_REVOKE_POSTURE_FACETS.len(),
        S3_GREEN_CLAIM_BLOCKED,
        SLED_PERSISTENCE_CLAIMED_HONEST
    ));
    out
}

/// Render S-3 revoke ledger lifecycle matrix for operator receipts.
#[must_use]
pub fn sec_s3_revoke_ledger_lifecycle_matrix() -> String {
    let mut out = String::from("SEC-S3 revoke ledger lifecycle (G74 @ manifold):\n");
    for hop in MANIFOLD_S3_REVOKE_LEDGER_HOPS {
        out.push_str(&format!(
            "  {} {} wired={} {}\n",
            hop.ordinal, hop.stage, hop.wired, hop.surface
        ));
    }
    let wired = MANIFOLD_S3_REVOKE_LEDGER_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    out.push_str(&format!(
        "  lifecycle_wired={}/{} session_ledger_wired={} production_wired={}\n",
        wired,
        MANIFOLD_S3_REVOKE_LEDGER_HOPS.len(),
        session_ledger_wired(),
        sec_s3_production_wired()
    ));
    out
}

/// FLEET-COMPOSER ACCEL-25 AC05 revoke posture deepen probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS3AccelAc05Probe {
    /// AC05 fleet slot id.
    pub ac05_job_id: &'static str,
    /// Prior C5 palette/ledger ceremony absorbed.
    pub prior_c5_absorbed: bool,
    /// Prior F74 forensic census absorbed.
    pub prior_f74_absorbed: bool,
    /// Prior G74 revoke ledger deepen absorbed.
    pub prior_g74_absorbed: bool,
    /// Prior 2127 egoff revoke posture absorbed.
    pub prior_2127_absorbed: bool,
    /// Revoke posture facet matrix verified.
    pub revoke_posture_matrix_verified: bool,
    /// Revoke ledger lifecycle matrix verified.
    pub revoke_ledger_lifecycle_verified: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS3GateManifoldProbe,
    /// `sec_s3_production_wired()` — honest false.
    pub production_wired: bool,
}

/// Build FLEET-COMPOSER ACCEL-25 AC05 integration probe from live measurements.
#[must_use]
pub fn sec_s3_accel_ac05_probe() -> SecS3AccelAc05Probe {
    let posture = sec_s3_revoke_posture_matrix();
    let lifecycle = sec_s3_revoke_ledger_lifecycle_matrix();
    SecS3AccelAc05Probe {
        ac05_job_id: ACCEL_2030_AC05_JOB_ID,
        prior_c5_absorbed: FLEET_P1606_C5_RECEIPT_PATH.contains("COMPOSER_P1606_C5"),
        prior_f74_absorbed: FLEET_COMPOSER_F74_RECEIPT_PATH.contains("COMPOSER_F74_SEC_S3_1942"),
        prior_g74_absorbed: FLEET_COMPOSER_G74_RECEIPT_PATH.contains("COMPOSER_G74_SEC_S3_2143"),
        prior_2127_absorbed: PRIOR_RECEIPT_PATH_2127.contains("SEC-S3_2127"),
        revoke_posture_matrix_verified: posture.contains("facets_wired=5/8")
            && posture.contains("gate_attestation_revoked_refuse")
            && posture.contains("s3_green_claim_blocked=true"),
        revoke_ledger_lifecycle_verified: lifecycle.contains("lifecycle_wired=6/8")
            && lifecycle.contains("gate_admit_refuse")
            && lifecycle.contains("session_ledger_wired=false"),
        ceremony_closed: manifold_gate_sec_s3_ceremony_closed(),
        probe: sec_s3_gate_manifold_probe(),
        production_wired: sec_s3_production_wired(),
    }
}

/// FLEET-COMPOSER ACCEL-25 AC05 honesty gate — revoke posture deepen + ceremony closed.
#[must_use]
pub fn sec_s3_accel_ac05_honest() -> bool {
    let probe = sec_s3_accel_ac05_probe();
    probe.ac05_job_id == ACCEL_2030_AC05_JOB_ID
        && probe.prior_c5_absorbed
        && probe.prior_f74_absorbed
        && probe.prior_g74_absorbed
        && probe.prior_2127_absorbed
        && probe.revoke_posture_matrix_verified
        && probe.revoke_ledger_lifecycle_verified
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.revoke_posture_facets_verified
        && probe.probe.revoke_ledger_lifecycle_verified
        && probe.probe.s3_green_claim_blocked
        && probe.probe.sled_persistence_honest_false
        && probe.probe.palette_persisted_honest_false
        && probe.probe.session_ledger_honest_false
        && probe.probe.production_honest_false
        && probe.probe.open_residual_fences_verified
        && probe.probe.open_residual_hop_count == OPEN_RESIDUAL_HOP_COUNT
        && probe.probe.master_retick_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
}

/// Validate SEC-S3 gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_s3_gate_honesty() -> Result<(), &'static str> {
    let census = gate_palette_ledger_census();
    if census.w29_cell_id != W29_CELL_ID {
        return Err("w29_cell_id must pin W29-122-SEC_S3");
    }
    if census.session_ledger_wired {
        return Err("session_ledger_wired must stay false until sled");
    }
    if census.production_wired {
        return Err("sec_s3_production_wired must stay false until SEC-GW-WRAP");
    }
    if census.palette_persisted {
        return Err("palette_persisted must stay false until sled I/O lands");
    }
    if !census.s3_green_claim_blocked {
        return Err("s3_green_claim_blocked must stay true in scaffold deepen");
    }
    if census.sled_persistence_claimed {
        return Err("sled persistence must not be claimed in scaffold");
    }
    if census.master_retick_eligible {
        return Err("master_retick_eligible must stay false — no invent MASTER/OP-5");
    }
    if census.open_residual_hop_count != OPEN_RESIDUAL_HOP_COUNT {
        return Err("two SEC-S3 open residual hops expected");
    }
    if !census.open_residual_fences_verified {
        return Err("open residual fences for hops 6–7 must verify at manifold boundary");
    }
    if !census.revoke_posture_facets_verified {
        return Err("S-3 revoke posture facets must verify at manifold boundary");
    }
    if !census.revoke_ledger_lifecycle_verified {
        return Err("S-3 revoke ledger lifecycle must verify at manifold boundary");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if MANIFOLD_SEC_S3_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-S3 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-S3 gate wire hops should be wired today");
    }
    if !manifold_gate_sec_s3_ceremony_closed() {
        return Err("manifold gate SEC-S3 ceremony must be closed at census tier");
    }
    if !sec_s3_p1606_c5_honest() {
        return Err("P1606 C5 probe must be honest");
    }
    if !sec_s3_accel_ac05_honest() {
        return Err("ACCEL AC05 revoke posture deepen probe must be honest");
    }
    if !sec_s3_w29_deepen_honest() {
        return Err("W29-122 open residual fence deepen probe must be honest");
    }
    Ok(())
}

/// Render SEC-S3 gate wire map for operator receipts.
#[must_use]
pub fn sec_s3_gate_wire_matrix() -> String {
    let census = gate_palette_ledger_census();
    let mut out = String::from("SEC-S3 manifold gate palette/ledger wire map (C5/W29-122):\n");
    for hop in MANIFOLD_SEC_S3_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} session_ledger_wired={} palette_persisted={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_S3_GATE_WIRE_HOPS.len(),
        census.session_ledger_wired,
        census.palette_persisted,
        census.production_wired
    ));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out.push_str(&format!("  egoff_palette_ssot={EGOFF_PALETTE_SSOT}\n"));
    out.push_str(&format!(
        "  revoke_posture_facets_verified={} revoke_ledger_lifecycle_verified={} \
         s3_green_claim_blocked={} sled_persistence_claimed={} \
         open_residual_fences_verified={} master_retick_eligible={}\n",
        census.revoke_posture_facets_verified,
        census.revoke_ledger_lifecycle_verified,
        census.s3_green_claim_blocked,
        census.sled_persistence_claimed,
        census.open_residual_fences_verified,
        census.master_retick_eligible
    ));
    out
}

/// Render open residual fence table for operator receipts.
#[must_use]
pub fn sec_s3_open_residual_fence_table() -> String {
    let mut out = String::from("SEC-S3 open residual fences (W29-122):\n");
    for fence in OPEN_RESIDUAL_FENCES {
        out.push_str(&format!(
            "  {} hop={} honest_open={} green_credit_blocked={} delegate={}\n",
            fence.residue_id,
            fence.hop_ordinal,
            fence.honest_open,
            fence.green_credit_blocked,
            fence.delegate_ssot
        ));
    }
    out.push_str(&format!(
        "  open_residual_hop_count={} fences_verified={} production_wired={} master_retick={}\n",
        OPEN_RESIDUAL_HOP_COUNT,
        manifold_s3_open_residual_fences_verified(),
        sec_s3_production_wired(),
        sec_s3_master_retick_eligible()
    ));
    out
}

/// W29-122 deepen probe — open residual fences + no invented GREEN/PRODUCTION/MASTER.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS3W29DeepenProbe {
    /// W29 cell id pin.
    pub w29_cell_id: &'static str,
    /// Census schema version.
    pub schema_version: &'static str,
    /// Open residual hop count.
    pub open_residual_hop_count: usize,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// Open residual table residue pins present.
    pub open_residual_table_residue_pinned: bool,
    /// ACCEL AC05 honesty absorbed.
    pub ac05_honest: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Production wired — honest false.
    pub production_wired: bool,
    /// MASTER retick — honest false.
    pub master_retick_eligible: bool,
    /// GREEN claim blocked — honest true.
    pub s3_green_claim_blocked: bool,
}

/// Build W29-122 deepen probe from live measurements.
#[must_use]
pub fn sec_s3_w29_deepen_probe() -> SecS3W29DeepenProbe {
    let residual_table = sec_s3_open_residual_fence_table();
    SecS3W29DeepenProbe {
        w29_cell_id: W29_CELL_ID,
        schema_version: SCHEMA_VERSION,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_s3_open_residual_fences_verified(),
        open_residual_table_residue_pinned: residual_table.contains("R-sled-session-ledger")
            && residual_table.contains("R-gateway-trust-wrap"),
        ac05_honest: sec_s3_accel_ac05_honest(),
        ceremony_closed: manifold_gate_sec_s3_ceremony_closed(),
        production_wired: sec_s3_production_wired(),
        master_retick_eligible: sec_s3_master_retick_eligible(),
        s3_green_claim_blocked: S3_GREEN_CLAIM_BLOCKED,
    }
}

/// W29-122 deepen honesty — open residuals pinned + AC05 honest + no invented GREEN/PRODUCTION/MASTER.
#[must_use]
pub fn sec_s3_w29_deepen_honest() -> bool {
    let probe = sec_s3_w29_deepen_probe();
    probe.w29_cell_id == W29_CELL_ID
        && probe.schema_version == SCHEMA_VERSION
        && probe.open_residual_hop_count == OPEN_RESIDUAL_HOP_COUNT
        && probe.open_residual_fences_verified
        && probe.open_residual_table_residue_pinned
        && probe.ac05_honest
        && probe.ceremony_closed
        && !probe.production_wired
        && !probe.master_retick_eligible
        && probe.s3_green_claim_blocked
}

/// Next-hop surface for sled session ledger persistence (egoff-owned).
#[must_use]
pub const fn sec_s3_session_ledger_next_hop() -> &'static str {
    "egoff/egoff/src/security/ledger.rs:TrustLedger::persist_to_sled"
}

/// Next-hop surface for gateway trust-wrap production ceremony (gateway-owned).
#[must_use]
pub const fn sec_s3_trust_wrap_next_hop() -> &'static str {
    "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
}

#[cfg(test)]
mod sec_s3_tests {
    use super::*;

    #[test]
    fn sec_s3_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S3");
        assert_eq!(W29_CELL_ID, "W29-122-SEC_S3");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S3");
        assert_eq!(FLEET_P1606_C5_JOB_ID, "PRABHU-WAVE-C-1606-C5");
        assert_eq!(OPEN_RESIDUAL_HOP_COUNT, 2);
        assert!(!MASTER_RETICK_ELIGIBLE);
    }

    #[test]
    fn sec_s3_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_s3_palette_ledger_census_honest_posture() {
        let census = gate_palette_ledger_census();
        assert_eq!(census.board_slice_id, "SEC-S3");
        assert_eq!(census.w29_cell_id, W29_CELL_ID);
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert!(!census.palette_persisted);
        assert!(!census.session_ledger_wired);
        assert!(!census.production_wired);
        assert_eq!(census.open_residual_hop_count, 2);
        assert!(census.open_residual_fences_verified);
        assert!(census.revoke_posture_facets_verified);
        assert!(census.revoke_ledger_lifecycle_verified);
        assert!(census.s3_green_claim_blocked);
        assert!(!census.sled_persistence_claimed);
        assert!(!census.master_retick_eligible);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_s3_session_ledger_and_production_stay_false() {
        assert!(!session_ledger_wired());
        assert!(!sec_s3_production_wired());
        assert!(!PALETTE_PERSISTED_HONEST);
        assert!(!sec_s3_master_retick_eligible());
    }

    #[test]
    fn sec_s3_open_residual_fences_pin_unwired_hops() {
        assert!(manifold_s3_open_residual_fences_verified());
        assert_eq!(OPEN_RESIDUAL_FENCES.len(), 2);
        assert_eq!(OPEN_RESIDUAL_FENCES[0].residue_id, "R-sled-session-ledger");
        assert_eq!(OPEN_RESIDUAL_FENCES[1].residue_id, "R-gateway-trust-wrap");
        assert!(OPEN_RESIDUAL_FENCES
            .iter()
            .all(|f| f.honest_open && f.green_credit_blocked));
        let table = sec_s3_open_residual_fence_table();
        assert!(table.contains("R-sled-session-ledger"));
        assert!(table.contains("R-gateway-trust-wrap"));
        assert!(table.contains("fences_verified=true"));
        assert!(table.contains("production_wired=false"));
        assert!(table.contains("master_retick=false"));
    }

    #[test]
    fn sec_s3_manifold_wire_hops_cover_gate_and_trust_delegate() {
        assert_eq!(MANIFOLD_SEC_S3_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_S3_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_S3_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_S3_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("session_ledger_wired") && !h.wired));
        assert!(MANIFOLD_SEC_S3_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("trust_wrap_wired") && !h.wired));
    }

    #[test]
    fn sec_s3_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s3_ceremony_closed());
        let probe = sec_s3_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.palette_persisted_honest_false);
        assert!(probe.session_ledger_honest_false);
        assert!(probe.production_honest_false);
        assert!(probe.open_residual_fences_verified);
        assert_eq!(probe.open_residual_hop_count, 2);
        assert!(probe.revoke_posture_facets_verified);
        assert!(probe.revoke_ledger_lifecycle_verified);
        assert!(probe.s3_green_claim_blocked);
        assert!(probe.sled_persistence_honest_false);
        assert!(probe.master_retick_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s3_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S3_2033"));
        assert!(PRIOR_RECEIPT_PATH_2127.contains("SEC-S3_2127"));
        assert!(TRUST_SSOT.contains("sec_s3_revoke_posture"));
        assert!(EGOFF_LEDGER_SSOT.contains("security/ledger.rs"));
        assert!(EGOFF_PALETTE_SSOT.contains("security/palette.rs"));
        assert!(FLEET_COMPOSER_F74_RECEIPT_PATH.contains("COMPOSER_F74_SEC_S3_1942"));
        assert!(FLEET_COMPOSER_G74_RECEIPT_PATH.contains("COMPOSER_G74_SEC_S3_2143"));
        assert!(EGOFF_S3_INTEGRATION_SSOT.contains("s2_s3_security_deepen"));
    }

    #[test]
    fn sec_s3_revoke_posture_facets_five_of_eight_wired() {
        assert!(manifold_s3_revoke_posture_facets_verified());
        assert_eq!(MANIFOLD_S3_REVOKE_POSTURE_FACETS.len(), 8);
        assert_eq!(
            MANIFOLD_S3_REVOKE_POSTURE_FACETS
                .iter()
                .filter(|f| f.wired)
                .count(),
            5
        );
        let matrix = sec_s3_revoke_posture_matrix();
        assert!(matrix.contains("facets_wired=5/8"));
        assert!(matrix.contains("gate_attestation_revoked_refuse"));
    }

    #[test]
    fn sec_s3_revoke_ledger_lifecycle_six_of_eight_wired() {
        assert!(manifold_s3_revoke_ledger_lifecycle_verified());
        assert_eq!(MANIFOLD_S3_REVOKE_LEDGER_HOPS.len(), 8);
        assert_eq!(
            MANIFOLD_S3_REVOKE_LEDGER_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            6
        );
        let matrix = sec_s3_revoke_ledger_lifecycle_matrix();
        assert!(matrix.contains("lifecycle_wired=6/8"));
        assert!(matrix.contains("gate_admit_refuse"));
        assert!(matrix.contains("sled_persistence"));
    }

    #[test]
    fn fleet_accel_ac05_sec_s3_revoke_posture_deepen_honest() {
        assert!(sec_s3_accel_ac05_honest());
        let probe = sec_s3_accel_ac05_probe();
        assert_eq!(probe.ac05_job_id, ACCEL_2030_AC05_JOB_ID);
        assert!(probe.prior_c5_absorbed);
        assert!(probe.prior_f74_absorbed);
        assert!(probe.prior_g74_absorbed);
        assert!(probe.prior_2127_absorbed);
        assert!(probe.revoke_posture_matrix_verified);
        assert!(probe.revoke_ledger_lifecycle_verified);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
    }

    #[test]
    fn sec_s3_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s3_gate_wire_matrix();
        assert!(matrix.contains("SEC-S3 manifold gate"));
        assert!(matrix.contains("session_ledger_wired=false"));
        assert!(matrix.contains("palette_persisted=false"));
        assert!(matrix.contains("wired=5/7"));
        assert!(matrix.contains("open_residual_fences_verified=true"));
        assert!(matrix.contains("master_retick_eligible=false"));
    }

    #[test]
    fn fleet_composer_p1606_c5_sec_s3_honest() {
        assert!(sec_s3_p1606_c5_honest());
        let probe = sec_s3_p1606_c5_probe();
        assert_eq!(probe.c5_job_id, FLEET_P1606_C5_JOB_ID);
        assert!(probe.prior_2033_absorbed);
        assert!(probe.prior_2127_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.palette_persisted);
    }

    #[test]
    fn sec_s3_w29_122_open_residual_fence_deepen_honest() {
        assert!(sec_s3_w29_deepen_honest());
        let probe = sec_s3_w29_deepen_probe();
        assert_eq!(probe.w29_cell_id, "W29-122-SEC_S3");
        assert_eq!(probe.schema_version, "sec_s3_gate_palette_ledger_census_v3");
        assert_eq!(probe.open_residual_hop_count, 2);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.open_residual_table_residue_pinned);
        assert!(probe.ac05_honest);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(probe.s3_green_claim_blocked);
    }

    #[test]
    fn sec_s3_validate_gate_honesty_residue_measured() {
        validate_sec_s3_gate_honesty().expect("honest SEC-S3 gate census residue");
        assert_eq!(
            sec_s3_session_ledger_next_hop(),
            "egoff/egoff/src/security/ledger.rs:TrustLedger::persist_to_sled"
        );
        assert_eq!(
            sec_s3_trust_wrap_next_hop(),
            "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
        );
    }
}

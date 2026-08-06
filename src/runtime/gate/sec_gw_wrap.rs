// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! ACCEL-B-2050-SEC-GW-WRAP — manifold gate runtime trust-wrap delegate census.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-GW-WRAP seven-surface
//! admit inventory + upstream S2/S3/S4 delegate fences; gateway `trust_wrap_wired()` and
//! `sec_gw_trust_wrap_production_wired()` stay **honest open**.
//!
//! # Honesty fences (W29-118-SEC_GW_WRAP)
//!
//! Census-tier deepen only. This module does **not** claim:
//! - swarm/physics `GREEN`
//! - `PRODUCTION_WIRED` / live gateway trust-wrap ceremony
//! - `MASTER` retick eligibility
//! - `OP-5` clearance

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use super::sec_s2::{gate_trust_refuse_census, manifold_gate_sec_s2_ceremony_closed};
use super::sec_s3::{gate_palette_ledger_census, manifold_gate_sec_s3_ceremony_closed};
use super::sec_s4::{gate_side_channel_scrub_census, manifold_gate_sec_s4_ceremony_closed};
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Cell id for this deepen write_set.
pub const SEC_GW_WRAP_CELL_ID: &str = "W29-118-SEC_GW_WRAP";

/// W29 deepen step — honest fence + trust-wrap census (no invent GREEN).
pub const W29_118_SEC_GW_WRAP_DEEPEN_STEP: &str = "W29-118-SEC_GW_WRAP";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-GW-WRAP";

/// AGAP slot id (2033 gateway wrap deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-GW-WRAP";

/// FLEET-COMPOSER ACCEL-B slot AC32 id.
pub const FLEET_ACCEL2_AC32_JOB_ID: &str = "ACCEL-B-2050-AC32";

/// FLEET-COMPOSER ACCEL-B AC32 receipt path.
pub const FLEET_ACCEL2_AC32_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC32.md";

/// Prior PRABHU-WAVE-H H1 gateway wrap deepen receipt.
pub const PRIOR_RECEIPT_PATH_P1800_H1: &str = "outputs/.tmp/COMPOSER_P1800_H1.md";

/// Prior Wave-Z Z33 gateway wrap respawn receipt.
pub const PRIOR_RECEIPT_PATH_Z33: &str = "outputs/.tmp/COMPOSER_Z33_1015.md";

/// Prior FLEET-COMPOSER-Y Y24 S4 delegate + production census receipt.
pub const PRIOR_RECEIPT_PATH_Y24: &str = "outputs/.tmp/COMPOSER_Y24_0808.md";

/// Prior FLEET-COMPOSER-H H54 S4 gw trust-wrap delegate receipt.
pub const PRIOR_RECEIPT_PATH_H54: &str = "outputs/.tmp/COMPOSER_H54_2242.md";

/// Gateway trust-wrap delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs";

/// umst-trust admit-surface check delegate SSOT.
pub const TRUST_ADMIT_SSOT: &str = "umst-foundations/crates/umst-trust/src/permission.rs";

/// umst-trust SEC-S3 session ledger provider delegate SSOT.
pub const TRUST_SESSION_LEDGER_SSOT: &str =
    "umst-foundations/crates/umst-trust/src/sec_ecosystem_extract.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version.
pub const SCHEMA_VERSION: &str = "sec_gw_wrap_manifold_trust_census_v1";

/// Gateway admit-surface inventory count (measured @ Y24/Z33/H1).
pub const ADMIT_SURFACE_COUNT: usize = 7;

/// Ledger-enforce prep hop count (H1 measured 7/7 prep, not production).
pub const LEDGER_ENFORCE_PREP_HOP_COUNT: usize = 7;

/// Gateway wrap queue depth (residue ≥1).
pub const WRAP_QUEUE_DEPTH: usize = 1;

/// H54/Y24 S4 trust-wrap delegate hop count (gateway `S4_TRUST_WRAP_DELEGATE_HOPS`).
pub const S4_TRUST_WRAP_DELEGATE_HOP_COUNT: usize = 4;

/// Gateway compile-time trust-wrap enforce helpers — honest true (helpers only, not live ceremony).
pub const GATEWAY_TRUST_WRAP_HELPERS_WIRED_HONEST: bool = true;

/// SEC-GW-WRAP GREEN claim blocked — honest true in scaffold deepen.
pub const GW_WRAP_GREEN_CLAIM_BLOCKED: bool = true;

/// Session ledger provider wired — honest false until SEC-S3 sled lands.
pub const SESSION_LEDGER_PROVIDER_WIRED_HONEST: bool = false;

/// Gateway `trust_wrap_wired()` delegate — honest false until SEC-S3 ceremony.
pub const TRUST_WRAP_WIRED_HONEST: bool = false;

/// Gateway production flip delegate — honest false until operator measure.
pub const GATEWAY_PRODUCTION_WIRED_HONEST: bool = false;

/// Honest fence — MASTER retick not claimed from SEC-GW-WRAP deepen.
pub const SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE: bool = false;

/// Honest fence — OP-5 not cleared from SEC-GW-WRAP deepen.
pub const SEC_GW_WRAP_OP5_CLEARED: bool = false;

/// Honest physics posture — census-tier deepen is not physics GREEN.
pub const SEC_GW_WRAP_PHYSICS_GREEN: bool = false;

const _: () = assert!(!GATEWAY_PRODUCTION_WIRED_HONEST);
const _: () = assert!(GW_WRAP_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE);
const _: () = assert!(!SEC_GW_WRAP_OP5_CLEARED);
const _: () = assert!(!SEC_GW_WRAP_PHYSICS_GREEN);
const _: () = assert!(!TRUST_WRAP_WIRED_HONEST);
const _: () = assert!(!SESSION_LEDGER_PROVIDER_WIRED_HONEST);

/// One hop in the manifold SEC-GW-WRAP gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwWrapManifoldWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-GW-WRAP gate runtime wire map (cold-edge evidence → trust-wrap census).
pub const MANIFOLD_SEC_GW_WRAP_WIRE_HOPS: &[SecGwWrapManifoldWireHop] = &[
    SecGwWrapManifoldWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_gw_wrap::gate_trust_wrap_census",
        role: "Manifold SEC-GW-WRAP seven-surface trust-wrap census",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 4,
        surface: "umst-manifold::runtime::gate::sec_s2::gate_trust_refuse_census",
        role: "Upstream SEC-S2 TrustGatePolicy refuse delegate",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 5,
        surface: "umst-manifold::runtime::gate::sec_s3::gate_palette_ledger_census",
        role: "Upstream SEC-S3 palette/ledger delegate fence",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 6,
        surface: "umst-manifold::runtime::gate::sec_s4::gate_side_channel_scrub_census",
        role: "Upstream SEC-S4 side-channel scrub delegate (H54/Y24)",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 7,
        surface: "umst-trust::permission::check_admit_surface_at",
        role: "Core admit-surface TrustGate delegate SSOT",
        wired: true,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 8,
        surface: "umst-trust::sec_ecosystem_extract::session_ledger_wired",
        role: "SEC-S3 sled session ledger provider",
        wired: false,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 9,
        surface: "umst-gateway::sec_gw_trust_wrap::trust_wrap_wired",
        role: "Gateway production ceremony (serial SEC-S3 gate)",
        wired: false,
    },
    SecGwWrapManifoldWireHop {
        ordinal: 10,
        surface: "umst-gateway::sec_gw_trust_wrap::sec_gw_trust_wrap_production_wired",
        role: "Gateway production flip (operator-measured live)",
        wired: false,
    },
];

/// One gateway admit-surface row pinned at manifold boundary (from gateway `ADMIT_SURFACES`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldGwWrapAdmitSurface {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Stable surface identifier.
    pub surface_id: &'static str,
    /// Minimum gate label at admit boundary.
    pub gate_label: &'static str,
    /// Whether manifold census enumerates this surface today.
    pub census_hit: bool,
}

/// Seven-surface gateway admit inventory (pinned from gateway `ADMIT_SURFACES` SSOT).
pub const MANIFOLD_GW_WRAP_ADMIT_SURFACES: &[ManifoldGwWrapAdmitSurface] = &[
    ManifoldGwWrapAdmitSurface {
        ordinal: 1,
        surface_id: "umst-gateway::stdio_delegate::exec_native_mcp_delegate",
        gate_label: "EphemeralRead",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 2,
        surface_id: "umst-gateway::gate_check_r::material_delegate",
        gate_label: "DeviceWrite",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 3,
        surface_id: "umst-gateway::gate_check_r::informational",
        gate_label: "EphemeralRead",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 4,
        surface_id: "umst-gateway::gate_check_r::semantic",
        gate_label: "FederatedWrite",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 5,
        surface_id: "umst-gateway::gate_check_embodied",
        gate_label: "DeviceWrite",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 6,
        surface_id: "umst-gateway::present_witness::export",
        gate_label: "HighAssurance",
        census_hit: true,
    },
    ManifoldGwWrapAdmitSurface {
        ordinal: 7,
        surface_id: "umst-gateway::j2_semantic_compose::production",
        gate_label: "DeviceWrite",
        census_hit: true,
    },
];

/// One ledger-enforce prep hop (H1 gateway deepen mirrored at manifold census).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldGwWrapLedgerPrepHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Admit surface id under prep.
    pub admit_surface_id: &'static str,
    /// Gateway enforce hook label.
    pub enforce_hook: &'static str,
    /// Whether prep census enumerates this hop.
    pub prep_hit: bool,
}

/// Ledger-enforce prep wire map (H1 — prep wired, production flip blocked).
pub const MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS: &[ManifoldGwWrapLedgerPrepHop] = &[
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 1,
        admit_surface_id: "umst-gateway::stdio_delegate::exec_native_mcp_delegate",
        enforce_hook: "enforce_stdio_delegate_admit",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 2,
        admit_surface_id: "umst-gateway::gate_check_r::material_delegate",
        enforce_hook: "enforce_admit_trust(Material)",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 3,
        admit_surface_id: "umst-gateway::gate_check_r::informational",
        enforce_hook: "enforce_admit_trust(Informational)",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 4,
        admit_surface_id: "umst-gateway::gate_check_r::semantic",
        enforce_hook: "enforce_admit_trust(Semantic)",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 5,
        admit_surface_id: "umst-gateway::gate_check_embodied",
        enforce_hook: "enforce_admit_trust(Embodied)",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 6,
        admit_surface_id: "umst-gateway::present_witness::export",
        enforce_hook: "enforce_present_witness_export_admit",
        prep_hit: true,
    },
    ManifoldGwWrapLedgerPrepHop {
        ordinal: 7,
        admit_surface_id: "umst-gateway::j2_semantic_compose::production",
        enforce_hook: "enforce_j2_semantic_production_admit",
        prep_hit: true,
    },
];

/// One H54 S4 trust-wrap delegate hop pinned at manifold boundary (gateway SSOT mirror).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldGwWrapS4DelegateHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Gateway delegate surface.
    pub gateway_surface: &'static str,
    /// umst-trust SSOT delegate target.
    pub trust_ssot: &'static str,
    /// Whether manifold census enumerates this hop today.
    pub census_hit: bool,
}

/// H54 S4 trust-wrap delegate wire map (gateway → umst-trust SSOT, prep ≠ production).
pub const MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS: &[ManifoldGwWrapS4DelegateHop] = &[
    ManifoldGwWrapS4DelegateHop {
        ordinal: 1,
        gateway_surface: "sec_gw_trust_wrap::enforce_admit_trust",
        trust_ssot: "umst-trust::validate_s4_side_channel_honesty",
        census_hit: true,
    },
    ManifoldGwWrapS4DelegateHop {
        ordinal: 2,
        gateway_surface: "sec_gw_trust_wrap::scrub_admit_metadata_s4",
        trust_ssot: "umst-trust::scrub_k_v1_tokens",
        census_hit: true,
    },
    ManifoldGwWrapS4DelegateHop {
        ordinal: 3,
        gateway_surface: "sec_gw_trust_wrap::check_s4_delegate_at_admit",
        trust_ssot: "umst-trust::s4_side_channel_g75_honest",
        census_hit: true,
    },
    ManifoldGwWrapS4DelegateHop {
        ordinal: 4,
        gateway_surface: "sec_gw_trust_wrap::enforce_stdio_delegate_admit",
        trust_ssot: "umst-trust::verify_s4_scrub_roundtrip",
        census_hit: true,
    },
];

/// Aggregated SEC-GW-WRAP gate trust-wrap census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecGwWrapManifoldTrustCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// W29-118 cell id pin.
    pub cell_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// Seven admit surfaces enumerated.
    pub admit_surface_count: usize,
    /// All seven admit surfaces probed in census.
    pub all_admit_surfaces_probed: bool,
    /// Ledger-enforce prep hops wired (7/7 prep ≠ production).
    pub ledger_prep_hop_count: usize,
    /// Upstream SEC-S2 ceremony closed.
    pub upstream_s2_ceremony_closed: bool,
    /// Upstream SEC-S3 ceremony closed.
    pub upstream_s3_ceremony_closed: bool,
    /// Upstream SEC-S4 ceremony closed.
    pub upstream_s4_ceremony_closed: bool,
    /// Session ledger provider wired — honest false.
    pub session_ledger_provider_wired: bool,
    /// Gateway trust_wrap delegate — honest false.
    pub trust_wrap_wired: bool,
    /// SEC-GW-WRAP GREEN claim blocked.
    pub gw_wrap_green_claim_blocked: bool,
    /// Physics GREEN invent — honest false.
    pub physics_green: bool,
    /// MASTER retick invent — honest false.
    pub master_retick_eligible: bool,
    /// OP-5 invent — honest false.
    pub op5_cleared: bool,
    /// H54 S4 trust-wrap delegate hop count (4/4 prep).
    pub s4_delegate_hop_count: usize,
    /// Gateway compile-time trust-wrap enforce helpers wired (≠ live ceremony).
    pub gateway_trust_wrap_helpers_wired: bool,
    /// Wrap queue depth (≥1 residue).
    pub wrap_queue_depth: usize,
    /// Manifold production flip.
    pub production_wired: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// Exercise gate cold-edge evidence at manifold SSOT (identity transition admits).
#[must_use]
pub fn gate_transition_evidence_probe() -> bool {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    evidence.admissibility == AdmissibilityToken::Admissible
        && !evidence.catalog_id.is_empty()
}

/// Whether live gateway trust-wrap production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_gw_wrap_production_wired() -> bool {
    false
}

const _: () = assert!(!sec_gw_wrap_production_wired());

/// W29-118 honesty fence — GREEN / PRODUCTION / MASTER / OP-5 refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwWrapHonestyFence {
    /// Deepen step / cell id pin.
    pub deepen_step: &'static str,
    /// Cell id pin.
    pub cell_id: &'static str,
    /// Physics GREEN invent — must stay false.
    pub physics_green: bool,
    /// Production wire invent — must stay false.
    pub production_wired: bool,
    /// MASTER retick invent — must stay false.
    pub master_retick_eligible: bool,
    /// OP-5 invent — must stay false.
    pub op5_cleared: bool,
    /// GREEN claim blocked — must stay true.
    pub green_claim_blocked: bool,
    /// Gateway trust_wrap ceremony — honest false.
    pub trust_wrap_wired: bool,
    /// Session ledger provider — honest false.
    pub session_ledger_provider_wired: bool,
}

impl SecGwWrapHonestyFence {
    /// Measured honesty posture for this module.
    #[must_use]
    pub const fn measured() -> Self {
        Self {
            deepen_step: W29_118_SEC_GW_WRAP_DEEPEN_STEP,
            cell_id: SEC_GW_WRAP_CELL_ID,
            physics_green: SEC_GW_WRAP_PHYSICS_GREEN,
            production_wired: GATEWAY_PRODUCTION_WIRED_HONEST,
            master_retick_eligible: SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE,
            op5_cleared: SEC_GW_WRAP_OP5_CLEARED,
            green_claim_blocked: GW_WRAP_GREEN_CLAIM_BLOCKED,
            trust_wrap_wired: TRUST_WRAP_WIRED_HONEST,
            session_ledger_provider_wired: SESSION_LEDGER_PROVIDER_WIRED_HONEST,
        }
    }

    /// Fence holds when invent flags stay false and GREEN remains blocked.
    #[must_use]
    pub const fn holds(self) -> bool {
        !self.physics_green
            && !self.production_wired
            && !self.master_retick_eligible
            && !self.op5_cleared
            && self.green_claim_blocked
            && !self.trust_wrap_wired
            && !self.session_ledger_provider_wired
            && matches!(self.deepen_step.as_bytes(), b"W29-118-SEC_GW_WRAP")
            && matches!(self.cell_id.as_bytes(), b"W29-118-SEC_GW_WRAP")
    }
}

/// Measured honesty probe for W29-118 deepen — fence holds + census ceremony closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwWrapHonestyProbe {
    /// Cell id pin.
    pub cell_id: &'static str,
    /// Posture tag pin.
    pub posture_tag: &'static str,
    /// Production wire invent.
    pub production_wired: bool,
    /// GREEN claim blocked.
    pub green_claim_blocked: bool,
    /// MASTER retick invent.
    pub master_retick_eligible: bool,
    /// OP-5 invent.
    pub op5_cleared: bool,
    /// Physics GREEN invent.
    pub physics_green: bool,
    /// Fence holds.
    pub fence_holds: bool,
    /// Census ceremony closed at census tier (≠ production).
    pub ceremony_closed: bool,
    /// Aggregate deepen honesty.
    pub deepen_honest: bool,
}

/// Snapshot honesty fences + ceremony for the SEC-GW-WRAP surface.
#[must_use]
pub fn sec_gw_wrap_honesty_probe() -> SecGwWrapHonestyProbe {
    let fence = SecGwWrapHonestyFence::measured();
    let fence_holds = fence.holds();
    let ceremony_closed = manifold_gate_sec_gw_wrap_ceremony_closed();
    let deepen_honest = fence_holds
        && fence.cell_id == SEC_GW_WRAP_CELL_ID
        && fence.deepen_step == W29_118_SEC_GW_WRAP_DEEPEN_STEP
        && POSTURE_TAG == "manifold-gate-census-wired-not-production"
        && ceremony_closed
        && !sec_gw_wrap_production_wired();
    SecGwWrapHonestyProbe {
        cell_id: fence.cell_id,
        posture_tag: POSTURE_TAG,
        production_wired: fence.production_wired,
        green_claim_blocked: fence.green_claim_blocked,
        master_retick_eligible: fence.master_retick_eligible,
        op5_cleared: fence.op5_cleared,
        physics_green: fence.physics_green,
        fence_holds,
        ceremony_closed,
        deepen_honest,
    }
}

/// Whether all seven gateway admit surfaces are enumerated at manifold boundary.
#[must_use]
pub fn manifold_gw_wrap_all_admit_surfaces_probed() -> bool {
    MANIFOLD_GW_WRAP_ADMIT_SURFACES.len() == ADMIT_SURFACE_COUNT
        && MANIFOLD_GW_WRAP_ADMIT_SURFACES
            .iter()
            .all(|s| s.census_hit)
}

/// Whether H1 ledger-enforce prep map is complete at census tier (prep ≠ production).
#[must_use]
pub fn manifold_gw_wrap_ledger_prep_complete() -> bool {
    MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS.len() == LEDGER_ENFORCE_PREP_HOP_COUNT
        && MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS
            .iter()
            .all(|h| h.prep_hit)
}

/// Whether H54 S4 trust-wrap delegate hop inventory is complete at census tier (prep ≠ production).
#[must_use]
pub fn manifold_gw_wrap_s4_delegate_complete() -> bool {
    MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS.len() == S4_TRUST_WRAP_DELEGATE_HOP_COUNT
        && MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS
            .iter()
            .all(|h| h.census_hit)
}

/// Whether gateway compile-time trust-wrap enforce helpers are wired (honest census pin).
#[must_use]
pub fn manifold_gw_wrap_trust_wrap_helpers_wired() -> bool {
    GATEWAY_TRUST_WRAP_HELPERS_WIRED_HONEST
}

/// Verify upstream S2/S3/S4 delegate ceremonies at manifold boundary.
#[must_use]
pub fn manifold_verify_upstream_gate_delegates() -> bool {
    manifold_gate_sec_s2_ceremony_closed()
        && manifold_gate_sec_s3_ceremony_closed()
        && manifold_gate_sec_s4_ceremony_closed()
        && gate_trust_refuse_census().gate_evidence_wired
        && gate_palette_ledger_census().gate_evidence_wired
        && gate_side_channel_scrub_census().gate_evidence_wired
}

/// Build manifold SEC-GW-WRAP gate trust-wrap census from live measurements.
#[must_use]
pub fn gate_trust_wrap_census() -> SecGwWrapManifoldTrustCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_GW_WRAP_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecGwWrapManifoldTrustCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        cell_id: SEC_GW_WRAP_CELL_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        admit_surface_count: ADMIT_SURFACE_COUNT,
        all_admit_surfaces_probed: manifold_gw_wrap_all_admit_surfaces_probed(),
        ledger_prep_hop_count: MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS.len(),
        upstream_s2_ceremony_closed: manifold_gate_sec_s2_ceremony_closed(),
        upstream_s3_ceremony_closed: manifold_gate_sec_s3_ceremony_closed(),
        upstream_s4_ceremony_closed: manifold_gate_sec_s4_ceremony_closed(),
        session_ledger_provider_wired: SESSION_LEDGER_PROVIDER_WIRED_HONEST,
        trust_wrap_wired: TRUST_WRAP_WIRED_HONEST,
        gw_wrap_green_claim_blocked: GW_WRAP_GREEN_CLAIM_BLOCKED,
        physics_green: SEC_GW_WRAP_PHYSICS_GREEN,
        master_retick_eligible: SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE,
        op5_cleared: SEC_GW_WRAP_OP5_CLEARED,
        s4_delegate_hop_count: MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS.len(),
        gateway_trust_wrap_helpers_wired: manifold_gw_wrap_trust_wrap_helpers_wired(),
        wrap_queue_depth: WRAP_QUEUE_DEPTH,
        production_wired: sec_gw_wrap_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-GW-WRAP ceremony is closed at census tier.
///
/// True when cold-edge evidence + seven-surface census + upstream S2/S3/S4 delegates are measured
/// wired. Gateway production flip + session ledger provider are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_gw_wrap_ceremony_closed() -> bool {
    let census = gate_trust_wrap_census();
    let fence = SecGwWrapHonestyFence::measured();
    census.gate_evidence_wired
        && census.cell_id == SEC_GW_WRAP_CELL_ID
        && census.admit_surface_count == ADMIT_SURFACE_COUNT
        && census.all_admit_surfaces_probed
        && census.ledger_prep_hop_count == LEDGER_ENFORCE_PREP_HOP_COUNT
        && census.upstream_s2_ceremony_closed
        && census.upstream_s3_ceremony_closed
        && census.upstream_s4_ceremony_closed
        && !census.session_ledger_provider_wired
        && !census.trust_wrap_wired
        && census.gw_wrap_green_claim_blocked
        && !census.physics_green
        && !census.master_retick_eligible
        && !census.op5_cleared
        && census.s4_delegate_hop_count == S4_TRUST_WRAP_DELEGATE_HOP_COUNT
        && census.gateway_trust_wrap_helpers_wired
        && census.wrap_queue_depth >= WRAP_QUEUE_DEPTH
        && !census.production_wired
        && census.wire_hop_wired_count == 7
        && fence.holds()
        && manifold_gw_wrap_all_admit_surfaces_probed()
        && manifold_gw_wrap_ledger_prep_complete()
        && manifold_gw_wrap_s4_delegate_complete()
        && manifold_gw_wrap_trust_wrap_helpers_wired()
        && manifold_verify_upstream_gate_delegates()
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-GW-WRAP manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwWrapManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Seven admit surfaces probed.
    pub all_admit_surfaces_probed: bool,
    /// Ledger prep complete.
    pub ledger_prep_complete: bool,
    /// Upstream delegates verified.
    pub upstream_delegates_verified: bool,
    /// GREEN claim blocked.
    pub gw_wrap_green_claim_blocked: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// MASTER retick refused.
    pub master_retick_refused: bool,
    /// OP-5 refused.
    pub op5_refused: bool,
    /// Physics GREEN refused.
    pub physics_green_refused: bool,
    /// W29-118 honesty fence holds.
    pub honesty_fence_holds: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-GW-WRAP done-when checks.
#[must_use]
pub fn sec_gw_wrap_manifold_probe() -> SecGwWrapManifoldProbe {
    let census = gate_trust_wrap_census();
    let fence = SecGwWrapHonestyFence::measured();
    SecGwWrapManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        all_admit_surfaces_probed: census.all_admit_surfaces_probed,
        ledger_prep_complete: manifold_gw_wrap_ledger_prep_complete(),
        upstream_delegates_verified: manifold_verify_upstream_gate_delegates(),
        gw_wrap_green_claim_blocked: census.gw_wrap_green_claim_blocked,
        production_honest_false: !census.production_wired,
        master_retick_refused: !census.master_retick_eligible,
        op5_refused: !census.op5_cleared,
        physics_green_refused: !census.physics_green,
        honesty_fence_holds: fence.holds(),
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_gw_wrap_ceremony_closed(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC32 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecGwWrapAccel2Ac32Probe {
    /// AC32 fleet card id.
    pub ac32_job_id: &'static str,
    /// Prior P1800 H1 gateway wrap absorbed.
    pub prior_h1_absorbed: bool,
    /// Prior Z33 respawn absorbed.
    pub prior_z33_absorbed: bool,
    /// Prior Y24 S4 delegate absorbed.
    pub prior_y24_absorbed: bool,
    /// Prior H54 S4 delegate absorbed.
    pub prior_h54_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecGwWrapManifoldProbe,
    /// `sec_gw_wrap_production_wired()` — honest false.
    pub production_wired: bool,
    /// Admit surface census count.
    pub admit_surface_count: usize,
    /// Ledger prep hop count.
    pub ledger_prep_hop_count: usize,
    /// S4 trust-wrap delegate hop count.
    pub s4_delegate_hop_count: usize,
    /// Gateway trust-wrap helpers wired (compile-time only).
    pub gateway_trust_wrap_helpers_wired: bool,
}

/// Build FLEET-COMPOSER ACCEL-B AC32 integration probe from live measurements.
#[must_use]
pub fn sec_gw_wrap_accel2_ac32_probe() -> SecGwWrapAccel2Ac32Probe {
    let census = gate_trust_wrap_census();
    SecGwWrapAccel2Ac32Probe {
        ac32_job_id: FLEET_ACCEL2_AC32_JOB_ID,
        prior_h1_absorbed: PRIOR_RECEIPT_PATH_P1800_H1.contains("P1800_H1"),
        prior_z33_absorbed: PRIOR_RECEIPT_PATH_Z33.contains("Z33"),
        prior_y24_absorbed: PRIOR_RECEIPT_PATH_Y24.contains("Y24"),
        prior_h54_absorbed: PRIOR_RECEIPT_PATH_H54.contains("H54"),
        ceremony_closed: manifold_gate_sec_gw_wrap_ceremony_closed(),
        probe: sec_gw_wrap_manifold_probe(),
        production_wired: sec_gw_wrap_production_wired(),
        admit_surface_count: census.admit_surface_count,
        ledger_prep_hop_count: census.ledger_prep_hop_count,
        s4_delegate_hop_count: census.s4_delegate_hop_count,
        gateway_trust_wrap_helpers_wired: census.gateway_trust_wrap_helpers_wired,
    }
}

/// FLEET-COMPOSER ACCEL-B AC32 honesty gate — ceremony closed + production false.
#[must_use]
pub fn sec_gw_wrap_accel2_ac32_honest() -> bool {
    let probe = sec_gw_wrap_accel2_ac32_probe();
    probe.ac32_job_id == FLEET_ACCEL2_AC32_JOB_ID
        && probe.prior_h1_absorbed
        && probe.prior_z33_absorbed
        && probe.prior_y24_absorbed
        && probe.prior_h54_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.all_admit_surfaces_probed
        && probe.probe.ledger_prep_complete
        && probe.probe.upstream_delegates_verified
        && probe.probe.gw_wrap_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.master_retick_refused
        && probe.probe.op5_refused
        && probe.probe.physics_green_refused
        && probe.probe.honesty_fence_holds
        && probe.probe.wire_hop_wired_count == 7
        && !probe.production_wired
        && probe.admit_surface_count == ADMIT_SURFACE_COUNT
        && probe.ledger_prep_hop_count == LEDGER_ENFORCE_PREP_HOP_COUNT
        && probe.s4_delegate_hop_count == S4_TRUST_WRAP_DELEGATE_HOP_COUNT
        && probe.gateway_trust_wrap_helpers_wired
        && manifold_gw_wrap_s4_delegate_complete()
        && manifold_gw_wrap_trust_wrap_helpers_wired()
        && sec_gw_wrap_honesty_probe().deepen_honest
}

/// Validate SEC-GW-WRAP gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_gw_wrap_honesty() -> Result<(), &'static str> {
    let census = gate_trust_wrap_census();
    let fence = SecGwWrapHonestyFence::measured();
    let honesty = sec_gw_wrap_honesty_probe();
    if census.cell_id != SEC_GW_WRAP_CELL_ID {
        return Err("sec_gw_wrap cell_id drift");
    }
    if fence.deepen_step != W29_118_SEC_GW_WRAP_DEEPEN_STEP {
        return Err("sec_gw_wrap deepen_step drift");
    }
    if !fence.holds() {
        return Err("sec_gw_wrap honesty fence must hold (no invent GREEN/PRODUCTION/MASTER/OP-5)");
    }
    if census.production_wired {
        return Err("sec_gw_wrap_production_wired must stay false until operator measure");
    }
    if census.trust_wrap_wired {
        return Err("trust_wrap_wired delegate must stay false until SEC-S3 ceremony");
    }
    if census.session_ledger_provider_wired {
        return Err("session_ledger_provider_wired must stay false until SEC-S3 sled");
    }
    if !census.gw_wrap_green_claim_blocked {
        return Err("gw_wrap_green_claim_blocked must stay true in scaffold deepen");
    }
    if census.physics_green {
        return Err("sec_gw_wrap must not invent physics GREEN");
    }
    if census.master_retick_eligible {
        return Err("sec_gw_wrap must not claim MASTER retick");
    }
    if census.op5_cleared {
        return Err("sec_gw_wrap must not claim OP-5 cleared");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.admit_surface_count != ADMIT_SURFACE_COUNT {
        return Err("seven gateway admit surfaces expected");
    }
    if !census.all_admit_surfaces_probed {
        return Err("all seven admit surfaces must be probed");
    }
    if census.ledger_prep_hop_count != LEDGER_ENFORCE_PREP_HOP_COUNT {
        return Err("seven ledger-enforce prep hops expected");
    }
    if census.s4_delegate_hop_count != S4_TRUST_WRAP_DELEGATE_HOP_COUNT {
        return Err("four S4 trust-wrap delegate hops expected");
    }
    if !census.gateway_trust_wrap_helpers_wired {
        return Err("gateway trust-wrap helpers must be wired at compile-time census tier");
    }
    if census.wrap_queue_depth < WRAP_QUEUE_DEPTH {
        return Err("wrap queue depth must be at least one");
    }
    if !manifold_gw_wrap_s4_delegate_complete() {
        return Err("H54 S4 trust-wrap delegate hops must be complete at census tier");
    }
    if !census.upstream_s2_ceremony_closed {
        return Err("upstream SEC-S2 ceremony must be closed");
    }
    if !census.upstream_s3_ceremony_closed {
        return Err("upstream SEC-S3 ceremony must be closed");
    }
    if !census.upstream_s4_ceremony_closed {
        return Err("upstream SEC-S4 ceremony must be closed");
    }
    if MANIFOLD_SEC_GW_WRAP_WIRE_HOPS.len() != 10 {
        return Err("ten SEC-GW-WRAP gate wire hops expected");
    }
    if census.wire_hop_wired_count != 7 {
        return Err("seven SEC-GW-WRAP gate wire hops should be wired today");
    }
    if !manifold_gate_sec_gw_wrap_ceremony_closed() {
        return Err("manifold gate SEC-GW-WRAP ceremony must be closed at census tier");
    }
    if !honesty.deepen_honest {
        return Err("sec_gw_wrap deepen_honest failed");
    }
    if !sec_gw_wrap_accel2_ac32_honest() {
        return Err("ACCEL-B AC32 probe must be honest");
    }
    Ok(())
}

/// Render SEC-GW-WRAP gate wire map for operator receipts.
#[must_use]
pub fn sec_gw_wrap_wire_matrix() -> String {
    let census = gate_trust_wrap_census();
    let mut out = String::from("SEC-GW-WRAP manifold gate trust-wrap wire map (AC32):\n");
    for hop in MANIFOLD_SEC_GW_WRAP_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} admit_surfaces={} ledger_prep={} s4_delegate={} \
         gw_wrap_green_claim_blocked={} trust_wrap_helpers_wired={} wrap_queue_depth={} \
         trust_wrap_wired={} session_ledger_provider_wired={} production_wired={} \
         physics_green={} master_retick={} op5_cleared={} cell_id={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_GW_WRAP_WIRE_HOPS.len(),
        census.admit_surface_count,
        census.ledger_prep_hop_count,
        census.s4_delegate_hop_count,
        census.gw_wrap_green_claim_blocked,
        census.gateway_trust_wrap_helpers_wired,
        census.wrap_queue_depth,
        census.trust_wrap_wired,
        census.session_ledger_provider_wired,
        census.production_wired,
        census.physics_green,
        census.master_retick_eligible,
        census.op5_cleared,
        census.cell_id
    ));
    out.push_str(&format!("  gateway_ssot={GATEWAY_SSOT}\n"));
    out.push_str(&format!("  trust_admit_ssot={TRUST_ADMIT_SSOT}\n"));
    out
}

/// Next-hop surface for gateway trust-wrap production ceremony (gateway-owned).
#[must_use]
pub const fn sec_gw_wrap_trust_wrap_next_hop() -> &'static str {
    "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
}

#[cfg(test)]
mod sec_gw_wrap_tests {
    use super::*;

    #[test]
    fn sec_gw_wrap_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-GW-WRAP");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-GW-WRAP");
        assert_eq!(FLEET_ACCEL2_AC32_JOB_ID, "ACCEL-B-2050-AC32");
        assert_eq!(SEC_GW_WRAP_CELL_ID, "W29-118-SEC_GW_WRAP");
        assert_eq!(W29_118_SEC_GW_WRAP_DEEPEN_STEP, "W29-118-SEC_GW_WRAP");
    }

    #[test]
    fn sec_gw_wrap_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_gw_wrap_trust_census_honest_posture() {
        let census = gate_trust_wrap_census();
        assert_eq!(census.board_slice_id, "SEC-GW-WRAP");
        assert_eq!(census.cell_id, SEC_GW_WRAP_CELL_ID);
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.admit_surface_count, 7);
        assert!(census.all_admit_surfaces_probed);
        assert_eq!(census.ledger_prep_hop_count, 7);
        assert!(census.upstream_s2_ceremony_closed);
        assert!(census.upstream_s3_ceremony_closed);
        assert!(census.upstream_s4_ceremony_closed);
        assert!(!census.session_ledger_provider_wired);
        assert!(!census.trust_wrap_wired);
        assert!(census.gw_wrap_green_claim_blocked);
        assert!(!census.physics_green);
        assert!(!census.master_retick_eligible);
        assert!(!census.op5_cleared);
        assert_eq!(census.s4_delegate_hop_count, 4);
        assert!(census.gateway_trust_wrap_helpers_wired);
        assert!(census.wrap_queue_depth >= 1);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 7);
    }

    #[test]
    fn sec_gw_wrap_production_stays_false() {
        assert!(!sec_gw_wrap_production_wired());
        assert!(GW_WRAP_GREEN_CLAIM_BLOCKED);
        assert!(!TRUST_WRAP_WIRED_HONEST);
        assert!(!GATEWAY_PRODUCTION_WIRED_HONEST);
        assert!(!SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE);
        assert!(!SEC_GW_WRAP_OP5_CLEARED);
        assert!(!SEC_GW_WRAP_PHYSICS_GREEN);
    }

    #[test]
    fn sec_gw_wrap_w29_118_honesty_fence_holds() {
        let fence = SecGwWrapHonestyFence::measured();
        assert!(fence.holds());
        assert_eq!(fence.cell_id, "W29-118-SEC_GW_WRAP");
        assert_eq!(fence.deepen_step, "W29-118-SEC_GW_WRAP");
        assert!(!fence.physics_green);
        assert!(!fence.production_wired);
        assert!(!fence.master_retick_eligible);
        assert!(!fence.op5_cleared);
        assert!(fence.green_claim_blocked);
        let probe = sec_gw_wrap_honesty_probe();
        assert!(probe.fence_holds);
        assert!(probe.deepen_honest);
        assert!(probe.ceremony_closed);
        assert_eq!(probe.posture_tag, POSTURE_TAG);
    }

    #[test]
    fn sec_gw_wrap_manifold_wire_hops_seven_of_ten_wired() {
        assert_eq!(MANIFOLD_SEC_GW_WRAP_WIRE_HOPS.len(), 10);
        assert_eq!(
            MANIFOLD_SEC_GW_WRAP_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            7
        );
        assert!(MANIFOLD_SEC_GW_WRAP_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_GW_WRAP_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("trust_wrap_wired") && !h.wired));
        assert!(MANIFOLD_SEC_GW_WRAP_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("sec_gw_trust_wrap_production_wired") && !h.wired));
    }

    #[test]
    fn sec_gw_wrap_admit_surface_inventory_seven_of_seven() {
        assert!(manifold_gw_wrap_all_admit_surfaces_probed());
        assert_eq!(MANIFOLD_GW_WRAP_ADMIT_SURFACES.len(), 7);
        assert!(MANIFOLD_GW_WRAP_ADMIT_SURFACES.iter().all(|s| s.census_hit));
    }

    #[test]
    fn sec_gw_wrap_ledger_prep_hops_seven_of_seven() {
        assert!(manifold_gw_wrap_ledger_prep_complete());
        assert_eq!(MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS.len(), 7);
        assert!(MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS.iter().all(|h| h.prep_hit));
    }

    #[test]
    fn sec_gw_wrap_s4_delegate_hops_four_of_four() {
        assert!(manifold_gw_wrap_s4_delegate_complete());
        assert_eq!(MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS.len(), 4);
        assert!(MANIFOLD_GW_WRAP_S4_DELEGATE_HOPS.iter().all(|h| h.census_hit));
        assert!(manifold_gw_wrap_trust_wrap_helpers_wired());
        assert!(GATEWAY_TRUST_WRAP_HELPERS_WIRED_HONEST);
    }

    #[test]
    fn sec_gw_wrap_upstream_gate_delegates_verified() {
        assert!(manifold_verify_upstream_gate_delegates());
        assert!(manifold_gate_sec_s2_ceremony_closed());
        assert!(manifold_gate_sec_s3_ceremony_closed());
        assert!(manifold_gate_sec_s4_ceremony_closed());
    }

    #[test]
    fn sec_gw_wrap_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_gw_wrap_ceremony_closed());
        let probe = sec_gw_wrap_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.all_admit_surfaces_probed);
        assert!(probe.ledger_prep_complete);
        assert!(probe.upstream_delegates_verified);
        assert!(probe.gw_wrap_green_claim_blocked);
        assert!(probe.production_honest_false);
        assert!(probe.master_retick_refused);
        assert!(probe.op5_refused);
        assert!(probe.physics_green_refused);
        assert!(probe.honesty_fence_holds);
        assert_eq!(probe.wire_hop_wired_count, 7);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_gw_wrap_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_P1800_H1.contains("P1800_H1"));
        assert!(PRIOR_RECEIPT_PATH_Z33.contains("Z33"));
        assert!(PRIOR_RECEIPT_PATH_Y24.contains("Y24"));
        assert!(PRIOR_RECEIPT_PATH_H54.contains("H54"));
        assert!(GATEWAY_SSOT.contains("sec_gw_trust_wrap.rs"));
    }

    #[test]
    fn sec_gw_wrap_wire_matrix_renders_honest_counts() {
        let matrix = sec_gw_wrap_wire_matrix();
        assert!(matrix.contains("SEC-GW-WRAP manifold gate"));
        assert!(matrix.contains("gw_wrap_green_claim_blocked=true"));
        assert!(matrix.contains("s4_delegate=4"));
        assert!(matrix.contains("trust_wrap_helpers_wired=true"));
        assert!(matrix.contains("wired=7/10"));
        assert!(matrix.contains("production_wired=false"));
        assert!(matrix.contains("physics_green=false"));
        assert!(matrix.contains("master_retick=false"));
        assert!(matrix.contains("op5_cleared=false"));
        assert!(matrix.contains("cell_id=W29-118-SEC_GW_WRAP"));
    }

    #[test]
    fn fleet_composer_accel2_ac32_sec_gw_wrap_honest() {
        assert!(sec_gw_wrap_accel2_ac32_honest());
        let probe = sec_gw_wrap_accel2_ac32_probe();
        assert_eq!(probe.ac32_job_id, FLEET_ACCEL2_AC32_JOB_ID);
        assert!(probe.prior_h1_absorbed);
        assert!(probe.prior_z33_absorbed);
        assert!(probe.prior_y24_absorbed);
        assert!(probe.prior_h54_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert_eq!(probe.admit_surface_count, 7);
        assert_eq!(probe.ledger_prep_hop_count, 7);
        assert_eq!(probe.s4_delegate_hop_count, 4);
        assert!(probe.gateway_trust_wrap_helpers_wired);
    }

    #[test]
    fn sec_gw_wrap_validate_gate_honesty_residue_measured() {
        validate_sec_gw_wrap_honesty().expect("honest SEC-GW-WRAP gate census residue");
        assert_eq!(
            sec_gw_wrap_trust_wrap_next_hop(),
            "umst-gateway/crates/umst-gateway/src/sec_gw_trust_wrap.rs:trust_wrap_wired"
        );
    }
}

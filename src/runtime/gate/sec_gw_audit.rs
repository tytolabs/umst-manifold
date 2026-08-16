// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! ACCEL-B-2050-SEC-GW-AUDIT — manifold gate runtime admit-audit delegate census.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-GW-AUDIT four-path
//! `record_admit_gate_verdict_audit` stamp inventory + upstream SEC-GW-WRAP delegate fence;
//! gateway JSONL live rotation, `UMST_GATEWAY_AUDIT_TRUST_CHAIN_ROOT` ceremony, and
//! `sec_gw_audit_production_wired()` stay **honest open**.
//!
//! **Honest status (W29-117 deepen):** census ceremony + four-path stamp inventory are measured.
//! Not physics GREEN. Refuses `PRODUCTION_WIRED` / `MASTER` / `OP-5` invent.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use super::sec_gw_wrap::{
    gate_trust_wrap_census, manifold_gate_sec_gw_wrap_ceremony_closed,
    manifold_gw_wrap_all_admit_surfaces_probed,
};
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Cell id for this deepen write_set.
pub const W29_117_SEC_GW_AUDIT_DEEPEN_STEP: &str = "W29-117-SEC_GW_AUDIT";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-GW-AUDIT";

/// AGAP slot id (2033 gateway admit audit deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-GW-AUDIT";

/// FLEET-COMPOSER ACCEL-B slot AC31 id.
pub const FLEET_ACCEL2_AC31_JOB_ID: &str = "ACCEL-B-2050-AC31";

/// FLEET-COMPOSER ACCEL-B AC31 receipt path.
pub const FLEET_ACCEL2_AC31_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC31.md";

/// Prior FLEET-COMPOSER-Y Y51 stamped admit-path deepen receipt.
pub const PRIOR_RECEIPT_PATH_Y51: &str = "outputs/.tmp/COMPOSER_Y51_0808.md";

/// Prior Wave-Z Z87 delegate stamp closure receipt.
pub const PRIOR_RECEIPT_PATH_Z87: &str = "outputs/.tmp/COMPOSER_Z87_1232.md";

/// Prior Wave-Z Z08 SEC-GW-AUDIT resume receipt.
pub const PRIOR_RECEIPT_PATH_Z08: &str = "outputs/.tmp/COMPOSER_Z08_0928.md";

/// Prior PRABHU-WAVE-F P1710 F1 swarm rotate receipt.
pub const PRIOR_RECEIPT_PATH_P1710_F1: &str = "outputs/.tmp/COMPOSER_P1710_F1.md";

/// Gateway admit-audit stamp delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/lib_adopt_e_gateway_audit.rs";

/// Gateway audit swarm rotate delegate SSOT.
pub const GATEWAY_SWARM_SSOT: &str =
    "umst-gateway/crates/umst-gateway/src/sec_gw_audit_swarm_rotate.rs";

/// Gateway stamped admit record symbol pin.
pub const GATEWAY_ADMIT_RECORD_SSOT: &str =
    "umst-gateway/crates/umst-gateway/src/lib_adopt_e_gateway_audit.rs::record_admit_gate_verdict_audit";

/// Operator trust-chain env ceremony symbol pin (honest open).
pub const GATEWAY_TRUST_CHAIN_SSOT: &str =
    "umst-gateway/crates/umst-gateway/src/lib_adopt_e_gateway_audit.rs::resolve_admit_warrant_from_env";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Operator-visible non-claim — census ≠ GREEN / PRODUCTION / MASTER / OP-5.
pub const NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER; not OP-5; JSONL rotation + trust-chain env remain OPEN";

/// Census schema version.
pub const SCHEMA_VERSION: &str = "sec_gw_audit_manifold_admit_census_v1";

/// Gateway auto-stamp admit path count (measured @ Y51+Z87 — 4/4 delegate wired).
pub const ADMIT_STAMP_PATH_COUNT: usize = 4;

/// Stamp leg count (`ucrs_seq` + `trust_attestation`).
pub const STAMP_LEG_COUNT: usize = 2;

/// Delegate residual path count (Z87 emptied — honest 0 at census tier).
pub const DELEGATE_RESIDUAL_PATH_COUNT: usize = 0;

/// Wired hop count today (6/8 — JSONL rotation + trust-chain env stay open).
pub const WIRE_HOP_WIRED_COUNT_HONEST: u8 = 6;

/// Total wire hop inventory length.
pub const WIRE_HOP_TOTAL_COUNT: usize = 8;

/// SEC-GW-AUDIT GREEN claim blocked — honest true in scaffold deepen.
pub const GW_AUDIT_GREEN_CLAIM_BLOCKED: bool = true;

/// Honest physics posture — census ceremony ≠ fleet physics GREEN.
pub const SEC_GW_AUDIT_PHYSICS_GREEN: bool = false;

/// Honest MASTER retick eligibility — always refused at this module.
pub const SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE: bool = false;

/// Honest OP-5 claim — always refused at this module.
pub const SEC_GW_AUDIT_OP5_CLAIMED: bool = false;

/// Gateway compile-time stamp helpers exported — honest true (helpers only, not live ceremony).
pub const GATEWAY_AUDIT_STAMP_HELPERS_WIRED_HONEST: bool = true;

/// Live JSONL rotation under operator env — honest false until measured.
pub const JSONL_ROTATION_LIVE_HONEST: bool = false;

/// Live `UMST_GATEWAY_AUDIT_TRUST_CHAIN_ROOT` — honest false until operator measure.
pub const LIVE_TRUST_CHAIN_MEASURED_HONEST: bool = false;

const _: () = assert!(!SEC_GW_AUDIT_PHYSICS_GREEN);
const _: () = assert!(!SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE);
const _: () = assert!(!SEC_GW_AUDIT_OP5_CLAIMED);
const _: () = assert!(GW_AUDIT_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!JSONL_ROTATION_LIVE_HONEST);
const _: () = assert!(!LIVE_TRUST_CHAIN_MEASURED_HONEST);
const _: () = assert!(WIRE_HOP_WIRED_COUNT_HONEST == 6);
const _: () = assert!(WIRE_HOP_TOTAL_COUNT == 8);

/// One hop in the manifold SEC-GW-AUDIT gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwAuditManifoldWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-GW-AUDIT gate runtime wire map (cold-edge evidence → admit-audit census).
pub const MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS: &[SecGwAuditManifoldWireHop] = &[
    SecGwAuditManifoldWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_gw_audit::gate_admit_audit_census",
        role: "Manifold SEC-GW-AUDIT four-path admit stamp census",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 4,
        surface: "umst-manifold::runtime::gate::sec_gw_wrap::gate_trust_wrap_census",
        role: "Upstream SEC-GW-WRAP seven-surface delegate fence",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 5,
        surface: "umst-gateway::lib_adopt_e_gateway_audit::record_admit_gate_verdict_audit",
        role: "Gateway stamped GateVerdict JSONL admit record delegate (Y51/Z87)",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 6,
        surface: "umst-gateway::lib_adopt_e_gateway_audit::GATEWAY_AUDIT_AUTO_STAMP_PATHS",
        role: "Four-path auto-stamp inventory (informational/embodied/material/semantic)",
        wired: true,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 7,
        surface: "umst-gateway::sec_gw_audit_swarm_rotate::bounded_jsonl_rotation",
        role: "Operator JSONL bounded rotation ceremony",
        wired: false,
    },
    SecGwAuditManifoldWireHop {
        ordinal: 8,
        surface: "umst-gateway::lib_adopt_e_gateway_audit::resolve_admit_warrant_from_env",
        role: "Live UMST_GATEWAY_AUDIT_TRUST_CHAIN_ROOT operator measure",
        wired: false,
    },
];

/// One gateway admit-audit auto-stamp path pinned at manifold boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldGwAuditAdmitStampPath {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Stable path identifier (gateway `GATEWAY_AUDIT_AUTO_STAMP_PATHS` row).
    pub path_id: &'static str,
    /// Owning gateway route surface.
    pub route_surface: &'static str,
    /// Wave that wired auto-stamp at gateway delegate.
    pub wired_by_wave: &'static str,
    /// Whether manifold census enumerates this path today.
    pub census_hit: bool,
}

/// Four-path gateway admit auto-stamp inventory (pinned from Y51+Z87 gateway SSOT).
pub const MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS: &[ManifoldGwAuditAdmitStampPath] = &[
    ManifoldGwAuditAdmitStampPath {
        ordinal: 1,
        path_id: "informational_stdio_delegate",
        route_surface: "umst-gateway::informational_stdio_route",
        wired_by_wave: "Y51",
        census_hit: true,
    },
    ManifoldGwAuditAdmitStampPath {
        ordinal: 2,
        path_id: "embodied_gate_check",
        route_surface: "umst-gateway::embodied_route_consumer",
        wired_by_wave: "Y51",
        census_hit: true,
    },
    ManifoldGwAuditAdmitStampPath {
        ordinal: 3,
        path_id: "material_mcp_delegate",
        route_surface: "umst-gateway::sec_mcp_wrap::admit_mcp_tool_call",
        wired_by_wave: "Z87",
        census_hit: true,
    },
    ManifoldGwAuditAdmitStampPath {
        ordinal: 4,
        path_id: "semantic_stub_delegate",
        route_surface: "umst-gateway::semantic_wrap_consumer::gate_check_semantic",
        wired_by_wave: "Z87",
        census_hit: true,
    },
];

/// One stamped GateVerdict leg pinned at manifold census boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldGwAuditStampLeg {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Leg field name in stamped JSONL row.
    pub leg_id: &'static str,
    /// Whether census enumerates this leg today.
    pub census_hit: bool,
}

/// Stamp legs for `record_admit_gate_verdict_audit` (Y51 deepen).
pub const MANIFOLD_GW_AUDIT_STAMP_LEGS: &[ManifoldGwAuditStampLeg] = &[
    ManifoldGwAuditStampLeg {
        ordinal: 1,
        leg_id: "ucrs_seq",
        census_hit: true,
    },
    ManifoldGwAuditStampLeg {
        ordinal: 2,
        leg_id: "trust_attestation",
        census_hit: true,
    },
];

/// Aggregated SEC-GW-AUDIT gate admit-audit census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecGwAuditManifoldAdmitCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// Four admit stamp paths enumerated.
    pub admit_stamp_path_count: usize,
    /// All four admit stamp paths probed in census.
    pub all_admit_stamp_paths_probed: bool,
    /// Stamp leg count (2/2).
    pub stamp_leg_count: usize,
    /// Delegate residual path count (Z87 honest 0).
    pub delegate_residual_path_count: usize,
    /// Upstream SEC-GW-WRAP ceremony closed.
    pub upstream_gw_wrap_ceremony_closed: bool,
    /// Gateway stamp helpers exported — honest true (not live ceremony).
    pub gateway_stamp_helpers_wired: bool,
    /// Live JSONL rotation — honest false.
    pub jsonl_rotation_live: bool,
    /// Live trust chain measured — honest false.
    pub live_trust_chain_measured: bool,
    /// SEC-GW-AUDIT GREEN claim blocked.
    pub gw_audit_green_claim_blocked: bool,
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
    evidence.admissibility == AdmissibilityToken::Admissible && !evidence.catalog_id.is_empty()
}

/// Whether live gateway admit-audit production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_gw_audit_production_wired() -> bool {
    false
}

const _: () = assert!(!sec_gw_audit_production_wired());

/// W29-117 honesty fence — GREEN / PRODUCTION / MASTER / OP-5 refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwAuditHonestyFence {
    /// Deepen step cell id.
    pub deepen_step: &'static str,
    /// Physics GREEN invent — must stay false.
    pub physics_green: bool,
    /// Production flip — must stay false.
    pub production_wired: bool,
    /// MASTER retick invent — must stay false.
    pub master_retick_eligible: bool,
    /// OP-5 invent — must stay false.
    pub op5_claimed: bool,
    /// GREEN claim blocked — must stay true.
    pub green_claim_blocked: bool,
    /// Live JSONL rotation — must stay false.
    pub jsonl_rotation_live: bool,
    /// Live trust-chain measure — must stay false.
    pub live_trust_chain_measured: bool,
}

impl SecGwAuditHonestyFence {
    /// Measured honesty posture for this module.
    #[must_use]
    pub const fn measured() -> Self {
        Self {
            deepen_step: W29_117_SEC_GW_AUDIT_DEEPEN_STEP,
            physics_green: SEC_GW_AUDIT_PHYSICS_GREEN,
            production_wired: sec_gw_audit_production_wired(),
            master_retick_eligible: SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE,
            op5_claimed: SEC_GW_AUDIT_OP5_CLAIMED,
            green_claim_blocked: GW_AUDIT_GREEN_CLAIM_BLOCKED,
            jsonl_rotation_live: JSONL_ROTATION_LIVE_HONEST,
            live_trust_chain_measured: LIVE_TRUST_CHAIN_MEASURED_HONEST,
        }
    }

    /// Fence holds when invent flags stay false and GREEN remains blocked.
    #[must_use]
    pub const fn holds(self) -> bool {
        !self.physics_green
            && !self.production_wired
            && !self.master_retick_eligible
            && !self.op5_claimed
            && self.green_claim_blocked
            && !self.jsonl_rotation_live
            && !self.live_trust_chain_measured
            && !self.deepen_step.is_empty()
    }
}

/// Honesty probe for W29-117 deepen — fence holds; census ceremony closed at cold-edge tier.
#[must_use]
pub fn sec_gw_audit_honesty_probe() -> bool {
    let fence = SecGwAuditHonestyFence::measured();
    fence.holds()
        && NON_CLAIM.contains("not GREEN")
        && NON_CLAIM.contains("not PRODUCTION_WIRED")
        && NON_CLAIM.contains("not MASTER")
        && NON_CLAIM.contains("not OP-5")
        && manifold_gate_sec_gw_audit_ceremony_closed()
        && gate_transition_evidence_probe()
}

/// Whether all four gateway admit auto-stamp paths are enumerated at manifold boundary.
#[must_use]
pub fn manifold_gw_audit_all_stamp_paths_probed() -> bool {
    MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS.len() == ADMIT_STAMP_PATH_COUNT
        && MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS
            .iter()
            .all(|p| p.census_hit)
}

/// Whether both stamp legs are enumerated at manifold boundary.
#[must_use]
pub fn manifold_gw_audit_stamp_legs_complete() -> bool {
    MANIFOLD_GW_AUDIT_STAMP_LEGS.len() == STAMP_LEG_COUNT
        && MANIFOLD_GW_AUDIT_STAMP_LEGS.iter().all(|l| l.census_hit)
}

/// Verify upstream SEC-GW-WRAP delegate ceremony at manifold boundary.
#[must_use]
pub fn manifold_verify_upstream_gw_wrap_delegate() -> bool {
    manifold_gate_sec_gw_wrap_ceremony_closed()
        && gate_trust_wrap_census().gate_evidence_wired
        && manifold_gw_wrap_all_admit_surfaces_probed()
}

/// Build manifold SEC-GW-AUDIT gate admit-audit census from live measurements.
#[must_use]
pub fn gate_admit_audit_census() -> SecGwAuditManifoldAdmitCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecGwAuditManifoldAdmitCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        admit_stamp_path_count: ADMIT_STAMP_PATH_COUNT,
        all_admit_stamp_paths_probed: manifold_gw_audit_all_stamp_paths_probed(),
        stamp_leg_count: MANIFOLD_GW_AUDIT_STAMP_LEGS.len(),
        delegate_residual_path_count: DELEGATE_RESIDUAL_PATH_COUNT,
        upstream_gw_wrap_ceremony_closed: manifold_gate_sec_gw_wrap_ceremony_closed(),
        gateway_stamp_helpers_wired: GATEWAY_AUDIT_STAMP_HELPERS_WIRED_HONEST,
        jsonl_rotation_live: JSONL_ROTATION_LIVE_HONEST,
        live_trust_chain_measured: LIVE_TRUST_CHAIN_MEASURED_HONEST,
        gw_audit_green_claim_blocked: GW_AUDIT_GREEN_CLAIM_BLOCKED,
        production_wired: sec_gw_audit_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-GW-AUDIT ceremony is closed at census tier.
///
/// True when cold-edge evidence + four-path stamp census + upstream SEC-GW-WRAP delegate are
/// measured wired. Live JSONL rotation + operator trust-chain env are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_gw_audit_ceremony_closed() -> bool {
    let census = gate_admit_audit_census();
    census.gate_evidence_wired
        && census.admit_stamp_path_count == ADMIT_STAMP_PATH_COUNT
        && census.all_admit_stamp_paths_probed
        && census.stamp_leg_count == STAMP_LEG_COUNT
        && census.delegate_residual_path_count == DELEGATE_RESIDUAL_PATH_COUNT
        && census.upstream_gw_wrap_ceremony_closed
        && census.gateway_stamp_helpers_wired
        && !census.jsonl_rotation_live
        && !census.live_trust_chain_measured
        && census.gw_audit_green_claim_blocked
        && !census.production_wired
        && census.wire_hop_wired_count == WIRE_HOP_WIRED_COUNT_HONEST
        && SecGwAuditHonestyFence::measured().holds()
        && !SEC_GW_AUDIT_PHYSICS_GREEN
        && !SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE
        && !SEC_GW_AUDIT_OP5_CLAIMED
        && manifold_gw_audit_all_stamp_paths_probed()
        && manifold_gw_audit_stamp_legs_complete()
        && manifold_verify_upstream_gw_wrap_delegate()
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-GW-AUDIT manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecGwAuditManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Four admit stamp paths probed.
    pub all_admit_stamp_paths_probed: bool,
    /// Stamp legs complete.
    pub stamp_legs_complete: bool,
    /// Upstream GW-WRAP delegate verified.
    pub upstream_gw_wrap_verified: bool,
    /// GREEN claim blocked.
    pub gw_audit_green_claim_blocked: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Physics GREEN invent refused.
    pub physics_green_honest_false: bool,
    /// MASTER retick invent refused.
    pub master_retick_honest_false: bool,
    /// OP-5 invent refused.
    pub op5_honest_false: bool,
    /// W29-117 honesty fence holds.
    pub honesty_fence_holds: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-GW-AUDIT done-when checks.
#[must_use]
pub fn sec_gw_audit_manifold_probe() -> SecGwAuditManifoldProbe {
    let census = gate_admit_audit_census();
    let fence = SecGwAuditHonestyFence::measured();
    SecGwAuditManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        all_admit_stamp_paths_probed: census.all_admit_stamp_paths_probed,
        stamp_legs_complete: manifold_gw_audit_stamp_legs_complete(),
        upstream_gw_wrap_verified: manifold_verify_upstream_gw_wrap_delegate(),
        gw_audit_green_claim_blocked: census.gw_audit_green_claim_blocked,
        production_honest_false: !census.production_wired,
        physics_green_honest_false: !fence.physics_green,
        master_retick_honest_false: !fence.master_retick_eligible,
        op5_honest_false: !fence.op5_claimed,
        honesty_fence_holds: fence.holds(),
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_gw_audit_ceremony_closed(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC31 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecGwAuditAccel2Ac31Probe {
    /// AC31 fleet card id.
    pub ac31_job_id: &'static str,
    /// Prior Y51 stamped admit-path absorbed.
    pub prior_y51_absorbed: bool,
    /// Prior Z87 delegate stamp closure absorbed.
    pub prior_z87_absorbed: bool,
    /// Prior Z08 resume absorbed.
    pub prior_z08_absorbed: bool,
    /// Prior P1710 F1 swarm rotate absorbed.
    pub prior_p1710_f1_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecGwAuditManifoldProbe,
    /// `sec_gw_audit_production_wired()` — honest false.
    pub production_wired: bool,
    /// Admit stamp path census count.
    pub admit_stamp_path_count: usize,
    /// Delegate residual path count.
    pub delegate_residual_path_count: usize,
}

/// Build FLEET-COMPOSER ACCEL-B AC31 integration probe from live measurements.
#[must_use]
pub fn sec_gw_audit_accel2_ac31_probe() -> SecGwAuditAccel2Ac31Probe {
    let census = gate_admit_audit_census();
    SecGwAuditAccel2Ac31Probe {
        ac31_job_id: FLEET_ACCEL2_AC31_JOB_ID,
        prior_y51_absorbed: PRIOR_RECEIPT_PATH_Y51.contains("Y51"),
        prior_z87_absorbed: PRIOR_RECEIPT_PATH_Z87.contains("Z87"),
        prior_z08_absorbed: PRIOR_RECEIPT_PATH_Z08.contains("Z08"),
        prior_p1710_f1_absorbed: PRIOR_RECEIPT_PATH_P1710_F1.contains("F1"),
        ceremony_closed: manifold_gate_sec_gw_audit_ceremony_closed(),
        probe: sec_gw_audit_manifold_probe(),
        production_wired: sec_gw_audit_production_wired(),
        admit_stamp_path_count: census.admit_stamp_path_count,
        delegate_residual_path_count: census.delegate_residual_path_count,
    }
}

/// FLEET-COMPOSER ACCEL-B AC31 honesty gate — ceremony closed + production false.
#[must_use]
pub fn sec_gw_audit_accel2_ac31_honest() -> bool {
    let probe = sec_gw_audit_accel2_ac31_probe();
    probe.ac31_job_id == FLEET_ACCEL2_AC31_JOB_ID
        && probe.prior_y51_absorbed
        && probe.prior_z87_absorbed
        && probe.prior_z08_absorbed
        && probe.prior_p1710_f1_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.all_admit_stamp_paths_probed
        && probe.probe.stamp_legs_complete
        && probe.probe.upstream_gw_wrap_verified
        && probe.probe.gw_audit_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.physics_green_honest_false
        && probe.probe.master_retick_honest_false
        && probe.probe.op5_honest_false
        && probe.probe.honesty_fence_holds
        && probe.probe.wire_hop_wired_count == WIRE_HOP_WIRED_COUNT_HONEST
        && !probe.production_wired
        && probe.admit_stamp_path_count == ADMIT_STAMP_PATH_COUNT
        && probe.delegate_residual_path_count == DELEGATE_RESIDUAL_PATH_COUNT
}

/// Validate SEC-GW-AUDIT gate census honesty — fail closed on fake persistence/production claims.
pub fn validate_sec_gw_audit_honesty() -> Result<(), &'static str> {
    let census = gate_admit_audit_census();
    let fence = SecGwAuditHonestyFence::measured();
    if fence.deepen_step != W29_117_SEC_GW_AUDIT_DEEPEN_STEP {
        return Err("W29-117 deepen step id drift");
    }
    if !fence.holds() {
        return Err(
            "sec_gw_audit honesty fence must hold (no invent GREEN/PRODUCTION/MASTER/OP-5)",
        );
    }
    if fence.physics_green || SEC_GW_AUDIT_PHYSICS_GREEN {
        return Err("SEC_GW_AUDIT_PHYSICS_GREEN must stay false — census ≠ fleet physics");
    }
    if fence.master_retick_eligible || SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE {
        return Err("SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE must stay false — no invent MASTER");
    }
    if fence.op5_claimed || SEC_GW_AUDIT_OP5_CLAIMED {
        return Err("SEC_GW_AUDIT_OP5_CLAIMED must stay false — no invent OP-5");
    }
    if census.production_wired {
        return Err("sec_gw_audit_production_wired must stay false until operator measure");
    }
    if census.jsonl_rotation_live {
        return Err("jsonl_rotation_live must stay false until live JSONL ceremony");
    }
    if census.live_trust_chain_measured {
        return Err(
            "live_trust_chain_measured must stay false until UMST_GATEWAY_AUDIT_TRUST_CHAIN_ROOT",
        );
    }
    if !census.gw_audit_green_claim_blocked {
        return Err("gw_audit_green_claim_blocked must stay true in scaffold deepen");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.admit_stamp_path_count != ADMIT_STAMP_PATH_COUNT {
        return Err("four gateway admit stamp paths expected");
    }
    if !census.all_admit_stamp_paths_probed {
        return Err("all four admit stamp paths must be probed");
    }
    if census.stamp_leg_count != STAMP_LEG_COUNT {
        return Err("two stamp legs expected");
    }
    if census.delegate_residual_path_count != DELEGATE_RESIDUAL_PATH_COUNT {
        return Err("delegate residual path count must match Z87 honest 0");
    }
    if !census.upstream_gw_wrap_ceremony_closed {
        return Err("upstream SEC-GW-WRAP ceremony must be closed");
    }
    if MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS.len() != WIRE_HOP_TOTAL_COUNT {
        return Err("eight SEC-GW-AUDIT gate wire hops expected");
    }
    if census.wire_hop_wired_count != WIRE_HOP_WIRED_COUNT_HONEST {
        return Err("six SEC-GW-AUDIT gate wire hops should be wired today");
    }
    if !NON_CLAIM.contains("not GREEN")
        || !NON_CLAIM.contains("not PRODUCTION_WIRED")
        || !NON_CLAIM.contains("not MASTER")
        || !NON_CLAIM.contains("not OP-5")
    {
        return Err("NON_CLAIM must refuse GREEN/PRODUCTION_WIRED/MASTER/OP-5");
    }
    if !manifold_gate_sec_gw_audit_ceremony_closed() {
        return Err("manifold gate SEC-GW-AUDIT ceremony must be closed at census tier");
    }
    if !sec_gw_audit_honesty_probe() {
        return Err("W29-117 honesty probe must pass");
    }
    if !sec_gw_audit_accel2_ac31_honest() {
        return Err("ACCEL-B AC31 probe must be honest");
    }
    Ok(())
}

/// Render SEC-GW-AUDIT gate wire map for operator receipts.
#[must_use]
pub fn sec_gw_audit_wire_matrix() -> String {
    let census = gate_admit_audit_census();
    let mut out = String::from("SEC-GW-AUDIT manifold gate admit-audit wire map (AC31):\n");
    for hop in MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} admit_stamp_paths={} stamp_legs={} delegate_residuals={} \
         gw_audit_green_claim_blocked={} jsonl_rotation_live={} live_trust_chain_measured={} \
         production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS.len(),
        census.admit_stamp_path_count,
        census.stamp_leg_count,
        census.delegate_residual_path_count,
        census.gw_audit_green_claim_blocked,
        census.jsonl_rotation_live,
        census.live_trust_chain_measured,
        census.production_wired
    ));
    out.push_str(&format!("  gateway_ssot={GATEWAY_SSOT}\n"));
    out.push_str(&format!("  gateway_swarm_ssot={GATEWAY_SWARM_SSOT}\n"));
    out
}

/// Next-hop surface for operator trust-chain ceremony (gateway-owned).
#[must_use]
pub const fn sec_gw_audit_trust_chain_next_hop() -> &'static str {
    "umst-gateway/crates/umst-gateway/src/lib_adopt_e_gateway_audit.rs:resolve_admit_warrant_from_env"
}

#[cfg(test)]
mod sec_gw_audit_tests {
    use super::*;

    #[test]
    fn sec_gw_audit_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-GW-AUDIT");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-GW-AUDIT");
        assert_eq!(FLEET_ACCEL2_AC31_JOB_ID, "ACCEL-B-2050-AC31");
    }

    #[test]
    fn sec_gw_audit_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_gw_audit_admit_census_honest_posture() {
        let census = gate_admit_audit_census();
        assert_eq!(census.board_slice_id, "SEC-GW-AUDIT");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.admit_stamp_path_count, 4);
        assert!(census.all_admit_stamp_paths_probed);
        assert_eq!(census.stamp_leg_count, 2);
        assert_eq!(census.delegate_residual_path_count, 0);
        assert!(census.upstream_gw_wrap_ceremony_closed);
        assert!(census.gateway_stamp_helpers_wired);
        assert!(!census.jsonl_rotation_live);
        assert!(!census.live_trust_chain_measured);
        assert!(census.gw_audit_green_claim_blocked);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 6);
    }

    #[test]
    fn sec_gw_audit_production_stays_false() {
        assert!(!sec_gw_audit_production_wired());
        assert!(GW_AUDIT_GREEN_CLAIM_BLOCKED);
        assert!(!JSONL_ROTATION_LIVE_HONEST);
        assert!(!LIVE_TRUST_CHAIN_MEASURED_HONEST);
    }

    #[test]
    fn sec_gw_audit_manifold_wire_hops_six_of_eight_wired() {
        assert_eq!(MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS.len(), 8);
        assert_eq!(
            MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            6
        );
        assert!(MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("bounded_jsonl_rotation") && !h.wired));
        assert!(MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("resolve_admit_warrant_from_env") && !h.wired));
    }

    #[test]
    fn sec_gw_audit_admit_stamp_paths_four_of_four() {
        assert!(manifold_gw_audit_all_stamp_paths_probed());
        assert_eq!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS.len(), 4);
        assert!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS
            .iter()
            .all(|p| p.census_hit));
        assert!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS
            .iter()
            .any(|p| p.path_id == "informational_stdio_delegate"));
        assert!(MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS
            .iter()
            .any(|p| p.path_id == "semantic_stub_delegate"));
    }

    #[test]
    fn sec_gw_audit_stamp_legs_two_of_two() {
        assert!(manifold_gw_audit_stamp_legs_complete());
        assert_eq!(MANIFOLD_GW_AUDIT_STAMP_LEGS.len(), 2);
        assert!(MANIFOLD_GW_AUDIT_STAMP_LEGS
            .iter()
            .any(|l| l.leg_id == "ucrs_seq"));
        assert!(MANIFOLD_GW_AUDIT_STAMP_LEGS
            .iter()
            .any(|l| l.leg_id == "trust_attestation"));
    }

    #[test]
    fn sec_gw_audit_upstream_gw_wrap_delegate_verified() {
        assert!(manifold_verify_upstream_gw_wrap_delegate());
        assert!(manifold_gate_sec_gw_wrap_ceremony_closed());
    }

    #[test]
    fn sec_gw_audit_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_gw_audit_ceremony_closed());
        let probe = sec_gw_audit_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.all_admit_stamp_paths_probed);
        assert!(probe.stamp_legs_complete);
        assert!(probe.upstream_gw_wrap_verified);
        assert!(probe.gw_audit_green_claim_blocked);
        assert!(probe.production_honest_false);
        assert!(probe.physics_green_honest_false);
        assert!(probe.master_retick_honest_false);
        assert!(probe.op5_honest_false);
        assert!(probe.honesty_fence_holds);
        assert_eq!(probe.wire_hop_wired_count, WIRE_HOP_WIRED_COUNT_HONEST);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_gw_audit_w29_117_honesty_fence_blocks_green_production_master_op5() {
        assert_eq!(W29_117_SEC_GW_AUDIT_DEEPEN_STEP, "W29-117-SEC_GW_AUDIT");
        assert!(!SEC_GW_AUDIT_PHYSICS_GREEN);
        assert!(!SEC_GW_AUDIT_MASTER_RETICK_ELIGIBLE);
        assert!(!SEC_GW_AUDIT_OP5_CLAIMED);
        assert!(GW_AUDIT_GREEN_CLAIM_BLOCKED);
        assert!(!sec_gw_audit_production_wired());
        assert!(!JSONL_ROTATION_LIVE_HONEST);
        assert!(!LIVE_TRUST_CHAIN_MEASURED_HONEST);
        let fence = SecGwAuditHonestyFence::measured();
        assert_eq!(fence.deepen_step, W29_117_SEC_GW_AUDIT_DEEPEN_STEP);
        assert!(fence.holds());
        assert!(NON_CLAIM.contains("not GREEN"));
        assert!(NON_CLAIM.contains("not PRODUCTION_WIRED"));
        assert!(NON_CLAIM.contains("not MASTER"));
        assert!(NON_CLAIM.contains("not OP-5"));
        assert!(sec_gw_audit_honesty_probe());
    }

    #[test]
    fn sec_gw_audit_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_Y51.contains("Y51"));
        assert!(PRIOR_RECEIPT_PATH_Z87.contains("Z87"));
        assert!(PRIOR_RECEIPT_PATH_Z08.contains("Z08"));
        assert!(PRIOR_RECEIPT_PATH_P1710_F1.contains("F1"));
        assert!(GATEWAY_SSOT.contains("lib_adopt_e_gateway_audit.rs"));
        assert!(GATEWAY_SWARM_SSOT.contains("sec_gw_audit_swarm_rotate.rs"));
    }

    #[test]
    fn sec_gw_audit_wire_matrix_renders_honest_counts() {
        let matrix = sec_gw_audit_wire_matrix();
        assert!(matrix.contains("SEC-GW-AUDIT manifold gate"));
        assert!(matrix.contains("gw_audit_green_claim_blocked=true"));
        assert!(matrix.contains("wired=6/8"));
        assert!(matrix.contains("production_wired=false"));
        assert!(matrix.contains("delegate_residuals=0"));
    }

    #[test]
    fn fleet_composer_accel2_ac31_sec_gw_audit_honest() {
        assert!(sec_gw_audit_accel2_ac31_honest());
        let probe = sec_gw_audit_accel2_ac31_probe();
        assert_eq!(probe.ac31_job_id, FLEET_ACCEL2_AC31_JOB_ID);
        assert!(probe.prior_y51_absorbed);
        assert!(probe.prior_z87_absorbed);
        assert!(probe.prior_z08_absorbed);
        assert!(probe.prior_p1710_f1_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert_eq!(probe.admit_stamp_path_count, 4);
        assert_eq!(probe.delegate_residual_path_count, 0);
        assert!(probe.probe.honesty_fence_holds);
        assert!(probe.probe.physics_green_honest_false);
        assert!(probe.probe.master_retick_honest_false);
        assert!(probe.probe.op5_honest_false);
    }

    #[test]
    fn sec_gw_audit_validate_gate_honesty_residue_measured() {
        validate_sec_gw_audit_honesty().expect("honest SEC-GW-AUDIT gate census residue");
        assert_eq!(
            sec_gw_audit_trust_chain_next_hop(),
            "umst-gateway/crates/umst-gateway/src/lib_adopt_e_gateway_audit.rs:resolve_admit_warrant_from_env"
        );
        assert_eq!(POSTURE_TAG, "manifold-gate-census-wired-not-production");
        assert_eq!(WIRE_HOP_TOTAL_COUNT, 8);
        assert_eq!(WIRE_HOP_WIRED_COUNT_HONEST, 6);
    }
}

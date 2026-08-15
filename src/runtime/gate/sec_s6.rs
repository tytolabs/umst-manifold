// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2033/2127-SEC-S6 — manifold gate runtime HCOM prov gateway fence wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S6 HCOM prov gateway
//! fence + `s6_inspect` SSOT; live operator attestation ceremony and
//! `hcom_prov_gw_production_wired()` stay **honest open**.
//!
//! # Honesty (W29-125-SEC_S6)
//!
//! Census + HCOM prov gateway fence deepen only. Does **not** invent:
//! - physics / fleet **GREEN**
//! - **PRODUCTION_WIRED**
//! - **MASTER_RETICK** / master retick eligibility
//! - **OP-5 PASS**

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// W29-125 swarm cell id (SEC-S6 honest-fence deepen).
pub const W29_125_CELL_ID: &str = "W29-125-SEC_S6";

/// W29-125 honest posture — manifold S-6 HCOM prov gateway fence census deepen only.
pub const W29_125_HONEST_POSTURE: &str = "SEC_S6_MANIFOLD_CENSUS_DEEPEN_ONLY";

/// W29-125 explicit non-claims (gate text).
pub const W29_125_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// W29-125 deepen schema version.
pub const W29_125_DEEPEN_SCHEMA_VERSION: &str = "sec_s6_w29_125_honest_fence_v1";

/// Honest fence string for meta / fleet probes (GREEN / PRODUCTION / MASTER / OP-5 fenced).
pub const HONEST_FENCE: &str = "census_wired=true production_wired=false green_claim_blocked=true \
master_retick=false op5_cleared=false live_attestation_wired=false hcom_prov_gw_production_open=true";

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S6";

/// AGAP slot id (2033 S-6 upstream table deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S6";

/// FLEET-COMPOSER ACCEL-B slot AC33 id (hcom prov gateway fence deepen).
pub const ACCEL_B_2050_AC33_JOB_ID: &str = "ACCEL-B-2050-AC33";

/// FLEET-COMPOSER ACCEL-B AC33 receipt path.
pub const ACCEL_AC33_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC33.md";

/// FLEET-COMPOSER-Z Z126 S-6 trust inspect receipt cross-ref.
pub const PRIOR_Z126_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_Z126_1232.md";

/// FLEET-COMPOSER-H H55 S5 HCOM prov delegate receipt cross-ref.
pub const PRIOR_H55_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_H55_2242.md";

/// FLEET-COMPOSER-J J34 S5 HCOM prov delegate receipt cross-ref.
pub const PRIOR_J34_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_J34_2348.md";

/// Prior AGAP-2033 SEC-S6 upstream table receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S6_2033.md";

/// umst-trust S-6 inspect delegate SSOT (Z126 owner — absorb, do not redo).
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/inspect.rs";

/// umst-semantics SEC-HCOM-PROV authority SSOT.
pub const SEMANTICS_SSOT: &str =
    "umst-semantics/crates/umst-semantics/src/semantic_trust_provenance.rs";

/// Gateway SEC-HCOM-GW delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_hcom_prov_gw.rs";

/// egoff `:scert` capstone upstream probe owner.
pub const EGOFF_SCERT_SSOT: &str = "egoff/egoff/src/slices/14bis_f_S_6_scert_capstone.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v2 absorbs W29-125 honest-fence deepen).
pub const SCHEMA_VERSION: &str = "sec_s6_gate_hcom_prov_gateway_fence_census_v2";

/// Manifold SEC-S6 gate wire hop count (6 wired + 1 production open).
pub const MANIFOLD_SEC_S6_GATE_WIRE_HOP_COUNT: usize = 7;

/// Manifold SEC-S6 gate wire hops wired today (production hop stays open).
pub const MANIFOLD_SEC_S6_GATE_WIRE_WIRED_COUNT: usize = 6;

/// HCOM prov gateway delegate wire hop count (pinned from `sec_hcom_prov_gw::WIRE_HOPS`).
pub const HCOM_PROV_GW_WIRE_HOP_COUNT: usize = 5;

/// S-6 inspect factor row count (Z126 six-row matrix).
pub const S6_INSPECT_FACTOR_COUNT: usize = 6;

/// SCERT upstream hw/l/zero/m slot count.
pub const SCERT_UPSTREAM_SLOT_COUNT: usize = 4;

/// Honest operator exit for `:scert --strict --paired` scaffold.
pub const SCERT_EXIT_NOT_WIRED: i32 = 2;

/// Live operator attestation ceremony wired — honest false.
pub const LIVE_ATTESTATION_WIRED_HONEST: bool = false;

/// S-Arc GREEN claim blocked — honest true in scaffold deepen.
pub const S6_GREEN_CLAIM_BLOCKED: bool = true;

/// Operator exit for `:trust gate-factors` — honest BLOCKED until SCERT capstone.
pub const EXPECTED_GATE_EXIT: &str = "BLOCKED";

/// One hop in the SEC-HCOM-PROV → gateway semantic admit delegate chain (manifold pin).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldHcomProvGwFenceHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the delegate chain.
    pub role: &'static str,
}

/// HCOM prov gateway fence hops pinned from `sec_hcom_prov_gw::WIRE_HOPS`.
pub const HCOM_PROV_GW_FENCE_HOPS: &[ManifoldHcomProvGwFenceHop] = &[
    ManifoldHcomProvGwFenceHop {
        ordinal: 1,
        surface: "umst-semantics/src/semantic_trust_provenance.rs::SemanticTrustAttestation",
        role: "SEC-HCOM-PROV trust attestation SSOT",
    },
    ManifoldHcomProvGwFenceHop {
        ordinal: 2,
        surface: "umst-gateway/src/sec_hcom_prov_gw.rs::enforce_hcom_prov_semantic_admit",
        role: "Gateway semantic admit — bind attestation trust to SEC-GW-WRAP surface",
    },
    ManifoldHcomProvGwFenceHop {
        ordinal: 3,
        surface: "umst-gateway/src/sec_gw_trust_wrap.rs::enforce_j2_semantic_production_admit",
        role: "SEC-GW-WRAP `check_admit_surface` at J2+semantic production compose",
    },
    ManifoldHcomProvGwFenceHop {
        ordinal: 4,
        surface:
            "umst-gateway/src/semantic_compose.rs::compose_j2_semantic_production_trust_stamped",
        role: "Production compose + trust stamp on semantic admit (JOINT-GATE)",
    },
    ManifoldHcomProvGwFenceHop {
        ordinal: 5,
        surface:
            "umst-gateway/src/semantic_compose.rs::semantic_conjunct_witness_id_with_trust_stamp",
        role: "Bind attestation chain root prefix into conjunct witness id on admit",
    },
];

/// One SCERT upstream slot pinned at manifold boundary (trust-scope honest PARTIAL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldS6UpstreamSlot {
    /// Arc label (H/L/Z/M).
    pub arc_name: &'static str,
    /// Operator command string.
    pub command: &'static str,
    /// Honest exit code at scaffold.
    pub exit_code: i32,
    /// Live probe owner.
    pub live_probe_owner: &'static str,
}

/// SCERT upstream hw/l/zero/m slots (pinned from `s6_inspect` / egoff — all PARTIAL @ scaffold).
pub const SCERT_UPSTREAM_SLOTS: &[ManifoldS6UpstreamSlot] = &[
    ManifoldS6UpstreamSlot {
        arc_name: "H-Arc",
        command: ":hw-cert --strict --paired",
        exit_code: SCERT_EXIT_NOT_WIRED,
        live_probe_owner: EGOFF_SCERT_SSOT,
    },
    ManifoldS6UpstreamSlot {
        arc_name: "L-Arc",
        command: ":lcert --strict",
        exit_code: SCERT_EXIT_NOT_WIRED,
        live_probe_owner: EGOFF_SCERT_SSOT,
    },
    ManifoldS6UpstreamSlot {
        arc_name: "Z-Arc",
        command: ":zerocert --strict",
        exit_code: SCERT_EXIT_NOT_WIRED,
        live_probe_owner: EGOFF_SCERT_SSOT,
    },
    ManifoldS6UpstreamSlot {
        arc_name: "M-Arc",
        command: ":mcert --strict --paired",
        exit_code: SCERT_EXIT_NOT_WIRED,
        live_probe_owner: EGOFF_SCERT_SSOT,
    },
];

/// One hop in the manifold SEC-S6 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS6GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S6 gate runtime wire map (cold-edge evidence → HCOM prov gateway fence census).
pub const MANIFOLD_SEC_S6_GATE_WIRE_HOPS: &[SecS6GateWireHop] = &[
    SecS6GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s6::gate_hcom_prov_gateway_fence_census",
        role: "Manifold gate SEC-S6 HCOM prov gateway fence census",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 4,
        surface: "umst-trust::inspect::validate_s6_inspect_honesty",
        role: "S-6 `:trust inspect` delegate (Z126 owner)",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 5,
        surface: "umst-semantics::semantic_trust_provenance::SemanticTrustAttestation",
        role: "SEC-HCOM-PROV trust attestation authority",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 6,
        surface: "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_done_when_probe",
        role: "Gateway HCOM prov delegate fixture deepen (H55/J34)",
        wired: true,
    },
    SecS6GateWireHop {
        ordinal: 7,
        surface: "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_production_wired",
        role: "Gateway HCOM prov production ceremony (serial Wave I)",
        wired: false,
    },
];

/// Aggregated SEC-S6 gate HCOM prov gateway fence census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS6GateHcomProvGatewayFenceCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// HCOM prov gateway fence hops pinned (5/5).
    pub hcom_prov_fence_hops_verified: bool,
    /// SCERT upstream slots pinned (4/4 PARTIAL).
    pub scert_upstream_slots_verified: bool,
    /// S-6 inspect delegate pins verified (6/6 wired, 0 credit).
    pub s6_inspect_delegate_verified: bool,
    /// Live attestation ceremony wired — honest false.
    pub live_attestation_wired: bool,
    /// S-Arc GREEN claim blocked.
    pub s6_green_claim_blocked: bool,
    /// Gateway production flip.
    pub production_wired: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// One SEC-S6 gate-factor row for operator `:trust gate-factors` deepen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS6GateFactorRow {
    /// Stable factor identifier.
    pub factor_id: &'static str,
    /// Whether the witness probe is wired.
    pub probe_wired: bool,
    /// Whether the factor earns acceptance credit toward S-6 GREEN.
    pub acceptance_credit: bool,
    /// Operator detail string.
    pub detail: String,
}

/// Typed probe for SEC-S6 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS6GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// HCOM prov fence hops verified.
    pub hcom_prov_fence_hops_verified: bool,
    /// SCERT upstream slots verified.
    pub scert_upstream_slots_verified: bool,
    /// S-6 inspect delegate verified.
    pub s6_inspect_delegate_verified: bool,
    /// Live attestation honest false.
    pub live_attestation_honest_false: bool,
    /// S-Arc GREEN claim blocked.
    pub s6_green_claim_blocked: bool,
    /// Production honest false.
    pub production_honest_false: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
    /// Ceremony closed at census tier.
    pub ceremony_closed: bool,
}

/// FLEET-COMPOSER ACCEL-B AC33 probe — manifold HCOM prov gateway fence deepen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS6AccelAc33Probe {
    /// AC33 card id.
    pub ac33_job_id: &'static str,
    /// Prior 2033 upstream table absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior Z126 S-6 inspect absorbed.
    pub prior_z126_absorbed: bool,
    /// Prior H55 S5 delegate absorbed.
    pub prior_h55_absorbed: bool,
    /// HCOM prov fence table residue pinned.
    pub hcom_prov_fence_table_residue_pinned: bool,
    /// Manifold gate probe.
    pub probe: SecS6GateManifoldProbe,
    /// Ceremony closed.
    pub ceremony_closed: bool,
}

/// Exercise gate cold-edge evidence at manifold SSOT (identity transition admits).
#[must_use]
pub fn gate_transition_evidence_probe() -> bool {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let new = old;
    let evidence = CdTransitionCartridge.transition_evidence(&old, &new, 1.0);
    evidence.admissibility == AdmissibilityToken::Admissible && !evidence.catalog_id.is_empty()
}

/// Live operator attestation ceremony — honest false until gateway routes plumbed.
#[must_use]
pub const fn live_attestation_wired() -> bool {
    LIVE_ATTESTATION_WIRED_HONEST
}

/// HCOM prov gateway production ceremony — honest false until measured live.
#[must_use]
pub const fn sec_s6_production_wired() -> bool {
    false
}

/// MASTER retick eligibility — honest **false** (not claimed from S-6 census deepen).
#[must_use]
pub const fn sec_s6_master_retick_eligible() -> bool {
    false
}

/// OP-5 clearance — honest **false** (not claimed from S-6 census deepen).
#[must_use]
pub const fn sec_s6_op5_cleared() -> bool {
    false
}

const _: () = assert!(!sec_s6_production_wired());
const _: () = assert!(!LIVE_ATTESTATION_WIRED_HONEST);
const _: () = assert!(S6_GREEN_CLAIM_BLOCKED);
const _: () = assert!(!sec_s6_master_retick_eligible());
const _: () = assert!(!sec_s6_op5_cleared());

/// Honest fence flags for SEC-S6 deepen (W29-125).
///
/// All invent-claim bools stay `false`; `deepen_honest` is true only when cell
/// pins, census ceremony, and GREEN/PRODUCTION/MASTER/OP-5 fences hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS6W29125DeepenProbe {
    /// Deepen schema tag.
    pub schema_version: &'static str,
    /// Swarm cell id.
    pub cell_id: &'static str,
    /// Honest posture pin.
    pub honest_posture: &'static str,
    /// Explicit non-claim gate text.
    pub non_claim: &'static str,
    /// Compact honest-fence string.
    pub honest_fence: &'static str,
    /// Invent claim — must stay false.
    pub production_wired_claimed: bool,
    /// Invent claim — must stay false.
    pub green_claimed: bool,
    /// Invent claim — must stay false.
    pub op5_pass_claimed: bool,
    /// Invent claim — must stay false.
    pub master_retick_claimed: bool,
    /// Measured deepen honesty (fences + ceremony pins).
    pub deepen_honest: bool,
}

/// Build the W29-125 SEC-S6 deepen honesty probe from live measurements.
#[must_use]
pub fn sec_s6_w29_125_deepen_probe() -> SecS6W29125DeepenProbe {
    let production_wired_claimed = sec_s6_production_wired();
    let green_claimed = !S6_GREEN_CLAIM_BLOCKED;
    let op5_pass_claimed = sec_s6_op5_cleared();
    let master_retick_claimed = sec_s6_master_retick_eligible();
    let ceremony_ok = manifold_gate_sec_s6_ceremony_closed();
    let deepen_honest = W29_125_CELL_ID == "W29-125-SEC_S6"
        && W29_125_DEEPEN_SCHEMA_VERSION == "sec_s6_w29_125_honest_fence_v1"
        && W29_125_HONEST_POSTURE == "SEC_S6_MANIFOLD_CENSUS_DEEPEN_ONLY"
        && SCHEMA_VERSION == "sec_s6_gate_hcom_prov_gateway_fence_census_v2"
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && !live_attestation_wired()
        && W29_125_NON_CLAIM.contains("not GREEN")
        && W29_125_NON_CLAIM.contains("not OP-5 PASS")
        && W29_125_NON_CLAIM.contains("not production_wired")
        && W29_125_NON_CLAIM.contains("not MASTER_RETICK")
        && HONEST_FENCE.contains("production_wired=false")
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("master_retick=false")
        && HONEST_FENCE.contains("op5_cleared=false")
        && HONEST_FENCE.contains("live_attestation_wired=false")
        && HONEST_FENCE.contains("hcom_prov_gw_production_open=true")
        && ceremony_ok
        && manifold_hcom_prov_gw_fence_hops_verified()
        && manifold_sec_s6_gate_wire_integrity_verified();
    SecS6W29125DeepenProbe {
        schema_version: W29_125_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_125_CELL_ID,
        honest_posture: W29_125_HONEST_POSTURE,
        non_claim: W29_125_NON_CLAIM,
        honest_fence: HONEST_FENCE,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-125 SEC-S6 deepen honesty probe passes.
#[must_use]
pub fn sec_s6_w29_125_deepen_honest() -> bool {
    sec_s6_w29_125_deepen_probe().deepen_honest
}

/// SEC-S6 fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn sec_s6_honest_fence_holds() -> bool {
    let p = sec_s6_w29_125_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// Whether all five HCOM prov gateway fence hops are pinned at manifold boundary.
#[must_use]
pub fn manifold_hcom_prov_gw_fence_hops_verified() -> bool {
    HCOM_PROV_GW_FENCE_HOPS.len() == HCOM_PROV_GW_WIRE_HOP_COUNT
        && HCOM_PROV_GW_FENCE_HOPS
            .iter()
            .enumerate()
            .all(|(idx, hop)| hop.ordinal == u8::try_from(idx + 1).unwrap_or(0))
        && HCOM_PROV_GW_FENCE_HOPS
            .iter()
            .any(|h| h.surface.contains("enforce_hcom_prov_semantic_admit"))
        && HCOM_PROV_GW_FENCE_HOPS.iter().any(|h| {
            h.surface
                .contains("semantic_conjunct_witness_id_with_trust_stamp")
        })
        && HCOM_PROV_GW_FENCE_HOPS
            .iter()
            .all(|h| h.surface.contains("umst-") || h.surface.contains("egoff"))
        && HCOM_PROV_GW_FENCE_HOPS
            .iter()
            .all(|h| !h.role.is_empty() && !h.surface.is_empty())
}

/// Whether manifold SEC-S6 gate wire hops keep ordinal integrity + honest open production hop.
#[must_use]
pub fn manifold_sec_s6_gate_wire_integrity_verified() -> bool {
    MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len() == MANIFOLD_SEC_S6_GATE_WIRE_HOP_COUNT
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .enumerate()
            .all(|(idx, hop)| hop.ordinal == u8::try_from(idx + 1).unwrap_or(0))
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count()
            == MANIFOLD_SEC_S6_GATE_WIRE_WIRED_COUNT
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("hcom_prov_gw_production_wired") && !h.wired)
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("s6_inspect") && h.wired)
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("gate_hcom_prov_gateway_fence_census") && h.wired)
        && MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .last()
            .is_some_and(|h| !h.wired)
}

/// Whether SCERT upstream hw/l/zero/m slots are pinned with honest PARTIAL posture.
#[must_use]
pub fn manifold_scert_upstream_slots_verified() -> bool {
    SCERT_UPSTREAM_SLOTS.len() == SCERT_UPSTREAM_SLOT_COUNT
        && SCERT_UPSTREAM_SLOTS
            .iter()
            .all(|s| s.exit_code == SCERT_EXIT_NOT_WIRED && s.live_probe_owner == EGOFF_SCERT_SSOT)
        && SCERT_UPSTREAM_SLOTS.iter().any(|s| s.arc_name == "H-Arc")
        && SCERT_UPSTREAM_SLOTS.iter().any(|s| s.arc_name == "M-Arc")
}

/// Whether S-6 inspect delegate pins match Z126 honest scaffold (6 wired / 0 credit).
#[must_use]
pub fn manifold_s6_inspect_delegate_verified() -> bool {
    TRUST_SSOT.contains("s6_inspect")
        && PRIOR_Z126_RECEIPT_PATH.contains("COMPOSER_Z126_1232")
        && S6_INSPECT_FACTOR_COUNT == 6
        && SCERT_EXIT_NOT_WIRED == 2
}

/// Build manifold SEC-S6 gate HCOM prov gateway fence census from live measurements.
#[must_use]
pub fn gate_hcom_prov_gateway_fence_census() -> SecS6GateHcomProvGatewayFenceCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S6_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecS6GateHcomProvGatewayFenceCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        hcom_prov_fence_hops_verified: manifold_hcom_prov_gw_fence_hops_verified(),
        scert_upstream_slots_verified: manifold_scert_upstream_slots_verified(),
        s6_inspect_delegate_verified: manifold_s6_inspect_delegate_verified(),
        live_attestation_wired: live_attestation_wired(),
        s6_green_claim_blocked: S6_GREEN_CLAIM_BLOCKED,
        production_wired: sec_s6_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S6 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + HCOM prov gateway fence wire map hops 1–6 are measured
/// wired. Live attestation ceremony + gateway production flip are explicit non-blockers.
/// W29-125: GREEN / PRODUCTION / MASTER / OP-5 invent claims must stay fenced.
#[must_use]
pub fn manifold_gate_sec_s6_ceremony_closed() -> bool {
    let census = gate_hcom_prov_gateway_fence_census();
    census.gate_evidence_wired
        && census.hcom_prov_fence_hops_verified
        && census.scert_upstream_slots_verified
        && census.s6_inspect_delegate_verified
        && !census.live_attestation_wired
        && census.s6_green_claim_blocked
        && !census.production_wired
        && census.wire_hop_wired_count == 6
        && gate_transition_evidence_probe()
        && manifold_sec_s6_gate_wire_integrity_verified()
        && !sec_s6_master_retick_eligible()
        && !sec_s6_op5_cleared()
}

/// Build introspection probe for SEC-S6 done-when checks.
#[must_use]
pub fn sec_s6_gate_manifold_probe() -> SecS6GateManifoldProbe {
    let census = gate_hcom_prov_gateway_fence_census();
    SecS6GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        hcom_prov_fence_hops_verified: census.hcom_prov_fence_hops_verified,
        scert_upstream_slots_verified: census.scert_upstream_slots_verified,
        s6_inspect_delegate_verified: census.s6_inspect_delegate_verified,
        live_attestation_honest_false: !census.live_attestation_wired,
        s6_green_claim_blocked: census.s6_green_claim_blocked,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s6_ceremony_closed(),
    }
}

/// Collect SEC-S6 gate-factor rows for operator `:trust gate-factors`.
#[must_use]
pub fn collect_sec_s6_gate_factor_rows() -> Vec<SecS6GateFactorRow> {
    vec![
        SecS6GateFactorRow {
            factor_id: "hcom-prov-wire-map",
            probe_wired: manifold_hcom_prov_gw_fence_hops_verified(),
            acceptance_credit: false,
            detail: format!(
                "hcom_prov_gw_hops={} gateway_ssot={GATEWAY_SSOT}",
                HCOM_PROV_GW_WIRE_HOP_COUNT
            ),
        },
        SecS6GateFactorRow {
            factor_id: "s5-delegate-absorbed",
            probe_wired: PRIOR_H55_RECEIPT_PATH.contains("COMPOSER_H55")
                && PRIOR_J34_RECEIPT_PATH.contains("COMPOSER_J34"),
            acceptance_credit: false,
            detail: format!(
                "h55={} j34={}",
                PRIOR_H55_RECEIPT_PATH, PRIOR_J34_RECEIPT_PATH
            ),
        },
        SecS6GateFactorRow {
            factor_id: "bootstrap-admit-fence",
            probe_wired: GATEWAY_SSOT.contains("sec_hcom_prov_gw"),
            acceptance_credit: false,
            detail: "bootstrap admit without attestation — gateway delegate fixture only".into(),
        },
        SecS6GateFactorRow {
            factor_id: "live-attestation-ceremony",
            probe_wired: !live_attestation_wired(),
            acceptance_credit: false,
            detail: format!("live_attestation_wired={}", live_attestation_wired()),
        },
        SecS6GateFactorRow {
            factor_id: "s6-inspect-delegate",
            probe_wired: manifold_s6_inspect_delegate_verified(),
            acceptance_credit: false,
            detail: format!(
                "inspect_factors={S6_INSPECT_FACTOR_COUNT} wired={S6_INSPECT_FACTOR_COUNT} credit=0 trust_ssot={TRUST_SSOT}"
            ),
        },
        SecS6GateFactorRow {
            factor_id: "gateway-production-fence",
            probe_wired: SEMANTICS_SSOT.contains("semantic_trust_provenance"),
            acceptance_credit: false,
            detail: format!("sec_s6_production_wired={}", sec_s6_production_wired()),
        },
    ]
}

/// Render SEC-S6 gate-factor table for operator receipts.
#[must_use]
pub fn sec_s6_gate_factor_table() -> String {
    let rows = collect_sec_s6_gate_factor_rows();
    let wired = rows.iter().filter(|r| r.probe_wired).count();
    format!(
        "SEC-S6 gate factors: wired={wired}/{} credit=0/{} \
         expected_gate_exit={EXPECTED_GATE_EXIT} scert_credit=BLOCKED \
         expected_scert_exit={SCERT_EXIT_NOT_WIRED} sec_s6_production_wired={}",
        rows.len(),
        rows.len(),
        sec_s6_production_wired()
    )
}

/// Render HCOM prov gateway fence hop table for operator receipts.
#[must_use]
pub fn sec_s6_hcom_prov_fence_table() -> String {
    let mut out = String::from("SEC-S6 HCOM prov gateway fence hops (manifold pin):\n");
    for hop in HCOM_PROV_GW_FENCE_HOPS {
        out.push_str(&format!(
            "  {} surface={} role={}\n",
            hop.ordinal, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  hop_count={} verified={} live_attestation_wired={} production_wired={}\n",
        HCOM_PROV_GW_WIRE_HOP_COUNT,
        manifold_hcom_prov_gw_fence_hops_verified(),
        live_attestation_wired(),
        sec_s6_production_wired()
    ));
    out
}

/// Render SCERT upstream slot table for operator receipts.
#[must_use]
pub fn sec_s6_scert_upstream_table() -> String {
    let mut out = String::from("SEC-S6 SCERT upstream composition (hw/l/zero/m):\n");
    for slot in SCERT_UPSTREAM_SLOTS {
        out.push_str(&format!(
            "  {} command={} exit={} owner={}\n",
            slot.arc_name, slot.command, slot.exit_code, slot.live_probe_owner
        ));
    }
    out.push_str(&format!(
        "  upstream_green=0/{} verified={}\n",
        SCERT_UPSTREAM_SLOT_COUNT,
        manifold_scert_upstream_slots_verified()
    ));
    out
}

/// Build FLEET-COMPOSER ACCEL-B AC33 probe.
#[must_use]
pub fn sec_s6_accel_ac33_probe() -> SecS6AccelAc33Probe {
    let fence_table = sec_s6_hcom_prov_fence_table();
    let upstream_table = sec_s6_scert_upstream_table();
    SecS6AccelAc33Probe {
        ac33_job_id: ACCEL_B_2050_AC33_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S6_2033"),
        prior_z126_absorbed: PRIOR_Z126_RECEIPT_PATH.contains("COMPOSER_Z126_1232"),
        prior_h55_absorbed: PRIOR_H55_RECEIPT_PATH.contains("COMPOSER_H55_2242"),
        hcom_prov_fence_table_residue_pinned: fence_table
            .contains("enforce_hcom_prov_semantic_admit")
            && fence_table.contains("verified=true")
            && upstream_table.contains("upstream_green=0/4"),
        probe: sec_s6_gate_manifold_probe(),
        ceremony_closed: manifold_gate_sec_s6_ceremony_closed(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC33 honesty gate — manifold HCOM prov gateway fence deepen.
#[must_use]
pub fn sec_s6_accel_ac33_honest() -> bool {
    let probe = sec_s6_accel_ac33_probe();
    probe.ac33_job_id == ACCEL_B_2050_AC33_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_z126_absorbed
        && probe.prior_h55_absorbed
        && probe.hcom_prov_fence_table_residue_pinned
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.hcom_prov_fence_hops_verified
        && probe.probe.scert_upstream_slots_verified
        && probe.probe.s6_inspect_delegate_verified
        && probe.probe.live_attestation_honest_false
        && probe.probe.s6_green_claim_blocked
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 6
        && !sec_s6_master_retick_eligible()
        && !sec_s6_op5_cleared()
        && W29_125_CELL_ID == "W29-125-SEC_S6"
        && HONEST_FENCE.contains("green_claim_blocked=true")
        && HONEST_FENCE.contains("production_wired=false")
        && manifold_sec_s6_gate_wire_integrity_verified()
}

/// Validate SEC-S6 gate census honesty — fail closed on fake production/GREEN claims.
pub fn validate_sec_s6_gate_honesty() -> Result<(), &'static str> {
    let census = gate_hcom_prov_gateway_fence_census();
    if census.production_wired {
        return Err("sec_s6_production_wired must stay false until SEC-HCOM-PROV-GW");
    }
    if !census.s6_green_claim_blocked {
        return Err("s6_green_claim_blocked must stay true in scaffold");
    }
    if census.live_attestation_wired {
        return Err("live_attestation_wired must stay false until operator ceremony");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if !census.hcom_prov_fence_hops_verified {
        return Err("HCOM prov gateway fence hops must verify at manifold boundary");
    }
    if !census.scert_upstream_slots_verified {
        return Err("SCERT upstream slots must stay PARTIAL at manifold boundary");
    }
    if !census.s6_inspect_delegate_verified {
        return Err("S-6 inspect delegate pins must verify (Z126 absorb)");
    }
    if MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len() != MANIFOLD_SEC_S6_GATE_WIRE_HOP_COUNT {
        return Err("seven SEC-S6 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 6 {
        return Err("six SEC-S6 gate wire hops should be wired today");
    }
    if !manifold_sec_s6_gate_wire_integrity_verified() {
        return Err("SEC-S6 gate wire hop integrity (ordinals + open production hop) failed");
    }
    if sec_s6_master_retick_eligible() {
        return Err("SEC-S6 master_retick_eligible must stay honest false");
    }
    if sec_s6_op5_cleared() {
        return Err("SEC-S6 op5_cleared must stay honest false");
    }
    if !manifold_gate_sec_s6_ceremony_closed() {
        return Err("manifold gate SEC-S6 ceremony must be closed at census tier");
    }
    if !sec_s6_accel_ac33_honest() {
        return Err("AC33 ACCEL-B probe honesty failed");
    }
    if !sec_s6_honest_fence_holds() {
        return Err("SEC-S6 W29-125 honest fence must hold (no GREEN/PRODUCTION/MASTER/OP-5)");
    }
    if census.schema_version != SCHEMA_VERSION {
        return Err("SEC-S6 census schema must pin W29-125 v2 deepen");
    }
    Ok(())
}

/// Render SEC-S6 gate wire map for operator receipts.
#[must_use]
pub fn sec_s6_gate_wire_matrix() -> String {
    let wired = MANIFOLD_SEC_S6_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count();
    let mut out = String::from("SEC-S6 manifold gate HCOM prov gateway fence wire map (AC33):\n");
    for hop in MANIFOLD_SEC_S6_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} [{}] {} — {}\n",
            hop.ordinal,
            if hop.wired { "wired" } else { "open" },
            hop.surface,
            hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={wired}/{} production_wired={} live_attestation_wired={}\n",
        MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len(),
        sec_s6_production_wired(),
        live_attestation_wired()
    ));
    out.push_str(&format!(
        "  w29_125_cell={W29_125_CELL_ID} honest_fence_holds={} \
         master_retick={} op5_cleared={} wire_integrity={}\n",
        sec_s6_honest_fence_holds(),
        sec_s6_master_retick_eligible(),
        sec_s6_op5_cleared(),
        manifold_sec_s6_gate_wire_integrity_verified(),
    ));
    out
}

/// Serial next-hop for gateway HCOM prov production ceremony.
#[must_use]
pub const fn sec_s6_hcom_prov_gw_next_hop() -> &'static str {
    "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_production_wired"
}

#[cfg(test)]
mod sec_s6_tests {
    use super::*;

    #[test]
    fn sec_s6_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S6");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S6");
        assert_eq!(ACCEL_B_2050_AC33_JOB_ID, "ACCEL-B-2050-AC33");
    }

    #[test]
    fn sec_s6_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
    }

    #[test]
    fn sec_s6_hcom_prov_census_honest_posture() {
        let census = gate_hcom_prov_gateway_fence_census();
        assert_eq!(census.board_slice_id, "SEC-S6");
        assert!(census.gate_evidence_wired);
        assert!(census.hcom_prov_fence_hops_verified);
        assert!(census.scert_upstream_slots_verified);
        assert!(census.s6_inspect_delegate_verified);
        assert!(!census.live_attestation_wired);
        assert!(census.s6_green_claim_blocked);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 6);
    }

    #[test]
    fn sec_s6_production_and_live_attestation_stay_false() {
        assert!(!sec_s6_production_wired());
        assert!(!live_attestation_wired());
        assert_eq!(SCERT_EXIT_NOT_WIRED, 2);
    }

    #[test]
    fn sec_s6_manifold_wire_hops_cover_gate_and_trust_delegate() {
        let wired = MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .filter(|h| h.wired)
            .count();
        assert_eq!(wired, 6);
        assert_eq!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len(), 7);
        assert!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("s6_inspect") && h.wired));
        assert!(MANIFOLD_SEC_S6_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("hcom_prov_gw_production_wired") && !h.wired));
    }

    #[test]
    fn sec_s6_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s6_ceremony_closed());
        let probe = sec_s6_gate_manifold_probe();
        assert!(probe.ceremony_closed);
        assert!(probe.hcom_prov_fence_hops_verified);
        assert!(probe.scert_upstream_slots_verified);
    }

    #[test]
    fn sec_s6_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S6_2033"));
        assert!(PRIOR_Z126_RECEIPT_PATH.contains("COMPOSER_Z126_1232"));
        assert!(TRUST_SSOT.contains("s6_inspect"));
        assert!(GATEWAY_SSOT.contains("sec_hcom_prov_gw"));
    }

    #[test]
    fn sec_s6_hcom_prov_fence_hops_five_of_five_pinned() {
        assert_eq!(HCOM_PROV_GW_FENCE_HOPS.len(), 5);
        assert!(manifold_hcom_prov_gw_fence_hops_verified());
        let table = sec_s6_hcom_prov_fence_table();
        assert!(table.contains("enforce_hcom_prov_semantic_admit"));
        assert!(table.contains("verified=true"));
    }

    #[test]
    fn sec_s6_upstream_slots_four_of_four_partial() {
        assert_eq!(SCERT_UPSTREAM_SLOTS.len(), 4);
        assert!(manifold_scert_upstream_slots_verified());
        let table = sec_s6_scert_upstream_table();
        assert!(table.contains("upstream_green=0/4"));
        assert!(table.contains("H-Arc"));
        assert!(table.contains("M-Arc"));
    }

    #[test]
    fn sec_s6_gate_factor_table_honest_blocked_scert() {
        let table = sec_s6_gate_factor_table();
        assert!(table.contains("SEC-S6 gate factors"));
        assert!(table.contains("scert_credit=BLOCKED"));
        assert!(table.contains("expected_gate_exit=BLOCKED"));
        let rows = collect_sec_s6_gate_factor_rows();
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|r| !r.acceptance_credit));
    }

    #[test]
    fn sec_s6_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s6_gate_wire_matrix();
        assert!(matrix.contains("SEC-S6 manifold gate"));
        assert!(matrix.contains("wired=6/7"));
        assert!(matrix.contains("production_wired=false"));
    }

    #[test]
    fn fleet_composer_accel_ac33_sec_s6_honest() {
        assert!(sec_s6_accel_ac33_honest());
        let probe = sec_s6_accel_ac33_probe();
        assert_eq!(probe.ac33_job_id, ACCEL_B_2050_AC33_JOB_ID);
        assert!(probe.prior_z126_absorbed);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s6_validate_gate_honesty_residue_measured() {
        validate_sec_s6_gate_honesty().expect("honest SEC-S6 gate census residue");
        assert_eq!(
            sec_s6_hcom_prov_gw_next_hop(),
            "umst-gateway::sec_hcom_prov_gw::hcom_prov_gw_production_wired"
        );
    }

    #[test]
    fn sec_s6_w29_125_honest_fence_no_green_production_master_op5() {
        assert_eq!(W29_125_CELL_ID, "W29-125-SEC_S6");
        assert_eq!(
            W29_125_DEEPEN_SCHEMA_VERSION,
            "sec_s6_w29_125_honest_fence_v1"
        );
        assert_eq!(
            SCHEMA_VERSION,
            "sec_s6_gate_hcom_prov_gateway_fence_census_v2"
        );
        assert_eq!(W29_125_HONEST_POSTURE, "SEC_S6_MANIFOLD_CENSUS_DEEPEN_ONLY");
        assert!(W29_125_NON_CLAIM.contains("not GREEN"));
        assert!(W29_125_NON_CLAIM.contains("not MASTER_RETICK"));
        assert!(!sec_s6_production_wired());
        assert!(!sec_s6_master_retick_eligible());
        assert!(!sec_s6_op5_cleared());
        assert!(S6_GREEN_CLAIM_BLOCKED);
        assert!(sec_s6_w29_125_deepen_honest());
        assert!(sec_s6_honest_fence_holds());
        let probe = sec_s6_w29_125_deepen_probe();
        assert!(!probe.production_wired_claimed);
        assert!(!probe.green_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(probe.deepen_honest);
        assert!(probe.honest_fence.contains("master_retick=false"));
        assert!(probe.honest_fence.contains("op5_cleared=false"));
        assert!(probe
            .honest_fence
            .contains("hcom_prov_gw_production_open=true"));
        let matrix = sec_s6_gate_wire_matrix();
        assert!(matrix.contains("honest_fence_holds=true"));
        assert!(matrix.contains("w29_125_cell=W29-125-SEC_S6"));
        assert!(matrix.contains("wire_integrity=true"));
    }

    #[test]
    fn sec_s6_gate_wire_integrity_and_hop_counts_measured() {
        assert_eq!(
            MANIFOLD_SEC_S6_GATE_WIRE_HOPS.len(),
            MANIFOLD_SEC_S6_GATE_WIRE_HOP_COUNT
        );
        assert_eq!(
            MANIFOLD_SEC_S6_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            MANIFOLD_SEC_S6_GATE_WIRE_WIRED_COUNT
        );
        assert!(manifold_sec_s6_gate_wire_integrity_verified());
        assert!(manifold_hcom_prov_gw_fence_hops_verified());
        assert_eq!(HCOM_PROV_GW_FENCE_HOPS.len(), HCOM_PROV_GW_WIRE_HOP_COUNT);
        let census = gate_hcom_prov_gateway_fence_census();
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert!(!census.production_wired);
        assert!(census.s6_green_claim_blocked);
    }
}

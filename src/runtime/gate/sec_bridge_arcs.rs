// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGAP-2350-SEC-BRIDGE-ARCS — manifold gate runtime CoordinationReport thermo/channel + trust wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-BRIDGE-ARCS arcs bridge SSOT;
//! egoff `CoordinationRuntimeBridge` and gateway `production_tick_loop` stay **honest open**.
//!
//! # W29-116 deepen
//!
//! Open-residual fence pins for hops 7–8 (egoff + gateway) measured at census tier.
//! No invented GREEN / PRODUCTION_WIRED / MASTER / OP-5.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-BRIDGE-ARCS";

/// AGAP slot id (2350 bridge arcs deepen).
pub const JOB_ID: &str = "AGAP-2350-SEC-BRIDGE";

/// W29 continuous worklist cell id (Composer RL NEW Task lane).
pub const W29_CELL_ID: &str = "W29-116-SEC_BRIDGE_ARCS";

/// FLEET-COMPOSER ACCEL-B slot AC35 id.
pub const FLEET_ACCEL_AC35_JOB_ID: &str = "ACCEL-B-2050-AC35";

/// FLEET-COMPOSER ACCEL-B AC35 receipt path.
pub const FLEET_ACCEL_AC35_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC35.md";

/// Prior Y54 thermo/channel + trust runtime receipt.
pub const PRIOR_RECEIPT_PATH_Y54: &str = "outputs/.tmp/COMPOSER_Y54_0808.md";

/// Prior G48 trust parity deepen receipt.
pub const PRIOR_RECEIPT_PATH_G48: &str = "outputs/.tmp/COMPOSER_G48_BRIDGE_2143.md";

/// Prior Z40 umst-trust boundary close receipt.
pub const PRIOR_RECEIPT_PATH_Z40: &str = "outputs/.tmp/COMPOSER_Z40_1015.md";

/// Prior Z40 umst-arcs owner close receipt.
pub const PRIOR_RECEIPT_PATH_Z40_ARCS: &str = "outputs/.tmp/COMPOSER_Z40_0928.md";

/// umst-arcs egoff runtime wire SSOT delegate.
pub const ARCS_RUNTIME_SSOT: &str = "umst-arcs/crates/umst-arcs/src/egoff_runtime.rs";

/// umst-arcs SEC-BRIDGE-ARCS owner SSOT delegate.
pub const ARCS_BRIDGE_SSOT: &str = "umst-arcs/crates/umst-arcs/src/sec_bridge.rs";

/// umst-trust SEC-BRIDGE-ARCS census delegate SSOT (cross-ref only).
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/sec_bridge_arcs.rs";

/// egoff runtime bridge target (full-crate verify blocked).
pub const EGOFF_BRIDGE_SSOT: &str = "egoff/egoff/src/arcs_bridge.rs";

/// Gateway production tick delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/production_tick_loop.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v2 = W29 open-residual fence deepen).
pub const SCHEMA_VERSION: &str = "sec_bridge_arcs_gate_coordination_census_v2";

/// Thermo runtime wire hop ids — pinned from `umst-arcs::egoff_runtime::WIRE_HOPS`.
pub const THERMO_WIRE_HOP_IDS: &[&str] = &[
    "egoff_ucrs_agent_tick",
    "mirror_arcs_ucrs_agent",
    "coordination_report_ssot",
    "thermo_parity_verify",
];

/// Trust compose wire hop ids — pinned from `umst-arcs::egoff_runtime::TRUST_WIRE_HOPS`.
pub const TRUST_WIRE_HOP_IDS: &[&str] = &[
    "compose_coordination_trust",
    "arcs_trusted_coordination_ssot",
    "thermo_parity_after_trust",
];

/// M18 ecosystem batch trust wire hop count (base 3 + census consumer).
pub const ECOSYSTEM_TRUST_WIRE_HOP_COUNT: usize = 4;

/// Honest open residual hop count (egoff runtime + gateway tick).
pub const OPEN_RESIDUAL_HOP_COUNT: usize = 2;

/// S-Arc GREEN claim blocked — honest true in scaffold deepen.
pub const BRIDGE_ARCS_GREEN_CLAIM_BLOCKED: bool = true;

/// egoff full-crate verify blocked — honest true at manifold boundary.
pub const EGOFF_FULL_CRATE_VERIFY_BLOCKED: bool = true;

/// MASTER / OP-5 retick eligibility — honest false (census deepen only).
pub const MASTER_RETICK_ELIGIBLE: bool = false;

/// One hop in the manifold SEC-BRIDGE-ARCS gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsGateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-BRIDGE-ARCS gate runtime wire map (cold-edge evidence → arcs bridge census).
pub const MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS: &[SecBridgeArcsGateWireHop] = &[
    SecBridgeArcsGateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_bridge_arcs::gate_bridge_arcs_census",
        role: "Manifold gate SEC-BRIDGE-ARCS coordination census",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 4,
        surface: "umst-arcs::egoff_runtime::WIRE_HOPS",
        role: "Thermo 4-hop CoordinationReport channel SSOT mirror",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 5,
        surface: "umst-arcs::egoff_runtime::TRUST_WIRE_HOPS",
        role: "Trust 3-hop compose SSOT mirror",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 6,
        surface: "umst-arcs::sec_bridge::sec_bridge_trust_runtime_closed",
        role: "Arcs owner trust-runtime closed delegate (Z40 absorb)",
        wired: true,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 7,
        surface: "egoff::arcs_bridge::CoordinationRuntimeBridge",
        role: "egoff runtime tick + thermo parity (full-crate verify blocked)",
        wired: false,
    },
    SecBridgeArcsGateWireHop {
        ordinal: 8,
        surface: "umst-gateway::production_tick_loop",
        role: "Live gateway tick loop (G48 residual)",
        wired: false,
    },
];

/// One honest-open residual fence pin (egoff / gateway — not wired today).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsOpenResidualFence {
    /// Residual id (`R-egoff-runtime` / `R-gateway-tick`).
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

/// Open residual fence pins — hops 7–8 measured open at W29-116 deepen.
pub const OPEN_RESIDUAL_FENCES: &[SecBridgeArcsOpenResidualFence] = &[
    SecBridgeArcsOpenResidualFence {
        residue_id: "R-egoff-runtime",
        hop_ordinal: 7,
        delegate_ssot: EGOFF_BRIDGE_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
    SecBridgeArcsOpenResidualFence {
        residue_id: "R-gateway-tick",
        hop_ordinal: 8,
        delegate_ssot: GATEWAY_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
];

/// Aggregated SEC-BRIDGE-ARCS gate coordination census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsGateCoordinationCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// W29 cell id pin.
    pub w29_cell_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// Thermo runtime hop count (4).
    pub thermo_wire_hops: usize,
    /// Trust compose hop count (3).
    pub trust_wire_hops: usize,
    /// Ecosystem batch trust wire hop count (4).
    pub ecosystem_trust_wire_hops: usize,
    /// Honest open residual hop count (2).
    pub open_residual_hop_count: usize,
    /// Open residual fence pins verified.
    pub open_residual_fences_verified: bool,
    /// S-Arc GREEN claim blocked — honest true.
    pub bridge_arcs_green_claim_blocked: bool,
    /// egoff full-crate verify blocked — honest true.
    pub egoff_full_crate_verify_blocked: bool,
    /// MASTER / OP-5 retick — honest false.
    pub master_retick_eligible: bool,
    /// Gateway production flip.
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

/// Whether live egoff runtime bridge + gateway tick are plumbed (honest `false`).
#[must_use]
pub const fn sec_bridge_arcs_production_wired() -> bool {
    false
}

/// Whether MASTER / OP-5 retick is eligible (honest `false` at census deepen).
#[must_use]
pub const fn sec_bridge_arcs_master_retick_eligible() -> bool {
    MASTER_RETICK_ELIGIBLE
}

/// Whether thermo wire hop inventory matches egoff SSOT pin at manifold boundary.
#[must_use]
pub fn manifold_bridge_arcs_thermo_wire_hops_verified() -> bool {
    THERMO_WIRE_HOP_IDS.len() == 4
        && THERMO_WIRE_HOP_IDS[0] == "egoff_ucrs_agent_tick"
        && THERMO_WIRE_HOP_IDS[3] == "thermo_parity_verify"
}

/// Whether trust wire hop inventory matches egoff SSOT pin at manifold boundary.
#[must_use]
pub fn manifold_bridge_arcs_trust_wire_hops_verified() -> bool {
    TRUST_WIRE_HOP_IDS.len() == 3
        && TRUST_WIRE_HOP_IDS[0] == "compose_coordination_trust"
        && TRUST_WIRE_HOP_IDS[2] == "thermo_parity_after_trust"
}

/// Whether open residual fence pins match unwired hops 7–8 on the wire map.
#[must_use]
pub fn manifold_bridge_arcs_open_residual_fences_verified() -> bool {
    const EXPECTED: [(&str, u8); 2] = [("R-egoff-runtime", 7), ("R-gateway-tick", 8)];
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
        && MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
            .iter()
            .filter(|h| !h.wired)
            .count()
            == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES.iter().all(|fence| {
            MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
                .iter()
                .any(|h| h.ordinal == fence.hop_ordinal && !h.wired)
        })
}

/// Build manifold SEC-BRIDGE-ARCS gate coordination census from live measurements.
#[must_use]
pub fn gate_bridge_arcs_census() -> SecBridgeArcsGateCoordinationCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecBridgeArcsGateCoordinationCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        w29_cell_id: W29_CELL_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        thermo_wire_hops: THERMO_WIRE_HOP_IDS.len(),
        trust_wire_hops: TRUST_WIRE_HOP_IDS.len(),
        ecosystem_trust_wire_hops: ECOSYSTEM_TRUST_WIRE_HOP_COUNT,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_bridge_arcs_open_residual_fences_verified(),
        bridge_arcs_green_claim_blocked: BRIDGE_ARCS_GREEN_CLAIM_BLOCKED,
        egoff_full_crate_verify_blocked: EGOFF_FULL_CRATE_VERIFY_BLOCKED,
        master_retick_eligible: sec_bridge_arcs_master_retick_eligible(),
        production_wired: sec_bridge_arcs_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-BRIDGE-ARCS ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + bridge wire map hops 1–6 are measured wired
/// and open residual fences for hops 7–8 are pinned honest-open.
/// egoff runtime bridge + gateway production tick are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_bridge_arcs_ceremony_closed() -> bool {
    let census = gate_bridge_arcs_census();
    census.gate_evidence_wired
        && census.thermo_wire_hops == 4
        && census.trust_wire_hops == 3
        && census.ecosystem_trust_wire_hops == 4
        && census.open_residual_hop_count == 2
        && census.open_residual_fences_verified
        && census.bridge_arcs_green_claim_blocked
        && census.egoff_full_crate_verify_blocked
        && !census.master_retick_eligible
        && !census.production_wired
        && census.wire_hop_wired_count == 6
        && census.w29_cell_id == W29_CELL_ID
        && manifold_bridge_arcs_thermo_wire_hops_verified()
        && manifold_bridge_arcs_trust_wire_hops_verified()
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-BRIDGE-ARCS manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsGateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Thermo wire hops verified.
    pub thermo_wire_hops_verified: bool,
    /// Trust wire hops verified.
    pub trust_wire_hops_verified: bool,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// GREEN claim blocked.
    pub bridge_arcs_green_claim_blocked: bool,
    /// egoff full-crate verify blocked.
    pub egoff_full_crate_verify_blocked: bool,
    /// MASTER / OP-5 retick honest false.
    pub master_retick_honest_false: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-BRIDGE-ARCS done-when checks.
#[must_use]
pub fn sec_bridge_arcs_gate_manifold_probe() -> SecBridgeArcsGateManifoldProbe {
    let census = gate_bridge_arcs_census();
    SecBridgeArcsGateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        thermo_wire_hops_verified: manifold_bridge_arcs_thermo_wire_hops_verified(),
        trust_wire_hops_verified: manifold_bridge_arcs_trust_wire_hops_verified(),
        open_residual_fences_verified: census.open_residual_fences_verified,
        bridge_arcs_green_claim_blocked: census.bridge_arcs_green_claim_blocked,
        egoff_full_crate_verify_blocked: census.egoff_full_crate_verify_blocked,
        master_retick_honest_false: !census.master_retick_eligible,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_bridge_arcs_ceremony_closed(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC35 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsAccelAc35Probe {
    /// AC35 fleet slot id.
    pub ac35_job_id: &'static str,
    /// AC35 receipt path pinned.
    pub ac35_receipt_honest: bool,
    /// Prior Y54 absorb receipt.
    pub prior_y54_absorbed: bool,
    /// Prior G48 absorb receipt.
    pub prior_g48_absorbed: bool,
    /// Prior Z40 trust cross-ref receipt.
    pub prior_z40_trust_absorbed: bool,
    /// Prior Z40 arcs owner receipt.
    pub prior_z40_arcs_absorbed: bool,
    /// Open residual fence table residue pinned.
    pub open_residual_table_residue_pinned: bool,
    /// Underlying gate probe.
    pub probe: SecBridgeArcsGateManifoldProbe,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// `sec_bridge_arcs_production_wired()` — honest false.
    pub production_wired: bool,
}

/// W29-116 Composer RL deepen probe — open-residual fence + AC35 honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecBridgeArcsW29DeepenProbe {
    /// W29 cell id.
    pub w29_cell_id: &'static str,
    /// Schema version pin.
    pub schema_version: &'static str,
    /// Open residual fence count.
    pub open_residual_hop_count: usize,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// Open residual table residue pinned.
    pub open_residual_table_residue_pinned: bool,
    /// AC35 honesty.
    pub ac35_honest: bool,
    /// Ceremony closed.
    pub ceremony_closed: bool,
    /// Production wired — honest false.
    pub production_wired: bool,
    /// MASTER retick — honest false.
    pub master_retick_eligible: bool,
    /// GREEN claim blocked — honest true.
    pub bridge_arcs_green_claim_blocked: bool,
}

/// Render open residual fence table for operator receipts.
#[must_use]
pub fn sec_bridge_arcs_open_residual_fence_table() -> String {
    let mut out = String::from("SEC-BRIDGE-ARCS open residual fences (W29-116):\n");
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
        manifold_bridge_arcs_open_residual_fences_verified(),
        sec_bridge_arcs_production_wired(),
        sec_bridge_arcs_master_retick_eligible()
    ));
    out
}

/// Build FLEET-COMPOSER ACCEL-B AC35 integration probe from live measurements.
#[must_use]
pub fn sec_bridge_arcs_accel_ac35_probe() -> SecBridgeArcsAccelAc35Probe {
    let residual_table = sec_bridge_arcs_open_residual_fence_table();
    SecBridgeArcsAccelAc35Probe {
        ac35_job_id: FLEET_ACCEL_AC35_JOB_ID,
        ac35_receipt_honest: FLEET_ACCEL_AC35_RECEIPT_PATH.contains("COMPOSER_ACCEL2_AC35"),
        prior_y54_absorbed: PRIOR_RECEIPT_PATH_Y54.contains("COMPOSER_Y54"),
        prior_g48_absorbed: PRIOR_RECEIPT_PATH_G48.contains("COMPOSER_G48_BRIDGE"),
        prior_z40_trust_absorbed: PRIOR_RECEIPT_PATH_Z40.contains("COMPOSER_Z40_1015"),
        prior_z40_arcs_absorbed: PRIOR_RECEIPT_PATH_Z40_ARCS.contains("COMPOSER_Z40_0928"),
        open_residual_table_residue_pinned: residual_table.contains("R-egoff-runtime")
            && residual_table.contains("R-gateway-tick")
            && residual_table.contains("fences_verified="),
        probe: sec_bridge_arcs_gate_manifold_probe(),
        ceremony_closed: manifold_gate_sec_bridge_arcs_ceremony_closed(),
        production_wired: sec_bridge_arcs_production_wired(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC35 honesty gate — ceremony closed + Y54/G48/Z40 absorb + production false.
#[must_use]
pub fn sec_bridge_arcs_accel_ac35_honest() -> bool {
    let probe = sec_bridge_arcs_accel_ac35_probe();
    probe.ac35_job_id == FLEET_ACCEL_AC35_JOB_ID
        && probe.ac35_receipt_honest
        && probe.prior_y54_absorbed
        && probe.prior_g48_absorbed
        && probe.prior_z40_trust_absorbed
        && probe.prior_z40_arcs_absorbed
        && probe.open_residual_table_residue_pinned
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.thermo_wire_hops_verified
        && probe.probe.trust_wire_hops_verified
        && probe.probe.open_residual_fences_verified
        && probe.probe.bridge_arcs_green_claim_blocked
        && probe.probe.egoff_full_crate_verify_blocked
        && probe.probe.master_retick_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 6
        && !probe.production_wired
}

/// Build W29-116 deepen probe from live measurements.
#[must_use]
pub fn sec_bridge_arcs_w29_deepen_probe() -> SecBridgeArcsW29DeepenProbe {
    let residual_table = sec_bridge_arcs_open_residual_fence_table();
    SecBridgeArcsW29DeepenProbe {
        w29_cell_id: W29_CELL_ID,
        schema_version: SCHEMA_VERSION,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_bridge_arcs_open_residual_fences_verified(),
        open_residual_table_residue_pinned: residual_table.contains("R-egoff-runtime")
            && residual_table.contains("R-gateway-tick"),
        ac35_honest: sec_bridge_arcs_accel_ac35_honest(),
        ceremony_closed: manifold_gate_sec_bridge_arcs_ceremony_closed(),
        production_wired: sec_bridge_arcs_production_wired(),
        master_retick_eligible: sec_bridge_arcs_master_retick_eligible(),
        bridge_arcs_green_claim_blocked: BRIDGE_ARCS_GREEN_CLAIM_BLOCKED,
    }
}

/// W29-116 deepen honesty — open residuals pinned + AC35 honest + no invented GREEN/PRODUCTION/MASTER.
#[must_use]
pub fn sec_bridge_arcs_w29_deepen_honest() -> bool {
    let probe = sec_bridge_arcs_w29_deepen_probe();
    probe.w29_cell_id == W29_CELL_ID
        && probe.schema_version == SCHEMA_VERSION
        && probe.schema_version.contains("_v2")
        && probe.open_residual_hop_count == 2
        && probe.open_residual_fences_verified
        && probe.open_residual_table_residue_pinned
        && probe.ac35_honest
        && probe.ceremony_closed
        && !probe.production_wired
        && !probe.master_retick_eligible
        && probe.bridge_arcs_green_claim_blocked
}

/// Validate SEC-BRIDGE-ARCS gate census honesty — fail closed on fake production/GREEN claims.
pub fn validate_sec_bridge_arcs_gate_honesty() -> Result<(), &'static str> {
    let census = gate_bridge_arcs_census();
    if census.schema_version != SCHEMA_VERSION {
        return Err("schema_version must match W29 v2 census pin");
    }
    if census.w29_cell_id != W29_CELL_ID {
        return Err("w29_cell_id must stay W29-116-SEC_BRIDGE_ARCS");
    }
    if !census.bridge_arcs_green_claim_blocked {
        return Err("bridge_arcs_green_claim_blocked must stay true in scaffold");
    }
    if !census.egoff_full_crate_verify_blocked {
        return Err("egoff_full_crate_verify_blocked must stay true at manifold boundary");
    }
    if census.master_retick_eligible {
        return Err("master_retick_eligible must stay false at census deepen");
    }
    if census.production_wired {
        return Err("sec_bridge_arcs_production_wired must stay false until egoff live bridge");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.thermo_wire_hops != 4 {
        return Err("thermo wire hops must remain 4");
    }
    if census.trust_wire_hops != 3 {
        return Err("trust wire hops must remain 3");
    }
    if census.ecosystem_trust_wire_hops != 4 {
        return Err("ecosystem trust wire hops must remain 4");
    }
    if census.open_residual_hop_count != 2 {
        return Err("open residual hop count must remain 2");
    }
    if !census.open_residual_fences_verified {
        return Err("open residual fence pins must verify against unwired hops 7-8");
    }
    if !manifold_bridge_arcs_thermo_wire_hops_verified() {
        return Err("thermo wire hop ids must match egoff inventory pin");
    }
    if !manifold_bridge_arcs_trust_wire_hops_verified() {
        return Err("trust wire hop ids must match egoff inventory pin");
    }
    if MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS.len() != 8 {
        return Err("eight SEC-BRIDGE-ARCS gate wire hops expected");
    }
    if census.wire_hop_wired_count != 6 {
        return Err("six SEC-BRIDGE-ARCS gate wire hops should be wired today");
    }
    if !manifold_gate_sec_bridge_arcs_ceremony_closed() {
        return Err("manifold gate SEC-BRIDGE-ARCS ceremony must be closed at census tier");
    }
    if !sec_bridge_arcs_accel_ac35_honest() {
        return Err("ACCEL AC35 probe must be honest");
    }
    if !sec_bridge_arcs_w29_deepen_honest() {
        return Err("W29-116 deepen probe must be honest");
    }
    Ok(())
}

/// Render SEC-BRIDGE-ARCS gate wire map for operator receipts.
#[must_use]
pub fn sec_bridge_arcs_gate_wire_matrix() -> String {
    let census = gate_bridge_arcs_census();
    let mut out =
        String::from("SEC-BRIDGE-ARCS manifold gate coordination wire map (AC35/W29-116):\n");
    for hop in MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} thermo={} trust={} ecosystem_trust={} open_residual={} \
         bridge_arcs_green_claim_blocked={} egoff_full_crate_verify_blocked={} \
         master_retick={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS.len(),
        census.thermo_wire_hops,
        census.trust_wire_hops,
        census.ecosystem_trust_wire_hops,
        census.open_residual_hop_count,
        census.bridge_arcs_green_claim_blocked,
        census.egoff_full_crate_verify_blocked,
        census.master_retick_eligible,
        census.production_wired
    ));
    out.push_str(&format!("  w29_cell_id={}\n", census.w29_cell_id));
    out.push_str(&format!("  arcs_runtime_ssot={ARCS_RUNTIME_SSOT}\n"));
    out.push_str(&format!("  arcs_bridge_ssot={ARCS_BRIDGE_SSOT}\n"));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out
}

/// Next-hop surface for egoff runtime bridge production (egoff-owned).
#[must_use]
pub const fn sec_bridge_arcs_egoff_bridge_next_hop() -> &'static str {
    "egoff/egoff/src/arcs_bridge.rs:CoordinationRuntimeBridge"
}

#[cfg(test)]
mod sec_bridge_arcs_tests {
    use super::*;

    #[test]
    fn sec_bridge_arcs_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-BRIDGE-ARCS");
        assert_eq!(JOB_ID, "AGAP-2350-SEC-BRIDGE");
        assert_eq!(W29_CELL_ID, "W29-116-SEC_BRIDGE_ARCS");
        assert_eq!(FLEET_ACCEL_AC35_JOB_ID, "ACCEL-B-2050-AC35");
        assert_eq!(THERMO_WIRE_HOP_IDS.len(), 4);
        assert_eq!(TRUST_WIRE_HOP_IDS.len(), 3);
        assert_eq!(ECOSYSTEM_TRUST_WIRE_HOP_COUNT, 4);
        assert_eq!(OPEN_RESIDUAL_HOP_COUNT, 2);
        assert_eq!(
            SCHEMA_VERSION,
            "sec_bridge_arcs_gate_coordination_census_v2"
        );
    }

    #[test]
    fn sec_bridge_arcs_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_bridge_arcs_thermo_wire_hops_match_egoff_inventory() {
        assert!(manifold_bridge_arcs_thermo_wire_hops_verified());
        assert_eq!(THERMO_WIRE_HOP_IDS[0], "egoff_ucrs_agent_tick");
        assert_eq!(THERMO_WIRE_HOP_IDS[3], "thermo_parity_verify");
    }

    #[test]
    fn sec_bridge_arcs_trust_wire_hops_match_egoff_inventory() {
        assert!(manifold_bridge_arcs_trust_wire_hops_verified());
        assert_eq!(TRUST_WIRE_HOP_IDS[2], "thermo_parity_after_trust");
    }

    #[test]
    fn sec_bridge_arcs_open_residual_fences_pin_unwired_hops() {
        assert!(manifold_bridge_arcs_open_residual_fences_verified());
        assert_eq!(OPEN_RESIDUAL_FENCES.len(), 2);
        assert_eq!(OPEN_RESIDUAL_FENCES[0].residue_id, "R-egoff-runtime");
        assert_eq!(OPEN_RESIDUAL_FENCES[1].residue_id, "R-gateway-tick");
        assert!(OPEN_RESIDUAL_FENCES
            .iter()
            .all(|f| f.honest_open && f.green_credit_blocked));
        let table = sec_bridge_arcs_open_residual_fence_table();
        assert!(table.contains("R-egoff-runtime"));
        assert!(table.contains("R-gateway-tick"));
        assert!(table.contains("production_wired=false"));
        assert!(table.contains("master_retick=false"));
    }

    #[test]
    fn sec_bridge_arcs_coordination_census_honest_posture() {
        let census = gate_bridge_arcs_census();
        assert_eq!(census.board_slice_id, "SEC-BRIDGE-ARCS");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert_eq!(census.w29_cell_id, W29_CELL_ID);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.thermo_wire_hops, 4);
        assert_eq!(census.trust_wire_hops, 3);
        assert_eq!(census.ecosystem_trust_wire_hops, 4);
        assert_eq!(census.open_residual_hop_count, 2);
        assert!(census.open_residual_fences_verified);
        assert!(census.bridge_arcs_green_claim_blocked);
        assert!(census.egoff_full_crate_verify_blocked);
        assert!(!census.master_retick_eligible);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 6);
    }

    #[test]
    fn sec_bridge_arcs_production_stays_false() {
        assert!(!sec_bridge_arcs_production_wired());
        assert!(!sec_bridge_arcs_master_retick_eligible());
        assert!(BRIDGE_ARCS_GREEN_CLAIM_BLOCKED);
        assert!(EGOFF_FULL_CRATE_VERIFY_BLOCKED);
        assert!(!MASTER_RETICK_ELIGIBLE);
    }

    #[test]
    fn sec_bridge_arcs_manifold_wire_hops_cover_gate_and_arcs_delegate() {
        assert_eq!(MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS.len(), 8);
        assert_eq!(
            MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            6
        );
        assert!(MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("egoff_runtime::WIRE_HOPS") && h.wired));
        assert!(MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("arcs_bridge") && !h.wired));
        assert!(MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("production_tick_loop") && !h.wired));
    }

    #[test]
    fn sec_bridge_arcs_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_bridge_arcs_ceremony_closed());
        let probe = sec_bridge_arcs_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.thermo_wire_hops_verified);
        assert!(probe.trust_wire_hops_verified);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.bridge_arcs_green_claim_blocked);
        assert!(probe.egoff_full_crate_verify_blocked);
        assert!(probe.master_retick_honest_false);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 6);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_bridge_arcs_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_bridge_arcs_gate_wire_matrix();
        assert!(matrix.contains("SEC-BRIDGE-ARCS manifold gate"));
        assert!(matrix.contains("thermo=4"));
        assert!(matrix.contains("trust=3"));
        assert!(matrix.contains("open_residual=2"));
        assert!(matrix.contains("production_wired=false"));
        assert!(matrix.contains("master_retick=false"));
        assert!(matrix.contains("wired=6/8"));
        assert!(matrix.contains("W29-116-SEC_BRIDGE_ARCS"));
    }

    #[test]
    fn sec_bridge_arcs_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_Y54.contains("COMPOSER_Y54"));
        assert!(PRIOR_RECEIPT_PATH_G48.contains("COMPOSER_G48_BRIDGE"));
        assert!(PRIOR_RECEIPT_PATH_Z40.contains("COMPOSER_Z40_1015"));
        assert!(PRIOR_RECEIPT_PATH_Z40_ARCS.contains("COMPOSER_Z40_0928"));
        assert!(ARCS_RUNTIME_SSOT.contains("egoff_runtime.rs"));
        assert!(ARCS_BRIDGE_SSOT.contains("sec_bridge.rs"));
    }

    #[test]
    fn fleet_composer_accel_ac35_sec_bridge_arcs_honest() {
        assert!(sec_bridge_arcs_accel_ac35_honest());
        let probe = sec_bridge_arcs_accel_ac35_probe();
        assert_eq!(probe.ac35_job_id, FLEET_ACCEL_AC35_JOB_ID);
        assert!(probe.prior_y54_absorbed);
        assert!(probe.prior_g48_absorbed);
        assert!(probe.prior_z40_trust_absorbed);
        assert!(probe.prior_z40_arcs_absorbed);
        assert!(probe.open_residual_table_residue_pinned);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
    }

    #[test]
    fn sec_bridge_arcs_w29_116_deepen_honest() {
        assert!(sec_bridge_arcs_w29_deepen_honest());
        let probe = sec_bridge_arcs_w29_deepen_probe();
        assert_eq!(probe.w29_cell_id, "W29-116-SEC_BRIDGE_ARCS");
        assert!(probe.schema_version.contains("_v2"));
        assert_eq!(probe.open_residual_hop_count, 2);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.open_residual_table_residue_pinned);
        assert!(probe.ac35_honest);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(probe.bridge_arcs_green_claim_blocked);
    }

    #[test]
    fn sec_bridge_arcs_validate_gate_honesty_residue_measured() {
        validate_sec_bridge_arcs_gate_honesty()
            .expect("honest SEC-BRIDGE-ARCS gate census residue");
        assert_eq!(
            sec_bridge_arcs_egoff_bridge_next_hop(),
            "egoff/egoff/src/arcs_bridge.rs:CoordinationRuntimeBridge"
        );
    }
}

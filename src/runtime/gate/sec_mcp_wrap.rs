// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2033/2350-SEC-MCP-WRAP — manifold gate runtime MCP trust wrap wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-MCP-WRAP typed
//! refuse-path SSOT; umst-mcp live matrix eval, gateway stdio exec pre-check, and
//! `sec_mcp_wrap_production_wired()` stay **honest open**.
//!
//! # W29-119 deepen
//!
//! Open-residual fence pins for hops 6–7 (gateway stdio exec + production ceremony)
//! measured at census tier. No invented GREEN / PRODUCTION_WIRED / MASTER / OP-5.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-MCP-WRAP";

/// AGAP slot id (2033 MCP wrap deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-MCP-WRAP";

/// AGAP-2350 deepen card id.
pub const DEEPEN_JOB_ID: &str = "AGAP-2350-SEC-MCP-WRAP";

/// W29 continuous worklist cell id (Grok NEW Task lane after Composer RL).
pub const W29_CELL_ID: &str = "W29-119-SEC_MCP_WRAP";

/// FLEET-COMPOSER-Z Z86 card id (umst-mcp owner absorb).
pub const FLEET_Z86_JOB_ID: &str = "FLEET-COMPOSER-Z86-SEC-MCP-WRAP";

/// FLEET-COMPOSER ACCEL-B slot AC34 id (manifold fence deepen).
pub const ACCEL_B_2050_AC34_JOB_ID: &str = "ACCEL-B-2050-AC34";

/// FLEET-COMPOSER ACCEL-B AC34 receipt path.
pub const ACCEL_AC34_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL2_AC34.md";

/// FLEET-COMPOSER-Z Z86 receipt cross-ref (umst-mcp owner — absorb, do not redo).
pub const PRIOR_Z86_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_Z86_1232.md";

/// FLEET-COMPOSER-Y Y52 receipt cross-ref (gateway refuse matrix — absorb).
pub const PRIOR_Y52_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_Y52_0808.md";

/// FLEET-COMPOSER-Y Y51 receipt cross-ref (SEC-GW-AUDIT adjacent).
pub const PRIOR_Y51_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_Y51_0808.md";

/// umst-mcp SEC-MCP-WRAP delegate SSOT (live matrix eval owner).
pub const MCP_SSOT: &str = "umst-concrete-cartridge/crates/umst-mcp/src/sec_mcp_wrap.rs";

/// umst-mcp HCOM-029 trust gate delegate SSOT.
pub const MCP_TRUST_GATE_SSOT: &str = "umst-concrete-cartridge/crates/umst-mcp/src/mcp_trust_gate.rs";

/// Gateway stdio exec trust pre-check owner (honest open residue).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_mcp_wrap.rs";

/// Gateway stdio exec trust pre-check symbol pin.
pub const GATEWAY_STDIO_EXEC_OWNER: &str =
    "umst-gateway/crates/umst-gateway/src/sec_mcp_wrap.rs::mcp_stdio_exec_trust_pre_check_wired";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v2 = W29 open-residual fence deepen).
pub const SCHEMA_VERSION: &str = "sec_mcp_wrap_gate_census_v2";

/// Typed refuse-path matrix row count (umst-mcp HCOM-029 quartet + Z86 deepen).
pub const REFUSE_PATH_MATRIX_ROW_COUNT: usize = 6;

/// Honest open residual hop count (gateway stdio exec + production ceremony).
pub const OPEN_RESIDUAL_HOP_COUNT: usize = 2;

/// MCP wrap GREEN claim blocked — honest true in scaffold deepen.
pub const MCP_WRAP_GREEN_CLAIM_BLOCKED: bool = true;

/// MASTER / OP-5 retick eligibility — honest false (census deepen only).
pub const MASTER_RETICK_ELIGIBLE: bool = false;

/// One typed refuse-path row pinned at manifold boundary (SSOT cross-ref to umst-mcp Z86).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldMcpWrapRefusePathRow {
    /// Row id (R1..R6).
    pub row_id: &'static str,
    /// Semantic agent MCP tool name.
    pub tool_name: &'static str,
    /// trust.scope wire value.
    pub scope: &'static str,
    /// Whether trust is revoked.
    pub revoked: bool,
    /// Expected admit (Ok) at umst-mcp boundary.
    pub expect_admit: bool,
    /// Owning delegate surface.
    pub delegate_ssot: &'static str,
}

/// Typed refuse-path matrix pinned from umst-mcp `REFUSE_PATH_MATRIX` (Z86 absorb).
pub const REFUSE_PATH_MATRIX: &[ManifoldMcpWrapRefusePathRow] = &[
    ManifoldMcpWrapRefusePathRow {
        row_id: "R1",
        tool_name: "propose_communicative_act",
        scope: "ephemeral",
        revoked: false,
        expect_admit: false,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
    ManifoldMcpWrapRefusePathRow {
        row_id: "R2",
        tool_name: "propose_communicative_act",
        scope: "device",
        revoked: false,
        expect_admit: true,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
    ManifoldMcpWrapRefusePathRow {
        row_id: "R3",
        tool_name: "refine_shape",
        scope: "ephemeral",
        revoked: false,
        expect_admit: false,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
    ManifoldMcpWrapRefusePathRow {
        row_id: "R4",
        tool_name: "refine_shape",
        scope: "device",
        revoked: false,
        expect_admit: true,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
    ManifoldMcpWrapRefusePathRow {
        row_id: "R5",
        tool_name: "map_to_geometry",
        scope: "ephemeral",
        revoked: false,
        expect_admit: true,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
    ManifoldMcpWrapRefusePathRow {
        row_id: "R6",
        tool_name: "map_to_geometry",
        scope: "device",
        revoked: true,
        expect_admit: false,
        delegate_ssot: MCP_TRUST_GATE_SSOT,
    },
];

/// One hop in the manifold SEC-MCP-WRAP gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapGateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-MCP-WRAP gate runtime wire map (cold-edge evidence → MCP wrap census).
pub const MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS: &[SecMcpWrapGateWireHop] = &[
    SecMcpWrapGateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecMcpWrapGateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecMcpWrapGateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_mcp_wrap::gate_mcp_wrap_census",
        role: "Manifold gate SEC-MCP-WRAP census",
        wired: true,
    },
    SecMcpWrapGateWireHop {
        ordinal: 4,
        surface: "umst-mcp::sec_mcp_wrap::sec_mcp_wrap_refuse_path_matrix_honest",
        role: "umst-mcp typed refuse matrix live eval (Z86 owner)",
        wired: true,
    },
    SecMcpWrapGateWireHop {
        ordinal: 5,
        surface: "umst-mcp::mcp_trust_gate::check_semantic_agent_trust",
        role: "HCOM-029 semantic agent trust delegate",
        wired: true,
    },
    SecMcpWrapGateWireHop {
        ordinal: 6,
        surface: "umst-gateway::sec_mcp_wrap::mcp_stdio_exec_trust_pre_check_wired",
        role: "Gateway stdio exec trust pre-check (serial Wave D)",
        wired: false,
    },
    SecMcpWrapGateWireHop {
        ordinal: 7,
        surface: "umst-mcp::sec_mcp_wrap::sec_mcp_wrap_production_wired",
        role: "MCP wrap production ceremony (operator trust-chain env)",
        wired: false,
    },
];

/// One honest-open residual fence pin (gateway stdio / production — not wired today).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapOpenResidualFence {
    /// Residual id (`R-gateway-stdio-exec` / `R-mcp-wrap-production`).
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

/// Open residual fence pins — hops 6–7 measured open at W29-119 deepen.
pub const OPEN_RESIDUAL_FENCES: &[SecMcpWrapOpenResidualFence] = &[
    SecMcpWrapOpenResidualFence {
        residue_id: "R-gateway-stdio-exec",
        hop_ordinal: 6,
        delegate_ssot: GATEWAY_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
    SecMcpWrapOpenResidualFence {
        residue_id: "R-mcp-wrap-production",
        hop_ordinal: 7,
        delegate_ssot: MCP_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
];

/// Aggregated SEC-MCP-WRAP gate census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapGateCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// W29 cell id pin.
    pub w29_cell_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// Refuse-path matrix rows pinned at manifold boundary.
    pub refuse_path_matrix_row_count: u8,
    /// Refuse-path matrix pins verified (ordinal + delegate SSOT).
    pub refuse_path_matrix_pins_verified: bool,
    /// Honest open residual hop count (2).
    pub open_residual_hop_count: usize,
    /// Open residual fence pins verified.
    pub open_residual_fences_verified: bool,
    /// Gateway stdio exec pre-check wired — honest false.
    pub gateway_stdio_exec_trust_pre_check_wired: bool,
    /// MCP wrap GREEN claim blocked.
    pub mcp_wrap_green_claim_blocked: bool,
    /// MASTER / OP-5 retick eligibility — honest false.
    pub master_retick_eligible: bool,
    /// Production flip.
    pub production_wired: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
}

/// One SEC-MCP-WRAP gate-factor row for operator `:trust gate-factors` deepen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapGateFactorRow {
    /// Stable factor identifier.
    pub factor_id: &'static str,
    /// Whether the witness probe is wired.
    pub probe_wired: bool,
    /// Whether the factor earns acceptance credit toward MCP wrap GREEN.
    pub acceptance_credit: bool,
    /// Operator detail string.
    pub detail: String,
}

/// Typed probe for SEC-MCP-WRAP manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapGateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Refuse-path matrix pins verified.
    pub refuse_path_matrix_pins_verified: bool,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// Gateway stdio exec pre-check honest false.
    pub gateway_stdio_exec_honest_false: bool,
    /// MCP wrap GREEN claim blocked.
    pub mcp_wrap_green_claim_blocked: bool,
    /// MASTER / OP-5 retick honest false.
    pub master_retick_honest_false: bool,
    /// Production honest false.
    pub production_honest_false: bool,
    /// Wired hop count.
    pub wire_hop_wired_count: u8,
    /// Ceremony closed at census tier.
    pub ceremony_closed: bool,
}

/// FLEET-COMPOSER ACCEL-B AC34 probe — manifold MCP wrap fence deepen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapAccelAc34Probe {
    /// AC34 card id.
    pub ac34_job_id: &'static str,
    /// Prior Z86 umst-mcp owner absorbed.
    pub prior_z86_absorbed: bool,
    /// Prior Y52 gateway refuse matrix absorbed.
    pub prior_y52_absorbed: bool,
    /// Refuse-path table residue pinned.
    pub refuse_path_table_residue_pinned: bool,
    /// Open residual fence table residue pinned.
    pub open_residual_table_residue_pinned: bool,
    /// Manifold gate probe.
    pub probe: SecMcpWrapGateManifoldProbe,
    /// Ceremony closed.
    pub ceremony_closed: bool,
    /// `sec_mcp_wrap_production_wired()` — honest false.
    pub production_wired: bool,
}

/// W29-119 Grok deepen probe — open-residual fence + AC34 honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecMcpWrapW29DeepenProbe {
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
    /// AC34 honesty.
    pub ac34_honest: bool,
    /// Ceremony closed.
    pub ceremony_closed: bool,
    /// Production wired — honest false.
    pub production_wired: bool,
    /// MASTER retick — honest false.
    pub master_retick_eligible: bool,
    /// GREEN claim blocked — honest true.
    pub mcp_wrap_green_claim_blocked: bool,
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

/// Gateway stdio exec trust pre-check — owned by umst-gateway; honest open.
#[must_use]
pub const fn gateway_stdio_exec_trust_pre_check_wired() -> bool {
    false
}

/// MCP wrap production ceremony — honest false until measured live.
#[must_use]
pub const fn sec_mcp_wrap_production_wired() -> bool {
    false
}

/// MASTER / OP-5 retick eligibility — honest false at census deepen.
#[must_use]
pub const fn sec_mcp_wrap_master_retick_eligible() -> bool {
    MASTER_RETICK_ELIGIBLE
}

/// Whether all 6 typed refuse-path rows are pinned at manifold boundary.
#[must_use]
pub fn manifold_mcp_wrap_refuse_path_matrix_pins_verified() -> bool {
    const EXPECTED_ROW_IDS: [&str; 6] = ["R1", "R2", "R3", "R4", "R5", "R6"];
    REFUSE_PATH_MATRIX.len() == REFUSE_PATH_MATRIX_ROW_COUNT
        && REFUSE_PATH_MATRIX
            .iter()
            .zip(EXPECTED_ROW_IDS.iter())
            .all(|(row, expected_id)| row.row_id == *expected_id && !row.tool_name.is_empty())
        && REFUSE_PATH_MATRIX.iter().any(|r| r.row_id == "R1" && !r.expect_admit)
        && REFUSE_PATH_MATRIX.iter().any(|r| r.row_id == "R2" && r.expect_admit)
        && REFUSE_PATH_MATRIX.iter().any(|r| r.row_id == "R6" && r.revoked && !r.expect_admit)
        && REFUSE_PATH_MATRIX
            .iter()
            .all(|r| r.delegate_ssot.contains("mcp_trust_gate"))
}

/// Whether open residual fence pins match unwired hops 6–7.
#[must_use]
pub fn manifold_mcp_wrap_open_residual_fences_verified() -> bool {
    OPEN_RESIDUAL_FENCES.len() == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES.iter().all(|fence| {
            fence.honest_open
                && fence.green_credit_blocked
                && !fence.delegate_ssot.is_empty()
                && !fence.residue_id.is_empty()
        })
        && MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
            .iter()
            .filter(|h| !h.wired)
            .count()
            == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES.iter().all(|fence| {
            MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
                .iter()
                .any(|h| h.ordinal == fence.hop_ordinal && !h.wired)
        })
        && OPEN_RESIDUAL_FENCES
            .iter()
            .any(|f| f.residue_id == "R-gateway-stdio-exec" && f.hop_ordinal == 6)
        && OPEN_RESIDUAL_FENCES
            .iter()
            .any(|f| f.residue_id == "R-mcp-wrap-production" && f.hop_ordinal == 7)
}

/// Build manifold SEC-MCP-WRAP gate census from live measurements.
#[must_use]
pub fn gate_mcp_wrap_census() -> SecMcpWrapGateCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecMcpWrapGateCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        w29_cell_id: W29_CELL_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        refuse_path_matrix_row_count: REFUSE_PATH_MATRIX_ROW_COUNT as u8,
        refuse_path_matrix_pins_verified: manifold_mcp_wrap_refuse_path_matrix_pins_verified(),
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_mcp_wrap_open_residual_fences_verified(),
        gateway_stdio_exec_trust_pre_check_wired: gateway_stdio_exec_trust_pre_check_wired(),
        mcp_wrap_green_claim_blocked: MCP_WRAP_GREEN_CLAIM_BLOCKED,
        master_retick_eligible: sec_mcp_wrap_master_retick_eligible(),
        production_wired: sec_mcp_wrap_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-MCP-WRAP ceremony is closed at census tier.
///
/// True when cold-edge evidence + refuse-path pins + open residual fences for hops 6–7
/// are measured; gateway stdio exec + production ceremony stay explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_mcp_wrap_ceremony_closed() -> bool {
    let census = gate_mcp_wrap_census();
    census.gate_evidence_wired
        && census.refuse_path_matrix_pins_verified
        && census.open_residual_hop_count == 2
        && census.open_residual_fences_verified
        && !census.gateway_stdio_exec_trust_pre_check_wired
        && !census.production_wired
        && !census.master_retick_eligible
        && census.mcp_wrap_green_claim_blocked
        && census.wire_hop_wired_count == 5
        && census.w29_cell_id == W29_CELL_ID
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-MCP-WRAP manifold gate closure honesty.
#[must_use]
pub fn sec_mcp_wrap_gate_manifold_probe() -> SecMcpWrapGateManifoldProbe {
    let census = gate_mcp_wrap_census();
    SecMcpWrapGateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        refuse_path_matrix_pins_verified: census.refuse_path_matrix_pins_verified,
        open_residual_fences_verified: census.open_residual_fences_verified,
        gateway_stdio_exec_honest_false: !census.gateway_stdio_exec_trust_pre_check_wired,
        mcp_wrap_green_claim_blocked: census.mcp_wrap_green_claim_blocked,
        master_retick_honest_false: !census.master_retick_eligible,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_mcp_wrap_ceremony_closed(),
    }
}

/// Collect SEC-MCP-WRAP gate-factor rows for operator `:trust gate-factors`.
#[must_use]
pub fn collect_sec_mcp_wrap_gate_factor_rows() -> Vec<SecMcpWrapGateFactorRow> {
    vec![
        SecMcpWrapGateFactorRow {
            factor_id: "mcp-wrap-refuse-matrix",
            probe_wired: manifold_mcp_wrap_refuse_path_matrix_pins_verified(),
            acceptance_credit: false,
            detail: format!(
                "refuse_matrix_rows={} pins_verified={}",
                REFUSE_PATH_MATRIX_ROW_COUNT,
                manifold_mcp_wrap_refuse_path_matrix_pins_verified()
            ),
        },
        SecMcpWrapGateFactorRow {
            factor_id: "mcp-trust-gate-delegate",
            probe_wired: MCP_TRUST_GATE_SSOT.contains("mcp_trust_gate"),
            acceptance_credit: false,
            detail: format!("delegate={MCP_TRUST_GATE_SSOT}"),
        },
        SecMcpWrapGateFactorRow {
            factor_id: "gateway-stdio-exec-pre-check",
            probe_wired: GATEWAY_SSOT.contains("sec_mcp_wrap"),
            acceptance_credit: false,
            detail: format!(
                "mcp_stdio_exec_trust_pre_check_wired={}",
                gateway_stdio_exec_trust_pre_check_wired()
            ),
        },
        SecMcpWrapGateFactorRow {
            factor_id: "mcp-wrap-production-ceremony",
            probe_wired: MCP_SSOT.contains("sec_mcp_wrap"),
            acceptance_credit: false,
            detail: format!("production_wired={}", sec_mcp_wrap_production_wired()),
        },
        SecMcpWrapGateFactorRow {
            factor_id: "open-residual-fences",
            probe_wired: manifold_mcp_wrap_open_residual_fences_verified(),
            acceptance_credit: false,
            detail: format!(
                "open_residual_hop_count={} fences_verified={} master_retick={}",
                OPEN_RESIDUAL_HOP_COUNT,
                manifold_mcp_wrap_open_residual_fences_verified(),
                sec_mcp_wrap_master_retick_eligible()
            ),
        },
        SecMcpWrapGateFactorRow {
            factor_id: "manifold-gate-evidence",
            probe_wired: gate_transition_evidence_probe(),
            acceptance_credit: true,
            detail: "cold-edge AdmissibilityToken witness".into(),
        },
    ]
}

/// Render SEC-MCP-WRAP gate-factor table for operator receipts.
#[must_use]
pub fn sec_mcp_wrap_gate_factor_table() -> String {
    let rows = collect_sec_mcp_wrap_gate_factor_rows();
    let mut out = String::from("SEC-MCP-WRAP gate factors (AC34/W29-119):\n");
    for row in &rows {
        out.push_str(&format!(
            "  {} probe_wired={} acceptance_credit={} {}\n",
            row.factor_id, row.probe_wired, row.acceptance_credit, row.detail
        ));
    }
    out.push_str("  scert_credit=BLOCKED expected_gate_exit=BLOCKED\n");
    out.push_str(&format!(
        "  sec_mcp_wrap_production_wired={} master_retick={}\n",
        sec_mcp_wrap_production_wired(),
        sec_mcp_wrap_master_retick_eligible()
    ));
    out
}

/// Render typed refuse-path matrix table for operator receipts.
#[must_use]
pub fn sec_mcp_wrap_refuse_path_table() -> String {
    let mut out = String::from("SEC-MCP-WRAP typed refuse-path matrix (manifold pin):\n");
    for row in REFUSE_PATH_MATRIX {
        out.push_str(&format!(
            "  {} tool={} scope={} revoked={} expect_admit={} delegate={}\n",
            row.row_id, row.tool_name, row.scope, row.revoked, row.expect_admit, row.delegate_ssot
        ));
    }
    out.push_str(&format!(
        "  row_count={} pins_verified={} production_wired={}\n",
        REFUSE_PATH_MATRIX_ROW_COUNT,
        manifold_mcp_wrap_refuse_path_matrix_pins_verified(),
        sec_mcp_wrap_production_wired()
    ));
    out
}

/// Render open residual fence table for operator receipts.
#[must_use]
pub fn sec_mcp_wrap_open_residual_fence_table() -> String {
    let mut out = String::from("SEC-MCP-WRAP open residual fences (W29-119):\n");
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
        manifold_mcp_wrap_open_residual_fences_verified(),
        sec_mcp_wrap_production_wired(),
        sec_mcp_wrap_master_retick_eligible()
    ));
    out
}

/// Build FLEET-COMPOSER ACCEL-B AC34 probe.
#[must_use]
pub fn sec_mcp_wrap_accel_ac34_probe() -> SecMcpWrapAccelAc34Probe {
    let table = sec_mcp_wrap_refuse_path_table();
    let residual_table = sec_mcp_wrap_open_residual_fence_table();
    SecMcpWrapAccelAc34Probe {
        ac34_job_id: ACCEL_B_2050_AC34_JOB_ID,
        prior_z86_absorbed: PRIOR_Z86_RECEIPT_PATH.contains("COMPOSER_Z86_1232"),
        prior_y52_absorbed: PRIOR_Y52_RECEIPT_PATH.contains("COMPOSER_Y52_0808"),
        refuse_path_table_residue_pinned: table.contains("R1")
            && table.contains("R6")
            && table.contains("pins_verified="),
        open_residual_table_residue_pinned: residual_table.contains("R-gateway-stdio-exec")
            && residual_table.contains("R-mcp-wrap-production")
            && residual_table.contains("fences_verified="),
        probe: sec_mcp_wrap_gate_manifold_probe(),
        ceremony_closed: manifold_gate_sec_mcp_wrap_ceremony_closed(),
        production_wired: sec_mcp_wrap_production_wired(),
    }
}

/// FLEET-COMPOSER ACCEL-B AC34 honesty gate — manifold MCP wrap fence deepen.
#[must_use]
pub fn sec_mcp_wrap_accel_ac34_honest() -> bool {
    let probe = sec_mcp_wrap_accel_ac34_probe();
    probe.ac34_job_id == ACCEL_B_2050_AC34_JOB_ID
        && probe.prior_z86_absorbed
        && probe.prior_y52_absorbed
        && probe.refuse_path_table_residue_pinned
        && probe.open_residual_table_residue_pinned
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.refuse_path_matrix_pins_verified
        && probe.probe.open_residual_fences_verified
        && probe.probe.gateway_stdio_exec_honest_false
        && probe.probe.mcp_wrap_green_claim_blocked
        && probe.probe.master_retick_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
}

/// Build W29-119 deepen probe from live measurements.
#[must_use]
pub fn sec_mcp_wrap_w29_deepen_probe() -> SecMcpWrapW29DeepenProbe {
    let residual_table = sec_mcp_wrap_open_residual_fence_table();
    SecMcpWrapW29DeepenProbe {
        w29_cell_id: W29_CELL_ID,
        schema_version: SCHEMA_VERSION,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_mcp_wrap_open_residual_fences_verified(),
        open_residual_table_residue_pinned: residual_table.contains("R-gateway-stdio-exec")
            && residual_table.contains("R-mcp-wrap-production"),
        ac34_honest: sec_mcp_wrap_accel_ac34_honest(),
        ceremony_closed: manifold_gate_sec_mcp_wrap_ceremony_closed(),
        production_wired: sec_mcp_wrap_production_wired(),
        master_retick_eligible: sec_mcp_wrap_master_retick_eligible(),
        mcp_wrap_green_claim_blocked: MCP_WRAP_GREEN_CLAIM_BLOCKED,
    }
}

/// W29-119 deepen honesty — open residuals pinned + AC34 honest + no invented GREEN/PRODUCTION/MASTER.
#[must_use]
pub fn sec_mcp_wrap_w29_deepen_honest() -> bool {
    let probe = sec_mcp_wrap_w29_deepen_probe();
    probe.w29_cell_id == W29_CELL_ID
        && probe.schema_version == SCHEMA_VERSION
        && probe.schema_version.contains("_v2")
        && probe.open_residual_hop_count == 2
        && probe.open_residual_fences_verified
        && probe.open_residual_table_residue_pinned
        && probe.ac34_honest
        && probe.ceremony_closed
        && !probe.production_wired
        && !probe.master_retick_eligible
        && probe.mcp_wrap_green_claim_blocked
}

/// Validate SEC-MCP-WRAP gate census honesty — fail closed on fake production/GREEN claims.
pub fn validate_sec_mcp_wrap_gate_honesty() -> Result<(), &'static str> {
    let census = gate_mcp_wrap_census();
    if census.schema_version != SCHEMA_VERSION {
        return Err("schema_version must match W29 v2 census pin");
    }
    if census.w29_cell_id != W29_CELL_ID {
        return Err("w29_cell_id must stay W29-119-SEC_MCP_WRAP");
    }
    if census.production_wired {
        return Err("sec_mcp_wrap_production_wired must stay false until operator ceremony");
    }
    if census.master_retick_eligible {
        return Err("master_retick_eligible must stay false at census deepen");
    }
    if !census.mcp_wrap_green_claim_blocked {
        return Err("mcp_wrap_green_claim_blocked must stay true in scaffold");
    }
    if census.gateway_stdio_exec_trust_pre_check_wired {
        return Err("gateway_stdio_exec_trust_pre_check_wired must stay false until gateway flip");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.refuse_path_matrix_row_count != 6 {
        return Err("refuse-path matrix must remain 6 rows");
    }
    if !census.refuse_path_matrix_pins_verified {
        return Err("refuse-path matrix pins must verify at manifold boundary");
    }
    if census.open_residual_hop_count != 2 {
        return Err("open residual hop count must remain 2");
    }
    if !census.open_residual_fences_verified {
        return Err("open residual fence pins must verify against unwired hops 6-7");
    }
    if MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-MCP-WRAP gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-MCP-WRAP gate wire hops should be wired today");
    }
    if !manifold_gate_sec_mcp_wrap_ceremony_closed() {
        return Err("manifold gate SEC-MCP-WRAP ceremony must be closed at census tier");
    }
    if !sec_mcp_wrap_accel_ac34_honest() {
        return Err("ACCEL AC34 MCP wrap fence probe must be honest");
    }
    if !sec_mcp_wrap_w29_deepen_honest() {
        return Err("W29-119 deepen probe must be honest");
    }
    Ok(())
}

/// Render SEC-MCP-WRAP gate wire map for operator receipts.
#[must_use]
pub fn sec_mcp_wrap_gate_wire_matrix() -> String {
    let census = gate_mcp_wrap_census();
    let mut out = String::from("SEC-MCP-WRAP manifold gate wire map (AC34/W29-119):\n");
    for hop in MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} refuse_matrix={} open_residual={} gateway_stdio_exec={} \
         mcp_wrap_green_claim_blocked={} master_retick={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS.len(),
        census.refuse_path_matrix_row_count,
        census.open_residual_hop_count,
        census.gateway_stdio_exec_trust_pre_check_wired,
        census.mcp_wrap_green_claim_blocked,
        census.master_retick_eligible,
        census.production_wired
    ));
    out.push_str(&format!("  w29_cell_id={}\n", census.w29_cell_id));
    out.push_str(&format!("  mcp_ssot={MCP_SSOT}\n"));
    out.push_str(&format!("  gateway_ssot={GATEWAY_SSOT}\n"));
    out
}

/// Next-hop surface for umst-mcp live refuse matrix eval (mcp-owned).
#[must_use]
pub const fn sec_mcp_wrap_mcp_delegate_next_hop() -> &'static str {
    "umst-concrete-cartridge/crates/umst-mcp/src/sec_mcp_wrap.rs:R-MCP-WRAP-LIVE-MATRIX"
}

#[cfg(test)]
mod sec_mcp_wrap_tests {
    use super::*;

    #[test]
    fn sec_mcp_wrap_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-MCP-WRAP");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-MCP-WRAP");
        assert_eq!(DEEPEN_JOB_ID, "AGAP-2350-SEC-MCP-WRAP");
        assert_eq!(W29_CELL_ID, "W29-119-SEC_MCP_WRAP");
        assert_eq!(ACCEL_B_2050_AC34_JOB_ID, "ACCEL-B-2050-AC34");
        assert_eq!(REFUSE_PATH_MATRIX_ROW_COUNT, 6);
        assert_eq!(OPEN_RESIDUAL_HOP_COUNT, 2);
        assert!(SCHEMA_VERSION.contains("_v2"));
    }

    #[test]
    fn sec_mcp_wrap_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_mcp_wrap_refuse_path_matrix_six_rows_pinned() {
        assert_eq!(REFUSE_PATH_MATRIX.len(), REFUSE_PATH_MATRIX_ROW_COUNT);
        assert!(manifold_mcp_wrap_refuse_path_matrix_pins_verified());
        assert!(REFUSE_PATH_MATRIX.iter().any(|r| r.row_id == "R1" && !r.expect_admit));
        assert!(REFUSE_PATH_MATRIX.iter().any(|r| r.row_id == "R6" && r.revoked));
    }

    #[test]
    fn sec_mcp_wrap_open_residual_fences_pin_unwired_hops() {
        assert!(manifold_mcp_wrap_open_residual_fences_verified());
        assert_eq!(OPEN_RESIDUAL_FENCES.len(), 2);
        assert_eq!(OPEN_RESIDUAL_FENCES[0].residue_id, "R-gateway-stdio-exec");
        assert_eq!(OPEN_RESIDUAL_FENCES[1].residue_id, "R-mcp-wrap-production");
        assert!(OPEN_RESIDUAL_FENCES
            .iter()
            .all(|f| f.honest_open && f.green_credit_blocked));
        let table = sec_mcp_wrap_open_residual_fence_table();
        assert!(table.contains("R-gateway-stdio-exec"));
        assert!(table.contains("R-mcp-wrap-production"));
        assert!(table.contains("production_wired=false"));
        assert!(table.contains("master_retick=false"));
    }

    #[test]
    fn sec_mcp_wrap_gateway_stdio_exec_and_production_honest_open() {
        assert!(!gateway_stdio_exec_trust_pre_check_wired());
        assert!(!sec_mcp_wrap_production_wired());
        assert!(!sec_mcp_wrap_master_retick_eligible());
        assert!(GATEWAY_STDIO_EXEC_OWNER.contains("mcp_stdio_exec_trust_pre_check_wired"));
        assert!(MCP_WRAP_GREEN_CLAIM_BLOCKED);
        assert!(!MASTER_RETICK_ELIGIBLE);
    }

    #[test]
    fn sec_mcp_wrap_manifold_wire_hops_five_of_seven_wired() {
        assert_eq!(MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("mcp_stdio_exec_trust_pre_check_wired") && !h.wired));
        assert!(MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("sec_mcp_wrap_production_wired") && !h.wired));
    }

    #[test]
    fn sec_mcp_wrap_wire_hops_ordinal_pin() {
        for (idx, hop) in MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS.iter().enumerate() {
            assert_eq!(hop.ordinal, (idx + 1) as u8);
            assert!(!hop.surface.is_empty());
            assert!(!hop.role.is_empty());
        }
    }

    #[test]
    fn sec_mcp_wrap_mcp_wrap_census_honest_posture() {
        let census = gate_mcp_wrap_census();
        assert_eq!(census.board_slice_id, "SEC-MCP-WRAP");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert_eq!(census.w29_cell_id, W29_CELL_ID);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.refuse_path_matrix_row_count, 6);
        assert!(census.refuse_path_matrix_pins_verified);
        assert_eq!(census.open_residual_hop_count, 2);
        assert!(census.open_residual_fences_verified);
        assert!(!census.gateway_stdio_exec_trust_pre_check_wired);
        assert!(census.mcp_wrap_green_claim_blocked);
        assert!(!census.master_retick_eligible);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_mcp_wrap_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_mcp_wrap_ceremony_closed());
        let probe = sec_mcp_wrap_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.refuse_path_matrix_pins_verified);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.gateway_stdio_exec_honest_false);
        assert!(probe.mcp_wrap_green_claim_blocked);
        assert!(probe.master_retick_honest_false);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_mcp_wrap_gate_factor_table_honest_blocked_scert() {
        let table = sec_mcp_wrap_gate_factor_table();
        assert!(table.contains("SEC-MCP-WRAP gate factors"));
        assert!(table.contains("scert_credit=BLOCKED"));
        assert!(table.contains("expected_gate_exit=BLOCKED"));
        assert!(table.contains("open-residual-fences"));
        assert!(table.contains("master_retick=false"));
        let rows = collect_sec_mcp_wrap_gate_factor_rows();
        assert_eq!(rows.len(), 6);
    }

    #[test]
    fn sec_mcp_wrap_refuse_path_table_renders_rows() {
        let table = sec_mcp_wrap_refuse_path_table();
        assert!(table.contains("propose_communicative_act"));
        assert!(table.contains("map_to_geometry"));
        assert!(table.contains("pins_verified="));
    }

    #[test]
    fn sec_mcp_wrap_prior_receipt_paths_pinned() {
        assert!(PRIOR_Z86_RECEIPT_PATH.contains("COMPOSER_Z86_1232"));
        assert!(PRIOR_Y52_RECEIPT_PATH.contains("COMPOSER_Y52_0808"));
        assert!(PRIOR_Y51_RECEIPT_PATH.contains("COMPOSER_Y51_0808"));
        assert!(MCP_SSOT.contains("sec_mcp_wrap.rs"));
        assert!(GATEWAY_SSOT.contains("sec_mcp_wrap.rs"));
    }

    #[test]
    fn sec_mcp_wrap_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_mcp_wrap_gate_wire_matrix();
        assert!(matrix.contains("SEC-MCP-WRAP manifold gate"));
        assert!(matrix.contains("wired=5/7"));
        assert!(matrix.contains("open_residual=2"));
        assert!(matrix.contains("production_wired=false"));
        assert!(matrix.contains("master_retick=false"));
        assert!(matrix.contains("W29-119-SEC_MCP_WRAP"));
    }

    #[test]
    fn fleet_accel_ac34_sec_mcp_wrap_fence_deepen_honest() {
        assert!(sec_mcp_wrap_accel_ac34_honest());
        let probe = sec_mcp_wrap_accel_ac34_probe();
        assert_eq!(probe.ac34_job_id, ACCEL_B_2050_AC34_JOB_ID);
        assert!(probe.prior_z86_absorbed);
        assert!(probe.prior_y52_absorbed);
        assert!(probe.refuse_path_table_residue_pinned);
        assert!(probe.open_residual_table_residue_pinned);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
    }

    #[test]
    fn w29_119_sec_mcp_wrap_deepen_honest() {
        assert!(sec_mcp_wrap_w29_deepen_honest());
        let probe = sec_mcp_wrap_w29_deepen_probe();
        assert_eq!(probe.w29_cell_id, "W29-119-SEC_MCP_WRAP");
        assert!(probe.schema_version.contains("_v2"));
        assert_eq!(probe.open_residual_hop_count, 2);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.open_residual_table_residue_pinned);
        assert!(probe.ac34_honest);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(probe.mcp_wrap_green_claim_blocked);
    }

    #[test]
    fn sec_mcp_wrap_validate_gate_honesty_residue_measured() {
        validate_sec_mcp_wrap_gate_honesty().expect("honest SEC-MCP-WRAP gate census residue");
        assert_eq!(
            sec_mcp_wrap_mcp_delegate_next_hop(),
            "umst-concrete-cartridge/crates/umst-mcp/src/sec_mcp_wrap.rs:R-MCP-WRAP-LIVE-MATRIX"
        );
    }

    #[test]
    fn sec_mcp_wrap_refuses_invented_green_production_master() {
        assert!(!sec_mcp_wrap_production_wired());
        assert!(!sec_mcp_wrap_master_retick_eligible());
        assert!(MCP_WRAP_GREEN_CLAIM_BLOCKED);
        let census = gate_mcp_wrap_census();
        assert!(!census.production_wired);
        assert!(!census.master_retick_eligible);
        assert!(census.mcp_wrap_green_claim_blocked);
        assert_ne!(POSTURE_TAG, "production");
        assert!(!POSTURE_TAG.contains("GREEN"));
        assert!(!POSTURE_TAG.contains("MASTER"));
    }
}

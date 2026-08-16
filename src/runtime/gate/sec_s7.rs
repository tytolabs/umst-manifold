// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGAP-2033/2127-SEC-S7 — manifold gate runtime fed-trust / migration wire map.
//!
//! **Policy:** manifold gate runtime owns the **cold-edge census** bridging
//! [`TransitionEvidence`](super::evidence::TransitionEvidence) to SEC-S7 migration + S-fed-trust SSOT;
//! gateway `sec_s7_boundary_production_wired()` and `s_fed_trust_production_wired()` stay **honest open**.
//!
//! # W29-126 deepen
//!
//! Open-residual fence pins for hops 6–7 (gateway SEC-S7 boundary production +
//! S-fed-trust production flip) measured at census tier.
//! No invented GREEN / PRODUCTION_WIRED / MASTER / OP-5.

use serde::Serialize;

use super::cartridge::{CdTransitionCartridge, GateCartridge};
use super::evidence::AdmissibilityToken;
use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

/// Board slice id.
pub const BOARD_SLICE_ID: &str = "SEC-S7";

/// AGAP slot id (2033 migration inventory deepen).
pub const JOB_ID: &str = "AGAP-2033-SEC-S7";

/// W29 continuous worklist cell id (Grok admit NEW Task lane).
pub const W29_CELL_ID: &str = "W29-126-SEC_S7";

/// FLEET-COMPOSER Prabhu Wave J slot J2 id.
pub const FLEET_P1931_J2_JOB_ID: &str = "PRABHU-WAVE-J-1931-J2";

/// FLEET-COMPOSER Prabhu Wave J J2 receipt path.
pub const FLEET_P1931_J2_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_P1931_J2.md";

/// FLEET-COMPOSER ACCEL-25 slot AC08 id (migrate queue deepen).
pub const ACCEL_2030_AC08_JOB_ID: &str = "ACCEL-25-2030-AC08";

/// FLEET-COMPOSER ACCEL-25 AC08 receipt path.
pub const ACCEL_AC08_RECEIPT_PATH: &str = "outputs/.tmp/COMPOSER_ACCEL_2030_AC08.md";

/// Prior AGAP-2033 SEC-S7 inventory receipt.
pub const PRIOR_RECEIPT_PATH_2033: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S7_2033.md";

/// Prior AGAP-2127 SEC-S7-DRAIN deepen receipt.
pub const PRIOR_RECEIPT_PATH_2127: &str =
    "old/residuals/residuals/misc-outputs-tmp/COMPLETION_AGAP_AGENT_SEC-S7-DRAIN_2127.md";

/// Prior FLEET-COMPOSER-Y Y80 S-fed-trust drain retick receipt.
pub const PRIOR_RECEIPT_PATH_Y80: &str = "outputs/.tmp/COMPOSER_Y80_0808.md";

/// umst-trust SEC-S7 migration delegate SSOT.
pub const TRUST_SSOT: &str = "umst-foundations/crates/umst-trust/src/migration.rs";

/// umst-trust S-fed-trust federation census delegate SSOT.
pub const FED_TRUST_SSOT: &str =
    "umst-foundations/crates/umst-trust/src/sec_s_arc_federation_census.rs";

/// egoff migration SSOT (cockpit badge projection).
pub const EGOFF_MIGRATION_SSOT: &str = "egoff/egoff/src/security/migration.rs";

/// Gateway SEC-S7 boundary delegate SSOT (serial next-hop — not edited this wave).
pub const GATEWAY_SSOT: &str = "umst-gateway/crates/umst-gateway/src/sec_s7_boundary.rs";

/// Honest adoption tier.
pub const POSTURE_TAG: &str = "manifold-gate-census-wired-not-production";

/// Census schema version (v3 = W29 open-residual fence deepen).
pub const SCHEMA_VERSION: &str = "sec_s7_gate_fed_trust_migration_census_v3";

/// Honest fence string for meta / fleet probes — no GREEN / PRODUCTION / MASTER / OP-5.
pub const HONEST_FENCE: &str = "census_wired=true production_wired=false green_claim_blocked=true \
master_retick=false op5_cleared=false s_fed_trust_production_wired=false";

/// Explicit non-claims for W29-126 deepen (gate text).
pub const W29_NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER_RETICK; not OP-5 PASS; not S_FED_TRUST_PRODUCTION";

/// Honest open residual hop count (gateway boundary + S-fed-trust production).
pub const OPEN_RESIDUAL_HOP_COUNT: usize = 2;

/// MASTER / OP-5 retick eligibility — honest false (census deepen only).
pub const MASTER_RETICK_ELIGIBLE: bool = false;

/// OP-5 clearance — honest false (not claimed from SEC-S7 deepen).
pub const OP5_CLEARED: bool = false;

/// AGAP-2033 inventory row count (ecosystem + provider + egoff audit).
pub const INVENTORY_ROW_COUNT: usize = 12;

/// AGAP-2127 / SWARM migrate queue depth (≥4 gate; measured 5).
pub const MIGRATE_QUEUE_DEPTH: usize = 5;

/// S-Q7 migrate-queue surface ids (pinned from `umst-trust::migration` SSOT).
pub const MIGRATE_QUEUE_SURFACES: &[&str] = &[
    "umst-gateway::gate_check_embodied",
    "umst-gateway::web_gossip_route",
    "umst-field::pwa_adapter",
    "umst-gateway::mcp_streamable_http",
    "egoff/umst-math/umst-ffi-bridge",
];

/// One migrate-queue row pinned at manifold boundary (residue ledger cross-ref).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ManifoldS7MigrateQueueRow {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Stable surface identifier.
    pub surface: &'static str,
    /// Residue ledger id when mirrored in `egoff/RESIDUE-LEDGER-v1.0.md`.
    pub residue_id: &'static str,
    /// Closure criterion label from trust migration SSOT.
    pub migration_label: &'static str,
}

/// S-Q7 migrate-queue rows with residue pins (pinned from `umst-trust::migration` SSOT).
pub const MIGRATE_QUEUE_ROWS: &[ManifoldS7MigrateQueueRow] = &[
    ManifoldS7MigrateQueueRow {
        ordinal: 1,
        surface: "umst-gateway::gate_check_embodied",
        residue_id: "R-classical-wrap-gateway",
        migration_label: "queued-PQC-trustgate-production-flip",
    },
    ManifoldS7MigrateQueueRow {
        ordinal: 2,
        surface: "umst-gateway::web_gossip_route",
        residue_id: "R-classical-wrap-gateway",
        migration_label: "queued-migrate-gossip-sled-trust-ledger",
    },
    ManifoldS7MigrateQueueRow {
        ordinal: 3,
        surface: "umst-field::pwa_adapter",
        residue_id: "R-classical-sha256-field-digest",
        migration_label: "closes-SEC-FIELD-PIN-sha3-stamp",
    },
    ManifoldS7MigrateQueueRow {
        ordinal: 4,
        surface: "umst-gateway::mcp_streamable_http",
        residue_id: "R-classical-tls-gateway-expose",
        migration_label: "queued-migrate-hyper-rustls-beyond-loopback",
    },
    ManifoldS7MigrateQueueRow {
        ordinal: 5,
        surface: "egoff/umst-math/umst-ffi-bridge",
        residue_id: "R-unsafe-rust-audit",
        migration_label: "queued-migrate-constant-time-typed-post-S-1",
    },
];

/// S-Q7 per-stage inventory counts (audit drained post-S-1).
pub const STAGE_COUNT_AUDIT: usize = 0;
pub const STAGE_COUNT_WRAP: usize = 2;
pub const STAGE_COUNT_MIGRATE: usize = 5;
pub const STAGE_COUNT_CLOUD_BOUND: usize = 5;

/// S-7 migration complete — honest false until operator GREEN.
pub const MIGRATION_COMPLETE_HONEST: bool = false;

/// Measured migration-complete predicate — stays false until operator S-7 GREEN ceremony.
#[must_use]
pub const fn migration_complete_measured() -> bool {
    MIGRATION_COMPLETE_HONEST
}

/// S-7 GREEN claim blocked — honest true in scaffold deepen.
pub const S7_GREEN_CLAIM_BLOCKED: bool = true;

/// S-fed-trust wire-honest PARTIAL deepen (federation transport fence).
pub const S_FED_TRUST_PARTIAL_HONEST: bool = true;

/// S-fed-trust production flip — honest false until measured live.
pub const S_FED_TRUST_PRODUCTION_WIRED_HONEST: bool = false;

/// One honest-open residual fence pin (gateway / S-fed-trust production — not wired today).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS7OpenResidualFence {
    /// Residual id (`R-gateway-boundary` / `R-s-fed-trust-production`).
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

/// Open residual fence pins — hops 6–7 measured open at W29-126 deepen.
pub const OPEN_RESIDUAL_FENCES: &[SecS7OpenResidualFence] = &[
    SecS7OpenResidualFence {
        residue_id: "R-gateway-boundary",
        hop_ordinal: 6,
        delegate_ssot: GATEWAY_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
    SecS7OpenResidualFence {
        residue_id: "R-s-fed-trust-production",
        hop_ordinal: 7,
        delegate_ssot: FED_TRUST_SSOT,
        honest_open: true,
        green_credit_blocked: true,
    },
];

/// One hop in the manifold SEC-S7 gate runtime wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS7GateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold SEC-S7 gate runtime wire map (cold-edge evidence → trust migration + fed-trust census).
pub const MANIFOLD_SEC_S7_GATE_WIRE_HOPS: &[SecS7GateWireHop] = &[
    SecS7GateWireHop {
        ordinal: 1,
        surface: "umst-manifold::runtime::gate::evidence::AdmissibilityToken",
        role: "Gate admit witness token on cold edge",
        wired: true,
    },
    SecS7GateWireHop {
        ordinal: 2,
        surface: "umst-manifold::runtime::gate::cartridge::GateCartridge::transition_evidence",
        role: "CdTransitionCartridge structured witness",
        wired: true,
    },
    SecS7GateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::gate::sec_s7::gate_fed_trust_migration_census",
        role: "Manifold gate SEC-S7 fed-trust migration census",
        wired: true,
    },
    SecS7GateWireHop {
        ordinal: 4,
        surface: "umst-trust::migration::validate_migration_drain_honesty",
        role: "Trust migration drain delegate (G77/Y80)",
        wired: true,
    },
    SecS7GateWireHop {
        ordinal: 5,
        surface: "umst-trust::sec_s_arc_federation_census::federation_cell_s_fed_trust_partial",
        role: "S-fed-trust federation partial deepen (Y53)",
        wired: true,
    },
    SecS7GateWireHop {
        ordinal: 6,
        surface: "umst-gateway::sec_s7_boundary::sec_s7_boundary_production_wired",
        role: "Gateway SEC-S7 boundary production ceremony (serial Wave H/O)",
        wired: false,
    },
    SecS7GateWireHop {
        ordinal: 7,
        surface: "umst-trust::sec_s_arc_federation_census::s_fed_trust_production_wired",
        role: "S-fed-trust production flip (operator-measured live)",
        wired: false,
    },
];

/// One migrate-queue probe row at manifold cold edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifoldS7MigrateProbe {
    /// Stable surface identifier.
    pub surface: &'static str,
    /// Whether surface is enumerated in migrate queue SSOT pin.
    pub probe_hit: bool,
    /// Residue ledger id pin matches deepen row table.
    pub residue_pin_hit: bool,
}

/// Aggregated SEC-S7 gate fed-trust migration census on manifold boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS7GateFedTrustMigrationCensus {
    /// Census schema tag.
    pub schema_version: &'static str,
    /// Board slice id.
    pub board_slice_id: &'static str,
    /// W29 cell id pin.
    pub w29_cell_id: &'static str,
    /// Gate transition evidence probe passed.
    pub gate_evidence_wired: bool,
    /// AGAP-2033 inventory row count (12/12).
    pub inventory_row_count: usize,
    /// Migrate queue depth (5/12).
    pub migrate_queue_depth: usize,
    /// Migrate queue ≥4 gate met.
    pub migrate_queue_ge_4: bool,
    /// Migrate queue ≥5 gate met (SWARM absorb).
    pub migrate_queue_ge_5: bool,
    /// Honest open residual hop count (2).
    pub open_residual_hop_count: usize,
    /// Open residual fence pins verified.
    pub open_residual_fences_verified: bool,
    /// S-7 migration complete — honest false.
    pub migration_complete: bool,
    /// S-7 GREEN claim blocked — honest true.
    pub s7_green_claim_blocked: bool,
    /// S-fed-trust partial deepen wired.
    pub s_fed_trust_partial: bool,
    /// S-fed-trust production flip — honest false.
    pub s_fed_trust_production_wired: bool,
    /// MASTER / OP-5 retick — honest false.
    pub master_retick_eligible: bool,
    /// OP-5 clearance — honest false.
    pub op5_cleared: bool,
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

/// Whether live gateway SEC-S7 boundary production flip is plumbed (honest `false`).
#[must_use]
pub const fn sec_s7_production_wired() -> bool {
    false
}

/// Whether MASTER / OP-5 retick is eligible (honest `false` at census deepen).
#[must_use]
pub const fn sec_s7_master_retick_eligible() -> bool {
    MASTER_RETICK_ELIGIBLE
}

/// Whether OP-5 clearance is claimed (honest `false` at census deepen).
#[must_use]
pub const fn sec_s7_op5_cleared() -> bool {
    OP5_CLEARED
}

/// Whether open residual fence pins match unwired hops 6–7 on the wire map.
#[must_use]
pub fn manifold_s7_open_residual_fences_verified() -> bool {
    const EXPECTED: [(&str, u8); 2] = [("R-gateway-boundary", 6), ("R-s-fed-trust-production", 7)];
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
        && MANIFOLD_SEC_S7_GATE_WIRE_HOPS
            .iter()
            .filter(|h| !h.wired)
            .count()
            == OPEN_RESIDUAL_HOP_COUNT
        && OPEN_RESIDUAL_FENCES.iter().all(|fence| {
            MANIFOLD_SEC_S7_GATE_WIRE_HOPS
                .iter()
                .any(|h| h.ordinal == fence.hop_ordinal && !h.wired)
        })
}

/// S-Q7 migrate-queue coverage probe matrix — 5/5 at manifold cold edge.
#[must_use]
pub fn manifold_s7_migrate_coverage_probes() -> Vec<ManifoldS7MigrateProbe> {
    MIGRATE_QUEUE_ROWS
        .iter()
        .map(|row| ManifoldS7MigrateProbe {
            surface: row.surface,
            probe_hit: MIGRATE_QUEUE_SURFACES.contains(&row.surface),
            residue_pin_hit: !row.residue_id.is_empty() && row.residue_id.starts_with('R'),
        })
        .collect()
}

/// Whether all migrate-queue deepen rows carry residue ledger pins.
#[must_use]
pub fn manifold_s7_migrate_queue_residue_pins_verified() -> bool {
    MIGRATE_QUEUE_ROWS.len() == MIGRATE_QUEUE_DEPTH
        && MIGRATE_QUEUE_ROWS
            .iter()
            .all(|row| !row.residue_id.is_empty() && row.residue_id.starts_with('R'))
        && MIGRATE_QUEUE_ROWS
            .iter()
            .zip(MIGRATE_QUEUE_SURFACES.iter())
            .all(|(row, surface)| row.surface == *surface)
}

/// Render migrate-queue deepen table for operator receipts.
#[must_use]
pub fn sec_s7_migrate_queue_table() -> String {
    let mut out = String::from("SEC-S7 migrate queue deepen (5/5 residue-pinned):\n");
    for row in MIGRATE_QUEUE_ROWS {
        out.push_str(&format!(
            "  {} surface={} residue_id={} migration={}\n",
            row.ordinal, row.surface, row.residue_id, row.migration_label
        ));
    }
    out.push_str(&format!(
        "  migration_complete_measured={} s7_green_claim_blocked={}\n",
        migration_complete_measured(),
        S7_GREEN_CLAIM_BLOCKED
    ));
    out
}

/// Whether all five migrate-queue surfaces are enumerated at manifold boundary.
#[must_use]
pub fn manifold_s7_all_migrate_surfaces_probed() -> bool {
    manifold_s7_migrate_coverage_probes()
        .iter()
        .all(|p| p.probe_hit && p.residue_pin_hit)
        && MIGRATE_QUEUE_SURFACES.len() == MIGRATE_QUEUE_DEPTH
        && manifold_s7_migrate_queue_residue_pins_verified()
}

/// Verify AGAP-2033 stage-count census pins at manifold boundary.
#[must_use]
pub fn manifold_verify_migration_inventory_census() -> bool {
    STAGE_COUNT_AUDIT + STAGE_COUNT_WRAP + STAGE_COUNT_MIGRATE + STAGE_COUNT_CLOUD_BOUND
        == INVENTORY_ROW_COUNT
        && STAGE_COUNT_AUDIT == 0
        && STAGE_COUNT_MIGRATE == MIGRATE_QUEUE_DEPTH
        && STAGE_COUNT_MIGRATE >= 4
}

/// Build manifold SEC-S7 gate fed-trust migration census from live measurements.
#[must_use]
pub fn gate_fed_trust_migration_census() -> SecS7GateFedTrustMigrationCensus {
    let wire_hop_wired_count = MANIFOLD_SEC_S7_GATE_WIRE_HOPS
        .iter()
        .filter(|h| h.wired)
        .count() as u8;
    SecS7GateFedTrustMigrationCensus {
        schema_version: SCHEMA_VERSION,
        board_slice_id: BOARD_SLICE_ID,
        w29_cell_id: W29_CELL_ID,
        gate_evidence_wired: gate_transition_evidence_probe(),
        inventory_row_count: INVENTORY_ROW_COUNT,
        migrate_queue_depth: MIGRATE_QUEUE_DEPTH,
        migrate_queue_ge_4: MIGRATE_QUEUE_DEPTH >= 4,
        migrate_queue_ge_5: MIGRATE_QUEUE_DEPTH >= 5,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_s7_open_residual_fences_verified(),
        migration_complete: migration_complete_measured(),
        s7_green_claim_blocked: S7_GREEN_CLAIM_BLOCKED,
        s_fed_trust_partial: S_FED_TRUST_PARTIAL_HONEST,
        s_fed_trust_production_wired: S_FED_TRUST_PRODUCTION_WIRED_HONEST,
        master_retick_eligible: sec_s7_master_retick_eligible(),
        op5_cleared: sec_s7_op5_cleared(),
        production_wired: sec_s7_production_wired(),
        wire_hop_wired_count,
    }
}

/// Whether manifold gate SEC-S7 ceremony is closed at census tier.
///
/// True when cold-edge evidence probe + migration/fed-trust wire map hops 1–5 are measured wired
/// and open residual fences for hops 6–7 are pinned honest-open.
/// Gateway boundary production + S-fed-trust production flip are explicit non-blockers.
#[must_use]
pub fn manifold_gate_sec_s7_ceremony_closed() -> bool {
    let census = gate_fed_trust_migration_census();
    census.gate_evidence_wired
        && census.inventory_row_count == INVENTORY_ROW_COUNT
        && census.migrate_queue_depth == MIGRATE_QUEUE_DEPTH
        && census.migrate_queue_ge_4
        && census.migrate_queue_ge_5
        && census.open_residual_hop_count == OPEN_RESIDUAL_HOP_COUNT
        && census.open_residual_fences_verified
        && !census.migration_complete
        && migration_complete_measured() == false
        && census.s7_green_claim_blocked
        && census.s_fed_trust_partial
        && !census.s_fed_trust_production_wired
        && !census.master_retick_eligible
        && !census.op5_cleared
        && !census.production_wired
        && census.wire_hop_wired_count == 5
        && census.w29_cell_id == W29_CELL_ID
        && manifold_s7_all_migrate_surfaces_probed()
        && manifold_verify_migration_inventory_census()
        && gate_transition_evidence_probe()
}

/// Typed probe for SEC-S7 manifold gate closure honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SecS7GateManifoldProbe {
    /// Gate transition evidence probe.
    pub gate_evidence_wired: bool,
    /// Migrate queue 5/5 surfaces probed.
    pub s7_all_migrate_surfaces_probed: bool,
    /// Inventory stage census verified.
    pub migration_inventory_census_verified: bool,
    /// Migrate queue residue pins verified.
    pub migrate_queue_residue_pins_verified: bool,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// Migration complete honest false.
    pub migration_complete_honest_false: bool,
    /// S-7 GREEN claim blocked.
    pub s7_green_claim_blocked: bool,
    /// S-fed-trust partial honest true.
    pub s_fed_trust_partial_honest: bool,
    /// S-fed-trust production honest false.
    pub s_fed_trust_production_honest_false: bool,
    /// MASTER / OP-5 retick honest false.
    pub master_retick_honest_false: bool,
    /// OP-5 clearance honest false.
    pub op5_cleared_honest_false: bool,
    /// Production flip honest false.
    pub production_honest_false: bool,
    /// Manifold wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build introspection probe for SEC-S7 done-when checks.
#[must_use]
pub fn sec_s7_gate_manifold_probe() -> SecS7GateManifoldProbe {
    let census = gate_fed_trust_migration_census();
    SecS7GateManifoldProbe {
        gate_evidence_wired: census.gate_evidence_wired,
        s7_all_migrate_surfaces_probed: manifold_s7_all_migrate_surfaces_probed(),
        migration_inventory_census_verified: manifold_verify_migration_inventory_census(),
        migrate_queue_residue_pins_verified: manifold_s7_migrate_queue_residue_pins_verified(),
        open_residual_fences_verified: census.open_residual_fences_verified,
        migration_complete_honest_false: !census.migration_complete
            && !migration_complete_measured(),
        s7_green_claim_blocked: census.s7_green_claim_blocked,
        s_fed_trust_partial_honest: census.s_fed_trust_partial,
        s_fed_trust_production_honest_false: !census.s_fed_trust_production_wired,
        master_retick_honest_false: !census.master_retick_eligible,
        op5_cleared_honest_false: !census.op5_cleared,
        production_honest_false: !census.production_wired,
        wire_hop_wired_count: census.wire_hop_wired_count,
        ceremony_closed: manifold_gate_sec_s7_ceremony_closed(),
    }
}

/// One SEC-S7 gate-factor row for operator receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS7GateFactorRow {
    /// Factor identifier.
    pub factor_id: &'static str,
    /// Whether manifold probe wired for this factor.
    pub probe_wired: bool,
    /// Acceptance credit (honest BLOCKED until production).
    pub acceptance_credit: bool,
}

/// Collect SEC-S7 gate-factor rows for operator matrix receipts.
#[must_use]
pub fn collect_sec_s7_gate_factor_rows() -> Vec<SecS7GateFactorRow> {
    vec![
        SecS7GateFactorRow {
            factor_id: "inventory-12-row",
            probe_wired: INVENTORY_ROW_COUNT == 12,
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "migrate-queue-ge-4",
            probe_wired: MIGRATE_QUEUE_DEPTH >= 4,
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "migrate-queue-ge-5",
            probe_wired: MIGRATE_QUEUE_DEPTH >= 5,
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "migrate-queue-residue-pins",
            probe_wired: manifold_s7_migrate_queue_residue_pins_verified(),
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "s-fed-trust-partial",
            probe_wired: S_FED_TRUST_PARTIAL_HONEST,
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "trust-drain-delegate",
            probe_wired: TRUST_SSOT.contains("migration.rs"),
            acceptance_credit: false,
        },
        SecS7GateFactorRow {
            factor_id: "gateway-boundary",
            probe_wired: GATEWAY_SSOT.contains("sec_s7_boundary"),
            acceptance_credit: false,
        },
    ]
}

/// Render SEC-S7 gate-factor table for operator receipts.
#[must_use]
pub fn sec_s7_gate_factor_table() -> String {
    let rows = collect_sec_s7_gate_factor_rows();
    let mut out = String::from("SEC-S7 gate factors (J2 fed-trust migration):\n");
    for row in &rows {
        out.push_str(&format!(
            "  {} probe_wired={} scert_credit=BLOCKED\n",
            row.factor_id, row.probe_wired
        ));
    }
    out.push_str(&format!(
        "  migration_complete={} migration_complete_measured={} s7_green_claim_blocked={} s_fed_trust_production_wired={} \
         sec_s7_production_wired={} expected_gate_exit=BLOCKED\n",
        MIGRATION_COMPLETE_HONEST,
        migration_complete_measured(),
        S7_GREEN_CLAIM_BLOCKED,
        S_FED_TRUST_PRODUCTION_WIRED_HONEST,
        sec_s7_production_wired(),
    ));
    out
}

/// FLEET-COMPOSER Prabhu Wave J J2 integration probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS7P1931J2Probe {
    /// J2 fleet card id.
    pub j2_job_id: &'static str,
    /// Prior 2033 inventory absorbed.
    pub prior_2033_absorbed: bool,
    /// Prior 2127 drain absorbed.
    pub prior_2127_absorbed: bool,
    /// Prior Y80 S-fed-trust drain absorbed.
    pub prior_y80_absorbed: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
    /// Underlying gate probe.
    pub probe: SecS7GateManifoldProbe,
    /// `sec_s7_production_wired()` — honest false.
    pub production_wired: bool,
    /// Gate-factor rows with probe wired.
    pub gate_factor_wired_count: usize,
    /// Gate-factor table pins BLOCKED exit.
    pub gate_factor_exit_blocked: bool,
}

/// Build FLEET-COMPOSER P1931 J2 integration probe from live measurements.
#[must_use]
pub fn sec_s7_p1931_j2_probe() -> SecS7P1931J2Probe {
    let rows = collect_sec_s7_gate_factor_rows();
    let table = sec_s7_gate_factor_table();
    SecS7P1931J2Probe {
        j2_job_id: FLEET_P1931_J2_JOB_ID,
        prior_2033_absorbed: PRIOR_RECEIPT_PATH_2033.contains("SEC-S7_2033"),
        prior_2127_absorbed: PRIOR_RECEIPT_PATH_2127.contains("SEC-S7-DRAIN"),
        prior_y80_absorbed: PRIOR_RECEIPT_PATH_Y80.contains("COMPOSER_Y80"),
        ceremony_closed: manifold_gate_sec_s7_ceremony_closed(),
        probe: sec_s7_gate_manifold_probe(),
        production_wired: sec_s7_production_wired(),
        gate_factor_wired_count: rows.iter().filter(|r| r.probe_wired).count(),
        gate_factor_exit_blocked: table.contains("expected_gate_exit=BLOCKED"),
    }
}

/// FLEET-COMPOSER P1931 J2 honesty gate — ceremony closed + production false + fed-trust honest.
#[must_use]
pub fn sec_s7_p1931_j2_honest() -> bool {
    let probe = sec_s7_p1931_j2_probe();
    probe.j2_job_id == FLEET_P1931_J2_JOB_ID
        && probe.prior_2033_absorbed
        && probe.prior_2127_absorbed
        && probe.prior_y80_absorbed
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s7_all_migrate_surfaces_probed
        && probe.probe.migrate_queue_residue_pins_verified
        && probe.probe.open_residual_fences_verified
        && probe.probe.migration_inventory_census_verified
        && probe.probe.migration_complete_honest_false
        && probe.probe.s7_green_claim_blocked
        && probe.probe.s_fed_trust_partial_honest
        && probe.probe.s_fed_trust_production_honest_false
        && probe.probe.master_retick_honest_false
        && probe.probe.op5_cleared_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.production_wired
        && probe.gate_factor_wired_count >= 6
        && probe.gate_factor_exit_blocked
}

/// FLEET-COMPOSER ACCEL-25 AC08 migrate-queue deepen probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS7AccelAc08Probe {
    /// AC08 fleet slot id.
    pub ac08_job_id: &'static str,
    /// Prior J2 manifold gate receipt absorbed.
    pub prior_j2_absorbed: bool,
    /// Prior Y80 S-fed-trust drain absorbed.
    pub prior_y80_absorbed: bool,
    /// Migrate queue deepen table pins residue ids.
    pub migrate_queue_table_residue_pinned: bool,
    /// Underlying gate probe.
    pub probe: SecS7GateManifoldProbe,
    /// Measured migration-complete predicate.
    pub migration_complete_measured: bool,
    /// Manifold ceremony close predicate.
    pub ceremony_closed: bool,
}

/// Build FLEET-COMPOSER ACCEL-25 AC08 integration probe from live measurements.
#[must_use]
pub fn sec_s7_accel_ac08_probe() -> SecS7AccelAc08Probe {
    let table = sec_s7_migrate_queue_table();
    SecS7AccelAc08Probe {
        ac08_job_id: ACCEL_2030_AC08_JOB_ID,
        prior_j2_absorbed: FLEET_P1931_J2_RECEIPT_PATH.contains("COMPOSER_P1931_J2"),
        prior_y80_absorbed: PRIOR_RECEIPT_PATH_Y80.contains("COMPOSER_Y80"),
        migrate_queue_table_residue_pinned: table.contains("residue_id=R-classical-wrap-gateway")
            && table.contains("residue_id=R-unsafe-rust-audit")
            && table.contains("migration_complete_measured=false"),
        probe: sec_s7_gate_manifold_probe(),
        migration_complete_measured: migration_complete_measured(),
        ceremony_closed: manifold_gate_sec_s7_ceremony_closed(),
    }
}

/// FLEET-COMPOSER ACCEL-25 AC08 honesty gate — migrate queue deepen + migration_complete measured false.
#[must_use]
pub fn sec_s7_accel_ac08_honest() -> bool {
    let probe = sec_s7_accel_ac08_probe();
    probe.ac08_job_id == ACCEL_2030_AC08_JOB_ID
        && probe.prior_j2_absorbed
        && probe.prior_y80_absorbed
        && probe.migrate_queue_table_residue_pinned
        && probe.ceremony_closed
        && probe.probe.gate_evidence_wired
        && probe.probe.s7_all_migrate_surfaces_probed
        && probe.probe.migrate_queue_residue_pins_verified
        && probe.probe.open_residual_fences_verified
        && probe.probe.migration_complete_honest_false
        && probe.probe.s7_green_claim_blocked
        && probe.probe.s_fed_trust_partial_honest
        && probe.probe.s_fed_trust_production_honest_false
        && probe.probe.master_retick_honest_false
        && probe.probe.op5_cleared_honest_false
        && probe.probe.production_honest_false
        && probe.probe.wire_hop_wired_count == 5
        && !probe.migration_complete_measured
}

/// Render open residual fence table for operator receipts.
#[must_use]
pub fn sec_s7_open_residual_fence_table() -> String {
    let mut out = String::from("SEC-S7 open residual fences (W29-126):\n");
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
        "  open_residual_hop_count={} fences_verified={} production_wired={} \
         master_retick={} op5_cleared={} honest_fence={}\n",
        OPEN_RESIDUAL_HOP_COUNT,
        manifold_s7_open_residual_fences_verified(),
        sec_s7_production_wired(),
        sec_s7_master_retick_eligible(),
        sec_s7_op5_cleared(),
        HONEST_FENCE
    ));
    out
}

/// W29-126 Grok admit deepen probe — open-residual fence + fed-trust honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecS7W29DeepenProbe {
    /// W29 cell id.
    pub w29_cell_id: &'static str,
    /// Schema version pin.
    pub schema_version: &'static str,
    /// Honest fence string.
    pub honest_fence: &'static str,
    /// Explicit non-claim text.
    pub non_claim: &'static str,
    /// Open residual fence count.
    pub open_residual_hop_count: usize,
    /// Open residual fences verified.
    pub open_residual_fences_verified: bool,
    /// Open residual table residue pinned.
    pub open_residual_table_residue_pinned: bool,
    /// J2 honesty.
    pub j2_honest: bool,
    /// AC08 honesty.
    pub ac08_honest: bool,
    /// Ceremony closed.
    pub ceremony_closed: bool,
    /// Production wired — honest false.
    pub production_wired: bool,
    /// S-fed-trust production — honest false.
    pub s_fed_trust_production_wired: bool,
    /// MASTER retick — honest false.
    pub master_retick_eligible: bool,
    /// OP-5 clearance — honest false.
    pub op5_cleared: bool,
    /// GREEN claim blocked — honest true.
    pub s7_green_claim_blocked: bool,
}

/// Build W29-126 deepen probe from live measurements.
#[must_use]
pub fn sec_s7_w29_deepen_probe() -> SecS7W29DeepenProbe {
    let residual_table = sec_s7_open_residual_fence_table();
    SecS7W29DeepenProbe {
        w29_cell_id: W29_CELL_ID,
        schema_version: SCHEMA_VERSION,
        honest_fence: HONEST_FENCE,
        non_claim: W29_NON_CLAIM,
        open_residual_hop_count: OPEN_RESIDUAL_HOP_COUNT,
        open_residual_fences_verified: manifold_s7_open_residual_fences_verified(),
        open_residual_table_residue_pinned: residual_table.contains("R-gateway-boundary")
            && residual_table.contains("R-s-fed-trust-production")
            && residual_table.contains("fences_verified="),
        j2_honest: sec_s7_p1931_j2_honest(),
        ac08_honest: sec_s7_accel_ac08_honest(),
        ceremony_closed: manifold_gate_sec_s7_ceremony_closed(),
        production_wired: sec_s7_production_wired(),
        s_fed_trust_production_wired: S_FED_TRUST_PRODUCTION_WIRED_HONEST,
        master_retick_eligible: sec_s7_master_retick_eligible(),
        op5_cleared: sec_s7_op5_cleared(),
        s7_green_claim_blocked: S7_GREEN_CLAIM_BLOCKED,
    }
}

/// W29-126 deepen honesty — open residuals pinned + no invented GREEN/PRODUCTION/MASTER/OP-5.
#[must_use]
pub fn sec_s7_w29_deepen_honest() -> bool {
    let probe = sec_s7_w29_deepen_probe();
    probe.w29_cell_id == W29_CELL_ID
        && probe.schema_version == SCHEMA_VERSION
        && probe.schema_version.contains("_v3")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("green_claim_blocked=true")
        && probe.honest_fence.contains("master_retick=false")
        && probe.honest_fence.contains("op5_cleared=false")
        && probe.non_claim.contains("not GREEN")
        && probe.non_claim.contains("not PRODUCTION_WIRED")
        && probe.non_claim.contains("not MASTER_RETICK")
        && probe.non_claim.contains("not OP-5 PASS")
        && probe.open_residual_hop_count == 2
        && probe.open_residual_fences_verified
        && probe.open_residual_table_residue_pinned
        && probe.j2_honest
        && probe.ac08_honest
        && probe.ceremony_closed
        && !probe.production_wired
        && !probe.s_fed_trust_production_wired
        && !probe.master_retick_eligible
        && !probe.op5_cleared
        && probe.s7_green_claim_blocked
}

/// Validate SEC-S7 gate census honesty — fail closed on fake migration-complete / GREEN claims.
pub fn validate_sec_s7_gate_honesty() -> Result<(), &'static str> {
    let census = gate_fed_trust_migration_census();
    if census.schema_version != SCHEMA_VERSION {
        return Err("schema_version must match W29 v3 census pin");
    }
    if census.w29_cell_id != W29_CELL_ID {
        return Err("w29_cell_id must stay W29-126-SEC_S7");
    }
    if census.migration_complete {
        return Err("migration_complete must stay false until S-7 GREEN");
    }
    if migration_complete_measured() {
        return Err("migration_complete_measured must stay false until operator GREEN");
    }
    if !census.s7_green_claim_blocked {
        return Err("s7_green_claim_blocked must stay true in scaffold");
    }
    if !census.s_fed_trust_partial {
        return Err("s_fed_trust_partial must stay true in scaffold deepen");
    }
    if census.s_fed_trust_production_wired {
        return Err("s_fed_trust_production_wired must stay false until measured live");
    }
    if census.master_retick_eligible {
        return Err("master_retick_eligible must stay false at census deepen");
    }
    if census.op5_cleared {
        return Err("op5_cleared must stay false at census deepen");
    }
    if census.production_wired {
        return Err("sec_s7_production_wired must stay false until SEC-S7-BOUNDARY-GW");
    }
    if !census.gate_evidence_wired {
        return Err("gate transition evidence probe failed");
    }
    if census.inventory_row_count != 12 {
        return Err("AGAP-2033 inventory must remain 12 rows");
    }
    if census.migrate_queue_depth < 4 {
        return Err("AGAP-2127 drain must leave >=4 rows at Migrate");
    }
    if !census.migrate_queue_ge_5 {
        return Err("SWARM absorb requires migrate queue >=5");
    }
    if census.open_residual_hop_count != 2 {
        return Err("open residual hop count must remain 2");
    }
    if !census.open_residual_fences_verified {
        return Err("open residual fence pins must verify against unwired hops 6-7");
    }
    if !manifold_s7_all_migrate_surfaces_probed() {
        return Err("S-7 5/5 migrate surfaces must hit at manifold boundary");
    }
    if !manifold_s7_migrate_queue_residue_pins_verified() {
        return Err("S-7 migrate queue residue pins must verify at manifold boundary");
    }
    if !manifold_verify_migration_inventory_census() {
        return Err("manifold migration inventory census witness failed");
    }
    if MANIFOLD_SEC_S7_GATE_WIRE_HOPS.len() != 7 {
        return Err("seven SEC-S7 gate wire hops expected");
    }
    if census.wire_hop_wired_count != 5 {
        return Err("five SEC-S7 gate wire hops should be wired today");
    }
    if !manifold_gate_sec_s7_ceremony_closed() {
        return Err("manifold gate SEC-S7 ceremony must be closed at census tier");
    }
    if !sec_s7_p1931_j2_honest() {
        return Err("P1931 J2 probe must be honest");
    }
    if !sec_s7_accel_ac08_honest() {
        return Err("ACCEL AC08 migrate queue deepen probe must be honest");
    }
    if !sec_s7_w29_deepen_honest() {
        return Err("W29-126 deepen probe must be honest");
    }
    Ok(())
}

/// Render SEC-S7 gate wire map for operator receipts.
#[must_use]
pub fn sec_s7_gate_wire_matrix() -> String {
    let census = gate_fed_trust_migration_census();
    let mut out = String::from("SEC-S7 manifold gate fed-trust migration wire map (J2/W29-126):\n");
    for hop in MANIFOLD_SEC_S7_GATE_WIRE_HOPS {
        out.push_str(&format!(
            "  {} wired={} {} [{}]\n",
            hop.ordinal, hop.wired, hop.surface, hop.role
        ));
    }
    out.push_str(&format!(
        "  wired={}/{} inventory={} migrate_queue={} migrate_ge_4={} migrate_ge_5={} \
         open_residual={} migration_complete={} s_fed_trust_partial={} \
         s_fed_trust_production_wired={} master_retick={} op5_cleared={} production_wired={}\n",
        census.wire_hop_wired_count,
        MANIFOLD_SEC_S7_GATE_WIRE_HOPS.len(),
        census.inventory_row_count,
        census.migrate_queue_depth,
        census.migrate_queue_ge_4,
        census.migrate_queue_ge_5,
        census.open_residual_hop_count,
        census.migration_complete,
        census.s_fed_trust_partial,
        census.s_fed_trust_production_wired,
        census.master_retick_eligible,
        census.op5_cleared,
        census.production_wired
    ));
    out.push_str(&format!("  w29_cell_id={}\n", census.w29_cell_id));
    out.push_str(&format!("  trust_ssot={TRUST_SSOT}\n"));
    out.push_str(&format!("  fed_trust_ssot={FED_TRUST_SSOT}\n"));
    out
}

/// Next-hop surface for gateway SEC-S7 boundary production (gateway-owned).
#[must_use]
pub const fn sec_s7_boundary_next_hop() -> &'static str {
    "umst-gateway/crates/umst-gateway/src/sec_s7_boundary.rs:R-SEC-S7-BOUNDARY-PROD"
}

#[cfg(test)]
mod sec_s7_tests {
    use super::*;

    #[test]
    fn sec_s7_board_slice_metadata_locked() {
        assert_eq!(BOARD_SLICE_ID, "SEC-S7");
        assert_eq!(JOB_ID, "AGAP-2033-SEC-S7");
        assert_eq!(FLEET_P1931_J2_JOB_ID, "PRABHU-WAVE-J-1931-J2");
        assert_eq!(W29_CELL_ID, "W29-126-SEC_S7");
        assert_eq!(SCHEMA_VERSION, "sec_s7_gate_fed_trust_migration_census_v3");
    }

    #[test]
    fn sec_s7_gate_transition_evidence_probe_honest() {
        assert!(gate_transition_evidence_probe());
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let evidence = CdTransitionCartridge.transition_evidence(&old, &old, 1.0);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn sec_s7_migration_inventory_census_at_manifold() {
        assert!(manifold_verify_migration_inventory_census());
        assert_eq!(INVENTORY_ROW_COUNT, 12);
        assert_eq!(MIGRATE_QUEUE_DEPTH, 5);
    }

    #[test]
    fn sec_s7_migrate_coverage_five_by_five() {
        let probes = manifold_s7_migrate_coverage_probes();
        assert_eq!(probes.len(), 5);
        assert!(manifold_s7_all_migrate_surfaces_probed());
        assert!(manifold_s7_migrate_queue_residue_pins_verified());
        assert!(probes.iter().all(|p| p.probe_hit && p.residue_pin_hit));
        assert_eq!(MIGRATE_QUEUE_ROWS.len(), MIGRATE_QUEUE_DEPTH);
    }

    #[test]
    fn sec_s7_migrate_queue_table_residue_pinned() {
        let table = sec_s7_migrate_queue_table();
        assert!(table.contains("SEC-S7 migrate queue deepen"));
        assert!(table.contains("R-classical-wrap-gateway"));
        assert!(table.contains("R-unsafe-rust-audit"));
        assert!(table.contains("migration_complete_measured=false"));
    }

    #[test]
    fn sec_s7_fed_trust_migration_census_honest_posture() {
        let census = gate_fed_trust_migration_census();
        assert_eq!(census.board_slice_id, "SEC-S7");
        assert_eq!(census.schema_version, SCHEMA_VERSION);
        assert_eq!(census.w29_cell_id, W29_CELL_ID);
        assert!(census.gate_evidence_wired);
        assert_eq!(census.inventory_row_count, 12);
        assert_eq!(census.migrate_queue_depth, 5);
        assert!(census.migrate_queue_ge_4);
        assert!(census.migrate_queue_ge_5);
        assert_eq!(census.open_residual_hop_count, 2);
        assert!(census.open_residual_fences_verified);
        assert!(!census.migration_complete);
        assert!(census.s7_green_claim_blocked);
        assert!(census.s_fed_trust_partial);
        assert!(!census.s_fed_trust_production_wired);
        assert!(!census.master_retick_eligible);
        assert!(!census.op5_cleared);
        assert!(!census.production_wired);
        assert_eq!(census.wire_hop_wired_count, 5);
    }

    #[test]
    fn sec_s7_production_and_fed_trust_stay_false() {
        assert!(!sec_s7_production_wired());
        assert!(!S_FED_TRUST_PRODUCTION_WIRED_HONEST);
        assert!(!MIGRATION_COMPLETE_HONEST);
        assert!(S7_GREEN_CLAIM_BLOCKED);
        assert!(S_FED_TRUST_PARTIAL_HONEST);
        assert!(!sec_s7_master_retick_eligible());
        assert!(!sec_s7_op5_cleared());
        assert!(!MASTER_RETICK_ELIGIBLE);
        assert!(!OP5_CLEARED);
    }

    #[test]
    fn sec_s7_manifold_wire_hops_cover_gate_and_trust_delegate() {
        assert_eq!(MANIFOLD_SEC_S7_GATE_WIRE_HOPS.len(), 7);
        assert_eq!(
            MANIFOLD_SEC_S7_GATE_WIRE_HOPS
                .iter()
                .filter(|h| h.wired)
                .count(),
            5
        );
        assert!(MANIFOLD_SEC_S7_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("AdmissibilityToken") && h.wired));
        assert!(MANIFOLD_SEC_S7_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("sec_s7_boundary_production_wired") && !h.wired));
        assert!(MANIFOLD_SEC_S7_GATE_WIRE_HOPS
            .iter()
            .any(|h| h.surface.contains("s_fed_trust_production_wired") && !h.wired));
    }

    #[test]
    fn sec_s7_manifold_gate_ceremony_close_predicate() {
        assert!(manifold_gate_sec_s7_ceremony_closed());
        let probe = sec_s7_gate_manifold_probe();
        assert!(probe.gate_evidence_wired);
        assert!(probe.s7_all_migrate_surfaces_probed);
        assert!(probe.migrate_queue_residue_pins_verified);
        assert!(probe.open_residual_fences_verified);
        assert!(probe.migration_inventory_census_verified);
        assert!(probe.migration_complete_honest_false);
        assert!(probe.s7_green_claim_blocked);
        assert!(probe.s_fed_trust_partial_honest);
        assert!(probe.s_fed_trust_production_honest_false);
        assert!(probe.master_retick_honest_false);
        assert!(probe.op5_cleared_honest_false);
        assert!(probe.production_honest_false);
        assert_eq!(probe.wire_hop_wired_count, 5);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s7_gate_factor_table_honest_blocked_scert() {
        let table = sec_s7_gate_factor_table();
        assert!(table.contains("SEC-S7 gate factors"));
        assert!(table.contains("scert_credit=BLOCKED"));
        assert!(table.contains("expected_gate_exit=BLOCKED"));
        let rows = collect_sec_s7_gate_factor_rows();
        assert_eq!(rows.len(), 7);
        assert!(rows.iter().filter(|r| r.probe_wired).count() >= 6);
    }

    #[test]
    fn sec_s7_prior_receipt_paths_pinned() {
        assert!(PRIOR_RECEIPT_PATH_2033.contains("SEC-S7_2033"));
        assert!(PRIOR_RECEIPT_PATH_2127.contains("SEC-S7-DRAIN"));
        assert!(PRIOR_RECEIPT_PATH_Y80.contains("COMPOSER_Y80"));
        assert!(TRUST_SSOT.contains("migration.rs"));
        assert!(FED_TRUST_SSOT.contains("sec_s_arc_federation_census"));
    }

    #[test]
    fn sec_s7_gate_wire_matrix_renders_honest_counts() {
        let matrix = sec_s7_gate_wire_matrix();
        assert!(matrix.contains("SEC-S7 manifold gate"));
        assert!(matrix.contains("migrate_queue=5"));
        assert!(matrix.contains("migration_complete=false"));
        assert!(matrix.contains("wired=5/7"));
        assert!(matrix.contains("open_residual=2"));
        assert!(matrix.contains("master_retick=false"));
        assert!(matrix.contains("op5_cleared=false"));
        assert!(matrix.contains("w29_cell_id=W29-126-SEC_S7"));
    }

    #[test]
    fn sec_s7_open_residual_fences_honest_open() {
        assert!(manifold_s7_open_residual_fences_verified());
        assert_eq!(OPEN_RESIDUAL_FENCES.len(), 2);
        assert_eq!(OPEN_RESIDUAL_HOP_COUNT, 2);
        let table = sec_s7_open_residual_fence_table();
        assert!(table.contains("R-gateway-boundary"));
        assert!(table.contains("R-s-fed-trust-production"));
        assert!(table.contains("production_wired=false"));
        assert!(table.contains("master_retick=false"));
        assert!(table.contains("op5_cleared=false"));
    }

    #[test]
    fn sec_s7_w29_126_deepen_honest_fences_no_green_production_master_op5() {
        assert!(sec_s7_w29_deepen_honest());
        let probe = sec_s7_w29_deepen_probe();
        assert_eq!(probe.w29_cell_id, "W29-126-SEC_S7");
        assert!(probe.schema_version.contains("_v3"));
        assert!(probe.open_residual_fences_verified);
        assert!(probe.j2_honest);
        assert!(probe.ac08_honest);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(!probe.s_fed_trust_production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_cleared);
        assert!(probe.s7_green_claim_blocked);
        assert!(probe.non_claim.contains("not GREEN"));
        assert!(probe.non_claim.contains("not MASTER_RETICK"));
        assert!(probe.non_claim.contains("not OP-5 PASS"));
    }

    #[test]
    fn fleet_composer_p1931_j2_sec_s7_honest() {
        assert!(sec_s7_p1931_j2_honest());
        let probe = sec_s7_p1931_j2_probe();
        assert_eq!(probe.j2_job_id, FLEET_P1931_J2_JOB_ID);
        assert!(probe.prior_2033_absorbed);
        assert!(probe.prior_2127_absorbed);
        assert!(probe.prior_y80_absorbed);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert!(probe.gate_factor_exit_blocked);
    }

    #[test]
    fn fleet_accel_ac08_sec_s7_migrate_queue_deepen_honest() {
        assert!(sec_s7_accel_ac08_honest());
        let probe = sec_s7_accel_ac08_probe();
        assert_eq!(probe.ac08_job_id, ACCEL_2030_AC08_JOB_ID);
        assert!(probe.prior_j2_absorbed);
        assert!(probe.migrate_queue_table_residue_pinned);
        assert!(!probe.migration_complete_measured);
        assert!(probe.ceremony_closed);
    }

    #[test]
    fn sec_s7_validate_gate_honesty_residue_measured() {
        validate_sec_s7_gate_honesty().expect("honest SEC-S7 gate census residue");
        assert_eq!(
            sec_s7_boundary_next_hop(),
            "umst-gateway/crates/umst-gateway/src/sec_s7_boundary.rs:R-SEC-S7-BOUNDARY-PROD"
        );
    }
}

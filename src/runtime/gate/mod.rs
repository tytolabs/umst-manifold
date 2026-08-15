// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Gate evidence contracts for cold-edge telemetry.
//!
//! # Barrel deepen (W30-234-GATE_MOD)
//!
//! Inventory + honesty rollup over SEC cold-edge arcs and core evidence modules.
//! Measured fences only: `production_wired=false`, GREEN claims blocked,
//! MASTER / OP-5 not claimed. Tool readiness ≠ physics GREEN.

pub mod admissibility_margin;
pub mod cartridge;
#[cfg(feature = "ucrs-provenance")]
pub mod cold_wire;
pub mod evidence;
pub mod explain_codes;
pub mod sec_bridge_arcs;
pub mod sec_gw_audit;
pub mod sec_gw_wrap;
pub mod sec_mcp_wrap;
pub mod sec_s1;
pub mod sec_s2;
pub mod sec_s3;
pub mod sec_s4;
pub mod sec_s5;
pub mod sec_s6;
pub mod sec_s7;

pub use admissibility_margin::{
    admissibility_from_margin, admissibility_margin_from_dissipation, AdmissibilityMargin,
    ADMISSIBILITY_MARGIN_EPS,
};
pub use cartridge::{CdTransitionCartridge, GateCartridge};
#[cfg(feature = "ucrs-provenance")]
pub use cold_wire::{transition_evidence_to_wire, SpineEventCost, TransitionEvidenceWire};
pub use evidence::{
    explain_cd_transition_host, AdmissibilityToken, ConstraintExplanation, TransitionEvidence,
    UcrsObservedAtWire,
};
pub use explain_codes::{
    fields_for_code, remediation_for_code, GateFieldIssue, MANIFEST_BRIDGE_DISABLED,
    MIX_SPEC_RATIONAL_PARSE_FAIL, MIX_SPEC_WIRE_INVALID, THERMODYNAMIC_CD_FAIL, THERMODYNAMIC_FAIL,
    TOP_GATE_EXPLAIN_CODES,
};
pub use sec_bridge_arcs::{
    gate_bridge_arcs_census,
    gate_transition_evidence_probe as sec_bridge_arcs_gate_transition_evidence_probe,
    manifold_bridge_arcs_thermo_wire_hops_verified, manifold_bridge_arcs_trust_wire_hops_verified,
    manifold_gate_sec_bridge_arcs_ceremony_closed, sec_bridge_arcs_accel_ac35_honest,
    sec_bridge_arcs_accel_ac35_probe, sec_bridge_arcs_egoff_bridge_next_hop,
    sec_bridge_arcs_gate_manifold_probe, sec_bridge_arcs_gate_wire_matrix,
    sec_bridge_arcs_production_wired, validate_sec_bridge_arcs_gate_honesty,
    SecBridgeArcsAccelAc35Probe, SecBridgeArcsGateCoordinationCensus,
    SecBridgeArcsGateManifoldProbe, SecBridgeArcsGateWireHop,
    BOARD_SLICE_ID as SEC_BRIDGE_ARCS_BOARD_SLICE_ID, BRIDGE_ARCS_GREEN_CLAIM_BLOCKED,
    EGOFF_FULL_CRATE_VERIFY_BLOCKED, FLEET_ACCEL_AC35_JOB_ID, FLEET_ACCEL_AC35_RECEIPT_PATH,
    MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS,
};
pub use sec_gw_audit::{
    gate_admit_audit_census, manifold_gate_sec_gw_audit_ceremony_closed,
    manifold_gw_audit_all_stamp_paths_probed, manifold_gw_audit_stamp_legs_complete,
    manifold_verify_upstream_gw_wrap_delegate, sec_gw_audit_accel2_ac31_honest,
    sec_gw_audit_accel2_ac31_probe, sec_gw_audit_manifold_probe, sec_gw_audit_production_wired,
    sec_gw_audit_trust_chain_next_hop, sec_gw_audit_wire_matrix, validate_sec_gw_audit_honesty,
    SecGwAuditAccel2Ac31Probe, SecGwAuditManifoldAdmitCensus, SecGwAuditManifoldProbe,
    SecGwAuditManifoldWireHop, ADMIT_STAMP_PATH_COUNT,
    BOARD_SLICE_ID as SEC_GW_AUDIT_BOARD_SLICE_ID, FLEET_ACCEL2_AC31_JOB_ID,
    FLEET_ACCEL2_AC31_RECEIPT_PATH, GW_AUDIT_GREEN_CLAIM_BLOCKED,
    MANIFOLD_GW_AUDIT_ADMIT_STAMP_PATHS, MANIFOLD_GW_AUDIT_STAMP_LEGS,
    MANIFOLD_SEC_GW_AUDIT_WIRE_HOPS,
};
pub use sec_gw_wrap::{
    gate_trust_wrap_census, manifold_gate_sec_gw_wrap_ceremony_closed,
    manifold_gw_wrap_all_admit_surfaces_probed, manifold_gw_wrap_ledger_prep_complete,
    manifold_verify_upstream_gate_delegates, sec_gw_wrap_accel2_ac32_honest,
    sec_gw_wrap_accel2_ac32_probe, sec_gw_wrap_honesty_probe, sec_gw_wrap_manifold_probe,
    sec_gw_wrap_production_wired, sec_gw_wrap_trust_wrap_next_hop, sec_gw_wrap_wire_matrix,
    validate_sec_gw_wrap_honesty, SecGwWrapAccel2Ac32Probe, SecGwWrapHonestyFence,
    SecGwWrapHonestyProbe, SecGwWrapManifoldProbe, SecGwWrapManifoldTrustCensus,
    SecGwWrapManifoldWireHop, ADMIT_SURFACE_COUNT as GW_WRAP_ADMIT_SURFACE_COUNT,
    BOARD_SLICE_ID as SEC_GW_WRAP_BOARD_SLICE_ID, FLEET_ACCEL2_AC32_JOB_ID,
    FLEET_ACCEL2_AC32_RECEIPT_PATH, GW_WRAP_GREEN_CLAIM_BLOCKED, MANIFOLD_GW_WRAP_ADMIT_SURFACES,
    MANIFOLD_GW_WRAP_LEDGER_PREP_HOPS, MANIFOLD_SEC_GW_WRAP_WIRE_HOPS, SEC_GW_WRAP_CELL_ID,
    SEC_GW_WRAP_MASTER_RETICK_ELIGIBLE, SEC_GW_WRAP_OP5_CLEARED, SEC_GW_WRAP_PHYSICS_GREEN,
    W29_118_SEC_GW_WRAP_DEEPEN_STEP,
};
pub use sec_mcp_wrap::{
    collect_sec_mcp_wrap_gate_factor_rows, gate_mcp_wrap_census,
    gate_transition_evidence_probe as sec_mcp_wrap_gate_transition_evidence_probe,
    manifold_gate_sec_mcp_wrap_ceremony_closed, manifold_mcp_wrap_refuse_path_matrix_pins_verified,
    sec_mcp_wrap_accel_ac34_honest, sec_mcp_wrap_accel_ac34_probe, sec_mcp_wrap_gate_factor_table,
    sec_mcp_wrap_gate_manifold_probe, sec_mcp_wrap_gate_wire_matrix,
    sec_mcp_wrap_mcp_delegate_next_hop, sec_mcp_wrap_production_wired,
    sec_mcp_wrap_refuse_path_table, validate_sec_mcp_wrap_gate_honesty, SecMcpWrapAccelAc34Probe,
    SecMcpWrapGateCensus, SecMcpWrapGateFactorRow, SecMcpWrapGateManifoldProbe,
    SecMcpWrapGateWireHop, ACCEL_AC34_RECEIPT_PATH, ACCEL_B_2050_AC34_JOB_ID,
    BOARD_SLICE_ID as SEC_MCP_WRAP_BOARD_SLICE_ID, DEEPEN_JOB_ID as SEC_MCP_WRAP_DEEPEN_JOB_ID,
    FLEET_Z86_JOB_ID, GATEWAY_STDIO_EXEC_OWNER, MANIFOLD_SEC_MCP_WRAP_GATE_WIRE_HOPS, MCP_SSOT,
    MCP_WRAP_GREEN_CLAIM_BLOCKED, REFUSE_PATH_MATRIX_ROW_COUNT,
};
pub use sec_s1::{
    collect_sec_s1_gate_factor_rows,
    gate_transition_evidence_probe as sec_s1_gate_transition_evidence_probe, gate_trust_census,
    manifold_gate_sec_s1_ceremony_closed, manifold_s1_all_factors_probed,
    manifold_s1_factor_coverage_probes, manifold_verify_trust_gate_s1_pins,
    sec_s1_accel_ac28_honest, sec_s1_accel_ac28_probe, sec_s1_gate_factor_table,
    sec_s1_gate_manifold_probe, sec_s1_gate_wire_matrix, sec_s1_production_wired,
    sec_s1_session_ledger_next_hop, session_ledger_wired as sec_s1_session_ledger_wired,
    validate_sec_s1_gate_honesty, ManifoldS1FactorProbe, SecS1AccelAc28Probe, SecS1GateFactorRow,
    SecS1GateManifoldProbe, SecS1GateTrustCensus, SecS1GateWireHop,
    BOARD_SLICE_ID as SEC_S1_BOARD_SLICE_ID, EXPECTED_GATE_EXIT as SEC_S1_EXPECTED_GATE_EXIT,
    FLEET_ACCEL2_AC28_JOB_ID, FLEET_ACCEL2_AC28_RECEIPT_PATH, MANIFOLD_SEC_S1_GATE_WIRE_HOPS,
    S1_FACTOR_IDS, S1_FACTOR_ROW_COUNT, S1_GREEN_CLAIM_BLOCKED,
};
pub use sec_s2::{
    collect_sec_s2_gate_factor_rows,
    gate_transition_evidence_probe as sec_s2_gate_transition_evidence_probe,
    gate_trust_refuse_census, manifold_gate_sec_s2_ceremony_closed,
    manifold_s2_all_refuse_paths_probed, manifold_s2_extract_fence_facets_verified,
    manifold_s2_refuse_path_coverage_probes, manifold_verify_trust_gate_policy_pins,
    sec_s2_accel_ac29_honest, sec_s2_accel_ac29_probe, sec_s2_extract_fence_wired_count,
    sec_s2_extract_production_fence_matrix, sec_s2_extract_production_fence_next_hop,
    sec_s2_gate_factor_exit_expectations, sec_s2_gate_factor_table, sec_s2_gate_manifold_probe,
    sec_s2_gate_wire_matrix, sec_s2_p1941_k2_honest, sec_s2_p1941_k2_probe,
    sec_s2_production_wired, sec_s2_trust_extract_production_wired, validate_sec_s2_gate_honesty,
    ManifoldS2ExtractProductionFenceFacet, ManifoldS2RefusePathProbe, SecS2AccelAc29Probe,
    SecS2GateFactorExitExpectations, SecS2GateFactorRow, SecS2GateManifoldProbe,
    SecS2GateTrustRefuseCensus, SecS2GateWireHop, SecS2P1941K2Probe,
    BOARD_SLICE_ID as SEC_S2_BOARD_SLICE_ID, CLASSICAL_WRAP_TOTAL, CLASSICAL_WRAP_WRAPPED,
    EXPECTED_GATE_EXIT, EXTRACT_SSOT, FLEET_ACCEL_AC29_JOB_ID, FLEET_ACCEL_AC29_RECEIPT_PATH,
    FLEET_P1941_K2_JOB_ID, FLEET_P1941_K2_RECEIPT_PATH,
    MANIFOLD_S2_EXTRACT_PRODUCTION_FENCE_FACETS, MANIFOLD_SEC_S2_GATE_WIRE_HOPS,
    S2_EXTRACT_FENCE_FACET_COUNT, S2_EXTRACT_FENCE_FACET_IDS, S2_EXTRACT_FENCE_WIRED_COUNT,
    S2_FACTOR_ROW_COUNT, S2_GATE_FACTOR_PALETTE_KEYS, S2_GREEN_CLAIM_BLOCKED,
    S2_REFUSE_PATH_FACTOR_IDS, TRUST_ADT_SSOT, TRUST_GATE_POLICY_STRICT,
    TRUST_GATE_POLICY_WARN_ONLY, UCRS_WIRE_PARITY_TEST, WRAP_QUEUE_DEPTH,
};
pub use sec_s3::{
    gate_palette_ledger_census, gate_transition_evidence_probe,
    manifold_gate_sec_s3_ceremony_closed, sec_s3_accel_ac05_honest, sec_s3_accel_ac05_probe,
    sec_s3_gate_manifold_probe, sec_s3_gate_wire_matrix, sec_s3_p1606_c5_honest,
    sec_s3_p1606_c5_probe, sec_s3_production_wired, sec_s3_session_ledger_next_hop,
    session_ledger_wired, validate_sec_s3_gate_honesty, SecS3AccelAc05Probe,
    SecS3GateManifoldProbe, SecS3GatePaletteLedgerCensus, SecS3GateWireHop, SecS3P1606C5Probe,
    BOARD_SLICE_ID as SEC_S3_BOARD_SLICE_ID, FLEET_P1606_C5_JOB_ID, FLEET_P1606_C5_RECEIPT_PATH,
    MANIFOLD_SEC_S3_GATE_WIRE_HOPS, PALETTE_PERSISTED_HONEST, S3_GREEN_CLAIM_BLOCKED,
};
pub use sec_s4::{
    gate_side_channel_scrub_census,
    gate_transition_evidence_probe as sec_s4_gate_transition_evidence_probe,
    manifold_gate_sec_s4_ceremony_closed, manifold_ls5_all_k_v1_probed,
    manifold_ls5_k_v1_coverage_probes, manifold_scrub_k_v1_tokens, manifold_verify_scrub_roundtrip,
    sec_s4_accel_ac06_honest, sec_s4_accel_ac06_probe, sec_s4_gate_manifold_probe,
    sec_s4_gate_wire_matrix, sec_s4_l_s5_proof_next_hop, sec_s4_p1800_h3_honest,
    sec_s4_p1800_h3_probe, sec_s4_production_wired, validate_sec_s4_gate_honesty,
    ManifoldLs5Kv1Probe, SecS4AccelAc06Probe, SecS4GateManifoldProbe,
    SecS4GateSideChannelScrubCensus, SecS4GateWireHop, SecS4P1800H3Probe,
    BOARD_SLICE_ID as SEC_S4_BOARD_SLICE_ID, FLEET_P1800_H3_JOB_ID, FLEET_P1800_H3_RECEIPT_PATH,
    K_V1_PATTERNS, L_S5_PROOF_WIRED_HONEST, MANIFOLD_SEC_S4_GATE_WIRE_HOPS, SCRUB_PLACEHOLDER,
};
pub use sec_s5::{
    collect_sec_s5_gate_factor_rows, gate_synthetic_consensus_census,
    gate_transition_evidence_probe as sec_s5_gate_transition_evidence_probe,
    manifold_gate_sec_s5_ceremony_closed, manifold_s5_all_scenarios_probed,
    manifold_s5_consensus_coverage_probes, manifold_verify_s5_consensus_algebra_roundtrip,
    sec_s5_accel_ac07_honest, sec_s5_accel_ac07_probe, sec_s5_gate_factor_table,
    sec_s5_gate_manifold_probe, sec_s5_gate_wire_matrix, sec_s5_ln0_proof_next_hop,
    sec_s5_p1812_i2_honest, sec_s5_p1812_i2_probe, sec_s5_production_wired,
    validate_sec_s5_gate_honesty, SecS5AccelAc07Probe, SecS5GateFactorRow, SecS5GateManifoldProbe,
    SecS5GateSyntheticConsensusCensus, SecS5GateWireHop, SecS5P1812I2Probe,
    BOARD_SLICE_ID as SEC_S5_BOARD_SLICE_ID, FLEET_P1812_I2_JOB_ID, FLEET_P1812_I2_RECEIPT_PATH,
    LIVE_FANOUT_WIRED_HONEST, LN0_PROOF_WIRED_HONEST, MANIFOLD_SEC_S5_GATE_WIRE_HOPS,
    S5_CONSENSUS_PROBE_SCENARIOS, S5_GREEN_CLAIM_BLOCKED,
};
pub use sec_s6::{
    collect_sec_s6_gate_factor_rows, gate_hcom_prov_gateway_fence_census,
    gate_transition_evidence_probe as sec_s6_gate_transition_evidence_probe,
    manifold_gate_sec_s6_ceremony_closed, manifold_hcom_prov_gw_fence_hops_verified,
    manifold_s6_inspect_delegate_verified, manifold_scert_upstream_slots_verified,
    sec_s6_accel_ac33_honest, sec_s6_accel_ac33_probe, sec_s6_gate_factor_table,
    sec_s6_gate_manifold_probe, sec_s6_gate_wire_matrix, sec_s6_hcom_prov_fence_table,
    sec_s6_hcom_prov_gw_next_hop, sec_s6_production_wired, sec_s6_scert_upstream_table,
    validate_sec_s6_gate_honesty, ManifoldHcomProvGwFenceHop, ManifoldS6UpstreamSlot,
    SecS6AccelAc33Probe, SecS6GateFactorRow, SecS6GateHcomProvGatewayFenceCensus,
    SecS6GateManifoldProbe, SecS6GateWireHop, ACCEL_AC33_RECEIPT_PATH, ACCEL_B_2050_AC33_JOB_ID,
    BOARD_SLICE_ID as SEC_S6_BOARD_SLICE_ID, EXPECTED_GATE_EXIT as SEC_S6_EXPECTED_GATE_EXIT,
    HCOM_PROV_GW_FENCE_HOPS, HCOM_PROV_GW_WIRE_HOP_COUNT, LIVE_ATTESTATION_WIRED_HONEST,
    MANIFOLD_SEC_S6_GATE_WIRE_HOPS, S6_GREEN_CLAIM_BLOCKED, S6_INSPECT_FACTOR_COUNT,
    SCERT_EXIT_NOT_WIRED, SCERT_UPSTREAM_SLOTS,
};
pub use sec_s7::{
    collect_sec_s7_gate_factor_rows, gate_fed_trust_migration_census,
    gate_transition_evidence_probe as sec_s7_gate_transition_evidence_probe,
    manifold_gate_sec_s7_ceremony_closed, manifold_s7_all_migrate_surfaces_probed,
    manifold_s7_migrate_coverage_probes, manifold_verify_migration_inventory_census,
    sec_s7_accel_ac08_honest, sec_s7_accel_ac08_probe, sec_s7_boundary_next_hop,
    sec_s7_gate_factor_table, sec_s7_gate_manifold_probe, sec_s7_gate_wire_matrix,
    sec_s7_p1931_j2_honest, sec_s7_p1931_j2_probe, sec_s7_production_wired,
    validate_sec_s7_gate_honesty, SecS7AccelAc08Probe, SecS7GateFedTrustMigrationCensus,
    SecS7GateManifoldProbe, SecS7GateWireHop, SecS7P1931J2Probe,
    BOARD_SLICE_ID as SEC_S7_BOARD_SLICE_ID, FLEET_P1931_J2_JOB_ID, FLEET_P1931_J2_RECEIPT_PATH,
    INVENTORY_ROW_COUNT, MANIFOLD_SEC_S7_GATE_WIRE_HOPS, MIGRATE_QUEUE_DEPTH,
    MIGRATION_COMPLETE_HONEST, S7_GREEN_CLAIM_BLOCKED, S_FED_TRUST_PARTIAL_HONEST,
    S_FED_TRUST_PRODUCTION_WIRED_HONEST,
};

// ── W30-234-GATE_MOD · runtime gate barrel census ────────────────────────────

/// Cell id for this barrel deepen.
pub const RUNTIME_GATE_MOD_CELL_ID: &str = "W30-234-GATE_MOD";

/// Honest posture tag — census wired, not production / not physics GREEN.
pub const RUNTIME_GATE_MOD_POSTURE_TAG: &str = "runtime-gate-barrel-census-wired-not-production";

/// Always-on child modules (excludes feature-gated `cold_wire`).
pub const RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT: usize = 15;

/// Core evidence modules under this barrel (non-SEC).
pub const RUNTIME_GATE_CORE_MODULE_COUNT: usize = 4;

/// SEC cold-edge arcs rolled into the barrel (matches gate_deepen arc count).
pub const RUNTIME_GATE_SEC_ARC_COUNT: usize = 11;

/// Barrel-level GREEN claim fence — honest true (claims blocked).
pub const RUNTIME_GATE_GREEN_CLAIM_BLOCKED: bool = true;

/// Role of a child module in the runtime gate barrel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeGateModuleRole {
    /// Core cold-edge evidence / cartridge surface.
    CoreEvidence,
    /// SEC admit / refuse / wrap / audit arc.
    SecArc,
}

/// One inventory row for a runtime gate child module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeGateModuleBand {
    /// Stable band id (`runtime_gate:<module>`).
    pub band_id: &'static str,
    /// Child module name.
    pub module: &'static str,
    /// Role in the barrel.
    pub role: RuntimeGateModuleRole,
}

/// Frozen always-on module inventory — length = [`RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT`].
pub const RUNTIME_GATE_MODULE_BANDS: &[RuntimeGateModuleBand] = &[
    RuntimeGateModuleBand {
        band_id: "runtime_gate:admissibility_margin",
        module: "admissibility_margin",
        role: RuntimeGateModuleRole::CoreEvidence,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:cartridge",
        module: "cartridge",
        role: RuntimeGateModuleRole::CoreEvidence,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:evidence",
        module: "evidence",
        role: RuntimeGateModuleRole::CoreEvidence,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:explain_codes",
        module: "explain_codes",
        role: RuntimeGateModuleRole::CoreEvidence,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s1",
        module: "sec_s1",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s2",
        module: "sec_s2",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s3",
        module: "sec_s3",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s4",
        module: "sec_s4",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s5",
        module: "sec_s5",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s6",
        module: "sec_s6",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_s7",
        module: "sec_s7",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_mcp_wrap",
        module: "sec_mcp_wrap",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_gw_wrap",
        module: "sec_gw_wrap",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_gw_audit",
        module: "sec_gw_audit",
        role: RuntimeGateModuleRole::SecArc,
    },
    RuntimeGateModuleBand {
        band_id: "runtime_gate:sec_bridge_arcs",
        module: "sec_bridge_arcs",
        role: RuntimeGateModuleRole::SecArc,
    },
];

/// Barrel census probe — measured inventory + honesty fences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeGateModCensusProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub always_on_module_count: usize,
    pub core_module_count: usize,
    pub sec_arc_count: usize,
    pub barrel_reexports_wired: bool,
    pub sec_validate_all_honest: bool,
    pub green_claim_blocked_all_arcs: bool,
    pub production_wired_any_arc: bool,
    /// Barrel production fence — always false until measured live wire.
    pub production_wired: bool,
    /// Barrel GREEN fence — always true (claims blocked).
    pub green_claim_blocked: bool,
    /// MASTER retick not claimed from barrel deepen.
    pub master_retick_eligible: bool,
    /// OP-5 not claimed from barrel deepen.
    pub op5_cleared: bool,
    pub census_deepen_honest: bool,
}

/// Whether live barrel re-exports resolve for core + SEC surfaces.
#[must_use]
pub fn runtime_gate_barrel_reexports_wired() -> bool {
    ADMISSIBILITY_MARGIN_EPS.is_finite()
        && !TOP_GATE_EXPLAIN_CODES.is_empty()
        && !MANIFOLD_SEC_S1_GATE_WIRE_HOPS.is_empty()
        && !MANIFOLD_SEC_S2_GATE_WIRE_HOPS.is_empty()
        && !MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS.is_empty()
        && S2_EXTRACT_FENCE_FACET_IDS.len() == S2_EXTRACT_FENCE_FACET_COUNT
        && S1_GREEN_CLAIM_BLOCKED
        && S2_GREEN_CLAIM_BLOCKED
        && S3_GREEN_CLAIM_BLOCKED
        && S5_GREEN_CLAIM_BLOCKED
        && S6_GREEN_CLAIM_BLOCKED
        && S7_GREEN_CLAIM_BLOCKED
        && MCP_WRAP_GREEN_CLAIM_BLOCKED
        && GW_WRAP_GREEN_CLAIM_BLOCKED
        && GW_AUDIT_GREEN_CLAIM_BLOCKED
        && BRIDGE_ARCS_GREEN_CLAIM_BLOCKED
}

/// Whether every SEC arc `validate_*_honesty` residue passes.
#[must_use]
pub fn runtime_gate_sec_validate_all_honest() -> bool {
    validate_sec_s1_gate_honesty().is_ok()
        && validate_sec_s2_gate_honesty().is_ok()
        && validate_sec_s3_gate_honesty().is_ok()
        && validate_sec_s4_gate_honesty().is_ok()
        && validate_sec_s5_gate_honesty().is_ok()
        && validate_sec_s6_gate_honesty().is_ok()
        && validate_sec_s7_gate_honesty().is_ok()
        && validate_sec_mcp_wrap_gate_honesty().is_ok()
        && validate_sec_gw_wrap_honesty().is_ok()
        && validate_sec_gw_audit_honesty().is_ok()
        && validate_sec_bridge_arcs_gate_honesty().is_ok()
}

/// Whether any SEC arc claims production wired (must stay false).
#[must_use]
pub fn runtime_gate_production_wired_any_arc() -> bool {
    sec_s1_production_wired()
        || sec_s2_production_wired()
        || sec_s3_production_wired()
        || sec_s4_production_wired()
        || sec_s5_production_wired()
        || sec_s6_production_wired()
        || sec_s7_production_wired()
        || sec_mcp_wrap_production_wired()
        || sec_gw_wrap_production_wired()
        || sec_gw_audit_production_wired()
        || sec_bridge_arcs_production_wired()
}

/// Whether GREEN claim fences remain blocked across exported SEC arcs.
#[must_use]
pub fn runtime_gate_green_claim_blocked_all_arcs() -> bool {
    S1_GREEN_CLAIM_BLOCKED
        && S2_GREEN_CLAIM_BLOCKED
        && S3_GREEN_CLAIM_BLOCKED
        && !L_S5_PROOF_WIRED_HONEST
        && S5_GREEN_CLAIM_BLOCKED
        && S6_GREEN_CLAIM_BLOCKED
        && S7_GREEN_CLAIM_BLOCKED
        && MCP_WRAP_GREEN_CLAIM_BLOCKED
        && GW_WRAP_GREEN_CLAIM_BLOCKED
        && GW_AUDIT_GREEN_CLAIM_BLOCKED
        && BRIDGE_ARCS_GREEN_CLAIM_BLOCKED
        && RUNTIME_GATE_GREEN_CLAIM_BLOCKED
}

/// Barrel production path — honest false until measured live production wire.
#[must_use]
pub const fn runtime_gate_mod_production_wired() -> bool {
    false
}

/// MASTER retick eligibility — honest false (not claimed from barrel deepen).
#[must_use]
pub const fn runtime_gate_mod_master_retick_eligible() -> bool {
    false
}

/// OP-5 clearance — honest false (not claimed from barrel deepen).
#[must_use]
pub const fn runtime_gate_mod_op5_cleared() -> bool {
    false
}

/// Build runtime gate barrel census probe from live module measurements.
#[must_use]
pub fn runtime_gate_mod_census_probe() -> RuntimeGateModCensusProbe {
    let barrel_reexports_wired = runtime_gate_barrel_reexports_wired();
    let sec_validate_all_honest = runtime_gate_sec_validate_all_honest();
    let green_claim_blocked_all_arcs = runtime_gate_green_claim_blocked_all_arcs();
    let production_wired_any_arc = runtime_gate_production_wired_any_arc();
    let core_module_count = RUNTIME_GATE_MODULE_BANDS
        .iter()
        .filter(|b| b.role == RuntimeGateModuleRole::CoreEvidence)
        .count();
    let sec_arc_count = RUNTIME_GATE_MODULE_BANDS
        .iter()
        .filter(|b| b.role == RuntimeGateModuleRole::SecArc)
        .count();
    let census_deepen_honest = RUNTIME_GATE_MODULE_BANDS.len()
        == RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT
        && core_module_count == RUNTIME_GATE_CORE_MODULE_COUNT
        && sec_arc_count == RUNTIME_GATE_SEC_ARC_COUNT
        && barrel_reexports_wired
        && sec_validate_all_honest
        && green_claim_blocked_all_arcs
        && !production_wired_any_arc
        && !runtime_gate_mod_production_wired()
        && RUNTIME_GATE_GREEN_CLAIM_BLOCKED
        && !runtime_gate_mod_master_retick_eligible()
        && !runtime_gate_mod_op5_cleared();

    RuntimeGateModCensusProbe {
        cell_id: RUNTIME_GATE_MOD_CELL_ID,
        posture_tag: RUNTIME_GATE_MOD_POSTURE_TAG,
        always_on_module_count: RUNTIME_GATE_MODULE_BANDS.len(),
        core_module_count,
        sec_arc_count,
        barrel_reexports_wired,
        sec_validate_all_honest,
        green_claim_blocked_all_arcs,
        production_wired_any_arc,
        production_wired: runtime_gate_mod_production_wired(),
        green_claim_blocked: RUNTIME_GATE_GREEN_CLAIM_BLOCKED,
        master_retick_eligible: runtime_gate_mod_master_retick_eligible(),
        op5_cleared: runtime_gate_mod_op5_cleared(),
        census_deepen_honest,
    }
}

/// Honesty gate — census wired; production / GREEN / MASTER / OP-5 blocked.
#[must_use]
pub fn runtime_gate_mod_census_honest(probe: &RuntimeGateModCensusProbe) -> bool {
    probe.cell_id == RUNTIME_GATE_MOD_CELL_ID
        && probe.posture_tag == RUNTIME_GATE_MOD_POSTURE_TAG
        && probe.always_on_module_count == RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT
        && probe.core_module_count == RUNTIME_GATE_CORE_MODULE_COUNT
        && probe.sec_arc_count == RUNTIME_GATE_SEC_ARC_COUNT
        && probe.barrel_reexports_wired
        && probe.sec_validate_all_honest
        && probe.green_claim_blocked_all_arcs
        && !probe.production_wired_any_arc
        && !probe.production_wired
        && probe.green_claim_blocked
        && !probe.master_retick_eligible
        && !probe.op5_cleared
        && probe.census_deepen_honest
}

/// Deepen honesty predicate for W30-234 barrel census.
#[must_use]
pub fn runtime_gate_mod_census_deepen_honest(probe: &RuntimeGateModCensusProbe) -> bool {
    runtime_gate_mod_census_honest(probe)
        && probe.cell_id == "W30-234-GATE_MOD"
        && probe.sec_arc_count == 11
        && !probe.production_wired
        && probe.green_claim_blocked
        && !probe.master_retick_eligible
        && !probe.op5_cleared
}

/// Validate runtime gate barrel census — fail closed on drift or invented posture.
pub fn verify_runtime_gate_mod_census() -> Result<RuntimeGateModCensusProbe, String> {
    let probe = runtime_gate_mod_census_probe();
    if probe.always_on_module_count != RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT {
        return Err(format!(
            "runtime gate module count drift: expected {RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT}, got {}",
            probe.always_on_module_count
        ));
    }
    if probe.sec_arc_count != RUNTIME_GATE_SEC_ARC_COUNT {
        return Err(format!(
            "runtime gate SEC arc count drift: expected {RUNTIME_GATE_SEC_ARC_COUNT}, got {}",
            probe.sec_arc_count
        ));
    }
    if !probe.barrel_reexports_wired {
        return Err("runtime gate barrel re-exports not wired".into());
    }
    if !probe.sec_validate_all_honest {
        return Err("runtime gate SEC validate honesty residue failed".into());
    }
    if probe.production_wired_any_arc || probe.production_wired {
        return Err("runtime gate production_wired must stay honest false".into());
    }
    if !probe.green_claim_blocked || !probe.green_claim_blocked_all_arcs {
        return Err("runtime gate GREEN claim must stay blocked".into());
    }
    if probe.master_retick_eligible {
        return Err("runtime gate must not invent MASTER retick eligibility".into());
    }
    if probe.op5_cleared {
        return Err("runtime gate must not invent OP-5 clearance".into());
    }
    if !runtime_gate_mod_census_deepen_honest(&probe) {
        return Err("runtime gate barrel deepen honesty predicate failed".into());
    }
    Ok(probe)
}

/// Validate barrel honesty residue — maps verify errors to static str for consumers.
pub fn validate_runtime_gate_mod_honesty() -> Result<(), &'static str> {
    match verify_runtime_gate_mod_census() {
        Ok(_) => Ok(()),
        Err(_) => Err("runtime gate mod census honesty failed"),
    }
}

#[cfg(test)]
mod runtime_gate_mod_census_tests {
    use super::*;

    #[test]
    fn runtime_gate_module_band_inventory_matches_pinned_counts() {
        assert_eq!(
            RUNTIME_GATE_MODULE_BANDS.len(),
            RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT
        );
        assert_eq!(RUNTIME_GATE_ALWAYS_ON_MODULE_COUNT, 15);
        assert_eq!(RUNTIME_GATE_CORE_MODULE_COUNT, 4);
        assert_eq!(RUNTIME_GATE_SEC_ARC_COUNT, 11);
        let core = RUNTIME_GATE_MODULE_BANDS
            .iter()
            .filter(|b| b.role == RuntimeGateModuleRole::CoreEvidence)
            .count();
        let arcs = RUNTIME_GATE_MODULE_BANDS
            .iter()
            .filter(|b| b.role == RuntimeGateModuleRole::SecArc)
            .count();
        assert_eq!(core, 4);
        assert_eq!(arcs, 11);
    }

    #[test]
    fn runtime_gate_barrel_reexports_include_bridge_arcs() {
        assert!(runtime_gate_barrel_reexports_wired());
        assert!(!MANIFOLD_SEC_BRIDGE_ARCS_GATE_WIRE_HOPS.is_empty());
        assert!(BRIDGE_ARCS_GREEN_CLAIM_BLOCKED);
        assert!(!sec_bridge_arcs_production_wired());
        assert_eq!(SEC_BRIDGE_ARCS_BOARD_SLICE_ID, "SEC-BRIDGE-ARCS");
    }

    #[test]
    fn runtime_gate_mod_census_probe_honest_not_green() {
        let probe = runtime_gate_mod_census_probe();
        assert!(runtime_gate_mod_census_honest(&probe));
        assert!(runtime_gate_mod_census_deepen_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.production_wired_any_arc);
        assert!(probe.green_claim_blocked);
        assert!(probe.green_claim_blocked_all_arcs);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_cleared);
        assert_eq!(probe.cell_id, "W30-234-GATE_MOD");
        assert_eq!(probe.sec_arc_count, 11);
    }

    #[test]
    fn runtime_gate_mod_fences_block_invented_posture() {
        assert!(!runtime_gate_mod_production_wired());
        assert!(!runtime_gate_mod_master_retick_eligible());
        assert!(!runtime_gate_mod_op5_cleared());
        assert!(RUNTIME_GATE_GREEN_CLAIM_BLOCKED);
        assert!(runtime_gate_green_claim_blocked_all_arcs());
        assert!(!runtime_gate_production_wired_any_arc());
    }

    #[test]
    fn runtime_gate_mod_verify_and_validate_pass() {
        verify_runtime_gate_mod_census().expect("runtime gate barrel census");
        validate_runtime_gate_mod_honesty().expect("runtime gate barrel honesty");
    }

    #[test]
    fn runtime_gate_sec_accel_helpers_reexported() {
        assert!(sec_s3_accel_ac05_honest());
        assert!(sec_s4_accel_ac06_honest());
        assert!(sec_s5_accel_ac07_honest());
        assert!(sec_s7_accel_ac08_honest());
        assert!(sec_bridge_arcs_accel_ac35_honest());
        assert!(S3_GREEN_CLAIM_BLOCKED);
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! `BurnAlgebra` production posture — slice-3 defer anchor for `umst-runtime`.
//!
//! **Honest boundary:** `umst-runtime` is the designated `burn::Tensor` home per
//! [`docs/C2_TENSOR_ALGEBRA_DESIGN.md`](../../../../docs/C2_TENSOR_ALGEBRA_DESIGN.md).
//! Production `impl TensorAlgebra for BurnAlgebra` over rank-1+ `burn::Tensor` is **not**
//! landed here — slice-2 0D `BurnScalar` prototype lives in `umst-cartridge-continuum`.
//!
//! Witness: [`ws_faithful_all`](../../../../umst-cartridge-api/src/ws_faithful_all.rs) · PBM-009.
//! Slice-3 0D lift step: [`atoms_tensor_lift`](../../../../src/runtime/atoms_tensor_lift.rs) · PBM-010.
//! Slice-3b rank-1+ ledger: [`atoms_tensor_lift_ledger`](../../../../src/runtime/atoms_tensor_lift_ledger.rs) · AGAP-2001-PBM-010.
//! Slice residual rows: [`atoms_tensor_lift_residual`](../../../../src/runtime/atoms_tensor_lift_residual.rs) · AGAP-2033-PBM-010.
//! Slice-3c adapter scaffold: [`atoms_tensor_lift_adapter`](../../../../src/runtime/atoms_tensor_lift_adapter.rs) · AGAP-2127-PBM-010.

/// Slice identifier for runtime tensor lift.
pub const SLICE_ID: &str = "slice-3";

/// Slice-3c rank-1+ adapter identifier.
pub const SLICE3C_ID: &str = "slice-3c";

/// Slice-3d tensor op spec identifier.
pub const SLICE3D_ID: &str = "slice-3d";

/// Slice-3b rank-1+ ledger identifier.
pub const SLICE3B_ID: &str = "slice-3b";

/// Parent residue — tensor production path not closed.
pub const PARENT_RESIDUE_ID: &str = "R-faithful-decomp-B1";

/// R-atoms-scalar parent for F1 rank-1+ ledger rows.
pub const R_ATOMS_SCALAR_ID: &str = "R-atoms-scalar";

/// PBM-009 workstream cross-ref (faithful-all tensor row).
pub const PBM_ID: &str = "PBM-009";

/// PBM-010 workstream cross-ref (R-atoms-scalar F1 lift step).
pub const PBM010_ID: &str = "PBM-010";

/// Honest posture — runtime burn home exists; `BurnAlgebra` prod impl **open**.
pub const POSTURE_TAG: &str = "HONEST_PARTIAL";

/// Whether the slice-3 0D atom lift step is landed (`atoms_tensor_lift`).
pub const LIFT_STEP_LANDED: bool = true;

/// Whether production rank-1+ `burn::Tensor` monomorphization is landed.
pub const PRODUCTION_LANDED: bool = false;

/// Whether slice-3b rank-1+ ledger rows are landed (AGAP-2001-PBM-010).
pub const RANK1_PLUS_LEDGER_LANDED: bool = true;

/// Whether slice-3b rank-1+ tensor lift is landed.
pub const RANK1_PLUS_LIFT_LANDED: bool = false;

/// Whether slice-3c adapter contract scaffold is landed (AGAP-2127-PBM-010).
pub const ADAPTER_SCAFFOLD_LANDED: bool = true;

/// Whether slice-3d tensor op spec is landed (SWARM-C25-0831-89).
pub const OP_SPEC_LANDED: bool = true;

/// Whether slice-3c rank-1+ `impl TensorAlgebra` is landed.
pub const RANK1_PLUS_IMPL_LANDED: bool = false;

/// Whether slice residual rows are landed (AGAP-2033-PBM-010).
pub const SLICE_RESIDUAL_ROWS_LANDED: bool = true;

/// Whether slice-2 continuum prototype is on disk (cross-crate witness).
pub const SLICE2_PROTOTYPE_LANDED: bool = true;

/// Continuum fence prototype path.
pub const SLICE2_PROTOTYPE_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/src/tensor_lift/burn_algebra.rs";

/// Slice-2 ψ/𝒟 parity integration witness.
pub const SLICE2_WITNESS_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/tests/b1_tensor_parity_probe.rs";

/// Slice-3 0D lift step (landed @ PBM-010).
pub const SLICE3_LIFT_STEP_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift.rs";

/// Slice-3b rank-1+ ledger (landed @ AGAP-2001-PBM-010).
pub const SLICE3B_LEDGER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ledger.rs";

/// Slice residual rows (landed @ AGAP-2033-PBM-010).
pub const SLICE_RESIDUAL_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_residual.rs";

/// Slice-3c adapter contract scaffold (landed @ AGAP-2127-PBM-010).
pub const SLICE3C_ADAPTER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_adapter.rs";

/// Slice-3d tensor op spec (landed @ SWARM-C25-0831-89).
pub const SLICE3D_OPS_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ops.rs";

/// Planned rank-1+ adapter crate (not created).
pub const SLICE3_ADAPTER_PATH: &str = "umst-runtime/crates/umst-algebra-burn/";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// B6 growth tensor `F_g` defer anchor — blocks rank-1+ production until W4-B6-2.
pub const B6_GROWTH_TENSOR_DEFER_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-active/src/state.rs";

/// B6 faithful partial witness (slice-0 ψ_fuel + P_input only).
pub const B6_FAITHFUL_WITNESS_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-active/tests/faithful_decomposition.rs";

/// Whether B6 growth tensor ψ terms block production tensor closure.
pub const B6_GROWTH_TENSOR_BLOCKS_PRODUCTION: bool = true;

/// Fleet receipt for PBM-009 tensor row + B6 deepen (AGAP-2127).
pub const RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-009-B6_2127";

/// Prior receipt (AGAP-2033 B5 witnessed deepen).
pub const PRIOR_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-009_2033";

/// Fleet receipt for PBM-010 F1 lift step (slice-3).
pub const PBM010_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2001";

/// Slice residual deepen receipt (AGAP-2033-PBM-010).
pub const PBM010_SLICE_RESIDUAL_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2033";

/// Slice-3c adapter deepen receipt (AGAP-2127-PBM-010).
pub const PBM010_SLICE3C_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2127";

/// Slice-3d tensor op spec deepen receipt (SWARM-C25-0831-89).
pub const PBM010_SLICE3D_RECEIPT_SLUG: &str = "COMPLETION_SWARM_SWARM-C25-0831-89_0831";

/// A3 nested `umst-runtime` alias present under `umst-manifold/crates/`.
pub const NESTED_RUNTIME_ALIAS_PRESENT: bool = true;

/// Top-level `umst-runtime/` crate absent (operator GitHub 301 deferred).
pub const TOP_LEVEL_RUNTIME_PRESENT: bool = false;

/// GitHub 301 rename deferred — `umst-manifold` remains physics-solver dependency hub.
pub const GITHUB_301_DEFERRED: bool = true;

/// Whether physics solvers still hub exclusively in `umst-manifold` (not ported out).
pub const PHYSICS_SOLVER_HUB_NOT_PORTED: bool = true;

/// Nested A3 alias path (present).
pub const NESTED_RUNTIME_PATH: &str = "umst-manifold/crates/umst-runtime/";

/// Physics solver submodule hub root.
pub const PHYSICS_SOLVER_HUB_PATH: &str = "umst-manifold/src/physics/solvers/";

/// Mechanics solver hub (parent `physics` module, not `solvers/`).
pub const MECHANICS_SOLVER_HUB_PATH: &str = "umst-manifold/src/physics/mechanics.rs";

/// Solver-Status SSOT (main solver table).
pub const SOLVER_STATUS_DOC_PATH: &str = "umst-manifold/docs/Solver-Status.md";

/// Always-on solver submodules (no `#[cfg(feature)]` on `pub mod`).
pub const ALWAYS_ON_SOLVER_MODULE_COUNT: usize = 13;

/// Feature-gated solver submodules (`thmc-coupled`, `solver-experimental`).
pub const FEATURE_GATED_SOLVER_MODULE_COUNT: usize = 5;

/// Supporting infra modules (fixed-point, Krylov host, THMC residual inventory).
pub const SUPPORTING_SOLVER_INFRA_MODULE_COUNT: usize = 5;

/// Total solver source files under hub (excluding `mod.rs`).
pub const TOTAL_SOLVER_SOURCE_FILE_COUNT: usize = 18;

/// Primary solver lanes from Solver-Status main table (measured @ W4 deepen).
pub const PRIMARY_SOLVER_LANE_COUNT: usize = 9;

/// Stable-lane solver rows still hubbing in manifold.
pub const STABLE_LANE_SOLVER_COUNT: usize = 1;

/// Research-lane solver rows still hubbing in manifold.
pub const RESEARCH_LANE_SOLVER_COUNT: usize = 8;

/// Fleet receipt for PORT-RT-NEST-SOLVER-INV-W4 deepen.
pub const SOLVER_HUB_RECEIPT_SLUG: &str = "PORT-RT-NEST-SOLVER-INV-W4";

/// One physics solver lane still hosted in `umst-manifold` hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverHubRow {
    /// Solver-Status lane id (`topology`, `mechanics`, `thmc`, …).
    pub lane_id: &'static str,
    /// Primary solver type surface.
    pub solver_type: &'static str,
    /// Manifold hub module basename.
    pub hub_module: &'static str,
    /// Manifold hub source path.
    pub hub_path: &'static str,
    /// Cargo feature lane (`stable` | `research`).
    pub feature_lane: &'static str,
    /// Whether this lane has been ported out of manifold hub.
    pub ported_out: bool,
    /// Honest hub status — all primary lanes remain `HUB_IN_MANIFOLD`.
    pub hub_status: &'static str,
}

/// Frozen census — which physics solvers still hub in `umst-manifold` (measured @ W4 deepen).
pub const SOLVER_HUB_ROWS: &[SolverHubRow] = &[
    SolverHubRow {
        lane_id: "topology",
        solver_type: "TopologyOptimizer",
        hub_module: "topology_solver",
        hub_path: "umst-manifold/src/physics/solvers/topology_solver.rs",
        feature_lane: "stable",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "mechanics",
        solver_type: "VectorMechanicsSolver",
        hub_module: "mechanics",
        hub_path: MECHANICS_SOLVER_HUB_PATH,
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "fracture",
        solver_type: "PhaseFieldFractureSolver",
        hub_module: "fracture_field",
        hub_path: "umst-manifold/src/physics/solvers/fracture_field.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "acoustics",
        solver_type: "AcousticWaveSolver",
        hub_module: "acoustics",
        hub_path: "umst-manifold/src/physics/solvers/acoustics.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "electrochemistry",
        solver_type: "ElectroChemicalSolver",
        hub_module: "electrochemistry",
        hub_path: "umst-manifold/src/physics/solvers/electrochemistry.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "photonics",
        solver_type: "PhotonicsSolver",
        hub_module: "photonics",
        hub_path: "umst-manifold/src/physics/solvers/photonics.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "rheology",
        solver_type: "BinghamFlowSolver",
        hub_module: "rheology_flow",
        hub_path: "umst-manifold/src/physics/solvers/rheology_flow.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "thmc",
        solver_type: "ThmcSolver",
        hub_module: "thmc",
        hub_path: "umst-manifold/src/physics/solvers/thmc.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
    SolverHubRow {
        lane_id: "statmech",
        solver_type: "StatisticalBridge",
        hub_module: "statistical_mechanics",
        hub_path: "umst-manifold/src/physics/solvers/statistical_mechanics.rs",
        feature_lane: "research",
        ported_out: false,
        hub_status: "HUB_IN_MANIFOLD",
    },
];

/// Fleet census row for nested A3 burn algebra + solver hub posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnAlgebraSolverDepthSummary {
    pub pbm_id: &'static str,
    pub parent_residue_id: &'static str,
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub production_landed: bool,
    pub nested_runtime_alias_present: bool,
    pub top_level_runtime_present: bool,
    pub github_301_deferred: bool,
    pub physics_solver_hub_not_ported: bool,
    pub always_on_solver_module_count: usize,
    pub feature_gated_solver_module_count: usize,
    pub supporting_solver_infra_module_count: usize,
    pub total_solver_source_file_count: usize,
    pub primary_solver_lane_count: usize,
    pub stable_lane_solver_count: usize,
    pub research_lane_solver_count: usize,
    pub manifold_hub_ported_lane_count: usize,
    pub manifold_hub_unported_lane_count: usize,
}

/// Count primary solver lanes still hubbing in manifold (not ported out).
#[must_use]
pub const fn manifold_hub_unported_lane_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < SOLVER_HUB_ROWS.len() {
        if !SOLVER_HUB_ROWS[i].ported_out {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count primary solver lanes ported out of manifold hub (honest: zero).
#[must_use]
pub const fn manifold_hub_ported_lane_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < SOLVER_HUB_ROWS.len() {
        if SOLVER_HUB_ROWS[i].ported_out {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Frozen depth summary — honest partial on manifold solver hub census only.
#[must_use]
pub const fn burn_algebra_solver_depth_summary() -> BurnAlgebraSolverDepthSummary {
    BurnAlgebraSolverDepthSummary {
        pbm_id: PBM_ID,
        parent_residue_id: PARENT_RESIDUE_ID,
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        production_landed: PRODUCTION_LANDED,
        nested_runtime_alias_present: NESTED_RUNTIME_ALIAS_PRESENT,
        top_level_runtime_present: TOP_LEVEL_RUNTIME_PRESENT,
        github_301_deferred: GITHUB_301_DEFERRED,
        physics_solver_hub_not_ported: PHYSICS_SOLVER_HUB_NOT_PORTED,
        always_on_solver_module_count: ALWAYS_ON_SOLVER_MODULE_COUNT,
        feature_gated_solver_module_count: FEATURE_GATED_SOLVER_MODULE_COUNT,
        supporting_solver_infra_module_count: SUPPORTING_SOLVER_INFRA_MODULE_COUNT,
        total_solver_source_file_count: TOTAL_SOLVER_SOURCE_FILE_COUNT,
        primary_solver_lane_count: PRIMARY_SOLVER_LANE_COUNT,
        stable_lane_solver_count: STABLE_LANE_SOLVER_COUNT,
        research_lane_solver_count: RESEARCH_LANE_SOLVER_COUNT,
        manifold_hub_ported_lane_count: manifold_hub_ported_lane_count(),
        manifold_hub_unported_lane_count: manifold_hub_unported_lane_count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burn_algebra_posture_metadata_locked() {
        assert_eq!(SLICE_ID, "slice-3");
        assert_eq!(SLICE3B_ID, "slice-3b");
        assert_eq!(SLICE3C_ID, "slice-3c");
        assert_eq!(SLICE3D_ID, "slice-3d");
        assert_eq!(PARENT_RESIDUE_ID, "R-faithful-decomp-B1");
        assert_eq!(R_ATOMS_SCALAR_ID, "R-atoms-scalar");
        assert_eq!(PBM_ID, "PBM-009");
        assert_eq!(PBM010_ID, "PBM-010");
        assert_eq!(POSTURE_TAG, "HONEST_PARTIAL");
        assert!(LIFT_STEP_LANDED);
        assert!(!PRODUCTION_LANDED);
        assert!(RANK1_PLUS_LEDGER_LANDED);
        assert!(!RANK1_PLUS_LIFT_LANDED);
        assert!(ADAPTER_SCAFFOLD_LANDED);
        assert!(OP_SPEC_LANDED);
        assert!(!RANK1_PLUS_IMPL_LANDED);
        assert!(SLICE_RESIDUAL_ROWS_LANDED);
        assert!(SLICE2_PROTOTYPE_LANDED);
    }

    #[test]
    fn burn_algebra_posture_slice_paths_honest() {
        assert!(SLICE2_PROTOTYPE_PATH.contains("continuum"));
        assert!(SLICE2_WITNESS_PATH.contains("b1_tensor_parity_probe"));
        assert!(SLICE3_LIFT_STEP_PATH.contains("atoms_tensor_lift"));
        assert!(SLICE3B_LEDGER_PATH.contains("atoms_tensor_lift_ledger"));
        assert!(SLICE_RESIDUAL_PATH.contains("atoms_tensor_lift_residual"));
        assert!(SLICE3C_ADAPTER_PATH.contains("atoms_tensor_lift_adapter"));
        assert!(SLICE3D_OPS_PATH.contains("atoms_tensor_lift_ops"));
        assert!(SLICE3_ADAPTER_PATH.contains("umst-algebra-burn"));
        assert!(DESIGN_DOC_PATH.contains("C2_TENSOR_ALGEBRA"));
        assert!(B6_GROWTH_TENSOR_BLOCKS_PRODUCTION);
        assert!(B6_GROWTH_TENSOR_DEFER_PATH.contains("umst-cartridge-active"));
        assert!(B6_FAITHFUL_WITNESS_PATH.contains("faithful_decomposition"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-009-B6_2127");
        assert_eq!(PRIOR_RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-009_2033");
        assert!(PBM010_RECEIPT_SLUG.contains("PBM-010_2001"));
        assert!(PBM010_SLICE_RESIDUAL_RECEIPT_SLUG.contains("PBM-010_2033"));
        assert!(PBM010_SLICE3C_RECEIPT_SLUG.contains("PBM-010_2127"));
        assert!(PBM010_SLICE3D_RECEIPT_SLUG.contains("SWARM-C25-0831-89"));
    }

    #[test]
    fn burn_algebra_posture_solver_hub_a3_nested() {
        assert!(NESTED_RUNTIME_ALIAS_PRESENT);
        assert!(!TOP_LEVEL_RUNTIME_PRESENT);
        assert!(GITHUB_301_DEFERRED);
        assert!(PHYSICS_SOLVER_HUB_NOT_PORTED);
        assert!(NESTED_RUNTIME_PATH.contains("umst-manifold/crates/umst-runtime"));
        assert!(PHYSICS_SOLVER_HUB_PATH.starts_with("umst-manifold/"));
        assert_eq!(ALWAYS_ON_SOLVER_MODULE_COUNT, 13);
        assert_eq!(FEATURE_GATED_SOLVER_MODULE_COUNT, 5);
        let summary = burn_algebra_solver_depth_summary();
        assert!(summary.nested_runtime_alias_present);
        assert!(!summary.top_level_runtime_present);
        assert!(summary.github_301_deferred);
        assert!(summary.physics_solver_hub_not_ported);
    }

    #[test]
    fn burn_algebra_posture_solver_hub_counts_measured() {
        assert_eq!(SUPPORTING_SOLVER_INFRA_MODULE_COUNT, 5);
        assert_eq!(
            ALWAYS_ON_SOLVER_MODULE_COUNT + FEATURE_GATED_SOLVER_MODULE_COUNT,
            TOTAL_SOLVER_SOURCE_FILE_COUNT
        );
        assert_eq!(PRIMARY_SOLVER_LANE_COUNT, 9);
        assert_eq!(
            STABLE_LANE_SOLVER_COUNT + RESEARCH_LANE_SOLVER_COUNT,
            PRIMARY_SOLVER_LANE_COUNT
        );
        assert_eq!(STABLE_LANE_SOLVER_COUNT, 1);
        assert_eq!(RESEARCH_LANE_SOLVER_COUNT, 8);
        let summary = burn_algebra_solver_depth_summary();
        assert_eq!(summary.always_on_solver_module_count, 13);
        assert_eq!(summary.feature_gated_solver_module_count, 5);
        assert_eq!(summary.total_solver_source_file_count, 18);
        assert_eq!(summary.primary_solver_lane_count, 9);
    }

    #[test]
    fn burn_algebra_posture_solver_hub_rows_manifold_only() {
        assert_eq!(SOLVER_HUB_ROWS.len(), 9);
        assert_eq!(manifold_hub_unported_lane_count(), 9);
        assert_eq!(manifold_hub_ported_lane_count(), 0);
        assert!(MECHANICS_SOLVER_HUB_PATH.contains("mechanics.rs"));
        assert!(SOLVER_STATUS_DOC_PATH.contains("Solver-Status"));
        assert_eq!(SOLVER_HUB_RECEIPT_SLUG, "PORT-RT-NEST-SOLVER-INV-W4");
        for row in SOLVER_HUB_ROWS {
            assert!(row.hub_path.starts_with("umst-manifold/"));
            assert!(!row.ported_out);
            assert_eq!(row.hub_status, "HUB_IN_MANIFOLD");
        }
        let stable: Vec<_> = SOLVER_HUB_ROWS
            .iter()
            .filter(|r| r.feature_lane == "stable")
            .collect();
        assert_eq!(stable.len(), 1);
        assert_eq!(stable[0].lane_id, "topology");
        let research: Vec<_> = SOLVER_HUB_ROWS
            .iter()
            .filter(|r| r.feature_lane == "research")
            .collect();
        assert_eq!(research.len(), 8);
        let summary = burn_algebra_solver_depth_summary();
        assert_eq!(summary.manifold_hub_unported_lane_count, 9);
        assert_eq!(summary.manifold_hub_ported_lane_count, 0);
        assert!(!summary.production_landed);
    }
}

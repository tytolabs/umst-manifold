// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `BurnAlgebra` slice-3d tensor op spec posture — rank-1+ defer anchor for `umst-runtime`.
//!
//! **Honest boundary:** `umst-runtime` is the designated `burn::Tensor` home per
//! [`docs/C2_TENSOR_ALGEBRA_DESIGN.md`](../../../../docs/C2_TENSOR_ALGEBRA_DESIGN.md).
//! Production `impl TensorAlgebra` over rank-1+ `burn::Tensor` is **not** landed — tensor op
//! spec census only. Burn tensor ops still **hub** in `umst-manifold/src/runtime/` until
//! `umst-algebra-burn` lands and GitHub 301 completes.
//!
//! Witness: [`atoms_tensor_lift_ops`](../../../../src/runtime/atoms_tensor_lift_ops.rs) · PBM-010.

/// Slice identifier for runtime rank-1+ tensor op spec.
pub const SLICE_ID: &str = "slice-3d";

/// Parent residue — F1 rank-1+ tensor path not closed.
pub const PARENT_RESIDUE_ID: &str = "R-atoms-scalar";

/// PBM-010 workstream cross-ref.
pub const PBM_ID: &str = "PBM-010";

/// Honest posture — op spec landed; production impl **open**.
pub const POSTURE_TAG: &str = "OP_SPEC_PARTIAL";

/// Whether slice-3d tensor op spec rows are landed.
pub const OP_SPEC_LANDED: bool = true;

/// Whether rank-1+ `impl TensorAlgebra` is landed.
pub const RANK1_PLUS_IMPL_LANDED: bool = false;

/// Whether the planned `umst-algebra-burn` crate exists.
pub const ADAPTER_CRATE_LANDED: bool = false;

/// Slice-3c adapter contract prerequisite (AGAP-2127-PBM-010).
pub const SLICE3C_ADAPTER_LANDED: bool = true;

/// Slice-3b ledger prerequisite (AGAP-2001-PBM-010).
pub const SLICE3B_LEDGER_LANDED: bool = true;

/// A3 nested `umst-runtime` alias present under `umst-manifold/crates/`.
pub const NESTED_RUNTIME_ALIAS_PRESENT: bool = true;

/// Top-level `umst-runtime/` crate absent (operator GitHub 301 deferred).
pub const TOP_LEVEL_RUNTIME_PRESENT: bool = false;

/// GitHub 301 rename deferred — `umst-manifold` remains burn-op dependency hub.
pub const GITHUB_301_DEFERRED: bool = true;

/// Whether 0D `TensorAlgebra` over `burn::Tensor` is landed in manifold hub.
pub const ZERO_D_TENSOR_ALGEBRA_LANDED: bool = true;

/// Measured tensor op spec row count (witness: `TENSOR_OP_SPEC_ROWS` in manifold).
pub const TENSOR_OP_SPEC_ROW_COUNT: usize = 6;

/// Rank-3 THMC field rows (T, H, u, d, alpha).
pub const RANK3_OP_SPEC_ROW_COUNT: usize = 5;

/// Rank-4 SmallStrain row.
pub const RANK4_OP_SPEC_ROW_COUNT: usize = 1;

/// Rows with `contract`/`grad` design specified (all 6 @ slice-3d deepen).
pub const DESIGN_SPECIFIED_ROW_COUNT: usize = 6;

/// Rows with deferred rank-1+ `impl TensorAlgebra` (all 6).
pub const IMPL_DEFERRED_ROW_COUNT: usize = 6;

/// Nested A3 alias path (present).
pub const NESTED_RUNTIME_PATH: &str = "umst-manifold/crates/umst-runtime/";

/// Slice-3d tensor op spec (landed @ SWARM-C25-0831-89).
pub const SLICE3D_OPS_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ops.rs";

/// Slice-3 0D lift step — only landed `TensorAlgebra` impl (rank-0).
pub const SLICE3_LIFT_STEP_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift.rs";

/// Slice-3c adapter contract scaffold cross-ref.
pub const SLICE3C_ADAPTER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_adapter.rs";

/// Slice-3b rank-1+ ledger cross-ref.
pub const SLICE3B_LEDGER_PATH: &str = "umst-manifold/src/runtime/atoms_tensor_lift_ledger.rs";

/// P3 field carrier SSOT (manifold hub).
pub const FIELD_SSOT_PATH: &str = "umst-manifold/src/core/field.rs";

/// Planned adapter crate (not created).
pub const ADAPTER_CRATE_PATH: &str = "umst-runtime/crates/umst-algebra-burn/";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Fleet receipt for PBM-010 slice-3d deepen.
pub const RECEIPT_SLUG: &str = "COMPLETION_SWARM_SWARM-C25-0831-89_0831";

/// Prior receipt (slice-3c adapter @ AGAP-2127).
pub const PRIOR_RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_PBM-010_2127";

/// One burn tensor op still hosted in `umst-manifold` hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnOpHubRow {
    /// `TensorAlgebra` op name (`add`, `mul`, `contract`, `grad`).
    pub op_name: &'static str,
    /// Burn tensor rank for this op surface (0 = landed 0D impl).
    pub tensor_rank: u8,
    /// Manifold hub module basename.
    pub hub_module: &'static str,
    /// Manifold hub source path.
    pub hub_path: &'static str,
    /// Whether production `impl TensorAlgebra` is landed for this rank.
    pub impl_landed: bool,
    /// Honest hub status — `LANDED`, `DESIGN_SPECIFIED`, or `DEFERRED`.
    pub hub_status: &'static str,
}

/// Frozen census — which burn ops still hub in `umst-manifold` (measured @ W3 deepen).
pub const BURN_OP_HUB_ROWS: &[BurnOpHubRow] = &[
    BurnOpHubRow {
        op_name: "add",
        tensor_rank: 0,
        hub_module: "atoms_tensor_lift",
        hub_path: SLICE3_LIFT_STEP_PATH,
        impl_landed: true,
        hub_status: "LANDED",
    },
    BurnOpHubRow {
        op_name: "mul",
        tensor_rank: 0,
        hub_module: "atoms_tensor_lift",
        hub_path: SLICE3_LIFT_STEP_PATH,
        impl_landed: true,
        hub_status: "LANDED",
    },
    BurnOpHubRow {
        op_name: "contract",
        tensor_rank: 0,
        hub_module: "atoms_tensor_lift",
        hub_path: SLICE3_LIFT_STEP_PATH,
        impl_landed: true,
        hub_status: "LANDED",
    },
    BurnOpHubRow {
        op_name: "grad",
        tensor_rank: 0,
        hub_module: "atoms_tensor_lift",
        hub_path: SLICE3_LIFT_STEP_PATH,
        impl_landed: true,
        hub_status: "LANDED",
    },
    BurnOpHubRow {
        op_name: "contract",
        tensor_rank: 3,
        hub_module: "atoms_tensor_lift_ops",
        hub_path: SLICE3D_OPS_PATH,
        impl_landed: false,
        hub_status: "DESIGN_SPECIFIED",
    },
    BurnOpHubRow {
        op_name: "grad",
        tensor_rank: 3,
        hub_module: "atoms_tensor_lift_ops",
        hub_path: SLICE3D_OPS_PATH,
        impl_landed: false,
        hub_status: "DESIGN_SPECIFIED",
    },
    BurnOpHubRow {
        op_name: "contract",
        tensor_rank: 4,
        hub_module: "atoms_tensor_lift_ops",
        hub_path: SLICE3D_OPS_PATH,
        impl_landed: false,
        hub_status: "DESIGN_SPECIFIED",
    },
    BurnOpHubRow {
        op_name: "grad",
        tensor_rank: 4,
        hub_module: "atoms_tensor_lift_ops",
        hub_path: SLICE3D_OPS_PATH,
        impl_landed: false,
        hub_status: "DESIGN_SPECIFIED",
    },
];

/// Fleet census row for slice-3d burn op hub posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BurnAlgebraOpsDepthSummary {
    pub pbm_id: &'static str,
    pub parent_residue_id: &'static str,
    pub slice_id: &'static str,
    pub posture_tag: &'static str,
    pub op_spec_landed: bool,
    pub rank1_plus_impl_landed: bool,
    pub adapter_crate_landed: bool,
    pub nested_runtime_alias_present: bool,
    pub top_level_runtime_present: bool,
    pub github_301_deferred: bool,
    pub zero_d_tensor_algebra_landed: bool,
    pub tensor_op_spec_row_count: usize,
    pub design_specified_row_count: usize,
    pub impl_deferred_row_count: usize,
    pub manifold_hub_landed_op_count: usize,
    pub manifold_hub_deferred_op_count: usize,
}

/// Count hub rows with landed production impl (0D only).
#[must_use]
pub const fn manifold_hub_landed_op_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < BURN_OP_HUB_ROWS.len() {
        if BURN_OP_HUB_ROWS[i].impl_landed {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count hub rows with deferred rank-1+ impl.
#[must_use]
pub const fn manifold_hub_deferred_op_count() -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < BURN_OP_HUB_ROWS.len() {
        if !BURN_OP_HUB_ROWS[i].impl_landed {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Frozen depth summary — honest op spec partial on manifold hub census only.
#[must_use]
pub const fn burn_algebra_ops_depth_summary() -> BurnAlgebraOpsDepthSummary {
    BurnAlgebraOpsDepthSummary {
        pbm_id: PBM_ID,
        parent_residue_id: PARENT_RESIDUE_ID,
        slice_id: SLICE_ID,
        posture_tag: POSTURE_TAG,
        op_spec_landed: OP_SPEC_LANDED,
        rank1_plus_impl_landed: RANK1_PLUS_IMPL_LANDED,
        adapter_crate_landed: ADAPTER_CRATE_LANDED,
        nested_runtime_alias_present: NESTED_RUNTIME_ALIAS_PRESENT,
        top_level_runtime_present: TOP_LEVEL_RUNTIME_PRESENT,
        github_301_deferred: GITHUB_301_DEFERRED,
        zero_d_tensor_algebra_landed: ZERO_D_TENSOR_ALGEBRA_LANDED,
        tensor_op_spec_row_count: TENSOR_OP_SPEC_ROW_COUNT,
        design_specified_row_count: DESIGN_SPECIFIED_ROW_COUNT,
        impl_deferred_row_count: IMPL_DEFERRED_ROW_COUNT,
        manifold_hub_landed_op_count: manifold_hub_landed_op_count(),
        manifold_hub_deferred_op_count: manifold_hub_deferred_op_count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burn_algebra_ops_posture_metadata_locked() {
        let summary = burn_algebra_ops_depth_summary();
        assert_eq!(SLICE_ID, "slice-3d");
        assert_eq!(PARENT_RESIDUE_ID, "R-atoms-scalar");
        assert_eq!(PBM_ID, "PBM-010");
        assert_eq!(POSTURE_TAG, "OP_SPEC_PARTIAL");
        assert!(OP_SPEC_LANDED);
        assert!(!RANK1_PLUS_IMPL_LANDED);
        assert!(!ADAPTER_CRATE_LANDED);
        assert!(SLICE3C_ADAPTER_LANDED);
        assert!(SLICE3B_LEDGER_LANDED);
        assert_eq!(summary.posture_tag, POSTURE_TAG);
        assert!(!summary.rank1_plus_impl_landed);
    }

    #[test]
    fn burn_algebra_ops_posture_slice_paths_honest() {
        assert!(SLICE3D_OPS_PATH.contains("atoms_tensor_lift_ops"));
        assert!(SLICE3_LIFT_STEP_PATH.contains("atoms_tensor_lift"));
        assert!(SLICE3C_ADAPTER_PATH.contains("atoms_tensor_lift_adapter"));
        assert!(SLICE3B_LEDGER_PATH.contains("atoms_tensor_lift_ledger"));
        assert!(FIELD_SSOT_PATH.contains("field.rs"));
        assert!(ADAPTER_CRATE_PATH.contains("umst-algebra-burn"));
        assert!(DESIGN_DOC_PATH.contains("C2_TENSOR_ALGEBRA"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_SWARM_SWARM-C25-0831-89_0831");
        assert_eq!(PRIOR_RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_PBM-010_2127");
    }

    #[test]
    fn burn_algebra_ops_a3_nested_alias_honest() {
        assert!(NESTED_RUNTIME_ALIAS_PRESENT);
        assert!(!TOP_LEVEL_RUNTIME_PRESENT);
        assert!(GITHUB_301_DEFERRED);
        assert!(NESTED_RUNTIME_PATH.contains("umst-manifold/crates/umst-runtime"));
        let summary = burn_algebra_ops_depth_summary();
        assert!(summary.nested_runtime_alias_present);
        assert!(!summary.top_level_runtime_present);
        assert!(summary.github_301_deferred);
    }

    #[test]
    fn burn_algebra_ops_tensor_spec_counts_measured() {
        assert_eq!(TENSOR_OP_SPEC_ROW_COUNT, 6);
        assert_eq!(RANK3_OP_SPEC_ROW_COUNT + RANK4_OP_SPEC_ROW_COUNT, 6);
        assert_eq!(DESIGN_SPECIFIED_ROW_COUNT, 6);
        assert_eq!(IMPL_DEFERRED_ROW_COUNT, 6);
        let summary = burn_algebra_ops_depth_summary();
        assert_eq!(summary.tensor_op_spec_row_count, 6);
        assert_eq!(summary.design_specified_row_count, 6);
        assert_eq!(summary.impl_deferred_row_count, 6);
    }

    #[test]
    fn burn_algebra_ops_hub_rows_manifold_only() {
        assert_eq!(BURN_OP_HUB_ROWS.len(), 8);
        assert_eq!(manifold_hub_landed_op_count(), 4);
        assert_eq!(manifold_hub_deferred_op_count(), 4);
        for row in BURN_OP_HUB_ROWS {
            assert!(row.hub_path.starts_with("umst-manifold/"));
            assert!(!row.hub_path.contains("umst-algebra-burn"));
        }
        let landed: Vec<_> = BURN_OP_HUB_ROWS.iter().filter(|r| r.impl_landed).collect();
        assert_eq!(landed.len(), 4);
        for row in landed {
            assert_eq!(row.tensor_rank, 0);
            assert_eq!(row.hub_status, "LANDED");
            assert_eq!(row.hub_module, "atoms_tensor_lift");
        }
        let deferred: Vec<_> = BURN_OP_HUB_ROWS.iter().filter(|r| !r.impl_landed).collect();
        assert_eq!(deferred.len(), 4);
        for row in deferred {
            assert!(row.tensor_rank == 3 || row.tensor_rank == 4);
            assert_eq!(row.hub_status, "DESIGN_SPECIFIED");
            assert_eq!(row.hub_module, "atoms_tensor_lift_ops");
        }
    }

    #[test]
    fn burn_algebra_ops_zero_d_landed_rank1_plus_open() {
        assert!(ZERO_D_TENSOR_ALGEBRA_LANDED);
        assert!(!RANK1_PLUS_IMPL_LANDED);
        assert!(!ADAPTER_CRATE_LANDED);
        let summary = burn_algebra_ops_depth_summary();
        assert!(summary.zero_d_tensor_algebra_landed);
        assert_eq!(summary.manifold_hub_landed_op_count, 4);
        assert_eq!(summary.manifold_hub_deferred_op_count, 4);
    }
}

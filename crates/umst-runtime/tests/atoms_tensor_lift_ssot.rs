// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-1920-PBM-010 — `umst-runtime` alias surfaces slice-3 atom tensor lift step.

use umst_cartridge_api::TensorAlgebra;
use umst_runtime::burn_algebra_posture::{
    ADAPTER_SCAFFOLD_LANDED, LIFT_STEP_LANDED, OP_SPEC_LANDED, PBM010_ID, PBM010_RECEIPT_SLUG,
    PBM010_SLICE3C_RECEIPT_SLUG, PBM010_SLICE3D_RECEIPT_SLUG, PBM010_SLICE_RESIDUAL_RECEIPT_SLUG,
    RANK1_PLUS_IMPL_LANDED, RANK1_PLUS_LEDGER_LANDED, RANK1_PLUS_LIFT_LANDED, SLICE3B_ID,
    SLICE3B_LEDGER_PATH, SLICE3C_ADAPTER_PATH, SLICE3C_ID, SLICE3D_ID, SLICE3D_OPS_PATH,
    SLICE3_LIFT_STEP_PATH, SLICE_RESIDUAL_PATH, SLICE_RESIDUAL_ROWS_LANDED,
};
use umst_runtime::runtime::atoms_tensor_lift::{
    atoms_tensor_lift_depth_summary, lift_atom_scalar, BurnAtomAlgebra,
    LIFT_STEP_LANDED as ATOMS_LIFT, PBM_ID, RANK1_PLUS_DEFERRED,
};
use umst_runtime::runtime::atoms_tensor_lift_adapter::{
    adapter_deferred_row_count, atoms_tensor_lift_adapter_depth_summary, ADAPTER_CONTRACT_ROWS,
    ADAPTER_SCAFFOLD_LANDED as ADAPTER_SCAFFOLD,
};
use umst_runtime::runtime::atoms_tensor_lift_ledger::{
    atoms_tensor_lift_ledger_depth_summary, rank1_plus_open_row_count,
    RANK1_PLUS_LEDGER_LANDED as LEDGER_LANDED, RANK1_PLUS_LEDGER_ROWS,
};
use umst_runtime::runtime::atoms_tensor_lift_ops::{
    atoms_tensor_lift_ops_depth_summary, op_design_specified_row_count, op_impl_deferred_row_count,
    OP_SPEC_LANDED as OPS_LANDED, TENSOR_OP_SPEC_ROWS,
};
use umst_runtime::runtime::atoms_tensor_lift_residual::{
    atoms_tensor_lift_residual_depth_summary, f1_fully_closed, slice_residual_blocking_row_count,
    slice_residual_open_row_count, F1SliceResidualId, SLICE_RESIDUAL_ROWS,
    SLICE_RESIDUAL_ROWS_LANDED as RESIDUAL_LANDED,
};

#[test]
fn runtime_alias_surfaces_pbm010_tensor_lift_posture() {
    assert_eq!(PBM010_ID, "PBM-010");
    assert_eq!(SLICE3B_ID, "slice-3b");
    assert!(LIFT_STEP_LANDED);
    assert!(RANK1_PLUS_LEDGER_LANDED);
    assert!(!RANK1_PLUS_LIFT_LANDED);
    assert!(SLICE3_LIFT_STEP_PATH.contains("atoms_tensor_lift"));
    assert!(SLICE3B_LEDGER_PATH.contains("atoms_tensor_lift_ledger"));
    assert!(SLICE_RESIDUAL_PATH.contains("atoms_tensor_lift_residual"));
    assert!(SLICE3C_ADAPTER_PATH.contains("atoms_tensor_lift_adapter"));
    assert_eq!(SLICE3C_ID, "slice-3c");
    assert_eq!(SLICE3D_ID, "slice-3d");
    assert!(ADAPTER_SCAFFOLD_LANDED);
    assert!(OP_SPEC_LANDED);
    assert!(!RANK1_PLUS_IMPL_LANDED);
    assert!(SLICE_RESIDUAL_ROWS_LANDED);
    assert!(SLICE3D_OPS_PATH.contains("atoms_tensor_lift_ops"));
    assert!(PBM010_RECEIPT_SLUG.contains("PBM-010_2001"));
    assert!(PBM010_SLICE_RESIDUAL_RECEIPT_SLUG.contains("PBM-010_2033"));
    assert!(PBM010_SLICE3C_RECEIPT_SLUG.contains("PBM-010_2127"));
    assert!(PBM010_SLICE3D_RECEIPT_SLUG.contains("SWARM-C25-0831-89"));
}

#[test]
fn runtime_alias_atoms_tensor_lift_depth_summary() {
    let summary = atoms_tensor_lift_depth_summary();
    assert_eq!(summary.pbm_id, PBM_ID);
    assert!(ATOMS_LIFT);
    assert!(RANK1_PLUS_DEFERRED);
}

#[test]
fn runtime_alias_lift_atom_scalar_produces_burn_tensor_field() {
    let device = Default::default();
    let field = lift_atom_scalar(&device, 1.2e-4);
    let host = field.to_host_scalar();
    assert!((host - 1.2e-4).abs() < 1e-5);
}

#[test]
fn runtime_alias_burn_atom_algebra_is_real_tensor_instance() {
    let device = Default::default();
    let z = <BurnAtomAlgebra as TensorAlgebra>::zero();
    assert!((z.to_host_scalar()).abs() < f64::EPSILON);
    let lifted = lift_atom_scalar(&device, 4.0);
    let doubled = <BurnAtomAlgebra as TensorAlgebra>::mul(lifted, lift_atom_scalar(&device, 2.0));
    assert!((doubled.to_host_scalar() - 8.0).abs() < 1e-5);
}

#[test]
fn runtime_alias_rank1_plus_ledger_depth_summary() {
    let ledger = atoms_tensor_lift_ledger_depth_summary();
    assert_eq!(ledger.slice_id, "slice-3b");
    assert!(LEDGER_LANDED);
    assert!(!ledger.rank1_plus_lift_landed);
    assert_eq!(rank1_plus_open_row_count(), 6);
    assert_eq!(RANK1_PLUS_LEDGER_ROWS.len(), 6);
    let lift = atoms_tensor_lift_depth_summary();
    assert_eq!(lift.slice_id, "slice-3");
    assert!(ATOMS_LIFT);
    assert!(RANK1_PLUS_DEFERRED);
}

#[test]
fn runtime_alias_slice_residual_rows_depth_summary() {
    let residual = atoms_tensor_lift_residual_depth_summary();
    assert_eq!(residual.pbm_id, "PBM-010");
    assert_eq!(residual.parent_residue_id, "R-atoms-scalar");
    assert!(RESIDUAL_LANDED);
    assert!(!residual.f1_fully_closed);
    assert!(!f1_fully_closed());
    assert_eq!(slice_residual_open_row_count(), 1);
    assert_eq!(slice_residual_blocking_row_count(), 4);
    assert_eq!(SLICE_RESIDUAL_ROWS.len(), 8);
    assert!(SLICE_RESIDUAL_ROWS
        .iter()
        .any(|r| r.id == F1SliceResidualId::Slice3cBurnAdapter));
    assert!(SLICE_RESIDUAL_ROWS
        .iter()
        .any(|r| r.id == F1SliceResidualId::Slice3dTensorOps));
}

#[test]
fn runtime_alias_slice3d_ops_depth_summary() {
    let ops = atoms_tensor_lift_ops_depth_summary();
    assert_eq!(ops.slice_id, "slice-3d");
    assert!(OPS_LANDED);
    assert!(!ops.rank1_plus_impl_landed);
    assert!(!ops.adapter_crate_landed);
    assert_eq!(op_design_specified_row_count(), 6);
    assert_eq!(op_impl_deferred_row_count(), 6);
    assert_eq!(TENSOR_OP_SPEC_ROWS.len(), 6);
    assert_eq!(ADAPTER_CONTRACT_ROWS.len(), 6);
}

#[test]
fn runtime_alias_slice3c_adapter_depth_summary() {
    let adapter = atoms_tensor_lift_adapter_depth_summary();
    assert_eq!(adapter.slice_id, "slice-3c");
    assert!(ADAPTER_SCAFFOLD);
    assert!(!adapter.rank1_plus_impl_landed);
    assert!(!adapter.adapter_crate_landed);
    assert_eq!(adapter_deferred_row_count(), 6);
    assert_eq!(ADAPTER_CONTRACT_ROWS.len(), 6);
    assert_eq!(RANK1_PLUS_LEDGER_ROWS.len(), 6);
}

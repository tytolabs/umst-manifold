// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2037-LIB-NALGEBRA — `umst-runtime` alias surfaces slice-4 nalgebra tensor lift scaffold.

use umst_cartridge_api::TensorAlgebra;
use umst_runtime::nalgebra_algebra_posture::{
    BLUEPRINT_ROW, CARTRIDGE_LIFT_LANDED, LIB_ID, NUM_DUAL_LANDED, POSTURE_TAG, RECEIPT_SLUG,
    SCAFFOLD_LANDED, SLICE4_SCAFFOLD_PATH, SLICE_ID,
};
use umst_runtime::runtime::nalgebra_algebra::{
    lift_strain_voigt6, nalgebra_tensor_lift_depth_summary, NalgebraAlgebra,
    CARTRIDGE_LIFT_DEFERRED, NUM_DUAL_DEFERRED, SCAFFOLD_LANDED as ATOMS_SCAFFOLD,
};

#[test]
fn runtime_alias_surfaces_lib_nalgebra_tensor_lift_posture() {
    assert_eq!(LIB_ID, "LIB-LEARN-F-NALGEBRA");
    assert_eq!(BLUEPRINT_ROW, "F-09");
    assert_eq!(SLICE_ID, "slice-4");
    assert_eq!(POSTURE_TAG, "HONEST_PARTIAL");
    assert!(SCAFFOLD_LANDED);
    assert!(!CARTRIDGE_LIFT_LANDED);
    assert!(NUM_DUAL_LANDED);
    assert!(SLICE4_SCAFFOLD_PATH.contains("nalgebra_algebra"));
    assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_LIB-NALGEBRA_2037");
}

#[test]
fn runtime_alias_nalgebra_tensor_lift_depth_summary() {
    let summary = nalgebra_tensor_lift_depth_summary();
    assert_eq!(summary.lib_id, LIB_ID);
    assert_eq!(summary.slice_id, "slice-4");
    assert!(ATOMS_SCAFFOLD);
    assert!(CARTRIDGE_LIFT_DEFERRED);
    assert!(!NUM_DUAL_DEFERRED);
}

#[test]
fn runtime_alias_lift_strain_voigt6_produces_matrix3_field() {
    let field = lift_strain_voigt6([1.0e-4, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let voigt = field.to_voigt6_symmetric();
    assert!((voigt[0] - 1.0e-4).abs() < 1e-12);
}

#[test]
fn runtime_alias_nalgebra_algebra_is_real_tensor_instance() {
    let z = <NalgebraAlgebra as TensorAlgebra>::zero();
    assert!(z.frobenius_norm().abs() < f64::EPSILON);
    let a = lift_strain_voigt6([2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let b = lift_strain_voigt6([3.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let product = <NalgebraAlgebra as TensorAlgebra>::mul(a, b);
    let sum = <NalgebraAlgebra as TensorAlgebra>::add(product, z);
    assert!((sum.to_voigt6_symmetric()[0] - 6.0).abs() < 1e-12);
}

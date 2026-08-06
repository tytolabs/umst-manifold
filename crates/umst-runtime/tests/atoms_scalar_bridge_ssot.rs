// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-1920-B4-076 — `umst-runtime` alias surfaces R-ATOMS-SC-05 cast bridge unchanged.

use umst_runtime::gate::transition_proposal::ThermodynamicStateSnapshot;
use umst_runtime::runtime::atoms_scalar_bridge::{
    atoms_scalar_runtime_depth_summary, thermodynamic_snapshot_from_burn_lane,
    thermodynamic_snapshot_to_burn_lane, PRODUCTION_TENSOR_DEFERRED, RUNTIME_BRIDGE_LANDED,
    SUB_RESIDUE_ID,
};

#[test]
fn runtime_alias_surfaces_atoms_scalar_bridge() {
    let summary = atoms_scalar_runtime_depth_summary();
    assert_eq!(summary.sub_residue_id, SUB_RESIDUE_ID);
    assert!(RUNTIME_BRIDGE_LANDED);
    assert!(PRODUCTION_TENSOR_DEFERRED);
}

#[test]
fn runtime_alias_burn_lane_cast_preserves_gate_shape() {
    let snapshot = ThermodynamicStateSnapshot::from_mix_calibrated(0.42, 0.25, 300.0, 35.0);
    let lane = thermodynamic_snapshot_to_burn_lane(&snapshot);
    let restored = thermodynamic_snapshot_from_burn_lane(&lane);
    assert!((restored.density - snapshot.density).abs() < 1e-2);
    assert!((restored.temperature - snapshot.temperature).abs() < 1e-3);
}

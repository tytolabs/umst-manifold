// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// AGAP-2033-MANIFOLD-SEM — HCOM-006 semantic lanes × WEB-005 constitutive residual bridge.

use umst_gate::ConjunctVerdict;
use umst_manifold::core::{LANE_RELATION_GRAPH, LANE_TOPOLOGY_SIGNATURE, SEMANTIC_LANE_BASE};
use umst_manifold::gate::{
    canonical_web_semantic_gate_outcome, canonical_web_transition_from_tensors_with_semantic,
};
use umst_manifold::web_constitutive::{
    semantic_transition_witness_from_tensors, slice_layout, web_semantic_lane_overlap_valid,
    WebConstitutiveModel, DEFAULT_INT_TOLERANCE, DEFAULT_SEMANTIC_DEFECT_TOLERANCE,
};
use umst_manifold::runtime::catalog::pin_witness_ok;

#[test]
fn semantic_lane_overlap_links_web_behavior_ucrs_head() {
    assert!(web_semantic_lane_overlap_valid());
    assert_eq!(slice_layout::BEHAVIOR_UCRS.start, 56);
    assert_eq!(SEMANTIC_LANE_BASE, 57);
}

#[test]
fn catalog_pin_witness_ok_on_manifold_ssot() {
    pin_witness_ok().expect("catalog pin witness");
}

#[test]
fn semantic_residual_rejects_dec_defect_on_tensor_path() {
    let model = WebConstitutiveModel::cartridge();
    let old = [0.0_f64; slice_layout::DIM];
    let mut new = [0.0_f64; slice_layout::DIM];
    new[0] = 1.0;
    new[LANE_RELATION_GRAPH] = 0.8;

    let (_, semantic_witness, _, _, _, semantic, composed) =
        canonical_web_transition_from_tensors_with_semantic(
            &model,
            &old,
            &new,
            DEFAULT_INT_TOLERANCE,
            DEFAULT_SEMANTIC_DEFECT_TOLERANCE,
        );

    assert!(!semantic.is_accepted());
    assert!(!semantic.dec_defect_ok);
    assert_ne!(composed, ConjunctVerdict::Accepted);
    assert_eq!(
        canonical_web_semantic_gate_outcome(&semantic_witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE),
        semantic
    );
}

#[test]
fn semantic_residual_accepts_consistent_lanes_on_tensor_path() {
    let model = WebConstitutiveModel::cartridge();
    let mut old = [0.0_f64; slice_layout::DIM];
    let mut new = [0.0_f64; slice_layout::DIM];
    old[0] = 1.0;
    new[0] = 1.0;
    new[LANE_RELATION_GRAPH] = 0.2;
    new[LANE_TOPOLOGY_SIGNATURE] = 1.0;
    new[SEMANTIC_LANE_BASE + 5] = 6.0; // mi_value meets chair fixture

    let (_, _, _, _, web, semantic, composed) =
        canonical_web_transition_from_tensors_with_semantic(
            &model,
            &old,
            &new,
            DEFAULT_INT_TOLERANCE,
            DEFAULT_SEMANTIC_DEFECT_TOLERANCE,
        );

    assert!(web.cost_legs_valid);
    assert!(semantic.is_accepted());
    assert_eq!(composed, ConjunctVerdict::Accepted);
}

#[test]
fn semantic_witness_from_tensors_matches_route_delegation() {
    let old = [0.0_f64; slice_layout::DIM];
    let mut new = [0.0_f64; slice_layout::DIM];
    new[LANE_RELATION_GRAPH] = 0.4;
    let witness = semantic_transition_witness_from_tensors(&old, &new);
    let routed = canonical_web_semantic_gate_outcome(&witness, DEFAULT_SEMANTIC_DEFECT_TOLERANCE);
    assert!(!routed.is_accepted());
}

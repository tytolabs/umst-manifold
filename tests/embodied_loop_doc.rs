// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! M5-C11 doc→code crosswalk for the constitutional embodied loop.

use umst_manifold::embodied::{
    crosswalk_for, doc_code_gaps, loop_closed_per_spec, loop_composition_pct, phase_posture,
    phases_loop_composed, phases_with_code_anchor, DocCodePosture, FUNNEL_SPEC, LOOP_CROSSWALK,
    LoopPhase,
};

#[test]
fn m5_c11_crosswalk_authority() {
    assert_eq!(LOOP_CROSSWALK.len(), 6);
    assert_eq!(phases_with_code_anchor(), 5);
    assert_eq!(phases_loop_composed(), 0);
    assert_eq!(loop_composition_pct(), 0);
    assert!(!loop_closed_per_spec());
}

#[test]
fn sense_leg_partial_world_observation_gap() {
    let row = crosswalk_for(LoopPhase::Sense).expect("sense row");
    assert!(row.code_anchor.contains("FieldSense"));
    assert!(matches!(
        phase_posture(LoopPhase::Sense),
        DocCodePosture::Partial { gap } if gap.contains("WorldObservation")
    ));
}

#[test]
fn command_leg_no_embodied_gateway_route() {
    let row = crosswalk_for(LoopPhase::Command).expect("command row");
    assert!(row.code_anchor.contains("gate_check_r"));
    assert!(matches!(
        phase_posture(LoopPhase::Command),
        DocCodePosture::Partial { gap } if gap.contains("embodied")
    ));
}

#[test]
fn present_leg_scaffold_present_fn_live() {
    let row = crosswalk_for(LoopPhase::Present).expect("present row");
    assert!(row.code_anchor.contains("scene.rs::present"));
    assert_eq!(phase_posture(LoopPhase::Present), DocCodePosture::Scaffold);
}

#[test]
fn actuate_leg_scaffold_robot_adapter() {
    let row = crosswalk_for(LoopPhase::Actuate).expect("actuate row");
    assert!(row.code_anchor.contains("RobotAdapter"));
    assert_eq!(phase_posture(LoopPhase::Actuate), DocCodePosture::Scaffold);
}

#[test]
fn loop_close_absent() {
    assert_eq!(
        phase_posture(LoopPhase::LoopClose),
        DocCodePosture::Absent
    );
}

#[test]
fn doc_code_gaps_match_crosswalk_rows() {
    assert_eq!(doc_code_gaps().len(), LOOP_CROSSWALK.len());
}

#[test]
fn funnel_spec_pinned() {
    assert_eq!(
        FUNNEL_SPEC,
        "sense → command → gate → {present, actuate} → sense"
    );
}

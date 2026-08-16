// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! W2-35 orchestrator fragment audit — extends W1-19 embodied loop wiring.

use umst_manifold::embodied::{
    audit_report, fragment_status, phase_wired, scaffold_coverage_pct, unwired_gaps, ActuateDesign,
    EmbodiedFragment, EmbodiedLoopSlots, FieldSenseClient, FragmentWireStatus, LoopPhase,
    NullFieldSenseClient, NullRobotExecutor, NullSenseLoopCloser, NullXrPresenter, RobotExecutor,
    SenseLoopCloser, XrPresenter,
};

#[test]
fn fragment_audit_matches_1052_authority() {
    let report = audit_report();
    assert_eq!(report.len(), 7);

    let wired: Vec<_> = report
        .iter()
        .filter(|(_, s)| s.is_wired())
        .map(|(f, _)| *f)
        .collect();
    assert_eq!(
        wired,
        vec![
            EmbodiedFragment::ManifoldGateway,
            EmbodiedFragment::HostTransitionGates,
        ]
    );

    assert!(matches!(
        fragment_status(EmbodiedFragment::ThermodynamicCbf),
        FragmentWireStatus::Partial { gap } if gap.contains("robot")
    ));
}

#[test]
fn scaffold_coverage_honest_at_22_pct() {
    assert_eq!(scaffold_coverage_pct(), 22);
}

#[test]
fn unwired_gaps_enumerate_78_pct_hole() {
    let gaps = unwired_gaps();
    assert_eq!(gaps.len(), 4);
    assert!(gaps.iter().any(|g| g.contains("umst-field")));
    assert!(gaps.iter().any(|g| g.contains("umst-xr")));
    assert!(gaps.iter().any(|g| g.contains("umst-robots")));
    assert!(gaps.iter().any(|g| g.contains("loop close")));
}

#[test]
fn loop_phases_only_gate_wired() {
    assert!(phase_wired(LoopPhase::Gate));
    for phase in [
        LoopPhase::Sense,
        LoopPhase::Command,
        LoopPhase::Present,
        LoopPhase::Actuate,
        LoopPhase::LoopClose,
    ] {
        assert!(!phase_wired(phase), "{phase:?} must remain unwired");
    }
}

#[test]
fn embodied_loop_slots_default_fail_closed() {
    let slots = EmbodiedLoopSlots::new();
    assert!(!slots.all_gaps_filled());
    assert_eq!(EmbodiedLoopSlots::missing_slots().len(), 4);
}

#[test]
fn null_slot_impls_reject_without_panic() {
    let mut sense = NullFieldSenseClient;
    let present = NullXrPresenter;
    let mut actuate = NullRobotExecutor;
    let mut closer = NullSenseLoopCloser;

    assert!(sense.sense().is_err());
    assert!(present.present(&[0u8; 32]).is_err());
    assert!(actuate
        .actuate(&ActuateDesign {
            design_digest: [0u8; 32],
        })
        .is_err());
    assert!(closer.close_loop().is_err());
}

#[test]
fn slots_populate_enables_gap_fill_check() {
    let mut slots = EmbodiedLoopSlots::new();
    slots.field_sense = Some(Box::new(RecordingFieldSense));
    slots.xr_present = Some(Box::new(RecordingXrPresenter));
    slots.robot_actuate = Some(Box::new(RecordingRobotExecutor));
    slots.loop_close = Some(Box::new(RecordingLoopCloser));
    assert!(slots.all_gaps_filled());
}

struct RecordingFieldSense;

impl umst_manifold::embodied::FieldSenseClient for RecordingFieldSense {
    fn sense(
        &mut self,
    ) -> Result<umst_manifold::embodied::SenseObservation, umst_manifold::embodied::FieldSenseError>
    {
        Ok(umst_manifold::embodied::SenseObservation {
            witness_digest: [1u8; 32],
        })
    }
}

struct RecordingXrPresenter;

impl umst_manifold::embodied::XrPresenter for RecordingXrPresenter {
    fn present(
        &self,
        digest: &[u8; 32],
    ) -> Result<umst_manifold::embodied::PresentScene, umst_manifold::embodied::PresentError> {
        Ok(umst_manifold::embodied::PresentScene {
            scene_digest: *digest,
        })
    }
}

struct RecordingRobotExecutor;

impl umst_manifold::embodied::RobotExecutor for RecordingRobotExecutor {
    fn actuate(
        &mut self,
        design: &umst_manifold::embodied::ActuateDesign,
    ) -> Result<(), umst_manifold::embodied::ActuateError> {
        let _ = design;
        Ok(())
    }
}

struct RecordingLoopCloser;

impl umst_manifold::embodied::SenseLoopCloser for RecordingLoopCloser {
    fn close_loop(
        &mut self,
    ) -> Result<umst_manifold::embodied::SenseObservation, umst_manifold::embodied::LoopCloseError>
    {
        Ok(umst_manifold::embodied::SenseObservation {
            witness_digest: [2u8; 32],
        })
    }
}

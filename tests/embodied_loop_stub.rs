// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! M5-C07 orchestrator embodied-loop stub — integration tests (not W1-19).

use umst_manifold::embodied::{
    embodied_loop_tick_stub, ActuateDesign, EmbodiedLoopSlots, EmbodiedLoopStub, FieldSenseClient,
    FieldSenseError, LoopCloseError, LoopStubReject, LoopTickPhase, PresentError, PresentScene,
    RobotExecutor, SenseLoopCloser, SenseObservation, XrPresenter,
};

struct RecordingSense;

impl FieldSenseClient for RecordingSense {
    fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
        Ok(SenseObservation {
            witness_digest: [0x11; 32],
        })
    }
}

struct RecordingPresent;

impl XrPresenter for RecordingPresent {
    fn present(&self, digest: &[u8; 32]) -> Result<PresentScene, PresentError> {
        Ok(PresentScene {
            scene_digest: *digest,
        })
    }
}

struct RecordingActuate;

impl RobotExecutor for RecordingActuate {
    fn actuate(
        &mut self,
        _design: &ActuateDesign,
    ) -> Result<(), umst_manifold::embodied::ActuateError> {
        Ok(())
    }
}

struct RecordingCloser;

impl SenseLoopCloser for RecordingCloser {
    fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError> {
        Ok(SenseObservation {
            witness_digest: [0x22; 32],
        })
    }
}

fn full_slots() -> EmbodiedLoopSlots {
    let mut slots = EmbodiedLoopSlots::new();
    slots.field_sense = Some(Box::new(RecordingSense));
    slots.xr_present = Some(Box::new(RecordingPresent));
    slots.robot_actuate = Some(Box::new(RecordingActuate));
    slots.loop_close = Some(Box::new(RecordingCloser));
    slots
}

#[test]
fn m5_c07_stub_sequences_constitutional_phases() {
    let result = embodied_loop_tick_stub(full_slots()).expect("full tick");
    assert_eq!(
        result.phases_completed,
        vec![
            LoopTickPhase::Sense,
            LoopTickPhase::Command,
            LoopTickPhase::Gate,
            LoopTickPhase::Present,
            LoopTickPhase::Actuate,
            LoopTickPhase::LoopClose,
        ]
    );
    assert_eq!(result.scaffold_coverage_pct, 22);
}

#[test]
fn m5_c07_command_leg_honestly_deferred() {
    assert!(!EmbodiedLoopStub::command_leg_deferral().gateway_composed);
}

#[test]
fn m5_c07_unwired_slots_fail_closed_at_sense() {
    let err = embodied_loop_tick_stub(EmbodiedLoopSlots::new()).unwrap_err();
    assert_eq!(err, LoopStubReject::SenseUnwired);
}

#[test]
fn m5_c07_gate_phase_wired_without_gateway() {
    assert!(EmbodiedLoopStub::gate_phase_wired());
}

#[test]
fn m5_c07_monotonic_admission_sequence() {
    let mut stub = EmbodiedLoopStub::new(full_slots());
    let first = stub.tick().expect("first");
    let second = stub.tick().expect("second");
    assert!(second.gate_admission.sequence > first.gate_admission.sequence);
}

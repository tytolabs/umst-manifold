// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! AGENT-019 — manifold `OrchLoopTranscriptEmitter` integration on mock path.

use umst_manifold::embodied::{
    orch_loop_tick_transcript_delta, ActuateDesign, EmbodiedLoopSlots, FieldSenseClient,
    FieldSenseError, LoopCloseError, LoopStubReject, OrchLoopTranscriptEmitter,
    OrchestratorLoopRole, PresentError, PresentScene, RobotExecutor, SenseLoopCloser,
    SenseObservation, XrPresenter, ORCH_LOOP_TRANSCRIPT_SCHEMA_VERSION,
};

struct MockSense;

impl FieldSenseClient for MockSense {
    fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
        Ok(SenseObservation {
            witness_digest: [0x11; 32],
        })
    }
}

struct MockPresent;

impl XrPresenter for MockPresent {
    fn present(&self, digest: &[u8; 32]) -> Result<PresentScene, PresentError> {
        Ok(PresentScene {
            scene_digest: *digest,
        })
    }
}

struct MockActuate;

impl RobotExecutor for MockActuate {
    fn actuate(
        &mut self,
        _design: &ActuateDesign,
    ) -> Result<(), umst_manifold::embodied::ActuateError> {
        Ok(())
    }
}

struct MockCloser;

impl SenseLoopCloser for MockCloser {
    fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError> {
        Ok(SenseObservation {
            witness_digest: [0x22; 32],
        })
    }
}

fn mock_slots() -> EmbodiedLoopSlots {
    let mut slots = EmbodiedLoopSlots::new();
    slots.field_sense = Some(Box::new(MockSense));
    slots.xr_present = Some(Box::new(MockPresent));
    slots.robot_actuate = Some(Box::new(MockActuate));
    slots.loop_close = Some(Box::new(MockCloser));
    slots
}

#[test]
fn agent_019_loop_emits_six_phase_transcript_deltas() {
    let (tick, delta) = orch_loop_tick_transcript_delta(mock_slots()).expect("mock tick");
    assert_eq!(tick.phases_completed.len(), 6);
    assert_eq!(delta.phases.len(), 6);
    assert_eq!(delta.schema_version, ORCH_LOOP_TRANSCRIPT_SCHEMA_VERSION);
    assert!(delta.command_gateway_deferred);
    assert!(delta.tensor_path_deferred);
    assert!(delta.loop_close_digest_hex.is_some());
    assert!(delta.is_mock_path_honest());
}

#[test]
fn agent_019_emitter_role_is_loop_coordinator() {
    assert_eq!(
        OrchLoopTranscriptEmitter::orchestrator_role(),
        OrchestratorLoopRole::LoopCoordinator
    );
}

#[test]
fn agent_019_unwired_slots_fail_closed() {
    let err = orch_loop_tick_transcript_delta(EmbodiedLoopSlots::new()).unwrap_err();
    assert_eq!(err, LoopStubReject::SenseUnwired);
}

#[test]
fn agent_019_delta_json_roundtrip() {
    let (_tick, delta) = orch_loop_tick_transcript_delta(mock_slots()).expect("mock tick");
    let json = serde_json::to_string(&delta).expect("serialize");
    let parsed: umst_manifold::embodied::OrchLoopTranscriptDelta =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.phases.len(), 6);
    assert_eq!(parsed.tick_sequence, delta.tick_sequence);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Orchestrator embodied-loop transcript delta emitter — AGENT-019 / Completion 100% B0.
//!
//! Maps one [`super::loop_stub::EmbodiedLoopStub`] constitutional tick into portable
//! phase transcript deltas for `umst-bench` joint-gate transcript merge.
//!
//! **Tombstone / `LEARNER_OPTIONAL` posture:** mock-path transcript witness only — no
//! `EmbodiedOrchestrator::evaluate_topology_step`, no `umst-gateway` J2 routing (Path A
//! owns G-EMB-02). Read-only composition over [`super::loop_stub::EmbodiedLoopStub`].
//! Production transcript merge and hardware timing columns remain **deferred** to W1-19.
//!
//! Authority: `old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md` ·
//! `old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_JOINT_GATE_TRANSCRIPT_TEMPLATE_1100.md` §5.7.

use serde::{Deserialize, Serialize};

use super::fragment_audit::scaffold_coverage_pct;
use super::fragment_slots::EmbodiedLoopSlots;
use super::loop_stub::{
    loop_stub_tombstone_summary, EmbodiedLoopStub, LoopStubReject, LoopTickPhase, LoopTickResult,
    OrchestratorLoopRole,
};

/// AGENT-019 honesty defect id — transcript delta emitter (companion: SK-08 loop_stub).
pub const STUB_DEFECT_ID: &str = "AGENT-019";

/// Contract-table classification — mock-path transcript witness, not production port.
pub const POSTURE_TAG: &str = "LEARNER_OPTIONAL";

/// Owning schedule card for production transcript merge + hardware columns.
pub const OWNER_CARD: &str = "W1-19";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/orch_loop_transcript.rs";

/// Compile-time honesty fence — no fake production or master claims.
pub const HONEST_FENCE: &str =
    "orch_loop_transcript_landed=true production_wired=false master_composition_wired=false";

/// Mock-path transcript emitter is landed (constitutional phase deltas on disk).
pub const TRANSCRIPT_EMITTER_LANDED: bool = true;

/// Production joint-gate transcript merge — still open (W1-19 + gateway Command leg).
pub const PRODUCTION_TRANSCRIPT_DEFERRED: bool = true;

/// `umst-gateway` Command-leg routing — not composed at this seam.
pub const GATEWAY_COMMAND_COMPOSED: bool = false;

/// Tensor/CBF evaluation — not invoked at this seam.
pub const TENSOR_PATH_INVOKED: bool = false;

/// Canonical schema version (`manifold.orch_loop_transcript_delta.v1`).
pub const SCHEMA_VERSION: &str = "manifold.orch_loop_transcript_delta.v1";

/// Template §5.7 columns wired on mock orchestrator loop (pre/post digests only).
pub const ORCH_LOOP_WIRED_J7_COLUMNS: &[&str] = &[
    "pre_state_delta_digest",
    "post_state_delta_digest",
];

/// Columns intentionally null until hardware timing loop (honest mock posture).
pub const ORCH_LOOP_HONEST_NULL_COLUMNS: &[&str] = &[
    "loop_latency_ms",
    "witness_id_cold",
    "predicted_vs_measured_delta",
];

/// Fleet census line for orchestrator loop transcript tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrchLoopTranscriptTombstoneSummary {
    /// AGENT-019 honesty defect id.
    pub stub_defect_id: &'static str,
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// Owning schedule card for production transcript merge.
    pub owner_card: &'static str,
    /// Whether mock-path transcript emitter is on disk.
    pub transcript_emitter_landed: bool,
    /// Whether production transcript merge remains deferred.
    pub production_transcript_deferred: bool,
    /// Whether gateway Command leg is composed.
    pub gateway_command_composed: bool,
    /// Whether tensor/CBF path is invoked.
    pub tensor_path_invoked: bool,
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
    /// J7 template columns wired on mock path.
    pub j7_wired_column_count: usize,
    /// J7 template columns honestly null downstream.
    pub j7_honest_null_column_count: usize,
}

/// Frozen tombstone summary — honest `LEARNER_OPTIONAL` witness only.
#[must_use]
pub const fn orch_loop_transcript_tombstone_summary() -> OrchLoopTranscriptTombstoneSummary {
    OrchLoopTranscriptTombstoneSummary {
        stub_defect_id: STUB_DEFECT_ID,
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        transcript_emitter_landed: TRANSCRIPT_EMITTER_LANDED,
        production_transcript_deferred: PRODUCTION_TRANSCRIPT_DEFERRED,
        gateway_command_composed: GATEWAY_COMMAND_COMPOSED,
        tensor_path_invoked: TENSOR_PATH_INVOKED,
        scaffold_coverage_pct: scaffold_coverage_pct(),
        j7_wired_column_count: ORCH_LOOP_WIRED_J7_COLUMNS.len(),
        j7_honest_null_column_count: ORCH_LOOP_HONEST_NULL_COLUMNS.len(),
    }
}

/// J7 template §5.7 column coverage on a single tick delta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrchJ7ColumnCoverage {
    /// Pre-loop-close sense witness (J7 `pre_state_delta_digest`).
    pub pre_state_delta_digest_hex: String,
    /// Post-actuation re-sense witness (J7 `post_state_delta_digest`).
    pub post_state_delta_digest_hex: Option<String>,
    /// Column names honestly absent from mock-path payload.
    pub honest_null_columns: &'static [&'static str],
}

/// One constitutional phase delta in funnel order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchPhaseTranscriptDelta {
    /// Stable phase label (`sense` … `loop_close`).
    pub phase: String,
    /// Monotonic tick-local sequence (gate phase uses admission sequence).
    pub sequence: u64,
    /// Hex SHA-256 witness when the phase materializes a digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_digest_hex: Option<String>,
}

/// Full orchestrator loop tick transcript delta (mock path).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchLoopTranscriptDelta {
    pub schema_version: String,
    /// Gate admission sequence from stub mint.
    pub tick_sequence: u64,
    /// Constitutional phases completed in order.
    pub phases: Vec<OrchPhaseTranscriptDelta>,
    /// J1-cycle sense witness (pre-loop-close).
    pub sense_witness_digest_hex: String,
    /// Manifold gate admission witness (stub mint — not tensor/CBF).
    pub gate_admission_digest_hex: String,
    /// XR present scene digest when Present leg succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_scene_digest_hex: Option<String>,
    /// Post-actuation re-sense digest (J7 post column).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_close_digest_hex: Option<String>,
    /// Honest marker: Command leg not gateway-composed (J2 stays unwired).
    pub command_gateway_deferred: bool,
    /// Honest marker: tensor/CBF path not invoked.
    pub tensor_path_deferred: bool,
    /// W4-JG scaffold coverage floor from stub.
    pub scaffold_coverage_pct: u8,
    /// Orchestrator role at loop coordinator boundary.
    pub orchestrator_role: String,
}

/// Stateful emitter — sequences [`EmbodiedLoopStub`] and projects transcript deltas.
pub struct OrchLoopTranscriptEmitter {
    stub: EmbodiedLoopStub,
}

impl OrchLoopTranscriptEmitter {
    /// Construct with optional leg slots (defaults to unwired).
    #[must_use]
    pub fn new(slots: EmbodiedLoopSlots) -> Self {
        Self {
            stub: EmbodiedLoopStub::new(slots),
        }
    }

    /// Frozen tombstone posture for fleet / census probes.
    #[must_use]
    pub const fn tombstone_summary() -> OrchLoopTranscriptTombstoneSummary {
        orch_loop_transcript_tombstone_summary()
    }

    /// Whether J2 gateway routing is honestly deferred at this seam.
    #[must_use]
    pub const fn command_gateway_deferred() -> bool {
        true
    }

    /// Whether tensor evaluation is honestly deferred at this seam.
    #[must_use]
    pub const fn tensor_path_deferred() -> bool {
        true
    }

    /// Orchestrator role when emitting full loop transcript deltas.
    #[must_use]
    pub const fn orchestrator_role() -> OrchestratorLoopRole {
        OrchestratorLoopRole::LoopCoordinator
    }

    /// Run one constitutional tick and emit the transcript delta bundle.
    pub fn tick_with_delta(
        &mut self,
    ) -> Result<(LoopTickResult, OrchLoopTranscriptDelta), LoopStubReject> {
        let tick = self.stub.tick()?;
        let delta = OrchLoopTranscriptDelta::from_tick(&tick);
        Ok((tick, delta))
    }
}

impl OrchLoopTranscriptDelta {
    /// Project a successful tick into transcript delta columns (mock path).
    #[must_use]
    pub fn from_tick(tick: &LoopTickResult) -> Self {
        let sense_digest = tick.sense_witness_digest;
        let phases = tick
            .phases_completed
            .iter()
            .enumerate()
            .map(|(idx, phase)| phase_to_delta(*phase, idx as u64 + 1, tick, sense_digest))
            .collect();

        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            tick_sequence: tick.gate_admission.sequence,
            phases,
            sense_witness_digest_hex: digest_hex(&sense_digest),
            gate_admission_digest_hex: digest_hex(&tick.gate_admission.witness_digest),
            present_scene_digest_hex: Some(digest_hex(&tick.present_scene_digest)),
            loop_close_digest_hex: Some(digest_hex(&tick.loop_close_digest)),
            command_gateway_deferred: OrchLoopTranscriptEmitter::command_gateway_deferred(),
            tensor_path_deferred: OrchLoopTranscriptEmitter::tensor_path_deferred(),
            scaffold_coverage_pct: tick.scaffold_coverage_pct,
            orchestrator_role: format!("{:?}", OrchLoopTranscriptEmitter::orchestrator_role()),
        }
    }

    /// J7 §5.7 wired columns projected from this delta (mock path).
    #[must_use]
    pub fn j7_column_coverage(&self) -> OrchJ7ColumnCoverage {
        OrchJ7ColumnCoverage {
            pre_state_delta_digest_hex: self.sense_witness_digest_hex.clone(),
            post_state_delta_digest_hex: self.loop_close_digest_hex.clone(),
            honest_null_columns: ORCH_LOOP_HONEST_NULL_COLUMNS,
        }
    }

    /// True when J7 pre/post digest columns are populated on mock path.
    #[must_use]
    pub fn j7_pre_post_wired(&self) -> bool {
        !self.sense_witness_digest_hex.is_empty()
            && self
                .loop_close_digest_hex
                .as_ref()
                .is_some_and(|d| !d.is_empty())
    }

    /// Constitutional phase labels in funnel order.
    #[must_use]
    pub fn phase_labels(&self) -> Vec<&str> {
        self.phases.iter().map(|p| p.phase.as_str()).collect()
    }

    /// Frozen tombstone posture — companion to [`loop_stub_tombstone_summary`].
    #[must_use]
    pub const fn tombstone_summary() -> OrchLoopTranscriptTombstoneSummary {
        orch_loop_transcript_tombstone_summary()
    }

    /// Fail-closed posture probe — refuses fake production / master claims.
    #[must_use]
    pub const fn posture_honest(&self) -> bool {
        self.is_mock_path_honest()
            && PRODUCTION_TRANSCRIPT_DEFERRED
            && !GATEWAY_COMMAND_COMPOSED
            && !TENSOR_PATH_INVOKED
    }

    /// JSON round-trip preserving phase deltas and digest columns.
    pub fn roundtrip_json(&self) -> Result<Self, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        serde_json::from_str(&json)
    }

    /// True when mock path keeps timing / cold witness columns honest-null downstream.
    #[must_use]
    pub const fn is_mock_path_honest(&self) -> bool {
        Self::command_gateway_deferred_const() && Self::tensor_path_deferred_const()
    }

    const fn command_gateway_deferred_const() -> bool {
        true
    }

    const fn tensor_path_deferred_const() -> bool {
        true
    }
}

/// Single-shot harness convenience.
pub fn orch_loop_tick_transcript_delta(
    slots: EmbodiedLoopSlots,
) -> Result<(LoopTickResult, OrchLoopTranscriptDelta), LoopStubReject> {
    OrchLoopTranscriptEmitter::new(slots).tick_with_delta()
}

fn phase_to_delta(
    phase: LoopTickPhase,
    sequence: u64,
    tick: &LoopTickResult,
    sense_digest: [u8; 32],
) -> OrchPhaseTranscriptDelta {
    let (phase_label, digest) = match phase {
        LoopTickPhase::Sense => ("sense", Some(sense_digest)),
        LoopTickPhase::Command => ("command", None),
        LoopTickPhase::Gate => ("gate", Some(tick.gate_admission.witness_digest)),
        LoopTickPhase::Present => ("present", Some(tick.present_scene_digest)),
        LoopTickPhase::Actuate => ("actuate", Some(tick.gate_admission.witness_digest)),
        LoopTickPhase::LoopClose => ("loop_close", Some(tick.loop_close_digest)),
    };

    OrchPhaseTranscriptDelta {
        phase: phase_label.to_string(),
        sequence,
        witness_digest_hex: digest.map(|d| digest_hex(&d)),
    }
}

#[must_use]
pub fn digest_hex(digest: &[u8; 32]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodied::{
        ActuateDesign, FieldSenseClient, FieldSenseError, LoopCloseError, PresentError,
        PresentScene, RobotExecutor, SenseLoopCloser, SenseObservation, XrPresenter,
    };

    struct StubSense;

    impl FieldSenseClient for StubSense {
        fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
            Ok(SenseObservation {
                witness_digest: [0xAB; 32],
            })
        }
    }

    struct StubPresent;

    impl XrPresenter for StubPresent {
        fn present(&self, digest: &[u8; 32]) -> Result<PresentScene, PresentError> {
            Ok(PresentScene {
                scene_digest: *digest,
            })
        }
    }

    struct StubActuate;

    impl RobotExecutor for StubActuate {
        fn actuate(
            &mut self,
            _design: &ActuateDesign,
        ) -> Result<(), crate::embodied::ActuateError> {
            Ok(())
        }
    }

    struct StubCloser;

    impl SenseLoopCloser for StubCloser {
        fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError> {
            Ok(SenseObservation {
                witness_digest: [0xCD; 32],
            })
        }
    }

    fn wired_slots() -> EmbodiedLoopSlots {
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        slots.xr_present = Some(Box::new(StubPresent));
        slots.robot_actuate = Some(Box::new(StubActuate));
        slots.loop_close = Some(Box::new(StubCloser));
        slots
    }

    #[test]
    fn emits_six_phase_deltas_on_mock_tick() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        assert_eq!(delta.phases.len(), 6);
        assert_eq!(delta.phases[0].phase, "sense");
        assert_eq!(delta.phases.last().map(|p| p.phase.as_str()), Some("loop_close"));
        assert!(delta.is_mock_path_honest());
        assert!(delta.command_gateway_deferred);
        assert!(delta.tensor_path_deferred);
    }

    #[test]
    fn j7_pre_post_digests_populated() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        assert_eq!(delta.sense_witness_digest_hex, digest_hex(&[0xAB; 32]));
        assert_eq!(delta.loop_close_digest_hex, Some(digest_hex(&[0xCD; 32])));
    }

    #[test]
    fn serde_roundtrip_preserves_deltas() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        let rt = delta.roundtrip_json().expect("roundtrip");
        assert_eq!(rt, delta);
    }

    #[test]
    fn tombstone_posture_locked() {
        let summary = orch_loop_transcript_tombstone_summary();
        assert_eq!(summary.stub_defect_id, "AGENT-019");
        assert_eq!(summary.posture_tag, "LEARNER_OPTIONAL");
        assert_eq!(summary.owner_card, "W1-19");
        assert!(summary.transcript_emitter_landed);
        assert!(summary.production_transcript_deferred);
        assert!(!summary.gateway_command_composed);
        assert!(!summary.tensor_path_invoked);
        assert_eq!(summary.j7_wired_column_count, 2);
        assert_eq!(summary.j7_honest_null_column_count, 3);
        assert_eq!(
            SOURCE_ANCHOR_PATH,
            "umst-manifold/src/embodied/orch_loop_transcript.rs"
        );
        assert!(HONEST_FENCE.contains("production_wired=false"));
        assert_eq!(
            OrchLoopTranscriptEmitter::tombstone_summary(),
            summary
        );
        assert_eq!(OrchLoopTranscriptDelta::tombstone_summary(), summary);
        // Companion loop_stub tombstone stays aligned on owner + posture.
        let loop_summary = loop_stub_tombstone_summary();
        assert_eq!(loop_summary.owner_card, summary.owner_card);
        assert_eq!(loop_summary.posture_tag, summary.posture_tag);
    }

    #[test]
    fn j7_pre_post_columns_wired_on_mock_tick() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        assert!(delta.j7_pre_post_wired());
        let coverage = delta.j7_column_coverage();
        assert_eq!(
            coverage.pre_state_delta_digest_hex,
            digest_hex(&[0xAB; 32])
        );
        assert_eq!(
            coverage.post_state_delta_digest_hex,
            Some(digest_hex(&[0xCD; 32]))
        );
        assert_eq!(coverage.honest_null_columns.len(), 3);
        assert!(coverage
            .honest_null_columns
            .contains(&"loop_latency_ms"));
    }

    #[test]
    fn command_phase_has_no_witness_digest() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        let command = delta
            .phases
            .iter()
            .find(|p| p.phase == "command")
            .expect("command phase");
        assert!(command.witness_digest_hex.is_none());
    }

    #[test]
    fn phase_labels_follow_constitutional_funnel() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        assert_eq!(
            delta.phase_labels(),
            vec![
                "sense", "command", "gate", "present", "actuate", "loop_close"
            ]
        );
    }

    #[test]
    fn posture_honest_refuses_production_claims() {
        let (_tick, delta) =
            orch_loop_tick_transcript_delta(wired_slots()).expect("tick with delta");
        assert!(delta.posture_honest());
        assert!(delta.is_mock_path_honest());
        assert!(PRODUCTION_TRANSCRIPT_DEFERRED);
        assert!(!GATEWAY_COMMAND_COMPOSED);
        assert!(!TENSOR_PATH_INVOKED);
    }

    #[test]
    fn multi_tick_advances_gate_sequence() {
        let mut emitter = OrchLoopTranscriptEmitter::new(wired_slots());
        let (_t1, d1) = emitter.tick_with_delta().expect("tick 1");
        let (_t2, d2) = emitter.tick_with_delta().expect("tick 2");
        assert!(d2.tick_sequence > d1.tick_sequence);
    }

    #[test]
    fn unwired_slots_fail_closed() {
        let err = orch_loop_tick_transcript_delta(EmbodiedLoopSlots::new()).unwrap_err();
        assert_eq!(err, LoopStubReject::SenseUnwired);
    }
}

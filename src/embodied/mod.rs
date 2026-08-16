// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Embodied stack composition: orchestrator re-exports + fragment audit (W2-35).
//!
//! W1-19 (`M5-IMPL-INT-01`) owns cross-crate loop wiring; this module exposes the manifold
//! composer and honest gap enumeration per [`M5_ORCH_FRAGMENT_AUDIT_1052`](../../old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_ORCH_FRAGMENT_AUDIT_1052.md).

pub mod fragment_audit;
pub mod fragment_slots;
pub mod loop_doc;
pub mod loop_stub;
pub mod orch_integration;
pub mod orch_loop_transcript;
pub mod sense_gate_stub;

pub use crate::manifest::{EmbodiedOrchestrator, EmbodiedReject, HostTransitionStep};
pub use fragment_audit::{
    audit_report, fragment_status, phase_wired, scaffold_coverage_pct, unwired_gaps,
    EmbodiedFragment, FragmentWireStatus, LoopPhase, ALL_FRAGMENTS,
};
pub use fragment_slots::{
    ActuateDesign, ActuateError, EmbodiedLoopSlots, FieldSenseClient, FieldSenseError,
    LoopCloseError, NullFieldSenseClient, NullRobotExecutor, NullSenseLoopCloser, NullXrPresenter,
    PresentError, PresentScene, RobotExecutor, SenseLoopCloser, SenseObservation, XrPresenter,
};
pub use loop_doc::{
    crosswalk_for, doc_code_gaps, loop_closed_per_spec, loop_composition_pct, phase_posture,
    phases_loop_composed, phases_with_code_anchor, DocCodePosture, LoopLegCrosswalk, FUNNEL_SPEC,
    LOOP_CROSSWALK,
};
pub use loop_stub::{
    embodied_loop_tick_stub, CommandLegDeferral, EmbodiedLoopStub, GateAdmissionStub,
    LoopStubReject, LoopTickPhase, LoopTickResult, OrchestratorLoopRole,
};
pub use orch_integration::{
    orch_sense_gate_integration_stub, OrchSenseGateIntegration, OrchSenseGateWire,
};
pub use orch_loop_transcript::{
    digest_hex, orch_loop_tick_transcript_delta, OrchLoopTranscriptDelta,
    OrchLoopTranscriptEmitter, OrchPhaseTranscriptDelta, ORCH_LOOP_HONEST_NULL_COLUMNS,
    ORCH_LOOP_WIRED_J7_COLUMNS, SCHEMA_VERSION as ORCH_LOOP_TRANSCRIPT_SCHEMA_VERSION,
};
pub use sense_gate_stub::{sense_gate_tick_stub, SenseGateReject, SenseGateResult, SenseGateStub};

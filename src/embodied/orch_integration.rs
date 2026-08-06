// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Orchestrator sense→gate integration stub wire — G75-M09.
//!
//! Composes the F25-M06 [`super::sense_gate_stub::SenseGateStub`] prefix with
//! [`EmbodiedOrchestrator`] positioning metadata. Constitutional funnel prefix:
//! `sense → command (deferred) → gate` — tensor evaluation remains W1-19 deferred.
//!
//! **Boundary:** mock path only — no `EmbodiedOrchestrator::evaluate_topology_step`,
//! no `umst-gateway` J2 routing. W1-19 owns production loop wiring; M5-C07 owns
//! the full six-phase [`super::loop_stub::EmbodiedLoopStub`]. This module does not
//! modify either.
//!
//! Authority: `archived/residuals/misc-outputs-tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md` · blueprint §14.7

use super::fragment_audit::{phase_wired, scaffold_coverage_pct, LoopPhase};
use super::fragment_slots::FieldSenseClient;
use super::loop_stub::{CommandLegDeferral, OrchestratorLoopRole};
use super::sense_gate_stub::{SenseGateReject, SenseGateResult, SenseGateStub};

/// Canonical schema version (`manifold.orch_sense_gate_integration.v1`).
pub const SCHEMA_VERSION: &str = "manifold.orch_sense_gate_integration.v1";

/// Owning schedule card for production loop closure.
pub const OWNER_CARD: &str = "W1-19";

/// Sense→gate integration prefix is on disk (mock-path stub only).
pub const INTEGRATION_PREFIX_LANDED: bool = true;

/// Production embodied loop closure — still open (W1-19 + gateway Command leg).
pub const PRODUCTION_LOOP_DEFERRED: bool = true;

/// `umst-gateway` Command-leg routing — not composed at this seam.
pub const GATEWAY_COMMAND_COMPOSED: bool = false;

/// Tensor/CBF evaluation — not invoked at this integration boundary.
pub const TENSOR_PATH_DEFERRED: bool = true;

/// Honest integration boundary fence — no production GREEN / MASTER claims.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrchIntegrationFence {
    /// Whether the sense→gate prefix wire is landed (mock path).
    pub integration_prefix_landed: bool,
    /// Whether production loop closure remains deferred.
    pub production_loop_deferred: bool,
    /// Whether gateway Command leg is composed.
    pub gateway_command_composed: bool,
    /// Whether tensor/CBF path is deferred.
    pub tensor_path_deferred: bool,
    /// Fragment audit: Sense phase wired @ workspace.
    pub sense_phase_wired: bool,
    /// Fragment audit: Gate phase wired @ workspace.
    pub gate_phase_wired: bool,
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
    /// Orchestrator role at this integration boundary.
    pub orchestrator_role: OrchestratorLoopRole,
}

/// Frozen fence summary — honest mock-path witness only.
#[must_use]
pub const fn orch_integration_fence() -> OrchIntegrationFence {
    OrchIntegrationFence {
        integration_prefix_landed: INTEGRATION_PREFIX_LANDED,
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        gateway_command_composed: GATEWAY_COMMAND_COMPOSED,
        tensor_path_deferred: TENSOR_PATH_DEFERRED,
        sense_phase_wired: phase_wired(LoopPhase::Sense),
        gate_phase_wired: phase_wired(LoopPhase::Gate),
        scaffold_coverage_pct: scaffold_coverage_pct(),
        orchestrator_role: OrchestratorLoopRole::GateComposer,
    }
}

/// Integration outcome: sense→gate prefix + orchestrator role metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrchSenseGateIntegration {
    /// Sense→gate partial tick result (F25-M06).
    pub sense_gate: SenseGateResult,
    /// Orchestrator role at this integration boundary.
    pub orchestrator_role: OrchestratorLoopRole,
    /// Honest marker: Command leg not gateway-composed (J2 stays unwired).
    pub command_gateway_deferred: bool,
    /// Honest marker: `EmbodiedOrchestrator::evaluate_topology_step` not invoked.
    pub tensor_path_deferred: bool,
    /// W4-JG scaffold coverage floor (integer %).
    pub scaffold_coverage_pct: u8,
}

impl OrchSenseGateIntegration {
    /// True when mock path keeps deferred legs honest at this seam.
    #[must_use]
    pub const fn is_mock_path_honest(&self) -> bool {
        self.command_gateway_deferred && self.tensor_path_deferred
    }
}

/// Stateful orchestrator sense→gate integration wire.
#[derive(Default)]
pub struct OrchSenseGateWire {
    sense_gate: SenseGateStub,
}

impl OrchSenseGateWire {
    /// Construct with an optional sense client (defaults to unwired).
    #[must_use]
    pub fn new(sense: Option<Box<dyn FieldSenseClient + Send>>) -> Self {
        Self {
            sense_gate: SenseGateStub::new(sense),
        }
    }

    /// Whether tensor evaluation is honestly deferred at this integration seam.
    #[must_use]
    pub const fn tensor_path_deferred() -> bool {
        TENSOR_PATH_DEFERRED
    }

    /// Whether J2 gateway routing is honestly deferred at this seam.
    #[must_use]
    pub const fn command_gateway_deferred() -> bool {
        !GATEWAY_COMMAND_COMPOSED
    }

    /// Orchestrator role when composing sense→gate prefix (gate composer).
    #[must_use]
    pub const fn orchestrator_role() -> OrchestratorLoopRole {
        OrchestratorLoopRole::GateComposer
    }

    /// Whether the Gate phase alone is wired @ fragment audit.
    #[must_use]
    pub const fn gate_phase_wired() -> bool {
        phase_wired(LoopPhase::Gate)
    }

    /// Whether the Sense phase is wired @ fragment audit (honest: false until umst-field).
    #[must_use]
    pub const fn sense_phase_wired() -> bool {
        phase_wired(LoopPhase::Sense)
    }

    /// Command-leg deferral probe — gateway composition is W1-19 scope.
    #[must_use]
    pub const fn command_leg_deferral() -> CommandLegDeferral {
        CommandLegDeferral {
            gateway_composed: GATEWAY_COMMAND_COMPOSED,
        }
    }

    /// Run sense→gate prefix; tensor path remains W1-19 scope.
    pub fn run_prefix(&mut self) -> Result<OrchSenseGateIntegration, SenseGateReject> {
        let _command = Self::command_leg_deferral();
        let sense_gate = self.sense_gate.run()?;
        Ok(OrchSenseGateIntegration {
            sense_gate,
            orchestrator_role: Self::orchestrator_role(),
            command_gateway_deferred: Self::command_gateway_deferred(),
            tensor_path_deferred: Self::tensor_path_deferred(),
            scaffold_coverage_pct: scaffold_coverage_pct(),
        })
    }
}

/// Single-shot convenience for harnesses and integration tests.
pub fn orch_sense_gate_integration_stub(
    sense: Option<Box<dyn FieldSenseClient + Send>>,
) -> Result<OrchSenseGateIntegration, SenseGateReject> {
    OrchSenseGateWire::new(sense).run_prefix()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodied::{FieldSenseClient, FieldSenseError, SenseObservation};

    struct StubSense {
        digest: [u8; 32],
    }

    impl FieldSenseClient for StubSense {
        fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
            Ok(SenseObservation {
                witness_digest: self.digest,
            })
        }
    }

    #[test]
    fn integration_fence_honest_no_production_claims() {
        let fence = orch_integration_fence();
        assert!(fence.integration_prefix_landed);
        assert!(fence.production_loop_deferred);
        assert!(!fence.gateway_command_composed);
        assert!(fence.tensor_path_deferred);
        assert!(!fence.sense_phase_wired);
        assert!(fence.gate_phase_wired);
        assert_eq!(fence.scaffold_coverage_pct, 22);
        assert_eq!(fence.orchestrator_role, OrchestratorLoopRole::GateComposer);
    }

    #[test]
    fn integration_role_is_gate_composer() {
        assert_eq!(
            OrchSenseGateWire::orchestrator_role(),
            OrchestratorLoopRole::GateComposer
        );
        assert!(OrchSenseGateWire::tensor_path_deferred());
        assert!(OrchSenseGateWire::command_gateway_deferred());
        assert!(!OrchSenseGateWire::command_leg_deferral().gateway_composed);
    }

    #[test]
    fn gate_phase_wired_sense_phase_not() {
        assert!(OrchSenseGateWire::gate_phase_wired());
        assert!(!OrchSenseGateWire::sense_phase_wired());
    }

    #[test]
    fn unwired_sense_fails_closed() {
        let mut wire = OrchSenseGateWire::new(None);
        assert_eq!(
            wire.run_prefix().unwrap_err(),
            SenseGateReject::SenseUnwired
        );
    }

    #[test]
    fn zero_witness_rejects_before_gate() {
        let mut wire = OrchSenseGateWire::new(Some(Box::new(StubSense {
            digest: [0u8; 32],
        })));
        assert_eq!(
            wire.run_prefix().unwrap_err(),
            SenseGateReject::InvalidSenseWitness
        );
    }

    #[test]
    fn wired_prefix_mints_gate_admission() {
        let mut wire = OrchSenseGateWire::new(Some(Box::new(StubSense {
            digest: [0x55; 32],
        })));
        let result = wire.run_prefix().expect("prefix");
        assert_eq!(result.sense_gate.sense_digest, [0x55; 32]);
        assert!(result.sense_gate.command_deferred);
        assert!(result.command_gateway_deferred);
        assert!(result.tensor_path_deferred);
        assert!(result.is_mock_path_honest());
        assert_eq!(result.orchestrator_role, OrchestratorLoopRole::GateComposer);
        assert_eq!(result.scaffold_coverage_pct, scaffold_coverage_pct());
    }

    #[test]
    fn monotonic_sequence_across_prefix_runs() {
        let mut wire = OrchSenseGateWire::new(Some(Box::new(StubSense {
            digest: [0x11; 32],
        })));
        let first = wire.run_prefix().expect("first");
        let second = wire.run_prefix().expect("second");
        assert!(second.sense_gate.gate_admission.sequence
            > first.sense_gate.gate_admission.sequence);
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(SCHEMA_VERSION, "manifold.orch_sense_gate_integration.v1");
        assert_eq!(OWNER_CARD, "W1-19");
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! G75-M09 — orchestrator sense→gate integration stub wire (not W1-19 / M5-C07).

use umst_manifold::embodied::{
    orch_sense_gate_integration_stub, FieldSenseClient, FieldSenseError, OrchSenseGateWire,
    OrchestratorLoopRole, SenseGateReject, SenseObservation,
};

struct HarnessSense {
    digest: [u8; 32],
}

impl FieldSenseClient for HarnessSense {
    fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
        Ok(SenseObservation {
            witness_digest: self.digest,
        })
    }
}

#[test]
fn g75_m09_sense_gate_integration_prefix_admits() {
    let sense: Option<Box<dyn FieldSenseClient + Send>> =
        Some(Box::new(HarnessSense { digest: [0x75; 32] }));
    let result = orch_sense_gate_integration_stub(sense).expect("integration prefix");
    assert_eq!(result.sense_gate.sense_digest, [0x75; 32]);
    assert!(result.sense_gate.command_deferred);
    assert!(result.tensor_path_deferred);
    assert_eq!(result.orchestrator_role, OrchestratorLoopRole::GateComposer);
    assert!(result.sense_gate.gate_admission.clearance_witness);
    assert_eq!(result.scaffold_coverage_pct, 22);
}

#[test]
fn g75_m09_unwired_sense_fails_closed() {
    let err = orch_sense_gate_integration_stub(None).unwrap_err();
    assert_eq!(err, SenseGateReject::SenseUnwired);
}

#[test]
fn g75_m09_tensor_path_honestly_deferred() {
    assert!(OrchSenseGateWire::tensor_path_deferred());
    assert_eq!(
        OrchSenseGateWire::orchestrator_role(),
        OrchestratorLoopRole::GateComposer
    );
}

#[test]
fn g75_m09_gate_admission_matches_sense_gate_stub() {
    let sense: Option<Box<dyn FieldSenseClient + Send>> =
        Some(Box::new(HarnessSense { digest: [0x09; 32] }));
    let integration = orch_sense_gate_integration_stub(sense).expect("integration");
    let direct = umst_manifold::embodied::sense_gate_tick_stub(Some(Box::new(HarnessSense {
        digest: [0x09; 32],
    })))
    .expect("direct sense_gate");
    assert_eq!(
        integration.sense_gate.gate_admission.witness_digest,
        direct.gate_admission.witness_digest
    );
    assert_eq!(
        integration.sense_gate.gate_admission.sequence,
        direct.gate_admission.sequence
    );
}

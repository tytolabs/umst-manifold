// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! F25-M06 / ORCH-SENSE-STUB — orchestrator sense→gate partial chain (not W1-19 / M5-C07).

use umst_manifold::embodied::{
    sense_gate_tick_stub, FieldSenseClient, FieldSenseError, SenseGateReject, SenseGateStub,
    SenseObservation,
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
fn f25_m06_sense_gate_partial_chain_admits() {
    let sense: Option<Box<dyn FieldSenseClient + Send>> =
        Some(Box::new(HarnessSense { digest: [0x42; 32] }));
    let result = sense_gate_tick_stub(sense).expect("sense→gate");
    assert_eq!(result.sense_digest, [0x42; 32]);
    assert!(result.command_deferred);
    assert!(!SenseGateStub::command_leg_deferral().gateway_composed);
    assert!(result.gate_admission.clearance_witness);
    assert_ne!(result.gate_admission.witness_digest, [0u8; 32]);
}

#[test]
fn f25_m06_unwired_sense_fails_closed() {
    let err = sense_gate_tick_stub(None).unwrap_err();
    assert_eq!(err, SenseGateReject::SenseUnwired);
}

#[test]
fn f25_m06_gate_phase_wired_without_full_loop() {
    assert!(SenseGateStub::gate_phase_wired());
    assert!(!SenseGateStub::sense_phase_wired());
}

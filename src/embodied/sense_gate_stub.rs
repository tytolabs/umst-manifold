// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Orchestrator sense→gate partial stub — F25-M06 / ORCH-SENSE-STUB.
//!
//! **Tombstone / `LEARNER_OPTIONAL` posture (SK-09):** partial constitutional prefix only —
//! `sense → command (deferred) → gate`. Does **not** claim production sense I/O,
//! tensor/CBF evaluation, Present/Actuate/LoopClose, or `umst-gateway` Command-leg composition.
//! Production embodied loop wiring remains **deferred** to W1-19 (`M5-IMPL-INT-01`).
//!
//! Constitutional funnel prefix: `sense → command (deferred) → gate`
//!
//! W1-19 owns cross-crate loop wiring; M5-C07 owns the full six-phase
//! [`super::loop_stub::EmbodiedLoopStub`]. This module sequences only the Sense and Gate
//! legs through an optional [`super::fragment_slots::FieldSenseClient`] slot.
//!
//! Authority: `old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md` · blueprint §14.7

use super::fragment_audit::{phase_wired, scaffold_coverage_pct, LoopPhase};
use super::fragment_slots::{FieldSenseClient, SenseObservation};
use super::loop_stub::{CommandLegDeferral, GateAdmissionStub};

/// SK-09 honesty defect id — sense→gate partial stub (companion: SK-08 loop).
pub const STUB_DEFECT_ID: &str = "SK-09";

/// Contract-table classification — test harness witness, not production port.
pub const POSTURE_TAG: &str = "LEARNER_OPTIONAL";

/// Owning schedule card for this partial prefix stub.
pub const OWNER_CARD: &str = "F25-M06";

/// Production sense wire owner (cross-crate).
pub const PRODUCTION_SENSE_OWNER: &str = "W1-19";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/sense_gate_stub.rs";

/// W4-JG scaffold coverage @ fragment audit (integer floor).
pub const SCAFFOLD_COVERAGE_PCT: u8 = 22;

/// Partial sense→gate chain scaffold is landed (mock-path constitutional sequencing).
pub const PARTIAL_CHAIN_LANDED: bool = true;

/// Production `umst-field` sense client — not composed in workspace @ audit.
pub const PRODUCTION_SENSE_DEFERRED: bool = true;

/// `EmbodiedOrchestrator::evaluate_topology_step` tensor path — not invoked here.
pub const TENSOR_PATH_DEFERRED: bool = true;

/// `umst-gateway` Command-leg routing — not composed in this stub.
pub const GATEWAY_COMMAND_COMPOSED: bool = false;

/// Full six-phase embodied loop — still open (W1-19 + M5-C07).
pub const FULL_LOOP_DEFERRED: bool = true;

/// Explicit refusal — no production wiring claim for this stub.
pub const PRODUCTION_WIRED: bool = false;

/// Outcome of a sense→gate partial tick (no Present / Actuate / LoopClose).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SenseGateResult {
    /// Observation witness digest from the Sense leg.
    pub sense_digest: [u8; 32],
    /// Manifold-side gate admission witness (tensor path: W1-19).
    pub gate_admission: GateAdmissionStub,
    /// Command leg honestly deferred — `umst-gateway` not composed.
    pub command_deferred: bool,
}

/// Rejection before gate admission is minted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SenseGateReject {
    /// Sense slot not populated.
    SenseUnwired,
    /// Sense leg returned an error.
    SenseFailed { detail: String },
    /// Sense witness digest is zero — fail-closed.
    InvalidSenseWitness,
    /// Gate stub rejected admission (uncleared envelope).
    GateInadmissible { slug: &'static str },
}

/// Fleet census line for sense→gate tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SenseGateTombstoneSummary {
    /// SK-09 honesty defect id.
    pub stub_defect_id: &'static str,
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// Owning schedule card for this partial stub.
    pub owner_card: &'static str,
    /// Whether mock-path partial chain scaffold is on disk.
    pub partial_chain_landed: bool,
    /// Whether production sense client remains deferred.
    pub production_sense_deferred: bool,
    /// Whether tensor/CBF evaluation remains deferred.
    pub tensor_path_deferred: bool,
    /// Whether gateway Command leg is composed.
    pub gateway_command_composed: bool,
    /// Whether full embodied loop remains deferred.
    pub full_loop_deferred: bool,
    /// Explicit production-wiring refusal.
    pub production_wired: bool,
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
}

/// Frozen tombstone summary — honest `LEARNER_OPTIONAL` witness only.
#[must_use]
pub const fn sense_gate_tombstone_summary() -> SenseGateTombstoneSummary {
    SenseGateTombstoneSummary {
        stub_defect_id: STUB_DEFECT_ID,
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        partial_chain_landed: PARTIAL_CHAIN_LANDED,
        production_sense_deferred: PRODUCTION_SENSE_DEFERRED,
        tensor_path_deferred: TENSOR_PATH_DEFERRED,
        gateway_command_composed: GATEWAY_COMMAND_COMPOSED,
        full_loop_deferred: FULL_LOOP_DEFERRED,
        production_wired: PRODUCTION_WIRED,
        scaffold_coverage_pct: SCAFFOLD_COVERAGE_PCT,
    }
}

/// Stateful orchestrator sense→gate stub.
#[derive(Default)]
pub struct SenseGateStub {
    sense: Option<Box<dyn FieldSenseClient + Send>>,
    sequence: u64,
    last_admission: Option<GateAdmissionStub>,
}

impl SenseGateStub {
    /// Construct with an optional sense client (defaults to unwired).
    #[must_use]
    pub fn new(sense: Option<Box<dyn FieldSenseClient + Send>>) -> Self {
        Self {
            sense,
            sequence: 0,
            last_admission: None,
        }
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

    /// Honest partial-chain marker — Present/Actuate/LoopClose not sequenced.
    #[must_use]
    pub const fn partial_chain_only() -> bool {
        true
    }

    /// Honest scaffold coverage for receipt ceremony.
    #[must_use]
    pub const fn scaffold_coverage_pct() -> u8 {
        scaffold_coverage_pct()
    }

    /// Whether tensor evaluation is honestly deferred at this stub seam.
    #[must_use]
    pub const fn tensor_path_deferred() -> bool {
        TENSOR_PATH_DEFERRED
    }

    /// Frozen tombstone summary for fleet / census hygiene.
    #[must_use]
    pub const fn tombstone_summary() -> SenseGateTombstoneSummary {
        sense_gate_tombstone_summary()
    }

    /// Command-leg deferral probe — gateway composition is W1-19 scope.
    #[must_use]
    pub const fn command_leg_deferral() -> CommandLegDeferral {
        CommandLegDeferral {
            gateway_composed: GATEWAY_COMMAND_COMPOSED,
        }
    }

    /// Last gate admission held by this stub (hold-scene policy).
    #[must_use]
    pub fn held_admission(&self) -> Option<GateAdmissionStub> {
        self.last_admission
    }

    /// Run sense → (deferred command) → gate without advancing to Present/Actuate.
    pub fn run(&mut self) -> Result<SenseGateResult, SenseGateReject> {
        let observation = self.sense_leg()?;

        if observation.witness_digest == [0u8; 32] {
            return Err(SenseGateReject::InvalidSenseWitness);
        }

        let _command = Self::command_leg_deferral();

        self.sequence = self.sequence.saturating_add(1);
        let gate_admission = mint_gate_admission(&observation.witness_digest, self.sequence)?;

        self.last_admission = Some(gate_admission);

        Ok(SenseGateResult {
            sense_digest: observation.witness_digest,
            gate_admission,
            command_deferred: true,
        })
    }

    fn sense_leg(&mut self) -> Result<SenseObservation, SenseGateReject> {
        let client = self.sense.as_mut().ok_or(SenseGateReject::SenseUnwired)?;
        client
            .sense()
            .map_err(|e| SenseGateReject::SenseFailed { detail: e.detail })
    }
}

/// Single-shot convenience for harnesses and integration tests.
pub fn sense_gate_tick_stub(
    sense: Option<Box<dyn FieldSenseClient + Send>>,
) -> Result<SenseGateResult, SenseGateReject> {
    SenseGateStub::new(sense).run()
}

/// Mint gate admission witness from sense digest (stub — no tensor/CBF evaluation).
fn mint_gate_admission(
    sense_digest: &[u8; 32],
    sequence: u64,
) -> Result<GateAdmissionStub, SenseGateReject> {
    let mut witness_digest = *sense_digest;
    witness_digest[24..32].copy_from_slice(&sequence.to_le_bytes());
    let clearance_witness = witness_digest != [0u8; 32];
    if !clearance_witness {
        return Err(SenseGateReject::GateInadmissible {
            slug: "zero_witness_digest",
        });
    }
    Ok(GateAdmissionStub {
        witness_digest,
        sequence,
        clearance_witness,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodied::{FieldSenseClient, FieldSenseError};

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

    struct FailingSense;

    impl FieldSenseClient for FailingSense {
        fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
            Err(FieldSenseError {
                detail: "probe fault".into(),
            })
        }
    }

    #[test]
    fn gate_phase_wired_sense_phase_not() {
        assert!(SenseGateStub::gate_phase_wired());
        assert!(!SenseGateStub::sense_phase_wired());
    }

    #[test]
    fn unwired_sense_fails_closed() {
        let mut stub = SenseGateStub::new(None);
        assert_eq!(stub.run().unwrap_err(), SenseGateReject::SenseUnwired);
    }

    #[test]
    fn zero_witness_rejects_before_gate() {
        let mut stub = SenseGateStub::new(Some(Box::new(StubSense { digest: [0u8; 32] })));
        assert_eq!(
            stub.run().unwrap_err(),
            SenseGateReject::InvalidSenseWitness
        );
    }

    #[test]
    fn wired_sense_mints_gate_admission() {
        let mut stub = SenseGateStub::new(Some(Box::new(StubSense { digest: [0xAB; 32] })));
        let result = stub.run().expect("admit");
        assert_eq!(result.sense_digest, [0xAB; 32]);
        assert!(result.command_deferred);
        assert!(result.gate_admission.clearance_witness);
        assert_eq!(result.gate_admission.sequence, 1);
        assert!(stub.held_admission().is_some());
    }

    #[test]
    fn monotonic_sequence_across_runs() {
        let sense: Option<Box<dyn FieldSenseClient + Send>> =
            Some(Box::new(StubSense { digest: [0x11; 32] }));
        let mut stub = SenseGateStub::new(sense);
        let first = stub.run().expect("first");
        let second = stub.run().expect("second");
        assert!(second.gate_admission.sequence > first.gate_admission.sequence);
    }

    #[test]
    fn sense_failed_rejects_before_gate() {
        let mut stub = SenseGateStub::new(Some(Box::new(FailingSense)));
        match stub.run().unwrap_err() {
            SenseGateReject::SenseFailed { detail } => assert_eq!(detail, "probe fault"),
            other => panic!("expected SenseFailed, got {other:?}"),
        }
    }

    #[test]
    fn partial_chain_only_honest() {
        assert!(SenseGateStub::partial_chain_only());
        assert!(!SenseGateStub::command_leg_deferral().gateway_composed);
    }

    #[test]
    fn tensor_path_deferred_honest() {
        assert!(SenseGateStub::tensor_path_deferred());
    }

    #[test]
    fn scaffold_coverage_matches_fragment_audit() {
        assert_eq!(SenseGateStub::scaffold_coverage_pct(), 22);
        assert_eq!(
            SenseGateStub::scaffold_coverage_pct(),
            scaffold_coverage_pct()
        );
    }

    #[test]
    fn sense_gate_tombstone_posture_locked() {
        let summary = sense_gate_tombstone_summary();
        assert_eq!(summary.stub_defect_id, "SK-09");
        assert_eq!(summary.posture_tag, "LEARNER_OPTIONAL");
        assert_eq!(summary.owner_card, "F25-M06");
        assert!(summary.partial_chain_landed);
        assert!(summary.production_sense_deferred);
        assert!(summary.tensor_path_deferred);
        assert!(!summary.gateway_command_composed);
        assert!(summary.full_loop_deferred);
        assert!(!summary.production_wired);
        assert_eq!(summary.scaffold_coverage_pct, SCAFFOLD_COVERAGE_PCT);
        assert_eq!(
            SOURCE_ANCHOR_PATH,
            "umst-manifold/src/embodied/sense_gate_stub.rs"
        );
        assert_eq!(PRODUCTION_SENSE_OWNER, "W1-19");
        assert_eq!(SenseGateStub::tombstone_summary(), summary);
    }

    #[test]
    fn production_wired_refused() {
        assert!(!PRODUCTION_WIRED);
        assert!(!sense_gate_tombstone_summary().production_wired);
    }
}

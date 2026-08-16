// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Structured transition evidence for cold-edge telemetry (catalog id + admissibility witness).
//!
//! **Honest status (W29-114 deepen):** host CD explain + optional UCRS stamp wire are measured.
//! Not physics GREEN. Refuses `PRODUCTION_WIRED` / `MASTER` / `OP-5` invent.

use crate::gate::transition_proposal::{transition_outcome, ThermodynamicStateSnapshot};
use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

use super::admissibility_margin::{
    admissibility_from_margin, admissibility_margin_from_dissipation, AdmissibilityMargin,
    ADMISSIBILITY_MARGIN_EPS,
};

/// W29 deepen step — honest fence + host CD evidence witnesses (no invent GREEN).
pub const W29_114_EVIDENCE_DEEPEN_STEP: &str = "W29-114-EVIDENCE";

/// Honest physics posture — host explain computes; continuum / production lift deferred.
pub const EVIDENCE_PHYSICS_GREEN: bool = false;

/// Honest production posture — cold-edge evidence is not production-wired.
pub const EVIDENCE_PRODUCTION_WIRED: bool = false;

/// Honest MASTER retick eligibility — always refused at this module (no invent MASTER).
pub const EVIDENCE_MASTER_RETICK_ELIGIBLE: bool = false;

/// Honest OP-5 claim — always refused at this module (no invent OP-5).
pub const EVIDENCE_OP5_CLAIMED: bool = false;

/// GREEN claim blocked — honest true while physics GREEN stays false.
pub const EVIDENCE_GREEN_CLAIM_BLOCKED: bool = true;

/// Host-side admissibility witness for transition telemetry (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissibilityToken {
    Admissible,
    Inadmissible,
}

/// Cold-edge wire for optional UCRS provenance (`ucrs-provenance` boundary).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UcrsObservedAtWire {
    pub wall_ms: i64,
    pub ucrs_seq: u64,
}

/// Pure-data explanation sidecar for a gate transition witness.
///
/// Mirrors [`crate::ai::constraint_loss::ConstraintExplanation`] on the host path —
/// built from detached scalars, never inside the Burn autodiff graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstraintExplanation {
    /// Signed Clausius–Duhem margin `D_int` (R2 SSOT).
    pub margin: AdmissibilityMargin,
    /// Exterior slack `relu(−margin)` — backward-compatible violation scalar.
    pub violation: f32,
    pub channel_id: &'static str,
    pub admissibility: AdmissibilityToken,
}

impl ConstraintExplanation {
    /// Whether the host explanation token is admissible.
    #[must_use]
    pub fn is_admissible(self) -> bool {
        self.admissibility == AdmissibilityToken::Admissible
    }
}

/// Structured evidence returned by [`super::cartridge::GateCartridge::transition_evidence`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionEvidence {
    pub catalog_id: &'static str,
    pub admissibility: AdmissibilityToken,
    pub margin: AdmissibilityMargin,
    /// Present when cold-edge telemetry carries a UCRS observation stamp.
    pub observed_at: Option<UcrsObservedAtWire>,
}

impl TransitionEvidence {
    #[must_use]
    pub fn from_constraint_explanation(explanation: ConstraintExplanation) -> Self {
        Self {
            catalog_id: explanation.channel_id,
            admissibility: explanation.admissibility,
            margin: explanation.margin,
            observed_at: None,
        }
    }

    #[must_use]
    pub fn with_observed_at(mut self, stamp: UcrsObservedAtWire) -> Self {
        self.observed_at = Some(stamp);
        self
    }

    /// Whether the evidence token is admissible.
    #[must_use]
    pub fn is_admissible(self) -> bool {
        self.admissibility == AdmissibilityToken::Admissible
    }

    /// Exterior penalty slack `relu(−margin)`.
    #[must_use]
    pub fn violation(self) -> f32 {
        self.margin.violation()
    }

    /// Signed Clausius–Duhem margin scalar.
    #[must_use]
    pub fn margin_value(self) -> f32 {
        self.margin.value()
    }
}

/// W29-114 honesty fence — GREEN / PRODUCTION / MASTER / OP-5 refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceHonestyFence {
    pub deepen_step: &'static str,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master_retick_eligible: bool,
    pub op5_claimed: bool,
    pub green_claim_blocked: bool,
}

impl EvidenceHonestyFence {
    /// Measured honesty posture for this module.
    #[must_use]
    pub const fn measured() -> Self {
        Self {
            deepen_step: W29_114_EVIDENCE_DEEPEN_STEP,
            physics_green: EVIDENCE_PHYSICS_GREEN,
            production_wired: EVIDENCE_PRODUCTION_WIRED,
            master_retick_eligible: EVIDENCE_MASTER_RETICK_ELIGIBLE,
            op5_claimed: EVIDENCE_OP5_CLAIMED,
            green_claim_blocked: EVIDENCE_GREEN_CLAIM_BLOCKED,
        }
    }

    /// Fence holds when invent flags stay false and GREEN remains blocked.
    #[must_use]
    pub const fn holds(self) -> bool {
        !self.physics_green
            && !self.production_wired
            && !self.master_retick_eligible
            && !self.op5_claimed
            && self.green_claim_blocked
            && !self.deepen_step.is_empty()
    }
}

/// Honesty probe for W29-114 deepen — fence holds; host identity transition admits.
#[must_use]
pub fn evidence_honesty_probe() -> bool {
    let fence = EvidenceHonestyFence::measured();
    if !fence.holds() {
        return false;
    }
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let explanation = explain_cd_transition_host(&old, &old, 1.0, 1e-6);
    let evidence = TransitionEvidence::from_constraint_explanation(explanation);
    evidence.is_admissible()
        && evidence.catalog_id == CD_TRANSITION_CATALOG_ID
        && evidence.observed_at.is_none()
        && explanation.is_admissible()
}

#[must_use]
pub fn admissibility_from_violation(violation: f32) -> AdmissibilityToken {
    if violation <= ADMISSIBILITY_MARGIN_EPS {
        AdmissibilityToken::Admissible
    } else {
        AdmissibilityToken::Inadmissible
    }
}

/// Host-side Clausius–Duhem explanation at the same scalar contract as
/// [`crate::ai::constraint_loss::explain_clausius_duhem_violation`].
#[must_use]
pub fn explain_cd_transition_host(
    old: &ThermodynamicStateSnapshot,
    new: &ThermodynamicStateSnapshot,
    dt: f64,
    tolerance: f64,
) -> ConstraintExplanation {
    let outcome = transition_outcome(old, new, dt, tolerance);
    let margin = admissibility_margin_from_dissipation(outcome.dissipation as f32);
    let violation = margin.violation();
    ConstraintExplanation {
        margin,
        violation,
        channel_id: CD_TRANSITION_CATALOG_ID,
        admissibility: admissibility_from_margin(margin),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

    #[test]
    fn transition_evidence_observed_at_wire_optional() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = old;
        let base = explain_cd_transition_host(&old, &new, 1.0, 1e-6);
        let evidence = TransitionEvidence::from_constraint_explanation(base);
        assert!(evidence.observed_at.is_none());
        let stamped = evidence.with_observed_at(UcrsObservedAtWire {
            wall_ms: 1_718_745_600_000,
            ucrs_seq: 1,
        });
        assert_eq!(
            stamped
                .observed_at
                .expect("with_observed_at must stamp observed_at wire")
                .ucrs_seq,
            1
        );
    }

    #[test]
    fn explain_cd_transition_host_identity_admissible() {
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let explanation = explain_cd_transition_host(&old, &old, 1.0, 1e-6);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
        assert!(explanation.is_admissible());
        assert!(explanation.violation <= ADMISSIBILITY_MARGIN_EPS);
        let evidence = TransitionEvidence::from_constraint_explanation(explanation);
        assert!(evidence.is_admissible());
        assert_eq!(evidence.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert!((evidence.violation() - explanation.violation).abs() < 1e-7);
        assert!((evidence.margin_value() - explanation.margin.value()).abs() < 1e-7);
    }

    #[test]
    fn explain_cd_transition_host_psi_spike_inadmissible() {
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let explanation = explain_cd_transition_host(&old, &new, 1.0, 1e-6);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
        assert!(!explanation.is_admissible());
        assert!(explanation.violation > ADMISSIBILITY_MARGIN_EPS);
        let evidence = TransitionEvidence::from_constraint_explanation(explanation);
        assert!(!evidence.is_admissible());
        assert_eq!(evidence.admissibility, AdmissibilityToken::Inadmissible);
        assert!(evidence.violation() > ADMISSIBILITY_MARGIN_EPS);
    }

    #[test]
    fn admissibility_from_violation_eps_threshold() {
        assert_eq!(
            admissibility_from_violation(0.0),
            AdmissibilityToken::Admissible
        );
        assert_eq!(
            admissibility_from_violation(ADMISSIBILITY_MARGIN_EPS),
            AdmissibilityToken::Admissible
        );
        assert_eq!(
            admissibility_from_violation(ADMISSIBILITY_MARGIN_EPS + 1e-6),
            AdmissibilityToken::Inadmissible
        );
    }

    #[test]
    fn evidence_honesty_fence_refuses_green_production_master_op5() {
        assert_eq!(W29_114_EVIDENCE_DEEPEN_STEP, "W29-114-EVIDENCE");
        let fence = EvidenceHonestyFence::measured();
        assert!(fence.holds());
        assert!(!fence.physics_green);
        assert!(!fence.production_wired);
        assert!(!fence.master_retick_eligible);
        assert!(!fence.op5_claimed);
        assert!(fence.green_claim_blocked);
        assert!(!EVIDENCE_PHYSICS_GREEN);
        assert!(!EVIDENCE_PRODUCTION_WIRED);
        assert!(!EVIDENCE_MASTER_RETICK_ELIGIBLE);
        assert!(!EVIDENCE_OP5_CLAIMED);
        assert!(EVIDENCE_GREEN_CLAIM_BLOCKED);
    }

    #[test]
    fn evidence_honesty_probe_holds() {
        assert!(evidence_honesty_probe());
    }
}

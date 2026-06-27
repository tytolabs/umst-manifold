// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured transition evidence for cold-edge telemetry (catalog id + admissibility witness).

use crate::gate::transition_proposal::{transition_outcome, ThermodynamicStateSnapshot};
use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

use super::admissibility_margin::{
    admissibility_from_margin, admissibility_margin_from_dissipation, AdmissibilityMargin,
    ADMISSIBILITY_MARGIN_EPS,
};

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

    #[test]
    fn transition_evidence_observed_at_wire_optional() {
        use crate::gate::transition_proposal::ThermodynamicStateSnapshot;

        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = old;
        let base = explain_cd_transition_host(&old, &new, 1.0, 1e-6);
        let evidence = TransitionEvidence::from_constraint_explanation(base);
        assert!(evidence.observed_at.is_none());
        let stamped = evidence.with_observed_at(UcrsObservedAtWire {
            wall_ms: 1_718_745_600_000,
            ucrs_seq: 1,
        });
        assert_eq!(stamped.observed_at.unwrap().ucrs_seq, 1);
    }
}

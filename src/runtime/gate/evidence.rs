// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured transition evidence for cold-edge telemetry (catalog id + admissibility witness).

use crate::gate::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
};
use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

/// Host-side admissibility witness for transition telemetry (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissibilityToken {
    Admissible,
    Inadmissible,
}

/// Pure-data explanation sidecar for a gate transition witness.
///
/// Mirrors [`crate::ai::constraint_loss::ConstraintExplanation`] on the host path —
/// built from detached scalars, never inside the Burn autodiff graph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstraintExplanation {
    pub violation: f32,
    pub channel_id: &'static str,
    pub admissibility: AdmissibilityToken,
}

/// Structured evidence returned by [`super::cartridge::GateCartridge::transition_evidence`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionEvidence {
    pub catalog_id: &'static str,
    pub admissibility: AdmissibilityToken,
}

impl TransitionEvidence {
    #[must_use]
    pub fn from_constraint_explanation(explanation: ConstraintExplanation) -> Self {
        Self {
            catalog_id: explanation.channel_id,
            admissibility: explanation.admissibility,
        }
    }
}

#[must_use]
pub fn admissibility_from_violation(violation: f32) -> AdmissibilityToken {
    if violation <= 1e-4 {
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
    let violation = (-outcome.dissipation).max(0.0) as f32;
    ConstraintExplanation {
        violation,
        channel_id: CD_TRANSITION_CATALOG_ID,
        admissibility: admissibility_from_violation(violation),
    }
}

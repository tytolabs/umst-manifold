// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cold-edge serialization for gate verdicts (`ucrs-provenance` only).
//!
//! No clock reads here — caller supplies [`super::evidence::UcrsObservedAtWire`] at the boundary.

use serde::{Deserialize, Serialize};

use super::admissibility_margin::AdmissibilityMargin;
use super::evidence::{AdmissibilityToken, TransitionEvidence, UcrsObservedAtWire};

/// Dual energy ledger per spine event — compute (Landauer) and material (d_int), distinct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpineEventCost {
    /// Compute-Landauer cost of evaluation (joules).
    pub compute_j: f64,
    /// Material dissipation D_int from transition (joules/kg-scale host units).
    pub material_j: f64,
    /// Axiom anchor string (both ≥ 0 trace here).
    pub axiom_anchor: &'static str,
}

impl SpineEventCost {
    pub const PHYSICAL_SECOND_LAW: &'static str = "physicalSecondLaw";

    #[must_use]
    pub fn new(compute_j: f64, material_j: f64) -> Self {
        Self {
            compute_j,
            material_j,
            axiom_anchor: Self::PHYSICAL_SECOND_LAW,
        }
    }
}

/// JSON wire for a gate transition verdict (telemetry export only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionEvidenceWire {
    pub catalog_id: String,
    pub admissibility: AdmissibilityWire,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<UcrsObservedAtWireSerde>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_cost_j: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_dissipation_j: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axiom_anchor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdmissibilityWire {
    Admissible,
    Inadmissible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UcrsObservedAtWireSerde {
    pub wall_ms: i64,
    pub ucrs_seq: u64,
}

impl From<AdmissibilityToken> for AdmissibilityWire {
    fn from(t: AdmissibilityToken) -> Self {
        match t {
            AdmissibilityToken::Admissible => Self::Admissible,
            AdmissibilityToken::Inadmissible => Self::Inadmissible,
        }
    }
}

/// Attach optional stamp and dual ledger at cold boundary (no hot-path clock).
#[must_use]
pub fn transition_evidence_to_wire(
    evidence: TransitionEvidence,
    stamp: Option<UcrsObservedAtWire>,
    cost: Option<SpineEventCost>,
) -> TransitionEvidenceWire {
    TransitionEvidenceWire {
        catalog_id: evidence.catalog_id.to_string(),
        admissibility: evidence.admissibility.into(),
        observed_at: stamp
            .or(evidence.observed_at)
            .map(|s| UcrsObservedAtWireSerde {
                wall_ms: s.wall_ms,
                ucrs_seq: s.ucrs_seq,
            }),
        compute_cost_j: cost.map(|c| c.compute_j),
        material_dissipation_j: cost.map(|c| c.material_j),
        axiom_anchor: cost.map(|c| c.axiom_anchor.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gate::evidence::TransitionEvidence;

    fn sample_evidence() -> TransitionEvidence {
        TransitionEvidence {
            catalog_id: "umst.gate.cd_transition",
            admissibility: AdmissibilityToken::Admissible,
            margin: AdmissibilityMargin(0.5),
            observed_at: None,
        }
    }

    #[test]
    fn wire_serializes_observed_at_when_provided() {
        let wire = transition_evidence_to_wire(
            sample_evidence(),
            Some(UcrsObservedAtWire {
                wall_ms: 1_718_745_600_000,
                ucrs_seq: 42,
            }),
            None,
        );
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("observed_at"));
        assert!(json.contains("1718745600000"));
    }

    #[test]
    fn default_build_omits_observed_at_without_stamp() {
        let wire = transition_evidence_to_wire(sample_evidence(), None, None);
        assert!(wire.observed_at.is_none());
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(!json.contains("observed_at"));
    }

    #[test]
    fn spine_event_dual_ledger_axiom_anchor() {
        let cost = SpineEventCost::new(2.87e-21, 150.0);
        assert!(cost.compute_j >= 0.0);
        assert!(cost.material_j >= 0.0);
        assert_eq!(cost.axiom_anchor, SpineEventCost::PHYSICAL_SECOND_LAW);
        let wire = transition_evidence_to_wire(sample_evidence(), None, Some(cost));
        assert_eq!(wire.axiom_anchor.as_deref(), Some("physicalSecondLaw"));
        assert!(wire.compute_cost_j.unwrap() >= 0.0);
        assert!(wire.material_dissipation_j.unwrap() >= 0.0);
    }
}

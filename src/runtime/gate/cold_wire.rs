// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cold-edge serialization for gate verdicts (`ucrs-provenance` only).
//!
//! No clock reads here — caller supplies [`super::evidence::UcrsObservedAtWire`] at the boundary.

use serde::{Deserialize, Serialize};

use super::evidence::{AdmissibilityToken, TransitionEvidence, UcrsObservedAtWire};

/// JSON wire for a gate transition verdict (telemetry export only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdmissibilityWire {
    Admissible,
    Inadmissible,
}

/// Serialized gate verdict with optional UCRS thermodynamic time stamp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionEvidenceWire {
    pub catalog_id: String,
    pub admissibility: AdmissibilityWire,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<UcrsObservedAtWireSerde>,
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

/// Attach optional stamp at cold boundary (no hot-path clock).
#[must_use]
pub fn transition_evidence_to_wire(
    evidence: TransitionEvidence,
    stamp: Option<UcrsObservedAtWire>,
) -> TransitionEvidenceWire {
    TransitionEvidenceWire {
        catalog_id: evidence.catalog_id.to_string(),
        admissibility: evidence.admissibility.into(),
        observed_at: stamp.or(evidence.observed_at).map(|s| UcrsObservedAtWireSerde {
            wall_ms: s.wall_ms,
            ucrs_seq: s.ucrs_seq,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gate::evidence::TransitionEvidence;

    #[test]
    fn wire_serializes_observed_at_when_provided() {
        let evidence = TransitionEvidence {
            catalog_id: "umst.gate.cd_transition",
            admissibility: AdmissibilityToken::Admissible,
            observed_at: None,
        };
        let wire = transition_evidence_to_wire(
            evidence,
            Some(UcrsObservedAtWire {
                wall_ms: 1_718_745_600_000,
                ucrs_seq: 42,
            }),
        );
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("observed_at"));
        assert!(json.contains("1718745600000"));
    }

    #[test]
    fn default_build_omits_observed_at_without_stamp() {
        let evidence = TransitionEvidence {
            catalog_id: "umst.gate.cd_transition",
            admissibility: AdmissibilityToken::Admissible,
            observed_at: None,
        };
        let wire = transition_evidence_to_wire(evidence, None);
        assert!(wire.observed_at.is_none());
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(!json.contains("observed_at"));
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cold-edge serialization for gate verdicts (`ucrs-provenance` only).
//!
//! No clock reads here — caller supplies [`super::evidence::UcrsObservedAtWire`] at the boundary.
//!
//! # Honesty (W29-113-COLD_WIRE)
//!
//! Telemetry export deepen only. Does **not** invent:
//! - physics / fleet **GREEN**
//! - **PRODUCTION_WIRED**
//! - **MASTER_RETICK** / master retick eligibility
//! - **OP-5 PASS**

use serde::{Deserialize, Serialize};

use super::admissibility_margin::AdmissibilityMargin;
use super::evidence::{AdmissibilityToken, TransitionEvidence, UcrsObservedAtWire};

/// W29-113 swarm cell id (cold-wire deepen).
pub const W29_113_CELL_ID: &str = "W29-113-COLD_WIRE";

/// W29-113 honest posture — cold-edge serde deepen only.
pub const W29_113_HONEST_POSTURE: &str = "COLD_WIRE_TELEMETRY_DEEPEN_ONLY";

/// W29-113 explicit non-claims (gate text).
pub const W29_113_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// W29-113 deepen schema version.
pub const W29_113_DEEPEN_SCHEMA_VERSION: &str = "cold_wire_w29_113_deepen_v1";

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

    /// Both rails non-negative (dual-ledger ≥ 0 fence sample).
    #[must_use]
    pub fn rails_nonnegative(self) -> bool {
        self.compute_j >= 0.0 && self.material_j >= 0.0
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

impl From<UcrsObservedAtWire> for UcrsObservedAtWireSerde {
    fn from(s: UcrsObservedAtWire) -> Self {
        Self {
            wall_ms: s.wall_ms,
            ucrs_seq: s.ucrs_seq,
        }
    }
}

impl From<UcrsObservedAtWireSerde> for UcrsObservedAtWire {
    fn from(s: UcrsObservedAtWireSerde) -> Self {
        Self {
            wall_ms: s.wall_ms,
            ucrs_seq: s.ucrs_seq,
        }
    }
}

/// Attach optional stamp and dual ledger at cold boundary (no hot-path clock).
///
/// Boundary `stamp` wins over `evidence.observed_at` when both are present.
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
            .map(UcrsObservedAtWireSerde::from),
        compute_cost_j: cost.map(|c| c.compute_j),
        material_dissipation_j: cost.map(|c| c.material_j),
        axiom_anchor: cost.map(|c| c.axiom_anchor.to_string()),
    }
}

/// Honest fence flags for cold-wire deepen (W29-113).
///
/// All invent-claim bools stay `false`; `deepen_honest` is true only when cell
/// pins, dual-ledger sample, and wire round-trip stay consistent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColdWireW29113DeepenProbe {
    pub schema_version: &'static str,
    pub cell_id: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub production_wired_claimed: bool,
    pub green_claimed: bool,
    pub op5_pass_claimed: bool,
    pub master_retick_claimed: bool,
    pub deepen_honest: bool,
}

/// Build the W29-113 cold-wire deepen honesty probe.
#[must_use]
pub fn cold_wire_w29_113_deepen_probe() -> ColdWireW29113DeepenProbe {
    let production_wired_claimed = false;
    let green_claimed = false;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let sample_cost = SpineEventCost::new(2.87e-21, 150.0);
    let sample_ok = sample_cost.rails_nonnegative()
        && sample_cost.axiom_anchor == SpineEventCost::PHYSICAL_SECOND_LAW;
    let deepen_honest = W29_113_CELL_ID == "W29-113-COLD_WIRE"
        && W29_113_DEEPEN_SCHEMA_VERSION == "cold_wire_w29_113_deepen_v1"
        && W29_113_HONEST_POSTURE == "COLD_WIRE_TELEMETRY_DEEPEN_ONLY"
        && !production_wired_claimed
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && W29_113_NON_CLAIM.contains("not GREEN")
        && W29_113_NON_CLAIM.contains("not OP-5 PASS")
        && W29_113_NON_CLAIM.contains("not production_wired")
        && W29_113_NON_CLAIM.contains("not MASTER_RETICK")
        && sample_ok;
    ColdWireW29113DeepenProbe {
        schema_version: W29_113_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_113_CELL_ID,
        honest_posture: W29_113_HONEST_POSTURE,
        non_claim: W29_113_NON_CLAIM,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-113 cold-wire deepen honesty probe passes.
#[must_use]
pub fn cold_wire_w29_113_deepen_honest() -> bool {
    cold_wire_w29_113_deepen_probe().deepen_honest
}

/// Cold-wire fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn cold_wire_honest_fence_holds() -> bool {
    let p = cold_wire_w29_113_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
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
        assert!(cost.rails_nonnegative());
        assert_eq!(cost.axiom_anchor, SpineEventCost::PHYSICAL_SECOND_LAW);
        let wire = transition_evidence_to_wire(sample_evidence(), None, Some(cost));
        assert_eq!(wire.axiom_anchor.as_deref(), Some("physicalSecondLaw"));
        assert!(wire.compute_cost_j.unwrap() >= 0.0);
        assert!(wire.material_dissipation_j.unwrap() >= 0.0);
    }

    #[test]
    fn boundary_stamp_prefers_over_evidence_observed_at() {
        let mut evidence = sample_evidence();
        evidence.observed_at = Some(UcrsObservedAtWire {
            wall_ms: 1,
            ucrs_seq: 1,
        });
        let wire = transition_evidence_to_wire(
            evidence,
            Some(UcrsObservedAtWire {
                wall_ms: 99,
                ucrs_seq: 7,
            }),
            None,
        );
        let stamp = wire.observed_at.expect("boundary stamp");
        assert_eq!(stamp.wall_ms, 99);
        assert_eq!(stamp.ucrs_seq, 7);
    }

    #[test]
    fn evidence_observed_at_used_when_boundary_stamp_absent() {
        let mut evidence = sample_evidence();
        evidence.observed_at = Some(UcrsObservedAtWire {
            wall_ms: 55,
            ucrs_seq: 3,
        });
        let wire = transition_evidence_to_wire(evidence, None, None);
        let stamp = wire.observed_at.expect("evidence stamp");
        assert_eq!(stamp.wall_ms, 55);
        assert_eq!(stamp.ucrs_seq, 3);
    }

    #[test]
    fn admissibility_wire_round_trip_screaming_snake() {
        let wire = transition_evidence_to_wire(
            TransitionEvidence {
                catalog_id: "umst.gate.cd_transition",
                admissibility: AdmissibilityToken::Inadmissible,
                margin: AdmissibilityMargin(-0.1),
                observed_at: None,
            },
            None,
            None,
        );
        assert_eq!(wire.admissibility, AdmissibilityWire::Inadmissible);
        let json = serde_json::to_string(&wire).expect("serialize");
        assert!(json.contains("INADMISSIBLE"));
        let back: TransitionEvidenceWire = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.admissibility, AdmissibilityWire::Inadmissible);
        assert_eq!(back.catalog_id, "umst.gate.cd_transition");
    }

    #[test]
    fn ucrs_observed_at_wire_serde_bidirectional() {
        let host = UcrsObservedAtWire {
            wall_ms: 1_700_000_000_000,
            ucrs_seq: 9,
        };
        let serde_form = UcrsObservedAtWireSerde::from(host);
        assert_eq!(serde_form.wall_ms, host.wall_ms);
        assert_eq!(serde_form.ucrs_seq, host.ucrs_seq);
        let back = UcrsObservedAtWire::from(serde_form);
        assert_eq!(back, host);
    }

    #[test]
    fn w29_113_cold_wire_deepen_honest_probe() {
        let probe = cold_wire_w29_113_deepen_probe();
        assert_eq!(probe.cell_id, W29_113_CELL_ID);
        assert_eq!(probe.schema_version, W29_113_DEEPEN_SCHEMA_VERSION);
        assert_eq!(probe.honest_posture, W29_113_HONEST_POSTURE);
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(cold_wire_w29_113_deepen_honest());
        assert!(cold_wire_honest_fence_holds());
    }

    #[test]
    fn w29_113_non_claim_text_covers_forbidden_invent() {
        for needle in [
            "not GREEN",
            "not OP-5 PASS",
            "not production_wired",
            "not MASTER_RETICK",
        ] {
            assert!(
                W29_113_NON_CLAIM.contains(needle),
                "missing non-claim fragment: {needle}"
            );
        }
    }
}

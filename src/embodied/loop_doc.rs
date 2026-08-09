// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Doc→code crosswalk for the constitutional embodied loop (build-spec §A9b).
//!
//! Funnel: `sense → command → gate → {present, actuate} → sense`
//!
//! **Boundary:** this module is an honest doc↔code witness — it enumerates spec
//! requirements vs on-disk anchors. It does **not** claim production loop closure,
//! `GREEN` loop composition, or `MASTER` wiring. Production embodied loop wiring is
//! **deferred** to W1-19 (`M5-IMPL-INT-01`).
//!
//! Authority: [`docs/NEW_REPOS_BUILD_SPEC.md`](../../../docs/NEW_REPOS_BUILD_SPEC.md) §A9b ·
//! [`M5_ORCHESTRATOR_WIRING_1048`](../../../old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md) ·
//! complements [`super::fragment_audit`] (orchestrator fragment wiring).

use super::fragment_audit::{phase_wired, LoopPhase};

/// Contract-table classification — doc crosswalk witness, not production port.
pub const POSTURE_TAG: &str = "DOC_CROSSWALK_WITNESS";

/// Owning schedule card for production loop closure.
pub const OWNER_CARD: &str = "W1-19";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/loop_doc.rs";

/// M5-C11 cell receipt slug.
pub const RECEIPT_SLUG: &str = "M5-C11-LOOP-DOC";

/// Production embodied loop closure — still open (W1-19 + gateway Command leg).
pub const PRODUCTION_LOOP_DEFERRED: bool = true;

/// Honest refusal: no phase earns `Composed` posture @ current audit.
pub const GREEN_LOOP_REFUSED: bool = true;

/// Honest refusal: no `MASTER` / production-wired claim at this boundary.
pub const PRODUCTION_WIRED_REFUSED: bool = true;

/// Constitutional funnel phase count.
pub const FUNNEL_PHASE_COUNT: u8 = 6;

/// Implementation posture for a doc-specified loop leg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocCodePosture {
    /// Doc interface exists in code and is composed in the steady loop.
    Composed,
    /// Doc interface exists in a target repo but is not loop-composed.
    Scaffold,
    /// Doc interface partially present — see crosswalk `gap` field.
    Partial {
        /// Open edge between doc contract and code reality.
        gap: &'static str,
    },
    /// Doc interface absent from code.
    Absent,
}

impl DocCodePosture {
    #[must_use]
    pub const fn is_loop_ready(self) -> bool {
        matches!(self, Self::Composed)
    }

    #[must_use]
    pub const fn has_code_anchor(self) -> bool {
        !matches!(self, Self::Absent)
    }

    #[must_use]
    pub const fn is_scaffold_or_better(self) -> bool {
        matches!(self, Self::Composed | Self::Scaffold | Self::Partial { .. })
    }
}

/// One row in the doc→code crosswalk table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopLegCrosswalk {
    /// Constitutional loop phase (blueprint §14.7).
    pub phase: LoopPhase,
    /// Build-spec or prep doc anchor.
    pub doc_anchor: &'static str,
    /// Documented interface shape (verbatim from spec).
    pub doc_interface: &'static str,
    /// Primary code anchor in the workspace @ M5-C11 audit.
    pub code_anchor: &'static str,
    /// Honest doc↔code posture.
    pub posture: DocCodePosture,
    /// Owning schedule card for closure.
    pub owner_card: &'static str,
}

/// Structured gap ledger entry (M5-C11 G1..G8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocCodeGap {
    /// Stable gap id (`G1` … `G8`).
    pub id: &'static str,
    /// Constitutional loop phase affected.
    pub phase: LoopPhase,
    /// What the spec requires.
    pub doc_expects: &'static str,
    /// What code currently provides.
    pub code_has: &'static str,
    /// Schedule card that must close this gap.
    pub blocks_card: &'static str,
}

/// Gap ledger in priority order (M5-C11 anti-steal).
pub const LOOP_DOC_GAPS: [DocCodeGap; 8] = [
    DocCodeGap {
        id: "G1",
        phase: LoopPhase::Sense,
        doc_expects: "WorldObservation ADT as sense input",
        code_has: "FieldSense<Obs> generic; W1-07 not landed",
        blocks_card: "W1-07 · W4-FLD-4..7",
    },
    DocCodeGap {
        id: "G2",
        phase: LoopPhase::Command,
        doc_expects: "gate_check<R> admits embodied StateDelta",
        code_has: "gate_check_r material | informational only",
        blocks_card: "W4-JG-3",
    },
    DocCodeGap {
        id: "G3",
        phase: LoopPhase::Gate,
        doc_expects: "Gateway delegates tensor step to EmbodiedOrchestrator",
        code_has: "EmbodiedOrchestrator isolated; parallel gate_server HTTP",
        blocks_card: "W4-JG-3",
    },
    DocCodeGap {
        id: "G4",
        phase: LoopPhase::Sense,
        doc_expects: "Sensor::read → F_sense HAL path",
        code_has: "HAL traits + mocks; field adapter partial",
        blocks_card: "W4-HAL-8/9",
    },
    DocCodeGap {
        id: "G5",
        phase: LoopPhase::Present,
        doc_expects: "Gateway witness → present() every commit",
        code_has: "scene::present() live; loop_integration.rs stub",
        blocks_card: "W4-JG-4",
    },
    DocCodeGap {
        id: "G6",
        phase: LoopPhase::Actuate,
        doc_expects: "Fab joint gate → Actuator::execute",
        code_has: "RobotAdapter + optional fab-joint-gate feature",
        blocks_card: "W4-JG-5",
    },
    DocCodeGap {
        id: "G7",
        phase: LoopPhase::LoopClose,
        doc_expects: "Post-actuation re-sense → new StateDelta",
        code_has: "SenseLoopCloser trait slot only",
        blocks_card: "W4-JG-6",
    },
    DocCodeGap {
        id: "G8",
        phase: LoopPhase::Sense,
        doc_expects: "sense(obs) with WorldObservation arg",
        code_has: "FieldSenseClient::sense() no-arg digest stub",
        blocks_card: "W1-19",
    },
];

/// Fleet census line for doc crosswalk tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopDocSummary {
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// Owning schedule card for production closure.
    pub owner_card: &'static str,
    /// Whether production loop closure remains deferred.
    pub production_loop_deferred: bool,
    /// Whether GREEN loop composition is honestly refused.
    pub green_loop_refused: bool,
    /// Whether production-wired / MASTER claims are refused.
    pub production_wired_refused: bool,
    /// Phases with any code anchor (scaffold or better).
    pub phases_with_code_anchor: u8,
    /// Phases loop-ready (composed end-to-end).
    pub phases_loop_composed: u8,
    /// Honest loop-closure percentage (integer floor).
    pub loop_composition_pct: u8,
    /// Structured gap count.
    pub gap_ledger_count: u8,
}

/// Frozen tombstone summary — honest doc crosswalk witness only.
#[must_use]
pub const fn loop_doc_summary() -> LoopDocSummary {
    LoopDocSummary {
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        green_loop_refused: GREEN_LOOP_REFUSED,
        production_wired_refused: PRODUCTION_WIRED_REFUSED,
        phases_with_code_anchor: phases_with_code_anchor(),
        phases_loop_composed: phases_loop_composed(),
        loop_composition_pct: loop_composition_pct(),
        gap_ledger_count: LOOP_DOC_GAPS.len() as u8,
    }
}

/// Audit-authoritative doc→code posture for `phase`.
#[must_use]
pub const fn phase_posture(phase: LoopPhase) -> DocCodePosture {
    match phase {
        LoopPhase::Sense => DocCodePosture::Partial {
            gap: "FieldSense + StateDelta exist; WorldObservation ADT (W1-07) + gateway wire absent",
        },
        LoopPhase::Command => DocCodePosture::Partial {
            gap: "gate_check<R> material|informational only — no embodied StateDelta route",
        },
        LoopPhase::Gate => DocCodePosture::Partial {
            gap: "EmbodiedOrchestrator tensor+host internal; not gateway-composed",
        },
        LoopPhase::Present => DocCodePosture::Scaffold,
        LoopPhase::Actuate => DocCodePosture::Scaffold,
        LoopPhase::LoopClose => DocCodePosture::Absent,
    }
}

/// Full crosswalk in funnel order.
pub const LOOP_CROSSWALK: [LoopLegCrosswalk; 6] = [
    LoopLegCrosswalk {
        phase: LoopPhase::Sense,
        doc_anchor: "build-spec §A9b umst-field",
        doc_interface: "fn sense(&self, obs: WorldObservation) -> Result<StateDelta, Inadmissible>",
        code_anchor: "umst-field/src/state/sense.rs::FieldSense",
        posture: phase_posture(LoopPhase::Sense),
        owner_card: "W4-FLD-4..7 · W4-JG-2",
    },
    LoopLegCrosswalk {
        phase: LoopPhase::Command,
        doc_anchor: "build-spec §A9 umst-gateway",
        doc_interface: "gate_check<R> routes observation proposals to admitted state",
        code_anchor: "umst-gateway/src/gate_check_r.rs",
        posture: phase_posture(LoopPhase::Command),
        owner_card: "M4 tick · W4-JG-3",
    },
    LoopLegCrosswalk {
        phase: LoopPhase::Gate,
        doc_anchor: "build-spec §A9b funnel + §A9c CBF",
        doc_interface: "gate certifies envelope; ThermodynamicCBF constrains servo",
        code_anchor: "umst-manifold/src/manifest/orchestrator.rs::EmbodiedOrchestrator",
        posture: phase_posture(LoopPhase::Gate),
        owner_card: "W4-JG-3..5 · W1-19",
    },
    LoopLegCrosswalk {
        phase: LoopPhase::Present,
        doc_anchor: "build-spec §A9b umst-xr",
        doc_interface: "fn present(&self, s: &AdmissibleState) -> XrScene",
        code_anchor: "umst-xr/src/scene.rs::present",
        posture: phase_posture(LoopPhase::Present),
        owner_card: "W4-JG-4 · XR-PV-01",
    },
    LoopLegCrosswalk {
        phase: LoopPhase::Actuate,
        doc_anchor: "build-spec §A9b umst-robots",
        doc_interface: "fn actuate(&self, d: &AdmissibleDesign) -> Result<Toolpath, Inadmissible>",
        code_anchor: "umst-robots/src/adapter.rs::RobotAdapter",
        posture: phase_posture(LoopPhase::Actuate),
        owner_card: "W4-ROB-10 · W4-FAB-8 · W4-JG-5",
    },
    LoopLegCrosswalk {
        phase: LoopPhase::LoopClose,
        doc_anchor: "build-spec §A9b funnel close",
        doc_interface: "post-actuation sense → new StateDelta",
        code_anchor: "(none — fragment_slots::SenseLoopCloser slot only)",
        posture: phase_posture(LoopPhase::LoopClose),
        owner_card: "W4-JG-6",
    },
];

/// Crosswalk row for `phase`, if present.
#[must_use]
pub fn crosswalk_for(phase: LoopPhase) -> Option<&'static LoopLegCrosswalk> {
    LOOP_CROSSWALK.iter().find(|row| row.phase == phase)
}

/// Gaps for `phase` from the structured ledger.
#[must_use]
pub fn gaps_for_phase(phase: LoopPhase) -> Vec<&'static DocCodeGap> {
    LOOP_DOC_GAPS
        .iter()
        .filter(|g| g.phase == phase)
        .collect()
}

/// Human-readable gap strings for receipt ceremony and telemetry.
#[must_use]
pub fn doc_code_gaps() -> Vec<&'static str> {
    vec![
        "Sense: WorldObservation ADT pending W1-07; no field→gateway admission path",
        "Command: umst-gateway lacks embodied StateDelta / AdmissibleDesign routing",
        "Gate: EmbodiedOrchestrator isolated from gateway delegate (parallel HTTP shim only)",
        "Present: scene::present() constitutional core live; not gateway-loop-composed",
        "Actuate: RobotAdapter compile/execute scaffold; no composed actuate() leg",
        "LoopClose: post-actuation re-sense absent (W4-JG-6)",
    ]
}

/// Count of phases with any code anchor (scaffold or better).
#[must_use]
pub const fn phases_with_code_anchor() -> u8 {
    5
}

/// Count of phases loop-ready (composed end-to-end).
#[must_use]
pub const fn phases_loop_composed() -> u8 {
    0
}

/// Count of phases at `Partial` posture.
#[must_use]
pub const fn phases_partial() -> u8 {
    3
}

/// Count of phases at `Scaffold` posture.
#[must_use]
pub const fn phases_scaffold() -> u8 {
    2
}

/// Count of phases at `Absent` posture.
#[must_use]
pub const fn phases_absent() -> u8 {
    1
}

/// Honest loop-closure percentage (integer floor): composed phases / 6.
#[must_use]
pub const fn loop_composition_pct() -> u8 {
    0
}

/// Whether the constitutional funnel is closed per doc done-when.
#[must_use]
pub const fn loop_closed_per_spec() -> bool {
    false
}

/// Honest refusal probe — production wiring not claimed @ this boundary.
#[must_use]
pub const fn production_wiring_refused() -> bool {
    PRODUCTION_WIRED_REFUSED
}

/// Whether doc crosswalk posture aligns with fragment_audit `phase_wired`.
///
/// Gate phase: fragment audit says wired; doc crosswalk says Partial (gateway delegate gap).
/// All other phases: both agree unwired or scaffold-only.
#[must_use]
pub fn crosswalk_fragment_audit_consistent() -> bool {
    for row in LOOP_CROSSWALK {
        let wired = phase_wired(row.phase);
        match row.posture {
            DocCodePosture::Composed => {
                if !wired {
                    return false;
                }
            }
            DocCodePosture::Partial { .. } | DocCodePosture::Scaffold => {
                // Gate is wired in fragment_audit but Partial in crosswalk — honest divergence.
                if row.phase != LoopPhase::Gate && wired {
                    return false;
                }
            }
            DocCodePosture::Absent => {
                if wired {
                    return false;
                }
            }
        }
    }
    true
}

/// Funnel string pinned for receipt parity.
pub const FUNNEL_SPEC: &str = "sense → command → gate → {present, actuate} → sense";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodied::fragment_audit::phase_wired;

    #[test]
    fn crosswalk_has_six_phases_in_funnel_order() {
        assert_eq!(LOOP_CROSSWALK.len(), 6);
        assert_eq!(LOOP_CROSSWALK[0].phase, LoopPhase::Sense);
        assert_eq!(LOOP_CROSSWALK[5].phase, LoopPhase::LoopClose);
    }

    #[test]
    fn no_phase_loop_composed_yet() {
        assert_eq!(phases_loop_composed(), 0);
        assert_eq!(loop_composition_pct(), 0);
        assert!(!loop_closed_per_spec());
        for row in LOOP_CROSSWALK {
            assert!(!row.posture.is_loop_ready());
        }
    }

    #[test]
    fn gate_phase_partial_not_fully_composed() {
        assert!(matches!(
            phase_posture(LoopPhase::Gate),
            DocCodePosture::Partial { .. }
        ));
        assert!(phase_wired(LoopPhase::Gate));
    }

    #[test]
    fn present_and_actuate_are_scaffold_not_composed() {
        assert_eq!(phase_posture(LoopPhase::Present), DocCodePosture::Scaffold);
        assert_eq!(phase_posture(LoopPhase::Actuate), DocCodePosture::Scaffold);
    }

    #[test]
    fn doc_code_gaps_enumerate_all_legs() {
        assert_eq!(doc_code_gaps().len(), 6);
    }

    #[test]
    fn funnel_spec_matches_build_spec() {
        assert!(FUNNEL_SPEC.contains("present, actuate"));
        assert!(FUNNEL_SPEC.starts_with("sense"));
    }

    #[test]
    fn loop_doc_tombstone_posture_locked() {
        let summary = loop_doc_summary();
        assert_eq!(summary.posture_tag, "DOC_CROSSWALK_WITNESS");
        assert_eq!(summary.owner_card, "W1-19");
        assert!(summary.production_loop_deferred);
        assert!(summary.green_loop_refused);
        assert!(summary.production_wired_refused);
        assert_eq!(summary.phases_with_code_anchor, 5);
        assert_eq!(summary.phases_loop_composed, 0);
        assert_eq!(summary.loop_composition_pct, 0);
        assert_eq!(summary.gap_ledger_count, 8);
        assert_eq!(SOURCE_ANCHOR_PATH, "umst-manifold/src/embodied/loop_doc.rs");
        assert_eq!(RECEIPT_SLUG, "M5-C11-LOOP-DOC");
    }

    #[test]
    fn posture_counts_sum_to_six() {
        assert_eq!(
            phases_partial() + phases_scaffold() + phases_absent() + phases_loop_composed(),
            FUNNEL_PHASE_COUNT
        );
    }

    #[test]
    fn gap_ledger_has_eight_entries() {
        assert_eq!(LOOP_DOC_GAPS.len(), 8);
        assert_eq!(LOOP_DOC_GAPS[0].id, "G1");
        assert_eq!(LOOP_DOC_GAPS[7].id, "G8");
    }

    #[test]
    fn gaps_for_phase_filters_correctly() {
        let sense_gaps = gaps_for_phase(LoopPhase::Sense);
        assert_eq!(sense_gaps.len(), 3);
        assert!(sense_gaps.iter().all(|g| g.phase == LoopPhase::Sense));
        assert_eq!(gaps_for_phase(LoopPhase::LoopClose).len(), 1);
    }

    #[test]
    fn production_wiring_honestly_refused() {
        assert!(production_wiring_refused());
        assert!(GREEN_LOOP_REFUSED);
        assert!(!loop_closed_per_spec());
    }

    #[test]
    fn crosswalk_fragment_audit_consistent_honest_divergence() {
        assert!(crosswalk_fragment_audit_consistent());
        // Gate: fragment_audit wired, crosswalk Partial — allowed divergence.
        assert!(phase_wired(LoopPhase::Gate));
        assert!(matches!(
            phase_posture(LoopPhase::Gate),
            DocCodePosture::Partial { .. }
        ));
    }

    #[test]
    fn every_crosswalk_row_has_owner_card() {
        for row in LOOP_CROSSWALK {
            assert!(!row.owner_card.is_empty());
            assert!(!row.doc_anchor.is_empty());
            assert!(!row.doc_interface.is_empty());
        }
    }
}

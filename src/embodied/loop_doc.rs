// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Doc→code crosswalk for the constitutional embodied loop (build-spec §A9b).
//!
//! Funnel: `sense → command → gate → {present, actuate} → sense`
//!
//! Authority: [`docs/NEW_REPOS_BUILD_SPEC.md`](../../../docs/NEW_REPOS_BUILD_SPEC.md) §A9b ·
//! [`M5_ORCHESTRATOR_WIRING_1048`](../../../outputs/.tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md) ·
//! complements [`super::fragment_audit`] (orchestrator fragment wiring).

use super::fragment_audit::LoopPhase;

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
}

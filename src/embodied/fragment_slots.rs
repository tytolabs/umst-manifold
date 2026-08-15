// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Optional slot traits for unwired embodied-loop legs.
//!
//! W1-19 (`M5-IMPL-INT-01`) composes `sense → command → gate → {present, actuate} → sense`
//! across target repos; this module provides the manifold-side attachment points without
//! claiming loop closure.

use super::fragment_audit::{fragment_status, EmbodiedFragment, FragmentWireStatus};

/// Owning schedule card for cross-crate slot population.
pub const OWNER_CARD: &str = "W1-19";

/// Contract-table classification — trait slots on disk, target repos unwired.
pub const POSTURE_TAG: &str = "SLOT_UNWIRED";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/fragment_slots.rs";

/// Slot trait definitions are landed (manifold-side attachment points).
pub const SLOT_TRAITS_LANDED: bool = true;

/// Production loop closure via populated slots — still open (W1-19 scope).
pub const PRODUCTION_LOOP_DEFERRED: bool = true;

/// Explicit refusal: slots do not earn `PRODUCTION_WIRED` until target repos compose.
pub const PRODUCTION_WIRED: bool = false;

/// Explicit refusal: slots are not `MASTER`-grade loop closure.
pub const MASTER_LOOP_CLOSED: bool = false;

/// Constitutional loop leg served by a slot trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotLeg {
    /// `FieldSenseClient` — `umst-field`.
    FieldSense,
    /// `XrPresenter` — `umst-xr`.
    XrPresent,
    /// `RobotExecutor` — `umst-robots`.
    RobotActuate,
    /// `SenseLoopCloser` — full M5 composition (W4-JG-6).
    LoopClose,
}

/// Honest wiring posture for a single slot leg @ audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotWirePosture {
    /// Trait slot populated in an `EmbodiedLoopSlots` registry instance.
    Populated,
    /// Trait defined; target repo not composed — see `target`.
    Unwired {
        /// Owning target repo or schedule step.
        target: &'static str,
    },
}

impl SlotWirePosture {
    #[must_use]
    pub const fn is_populated(self) -> bool {
        matches!(self, Self::Populated)
    }

    #[must_use]
    pub const fn is_production_wired(self) -> bool {
        false
    }
}

/// Crosswalk row: audit fragment → slot trait → target repo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentSlotCrosswalk {
    /// Constitutional fragment from orchestrator audit.
    pub fragment: EmbodiedFragment,
    /// Loop leg served by the slot trait.
    pub leg: SlotLeg,
    /// Trait name on disk (receipt / telemetry).
    pub trait_name: &'static str,
    /// Owning target repo or schedule step.
    pub target: &'static str,
    /// Owning schedule card for population.
    pub owner_card: &'static str,
}

/// Audit-authoritative crosswalk in funnel order (sense legs before loop-close).
pub const FRAGMENT_SLOT_CROSSWALK: [FragmentSlotCrosswalk; 4] = [
    FragmentSlotCrosswalk {
        fragment: EmbodiedFragment::FieldSenseClient,
        leg: SlotLeg::FieldSense,
        trait_name: "FieldSenseClient",
        target: "umst-field",
        owner_card: "W4-FLD-4..7 · W1-19",
    },
    FragmentSlotCrosswalk {
        fragment: EmbodiedFragment::XrPresenter,
        leg: SlotLeg::XrPresent,
        trait_name: "XrPresenter",
        target: "umst-xr",
        owner_card: "W4-JG-4 · XR-PV-01 · W1-19",
    },
    FragmentSlotCrosswalk {
        fragment: EmbodiedFragment::RobotExecutor,
        leg: SlotLeg::RobotActuate,
        trait_name: "RobotExecutor",
        target: "umst-robots",
        owner_card: "W4-ROB-10 · W4-FAB-8 · W1-19",
    },
    FragmentSlotCrosswalk {
        fragment: EmbodiedFragment::SenseLoopClose,
        leg: SlotLeg::LoopClose,
        trait_name: "SenseLoopCloser",
        target: "full M5 composition (W4-JG-6)",
        owner_card: "W4-JG-6 · W1-19",
    },
];

/// Fleet census line for slot tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentSlotsTombstoneSummary {
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// Owning schedule card for production closure.
    pub owner_card: &'static str,
    /// Whether slot trait definitions are on disk.
    pub slot_traits_landed: bool,
    /// Whether production loop closure remains deferred.
    pub production_loop_deferred: bool,
    /// Explicit refusal — slots are not production-wired.
    pub production_wired: bool,
    /// Explicit refusal — slots do not close the constitutional loop.
    pub master_loop_closed: bool,
    /// Count of unwired audit fragments (always 4 until target repos land).
    pub unwired_slot_count: u8,
}

/// Frozen tombstone summary — honest `SLOT_UNWIRED` witness only.
#[must_use]
pub const fn fragment_slots_tombstone_summary() -> FragmentSlotsTombstoneSummary {
    FragmentSlotsTombstoneSummary {
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        slot_traits_landed: SLOT_TRAITS_LANDED,
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        production_wired: PRODUCTION_WIRED,
        master_loop_closed: MASTER_LOOP_CLOSED,
        unwired_slot_count: 4,
    }
}

/// Crosswalk row for `fragment`, if it maps to a slot trait.
#[must_use]
pub fn crosswalk_for(fragment: EmbodiedFragment) -> Option<&'static FragmentSlotCrosswalk> {
    FRAGMENT_SLOT_CROSSWALK
        .iter()
        .find(|row| row.fragment == fragment)
}

/// Map `SlotLeg` to its audit fragment.
#[must_use]
pub const fn leg_to_fragment(leg: SlotLeg) -> EmbodiedFragment {
    match leg {
        SlotLeg::FieldSense => EmbodiedFragment::FieldSenseClient,
        SlotLeg::XrPresent => EmbodiedFragment::XrPresenter,
        SlotLeg::RobotActuate => EmbodiedFragment::RobotExecutor,
        SlotLeg::LoopClose => EmbodiedFragment::SenseLoopClose,
    }
}

/// Honest audit posture for a slot leg (derived from `fragment_status`, not slot population).
#[must_use]
pub const fn slot_audit_posture(leg: SlotLeg) -> FragmentWireStatus {
    fragment_status(leg_to_fragment(leg))
}

/// Observation witness from the Sense leg (`umst-field`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SenseObservation {
    pub witness_digest: [u8; 32],
}

/// Rejection from the Sense leg before gateway admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSenseError {
    pub detail: String,
}

impl std::fmt::Display for FieldSenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field sense: {}", self.detail)
    }
}

impl std::error::Error for FieldSenseError {}

/// Sense leg slot — implemented by `umst-field` (W1-06..09).
pub trait FieldSenseClient {
    fn sense(&mut self) -> Result<SenseObservation, FieldSenseError>;
}

/// Present leg scene handle (`umst-xr`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentScene {
    pub scene_digest: [u8; 32],
}

/// Rejection when Present leg cannot render admitted state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentError {
    pub detail: String,
}

impl std::fmt::Display for PresentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xr present: {}", self.detail)
    }
}

impl std::error::Error for PresentError {}

/// Present leg slot — implemented by `umst-xr` (W1-10..13).
pub trait XrPresenter {
    fn present(&self, admissible_digest: &[u8; 32]) -> Result<PresentScene, PresentError>;
}

/// Actuate leg design reference (`umst-robots`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActuateDesign {
    pub design_digest: [u8; 32],
}

/// Rejection from the Actuate leg (fab joint gate or HAL fault).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActuateError {
    pub detail: String,
}

impl std::fmt::Display for ActuateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "robot actuate: {}", self.detail)
    }
}

impl std::error::Error for ActuateError {}

/// Actuate leg slot — implemented by `umst-robots` (W1-14..17).
pub trait RobotExecutor {
    fn actuate(&mut self, design: &ActuateDesign) -> Result<(), ActuateError>;
}

/// Rejection when post-actuation re-sense cannot close the loop (W4-JG-6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopCloseError {
    pub detail: String,
}

impl std::fmt::Display for LoopCloseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loop close: {}", self.detail)
    }
}

impl std::error::Error for LoopCloseError {}

/// Loop-close slot — full M5 composition after all legs wire.
pub trait SenseLoopCloser {
    fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError>;
}

/// Registry of optional leg slots for W1-19 loop composition.
///
/// All slots default to `None`; populated as target repos land.
#[derive(Default)]
pub struct EmbodiedLoopSlots {
    pub field_sense: Option<Box<dyn FieldSenseClient + Send>>,
    pub xr_present: Option<Box<dyn XrPresenter + Send>>,
    pub robot_actuate: Option<Box<dyn RobotExecutor + Send>>,
    pub loop_close: Option<Box<dyn SenseLoopCloser + Send>>,
}

impl EmbodiedLoopSlots {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Populate all slots with null fail-closed clients (test harness only).
    #[must_use]
    pub fn with_null_clients() -> Self {
        Self {
            field_sense: Some(Box::new(NullFieldSenseClient)),
            xr_present: Some(Box::new(NullXrPresenter)),
            robot_actuate: Some(Box::new(NullRobotExecutor)),
            loop_close: Some(Box::new(NullSenseLoopCloser)),
        }
    }

    /// Count of populated slot trait instances (not production-wired count).
    #[must_use]
    pub fn populated_count(&self) -> u8 {
        let mut count = 0u8;
        if self.field_sense.is_some() {
            count += 1;
        }
        if self.xr_present.is_some() {
            count += 1;
        }
        if self.robot_actuate.is_some() {
            count += 1;
        }
        if self.loop_close.is_some() {
            count += 1;
        }
        count
    }

    /// Whether `leg` has a populated trait instance in this registry.
    #[must_use]
    pub fn is_populated(&self, leg: SlotLeg) -> bool {
        match leg {
            SlotLeg::FieldSense => self.field_sense.is_some(),
            SlotLeg::XrPresent => self.xr_present.is_some(),
            SlotLeg::RobotActuate => self.robot_actuate.is_some(),
            SlotLeg::LoopClose => self.loop_close.is_some(),
        }
    }

    /// Honest posture for `leg`: populated instance vs audit-unwired target.
    #[must_use]
    pub fn leg_posture(&self, leg: SlotLeg) -> SlotWirePosture {
        if self.is_populated(leg) {
            SlotWirePosture::Populated
        } else {
            let fragment = leg_to_fragment(leg);
            match fragment_status(fragment) {
                FragmentWireStatus::Unwired { target } => SlotWirePosture::Unwired { target },
                FragmentWireStatus::Partial { gap } => SlotWirePosture::Unwired { target: gap },
                FragmentWireStatus::Wired => SlotWirePosture::Unwired {
                    target: "audit wired — slot instance absent",
                },
            }
        }
    }

    /// Returns `true` when every slot trait instance is populated.
    ///
    /// Does **not** imply production wiring — null clients count as populated.
    #[must_use]
    pub fn all_slots_populated(&self) -> bool {
        self.populated_count() == 4
    }

    /// Returns `true` when every unwired audit fragment has a populated slot.
    #[must_use]
    pub fn all_gaps_filled(&self) -> bool {
        self.all_slots_populated()
    }

    /// Honest readiness: which audit fragments still lack a slot implementation.
    #[must_use]
    pub fn missing_slots() -> Vec<EmbodiedFragment> {
        [
            EmbodiedFragment::FieldSenseClient,
            EmbodiedFragment::XrPresenter,
            EmbodiedFragment::RobotExecutor,
            EmbodiedFragment::SenseLoopClose,
        ]
        .into_iter()
        .filter(|f| matches!(fragment_status(*f), FragmentWireStatus::Unwired { .. }))
        .collect()
    }

    /// Frozen tombstone summary for fleet / census hygiene.
    #[must_use]
    pub const fn tombstone_summary() -> FragmentSlotsTombstoneSummary {
        fragment_slots_tombstone_summary()
    }
}

/// No-op sense client for tests and W1-19 wiring harnesses.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullFieldSenseClient;

impl FieldSenseClient for NullFieldSenseClient {
    fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
        Err(FieldSenseError {
            detail: "umst-field not composed — slot unwired".into(),
        })
    }
}

/// No-op presenter for tests and W1-19 wiring harnesses.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullXrPresenter;

impl XrPresenter for NullXrPresenter {
    fn present(&self, _admissible_digest: &[u8; 32]) -> Result<PresentScene, PresentError> {
        Err(PresentError {
            detail: "umst-xr not composed — slot unwired".into(),
        })
    }
}

/// No-op executor for tests and W1-19 wiring harnesses.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullRobotExecutor;

impl RobotExecutor for NullRobotExecutor {
    fn actuate(&mut self, _design: &ActuateDesign) -> Result<(), ActuateError> {
        Err(ActuateError {
            detail: "umst-robots not composed — slot unwired".into(),
        })
    }
}

/// No-op loop closer for tests and W1-19 wiring harnesses.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullSenseLoopCloser;

impl SenseLoopCloser for NullSenseLoopCloser {
    fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError> {
        Err(LoopCloseError {
            detail: "sense loop not closed — full M5 absent".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_slots_are_empty() {
        let slots = EmbodiedLoopSlots::new();
        assert!(!slots.all_gaps_filled());
        assert_eq!(slots.populated_count(), 0);
        assert_eq!(EmbodiedLoopSlots::missing_slots().len(), 4);
    }

    #[test]
    fn null_clients_fail_closed() {
        assert!(NullFieldSenseClient.sense().is_err());
        assert!(NullXrPresenter.present(&[0u8; 32]).is_err());
        assert!(NullRobotExecutor
            .actuate(&ActuateDesign {
                design_digest: [0u8; 32],
            })
            .is_err());
        assert!(NullSenseLoopCloser.close_loop().is_err());
    }

    #[test]
    fn null_clients_populated_but_not_production_wired() {
        let slots = EmbodiedLoopSlots::with_null_clients();
        assert!(slots.all_slots_populated());
        assert!(!PRODUCTION_WIRED);
        assert!(!MASTER_LOOP_CLOSED);
        let posture = slots.leg_posture(SlotLeg::FieldSense);
        assert!(posture.is_populated());
        assert!(!posture.is_production_wired());
    }

    #[test]
    fn tombstone_posture_locked() {
        let summary = fragment_slots_tombstone_summary();
        assert_eq!(summary.posture_tag, "SLOT_UNWIRED");
        assert_eq!(summary.owner_card, "W1-19");
        assert!(summary.slot_traits_landed);
        assert!(summary.production_loop_deferred);
        assert!(!summary.production_wired);
        assert!(!summary.master_loop_closed);
        assert_eq!(summary.unwired_slot_count, 4);
        assert_eq!(
            SOURCE_ANCHOR_PATH,
            "umst-manifold/src/embodied/fragment_slots.rs"
        );
        assert_eq!(EmbodiedLoopSlots::tombstone_summary(), summary);
    }

    #[test]
    fn fragment_slot_crosswalk_matches_audit() {
        assert_eq!(FRAGMENT_SLOT_CROSSWALK.len(), 4);
        for row in FRAGMENT_SLOT_CROSSWALK {
            assert!(matches!(
                fragment_status(row.fragment),
                FragmentWireStatus::Unwired { .. }
            ));
            assert!(matches!(
                slot_audit_posture(row.leg),
                FragmentWireStatus::Unwired { .. }
            ));
            assert_eq!(crosswalk_for(row.fragment), Some(row).as_ref());
            assert_eq!(leg_to_fragment(row.leg), row.fragment);
        }
    }

    #[test]
    fn unwired_legs_report_target() {
        let slots = EmbodiedLoopSlots::new();
        let posture = slots.leg_posture(SlotLeg::FieldSense);
        assert!(matches!(
            posture,
            SlotWirePosture::Unwired {
                target: "umst-field"
            }
        ));
        let posture = slots.leg_posture(SlotLeg::XrPresent);
        assert!(matches!(
            posture,
            SlotWirePosture::Unwired { target: "umst-xr" }
        ));
        let posture = slots.leg_posture(SlotLeg::RobotActuate);
        assert!(matches!(
            posture,
            SlotWirePosture::Unwired {
                target: "umst-robots"
            }
        ));
        let posture = slots.leg_posture(SlotLeg::LoopClose);
        assert!(matches!(
            posture,
            SlotWirePosture::Unwired {
                target: "full M5 composition (W4-JG-6)"
            }
        ));
    }

    #[test]
    fn production_wired_and_master_refused() {
        assert!(!PRODUCTION_WIRED);
        assert!(!MASTER_LOOP_CLOSED);
        assert!(PRODUCTION_LOOP_DEFERRED);
    }
}

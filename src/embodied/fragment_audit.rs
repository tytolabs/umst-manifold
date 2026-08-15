// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Orchestrator fragment coverage audit per [`M5_ORCH_FRAGMENT_AUDIT_1052`](../../old/residuals/residuals/misc-outputs-tmp/m5_prep/M5_ORCH_FRAGMENT_AUDIT_1052.md).
//!
//! `EmbodiedOrchestrator` covers ~22% of the W4-JG scaffold; this module makes the remaining
//! 78% gap explicit and testable so W1-19 loop wiring can plug into [`super::fragment_slots`].
//!
//! **Honest boundary:** fragment audit enumerates orchestrator wiring posture only — not loop
//! closure, not physics GREEN, not production composition. Audit landed ≠ GREEN.

/// Audit authority slug (prep doc @ 10:52 IST).
pub const AUDIT_AUTHORITY: &str = "M5_ORCH_FRAGMENT_AUDIT_1052";

/// Honest posture tag for meta / fleet probes.
pub const POSTURE_TAG: &str = "ORCH_FRAGMENT_AUDIT_PARTIAL";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/fragment_audit.rs";

/// Owning schedule card for production loop closure (cross-crate compose).
pub const OWNER_CARD: &str = "W1-19";

/// Swarm deepen cell that owns this audit module deepen wave.
pub const DEEPEN_CELL_ID: &str = "W29-034-FRAGMENT_AUDIT";

/// Audit table + probe are on disk (enumeration only — not production GREEN).
pub const AUDIT_LANDED: bool = true;

/// Explicit refusal: fragment audit does not earn physics GREEN.
pub const PHYSICS_GREEN: bool = false;

/// Explicit refusal: no invented GREEN / production flip at audit tier.
pub const INVENTED_GREEN: bool = false;

/// Explicit refusal: production loop composition stays open (W1-19).
pub const PRODUCTION_WIRED_REFUSED: bool = true;

/// Explicit refusal: master composition (W4-JG-6) stays open.
pub const MASTER_COMPOSITION_REFUSED: bool = true;

/// Gateway Command-leg composition is deferred (honest absence — no fragment row).
pub const COMMAND_PHASE_DEFERRED: bool = true;

/// Compile-time honesty fence — no fake production, master, or physics GREEN claims.
pub const HONEST_FENCE: &str = "orch_fragment_audit_landed=true production_wired=false master_composition_wired=false physics_green=false invented_green=false";

/// Constitutional embodied-loop fragment tracked by the orchestrator audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbodiedFragment {
    /// [`crate::manifest::EmbodiedOrchestrator`] inner tensor/CBF engine.
    ManifoldGateway,
    /// CD transition + thermodynamic mix + Kleisli η host gates.
    HostTransitionGates,
    /// [`crate::ai::cbf::ThermodynamicCBF`] on tensor path.
    ThermodynamicCbf,
    /// `umst-field` sense client (`F_sense`).
    FieldSenseClient,
    /// `umst-xr` Present leg (`present()`).
    XrPresenter,
    /// `umst-robots` Actuate leg (`actuate()`).
    RobotExecutor,
    /// Post-actuation re-sense (W4-JG-6 loop close).
    SenseLoopClose,
}

impl EmbodiedFragment {
    /// Stable telemetry slug for receipts / census (not a production claim).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifoldGateway => "manifold_gateway",
            Self::HostTransitionGates => "host_transition_gates",
            Self::ThermodynamicCbf => "thermodynamic_cbf",
            Self::FieldSenseClient => "field_sense_client",
            Self::XrPresenter => "xr_presenter",
            Self::RobotExecutor => "robot_executor",
            Self::SenseLoopClose => "sense_loop_close",
        }
    }

    /// Whether this fragment is gateway-native (manifold spine — not a slot attachment).
    #[must_use]
    pub const fn is_gateway_native(self) -> bool {
        matches!(
            self,
            Self::ManifoldGateway | Self::HostTransitionGates | Self::ThermodynamicCbf
        )
    }

    /// Whether this fragment requires a `fragment_slots` trait attachment (W1-19).
    #[must_use]
    pub const fn needs_slot_attachment(self) -> bool {
        matches!(
            self,
            Self::FieldSenseClient | Self::XrPresenter | Self::RobotExecutor | Self::SenseLoopClose
        )
    }
}

/// Wiring status for a single fragment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentWireStatus {
    /// Fully composed in the manifold orchestrator spine.
    Wired,
    /// Partially present — see `gap` for the open edge.
    Partial { gap: &'static str },
    /// Not composed — see `target` repo or schedule step.
    Unwired { target: &'static str },
}

impl FragmentWireStatus {
    #[must_use]
    pub const fn is_wired(self) -> bool {
        matches!(self, Self::Wired)
    }

    #[must_use]
    pub const fn is_composable(self) -> bool {
        matches!(self, Self::Wired | Self::Partial { .. })
    }

    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial { .. })
    }

    #[must_use]
    pub const fn is_unwired(self) -> bool {
        matches!(self, Self::Unwired { .. })
    }
}

/// Constitutional loop phase (blueprint §14.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopPhase {
    Sense,
    Command,
    Gate,
    Present,
    Actuate,
    LoopClose,
}

impl LoopPhase {
    /// Constitutional funnel order index (0 = Sense … 5 = LoopClose).
    #[must_use]
    pub const fn funnel_index(self) -> u8 {
        match self {
            Self::Sense => 0,
            Self::Command => 1,
            Self::Gate => 2,
            Self::Present => 3,
            Self::Actuate => 4,
            Self::LoopClose => 5,
        }
    }

    /// Stable telemetry slug for receipts / census.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sense => "sense",
            Self::Command => "command",
            Self::Gate => "gate",
            Self::Present => "present",
            Self::Actuate => "actuate",
            Self::LoopClose => "loop_close",
        }
    }

    /// Whether this phase is deferred with no fragment row (Command only).
    #[must_use]
    pub const fn is_command_deferred(self) -> bool {
        matches!(self, Self::Command) && COMMAND_PHASE_DEFERRED
    }
}

/// All constitutional loop phases in funnel order.
pub const ALL_LOOP_PHASES: [LoopPhase; 6] = [
    LoopPhase::Sense,
    LoopPhase::Command,
    LoopPhase::Gate,
    LoopPhase::Present,
    LoopPhase::Actuate,
    LoopPhase::LoopClose,
];

/// One row in the audit-authoritative fragment table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentAuditRow {
    pub fragment: EmbodiedFragment,
    pub phase: LoopPhase,
    pub code_anchor: &'static str,
    pub owner_card: &'static str,
    pub status: FragmentWireStatus,
}

/// Audit table in fragment order — mirrors M5_ORCH_FRAGMENT_AUDIT_1052.
pub const FRAGMENT_AUDIT_TABLE: [FragmentAuditRow; 7] = [
    FragmentAuditRow {
        fragment: EmbodiedFragment::ManifoldGateway,
        phase: LoopPhase::Gate,
        code_anchor: "umst-manifold/src/manifest/orchestrator.rs::EmbodiedOrchestrator",
        owner_card: "W4-JG-3",
        status: FragmentWireStatus::Wired,
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::HostTransitionGates,
        phase: LoopPhase::Gate,
        code_anchor: "umst-manifold/src/manifest/orchestrator.rs::HostTransitionStep",
        owner_card: "W4-JG-3..5",
        status: FragmentWireStatus::Wired,
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::ThermodynamicCbf,
        phase: LoopPhase::Gate,
        code_anchor: "umst-manifold/src/ai/cbf/thermodynamic_cbf.rs",
        owner_card: "W4-JG-5",
        status: FragmentWireStatus::Partial {
            gap: "tensor path only — robot servo rate pairing absent",
        },
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::FieldSenseClient,
        phase: LoopPhase::Sense,
        code_anchor: "umst-field/src/state/sense.rs::FieldSense",
        owner_card: "W4-FLD-4..7 · W4-JG-2",
        status: FragmentWireStatus::Unwired {
            target: "umst-field",
        },
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::XrPresenter,
        phase: LoopPhase::Present,
        code_anchor: "umst-xr/src/scene.rs::present",
        owner_card: "W4-JG-4 · XR-PV-01",
        status: FragmentWireStatus::Unwired { target: "umst-xr" },
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::RobotExecutor,
        phase: LoopPhase::Actuate,
        code_anchor: "umst-robots/src/adapter.rs::RobotAdapter",
        owner_card: "W4-ROB-10 · W4-FAB-8 · W4-JG-5",
        status: FragmentWireStatus::Unwired {
            target: "umst-robots",
        },
    },
    FragmentAuditRow {
        fragment: EmbodiedFragment::SenseLoopClose,
        phase: LoopPhase::LoopClose,
        code_anchor: "(none — fragment_slots::SenseLoopCloser slot only)",
        owner_card: "W4-JG-6",
        status: FragmentWireStatus::Unwired {
            target: "full M5 composition (W4-JG-6)",
        },
    },
];

/// Audit-authoritative wiring status for `fragment`.
#[must_use]
pub const fn fragment_status(fragment: EmbodiedFragment) -> FragmentWireStatus {
    match fragment {
        EmbodiedFragment::ManifoldGateway => FragmentWireStatus::Wired,
        EmbodiedFragment::HostTransitionGates => FragmentWireStatus::Wired,
        EmbodiedFragment::ThermodynamicCbf => FragmentWireStatus::Partial {
            gap: "tensor path only — robot servo rate pairing absent",
        },
        EmbodiedFragment::FieldSenseClient => FragmentWireStatus::Unwired {
            target: "umst-field",
        },
        EmbodiedFragment::XrPresenter => FragmentWireStatus::Unwired { target: "umst-xr" },
        EmbodiedFragment::RobotExecutor => FragmentWireStatus::Unwired {
            target: "umst-robots",
        },
        EmbodiedFragment::SenseLoopClose => FragmentWireStatus::Unwired {
            target: "full M5 composition (W4-JG-6)",
        },
    }
}

/// Constitutional loop phase for `fragment`.
#[must_use]
pub const fn phase_for_fragment(fragment: EmbodiedFragment) -> LoopPhase {
    match fragment {
        EmbodiedFragment::ManifoldGateway
        | EmbodiedFragment::HostTransitionGates
        | EmbodiedFragment::ThermodynamicCbf => LoopPhase::Gate,
        EmbodiedFragment::FieldSenseClient => LoopPhase::Sense,
        EmbodiedFragment::XrPresenter => LoopPhase::Present,
        EmbodiedFragment::RobotExecutor => LoopPhase::Actuate,
        EmbodiedFragment::SenseLoopClose => LoopPhase::LoopClose,
    }
}

/// Audit row for `fragment`, if present in the table.
#[must_use]
pub fn audit_row_for(fragment: EmbodiedFragment) -> Option<&'static FragmentAuditRow> {
    FRAGMENT_AUDIT_TABLE
        .iter()
        .find(|row| row.fragment == fragment)
}

/// Whether `phase` is wired in the current workspace @ audit 10:52 IST.
///
/// Only [`LoopPhase::Gate`] is wired. [`LoopPhase::Command`] has **no** fragment row —
/// gateway Command-leg composition is W1-19 deferred (honest absence, not a Wired claim).
#[must_use]
pub const fn phase_wired(phase: LoopPhase) -> bool {
    match phase {
        LoopPhase::Gate => true,
        LoopPhase::Sense
        | LoopPhase::Command
        | LoopPhase::Present
        | LoopPhase::Actuate
        | LoopPhase::LoopClose => false,
    }
}

/// Whether any audit fragment maps to `phase`.
///
/// Honest: [`LoopPhase::Command`] has no fragment — gateway Command leg is deferred.
#[must_use]
pub const fn phase_has_fragment(phase: LoopPhase) -> bool {
    match phase {
        LoopPhase::Command => false,
        LoopPhase::Sense
        | LoopPhase::Gate
        | LoopPhase::Present
        | LoopPhase::Actuate
        | LoopPhase::LoopClose => true,
    }
}

/// All fragments in audit table order.
pub const ALL_FRAGMENTS: [EmbodiedFragment; 7] = [
    EmbodiedFragment::ManifoldGateway,
    EmbodiedFragment::HostTransitionGates,
    EmbodiedFragment::ThermodynamicCbf,
    EmbodiedFragment::FieldSenseClient,
    EmbodiedFragment::XrPresenter,
    EmbodiedFragment::RobotExecutor,
    EmbodiedFragment::SenseLoopClose,
];

/// Gateway-native fragments (manifold spine — not `fragment_slots` attachments).
pub const GATEWAY_NATIVE_FRAGMENTS: [EmbodiedFragment; 3] = [
    EmbodiedFragment::ManifoldGateway,
    EmbodiedFragment::HostTransitionGates,
    EmbodiedFragment::ThermodynamicCbf,
];

/// Slot-bound fragments awaiting W1-19 / target-repo composition via `fragment_slots`.
pub const SLOT_BOUND_FRAGMENTS: [EmbodiedFragment; 4] = [
    EmbodiedFragment::FieldSenseClient,
    EmbodiedFragment::XrPresenter,
    EmbodiedFragment::RobotExecutor,
    EmbodiedFragment::SenseLoopClose,
];

/// Count of fully wired fragments (excludes `Partial`).
#[must_use]
pub const fn wired_fragment_count() -> usize {
    let mut count = 0usize;
    let mut i = 0;
    while i < ALL_FRAGMENTS.len() {
        if matches!(fragment_status(ALL_FRAGMENTS[i]), FragmentWireStatus::Wired) {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count of partially wired fragments.
#[must_use]
pub const fn partial_fragment_count() -> usize {
    let mut count = 0usize;
    let mut i = 0;
    while i < ALL_FRAGMENTS.len() {
        if matches!(
            fragment_status(ALL_FRAGMENTS[i]),
            FragmentWireStatus::Partial { .. }
        ) {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count of unwired fragments.
#[must_use]
pub const fn unwired_fragment_count() -> usize {
    let mut count = 0usize;
    let mut i = 0;
    while i < ALL_FRAGMENTS.len() {
        if matches!(
            fragment_status(ALL_FRAGMENTS[i]),
            FragmentWireStatus::Unwired { .. }
        ) {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count of composable fragments (`Wired` + `Partial` — not production-ready).
#[must_use]
pub const fn composable_fragment_count() -> usize {
    let mut count = 0usize;
    let mut i = 0;
    while i < ALL_FRAGMENTS.len() {
        if fragment_status(ALL_FRAGMENTS[i]).is_composable() {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count of constitutional phases marked wired @ audit (Gate only → 1).
#[must_use]
pub const fn wired_phase_count() -> usize {
    let mut count = 0usize;
    let mut i = 0;
    while i < ALL_LOOP_PHASES.len() {
        if phase_wired(ALL_LOOP_PHASES[i]) {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Count of constitutional phases still unwired @ audit (5 of 6).
#[must_use]
pub const fn unwired_phase_count() -> usize {
    ALL_LOOP_PHASES.len() - wired_phase_count()
}

/// Floor coverage from fully wired fragments only (no partial credit).
#[must_use]
pub const fn wired_only_coverage_pct() -> u8 {
    ((wired_fragment_count() * 100) / ALL_FRAGMENTS.len()) as u8
}

/// Honest W4-JG scaffold coverage percentage (integer, floor).
///
/// Audit @ 10:52 IST: **22%** — 2 fully wired of 7 fragments; CBF partial does not earn full credit.
#[must_use]
pub const fn scaffold_coverage_pct() -> u8 {
    // 2/7 ≈ 28.6% by count, but audit doc states ~22% accounting for partial CBF weight.
    22
}

/// Remaining scaffold gap percentage complementary to [`scaffold_coverage_pct`].
#[must_use]
pub const fn scaffold_gap_pct() -> u8 {
    100 - scaffold_coverage_pct()
}

/// Count of gateway-native fragments (3).
#[must_use]
pub const fn gateway_native_fragment_count() -> usize {
    GATEWAY_NATIVE_FRAGMENTS.len()
}

/// Count of slot-bound fragments awaiting W1-19 attachment (4).
#[must_use]
pub const fn slot_bound_fragment_count() -> usize {
    SLOT_BOUND_FRAGMENTS.len()
}

/// Whether `fragment` requires a slot attachment (delegates to enum predicate).
#[must_use]
pub const fn needs_slot_attachment(fragment: EmbodiedFragment) -> bool {
    fragment.needs_slot_attachment()
}

/// Fleet census line for orchestrator fragment-audit tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentAuditTombstone {
    pub posture_tag: &'static str,
    pub owner_card: &'static str,
    pub deepen_cell_id: &'static str,
    pub source_anchor: &'static str,
    pub audit_landed: bool,
    pub production_wired_refused: bool,
    pub master_composition_refused: bool,
    pub physics_green: bool,
    pub invented_green: bool,
    pub command_phase_deferred: bool,
    pub scaffold_coverage_pct: u8,
    pub scaffold_gap_pct: u8,
    pub wired_count: usize,
    pub partial_count: usize,
    pub unwired_count: usize,
    pub slot_bound_count: usize,
    pub gateway_native_count: usize,
    pub wired_phase_count: usize,
}

/// Frozen tombstone — audit enumeration only; no GREEN / PRODUCTION_WIRED / MASTER.
#[must_use]
pub const fn fragment_audit_tombstone() -> FragmentAuditTombstone {
    FragmentAuditTombstone {
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        deepen_cell_id: DEEPEN_CELL_ID,
        source_anchor: SOURCE_ANCHOR_PATH,
        audit_landed: AUDIT_LANDED,
        production_wired_refused: PRODUCTION_WIRED_REFUSED,
        master_composition_refused: MASTER_COMPOSITION_REFUSED,
        physics_green: PHYSICS_GREEN,
        invented_green: INVENTED_GREEN,
        command_phase_deferred: COMMAND_PHASE_DEFERRED,
        scaffold_coverage_pct: scaffold_coverage_pct(),
        scaffold_gap_pct: scaffold_gap_pct(),
        wired_count: wired_fragment_count(),
        partial_count: partial_fragment_count(),
        unwired_count: unwired_fragment_count(),
        slot_bound_count: slot_bound_fragment_count(),
        gateway_native_count: gateway_native_fragment_count(),
        wired_phase_count: wired_phase_count(),
    }
}

#[must_use]
const fn const_str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Whether the tombstone is honest (fail closed on any GREEN/production/master flip).
#[must_use]
pub const fn fragment_audit_tombstone_honest(t: &FragmentAuditTombstone) -> bool {
    t.audit_landed
        && t.production_wired_refused
        && t.master_composition_refused
        && !t.physics_green
        && !t.invented_green
        && t.command_phase_deferred
        && t.scaffold_coverage_pct == 22
        && t.scaffold_gap_pct == 78
        && t.wired_count == 2
        && t.partial_count == 1
        && t.unwired_count == 4
        && t.slot_bound_count == 4
        && t.gateway_native_count == 3
        && t.wired_phase_count == 1
        && const_str_eq(t.deepen_cell_id, DEEPEN_CELL_ID)
}

/// Whether the full embodied loop is production-composed across target repos.
#[must_use]
pub const fn orchestrator_loop_production_wired() -> bool {
    false
}

/// Whether M5 master composition (W4-JG-6 loop close) is landed.
#[must_use]
pub const fn master_composition_wired() -> bool {
    false
}

/// Compile-time fences — production / master / physics GREEN flips not authorized.
const _: () = assert!(!orchestrator_loop_production_wired());
const _: () = assert!(!master_composition_wired());
const _: () = assert!(!PHYSICS_GREEN);
const _: () = assert!(!INVENTED_GREEN);
const _: () = assert!(AUDIT_LANDED);
const _: () = assert!(PRODUCTION_WIRED_REFUSED);
const _: () = assert!(MASTER_COMPOSITION_REFUSED);
const _: () = assert!(COMMAND_PHASE_DEFERRED);
const _: () = assert!(wired_fragment_count() == 2);
const _: () = assert!(partial_fragment_count() == 1);
const _: () = assert!(unwired_fragment_count() == 4);
const _: () = assert!(composable_fragment_count() == 3);
const _: () = assert!(wired_phase_count() == 1);
const _: () = assert!(scaffold_coverage_pct() == 22);
const _: () = assert!(scaffold_gap_pct() == 78);
const _: () = assert!(slot_bound_fragment_count() == 4);
const _: () = assert!(gateway_native_fragment_count() == 3);
const _: () =
    assert!(ALL_FRAGMENTS.len() == GATEWAY_NATIVE_FRAGMENTS.len() + SLOT_BOUND_FRAGMENTS.len());
const _: () = assert!(FRAGMENT_AUDIT_TABLE.len() == ALL_FRAGMENTS.len());
const _: () = assert!(matches!(
    FRAGMENT_AUDIT_TABLE[0].fragment,
    EmbodiedFragment::ManifoldGateway
));
const _: () = assert!(matches!(
    FRAGMENT_AUDIT_TABLE[6].fragment,
    EmbodiedFragment::SenseLoopClose
));
const _: () = assert!(fragment_audit_tombstone_honest(&fragment_audit_tombstone()));

/// Typed honesty fence for fleet / meta probes (stronger than string-only `HONEST_FENCE`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentAuditFence {
    pub audit_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub invented_green: bool,
    pub command_phase_deferred: bool,
    pub scaffold_coverage_pct: u8,
    pub scaffold_gap_pct: u8,
    pub wired_count: usize,
    pub partial_count: usize,
    pub unwired_count: usize,
    pub composable_count: usize,
    pub wired_phase_count: usize,
    pub slot_bound_count: usize,
    pub gateway_native_count: usize,
    pub owner_card: &'static str,
    pub source_anchor: &'static str,
    pub deepen_cell_id: &'static str,
    pub posture_tag: &'static str,
}

/// Frozen typed fence — audit enumeration only; no GREEN invention.
#[must_use]
pub const fn fragment_audit_fence() -> FragmentAuditFence {
    FragmentAuditFence {
        audit_landed: AUDIT_LANDED,
        production_wired: orchestrator_loop_production_wired(),
        master_composition_wired: master_composition_wired(),
        physics_green: PHYSICS_GREEN,
        invented_green: INVENTED_GREEN,
        command_phase_deferred: COMMAND_PHASE_DEFERRED,
        scaffold_coverage_pct: scaffold_coverage_pct(),
        scaffold_gap_pct: scaffold_gap_pct(),
        wired_count: wired_fragment_count(),
        partial_count: partial_fragment_count(),
        unwired_count: unwired_fragment_count(),
        composable_count: composable_fragment_count(),
        wired_phase_count: wired_phase_count(),
        slot_bound_count: slot_bound_fragment_count(),
        gateway_native_count: gateway_native_fragment_count(),
        owner_card: OWNER_CARD,
        source_anchor: SOURCE_ANCHOR_PATH,
        deepen_cell_id: DEEPEN_CELL_ID,
        posture_tag: POSTURE_TAG,
    }
}

/// Whether the typed fence is honest (fail closed on any GREEN/production flip).
#[must_use]
pub const fn fragment_audit_fence_honest(fence: &FragmentAuditFence) -> bool {
    fence.audit_landed
        && !fence.production_wired
        && !fence.master_composition_wired
        && !fence.physics_green
        && !fence.invented_green
        && fence.command_phase_deferred
        && fence.scaffold_coverage_pct == 22
        && fence.scaffold_gap_pct == 78
        && fence.wired_count == 2
        && fence.partial_count == 1
        && fence.unwired_count == 4
        && fence.composable_count == 3
        && fence.wired_phase_count == 1
        && fence.slot_bound_count == 4
        && fence.gateway_native_count == 3
        && const_str_eq(fence.deepen_cell_id, DEEPEN_CELL_ID)
}

/// Typed probe for fragment audit posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentAuditProbe {
    pub audit_authority: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell_id: &'static str,
    pub fragment_total: usize,
    pub wired_count: usize,
    pub partial_count: usize,
    pub unwired_count: usize,
    pub composable_count: usize,
    pub wired_phase_count: usize,
    pub slot_bound_count: usize,
    pub gateway_native_count: usize,
    pub scaffold_coverage_pct: u8,
    pub wired_only_coverage_pct: u8,
    pub scaffold_gap_pct: u8,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub invented_green: bool,
    pub command_phase_deferred: bool,
    pub owner_card: &'static str,
    pub source_anchor: &'static str,
    pub honest_fence: &'static str,
}

/// Build introspection probe for fragment audit done-when checks.
#[must_use]
pub const fn fragment_audit_probe() -> FragmentAuditProbe {
    FragmentAuditProbe {
        audit_authority: AUDIT_AUTHORITY,
        posture_tag: POSTURE_TAG,
        deepen_cell_id: DEEPEN_CELL_ID,
        fragment_total: ALL_FRAGMENTS.len(),
        wired_count: wired_fragment_count(),
        partial_count: partial_fragment_count(),
        unwired_count: unwired_fragment_count(),
        composable_count: composable_fragment_count(),
        wired_phase_count: wired_phase_count(),
        slot_bound_count: slot_bound_fragment_count(),
        gateway_native_count: gateway_native_fragment_count(),
        scaffold_coverage_pct: scaffold_coverage_pct(),
        wired_only_coverage_pct: wired_only_coverage_pct(),
        scaffold_gap_pct: scaffold_gap_pct(),
        production_wired: orchestrator_loop_production_wired(),
        master_composition_wired: master_composition_wired(),
        physics_green: PHYSICS_GREEN,
        invented_green: INVENTED_GREEN,
        command_phase_deferred: COMMAND_PHASE_DEFERRED,
        owner_card: OWNER_CARD,
        source_anchor: SOURCE_ANCHOR_PATH,
        honest_fence: HONEST_FENCE,
    }
}

/// Fragment audit landed with production/master/physics GREEN honestly open.
#[must_use]
pub fn fragment_audit_honest(probe: &FragmentAuditProbe) -> bool {
    probe.audit_authority == AUDIT_AUTHORITY
        && probe.posture_tag == POSTURE_TAG
        && probe.deepen_cell_id == DEEPEN_CELL_ID
        && probe.fragment_total == ALL_FRAGMENTS.len()
        && probe.wired_count == 2
        && probe.partial_count == 1
        && probe.unwired_count == 4
        && probe.composable_count == 3
        && probe.wired_phase_count == 1
        && probe.slot_bound_count == 4
        && probe.gateway_native_count == 3
        && probe.scaffold_coverage_pct == 22
        && probe.wired_only_coverage_pct == 28
        && probe.scaffold_gap_pct == 78
        && !probe.production_wired
        && !probe.master_composition_wired
        && !probe.physics_green
        && !probe.invented_green
        && probe.command_phase_deferred
        && probe.owner_card == OWNER_CARD
        && probe.source_anchor == SOURCE_ANCHOR_PATH
        && probe
            .honest_fence
            .contains("orch_fragment_audit_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe
            .honest_fence
            .contains("master_composition_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("invented_green=false")
        && fragment_audit_fence_honest(&fragment_audit_fence())
        && fragment_audit_tombstone_honest(&fragment_audit_tombstone())
}

/// Validate fragment audit honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_fragment_audit_honesty() -> Result<(), &'static str> {
    let probe = fragment_audit_probe();
    if probe.production_wired {
        return Err("orchestrator_loop_production_wired must stay false until W1-19 lands");
    }
    if probe.master_composition_wired {
        return Err("master_composition_wired must stay false until W4-JG-6 closes");
    }
    if probe.physics_green || probe.invented_green {
        return Err("physics_green/invented_green must stay false — audit is enumeration only");
    }
    if !probe.command_phase_deferred {
        return Err("COMMAND_PHASE_DEFERRED must stay true (no Command fragment row)");
    }
    if probe.scaffold_coverage_pct != 22 {
        return Err("scaffold_coverage_pct pinned at 22 per M5_ORCH_FRAGMENT_AUDIT_1052");
    }
    if probe.scaffold_gap_pct != 78 {
        return Err("scaffold_gap_pct must remain 78 complementary to 22% coverage");
    }
    if phase_has_fragment(LoopPhase::Command) {
        return Err("Command phase must have no fragment row (gateway deferred W1-19)");
    }
    if !LoopPhase::Command.is_command_deferred() {
        return Err("Command phase must report is_command_deferred");
    }
    if SLOT_BOUND_FRAGMENTS.len() != unwired_fragment_count() {
        return Err("slot-bound set must equal unwired count @ audit");
    }
    for f in SLOT_BOUND_FRAGMENTS {
        if !f.needs_slot_attachment() {
            return Err("slot-bound fragment must need_slot_attachment");
        }
        if !matches!(fragment_status(f), FragmentWireStatus::Unwired { .. }) {
            return Err("slot-bound fragment must remain Unwired @ audit");
        }
    }
    for f in GATEWAY_NATIVE_FRAGMENTS {
        if !f.is_gateway_native() {
            return Err("gateway-native fragment predicate failed");
        }
        if f.needs_slot_attachment() {
            return Err("gateway-native fragment must not need_slot_attachment");
        }
    }
    if !fragment_audit_honest(&probe) {
        return Err("fragment_audit_honest failed");
    }
    if !fragment_audit_fence_honest(&fragment_audit_fence()) {
        return Err("fragment_audit_fence_honest failed");
    }
    if !fragment_audit_tombstone_honest(&fragment_audit_tombstone()) {
        return Err("fragment_audit_tombstone_honest failed");
    }
    Ok(())
}

/// Full audit table for telemetry and receipt ceremony.
#[must_use]
pub fn audit_report() -> Vec<(EmbodiedFragment, FragmentWireStatus)> {
    ALL_FRAGMENTS
        .iter()
        .copied()
        .map(|f| (f, fragment_status(f)))
        .collect()
}

/// Gap fragments that must be wired before loop-close claims (W4-JG-6).
#[must_use]
pub fn unwired_gaps() -> Vec<&'static str> {
    vec![
        "field client (umst-field)",
        "XR presenter (umst-xr)",
        "robot executor (umst-robots)",
        "sense loop close (full M5)",
    ]
}

/// Unwired fragment target slugs derived from [`fragment_status`] (not hand-maintained labels).
#[must_use]
pub fn unwired_targets() -> Vec<&'static str> {
    ALL_FRAGMENTS
        .iter()
        .filter_map(|f| match fragment_status(*f) {
            FragmentWireStatus::Unwired { target } => Some(target),
            _ => None,
        })
        .collect()
}

/// Partial-gap strings derived from [`fragment_status`].
#[must_use]
pub fn partial_gaps() -> Vec<&'static str> {
    ALL_FRAGMENTS
        .iter()
        .filter_map(|f| match fragment_status(*f) {
            FragmentWireStatus::Partial { gap } => Some(gap),
            _ => None,
        })
        .collect()
}

/// Fragments whose phase equals `phase` (empty for [`LoopPhase::Command`]).
#[must_use]
pub fn fragments_for_phase(phase: LoopPhase) -> Vec<EmbodiedFragment> {
    ALL_FRAGMENTS
        .iter()
        .copied()
        .filter(|f| phase_for_fragment(*f) == phase)
        .collect()
}

/// Phases that are still unwired @ audit (everything except Gate).
#[must_use]
pub fn unwired_phases() -> Vec<LoopPhase> {
    ALL_LOOP_PHASES
        .iter()
        .copied()
        .filter(|p| !phase_wired(*p))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_matches_1052_table() {
        assert!(fragment_status(EmbodiedFragment::ManifoldGateway).is_wired());
        assert!(fragment_status(EmbodiedFragment::HostTransitionGates).is_wired());
        assert!(matches!(
            fragment_status(EmbodiedFragment::ThermodynamicCbf),
            FragmentWireStatus::Partial { .. }
        ));
        assert!(matches!(
            fragment_status(EmbodiedFragment::FieldSenseClient),
            FragmentWireStatus::Unwired {
                target: "umst-field"
            }
        ));
        assert!(matches!(
            fragment_status(EmbodiedFragment::XrPresenter),
            FragmentWireStatus::Unwired { target: "umst-xr" }
        ));
        assert!(matches!(
            fragment_status(EmbodiedFragment::RobotExecutor),
            FragmentWireStatus::Unwired {
                target: "umst-robots"
            }
        ));
        assert!(matches!(
            fragment_status(EmbodiedFragment::SenseLoopClose),
            FragmentWireStatus::Unwired { .. }
        ));
    }

    #[test]
    fn coverage_is_22_pct() {
        assert_eq!(scaffold_coverage_pct(), 22);
        assert_eq!(scaffold_gap_pct(), 78);
        assert_eq!(wired_fragment_count(), 2);
        assert_eq!(partial_fragment_count(), 1);
        assert_eq!(unwired_fragment_count(), 4);
        assert_eq!(composable_fragment_count(), 3);
        assert_eq!(wired_only_coverage_pct(), 28);
    }

    #[test]
    fn only_gate_phase_wired() {
        assert!(phase_wired(LoopPhase::Gate));
        assert!(!phase_wired(LoopPhase::Sense));
        assert!(!phase_wired(LoopPhase::Command));
        assert!(!phase_wired(LoopPhase::Present));
        assert!(!phase_wired(LoopPhase::Actuate));
        assert!(!phase_wired(LoopPhase::LoopClose));
        assert_eq!(wired_phase_count(), 1);
        assert_eq!(unwired_phase_count(), 5);
        assert_eq!(unwired_phases().len(), 5);
    }

    #[test]
    fn command_phase_has_no_fragment_row() {
        assert!(!phase_has_fragment(LoopPhase::Command));
        assert!(fragments_for_phase(LoopPhase::Command).is_empty());
        assert_eq!(fragments_for_phase(LoopPhase::Gate).len(), 3);
        assert_eq!(fragments_for_phase(LoopPhase::Sense).len(), 1);
    }

    #[test]
    fn fragment_audit_table_matches_status_fn() {
        assert_eq!(FRAGMENT_AUDIT_TABLE.len(), ALL_FRAGMENTS.len());
        for row in FRAGMENT_AUDIT_TABLE {
            assert_eq!(row.status, fragment_status(row.fragment));
            assert_eq!(row.phase, phase_for_fragment(row.fragment));
            assert_eq!(audit_row_for(row.fragment), Some(&row));
        }
    }

    #[test]
    fn fragment_audit_probe_honest_fence() {
        let probe = fragment_audit_probe();
        assert!(fragment_audit_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(!probe.physics_green);
        assert!(!probe.invented_green);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert_eq!(probe.owner_card, "W1-19");
        assert_eq!(probe.source_anchor, SOURCE_ANCHOR_PATH);
        validate_fragment_audit_honesty().expect("validate_fragment_audit_honesty");
    }

    #[test]
    fn typed_fence_refuses_green() {
        let fence = fragment_audit_fence();
        assert!(fragment_audit_fence_honest(&fence));
        assert!(fence.audit_landed);
        assert!(!fence.production_wired);
        assert!(!fence.master_composition_wired);
        assert!(!fence.physics_green);
        assert!(!fence.invented_green);
        assert!(fence.command_phase_deferred);
        assert_eq!(fence.scaffold_coverage_pct, 22);
        assert_eq!(fence.scaffold_gap_pct, 78);
        assert_eq!(fence.wired_count, 2);
        assert_eq!(fence.unwired_count, 4);
        assert_eq!(fence.composable_count, 3);
        assert_eq!(fence.wired_phase_count, 1);
        assert_eq!(fence.slot_bound_count, 4);
        assert_eq!(fence.gateway_native_count, 3);
        assert_eq!(fence.owner_card, OWNER_CARD);
        assert_eq!(fence.posture_tag, POSTURE_TAG);
        assert_eq!(fence.deepen_cell_id, DEEPEN_CELL_ID);
    }

    #[test]
    fn production_master_physics_stay_false() {
        assert!(!orchestrator_loop_production_wired());
        assert!(!master_composition_wired());
        assert!(!PHYSICS_GREEN);
        assert!(!INVENTED_GREEN);
        assert!(AUDIT_LANDED);
        assert!(PRODUCTION_WIRED_REFUSED);
        assert!(MASTER_COMPOSITION_REFUSED);
        assert!(COMMAND_PHASE_DEFERRED);
    }

    #[test]
    fn unwired_targets_match_audit_table() {
        let targets = unwired_targets();
        assert_eq!(targets.len(), 4);
        assert!(targets.contains(&"umst-field"));
        assert!(targets.contains(&"umst-xr"));
        assert!(targets.contains(&"umst-robots"));
        assert_eq!(partial_gaps().len(), 1);
        assert_eq!(unwired_gaps().len(), 4);
        assert_eq!(audit_report().len(), 7);
    }

    #[test]
    fn loop_phase_funnel_indices_are_dense() {
        for (i, phase) in ALL_LOOP_PHASES.iter().enumerate() {
            assert_eq!(phase.funnel_index() as usize, i);
        }
    }

    #[test]
    fn wire_status_predicates() {
        assert!(FragmentWireStatus::Wired.is_wired());
        assert!(FragmentWireStatus::Wired.is_composable());
        assert!(!FragmentWireStatus::Wired.is_partial());
        assert!(!FragmentWireStatus::Wired.is_unwired());
        let partial = FragmentWireStatus::Partial { gap: "x" };
        assert!(partial.is_partial());
        assert!(partial.is_composable());
        assert!(!partial.is_wired());
        let unwired = FragmentWireStatus::Unwired { target: "y" };
        assert!(unwired.is_unwired());
        assert!(!unwired.is_composable());
    }

    #[test]
    fn slot_bound_and_gateway_partition_audit_table() {
        assert_eq!(
            GATEWAY_NATIVE_FRAGMENTS.len() + SLOT_BOUND_FRAGMENTS.len(),
            ALL_FRAGMENTS.len()
        );
        for f in GATEWAY_NATIVE_FRAGMENTS {
            assert!(f.is_gateway_native());
            assert!(!f.needs_slot_attachment());
            assert!(!needs_slot_attachment(f));
        }
        for f in SLOT_BOUND_FRAGMENTS {
            assert!(f.needs_slot_attachment());
            assert!(!f.is_gateway_native());
            assert!(matches!(
                fragment_status(f),
                FragmentWireStatus::Unwired { .. }
            ));
        }
        assert_eq!(slot_bound_fragment_count(), unwired_fragment_count());
    }

    #[test]
    fn fragment_and_phase_slugs_stable() {
        assert_eq!(
            EmbodiedFragment::ManifoldGateway.as_str(),
            "manifold_gateway"
        );
        assert_eq!(
            EmbodiedFragment::SenseLoopClose.as_str(),
            "sense_loop_close"
        );
        assert_eq!(LoopPhase::Gate.as_str(), "gate");
        assert_eq!(LoopPhase::Command.as_str(), "command");
        assert!(LoopPhase::Command.is_command_deferred());
        assert!(!LoopPhase::Gate.is_command_deferred());
    }

    #[test]
    fn tombstone_refuses_green_production_master() {
        let t = fragment_audit_tombstone();
        assert!(fragment_audit_tombstone_honest(&t));
        assert_eq!(t.deepen_cell_id, "W29-034-FRAGMENT_AUDIT");
        assert_eq!(t.posture_tag, POSTURE_TAG);
        assert!(t.production_wired_refused);
        assert!(t.master_composition_refused);
        assert!(!t.physics_green);
        assert!(!t.invented_green);
        assert_eq!(t.scaffold_coverage_pct, 22);
        assert_eq!(t.scaffold_gap_pct, 78);
        assert_eq!(t.slot_bound_count, 4);
        assert_eq!(t.gateway_native_count, 3);
        validate_fragment_audit_honesty().expect("validate_fragment_audit_honesty");
    }

    #[test]
    fn audit_table_order_matches_all_fragments() {
        for (i, frag) in ALL_FRAGMENTS.iter().enumerate() {
            assert_eq!(FRAGMENT_AUDIT_TABLE[i].fragment, *frag);
        }
    }
}

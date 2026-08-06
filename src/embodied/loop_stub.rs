// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Orchestrator embodied-loop tick stub — M5-C07 / W4-JG composition scaffold.
//!
//! **Tombstone / `LEARNER_OPTIONAL` posture (SK-08):** this module is an honest
//! learner-optional witness — it sequences slot traits through mock/test harnesses
//! only. It does **not** claim production loop closure, live HAL I/O, or
//! `umst-gateway` Command-leg composition. Production embodied loop wiring is
//! **deferred** to W1-19 (`M5-IMPL-INT-01`).
//!
//! W29-037 deepen — six-phase honest fence matrix + gap inventory + deepen probe;
//! refuses `production_wired` / physics GREEN / MASTER / OP-5 invent.
//!
//! Constitutional funnel: `sense → command → gate → {present, actuate} → sense`
//!
//! W1-19 owns cross-crate loop wiring; this module sequences the manifold-side tick
//! through [`EmbodiedLoopSlots`] without claiming loop closure.
//!
//! Authority: `archived/residuals/misc-outputs-tmp/m5_prep/M5_ORCHESTRATOR_WIRING_1048.md` ·
//! SK-08 stub map @ `outputs/.tmp/UMST_WEB_RECONCILE_2009.md` §3.

use super::fragment_audit::{phase_wired, scaffold_coverage_pct, LoopPhase};
use super::fragment_slots::{ActuateDesign, EmbodiedLoopSlots};

/// Tensor/CBF evaluation on gate path — honestly not invoked in this stub.
pub const TENSOR_CBF_EVALUATED: bool = false;

/// Production embodied loop closure — not wired (W1-19 scope).
pub const PRODUCTION_LOOP_WIRED: bool = false;

/// SK-08 honesty defect id — embodied loop stub (companion: SK-09 sense_gate).
pub const STUB_DEFECT_ID: &str = "SK-08";

/// Contract-table classification — test harness witness, not production port.
pub const POSTURE_TAG: &str = "LEARNER_OPTIONAL";

/// Owning schedule card for production loop closure.
pub const OWNER_CARD: &str = "W1-19";

/// Primary source anchor for fleet / census hygiene.
pub const SOURCE_ANCHOR_PATH: &str = "umst-manifold/src/embodied/loop_stub.rs";

/// AC14 receipt slug — tombstone posture clarify @ 20:33 IST.
pub const RECEIPT_SLUG: &str = "COMPOSER_ACCEL_2030_AC14";

/// W29-037 cell id (manifold loop_stub deepen).
pub const LOOP_STUB_CELL_ID: &str = "W29-037-LOOP_STUB";

/// W29 deepen posture — mock tick scaffold only; no GREEN / OP-5 invent.
pub const LOOP_STUB_DEEPEN_POSTURE: &str = "honest-loop-stub-learner-optional-deepen-v2";

/// Admit lane stamp for this deepen pass (Grok coding fallback).
pub const LOOP_STUB_ADMIT_LANE: &str = "umst-admit-grok";

/// Pinned model id for this deepen pass.
pub const LOOP_STUB_ADMIT_MODEL: &str = "cursor-grok-4.5-high";

/// Compile-time honesty fence string for meta / fleet probes.
pub const LOOP_STUB_HONEST_FENCE: &str =
    "tick_scaffold_landed=true production_wired=false physics_green=false master_retick=false gateway_composed=false op5=false";

/// Constitutional funnel phase count (Sense…LoopClose).
pub const FUNNEL_PHASE_COUNT: usize = 6;

/// W4-JG scaffold coverage @ fragment audit (integer floor).
pub const SCAFFOLD_COVERAGE_PCT: u8 = 22;

/// Slot-trait tick scaffold is landed (mock-path constitutional sequencing).
pub const TICK_SCAFFOLD_LANDED: bool = true;

/// Production embodied loop closure — still open (W1-19 + gateway Command leg).
pub const PRODUCTION_LOOP_DEFERRED: bool = true;

/// `umst-gateway` Command-leg routing — not composed in this stub.
pub const GATEWAY_COMMAND_COMPOSED: bool = false;

/// Explicit refusal — no production wiring claim for this stub.
pub const PRODUCTION_WIRED: bool = false;

/// Explicit refusal — no physics GREEN invent at SK-08 stub seam.
pub const PHYSICS_GREEN_CLAIMED: bool = false;

/// Explicit refusal — MASTER retick not earned by mock tick scaffold.
pub const MASTER_RETICK_ELIGIBLE: bool = false;

/// Explicit refusal — OP-5 not earned by mock tick scaffold.
pub const OP5_CLAIMED: bool = false;

/// Audit-wired phase count today (Gate only).
pub const AUDIT_WIRED_PHASE_COUNT: usize = 1;

/// Audit-unwired phase count today (Sense/Command/Present/Actuate/LoopClose).
pub const AUDIT_UNWIRED_PHASE_COUNT: usize = FUNNEL_PHASE_COUNT - AUDIT_WIRED_PHASE_COUNT;

/// Fleet census line for embodied loop tombstone posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopStubTombstoneSummary {
    /// SK-08 honesty defect id.
    pub stub_defect_id: &'static str,
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// Owning schedule card for production closure.
    pub owner_card: &'static str,
    /// Whether mock-path tick scaffold is on disk.
    pub tick_scaffold_landed: bool,
    /// Whether production loop closure remains deferred.
    pub production_loop_deferred: bool,
    /// Whether gateway Command leg is composed.
    pub gateway_command_composed: bool,
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
}

/// Frozen tombstone summary — honest `LEARNER_OPTIONAL` witness only.
#[must_use]
pub const fn loop_stub_tombstone_summary() -> LoopStubTombstoneSummary {
    LoopStubTombstoneSummary {
        stub_defect_id: STUB_DEFECT_ID,
        posture_tag: POSTURE_TAG,
        owner_card: OWNER_CARD,
        tick_scaffold_landed: TICK_SCAFFOLD_LANDED,
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        gateway_command_composed: GATEWAY_COMMAND_COMPOSED,
        scaffold_coverage_pct: SCAFFOLD_COVERAGE_PCT,
    }
}

/// Tick phase markers aligned with blueprint §14.7.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoopTickPhase {
    Sense,
    Command,
    Gate,
    Present,
    Actuate,
    LoopClose,
}

impl LoopTickPhase {
    /// Map tick phase to fragment-audit [`LoopPhase`] for wiring probes.
    #[must_use]
    pub const fn to_audit_phase(self) -> LoopPhase {
        match self {
            Self::Sense => LoopPhase::Sense,
            Self::Command => LoopPhase::Command,
            Self::Gate => LoopPhase::Gate,
            Self::Present => LoopPhase::Present,
            Self::Actuate => LoopPhase::Actuate,
            Self::LoopClose => LoopPhase::LoopClose,
        }
    }

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

    /// Stable telemetry slug for census / gap inventory rows.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Sense => "sense",
            Self::Command => "command",
            Self::Gate => "gate",
            Self::Present => "present",
            Self::Actuate => "actuate",
            Self::LoopClose => "loop_close",
        }
    }
}

/// Honest per-phase wiring fence — audit truth, not slot population.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopPhaseFence {
    /// Tick phase under probe.
    pub phase: LoopTickPhase,
    /// Whether fragment audit marks this phase wired in the workspace.
    pub audit_wired: bool,
}

/// Full six-phase honest boundary fence for the embodied loop stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopStubHonestFence {
    /// Gate phase wired @ fragment audit (only true phase today).
    pub gate_phase_wired: bool,
    /// Sense phase wired @ fragment audit (false until umst-field composed).
    pub sense_phase_wired: bool,
    /// Command gateway composed (false — W1-19 scope).
    pub command_gateway_composed: bool,
    /// Present phase wired @ fragment audit.
    pub present_phase_wired: bool,
    /// Actuate phase wired @ fragment audit.
    pub actuate_phase_wired: bool,
    /// Loop-close phase wired @ fragment audit.
    pub loop_close_phase_wired: bool,
    /// Production loop closure deferred (always true for SK-08).
    pub production_loop_deferred: bool,
    /// Tensor/CBF path evaluated on gate (false — stub mint only).
    pub tensor_cbf_evaluated: bool,
    /// Production loop wired end-to-end (false — honest fence).
    pub production_loop_wired: bool,
}

/// Frozen honest fence — compile-time boundary witness.
#[must_use]
pub const fn loop_stub_honest_fence() -> LoopStubHonestFence {
    LoopStubHonestFence {
        gate_phase_wired: phase_wired(LoopPhase::Gate),
        sense_phase_wired: phase_wired(LoopPhase::Sense),
        command_gateway_composed: GATEWAY_COMMAND_COMPOSED,
        present_phase_wired: phase_wired(LoopPhase::Present),
        actuate_phase_wired: phase_wired(LoopPhase::Actuate),
        loop_close_phase_wired: phase_wired(LoopPhase::LoopClose),
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        tensor_cbf_evaluated: TENSOR_CBF_EVALUATED,
        production_loop_wired: PRODUCTION_LOOP_WIRED,
    }
}

/// Funnel-ordered tick phases for census / deepen matrix.
pub const FUNNEL_TICK_PHASES: [LoopTickPhase; FUNNEL_PHASE_COUNT] = [
    LoopTickPhase::Sense,
    LoopTickPhase::Command,
    LoopTickPhase::Gate,
    LoopTickPhase::Present,
    LoopTickPhase::Actuate,
    LoopTickPhase::LoopClose,
];

/// Six-phase audit fence matrix — honest wiring rows, not slot population.
#[must_use]
pub const fn loop_stub_phase_fence_matrix() -> [LoopPhaseFence; FUNNEL_PHASE_COUNT] {
    [
        LoopPhaseFence {
            phase: LoopTickPhase::Sense,
            audit_wired: phase_wired(LoopPhase::Sense),
        },
        LoopPhaseFence {
            phase: LoopTickPhase::Command,
            audit_wired: phase_wired(LoopPhase::Command),
        },
        LoopPhaseFence {
            phase: LoopTickPhase::Gate,
            audit_wired: phase_wired(LoopPhase::Gate),
        },
        LoopPhaseFence {
            phase: LoopTickPhase::Present,
            audit_wired: phase_wired(LoopPhase::Present),
        },
        LoopPhaseFence {
            phase: LoopTickPhase::Actuate,
            audit_wired: phase_wired(LoopPhase::Actuate),
        },
        LoopPhaseFence {
            phase: LoopTickPhase::LoopClose,
            audit_wired: phase_wired(LoopPhase::LoopClose),
        },
    ]
}

/// Count of phases marked wired @ fragment audit (Gate only today).
#[must_use]
pub const fn loop_stub_audit_wired_phase_count() -> usize {
    let matrix = loop_stub_phase_fence_matrix();
    let mut n = 0usize;
    let mut i = 0usize;
    while i < FUNNEL_PHASE_COUNT {
        if matrix[i].audit_wired {
            n += 1;
        }
        i += 1;
    }
    n
}

/// Count of phases marked unwired @ fragment audit (five today).
#[must_use]
pub const fn loop_stub_audit_unwired_phase_count() -> usize {
    FUNNEL_PHASE_COUNT - loop_stub_audit_wired_phase_count()
}

/// Honest audit gap inventory — unwired phase slugs only (no GREEN invent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopStubGapInventory {
    /// Audit-wired phase count (Gate only today).
    pub audit_wired_phase_count: usize,
    /// Audit-unwired phase count.
    pub audit_unwired_phase_count: usize,
    /// Unwired phase telemetry slugs (Sense…LoopClose minus Gate).
    pub unwired_phase_slugs: [&'static str; AUDIT_UNWIRED_PHASE_COUNT],
    /// Explicit OP-5 refusal.
    pub op5_claimed: bool,
    /// Explicit production-wiring refusal.
    pub production_wired: bool,
    /// Explicit physics-GREEN refusal.
    pub physics_green_claimed: bool,
    /// Explicit MASTER-retick refusal.
    pub master_retick_eligible: bool,
}

/// Frozen gap inventory for fleet / deepen census.
#[must_use]
pub const fn loop_stub_gap_inventory() -> LoopStubGapInventory {
    LoopStubGapInventory {
        audit_wired_phase_count: loop_stub_audit_wired_phase_count(),
        audit_unwired_phase_count: loop_stub_audit_unwired_phase_count(),
        unwired_phase_slugs: [
            LoopTickPhase::Sense.slug(),
            LoopTickPhase::Command.slug(),
            LoopTickPhase::Present.slug(),
            LoopTickPhase::Actuate.slug(),
            LoopTickPhase::LoopClose.slug(),
        ],
        op5_claimed: OP5_CLAIMED,
        production_wired: PRODUCTION_WIRED,
        physics_green_claimed: PHYSICS_GREEN_CLAIMED,
        master_retick_eligible: MASTER_RETICK_ELIGIBLE,
    }
}

/// Honest `production_wired` fence — never true until W1-19 lands measured wire.
#[must_use]
pub const fn loop_stub_production_wired() -> bool {
    PRODUCTION_WIRED
}

/// Physics GREEN fence — stub mint only; refuse invent.
#[must_use]
pub const fn loop_stub_physics_green_claimed() -> bool {
    PHYSICS_GREEN_CLAIMED
}

/// Master retick eligible — false @ SK-08 learner-optional deepen.
#[must_use]
pub const fn loop_stub_master_retick_eligible() -> bool {
    MASTER_RETICK_ELIGIBLE
}

/// OP-5 claim fence — stub mint only; refuse invent.
#[must_use]
pub const fn loop_stub_op5_claimed() -> bool {
    OP5_CLAIMED
}

/// Compile-time refusal pins — production / GREEN / MASTER / OP-5 stay closed.
const _: () = assert!(!PRODUCTION_WIRED);
const _: () = assert!(!PHYSICS_GREEN_CLAIMED);
const _: () = assert!(!MASTER_RETICK_ELIGIBLE);
const _: () = assert!(!OP5_CLAIMED);
const _: () = assert!(!PRODUCTION_LOOP_WIRED);
const _: () = assert!(!GATEWAY_COMMAND_COMPOSED);
const _: () = assert!(!TENSOR_CBF_EVALUATED);
const _: () = assert!(AUDIT_UNWIRED_PHASE_COUNT == 5);
const _: () = assert!(FUNNEL_PHASE_COUNT == AUDIT_WIRED_PHASE_COUNT + AUDIT_UNWIRED_PHASE_COUNT);

/// W29-037 deepen census — tick scaffold landed; production / GREEN / MASTER / OP-5 blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopStubDeepenProbe {
    /// Cell id for this deepen pass.
    pub cell_id: &'static str,
    /// SK-08 honesty defect id.
    pub stub_defect_id: &'static str,
    /// Contract-table classification tag.
    pub posture_tag: &'static str,
    /// W29 deepen posture slug.
    pub deepen_posture: &'static str,
    /// Admit lane stamp.
    pub admit_lane: &'static str,
    /// Pinned model id.
    pub admit_model: &'static str,
    /// Compile-time honesty fence string.
    pub honest_fence: &'static str,
    /// Owning schedule card for production closure.
    pub owner_card: &'static str,
    /// Whether mock-path tick scaffold is on disk.
    pub tick_scaffold_landed: bool,
    /// Constitutional funnel phase count.
    pub funnel_phase_count: usize,
    /// Audit-wired phase count (Gate only).
    pub audit_wired_phase_count: usize,
    /// Audit-unwired phase count.
    pub audit_unwired_phase_count: usize,
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
    /// Whether gateway Command leg is composed.
    pub gateway_command_composed: bool,
    /// Whether tensor/CBF path evaluated on gate.
    pub tensor_cbf_evaluated: bool,
    /// Whether production loop remains deferred.
    pub production_loop_deferred: bool,
    /// Explicit production-wiring refusal.
    pub production_wired: bool,
    /// Explicit physics-GREEN refusal.
    pub physics_green_claimed: bool,
    /// Explicit MASTER-retick refusal.
    pub master_retick_eligible: bool,
    /// Explicit OP-5 refusal.
    pub op5_claimed: bool,
    /// Frozen six-phase honest fence (audit + deferral).
    pub fence: LoopStubHonestFence,
    /// Audit gap inventory (unwired slugs + refusal pins).
    pub gap_inventory: LoopStubGapInventory,
}

/// Static honest deepen probe — surfaces scaffold without inventing GREEN.
#[must_use]
pub const fn loop_stub_deepen_probe() -> LoopStubDeepenProbe {
    LoopStubDeepenProbe {
        cell_id: LOOP_STUB_CELL_ID,
        stub_defect_id: STUB_DEFECT_ID,
        posture_tag: POSTURE_TAG,
        deepen_posture: LOOP_STUB_DEEPEN_POSTURE,
        admit_lane: LOOP_STUB_ADMIT_LANE,
        admit_model: LOOP_STUB_ADMIT_MODEL,
        honest_fence: LOOP_STUB_HONEST_FENCE,
        owner_card: OWNER_CARD,
        tick_scaffold_landed: TICK_SCAFFOLD_LANDED,
        funnel_phase_count: FUNNEL_PHASE_COUNT,
        audit_wired_phase_count: loop_stub_audit_wired_phase_count(),
        audit_unwired_phase_count: loop_stub_audit_unwired_phase_count(),
        scaffold_coverage_pct: SCAFFOLD_COVERAGE_PCT,
        gateway_command_composed: GATEWAY_COMMAND_COMPOSED,
        tensor_cbf_evaluated: TENSOR_CBF_EVALUATED,
        production_loop_deferred: PRODUCTION_LOOP_DEFERRED,
        production_wired: loop_stub_production_wired(),
        physics_green_claimed: loop_stub_physics_green_claimed(),
        master_retick_eligible: loop_stub_master_retick_eligible(),
        op5_claimed: loop_stub_op5_claimed(),
        fence: loop_stub_honest_fence(),
        gap_inventory: loop_stub_gap_inventory(),
    }
}

/// Honesty gate for operator receipts — scaffold only, no production / GREEN / MASTER / OP-5 flip.
#[must_use]
pub fn loop_stub_deepen_honest(probe: &LoopStubDeepenProbe) -> bool {
    probe.cell_id == LOOP_STUB_CELL_ID
        && probe.stub_defect_id == STUB_DEFECT_ID
        && probe.posture_tag == POSTURE_TAG
        && probe.deepen_posture == LOOP_STUB_DEEPEN_POSTURE
        && probe.admit_lane == LOOP_STUB_ADMIT_LANE
        && probe.admit_model == LOOP_STUB_ADMIT_MODEL
        && probe.honest_fence == LOOP_STUB_HONEST_FENCE
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master_retick=false")
        && probe.honest_fence.contains("gateway_composed=false")
        && probe.honest_fence.contains("op5=false")
        && probe.owner_card == OWNER_CARD
        && probe.tick_scaffold_landed
        && probe.funnel_phase_count == FUNNEL_PHASE_COUNT
        && probe.audit_wired_phase_count == AUDIT_WIRED_PHASE_COUNT
        && probe.audit_unwired_phase_count == AUDIT_UNWIRED_PHASE_COUNT
        && probe.scaffold_coverage_pct == SCAFFOLD_COVERAGE_PCT
        && !probe.gateway_command_composed
        && !probe.tensor_cbf_evaluated
        && probe.production_loop_deferred
        && !probe.production_wired
        && !probe.physics_green_claimed
        && !probe.master_retick_eligible
        && !probe.op5_claimed
        && probe.fence.gate_phase_wired
        && !probe.fence.sense_phase_wired
        && !probe.fence.command_gateway_composed
        && !probe.fence.present_phase_wired
        && !probe.fence.actuate_phase_wired
        && !probe.fence.loop_close_phase_wired
        && probe.fence.production_loop_deferred
        && !probe.fence.tensor_cbf_evaluated
        && !probe.fence.production_loop_wired
        && probe.gap_inventory.audit_wired_phase_count == AUDIT_WIRED_PHASE_COUNT
        && probe.gap_inventory.audit_unwired_phase_count == AUDIT_UNWIRED_PHASE_COUNT
        && !probe.gap_inventory.op5_claimed
        && !probe.gap_inventory.production_wired
        && !probe.gap_inventory.physics_green_claimed
        && !probe.gap_inventory.master_retick_eligible
}

/// Validate loop_stub deepen honesty — fail closed on fake GREEN / production / MASTER / OP-5.
pub fn validate_loop_stub_deepen_honesty() -> Result<(), &'static str> {
    let probe = loop_stub_deepen_probe();
    if probe.production_wired || loop_stub_production_wired() {
        return Err("PRODUCTION_WIRED must stay false until W1-19 measured wire");
    }
    if probe.physics_green_claimed || loop_stub_physics_green_claimed() {
        return Err("PHYSICS_GREEN_CLAIMED must stay false — no invent GREEN");
    }
    if probe.master_retick_eligible || loop_stub_master_retick_eligible() {
        return Err("MASTER_RETICK_ELIGIBLE must stay false at SK-08 stub deepen");
    }
    if probe.op5_claimed || loop_stub_op5_claimed() {
        return Err("OP5_CLAIMED must stay false — no invent OP-5");
    }
    if probe.gateway_command_composed || GATEWAY_COMMAND_COMPOSED {
        return Err("GATEWAY_COMMAND_COMPOSED must stay false — Command leg deferred");
    }
    if probe.tensor_cbf_evaluated || TENSOR_CBF_EVALUATED {
        return Err("TENSOR_CBF_EVALUATED must stay false — stub mint only");
    }
    if probe.audit_wired_phase_count != AUDIT_WIRED_PHASE_COUNT {
        return Err("audit_wired_phase_count pinned at Gate-only (1)");
    }
    if probe.audit_unwired_phase_count != AUDIT_UNWIRED_PHASE_COUNT {
        return Err("audit_unwired_phase_count pinned at 5");
    }
    if probe.scaffold_coverage_pct != SCAFFOLD_COVERAGE_PCT {
        return Err("scaffold_coverage_pct pinned at 22 per fragment audit");
    }
    if probe.gap_inventory != loop_stub_gap_inventory() {
        return Err("gap_inventory must match frozen loop_stub_gap_inventory");
    }
    if !loop_stub_deepen_honest(&probe) {
        return Err("loop_stub_deepen_honest failed");
    }
    Ok(())
}

/// Fleet census probe — deepen + tombstone + fence matrix + gap inventory rollup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoopStubCensusProbe {
    /// Cell id for this deepen pass.
    pub cell_id: &'static str,
    /// Source anchor path.
    pub source_anchor: &'static str,
    /// Receipt slug (AC14 tombstone).
    pub receipt_slug: &'static str,
    /// Tombstone summary.
    pub tombstone: LoopStubTombstoneSummary,
    /// Deepen probe.
    pub deepen: LoopStubDeepenProbe,
    /// Six-phase fence matrix.
    pub phase_matrix: [LoopPhaseFence; FUNNEL_PHASE_COUNT],
    /// Audit gap inventory.
    pub gap_inventory: LoopStubGapInventory,
}

/// Build census probe for fleet / vigil handback.
#[must_use]
pub const fn loop_stub_census_probe() -> LoopStubCensusProbe {
    LoopStubCensusProbe {
        cell_id: LOOP_STUB_CELL_ID,
        source_anchor: SOURCE_ANCHOR_PATH,
        receipt_slug: RECEIPT_SLUG,
        tombstone: loop_stub_tombstone_summary(),
        deepen: loop_stub_deepen_probe(),
        phase_matrix: loop_stub_phase_fence_matrix(),
        gap_inventory: loop_stub_gap_inventory(),
    }
}

/// Orchestrator position in the constitutional funnel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrchestratorLoopRole {
    /// Gate composer — tensor/CBF witness mint (manifold-side stub).
    GateComposer,
    /// Loop coordinator — sequences slots without owning world I/O.
    LoopCoordinator,
}

/// Gateway-admitted witness minted by the gate stub (full tensor path: W1-19).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GateAdmissionStub {
    /// Digest derived from sense witness + monotonic sequence.
    pub witness_digest: [u8; 32],
    /// Monotonic admission counter for hold-scene / stale checks.
    pub sequence: u64,
    /// Supervisory clearance badge (stub — always requires `true` at admission).
    pub clearance_witness: bool,
}

/// Command-leg deferral marker — `umst-gateway` not composed in this stub.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandLegDeferral {
    /// Honest marker: Command phase is not wired @ M5-C07.
    pub gateway_composed: bool,
}

/// One constitutional tick output (stub — no HAL / gateway I/O).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoopTickResult {
    /// Sense witness digest captured at T1 (J7 pre column on mock path).
    pub sense_witness_digest: [u8; 32],
    /// Phases completed in order before success or early reject.
    pub phases_completed: Vec<LoopTickPhase>,
    /// Gate admission witness when gate phase succeeds.
    pub gate_admission: GateAdmissionStub,
    /// Present-leg scene digest when XR slot succeeds.
    pub present_scene_digest: [u8; 32],
    /// Whether actuate leg reported success.
    pub actuated: bool,
    /// Post-actuation re-sense digest when loop-close succeeds.
    pub loop_close_digest: [u8; 32],
    /// Honest W4-JG scaffold coverage (integer floor).
    pub scaffold_coverage_pct: u8,
    /// Command leg honestly deferred — `umst-gateway` not composed.
    pub command_deferred: bool,
}

/// Reasons the orchestrator loop stub must reject before advancing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoopStubReject {
    /// Sense slot not populated.
    SenseUnwired,
    /// Sense leg returned an error.
    SenseFailed {
        detail: String,
    },
    /// Sense witness digest is zero — fail-closed.
    InvalidSenseWitness,
    /// Present slot not populated.
    PresentUnwired,
    PresentFailed {
        detail: String,
    },
    /// Actuate slot not populated.
    ActuateUnwired,
    ActuateFailed {
        detail: String,
    },
    /// Loop-close slot not populated.
    LoopCloseUnwired,
    LoopCloseFailed {
        detail: String,
    },
    /// Gate stub rejected admission (uncleared envelope).
    GateInadmissible {
        slug: &'static str,
    },
}

impl LoopStubReject {
    /// Constitutional phase at which this reject occurred (fail-closed boundary).
    #[must_use]
    pub const fn failed_phase(&self) -> LoopTickPhase {
        match self {
            Self::SenseUnwired | Self::SenseFailed { .. } | Self::InvalidSenseWitness => {
                LoopTickPhase::Sense
            }
            Self::GateInadmissible { .. } => LoopTickPhase::Gate,
            Self::PresentUnwired | Self::PresentFailed { .. } => LoopTickPhase::Present,
            Self::ActuateUnwired | Self::ActuateFailed { .. } => LoopTickPhase::Actuate,
            Self::LoopCloseUnwired | Self::LoopCloseFailed { .. } => LoopTickPhase::LoopClose,
        }
    }

    /// Static slug for telemetry (no user detail leakage).
    #[must_use]
    pub const fn slug(&self) -> &'static str {
        match self {
            Self::SenseUnwired => "sense_unwired",
            Self::SenseFailed { .. } => "sense_failed",
            Self::InvalidSenseWitness => "invalid_sense_witness",
            Self::PresentUnwired => "present_unwired",
            Self::PresentFailed { .. } => "present_failed",
            Self::ActuateUnwired => "actuate_unwired",
            Self::ActuateFailed { .. } => "actuate_failed",
            Self::LoopCloseUnwired => "loop_close_unwired",
            Self::LoopCloseFailed { .. } => "loop_close_failed",
            Self::GateInadmissible { slug } => slug,
        }
    }
}

/// Stateful orchestrator loop stub — sequences slot traits through one tick.
#[derive(Default)]
pub struct EmbodiedLoopStub {
    slots: EmbodiedLoopSlots,
    sequence: u64,
    last_admission: Option<GateAdmissionStub>,
}

impl EmbodiedLoopStub {
    /// Construct with optional leg slots (all default to unwired).
    #[must_use]
    pub fn new(slots: EmbodiedLoopSlots) -> Self {
        Self {
            slots,
            sequence: 0,
            last_admission: None,
        }
    }

    /// Orchestrator role at this boundary (gate composer + loop coordinator).
    #[must_use]
    pub const fn loop_role() -> OrchestratorLoopRole {
        OrchestratorLoopRole::LoopCoordinator
    }

    /// Whether the gate phase alone is wired in the current workspace.
    #[must_use]
    pub const fn gate_phase_wired() -> bool {
        phase_wired(LoopPhase::Gate)
    }

    /// Honest scaffold coverage for receipt ceremony.
    #[must_use]
    pub const fn scaffold_coverage_pct() -> u8 {
        scaffold_coverage_pct()
    }

    /// Last gate admission held by this stub (hold-scene policy).
    #[must_use]
    pub fn held_admission(&self) -> Option<GateAdmissionStub> {
        self.last_admission
    }

    /// Command-leg deferral probe — gateway composition is W1-19 scope.
    #[must_use]
    pub const fn command_leg_deferral() -> CommandLegDeferral {
        CommandLegDeferral {
            gateway_composed: GATEWAY_COMMAND_COMPOSED,
        }
    }

    /// Frozen tombstone summary for fleet / census hygiene.
    #[must_use]
    pub const fn tombstone_summary() -> LoopStubTombstoneSummary {
        loop_stub_tombstone_summary()
    }

    /// Honest six-phase boundary fence (audit wiring + deferral markers).
    #[must_use]
    pub const fn honest_fence() -> LoopStubHonestFence {
        loop_stub_honest_fence()
    }

    /// W29-037 deepen probe (scaffold landed; GREEN / production / MASTER refused).
    #[must_use]
    pub const fn deepen_probe() -> LoopStubDeepenProbe {
        loop_stub_deepen_probe()
    }

    /// Fleet census probe (tombstone + deepen + phase matrix).
    #[must_use]
    pub const fn census_probe() -> LoopStubCensusProbe {
        loop_stub_census_probe()
    }

    /// Six-phase audit fence matrix.
    #[must_use]
    pub const fn phase_fence_matrix() -> [LoopPhaseFence; FUNNEL_PHASE_COUNT] {
        loop_stub_phase_fence_matrix()
    }

    /// Audit gap inventory (unwired slugs + refusal pins).
    #[must_use]
    pub const fn gap_inventory() -> LoopStubGapInventory {
        loop_stub_gap_inventory()
    }

    /// Per-phase wiring probe against fragment audit (not slot population).
    #[must_use]
    pub const fn phase_wired_at_audit(phase: LoopTickPhase) -> bool {
        phase_wired(phase.to_audit_phase())
    }

    /// Build a per-phase fence row for `phase`.
    #[must_use]
    pub const fn phase_fence(phase: LoopTickPhase) -> LoopPhaseFence {
        LoopPhaseFence {
            phase,
            audit_wired: phase_wired(phase.to_audit_phase()),
        }
    }

    /// Monotonic admission sequence (0 before first successful tick).
    #[must_use]
    pub fn admission_sequence(&self) -> u64 {
        self.sequence
    }

    /// Run one constitutional tick through populated slots.
    ///
    /// Fail-closed: missing slots, zero sense witness, or leg errors abort without
    /// partial actuation.
    pub fn tick(&mut self) -> Result<LoopTickResult, LoopStubReject> {
        let mut phases = Vec::with_capacity(6);

        // Sense
        let sense = self.sense_leg()?;
        phases.push(LoopTickPhase::Sense);

        if sense.witness_digest == [0u8; 32] {
            return Err(LoopStubReject::InvalidSenseWitness);
        }

        // Command — honest deferral stub (no umst-gateway routing)
        let _command = Self::command_leg_deferral();
        phases.push(LoopTickPhase::Command);

        // Gate — manifold-side witness mint (tensor path delegated to W1-19)
        self.sequence = self.sequence.saturating_add(1);
        let admission = mint_gate_admission(&sense.witness_digest, self.sequence)?;
        phases.push(LoopTickPhase::Gate);

        // Present
        let scene = self.present_leg(&admission.witness_digest)?;
        phases.push(LoopTickPhase::Present);

        // Actuate
        self.actuate_leg(&admission.witness_digest)?;
        phases.push(LoopTickPhase::Actuate);

        // Loop close
        let closed = self.loop_close_leg()?;
        phases.push(LoopTickPhase::LoopClose);

        self.last_admission = Some(admission);

        Ok(LoopTickResult {
            sense_witness_digest: sense.witness_digest,
            phases_completed: phases,
            present_scene_digest: scene.scene_digest,
            actuated: true,
            loop_close_digest: closed.witness_digest,
            gate_admission: admission,
            scaffold_coverage_pct: Self::scaffold_coverage_pct(),
            command_deferred: !GATEWAY_COMMAND_COMPOSED,
        })
    }

    fn sense_leg(
        &mut self,
    ) -> Result<super::fragment_slots::SenseObservation, LoopStubReject> {
        let client = self
            .slots
            .field_sense
            .as_mut()
            .ok_or(LoopStubReject::SenseUnwired)?;
        client.sense().map_err(|e| LoopStubReject::SenseFailed {
            detail: e.detail,
        })
    }

    fn present_leg(
        &self,
        admissible_digest: &[u8; 32],
    ) -> Result<super::fragment_slots::PresentScene, LoopStubReject> {
        let presenter = self
            .slots
            .xr_present
            .as_ref()
            .ok_or(LoopStubReject::PresentUnwired)?;
        presenter
            .present(admissible_digest)
            .map_err(|e| LoopStubReject::PresentFailed {
                detail: e.detail,
            })
    }

    fn actuate_leg(&mut self, design_digest: &[u8; 32]) -> Result<(), LoopStubReject> {
        let executor = self
            .slots
            .robot_actuate
            .as_mut()
            .ok_or(LoopStubReject::ActuateUnwired)?;
        executor
            .actuate(&ActuateDesign {
                design_digest: *design_digest,
            })
            .map_err(|e| LoopStubReject::ActuateFailed {
                detail: e.detail,
            })
    }

    fn loop_close_leg(
        &mut self,
    ) -> Result<super::fragment_slots::SenseObservation, LoopStubReject> {
        let closer = self
            .slots
            .loop_close
            .as_mut()
            .ok_or(LoopStubReject::LoopCloseUnwired)?;
        closer.close_loop().map_err(|e| LoopStubReject::LoopCloseFailed {
            detail: e.detail,
        })
    }
}

/// Mint gate admission witness from sense digest (stub — no tensor/CBF evaluation).
fn mint_gate_admission(
    sense_digest: &[u8; 32],
    sequence: u64,
) -> Result<GateAdmissionStub, LoopStubReject> {
    let mut witness_digest = *sense_digest;
    witness_digest[24..32].copy_from_slice(&sequence.to_le_bytes());
    let clearance_witness = witness_digest != [0u8; 32];
    if !clearance_witness {
        return Err(LoopStubReject::GateInadmissible {
            slug: "zero_witness_digest",
        });
    }
    Ok(GateAdmissionStub {
        witness_digest,
        sequence,
        clearance_witness,
    })
}

/// Single-shot tick convenience for harnesses.
pub fn embodied_loop_tick_stub(
    slots: EmbodiedLoopSlots,
) -> Result<LoopTickResult, LoopStubReject> {
    EmbodiedLoopStub::new(slots).tick()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embodied::{
        FieldSenseError, FieldSenseClient, LoopCloseError, PresentError, PresentScene,
        RobotExecutor, SenseObservation, SenseLoopCloser, XrPresenter,
    };

    struct StubSense;

    impl FieldSenseClient for StubSense {
        fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
            Ok(SenseObservation {
                witness_digest: [0xAB; 32],
            })
        }
    }

    struct StubPresent;

    impl XrPresenter for StubPresent {
        fn present(&self, digest: &[u8; 32]) -> Result<PresentScene, PresentError> {
            Ok(PresentScene {
                scene_digest: *digest,
            })
        }
    }

    struct StubActuate;

    impl RobotExecutor for StubActuate {
        fn actuate(&mut self, _design: &ActuateDesign) -> Result<(), super::super::fragment_slots::ActuateError> {
            Ok(())
        }
    }

    struct StubCloser;

    impl SenseLoopCloser for StubCloser {
        fn close_loop(&mut self) -> Result<SenseObservation, LoopCloseError> {
            Ok(SenseObservation {
                witness_digest: [0xCD; 32],
            })
        }
    }

    fn wired_slots() -> EmbodiedLoopSlots {
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        slots.xr_present = Some(Box::new(StubPresent));
        slots.robot_actuate = Some(Box::new(StubActuate));
        slots.loop_close = Some(Box::new(StubCloser));
        slots
    }

    #[test]
    fn gate_phase_wired_honest() {
        assert!(EmbodiedLoopStub::gate_phase_wired());
        assert!(!EmbodiedLoopStub::command_leg_deferral().gateway_composed);
    }

    #[test]
    fn scaffold_coverage_22_pct() {
        assert_eq!(EmbodiedLoopStub::scaffold_coverage_pct(), 22);
    }

    #[test]
    fn default_slots_fail_at_sense_unwired() {
        let mut stub = EmbodiedLoopStub::new(EmbodiedLoopSlots::new());
        let err = stub.tick().unwrap_err();
        assert_eq!(err, LoopStubReject::SenseUnwired);
    }

    #[test]
    fn wired_slots_complete_full_tick() {
        let mut stub = EmbodiedLoopStub::new(wired_slots());
        let result = stub.tick().expect("tick");
        assert_eq!(result.phases_completed.len(), 6);
        assert_eq!(
            result.phases_completed[0],
            LoopTickPhase::Sense
        );
        assert_eq!(
            result.phases_completed.last(),
            Some(&LoopTickPhase::LoopClose)
        );
        assert!(result.actuated);
        assert_eq!(result.loop_close_digest, [0xCD; 32]);
        assert_eq!(result.scaffold_coverage_pct, 22);
        assert!(result.command_deferred);
        assert!(stub.held_admission().is_some());
    }

    #[test]
    fn zero_sense_witness_rejects_before_gate() {
        struct ZeroSense;
        impl FieldSenseClient for ZeroSense {
            fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
                Ok(SenseObservation {
                    witness_digest: [0u8; 32],
                })
            }
        }
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(ZeroSense));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err, LoopStubReject::InvalidSenseWitness);
    }

    #[test]
    fn loop_stub_tombstone_posture_locked() {
        let summary = loop_stub_tombstone_summary();
        assert_eq!(summary.stub_defect_id, "SK-08");
        assert_eq!(summary.posture_tag, "LEARNER_OPTIONAL");
        assert_eq!(summary.owner_card, "W1-19");
        assert!(summary.tick_scaffold_landed);
        assert!(summary.production_loop_deferred);
        assert!(!summary.gateway_command_composed);
        assert_eq!(summary.scaffold_coverage_pct, SCAFFOLD_COVERAGE_PCT);
        assert_eq!(SOURCE_ANCHOR_PATH, "umst-manifold/src/embodied/loop_stub.rs");
        assert_eq!(RECEIPT_SLUG, "COMPOSER_ACCEL_2030_AC14");
        assert_eq!(EmbodiedLoopStub::tombstone_summary(), summary);
    }

    #[test]
    fn loop_role_is_coordinator() {
        assert_eq!(
            EmbodiedLoopStub::loop_role(),
            OrchestratorLoopRole::LoopCoordinator
        );
    }

    #[test]
    fn honest_fence_only_gate_wired() {
        let fence = loop_stub_honest_fence();
        assert!(fence.gate_phase_wired);
        assert!(!fence.sense_phase_wired);
        assert!(!fence.command_gateway_composed);
        assert!(!fence.present_phase_wired);
        assert!(!fence.actuate_phase_wired);
        assert!(!fence.loop_close_phase_wired);
        assert!(fence.production_loop_deferred);
        assert!(!fence.tensor_cbf_evaluated);
        assert!(!fence.production_loop_wired);
        assert_eq!(EmbodiedLoopStub::honest_fence(), fence);
    }

    #[test]
    fn phase_fence_matches_audit() {
        assert!(EmbodiedLoopStub::phase_wired_at_audit(LoopTickPhase::Gate));
        assert!(!EmbodiedLoopStub::phase_wired_at_audit(LoopTickPhase::Sense));
        assert!(!EmbodiedLoopStub::phase_wired_at_audit(LoopTickPhase::LoopClose));
        let gate_fence = EmbodiedLoopStub::phase_fence(LoopTickPhase::Gate);
        assert_eq!(gate_fence.phase, LoopTickPhase::Gate);
        assert!(gate_fence.audit_wired);
    }

    #[test]
    fn tick_phase_funnel_order() {
        assert_eq!(LoopTickPhase::Sense.funnel_index(), 0);
        assert_eq!(LoopTickPhase::LoopClose.funnel_index(), 5);
        assert_eq!(
            LoopTickPhase::Gate.to_audit_phase(),
            super::super::fragment_audit::LoopPhase::Gate
        );
    }

    #[test]
    fn wired_tick_marks_command_deferred() {
        let mut stub = EmbodiedLoopStub::new(wired_slots());
        let result = stub.tick().expect("tick");
        assert!(result.command_deferred);
        assert_eq!(stub.admission_sequence(), 1);
    }

    #[test]
    fn admission_sequence_monotonic() {
        let mut stub = EmbodiedLoopStub::new(wired_slots());
        let first = stub.tick().expect("tick1");
        let second = stub.tick().expect("tick2");
        assert_eq!(first.gate_admission.sequence, 1);
        assert_eq!(second.gate_admission.sequence, 2);
        assert!(second.gate_admission.sequence > first.gate_admission.sequence);
    }

    #[test]
    fn present_unwired_rejects_at_present_phase() {
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::Present);
        assert_eq!(err.slug(), "present_unwired");
    }

    #[test]
    fn actuate_unwired_rejects_at_actuate_phase() {
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        slots.xr_present = Some(Box::new(StubPresent));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::Actuate);
        assert_eq!(err.slug(), "actuate_unwired");
    }

    #[test]
    fn loop_close_unwired_rejects_at_close_phase() {
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        slots.xr_present = Some(Box::new(StubPresent));
        slots.robot_actuate = Some(Box::new(StubActuate));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::LoopClose);
        assert_eq!(err.slug(), "loop_close_unwired");
    }

    #[test]
    fn sense_unwired_slug_and_phase() {
        let mut stub = EmbodiedLoopStub::new(EmbodiedLoopSlots::new());
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::Sense);
        assert_eq!(err.slug(), "sense_unwired");
    }

    #[test]
    fn production_loop_wired_fence_false() {
        assert!(!PRODUCTION_LOOP_WIRED);
        assert!(!TENSOR_CBF_EVALUATED);
        assert!(!PRODUCTION_WIRED);
        assert!(!PHYSICS_GREEN_CLAIMED);
        assert!(!MASTER_RETICK_ELIGIBLE);
        assert!(!OP5_CLAIMED);
        assert!(!loop_stub_production_wired());
        assert!(!loop_stub_physics_green_claimed());
        assert!(!loop_stub_master_retick_eligible());
        assert!(!loop_stub_op5_claimed());
    }

    #[test]
    fn loop_stub_deepen_probe_honest_fences_hold() {
        let probe = loop_stub_deepen_probe();
        assert!(loop_stub_deepen_honest(&probe));
        assert_eq!(probe.cell_id, "W29-037-LOOP_STUB");
        assert_eq!(probe.stub_defect_id, "SK-08");
        assert_eq!(probe.posture_tag, "LEARNER_OPTIONAL");
        assert_eq!(probe.deepen_posture, LOOP_STUB_DEEPEN_POSTURE);
        assert_eq!(probe.admit_lane, "umst-admit-grok");
        assert_eq!(probe.admit_model, "cursor-grok-4.5-high");
        assert!(probe.tick_scaffold_landed);
        assert_eq!(probe.funnel_phase_count, 6);
        assert_eq!(probe.audit_wired_phase_count, 1);
        assert_eq!(probe.audit_unwired_phase_count, 5);
        assert_eq!(probe.scaffold_coverage_pct, 22);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green_claimed);
        assert!(!probe.master_retick_eligible);
        assert!(!probe.op5_claimed);
        assert!(!probe.gateway_command_composed);
        assert!(!probe.tensor_cbf_evaluated);
        assert!(probe.production_loop_deferred);
        assert_eq!(probe.gap_inventory, loop_stub_gap_inventory());
        assert_eq!(EmbodiedLoopStub::deepen_probe(), probe);
        assert!(validate_loop_stub_deepen_honesty().is_ok());
    }

    #[test]
    fn loop_stub_phase_fence_matrix_gate_only() {
        let matrix = loop_stub_phase_fence_matrix();
        assert_eq!(matrix.len(), FUNNEL_PHASE_COUNT);
        assert_eq!(matrix[0].phase, LoopTickPhase::Sense);
        assert!(!matrix[0].audit_wired);
        assert_eq!(matrix[1].phase, LoopTickPhase::Command);
        assert!(!matrix[1].audit_wired);
        assert_eq!(matrix[2].phase, LoopTickPhase::Gate);
        assert!(matrix[2].audit_wired);
        assert_eq!(matrix[3].phase, LoopTickPhase::Present);
        assert!(!matrix[3].audit_wired);
        assert_eq!(matrix[4].phase, LoopTickPhase::Actuate);
        assert!(!matrix[4].audit_wired);
        assert_eq!(matrix[5].phase, LoopTickPhase::LoopClose);
        assert!(!matrix[5].audit_wired);
        assert_eq!(loop_stub_audit_wired_phase_count(), 1);
        assert_eq!(loop_stub_audit_unwired_phase_count(), 5);
        assert_eq!(EmbodiedLoopStub::phase_fence_matrix(), matrix);
        assert_eq!(FUNNEL_TICK_PHASES.len(), 6);
        assert_eq!(FUNNEL_TICK_PHASES[0], LoopTickPhase::Sense);
        assert_eq!(FUNNEL_TICK_PHASES[5], LoopTickPhase::LoopClose);
        assert_eq!(LoopTickPhase::Gate.slug(), "gate");
        assert_eq!(LoopTickPhase::LoopClose.slug(), "loop_close");
    }

    #[test]
    fn loop_stub_census_probe_chains_tombstone_and_deepen() {
        let census = loop_stub_census_probe();
        assert_eq!(census.cell_id, LOOP_STUB_CELL_ID);
        assert_eq!(census.source_anchor, SOURCE_ANCHOR_PATH);
        assert_eq!(census.receipt_slug, RECEIPT_SLUG);
        assert_eq!(census.tombstone, loop_stub_tombstone_summary());
        assert_eq!(census.deepen, loop_stub_deepen_probe());
        assert_eq!(census.phase_matrix, loop_stub_phase_fence_matrix());
        assert_eq!(census.gap_inventory, loop_stub_gap_inventory());
        assert!(loop_stub_deepen_honest(&census.deepen));
        assert_eq!(EmbodiedLoopStub::census_probe(), census);
        assert!(census.deepen.honest_fence.contains("production_wired=false"));
        assert!(census.deepen.honest_fence.contains("physics_green=false"));
        assert!(census.deepen.honest_fence.contains("master_retick=false"));
        assert!(census.deepen.honest_fence.contains("op5=false"));
    }

    #[test]
    fn loop_stub_honest_fence_string_locked() {
        assert!(LOOP_STUB_HONEST_FENCE.contains("tick_scaffold_landed=true"));
        assert!(LOOP_STUB_HONEST_FENCE.contains("production_wired=false"));
        assert!(LOOP_STUB_HONEST_FENCE.contains("physics_green=false"));
        assert!(LOOP_STUB_HONEST_FENCE.contains("master_retick=false"));
        assert!(LOOP_STUB_HONEST_FENCE.contains("gateway_composed=false"));
        assert!(LOOP_STUB_HONEST_FENCE.contains("op5=false"));
        assert_eq!(LOOP_STUB_CELL_ID, "W29-037-LOOP_STUB");
        assert_eq!(LOOP_STUB_ADMIT_LANE, "umst-admit-grok");
        assert_eq!(LOOP_STUB_ADMIT_MODEL, "cursor-grok-4.5-high");
    }

    #[test]
    fn loop_stub_gap_inventory_five_unwired() {
        let gaps = loop_stub_gap_inventory();
        assert_eq!(gaps.audit_wired_phase_count, 1);
        assert_eq!(gaps.audit_unwired_phase_count, 5);
        assert_eq!(gaps.unwired_phase_slugs, ["sense", "command", "present", "actuate", "loop_close"]);
        assert!(!gaps.op5_claimed);
        assert!(!gaps.production_wired);
        assert!(!gaps.physics_green_claimed);
        assert!(!gaps.master_retick_eligible);
        assert_eq!(EmbodiedLoopStub::gap_inventory(), gaps);
    }

    #[test]
    fn sense_failed_rejects_at_sense_phase() {
        struct FailSense;
        impl FieldSenseClient for FailSense {
            fn sense(&mut self) -> Result<SenseObservation, FieldSenseError> {
                Err(FieldSenseError {
                    detail: "mock sense fail".into(),
                })
            }
        }
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(FailSense));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::Sense);
        assert_eq!(err.slug(), "sense_failed");
        assert!(matches!(err, LoopStubReject::SenseFailed { .. }));
    }

    #[test]
    fn actuate_failed_rejects_at_actuate_phase() {
        struct FailActuate;
        impl RobotExecutor for FailActuate {
            fn actuate(
                &mut self,
                _design: &ActuateDesign,
            ) -> Result<(), super::super::fragment_slots::ActuateError> {
                Err(super::super::fragment_slots::ActuateError {
                    detail: "mock actuate fail".into(),
                })
            }
        }
        let mut slots = EmbodiedLoopSlots::new();
        slots.field_sense = Some(Box::new(StubSense));
        slots.xr_present = Some(Box::new(StubPresent));
        slots.robot_actuate = Some(Box::new(FailActuate));
        let mut stub = EmbodiedLoopStub::new(slots);
        let err = stub.tick().unwrap_err();
        assert_eq!(err.failed_phase(), LoopTickPhase::Actuate);
        assert_eq!(err.slug(), "actuate_failed");
        assert!(matches!(err, LoopStubReject::ActuateFailed { .. }));
    }
}

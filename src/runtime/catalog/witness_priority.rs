// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Optional adaptive catalog **module** priority scheduler (telemetry / CI scaffolding).
//!
//! **Not on the inference hot path.** Bumps Lean `module` ordering hints from
//! `formal-witness` reject counts and bounded learning signals. Feeds
//! [`docs/ADAPTIVE_WITNESS_COVERAGE.md`](../../../docs/ADAPTIVE_WITNESS_COVERAGE.md).
//!
//! **TCB:** [`WitnessTcbAxiom::PhysicalSecondLaw`] only (`physicalSecondLaw` in `LandauerLaw.lean`);
//! no new Lean or Rust axioms.
//!
//! # Honest boundary (W29-110)
//!
//! Queue contracts land for adaptive-coverage CI / telemetry. Disabled by default on
//! [`crate::manifest::UmstManifest`]. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`,
//! not OP-5. Semantic-CBF rejects stay unwired here (HCOM-004 cold path).

use std::collections::BTreeMap;

use crate::runtime::catalog::traceability::{
    CATALOG_MODULE_WIRED, LANDAUER_CBF_CATALOG_ID, SEMANTIC_CBF_CATALOG_ID,
};

/// Lean module carrying the sole project axiom (`LandauerLaw.lean`).
pub const LANDAUER_LAW_LEAN_MODULE: &str = "LandauerLaw";

/// Allowlisted TCB axiom token (cartridge / profile parity).
pub const PHYSICAL_SECOND_LAW_AXIOM: &str = "physicalSecondLaw";

/// W29 deepen cell — adaptive witness priority honest fence bundle.
pub const W29_WITNESS_PRIORITY_DEEPEN_CELL: &str = "W29-110-WITNESS_PRIORITY";

/// Honest posture tag — telemetry / CI scheduler; hot-path wiring refused.
pub const WITNESS_PRIORITY_POSTURE_TAG: &str = "honest-adaptive-witness-priority-telemetry-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const WITNESS_PRIORITY_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by adaptive priority scaffolding alone.
pub const WITNESS_PRIORITY_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const WITNESS_PRIORITY_MASTER: bool = false;

/// OP-5 ceremony pin — not claimed by this module.
pub const WITNESS_PRIORITY_OP5: bool = false;

/// Whether reject/learning priority queue contracts are landed in this module.
pub const WITNESS_PRIORITY_QUEUE_LANDED: bool = true;

/// Whether the default manifest/runtime queue is inert (disabled) until opt-in.
pub const WITNESS_PRIORITY_DEFAULT_DISABLED: bool = true;

/// Whether semantic-CBF catalog rejects are wired into this priority queue (honest open).
pub const WITNESS_PRIORITY_SEMANTIC_CBF_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const WITNESS_PRIORITY_HONEST_FENCE: &str = concat!(
    "adaptive_queue_landed=true|default_disabled=true|tcb_physical_second_law_only=true|",
    "landauer_reject_bump_landed=true|learning_signal_clamp_landed=true|",
    "semantic_cbf_wired=false|hot_path_wired=false|",
    "production_wired=false|physics_green=false|master=false|op5=false"
);

/// Typed probe for adaptive witness-priority posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WitnessPriorityPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub queue_landed: bool,
    pub default_disabled: bool,
    pub semantic_cbf_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for adaptive witness priority.
#[must_use]
pub fn witness_priority_honest_posture_bundle() -> WitnessPriorityPostureProbe {
    WitnessPriorityPostureProbe {
        physics_green: WITNESS_PRIORITY_PHYSICS_GREEN,
        production_wired: WITNESS_PRIORITY_PRODUCTION_WIRED,
        master: WITNESS_PRIORITY_MASTER,
        op5: WITNESS_PRIORITY_OP5,
        queue_landed: WITNESS_PRIORITY_QUEUE_LANDED,
        default_disabled: WITNESS_PRIORITY_DEFAULT_DISABLED,
        semantic_cbf_wired: WITNESS_PRIORITY_SEMANTIC_CBF_WIRED,
        honest_fence: WITNESS_PRIORITY_HONEST_FENCE,
        posture_tag: WITNESS_PRIORITY_POSTURE_TAG,
        deepen_cell: W29_WITNESS_PRIORITY_DEEPEN_CELL,
    }
}

/// Adaptive priority scaffolding landed with production / master / OP-5 honestly open.
#[must_use]
pub fn witness_priority_posture_honest(probe: &WitnessPriorityPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.queue_landed
        && probe.default_disabled
        && !probe.semantic_cbf_wired
        && probe.deepen_cell == W29_WITNESS_PRIORITY_DEEPEN_CELL
        && probe.posture_tag == WITNESS_PRIORITY_POSTURE_TAG
        && probe.honest_fence.contains("adaptive_queue_landed=true")
        && probe.honest_fence.contains("default_disabled=true")
        && probe
            .honest_fence
            .contains("tcb_physical_second_law_only=true")
        && probe
            .honest_fence
            .contains("landauer_reject_bump_landed=true")
        && probe
            .honest_fence
            .contains("learning_signal_clamp_landed=true")
        && probe.honest_fence.contains("semantic_cbf_wired=false")
        && probe.honest_fence.contains("hot_path_wired=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// TCB axiom closure for adaptive witness scheduling — **one** variant only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WitnessTcbAxiom {
    /// `LandauerLaw.physicalSecondLaw` — operational Landauer/CBF envelope.
    PhysicalSecondLaw,
}

impl WitnessTcbAxiom {
    /// Allowlist mirrored by `formal_anchors` (`{NONE, physicalSecondLaw}`).
    pub const ALLOWED_TOKENS: [&'static str; 2] = ["NONE", PHYSICAL_SECOND_LAW_AXIOM];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalSecondLaw => PHYSICAL_SECOND_LAW_AXIOM,
        }
    }
}

/// Returns `true` when `token` is admissible on the adaptive witness path.
#[must_use]
pub fn tcb_axiom_token_allowed(token: &str) -> bool {
    WitnessTcbAxiom::ALLOWED_TOKENS.contains(&token)
}

/// Bounded epistemic / MI surrogate hint keyed by gate `catalog_id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WitnessLearningSignal {
    pub catalog_id: &'static str,
    /// Surrogate weight in `1..=32` (0 = no bump).
    pub weight: u8,
}

/// Priority queue over Lean catalog **modules** driven by formal-witness rejects + learning.
///
/// Disabled by default; enable via [`Self::for_adaptive_coverage`] or attach on
/// [`crate::manifest::UmstManifest`].
#[derive(Debug, Clone)]
pub struct WitnessPriorityQueue {
    enabled: bool,
    tcb_axiom: WitnessTcbAxiom,
    reject_counts: BTreeMap<&'static str, u32>,
    module_scores: BTreeMap<&'static str, u32>,
}

const REJECT_BUMP: u32 = 10;
const LEARNING_UNIT: u32 = 3;
const LANDAUER_AXIOM_MODULE_EXTRA: u32 = 5;
const LEARNING_WEIGHT_CAP: u8 = 32;

impl Default for WitnessPriorityQueue {
    fn default() -> Self {
        Self::disabled()
    }
}

impl WitnessPriorityQueue {
    /// Inert queue (manifest / runtime default).
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            tcb_axiom: WitnessTcbAxiom::PhysicalSecondLaw,
            reject_counts: BTreeMap::new(),
            module_scores: BTreeMap::new(),
        }
    }

    /// Enabled queue for adaptive coverage experiments and CI.
    #[must_use]
    pub fn for_adaptive_coverage() -> Self {
        Self {
            enabled: true,
            tcb_axiom: WitnessTcbAxiom::PhysicalSecondLaw,
            reject_counts: BTreeMap::new(),
            module_scores: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub fn tcb_axiom(&self) -> WitnessTcbAxiom {
        self.tcb_axiom
    }

    #[must_use]
    pub fn reject_count(&self, catalog_id: &str) -> u32 {
        self.reject_counts.get(catalog_id).copied().unwrap_or(0)
    }

    /// Sum of all recorded reject counts (telemetry aggregate).
    #[must_use]
    pub fn total_rejects(&self) -> u32 {
        self.reject_counts.values().copied().sum()
    }

    /// Number of Lean modules currently carrying a nonzero adaptive score.
    #[must_use]
    pub fn scored_module_count(&self) -> usize {
        self.module_scores.values().filter(|&&s| s > 0).count()
    }

    /// Stable reject-bump constants for CI / docs parity.
    #[must_use]
    pub const fn reject_bump_unit() -> u32 {
        REJECT_BUMP
    }

    /// Stable learning-unit multiplier for CI / docs parity.
    #[must_use]
    pub const fn learning_unit() -> u32 {
        LEARNING_UNIT
    }

    /// Extra bump applied only to [`LANDAUER_LAW_LEAN_MODULE`] on Landauer CBF rejects.
    #[must_use]
    pub const fn landauer_axiom_module_extra() -> u32 {
        LANDAUER_AXIOM_MODULE_EXTRA
    }

    /// Clamp applied to [`WitnessLearningSignal::weight`].
    #[must_use]
    pub const fn learning_weight_cap() -> u8 {
        LEARNING_WEIGHT_CAP
    }

    /// Lean modules mapped to `catalog_id` via [`CATALOG_MODULE_WIRED`].
    #[must_use]
    pub fn lean_modules_for_catalog_id(catalog_id: &str) -> Vec<&'static str> {
        CATALOG_MODULE_WIRED
            .iter()
            .filter(|(_, ids)| ids.contains(&catalog_id))
            .map(|(module, _)| *module)
            .collect()
    }

    /// Clear reject counts and module scores (keeps enabled / TCB flags).
    pub fn clear_scores(&mut self) {
        self.reject_counts.clear();
        self.module_scores.clear();
    }

    fn bump_modules(&mut self, catalog_id: &'static str, score: u32) {
        for module in Self::lean_modules_for_catalog_id(catalog_id) {
            let extra = if module == LANDAUER_LAW_LEAN_MODULE
                && catalog_id == LANDAUER_CBF_CATALOG_ID
                && self.tcb_axiom == WitnessTcbAxiom::PhysicalSecondLaw
            {
                LANDAUER_AXIOM_MODULE_EXTRA
            } else {
                0
            };
            *self.module_scores.entry(module).or_insert(0) += score + extra;
        }
    }

    /// Record one `formal-witness` (or gateway) reject by stable `catalog_id`.
    pub fn record_reject(&mut self, catalog_id: &'static str) {
        if !self.enabled {
            return;
        }
        *self.reject_counts.entry(catalog_id).or_insert(0) += 1;
        self.bump_modules(catalog_id, REJECT_BUMP);
    }

    /// Apply bounded learning signals (post-CBF MI / epistemic surrogates).
    pub fn apply_learning_signals(&mut self, signals: &[WitnessLearningSignal]) {
        if !self.enabled {
            return;
        }
        for signal in signals {
            if signal.weight == 0 {
                continue;
            }
            let w = u32::from(signal.weight.min(LEARNING_WEIGHT_CAP)) * LEARNING_UNIT;
            self.bump_modules(signal.catalog_id, w);
        }
    }

    /// Lean modules ordered by descending adaptive priority (stable name tie-break).
    #[must_use]
    pub fn ordered_modules(&self) -> Vec<(&'static str, u32)> {
        let mut out: Vec<_> = self
            .module_scores
            .iter()
            .map(|(&module, &score)| (module, score))
            .collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        out
    }

    #[must_use]
    pub fn priority_of_module(&self, lean_module: &str) -> u32 {
        self.module_scores.get(lean_module).copied().unwrap_or(0)
    }

    /// Snapshot of `(module, score)` pairs in ascending module-name order (stable export).
    #[must_use]
    pub fn score_snapshot(&self) -> Vec<(&'static str, u32)> {
        self.module_scores
            .iter()
            .map(|(&module, &score)| (module, score))
            .collect()
    }
}

#[cfg(feature = "formal-witness")]
impl WitnessPriorityQueue {
    /// Record a structured [`crate::ai::formal::FormalReject`] from the witness ladder.
    pub fn record_formal_reject(&mut self, reject: &crate::ai::formal::FormalReject) {
        self.record_reject(reject.catalog_id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;

    #[test]
    fn tcb_tokens_physical_second_law_only() {
        assert!(tcb_axiom_token_allowed("NONE"));
        assert!(tcb_axiom_token_allowed(PHYSICAL_SECOND_LAW_AXIOM));
        assert!(!tcb_axiom_token_allowed("extraAxiom"));
        assert_eq!(
            WitnessPriorityQueue::for_adaptive_coverage().tcb_axiom(),
            WitnessTcbAxiom::PhysicalSecondLaw
        );
        assert_eq!(
            WitnessTcbAxiom::PhysicalSecondLaw.as_str(),
            PHYSICAL_SECOND_LAW_AXIOM
        );
    }

    #[test]
    fn landauer_rejects_bump_landauer_law_module_first() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        let ordered = q.ordered_modules();
        assert!(
            !ordered.is_empty(),
            "LandauerLaw must receive score from landauer_cbf rejects"
        );
        assert_eq!(ordered[0].0, LANDAUER_LAW_LEAN_MODULE);
        assert!(
            q.priority_of_module(LANDAUER_LAW_LEAN_MODULE) > q.priority_of_module("DoubleSlit"),
            "second-law TCB module should outrank unrelated modules"
        );
        assert_eq!(q.reject_count(LANDAUER_CBF_CATALOG_ID), 2);
        assert_eq!(q.total_rejects(), 2);
        // Two rejects → 2 * (REJECT_BUMP + LANDAUER_AXIOM_MODULE_EXTRA) on LandauerLaw.
        assert_eq!(
            q.priority_of_module(LANDAUER_LAW_LEAN_MODULE),
            2 * (REJECT_BUMP + LANDAUER_AXIOM_MODULE_EXTRA)
        );
    }

    #[test]
    fn learning_signals_add_bounded_priority() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.apply_learning_signals(&[WitnessLearningSignal {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            weight: 4,
        }]);
        assert!(q.priority_of_module(LANDAUER_LAW_LEAN_MODULE) > 0);
        assert_eq!(
            q.priority_of_module(LANDAUER_LAW_LEAN_MODULE),
            4 * LEARNING_UNIT + LANDAUER_AXIOM_MODULE_EXTRA
        );
    }

    #[test]
    fn learning_weight_clamped_at_thirty_two() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.apply_learning_signals(&[WitnessLearningSignal {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            weight: 255,
        }]);
        let expected = u32::from(LEARNING_WEIGHT_CAP) * LEARNING_UNIT + LANDAUER_AXIOM_MODULE_EXTRA;
        assert_eq!(q.priority_of_module(LANDAUER_LAW_LEAN_MODULE), expected);
        assert_eq!(WitnessPriorityQueue::learning_weight_cap(), 32);
    }

    #[test]
    fn zero_weight_learning_is_noop() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.apply_learning_signals(&[WitnessLearningSignal {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            weight: 0,
        }]);
        assert!(q.ordered_modules().is_empty());
        assert_eq!(q.scored_module_count(), 0);
    }

    #[test]
    fn disabled_queue_is_noop() {
        let mut q = WitnessPriorityQueue::disabled();
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        q.apply_learning_signals(&[WitnessLearningSignal {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            weight: 8,
        }]);
        assert!(!q.is_enabled());
        assert!(q.ordered_modules().is_empty());
        assert_eq!(q.total_rejects(), 0);
        assert_eq!(WitnessPriorityQueue::default().is_enabled(), false);
    }

    #[test]
    fn clear_scores_resets_telemetry_keeps_enabled() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        assert!(q.scored_module_count() > 0);
        q.clear_scores();
        assert!(q.is_enabled());
        assert_eq!(q.total_rejects(), 0);
        assert!(q.ordered_modules().is_empty());
        assert!(q.score_snapshot().is_empty());
    }

    #[test]
    fn ordered_modules_stable_name_tiebreak() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        // cd_transition maps to several modules at equal REJECT_BUMP (no Landauer extra).
        q.record_reject(CD_TRANSITION_CATALOG_ID);
        let ordered = q.ordered_modules();
        assert!(!ordered.is_empty());
        let scores: Vec<u32> = ordered.iter().map(|(_, s)| *s).collect();
        assert!(scores.windows(2).all(|w| w[0] >= w[1]));
        let same_score_names: Vec<&str> = ordered
            .iter()
            .filter(|(_, s)| *s == REJECT_BUMP)
            .map(|(n, _)| *n)
            .collect();
        let mut sorted = same_score_names.clone();
        sorted.sort_unstable();
        assert_eq!(
            same_score_names, sorted,
            "equal scores must sort by ascending module name; got {ordered:?}"
        );
    }

    #[test]
    fn semantic_cbf_catalog_unwired_in_priority_map() {
        assert!(!WITNESS_PRIORITY_SEMANTIC_CBF_WIRED);
        let modules = WitnessPriorityQueue::lean_modules_for_catalog_id(SEMANTIC_CBF_CATALOG_ID);
        assert!(
            modules.is_empty(),
            "semantic_cbf must stay out of CATALOG_MODULE_WIRED priority bumps"
        );
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.record_reject(SEMANTIC_CBF_CATALOG_ID);
        assert_eq!(q.reject_count(SEMANTIC_CBF_CATALOG_ID), 1);
        assert!(
            q.ordered_modules().is_empty(),
            "unwired semantic_cbf rejects must not invent module scores"
        );
    }

    #[test]
    fn landauer_wiring_includes_tcb_module() {
        let modules = WitnessPriorityQueue::lean_modules_for_catalog_id(LANDAUER_CBF_CATALOG_ID);
        assert!(
            modules.contains(&LANDAUER_LAW_LEAN_MODULE),
            "LandauerLaw must remain in landauer_cbf wired set"
        );
        assert_eq!(WitnessPriorityQueue::reject_bump_unit(), 10);
        assert_eq!(WitnessPriorityQueue::learning_unit(), 3);
        assert_eq!(WitnessPriorityQueue::landauer_axiom_module_extra(), 5);
    }

    #[test]
    fn witness_priority_honest_fence_no_green_invent() {
        let probe = witness_priority_honest_posture_bundle();
        assert!(witness_priority_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(probe.queue_landed);
        assert!(probe.default_disabled);
        assert!(!probe.semantic_cbf_wired);
        assert_eq!(probe.deepen_cell, W29_WITNESS_PRIORITY_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, WITNESS_PRIORITY_POSTURE_TAG);
        assert!(!probe.honest_fence.contains("production_wired=true"));
        assert!(!probe.honest_fence.contains("physics_green=true"));
        assert!(!probe.honest_fence.contains("master=true"));
        assert!(!probe.honest_fence.contains("op5=true"));
    }

    #[test]
    fn posture_rejects_tampered_fence() {
        let mut bad = witness_priority_honest_posture_bundle();
        bad.physics_green = true;
        assert!(!witness_priority_posture_honest(&bad));
        bad = witness_priority_honest_posture_bundle();
        bad.production_wired = true;
        assert!(!witness_priority_posture_honest(&bad));
        bad = witness_priority_honest_posture_bundle();
        bad.master = true;
        assert!(!witness_priority_posture_honest(&bad));
        bad = witness_priority_honest_posture_bundle();
        bad.op5 = true;
        assert!(!witness_priority_posture_honest(&bad));
        bad = witness_priority_honest_posture_bundle();
        bad.semantic_cbf_wired = true;
        assert!(!witness_priority_posture_honest(&bad));
        bad = witness_priority_honest_posture_bundle();
        bad.honest_fence = "production_wired=true|physics_green=true";
        assert!(!witness_priority_posture_honest(&bad));
    }
}

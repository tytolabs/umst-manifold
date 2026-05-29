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

use std::collections::BTreeMap;

use crate::runtime::catalog::traceability::{CATALOG_MODULE_WIRED, LANDAUER_CBF_CATALOG_ID};

/// Lean module carrying the sole project axiom (`LandauerLaw.lean`).
pub const LANDAUER_LAW_LEAN_MODULE: &str = "LandauerLaw";

/// Allowlisted TCB axiom token (cartridge / profile parity).
pub const PHYSICAL_SECOND_LAW_AXIOM: &str = "physicalSecondLaw";

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

    fn lean_modules_for_catalog_id(catalog_id: &str) -> Vec<&'static str> {
        CATALOG_MODULE_WIRED
            .iter()
            .filter(|(_, ids)| ids.contains(&catalog_id))
            .map(|(module, _)| *module)
            .collect()
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
            let w = u32::from(signal.weight.min(32)) * LEARNING_UNIT;
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

    #[test]
    fn tcb_tokens_physical_second_law_only() {
        assert!(tcb_axiom_token_allowed("NONE"));
        assert!(tcb_axiom_token_allowed(PHYSICAL_SECOND_LAW_AXIOM));
        assert!(!tcb_axiom_token_allowed("extraAxiom"));
        assert_eq!(
            WitnessPriorityQueue::for_adaptive_coverage().tcb_axiom(),
            WitnessTcbAxiom::PhysicalSecondLaw
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
    }

    #[test]
    fn learning_signals_add_bounded_priority() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.apply_learning_signals(&[WitnessLearningSignal {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            weight: 4,
        }]);
        assert!(q.priority_of_module(LANDAUER_LAW_LEAN_MODULE) > 0);
    }

    #[test]
    fn disabled_queue_is_noop() {
        let mut q = WitnessPriorityQueue::disabled();
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        assert!(q.ordered_modules().is_empty());
    }
}

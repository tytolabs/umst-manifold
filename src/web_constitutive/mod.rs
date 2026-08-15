// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `WebConstitutiveModel` — informational domain cartridge (build-spec §A8 · WEB-005).
//!
//! Parallel to concrete [`umst_cartridge_concrete::evaluate_material_conjuncts`]: extracts web
//! constitutive quantities from the 64D `WebStateTensor` layout and routes them through Core
//! `gate<R>`. Host informational gate lives in `umst-web::response`; this module is the manifold
//! tensor-engine cartridge surface.

pub mod semantic_residual;

use std::ops::Range;

pub use semantic_residual::{
    evaluate_semantic_conjuncts, residual_from_row, residual_from_web_tensor,
    semantic_transition_witness_from_tensors, web_semantic_lane_overlap_valid,
    WebSemanticGateOutcome, WebSemanticResidual, WebSemanticTransitionWitness,
    DEFAULT_SEMANTIC_DEFECT_TOLERANCE, SEMANTIC_RESIDUAL_HOOK_V1,
};

use umst_gate::{
    core_gate, gate, AdmissibilityResponse, ConjunctVerdict, CoreGateOutcome, GateRejectReason,
};

/// Default complexity weight λ in `𝒟_web_int`.
pub const DEFAULT_COMPLEXITY_WEIGHT: f64 = 1.0;

/// Default Landauer rendering weight μ in `𝒟_web_int`.
pub const DEFAULT_LANDAUER_WEIGHT: f64 = 1.0;

/// Intent-fidelity slack ε_int for near-neutral informational transitions.
pub const DEFAULT_INT_TOLERANCE: f64 = 1e-9;

/// 64D web state tensor slice layout (WEB-004 · blueprint §3).
pub mod slice_layout {
    use super::Range;

    /// Total tensor dimension.
    pub const DIM: usize = 64;
    /// Intent slice [0, 16).
    pub const INTENT: Range<usize> = 0..16;
    /// Structure slice [16, 32).
    pub const STRUCTURE: Range<usize> = 16..32;
    /// Semantics slice [32, 48).
    pub const SEMANTICS: Range<usize> = 32..48;
    /// Presentation slice [48, 56).
    pub const PRESENTATION: Range<usize> = 48..56;
    /// Behavior + constraints + UCRS slice [56, 64).
    pub const BEHAVIOR_UCRS: Range<usize> = 56..64;

    /// Complexity-bearing slices (structure + semantics + presentation).
    pub const COMPLEXITY_SLICES: [Range<usize>; 3] = [STRUCTURE, SEMANTICS, PRESENTATION];
}

/// Scalar legs extracted from a web state snapshot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebConstitutiveQuantities {
    /// ΔIntentFidelity leg (benefit — maps to Core `dissipation`).
    pub intent_fidelity: f64,
    /// ΔComplexityCost leg (cost — weighted in `power_input`).
    pub complexity_cost: f64,
    /// Landauer rendering cost leg (cost — weighted in `power_input`).
    pub landauer_rendering: f64,
    /// Complexity weight λ.
    pub complexity_weight: f64,
    /// Landauer weight μ.
    pub landauer_weight: f64,
    /// Optional monotone a11y hook ∈ [0, 1].
    pub accessibility_coverage: Option<f64>,
    /// Optional monotone perf hook ∈ [0, 1].
    pub perf_budget_consumed: Option<f64>,
}

impl WebConstitutiveQuantities {
    /// Balanced fixture: `𝒟_web_int = 0` at declared legs.
    #[must_use]
    pub const fn balanced() -> Self {
        Self {
            intent_fidelity: 1.0,
            complexity_cost: 0.5,
            landauer_rendering: 0.5,
            complexity_weight: DEFAULT_COMPLEXITY_WEIGHT,
            landauer_weight: DEFAULT_LANDAUER_WEIGHT,
            accessibility_coverage: None,
            perf_budget_consumed: None,
        }
    }

    /// Under-budget fixture for gate rejection tests.
    #[must_use]
    pub const fn under_budget() -> Self {
        Self {
            intent_fidelity: 0.25,
            complexity_cost: 0.5,
            landauer_rendering: 0.5,
            complexity_weight: DEFAULT_COMPLEXITY_WEIGHT,
            landauer_weight: DEFAULT_LANDAUER_WEIGHT,
            accessibility_coverage: None,
            perf_budget_consumed: None,
        }
    }

    /// `𝒟_web_int = ΔIntentFidelity − λ·ΔComplexityCost − μ·LandauerRenderingCost`.
    #[must_use]
    pub fn web_int_dissipation(self) -> f64 {
        self.intent_fidelity
            - self.complexity_weight * self.complexity_cost
            - self.landauer_weight * self.landauer_rendering
    }

    /// Whether scalar cost legs are physically valid (non-negative).
    #[must_use]
    pub fn cost_legs_valid(self) -> bool {
        self.complexity_cost >= 0.0 && self.landauer_rendering >= 0.0
    }
}

/// Witness for web transition conjuncts (old → new tensor snapshots).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebTransitionWitness {
    /// Prior web state quantities.
    pub old: WebConstitutiveQuantities,
    /// Proposed web state quantities.
    pub new: WebConstitutiveQuantities,
}

impl WebTransitionWitness {
    /// Whether monotone a11y/perf hooks respect non-regression when both sides declare them.
    #[must_use]
    pub fn monotone_respected(self) -> bool {
        match (
            self.old.accessibility_coverage,
            self.new.accessibility_coverage,
        ) {
            (Some(prev), Some(next)) if next < prev => return false,
            _ => {}
        }
        match (self.old.perf_budget_consumed, self.new.perf_budget_consumed) {
            (Some(prev), Some(next)) if next < prev => return false,
            _ => {}
        }
        true
    }
}

/// Gate-checkable informational constitutive response (Core `gate<R>` input).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebConstitutiveResponse {
    /// ΔIntentFidelity leg.
    pub intent_fidelity: f64,
    /// ΔComplexityCost leg.
    pub complexity_cost: f64,
    /// Landauer rendering cost leg.
    pub landauer_rendering: f64,
    /// Complexity weight λ.
    pub complexity_weight: f64,
    /// Landauer weight μ.
    pub landauer_weight: f64,
}

impl AdmissibilityResponse for WebConstitutiveResponse {
    fn dissipation(&self) -> f64 {
        self.intent_fidelity
    }

    fn power_input(&self) -> f64 {
        self.complexity_weight * self.complexity_cost
            + self.landauer_weight * self.landauer_rendering
    }
}

impl From<WebConstitutiveQuantities> for WebConstitutiveResponse {
    fn from(q: WebConstitutiveQuantities) -> Self {
        Self {
            intent_fidelity: q.intent_fidelity,
            complexity_cost: q.complexity_cost,
            landauer_rendering: q.landauer_rendering,
            complexity_weight: q.complexity_weight,
            landauer_weight: q.landauer_weight,
        }
    }
}

/// Outcome of web domain conjunct evaluation (orthogonal to Core).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebGateOutcome {
    /// Primary discriminant — cost-leg validity ∧ monotone hooks.
    pub verdict: ConjunctVerdict,
    /// Whether cost legs are non-negative.
    pub cost_legs_valid: bool,
    /// Whether monotone a11y/perf hooks respected (or omitted).
    pub monotone_respected: bool,
}

impl WebGateOutcome {
    /// Whether the web domain conjunct cluster accepted the transition.
    #[must_use]
    pub fn is_accepted(self) -> bool {
        self.verdict.is_accepted()
    }
}

/// Informational domain constitutive model — extracts quantities from 64D web tensors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WebConstitutiveModel {
    /// Complexity weight λ.
    pub complexity_weight: f64,
    /// Landauer weight μ.
    pub landauer_weight: f64,
    /// Bits resolved per unit L2 norm of presentation slice (Landauer proxy).
    pub landauer_bits_per_render_unit: f64,
}

impl Default for WebConstitutiveModel {
    fn default() -> Self {
        Self::cartridge()
    }
}

impl WebConstitutiveModel {
    /// Default A8 cartridge parameters.
    #[must_use]
    pub const fn cartridge() -> Self {
        Self {
            complexity_weight: DEFAULT_COMPLEXITY_WEIGHT,
            landauer_weight: DEFAULT_LANDAUER_WEIGHT,
            landauer_bits_per_render_unit: 1.0,
        }
    }

    /// Extract absolute quantities from a 64D tensor snapshot.
    #[must_use]
    pub fn quantities_from_tensor(
        &self,
        tensor: &[f64; slice_layout::DIM],
    ) -> WebConstitutiveQuantities {
        let intent_fidelity = slice_l2_norm(&tensor[slice_layout::INTENT.clone()]);
        let complexity_cost = complexity_entropy(tensor);
        let landauer_rendering =
            self.landauer_from_presentation(&tensor[slice_layout::PRESENTATION.clone()]);
        let (accessibility_coverage, perf_budget_consumed) = monotone_hooks_from_behavior(tensor);

        WebConstitutiveQuantities {
            intent_fidelity,
            complexity_cost,
            landauer_rendering,
            complexity_weight: self.complexity_weight,
            landauer_weight: self.landauer_weight,
            accessibility_coverage,
            perf_budget_consumed,
        }
    }

    /// Build a transition witness from old/new tensor snapshots.
    #[must_use]
    pub fn transition_witness_from_tensors(
        &self,
        old: &[f64; slice_layout::DIM],
        new: &[f64; slice_layout::DIM],
    ) -> WebTransitionWitness {
        let old_abs = self.quantities_from_tensor(old);
        let new_abs = self.quantities_from_tensor(new);

        let intent_fidelity = cosine_similarity(
            &old[slice_layout::INTENT.clone()],
            &new[slice_layout::INTENT.clone()],
        );
        let complexity_cost = (new_abs.complexity_cost - old_abs.complexity_cost).max(0.0);
        let landauer_rendering = new_abs.landauer_rendering;

        WebTransitionWitness {
            old: old_abs,
            new: WebConstitutiveQuantities {
                intent_fidelity,
                complexity_cost,
                landauer_rendering,
                complexity_weight: self.complexity_weight,
                landauer_weight: self.landauer_weight,
                accessibility_coverage: new_abs.accessibility_coverage,
                perf_budget_consumed: new_abs.perf_budget_consumed,
            },
        }
    }

    /// Evaluate gate-checkable response from a transition witness (uses `new` legs).
    #[must_use]
    pub fn evaluate_response(&self, witness: &WebTransitionWitness) -> WebConstitutiveResponse {
        witness.new.into()
    }

    fn landauer_from_presentation(&self, presentation: &[f64]) -> f64 {
        slice_l2_norm(presentation) * self.landauer_bits_per_render_unit
    }
}

/// Web domain conjunct evaluation — non-negative cost legs ∧ monotone hooks.
///
/// SSOT parallel to [`umst_cartridge_concrete::evaluate_material_conjuncts`].
#[must_use]
pub fn evaluate_web_conjuncts(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
) -> WebGateOutcome {
    let cost_legs_valid = response.complexity_cost >= 0.0 && response.landauer_rendering >= 0.0;
    let monotone_respected = witness.monotone_respected();

    let verdict = if !cost_legs_valid {
        ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
    } else if !monotone_respected {
        ConjunctVerdict::Rejected(GateRejectReason::MalformedInput)
    } else {
        ConjunctVerdict::Accepted
    };

    WebGateOutcome {
        verdict,
        cost_legs_valid,
        monotone_respected,
    }
}

/// Informational domain gate — web conjuncts ∧ Core `gate<R>`.
///
/// Informational transitions are mass-neutral: `mass_conserved = true`.
#[must_use]
pub fn web_gate(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
    tolerance: f64,
) -> (CoreGateOutcome, WebGateOutcome, ConjunctVerdict) {
    let web = evaluate_web_conjuncts(response, witness);
    let core = if web.is_accepted() {
        gate(response, true, tolerance)
    } else {
        core_gate(response, true, tolerance)
    };
    let composed = ConjunctVerdict::compose(web.verdict, core.conjunct_verdict());
    (core, web, composed)
}

/// Convenience: evaluate full web transition from model + witness.
#[must_use]
pub fn web_transition_gate_outcome(
    model: &WebConstitutiveModel,
    witness: &WebTransitionWitness,
    tolerance: f64,
) -> (
    WebConstitutiveResponse,
    CoreGateOutcome,
    WebGateOutcome,
    ConjunctVerdict,
) {
    let response = model.evaluate_response(witness);
    let (core, web, composed) = web_gate(&response, witness, tolerance);
    (response, core, web, composed)
}

/// Informational domain gate with HCOM-006 semantic residual conjuncts.
///
/// Composes web domain ∧ semantic residual ∧ Core `gate<R>`.
#[must_use]
pub fn web_gate_with_semantic(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
    semantic_witness: &WebSemanticTransitionWitness,
    tolerance: f64,
    semantic_defect_tolerance: f64,
) -> (
    CoreGateOutcome,
    WebGateOutcome,
    WebSemanticGateOutcome,
    ConjunctVerdict,
) {
    let web = evaluate_web_conjuncts(response, witness);
    let semantic = evaluate_semantic_conjuncts(semantic_witness, semantic_defect_tolerance);
    let core = if web.is_accepted() && semantic.is_accepted() {
        gate(response, true, tolerance)
    } else {
        core_gate(response, true, tolerance)
    };
    let composed = ConjunctVerdict::compose(
        ConjunctVerdict::compose(web.verdict, semantic.verdict),
        core.conjunct_verdict(),
    );
    (core, web, semantic, composed)
}

/// Full web transition from tensors including semantic lane residuals.
#[must_use]
pub fn web_transition_gate_outcome_with_semantic(
    model: &WebConstitutiveModel,
    old: &[f64; slice_layout::DIM],
    new: &[f64; slice_layout::DIM],
    tolerance: f64,
    semantic_defect_tolerance: f64,
) -> (
    WebTransitionWitness,
    WebSemanticTransitionWitness,
    WebConstitutiveResponse,
    CoreGateOutcome,
    WebGateOutcome,
    WebSemanticGateOutcome,
    ConjunctVerdict,
) {
    let witness = model.transition_witness_from_tensors(old, new);
    let semantic_witness = semantic_transition_witness_from_tensors(old, new);
    let response = model.evaluate_response(&witness);
    let (core, web, semantic, composed) = web_gate_with_semantic(
        &response,
        &witness,
        &semantic_witness,
        tolerance,
        semantic_defect_tolerance,
    );
    (
        witness,
        semantic_witness,
        response,
        core,
        web,
        semantic,
        composed,
    )
}

fn slice_l2_norm(slice: &[f64]) -> f64 {
    slice.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na = slice_l2_norm(a);
    let nb = slice_l2_norm(b);
    if na <= f64::EPSILON || nb <= f64::EPSILON {
        return 0.0;
    }
    (dot / (na * nb)).clamp(-1.0, 1.0)
}

fn complexity_entropy(tensor: &[f64]) -> f64 {
    let mut values = Vec::new();
    for range in slice_layout::COMPLEXITY_SLICES {
        values.extend_from_slice(&tensor[range]);
    }
    let total: f64 = values.iter().map(|x| x.abs()).sum();
    if total <= f64::EPSILON {
        return 0.0;
    }
    values
        .iter()
        .map(|&x| {
            let p = x.abs() / total;
            if p <= f64::EPSILON {
                0.0
            } else {
                -p * p.ln()
            }
        })
        .sum()
}

fn monotone_hooks_from_behavior(tensor: &[f64]) -> (Option<f64>, Option<f64>) {
    let behavior = &tensor[slice_layout::BEHAVIOR_UCRS.clone()];
    if behavior.len() < 2 {
        return (None, None);
    }
    let a11y = behavior[0];
    let perf = behavior[1];
    let a11y_ok = (0.0..=1.0).contains(&a11y);
    let perf_ok = (0.0..=1.0).contains(&perf);
    (a11y_ok.then_some(a11y), perf_ok.then_some(perf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use umst_gate::ConjunctVerdict;

    fn unit_intent_tensor() -> [f64; slice_layout::DIM] {
        let mut t = [0.0; slice_layout::DIM];
        t[0] = 1.0;
        t
    }

    fn heavy_presentation_tensor() -> [f64; slice_layout::DIM] {
        let mut t = unit_intent_tensor();
        for i in slice_layout::STRUCTURE.clone() {
            t[i] = 0.3;
        }
        for i in slice_layout::PRESENTATION.clone() {
            t[i] = 0.8;
        }
        t[slice_layout::BEHAVIOR_UCRS.start] = 1.0;
        t[slice_layout::BEHAVIOR_UCRS.start + 1] = 0.2;
        t
    }

    #[test]
    fn quantities_from_tensor_neutral_intent() {
        let model = WebConstitutiveModel::cartridge();
        let q = model.quantities_from_tensor(&unit_intent_tensor());
        assert!((q.intent_fidelity - 1.0).abs() < f64::EPSILON);
        assert!(q.complexity_cost >= 0.0);
        assert!(q.landauer_rendering >= 0.0);
    }

    #[test]
    fn balanced_quantities_pass_web_gate() {
        let witness = WebTransitionWitness {
            old: WebConstitutiveQuantities::balanced(),
            new: WebConstitutiveQuantities::balanced(),
        };
        let response = WebConstitutiveResponse::from(witness.new);
        let (_, web, composed) = web_gate(&response, &witness, DEFAULT_INT_TOLERANCE);
        assert!(web.is_accepted());
        assert_eq!(composed, ConjunctVerdict::Accepted);
    }

    #[test]
    fn under_budget_rejects_core_gate() {
        let witness = WebTransitionWitness {
            old: WebConstitutiveQuantities::balanced(),
            new: WebConstitutiveQuantities::under_budget(),
        };
        let response = WebConstitutiveResponse::from(witness.new);
        let (_, _, composed) = web_gate(&response, &witness, DEFAULT_INT_TOLERANCE);
        assert_eq!(
            composed,
            ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
        );
    }

    #[test]
    fn negative_complexity_rejects_web_conjunct() {
        let witness = WebTransitionWitness {
            old: WebConstitutiveQuantities::balanced(),
            new: WebConstitutiveQuantities {
                complexity_cost: -0.1,
                ..WebConstitutiveQuantities::balanced()
            },
        };
        let response = WebConstitutiveResponse::from(witness.new);
        let web = evaluate_web_conjuncts(&response, &witness);
        assert!(!web.is_accepted());
        assert!(!web.cost_legs_valid);
    }

    #[test]
    fn monotone_regression_rejects_web_conjunct() {
        let witness = WebTransitionWitness {
            old: WebConstitutiveQuantities {
                accessibility_coverage: Some(1.0),
                perf_budget_consumed: Some(0.1),
                ..WebConstitutiveQuantities::balanced()
            },
            new: WebConstitutiveQuantities {
                accessibility_coverage: Some(0.5),
                perf_budget_consumed: Some(0.1),
                ..WebConstitutiveQuantities::balanced()
            },
        };
        let response = WebConstitutiveResponse::from(witness.new);
        let web = evaluate_web_conjuncts(&response, &witness);
        assert!(!web.is_accepted());
        assert!(!web.monotone_respected);
    }

    #[test]
    fn transition_from_tensors_increases_complexity_on_heavy_presentation() {
        let model = WebConstitutiveModel::cartridge();
        let old = unit_intent_tensor();
        let new = heavy_presentation_tensor();
        let witness = model.transition_witness_from_tensors(&old, &new);
        assert!(witness.new.complexity_cost > 0.0);
        assert!(witness.new.landauer_rendering > 0.0);
    }

    #[test]
    fn web_int_dissipation_matches_core_net() {
        let q = WebConstitutiveQuantities::balanced();
        let response = WebConstitutiveResponse::from(q);
        assert!((q.web_int_dissipation() - response.net_dissipation()).abs() < f64::EPSILON);
    }
}

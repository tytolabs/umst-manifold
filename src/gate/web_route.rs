// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Phase A8 — canonical web/informational gate routing (build-spec §A8 · WEB-005).
//!
//! Routes web constitutive quantities through Core `gate<R>` + web domain conjuncts.
//! Host `gate_informational` in `umst-web::response` delegates scalar legs here via
//! [`crate::web_constitutive`].

use umst_gate::{ConjunctVerdict, CoreGateOutcome};

use crate::web_constitutive::{
    evaluate_semantic_conjuncts, evaluate_web_conjuncts, web_gate, web_transition_gate_outcome,
    web_transition_gate_outcome_with_semantic, WebConstitutiveModel, WebConstitutiveQuantities,
    WebConstitutiveResponse, WebGateOutcome, WebSemanticGateOutcome, WebSemanticTransitionWitness,
    WebTransitionWitness, DEFAULT_INT_TOLERANCE,
};

/// Canonical web domain conjunct outcome (informational cartridge).
#[must_use]
pub fn canonical_web_gate_outcome(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
) -> WebGateOutcome {
    evaluate_web_conjuncts(response, witness)
}

/// Canonical informational transition — Core ∧ web conjuncts at default tolerance.
#[must_use]
pub fn canonical_web_transition_admissible(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
) -> bool {
    let (_, _, composed) = web_gate(response, witness, DEFAULT_INT_TOLERANCE);
    composed.is_accepted()
}

/// Canonical informational transition with explicit tolerance.
#[must_use]
pub fn canonical_web_transition_outcome(
    response: &WebConstitutiveResponse,
    witness: &WebTransitionWitness,
    tolerance: f64,
) -> (CoreGateOutcome, WebGateOutcome, ConjunctVerdict) {
    web_gate(response, witness, tolerance)
}

/// Evaluate full web transition from 64D tensors via [`WebConstitutiveModel`].
#[must_use]
pub fn canonical_web_transition_from_tensors(
    model: &WebConstitutiveModel,
    old: &[f64; 64],
    new: &[f64; 64],
    tolerance: f64,
) -> (
    WebTransitionWitness,
    WebConstitutiveResponse,
    CoreGateOutcome,
    WebGateOutcome,
    ConjunctVerdict,
) {
    let witness = model.transition_witness_from_tensors(old, new);
    let (response, core, web, composed) = web_transition_gate_outcome(model, &witness, tolerance);
    (witness, response, core, web, composed)
}

/// Bridge scalar legs from `umst-web::WebStateTensor` into manifold web gate.
///
/// Integration point: `umst-web` host calls this with projected quantities.
#[must_use]
pub fn canonical_web_gate_from_quantities(
    new: WebConstitutiveQuantities,
    old: WebConstitutiveQuantities,
    tolerance: f64,
) -> (CoreGateOutcome, WebGateOutcome, ConjunctVerdict) {
    let witness = WebTransitionWitness { old, new };
    let response = WebConstitutiveResponse::from(new);
    web_gate(&response, &witness, tolerance)
}

/// Canonical web transition with HCOM-006 semantic residual conjuncts.
#[must_use]
pub fn canonical_web_transition_from_tensors_with_semantic(
    model: &WebConstitutiveModel,
    old: &[f64; 64],
    new: &[f64; 64],
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
    web_transition_gate_outcome_with_semantic(model, old, new, tolerance, semantic_defect_tolerance)
}

/// Semantic residual conjunct outcome from 64D tensor rows (HCOM-006 bridge).
#[must_use]
pub fn canonical_web_semantic_gate_outcome(
    semantic_witness: &WebSemanticTransitionWitness,
    defect_tolerance: f64,
) -> WebSemanticGateOutcome {
    evaluate_semantic_conjuncts(semantic_witness, defect_tolerance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use umst_gate::ConjunctVerdict;

    #[test]
    fn route_delegates_to_web_gate() {
        let old = WebConstitutiveQuantities::balanced();
        let new = WebConstitutiveQuantities::balanced();
        let witness = WebTransitionWitness { old, new };
        let response = WebConstitutiveResponse::from(new);
        let routed = canonical_web_transition_outcome(&response, &witness, DEFAULT_INT_TOLERANCE);
        let direct = web_gate(&response, &witness, DEFAULT_INT_TOLERANCE);
        assert_eq!(routed, direct);
    }

    #[test]
    fn quantities_bridge_accepts_balanced() {
        let q = WebConstitutiveQuantities::balanced();
        let (_, _, composed) = canonical_web_gate_from_quantities(q, q, DEFAULT_INT_TOLERANCE);
        assert_eq!(composed, ConjunctVerdict::Accepted);
    }

    #[test]
    fn web_route_admissible_matches_transition_outcome() {
        let old = WebConstitutiveQuantities::balanced();
        let new = WebConstitutiveQuantities::balanced();
        let witness = WebTransitionWitness { old, new };
        let response = WebConstitutiveResponse::from(new);
        assert!(canonical_web_transition_admissible(&response, &witness));
        assert_eq!(
            canonical_web_gate_outcome(&response, &witness).verdict,
            ConjunctVerdict::Accepted
        );
    }

    #[test]
    fn web_route_semantic_gate_accepts_zero_defect() {
        use crate::web_constitutive::WebSemanticResidual;
        let semantic = WebSemanticTransitionWitness {
            old: WebSemanticResidual::neutral(),
            new: WebSemanticResidual::neutral(),
        };
        let outcome = canonical_web_semantic_gate_outcome(&semantic, DEFAULT_INT_TOLERANCE);
        assert!(outcome.is_accepted());
    }
}

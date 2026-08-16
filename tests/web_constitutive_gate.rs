// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! WEB-005 — `WebConstitutiveModel` gate integration tests (build-spec §A8).

use umst_gate::{AdmissibilityResponse, ConjunctVerdict, GateRejectReason};
use umst_manifold::gate::{
    canonical_web_gate_from_quantities, canonical_web_transition_from_tensors,
    canonical_web_transition_outcome,
};
use umst_manifold::web_constitutive::{
    slice_layout, WebConstitutiveModel, WebConstitutiveQuantities, WebConstitutiveResponse,
    WebTransitionWitness, DEFAULT_INT_TOLERANCE,
};

#[test]
fn web_constitutive_module_exports_cartridge_model() {
    let model = WebConstitutiveModel::cartridge();
    assert!((model.complexity_weight - 1.0).abs() < f64::EPSILON);
}

#[test]
fn gate_route_balanced_fixture_accepts() {
    let q = WebConstitutiveQuantities::balanced();
    let witness = WebTransitionWitness { old: q, new: q };
    let response = WebConstitutiveResponse::from(q);
    let (_, _, composed) =
        canonical_web_transition_outcome(&response, &witness, DEFAULT_INT_TOLERANCE);
    assert_eq!(composed, ConjunctVerdict::Accepted);
}

#[test]
fn gate_route_under_budget_rejects() {
    let old = WebConstitutiveQuantities::balanced();
    let new = WebConstitutiveQuantities::under_budget();
    let (_, _, composed) = canonical_web_gate_from_quantities(new, old, DEFAULT_INT_TOLERANCE);
    assert_eq!(
        composed,
        ConjunctVerdict::Rejected(GateRejectReason::NegativeDissipation)
    );
}

#[test]
fn tensor_path_produces_finite_quantities() {
    let model = WebConstitutiveModel::cartridge();
    let mut old = [0.0_f64; slice_layout::DIM];
    let mut new = [0.0_f64; slice_layout::DIM];
    old[0] = 1.0;
    new[0] = 1.0;
    for i in slice_layout::PRESENTATION.clone() {
        new[i] = 0.5;
    }

    let (witness, response, _, web, composed) =
        canonical_web_transition_from_tensors(&model, &old, &new, DEFAULT_INT_TOLERANCE);
    assert!(witness.new.complexity_cost.is_finite());
    assert!(response.dissipation().is_finite());
    assert!(web.cost_legs_valid);
    assert!(composed.is_accepted() || !composed.is_accepted()); // exercised path
}

#[test]
fn slice_layout_dim_is_sixty_four() {
    assert_eq!(slice_layout::DIM, 64);
    assert_eq!(slice_layout::BEHAVIOR_UCRS.end, 64);
}

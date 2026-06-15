// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Reject-path telemetry: every hot gate carries a stable `catalog_id` slug.

use umst_manifold::ai::formal::{FormalReject, LANDAUER_CBF_CATALOG_ID};
use umst_manifold::embodied::{EmbodiedReject, HostTransitionStep};
use umst_manifold::gate::http_manifest::MixProposal;
use umst_manifold::gate::{
    default_gate_manifest, evaluate_http_mix_manifest, gate_json_parse_response,
    AdmissibilityVerdict, GateEvaluator, ThermodynamicState, ThermodynamicTransitionEvaluator,
    TransitionGateEvaluator,
};
use umst_manifold::runtime::catalog::traceability::{
    CD_TRANSITION_CATALOG_ID, HTTP_SHIM_CATALOG_ID, THERMODYNAMIC_MIX_CATALOG_ID,
};

#[test]
fn cd_transition_evaluator_catalog_id_stable() {
    let eval = ThermodynamicTransitionEvaluator::new();
    assert_eq!(eval.catalog_id(), CD_TRANSITION_CATALOG_ID);
}

#[test]
fn cd_transition_mass_reject_maps_to_host_transition_slug() {
    let mut eval = ThermodynamicTransitionEvaluator::new();
    let old = ThermodynamicState {
        density: 2400.0,
        temperature: 293.0,
        free_energy: 0.0,
        entropy: 0.1,
        reaction_extent: 0.3,
        strength: 10.0,
    };
    let mut new = old.clone();
    new.density = 2280.0;
    let tv = eval.check_transition_host(&old, &new, 3600.0);
    assert!(!tv.admissible);
    let rej = EmbodiedReject::HostTransition {
        catalog_id: CD_TRANSITION_CATALOG_ID,
        verdict: tv.rest_verdict(),
    };
    assert_eq!(rej.catalog_id(), CD_TRANSITION_CATALOG_ID);
    assert_eq!(rej.catalog_id(), eval.catalog_id());
    assert_eq!(tv.rest_verdict(), AdmissibilityVerdict::MassViolation);
}

#[test]
fn thermodynamic_mix_slug_matches_registry_routing() {
    assert_eq!(THERMODYNAMIC_MIX_CATALOG_ID, "thermodynamic_mix");
    let step = HostTransitionStep {
        catalog_id: THERMODYNAMIC_MIX_CATALOG_ID,
        old_state: &ThermodynamicState::new(),
        new_state: &ThermodynamicState::new(),
        dt_s: 1.0,
    };
    assert_eq!(step.catalog_id, THERMODYNAMIC_MIX_CATALOG_ID);
}

#[test]
fn formal_cbf_reject_catalog_id_is_landauer_slug() {
    let rej = FormalReject::ThermodynamicControlBarrier {
        catalog_id: LANDAUER_CBF_CATALOG_ID,
        detail: "test".into(),
    };
    assert_eq!(rej.catalog_id(), LANDAUER_CBF_CATALOG_ID);
    assert!(rej.to_string().contains(LANDAUER_CBF_CATALOG_ID));
}

#[test]
fn http_mix_reject_includes_http_shim_catalog_id() {
    let m = default_gate_manifest();
    let reject = MixProposal {
        constituent_primary_kg: 400.0,
        constituent_secondary_kg: 0.0,
        constituent_tertiary_kg: 0.0,
        water: 200.0,
        age_days: 28.0,
        predicted_strength_mpa: 1.0e9,
        temperature_c: 20.0,
    };
    let r = evaluate_http_mix_manifest(&reject, &m);
    assert!(!r.admissible);
    assert_eq!(r.catalog_id.as_deref(), Some(HTTP_SHIM_CATALOG_ID));
}

#[test]
fn http_json_parse_reject_carries_catalog_id() {
    let r = gate_json_parse_response();
    assert_eq!(r.catalog_id.as_deref(), Some(HTTP_SHIM_CATALOG_ID));
}

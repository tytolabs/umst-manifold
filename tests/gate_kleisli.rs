// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Integration checks for `umst_manifold::gate`: Kleisli pipeline, evaluator registry, CBF adapter.

use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::gate::{
    gate_arrow, kleisli_compose_pair, AdmissibilityVerdict, Admissible, GateEvaluator,
    GateEvaluatorRegistry, GateThermodynamicCBF, KleisliPipeline, KleisliUnitEvaluator,
    ThermodynamicMixEvaluator, ThermodynamicMixFilter, ThermodynamicStateSnapshot,
    ThermodynamicTransitionContext,
};

type B = NdArray<f32>;

#[test]
fn kleisli_unit_evaluator_catalog_surface_stable() {
    let ge = KleisliUnitEvaluator::new();
    assert_eq!(ge.catalog_id(), "umst.gate.kleisli_unit");
    assert_eq!(ge.gate_family(), "kleisli_admissibility_unit");
}

#[test]
fn registry_routes_kleisli_unit_by_catalog_id() {
    let mut reg = GateEvaluatorRegistry::default();
    reg.register_kleisli(KleisliUnitEvaluator::new());

    let v = reg
        .evaluate_kleisli_unit("umst.gate.kleisli_unit")
        .expect("registered kleisli unit evaluator");
    assert_eq!(v, AdmissibilityVerdict::Accepted);
    assert_eq!(v.as_str(), AdmissibilityVerdict::ACCEPTED);

    let state = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let ctx = ThermodynamicTransitionContext {
        old_state: &state,
        new_state: &state,
        dt_seconds: 0.0,
    };
    let v_reflex = reg
        .evaluate_mut("umst.gate.kleisli_unit", ctx)
        .expect("reflexive step via registry");
    assert_eq!(v_reflex, AdmissibilityVerdict::Accepted);
}

#[test]
fn kleisli_compose_preserves_admissibility_chain() {
    let inc = kleisli_compose_pair(
        |x: i32| Admissible::pure(x + 1),
        |y: i32| Admissible::pure(y * 2),
        "inc_then_double",
    );
    let out = inc.run(5);
    assert!(out.result.admissible);
    assert_eq!(out.value, 12);
}

#[test]
fn registry_routes_mix_evaluator_to_rest_verdict_strings() {
    let mut reg = GateEvaluatorRegistry::default();
    reg.register(ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new()));

    let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
    let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
    let ctx = ThermodynamicTransitionContext {
        old_state: &old,
        new_state: &new,
        dt_seconds: 3600.0,
    };
    let v = reg
        .evaluate_mut("thermodynamic_mix", ctx)
        .expect("registered evaluator");

    assert_eq!(v, AdmissibilityVerdict::Accepted);
    assert_eq!(v.as_str(), AdmissibilityVerdict::ACCEPTED);
}

#[test]
fn gate_cbf_delegates_verify_tensor_update() {
    let dev = NdArrayDevice::default();
    let mut gate_cbf = GateThermodynamicCBF::new(300.0_f64, 1.0e-12_f64);
    gate_cbf.k_phys_dint_to_joules = 1.0;
    let d_int = Tensor::<B, 1>::from_floats([-1.0e6_f32], &dev);
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
    gate_cbf
        .verify_tensor_update(d_int, info_gain)
        .expect("delegate matches ThermodynamicCBF clamp semantics");
}

#[test]
fn kleisli_mass_gate_then_identity_pipeline() {
    let pipe = KleisliPipeline::new("bulk_step");
    let ok = gate_arrow("positive_denominator", |x: &f64| {
        if *x > 0.0 {
            (true, 0.0, None)
        } else {
            (false, -1.0, Some("non_positive".into()))
        }
    });
    let seq = pipe.run_sequence(3.14_f64, &[&ok]);
    assert!(seq.result.admissible);
}

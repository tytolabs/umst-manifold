// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Parity between **host** Clausius–Duhem transition gates and scalar **Landauer**/dissipation bridging.

use umst_manifold::ai::cbf::ThermodynamicCBF;
use umst_manifold::gate::cbf_bridge::cd_dissipation_proxy_to_entropy_joules;
use umst_manifold::gate::ThermodynamicState;
use umst_manifold::gate::{GateEvaluator, ThermodynamicGate, ThermodynamicTransitionEvaluator};
use umst_manifold::runtime::catalog::witness_catalog_quickcheck_ok;
#[path = "injection_mechanism_fixture.rs"]
mod injection_mechanism_fixture;

use injection_mechanism_fixture::InjectionFixtureParams;

fn closure_params() -> InjectionFixtureParams {
    InjectionFixtureParams
}

#[test]
fn catalog_witness_quickcheck() {
    assert!(
        witness_catalog_quickcheck_ok(),
        "bundled catalog lock must parse"
    );
}

#[test]
fn transition_positive_drives_nonnegative_entropy_proxy_f64() {
    let mut tg = ThermodynamicGate::new();
    let old = ThermodynamicState::from_mix_with_params(0.5, 0.4, 293.0, &closure_params());
    let new = ThermodynamicState::from_mix_with_params(0.5, 0.65, 293.0, &closure_params());
    let verdict = tg.check_transition(&old, &new, 86400.0_f64);

    assert!(verdict.accepted, "sanity: forward hydration should admit");
    let joules_like = cd_dissipation_proxy_to_entropy_joules(verdict.dissipation, 1.0, 1.0);
    assert!(
        joules_like >= 0.0,
        "nonnegative D_int → nonnegative scalar proxy: {joules_like}"
    );
}

#[test]
fn cbf_accepts_after_positive_proxy_deduction() {
    let mut cbf = ThermodynamicCBF::new(300.0_f64, 1.0e-6_f64);
    cbf.k_phys_dint_to_joules = 0.0;
    let work = cd_dissipation_proxy_to_entropy_joules(1.0_f64, 1.0, 1.0);
    let out = cbf
        .verify_and_deduct_update(work, 0.0_f64)
        .expect("nonnegative generalized entropy admission");
    assert!(out >= 0.0);
}

#[test]
fn gate_evaluator_wires_catalog_id() {
    let eval = ThermodynamicTransitionEvaluator::new();
    assert_eq!(eval.catalog_id(), "umst.gate.cd_transition");
}

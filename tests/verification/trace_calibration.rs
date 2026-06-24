// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track G.3 — η from emitted traces (`trace-calibration` feature).

use std::f64::consts::LN_2;

use burn::prelude::Backend;
use burn::tensor::Tensor;
use burn_ndarray::NdArray;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::ros::{
    calibrate_eta_bound_from_trace, prototype_eta_from_trace, step_mi_excess_over_catalog,
    step_mi_within_catalog, EmittedStepRecord, EmittedTraceSchema, CATALOG_STEP_MI_UPPER_NAT,
};

struct EtaStubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for EtaStubCartridge {
    fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
        let d = mix.fractions.device();
        PhysicalResult {
            free_energy: Tensor::zeros([1, 1], &d),
            dissipation: Tensor::zeros([1, 1], &d),
            safety_margin: Tensor::zeros([1, 1], &d),
            cost: Tensor::zeros([1, 1], &d),
            damage: Tensor::zeros([1, 1], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, 1], &d),
        }
    }

    fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
        let d = m.scalar_features.device();
        let n = m.scalar_features.dims()[0];
        PhysicalResult {
            free_energy: Tensor::zeros([1, n], &d),
            dissipation: Tensor::zeros([1, n], &d),
            safety_margin: Tensor::zeros([1, n], &d),
            cost: Tensor::zeros([1, n], &d),
            damage: Tensor::zeros([1, n], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, n], &d),
        }
    }
}

#[test]
fn sample_fixture_all_steps_within_catalog_mi_band() {
    let trace = EmittedTraceSchema::sample_fixture();
    let report = calibrate_eta_bound_from_trace(&trace);
    assert!(report.all_within_catalog);
    assert_eq!(report.eta_bound_suggested, 0.0);
    assert_eq!(report.steps_checked, 2);
}

#[test]
fn eta_bound_positive_when_step_mi_exceeds_catalog() {
    let over = EmittedStepRecord::new(CATALOG_STEP_MI_UPPER_NAT + 0.01, 1.0e-21);
    let trace = EmittedTraceSchema::new(1, 300.0, vec![over]);
    let report = calibrate_eta_bound_from_trace(&trace);
    assert!(!report.all_within_catalog);
    assert!(report.eta_bound_suggested > 0.0);
}

#[test]
fn step_mi_excess_helpers_match_catalog_upper() {
    assert_eq!(step_mi_excess_over_catalog(0.1), 0.0);
    assert!(step_mi_within_catalog(0.1));
    assert!(!step_mi_within_catalog(CATALOG_STEP_MI_UPPER_NAT + 1e-6));
}

#[test]
fn manifold_gateway_eta_wired_from_catalog_report() {
    let trace = EmittedTraceSchema::sample_fixture();
    let report = calibrate_eta_bound_from_trace(&trace);
    let mut gateway: ManifoldGateway<NdArray<f32>, EtaStubCartridge> =
        ManifoldGateway::new(EtaStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.calibrate_eta_from_trace(&trace);
    assert_eq!(gateway.eta, report.eta_bound_suggested as f32);
    assert_eq!(gateway.eta, 0.0);
}

#[test]
fn manifold_gateway_eta_positive_on_catalog_mi_overrun() {
    let over = EmittedStepRecord::new(LN_2 + 0.01, 1.0e-21);
    let trace = EmittedTraceSchema::new(1, 300.0, vec![over]);
    let report = calibrate_eta_bound_from_trace(&trace);
    let mut gateway: ManifoldGateway<NdArray<f32>, EtaStubCartridge> =
        ManifoldGateway::new(EtaStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.calibrate_eta_from_trace(&trace);
    assert!(gateway.eta > 0.0);
    assert_eq!(gateway.eta, report.eta_bound_suggested as f32);
}

#[test]
fn prototype_eta_from_calibration_envelope_fixture() {
    let schema = EmittedTraceSchema::sample_calibration_envelope_fixture();
    let eta = prototype_eta_from_trace(&schema);
    assert!(eta > 0.0 && eta <= 1.0);
    assert!((eta - 0.5).abs() < 0.01);
}

#[test]
fn prototype_eta_zero_when_envelope_empty() {
    let schema = EmittedTraceSchema::new(0, 300.0, vec![]);
    assert_eq!(prototype_eta_from_trace(&schema), 0.0);
}

#[test]
fn manifold_gateway_prototype_envelope_eta_distinct_from_catalog() {
    let schema = EmittedTraceSchema::sample_calibration_envelope_fixture();
    let catalog = calibrate_eta_bound_from_trace(&schema);
    let prototype = prototype_eta_from_trace(&schema);
    let mut gateway: ManifoldGateway<NdArray<f32>, EtaStubCartridge> =
        ManifoldGateway::new(EtaStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.calibrate_eta_from_prototype_envelope(&schema);
    assert_eq!(gateway.eta, prototype);
    assert_eq!(catalog.eta_bound_suggested, 0.0);
    assert!(prototype > 0.0);
}

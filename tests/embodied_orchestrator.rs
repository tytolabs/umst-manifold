// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! [`EmbodiedOrchestrator`]: gateway tensor path + optional host gates via registry / CD transition.

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::info_gain::suggested_info_gain_from_batched_nodal_scalars;
use umst_manifold::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::embodied::{EmbodiedOrchestrator, EmbodiedReject, HostTransitionStep};
use umst_manifold::gate::{
    AdmissibilityVerdict, GateEvaluatorRegistry, KleisliUnitEvaluator, ThermodynamicMixEvaluator,
    ThermodynamicMixFilter, ThermodynamicState,
};
use umst_manifold::manifest::UmstManifest;
use umst_manifold::runtime::catalog::traceability::{
    CD_TRANSITION_CATALOG_ID, THERMODYNAMIC_MIX_CATALOG_ID,
};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn tiny_umst() -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let n = 2usize;
    let f = 5usize;
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    let scalar_features = Tensor::<B, 2>::zeros([n, f], &dev);
    let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &dev);
    let matrix_features = Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features,
        vector_features,
        matrix_features,
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

struct GatewayStubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for GatewayStubCartridge {
    fn compute_all(&self, mix: &MixTensor<Bk>) -> PhysicalResult<Bk> {
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

fn golden_identity_host() -> (ThermodynamicState, ThermodynamicState, f64) {
    let s = ThermodynamicState {
        density: 2400.0,
        temperature: 293.15,
        free_energy: -1.35e5,
        entropy: 0.05,
        hydration_degree: 0.42,
        strength: 12.7,
    };
    (s.clone(), s, 1.0)
}

fn golden_mass_reject_host() -> (ThermodynamicState, ThermodynamicState, f64) {
    let old = ThermodynamicState {
        density: 2400.0,
        temperature: 293.0,
        free_energy: 0.0,
        entropy: 0.1,
        hydration_degree: 0.3,
        strength: 10.0,
    };
    let mut new = old.clone();
    new.density = 2280.0;
    (old, new, 3600.0)
}

#[test]
fn embodied_gateway_only_accepts_without_host_gate() {
    let baseline = tiny_umst();
    let mut proposed = tiny_umst();
    proposed.scalar_features = proposed.scalar_features.clone().add_scalar(0.1_f32);

    let baseline_batched = baseline.scalar_features.clone().unsqueeze_dim::<3>(0);
    let proposed_batched = proposed.scalar_features.clone().unsqueeze_dim::<3>(0);
    let info_gain =
        suggested_info_gain_from_batched_nodal_scalars(baseline_batched, proposed_batched);

    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);
    let result = orch.evaluate_topology_step(proposed, info_gain, None);
    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn embodied_host_cd_transition_rejects_mass_before_gateway() {
    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());
    let (old, new, dt) = golden_mass_reject_host();

    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);
    let host = HostTransitionStep {
        catalog_id: "umst.gate.cd_transition",
        old_state: &old,
        new_state: &new,
        dt_s: dt,
    };
    let result = orch.evaluate_topology_step(proposed, info_gain, Some(host));
    let err = match result {
        Err(e) => e,
        Ok(_) => panic!("expected host mass rejection"),
    };
    assert!(matches!(
        err,
        EmbodiedReject::HostTransition {
            catalog_id: CD_TRANSITION_CATALOG_ID,
            verdict: AdmissibilityVerdict::MassViolation,
        }
    ));
    assert_eq!(err.catalog_id(), CD_TRANSITION_CATALOG_ID);
}

#[test]
fn embodied_mix_registry_accepts_forward_hydration() {
    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());

    let mut reg = GateEvaluatorRegistry::default();
    reg.register(ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new()));

    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64)
        .with_mix_registry(reg);

    let old = ThermodynamicState::from_mix(0.5, 0.3, 293.0);
    let new = ThermodynamicState::from_mix(0.5, 0.5, 293.0);
    let host = HostTransitionStep {
        catalog_id: THERMODYNAMIC_MIX_CATALOG_ID,
        old_state: &old,
        new_state: &new,
        dt_s: 3600.0,
    };

    assert!(orch
        .evaluate_topology_step(proposed, info_gain, Some(host))
        .is_ok());
}

#[test]
fn embodied_mix_registry_rejects_reverse_hydration_with_catalog_id() {
    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());

    let mut reg = GateEvaluatorRegistry::default();
    reg.register(ThermodynamicMixEvaluator::new(ThermodynamicMixFilter::new()));

    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64)
        .with_mix_registry(reg);

    let old = ThermodynamicState::from_mix(0.5, 0.5, 293.0);
    let new = ThermodynamicState::from_mix(0.5, 0.3, 293.0);
    let host = HostTransitionStep {
        catalog_id: THERMODYNAMIC_MIX_CATALOG_ID,
        old_state: &old,
        new_state: &new,
        dt_s: 3600.0,
    };

    let err = match orch.evaluate_topology_step(proposed, info_gain, Some(host)) {
        Err(e) => e,
        Ok(_) => panic!("reverse hydration should reject"),
    };
    assert!(matches!(
        err,
        EmbodiedReject::HostTransition {
            catalog_id: THERMODYNAMIC_MIX_CATALOG_ID,
            ..
        }
    ));
    assert_eq!(err.catalog_id(), THERMODYNAMIC_MIX_CATALOG_ID);
}

#[test]
fn embodied_host_kleisli_unit_routes_via_default_registry() {
    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());
    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);

    let state = ThermodynamicState::from_mix(0.5, 0.3, 293.0);
    let host = HostTransitionStep {
        catalog_id: KleisliUnitEvaluator::CATALOG_ID,
        old_state: &state,
        new_state: &state,
        dt_s: 0.0,
    };

    assert!(orch
        .evaluate_topology_step(proposed, info_gain, Some(host))
        .is_ok());
}

#[test]
fn embodied_kleisli_missing_registry_returns_slug_not_generic_missing() {
    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());
    let reg = GateEvaluatorRegistry::default();
    let mut orch = EmbodiedOrchestrator::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64)
        .with_mix_registry(reg);

    let state = ThermodynamicState::from_mix(0.5, 0.3, 293.0);
    let host = HostTransitionStep {
        catalog_id: KleisliUnitEvaluator::CATALOG_ID,
        old_state: &state,
        new_state: &state,
        dt_s: 0.0,
    };

    let err = match orch.evaluate_topology_step(proposed, info_gain, Some(host)) {
        Err(e) => e,
        Ok(_) => panic!("unregistered kleisli unit should fail"),
    };
    assert!(matches!(err, EmbodiedReject::HostRegistryMissing { .. }));
    assert_eq!(err.catalog_id(), KleisliUnitEvaluator::CATALOG_ID);
}

#[test]
fn embodied_from_manifest_dual_run_requires_host_step() {
    let manifest = UmstManifest::default();
    let mut orch = EmbodiedOrchestrator::from_manifest(GatewayStubCartridge, &manifest);
    orch.dual_run = true;

    let proposed = tiny_umst();
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &device());
    let err = match orch.evaluate_topology_step(proposed, info_gain, None) {
        Err(e) => e,
        Ok(_) => panic!("dual_run without host step"),
    };
    assert!(matches!(err, EmbodiedReject::HostRegistryMissing { .. }));
    assert_eq!(err.catalog_id(), CD_TRANSITION_CATALOG_ID);
}

#[test]
fn embodied_dual_run_accepts_when_host_and_gateway_pass() {
    let baseline = tiny_umst();
    let mut proposed = tiny_umst();
    proposed.scalar_features = proposed.scalar_features.clone().add_scalar(0.1_f32);
    let baseline_batched = baseline.scalar_features.clone().unsqueeze_dim::<3>(0);
    let proposed_batched = proposed.scalar_features.clone().unsqueeze_dim::<3>(0);
    let info_gain =
        suggested_info_gain_from_batched_nodal_scalars(baseline_batched, proposed_batched);

    let mut manifest = UmstManifest::default();
    manifest.dual_run = true;
    let mut orch = EmbodiedOrchestrator::from_manifest(GatewayStubCartridge, &manifest);

    let (old, new, dt) = golden_identity_host();
    let host = HostTransitionStep {
        catalog_id: "umst.gate.cd_transition",
        old_state: &old,
        new_state: &new,
        dt_s: dt,
    };

    assert!(orch
        .evaluate_topology_step(proposed, info_gain, Some(host))
        .is_ok());
}

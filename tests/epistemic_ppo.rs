// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Epistemic PPO integration tests (`--features epistemic-ppo` only).
//!
//! Histogram MI feeds the Landauer CBF branch; epistemic bonus is applied post-R2 per
//! [`GOD_GRADE_WITNESS_LADDER`](../docs/RELEASE_WITNESS_LADDER.md).

#![cfg(feature = "epistemic-ppo")]

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::adjoint::AdjointNeuralODE;
use umst_manifold::ai::info_gain::{
    histogram_info_gain_tensor, nodal_scalar_means, MutualInfoEstimator,
};
use umst_manifold::ai::liquid_ppo::BurnLiquidPPOAgent;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{StatePoint, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::{UMST_SCALAR_CHANNEL_COUNT, SCALAR_INTERNAL_VARIABLE_0};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn umst_with_hydration(hydration: f32, n: usize, f: usize) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    let mut data = vec![0.0_f32; n * f];
    for i in 0..n {
        data[i * f + SCALAR_INTERNAL_VARIABLE_0] = hydration;
    }
    let scalar_features = Tensor::<B, 2>::from_data(Data::new(data, Shape::new([n, f])), &dev);
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

/// Dissipation rises when hydration deviates from 0.35 — gate rejects large policy jumps.
struct GateAwareCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for GateAwareCartridge {
    fn compute_all(&self, mix: &StatePoint<Bk>) -> PhysicalResult<Bk> {
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
        let alpha_col = m.scalar_features.clone().slice([
            0..n,
            SCALAR_INTERNAL_VARIABLE_0..SCALAR_INTERNAL_VARIABLE_0 + 1,
        ]);
        let target = 0.35_f32;
        let dissipation = alpha_col
            .sub_scalar(target)
            .powf_scalar(2.0)
            .mul_scalar(50.0);
        PhysicalResult {
            free_energy: Tensor::zeros([1, n], &d),
            dissipation,
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
fn adjoint_forward_mutates_policy_editable_scalars() {
    let dev = device();
    let n = 2usize;
    let f = 7usize;
    let mut umst = umst_with_hydration(0.5, n, f);
    umst.policy_editable_mask =
        Tensor::<B, 2>::from_data(Data::new(vec![1.0_f32, 0.0_f32], Shape::new([n, 1])), &dev);

    let mut ode = AdjointNeuralODE::<B>::new(&dev);
    ode.policy_weights = Tensor::<B, 1>::full([1024], 0.5_f32, &dev);

    let before: Vec<f32> = umst.scalar_features.clone().into_data().value;
    let out = ode.forward(umst, 0.0, 1.0);
    let after: Vec<f32> = out.scalar_features.into_data().value;

    assert!(
        (after[2] - before[2]).abs() > 1e-5,
        "editable node hydration should change"
    );
    assert!(
        (after[f + 2] - before[f + 2]).abs() < 1e-5,
        "masked node hydration should stay fixed"
    );
}

#[test]
fn histogram_mi_tensor_respects_landauer_ln2_cap() {
    let dev = device();
    let mut est = MutualInfoEstimator::for_material_proxy();
    for i in 0..400 {
        let x = i as f64 / 400.0;
        est.update(
            &[x; UMST_SCALAR_CHANNEL_COUNT],
            &[x; UMST_SCALAR_CHANNEL_COUNT],
        );
    }
    let t = histogram_info_gain_tensor::<B>(
        &mut est,
        &[0.9; UMST_SCALAR_CHANNEL_COUNT],
        &[0.9; UMST_SCALAR_CHANNEL_COUNT],
        &dev,
    );
    let v: f32 = t.into_data().value[0];
    let v = v as f64;
    assert!(v <= f64::ln(2.0) + 1e-6);
    assert!(v >= 0.0);
}

#[test]
fn manifold_gateway_alpha_beta_gamma_weights() {
    let dev = device();
    let umst = umst_with_hydration(0.4, 2, UMST_SCALAR_CHANNEL_COUNT);
    let info = Tensor::<B, 1>::full([1], 0.001_f32, &dev);

    let mut g_base = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-6_f64);
    g_base.alpha = 1.0;
    g_base.beta = 0.0;
    g_base.gamma = 0.0;
    let r_base = g_base
        .evaluate_topology_step(umst.clone(), info.clone())
        .unwrap()
        .1
        .into_data()
        .value[0];

    let mut g_pen = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-6_f64);
    g_pen.alpha = 1.0;
    g_pen.beta = 10.0;
    g_pen.gamma = 0.0;
    let r_pen = g_pen
        .evaluate_topology_step(umst, info)
        .unwrap()
        .1
        .into_data()
        .value[0];

    assert!(r_pen < r_base, "higher beta should reduce reward");
}

#[test]
fn epistemic_training_improves_gate_pass_rate() {
    let dev = device();
    let n = 2usize;
    let f = 7usize;
    let mut gateway = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-4_f64);
    gateway.beta = 0.1;
    let mut agent = BurnLiquidPPOAgent::new(gateway);

    // Large positive weights → ODE pushes hydration away from target → CBF failures.
    agent.ode_solver.policy_weights = Tensor::<B, 1>::full([1024], 2.0_f32, &dev);

    let dt = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
    let dummy_info = Tensor::<B, 1>::zeros([1], &dev);

    let mut passes_first = 0usize;
    for _ in 0..8 {
        let state = umst_with_hydration(0.5, n, f);
        if agent
            .step_and_learn(state, 0.0_f32, 0.5_f32, dummy_info.clone(), dt.clone())
            .is_ok()
        {
            passes_first += 1;
        }
    }

    let mut passes_last = 0usize;
    for _ in 0..8 {
        let state = umst_with_hydration(0.5, n, f);
        if agent
            .step_and_learn(state, 0.0_f32, 0.5_f32, dummy_info.clone(), dt.clone())
            .is_ok()
        {
            passes_last += 1;
        }
    }

    assert!(
        passes_last >= passes_first,
        "gate pass rate should not decrease: first={passes_first} last={passes_last}"
    );
    assert!(
        passes_last > passes_first || passes_last >= 4,
        "training should improve or sustain high pass rate: first={passes_first} last={passes_last}"
    );
}

#[test]
fn nodal_scalar_means_matches_manual_average() {
    let dev = device();
    let t = Tensor::<B, 2>::from_data(
        Data::new(vec![1.0_f32, 3.0_f32, 5.0_f32, 7.0_f32], Shape::new([2, 2])),
        &dev,
    );
    let m = nodal_scalar_means(&t, 2);
    assert!((m[0] - 3.0).abs() < 1e-5);
    assert!((m[1] - 5.0).abs() < 1e-5);
}

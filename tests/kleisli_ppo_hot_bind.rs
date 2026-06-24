// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Kleisli penalize hot-bind integration (`--features kleisli-ppo-hot-bind` only).
//!
//! Verifies [`BurnLiquidPPOAgent::step_and_learn`] routes through
//! [`ManifoldGateway::constraint_loss_penalty`] when `lambda_cd ≠ 0`.

#![cfg(feature = "kleisli-ppo-hot-bind")]

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::constraint_loss::clausius_duhem_violation;
use umst_manifold::ai::liquid_ppo::BurnLiquidPPOAgent;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{StatePoint, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn tiny_umst() -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let n = 2usize;
    let f = UMST_SCALAR_CHANNEL_COUNT;
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

struct PpoChainStubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoChainStubCartridge {
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
fn kleisli_ppo_hot_bind_step_and_learn_uses_constraint_loss_path() {
    let dev = device();
    let mut gateway = ManifoldGateway::new(PpoChainStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.lambda_cd = 0.5_f32;
    let mut agent = BurnLiquidPPOAgent::new(gateway);
    let state = tiny_umst();

    let info = Tensor::<B, 1>::full([1], 0.01_f32, &dev);
    let dt_rat = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
    let w0 = agent.ode_solver.policy_weights.clone().into_data().value[0];
    let out = agent.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
    assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
    let w1 = agent.ode_solver.policy_weights.clone().into_data().value[0];
    assert!(w0.is_finite() && w1.is_finite(), "weights must stay finite");
    assert_ne!(
        w0, w1,
        "Kleisli penalize path should move policy_weights after backward"
    );
}

#[test]
fn kleisli_ppo_hot_bind_constraint_loss_penalty_nonzero_when_lambda_set() {
    let dev = device();
    let mut gateway = ManifoldGateway::new(PpoChainStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.lambda_cd = 1.0_f32;
    let rho = Tensor::<B, 1>::full([1], 2400.0_f32, &dev);
    let old_fe = Tensor::<B, 1>::full([1], -1.0e5_f32, &dev);
    let new_fe = Tensor::<B, 1>::full([1], -1.0e4_f32, &dev);
    let dt = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
    let penalty = gateway.constraint_loss_penalty(
        rho.clone(),
        rho.clone(),
        old_fe.clone(),
        new_fe.clone(),
        dt.clone(),
    );
    let raw = clausius_duhem_violation(rho.clone(), rho, old_fe, new_fe, dt);
    let p: Vec<f32> = penalty.into_data().value;
    let r: Vec<f32> = raw.into_data().value;
    assert!(
        (p[0] - r[0]).abs() < 1e-3,
        "penalty should equal λ_cd · violation, got {} vs {}",
        p[0],
        r[0]
    );
}

#[test]
fn kleisli_ppo_hot_bind_landauer_penalty_nonzero_when_lambda_set() {
    let dev = device();
    let mut gateway = ManifoldGateway::new(PpoChainStubCartridge, 300.0_f64, 1.0e-12_f64);
    gateway.lambda_landauer = 0.5_f32;
    let info_bits = Tensor::<B, 1>::full([1], 2.0_f32, &dev);
    let penalty = gateway.landauer_constraint_loss_penalty(info_bits.clone());
    let p: Vec<f32> = penalty.into_data().value;
    assert!(
        p[0] > 0.0,
        "Landauer penalty must be positive when bits exceed credit, got {}",
        p[0]
    );

    let rho = Tensor::<B, 1>::full([1], 2400.0_f32, &dev);
    let old_fe = Tensor::<B, 1>::full([1], -1.0e5_f32, &dev);
    let new_fe = Tensor::<B, 1>::full([1], -1.0e4_f32, &dev);
    let dt = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
    let total =
        gateway.total_constraint_loss_penalty(rho.clone(), rho, old_fe, new_fe, dt, info_bits);
    let t: Vec<f32> = total.into_data().value;
    assert!(
        t[0].is_finite() && t[0] > 0.0,
        "total penalty must be finite and positive"
    );
}

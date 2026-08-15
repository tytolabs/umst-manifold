// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP P3.5 — THMC ↔ UMST writeback parity (`sync_thmc_to_umst`).

#![cfg(feature = "thmc-coupled")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::umst_schema::{
    SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE, UMST_SCALAR_CHANNEL_COUNT,
};
use umst_manifold::physics::error::PhysicsError;
use umst_manifold::physics::solvers::{ThmcSolver, ThmcState};
use umst_manifold::physics::thmc_umst_sync::sync_thmc_to_umst;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn toy_umst(n: usize, t: f32, h: f32, d: f32) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[SCALAR_TEMPERATURE] = t;
    flat[SCALAR_HUMIDITY] = h;
    flat[SCALAR_DAMAGE] = d;
    if n > 1 {
        flat[f + SCALAR_TEMPERATURE] = t + 1.0;
        flat[f + SCALAR_HUMIDITY] = h + 0.1;
        flat[f + SCALAR_DAMAGE] = d + 0.05;
    }
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features: scalars,
        vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
        matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

#[test]
fn sync_thmc_rejects_batch_gt_one() {
    let dev = device();
    let n = 2usize;
    let mut umst = toy_umst(n, 300.0, 0.5, 0.1);
    let state = ThmcState::from_tensors(
        Tensor::<B, 3>::full([2, n, 1], 310.0, &dev),
        Tensor::<B, 3>::full([2, n, 1], 0.6, &dev),
        Tensor::<B, 3>::zeros([2, n, 3], &dev),
        Tensor::<B, 3>::zeros([2, n, 1], &dev),
        Tensor::<B, 3>::zeros([2, n, 1], &dev),
        0.0,
    );
    let err = sync_thmc_to_umst(&state, &mut umst).unwrap_err();
    assert!(matches!(err, PhysicsError::InvariantViolation { .. }));
}

#[test]
fn thmc_umst_writeback_roundtrip_after_step() {
    let dev = device();
    let n = 2usize;
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut umst = toy_umst(n, 100.0, 0.4, 0.2);
    let state = ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 305.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.55, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.3, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.15, &dev),
        0.0,
    );
    let mut solver = ThmcSolver {
        dt: 0.001,
        max_newton: 1,
        tol: 1e-6,
        drying_last_node_evaporation_k: 0.0,
        ..Default::default()
    };
    struct Stub;
    impl<Bk: burn::tensor::backend::Backend<FloatElem = f32>>
        umst_manifold::core::traits::IScienceCartridge<Bk> for Stub
    {
        fn compute_all(
            &self,
            mix: &umst_manifold::core::tensors::MaterialCompositionTensor<Bk>,
        ) -> umst_manifold::core::traits::PhysicalResult<Bk> {
            let d = mix.fractions.device();
            umst_manifold::core::traits::PhysicalResult {
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
        fn compute_topology(
            &self,
            m: &UnifiedMaterialStateTensor<Bk>,
        ) -> umst_manifold::core::traits::PhysicalResult<Bk> {
            let nn = m.scalar_features.dims()[0];
            let d = m.scalar_features.device();
            umst_manifold::core::traits::PhysicalResult {
                free_energy: Tensor::zeros([1, nn], &d),
                dissipation: Tensor::zeros([1, nn], &d),
                safety_margin: Tensor::zeros([1, nn], &d),
                cost: Tensor::zeros([1, nn], &d),
                damage: Tensor::zeros([1, nn], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, nn], &d),
            }
        }
    }
    let post = solver
        .step(&Stub, state, &mut umst)
        .expect(
            "ThmcSolver::step on 2-node toy UMST for writeback roundtrip witness (FP §6 Track G THMC UMST writeback parity)",
        );
    let out = umst.scalar_features.clone().into_data().value;
    let t0 = post
        .thermal
        .temperature
        .as_tensor()
        .clone()
        .into_data()
        .value[0];
    let h0 = post.hydro.humidity.as_tensor().clone().into_data().value[0];
    let d0 = post.damage.as_tensor().clone().into_data().value[0];
    assert!((out[SCALAR_TEMPERATURE] - t0).abs() < 1e-4);
    assert!((out[SCALAR_HUMIDITY] - h0).abs() < 1e-4);
    assert!((out[SCALAR_DAMAGE] - d0).abs() < 1e-4);
    assert!(
        (out[f + SCALAR_TEMPERATURE]
            - post
                .thermal
                .temperature
                .as_tensor()
                .clone()
                .into_data()
                .value[1])
            .abs()
            < 1e-4
    );
}

#[test]
fn sync_thmc_idempotent_on_second_sync() {
    let dev = device();
    let n = 2usize;
    let mut umst = toy_umst(n, 300.0, 0.5, 0.1);
    let state = ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 310.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.6, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::zeros([1, n, 1], &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.2, &dev),
        0.0,
    );
    sync_thmc_to_umst(&state, &mut umst).expect(
        "sync_thmc_to_umst first pass on 2-node toy UMST for idempotency baseline (FP §6 Track G THMC UMST writeback parity)",
    );
    let snap = umst.scalar_features.clone().into_data().value;
    sync_thmc_to_umst(&state, &mut umst).expect(
        "sync_thmc_to_umst re-application on unchanged ThmcState for FP §6 idempotency witness (FP §6 Track G THMC UMST writeback parity)",
    );
    let again = umst.scalar_features.clone().into_data().value;
    assert_eq!(snap, again);
}

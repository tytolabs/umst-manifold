// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! RW-FP-PRABHU PB-2 — THMC operator-split step timing on minimal grid.

#![cfg(feature = "thmc-coupled")]

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::physics::solvers::{ThmcSolver, ThmcState};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

struct StubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for StubCartridge {
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

fn toy_umst(n: usize) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[0] = 300.0;
    flat[1] = 0.5;
    flat[2] = 0.1;
    if n > 1 {
        flat[f] = 300.0;
        flat[f + 1] = 0.5;
        flat[f + 2] = 0.1;
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

fn equilibrated_state(n: usize) -> ThmcState<B> {
    let dev = device();
    ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 300.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.5, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::full([1, n, 1], 1.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.1, &dev),
        0.0,
    )
}

fn max_abs_tensor3(a: &Tensor<B, 3>, b: &Tensor<B, 3>) -> f32 {
    let va = a.clone().into_data().value;
    let vb = b.clone().into_data().value;
    va.iter()
        .zip(vb.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

#[test]
fn prabhu_thmc_step_timing() {
    let n = 2usize;
    let mut umst = toy_umst(n);
    let state = equilibrated_state(n);
    let mut solver = ThmcSolver {
        dt: 1e-4,
        max_newton: 1,
        tol: 1e-6,
        drying_last_node_evaporation_k: 0.0,
        ..Default::default()
    };

    // Warm-up step
    let _ = solver
        .step(&StubCartridge, state.clone(), &mut umst)
        .expect(
            "ThmcSolver::step warm-up on minimal toy grid (PB-2 timing harness, FP §6 witness)",
        );

    let state2 = equilibrated_state(n);
    let start = std::time::Instant::now();
    let post = solver
        .step(&StubCartridge, state2, &mut umst)
        .expect("ThmcSolver::step measured pass on equilibrated state (PB-2 thmc_step_ms_per_node witness, FP §6)");
    let elapsed_ms = start.elapsed().as_secs_f64() * 1e3;
    let ms_per_node = elapsed_ms / n as f64;

    // FP §6 idempotency drift check
    let snap = post.clone();
    let post2 = solver
        .step(&StubCartridge, post, &mut umst)
        .expect("ThmcSolver::step re-application on equilibrated post-step state (PB-2 FP §6 idempotency drift check)");
    let drift_tol = 1e-5_f32;
    assert!(
        max_abs_tensor3(
            post2.thermal.temperature.as_tensor(),
            snap.thermal.temperature.as_tensor()
        ) < drift_tol,
        "idempotency drift on temperature"
    );

    println!("thmc_step_ms_per_node={ms_per_node:.6}");
    eprintln!("prabhu_pb2_ok nodes={n} step_ms={elapsed_ms:.6} idempotent=ok");
}

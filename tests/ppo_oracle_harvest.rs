// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Oracle harvest for `golden_learner_burnliquid_ppo_v0` — pinned inputs only.
//!
//! Trajectories cite existing tests:
//! - `liquid_ppo::tests::burn_liquid_ppo_step_finite_backward_chain_smoke`
//! - `epistemic_ppo::histogram_mi_tensor_respects_landauer_ln2_cap`
//! - `epistemic_ppo::manifold_gateway_alpha_beta_gamma_weights`
//! - `epistemic_ppo::nodal_scalar_means_matches_manual_average`

#![cfg(feature = "epistemic-ppo")]

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::info_gain::{
    histogram_info_gain_tensor, nodal_scalar_means, MutualInfoEstimator,
};
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::{SCALAR_INTERNAL_VARIABLE_0, UMST_SCALAR_CHANNEL_COUNT};

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

struct GateAwareCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for GateAwareCartridge {
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

#[must_use]
pub fn harvest_histogram_info_gain_landauer() -> f64 {
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
    t.into_data().value[0] as f64
}

#[must_use]
pub fn harvest_manifold_gateway_rewards() -> (f64, f64) {
    let dev = device();
    let umst = umst_with_hydration(0.4, 2, UMST_SCALAR_CHANNEL_COUNT);
    let info = Tensor::<B, 1>::full([1], 0.001_f32, &dev);

    let mut g_base = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-6_f64);
    g_base.alpha = 1.0;
    g_base.beta = 0.0;
    g_base.gamma = 0.0;
    let r_base = g_base
        .evaluate_topology_step(umst.clone(), info.clone())
        .expect("gateway base reward")
        .1
        .into_data()
        .value[0] as f64;

    let mut g_pen = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-6_f64);
    g_pen.alpha = 1.0;
    g_pen.beta = 10.0;
    g_pen.gamma = 0.0;
    let r_pen = g_pen
        .evaluate_topology_step(umst, info)
        .expect("gateway penalty reward")
        .1
        .into_data()
        .value[0] as f64;

    (r_base, r_pen)
}

#[test]
fn ppo_oracle_harvest_pins_from_existing_tests() {
    let info_gain = harvest_histogram_info_gain_landauer();
    let (r_base, r_pen) = harvest_manifold_gateway_rewards();
    let dev = device();
    let t = Tensor::<B, 2>::from_data(
        Data::new(vec![1.0_f32, 3.0_f32, 5.0_f32, 7.0_f32], Shape::new([2, 2])),
        &dev,
    );
    let m = nodal_scalar_means(&t, 2);

    eprintln!("HARVEST histogram_info_gain_bits={info_gain:.17e}");
    eprintln!("HARVEST manifold_gateway_r_base={r_base:.17e}");
    eprintln!("HARVEST manifold_gateway_r_pen={r_pen:.17e}");
    eprintln!("HARVEST nodal_scalar_means=[{}, {}]", m[0], m[1]);

    assert!((info_gain - 0.6931471805599453).abs() < 1e-12);
    assert!((r_base - (-5.741957770157447e-24)).abs() < 1e-30);
    assert!((r_pen - (-2.5000000000000044)).abs() < 1e-12);
    assert!((m[0] - 3.0).abs() < 1e-12);
    assert!((m[1] - 5.0).abs() < 1e-12);
}

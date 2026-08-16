// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Integration test: [`ManifoldGateway::evaluate_topology_step`] with a stub cartridge and a finite
//! [`suggested_info_gain_from_batched_nodal_scalars`] tensor feeding the Landauer branch (same
//! surrogate family as [`umst_manifold::ai::info_gain::suggested_info_gain_from_state_delta`]).

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::info_gain::suggested_info_gain_from_batched_nodal_scalars;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
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

/// Minimal cartridge: finite nodal [`PhysicalResult`] (zeros).
struct GatewayStubCartridge;

impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for GatewayStubCartridge {
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
fn manifold_gateway_accepts_step_with_suggested_info_gain() {
    let baseline = tiny_umst();
    let mut proposed = tiny_umst();
    proposed.scalar_features = proposed.scalar_features.clone().add_scalar(0.1_f32);

    let baseline_batched = baseline.scalar_features.clone().unsqueeze_dim::<3>(0);
    let proposed_batched = proposed.scalar_features.clone().unsqueeze_dim::<3>(0);
    let info_gain =
        suggested_info_gain_from_batched_nodal_scalars(baseline_batched, proposed_batched);
    assert_eq!(info_gain.dims(), [1]);

    // Enough Landauer budget for the summed surrogate bits (same scale as `tests/cbf.rs`).
    let mut gateway = ManifoldGateway::new(GatewayStubCartridge, 300.0_f64, 1.0e-12_f64);
    let result = gateway.evaluate_topology_step(proposed, info_gain);
    assert!(result.is_ok(), "expected Ok, got {:?}", result.err());

    let (_verified, reward) = result.expect(
        "ManifoldGateway::evaluate_topology_step with suggested_info_gain_from_batched_nodal_scalars surrogate (FP §6 Track G epistemic sensor harness)",
    );
    let rv: Vec<f32> = reward.into_data().value;
    assert!(rv[0].is_finite(), "reward should be finite");
}

/// `information_density` reward term: **η · mean(information_density)** (see `ManifoldGateway::eta`).
#[cfg(feature = "information_density")]
mod information_density_reward {
    use super::*;
    use approx::assert_abs_diff_eq;

    const INFO_FILL: f32 = 3.0_f32;

    struct StubConstInfo;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for StubConstInfo {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::zeros([1, 1], &d),
                dissipation: Tensor::zeros([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::zeros([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                information_density: Tensor::zeros([1, 1], &d).add_scalar(INFO_FILL),
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
                information_density: Tensor::zeros([1, n], &d).add_scalar(INFO_FILL),
            }
        }
    }

    #[test]
    fn manifold_gateway_eta_matches_mean_information_density() {
        let proposed = tiny_umst();
        let baseline = tiny_umst();
        let baseline_batched = baseline.scalar_features.clone().unsqueeze_dim::<3>(0);
        let proposed_batched = proposed.scalar_features.clone().unsqueeze_dim::<3>(0);
        let info_gain =
            suggested_info_gain_from_batched_nodal_scalars(baseline_batched, proposed_batched);

        let mut g0 = ManifoldGateway::new(StubConstInfo, 300.0_f64, 1.0e-12_f64);
        g0.eta = 0.0_f32;
        let r0 = g0
            .evaluate_topology_step(proposed.clone(), info_gain.clone())
            .expect(
                "ManifoldGateway topology step with eta=0 on StubConstInfo cartridge for information_density reward baseline (FP §6 Track G epistemic sensor harness)",
            )
            .1
            .into_data()
            .value[0];

        let mut g1 = ManifoldGateway::new(StubConstInfo, 300.0_f64, 1.0e-12_f64);
        let eta = 2.0_f32;
        g1.eta = eta;
        let r1 = g1
            .evaluate_topology_step(proposed, info_gain)
            .expect(
                "ManifoldGateway topology step with eta=2 on StubConstInfo cartridge for information_density reward scaling witness (FP §6 Track G epistemic sensor harness)",
            )
            .1
            .into_data()
            .value[0];

        let expected_delta = eta * INFO_FILL;
        assert_abs_diff_eq!(r1 - r0, expected_delta, epsilon = 1.0e-4_f32);
    }
}

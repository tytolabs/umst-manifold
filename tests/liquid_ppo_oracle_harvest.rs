// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Oracle harvest for `golden_learner_burnliquid_ppo_v0` — pinned inputs only.
//!
//! Trajectory cites existing unit test:
//! - `liquid_ppo::tests::burn_liquid_ppo_step_finite_backward_chain_smoke`

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::liquid_ppo::BurnLiquidPPOAgent;
use umst_manifold::ai::ppo::ManifoldGateway;
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

type B = NdArray<f32>;

const GATEWAY_TEMPERATURE_K: f64 = 300.0;
const GATEWAY_INITIAL_CREDIT_J: f64 = 1.0e-12;
const INFO_GAIN_BITS: f32 = 0.01_f32;
const DT_RATIO: f32 = 1.0_f32;
const T_START: f32 = 0.0_f32;
const T_END: f32 = 1.0_f32;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

/// Two-node `tiny_umst` — verbatim from `liquid_ppo::tests`.
#[must_use]
pub fn harvest_tiny_umst() -> UnifiedMaterialStateTensor<B> {
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

/// Harvest oracle: `BurnLiquidPPOAgent::step_and_learn` smoke chain.
#[must_use]
pub fn harvest_burnliquid_ppo_policy_weight_delta() -> (f32, f32) {
    let dev = device();
    let gateway = ManifoldGateway::new(
        PpoChainStubCartridge,
        GATEWAY_TEMPERATURE_K,
        GATEWAY_INITIAL_CREDIT_J,
    );
    let mut agent = BurnLiquidPPOAgent::new(gateway);
    let state = harvest_tiny_umst();
    let info = Tensor::<B, 1>::full([1], INFO_GAIN_BITS, &dev);
    let dt_rat = Tensor::<B, 1>::full([1], DT_RATIO, &dev);
    let w0 = agent.ode_solver.policy_weights.clone().into_data().value[0];
    agent
        .step_and_learn(state, T_START, T_END, info, dt_rat)
        .expect("BurnLiquidPPOAgent::step_and_learn smoke oracle harvest");
    let w1 = agent.ode_solver.policy_weights.clone().into_data().value[0];
    (w0, w1)
}

fn f32_hex(bits: f32) -> String {
    format!("{:016x}", bits.to_bits())
}

#[test]
fn liquid_ppo_oracle_harvest_pins_from_backward_chain_smoke() {
    let (w0, w1) = harvest_burnliquid_ppo_policy_weight_delta();
    eprintln!("HARVEST burnliquid_ppo policy_weight_w0={w0:.17e} policy_weight_w1={w1:.17e}");
    eprintln!(
        "HARVEST_HEX policy_weight_w0={} policy_weight_w1={}",
        f32_hex(w0),
        f32_hex(w1)
    );
    assert!(w0.is_finite() && w1.is_finite());
    assert_ne!(w0, w1, "AdamW should move policy_weights after finite backward surrogate");
    // Pin vector 1 of `golden_learner_burnliquid_ppo_v0` (oracle-run harvest @ 2026-07-22).
    assert_eq!(w0, 0.0_f32);
    assert_eq!(w1, 1.681_801_4e-24_f32);
    assert_eq!(f32_hex(w0), "0000000000000000");
    assert_eq!(f32_hex(w1), "0000000018021f82");
}

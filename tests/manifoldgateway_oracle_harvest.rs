// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Oracle harvest for `golden_learner_manifoldgateway_v0` — pinned inputs only.
//!
//! Trajectory cites existing test:
//! - `gateway_info_gain.rs::manifold_gateway_accepts_step_with_suggested_info_gain`
//!
//! Authority: [`FIXTURE_LEARNER_MANIFOLDGATEWAY_EMIT_GAP_RECEIPT.md`](../../outputs/staged-web/swarm/FIXTURE_LEARNER_MANIFOLDGATEWAY_EMIT_GAP_RECEIPT.md)
//! · [`MF_LO_WLEARN_EMIT_SCHEDULE.md`](../../outputs/staged-web/swarm/MF_LO_WLEARN_EMIT_SCHEDULE.md) step 3.

use burn::tensor::backend::Backend;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::gate::{suggested_info_gain_from_batched_nodal_scalars, GateManifoldGateway};

type B = NdArray<f32>;

const GATEWAY_TEMPERATURE_K: f64 = 300.0;
const GATEWAY_INITIAL_CREDIT_J: f64 = 1.0e-12;
const SCALAR_DELTA: f32 = 0.1_f32;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

/// Two-node `tiny_umst` — verbatim from `gateway_info_gain.rs`.
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

/// Minimal cartridge: finite nodal [`PhysicalResult`] (zeros) — verbatim from `gateway_info_gain.rs`.
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

/// Harvest oracle: `GateManifoldGateway::evaluate_topology_step` on smoke-1 vector.
///
/// Pins: `tiny_umst` baseline/proposed (+0.1 scalar delta) + `suggested_info_gain_from_batched_nodal_scalars`.
#[must_use]
pub fn harvest_smoke1_suggested_info_gain() -> (f32, f32) {
    let baseline = harvest_tiny_umst();
    let mut proposed = harvest_tiny_umst();
    proposed.scalar_features = proposed.scalar_features.clone().add_scalar(SCALAR_DELTA);

    let baseline_batched = baseline.scalar_features.clone().unsqueeze_dim::<3>(0);
    let proposed_batched = proposed.scalar_features.clone().unsqueeze_dim::<3>(0);
    let info_gain =
        suggested_info_gain_from_batched_nodal_scalars(baseline_batched, proposed_batched);
    let info_gain_bits: f32 = info_gain.clone().into_data().value[0];

    let mut gateway = GateManifoldGateway::new(
        GatewayStubCartridge,
        GATEWAY_TEMPERATURE_K,
        GATEWAY_INITIAL_CREDIT_J,
    );
    let result = gateway
        .evaluate_topology_step(proposed, info_gain)
        .expect("GateManifoldGateway::evaluate_topology_step smoke-1 oracle harvest");

    let reward_scalar: f32 = result.1.into_data().value[0];
    (info_gain_bits, reward_scalar)
}

fn f32_hex(bits: f32) -> String {
    format!("{:016x}", bits.to_bits())
}

#[test]
fn manifoldgateway_oracle_harvest_pins_from_gateway_info_gain() {
    let (info_gain_bits, reward_scalar) = harvest_smoke1_suggested_info_gain();

    eprintln!(
        "HARVEST smoke1_suggested_info_gain info_gain_bits={info_gain_bits:.17e} reward_scalar={reward_scalar:.17e}"
    );
    eprintln!(
        "HARVEST_HEX info_gain_bits={} reward_scalar={}",
        f32_hex(info_gain_bits),
        f32_hex(reward_scalar)
    );

    let harvest_json = format!(
        r#"{{"info_gain_bits":{info_gain_bits:.17},"reward_scalar":{reward_scalar:.17},"info_gain_bits_hex":"{}","reward_scalar_hex":"{}"}}"#,
        f32_hex(info_gain_bits),
        f32_hex(reward_scalar),
    );
    eprintln!("HARVEST_JSON {harvest_json}");

    assert!(info_gain_bits.is_finite() && info_gain_bits > 0.0);
    assert!(reward_scalar.is_finite());
    assert!(
        reward_scalar < 0.0,
        "Landauer erasure subtracts from zero physics reward"
    );
}

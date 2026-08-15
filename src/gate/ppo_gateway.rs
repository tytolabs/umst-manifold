// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thin gate-layer alias around [`crate::ai::ppo::ManifoldGateway`] (no duplicated physics).
//!
//! Composition:
//! - **PPO-GW-001** · policy-facing IO barrier @ [`crate::ai::ppo::ManifoldGateway`]
//! - **PPO-GW-002** · thermodynamic CBF credit pinned via [`crate::ai::cbf::ThermodynamicCBF`]
//! - **PPO-GW-003** · topology step witness @ `evaluate_topology_step` (delegates to inner)

use crate::ai::ppo::ManifoldGateway as ManifoldGatewayInner;
use crate::core::traits::IScienceCartridge;
use burn::tensor::backend::Backend;

/// PORT_GRAIN_BAND id @ [`super::GATE_MODULE_BANDS`].
pub const PPO_GATEWAY_BAND_ID: &str = "gate:ppo_gateway";

/// PORT-MF-PPO-GATEWAY-W2 cell id (wave-2 gate band deepen).
pub const PPO_GATEWAY_CELL_ID: &str = "PORT-MF-PPO-GATEWAY-W2";

/// Honest posture — thin alias only; tests deepen wrapper contract (`MASTER_RETICK=no`).
pub const PPO_GATEWAY_POSTURE_TAG: &str = "honest-manifold-gateway-alias-only";

/// Inner morphism SSOT @ [`crate::ai::ppo::ManifoldGateway`].
pub const PPO_GATEWAY_INNER_MORPHISM: &str = "ManifoldGateway";

pub struct GateManifoldGateway<B: Backend, C: IScienceCartridge<B>>(pub ManifoldGatewayInner<B, C>);

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> GateManifoldGateway<B, C> {
    pub fn new(cartridge: C, temperature_k: f64, initial_credit_joules: f64) -> Self {
        Self(ManifoldGatewayInner::new(
            cartridge,
            temperature_k,
            initial_credit_joules,
        ))
    }

    pub fn into_inner(self) -> ManifoldGatewayInner<B, C> {
        self.0
    }
}

impl<B: Backend, C: IScienceCartridge<B>> std::ops::Deref for GateManifoldGateway<B, C> {
    type Target = ManifoldGatewayInner<B, C>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Backend, C: IScienceCartridge<B>> std::ops::DerefMut for GateManifoldGateway<B, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Whether the PPO gateway alias metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn ppo_gateway_morphism_pinned() -> bool {
    PPO_GATEWAY_BAND_ID == "gate:ppo_gateway"
        && PPO_GATEWAY_CELL_ID == "PORT-MF-PPO-GATEWAY-W2"
        && PPO_GATEWAY_POSTURE_TAG == "honest-manifold-gateway-alias-only"
        && PPO_GATEWAY_INNER_MORPHISM == "ManifoldGateway"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use approx::assert_relative_eq;
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    const GATEWAY_TEMP_K: f64 = 300.0;
    const GATEWAY_CREDIT_J: f64 = 1.0e-12;
    const SCALAR_DELTA: f32 = 0.1_f32;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    /// Two-node `tiny_umst` — verbatim from [`crate::ai::liquid_ppo::tests`].
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

    struct PpoGatewayStubCartridge;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoGatewayStubCartridge {
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

    fn gateway() -> GateManifoldGateway<B, PpoGatewayStubCartridge> {
        GateManifoldGateway::new(PpoGatewayStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J)
    }

    fn inner_gateway() -> ManifoldGateway<B, PpoGatewayStubCartridge> {
        ManifoldGateway::new(PpoGatewayStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J)
    }

    fn info_gain_tensor() -> Tensor<B, 1> {
        Tensor::<B, 1>::full([1], SCALAR_DELTA, &device())
    }

    #[test]
    fn gate_ppo_gateway_doc_cites_manifold_gateway_and_cbf() {
        let doc = include_str!("ppo_gateway.rs");
        assert!(doc.contains("ManifoldGateway"));
        assert!(doc.contains("ThermodynamicCBF"));
        assert!(doc.contains("PPO-GW-001"));
        assert!(doc.contains("evaluate_topology_step"));
    }

    #[test]
    fn gate_ppo_gateway_morphism_metadata_pinned() {
        assert!(ppo_gateway_morphism_pinned());
        assert_eq!(PPO_GATEWAY_BAND_ID, "gate:ppo_gateway");
        assert_eq!(PPO_GATEWAY_CELL_ID, "PORT-MF-PPO-GATEWAY-W2");
        assert_eq!(PPO_GATEWAY_INNER_MORPHISM, "ManifoldGateway");
    }

    #[test]
    fn gate_ppo_gateway_new_pins_temperature_and_credit_via_cbf() {
        let gate = gateway();
        assert_relative_eq!(gate.cbf.temperature_k, GATEWAY_TEMP_K, epsilon = 1.0e-9);
        assert_relative_eq!(
            gate.cbf.available_credit_joules,
            GATEWAY_CREDIT_J,
            epsilon = 1.0e-18
        );
    }

    #[test]
    fn gate_ppo_gateway_new_matches_inner_manifold_gateway() {
        let gate = gateway();
        let inner = inner_gateway();
        assert_relative_eq!(
            gate.cbf.temperature_k,
            inner.cbf.temperature_k,
            epsilon = 1.0e-9
        );
        assert_relative_eq!(
            gate.cbf.available_credit_joules,
            inner.cbf.available_credit_joules,
            epsilon = 1.0e-18
        );
        assert_relative_eq!(
            f64::from(gate.alpha),
            f64::from(inner.alpha),
            epsilon = 1.0e-6
        );
        assert_relative_eq!(
            f64::from(gate.beta),
            f64::from(inner.beta),
            epsilon = 1.0e-6
        );
        assert_relative_eq!(
            f64::from(gate.gamma),
            f64::from(inner.gamma),
            epsilon = 1.0e-6
        );
        assert_relative_eq!(
            f64::from(gate.zeta),
            f64::from(inner.zeta),
            epsilon = 1.0e-6
        );
        assert_relative_eq!(f64::from(gate.eta), f64::from(inner.eta), epsilon = 1.0e-6);
    }

    #[test]
    fn gate_ppo_gateway_default_reward_weights_pinned() {
        let gate = gateway();
        assert_relative_eq!(f64::from(gate.alpha), 1.0, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gate.beta), 0.5, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gate.gamma), 2.0, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gate.zeta), 0.0, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gate.eta), 0.0, epsilon = 1.0e-6);
    }

    #[test]
    fn gate_ppo_gateway_into_inner_preserves_cbf_state() {
        let gate = gateway();
        let temp = gate.cbf.temperature_k;
        let credit = gate.cbf.available_credit_joules;
        let inner = gate.into_inner();
        assert_relative_eq!(inner.cbf.temperature_k, temp, epsilon = 1.0e-9);
        assert_relative_eq!(inner.cbf.available_credit_joules, credit, epsilon = 1.0e-18);
    }

    #[test]
    fn gate_ppo_gateway_deref_exposes_cartridge_and_telemetry() {
        let gate = gateway();
        let _ = &gate.cartridge;
        assert_relative_eq!(gate.telemetry().acceptance_rate(), 0.0, epsilon = 1.0e-30);
        assert_relative_eq!(gate.telemetry().rejection_rate(), 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn gate_ppo_gateway_deref_mut_allows_reward_weight_mutation() {
        let mut gate = gateway();
        gate.zeta = 0.25_f32;
        gate.alpha = 1.5_f32;
        assert_relative_eq!(f64::from(gate.zeta), 0.25, epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gate.alpha), 1.5, epsilon = 1.0e-6);
    }

    #[test]
    fn gate_ppo_gateway_evaluate_topology_step_smoke_accepts_tiny_umst() {
        let mut gate = gateway();
        let state = tiny_umst();
        let info = info_gain_tensor();
        let out = gate.evaluate_topology_step(state, info);
        assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
        let (verified, reward) = out.expect("topology step");
        assert_eq!(verified.state.scalar_features.dims()[0], 2);
        let reward_v: Vec<f32> = reward.into_data().value;
        assert_eq!(reward_v.len(), 1);
        assert!(reward_v[0].is_finite());
    }

    #[test]
    fn gate_ppo_gateway_evaluate_matches_inner_on_same_inputs() {
        let mut gate = gateway();
        let mut inner = inner_gateway();
        let state_gate = tiny_umst();
        let state_inner = tiny_umst();
        let info = info_gain_tensor();
        let gate_out = gate
            .evaluate_topology_step(state_gate, info.clone())
            .expect("gate topology step");
        let inner_out = inner
            .evaluate_topology_step(state_inner, info)
            .expect("inner topology step");
        let gate_reward: Vec<f32> = gate_out.1.into_data().value;
        let inner_reward: Vec<f32> = inner_out.1.into_data().value;
        assert_eq!(gate_reward.len(), inner_reward.len());
        for (g, i) in gate_reward.iter().zip(inner_reward.iter()) {
            assert_relative_eq!(f64::from(*g), f64::from(*i), epsilon = 1.0e-5);
        }
    }

    #[test]
    fn gate_ppo_gateway_topology_step_deducts_credit_monotonically() {
        let mut gate = gateway();
        let credit0 = gate.cbf.available_credit_joules;
        let state = tiny_umst();
        let info = info_gain_tensor();
        gate.evaluate_topology_step(state, info)
            .expect("first topology step");
        let credit1 = gate.cbf.available_credit_joules;
        assert!(
            credit1 <= credit0,
            "CBF credit must not increase after admissible step: {credit0} -> {credit1}"
        );
    }

    #[test]
    fn gate_ppo_gateway_wrapper_is_transparent_newtype() {
        let gate = gateway();
        assert_relative_eq!(gate.0.cbf.temperature_k, GATEWAY_TEMP_K, epsilon = 1.0e-9);
        assert_relative_eq!(
            gate.0.cbf.available_credit_joules,
            GATEWAY_CREDIT_J,
            epsilon = 1.0e-18
        );
    }

    #[test]
    fn w8e14_gate_ppo_gateway_posture_tag_honest_not_green() {
        assert!(ppo_gateway_morphism_pinned());
        assert!(PPO_GATEWAY_POSTURE_TAG.contains("honest"));
        assert!(!PPO_GATEWAY_POSTURE_TAG
            .to_ascii_lowercase()
            .contains("green"));
    }
}

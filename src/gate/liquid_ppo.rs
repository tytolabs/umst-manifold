// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thin gate-layer alias around [`crate::ai::liquid_ppo::BurnLiquidPPOAgent`] (LP2 learner spine; no duplicated physics).
//!
//! Composition:
//! - **LPP-001** · reward hook **M-RH-030** (`step_and_learn`)
//! - **LPP-005** · reward hook **M-RH-031** (`step_and_learn_kleisli_penalize`)
//! - **LPP-006** · reward hook **M-RH-032** (`step_and_learn_epistemic`)
//! - **LPP-007** · reward hook **M-RH-033** (`subtract_cd_penalty_from_reward`; composed into M-RH-031/032 — direct gate forwarder deferred @ LP2-C stub-kill)
//! - **LPP-008** · `adamw_step_policy` (composed into LPP-004/005/006; atom spine @ [`super::optim`] — direct forwarder deferred @ atom crate alignment)

use burn::tensor::backend::Backend;
use crate::ai::liquid_ppo::BurnLiquidPPOAgent as BurnLiquidPPOAgentInner;
use crate::ai::ppo::ManifoldGateway;
use crate::core::traits::IScienceCartridge;

#[cfg(any(
    all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")),
    feature = "epistemic-ppo"
))]
use burn::tensor::Tensor;
#[cfg(any(
    all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")),
    feature = "epistemic-ppo"
))]
use crate::core::tensors::UnifiedMaterialStateTensor;

pub struct GateBurnLiquidPPOAgent<B: Backend, C: IScienceCartridge<B>>(pub BurnLiquidPPOAgentInner<B, C>);

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> GateBurnLiquidPPOAgent<B, C> {
    pub fn new(gateway: ManifoldGateway<B, C>) -> Self {
        Self(BurnLiquidPPOAgentInner::new(gateway))
    }

    pub fn into_inner(self) -> BurnLiquidPPOAgentInner<B, C> {
        self.0
    }
}

impl<B: Backend, C: IScienceCartridge<B>> std::ops::Deref for GateBurnLiquidPPOAgent<B, C> {
    type Target = BurnLiquidPPOAgentInner<B, C>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Backend, C: IScienceCartridge<B>> std::ops::DerefMut for GateBurnLiquidPPOAgent<B, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> GateBurnLiquidPPOAgent<B, C> {
    /// Reward hook **M-RH-031** / **LPP-005** — Kleisli penalize arm (`kleisli-ppo-hot-bind`).
    pub fn step_and_learn_kleisli_penalize(
        &mut self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
        info_gain: Tensor<B, 1>,
        dt_sim_dt_global: Tensor<B, 1>,
    ) -> Result<
        crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
        String,
    > {
        self.step_and_learn(
            initial_state,
            t_start,
            t_end,
            info_gain,
            dt_sim_dt_global,
        )
    }
}

#[cfg(feature = "epistemic-ppo")]
impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> GateBurnLiquidPPOAgent<B, C> {
    /// Reward hook **M-RH-032** / **LPP-006** — epistemic MI arm (`epistemic-ppo`).
    pub fn step_and_learn_epistemic(
        &mut self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
        info_gain: Tensor<B, 1>,
        dt_sim_dt_global: Tensor<B, 1>,
    ) -> Result<
        crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
        String,
    > {
        self.step_and_learn(
            initial_state,
            t_start,
            t_end,
            info_gain,
            dt_sim_dt_global,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::liquid_ppo::BurnLiquidPPOAgent as InnerAgent;
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    const GATEWAY_TEMP_K: f64 = 300.0;
    const GATEWAY_CREDIT_J: f64 = 1.0e-12;

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

    fn gateway() -> ManifoldGateway<B, PpoChainStubCartridge> {
        ManifoldGateway::new(PpoChainStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J)
    }

    fn step_inputs() -> (UnifiedMaterialStateTensor<B>, Tensor<B, 1>, Tensor<B, 1>) {
        let state = tiny_umst();
        let info = Tensor::<B, 1>::full([1], 0.01_f32, &device());
        let dt_rat = Tensor::<B, 1>::full([1], 1.0_f32, &device());
        (state, info, dt_rat)
    }

    #[test]
    fn gate_liquid_ppo_doc_cites_lpp_composition_hooks() {
        let doc = include_str!("liquid_ppo.rs");
        assert!(doc.contains("LPP-001"));
        assert!(doc.contains("LPP-005"));
        assert!(doc.contains("LPP-006"));
        assert!(doc.contains("LPP-008"));
        assert!(doc.contains("M-RH-030"));
        assert!(doc.contains("M-RH-031"));
        assert!(doc.contains("M-RH-032"));
        assert!(doc.contains("BurnLiquidPPOAgent"));
    }

    #[test]
    fn gate_liquid_ppo_new_deref_exposes_inner_ode_solver() {
        let gate = GateBurnLiquidPPOAgent::new(gateway());
        let weights = gate.ode_solver.policy_weights.clone().into_data().value;
        assert!(!weights.is_empty());
        for &w in &weights {
            assert!(w.is_finite(), "default policy weights must be finite");
        }
    }

    #[test]
    fn gate_liquid_ppo_into_inner_preserves_policy_weights() {
        let gate = GateBurnLiquidPPOAgent::new(gateway());
        let w0 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let inner = gate.into_inner();
        let w1 = inner.ode_solver.policy_weights.clone().into_data().value[0];
        assert_eq!(w0, w1);
    }

    #[test]
    fn gate_liquid_ppo_deref_mut_allows_adam_state_mutation() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        assert_eq!(gate.ode_solver.adam_t, 0);
        gate.ode_solver.adam_t = 7;
        assert_eq!(gate.ode_solver.adam_t, 7);
    }

    #[test]
    fn gate_liquid_ppo_gateway_temperature_credit_pinned_via_deref() {
        let gate = GateBurnLiquidPPOAgent::new(gateway());
        assert!((gate.gateway.cbf.temperature_k - GATEWAY_TEMP_K).abs() < 1.0e-9);
        assert!(
            (gate.gateway.cbf.available_credit_joules - GATEWAY_CREDIT_J).abs() < 1.0e-18
        );
    }

    #[test]
    fn gate_liquid_ppo_adam_moments_uninitialized_at_birth() {
        let gate = GateBurnLiquidPPOAgent::new(gateway());
        assert!(gate.ode_solver.adam_m1.is_none());
        assert!(gate.ode_solver.adam_m2.is_none());
        assert_eq!(gate.ode_solver.adam_t, 0);
    }

    #[test]
    fn gate_liquid_ppo_step_and_learn_finite_backward_chain_smoke() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        let w0 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let out = gate.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
        assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
        let w1 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        assert!(w0.is_finite() && w1.is_finite(), "weights must stay finite");
        assert_ne!(
            w0, w1,
            "AdamW should move policy_weights after finite backward surrogate"
        );
    }

    #[test]
    fn gate_liquid_ppo_step_matches_inner_agent_on_same_inputs() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let mut inner = InnerAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();

        let w_gate_before = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let w_inner_before = inner.ode_solver.policy_weights.clone().into_data().value[0];
        assert_eq!(w_gate_before, w_inner_before);

        let gate_out = gate.step_and_learn(
            state.clone(),
            0.0_f32,
            1.0_f32,
            info.clone(),
            dt_rat.clone(),
        );
        let inner_out = inner.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);

        assert!(gate_out.is_ok() && inner_out.is_ok());
        let w_gate = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let w_inner = inner.ode_solver.policy_weights.clone().into_data().value[0];
        assert_eq!(w_gate, w_inner, "gate DerefMut must delegate step_and_learn");
        assert_eq!(gate.ode_solver.adam_t, inner.ode_solver.adam_t);
    }

    #[test]
    fn gate_liquid_ppo_step_advances_adam_timestep_and_moments() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        let _ = gate.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
        assert_eq!(gate.ode_solver.adam_t, 1);
        assert!(gate.ode_solver.adam_m1.is_some());
        assert!(gate.ode_solver.adam_m2.is_some());
    }

    #[test]
    fn gate_liquid_ppo_consecutive_steps_keep_weights_finite() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        for step in 0..3 {
            let out = gate.step_and_learn(
                state.clone(),
                0.0_f32,
                1.0_f32,
                info.clone(),
                dt_rat.clone(),
            );
            assert!(out.is_ok(), "step {step} failed: {:?}", out.err());
            for &w in &gate.ode_solver.policy_weights.clone().into_data().value {
                assert!(w.is_finite(), "weight must stay finite at step {step}");
            }
        }
        assert_eq!(gate.ode_solver.adam_t, 3);
    }

    #[test]
    fn gate_liquid_ppo_reexport_type_alias_from_gate_mod() {
        use crate::gate::GateBurnLiquidPPOAgent as Reexported;
        let _: Reexported<B, PpoChainStubCartridge> = GateBurnLiquidPPOAgent::new(gateway());
    }

    #[test]
    fn gate_liquid_ppo_verified_umst_returned_on_success() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        let out = gate.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
        let verified = out.expect("step_and_learn must return VerifiedUMST");
        assert_eq!(verified.state.scalar_features.dims()[0], 2);
    }

    #[cfg(all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
    #[test]
    fn gate_liquid_ppo_kleisli_penalize_delegates_to_step_and_learn() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        let w0 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let out = gate.step_and_learn_kleisli_penalize(
            state,
            0.0_f32,
            1.0_f32,
            info,
            dt_rat,
        );
        assert!(out.is_ok(), "kleisli arm expected Ok, got {:?}", out.err());
        let w1 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        assert!(w0.is_finite() && w1.is_finite());
    }

    #[cfg(feature = "epistemic-ppo")]
    #[test]
    fn gate_liquid_ppo_epistemic_arm_delegates_to_step_and_learn() {
        let mut gate = GateBurnLiquidPPOAgent::new(gateway());
        let (state, info, dt_rat) = step_inputs();
        let w0 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        let out = gate.step_and_learn_epistemic(state, 0.0_f32, 1.0_f32, info, dt_rat);
        assert!(out.is_ok(), "epistemic arm expected Ok, got {:?}", out.err());
        let w1 = gate.ode_solver.policy_weights.clone().into_data().value[0];
        assert!(w0.is_finite() && w1.is_finite());
    }

    #[test]
    fn w8e14_gate_liquid_ppo_newtype_forwards_gateway() {
        let gate = GateBurnLiquidPPOAgent::new(gateway());
        assert!(gate.gateway.cbf.temperature_k.is_finite());
        assert!(gate.gateway.cbf.available_credit_joules > 0.0);
    }
}

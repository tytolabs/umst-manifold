// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Liquid Time-Constant (LTC) / Neural ODE integration for PPO.
//!
//! Replaces the standard discrete MLP path with a continuous-time differential equation solver.
//! Instead of mapping Action = f(State), it solves:
//! d(Action)/dt = f_theta(Action, State)
//! This matches the continuous rheological evolution of printing concrete.
//!
//! Policy weights use **AdamW**-style tensor updates (Burn-default \(\beta_1,\beta_2,\varepsilon\), weight decay)
//! so [`AdjointNeuralODE::backward_adjoint`] gradients flow into [`AdjointNeuralODE::policy_weights`] without
//! `.into_scalar()` / `.into_data()` on the hot path. A separate `burn::module::Module` plus `burn::optim::Optimizer`
//! path over the same flattened weights is intentionally **not** supported here (see [`crate::ai::adjoint`] F1.4).
//!
//! **F1.4:** The supported optimization surface is **AdamW on the flat `policy_weights` vector** paired
//! with [`AdjointNeuralODE`], not a separate Burn `Module` + `burn::optim::Optimizer` over the same
//! parameters (see module notes in [`crate::ai::adjoint`]).

use crate::ai::adjoint::AdjointNeuralODE;
use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use burn::tensor::{backend::Backend, Tensor};

#[cfg(feature = "epistemic-ppo")]
use crate::ai::info_gain::{
    histogram_info_gain_tensor, nodal_scalar_means, EpistemicStateTracker, MutualInfoEstimator,
};

/// The Burn-Native Liquid PPO Agent.
/// Drives the material state forward in continuous time using the Adjoint Method,
/// and validates the results against the Thermodynamic Manifold Gateway.
pub struct BurnLiquidPPOAgent<B: Backend, C: IScienceCartridge<B>> {
    pub ode_solver: AdjointNeuralODE<B>,
    pub gateway: ManifoldGateway<B, C>,
    #[cfg(feature = "epistemic-ppo")]
    pub mi_estimator: MutualInfoEstimator,
    #[cfg(feature = "epistemic-ppo")]
    pub epistemic_tracker: EpistemicStateTracker,
}

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> BurnLiquidPPOAgent<B, C> {
    pub fn new(gateway: ManifoldGateway<B, C>) -> Self {
        Self {
            ode_solver: AdjointNeuralODE::default(),
            gateway,
            #[cfg(feature = "epistemic-ppo")]
            mi_estimator: MutualInfoEstimator::for_material_proxy(),
            #[cfg(feature = "epistemic-ppo")]
            epistemic_tracker: EpistemicStateTracker::new(),
        }
    }

    /// Primary execution step: The Brain wires into the Body.
    /// 1. Solves the Neural ODE forward to get the proposed topology.
    /// 2. Passes the topology through the Thermodynamic Gateway.
    /// 3. Backpropagates the spatial reward using the O(1) memory Adjoint backward pass.
    ///
    /// With **`epistemic-ppo`**, ignores the caller `info_gain` and derives histogram MI from the
    /// baseline→proposed scalar transition (R2/CBF envelope); epistemic bonus is added post-CBF.
    pub fn step_and_learn(
        &mut self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
        info_gain: Tensor<B, 1>,
        dt_sim_dt_global: Tensor<B, 1>, // Differentiable time dilation tensor
    ) -> Result<
        crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
        String,
    > {
        #[cfg(feature = "epistemic-ppo")]
        {
            let _ = info_gain;
            return self.step_and_learn_epistemic(initial_state, t_start, t_end, dt_sim_dt_global);
        }

        #[cfg(not(feature = "epistemic-ppo"))]
        {
            self.step_and_learn_stub(initial_state, t_start, t_end, info_gain, dt_sim_dt_global)
        }
    }

    #[cfg(not(feature = "epistemic-ppo"))]
    fn step_and_learn_stub(
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
        // 1. FORWARD PASS: Solve Neural ODE without storing intermediate states
        let proposed_topology = self.ode_solver.forward(initial_state, t_start, t_end);

        // 2. THERMODYNAMIC GATE: Wire the Brain to the Physics Body
        match self
            .gateway
            .evaluate_topology_step(proposed_topology, info_gain)
        {
            Ok((verified_state, spatial_reward)) => {
                // 3. BACKWARD PASS: Optimize the Neural ODE weights using the Adjoint State Method
                // Note: final_state is safe to extract because the VerifiedUMST guarantees it's valid.
                let final_state_raw = verified_state.state.clone();

                let gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward, // Target from the physical thermodynamic outcome
                    t_start,
                    t_end,
                    dt_sim_dt_global, // Time Dilation optimization
                );

                const ADAM_LR: f32 = 1e-3;
                let (w_new, m1, m2, t) = adamw_step_policy(
                    self.ode_solver.policy_weights.clone(),
                    gradients,
                    ADAM_LR,
                    self.ode_solver.adam_m1.take(),
                    self.ode_solver.adam_m2.take(),
                    self.ode_solver.adam_t,
                );
                self.ode_solver.policy_weights = w_new;
                self.ode_solver.adam_m1 = Some(m1);
                self.ode_solver.adam_m2 = Some(m2);
                self.ode_solver.adam_t = t;

                Ok(verified_state)
            }
            Err(e) => {
                // The AI proposed a topology that broke the 2nd Law of Thermodynamics.
                Err(e)
            }
        }
    }

    #[cfg(feature = "epistemic-ppo")]
    fn step_and_learn_epistemic(
        &mut self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
        dt_sim_dt_global: Tensor<B, 1>,
    ) -> Result<
        crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
        String,
    > {
        let device = initial_state.scalar_features.device();
        let baseline_scalars = initial_state.scalar_features.clone();

        let proposed_topology = self
            .ode_solver
            .forward(initial_state, t_start, t_end);

        let state_vec = nodal_scalar_means(&baseline_scalars, 6);
        let obs_vec = nodal_scalar_means(&proposed_topology.scalar_features, 6);
        let info_gain = histogram_info_gain_tensor(
            &mut self.mi_estimator,
            &state_vec,
            &obs_vec,
            &device,
        );
        self.epistemic_tracker.update(self.mi_estimator.estimate());

        match self
            .gateway
            .evaluate_topology_step(proposed_topology, info_gain)
        {
            Ok((verified_state, mut spatial_reward)) => {
                let bonus = self.epistemic_tracker.epistemic_bonus() as f32;
                spatial_reward = spatial_reward.add_scalar(bonus);

                let final_state_raw = verified_state.state.clone();
                let gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward,
                    t_start,
                    t_end,
                    dt_sim_dt_global,
                );

                const ADAM_LR: f32 = 1e-3;
                let (w_new, m1, m2, t) = adamw_step_policy(
                    self.ode_solver.policy_weights.clone(),
                    gradients,
                    ADAM_LR,
                    self.ode_solver.adam_m1.take(),
                    self.ode_solver.adam_m2.take(),
                    self.ode_solver.adam_t,
                );
                self.ode_solver.policy_weights = w_new;
                self.ode_solver.adam_m1 = Some(m1);
                self.ode_solver.adam_m2 = Some(m2);
                self.ode_solver.adam_t = t;

                Ok(verified_state)
            }
            Err(e) => Err(e),
        }
    }
}

/// AdamW tensor step (decoupled WD) — mirrors Burn AdamW defaults without host scalar reads.
fn adamw_step_policy<B: Backend<FloatElem = f32>>(
    weights: Tensor<B, 1>,
    grad: Tensor<B, 1>,
    lr: f32,
    m1: Option<Tensor<B, 1>>,
    m2: Option<Tensor<B, 1>>,
    t: usize,
) -> (Tensor<B, 1>, Tensor<B, 1>, Tensor<B, 1>, usize) {
    const BETA1: f32 = 0.9;
    const BETA2: f32 = 0.999;
    const EPS_ADAM: f32 = 1e-5;
    const WD: f32 = 1e-4;

    let tensor_updated = weights.clone().sub(weights.mul_scalar(lr * WD));

    let (moment_1, moment_2, time) = match (m1, m2) {
        (Some(m1_prev), Some(m2_prev)) => {
            let m1_new = m1_prev
                .mul_scalar(BETA1)
                .add(grad.clone().mul_scalar(1.0_f32 - BETA1));
            let m2_new = m2_prev.mul_scalar(BETA2).add(
                grad.clone()
                    .powf_scalar(2.0_f32)
                    .mul_scalar(1.0_f32 - BETA2),
            );
            (m1_new, m2_new, t + 1)
        }
        _ => {
            let m1_new = grad.clone().mul_scalar(1.0_f32 - BETA1);
            let m2_new = grad.powf_scalar(2.0_f32).mul_scalar(1.0_f32 - BETA2);
            (m1_new, m2_new, 1)
        }
    };

    let time_i = time as i32;
    let m1c = moment_1.clone().div_scalar(1.0_f32 - BETA1.powi(time_i));
    let m2c = moment_2.clone().div_scalar(1.0_f32 - BETA2.powi(time_i));
    let raw_delta = m1c.div(m2c.sqrt().add_scalar(EPS_ADAM));
    let new_w = tensor_updated.sub(raw_delta.mul_scalar(lr));

    (new_w, moment_1, moment_2, time)
}

#[cfg(all(test, not(feature = "epistemic-ppo")))]
mod tests {
    use super::BurnLiquidPPOAgent;
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn tiny_umst() -> UnifiedMaterialStateTensor<B> {
        let dev = device();
        let n = 2usize;
        let f = 5usize;
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
        fn compute_all(&self, mix: &MixTensor<Bk>) -> PhysicalResult<Bk> {
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

    /// Striatus Gate — PPO ↔ gateway ↔ finite backward surrogate (`adjoint`) smoke (default features).
    #[test]
    #[cfg(not(feature = "epistemic-ppo"))]
    fn burn_liquid_ppo_step_finite_backward_chain_smoke() {
        let dev = device();
        let gateway = ManifoldGateway::new(PpoChainStubCartridge, 300.0_f64, 1.0e-12_f64);
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
            "AdamW should move policy_weights after finite backward surrogate"
        );
    }
}

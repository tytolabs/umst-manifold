// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Liquid Time-Constant (LTC) / Neural ODE integration for PPO.
//!
//! Replaces the standard discrete MLP path with a continuous-time differential equation solver.
//! Instead of mapping Action = f(State), it solves:
//! d(Action)/dt = f_theta(Action, State)
//! This matches the continuous rheological evolution of printable bulk materials.
//!
//! Policy weights use **AdamW**-style tensor updates (Burn-default \(\beta_1,\beta_2,\varepsilon\), weight decay)
//! so [`AdjointNeuralODE::backward_adjoint`] gradients flow into [`AdjointNeuralODE::policy_weights`] without
//! `.into_scalar()` / `.into_data()` on the hot path. A separate `burn::module::Module` plus `burn::optim::Optimizer`
//! path over the same flattened weights is intentionally **not** supported here (see [`crate::ai::adjoint`] F1.4).
//!
//! **F1.4:** The supported optimization surface is **AdamW on the flat `policy_weights` vector** paired
//! with [`AdjointNeuralODE`], not a separate Burn `Module` + `burn::optim::Optimizer` over the same
//! parameters (see module notes in [`crate::ai::adjoint`]).
//!
//! # Honest boundary (W29-012)
//!
//! This module is the **Burn learner spine** — ODE forward, CBF gateway, adjoint backward, AdamW on
//! flat `policy_weights`. It does **not** attest physics GREEN, production orchestrator closure, or
//! swarm allocate policy. BIND / `liquid_ppo_allocate_ready` attestation is **harness-only** via
//! `umst-adk::liquid_ppo_bind` and bench witness artifacts — the source file never flips those bits.

/// W29 wave cell id — Burn learner spine deepen.
pub const LIQUID_PPO_CELL_ID: &str = "W29-012-LIQUID_PPO";

/// Primary training-step morphism @ SSOT (not gate alias).
pub const LIQUID_PPO_MORPHISM_ID: &str = "step_and_learn";

/// Honest posture — learner spine partial; tests deepen only (`MASTER_RETICK=no`).
pub const LIQUID_PPO_POSTURE_TAG: &str = "honest-burn-learner-spine-partial";

/// Compile-time honest fence — no production / GREEN / MASTER / allocate flip at source tier.
pub const LIQUID_PPO_HONEST_FENCE: &str =
    "burn_learner_spine_landed=true production_wired=false physics_green=false master_retick=false allocate_ready_source=false";

/// Honest non-claim @ source — BIND attestation is harness disk truth, not this module.
pub const SOURCE_NON_CLAIM: &str =
    "Burn learner spine + adjoint AdamW measured in crate tests; BIND/allocate attestation lives in umst-adk liquid_ppo_bind harness only; not physics GREEN.";

/// Golden learner fixture crosswalk (read-only; bytes attested @ bench harvest).
pub const GOLDEN_FIXTURE_PATH: &str =
    "crates/umst-bench/fixtures/golden_learner_burnliquid_ppo_v0.json";

/// Whether production training loop is closed end-to-end — **false** until orchestrator BIND + receipts.
pub const LIQUID_PPO_PRODUCTION_WIRED: bool = false;

/// Whether physics GREEN is claimed for the learner path — **false** (CBF gate is real; full closure is not).
pub const LIQUID_PPO_PHYSICS_GREEN: bool = false;

/// Whether MASTER retick is authorized for this cell — **false** @ W29 deepen tier.
pub const LIQUID_PPO_MASTER_RETICK: bool = false;

/// Source-tier allocate readiness — **false**; ADK harness decides `liquid_ppo_allocate_ready`.
pub const LIQUID_PPO_ALLOCATE_READY_SOURCE: bool = false;

/// Compile-time fence — production / MASTER flip not authorized at posture tier.
const _: () = assert!(!LIQUID_PPO_MASTER_RETICK);

/// Typed probe for W29 Burn learner posture honesty (meta / fleet probes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidPpoPostureProbe {
    pub cell_id: &'static str,
    pub morphism_id: &'static str,
    pub posture_tag: &'static str,
    pub honest_fence: &'static str,
    pub source_non_claim: &'static str,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master_retick: bool,
    pub allocate_ready_source: bool,
}

/// Build live posture probe from compile-time SSOT constants.
#[must_use]
pub fn liquid_ppo_posture_probe() -> LiquidPpoPostureProbe {
    LiquidPpoPostureProbe {
        cell_id: LIQUID_PPO_CELL_ID,
        morphism_id: LIQUID_PPO_MORPHISM_ID,
        posture_tag: LIQUID_PPO_POSTURE_TAG,
        honest_fence: LIQUID_PPO_HONEST_FENCE,
        source_non_claim: SOURCE_NON_CLAIM,
        production_wired: LIQUID_PPO_PRODUCTION_WIRED,
        physics_green: LIQUID_PPO_PHYSICS_GREEN,
        master_retick: LIQUID_PPO_MASTER_RETICK,
        allocate_ready_source: LIQUID_PPO_ALLOCATE_READY_SOURCE,
    }
}

/// Whether Burn learner morphism metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn liquid_ppo_morphism_pinned() -> bool {
    LIQUID_PPO_CELL_ID == "W29-012-LIQUID_PPO"
        && LIQUID_PPO_MORPHISM_ID == "step_and_learn"
        && LIQUID_PPO_POSTURE_TAG == "honest-burn-learner-spine-partial"
        && !LIQUID_PPO_PRODUCTION_WIRED
        && !LIQUID_PPO_PHYSICS_GREEN
        && !LIQUID_PPO_MASTER_RETICK
        && !LIQUID_PPO_ALLOCATE_READY_SOURCE
}

/// Validate Burn learner posture honesty — fail closed on fake production / GREEN / MASTER claims.
pub fn validate_liquid_ppo_posture_honesty() -> Result<(), &'static str> {
    let probe = liquid_ppo_posture_probe();
    if !liquid_ppo_morphism_pinned() {
        return Err("liquid_ppo_morphism_pinned failed");
    }
    if probe.posture_tag.to_ascii_lowercase().contains("green") {
        return Err("posture_tag must not claim green");
    }
    let lower = probe.source_non_claim.to_ascii_lowercase();
    if lower.contains("physics green") && probe.physics_green {
        return Err("source_non_claim must not pair with physics_green=true");
    }
    if !probe.honest_fence.contains("production_wired=false") {
        return Err("honest_fence missing production_wired=false");
    }
    if !probe.honest_fence.contains("physics_green=false") {
        return Err("honest_fence missing physics_green=false");
    }
    if !probe.honest_fence.contains("master_retick=false") {
        return Err("honest_fence missing master_retick=false");
    }
    if !probe.honest_fence.contains("allocate_ready_source=false") {
        return Err("honest_fence missing allocate_ready_source=false");
    }
    if probe.production_wired || probe.physics_green || probe.master_retick || probe.allocate_ready_source
    {
        return Err("honest booleans must stay false at W29 deepen tier");
    }
    Ok(())
}

use crate::ai::adjoint::AdjointNeuralODE;
use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use burn::tensor::{backend::Backend, Tensor};

/// AdamW learning rate on flat `policy_weights` (Burn-default scale).
pub const ADAM_LR_DEFAULT: f32 = 1e-3;

/// Default adapt routing when no optional PPO features are enabled.
pub const ADAPT_PATH_STUB: &str = "stub";

/// Kleisli penalize adapt arm (`kleisli-ppo-hot-bind`).
#[cfg(feature = "kleisli-ppo-hot-bind")]
pub const ADAPT_PATH_KLEISLI: &str = "kleisli-penalize";

/// Epistemic histogram-MI adapt arm (`epistemic-ppo`; wins over kleisli when both enabled).
#[cfg(feature = "epistemic-ppo")]
pub const ADAPT_PATH_EPISTEMIC: &str = "epistemic";

/// Active `step_and_learn` routing id — compile-time feature priority: epistemic > kleisli > stub.
#[must_use]
pub fn active_adapt_path() -> &'static str {
    #[cfg(feature = "epistemic-ppo")]
    {
        return ADAPT_PATH_EPISTEMIC;
    }
    #[cfg(all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
    {
        return ADAPT_PATH_KLEISLI;
    }
    ADAPT_PATH_STUB
}

#[cfg(feature = "epistemic-ppo")]
use crate::ai::info_gain::{
    histogram_info_gain_tensor, nodal_scalar_means, EpistemicStateTracker, MutualInfoEstimator,
};
#[cfg(feature = "epistemic-ppo")]
use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

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
    ///
    /// With **`kleisli-ppo-hot-bind`** (and not epistemic), routes through the Kleisli penalize hook
    /// ([`Self::step_and_learn_kleisli_penalize`]) instead of [`Self::step_and_learn_stub`].
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
            return self.step_and_learn_epistemic(
                initial_state,
                t_start,
                t_end,
                info_gain,
                dt_sim_dt_global,
            );
        }

        #[cfg(all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
        {
            self.step_and_learn_kleisli_penalize(
                initial_state,
                t_start,
                t_end,
                info_gain,
                dt_sim_dt_global,
            )
        }

        #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
        {
            self.step_and_learn_stub(initial_state, t_start, t_end, info_gain, dt_sim_dt_global)
        }
    }

    #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
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
        let proposed_topology = self.ode_solver.forward(initial_state, t_start, t_end);

        match self
            .gateway
            .evaluate_topology_step(proposed_topology, info_gain.clone())
        {
            Ok((verified_state, spatial_reward)) => {
                let final_state_raw = verified_state.state.clone();

                let gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward,
                    t_start,
                    t_end,
                    dt_sim_dt_global,
                );

                const ADAM_LR: f32 = ADAM_LR_DEFAULT;
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

    /// Kleisli **penalize** stage on the Burn hot path (`kleisli-ppo-hot-bind`).
    ///
    /// Same ODE → CBF → adjoint topology as [`Self::step_and_learn_stub`], but subtracts
    /// [`ManifoldGateway::constraint_loss_penalty`] from `spatial_reward` when `lambda_cd ≠ 0`.
    #[cfg(all(feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
    fn step_and_learn_kleisli_penalize(
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
        let baseline_state = initial_state.clone();
        let proposed_topology = self.ode_solver.forward(initial_state, t_start, t_end);

        match self
            .gateway
            .evaluate_topology_step(proposed_topology, info_gain.clone())
        {
            Ok((verified_state, spatial_reward)) => {
                let spatial_reward = self.subtract_cd_penalty_from_reward(
                    &baseline_state,
                    &verified_state,
                    spatial_reward,
                    &dt_sim_dt_global,
                    &info_gain,
                );
                let final_state_raw = verified_state.state.clone();

                let gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward,
                    t_start,
                    t_end,
                    dt_sim_dt_global,
                );

                const ADAM_LR: f32 = ADAM_LR_DEFAULT;
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

    #[cfg(feature = "epistemic-ppo")]
    fn step_and_learn_epistemic(
        &mut self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
        external_info_gain: Tensor<B, 1>,
        dt_sim_dt_global: Tensor<B, 1>,
    ) -> Result<
        crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
        String,
    > {
        let device = initial_state.scalar_features.device();
        let baseline_state = initial_state.clone();
        let baseline_scalars = initial_state.scalar_features.clone();

        let proposed_topology = self.ode_solver.forward(initial_state, t_start, t_end);

        let state_vec = nodal_scalar_means(&baseline_scalars, UMST_SCALAR_CHANNEL_COUNT);
        let obs_vec = nodal_scalar_means(
            &proposed_topology.scalar_features,
            UMST_SCALAR_CHANNEL_COUNT,
        );
        let mut info_gain =
            histogram_info_gain_tensor(&mut self.mi_estimator, &state_vec, &obs_vec, &device);
        #[cfg(feature = "kleisli-ppo-hot-bind")]
        {
            // Kleisli hot-bind keeps caller `info_gain` as a Landauer floor when histogram MI is ~0.
            info_gain = info_gain.max_pair(external_info_gain);
        }
        self.epistemic_tracker.update(self.mi_estimator.estimate());

        match self
            .gateway
            .evaluate_topology_step(proposed_topology, info_gain.clone())
        {
            Ok((verified_state, mut spatial_reward)) => {
                let bonus = self.epistemic_tracker.epistemic_bonus() as f32;
                spatial_reward = spatial_reward.add_scalar(bonus);

                spatial_reward = self.subtract_cd_penalty_from_reward(
                    &baseline_state,
                    &verified_state,
                    spatial_reward,
                    &dt_sim_dt_global,
                    &info_gain,
                );

                let final_state_raw = verified_state.state.clone();
                let gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward,
                    t_start,
                    t_end,
                    dt_sim_dt_global,
                );

                const ADAM_LR: f32 = ADAM_LR_DEFAULT;
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

    /// Kleisli **penalize** hook: subtract `λ_cd · relu(−D_int)` from the spatial reward tensor.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    fn subtract_cd_penalty_from_reward(
        &self,
        baseline_state: &UnifiedMaterialStateTensor<B>,
        verified_state: &crate::core::tensors::VerifiedUMST<
            B,
            crate::core::tensors::ClausiusDuhemProof,
        >,
        spatial_reward: Tensor<B, 1>,
        dt_sim_dt_global: &Tensor<B, 1>,
        info_gain: &Tensor<B, 1>,
    ) -> Tensor<B, 1> {
        if self.gateway.lambda_cd == 0.0_f32 && self.gateway.lambda_landauer == 0.0_f32 {
            return spatial_reward;
        }
        let device = baseline_state.scalar_features.device();
        let baseline_pr = self.gateway.cartridge.compute_topology(baseline_state);
        let proposed_pr = self
            .gateway
            .cartridge
            .compute_topology(&verified_state.state);
        let batch = spatial_reward.dims()[0];
        let rho = Tensor::<B, 1>::full([batch], 2400.0_f32, &device);
        let old_fe = baseline_pr.free_energy.mean_dim(1).squeeze(1);
        let new_fe = proposed_pr.free_energy.mean_dim(1).squeeze(1);
        let penalty = self.gateway.total_constraint_loss_penalty(
            rho.clone(),
            rho,
            old_fe,
            new_fe,
            dt_sim_dt_global.clone(),
            info_gain.clone(),
        );
        spatial_reward.sub(penalty)
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

#[cfg(all(
    test,
    not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))
))]
mod tests {
    use super::{
        active_adapt_path, adamw_step_policy, liquid_ppo_morphism_pinned, liquid_ppo_posture_probe,
        validate_liquid_ppo_posture_honesty, BurnLiquidPPOAgent, ADAM_LR_DEFAULT, ADAPT_PATH_STUB,
        GOLDEN_FIXTURE_PATH, LIQUID_PPO_CELL_ID, LIQUID_PPO_HONEST_FENCE, LIQUID_PPO_MORPHISM_ID,
        LIQUID_PPO_PHYSICS_GREEN, LIQUID_PPO_PRODUCTION_WIRED, SOURCE_NON_CLAIM,
    };
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
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

    #[test]
    fn burn_liquid_ppo_adapt_path_routes_stub_when_no_features() {
        assert_eq!(active_adapt_path(), ADAPT_PATH_STUB);
    }

    #[test]
    fn w29_liquid_ppo_posture_probe_honest_not_green() {
        let probe = liquid_ppo_posture_probe();
        assert_eq!(probe.cell_id, LIQUID_PPO_CELL_ID);
        assert_eq!(probe.morphism_id, LIQUID_PPO_MORPHISM_ID);
        assert!(probe.posture_tag.contains("honest"));
        assert!(!probe.posture_tag.to_ascii_lowercase().contains("green"));
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master_retick);
        assert!(!probe.allocate_ready_source);
        assert!(!LIQUID_PPO_PRODUCTION_WIRED);
        assert!(!LIQUID_PPO_PHYSICS_GREEN);
    }

    #[test]
    fn w29_liquid_ppo_morphism_pinned() {
        assert!(liquid_ppo_morphism_pinned());
        assert_eq!(
            LIQUID_PPO_HONEST_FENCE,
            "burn_learner_spine_landed=true production_wired=false physics_green=false master_retick=false allocate_ready_source=false"
        );
        assert!(SOURCE_NON_CLAIM.contains("not physics GREEN"));
        assert!(GOLDEN_FIXTURE_PATH.contains("golden_learner_burnliquid_ppo_v0"));
    }

    #[test]
    fn w29_liquid_ppo_validate_posture_honesty_ok() {
        assert!(validate_liquid_ppo_posture_honesty().is_ok());
    }

    #[test]
    fn burn_liquid_ppo_adam_lr_default_pinned() {
        assert!((ADAM_LR_DEFAULT - 1e-3_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn burn_liquid_ppo_adamw_step_policy_moves_weights() {
        let dev = device();
        let w = Tensor::<B, 1>::full([4], 1.0_f32, &dev);
        let g = Tensor::<B, 1>::full([4], 0.5_f32, &dev);
        let (w_new, _m1, _m2, t) = adamw_step_policy(w, g, ADAM_LR_DEFAULT, None, None, 0);
        assert_eq!(t, 1);
        let w0 = w_new.clone().into_data().value[0];
        assert_ne!(w0, 1.0_f32, "AdamW must move weights on non-zero grad");
        assert!(w0.is_finite());
    }

    #[test]
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

#[cfg(all(test, feature = "kleisli-ppo-hot-bind", not(feature = "epistemic-ppo")))]
mod kleisli_adapt_tests {
    use super::{active_adapt_path, BurnLiquidPPOAgent, ADAPT_PATH_KLEISLI};
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
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

    #[test]
    fn burn_liquid_ppo_kleisli_adapt_path_routes_penalize() {
        assert_eq!(active_adapt_path(), ADAPT_PATH_KLEISLI);
    }

    #[test]
    fn burn_liquid_ppo_kleisli_penalize_finite_backward_chain_smoke() {
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
    fn burn_liquid_ppo_kleisli_subtract_cd_penalty_noop_when_lambdas_zero() {
        let dev = device();
        let mut agent = BurnLiquidPPOAgent::new(ManifoldGateway::new(
            PpoChainStubCartridge,
            300.0_f64,
            1.0e-12_f64,
        ));
        let baseline = tiny_umst();
        let proposed = tiny_umst();
        let info = Tensor::<B, 1>::full([1], 0.01_f32, &dev);
        let dt_rat = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
        let reward = Tensor::<B, 1>::full([1], 1.5_f32, &dev);

        // Build a minimal VerifiedUMST wrapper for the penalty hook witness.
        let verified = agent
            .gateway
            .evaluate_topology_step(proposed.clone(), info.clone())
            .expect("stub topology must pass")
            .0;

        let adjusted = agent.subtract_cd_penalty_from_reward(
            &baseline,
            &verified,
            reward.clone(),
            &dt_rat,
            &info,
        );
        let before: f32 = reward.into_data().value[0];
        let after: f32 = adjusted.into_data().value[0];
        assert!(
            (before - after).abs() < 1e-6,
            "zero lambdas must leave reward unchanged: {before} vs {after}"
        );
    }
}

#[cfg(all(test, feature = "epistemic-ppo"))]
mod epistemic_adapt_tests {
    use super::{active_adapt_path, BurnLiquidPPOAgent, ADAPT_PATH_EPISTEMIC};
    use crate::ai::ppo::ManifoldGateway;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::{SCALAR_INTERNAL_VARIABLE_0, UMST_SCALAR_CHANNEL_COUNT};
    use burn::tensor::backend::Backend;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

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

    #[test]
    fn burn_liquid_ppo_epistemic_adapt_path_routes_histogram_mi() {
        assert_eq!(active_adapt_path(), ADAPT_PATH_EPISTEMIC);
    }

    #[test]
    fn burn_liquid_ppo_epistemic_step_finite_backward_chain_smoke() {
        let dev = device();
        let mut gateway = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-4_f64);
        gateway.beta = 0.1;
        let mut agent = BurnLiquidPPOAgent::new(gateway);
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let state = umst_with_hydration(0.5, n, f);

        let info = Tensor::<B, 1>::full([1], 0.01_f32, &dev);
        let dt_rat = Tensor::<B, 1>::full([1], 1.0_f32, &dev);
        let w0 = agent.ode_solver.policy_weights.clone().into_data().value[0];
        let out = agent.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
        assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
        let w1 = agent.ode_solver.policy_weights.clone().into_data().value[0];
        assert!(w0.is_finite() && w1.is_finite(), "weights must stay finite");
        assert_ne!(
            w0, w1,
            "epistemic path should move policy_weights after backward"
        );
    }

    #[test]
    fn burn_liquid_ppo_epistemic_tracker_updates_after_step() {
        let dev = device();
        let gateway = ManifoldGateway::new(GateAwareCartridge, 300.0_f64, 1.0e-4_f64);
        let mut agent = BurnLiquidPPOAgent::new(gateway);
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let state = umst_with_hydration(0.5, n, f);
        let info = Tensor::<B, 1>::full([1], 0.01_f32, &dev);
        let dt_rat = Tensor::<B, 1>::full([1], 1.0_f32, &dev);

        let bonus_before = agent.epistemic_tracker.epistemic_bonus();
        let _ = agent.step_and_learn(state, 0.0_f32, 1.0_f32, info, dt_rat);
        let bonus_after = agent.epistemic_tracker.epistemic_bonus();
        assert!(
            bonus_after.is_finite(),
            "epistemic bonus must stay finite after step"
        );
        assert!(
            bonus_after >= bonus_before,
            "tracker should not decrease bonus on MI update: before={bonus_before} after={bonus_after}"
        );
    }
}

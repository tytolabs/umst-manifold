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
//! `.into_scalar()` / `.into_data()` on the hot path. A separate [`burn::module::Module`] plus [`burn::optim::Optimizer`]
//! path over the same flattened weights is intentionally **not** supported here (see [`crate::ai::adjoint`] F1.4).
//!
//! **F1.4:** The supported optimization surface is **AdamW on the flat `policy_weights` vector** paired
//! with [`AdjointNeuralODE`], not a separate Burn [`Module`] + [`burn::optim::Optimizer`] over the same
//! parameters (see module notes in [`crate::ai::adjoint`]).

use crate::ai::adjoint::AdjointNeuralODE;
use crate::ai::ppo::ManifoldGateway;
use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use burn::tensor::{backend::Backend, Tensor};

/// The Burn-Native Liquid PPO Agent.
/// Drives the material state forward in continuous time using the Adjoint Method,
/// and validates the results against the Thermodynamic Manifold Gateway.
pub struct BurnLiquidPPOAgent<B: Backend, C: IScienceCartridge<B>> {
    pub ode_solver: AdjointNeuralODE<B>,
    pub gateway: ManifoldGateway<B, C>,
}

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> BurnLiquidPPOAgent<B, C> {
    pub fn new(gateway: ManifoldGateway<B, C>) -> Self {
        Self {
            ode_solver: AdjointNeuralODE::default(),
            gateway,
        }
    }

    /// Primary execution step: The Brain wires into the Body.
    /// 1. Solves the Neural ODE forward to get the proposed topology.
    /// 2. Passes the topology through the Thermodynamic Gateway.
    /// 3. Backpropagates the spatial reward using the O(1) memory Adjoint backward pass.
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

#[cfg(test)]
mod tests {}

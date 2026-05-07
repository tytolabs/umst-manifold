// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Liquid Time-Constant (LTC) / Neural ODE integration for PPO.
//!
//! Replaces the standard discrete MLP path with a continuous-time differential equation solver.
//! Instead of mapping Action = f(State), it solves:
//! d(Action)/dt = f_theta(Action, State)
//! This matches the continuous rheological evolution of printing concrete.

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

impl<B: Backend, C: IScienceCartridge<B>> BurnLiquidPPOAgent<B, C> {
    pub fn new(gateway: ManifoldGateway<B, C>) -> Self {
        Self {
            ode_solver: AdjointNeuralODE::new(),
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

                let _gradients = self.ode_solver.backward_adjoint(
                    final_state_raw,
                    spatial_reward, // Target from the physical thermodynamic outcome
                    t_start,
                    t_end,
                    dt_sim_dt_global, // Time Dilation optimization
                );

                // In a full implementation, optimizer.step(gradients) would happen here.

                Ok(verified_state)
            }
            Err(e) => {
                // The AI proposed a topology that broke the 2nd Law of Thermodynamics.
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {}

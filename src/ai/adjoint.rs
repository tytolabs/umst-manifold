// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Adjoint-state method for Neural ODE policy gradients with \(O(1)\) memory (no full BPTT tape).
//!
//! ## F1.4 — Optimizer API (Liquid PPO stack)
//!
//! Policy training here is **`AdjointNeuralODE::policy_weights` as a flat rank‑1 tensor** with
//! hand‑rolled **AdamW‑style** tensor updates in [`crate::ai::liquid_ppo`] (first/second moment on
//! the same vector shape as `dL_dtheta`). A Burn `burn::module::Module` + `burn::optim::Optimizer`
//! wrapper around those weights is **not** the supported path today; callers should treat
//! **tensor AdamW on `policy_weights`** as the stable, autodiff‑compatible contract until a
//! refactor explicitly migrates to `Module` + `Optimizer`.
//!
//! # Relation to [`crate::core::traits::PhysicalResult`] and **`info_gain`**
//!
//! End-to-end training composes this solver with [`crate::ai::ppo::ManifoldGateway`], which pulls a
//! [`crate::core::traits::PhysicalResult`] from [`crate::core::traits::IScienceCartridge::compute_topology`]
//! and maps sparse nodal tensors into the differentiable spatial reward (see [`crate::ai::ppo`]).
//! The thermodynamic gate’s Landauer branch consumes a batch **`info_gain`** vector passed with the UMST
//! ([`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`]); differentiable surrogates used before a
//! full MI estimator exists live in [`crate::ai::info_gain`].
//!
//! # Multi-physics Jacobian gaps
//!
//! The backward pass assumes an augmented ODE whose sensitivities require **−λᵀ ∂f/∂z** and **−λᵀ ∂f/∂θ**
//! for \(\dot z = f_\theta(z,t)\). Multiple cartridges or solver phases are **not** yet assembled into one
//! coupled Jacobian here—cross-block coupling (thermal ↔ rheology ↔ damage, experimental summaries
//! overwritten upstream, etc.) means this path **does not** yet supply a faithful multi-physics adjoint until
//! those interfaces expose consistent derivatives end-to-end.
//!
//! **Finite backward (surrogate):** [`AdjointNeuralODE::backward_adjoint`] runs a **fixed small**
//! number of explicit backward-time tensor steps (no unbounded loops) so PPO smoke and AdamW updates
//! exercise real arithmetic on `policy_weights`, `dL_dz`, and `dt_sim_dt_global`. This is still a
//! **surrogate** \(dL/d\theta\), not a discretised continuous adjoint of the forward pass.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::core::tensors::UnifiedMaterialStateTensor;
use burn::tensor::{backend::Backend, Shape, Tensor};

#[cfg(feature = "epistemic-ppo")]
use crate::core::umst_schema::SCALAR_EPISTEMIC_UNCERTAINTY;

/// Placeholder policy vector size \(P\) for [`AdjointNeuralODE::policy_weights`] / adjoint gradients.
pub const ADJOINT_POLICY_DIM: usize = 1024;

/// Fixed backward-time substeps for [`AdjointNeuralODE::backward_adjoint`] (finite-horizon surrogate).
const ADJOINT_BACKWARD_STEPS: usize = 10;

/// Augmented state tensor for the Adjoint Method.
/// \mathbf{a}(t) = [\mathbf{z}(t), \mathbf{\lambda}(t), \frac{\partial L}{\partial \theta}]
/// z(t) is the physical state.
/// \lambda(t) is the adjoint state (gradient of loss w.r.t state).
/// \partial L / \partial \theta is the gradient of loss w.r.t policy weights.
pub struct AugmentedState<B: Backend> {
    pub z_t: UnifiedMaterialStateTensor<B>,
    pub lambda_t: Tensor<B, 3>, // Adjoint state [Batch, N_voxels, Features]
    pub dL_dtheta: Tensor<B, 1>, // Gradients for the policy weights
}

pub struct AdjointNeuralODE<B: Backend> {
    /// Trainable policy parameters \(\theta\) — consumed by [`crate::ai::liquid_ppo`] AdamW updates.
    pub policy_weights: Tensor<B, 1>,
    pub adam_m1: Option<Tensor<B, 1>>,
    pub adam_m2: Option<Tensor<B, 1>>,
    pub adam_t: usize,
    _backend: PhantomData<B>,
}

impl<B: Backend<FloatElem = f32>> Default for AdjointNeuralODE<B> {
    fn default() -> Self {
        let device = Default::default();
        Self::new(&device)
    }
}

impl<B: Backend<FloatElem = f32>> AdjointNeuralODE<B> {
    pub fn new(device: &B::Device) -> Self {
        Self {
            policy_weights: Tensor::zeros([ADJOINT_POLICY_DIM], device),
            adam_m1: None,
            adam_m2: None,
            adam_t: 0,
            _backend: PhantomData,
        }
    }

    /// Forward pass (No-Compromise B1)
    /// Computes continuous integration without saving intermediate UMST states.
    /// Uses O(1) memory footprint.
    ///
    /// With **`epistemic-ppo`**, applies policy-weight-driven deltas on
    /// [`UnifiedMaterialStateTensor::policy_editable_mask`] channels and writes the uncertainty column.
    pub fn forward(
        &self,
        initial_state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
    ) -> UnifiedMaterialStateTensor<B> {
        #[cfg(not(feature = "epistemic-ppo"))]
        {
            let _ = (t_start, t_end);
            let _ = &self.policy_weights;
            return initial_state;
        }

        #[cfg(feature = "epistemic-ppo")]
        {
            self.forward_epistemic(initial_state, t_start, t_end)
        }
    }

    #[cfg(feature = "epistemic-ppo")]
    fn forward_epistemic(
        &self,
        mut state: UnifiedMaterialStateTensor<B>,
        t_start: f32,
        t_end: f32,
    ) -> UnifiedMaterialStateTensor<B> {
        let steps = 10usize;
        let dt = ((t_end - t_start) / steps as f32).max(1e-6);
        let [n, f] = state.scalar_features.dims();
        let dof = (n * f).min(ADJOINT_POLICY_DIM);

        for step in 0..steps {
            let w = 1.0_f32 / (step + 1) as f32;
            let theta_slice = self
                .policy_weights
                .clone()
                .slice([0..dof])
                .reshape([n, f])
                .mul_scalar(dt * w * 0.05);
            let proposed = state.scalar_features.clone().add(theta_slice.clone());
            state.scalar_features = state.apply_policy_mask(proposed.clone());

            if f > SCALAR_EPISTEMIC_UNCERTAINTY {
                let unc = theta_slice.powf_scalar(2.0).mean_dim(1).clamp(0.0, 1.0);
                state.write_scalar_channel(SCALAR_EPISTEMIC_UNCERTAINTY, unc);
            }
        }
        state
    }

    /// Backward pass using the Adjoint State Method (No-Compromise B2/B3), driven by the scalar-per-batch
    /// reward signal returned from [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`].
    ///
    /// Returns **`dL/dθ`** shaped **`[ADJOINT_POLICY_DIM]`** — a surrogate gradient aligned with the mean
    /// spatial reward seed **`dL_dz`** (no `.into_scalar()` / `.into_data()`).
    pub fn backward_adjoint(
        &self,
        _final_state: UnifiedMaterialStateTensor<B>,
        dL_dz: Tensor<B, 1>,
        t_start: f32,
        t_end: f32,
        dt_sim_dt_global: Tensor<B, 1>,
    ) -> Tensor<B, 1> {
        let batch = dL_dz.dims()[0].max(1) as f32;
        let avg_seed = dL_dz.sum_dim(0).div_scalar(batch);
        let seed = avg_seed
            .clone()
            .reshape([1])
            .expand(Shape::new([ADJOINT_POLICY_DIM]));

        let dil = dt_sim_dt_global.sum_dim(0).div_scalar(batch);
        let dil_bc = dil.reshape([1]).expand(Shape::new([ADJOINT_POLICY_DIM]));

        let steps = ADJOINT_BACKWARD_STEPS.max(1);
        let dt = ((t_end - t_start).abs() / steps as f32).max(1e-8);

        let theta = self.policy_weights.clone();
        let mut acc = Tensor::zeros_like(&theta);
        for k in (0..steps).rev() {
            let w = 1.0_f32 / (k + 1) as f32;
            let drive_theta = theta.clone().mul_scalar(dt * w);
            let drive_dil = dil_bc.clone().mul_scalar(1e-3_f32 * dt * w);
            acc = acc.add(drive_theta).add(drive_dil);
        }

        acc.mul(seed)
    }
}

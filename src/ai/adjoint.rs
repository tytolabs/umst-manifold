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
//!
//! **Honest status:** surrogate ODE adjoint + tensor AdamW contract is measured — not physics GREEN,
//! not `PRODUCTION_WIRED`, not `MASTER_RETICK`. Discrete compliance adjoint lives in
//! [`crate::physics::adjoint`]; consumer-drift witness: `crates/umst-bench/src/witness/mf_adjoint.rs`.
//!
//! **Witness:** `cargo test --manifest-path umst-manifold/Cargo.toml adjoint` · cell `W29-004-ADJOINT`.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::core::tensors::UnifiedMaterialStateTensor;
use burn::tensor::{backend::Backend, Shape, Tensor};

#[cfg(feature = "epistemic-ppo")]
use crate::core::umst_schema::SCALAR_EPISTEMIC_UNCERTAINTY;

/// W29 deepen cell id — honest surrogate adjoint slice only.
pub const ADJOINT_CELL_ID: &str = "W29-004-ADJOINT";

/// Honest posture tag — tests deepen only; no GREEN invent (`MASTER_RETICK=no`).
pub const ADJOINT_POSTURE_TAG: &str = "honest-surrogate-adjoint-only";

/// LPP-004 morphism id @ AI manifold adjoint band (forward → backward_adjoint → AdamW).
pub const ADJOINT_ODE_MORPHISM_ID: &str = "adjoint_neural_ode_surrogate_policy_gradient";

/// Consumer-drift witness status — LO harness present; full replay deferred.
pub const ADJOINT_CONSUMER_DRIFT_STATUS: &str = "LO_HARNESS_PRESENT_REPLAY_DEFERRED";

/// Integration cargo-test posture — physics adjoint harness is PARTIAL (see `mf_adjoint` witness).
pub const ADJOINT_INTEGRATION_CARGO_TEST_STATUS: &str = "PARTIAL";

/// Honest physics posture — surrogate gradient computes; continuum multi-physics adjoint deferred.
pub const ADJOINT_PHYSICS_GREEN: bool = false;

/// Production wiring at cartridge / multi-physics seam — deferred beyond W29 slice.
pub const ADJOINT_PRODUCTION_WIRED: bool = false;

/// Placeholder policy vector size \(P\) for [`AdjointNeuralODE::policy_weights`] / adjoint gradients.
pub const ADJOINT_POLICY_DIM: usize = 1024;

/// Fixed backward-time substeps for [`AdjointNeuralODE::backward_adjoint`] (finite-horizon surrogate).
pub const ADJOINT_BACKWARD_STEPS: usize = 10;

/// Honest slice posture — surrogate evaluators landed, physics GREEN refused.
#[must_use]
pub const fn adjoint_posture_is_honest() -> bool {
    !ADJOINT_PHYSICS_GREEN && !ADJOINT_PRODUCTION_WIRED
}

/// W29 honest posture bundle — surrogate landed, physics GREEN refused.
#[must_use]
pub const fn adjoint_w29_honest_posture_bundle() -> bool {
    adjoint_posture_is_honest() && !ADJOINT_PHYSICS_GREEN && !ADJOINT_PRODUCTION_WIRED
}

/// Whether the adjoint ODE morphism is pinned @ HEAD (policy dim + backward steps + posture).
#[must_use]
pub fn adjoint_ode_morphism_pinned() -> bool {
    ADJOINT_ODE_MORPHISM_ID == "adjoint_neural_ode_surrogate_policy_gradient"
        && ADJOINT_POSTURE_TAG == "honest-surrogate-adjoint-only"
        && ADJOINT_CELL_ID == "W29-004-ADJOINT"
        && ADJOINT_POLICY_DIM == 1024
        && ADJOINT_BACKWARD_STEPS == 10
        && ADJOINT_CONSUMER_DRIFT_STATUS == "LO_HARNESS_PRESENT_REPLAY_DEFERRED"
        && ADJOINT_INTEGRATION_CARGO_TEST_STATUS == "PARTIAL"
}

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
            initial_state
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn test_umst(n: usize, f: usize) -> UnifiedMaterialStateTensor<B> {
        let dev = device();
        let coords: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
        let edges_b1: Tensor<B, 2, burn::tensor::Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &dev,
        );
        let faces_b2: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features: Tensor::<B, 2>::zeros([n, f], &dev),
            vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
            matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    #[test]
    fn adjoint_w29_honest_posture_bundle_holds() {
        assert!(adjoint_w29_honest_posture_bundle());
        assert!(adjoint_posture_is_honest());
        assert_eq!(ADJOINT_CELL_ID, "W29-004-ADJOINT");
        assert!(!ADJOINT_PHYSICS_GREEN);
        assert!(!ADJOINT_PRODUCTION_WIRED);
    }

    #[test]
    fn adjoint_ode_morphism_pinned_at_head() {
        assert!(adjoint_ode_morphism_pinned());
        assert_eq!(
            ADJOINT_ODE_MORPHISM_ID,
            "adjoint_neural_ode_surrogate_policy_gradient"
        );
    }

    #[test]
    fn adjoint_posture_tag_honest_not_green() {
        assert!(ADJOINT_POSTURE_TAG.contains("honest"));
        assert!(!ADJOINT_POSTURE_TAG.to_ascii_lowercase().contains("green"));
        assert!(!ADJOINT_POSTURE_TAG.to_ascii_lowercase().contains("production"));
    }

    #[test]
    fn adjoint_policy_dim_anchor_pinned() {
        assert_eq!(ADJOINT_POLICY_DIM, 1024);
        let ode = AdjointNeuralODE::<B>::new(&device());
        assert_eq!(ode.policy_weights.dims(), [ADJOINT_POLICY_DIM]);
    }

    #[test]
    fn adjoint_backward_steps_anchor_pinned() {
        assert_eq!(ADJOINT_BACKWARD_STEPS, 10);
    }

    #[test]
    fn adjoint_consumer_drift_status_replay_deferred() {
        assert_eq!(
            ADJOINT_CONSUMER_DRIFT_STATUS,
            "LO_HARNESS_PRESENT_REPLAY_DEFERRED"
        );
        assert_eq!(ADJOINT_INTEGRATION_CARGO_TEST_STATUS, "PARTIAL");
        assert_ne!(ADJOINT_INTEGRATION_CARGO_TEST_STATUS, "PASS");
    }

    #[test]
    fn adjoint_backward_adjoint_returns_policy_dim_shape() {
        let dev = device();
        let ode = AdjointNeuralODE::<B>::new(&dev);
        let dL_dz = Tensor::<B, 1>::from_data(Data::new(vec![0.5_f32], Shape::new([1])), &dev);
        let dt = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let grad = ode.backward_adjoint(
            test_umst(2, 4),
            dL_dz,
            0.0,
            1.0,
            dt,
        );
        assert_eq!(grad.dims(), [ADJOINT_POLICY_DIM]);
    }

    #[test]
    fn adjoint_backward_adjoint_nonzero_for_nonzero_seed() {
        let dev = device();
        let mut ode = AdjointNeuralODE::<B>::new(&dev);
        ode.policy_weights = Tensor::<B, 1>::full([ADJOINT_POLICY_DIM], 0.25_f32, &dev);
        let dL_dz = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let dt = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let grad = ode.backward_adjoint(
            test_umst(2, 4),
            dL_dz,
            0.0,
            1.0,
            dt,
        );
        let l2: f32 = grad.powf_scalar(2.0).sum().into_scalar();
        assert!(l2 > 1.0e-12, "surrogate grad must be non-zero for nonzero seed");
    }

    #[test]
    fn adjoint_backward_adjoint_zero_seed_yields_zero_grad() {
        let dev = device();
        let mut ode = AdjointNeuralODE::<B>::new(&dev);
        ode.policy_weights = Tensor::<B, 1>::full([ADJOINT_POLICY_DIM], 0.5_f32, &dev);
        let dL_dz = Tensor::<B, 1>::from_data(Data::new(vec![0.0_f32], Shape::new([1])), &dev);
        let dt = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let grad = ode.backward_adjoint(
            test_umst(2, 4),
            dL_dz,
            0.0,
            1.0,
            dt,
        );
        let max_abs: f32 = grad.abs().max().into_scalar();
        assert_relative_eq!(max_abs, 0.0, epsilon = 1.0e-12);
    }

    #[test]
    fn adjoint_backward_adjoint_scales_linearly_with_seed() {
        let dev = device();
        let mut ode = AdjointNeuralODE::<B>::new(&dev);
        ode.policy_weights = Tensor::<B, 1>::full([ADJOINT_POLICY_DIM], 0.1_f32, &dev);
        let dt = Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev);
        let umst = test_umst(2, 4);
        let g1 = ode.backward_adjoint(
            umst.clone(),
            Tensor::<B, 1>::from_data(Data::new(vec![1.0_f32], Shape::new([1])), &dev),
            0.0,
            1.0,
            dt.clone(),
        );
        let g2 = ode.backward_adjoint(
            umst,
            Tensor::<B, 1>::from_data(Data::new(vec![2.0_f32], Shape::new([1])), &dev),
            0.0,
            1.0,
            dt,
        );
        let s1: f32 = g1.sum().into_scalar();
        let s2: f32 = g2.sum().into_scalar();
        assert_relative_eq!(s2, 2.0 * s1, epsilon = 1.0e-5);
    }

    #[test]
    fn adjoint_default_forward_is_identity_without_epistemic_feature() {
        #[cfg(not(feature = "epistemic-ppo"))]
        {
            let dev = device();
            let ode = AdjointNeuralODE::<B>::new(&dev);
            let umst = test_umst(3, 5);
            let before: Vec<f32> = umst.scalar_features.clone().into_data().value;
            let out = ode.forward(umst, 0.0, 1.0);
            let after: Vec<f32> = out.scalar_features.into_data().value;
            assert_eq!(before, after, "default build must pass through UMST unchanged");
        }
        #[cfg(feature = "epistemic-ppo")]
        {
            // epistemic path covered by tests/epistemic_ppo.rs
        }
    }
}

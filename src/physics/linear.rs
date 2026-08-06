// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Masked inner products for Krylov solves on constrained DOFs (Dirichlet via mask).
//!
//! Shared by mechanics today; future solvers (THMC implicit steps, fracture) should reuse
//! these primitives instead of duplicating `mask *` reduction patterns.
//!
//! # Honest boundary (W29-057)
//!
//! Burn-tensor masked reductions (`masked_dot`, `masked_norm_sq`) are the **device/autodiff**
//! Krylov primitives used by bar/Q1-hex adjoint compliance paths. Host fused PCG reductions
//! live in [`super::pcg_reduction`]. Unit contracts exercise mask zeros + free-DOF dots.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`.

use burn::tensor::{backend::Backend, Tensor};

/// W29 deepen cell — masked linear-algebra honest fence bundle.
pub const W29_LINEAR_DEEPEN_CELL: &str = "W29-057-LINEAR";

/// Honest posture tag — Burn masked Krylov reductions landed; fleet production wiring refused.
pub const LINEAR_POSTURE_TAG: &str = "honest-burn-masked-krylov-reductions";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const LINEAR_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by masked reduction primitives alone.
pub const LINEAR_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const LINEAR_MASTER: bool = false;

/// Whether Burn masked_dot / masked_norm_sq contracts are landed in this module.
pub const LINEAR_MASKED_REDUCTIONS_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const LINEAR_HONEST_FENCE: &str =
    "masked_reductions_landed=true masked_dot_wired=true masked_norm_sq_wired=true production_wired=false physics_green=false master=false";

/// Compile-time fence — production/master/physics GREEN flip not authorized.
const _: () = assert!(!LINEAR_PHYSICS_GREEN);
const _: () = assert!(!LINEAR_PRODUCTION_WIRED);
const _: () = assert!(!LINEAR_MASTER);

/// Typed probe for masked linear-algebra posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub masked_reductions_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for Burn masked Krylov reductions.
#[must_use]
pub fn linear_honest_posture_bundle() -> LinearPostureProbe {
    LinearPostureProbe {
        physics_green: LINEAR_PHYSICS_GREEN,
        production_wired: LINEAR_PRODUCTION_WIRED,
        master: LINEAR_MASTER,
        masked_reductions_landed: LINEAR_MASKED_REDUCTIONS_LANDED,
        honest_fence: LINEAR_HONEST_FENCE,
        posture_tag: LINEAR_POSTURE_TAG,
        deepen_cell: W29_LINEAR_DEEPEN_CELL,
    }
}

/// Masked reductions SSOT landed with production/master composition honestly open.
#[must_use]
pub fn linear_posture_honest(probe: &LinearPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.masked_reductions_landed
        && probe.honest_fence.contains("masked_reductions_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// \(\sum_i (a_i m_i)^2\) — masked squared norm.
pub fn masked_norm_sq<B: Backend<FloatElem = f32>>(
    a: &Tensor<B, 3>,
    mask: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let batch = a.dims()[0];
    let n = a.dims()[1];
    let am = a.clone().mul(mask.clone());
    am.clone()
        .mul(am)
        .reshape([batch, n * 3])
        .sum_dim(1)
        .reshape([batch])
}

/// \(\sum_i a_i b_i m_i\) — masked dot product.
pub fn masked_dot<B: Backend<FloatElem = f32>>(
    a: &Tensor<B, 3>,
    b: &Tensor<B, 3>,
    mask: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let batch = a.dims()[0];
    let n = a.dims()[1];
    a.clone()
        .mul(b.clone())
        .mul(mask.clone())
        .reshape([batch, n * 3])
        .sum_dim(1)
        .reshape([batch])
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn linear_honest_posture_refuses_green_and_production() {
        let probe = linear_honest_posture_bundle();
        assert!(linear_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.masked_reductions_landed);
        assert_eq!(probe.deepen_cell, W29_LINEAR_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, LINEAR_POSTURE_TAG);
    }

    #[test]
    fn linear_masked_dot_zeros_dirichlet_dofs() {
        let device = Default::default();
        // [B=1, N=2, 3] — node 0 free, node 1 Dirichlet (mask 0)
        let a = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 2.0, 3.0, 10.0, 20.0, 30.0], [1, 2, 3].into()),
            &device,
        );
        let b = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0], [1, 2, 3].into()),
            &device,
        );
        let mask = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 1.0, 1.0, 0.0, 0.0, 0.0], [1, 2, 3].into()),
            &device,
        );
        let d = masked_dot(&a, &b, &mask).into_data().value;
        assert_eq!(d.len(), 1);
        assert!((d[0] - 6.0).abs() < 1e-5, "masked_dot = {}", d[0]);
    }

    #[test]
    fn linear_masked_norm_sq_zeros_dirichlet_dofs() {
        let device = Default::default();
        let a = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0], [1, 2, 3].into()),
            &device,
        );
        let mask = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 1.0, 1.0, 0.0, 0.0, 0.0], [1, 2, 3].into()),
            &device,
        );
        let nsq = masked_norm_sq(&a, &mask).into_data().value;
        assert_eq!(nsq.len(), 1);
        // 1^2+2^2+3^2 = 14; Dirichlet DOFs excluded
        assert!((nsq[0] - 14.0).abs() < 1e-5, "masked_norm_sq = {}", nsq[0]);
    }

    #[test]
    fn linear_masked_dot_equals_norm_sq_when_a_eq_b() {
        let device = Default::default();
        let a = Tensor::<B, 3>::from_data(
            Data::new(vec![0.5_f32, -1.0, 2.0, 3.0, -4.0, 0.0], [1, 2, 3].into()),
            &device,
        );
        let mask = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 0.0, 1.0, 1.0, 0.0, 1.0], [1, 2, 3].into()),
            &device,
        );
        let dot = masked_dot(&a, &a, &mask).into_data().value[0];
        let nsq = masked_norm_sq(&a, &mask).into_data().value[0];
        assert!((dot - nsq).abs() < 1e-5, "dot={dot} nsq={nsq}");
    }
}

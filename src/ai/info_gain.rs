// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Epistemic **surrogates** for topology / policy gateways.
//!
//! # Open work
//!
//! **Today:** batch **scalar** mean-squared-delta surrogates ([`suggested_info_gain_from_state_delta`],
//! [`suggested_info_gain_from_batched_nodal_scalars`]) — cheap, differentiable hooks for gateways
//! without a graph-resolved information estimator.
//!
//! **Next (no large refactor):** optional **nodal / per-channel** extensions or plug-in MI estimators
//! at call sites where the integration boundary can justify them; **Landauer-consistent bit budgets**
//! stay explicitly caller-defined until a calibrated channel model connects measurements to bits.
//!
//! # What this module is *not*
//!
//! Nothing here computes **mutual information**, **conditional entropy**, or a
//! **Landauer‑consistent bit budget** from measurements. Those require an estimator
//! (e.g. contrastive / variational bounds, binning with finite‑sample correction, or a
//! calibrated channel model) and, where applicable, machine-checked statements in the
//! companion formal repository — not a squared‑error heuristic.
//!
//! # Formal proofs and documentation
//!
//! - Machine-checked developments and proof obligations for the wider UMST programme live in
//!   [**`umst-formal`**](https://github.com/tytolabs/umst-formal) (Lean / Coq / Agda tracks as
//!   described there). This crate stays executable numerics; **do not** treat rustdoc here as a
//!   certificate of information-theoretic correctness.
//! - Discrete calculus, conservation statements, and thermodynamic gate *intent* are discussed in
//!   [**Mathematical Foundations**](https://github.com/tytolabs/umst-manifold/blob/main/docs/Mathematical-Foundations.md)
//!   and [**Validation**](https://github.com/tytolabs/umst-manifold/blob/main/docs/Validation.md)
//!   in this repository.
//!
//! # Gateway contract
//!
//! [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`] and
//! [`crate::ai::cbf::ThermodynamicCBF::verify_tensor_update`] consume a batch vector
//! `Tensor<B, 1>` whose elements are **fed into the Landauer branch as “bits resolved” only by
//! convention of the caller**; the same method batch-sums **`d_int`** for the Clausius–Duhem material
//! branch (see [`ThermodynamicCBF::k_phys_dint_to_joules`](crate::ai::cbf::ThermodynamicCBF::k_phys_dint_to_joules)).
//! Until a real MI (or otherwise justified) estimator is wired in,
//! call sites should treat [`suggested_info_gain_from_state_delta`] as a **non-negative,
//! differentiable magnitude** for development — and document any rescaling at the integration
//! boundary.

use burn::tensor::{backend::Backend, Tensor};

/// LO harness morphism kind — mean squared state delta, **not** mutual information.
pub const SURROGATE_KIND: &str = "mean_squared_state_delta";

/// Honest admission: default MSE path is **not** production-wired to a calibrated MI channel.
pub const PRODUCTION_WIRED: bool = false;

/// Honest admission: surrogate magnitude does **not** certify physics GREEN.
pub const PHYSICS_GREEN: bool = false;

/// Oracle replay posture for integration harnesses (EGM-082 consumer-drift witness).
pub const ORACLE_CARGO_TEST_STATUS: &str = "BLOCKED";

/// Compile-time witness that the LO harness stays below production / physics claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurrogateAdmissionPosture {
    /// Callable MSE hooks only; histogram MI behind `epistemic-ppo`.
    DevelopmentSurrogate,
}

/// Active admission posture for the default (non-epistemic) integration path.
#[must_use]
pub const fn surrogate_admission_posture() -> SurrogateAdmissionPosture {
    SurrogateAdmissionPosture::DevelopmentSurrogate
}

/// Honest null fence: no `PRODUCTION_WIRED`, no `PHYSICS_GREEN`, oracle replay blocked.
#[must_use]
pub fn honest_fence_stub() -> bool {
    !PRODUCTION_WIRED
        && !PHYSICS_GREEN
        && ORACLE_CARGO_TEST_STATUS == "BLOCKED"
        && matches!(
            surrogate_admission_posture(),
            SurrogateAdmissionPosture::DevelopmentSurrogate
        )
}

/// Batched mean squared delta between two **aligned** second-order tensors.
///
/// # Tensor contract
///
/// - `baseline` and `proposed` must have identical shape `[B, D]` with `B ≥ 1`, `D ≥ 1`.
/// - Axis `0` is the **batch** dimension and must match [`crate::core::traits::PhysicalResult`]
///   fields that are reduced with `sum_dim(1)` in [`crate::ai::ppo::ManifoldGateway`] (e.g.
///   dissipation shaped `[B, N]` so that `d_int` is `[B]`).
/// - Axis `1` is a **flat feature** dimension. Typical uses:
///   - **Policy logits placeholder:** `D = num_actions` (or `D = num_actions × heads`).
///   - **UMST nodal scalars:** reshape a nodal slice `[N, F]` to `[1, N * F]` when a single graph
///     is evaluated as one batch row, or supply `[B, N * F]` / `[B, N, F]` via
///     [`suggested_info_gain_from_batched_nodal_scalars`].
///
/// # Epistemic meaning (explicitly limited)
///
/// Returns, per batch row `b`, the **mean** \(\frac{1}{D} \sum_j (p_{bj} - x_{bj})^2\).
/// This is a **squared‑error sensitivity** surrogate: it is **not** mutual information, not
/// conditional entropy, and not an experimental MI estimate. For proof‑relevant information
/// measures see [**`umst-formal`**](https://github.com/tytolabs/umst-formal).
///
/// # Panics
///
/// Debug builds: [`debug_assert_eq!`] on shapes. Release: mismatched shapes are a logic error;
/// keep shapes consistent with the gateway batch axis.
pub fn suggested_info_gain_from_state_delta<B: Backend<FloatElem = f32>>(
    baseline: Tensor<B, 2>,
    proposed: Tensor<B, 2>,
) -> Tensor<B, 1> {
    let shape_b = baseline.dims();
    let shape_p = proposed.dims();
    debug_assert_eq!(
        shape_b, shape_p,
        "suggested_info_gain_from_state_delta: baseline/proposed shape mismatch: {shape_b:?} vs {shape_p:?}"
    );
    assert_eq!(
        shape_b, shape_p,
        "suggested_info_gain_from_state_delta: baseline/proposed shape mismatch"
    );
    let b = shape_b[0];
    let d = shape_b[1];
    assert!(
        d > 0,
        "suggested_info_gain_from_state_delta: D must be positive"
    );

    proposed
        .sub(baseline)
        .powf_scalar(2.0)
        .sum_dim(1)
        .div_scalar(d as f32)
        .reshape([b])
}

/// Same surrogate as [`suggested_info_gain_from_state_delta`], for nodal UMST scalar blocks
/// shaped `[B, N, F]` (batch × active nodes × scalar channels).
///
/// Internally reshapes to `[B, N·F]` then applies the mean squared delta. Layout of `F` must match
/// between baseline and proposed (see [`crate::core::umst_schema`] for shared column semantics).
pub fn suggested_info_gain_from_batched_nodal_scalars<B: Backend<FloatElem = f32>>(
    baseline: Tensor<B, 3>,
    proposed: Tensor<B, 3>,
) -> Tensor<B, 1> {
    let shape_b = baseline.dims();
    let shape_p = proposed.dims();
    debug_assert_eq!(
        shape_b, shape_p,
        "suggested_info_gain_from_batched_nodal_scalars: shape mismatch: {shape_b:?} vs {shape_p:?}"
    );
    assert_eq!(
        shape_b, shape_p,
        "suggested_info_gain_from_batched_nodal_scalars: baseline/proposed shape mismatch"
    );
    let [batch, n, f] = shape_b;
    let d = n * f;
    assert!(
        d > 0,
        "suggested_info_gain_from_batched_nodal_scalars: N·F must be positive"
    );

    suggested_info_gain_from_state_delta(baseline.reshape([batch, d]), proposed.reshape([batch, d]))
}

#[cfg(feature = "epistemic-ppo")]
pub use crate::ai::epistemic_mi::{
    clamp_mi_for_landauer, EpistemicStateTracker, MutualInfoEstimator,
};

/// Histogram MI → batch `info_gain` tensor for the Landauer CBF branch (R2 envelope).
///
/// Updates `estimator` with `(state, observation)` host vectors, clamps to `ln 2`, and returns
/// `Tensor<B, 1>` shaped `[1]` for gateway admission.
#[cfg(feature = "epistemic-ppo")]
pub fn histogram_info_gain_tensor<B: Backend<FloatElem = f32>>(
    estimator: &mut MutualInfoEstimator,
    state: &[f64],
    observation: &[f64],
    device: &B::Device,
) -> Tensor<B, 1> {
    estimator.update(state, observation);
    let bits = clamp_mi_for_landauer(estimator.estimate()) as f32;
    Tensor::from_floats([bits], device)
}

/// Mean over active nodes of the first `dim` scalar columns → `f64` vector (host MI probe).
#[cfg(feature = "epistemic-ppo")]
pub fn nodal_scalar_means<B: Backend<FloatElem = f32>>(
    scalars: &Tensor<B, 2>,
    dim: usize,
) -> Vec<f64> {
    let [n, f] = scalars.dims();
    let dim = dim.min(f);
    if n == 0 || dim == 0 {
        return vec![0.0; dim];
    }
    let flat: Vec<f32> = scalars.clone().into_data().value;
    let mut out = vec![0.0_f64; dim];
    for j in 0..dim {
        let mut sum = 0.0_f64;
        for i in 0..n {
            sum += flat[i * f + j] as f64;
        }
        out[j] = sum / n as f64;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    #[test]
    fn honest_fence_pins_no_production_green() {
        assert_eq!(SURROGATE_KIND, "mean_squared_state_delta");
        assert!(!PRODUCTION_WIRED);
        assert!(!PHYSICS_GREEN);
        assert_eq!(ORACLE_CARGO_TEST_STATUS, "BLOCKED");
        assert!(honest_fence_stub());
        assert_eq!(
            surrogate_admission_posture(),
            SurrogateAdmissionPosture::DevelopmentSurrogate
        );
    }

    #[test]
    fn state_delta_matches_manual_mse_per_batch() {
        let dev = device();
        // B=2, D=2: second row has larger change → larger surrogate
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![0.0_f32, 0.0_f32, 1.0_f32, 1.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 1.0_f32, 3.0_f32, 3.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [2]);
        let row0 = (1.0_f32 + 1.0_f32) / 2.0_f32;
        let row1 = (4.0_f32 + 4.0_f32) / 2.0_f32;
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], row0, epsilon = 1.0e-5);
        assert_abs_diff_eq!(v[1], row1, epsilon = 1.0e-5);
    }

    #[test]
    fn state_delta_zero_when_baseline_equals_proposed() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32], Shape::new([2, 2])),
            &dev,
        );
        let proposed = baseline.clone();
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        for &row in &v {
            assert_abs_diff_eq!(row, 0.0_f32, epsilon = 1.0e-6);
        }
    }

    #[test]
    fn state_delta_outputs_are_non_negative() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(
                vec![-2.0_f32, 0.5_f32, 1.0_f32, -1.0_f32],
                Shape::new([2, 2]),
            ),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(
                vec![2.0_f32, -0.5_f32, -3.0_f32, 1.0_f32],
                Shape::new([2, 2]),
            ),
            &dev,
        );
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        let v: Vec<f32> = g.into_data().value;
        for &row in &v {
            assert!(
                row >= 0.0_f32,
                "MSE surrogate must be non-negative, got {row}"
            );
        }
    }

    #[test]
    fn state_delta_larger_delta_yields_larger_surrogate() {
        let dev = device();
        let baseline = Tensor::<B, 2>::zeros([1, 3], &dev);
        let small = Tensor::<B, 2>::from_data(
            Data::new(vec![0.1_f32, 0.1_f32, 0.1_f32], Shape::new([1, 3])),
            &dev,
        );
        let large = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 1.0_f32, 1.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let g_small = suggested_info_gain_from_state_delta(baseline.clone(), small);
        let g_large = suggested_info_gain_from_state_delta(baseline, large);
        let small_v = g_small.into_data().value[0];
        let large_v = g_large.into_data().value[0];
        assert!(
            large_v > small_v,
            "larger delta must yield larger surrogate"
        );
    }

    #[test]
    fn state_delta_single_feature_dimension() {
        let dev = device();
        let baseline =
            Tensor::<B, 2>::from_data(Data::new(vec![2.0_f32, 4.0_f32], Shape::new([2, 1])), &dev);
        let proposed =
            Tensor::<B, 2>::from_data(Data::new(vec![5.0_f32, 1.0_f32], Shape::new([2, 1])), &dev);
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 9.0_f32, epsilon = 1.0e-5);
        assert_abs_diff_eq!(v[1], 9.0_f32, epsilon = 1.0e-5);
    }

    #[test]
    fn state_delta_symmetric_under_baseline_proposed_swap() {
        let dev = device();
        let baseline = Tensor::<B, 2>::from_data(
            Data::new(vec![1.0_f32, 2.0_f32, 3.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let proposed = Tensor::<B, 2>::from_data(
            Data::new(vec![4.0_f32, 5.0_f32, 6.0_f32], Shape::new([1, 3])),
            &dev,
        );
        let g_fwd = suggested_info_gain_from_state_delta(baseline.clone(), proposed.clone());
        let g_rev = suggested_info_gain_from_state_delta(proposed, baseline);
        let fwd: Vec<f32> = g_fwd.into_data().value;
        let rev: Vec<f32> = g_rev.into_data().value;
        assert_abs_diff_eq!(fwd[0], rev[0], epsilon = 1.0e-6);
    }

    #[test]
    fn state_delta_batch_axis_matches_gateway_contract() {
        let dev = device();
        let b = 4usize;
        let d = 3usize;
        let baseline = Tensor::<B, 2>::zeros([b, d], &dev);
        let proposed =
            Tensor::<B, 2>::from_data(Data::new(vec![1.0_f32; b * d], Shape::new([b, d])), &dev);
        let g = suggested_info_gain_from_state_delta(baseline, proposed);
        assert_eq!(g.dims(), [b]);
        let v: Vec<f32> = g.into_data().value;
        assert_eq!(v.len(), b);
        for &row in &v {
            assert_abs_diff_eq!(row, 1.0_f32, epsilon = 1.0e-5);
        }
    }

    #[test]
    fn nodal_scalars_agree_with_flattened_equivalent() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.0_f32, 2.0_f32, 4.0_f32, 6.0_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let p = Tensor::<B, 3>::from_data(
            Data::new(
                vec![1.0_f32, 3.0_f32, 5.0_f32, 7.0_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let g3 = suggested_info_gain_from_batched_nodal_scalars(b.clone(), p.clone());
        let g2 = suggested_info_gain_from_state_delta(b.reshape([1, 4]), p.reshape([1, 4]));
        let a: Vec<f32> = g3.into_data().value;
        let c: Vec<f32> = g2.into_data().value;
        assert_abs_diff_eq!(a[0], c[0], epsilon = 1.0e-5);
    }

    #[test]
    fn nodal_scalars_multi_batch_rows_independent() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![
                    0.0_f32, 0.0_f32, //
                    1.0_f32, 1.0_f32, //
                    0.0_f32, 0.0_f32, //
                    0.0_f32, 0.0_f32,
                ],
                Shape::new([2, 2, 2]),
            ),
            &dev,
        );
        let p = Tensor::<B, 3>::from_data(
            Data::new(
                vec![
                    1.0_f32, 1.0_f32, //
                    1.0_f32, 1.0_f32, //
                    2.0_f32, 0.0_f32, //
                    0.0_f32, 0.0_f32,
                ],
                Shape::new([2, 2, 2]),
            ),
            &dev,
        );
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        assert_eq!(g.dims(), [2]);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 0.5_f32, epsilon = 1.0e-5);
        assert_abs_diff_eq!(v[1], 1.0_f32, epsilon = 1.0e-5);
    }

    #[test]
    fn nodal_scalars_zero_when_unchanged() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.5_f32, 1.5_f32, 2.5_f32, 3.5_f32],
                Shape::new([1, 2, 2]),
            ),
            &dev,
        );
        let p = b.clone();
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 0.0_f32, epsilon = 1.0e-6);
    }

    #[test]
    fn nodal_scalars_single_node_single_channel() {
        let dev = device();
        let b = Tensor::<B, 3>::from_data(Data::new(vec![3.0_f32], Shape::new([1, 1, 1])), &dev);
        let p = Tensor::<B, 3>::from_data(Data::new(vec![7.0_f32], Shape::new([1, 1, 1])), &dev);
        let g = suggested_info_gain_from_batched_nodal_scalars(b, p);
        let v: Vec<f32> = g.into_data().value;
        assert_abs_diff_eq!(v[0], 16.0_f32, epsilon = 1.0e-5);
    }
}

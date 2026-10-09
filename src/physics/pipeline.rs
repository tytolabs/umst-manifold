// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! FP §5 — `Result` Kleisli helpers for [`ThmcState`](super::solvers::ThmcState) composition (no IO, no solver state).
//!
//! The outer THMC post-step tick reads as a composition — `fracture → sync → gate → advance_time` —
//! chained through [`ok_state`] / [`and_then_unit`] / [`map_result`] instead of an imperative
//! `mut state` script. The one caller is [`super::solvers::thmc_epilogue::thmc_post_step_epilogue`]
//! (compiled with `thmc-coupled`).
//!
//! **Inner-loop exemption:** CG / PCG Krylov iterations and dense FD Newton hosts stay imperative —
//! see [`docs/FP_FIXED_POINT_CANONICAL.md`](../../docs/FP_FIXED_POINT_CANONICAL.md) and
//! [`super::solvers::fixed_point`] (tensor `mut` inner loops are not functor-wrapped).

use burn::tensor::backend::Backend;

use super::error::PhysicsError;
use super::solvers::ThmcState;

/// Carrier for FP §5 Kleisli composition over [`ThmcState`].
pub(crate) type ThmcStateResult<B> = Result<ThmcState<B>, PhysicsError>;

/// Monadic unit (η): lift a solved carrier into the success channel.
#[inline]
pub(crate) fn ok_state<B: Backend<FloatElem = f32>>(state: ThmcState<B>) -> ThmcStateResult<B> {
    Ok(state)
}

/// Functor map on the `Result` carrier (errors short-circuit).
#[inline]
pub(crate) fn map_result<B, F>(result: ThmcStateResult<B>, f: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcState<B>,
{
    result.map(f)
}

/// Kleisli bind over a unit effect: run `effect` on the carrier; preserve `state` on `Ok(())`.
///
/// Typical for UMST writeback (`sync_thmc_to_umst`) where the morphism returns `Result<(), E>`.
#[inline]
pub(crate) fn and_then_unit<B, F>(state: ThmcState<B>, effect: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(&ThmcState<B>) -> Result<(), PhysicsError>,
{
    effect(&state).map(|_| state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::backend::Backend;
    use burn::tensor::Tensor;
    use burn_ndarray::NdArray;

    use crate::physics::solvers::ThmcState;

    type TestBackend = NdArray<f32>;

    fn toy_state(dev: &<TestBackend as Backend>::Device) -> ThmcState<TestBackend> {
        let batch = 1usize;
        let n = 2usize;
        ThmcState::from_tensors(
            Tensor::zeros([batch, n, 1], dev),
            Tensor::zeros([batch, n, 1], dev),
            Tensor::zeros([batch, n, 1], dev),
            Tensor::zeros([batch, n, 1], dev),
            Tensor::zeros([batch, n, 1], dev),
            0.0,
        )
    }

    #[test]
    fn map_result_preserves_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = map_result(Ok(state), |mut s| {
            s.time = 4.0;
            s
        })
        .expect(
            "map_result on Ok carrier must preserve Ok channel and set time field (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn and_then_unit_preserves_state_on_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = and_then_unit(state, |_| Ok(())).expect(
            "and_then_unit on Ok unit effect must preserve carrier (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn and_then_unit_short_circuits_on_err() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let err = PhysicsError::InvariantViolation {
            context: "pipeline::tests::and_then_unit_short_circuits_on_err",
        };
        let out = and_then_unit(state, |_| Err(err.clone()));
        match out {
            Err(e) => assert_eq!(e, err),
            Ok(_) => panic!(
                "and_then_unit must short-circuit on Err (FP §6 Track G kleisli pipeline witness)"
            ),
        }
    }
}

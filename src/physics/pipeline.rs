// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP §5 — `Result` Kleisli helpers for [`ThmcState`](super::solvers::ThmcState) composition (no IO, no solver state).
//!
//! Outer THMC ticks should read as mathematical compositions:
//! `validate_pre → newton_loop → fracture → sync → gate → advance_time`, chained via
//! [`and_then_state`] / [`map_state`] instead of imperative `mut state` scripts.
//!
//! **Inner-loop exemption:** CG / PCG Krylov iterations and dense FD Newton hosts stay imperative —
//! see [`docs/FP_FIXED_POINT_CANONICAL.md`](../../docs/FP_FIXED_POINT_CANONICAL.md) and
//! [`super::solvers::fixed_point`] (tensor `mut` inner loops are not functor-wrapped).

use burn::tensor::backend::Backend;

use super::error::PhysicsError;
use super::solvers::ThmcState;

/// Kleisli bind for `ThmcState` morphisms: `(A → Result<B,E>)` chained left-to-right.
#[inline]
pub(crate) fn and_then_state<B, F>(
    state: ThmcState<B>,
    f: F,
) -> Result<ThmcState<B>, PhysicsError>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> Result<ThmcState<B>, PhysicsError>,
{
    f(state)
}

/// Functor map on the success channel (errors short-circuit).
#[inline]
pub(crate) fn map_state<B, F>(
    state: ThmcState<B>,
    f: F,
) -> Result<ThmcState<B>, PhysicsError>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcState<B>,
{
    Ok(f(state))
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
    fn and_then_state_short_circuits_on_err() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let err = PhysicsError::InvariantViolation {
            context: "pipeline::tests::and_then_state_short_circuits_on_err",
        };
        let out = and_then_state(state, |_| Err(err.clone()));
        assert_eq!(out.unwrap_err(), err);
    }

    #[test]
    fn and_then_state_chains_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = and_then_state(state, |mut s| {
            s.time = 1.0;
            Ok(s)
        })
        .expect("and_then_state chain on Ok morphism");
        assert!((out.time - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn map_state_preserves_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = map_state(state, |mut s| {
            s.time = 2.0;
            s
        })
        .expect("map_state on toy ThmcState");
        assert!((out.time - 2.0).abs() < f32::EPSILON);
    }
}

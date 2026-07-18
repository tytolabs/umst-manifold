// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP §5 — `Result` Kleisli helpers for [`ThmcState`](super::solvers::ThmcState) composition (no IO, no solver state).
//!
//! Outer THMC ticks should read as mathematical compositions:
//! `validate_pre → newton_loop → fracture → sync → gate → advance_time`, chained via
//! [`ok_state`] / [`and_then_result`] / [`map_result`] instead of imperative `mut state` scripts.
//!
//! Compiled only with `thmc-coupled` — primary production caller is
//! [`super::solvers::thmc_epilogue::thmc_post_step_epilogue`].
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

/// Kleisli bind on the `Result` carrier — mirrors orchestrator [`fold_plan_step`](crate::physics::orchestration::TopologyPhysicsOrchestrator::fold_plan_step).
#[inline]
pub(crate) fn and_then_result<B, F>(result: ThmcStateResult<B>, f: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcStateResult<B>,
{
    result.and_then(f)
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

/// Kleisli bind for `ThmcState` morphisms: `(A → Result<B,E>)` chained left-to-right.
#[inline]
pub(crate) fn and_then_state<B, F>(state: ThmcState<B>, f: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcStateResult<B>,
{
    f(state)
}

/// Functor map on the success channel (errors short-circuit).
#[inline]
pub(crate) fn map_state<B, F>(state: ThmcState<B>, f: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcState<B>,
{
    Ok(f(state))
}

/// Sequential composition of two state morphisms: `(g ∘ f)(s) = f(s).and_then(g)`.
#[inline]
pub(crate) fn compose_state_pair<B, F, G>(
    f: F,
    g: G,
) -> impl FnOnce(ThmcState<B>) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(ThmcState<B>) -> ThmcStateResult<B>,
    G: FnOnce(ThmcState<B>) -> ThmcStateResult<B>,
{
    move |state| f(state).and_then(g)
}

/// Fold left-to-right over state morphisms (orchestrator-style Kleisli fold).
pub(crate) fn fold_state_steps<B, I, F>(state: ThmcState<B>, steps: I) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    I: IntoIterator<Item = F>,
    F: FnMut(ThmcState<B>) -> ThmcStateResult<B>,
{
    steps.into_iter().try_fold(state, |s, mut step| step(s))
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
        match out {
            Err(e) => assert_eq!(e, err),
            Ok(_) => panic!(
                "and_then_state must short-circuit on Err (FP §6 Track G kleisli pipeline witness)"
            ),
        }
    }

    #[test]
    fn and_then_result_short_circuits_on_err() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let err = PhysicsError::InvariantViolation {
            context: "pipeline::tests::and_then_result_short_circuits_on_err",
        };
        let out = and_then_result(Ok(state), |_| Err(err.clone()));
        match out {
            Err(e) => assert_eq!(e, err),
            Ok(_) => panic!(
                "and_then_result must short-circuit on Err (FP §6 Track G kleisli pipeline witness)"
            ),
        }
    }

    #[test]
    fn and_then_state_chains_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = and_then_state(state, |mut s| {
            s.time = 1.0;
            Ok(s)
        })
        .expect(
            "and_then_state on Ok morphism must chain time field on toy ThmcState (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn compose_state_pair_chains_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let composed = compose_state_pair(
            |mut s| {
                s.time = 1.0;
                Ok(s)
            },
            |mut s| {
                s.time += 2.0;
                Ok(s)
            },
        );
        let out = composed(state).expect(
            "compose_state_pair must chain morphisms left-to-right on toy ThmcState (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn fold_state_steps_matches_compose() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let bump = |delta: f32| {
            move |mut s: ThmcState<TestBackend>| {
                s.time += delta;
                Ok(s)
            }
        };
        let out = fold_state_steps(state, [bump(1.0), bump(2.0)]).expect(
            "fold_state_steps must Kleisli-fold morphisms on toy ThmcState (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn map_state_preserves_ok() {
        let dev = Default::default();
        let state = toy_state(&dev);
        let out = map_state(state, |mut s| {
            s.time = 2.0;
            s
        })
        .expect(
            "map_state on toy ThmcState must preserve Ok channel and set time field (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 2.0).abs() < f32::EPSILON);
    }
}

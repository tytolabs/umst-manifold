// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP §5 — `Result` Kleisli helpers for [`ThmcState`](super::solvers::ThmcState) composition (no IO, no solver state).
//!
//! Outer THMC ticks should read as mathematical compositions:
//! `validate_pre → newton_loop → fracture → sync → gate → advance_time`, chained via
//! [`ok_state`] / [`and_then_result`] / [`and_then_unit`] / [`map_result`] / [`or_else_result`]
//! instead of imperative `mut state` scripts.
//!
//! Compiled only with `thmc-coupled` — primary production caller is
//! [`super::solvers::thmc_epilogue::thmc_post_step_epilogue`].
//!
//! **Inner-loop exemption:** CG / PCG Krylov iterations and dense FD Newton hosts stay imperative —
//! see [`docs/FP_FIXED_POINT_CANONICAL.md`](../../docs/FP_FIXED_POINT_CANONICAL.md) and
//! [`super::solvers::fixed_point`] (tensor `mut` inner loops are not functor-wrapped).
//!
//! # Honest boundary (W29-065)
//!
//! Kleisli `Result` helpers over [`ThmcState`] are the **outer-tick composition surface**
//! (`ok_state` / `and_then_*` / `map_*` / `or_else_result` / `compose_state_pair` /
//! `fold_state_steps`). Primary production caller under `thmc-coupled` is
//! [`super::solvers::thmc_epilogue::thmc_post_step_epilogue`]. Unit contracts:
//! `cargo test --manifest-path umst-manifold/Cargo.toml --features thmc-coupled pipeline --lib`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5. Inner Krylov / Newton hosts
//! stay imperative.

/// W29 deepen cell — THMC Kleisli pipeline honest fence bundle.
pub const W29_PIPELINE_DEEPEN_CELL: &str = "W29-065-PIPELINE";

/// Honest posture tag — Result Kleisli helpers landed; fleet production wiring refused.
pub const PIPELINE_POSTURE_TAG: &str = "honest-thmc-kleisli-pipeline-research-lane";

/// Honest physics posture — pipeline unit contracts pass; does not certify fleet physics GREEN.
pub const PIPELINE_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by Kleisli helpers alone.
pub const PIPELINE_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const PIPELINE_MASTER: bool = false;

/// Whether outer-tick Kleisli helpers (`ok_state` / bind / map / fold) are landed.
pub const PIPELINE_KLEISLI_HELPERS_LANDED: bool = true;

/// Whether CG / PCG / dense FD Newton inner loops are functor-wrapped (honestly deferred).
pub const PIPELINE_INNER_LOOP_FUNCTOR_WRAPPED: bool = false;

/// Whether `thmc_post_step_epilogue` is the measured primary caller under `thmc-coupled`.
pub const PIPELINE_EPILOGUE_CALLER_WIRED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const PIPELINE_HONEST_FENCE: &str =
    "kleisli_helpers_landed=true epilogue_caller_wired=true inner_loop_functor_wrapped=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!PIPELINE_PRODUCTION_WIRED);
const _: () = assert!(!PIPELINE_PHYSICS_GREEN);
const _: () = assert!(!PIPELINE_MASTER);
const _: () = assert!(!PIPELINE_INNER_LOOP_FUNCTOR_WRAPPED);
const _: () = assert!(PIPELINE_KLEISLI_HELPERS_LANDED);
const _: () = assert!(PIPELINE_EPILOGUE_CALLER_WIRED);

/// Typed probe for THMC Kleisli pipeline posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelinePostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub kleisli_helpers_landed: bool,
    pub inner_loop_functor_wrapped: bool,
    pub epilogue_caller_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for THMC Kleisli pipeline helpers.
#[must_use]
pub fn pipeline_honest_posture_bundle() -> PipelinePostureProbe {
    PipelinePostureProbe {
        physics_green: PIPELINE_PHYSICS_GREEN,
        production_wired: PIPELINE_PRODUCTION_WIRED,
        master: PIPELINE_MASTER,
        kleisli_helpers_landed: PIPELINE_KLEISLI_HELPERS_LANDED,
        inner_loop_functor_wrapped: PIPELINE_INNER_LOOP_FUNCTOR_WRAPPED,
        epilogue_caller_wired: PIPELINE_EPILOGUE_CALLER_WIRED,
        honest_fence: PIPELINE_HONEST_FENCE,
        posture_tag: PIPELINE_POSTURE_TAG,
        deepen_cell: W29_PIPELINE_DEEPEN_CELL,
    }
}

/// Kleisli pipeline SSOT landed with production/master/GREEN composition honestly open.
#[must_use]
pub fn pipeline_posture_honest(probe: &PipelinePostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.kleisli_helpers_landed
        && !probe.inner_loop_functor_wrapped
        && probe.epilogue_caller_wired
        && probe.honest_fence.contains("kleisli_helpers_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the Kleisli pipeline surface.
#[must_use]
pub fn pipeline_refuse_overclaim(probe: &PipelinePostureProbe) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("PIPELINE_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("PIPELINE_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("PIPELINE_MASTER must stay false — not claimed by Kleisli helpers alone");
    }
    if probe.inner_loop_functor_wrapped {
        return Err("inner Krylov/Newton loops must stay imperative (not functor-wrapped)");
    }
    if !probe.kleisli_helpers_landed {
        return Err("kleisli helpers must stay landed");
    }
    if !probe.epilogue_caller_wired {
        return Err("thmc_post_step_epilogue caller wiring must stay measured true");
    }
    if !pipeline_posture_honest(probe) {
        return Err("pipeline posture fence inconsistent");
    }
    Ok(())
}

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

/// Recover on the `Result` carrier (Kleisli alt): `Err(e)` delegates to `f(e)`.
#[inline]
pub(crate) fn or_else_result<B, F>(result: ThmcStateResult<B>, f: F) -> ThmcStateResult<B>
where
    B: Backend<FloatElem = f32>,
    F: FnOnce(PhysicsError) -> ThmcStateResult<B>,
{
    result.or_else(f)
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
    fn pipeline_honest_posture_refuses_green_and_production() {
        let probe = pipeline_honest_posture_bundle();
        assert!(pipeline_posture_honest(&probe));
        assert!(pipeline_refuse_overclaim(&probe).is_ok());
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.inner_loop_functor_wrapped);
        assert!(probe.kleisli_helpers_landed);
        assert!(probe.epilogue_caller_wired);
        assert_eq!(probe.deepen_cell, W29_PIPELINE_DEEPEN_CELL);
        assert!(probe
            .honest_fence
            .contains("inner_loop_functor_wrapped=false"));
        assert!(probe.honest_fence.contains("epilogue_caller_wired=true"));
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

    #[test]
    fn or_else_result_recovers_on_err() {
        let dev = Default::default();
        let err = PhysicsError::InvariantViolation {
            context: "pipeline::tests::or_else_result_recovers_on_err",
        };
        let out = or_else_result(Err(err), |_| {
            let mut s = toy_state(&dev);
            s.time = 5.0;
            Ok(s)
        })
        .expect(
            "or_else_result must recover Ok carrier from Err via alt morphism (FP §6 Track G kleisli pipeline witness)",
        );
        assert!((out.time - 5.0).abs() < f32::EPSILON);
    }
}

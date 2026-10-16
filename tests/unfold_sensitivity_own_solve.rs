// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The adjoint runs only on a state it solved itself (cell UNFOLD-SENSITIVITY-ONLY-FROM-OWN-SOLVE).
//!
//! `solve_then_sensitivity` is the single public route to a sensitivity; the crate-private
//! `implicit_sensitivity` is pinned unreachable by a `compile_fail` doctest. A composite that
//! binds a solved state to a step returning an unsolved value keeps the solve's certificate,
//! and has no route to the adjoint. The residual is `R(u, θ) = u³ + θ₀·u − θ₁`.

use burn::backend::Autodiff;
use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use ordered_float::NotNan;
use umst_manifold::solve::{solve_then_sensitivity, AdjointRefuse, ForwardSolve, ImplicitModel};
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    StrategyRung, UnfoldStop,
};
use umst_math::solve_kleisli::{nested_unfold, Converged, NestedRefuse};

type B = Autodiff<NdArray<f64>>;

const THETA: [f64; 2] = [1.5, 2.0];

fn r(u: f64) -> f64 {
    u.powi(3) + THETA[0] * u - THETA[1]
}

fn forward(
    start: f64,
    joules: f64,
) -> ForwardSolve<
    impl Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>,
    impl FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>,
> {
    ForwardSolve {
        initial: vec![start],
        residual_of: |u: &Vec<f64>| {
            NotNan::new(r(u[0]).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        },
        step: |u: &Vec<f64>, _rung: StrategyRung| {
            Ok(vec![u[0] - r(u[0]) / (3.0 * u[0] * u[0] + THETA[0])])
        },
        stop: UnfoldStop::new(
            ProblemTolerance::from_problem(1.0, 1e-13).expect("tolerance"),
            EnergyBudget::from_joules_at(joules, 293.15).expect("budget"),
            ProblemProgressWindow::from_krylov_restart(1).expect("window"),
            1.0,
        )
        .expect("stop"),
    }
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(1e-18).expect("meter")
}

fn residual_t(u: Tensor<B, 1>, th: Tensor<B, 1>) -> Tensor<B, 1> {
    let t0 = th.clone().narrow(0, 0, 1);
    let t1 = th.narrow(0, 1, 1);
    u.clone() * u.clone() * u.clone() + t0 * u - t1
}

fn model() -> ImplicitModel<
    'static,
    fn(Tensor<B, 1>, Tensor<B, 1>) -> Tensor<B, 1>,
    fn(Tensor<B, 1>) -> Tensor<B, 1>,
> {
    ImplicitModel {
        theta: &THETA,
        residual: residual_t,
        target: |u| u,
    }
}

#[test]
fn the_sensitivity_is_taken_at_the_state_its_own_solve_returns() {
    // Started far from the root, the route solves before it differentiates.
    let (state, s) = solve_then_sensitivity::<B, _, _, _, _, _>(
        forward(5.0, 1e-12),
        &meter(),
        &model(),
        &NdArrayDevice::default(),
    );
    let state = state.expect("forward converges");
    let s = s.expect("sensitivity");
    let u = state.value()[0];
    assert!(r(u).abs() <= 1e-13);
    assert_eq!(state.chain().links().len(), 1);
    assert!(s.forward_residual.hi() <= s.forward_tolerance.lo());
    let a = 3.0 * u * u + THETA[0];
    assert!((s.dj_dtheta[0] + u / a).abs() <= 1e-12);
    assert!((s.dj_dtheta[1] - 1.0 / a).abs() <= 1e-12);
}

#[test]
fn a_composite_ending_in_an_unsolved_value_keeps_a_certificate_that_does_not_describe_it() {
    let ForwardSolve {
        initial,
        residual_of,
        step,
        stop,
    } = forward(1.0, 1e-12);
    let forged = nested_unfold(initial, residual_of, step, stop, &meter())
        .and_then(|c| c.and_then(|_| Ok(Converged::unit(vec![5.0]))))
        .expect("bind of a pure step");
    // The chain still certifies the solve, while the value 5.0 solves nothing: this is the
    // state the crate-private adjoint can no longer be handed.
    assert_eq!(forged.chain().links().len(), 1);
    assert!(forged.chain().meets());
    assert!(r(forged.value()[0]).abs() > 100.0);
}

#[test]
fn a_forward_solve_that_does_not_converge_gets_no_sensitivity() {
    let (state, s) = solve_then_sensitivity::<B, _, _, _, _, _>(
        forward(5.0, 1e-19),
        &meter(),
        &model(),
        &NdArrayDevice::default(),
    );
    assert!(matches!(state, Err(NestedRefuse::BudgetSpent { .. })));
    assert!(matches!(
        s,
        Err(AdjointRefuse::NotConverged(
            NestedRefuse::BudgetSpent { .. }
        ))
    ));
}

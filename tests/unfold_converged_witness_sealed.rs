// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! A converged state is a sealed witness (cell UNFOLD-CONVERGED-WITNESS-SEALED).
//!
//! A residual certificate exists only where `unfold` measured a residual, a converged
//! composite with a certificate comes only from a converged solve, and the adjoint refuses
//! a state no solve produced. The residual is `R(u, θ) = u³ + θ₀·u − θ₁`.

use burn::backend::Autodiff;
use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use ordered_float::NotNan;
use umst_manifold::solve::{solve_then_sensitivity, AdjointRefuse, ForwardSolve, ImplicitModel};
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    StrategyRung, UnfoldStop,
};
use umst_math::solve_kleisli::{lift, nested_unfold, Converged, Nested, NestedRefuse};

type B = Autodiff<NdArray<f64>>;
type Forward = ForwardSolve<
    Box<dyn Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>>,
    Box<dyn FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>>,
>;

const THETA: [f64; 2] = [1.5, 2.0];

fn stop() -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, 1e-13).expect("tolerance"),
        EnergyBudget::from_joules_at(1e-12, 293.15).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn r(u: f64, th: [f64; 2]) -> f64 {
    u.powi(3) + th[0] * u - th[1]
}

fn forward(th: [f64; 2], start: f64) -> Forward {
    ForwardSolve {
        initial: vec![start],
        residual_of: Box::new(move |u: &Vec<f64>| {
            NotNan::new(r(u[0], th).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        }),
        step: Box::new(move |u: &Vec<f64>, _rung: StrategyRung| {
            Ok(vec![u[0] - r(u[0], th) / (3.0 * u[0] * u[0] + th[0])])
        }),
        stop: stop(),
    }
}

fn solve(th: [f64; 2], start: f64) -> Nested<Vec<f64>> {
    let ForwardSolve {
        initial,
        residual_of,
        step,
        stop,
    } = forward(th, start);
    nested_unfold(
        initial,
        residual_of,
        step,
        stop,
        &FixedJouleMeter::from_joules(1e-18).expect("meter"),
    )
}

fn residual_t(u: Tensor<B, 1>, th: Tensor<B, 1>) -> Tensor<B, 1> {
    let t0 = th.clone().narrow(0, 0, 1);
    let t1 = th.narrow(0, 1, 1);
    u.clone() * u.clone() * u.clone() + t0 * u - t1
}

fn sensitivity<R, S>(forward: ForwardSolve<R, S>) -> Result<Vec<f64>, AdjointRefuse>
where
    R: Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>,
    S: FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>,
{
    solve_then_sensitivity::<B, _, _, _, _, _>(
        forward,
        &FixedJouleMeter::from_joules(1e-18).expect("meter"),
        &ImplicitModel {
            theta: &THETA,
            residual: residual_t,
            target: |u: Tensor<B, 1>| u,
        },
        &NdArrayDevice::default(),
    )
    .1
    .map(|s| s.dj_dtheta)
}

#[test]
fn a_state_no_solve_produced_carries_no_certificate() {
    // The exact root of u³ + 1.5u − 2 is near 0.8761; a unit state there was never solved.
    let unit = Converged::unit(vec![0.8761]);
    assert!(unit.chain().links().is_empty());
    // The adjoint is reachable only through solve_then_sensitivity (the compile_fail
    // doctest of umst_manifold::solve::implicit_adjoint pins that), so this state has no
    // route to a sensitivity.
}

#[test]
fn a_solved_state_gets_its_sensitivity() {
    let s = sensitivity(forward(THETA, 1.0)).expect("converged state");
    let u = solve(THETA, 1.0).expect("converged").value()[0];
    // du/dθ = −(∂R/∂u)⁻¹ ∂R/∂θ with ∂R/∂θ = (u, −1).
    let a = 3.0 * u * u + THETA[0];
    assert!((s[0] + u / a).abs() <= 1e-12);
    assert!((s[1] - 1.0 / a).abs() <= 1e-12);
}

#[test]
fn every_certificate_states_what_unfold_measured() {
    let state = solve(THETA, 1.0)
        .and_then(|c| c.and_then(|u| solve([2.0, u[0]], 1.0)))
        .expect("both converge");
    let links = state.chain().links();
    assert_eq!(links.len(), 2);
    for link in links {
        assert!(link.meets());
        assert_eq!(link.meets(), link.residual() <= link.tolerance().residual());
    }
}

#[test]
fn only_a_converged_outcome_lifts_to_a_converged_composite() {
    let stalled = nested_unfold(
        vec![1.0],
        |_u: &Vec<f64>| Ok(NotNan::new(1.0).expect("finite")),
        |u: &Vec<f64>, _rung: StrategyRung| Ok(u.clone()),
        stop(),
        &FixedJouleMeter::from_joules(1e-18).expect("meter"),
    );
    assert!(matches!(stalled, Err(NestedRefuse::Stalled { .. })));
    let spend = match &stalled {
        Err(refuse) => refuse.ledger().entries()[0],
        Ok(_) => unreachable!("stalled"),
    };
    assert!(lift::<Vec<f64>>(Err(CombinatorRefuse::Unmeasured), spend).is_err());
    let stalling = ForwardSolve {
        initial: vec![1.0],
        residual_of: |_u: &Vec<f64>| Ok(NotNan::new(1.0).expect("finite")),
        step: |u: &Vec<f64>, _rung: StrategyRung| Ok(u.clone()),
        stop: stop(),
    };
    assert!(matches!(
        sensitivity(stalling),
        Err(AdjointRefuse::NotConverged(NestedRefuse::Stalled { .. }))
    ));
}

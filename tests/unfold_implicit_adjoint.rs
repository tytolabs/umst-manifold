// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! `du/dθ` by one Burn adjoint solve at `Converged` only (cell UNFOLD-IMPLICIT-ADJOINT).
//!
//! The implicit system
//! `R₁ = u₁³ + θ₀u₁ + u₂ − θ₁`, `R₂ = u₂³ + 2u₂ − u₁ − θ₀θ₁`
//! is solved by Newton under `unfold`; the adjoint sensitivity is checked against
//! central finite differences of re-solved states and against the implicit function
//! theorem evaluated by hand.

use burn::backend::Autodiff;
use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use ordered_float::NotNan;
use umst_manifold::solve::{
    solve_then_sensitivity, AdjointRefuse, ForwardSolve, ImplicitModel, ImplicitSensitivity,
};
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    StrategyRung, UnfoldStop,
};
use umst_math::solve_kleisli::{nested_unfold, Nested, NestedRefuse};

type B = Autodiff<NdArray<f64>>;

const THETA: [f64; 2] = [1.5, 2.0];

fn stop(joules: f64) -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, 1e-13).expect("tolerance"),
        EnergyBudget::from_joules_at(joules, 293.15).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(1e-18).expect("meter")
}

fn r_f64(u: &[f64], th: &[f64]) -> [f64; 2] {
    [
        u[0].powi(3) + th[0] * u[0] + u[1] - th[1],
        u[1].powi(3) + 2.0 * u[1] - u[0] - th[0] * th[1],
    ]
}

/// `∂R/∂u` at `(u, θ)`.
fn a_f64(u: &[f64], th: &[f64]) -> [[f64; 2]; 2] {
    [
        [3.0 * u[0] * u[0] + th[0], 1.0],
        [-1.0, 3.0 * u[1] * u[1] + 2.0],
    ]
}

fn solve2(a: [[f64; 2]; 2], b: [f64; 2]) -> [f64; 2] {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    [
        (b[0] * a[1][1] - a[0][1] * b[1]) / det,
        (a[0][0] * b[1] - a[1][0] * b[0]) / det,
    ]
}

fn max_abs_residual(th: [f64; 2]) -> impl Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse> {
    move |u: &Vec<f64>| {
        let r = r_f64(u, &th);
        NotNan::new(r[0].abs().max(r[1].abs())).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    }
}

fn newton(
    th: [f64; 2],
) -> impl FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse> {
    move |u: &Vec<f64>, _rung: StrategyRung| {
        let d = solve2(a_f64(u, &th), r_f64(u, &th));
        Ok(vec![u[0] - d[0], u[1] - d[1]])
    }
}

fn forward(th: [f64; 2]) -> Nested<Vec<f64>> {
    nested_unfold(
        vec![1.0, 1.0],
        max_abs_residual(th),
        newton(th),
        stop(1e-12),
        &meter(),
    )
}

fn residual_t(u: Tensor<B, 1>, th: Tensor<B, 1>) -> Tensor<B, 1> {
    let u1 = u.clone().narrow(0, 0, 1);
    let u2 = u.narrow(0, 1, 1);
    let t0 = th.clone().narrow(0, 0, 1);
    let t1 = th.narrow(0, 1, 1);
    let r1 =
        u1.clone() * u1.clone() * u1.clone() + t0.clone() * u1.clone() + u2.clone() - t1.clone();
    let r2 = u2.clone() * u2.clone() * u2.clone() + u2.clone() * 2.0 - u1 - t0 * t1;
    Tensor::cat(vec![r1, r2], 0)
}

fn component(k: usize) -> impl Fn(Tensor<B, 1>) -> Tensor<B, 1> {
    move |u: Tensor<B, 1>| u.narrow(0, k, 1)
}

fn sensitivity(k: usize) -> (Vec<f64>, ImplicitSensitivity) {
    let (state, s) = solve_then_sensitivity::<B, _, _, _, _, _>(
        ForwardSolve {
            initial: vec![1.0, 1.0],
            residual_of: max_abs_residual(THETA),
            step: newton(THETA),
            stop: stop(1e-12),
        },
        &meter(),
        &ImplicitModel {
            theta: &THETA,
            residual: residual_t,
            target: component(k),
        },
        &NdArrayDevice::default(),
    );
    let u = state.expect("forward converges").value().clone();
    (u, s.expect("sensitivity"))
}

#[test]
fn adjoint_agrees_with_finite_differences_and_the_implicit_function_theorem() {
    let h = 1e-6;
    for k in 0..2 {
        let (u, s) = sensitivity(k);
        assert!(
            s.adjoint_residual_inf <= 1e-14,
            "{}",
            s.adjoint_residual_inf
        );
        assert!(s.forward_residual.hi() <= s.forward_tolerance.lo());
        // The implicit function theorem by hand: du/dθ = −A⁻¹ ∂R/∂θ.
        let a = a_f64(&u, &THETA);
        let r_theta = [[u[0], -1.0], [-THETA[1], -THETA[0]]];
        for p in 0..2 {
            let col = solve2(a, [r_theta[0][p], r_theta[1][p]]);
            let ift = -col[k];
            assert!((s.dj_dtheta[p] - ift).abs() <= 1e-12, "k={k} p={p}");
            // Central differences of re-solved converged states.
            let mut plus = THETA;
            let mut minus = THETA;
            plus[p] += h;
            minus[p] -= h;
            let up = forward(plus).expect("θ+h converges").value()[k];
            let um = forward(minus).expect("θ−h converges").value()[k];
            let fd = (up - um) / (2.0 * h);
            assert!((s.dj_dtheta[p] - fd).abs() <= 1e-7, "k={k} p={p} fd={fd}");
        }
    }
}

#[test]
fn a_stalled_state_is_refused() {
    let (_, out) = solve_then_sensitivity::<B, _, _, _, _, _>(
        ForwardSolve {
            initial: vec![1.0, 1.0],
            residual_of: max_abs_residual(THETA),
            step: |u: &Vec<f64>, _rung: StrategyRung| Ok(u.clone()),
            stop: stop(1e-12),
        },
        &meter(),
        &ImplicitModel {
            theta: &THETA,
            residual: residual_t,
            target: component(0),
        },
        &NdArrayDevice::default(),
    );
    assert!(matches!(
        out,
        Err(AdjointRefuse::NotConverged(NestedRefuse::Stalled { .. }))
    ));
}

#[test]
fn a_budget_spent_state_is_refused() {
    let (_, out) = solve_then_sensitivity::<B, _, _, _, _, _>(
        ForwardSolve {
            initial: vec![1.0, 1.0],
            residual_of: max_abs_residual(THETA),
            step: newton(THETA),
            stop: stop(1e-19),
        },
        &meter(),
        &ImplicitModel {
            theta: &THETA,
            residual: residual_t,
            target: component(0),
        },
        &NdArrayDevice::default(),
    );
    assert!(matches!(
        out,
        Err(AdjointRefuse::NotConverged(
            NestedRefuse::BudgetSpent { .. }
        ))
    ));
}

#[test]
fn a_singular_jacobian_is_refused() {
    // R = u² − θ at θ = 0: u = 0 converges at once and ∂R/∂u = 0.
    let (_, out) = solve_then_sensitivity::<B, _, _, _, _, _>(
        ForwardSolve {
            initial: vec![0.0],
            residual_of: |u: &Vec<f64>| {
                NotNan::new(u[0] * u[0]).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
            },
            step: |u: &Vec<f64>, _rung: StrategyRung| Ok(u.clone()),
            stop: stop(1e-12),
        },
        &meter(),
        &ImplicitModel {
            theta: &[0.0],
            residual: |u: Tensor<B, 1>, th: Tensor<B, 1>| u.clone() * u - th,
            target: |u: Tensor<B, 1>| u,
        },
        &NdArrayDevice::default(),
    );
    assert_eq!(out, Err(AdjointRefuse::SingularJacobian { column: 0 }));
}

#[test]
fn a_target_of_the_wrong_length_is_refused() {
    let (_, out) = solve_then_sensitivity::<B, _, _, _, _, _>(
        ForwardSolve {
            initial: vec![1.0, 1.0],
            residual_of: max_abs_residual(THETA),
            step: newton(THETA),
            stop: stop(1e-12),
        },
        &meter(),
        &ImplicitModel {
            theta: &THETA,
            residual: residual_t,
            target: |u: Tensor<B, 1>| u,
        },
        &NdArrayDevice::default(),
    );
    assert_eq!(
        out,
        Err(AdjointRefuse::ShapeMismatch {
            expected: 1,
            found: 2
        })
    );
}

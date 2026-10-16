// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Calibration at fixed resolution (cell CALIBRATION-FIT-FIXED-RESOLUTION).
//!
//! The forward model is steady nonlinear drag: a force `f_e` drives a velocity `v` with
//! `R_e(v, θ) = θ₀·v + θ₁·v³ − f_e = 0`, so the dissipated power is
//! `D(v) = θ₀·v² + θ₁·v⁴`, non-negative for every `v` exactly when `θ₀, θ₁ ≥ 0`. A planted
//! dataset from known parameters is recovered, and a parameter vector the core gate
//! refuses stops the fit.

use burn::backend::Autodiff;
use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use ordered_float::NotNan;
use umst_gate::core_gate::ScalarConstitutiveResponse;
use umst_manifold::solve::{calibrate, CalibrationModel, CalibrationRefuse, ForwardSolve};
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    StrategyRung, UnfoldStop,
};
use umst_math::solve_kleisli::nested_unfold;

type B = Autodiff<NdArray<f64>>;
type ResidualOf = Box<dyn Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>>;
type Step = Box<dyn FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>>;

const FORCES: [f64; 4] = [0.5, 1.0, 2.0, 4.0];

fn stop(tolerance: f64, joules: f64, window: u32) -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, tolerance).expect("tolerance"),
        EnergyBudget::from_joules_at(joules, 293.15).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(window).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(1e-18).expect("meter")
}

fn drag_forward(force: f64, theta: &[f64]) -> ForwardSolve<ResidualOf, Step> {
    let (a, b) = (theta[0], theta[1]);
    let r = move |v: f64| a * v + b * v * v * v - force;
    ForwardSolve {
        initial: vec![force / a],
        residual_of: Box::new(move |v: &Vec<f64>| {
            NotNan::new(r(v[0]).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        }),
        step: Box::new(move |v: &Vec<f64>, _rung: StrategyRung| {
            Ok(vec![v[0] - r(v[0]) / (a + 3.0 * b * v[0] * v[0])])
        }),
        stop: stop(1e-13, 1e-12, 1),
    }
}

/// Observations planted from `theta`: each velocity solved to the forward tolerance.
fn planted(theta: [f64; 2]) -> Vec<f64> {
    FORCES
        .iter()
        .map(|&f| {
            let ForwardSolve {
                initial,
                residual_of,
                step,
                stop,
            } = drag_forward(f, &theta);
            nested_unfold(initial, residual_of, step, stop, &meter())
                .expect("planted solve converges")
                .value()[0]
        })
        .collect()
}

struct Drag {
    observed: Vec<f64>,
}

impl CalibrationModel<B> for Drag {
    type ResidualOf = ResidualOf;
    type Step = Step;

    fn experiments(&self) -> usize {
        FORCES.len()
    }

    fn observed(&self, experiment: usize) -> f64 {
        self.observed[experiment]
    }

    fn forward(&self, experiment: usize, theta: &[f64]) -> ForwardSolve<ResidualOf, Step> {
        drag_forward(FORCES[experiment], theta)
    }

    fn residual(&self, experiment: usize, v: Tensor<B, 1>, theta: Tensor<B, 1>) -> Tensor<B, 1> {
        let a = theta.clone().narrow(0, 0, 1);
        let b = theta.narrow(0, 1, 1);
        a * v.clone() + b * v.clone() * v.clone() * v - FORCES[experiment]
    }

    fn observable(&self, _experiment: usize, v: Tensor<B, 1>) -> Tensor<B, 1> {
        v
    }

    fn predicted(&self, _experiment: usize, v: &[f64]) -> f64 {
        v[0]
    }

    /// `D(v) = θ₀·v² + θ₁·v⁴ ≥ 0` for every `v` exactly when both coefficients are
    /// non-negative, so the gate judges the smaller coefficient (the dissipation at `v = 1`
    /// of the weaker term).
    fn response(&self, theta: &[f64]) -> ScalarConstitutiveResponse {
        ScalarConstitutiveResponse {
            dissipation: theta[0].min(theta[1]),
            power_input: 0.0,
        }
    }
}

fn fit(observed: Vec<f64>, theta0: Vec<f64>) -> Result<Vec<f64>, CalibrationRefuse> {
    calibrate::<B, _, _>(
        &Drag { observed },
        theta0,
        stop(1e-11, 1e-9, 2),
        &meter(),
        &NdArrayDevice::default(),
    )
    .map(|c| c.value().clone())
}

#[test]
fn planted_parameters_are_recovered() {
    let truth = [2.0, 0.5];
    let theta = fit(planted(truth), vec![1.0, 1.0]).expect("fit converges");
    // Stated error: the planted velocities carry the forward residual 1e-13, which the
    // conditioning of this design lifts to well below 1e-8 in θ.
    for (got, want) in theta.iter().zip(truth) {
        assert!((got - want).abs() <= 1e-8, "{theta:?}");
    }
}

#[test]
fn an_inadmissible_start_is_refused_before_any_solve() {
    let out = fit(planted([2.0, 0.5]), vec![1.0, -1.0]);
    assert!(matches!(
        out,
        Err(CalibrationRefuse::Inadmissible { ref theta, .. }) if theta == &vec![1.0, -1.0]
    ));
}

#[test]
fn a_step_into_an_inadmissible_theta_is_refused() {
    // Data planted with θ₁ < 0: a fit that follows it leaves the passive set.
    let out = fit(planted([2.0, -0.05]), vec![2.0, 0.5]);
    match out {
        Err(CalibrationRefuse::Inadmissible { theta, outcome }) => {
            assert!(theta[1] < 0.0);
            assert!(!outcome.is_accepted());
        }
        other => panic!("expected the gate's refusal, got {other:?}"),
    }
}

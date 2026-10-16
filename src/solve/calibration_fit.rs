// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Calibration at fixed resolution: Gauss–Newton over the parameters `θ` as an outer
//! unfold around the forward solves of each experiment.
//!
//! For experiments `e = 1..m` with observed values `y_e`, the misfit is
//! `r_e(θ) = J_e(u_e(θ)) − y_e`, where `u_e(θ)` solves `R_e(u, θ) = 0`. One outer step
//! solves every forward problem with [`solve_then_sensitivity`], which returns the
//! converged state and the row `dJ_e/dθ` by one adjoint solve, then takes the
//! Gauss–Newton step `θ₊ = θ − (JᵀJ)⁻¹ Jᵀ r` (Nocedal and Wright 2006, *Numerical
//! Optimization*, Springer, doi:10.1007/978-0-387-40065-5, the Gauss–Newton method of the
//! nonlinear least-squares chapter).
//!
//! Each accepted `θ` passes the core admissibility gate ([`core_gate`]: mass conservation
//! and Clausius–Duhem) on the constitutive response the model states for it. A parameter
//! update moves no mass, so the mass conjunct is true and the decision rests on the
//! dissipation inequality. The outer loop is a [`nested_unfold`]: its stop is the misfit
//! tolerance, its budget and its progress window, and its refusals are typed
//! ([`CalibrationRefuse`]). The adaptive extension (model and resolution moves between
//! steps) is cell `MANIFOLD-CALIBRATION-SOLVE`.

use std::cell::RefCell;

use burn::tensor::backend::AutodiffBackend;
use burn::tensor::Tensor;
use ordered_float::NotNan;
use umst_gate::core_gate::{core_gate, CoreGateOutcome, ScalarConstitutiveResponse};
use umst_math::solve_combinator::{CombinatorRefuse, StepEnergyMeter, StrategyRung, UnfoldStop};
use umst_math::solve_kleisli::{nested_unfold, Converged, NestedRefuse};

use super::implicit_adjoint::{
    solve_dense, solve_then_sensitivity, AdjointRefuse, ForwardSolve, ImplicitModel,
};

/// A forward model with experiments and observations, stated for the fit.
pub trait CalibrationModel<B: AutodiffBackend> {
    /// Residual norm of a forward state.
    type ResidualOf: Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>;
    /// One forward solver step.
    type Step: FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>;

    /// Number of experiments.
    fn experiments(&self) -> usize;
    /// Observed value `y_e`.
    fn observed(&self, experiment: usize) -> f64;
    /// The forward solve of experiment `e` at `θ`.
    fn forward(
        &self,
        experiment: usize,
        theta: &[f64],
    ) -> ForwardSolve<Self::ResidualOf, Self::Step>;
    /// `R_e(u, θ)` on Burn tensors.
    fn residual(&self, experiment: usize, u: Tensor<B, 1>, theta: Tensor<B, 1>) -> Tensor<B, 1>;
    /// `J_e(u)` on Burn tensors, of length 1.
    fn observable(&self, experiment: usize, u: Tensor<B, 1>) -> Tensor<B, 1>;
    /// `J_e(u)` on a converged state.
    fn predicted(&self, experiment: usize, u: &[f64]) -> f64;
    /// The constitutive response the core gate judges at `θ`.
    fn response(&self, theta: &[f64]) -> ScalarConstitutiveResponse;
}

/// Why the fit returns no parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum CalibrationRefuse {
    /// The model states no experiment.
    NoExperiments,
    /// A parameter vector failed the core admissibility gate.
    Inadmissible {
        /// The parameters refused.
        theta: Vec<f64>,
        /// The gate's verdict and terms.
        outcome: CoreGateOutcome,
    },
    /// A forward solve did not converge.
    Forward {
        /// Experiment index.
        experiment: usize,
        /// The nested solve's refusal.
        refuse: NestedRefuse,
    },
    /// The adjoint refused.
    Adjoint {
        /// Experiment index.
        experiment: usize,
        /// The adjoint's refusal.
        refuse: AdjointRefuse,
    },
    /// `JᵀJ` is singular: the experiments do not identify every parameter.
    Unidentified,
    /// The outer unfold stalled, spent its budget or refused.
    Outer(NestedRefuse),
}

/// Fit `θ` from `theta0` by Gauss–Newton under `stop`; each accepted `θ` passes the core
/// gate. Returns the converged parameters with the outer spend ledger and certificates.
pub fn calibrate<B, M, Meter>(
    model: &M,
    theta0: Vec<f64>,
    stop: UnfoldStop,
    meter: &Meter,
    device: &B::Device,
) -> Result<Converged<Vec<f64>>, CalibrationRefuse>
where
    B: AutodiffBackend,
    M: CalibrationModel<B>,
    Meter: StepEnergyMeter,
{
    if model.experiments() == 0 {
        return Err(CalibrationRefuse::NoExperiments);
    }
    admit(model, &theta0)?;
    let cause: RefCell<Option<CalibrationRefuse>> = RefCell::new(None);
    let carry = |refuse: CalibrationRefuse| {
        *cause.borrow_mut() = Some(refuse);
        CombinatorRefuse::StepRefused
    };
    let outcome = nested_unfold(
        theta0,
        |theta: &Vec<f64>| misfit(model, theta, meter).map_err(carry),
        |theta: &Vec<f64>, _rung| {
            gauss_newton_step::<B, M, Meter>(model, theta, meter, device).map_err(carry)
        },
        stop,
        meter,
    );
    outcome.map_err(|outer| match cause.borrow_mut().take() {
        Some(inner) => inner,
        None => CalibrationRefuse::Outer(outer),
    })
}

/// The core gate on the response the model states at `θ`.
fn admit<B: AutodiffBackend, M: CalibrationModel<B>>(
    model: &M,
    theta: &[f64],
) -> Result<(), CalibrationRefuse> {
    let outcome = core_gate(&model.response(theta), true, 0.0);
    if outcome.is_accepted() {
        Ok(())
    } else {
        Err(CalibrationRefuse::Inadmissible {
            theta: theta.to_vec(),
            outcome,
        })
    }
}

/// `max_e |J_e(u_e(θ)) − y_e|` over converged forward states.
fn misfit<B: AutodiffBackend, M: CalibrationModel<B>, Meter: StepEnergyMeter>(
    model: &M,
    theta: &[f64],
    meter: &Meter,
) -> Result<NotNan<f64>, CalibrationRefuse> {
    let mut worst = 0.0_f64;
    for e in 0..model.experiments() {
        let ForwardSolve {
            initial,
            residual_of,
            step,
            stop,
        } = model.forward(e, theta);
        let state = nested_unfold(initial, residual_of, step, stop, meter).map_err(|refuse| {
            CalibrationRefuse::Forward {
                experiment: e,
                refuse,
            }
        })?;
        worst = worst.max((model.predicted(e, state.value()) - model.observed(e)).abs());
    }
    NotNan::new(worst).map_err(|_| CalibrationRefuse::Unidentified)
}

/// One Gauss–Newton step from `θ`, admitted by the core gate.
fn gauss_newton_step<B, M, Meter>(
    model: &M,
    theta: &[f64],
    meter: &Meter,
    device: &B::Device,
) -> Result<Vec<f64>, CalibrationRefuse>
where
    B: AutodiffBackend,
    M: CalibrationModel<B>,
    Meter: StepEnergyMeter,
{
    let p = theta.len();
    let mut jtj = vec![vec![0.0; p]; p];
    let mut jtr = vec![0.0; p];
    for e in 0..model.experiments() {
        let (state, sensitivity) = solve_then_sensitivity::<B, _, _, _, _, _>(
            model.forward(e, theta),
            meter,
            &ImplicitModel {
                theta,
                residual: |u, t| model.residual(e, u, t),
                target: |u| model.observable(e, u),
            },
            device,
        );
        let state = state.map_err(|refuse| CalibrationRefuse::Forward {
            experiment: e,
            refuse,
        })?;
        let row = sensitivity
            .map_err(|refuse| CalibrationRefuse::Adjoint {
                experiment: e,
                refuse,
            })?
            .dj_dtheta;
        let r = model.predicted(e, state.value()) - model.observed(e);
        for i in 0..p {
            jtr[i] += row[i] * r;
            for k in 0..p {
                jtj[i][k] += row[i] * row[k];
            }
        }
    }
    let delta = solve_dense(jtj, jtr).map_err(|_| CalibrationRefuse::Unidentified)?;
    let next: Vec<f64> = theta.iter().zip(&delta).map(|(t, d)| t - d).collect();
    admit(model, &next)?;
    Ok(next)
}

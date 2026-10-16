// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Sensitivity of an implicit solve by one adjoint solve, at `Converged` only.
//!
//! For a residual `R(u, θ) = 0` solved to a converged state `u*` and a target
//! `J(u)`, the implicit function theorem gives
//! `dJ/dθ = −λᵀ ∂R/∂θ` with `(∂R/∂u)ᵀ λ = ∂J/∂u`, all at `(u*, θ)`.
//! With `J(u) = u_k` this is the row `du_k/dθ`.
//!
//! - Burn reverse mode supplies `∂J/∂u`, the rows of `∂R/∂u` (one pass per row) and
//!   the adjoint contraction `λᵀ ∂R/∂θ` (one pass).
//! - The adjoint system is solved densely in binary64 by Gaussian elimination with
//!   partial pivoting; an exactly zero pivot is the typed refusal
//!   [`AdjointRefuse::SingularJacobian`]. The solve's residual `‖Aᵀλ − ∂J/∂u‖∞` is
//!   measured afterwards and reported, not assumed.
//! - The state must be a [`Converged`](umst_math::solve_kleisli::Converged) composite from
//!   [`umst_math::solve_kleisli`] with a non-empty tolerance chain that meets; `Stalled`,
//!   `BudgetSpent` or a refusal is [`AdjointRefuse::NotConverged`], and a unit state that
//!   no solve produced is [`AdjointRefuse::Uncertified`].
//!
//! The derivative holds at the converged state `u*`, whose residual the certificate
//! chain bounds (`forward_residual`, `forward_tolerance`); the sensitivity carries no
//! claim beyond that state. Exact unilateral contact has no adjoint here: it stays a
//! typed absence (unfold blueprint §13); regularised contact enters as a smooth `R`.
//!
//! Consumer: [`super::calibration_fit`] takes one Gauss–Newton row per experiment from
//! [`solve_then_sensitivity`].
//!
//! This is the sensitivity path of `MANIFOLD-AUTODIFF-DERIVED-BRIDGE` for implicit
//! solves and supplies the dual weights of dual-weighted residual marking.

use burn::tensor::backend::{AutodiffBackend, Backend};
use burn::tensor::{Data, Shape, Tensor};
use ordered_float::NotNan;
use std::sync::Mutex;
use umst_math::solve_combinator::{CombinatorRefuse, StepEnergyMeter, StrategyRung, UnfoldStop};
use umst_math::solve_kleisli::{nested_unfold, Nested, NestedRefuse, SumEnclosure};

/// Why no sensitivity is returned.
#[derive(Clone, Debug, PartialEq)]
pub enum AdjointRefuse {
    /// The forward solve did not converge; its refusal is carried.
    NotConverged(NestedRefuse),
    /// The state carries no residual certificate (an empty tolerance chain): it was
    /// never solved, so the implicit function theorem has no converged state to act at.
    Uncertified,
    /// A certificate in the forward chain does not meet its tolerance.
    ChainNotMet,
    /// `R(u, θ)` or `J(u)` has the wrong length.
    ShapeMismatch {
        /// Required length.
        expected: usize,
        /// Length found.
        found: usize,
    },
    /// A Jacobian entry, gradient or adjoint value is NaN or infinite.
    NonFinite,
    /// A backward pass reached no gradient for `u` or `θ`: `R` or `J` does not depend on it.
    NoGradient,
    /// `∂R/∂u` has an exactly zero pivot in this column after partial pivoting.
    SingularJacobian {
        /// Elimination column at which the pivot vanished.
        column: usize,
    },
    /// An enclosure of the forward chain refused.
    Combinator(CombinatorRefuse),
}

/// `dJ/dθ` at a converged state, with the adjoint and its measured solve residual.
#[derive(Clone, Debug, PartialEq)]
pub struct ImplicitSensitivity {
    /// `dJ/dθ`, one entry per parameter.
    pub dj_dtheta: Vec<f64>,
    /// The adjoint `λ` solving `(∂R/∂u)ᵀ λ = ∂J/∂u`.
    pub adjoint: Vec<f64>,
    /// Measured `‖(∂R/∂u)ᵀ λ − ∂J/∂u‖∞` of the dense solve.
    pub adjoint_residual_inf: f64,
    /// Enclosure of the summed forward residuals of the certificate chain.
    pub forward_residual: SumEnclosure,
    /// Enclosure of the summed forward tolerances of the certificate chain.
    pub forward_tolerance: SumEnclosure,
}

/// The implicit model at fixed parameters: `R(u, θ)` and the target `J(u)`.
///
/// `residual(u, θ)` returns `R` of length `u.len()`; `target(u)` returns `J` of length 1.
pub struct ImplicitModel<'a, R, J> {
    /// The parameters `θ`.
    pub theta: &'a [f64],
    /// `R(u, θ)` on Burn tensors.
    pub residual: R,
    /// `J(u)` on Burn tensors.
    pub target: J,
}

/// The forward solve handed to [`nested_unfold`]: start, residual norm, step and stop policy.
pub struct ForwardSolve<ResidualOf, Step> {
    /// Initial state.
    pub initial: Vec<f64>,
    /// Residual norm of a state.
    pub residual_of: ResidualOf,
    /// One solver step.
    pub step: Step,
    /// Tolerance, budget, progress window and bits per step.
    pub stop: UnfoldStop,
}

/// `dJ/dθ` at the converged state of `state` by one adjoint solve.
///
/// Crate-private: a caller reaches it only through [`solve_then_sensitivity`], which hands
/// it the state its own forward solve produced, so a composite whose last value came from a
/// later pure step cannot carry the forward solve's certificates into the adjoint.
pub(crate) fn implicit_sensitivity<B, R, J>(
    state: &Nested<Vec<f64>>,
    model: &ImplicitModel<'_, R, J>,
    device: &B::Device,
) -> Result<ImplicitSensitivity, AdjointRefuse>
where
    B: AutodiffBackend,
    R: Fn(Tensor<B, 1>, Tensor<B, 1>) -> Tensor<B, 1>,
    J: Fn(Tensor<B, 1>) -> Tensor<B, 1>,
{
    let ImplicitModel {
        theta,
        residual,
        target,
    } = model;
    // burn-autodiff 0.13.2 keeps one process-wide graph server (`runtime/mutex.rs`,
    // `static SERVER`); backward passes on two threads at once drop each other's nodes, and
    // a gradient goes missing (9 of 40 parallel runs of this module's test, 0 of 40 serial,
    // 16 October 2026). The passes below therefore run one sensitivity at a time.
    let _serial = AUTODIFF_PASSES
        .lock()
        .unwrap_or_else(|held| held.into_inner());
    let certified = state
        .as_ref()
        .map_err(|r| AdjointRefuse::NotConverged(r.clone()))?;
    if certified.chain().links().is_empty() {
        return Err(AdjointRefuse::Uncertified);
    }
    if !certified.chain().meets() {
        return Err(AdjointRefuse::ChainNotMet);
    }
    let forward_residual = certified
        .chain()
        .residual_sum()
        .map_err(AdjointRefuse::Combinator)?;
    let forward_tolerance = certified
        .chain()
        .tolerance_sum()
        .map_err(AdjointRefuse::Combinator)?;
    let u = certified.value();
    let n = u.len();

    // ∂R/∂u by rows: one reverse pass per residual component.
    let mut jac = Vec::with_capacity(n);
    for i in 0..n {
        let u_t = tensor::<B>(u, device).require_grad();
        let r = residual(u_t.clone(), tensor::<B>(theta, device));
        same_len(n, r.dims()[0])?;
        let grads = r.narrow(0, i, 1).sum().backward();
        jac.push(gradient::<B>(&u_t, &grads, n)?);
    }

    // ∂J/∂u.
    let u_t = tensor::<B>(u, device).require_grad();
    let j = target(u_t.clone());
    same_len(1, j.dims()[0])?;
    let dj_du = gradient::<B>(&u_t, &j.sum().backward(), n)?;

    // (∂R/∂u)ᵀ λ = ∂J/∂u.
    let jac_t: Vec<Vec<f64>> = (0..n)
        .map(|c| jac.iter().map(|row| row[c]).collect())
        .collect();
    let adjoint = solve_dense(jac_t.clone(), dj_du.clone())?;
    let adjoint_residual_inf = jac_t
        .iter()
        .zip(&dj_du)
        .map(|(row, g)| (row.iter().zip(&adjoint).map(|(a, l)| a * l).sum::<f64>() - g).abs())
        .fold(0.0_f64, f64::max);

    // The adjoint contraction λᵀ ∂R/∂θ: one reverse pass.
    let theta_t = tensor::<B>(theta, device).require_grad();
    let r = residual(tensor::<B>(u, device), theta_t.clone());
    same_len(n, r.dims()[0])?;
    let grads = (r * tensor::<B>(&adjoint, device)).sum().backward();
    let contraction = gradient::<B>(&theta_t, &grads, theta.len())?;
    let dj_dtheta: Vec<f64> = contraction.iter().map(|c| -c).collect();
    finite(&dj_dtheta)?;
    finite(&[adjoint_residual_inf])?;

    Ok(ImplicitSensitivity {
        dj_dtheta,
        adjoint,
        adjoint_residual_inf,
        forward_residual,
        forward_tolerance,
    })
}

/// The only public route to a sensitivity: the Kleisli composite of the forward solve and
/// the adjoint, [`nested_unfold`] under
/// its own sub-meter, then [`implicit_sensitivity`] at its converged state.
///
/// ```compile_fail,E0603
/// use umst_manifold::solve::implicit_adjoint::implicit_sensitivity;
/// ```
pub fn solve_then_sensitivity<B, ResidualOf, Step, Meter, R, J>(
    forward: ForwardSolve<ResidualOf, Step>,
    meter: &Meter,
    model: &ImplicitModel<'_, R, J>,
    device: &B::Device,
) -> (Nested<Vec<f64>>, Result<ImplicitSensitivity, AdjointRefuse>)
where
    B: AutodiffBackend,
    ResidualOf: Fn(&Vec<f64>) -> Result<NotNan<f64>, CombinatorRefuse>,
    Step: FnMut(&Vec<f64>, StrategyRung) -> Result<Vec<f64>, CombinatorRefuse>,
    Meter: StepEnergyMeter,
    R: Fn(Tensor<B, 1>, Tensor<B, 1>) -> Tensor<B, 1>,
    J: Fn(Tensor<B, 1>) -> Tensor<B, 1>,
{
    let ForwardSolve {
        initial,
        residual_of,
        step,
        stop,
    } = forward;
    let state = nested_unfold(initial, residual_of, step, stop, meter);
    let sensitivity = implicit_sensitivity::<B, R, J>(&state, model, device);
    (state, sensitivity)
}

fn tensor<B: Backend>(xs: &[f64], device: &B::Device) -> Tensor<B, 1> {
    Tensor::from_data(
        Data::new(xs.to_vec(), Shape::new([xs.len()])).convert::<B::FloatElem>(),
        device,
    )
}

/// One sensitivity's backward passes at a time (see [`implicit_sensitivity`]).
static AUTODIFF_PASSES: Mutex<()> = Mutex::new(());

/// The gradient of the last backward pass with respect to `x`; a tensor the pass did
/// not reach is [`AdjointRefuse::NoGradient`].
fn gradient<B: AutodiffBackend>(
    x: &Tensor<B, 1>,
    grads: &B::Gradients,
    len: usize,
) -> Result<Vec<f64>, AdjointRefuse> {
    let values = match x.grad(grads) {
        Some(g) => g.into_data().convert::<f64>().value,
        None => return Err(AdjointRefuse::NoGradient),
    };
    same_len(len, values.len())?;
    finite(&values)?;
    Ok(values)
}

fn same_len(expected: usize, found: usize) -> Result<(), AdjointRefuse> {
    if expected == found {
        Ok(())
    } else {
        Err(AdjointRefuse::ShapeMismatch { expected, found })
    }
}

fn finite(xs: &[f64]) -> Result<(), AdjointRefuse> {
    if xs.iter().all(|x| x.is_finite()) {
        Ok(())
    } else {
        Err(AdjointRefuse::NonFinite)
    }
}

/// `M x = b` by Gaussian elimination with partial pivoting; an exactly zero pivot refuses.
pub(crate) fn solve_dense(
    mut m: Vec<Vec<f64>>,
    mut b: Vec<f64>,
) -> Result<Vec<f64>, AdjointRefuse> {
    let n = b.len();
    for col in 0..n {
        let pivot = (col..n)
            .max_by(|&i, &k| m[i][col].abs().total_cmp(&m[k][col].abs()))
            .unwrap_or(col);
        if m[pivot][col] == 0.0 {
            return Err(AdjointRefuse::SingularJacobian { column: col });
        }
        m.swap(col, pivot);
        b.swap(col, pivot);
        for row in col + 1..n {
            let factor = m[row][col] / m[col][col];
            for k in col..n {
                m[row][k] -= factor * m[col][k];
            }
            b[row] -= factor * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let tail: f64 = (row + 1..n).map(|k| m[row][k] * x[k]).sum();
        x[row] = (b[row] - tail) / m[row][row];
    }
    finite(&x)?;
    Ok(x)
}

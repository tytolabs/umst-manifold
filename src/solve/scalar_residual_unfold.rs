// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Scalar residual descent via [`umst_math::solve_combinator::unfold`].
//!
//! Replaces `repeat_controlled(max_iters, …)` for scalar / host `Copy` state when the caller
//! supplies problem tolerance, energy budget, and a step meter (not a compiled iteration cap).

use ordered_float::NotNan;
use umst_math::{
    unfold, CombinatorRefuse, EnergyBudget, ProblemProgressWindow, ProblemTolerance, SolveOutcome,
    StepEnergyMeter, StrategyRung,
};

/// Run a scalar fixed-point map until converge / stall / budget via the shared unfold combinator.
pub fn scalar_residual_unfold<M>(
    initial: f64,
    residual: impl Fn(f64) -> Result<f64, CombinatorRefuse>,
    mut step: impl FnMut(f64, StrategyRung) -> Result<f64, CombinatorRefuse>,
    tolerance: ProblemTolerance,
    budget: EnergyBudget,
    meter: &M,
    window: ProblemProgressWindow,
    bits_per_step: f64,
) -> Result<SolveOutcome<f64>, CombinatorRefuse>
where
    M: StepEnergyMeter,
{
    let residual_of = |x: &f64| {
        let r = residual(*x)?;
        NotNan::new(r).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
    };
    unfold(
        initial,
        residual_of,
        |x, rung| step(*x, rung),
        tolerance,
        budget,
        meter,
        window,
        bits_per_step,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use umst_math::{EnergyBudget, FixedJouleMeter, SolveOutcome};

    #[test]
    fn scalar_residual_unfold_heron_sqrt2_without_iter_cap() {
        let tolerance = ProblemTolerance::from_problem(1.0, 1e-12).expect("tol");
        let budget = EnergyBudget::from_joules_at(1.0, 293.15).expect("budget");
        let meter = FixedJouleMeter::from_joules(1e-6).expect("meter");
        let window = ProblemProgressWindow::from_krylov_restart(2).expect("window");
        let outcome = scalar_residual_unfold(
            1.0,
            |state| Ok((state - std::f64::consts::SQRT_2).abs()),
            |state, _rung| Ok(0.5 * (state + 2.0 / state)),
            tolerance,
            budget,
            &meter,
            window,
            1.0,
        )
        .expect("unfold");
        let root = match outcome {
            SolveOutcome::Converged { x, .. } => x,
            other => panic!("expected converge, got {other:?}"),
        };
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// registration_request: pub mod cg_stall_window;
//! Median gap between strict CG residual improvements (stall window).
//!
//! The CG stall window is the median gap between strict residual improvements. Callers
//! later feed this window to umst-math unfold. This module does not run a solver and does
//! not import `solve_combinator`.
//!
//! A solve stops only as `Converged{certificate}`, `Stalled{window evidence}`,
//! `BudgetSpent{progress}`, or a typed refusal. `n_unknowns` is escalation evidence, never
//! a stop. The stall window computed here is **not** `n_unknowns` and is not derived from
//! problem size.

/// Refusal when a stall window cannot be formed from the residual trace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StallWindowRefuse {
    /// At least one residual entry was not finite.
    NonFiniteResidual,
    /// No strict residual improvements were observed (no positive gaps).
    NonPositiveWindow,
}

/// Computes the median inter-improvement gap for a strict residual trace.
///
/// A strict improvement is an index `i > 0` with `residuals[i] < residuals[i - 1]`.
/// Gaps are distances between successive improvement indices; the first gap is the index
/// of the first improvement. The returned window is the median of those gaps (for an even
/// count, the lower of the two middle values). This quantity is unfold stall evidence, not
/// `n_unknowns`.
pub fn cg_stall_window(residuals: &[f64]) -> Result<u64, StallWindowRefuse> {
    for &r in residuals {
        if !r.is_finite() {
            return Err(StallWindowRefuse::NonFiniteResidual);
        }
    }

    let mut improvements: Vec<usize> = Vec::new();
    for i in 1..residuals.len() {
        if residuals[i] < residuals[i - 1] {
            improvements.push(i);
        }
    }

    if improvements.is_empty() {
        return Err(StallWindowRefuse::NonPositiveWindow);
    }

    let mut gaps: Vec<u64> = Vec::with_capacity(improvements.len());
    gaps.push(improvements[0] as u64);
    for k in 1..improvements.len() {
        let gap = improvements[k] - improvements[k - 1];
        gaps.push(gap as u64);
    }

    gaps.sort_unstable();
    let n = gaps.len();
    let median = if n % 2 == 1 {
        gaps[n / 2]
    } else {
        gaps[n / 2 - 1]
    };

    Ok(median)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_improvements_median_gap_one() {
        let residuals = [1.0, 0.8, 0.9, 0.4];
        assert_eq!(cg_stall_window(&residuals), Ok(1));
    }

    #[test]
    fn single_improvement_window_is_first_gap() {
        let residuals = [1.0, 0.5];
        assert_eq!(cg_stall_window(&residuals), Ok(1));
    }

    #[test]
    fn flat_residuals_refuse_non_positive_window() {
        let residuals = [1.0, 1.0, 1.0];
        assert_eq!(
            cg_stall_window(&residuals),
            Err(StallWindowRefuse::NonPositiveWindow)
        );
    }

    #[test]
    fn non_finite_residual_refuses() {
        let residuals = [1.0, f64::NAN];
        assert_eq!(
            cg_stall_window(&residuals),
            Err(StallWindowRefuse::NonFiniteResidual)
        );
    }

    /// Stall window is median improvement spacing, not problem `n_unknowns` (no size parameter).
    #[test]
    fn stall_window_is_not_n_unknowns() {
        let residuals = [10.0, 9.0, 8.0, 7.0];
        assert_eq!(cg_stall_window(&residuals), Ok(1));
    }
}

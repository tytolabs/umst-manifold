// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Core numeric mirror of `Lean/OrderStatisticsBand.lean` — sample-size budget (`n_quantile`).
//!
//! Percentile / band classification live in [`crate::kernels`] (Phase M-simd dispatch).

use crate::median_convergence;

/// Invalid-parameter reasons for [`n_quantile`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NQuantileError {
    /// `epsilon` not finite or not strictly positive.
    Epsilon,
    /// `delta` not finite or not in `(0, 1)`.
    Delta,
    /// `rho_min` not finite or not strictly positive.
    RhoMin,
    /// `q` not finite or not in `(0, 1)` (tracked for API symmetry with Lean; bound is independent of `q`).
    Q,
}

/// Theorem-derived quantile sample-size threshold — **same closed form** as [`median_convergence::n_warmup`].
///
/// Proof: `UMST.Formal.OrderStatisticsBand::order_statistic_concentration` (ceil cover; parameter `q` is tracked in Lean for the envelope registry mapping).
///
/// # Errors
///
/// Returns [`NQuantileError`] when any argument is outside the admissible open intervals (mirrors Lean side-conditions).
pub fn n_quantile(epsilon: f64, delta: f64, rho_min: f64, q: f64) -> Result<u64, NQuantileError> {
    if !epsilon.is_finite() || epsilon <= 0.0 {
        return Err(NQuantileError::Epsilon);
    }
    if !delta.is_finite() || delta <= 0.0 || delta >= 1.0 {
        return Err(NQuantileError::Delta);
    }
    if !rho_min.is_finite() || rho_min <= 0.0 {
        return Err(NQuantileError::RhoMin);
    }
    if !q.is_finite() || q <= 0.0 || q >= 1.0 {
        return Err(NQuantileError::Q);
    }
    Ok(median_convergence::n_warmup(epsilon, delta, rho_min))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::median_convergence;

    #[test]
    fn n_quantile_matches_n_warmup_for_interior_q() {
        let eps = 0.2;
        let del = 0.1;
        let rho = 0.5;
        assert_eq!(
            n_quantile(eps, del, rho, 0.25).unwrap(),
            median_convergence::n_warmup(eps, del, rho)
        );
        assert_eq!(
            n_quantile(eps, del, rho, 0.75).unwrap(),
            median_convergence::n_warmup(eps, del, rho)
        );
    }

    #[test]
    fn n_quantile_rejects_bad_q() {
        assert_eq!(n_quantile(0.1, 0.5, 0.2, 0.0), Err(NQuantileError::Q));
        assert_eq!(n_quantile(0.1, 0.5, 0.2, 1.0), Err(NQuantileError::Q));
    }

    #[test]
    fn n_quantile_monotone_in_epsilon_matches_median_path() {
        let n_lo = n_quantile(0.2, 0.1, 0.5, 0.4).unwrap();
        let n_hi = n_quantile(0.1, 0.1, 0.5, 0.4).unwrap();
        assert!(n_hi >= n_lo);
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Core numeric mirror of `Lean/MedianConvergence.lean`.

/// Theorem-derived warmup sample count `⌈ (2 / (ε² ρ_min²)) · ln(2/δ) ⌉` (natural logarithm).
///
/// Proof: `UMST.Formal.MedianConvergence::median_convergence_sample_size` (conservative ceiling covers the analytic bound).
///
/// # Panics
///
/// If `ε` or `ρ_min` are not finite and `> 0`, or if `δ` is not finite with `0 < δ < 1`, or if the
/// closed form is non-finite.
#[must_use]
pub fn n_warmup(epsilon: f64, delta: f64, rho_min: f64) -> u64 {
    assert!(
        epsilon.is_finite() && epsilon > 0.0,
        "n_warmup: epsilon must be finite and > 0"
    );
    assert!(
        delta.is_finite() && delta > 0.0 && delta < 1.0,
        "n_warmup: delta must be finite in (0, 1)"
    );
    assert!(
        rho_min.is_finite() && rho_min > 0.0,
        "n_warmup: rho_min must be finite and > 0"
    );
    let log_term = (2.0_f64 / delta).ln();
    assert!(
        log_term.is_finite() && log_term > 0.0,
        "n_warmup: log(2/delta) must be finite and positive"
    );
    let coef = 2.0 / (epsilon * epsilon * rho_min * rho_min);
    assert!(coef.is_finite() && coef > 0.0);
    let bound = coef * log_term;
    assert!(bound.is_finite() && bound > 0.0);
    let c = bound.ceil();
    assert!(c.is_finite());
    let n = c as u64;
    n.max(1)
}

/// Pragmatic cockpit warmup gate **`max(3, ⌈√W⌉)`** for window capacity `W` (with `W = 0` treated as `1`).
///
/// Proof: `UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible` (reference triple lower-bounds this expression at `W = 32`).
#[must_use]
pub fn sqrt_window_threshold(window_capacity: usize) -> u64 {
    let w = window_capacity.max(1);
    let c = (w as f64).sqrt().ceil();
    let s = if c.is_finite() && c > 0.0 {
        c as u64
    } else {
        3
    };
    s.max(3)
}

#[cfg(test)]
mod tests {
    use super::{n_warmup, sqrt_window_threshold};

    #[test]
    fn sqrt_window_matches_frugality_defaults() {
        assert_eq!(sqrt_window_threshold(32), 6);
        assert_eq!(sqrt_window_threshold(4), 3);
        assert_eq!(sqrt_window_threshold(1), 3);
        assert_eq!(sqrt_window_threshold(100), 10);
    }

    #[test]
    fn n_warmup_reference_triple() {
        assert_eq!(n_warmup(1.0, 0.5, 1.0), 3);
    }

    #[test]
    fn n_warmup_monotone_in_epsilon_numeric() {
        let n_lo = n_warmup(0.2, 0.1, 0.5);
        let n_hi = n_warmup(0.1, 0.1, 0.5);
        assert!(
            n_hi >= n_lo,
            "smaller epsilon should not decrease threshold"
        );
    }

    #[test]
    fn n_warmup_monotone_in_delta_numeric() {
        let n_lo = n_warmup(0.15, 0.2, 0.4);
        let n_hi = n_warmup(0.15, 0.05, 0.4);
        assert!(n_hi >= n_lo, "smaller delta should not decrease threshold");
    }

    #[test]
    #[should_panic(expected = "delta must be finite in (0, 1)")]
    fn n_warmup_rejects_invalid_delta() {
        let _ = n_warmup(0.1, 1.0, 0.2);
    }

    #[test]
    fn n_warmup_always_at_least_one() {
        assert!(n_warmup(0.9, 0.5, 0.2) >= 1);
    }
}

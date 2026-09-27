// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// registration_request: pub mod cg_spectral_window;
//! CG stall window from the Lanczos spectrum of the coefficient trace (second-law axiom).
//!
//! A healthy CG iteration carries a spectral contraction factor; the stall window is how many
//! steps are needed for an expected factor-of-two drop in the Gauss-quadrature lower bound on
//! the \(A\)-norm error. That window is derived from Ritz values of the Lanczos tridiagonal
//! built from CG coefficients—not from median gaps in the 2-norm residual trace.
//!
//! Strakoš, Zdeněk and Tichý, Petr, “On error estimation in the conjugate gradient method and
//! why it works in finite precision computations,” ETNA 13 (2002), 56–80. They show that the
//! Gauss-quadrature lower bound on the \(A\)-norm of the error is the Hestenes–Stiefel formula
//! and that this bound is stable in finite precision. The factor \((\sqrt{\kappa}-1)/(\sqrt{\kappa}+1)\)
//! is the classical CG convergence rate. Do not attribute that rate to a numbered theorem of ETNA 13.
//!
//! The median residual gap computed in `cg_stall_window` is not this window.
//! This module does not run a solver loop and does not call excitement selection.

/// One CG step: step length, conjugate-direction coupling, and squared residual norm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CgCoeff {
    /// CG step length \(\alpha_k\).
    pub alpha: f64,
    /// Coupling \(\beta_k\) (zero on step 0).
    pub beta: f64,
    /// Squared Euclidean residual norm \(\|r_k\|_2^2\).
    pub r_norm_sq: f64,
}

/// Spectral stall window and estimated condition data from a coefficient trace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpectralWindow {
    /// Estimated \(\kappa = \lambda_{\max}/\lambda_{\min}\) from Lanczos Ritz values.
    pub kappa_hat: f64,
    /// Estimated per-step contraction \(\rho = (\sqrt{\kappa}-1)/(\sqrt{\kappa}+1)\).
    pub rho_hat: f64,
    /// Steps for an expected factor-of-two drop in the \(A\)-norm lower bound, at least 1.
    pub window: u64,
}

/// Refusal when spectral quantities cannot be formed from the coefficient trace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpectralWindowRefuse {
    /// A coefficient field is not finite.
    NonFiniteCoeff,
    /// A step length is not strictly positive.
    NonPositiveAlpha,
    /// The coefficient trace is empty.
    EmptyRun,
    /// Lanczos coupling or Ritz spectrum is not positive definite.
    IndefiniteRitz,
}

/// Build Lanczos diagonal \(\theta\) and off-diagonal \(\delta\) from CG coefficients.
fn lanczos_tridiagonal(coeffs: &[CgCoeff]) -> Result<(Vec<f64>, Vec<f64>), SpectralWindowRefuse> {
    let n = coeffs.len();
    let mut theta = Vec::with_capacity(n);
    let mut delta = Vec::with_capacity(n.saturating_sub(1));

    for k in 0..n {
        let c = &coeffs[k];
        if !c.alpha.is_finite() || !c.beta.is_finite() || !c.r_norm_sq.is_finite() {
            return Err(SpectralWindowRefuse::NonFiniteCoeff);
        }
        if c.alpha <= 0.0 {
            return Err(SpectralWindowRefuse::NonPositiveAlpha);
        }

        let mut diag = 1.0 / c.alpha;
        if k > 0 {
            diag += coeffs[k - 1].beta / coeffs[k - 1].alpha;
        }
        theta.push(diag);

        if k + 1 < n {
            let beta_next = coeffs[k + 1].beta;
            if !beta_next.is_finite() {
                return Err(SpectralWindowRefuse::NonFiniteCoeff);
            }
            if beta_next < 0.0 {
                return Err(SpectralWindowRefuse::IndefiniteRitz);
            }
            // Edge between k and k+1: √β_{k+1} / α_k, the amendment's √β_k / α_{k−1}.
            delta.push(beta_next.sqrt() / c.alpha);
        }
    }

    Ok((theta, delta))
}

/// Number of eigenvalues strictly less than `mu` (Sturm sequence count).
fn sturm_count_less(theta: &[f64], delta: &[f64], mu: f64) -> usize {
    let n = theta.len();
    if n == 0 {
        return 0;
    }

    let mut sign_changes = 0usize;
    let mut p_prev = 1.0;
    let mut p_curr = theta[0] - mu;
    if p_curr == 0.0 {
        p_curr = -1e-30;
    }
    let mut prev_sign = 1.0;
    if prev_sign * p_curr < 0.0 {
        sign_changes += 1;
    }
    prev_sign = if p_curr < 0.0 { -1.0 } else { 1.0 };

    for i in 1..n {
        let off = delta[i - 1];
        let mut p_next = (theta[i] - mu) * p_curr - off * off * p_prev;
        if p_next == 0.0 {
            p_next = -1e-30 * prev_sign;
        }
        if prev_sign * p_next < 0.0 {
            sign_changes += 1;
        }
        prev_sign = if p_next < 0.0 { -1.0 } else { 1.0 };
        p_prev = p_curr;
        p_curr = p_next;
    }

    sign_changes
}

/// Gershgorin upper bound on the largest eigenvalue of a symmetric tridiagonal matrix.
fn gershgorin_upper(theta: &[f64], delta: &[f64]) -> f64 {
    let n = theta.len();
    match n {
        0 => 0.0,
        1 => theta[0],
        _ => {
            let mut upper = theta[0] + delta[0].abs();
            for i in 1..n - 1 {
                let row = theta[i] + delta[i - 1].abs() + delta[i].abs();
                if row > upper {
                    upper = row;
                }
            }
            let last = theta[n - 1] + delta[n - 2].abs();
            if last > upper {
                upper = last;
            }
            upper
        }
    }
}

/// Smallest and largest eigenvalues of a symmetric tridiagonal matrix.
fn tridiagonal_extreme_eigenvalues(theta: &[f64], delta: &[f64]) -> (f64, f64) {
    let n = theta.len();
    if n == 1 {
        return (theta[0], theta[0]);
    }

    let hi = gershgorin_upper(theta, delta);
    let mut low_bound = 0.0;
    let mut high_bound = hi;

    for _ in 0..128 {
        let mid = 0.5 * (low_bound + high_bound);
        if sturm_count_less(theta, delta, mid) >= 1 {
            high_bound = mid;
        } else {
            low_bound = mid;
        }
    }
    let lambda_min = high_bound;

    low_bound = lambda_min;
    high_bound = hi;
    for _ in 0..128 {
        let mid = 0.5 * (low_bound + high_bound);
        if sturm_count_less(theta, delta, mid) >= n {
            high_bound = mid;
        } else {
            low_bound = mid;
        }
    }
    let lambda_max = low_bound;

    (lambda_min, lambda_max)
}

fn contraction_window(rho_hat: f64) -> Result<u64, SpectralWindowRefuse> {
    if rho_hat == 0.0 {
        return Ok(1);
    }
    if !(rho_hat > 0.0 && rho_hat < 1.0) {
        return Err(SpectralWindowRefuse::IndefiniteRitz);
    }
    let denom = -rho_hat.ln();
    if !denom.is_finite() || denom <= 0.0 {
        return Err(SpectralWindowRefuse::IndefiniteRitz);
    }
    let raw = std::f64::consts::LN_2 / denom;
    if !raw.is_finite() {
        return Err(SpectralWindowRefuse::IndefiniteRitz);
    }
    let w = raw.ceil();
    let window = if w < 1.0 {
        1u64
    } else if w > (u64::MAX as f64) {
        u64::MAX
    } else {
        w as u64
    };
    Ok(window)
}

/// Form the spectral stall window from a full CG coefficient trace.
pub fn spectral_window(coeffs: &[CgCoeff]) -> Result<SpectralWindow, SpectralWindowRefuse> {
    if coeffs.is_empty() {
        return Err(SpectralWindowRefuse::EmptyRun);
    }

    let (theta, delta) = lanczos_tridiagonal(coeffs)?;
    let (lambda_min, lambda_max) = tridiagonal_extreme_eigenvalues(&theta, &delta);

    if lambda_min <= 0.0 || !lambda_min.is_finite() || !lambda_max.is_finite() {
        return Err(SpectralWindowRefuse::IndefiniteRitz);
    }

    let kappa_hat = lambda_max / lambda_min;
    if !kappa_hat.is_finite() || kappa_hat < 1.0 {
        return Err(SpectralWindowRefuse::IndefiniteRitz);
    }

    let sqrt_k = kappa_hat.sqrt();
    let rho_hat = (sqrt_k - 1.0) / (sqrt_k + 1.0);
    let window = contraction_window(rho_hat)?;

    Ok(SpectralWindow {
        kappa_hat,
        rho_hat,
        window,
    })
}

/// Hestenes–Stiefel delayed lower bound on the \(A\)-norm error:
/// \(\sum_{j=k}^{\min(k+\mathrm{delay},\,\mathrm{last})} \alpha_j \|r_j\|_2^2\).
pub fn a_norm_error_lower(
    coeffs: &[CgCoeff],
    k: usize,
    delay: usize,
) -> Result<f64, SpectralWindowRefuse> {
    if coeffs.is_empty() {
        return Err(SpectralWindowRefuse::EmptyRun);
    }

    let last = coeffs.len() - 1;
    if k > last {
        return Ok(0.0);
    }

    let end = if k + delay < last { k + delay } else { last };
    let mut sum = 0.0;
    for j in k..=end {
        let c = &coeffs[j];
        if !c.alpha.is_finite() || !c.r_norm_sq.is_finite() {
            return Err(SpectralWindowRefuse::NonFiniteCoeff);
        }
        if c.alpha <= 0.0 {
            return Err(SpectralWindowRefuse::NonPositiveAlpha);
        }
        let term = c.alpha * c.r_norm_sq;
        if !term.is_finite() {
            return Err(SpectralWindowRefuse::NonFiniteCoeff);
        }
        sum += term;
    }

    Ok(sum)
}

/// True when the delayed \(A\)-norm lower bound failed to halve across the spectral window
/// near the end of the trace.
pub fn stalled_a_norm(coeffs: &[CgCoeff]) -> Result<bool, SpectralWindowRefuse> {
    if coeffs.is_empty() {
        return Err(SpectralWindowRefuse::EmptyRun);
    }

    let spec = spectral_window(coeffs)?;
    let window = spec.window;
    if coeffs.len() < window as usize {
        return Ok(false);
    }

    let last_index = coeffs.len() - 1;
    let k = last_index.saturating_sub(window as usize);
    let w = window as usize;
    let k_late = k + w;
    if k_late > last_index {
        return Ok(false);
    }

    let delay = w;
    let early = a_norm_error_lower(coeffs, k, delay)?;
    let late = a_norm_error_lower(coeffs, k_late, delay)?;

    if early <= 0.0 || late <= 0.0 {
        return Ok(false);
    }

    Ok(late > early / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal_spd_window_matches_formula() {
        // Decoupled Lanczos tridiagonal with eigenvalues 1, 2, 4 (κ = 4).
        let coeffs = [
            CgCoeff {
                alpha: 1.0,
                beta: 0.0,
                r_norm_sq: 1.0,
            },
            CgCoeff {
                alpha: 0.5,
                beta: 0.0,
                r_norm_sq: 1.0,
            },
            CgCoeff {
                alpha: 0.25,
                beta: 0.0,
                r_norm_sq: 1.0,
            },
        ];

        let spec = spectral_window(&coeffs);
        let spec = match spec {
            Ok(s) => s,
            Err(e) => panic!("spectral_window refused: {:?}", e),
        };

        let kappa_expected: f64 = 4.0;
        let sqrt_k = kappa_expected.sqrt();
        let rho_expected = (sqrt_k - 1.0) / (sqrt_k + 1.0);
        let window_expected = 1u64;

        assert!((spec.kappa_hat - kappa_expected).abs() < 1e-8);
        assert!((spec.rho_hat - rho_expected).abs() < 1e-8);
        assert_eq!(spec.window, window_expected);
    }

    #[test]
    fn a_norm_delay_is_the_weighted_sum() {
        let coeffs = [
            CgCoeff {
                alpha: 0.2,
                beta: 0.0,
                r_norm_sq: 10.0,
            },
            CgCoeff {
                alpha: 0.3,
                beta: 0.1,
                r_norm_sq: 4.0,
            },
            CgCoeff {
                alpha: 0.5,
                beta: 0.0,
                r_norm_sq: 2.0,
            },
        ];

        let hand = 0.2 * 10.0 + 0.3 * 4.0;
        let est = a_norm_error_lower(&coeffs, 0, 1);
        assert_eq!(est, Ok(hand));
    }
}

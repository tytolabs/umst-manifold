// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Finite-sample Jarzynski free-energy estimator with its bias bounds.
//!
//! Works are taken in reduced units `w = βW` (multiples of k_B·T), so the estimator carries no physical constant;
//! the free-energy change comes back in the same units.
//!
//! **Estimator.** From N independent works `w_1 … w_N` of a protocol started in equilibrium,
//! `ΔF̂_N = −ln( (1/N) Σ e^{−w_i} )`, evaluated by log-sum-exp.
//!
//! **Bounds** (each follows from the integral fluctuation theorem ⟨e^{−w}⟩ = e^{−ΔF}, proved for finite
//! Gibbs-preserving protocols in `umst-formal` `Lean/FluctuationTheorem.lean`, `Protocol.integral_fluctuation`, with
//! the second law `Protocol.mean_work_ge_deltaF` derived from it):
//! - per sample, `ΔF̂_N ≤ w̄_N` (Jensen on the sample: the estimate never exceeds the sample mean work);
//! - in expectation, `ΔF ≤ E[ΔF̂_N] ≤ ⟨w⟩`: the bias is non-negative and at most the mean dissipated work
//!   `⟨w⟩ − ΔF` (Jensen on the concave logarithm, and the per-sample bound);
//! - `E[ΔF̂_N]` does not increase with N (Y. Burda, R. Grosse and R. Salakhutdinov, "Importance Weighted
//!   Autoencoders", ICLR 2016, Theorem 1, applied to the sample mean of e^{−w});
//! - for large N the bias is `Var(e^{−w}) / (2 N ⟨e^{−w}⟩²) + O(N⁻²)` (D. M. Zuckerman and T. B. Woolf, Phys. Rev.
//!   Lett. 89, 180602 (2002)); [`JarzynskiEstimate::delta_method_bias`] evaluates it from the sample and is an
//!   estimate, not a bound;
//! - when every work lies in a known support `[a, b]`, Hoeffding's inequality (W. Hoeffding, J. Am. Stat. Assoc.
//!   58, 13 (1963)) on `e^{−(w − a)} ∈ [e^{−(b − a)}, 1]` gives a confidence interval for ΔF at level 1 − δ
//!   ([`hoeffding_interval`]).
//!
//! **Self-test.** [`TwoLevelQuench`] is the exactly solvable sudden quench of a two-level system (levels 0 and ε₀,
//! switched to 0 and ε₁). Its ΔF, its work law and the exact expectation of the N-sample estimator (a binomial sum)
//! are computed in closed form; `tests/jarzynski_two_level.rs` checks the estimator and its bounds against them.

/// Refusal of an estimate: the input cannot carry one.
#[derive(Debug, Clone, PartialEq)]
pub enum JarzynskiRefusal {
    /// No samples.
    Empty,
    /// A work that is NaN or infinite, at this index.
    NonFinite {
        /// Index of the offending sample.
        index: usize,
    },
    /// A confidence level outside (0, 1).
    Confidence {
        /// The refused δ.
        delta: f64,
    },
    /// A work support whose lower end exceeds its upper end, or a sample outside it.
    Support {
        /// Index of the offending sample, when one lies outside the support.
        index: Option<usize>,
    },
}

/// A finite-sample Jarzynski estimate in units of k_B·T.
#[derive(Debug, Clone, PartialEq)]
pub struct JarzynskiEstimate {
    /// Number of samples N.
    pub samples: usize,
    /// ΔF̂_N = −ln((1/N) Σ e^{−w_i}).
    pub delta_f: f64,
    /// Sample mean work w̄_N.
    pub mean_work: f64,
    /// Sample variance of e^{−(w − w_min)} (unbiased; zero for one sample), with w_min the smallest sample.
    shifted_weight_variance: f64,
    /// Sample mean of e^{−(w − w_min)}.
    shifted_weight_mean: f64,
}

impl JarzynskiEstimate {
    /// Upper bound on the estimate from the sample itself: w̄_N − ΔF̂_N ≥ 0 (the sample's dissipated work).
    pub fn sample_dissipation(&self) -> f64 {
        self.mean_work - self.delta_f
    }

    /// Delta-method estimate of the bias, Var(e^{−w}) / (2 N ⟨e^{−w}⟩²), from the sample moments. An estimate of
    /// the leading-order bias, not a bound.
    pub fn delta_method_bias(&self) -> f64 {
        let mu = self.shifted_weight_mean;
        self.shifted_weight_variance / (2.0 * self.samples as f64 * mu * mu)
    }
}

fn check_finite(works: &[f64]) -> Result<(), JarzynskiRefusal> {
    if works.is_empty() {
        return Err(JarzynskiRefusal::Empty);
    }
    match works.iter().position(|w| !w.is_finite()) {
        Some(index) => Err(JarzynskiRefusal::NonFinite { index }),
        None => Ok(()),
    }
}

/// Jarzynski estimate from reduced works `w_i = β W_i`.
pub fn estimate(works: &[f64]) -> Result<JarzynskiEstimate, JarzynskiRefusal> {
    check_finite(works)?;
    let n = works.len() as f64;
    let w_min = works.iter().copied().fold(f64::INFINITY, f64::min);
    // e^{−(w − w_min)} ∈ (0, 1]: log-sum-exp without overflow.
    let shifted: Vec<f64> = works.iter().map(|w| (-(w - w_min)).exp()).collect();
    let shifted_mean = shifted.iter().sum::<f64>() / n;
    let shifted_variance = if works.len() > 1 {
        shifted
            .iter()
            .map(|x| (x - shifted_mean).powi(2))
            .sum::<f64>()
            / (n - 1.0)
    } else {
        0.0
    };
    let mean_work = works.iter().sum::<f64>() / n;
    // −ln(mean e^{−w}) = w_min − ln(mean e^{−(w − w_min)}); the Jensen bound ΔF̂ ≤ w̄ holds exactly and is restored
    // against rounding.
    let delta_f = (w_min - shifted_mean.ln()).min(mean_work);
    Ok(JarzynskiEstimate {
        samples: works.len(),
        delta_f,
        mean_work,
        shifted_weight_variance: shifted_variance,
        shifted_weight_mean: shifted_mean,
    })
}

/// A confidence interval for ΔF in units of k_B·T.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FreeEnergyInterval {
    /// Lower end.
    pub lower: f64,
    /// Upper end.
    pub upper: f64,
    /// δ: the interval holds ΔF with probability at least 1 − δ.
    pub delta: f64,
}

/// Hoeffding confidence interval for ΔF when every reduced work lies in the known support `[a, b]`.
///
/// With x = e^{−(w − a)} ∈ [e^{−(b − a)}, 1] and ε = (1 − e^{−(b − a)}) √(ln(2/δ) / (2N)),
/// P(|x̄ − E x| ≥ ε) ≤ δ, and ΔF = a − ln E x, so ΔF ∈ [a − ln(min(x̄ + ε, 1)), a − ln(max(x̄ − ε, e^{−(b − a)}))].
pub fn hoeffding_interval(
    works: &[f64],
    support: (f64, f64),
    delta: f64,
) -> Result<FreeEnergyInterval, JarzynskiRefusal> {
    check_finite(works)?;
    if !(delta > 0.0 && delta < 1.0) {
        return Err(JarzynskiRefusal::Confidence { delta });
    }
    let (a, b) = support;
    if !(a.is_finite() && b.is_finite() && a <= b) {
        return Err(JarzynskiRefusal::Support { index: None });
    }
    if let Some(index) = works.iter().position(|w| *w < a || *w > b) {
        return Err(JarzynskiRefusal::Support { index: Some(index) });
    }
    let n = works.len() as f64;
    let floor = (-(b - a)).exp();
    let mean = works.iter().map(|w| (-(w - a)).exp()).sum::<f64>() / n;
    let eps = (1.0 - floor) * ((2.0 / delta).ln() / (2.0 * n)).sqrt();
    let hi = (mean + eps).min(1.0);
    let lo = (mean - eps).max(floor);
    Ok(FreeEnergyInterval {
        lower: a - hi.ln(),
        upper: a - lo.ln(),
        delta,
    })
}

/// Sudden quench of a two-level system: energies (0, ε₀) switched to (0, ε₁), reduced units (βε).
///
/// Started in the Gibbs state of (0, ε₀), the work is 0 in the ground level and ε₁ − ε₀ in the upper level, which
/// is occupied with probability p = e^{−ε₀} / (1 + e^{−ε₀}); ΔF = −ln((1 + e^{−ε₁}) / (1 + e^{−ε₀})).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoLevelQuench {
    /// Initial upper level β ε₀.
    pub eps0: f64,
    /// Final upper level β ε₁.
    pub eps1: f64,
}

impl TwoLevelQuench {
    /// Occupation of the upper level in the initial Gibbs state.
    pub fn upper_occupation(&self) -> f64 {
        let b = (-self.eps0).exp();
        b / (1.0 + b)
    }

    /// Work of a sample in the upper level.
    pub fn upper_work(&self) -> f64 {
        self.eps1 - self.eps0
    }

    /// Exact free-energy change.
    pub fn delta_f(&self) -> f64 {
        -((1.0 + (-self.eps1).exp()) / (1.0 + (-self.eps0).exp())).ln()
    }

    /// Exact mean work ⟨w⟩ = p (ε₁ − ε₀).
    pub fn mean_work(&self) -> f64 {
        self.upper_occupation() * self.upper_work()
    }

    /// Exact ⟨e^{−w}⟩.
    pub fn mean_exp_work(&self) -> f64 {
        let p = self.upper_occupation();
        (1.0 - p) + p * (-self.upper_work()).exp()
    }

    /// Exact Var(e^{−w}).
    pub fn var_exp_work(&self) -> f64 {
        let p = self.upper_occupation();
        let r = (-self.upper_work()).exp();
        p * (1.0 - p) * (1.0 - r) * (1.0 - r)
    }

    /// Exact E[ΔF̂_N]: with k of N samples in the upper level (binomial), ΔF̂_N = −ln((N − k + k r)/N).
    pub fn expected_estimate(&self, n: usize) -> f64 {
        let p = self.upper_occupation();
        let r = (-self.upper_work()).exp();
        let nf = n as f64;
        let (lp, lq) = (p.ln(), (1.0 - p).ln());
        let mut log_choose = 0.0_f64;
        let mut total = 0.0;
        for k in 0..=n {
            if k > 0 {
                log_choose += ((n - k + 1) as f64).ln() - (k as f64).ln();
            }
            let kf = k as f64;
            let weight = (log_choose + kf * lp + (nf - kf) * lq).exp();
            total += weight * -(((nf - kf) + kf * r) / nf).ln();
        }
        total
    }

    /// Draw a reduced work from a uniform variate u ∈ [0, 1).
    pub fn sample_work(&self, u: f64) -> f64 {
        if u < self.upper_occupation() {
            self.upper_work()
        } else {
            0.0
        }
    }
}

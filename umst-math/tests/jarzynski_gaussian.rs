// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: LicenseRef-TYTO-Undecided
//! Self-test of the finite-sample Jarzynski estimator on synthetic Gaussian reduced work
//! (Jarzynski, Phys. Rev. Lett. 78, 2690, 1997): ΔF = μ − σ²/2, finite-N bias against the leading
//! term of Gore, Ritort and Bustamante (Proc. Natl. Acad. Sci. U.S.A. 100, 12564, 2003), and the
//! integral-fluctuation check ⟨e^{−(w − ΔF)}⟩ = 1 with a Hoeffding false-alarm rate at level δ.

use umst_math::jarzynski::{estimate, JarzynskiRefusal};

/// Exact rationals for the Gaussian parameters; `f64` enters only through [`Rat::to_f64`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rat {
    num: i64,
    den: i64,
}

impl Rat {
    const fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        Self { num, den }
    }

    fn sub(self, o: Self) -> Self {
        Self::new(
            self.num * o.den - o.num * self.den,
            self.den * o.den,
        )
    }

    fn div(self, o: Self) -> Self {
        Self::new(self.num * o.den, self.den * o.num)
    }

    fn half(self) -> Self {
        self.div(Self::new(2, 1))
    }

    /// ΔF = μ − σ²/2 in exact arithmetic.
    fn jarzynski_delta_f(mu: Self, sigma_sq: Self) -> Self {
        mu.sub(sigma_sq.half())
    }

    /// Upper bound on |self − to_f64(self)| from converting num/den separately (stated runtime edge).
    fn conversion_bound(self) -> f64 {
        let d = self.den.abs() as f64;
        2.0 * f64::EPSILON * (self.num.abs() as f64).max(d) / d
    }

    fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

/// SplitMix64 (G. L. Steele, D. Lea and C. H. Flood, OOPSLA 2014).
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Polar Box–Muller: one standard normal, then scale to N(μ, σ²).
fn sample_gaussian(rng: &mut SplitMix64, mu: f64, sigma: f64) -> f64 {
    let mut u1 = rng.next_unit();
    let u2 = rng.next_unit();
    while u1 <= f64::MIN_POSITIVE {
        u1 = rng.next_unit();
    }
    let mag = sigma * (-2.0 * u1.ln()).sqrt();
    let z = mag * (2.0 * std::f64::consts::PI * u2).cos();
    mu + z
}

/// Population ⟨e^{−tw}⟩ for w ~ N(μ, σ²): exp(−tμ + t²σ²/2), evaluated at the runtime edge.
fn mean_exp_neg_tw(mu: f64, sigma_sq: f64, t: f64) -> f64 {
    (-t * mu + 0.5 * t * t * sigma_sq).exp()
}

/// Leading finite-N bias of the Jarzynski estimator (Gore, Ritort and Bustamante, PNAS 2003, Eq. 5
/// / delta-method term): Var(e^{−w}) / (2N ⟨e^{−w}⟩²).
fn gore_leading_bias(mu: f64, sigma_sq: f64, n: usize) -> f64 {
    let mu_exp = mean_exp_neg_tw(mu, sigma_sq, 1.0);
    let mean_sq = mean_exp_neg_tw(mu, sigma_sq, 2.0);
    let var_exp = mean_sq - mu_exp * mu_exp;
    var_exp / (2.0 * n as f64 * mu_exp * mu_exp)
}

/// Hoeffding two-sided false-alarm check for ⟨e^{−(w − ΔF)}⟩ = 1 when w ∈ [a, b].
fn integral_fluctuation_alarm(
    works: &[f64],
    delta_f: f64,
    support: (f64, f64),
    delta: f64,
) -> Result<bool, JarzynskiRefusal> {
    if works.is_empty() {
        return Err(JarzynskiRefusal::Empty);
    }
    if !(delta > 0.0 && delta < 1.0) {
        return Err(JarzynskiRefusal::Confidence { delta });
    }
    let (a, b) = support;
    if !(a.is_finite() && b.is_finite() && a <= b) {
        return Err(JarzynskiRefusal::Support { index: None });
    }
    for (index, w) in works.iter().enumerate() {
        if !w.is_finite() {
            return Err(JarzynskiRefusal::NonFinite { index });
        }
        if *w < a || *w > b {
            return Err(JarzynskiRefusal::Support { index: Some(index) });
        }
    }
    let x_hi = (-(a - delta_f)).exp();
    let x_lo = (-(b - delta_f)).exp();
    let span = x_hi - x_lo;
    let n = works.len() as f64;
    let mean = works
        .iter()
        .map(|w| (-(w - delta_f)).exp())
        .sum::<f64>()
        / n;
    let eps = span * ((2.0 / delta).ln() / (2.0 * n)).sqrt();
    Ok(mean < 1.0 - eps || mean > 1.0 + eps)
}

fn draw_batch(
    rng: &mut SplitMix64,
    mu: f64,
    sigma: f64,
    n: usize,
    support: (f64, f64),
) -> Option<Vec<f64>> {
    let mut works = Vec::with_capacity(n);
    for _ in 0..n {
        let w = sample_gaussian(rng, mu, sigma);
        if w < support.0 || w > support.1 {
            return None;
        }
        works.push(w);
    }
    Some(works)
}

#[test]
fn gaussian_entropy_production_estimator_and_integral_fluctuation_meter() {
    let mu_r = Rat::new(7, 4);
    let sigma_sq_r = Rat::new(3, 8);
    let delta_f_r = Rat::jarzynski_delta_f(mu_r, sigma_sq_r);
    let bound = mu_r.conversion_bound()
        + sigma_sq_r.conversion_bound()
        + delta_f_r.conversion_bound();
    let mu = mu_r.to_f64();
    let sigma_sq = sigma_sq_r.to_f64();
    let sigma = sigma_sq.sqrt();
    let delta_f = delta_f_r.to_f64();

    let mean_exp = mean_exp_neg_tw(mu, sigma_sq, 1.0);
    assert!(
        (mean_exp - (-delta_f).exp()).abs() <= bound + 1e-12,
        "Jarzynski identity at the rational edge (bound {bound})"
    );

    let n_if = 64usize;
    let n_gore = 2048usize;
    let delta = 0.05_f64;
    let support = (mu - 4.0 * sigma, mu + 4.0 * sigma);
    let mut rng = SplitMix64(0xa055_1a42_026u64);

    let mut gore_works = None;
    while gore_works.is_none() {
        gore_works = draw_batch(&mut rng, mu, sigma, n_gore, support);
    }
    let gore_est = estimate(gore_works.as_ref().expect("batch")).expect("finite works");
    let leading = gore_leading_bias(mu, sigma_sq, n_gore);
    assert!(
        (gore_est.delta_method_bias() - leading).abs() <= 0.05 * leading + bound + 1e-11,
        "Gore leading {leading} vs delta-method {}",
        gore_est.delta_method_bias()
    );

    let mut estimates = Vec::new();
    let mut alarms = 0usize;
    let mut valid = 0usize;
    let replicates = 400usize;

    for _ in 0..replicates {
        let Some(works) = draw_batch(&mut rng, mu, sigma, n_if, support) else {
            continue;
        };
        valid += 1;
        let est = estimate(&works).expect("finite works");
        assert!(est.delta_f <= est.mean_work + 1e-12);
        estimates.push(est.delta_f);
        if integral_fluctuation_alarm(&works, delta_f, support, delta).expect("in support") {
            alarms += 1;
        }
    }
    assert!(valid >= replicates / 2, "too many support rejections");

    let r = valid as f64;
    let mean_est = estimates.iter().sum::<f64>() / r;
    let sd = (estimates
        .iter()
        .map(|e| (e - mean_est).powi(2))
        .sum::<f64>()
        / (r - 1.0))
    .sqrt();
    assert!(mean_est - delta_f >= -1e-12, "Monte Carlo bias negative");

    let false_alarm_rate = alarms as f64 / r;
    eprintln!(
        "IF false-alarm meter: replicates={valid} alarms={alarms} rate={false_alarm_rate:.6} delta={delta} spread_sd={sd:.6}"
    );
    assert!(
        false_alarm_rate <= delta + 3.0 * (delta * (1.0 - delta) / r).sqrt() + 1e-12,
        "false-alarm rate {false_alarm_rate} above δ={delta}"
    );
}

#[test]
fn three_percent_bias_injection_fails_replicate_centering() {
    let mu_r = Rat::new(7, 4);
    let sigma_sq_r = Rat::new(3, 8);
    let delta_f_r = Rat::jarzynski_delta_f(mu_r, sigma_sq_r);
    let mu = mu_r.to_f64();
    let sigma = sigma_sq_r.to_f64().sqrt();
    let delta_f = delta_f_r.to_f64();

    let n = 64usize;
    let support = (mu - 4.0 * sigma, mu + 4.0 * sigma);
    let mut rng = SplitMix64(0xb1a5_3fc7_2026u64);
    let replicates = 400usize;
    let mut estimates = Vec::with_capacity(replicates);
    for _ in 0..replicates {
        let Some(works) = draw_batch(&mut rng, mu, sigma, n, support) else {
            continue;
        };
        estimates.push(
            estimate(&works)
                .expect("finite works")
                .delta_f,
        );
    }
    let r = estimates.len() as f64;
    assert!(r >= replicates as f64 / 2.0);
    let mean = estimates.iter().sum::<f64>() / r;
    let sd = (estimates
        .iter()
        .map(|e| (e - mean).powi(2))
        .sum::<f64>()
        / (r - 1.0))
    .sqrt();
    let center = delta_f + gore_leading_bias(mu, sigma_sq_r.to_f64(), n);
    let biased = center * 1.03;
    assert!(
        (mean - biased).abs() > 4.0 * sd / r.sqrt() + 1e-12,
        "3% bias injection should break centering: mean {mean} vs {biased} (sd {sd})"
    );
}

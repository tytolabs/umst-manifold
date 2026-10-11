// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Self-test of the finite-sample Jarzynski estimator on synthetic Gaussian reduced work
//! (Jarzynski, Phys. Rev. Lett. 78, 2690, 1997): ΔF = μ − σ²/2, finite-N bias against the leading
//! term of Gore, Ritort and Bustamante (Proc. Natl. Acad. Sci. U.S.A. 100, 12564, 2003), and the
//! Hoeffding interval false-alarm rate via [`umst_math::jarzynski::hoeffding_interval`] at level δ.

use umst_math::jarzynski::estimate;

/// Exact rationals for the Gaussian parameters; `f64` enters only through [`Rat::to_f64`] at the sampling edge.
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
    fn conversion_bound_f64(self) -> f64 {
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

/// Non-test entry points: call runtime [`estimate`] and [`hoeffding_interval`]; rational meter admission.
mod runtime_meter {
    use super::Rat;
    use umst_math::jarzynski::{estimate, hoeffding_interval, FreeEnergyInterval, JarzynskiRefusal};

    /// Seeded-replicate tally of Hoeffding interval exclusions for exact ΔF at level δ.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct IntegralFluctuationFalseAlarmMeter {
        pub replicates_attempted: usize,
        pub replicates_valid: usize,
        pub alarms: usize,
        pub delta_level: Rat,
    }

    impl IntegralFluctuationFalseAlarmMeter {
        pub fn false_alarm_rate(&self) -> Rat {
            if self.replicates_valid == 0 {
                return Rat::new(0, 1);
            }
            Rat::new(self.alarms as i64, self.replicates_valid as i64)
        }
    }

    /// Population ⟨e^{−tw}⟩ for w ~ N(μ, σ²): exp(−tμ + t²σ²/2), at the runtime edge for Gore reference bias.
    fn mean_exp_neg_tw(mu: f64, sigma_sq: f64, t: f64) -> f64 {
        (-t * mu + 0.5 * t * t * sigma_sq).exp()
    }

    /// Leading finite-N bias (Gore, Ritort and Bustamante, PNAS 2003): Var(e^{−w}) / (2N ⟨e^{−w}⟩²).
    pub fn gore_population_leading_bias(mu: f64, sigma_sq: f64, n: usize) -> f64 {
        let mu_exp = mean_exp_neg_tw(mu, sigma_sq, 1.0);
        let mean_sq = mean_exp_neg_tw(mu, sigma_sq, 2.0);
        let var_exp = mean_sq - mu_exp * mu_exp;
        var_exp / (2.0 * n as f64 * mu_exp * mu_exp)
    }

    /// Exact ΔF from rationals lies outside the runtime Hoeffding interval (one false alarm draw).
    pub fn hoeffding_false_alarm(
        works: &[f64],
        delta_f_r: Rat,
        edge_slack: f64,
        support: (f64, f64),
        delta: Rat,
    ) -> Result<bool, JarzynskiRefusal> {
        let ci = hoeffding_interval(works, support, delta.to_f64())?;
        Ok(!interval_covers_delta_f(ci, delta_f_r, edge_slack))
    }

    fn interval_covers_delta_f(ci: FreeEnergyInterval, delta_f_r: Rat, edge_slack: f64) -> bool {
        let slack = edge_slack + delta_f_r.conversion_bound_f64();
        let df = delta_f_r.to_f64();
        ci.lower <= df + slack && df - slack <= ci.upper
    }

    /// Accumulate false alarms over seeded replicates using [`hoeffding_interval`].
    pub fn record_integral_fluctuation_false_alarm_meter(
        draw: &mut impl FnMut() -> Option<Vec<f64>>,
        delta_f_r: Rat,
        edge_slack: f64,
        support: (f64, f64),
        delta: Rat,
        replicates: usize,
    ) -> Result<IntegralFluctuationFalseAlarmMeter, JarzynskiRefusal> {
        let mut alarms = 0usize;
        let mut valid = 0usize;
        for _ in 0..replicates {
            let Some(works) = draw() else {
                continue;
            };
            valid += 1;
            if hoeffding_false_alarm(&works, delta_f_r, edge_slack, support, delta)? {
                alarms += 1;
            }
        }
        Ok(IntegralFluctuationFalseAlarmMeter {
            replicates_attempted: replicates,
            replicates_valid: valid,
            alarms,
            delta_level: delta,
        })
    }

    /// [`estimate`] delta-method bias vs Gore population leading term (runtime sample vs reference edge).
    pub fn gore_delta_method_admission(
        works: &[f64],
        mu: f64,
        sigma_sq: f64,
        relative_tol: Rat,
        edge_slack: f64,
    ) -> Result<(), JarzynskiRefusal> {
        let est = estimate(works)?;
        let leading = gore_population_leading_bias(mu, sigma_sq, works.len());
        let diff = (est.delta_method_bias() - leading).abs();
        let tol = relative_tol.to_f64() * leading.abs() + edge_slack;
        if diff > tol {
            return Err(JarzynskiRefusal::Support { index: None });
        }
        Ok(())
    }
}

#[test]
fn gaussian_entropy_production_estimator_and_integral_fluctuation_meter() {
    use runtime_meter::{
        gore_delta_method_admission, record_integral_fluctuation_false_alarm_meter,
        IntegralFluctuationFalseAlarmMeter,
    };

    let mu_r = Rat::new(7, 4);
    let sigma_sq_r = Rat::new(3, 8);
    let delta_f_r = Rat::jarzynski_delta_f(mu_r, sigma_sq_r);
    let edge_slack = mu_r.conversion_bound_f64()
        + sigma_sq_r.conversion_bound_f64()
        + delta_f_r.conversion_bound_f64();
    let mu = mu_r.to_f64();
    let sigma_sq = sigma_sq_r.to_f64();
    let sigma = sigma_sq.sqrt();
    let delta_f = delta_f_r.to_f64();

    let mean_exp = (-mu + 0.5 * sigma_sq).exp();
    assert!(
        (mean_exp - (-delta_f).exp()).abs() <= edge_slack + 1e-12,
        "Jarzynski identity at the rational edge"
    );

    let n_gore = 2048usize;
    let delta = Rat::new(1, 20);
    let support = (mu - 4.0 * sigma, mu + 4.0 * sigma);
    let mut rng = SplitMix64(0xa055_1a42_026u64);

    let mut gore_works = None;
    while gore_works.is_none() {
        gore_works = draw_batch(&mut rng, mu, sigma, n_gore, support);
    }
    gore_delta_method_admission(
        gore_works.as_ref().expect("batch"),
        mu,
        sigma_sq,
        Rat::new(1, 20),
        edge_slack,
    )
    .expect("Gore leading term vs delta-method bias");

    let n_if = 64usize;
    let replicates = 400usize;
    let meter = record_integral_fluctuation_false_alarm_meter(
        &mut || draw_batch(&mut rng, mu, sigma, n_if, support),
        delta_f_r,
        edge_slack,
        support,
        delta,
        replicates,
    )
    .expect("Hoeffding meter");
    assert!(
        meter.replicates_valid >= replicates / 2,
        "too many support rejections"
    );

    let mut estimates = Vec::new();
    for _ in 0..meter.replicates_valid {
        let Some(works) = draw_batch(&mut rng, mu, sigma, n_if, support) else {
            continue;
        };
        let est = estimate(&works).expect("finite works");
        assert!(est.delta_f <= est.mean_work + 1e-12);
        estimates.push(est.delta_f);
    }
    let r = estimates.len() as f64;
    let mean_est = estimates.iter().sum::<f64>() / r;
    let sd = (estimates
        .iter()
        .map(|e| (e - mean_est).powi(2))
        .sum::<f64>()
        / (r - 1.0))
    .sqrt();
    assert!(mean_est - delta_f >= -1e-12, "Monte Carlo bias negative");

    let r_meter = meter.replicates_valid as f64;
    let false_alarm_rate = meter.false_alarm_rate().to_f64();
    let delta_level = delta.to_f64();
    eprintln!(
        "IF false-alarm meter: replicates={} alarms={} rate={false_alarm_rate:.6} delta={delta_level:.6} spread_sd={sd:.6}",
        meter.replicates_valid,
        meter.alarms,
    );
    assert!(
        false_alarm_rate
            <= delta_level
                + 3.0 * (delta_level * (1.0 - delta_level) / r_meter).sqrt()
                + edge_slack,
        "false-alarm rate {false_alarm_rate} above δ={delta_level}"
    );
    let _typed: IntegralFluctuationFalseAlarmMeter = meter;
}

#[test]
fn three_percent_bias_injection_fails_replicate_centering() {
    use runtime_meter::gore_population_leading_bias;

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
    let center = delta_f + gore_population_leading_bias(mu, sigma_sq_r.to_f64(), n);
    let biased = center * 1.03;
    assert!(
        (mean - biased).abs() > 4.0 * sd / r.sqrt() + 1e-12,
        "3% bias injection should break centering: mean {mean} vs {biased} (sd {sd})"
    );
}

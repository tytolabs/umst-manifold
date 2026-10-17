// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Self-test of the finite-sample Jarzynski estimator on synthetic Gaussian reduced work
//! (Jarzynski, Phys. Rev. Lett. 78, 2690, 1997): ΔF = μ − σ²/2, finite-N bias against the leading
//! term of Gore, Ritort and Bustamante (Proc. Natl. Acad. Sci. U.S.A. 100, 12564, 2003), and the
//! false-alarm rate of [`umst_math::jarzynski_meter::integral_fluctuation_check`] at level δ, together with its power
//! against a wrong free-energy change.

use umst_math::jarzynski::estimate;
use umst_math::jarzynski_meter::{
    gaussian_leading_bias, integral_fluctuation_check, record_false_alarms, Level,
};

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
        Self::new(self.num * o.den - o.num * self.den, self.den * o.den)
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

#[test]
fn gaussian_entropy_production_estimator_and_integral_fluctuation_meter() {
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
    let delta_f_bound = delta_f_r.conversion_bound_f64();

    let mean_exp = (-mu + 0.5 * sigma_sq).exp();
    assert!(
        (mean_exp - (-delta_f).exp()).abs() <= edge_slack + 1e-12,
        "Jarzynski identity at the rational edge"
    );

    let n_gore = 2048usize;
    let level = Level::new(1, 20).expect("level");
    let support = (mu - 4.0 * sigma, mu + 4.0 * sigma);
    let mut rng = SplitMix64(0xa055_1a42_026u64);

    let mut gore_works = None;
    while gore_works.is_none() {
        gore_works = draw_batch(&mut rng, mu, sigma, n_gore, support);
    }
    let est = estimate(gore_works.as_ref().expect("batch")).expect("finite works");
    let leading = gaussian_leading_bias(mu, sigma_sq, n_gore);
    assert!(
        (est.delta_method_bias() - leading).abs() <= 0.05 * leading.abs() + edge_slack,
        "Gore leading term {leading} against delta-method bias {}",
        est.delta_method_bias()
    );

    let n_if = 64usize;
    let replicates = 400usize;
    let tally = record_false_alarms(
        &mut || draw_batch(&mut rng, mu, sigma, n_if, support),
        delta_f,
        delta_f_bound,
        support,
        level,
        replicates,
    )
    .expect("Hoeffding meter");
    assert!(
        tally.replicates_valid >= replicates / 2,
        "too many support rejections"
    );

    let mut estimates = Vec::new();
    for _ in 0..tally.replicates_valid {
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

    eprintln!(
        "IF false-alarm meter: replicates={} alarms={} rate={:.6} delta={:.6} spread_sd={sd:.6}",
        tally.replicates_valid,
        tally.alarms,
        tally.rate(),
        level.as_f64(),
    );
    assert!(
        tally.within_level(3.0),
        "false-alarm rate {} above δ={}",
        tally.rate(),
        level.as_f64()
    );
}

/// The meter has power: at N = 4096 the Hoeffding interval is narrow enough that a free-energy change wrong by
/// half a k_B·T raises the alarm on every replicate, while the true change stays within δ. A meter that never
/// alarms passes the false-alarm test above and fails here.
#[test]
fn wrong_free_energy_raises_the_alarm_and_the_true_one_stays_within_the_level() {
    let mu_r = Rat::new(7, 4);
    let sigma_sq_r = Rat::new(3, 8);
    let delta_f_r = Rat::jarzynski_delta_f(mu_r, sigma_sq_r);
    let mu = mu_r.to_f64();
    let sigma = sigma_sq_r.to_f64().sqrt();
    let delta_f = delta_f_r.to_f64();
    let bound = delta_f_r.conversion_bound_f64();
    let support = (mu - 4.0 * sigma, mu + 4.0 * sigma);
    let level = Level::new(1, 20).expect("level");
    let n = 4096usize;
    let replicates = 100usize;

    let mut rng = SplitMix64(0x70e5_c0de_2026u64);
    let true_tally = record_false_alarms(
        &mut || draw_batch(&mut rng, mu, sigma, n, support),
        delta_f,
        bound,
        support,
        level,
        replicates,
    )
    .expect("true change");
    assert!(true_tally.replicates_valid >= replicates / 2);
    assert!(
        true_tally.within_level(3.0),
        "true ΔF alarms at rate {}",
        true_tally.rate()
    );

    let mut rng = SplitMix64(0x70e5_c0de_2026u64);
    let wrong_tally = record_false_alarms(
        &mut || draw_batch(&mut rng, mu, sigma, n, support),
        delta_f + 0.5,
        bound,
        support,
        level,
        replicates,
    )
    .expect("wrong change");
    assert_eq!(
        wrong_tally.alarms, wrong_tally.replicates_valid,
        "a free-energy change wrong by 0.5 must alarm on every valid replicate"
    );
}

#[test]
fn alarm_follows_the_interval_and_the_stated_error_bound() {
    let works = [1.0, 1.1, 1.2, 1.3];
    let support = (0.5, 2.0);
    let level = Level::new(1, 10).expect("level");
    let ci = integral_fluctuation_check(&works, 1.0, 0.0, support, level)
        .expect("works in support")
        .interval();
    assert!(ci.lower < ci.upper);
    let inside = 0.5 * (ci.lower + ci.upper);
    assert!(
        !integral_fluctuation_check(&works, inside, 0.0, support, level)
            .unwrap()
            .is_alarm()
    );
    let just_below = ci.lower - 1e-3;
    assert!(
        integral_fluctuation_check(&works, just_below, 0.0, support, level)
            .unwrap()
            .is_alarm()
    );
    assert!(
        !integral_fluctuation_check(&works, just_below, 2e-3, support, level)
            .unwrap()
            .is_alarm(),
        "an error bound that reaches the interval removes the alarm"
    );
    let just_above = ci.upper + 1e-3;
    assert!(
        integral_fluctuation_check(&works, just_above, 0.0, support, level)
            .unwrap()
            .is_alarm()
    );
}

#[test]
fn level_and_tally_compare_by_exact_integers() {
    assert!(Level::new(0, 5).is_err());
    assert!(Level::new(5, 5).is_err());
    assert!(Level::new(6, 5).is_err());
    assert!(Level::new(1, 0).is_err());
    let level = Level::new(1, 20).expect("level");
    let at = umst_math::jarzynski_meter::FalseAlarmTally {
        replicates_attempted: 40,
        replicates_valid: 40,
        alarms: 2,
        level,
    };
    assert!(
        !at.rate_exceeds_level(),
        "2 / 40 equals δ = 1 / 20 and does not exceed it"
    );
    let over = umst_math::jarzynski_meter::FalseAlarmTally { alarms: 3, ..at };
    assert!(over.rate_exceeds_level());
    let invalid = umst_math::jarzynski_meter::FalseAlarmTally {
        replicates_attempted: 40,
        replicates_valid: 20,
        alarms: 2,
        level,
    };
    assert!(
        invalid.rate_exceeds_level(),
        "the rate counts valid replicates only: 2 / 20 exceeds 1 / 20"
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
        estimates.push(estimate(&works).expect("finite works").delta_f);
    }
    let r = estimates.len() as f64;
    assert!(r >= replicates as f64 / 2.0);
    let mean = estimates.iter().sum::<f64>() / r;
    let sd = (estimates.iter().map(|e| (e - mean).powi(2)).sum::<f64>() / (r - 1.0)).sqrt();
    let center = delta_f + gaussian_leading_bias(mu, sigma_sq_r.to_f64(), n);
    assert!(
        (mean - center).abs() <= 4.0 * sd / r.sqrt() + 1e-12,
        "replicate mean {mean} off the Gore centre {center}"
    );
    let biased = center * 1.03;
    assert!(
        (mean - biased).abs() > 4.0 * sd / r.sqrt() + 1e-12,
        "3% bias injection should break centering: mean {mean} vs {biased} (sd {sd})"
    );
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Self-test of the finite-sample Jarzynski estimator (`umst_math::jarzynski`) against the exactly solvable
//! two-level quench: exact free-energy change, exact expectation of the N-sample estimator, seeded Monte Carlo.
use umst_math::jarzynski::{estimate, hoeffding_interval, JarzynskiRefusal, TwoLevelQuench};

/// SplitMix64 (G. L. Steele, D. Lea and C. H. Flood, OOPSLA 2014): a fixed-seed stream for the Monte Carlo test.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_unit(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Quenches across the regimes: small and large switches, up and down.
fn quenches() -> [TwoLevelQuench; 4] {
    [
        TwoLevelQuench {
            eps0: 0.5,
            eps1: 1.5,
        },
        TwoLevelQuench {
            eps0: 2.0,
            eps1: 0.1,
        },
        TwoLevelQuench {
            eps0: 0.0,
            eps1: 6.0,
        },
        TwoLevelQuench {
            eps0: 1.0,
            eps1: -3.0,
        },
    ]
}

#[test]
fn two_level_obeys_jarzynski_and_the_second_law() {
    for q in quenches() {
        assert!((q.mean_exp_work() - (-q.delta_f()).exp()).abs() < 1e-12);
        assert!(q.delta_f() <= q.mean_work() + 1e-12);
    }
}

#[test]
fn one_sample_estimate_is_unbiased_for_the_mean_work() {
    for q in quenches() {
        assert!((q.expected_estimate(1) - q.mean_work()).abs() < 1e-12);
    }
}

#[test]
fn exact_bias_is_bounded_monotone_and_vanishes() {
    for q in quenches() {
        let dissipation = q.mean_work() - q.delta_f();
        let mut previous = f64::INFINITY;
        for n in 1..=256 {
            let e = q.expected_estimate(n);
            assert!(e >= q.delta_f() - 1e-12, "bias negative at N = {n}");
            assert!(
                e <= q.mean_work() + 1e-12,
                "bias above dissipation at N = {n}"
            );
            assert!(e <= previous + 1e-12, "bias rose at N = {n}");
            previous = e;
        }
        let bias_far = q.expected_estimate(4096) - q.delta_f();
        assert!(bias_far <= dissipation / 100.0 + 1e-12);
    }
}

#[test]
fn exact_bias_matches_the_delta_method_at_large_n() {
    for q in quenches() {
        let n = 4000;
        let exact = q.expected_estimate(n) - q.delta_f();
        let mu = q.mean_exp_work();
        let leading = q.var_exp_work() / (2.0 * n as f64 * mu * mu);
        assert!(
            (exact - leading).abs() <= 0.05 * leading,
            "{exact} vs {leading}"
        );
    }
}

#[test]
fn monte_carlo_estimates_center_on_the_exact_expectation() {
    let mut rng = SplitMix64(0x5EED_F00D);
    let n = 64;
    let replicates = 400;
    for q in quenches() {
        let mut estimates = Vec::with_capacity(replicates);
        let mut covered = 0usize;
        let lo_w = q.upper_work().min(0.0);
        let hi_w = q.upper_work().max(0.0);
        for _ in 0..replicates {
            let works: Vec<f64> = (0..n).map(|_| q.sample_work(rng.next_unit())).collect();
            let est = estimate(&works).expect("finite works");
            assert!(est.delta_f <= est.mean_work, "per-sample Jensen bound");
            assert!(est.sample_dissipation() >= 0.0);
            let ci = hoeffding_interval(&works, (lo_w, hi_w), 0.05).expect("in support");
            if ci.lower <= q.delta_f() && q.delta_f() <= ci.upper {
                covered += 1;
            }
            estimates.push(est.delta_f);
        }
        let r = replicates as f64;
        let mean = estimates.iter().sum::<f64>() / r;
        let sd = (estimates.iter().map(|e| (e - mean).powi(2)).sum::<f64>() / (r - 1.0)).sqrt();
        let exact = q.expected_estimate(n);
        assert!(
            (mean - exact).abs() <= 4.0 * sd / r.sqrt() + 1e-12,
            "{mean} vs {exact} (sd {sd})"
        );
        assert!(
            covered as f64 / r >= 0.95,
            "coverage {covered} of {replicates}"
        );
    }
}

#[test]
fn refusals_are_typed() {
    assert_eq!(estimate(&[]), Err(JarzynskiRefusal::Empty));
    assert_eq!(
        estimate(&[0.0, f64::NAN]),
        Err(JarzynskiRefusal::NonFinite { index: 1 })
    );
    assert_eq!(
        hoeffding_interval(&[0.5], (0.0, 1.0), 1.5),
        Err(JarzynskiRefusal::Confidence { delta: 1.5 })
    );
    assert_eq!(
        hoeffding_interval(&[2.0], (0.0, 1.0), 0.1),
        Err(JarzynskiRefusal::Support { index: Some(0) })
    );
    assert_eq!(
        hoeffding_interval(&[0.5], (1.0, 0.0), 0.1),
        Err(JarzynskiRefusal::Support { index: None })
    );
}

#[test]
fn estimate_survives_large_works() {
    let est = estimate(&[1000.0, 1001.0, 1002.0]).expect("finite");
    assert!(est.delta_f.is_finite());
    assert!(est.delta_f >= 1000.0 && est.delta_f <= est.mean_work);
}

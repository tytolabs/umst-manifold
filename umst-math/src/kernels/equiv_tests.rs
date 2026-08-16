// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! SIMD vs scalar equivalence — only compiled with `--features simd`.

use super::scalar;
use super::simd;

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn u01(seed: u64) -> f64 {
    (splitmix64(seed) as f64) / (u64::MAX as f64)
}

fn rho_close(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let d = (x - y).abs();
            d <= 1e-9 || d / x.abs().max(y.abs()).max(1e-300) <= 1e-12
        }
        _ => false,
    }
}

fn rho_max_diff(a: Option<f64>, b: Option<f64>) -> (f64, f64) {
    match (a, b) {
        (None, None) => (0.0, 0.0),
        (Some(x), Some(y)) => {
            let ad = (x - y).abs();
            let denom = x.abs().max(y.abs()).max(1e-300);
            let rd = ad / denom;
            (ad, rd)
        }
        _ => (f64::NAN, f64::NAN),
    }
}

#[test]
fn simd_scalar_equiv_256_seeds() {
    for seed in 0u64..256 {
        let n = 2 + (seed % 64) as usize;
        let mut xs = vec![0.0; n];
        let mut ys = vec![0.0; n];
        for i in 0..n {
            let t = seed.wrapping_mul(991).wrapping_add(i as u64);
            xs[i] = u01(t) * 20.0 - 10.0;
            ys[i] = u01(t.wrapping_add(333)) * 20.0 - 10.0;
        }
        let s = scalar::rho_mi_from_samples_scalar(&xs, &ys);
        let v = simd::rho_mi_from_samples_simd(&xs, &ys);
        assert!(rho_close(s, v), "seed {seed} rho: s={s:?} v={v:?}");

        let q = 0.05 + (seed % 90) as f64 / 100.0;
        assert_eq!(
            scalar::sample_percentile_scalar(&xs, q),
            simd::sample_percentile_simd(&xs, q),
            "seed {seed} percentile"
        );

        let eta = u01(seed.wrapping_mul(17)) * 3.0;
        assert_eq!(
            scalar::classify_band_scalar(&xs, eta),
            simd::classify_band_simd(&xs, eta),
            "seed {seed} band"
        );
    }
}

#[test]
fn simd_scalar_equiv_adversarial() {
    let run = |xs: &[f64], ys: &[f64], eta: f64| {
        let s = scalar::rho_mi_from_samples_scalar(xs, ys);
        let v = simd::rho_mi_from_samples_simd(xs, ys);
        assert!(
            rho_close(s, v),
            "adv rho xs={xs:?} ys={ys:?} s={s:?} v={v:?}"
        );
        assert_eq!(
            scalar::sample_percentile_scalar(xs, 0.5),
            simd::sample_percentile_simd(xs, 0.5)
        );
        assert_eq!(
            scalar::classify_band_scalar(xs, eta),
            simd::classify_band_simd(xs, eta)
        );
    };

    // all zeros — ρ undefined / None; both paths agree.
    run(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], 0.0);
    // all equal x, varying y (zero variance x)
    run(&[1.0, 1.0, 1.0], &[2.0, 3.0, 4.0], 1.5);
    // alternating large/small (finite)
    run(
        &[1e200, 1e-200, 1e200, 1e-200],
        &[1e200, 1e-200, 1e200, 1e-200],
        1e200,
    );
    // near-underflow magnitudes
    run(
        &[1e-300, 2e-300, 3e-300, 4e-300],
        &[1e-300, 2e-300, 3e-300, 4e-300],
        2e-300,
    );

    // Perfect correlation: y = 2x
    let xs: Vec<f64> = (0..5).map(|i| i as f64).collect();
    let ys: Vec<f64> = (0..5).map(|i| (2 * i) as f64).collect();
    let s = scalar::rho_mi_from_samples_scalar(&xs, &ys);
    let v = simd::rho_mi_from_samples_simd(&xs, &ys);
    assert!(rho_close(s, v), "line rho s={s:?} v={v:?}");
}

#[test]
fn simd_tolerance_receipt_adversarial_table() {
    // Fixed adversarial grid for archived `kernel_tolerance.jsonl` (script may grep this test name).
    let mut max_ad = 0.0f64;
    let mut max_rd = 0.0f64;
    for seed in 0u64..256 {
        let n = 2 + (seed % 64) as usize;
        let mut xs = vec![0.0; n];
        let mut ys = vec![0.0; n];
        for i in 0..n {
            let t = seed.wrapping_mul(1009).wrapping_add(i as u64);
            xs[i] = u01(t) * 20.0 - 10.0;
            ys[i] = u01(t.wrapping_add(601)) * 20.0 - 10.0;
        }
        let (ad, rd) = rho_max_diff(
            scalar::rho_mi_from_samples_scalar(&xs, &ys),
            simd::rho_mi_from_samples_simd(&xs, &ys),
        );
        max_ad = max_ad.max(ad);
        max_rd = max_rd.max(rd);
    }
    assert!(
        max_ad <= 1e-9 || max_rd <= 1e-12,
        "max_ad={max_ad} max_rd={max_rd}"
    );
}

//! Fixed-table SIMD vs scalar tolerance metrics for archived CI receipts (`Phase M-simd`).

use serde::Serialize;

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

fn rho_max_diff(a: Option<f64>, b: Option<f64>) -> (f64, f64) {
    match (a, b) {
        (None, None) => (0.0, 0.0),
        (Some(x), Some(y)) => {
            let ad = (x - y).abs();
            let denom = x.abs().max(y.abs()).max(1e-300);
            (ad, ad / denom)
        }
        _ => (f64::NAN, f64::NAN),
    }
}

/// One row for `kernel_tolerance.jsonl` (per target × kernel).
#[derive(Serialize)]
pub struct KernelToleranceRow {
    pub kernel: &'static str,
    pub target: String,
    pub max_abs_diff: f64,
    pub max_rel_diff: f64,
    pub n_seeds: u64,
}

const N_SEEDS: u64 = 256;

/// Worst-case ρ-MI absolute / relative diff between scalar and SIMD over the equivalence grid.
pub fn rho_max_diff_table() -> (f64, f64) {
    let mut max_ad = 0.0f64;
    let mut max_rd = 0.0f64;
    for seed in 0u64..N_SEEDS {
        let n = 2 + (seed % 64) as usize;
        let mut xs = vec![0.0; n];
        let mut ys = vec![0.0; n];
        for i in 0..n {
            let t = seed.wrapping_mul(991).wrapping_add(i as u64);
            xs[i] = u01(t) * 20.0 - 10.0;
            ys[i] = u01(t.wrapping_add(333)) * 20.0 - 10.0;
        }
        let (ad, rd) = rho_max_diff(
            scalar::rho_mi_from_samples_scalar(&xs, &ys),
            simd::rho_mi_from_samples_simd(&xs, &ys),
        );
        max_ad = max_ad.max(ad);
        max_rd = max_rd.max(rd);
    }
    (max_ad, max_rd)
}

fn percentile_max_diff_table() -> (f64, f64) {
    let mut max_ad = 0.0f64;
    let mut max_rd = 0.0f64;
    for seed in 0u64..N_SEEDS {
        let n = 2 + (seed % 64) as usize;
        let mut xs = vec![0.0; n];
        for (i, slot) in xs.iter_mut().enumerate() {
            let t = seed.wrapping_mul(991).wrapping_add(i as u64);
            *slot = u01(t) * 20.0 - 10.0;
        }
        let q = 0.05 + (seed % 90) as f64 / 100.0;
        let (ad, rd) = rho_max_diff(
            scalar::sample_percentile_scalar(&xs, q),
            simd::sample_percentile_simd(&xs, q),
        );
        max_ad = max_ad.max(ad);
        max_rd = max_rd.max(rd);
    }
    (max_ad, max_rd)
}

/// Three rows: ρ-MI, percentile, band (band is enum-exact → zero diffs).
#[must_use]
pub fn kernel_tolerance_rows(target: impl Into<String>) -> [KernelToleranceRow; 3] {
    let target = target.into();
    let (pad, prd) = rho_max_diff_table();
    let (qad, qrd) = percentile_max_diff_table();
    [
        KernelToleranceRow {
            kernel: "rho_mi_from_samples",
            target: target.clone(),
            max_abs_diff: pad,
            max_rel_diff: prd,
            n_seeds: N_SEEDS,
        },
        KernelToleranceRow {
            kernel: "sample_percentile",
            target: target.clone(),
            max_abs_diff: qad,
            max_rel_diff: qrd,
            n_seeds: N_SEEDS,
        },
        KernelToleranceRow {
            kernel: "classify_band",
            target,
            max_abs_diff: 0.0,
            max_rel_diff: 0.0,
            n_seeds: N_SEEDS,
        },
    ]
}

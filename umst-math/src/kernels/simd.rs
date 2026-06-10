//! SIMD kernels (`portable_simd` / `std::simd`). **Compile-time** selection only — no runtime CPUID (follow-up slice).
//!
//! Sort-based kernels ([`sample_percentile_simd`], [`classify_band_simd`]) intentionally mirror the scalar path
//! byte-for-byte so equivalence stays exact; the meaningful SIMD work is in [`rho_mi_from_samples_simd`].

use std::simd::num::SimdFloat;
use std::simd::Simd;

use crate::kahan::KahanSum;

use super::scalar::{
    classify_band_scalar, mean_from_slice, rho_mi_bits, sample_percentile_scalar, BandLabel,
};

/// Lane width for vectorized Pearson accumulation.
const LANES: usize = 4;
type F64xN = Simd<f64, LANES>;

/// Pearson ρ-MI with SIMD chunk accumulation and **Kahan** reduction for `Σ(dx·dy)`, `Σ(dx²)`, `Σ(dy²)`.
#[must_use]
pub fn rho_mi_from_samples_simd(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let mx = mean_from_slice(xs)?;
    let my = mean_from_slice(ys)?;
    let mut k_num = KahanSum::new();
    let mut k_dx2 = KahanSum::new();
    let mut k_dy2 = KahanSum::new();

    let n = xs.len();
    let chunks = n / LANES;
    let vec_end = chunks * LANES;

    let mxv = F64xN::splat(mx);
    let myv = F64xN::splat(my);

    let mut i = 0usize;
    while i < vec_end {
        let mut xa = [0.0f64; LANES];
        let mut ya = [0.0f64; LANES];
        for lane in 0..LANES {
            xa[lane] = xs[i + lane];
            ya[lane] = ys[i + lane];
            if !xa[lane].is_finite() || !ya[lane].is_finite() {
                return None;
            }
        }
        let xv = F64xN::from_array(xa);
        let yv = F64xN::from_array(ya);
        let dx = xv - mxv;
        let dy = yv - myv;
        let prod = dx * dy;
        let dx2 = dx * dx;
        let dy2 = dy * dy;
        k_num.add(prod.reduce_sum());
        k_dx2.add(dx2.reduce_sum());
        k_dy2.add(dy2.reduce_sum());
        i += LANES;
    }
    while i < n {
        let x = xs[i];
        let y = ys[i];
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let dx = x - mx;
        let dy = y - my;
        k_num.add(dx * dy);
        k_dx2.add(dx * dx);
        k_dy2.add(dy * dy);
        i += 1;
    }

    let num = k_num.total();
    let dx2 = k_dx2.total();
    let dy2 = k_dy2.total();
    let denom = (dx2 * dy2).sqrt();
    if denom <= 0.0 || !denom.is_finite() {
        return None;
    }
    let rho = num / denom;
    if !rho.is_finite() {
        return None;
    }
    Some(rho_mi_bits(rho))
}

/// Bit-identical to [`super::scalar::sample_percentile_scalar`] — sort path is scalar; SIMD feature gates dispatch only.
#[must_use]
pub fn sample_percentile_simd(samples: &[f64], q: f64) -> Option<f64> {
    sample_percentile_scalar(samples, q)
}

/// Enum-identical to [`super::scalar::classify_band_scalar`].
#[must_use]
pub fn classify_band_simd(samples: &[f64], eta: f64) -> Option<BandLabel> {
    classify_band_scalar(samples, eta)
}

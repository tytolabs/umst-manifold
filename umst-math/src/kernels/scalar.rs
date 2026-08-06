//! Scalar reference kernels — always built; SIMD paths must match these results (see `kernels::simd`).
//!
//! Proof anchors unchanged: `UMST.Formal.RhoEstimator`, `UMST.Formal.OrderStatisticsBand`.

/// Pragmatic quartile band label (mirrors `order_statistics_band` / cockpit η_cog band without Landauer splits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BandLabel {
    /// η strictly below rolling P25.
    Wasteful,
    /// η between P25 and P75 (inclusive boundaries per branch order at consumer).
    Balanced,
    /// η strictly above rolling P75 (before Landauer proximity split).
    Frugal,
}

/// Clamp |ρ| away from 1 so `log₂(1−ρ²)` stays finite.
const RHO_CLAMP_ABS: f64 = 0.9999;

/// `MI(ρ) = −½ · log₂(1 − ρ²)` in bits for `|ρ| < 1`.
///
/// Proof: UMST.Formal.RhoEstimator::rho_based_mi_formula
#[must_use]
pub fn rho_mi_bits(rho: f64) -> f64 {
    if !rho.is_finite() {
        return 0.0;
    }
    let r = rho.clamp(-RHO_CLAMP_ABS, RHO_CLAMP_ABS);
    let z = 1.0 - r * r;
    if z <= 0.0 || !z.is_finite() {
        return 0.0;
    }
    -0.5 * z.ln() / std::f64::consts::LN_2
}

/// Shared mean helper for scalar + SIMD Pearson paths.
pub(super) fn mean_from_slice(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut s = 0.0;
    let mut c = 0usize;
    for &x in xs {
        if !x.is_finite() {
            return None;
        }
        s += x;
        c += 1;
    }
    Some(s / c as f64)
}

/// Pearson correlation then [`rho_mi_bits`]; `None` if fewer than two finite samples or zero variance.
///
/// Proof: UMST.Formal.RhoEstimator::rho_based_mi_formula (plug-in on empirical ρ̂)
#[must_use]
pub fn rho_mi_from_samples_scalar(xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() || xs.len() < 2 {
        return None;
    }
    let mx = mean_from_slice(xs)?;
    let my = mean_from_slice(ys)?;
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for (x, y) in xs.iter().zip(ys.iter()) {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let dx = x - mx;
        let dy = y - my;
        num += dx * dy;
        dx2 += dx * dx;
        dy2 += dy * dy;
    }
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

/// NIST-style linear-interpolated percentile on a **sorted** slice (`q` is the fraction in \[0, 1\]).
#[must_use]
pub fn sample_percentile_presorted(sorted: &[f64], q: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    if sorted.len() == 1 {
        return Some(sorted[0]);
    }
    let p = q.clamp(0.0, 1.0);
    let rank = p * (sorted.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        Some(sorted[lo])
    } else {
        let frac = rank - lo as f64;
        Some(sorted[lo].mul_add(1.0 - frac, sorted[hi] * frac))
    }
}

/// Same as [`sample_percentile_presorted`] after copying `samples`, retaining only finite values, and sorting.
#[must_use]
pub fn sample_percentile_scalar(samples: &[f64], q: f64) -> Option<f64> {
    let mut v: Vec<f64> = samples.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sample_percentile_presorted(&v, q)
}

/// Pragmatic band classifier: η \< P25 → [`BandLabel::Wasteful`], η \> P75 → [`BandLabel::Frugal`], else [`BandLabel::Balanced`].
///
/// Proof: `UMST.Formal.OrderStatisticsBand::p25_p75_admissibility`
#[must_use]
pub fn classify_band_scalar(samples: &[f64], eta: f64) -> Option<BandLabel> {
    if !eta.is_finite() {
        return None;
    }
    let mut v: Vec<f64> = samples.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p25 = sample_percentile_presorted(&v, 0.25)?;
    let p75 = sample_percentile_presorted(&v, 0.75)?;
    Some(if eta < p25 {
        BandLabel::Wasteful
    } else if eta > p75 {
        BandLabel::Frugal
    } else {
        BandLabel::Balanced
    })
}

/// Uniform bucket-max downsample to `width` sparkline heights in `[0, max_scale]`.
///
/// Used by egoff H-6b bashtop cockpit stripe; scalar reference for optional SIMD path.
#[must_use]
pub fn downsample_sparkline_u64_scalar(samples: &[f64], width: usize, max_scale: u64) -> Vec<u64> {
    if width == 0 {
        return Vec::new();
    }
    if samples.is_empty() {
        return vec![0; width];
    }
    let finite: Vec<f64> = samples.iter().copied().filter(|x| x.is_finite()).collect();
    if finite.is_empty() {
        return vec![0; width];
    }
    let lo = finite.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = finite.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = if (hi - lo).abs() < 1e-12 { 1.0 } else { hi - lo };
    let n = finite.len();
    let mut out = Vec::with_capacity(width);
    for bucket in 0..width {
        let start = bucket * n / width;
        let end = ((bucket + 1) * n / width).max(start + 1).min(n);
        let peak = finite[start..end]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let norm = if peak.is_finite() {
            ((peak - lo) / span).clamp(0.0, 1.0)
        } else {
            0.0
        };
        out.push((norm * max_scale as f64).round() as u64);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernels;

    /// Deterministic PRNG (splitmix64) for parity tables — no external `rand` dev-dep.
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

    #[test]
    fn sample_percentile_matches_two_point_midpoint() {
        let s = [0.0_f64, 10.0];
        assert!((sample_percentile_presorted(&s, 0.25).unwrap() - 2.5).abs() < 1e-12);
        assert!((sample_percentile_presorted(&s, 0.75).unwrap() - 7.5).abs() < 1e-12);
    }

    #[test]
    fn classify_band_separates_by_quartiles() {
        let w = [0.3_f64, 0.35, 0.4, 0.45, 0.5, 0.55];
        assert_eq!(classify_band_scalar(&w, 0.25), Some(BandLabel::Wasteful));
        assert_eq!(classify_band_scalar(&w, 0.42), Some(BandLabel::Balanced));
        assert_eq!(classify_band_scalar(&w, 0.9), Some(BandLabel::Frugal));
    }

    #[test]
    fn classify_band_invariant_under_uniform_shift() {
        let w = [0.3_f64, 0.35, 0.4, 0.45, 0.5, 0.55];
        let w2 = [1000.3, 1000.35, 1000.4, 1000.45, 1000.5, 1000.55];
        assert_eq!(
            classify_band_scalar(&w, 0.25),
            classify_band_scalar(&w2, 1000.25)
        );
    }

    #[test]
    fn sample_percentile_empty_none() {
        let empty: [f64; 0] = [];
        assert!(sample_percentile_scalar(&empty, 0.5).is_none());
    }

    /// 1024-table parity: scalar entry points match the public dispatch path (default features → scalar).
    #[test]
    fn scalar_tables_match_dispatch_path_1024() {
        for k in 0..1024u64 {
            let n = 2 + (k % 48) as usize;
            let mut xs = vec![0.0; n];
            let mut ys = vec![0.0; n];
            for i in 0..n {
                let t = k.wrapping_mul(1_039).wrapping_add(i as u64);
                xs[i] = u01(t) * 10.0 - 5.0;
                ys[i] = u01(t.wrapping_add(777)) * 10.0 - 5.0;
            }
            #[cfg(not(feature = "simd"))]
            assert_eq!(
                rho_mi_from_samples_scalar(&xs, &ys),
                kernels::rho_mi_from_samples(&xs, &ys)
            );
            let q = 0.1 + (k % 80) as f64 / 100.0;
            assert_eq!(
                sample_percentile_scalar(&xs, q),
                kernels::sample_percentile(&xs, q)
            );
            let eta = u01(k.wrapping_mul(3)) * 2.0;
            assert_eq!(
                classify_band_scalar(&xs, eta),
                kernels::classify_band(&xs, eta)
            );
        }
    }
}

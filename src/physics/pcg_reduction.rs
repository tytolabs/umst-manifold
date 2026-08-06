// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Fused Krylov reductions for host Q1Hex PCG (H2) — f64 accumulators + chunked loops for LLVM SIMD.
//!
//! # Honest boundary (W29-064)
//!
//! Chunked host reductions (`dot_*`, `masked_dot_*`, `masked_norm_sq_*`, `norm_sq_f64`) are the
//! **measured** Krylov inner-product surface used by Q1Hex PCG. Unit contracts:
//! `cargo test -p umst-manifold pcg_reduction`. Not physics GREEN, not `PRODUCTION_WIRED`, not
//! `MASTER` / OP-5.

/// W29 deepen cell — PCG reduction honest fence bundle.
pub const W29_PCG_REDUCTION_DEEPEN_CELL: &str = "W29-064-PCG_REDUCTION";

/// Honest posture tag — Krylov reductions landed; fleet production wiring refused.
pub const PCG_REDUCTION_POSTURE_TAG: &str = "honest-pcg-krylov-reduction-research-lane";

/// Honest physics posture — reduction unit contracts pass; does not certify fleet physics GREEN.
pub const PCG_REDUCTION_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by host Krylov reductions alone.
pub const PCG_REDUCTION_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const PCG_REDUCTION_MASTER: bool = false;

/// Whether chunked f32/f64 Krylov reduction kernels are landed in this module.
pub const PCG_REDUCTION_KRYLOV_LANDED: bool = true;

/// Whether f64 masked-dot parity with f32 surface is landed.
pub const PCG_REDUCTION_MASKED_DOT_F64_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const PCG_REDUCTION_HONEST_FENCE: &str =
    "pcg_krylov_reductions_landed=true masked_dot_f64_landed=true chunked_f64_acc=true production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!PCG_REDUCTION_PRODUCTION_WIRED);
const _: () = assert!(!PCG_REDUCTION_PHYSICS_GREEN);
const _: () = assert!(!PCG_REDUCTION_MASTER);
const _: () = assert!(PCG_REDUCTION_KRYLOV_LANDED);
const _: () = assert!(PCG_REDUCTION_MASKED_DOT_F64_LANDED);

/// Typed probe for PCG reduction posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcgReductionPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub krylov_landed: bool,
    pub masked_dot_f64_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for PCG Krylov reductions.
#[must_use]
pub fn pcg_reduction_honest_posture_bundle() -> PcgReductionPostureProbe {
    PcgReductionPostureProbe {
        physics_green: PCG_REDUCTION_PHYSICS_GREEN,
        production_wired: PCG_REDUCTION_PRODUCTION_WIRED,
        master: PCG_REDUCTION_MASTER,
        krylov_landed: PCG_REDUCTION_KRYLOV_LANDED,
        masked_dot_f64_landed: PCG_REDUCTION_MASKED_DOT_F64_LANDED,
        honest_fence: PCG_REDUCTION_HONEST_FENCE,
        posture_tag: PCG_REDUCTION_POSTURE_TAG,
        deepen_cell: W29_PCG_REDUCTION_DEEPEN_CELL,
    }
}

/// Krylov SSOT landed with production/master/GREEN composition honestly open.
#[must_use]
pub fn pcg_reduction_posture_honest(probe: &PcgReductionPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.krylov_landed
        && probe.masked_dot_f64_landed
        && probe.honest_fence.contains("pcg_krylov_reductions_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the PCG reduction surface.
#[must_use]
pub fn pcg_reduction_refuse_overclaim(
    probe: &PcgReductionPostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("PCG_REDUCTION_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("PCG_REDUCTION_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("PCG_REDUCTION_MASTER must stay false — not claimed by Krylov reductions alone");
    }
    if !pcg_reduction_posture_honest(probe) {
        return Err("pcg_reduction posture fence inconsistent");
    }
    Ok(())
}

const CHUNK: usize = 8;

/// \(\sum_i a_i b_i\) with f64 accumulator (f32 lanes).
#[inline]
pub fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            acc += f64::from(a[j]) * f64::from(b[j]);
        }
        i += CHUNK;
    }
    while i < n {
        acc += f64::from(a[i]) * f64::from(b[i]);
        i += 1;
    }
    acc as f32
}

/// \(\sum_i a_i m_i b_i\).
#[inline]
pub fn masked_dot_f32(a: &[f32], b: &[f32], mask: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), mask.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            acc += f64::from(a[j]) * f64::from(mask[j]) * f64::from(b[j]);
        }
        i += CHUNK;
    }
    while i < n {
        acc += f64::from(a[i]) * f64::from(mask[i]) * f64::from(b[i]);
        i += 1;
    }
    acc as f32
}

/// \(\sum_i (a_i m_i)^2\).
#[inline]
pub fn masked_norm_sq_f32(a: &[f32], mask: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), mask.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            let v = f64::from(a[j]) * f64::from(mask[j]);
            acc += v * v;
        }
        i += CHUNK;
    }
    while i < n {
        let v = f64::from(a[i]) * f64::from(mask[i]);
        acc += v * v;
        i += 1;
    }
    acc as f32
}

/// \(\sum_i a_i b_i\) (f64 lane).
#[inline]
pub fn dot_f64(a: &[f64], b: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            acc += a[j] * b[j];
        }
        i += CHUNK;
    }
    while i < n {
        acc += a[i] * b[i];
        i += 1;
    }
    acc
}

/// \(\sum_i a_i m_i b_i\) (f64 lane) — parity with [`masked_dot_f32`].
#[inline]
pub fn masked_dot_f64(a: &[f64], b: &[f64], mask: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), mask.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            acc += a[j] * mask[j] * b[j];
        }
        i += CHUNK;
    }
    while i < n {
        acc += a[i] * mask[i] * b[i];
        i += 1;
    }
    acc
}

/// \(\sum_i (a_i m_i)^2\) (f64 lane).
#[inline]
pub fn masked_norm_sq_f64(a: &[f64], mask: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), mask.len());
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            let v = a[j] * mask[j];
            acc += v * v;
        }
        i += CHUNK;
    }
    while i < n {
        let v = a[i] * mask[i];
        acc += v * v;
        i += 1;
    }
    acc
}

/// \(\sum_i a_i^2\).
#[inline]
pub fn norm_sq_f64(a: &[f64]) -> f64 {
    let n = a.len();
    let mut acc = 0.0_f64;
    let mut i = 0;
    while i + CHUNK <= n {
        for k in 0..CHUNK {
            let j = i + k;
            acc += a[j] * a[j];
        }
        i += CHUNK;
    }
    while i < n {
        acc += a[i] * a[i];
        i += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_dot_f32(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    fn naive_masked_dot_f32(a: &[f32], b: &[f32], m: &[f32]) -> f32 {
        a.iter().zip(b).zip(m).map(|((x, y), w)| x * y * w).sum()
    }

    fn naive_masked_norm_sq_f32(a: &[f32], m: &[f32]) -> f32 {
        a.iter().zip(m).map(|(x, w)| {
            let v = x * w;
            v * v
        }).sum()
    }

    fn naive_dot_f64(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    fn naive_masked_dot_f64(a: &[f64], b: &[f64], m: &[f64]) -> f64 {
        a.iter().zip(b).zip(m).map(|((x, y), w)| x * y * w).sum()
    }

    fn naive_masked_norm_sq_f64(a: &[f64], m: &[f64]) -> f64 {
        a.iter().zip(m).map(|(x, w)| {
            let v = x * w;
            v * v
        }).sum()
    }

    fn naive_norm_sq_f64(a: &[f64]) -> f64 {
        a.iter().map(|x| x * x).sum()
    }

    #[test]
    fn pcg_reduction_honest_posture_refuses_green_and_production() {
        let probe = pcg_reduction_honest_posture_bundle();
        assert!(pcg_reduction_posture_honest(&probe));
        assert!(pcg_reduction_refuse_overclaim(&probe).is_ok());
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert_eq!(probe.deepen_cell, W29_PCG_REDUCTION_DEEPEN_CELL);
        assert!(probe.krylov_landed);
        assert!(probe.masked_dot_f64_landed);
    }

    #[test]
    fn dot_matches_naive() {
        let a: Vec<f32> = (0..1000).map(|i| (i as f32) * 1e-4).collect();
        let b: Vec<f32> = (0..1000).map(|i| ((i + 3) as f32) * 2e-4).collect();
        let d = dot_f32(&a, &b);
        let n = naive_dot_f32(&a, &b);
        assert!((d - n).abs() < 1e-3 * n.abs().max(1.0));
    }

    #[test]
    fn masked_dot_matches_naive() {
        let a: Vec<f32> = (0..512).map(|i| (i as f32).sin()).collect();
        let b: Vec<f32> = (0..512).map(|i| (i as f32).cos()).collect();
        let m: Vec<f32> = (0..512)
            .map(|i| if i % 3 == 0 { 1.0 } else { 0.0 })
            .collect();
        let fused = masked_dot_f32(&a, &b, &m);
        let naive = naive_masked_dot_f32(&a, &b, &m);
        assert!((fused - naive).abs() < 1e-5);
    }

    #[test]
    fn masked_norm_sq_f32_matches_naive_remainder() {
        // Length not multiple of CHUNK exercises scalar tail.
        let n = 19usize;
        let a: Vec<f32> = (0..n).map(|i| (i as f32) * 0.1 + 0.25).collect();
        let m: Vec<f32> = (0..n).map(|i| if i % 2 == 0 { 1.0 } else { 0.0 }).collect();
        let fused = masked_norm_sq_f32(&a, &m);
        let naive = naive_masked_norm_sq_f32(&a, &m);
        assert!((fused - naive).abs() < 1e-5);
    }

    #[test]
    fn f64_reductions_match_naive_including_empty() {
        assert_eq!(dot_f64(&[], &[]), 0.0);
        assert_eq!(masked_dot_f64(&[], &[], &[]), 0.0);
        assert_eq!(masked_norm_sq_f64(&[], &[]), 0.0);
        assert_eq!(norm_sq_f64(&[]), 0.0);

        let n = 37usize; // non-multiple of CHUNK
        let a: Vec<f64> = (0..n).map(|i| (i as f64) * 1e-3 - 0.2).collect();
        let b: Vec<f64> = (0..n).map(|i| ((n - i) as f64) * 2e-3).collect();
        let m: Vec<f64> = (0..n)
            .map(|i| if i % 4 == 0 { 0.0 } else { 1.0 })
            .collect();

        let d = dot_f64(&a, &b);
        assert!((d - naive_dot_f64(&a, &b)).abs() < 1e-12);

        let md = masked_dot_f64(&a, &b, &m);
        assert!((md - naive_masked_dot_f64(&a, &b, &m)).abs() < 1e-12);

        let ns = masked_norm_sq_f64(&a, &m);
        assert!((ns - naive_masked_norm_sq_f64(&a, &m)).abs() < 1e-12);

        let nsq = norm_sq_f64(&a);
        assert!((nsq - naive_norm_sq_f64(&a)).abs() < 1e-12);
    }
}

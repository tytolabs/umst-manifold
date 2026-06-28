// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Fused Krylov reductions for host Q1Hex PCG (H2) — f64 accumulators + chunked loops for LLVM SIMD.

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
        let naive: f32 = a.iter().zip(&b).zip(&m).map(|((x, y), w)| x * y * w).sum();
        assert!((fused - naive).abs() < 1e-5);
    }
}

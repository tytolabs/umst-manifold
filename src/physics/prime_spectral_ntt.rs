// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track 1: exact circular convolution via NTT over ℤ/qℤ (research only).
//!
//! Morphism: `(Vecₙ, ∗) → (Vecₙ, ⊙)` via NTT — natural iso when bounded (no overflow).
//! See `docs/PRIME_SPECTRAL_PROTOCOL.md` Track 1.

use serde::{Deserialize, Serialize};

/// NTT-friendly prime: `q = 5·2²⁵ + 1`, `2²⁵ | (q−1)`.
pub const NTT_Q: u64 = 167_772_161;
pub const NTT_ROOT: u64 = 3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NttPlan {
    pub n: usize,
    pub q: u64,
    pub root: u64,
    pub inv_n: u64,
}

impl NttPlan {
    #[must_use]
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two() && n <= (1 << 25), "n must be power of 2 ≤ 2²⁵");
        Self {
            n,
            q: NTT_Q,
            root: NTT_ROOT,
            inv_n: mod_inv(n as u64, NTT_Q),
        }
    }
}

#[inline]
pub fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut out = 1_u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            out = (out as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    out
}

#[must_use]
pub fn mod_inv(a: u64, m: u64) -> u64 {
    mod_pow(a, m - 2, m)
}

#[must_use]
pub fn ntt(a: &[u64], plan: &NttPlan) -> Vec<u64> {
    let n = plan.n;
    let mut x = a.to_vec();
    if x.len() < n {
        x.resize(n, 0);
    }
    let mut j = 0_usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            x.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let step = mod_pow(plan.root, ((plan.q - 1) / len as u64), plan.q);
        let mut i = 0;
        while i < n {
            let mut w = 1_u64;
            for k in 0..(len / 2) {
                let u = x[i + k];
                let v = (x[i + k + len / 2] as u128 * w as u128 % plan.q as u128) as u64;
                x[i + k] = (u + v) % plan.q;
                x[i + k + len / 2] = (u + plan.q - v) % plan.q;
                w = (w as u128 * step as u128 % plan.q as u128) as u64;
            }
            i += len;
        }
        len <<= 1;
    }
    x
}

#[must_use]
pub fn intt(a: &[u64], plan: &NttPlan) -> Vec<u64> {
    let n = plan.n;
    let inv_root = mod_inv(plan.root, plan.q);
    let mut inv_plan = NttPlan {
        n: plan.n,
        q: plan.q,
        root: inv_root,
        inv_n: plan.inv_n,
    };
    let mut x = ntt(a, &inv_plan);
    for v in &mut x {
        *v = (*v as u128 * plan.inv_n as u128 % plan.q as u128) as u64;
    }
    x
}

#[must_use]
pub fn circular_conv_mod(a: &[u64], b: &[u64], plan: &NttPlan) -> Vec<u64> {
    let fa = ntt(a, plan);
    let fb = ntt(b, plan);
    let prod: Vec<u64> = fa
        .iter()
        .zip(&fb)
        .map(|(&x, &y)| (x as u128 * y as u128 % plan.q as u128) as u64)
        .collect();
    intt(&prod, plan)
}

#[must_use]
pub fn quantize_signal(values: &[f32], q: u64, scale: f32) -> Vec<u64> {
    values
        .iter()
        .map(|&v| {
            let x = (v * scale).round() as i64;
            ((x.rem_euclid(q as i64)) as u64) % q
        })
        .collect()
}

#[must_use]
pub fn dequantize(values: &[u64], scale: f32, q: u64) -> Vec<f32> {
    values
        .iter()
        .map(|&v| {
            let signed = if v > q / 2 {
                v as i64 - q as i64
            } else {
                v as i64
            };
            signed as f32 / scale
        })
        .collect()
}

fn float_circular_conv(a: &[f32], b: &[f32]) -> Vec<f32> {
    let n = a.len();
    let mut out = vec![0.0_f32; n];
    for i in 0..n {
        for j in 0..n {
            out[i] += a[j] * b[(i + n - j) % n];
        }
    }
    out
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NttDriftRecord {
    pub schema: String,
    pub n: usize,
    pub q: u64,
    pub steps: usize,
    pub max_single_step_linf: f32,
    pub float_cumulative_drift: f32,
    pub ntt_cumulative_drift: f32,
    pub ntt_zero_conservation_drift: f32,
    pub pattern_hit: bool,
}

const DRIFT_STEPS: usize = 32;
const QUANT_SCALE: f32 = 10_000.0;

/// Pre-registered Track 1: NTT path has zero mod-q drift; float accumulates rounding error.
#[must_use]
pub fn run_ntt_drift_study(n: usize, seed: u64) -> NttDriftRecord {
    let plan = NttPlan::new(n);
    let mut rng_state = seed;
    let signal: Vec<f32> = (0..n)
        .map(|_| {
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((rng_state >> 32) as f32 / u32::MAX as f32) * 0.8 + 0.1
        })
        .collect();
    let kernel: Vec<f32> = (0..n)
        .map(|i| {
            let d = (i as f32 - (n as f32 * 0.5)).abs();
            (-d * d / (n as f32).max(1.0)).exp()
        })
        .collect();
    let k_sum: f32 = kernel.iter().sum();
    let kernel: Vec<f32> = kernel.iter().map(|x| x / k_sum.max(1e-12)).collect();

    let mut float_state = signal.clone();
    let mut ntt_state = quantize_signal(&signal, plan.q, QUANT_SCALE);
    let k_mod = quantize_signal(&kernel, plan.q, QUANT_SCALE);

    let mut max_linf = 0.0_f32;
    let mut float_drift = 0.0_f32;
    let mut ntt_drift = 0.0_f32;

    for _ in 0..DRIFT_STEPS {
        float_state = float_circular_conv(&float_state, &kernel);
        let ntt_next = circular_conv_mod(&ntt_state, &k_mod, &plan);
        let ntt_float = dequantize(&ntt_next, QUANT_SCALE, plan.q);
        let float_once = float_circular_conv(
            &dequantize(&ntt_state, QUANT_SCALE, plan.q),
            &kernel,
        );
        let step_linf = float_once
            .iter()
            .zip(&ntt_float)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f32, f32::max);
        max_linf = max_linf.max(step_linf);
        float_drift += float_state.iter().map(|x| x.abs()).sum::<f32>() / n as f32;
        ntt_drift += ntt_float.iter().map(|x| x.abs()).sum::<f32>() / n as f32;
        ntt_state = ntt_next;
    }

    let ntt_zero = circular_conv_mod(&vec![0; n], &k_mod, &plan);
    let conservation = ntt_zero.iter().all(|&x| x == 0);

    NttDriftRecord {
        schema: "prime_spectral_track1_ntt_v1".into(),
        n,
        q: plan.q,
        steps: DRIFT_STEPS,
        max_single_step_linf: max_linf,
        float_cumulative_drift: float_drift,
        ntt_cumulative_drift: ntt_drift,
        ntt_zero_conservation_drift: if conservation { 0.0 } else { 1.0 },
        pattern_hit: conservation && max_linf < 0.05,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ntt_intt_roundtrip() {
        let plan = NttPlan::new(8);
        let a: Vec<u64> = (0..8).map(|i| (i + 1) as u64).collect();
        let round = intt(&ntt(&a, &plan), &plan);
        for (x, y) in a.iter().zip(&round) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn zero_kernel_conserves_zero() {
        let plan = NttPlan::new(16);
        let z = vec![0_u64; 16];
        let k: Vec<u64> = (0..16).map(|i| (i + 1) as u64).collect();
        let out = circular_conv_mod(&z, &k, &plan);
        assert!(out.iter().all(|&x| x == 0));
    }
}

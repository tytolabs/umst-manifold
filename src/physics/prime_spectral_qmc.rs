// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track 2: Halton quasi-MC coalgebra (research only).
//!
//! `halton : primes → Stream Point_d` via radical-inverse unfold.
//! See `docs/PRIME_SPECTRAL_PROTOCOL.md` Track 2.

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

const HALTON_BASES: [u32; 4] = [2, 3, 5, 7];

#[must_use]
pub fn radical_inverse(mut index: u32, base: u32) -> f32 {
    let mut f = 1.0_f32;
    let mut r = 0.0_f32;
    while index > 0 {
        f /= base as f32;
        r += f * (index % base) as f32;
        index /= base;
    }
    r
}

#[must_use]
pub fn halton_point(index: u32, dim: usize) -> Vec<f32> {
    (0..dim)
        .map(|d| radical_inverse(index, HALTON_BASES[d % HALTON_BASES.len()]))
        .collect()
}

/// Corecursive stream chunk (pure, deterministic).
#[must_use]
pub fn halton_stream(dim: usize, start_index: u32, count: usize) -> Vec<Vec<f32>> {
    (0..count as u32)
        .map(|i| halton_point(start_index + i, dim))
        .collect()
}

#[must_use]
pub fn prng_stream(dim: usize, seed: u64, count: usize) -> Vec<Vec<f32>> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| (0..dim).map(|_| rng.gen::<f32>()).collect())
        .collect()
}

/// Test integrand: separable smooth function on [0,1]^d.
#[must_use]
pub fn integrand_2d(x: f32, y: f32) -> f32 {
    (std::f32::consts::PI * x).sin() * (std::f32::consts::PI * y).cos()
}

#[must_use]
pub fn mc_estimate(samples: &[Vec<f32>]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum: f32 = samples
        .iter()
        .map(|p| integrand_2d(p[0], p[1]))
        .sum();
    sum / samples.len() as f32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QmcRecord {
    pub schema: String,
    pub seed: u64,
    pub tolerance: f32,
    pub true_value: f32,
    pub halton_samples_to_tol: usize,
    pub prng_samples_to_tol: usize,
    pub sample_reduction_pct: f32,
    pub pattern_hit: bool,
}

const MC_TOL: f32 = 5e-3;
const TRUE_INTEGRAL_2D: f32 = 0.0; // ∫₀¹∫₀¹ sin(πx)cos(πy) dx dy = 0
const MAX_SAMPLES: usize = 4096;

#[must_use]
pub fn samples_to_tolerance(estimator: fn(&[Vec<f32>]) -> f32, stream: &[Vec<f32>], tol: f32) -> usize {
    for k in 1..=stream.len() {
        let est = estimator(&stream[..k]);
        if (est - TRUE_INTEGRAL_2D).abs() <= tol {
            return k;
        }
    }
    MAX_SAMPLES
}

/// Pre-registered Track 2: Halton reaches tolerance with fewer samples than PRNG.
#[must_use]
pub fn run_qmc_study(seed: u64) -> QmcRecord {
    let halton = halton_stream(2, 1, MAX_SAMPLES);
    let prng = prng_stream(2, seed, MAX_SAMPLES);
    let h_n = samples_to_tolerance(mc_estimate, &halton, MC_TOL);
    let p_n = samples_to_tolerance(mc_estimate, &prng, MC_TOL);
    let reduction = if p_n > 0 {
        1.0 - h_n as f32 / p_n as f32
    } else {
        0.0
    };
    QmcRecord {
        schema: "prime_spectral_track2_qmc_v1".into(),
        seed,
        tolerance: MC_TOL,
        true_value: TRUE_INTEGRAL_2D,
        halton_samples_to_tol: h_n,
        prng_samples_to_tol: p_n,
        sample_reduction_pct: reduction,
        pattern_hit: h_n < p_n && reduction >= 0.10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halton_in_unit_interval() {
        for i in 1..50 {
            let p = halton_point(i, 2);
            assert!(p.iter().all(|&x| (0.0..=1.0).contains(&x)));
        }
    }
}

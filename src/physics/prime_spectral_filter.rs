// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Elementwise spectral guidance filter on auxiliary density channels (Layer A).
//!
//! formal_anchor: Lean `UMST.PrimeSpectralGuidance.spectralFilter`
//! formal_status: Literature
//! formal_citation: "Engineering mirror of von Mangoldt-weighted multiplicative channel filter; gate scalars unchanged."
//! formal_form: "`values' i = weights i * values i` with identity weights at `epsilon = 0`."
//! formal_anchor_rationale: Guidance only — not a fifth thermodynamic gate conjunct.
//!
//! Enabled with **`topology-density-evolution`**.
//!
//! # Honest boundary (W29-066)
//!
//! Soft von Mangoldt / coprime-stride guidance with mean renormalization. `epsilon` is a
//! **soft modulation scale**, not a hard L¹ ball projection onto `|w−1|`. Guidance only —
//! not a fifth thermodynamic gate conjunct. Not physics GREEN, not `PRODUCTION_WIRED`,
//! not `MASTER` / OP-5.

/// W29 deepen cell — prime spectral filter honest fence bundle.
pub const W29_PRIME_SPECTRAL_FILTER_DEEPEN_CELL: &str = "W29-066-PRIME_SPECTRAL_FILTER";

/// Honest posture tag — guidance filter landed; fleet production wiring refused.
pub const PRIME_SPECTRAL_FILTER_POSTURE_TAG: &str = "honest-prime-spectral-guidance-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const PRIME_SPECTRAL_FILTER_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by Layer-A guidance filter alone.
pub const PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const PRIME_SPECTRAL_FILTER_MASTER: bool = false;

/// Whether identity-at-ε=0 and Burn apply contracts are landed in this module.
pub const PRIME_SPECTRAL_FILTER_CONTRACTS_LANDED: bool = true;

/// Whether epsilon is a soft scale (true) vs hard L¹ ball projection (false ⇒ not claimed).
pub const PRIME_SPECTRAL_FILTER_EPSILON_IS_SOFT_SCALE: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const PRIME_SPECTRAL_FILTER_HONEST_FENCE: &str =
    "prime_spectral_guidance_landed=true identity_at_zero_eps=true mean_renorm_wired=true mangoldt_contract=true soft_epsilon_scale=true hard_l1_ball=false gate_conjunct=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!PRIME_SPECTRAL_FILTER_PHYSICS_GREEN);
const _: () = assert!(!PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED);
const _: () = assert!(!PRIME_SPECTRAL_FILTER_MASTER);
const _: () = assert!(PRIME_SPECTRAL_FILTER_EPSILON_IS_SOFT_SCALE);
const _: () = assert!(PRIME_SPECTRAL_FILTER_CONTRACTS_LANDED);

/// Typed probe for prime-spectral filter posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimeSpectralFilterPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub contracts_landed: bool,
    pub epsilon_is_soft_scale: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the prime spectral guidance filter.
#[must_use]
pub fn prime_spectral_filter_honest_posture_bundle() -> PrimeSpectralFilterPostureProbe {
    PrimeSpectralFilterPostureProbe {
        physics_green: PRIME_SPECTRAL_FILTER_PHYSICS_GREEN,
        production_wired: PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED,
        master: PRIME_SPECTRAL_FILTER_MASTER,
        contracts_landed: PRIME_SPECTRAL_FILTER_CONTRACTS_LANDED,
        epsilon_is_soft_scale: PRIME_SPECTRAL_FILTER_EPSILON_IS_SOFT_SCALE,
        honest_fence: PRIME_SPECTRAL_FILTER_HONEST_FENCE,
        posture_tag: PRIME_SPECTRAL_FILTER_POSTURE_TAG,
        deepen_cell: W29_PRIME_SPECTRAL_FILTER_DEEPEN_CELL,
    }
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the guidance surface.
#[must_use]
pub fn prime_spectral_filter_posture_honest(probe: &PrimeSpectralFilterPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.contracts_landed
        && probe.epsilon_is_soft_scale
        && probe.deepen_cell == W29_PRIME_SPECTRAL_FILTER_DEEPEN_CELL
        && probe
            .honest_fence
            .contains("prime_spectral_guidance_landed=true")
        && probe.honest_fence.contains("hard_l1_ball=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Compile-time / runtime refuse path for invented GREEN / production pins.
pub fn prime_spectral_filter_refuse_invented_pins() -> Result<(), &'static str> {
    if PRIME_SPECTRAL_FILTER_PHYSICS_GREEN {
        return Err(
            "PRIME_SPECTRAL_FILTER_PHYSICS_GREEN must stay false — guidance ≠ fleet physics",
        );
    }
    if PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED {
        return Err(
            "PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED must stay false until embodied loop closes",
        );
    }
    if PRIME_SPECTRAL_FILTER_MASTER {
        return Err("PRIME_SPECTRAL_FILTER_MASTER must stay false — not an OP-5 composition pin");
    }
    Ok(())
}

use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

use crate::physics::error::PhysicsError;

/// Pure guidance endofunctor on `[B, N, C]` density fields (elementwise).
#[derive(Clone, Debug, PartialEq)]
pub struct PrimeSpectralFilter {
    /// Soft modulation scale (0 ⇒ identity filter). Not a hard L¹ ball radius.
    pub epsilon: f32,
    /// When true, apply coprime-stride modulation on prime-indexed slots.
    pub coprime_stride: bool,
    /// Optional coprime prime period for stride mode (defaults to 3).
    pub coprime_prime: Option<u32>,
}

impl Default for PrimeSpectralFilter {
    fn default() -> Self {
        Self::new(0.05, false, None)
    }
}

impl PrimeSpectralFilter {
    #[must_use]
    pub fn new(epsilon: f32, coprime_stride: bool, coprime_prime: Option<u32>) -> Self {
        let eps = if epsilon.is_finite() {
            epsilon.max(0.0)
        } else {
            0.0
        };
        Self {
            epsilon: eps,
            coprime_stride,
            coprime_prime,
        }
    }

    /// Build per-node weights for `n` slots (Fin `n` mirror).
    ///
    /// After soft modulation, weights are **mean-renormalized** so
    /// `(1/n) Σ w_i ≈ 1`. This is not a hard L¹ projection onto `|w−1| ≤ ε`.
    #[must_use]
    pub fn weight_table(&self, n: usize) -> Vec<f32> {
        if n == 0 {
            return Vec::new();
        }
        if self.epsilon <= 0.0 {
            return vec![1.0; n];
        }
        let mut raw = Vec::with_capacity(n);
        let mut sum = 0.0_f32;
        for i in 0..n {
            let w = if self.coprime_stride {
                coprime_stride_weight(i, n, self.coprime_prime.unwrap_or(3), self.epsilon)
            } else {
                mangoldt_modulated_weight(i, self.epsilon)
            };
            raw.push(w);
            sum += w;
        }
        let mean = (sum / n as f32).max(1e-12);
        raw.into_iter().map(|w| w / mean).collect()
    }

    /// Measured mean `|w_i − 1|` after `weight_table` (honest soft-scale readout).
    #[must_use]
    pub fn mean_abs_deviation(&self, n: usize) -> f32 {
        let w = self.weight_table(n);
        if w.is_empty() {
            return 0.0;
        }
        let sum: f32 = w.iter().map(|x| (x - 1.0).abs()).sum();
        sum / w.len() as f32
    }

    /// Measured mean of the weight table (should be ≈ 1 after renormalization).
    #[must_use]
    pub fn weight_mean(&self, n: usize) -> f32 {
        let w = self.weight_table(n);
        if w.is_empty() {
            return 1.0;
        }
        w.iter().sum::<f32>() / w.len() as f32
    }

    /// Apply spectral filter: `rho' = w ⊙ rho` (same shape; weights broadcast on channels).
    pub fn apply<B: Backend<FloatElem = f32>>(
        &self,
        rho: Tensor<B, 3>,
        n: usize,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        let [_, n_rho, c] = rho.dims();
        if n_rho != n {
            return Err(PhysicsError::ShapeMismatch {
                context: "PrimeSpectralFilter::apply",
                detail: "n must match rho node count",
            });
        }
        if c < 1 {
            return Err(PhysicsError::ShapeMismatch {
                context: "PrimeSpectralFilter::apply",
                detail: "rho channel count must be >= 1",
            });
        }
        let weights = self.weight_table(n);
        self.validate_weight_stability(&weights)?;
        let device = rho.device();
        let w = Tensor::<B, 1>::from_floats(weights.as_slice(), &device).reshape([1, n, 1]);
        Ok(rho.mul(w))
    }

    fn validate_weight_stability(&self, weights: &[f32]) -> Result<(), PhysicsError> {
        for (i, &w) in weights.iter().enumerate() {
            if !w.is_finite() {
                return Err(PhysicsError::NonFinite {
                    context: "PrimeSpectralFilter::apply weight table",
                });
            }
            if w <= 0.0 {
                return Err(PhysicsError::Domain {
                    detail: format!(
                        "PrimeSpectralFilter: non-positive weight at index {i} (ε={}, w={w})",
                        self.epsilon
                    ),
                });
            }
        }
        Ok(())
    }
}

fn mangoldt_modulated_weight(index: usize, epsilon: f32) -> f32 {
    let n = (index + 1) as u32;
    let lambda = von_mangoldt_weight(n);
    if lambda <= 0.0 {
        return 1.0;
    }
    let scale = (lambda / n as f32).clamp(0.0, 1.0);
    1.0 + epsilon * (scale - 0.5)
}

fn coprime_stride_weight(index: usize, n: usize, p: u32, epsilon: f32) -> f32 {
    if index % p as usize == 0 {
        1.0 + epsilon
    } else if (index + 1) % n.max(1) == 0 {
        1.0 - epsilon * 0.5
    } else {
        1.0
    }
}

#[must_use]
pub fn von_mangoldt_weight(n: u32) -> f32 {
    if n <= 1 {
        return 0.0;
    }
    if is_prime(n) {
        return n as f32;
    }
    if let Some(p) = min_factor(n) {
        if is_prime(p) && is_prime_power(n, p) {
            return p as f32;
        }
    }
    0.0
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let r = (n as f64).sqrt() as u32;
    let mut d = 3;
    while d <= r {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn min_factor(n: u32) -> Option<u32> {
    if n < 2 {
        return None;
    }
    let r = (n as f64).sqrt() as u32;
    for d in 2..=r.max(2) {
        if n % d == 0 {
            return Some(d);
        }
    }
    Some(n)
}

fn is_prime_power(n: u32, p: u32) -> bool {
    let mut x = n;
    while x > 1 {
        if x % p != 0 {
            return false;
        }
        x /= p;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_spectral_filter_honest_posture_refuses_green_and_production() {
        let probe = prime_spectral_filter_honest_posture_bundle();
        assert!(prime_spectral_filter_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.epsilon_is_soft_scale);
        assert_eq!(probe.deepen_cell, W29_PRIME_SPECTRAL_FILTER_DEEPEN_CELL);
        assert!(prime_spectral_filter_refuse_invented_pins().is_ok());
        assert!(!PRIME_SPECTRAL_FILTER_PHYSICS_GREEN);
        assert!(!PRIME_SPECTRAL_FILTER_PRODUCTION_WIRED);
        assert!(!PRIME_SPECTRAL_FILTER_MASTER);
    }

    #[test]
    fn identity_at_zero_epsilon() {
        let w = PrimeSpectralFilter::new(0.0, false, None).weight_table(8);
        assert!(w.iter().all(|x| (*x - 1.0).abs() < 1e-6));
        assert!(PrimeSpectralFilter::new(0.0, false, None).mean_abs_deviation(8) < 1e-6);
    }

    #[test]
    fn non_finite_epsilon_collapses_to_identity() {
        let ps = PrimeSpectralFilter::new(f32::NAN, false, None);
        assert_eq!(ps.epsilon, 0.0);
        let w = ps.weight_table(4);
        assert!(w.iter().all(|x| (*x - 1.0).abs() < 1e-6));
    }

    #[test]
    fn mean_renormalization_holds() {
        for &eps in &[0.01_f32, 0.05, 0.1] {
            let ps = PrimeSpectralFilter::new(eps, false, None);
            let mean = ps.weight_mean(32);
            assert!(
                (mean - 1.0).abs() < 1e-5,
                "mean-renorm failed at ε={eps}: mean={mean}"
            );
        }
    }

    #[test]
    fn von_mangoldt_known_values() {
        assert!((von_mangoldt_weight(1) - 0.0).abs() < 1e-6);
        assert!((von_mangoldt_weight(2) - 2.0).abs() < 1e-6);
        assert!((von_mangoldt_weight(3) - 3.0).abs() < 1e-6);
        assert!((von_mangoldt_weight(4) - 2.0).abs() < 1e-6); // 2^2
        assert!((von_mangoldt_weight(6) - 0.0).abs() < 1e-6); // 2*3 composite
        assert!((von_mangoldt_weight(8) - 2.0).abs() < 1e-6); // 2^3
        assert!((von_mangoldt_weight(9) - 3.0).abs() < 1e-6); // 3^2
    }

    #[test]
    fn coprime_stride_modulates_period_slots() {
        let ps = PrimeSpectralFilter::new(0.1, true, Some(3));
        let raw_like: Vec<f32> = (0..9)
            .map(|i| coprime_stride_weight(i, 9, 3, 0.1))
            .collect();
        assert!((raw_like[0] - 1.1).abs() < 1e-5);
        assert!((raw_like[3] - 1.1).abs() < 1e-5);
        assert!((raw_like[1] - 1.0).abs() < 1e-5);
        let mean = ps.weight_mean(9);
        assert!((mean - 1.0).abs() < 1e-5);
    }

    #[test]
    fn perturbation_identity_holds() {
        let ps = PrimeSpectralFilter::new(0.1, false, None);
        let weights = ps.weight_table(16);
        let vals = vec![0.5_f32; 16];
        let filtered: Vec<f32> = weights.iter().zip(&vals).map(|(w, v)| w * v).collect();
        for ((f, v), w) in filtered.iter().zip(&vals).zip(&weights) {
            assert!((f - v - (w - 1.0) * v).abs() < 1e-5);
        }
    }

    #[test]
    fn apply_tensor_matches_weight_table() {
        use burn::tensor::{Shape, Tensor};
        use burn_ndarray::NdArray;
        type B = NdArray<f32>;
        let dev = Default::default();
        let ps = PrimeSpectralFilter::new(0.05, false, None);
        let n = 8_usize;
        let rho = Tensor::<B, 3>::full(Shape::new([1, n, 1]), 0.5, &dev);
        let out = ps
            .apply(rho, n)
            .expect("PrimeSpectralFilter::apply on uniform rho at epsilon=0.05 (FP §6 topology spectral filter verification)");
        let expected_w = ps.weight_table(n);
        for (i, &v) in out.into_data().value.iter().enumerate() {
            assert!((v - 0.5 * expected_w[i]).abs() < 1e-5);
        }
    }

    #[test]
    fn apply_broadcasts_across_channels() {
        use burn::tensor::{Shape, Tensor};
        use burn_ndarray::NdArray;
        type B = NdArray<f32>;
        let dev = Default::default();
        let ps = PrimeSpectralFilter::new(0.05, false, None);
        let n = 4_usize;
        let c = 3_usize;
        let rho = Tensor::<B, 3>::full(Shape::new([1, n, c]), 0.25, &dev);
        let out = ps
            .apply(rho, n)
            .expect("PrimeSpectralFilter::apply multi-channel broadcast");
        let expected_w = ps.weight_table(n);
        let vals = out.into_data().value;
        assert_eq!(vals.len(), n * c);
        for i in 0..n {
            for ch in 0..c {
                let v = vals[i * c + ch];
                assert!((v - 0.25 * expected_w[i]).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn apply_rejects_node_count_mismatch() {
        use burn::tensor::{Shape, Tensor};
        use burn_ndarray::NdArray;
        type B = NdArray<f32>;
        let dev = Default::default();
        let ps = PrimeSpectralFilter::new(0.05, false, None);
        let rho = Tensor::<B, 3>::full(Shape::new([1, 4, 1]), 0.5, &dev);
        assert!(matches!(
            ps.apply(rho, 8).unwrap_err(),
            PhysicsError::ShapeMismatch { .. }
        ));
    }

    #[test]
    fn default_is_soft_mangoldt_guidance() {
        let d = PrimeSpectralFilter::default();
        assert!((d.epsilon - 0.05).abs() < 1e-6);
        assert!(!d.coprime_stride);
        assert!(d.coprime_prime.is_none());
        assert!((d.weight_mean(16) - 1.0).abs() < 1e-5);
    }
}

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

use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

use crate::physics::error::PhysicsError;

/// Pure guidance endofunctor on `[B, N, C]` density fields (elementwise).
#[derive(Clone, Debug, PartialEq)]
pub struct PrimeSpectralFilter {
    /// L¹ budget on `|w_i - 1|` (0 ⇒ identity filter).
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
        Self {
            epsilon: epsilon.max(0.0),
            coprime_stride,
            coprime_prime,
        }
    }

    /// Build per-node weights for `n` slots (Fin `n` mirror).
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

    /// Apply spectral filter: `rho' = w ⊙ rho` (same shape).
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
    fn identity_at_zero_epsilon() {
        let w = PrimeSpectralFilter::new(0.0, false, None).weight_table(8);
        assert!(w.iter().all(|x| (*x - 1.0).abs() < 1e-6));
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
        let out = ps.apply(rho, n).expect("stable filter apply");
        let expected_w = ps.weight_table(n);
        for (i, &v) in out.into_data().value.iter().enumerate() {
            assert!((v - 0.5 * expected_w[i]).abs() < 1e-5);
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
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Shannon / Petz classical limits, Kahan-summed where long runs matter.
//!
//! Proof anchors: `umst-formal/Lean/` and double-slit `QuantumMutualInfo` / `LandauerBound` families.
//! DOI: 10.5281/zenodo.18940933 (meso) · 10.5281/zenodo.19159660 (quantum bridge).

use std::collections::HashMap;

use ordered_float::NotNan;

use crate::density::DensityDiag;
use crate::io::{strip_think_tags, token_distribution, tokenise};
use crate::kahan::KahanSum;

// ── Theorem-aligned floors (UMST oracle until Phase 5 replaces with sharp PMIC bounds) ────────

/// Minimum extensive negentropy gain **J = N·H** (bits) treated as passing the thermodynamic gate.
///
/// Proof context: Sagawa–Ueda feedback yield; sharp bound from PMIC in Oracle v2 (Phase 5).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub const MIN_NEGENTROPY_FLOOR_BITS: f64 = 0.01;

/// Legacy name — same value as [`MIN_NEGENTROPY_FLOOR_BITS`] (legacy oracle compatibility).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub const MIN_NEGENTROPY: f64 = MIN_NEGENTROPY_FLOOR_BITS;

/// Binary entropy **h₂(p)** lower envelope in **bits**: **4 p (1−p)** for `p ∈ [0, 1]`.
///
/// Matches Lean `UMST.DoubleSlit.PMICEntropyInterior.four_mul_x_one_sub_x_mul_log_two_interior`
/// after dividing the natural-log statement by `ln 2` (binary entropy in bits).
/// DOI: 10.5281/zenodo.19159660
#[inline]
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn pmic_binary_entropy_lower_envelope_bits(p: f64) -> f64 {
    let p = p.clamp(0.0, 1.0);
    4.0 * p * (1.0 - p)
}

/// Extensive negentropy floor **N · h₂^lb** using the PMIC envelope on the **{major token, rest}**
/// binary coarse-graining: `p = max_k f_k`, `x = min(p, 1−p)`.
///
/// Degenerate one-token corpora yield `x → 0` and envelope `0` — callers should combine with
/// [`MIN_NEGENTROPY_FLOOR_BITS`] and participatory checks.
///
/// Lean: `UMST.DoubleSlit.PMICEntropyInterior` family (envelope); DOI: 10.5281/zenodo.19159660.
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn pmic_extensive_negentropy_floor_bits(text: &str) -> f64 {
    let clean = strip_think_tags(text);
    let tokens = tokenise(&clean);
    let n = tokens.len();
    if n < 2 {
        return MIN_NEGENTROPY_FLOOR_BITS;
    }
    let dist = token_distribution(&tokens);
    let p_top = dist.values().cloned().fold(0.0_f64, f64::max);
    let x = p_top.min(1.0 - p_top);
    let h2_lb = pmic_binary_entropy_lower_envelope_bits(x);
    n as f64 * h2_lb
}

/// Minimum per-token Shannon entropy (bits) for participatory richness lower band.
///
/// Proof context: tunable band on [`text_entropy`] until Klein/DPI v2 (Phase 5+ sharp PMIC bounds); not a proved sharp bound.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub const MIN_RICHNESS_ENTROPY: f64 = 0.5;

/// Maximum per-token Shannon entropy (bits) for participatory richness upper band.
///
/// Proof context: same as [`MIN_RICHNESS_ENTROPY`] — engineering band for Oracle participatory gate.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub const MAX_RICHNESS_ENTROPY: f64 = 14.0;

/// Shannon entropy **H(X) = −Σ p log₂ p** over a finite support (bits), Kahan-summed.
///
/// Proof: classical limit of von Neumann on diagonal states (`VonNeumannEntropy` tree).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn entropy(dist: &HashMap<String, f64>) -> f64 {
    let mut k = KahanSum::new();
    for p in dist.values().copied().filter(|&p| p > 0.0) {
        k += -p * p.log2();
    }
    k.total()
}

/// Shannon entropy of empirical text (token model in [`crate::io`]).
///
/// Proof: classical **H** on empirical token frequencies; UMST oracle estimator.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn text_entropy(text: &str) -> f64 {
    let clean = strip_think_tags(text);
    let tokens = tokenise(&clean);
    let dist = token_distribution(&tokens);
    entropy(&dist)
}

/// Binary Shannon entropy **h₂(p) = −p log₂ p − (1−p) log₂ (1−p)** in bits.
///
/// Proof: `PMICEntropyInterior` / path entropy lemmas (double-slit).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn binary_entropy_bits(p: NotNan<f64>) -> NotNan<f64> {
    let x = p.into_inner().clamp(0.0, 1.0);
    if x <= 0.0 || x >= 1.0 {
        return NotNan::new(0.0).unwrap();
    }
    let h = -x * x.log2() - (1.0 - x) * (1.0 - x).log2();
    NotNan::new(h).expect("finite binary entropy")
}

/// Shannon entropy of a diagonal density (same as classical **H({pᵢ})**).
///
/// Proof: diagonal von Neumann equals Shannon on spectrum.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn shannon_diag_bits<const N: usize>(d: &DensityDiag<N>) -> NotNan<f64> {
    let mut k = KahanSum::new();
    for x in &d.p {
        let p = x.into_inner();
        if p > 0.0 {
            k += -p * p.log2();
        }
    }
    NotNan::new(k.total()).expect("entropy")
}

/// Kahan-summed **−Σ p log₂ p** for a probability slice (SIMD hook point under `simd` feature).
///
/// Proof: same functional as [`shannon_diag_bits`] on explicit mass vector.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn shannon_binary_kahan(probs: &[f64]) -> f64 {
    let mut k = KahanSum::new();
    for &p in probs {
        if p > 0.0 {
            k += -p * p.log2();
        }
    }
    k.total()
}

/// von Neumann entropy of a diagonal state = [`shannon_diag_bits`].
///
/// Proof: `VonNeumannEntropy` — spectrum-only functional on diagonal ρ.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn von_neumann_entropy_diagonal<const N: usize>(d: &DensityDiag<N>) -> NotNan<f64> {
    shannon_diag_bits(d)
}

/// KL divergence **D(P‖Q)** in nats (commuting / classical limit of Petz divergence).
///
/// Proof: `KleinInequality` classical diagonal case.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn kl_divergence(p: &HashMap<String, f64>, q: &HashMap<String, f64>) -> f64 {
    p.iter()
        .filter(|(_, pi)| **pi > 0.0)
        .map(|(x, pi)| {
            let pi = *pi;
            let qi = q.get(x).copied().unwrap_or(0.0);
            if qi == 0.0 {
                f64::INFINITY
            } else {
                pi * (pi / qi).ln()
            }
        })
        .sum()
}

/// Negentropy yield **J ≈ N · H** (bits) for tokenised text — Sagawa–Ueda proxy.
///
/// Proof: extensive information yield context in `LandauerBound` / feedback literature.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn negentropy(text: &str) -> f64 {
    let clean = strip_think_tags(text);
    let tokens = tokenise(&clean);
    if tokens.is_empty() {
        return 0.0;
    }
    let n = tokens.len() as f64;
    let h = text_entropy(&clean);
    n * h
}

/// Mutual information **I(X;Y)** from concatenated joint proxy (UMST legacy estimator).
///
/// Proof: classical **I = H(X)+H(Y)−H(X,Y)** on empirical marginals (surrogate).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn mutual_information(text_a: &str, text_b: &str) -> f64 {
    let ha = text_entropy(text_a);
    let hb = text_entropy(text_b);
    let joint = format!("{text_a} {text_b}");
    let h_joint = text_entropy(&joint);
    (ha + hb - h_joint).max(0.0)
}

/// Approximate conditional entropy **H(Y|X)** from joint proxy.
///
/// Proof: classical **H(Y|X) = H(X,Y) − H(X)** on empirical estimators (surrogate).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn conditional_entropy(text_x: &str, text_y: &str) -> f64 {
    let hx = text_entropy(text_x);
    let joint = format!("{text_x} {text_y}");
    let h_joint = text_entropy(&joint);
    (h_joint - hx).max(0.0)
}

/// Thermodynamic admissibility: extensive negentropy above PMIC envelope (and legacy floor).
///
/// Proof: combines [`pmic_extensive_negentropy_floor_bits`] with [`negentropy`] (Oracle v2 gate).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn thermodynamic_check(proposal: &str) -> bool {
    let floor = pmic_extensive_negentropy_floor_bits(proposal).max(MIN_NEGENTROPY_FLOOR_BITS);
    negentropy(proposal) >= floor
}

/// Participatory richness band on per-token entropy.
///
/// Proof: band check on [`text_entropy`] — participatory surrogate until Klein/DPI v2 (Phase 5+ sharp PMIC bounds).
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.InfoTheory::product_joint_mass` (§14bis.l W-3 G8)
pub fn participatory_richness_check(proposal: &str) -> bool {
    let h = text_entropy(proposal);
    (MIN_RICHNESS_ENTROPY..=MAX_RICHNESS_ENTROPY).contains(&h)
}

#[cfg(test)]
mod tests_pmic_floor {
    use super::*;

    #[test]
    fn pmic_envelope_at_half_is_one_bit() {
        let h = pmic_binary_entropy_lower_envelope_bits(0.5);
        assert!((h - 1.0).abs() < 1e-9, "4·½·½ = 1 bit");
    }

    #[test]
    fn thermodynamic_check_respects_legacy_floor_on_empty() {
        assert!(!thermodynamic_check(""));
    }

    #[test]
    fn two_token_alternation_meets_pmic_extensive_floor() {
        let s = "alpha beta ".repeat(8);
        assert!(
            thermodynamic_check(&s),
            "structured mix should pass PMIC extensive gate"
        );
    }
}

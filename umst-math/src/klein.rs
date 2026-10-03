// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Klein / Petz spectral relative entropy **D(ρ‖σ) ≥ 0** (diagonal classical limit).

use crate::density::DensityDiag;

/// Spectral (diagonal) relative entropy **Σ pᵢ log(pᵢ/qᵢ)** in nats; `+∞` if support mismatch.
///
/// Proof: `KleinInequality` / `spectralRelativeEntropynonneg`.
/// DOI: 10.5281/zenodo.19159660
pub fn spectral_relative_entropy_diag<const N: usize>(
    p: &DensityDiag<N>,
    q: &DensityDiag<N>,
) -> f64 {
    let mut s = 0.0_f64;
    for i in 0..N {
        let pi = p.p[i].into_inner();
        let qi = q.p[i].into_inner();
        if pi > 0.0 && qi > 0.0 {
            s += pi * (pi / qi).ln();
        } else if pi > 0.0 && qi == 0.0 {
            return f64::INFINITY;
        }
    }
    s
}

/// Classical non-negativity check on common support.
///
/// Proof: same as [`spectral_relative_entropy_diag`] / Klein non-negativity.
/// DOI: 10.5281/zenodo.19159660
pub fn klein_spectral_relative_entropy_nonneg<const N: usize>(
    p: &DensityDiag<N>,
    q: &DensityDiag<N>,
) -> bool {
    spectral_relative_entropy_diag(p, q) >= -1e-12
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures;

    #[test]
    fn klein_nonneg_identical_distributions() {
        let p = fixtures::qubit_plus();
        assert!(klein_spectral_relative_entropy_nonneg(&p, &p));
    }
}

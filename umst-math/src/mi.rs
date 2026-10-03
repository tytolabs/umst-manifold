// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Quantum mutual information — product-state diagonal specialization.

use ordered_float::NotNan;

use crate::density::DensityDiag;
use crate::info_entropy::shannon_diag_bits;

/// **I(A:B) = S(A)+S(B)−S(AB)** for **product** diagonal states (independent marginals).
///
/// Proof: `QuantumMutualInfo` definitions.
/// DOI: 10.5281/zenodo.19159660
pub fn quantum_mi_product_diagonal<const NA: usize, const NB: usize, const NAB: usize>(
    joint: &DensityDiag<NAB>,
    marginal_a: &DensityDiag<NA>,
    marginal_b: &DensityDiag<NB>,
) -> Option<NotNan<f64>> {
    if NA * NB != NAB {
        return None;
    }
    let sa = shannon_diag_bits(marginal_a).into_inner();
    let sb = shannon_diag_bits(marginal_b).into_inner();
    let sab = shannon_diag_bits(joint).into_inner();
    let mi = (sa + sb - sab).max(0.0);
    Some(NotNan::new(mi).expect("MI"))
}

//! Full von Neumann entropy — diagonal specialization reuses Shannon on spectrum.

use ordered_float::NotNan;

use crate::density::DensityDiag;
use crate::info_entropy::shannon_diag_bits;

/// von Neumann entropy **S(ρ) = −Tr(ρ log ρ)** for diagonal ρ (bits).
///
/// Proof: `VonNeumannEntropy` module.
/// DOI: 10.5281/zenodo.19159660
pub fn von_neumann_entropy<const N: usize>(d: &DensityDiag<N>) -> NotNan<f64> {
    shannon_diag_bits(d)
}

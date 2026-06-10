//! PMIC — residual coherence capacity (RCC) from path / branch distributions.

use ordered_float::NotNan;

use crate::density::DensityDiag;
use crate::info_entropy::shannon_diag_bits;

/// **RCC = 1 − H(P)/log₂ N** for `N` balanced branches (uniform reference).
///
/// Proof: `LandauerBound` / `PMICVisibility` — path entropy collapse lemmas.
/// DOI: 10.5281/zenodo.19159660
pub fn residual_coherence_capacity<const N: usize>(dist: &DensityDiag<N>) -> NotNan<f64> {
    assert!(N > 1, "RCC requires at least two branches");
    let h = shannon_diag_bits(dist).into_inner();
    let max_h = (N as f64).log2();
    let rcc = 1.0 - (h / max_h).min(1.0);
    NotNan::new(rcc.clamp(0.0, 1.0)).expect("RCC in 0..1")
}

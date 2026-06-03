//! Data-processing inequality (DPI) — classical diagonal surrogate.

use crate::density::DensityDiag;
use crate::info_entropy::shannon_diag_bits;

/// Classical DPI surrogate: processing with a stochastic map cannot increase Shannon entropy
/// of the **pushforward** on the observed diagonal (equality only for reversible layers).
///
/// Proof: `vonNeumannEntropy_nondecreasing_unital_CPTP_n` (quantum); classical embedding.
/// DOI: 10.5281/zenodo.19159660
/// THEOREM-BOUND: `UMST.Formal.MedianConvergence::median_convergence_sample_size` (§14bis.l W-3 G8)
pub fn shannon_nondecreasing_under_marginalization<const N: usize, const M: usize>(
    before: &DensityDiag<N>,
    after: &DensityDiag<M>,
) -> bool {
    shannon_diag_bits(before).into_inner() + 1e-12 >= shannon_diag_bits(after).into_inner()
}

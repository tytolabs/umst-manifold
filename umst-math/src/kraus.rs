//! Lüders / which-path Kraus updates (diagonal classical layer).

use crate::density::DensityDiag;

/// Project diagonal state onto branch `k` (Lüders-style classical collapse).
///
/// Proof: `WhichPathMeasurementUpdate` / `measurementUpdateWhichPath`.
/// DOI: 10.5281/zenodo.19159660
pub fn lueders_branch<const N: usize>(d: &DensityDiag<N>, branch: usize) -> Option<DensityDiag<N>> {
    if branch >= N {
        return None;
    }
    let pk = d.p[branch].into_inner();
    if pk <= 0.0 {
        return None;
    }
    let mut raw = [0.0_f64; N];
    raw[branch] = 1.0;
    DensityDiag::try_from_diag(raw).ok()
}

/// Compose identity channel (no-op) on diagonal factors.
///
/// Proof: unital CPTP identity / `identityChannel` classical embedding.
/// DOI: 10.5281/zenodo.19159660
pub fn identity_channel<const N: usize>(d: &DensityDiag<N>) -> DensityDiag<N> {
    d.clone()
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Tensor products and partial traces on **diagonal** factors.

use ordered_float::NotNan;

use crate::density::DensityDiag;

/// Tensor product of diagonal factors **ρ ⊗ σ** (Kronecker of probability vectors).
///
/// Proof: `tensorDensity`, `TensorPartialTrace` lemmas.
/// DOI: 10.5281/zenodo.19159660
pub fn tensor_diagonal<const NA: usize, const NB: usize, const NAB: usize>(
    a: &DensityDiag<NA>,
    b: &DensityDiag<NB>,
) -> Option<DensityDiag<NAB>> {
    if NA * NB != NAB {
        return None;
    }
    let mut raw = vec![0.0_f64; NAB];
    let mut i = 0;
    for x in &a.p {
        for y in &b.p {
            raw[i] = x.into_inner() * y.into_inner();
            i += 1;
        }
    }
    let s: f64 = raw.iter().sum();
    if s <= 0.0 || !s.is_finite() {
        return None;
    }
    for x in &mut raw {
        *x /= s;
    }
    let arr: [f64; NAB] = raw.try_into().ok()?;
    DensityDiag::try_from_diag(arr).ok()
}

/// Partial trace over **B** (second factor), **N = NA·NB**, traced dimension **NB**.
///
/// Proof: `partial_trace` PSD / trace lemmas in `TensorPartialTrace`.
/// DOI: 10.5281/zenodo.19159660
pub fn partial_trace_second<const NA: usize, const NB: usize, const NAB: usize>(
    ab: &DensityDiag<NAB>,
) -> Option<DensityDiag<NA>> {
    if NA * NB != NAB {
        return None;
    }
    let mut marg = [0.0_f64; NA];
    for (ia, slot) in marg.iter_mut().enumerate().take(NA) {
        let mut s = 0.0;
        for ib in 0..NB {
            let idx = ia * NB + ib;
            s += ab.p[idx].into_inner();
        }
        *slot = s;
    }
    DensityDiag::try_from_diag(marg).ok()
}

/// PSD check for diagonal (nonnegative entries).
///
/// Proof: spectrum nonnegative ⇔ diagonal PSD in the computational basis.
/// DOI: 10.5281/zenodo.19159660
pub fn is_psd_diagonal<const N: usize>(d: &DensityDiag<N>) -> bool {
    d.p.iter().all(|x| x.into_inner() >= 0.0)
}

/// Trace-normalised check via scalar tolerance.
///
/// Proof: `DensityDiag::trace` equals classical probability mass (Lean `trace_one` witnesses).
/// DOI: 10.5281/zenodo.19159660
pub fn is_trace_one<const N: usize>(d: &DensityDiag<N>, eps: f64) -> bool {
    let t: NotNan<f64> = d.trace();
    (t.into_inner() - 1.0).abs() <= eps
}

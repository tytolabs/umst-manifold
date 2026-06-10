//! Golden qubit examples aligned with `ExamplesQubit` in Lean.
//!
//! Proof anchors: see `theorem_registry::THEOREM_REGISTRY` (quantum bridge DOI **10.5281/zenodo.19159660**).

use ordered_float::NotNan;

use crate::density::DensityDiag;

/// \|0⟩ diagonal (computational zero).
///
/// Proof: `ExamplesQubit` / computational basis pointer in double-slit tree.
/// DOI: 10.5281/zenodo.19159660
pub fn qubit_zero() -> DensityDiag<2> {
    DensityDiag::try_from_diag([1.0, 0.0]).expect("valid")
}

/// \|1⟩ diagonal (computational one).
///
/// Proof: same `ExamplesQubit` family as [`qubit_zero`] / [`qubit_plus`].
/// DOI: 10.5281/zenodo.19159660
pub fn qubit_one() -> DensityDiag<2> {
    DensityDiag::try_from_diag([0.0, 1.0]).expect("valid")
}

/// \|+⟩ = (|0⟩+|1⟩)/√2 diagonal in computational basis.
///
/// Proof: equal superposition mass vector (Hadamard branch in `ExamplesQubit` family).
/// DOI: 10.5281/zenodo.19159660
pub fn qubit_plus() -> DensityDiag<2> {
    let p = 0.5_f64;
    DensityDiag::try_from_diag([p, p]).expect("valid")
}

/// Convenience: extract **p(|0⟩)** for qubit diagnostics.
///
/// Proof: classical readout probability on computational **Z** basis (diagonal entry).
/// DOI: 10.5281/zenodo.19159660
pub fn p0(d: &DensityDiag<2>) -> NotNan<f64> {
    d.p[0]
}

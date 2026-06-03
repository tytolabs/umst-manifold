//! Diagonal density matrices `ρ = diag(p)` with **PSD** + **trace = 1**.

use ordered_float::NotNan;

use crate::kahan::KahanSum;

/// Diagonal density operator on a finite `N`-dimensional Hilbert space.
///
/// **PSD** is witnessed by `p[i] ≥ 0`; **trace-one** by `Σᵢ pᵢ = 1`.
/// Proof: `tensorDensity` / diagonal PSD constructors (double-slit formal tree).
/// DOI: 10.5281/zenodo.19159660
#[derive(Clone, Debug, PartialEq)]
pub struct DensityDiag<const N: usize> {
    /// Diagonal entries ρᵢᵢ (probability of basis state `i`).
    pub p: [NotNan<f64>; N],
}

impl<const N: usize> DensityDiag<N> {
    /// Trace as Kahan sum (drift-free for large `N`).
    ///
    /// Proof: `umst-formal-double-slit/Lean/TensorPartialTrace.lean` — partial trace preserves trace.
    /// DOI: 10.5281/zenodo.19159660
    pub fn trace(&self) -> NotNan<f64> {
        let mut k = KahanSum::new();
        for x in &self.p {
            k += x.into_inner();
        }
        NotNan::new(k.total()).expect("trace of nonnegative finite values")
    }

    /// Construct from raw probabilities; returns `Err` if invalid or non-finite.
    ///
    /// Proof: same module family as `tensorDensity` in Lean double-slit tree.
    /// DOI: 10.5281/zenodo.19159660
    pub fn try_from_diag(raw: [f64; N]) -> Result<Self, &'static str> {
        if N == 0 {
            return Err("N must be positive");
        }
        let mut k = KahanSum::new();
        for &x in raw.iter().take(N) {
            if x < 0.0 || !x.is_finite() {
                return Err("probabilities must be finite and nonnegative");
            }
            k += x;
        }
        if (k.total() - 1.0).abs() > 1e-9 {
            return Err("probabilities must sum to 1");
        }
        let vec: Vec<NotNan<f64>> = (0..N)
            .map(|i| NotNan::new(raw[i]).map_err(|_| "NaN in diagonal"))
            .collect::<Result<_, _>>()?;
        let p: [NotNan<f64>; N] = vec.try_into().map_err(|_| "internal length")?;
        Ok(DensityDiag { p })
    }

    /// Maximally mixed state `I/N`.
    ///
    /// Proof: `vonNeumannDiagonal_n_le_log_n` context (general dimension).
    /// DOI: 10.5281/zenodo.19159660
    pub fn maximally_mixed() -> Self {
        assert!(N > 0, "N>0");
        let v = 1.0 / (N as f64);
        let nn = NotNan::new(v).expect("1/N");
        DensityDiag {
            p: std::array::from_fn(|_| nn),
        }
    }
}

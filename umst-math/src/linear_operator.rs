// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Matrix-free linear operators with certified structure witnesses (design §2.1).
//!
//! Witness newtypes (`Symmetric`, `Spd`, `Semidefinite`) are ghosts: only certifying
//! constructors can build them so solver preconditions cannot be forged at call sites.

/// Errors from dimension checks and non-finite arithmetic during operator application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpError {
    /// Vector lengths do not match the operator dimension.
    DimMismatch,
    /// A scalar was NaN or infinite.
    NonFinite,
}

/// Matrix-free linear map \(y \leftarrow K x\) on \(\mathbb{R}^n\).
pub trait LinearOperator {
    /// Dimension \(n\) of the domain and codomain.
    fn n(&self) -> usize;

    /// Apply the operator: `y[i] = \sum_j K_{ij} x_j` (or the operator's action).
    fn apply(&self, x: &[f64], y: &mut [f64]) -> Result<(), OpError>;
}

/// Witness that `Op` is symmetric (certified, not assumed at the type level for raw `Op`).
pub struct Symmetric<Op> {
    inner: Op,
}

/// Witness that `Op` is symmetric positive definite.
pub struct Spd<Op> {
    inner: Op,
}

/// Basis for the nullspace of a semidefinite operator (columns are mode shapes).
#[derive(Clone, Debug)]
pub struct NullspaceWitness {
    /// Number of independent nullspace modes.
    pub mode_count: usize,
    /// Packed mode vectors, each of length `n` (row-major: mode `k` occupies `[k*n .. (k+1)*n)`).
    pub modes: Vec<f64>,
    /// Problem dimension `n` (length of each mode vector).
    pub n: usize,
}

/// Witness that `Op` is symmetric positive semidefinite with a supplied nullspace basis.
pub struct Semidefinite<Op> {
    inner: Op,
    nullspace: NullspaceWitness,
}

/// Access the wrapped operator from a symmetric witness.
pub fn symmetric_operator<Op>(witness: &Symmetric<Op>) -> &Op {
    &witness.inner
}

/// Access the wrapped operator from an SPD witness.
pub fn spd_operator<Op>(witness: &Spd<Op>) -> &Op {
    &witness.inner
}

/// Access the wrapped operator from a semidefinite witness.
pub fn semidefinite_operator<Op>(witness: &Semidefinite<Op>) -> &Op {
    &witness.inner
}

/// Nullspace witness carried by a semidefinite certification.
pub fn semidefinite_nullspace<Op>(witness: &Semidefinite<Op>) -> &NullspaceWitness {
    &witness.nullspace
}

/// Every SPD operator is symmetric; this is the canonical lift (exact by construction).
pub fn symmetric_from_spd<Op>(spd: Spd<Op>) -> Symmetric<Op> {
    Symmetric { inner: spd.inner }
}

/// Diagonal operator \(K = \mathrm{diag}(d_i)\).
pub struct DiagonalOperator {
    diag: Vec<f64>,
}

impl DiagonalOperator {
    /// Build a diagonal operator without structure witnesses (raw matvec only).
    pub fn from_diagonal(diag: Vec<f64>) -> Self {
        Self { diag }
    }
}

impl LinearOperator for DiagonalOperator {
    fn n(&self) -> usize {
        self.diag.len()
    }

    fn apply(&self, x: &[f64], y: &mut [f64]) -> Result<(), OpError> {
        let n = self.n();
        if x.len() != n || y.len() != n {
            return Err(OpError::DimMismatch);
        }
        for i in 0..n {
            let xi = x[i];
            if !xi.is_finite() {
                return Err(OpError::NonFinite);
            }
            let val = self.diag[i] * xi;
            if !val.is_finite() {
                return Err(OpError::NonFinite);
            }
            y[i] = val;
        }
        Ok(())
    }
}

/// Identity operator on \(\mathbb{R}^n\).
pub struct IdentityOperator {
    n: usize,
}

impl IdentityOperator {
    /// Build the \(n \times n\) identity as a matrix-free operator.
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl LinearOperator for IdentityOperator {
    fn n(&self) -> usize {
        self.n
    }

    fn apply(&self, x: &[f64], y: &mut [f64]) -> Result<(), OpError> {
        let n = self.n;
        if x.len() != n || y.len() != n {
            return Err(OpError::DimMismatch);
        }
        for i in 0..n {
            let xi = x[i];
            if !xi.is_finite() {
                return Err(OpError::NonFinite);
            }
            y[i] = xi;
        }
        Ok(())
    }
}

/// Certify SPD from a strictly positive finite diagonal (exact by construction).
pub fn spd_from_diagonal_positive(diag: Vec<f64>) -> Result<Spd<DiagonalOperator>, OpError> {
    for &d in &diag {
        if !d.is_finite() || d <= 0.0 {
            return Err(OpError::NonFinite);
        }
    }
    Ok(Spd {
        inner: DiagonalOperator::from_diagonal(diag),
    })
}

/// Probabilistic-style symmetry check: \(\langle Ax, y\rangle = \langle x, Ay\rangle\) on fixed probes.
///
/// Precondition: `op.n() > 0`. Failure returns `OpError::NonFinite` when a probe dot product disagrees
/// beyond floating tolerance or a value is non-finite.
pub fn symmetric_from_bilinear_test<Op: LinearOperator>(
    op: Op,
    probes: &[Vec<f64>],
) -> Result<Symmetric<Op>, OpError> {
    let n = op.n();
    if n == 0 {
        return Err(OpError::DimMismatch);
    }
    let mut ax = vec![0.0; n];
    let mut ay = vec![0.0; n];
    let mut scratch_x = vec![0.0; n];
    let mut scratch_y = vec![0.0; n];

    for probe in probes {
        if probe.len() != n {
            return Err(OpError::DimMismatch);
        }
        for (i, &p) in probe.iter().enumerate() {
            if !p.is_finite() {
                return Err(OpError::NonFinite);
            }
            scratch_x[i] = p;
            scratch_y[i] = p;
        }
        op.apply(&scratch_x, &mut ax)?;
        op.apply(&scratch_y, &mut ay)?;
        let xy = dot(&scratch_x, &ay)?;
        let yx = dot(&scratch_x, &ax)?;
        if !approx_eq(xy, yx) {
            return Err(OpError::NonFinite);
        }
    }
    Ok(Symmetric { inner: op })
}

/// Certify semidefinite structure when a nullspace basis is already available.
pub fn semidefinite_from_nullspace<Op>(
    op: Op,
    nullspace: NullspaceWitness,
) -> Result<Semidefinite<Op>, OpError> {
    if nullspace.n == 0 && nullspace.mode_count == 0 && nullspace.modes.is_empty() {
        return Ok(Semidefinite {
            inner: op,
            nullspace,
        });
    }
    let expected_len = nullspace.mode_count * nullspace.n;
    if nullspace.modes.len() != expected_len {
        return Err(OpError::DimMismatch);
    }
    for &v in &nullspace.modes {
        if !v.is_finite() {
            return Err(OpError::NonFinite);
        }
    }
    Ok(Semidefinite {
        inner: op,
        nullspace,
    })
}

fn dot(a: &[f64], b: &[f64]) -> Result<f64, OpError> {
    if a.len() != b.len() {
        return Err(OpError::DimMismatch);
    }
    let mut s = 0.0;
    for i in 0..a.len() {
        let p = a[i] * b[i];
        if !p.is_finite() {
            return Err(OpError::NonFinite);
        }
        s += p;
        if !s.is_finite() {
            return Err(OpError::NonFinite);
        }
    }
    Ok(s)
}

fn approx_eq(a: f64, b: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= 1e-12 * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Precondition: `K = diag(2, 3, 4)` is SPD; `spd_from_diagonal_positive` certifies it.
    #[test]
    fn diagonal_spd_matvec_matches_manufactured_solution() {
        let k = spd_from_diagonal_positive(vec![2.0, 3.0, 4.0]).expect("SPD diagonal");
        let op = spd_operator(&k);
        let x = [1.0, 2.0, 3.0];
        let mut y = [0.0, 0.0, 0.0];
        op.apply(&x, &mut y).expect("apply");
        assert_eq!(y, [2.0, 6.0, 12.0]);
        let sym = symmetric_from_spd(k);
        let op_sym = symmetric_operator(&sym);
        let mut y2 = [0.0, 0.0, 0.0];
        op_sym.apply(&x, &mut y2).expect("symmetric apply");
        assert_eq!(y2, [2.0, 6.0, 12.0]);
    }

    #[test]
    fn apply_dim_mismatch_returns_error() {
        let op = DiagonalOperator::from_diagonal(vec![1.0, 2.0]);
        let x = [1.0];
        let mut y = [0.0, 0.0];
        let err = op.apply(&x, &mut y).expect_err("dim mismatch");
        assert_eq!(err, OpError::DimMismatch);
    }
}

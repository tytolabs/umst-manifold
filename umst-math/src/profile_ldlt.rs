// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Symmetric profile (skyline) matrices and their `L D Lᵀ` factorisation.
//!
//! A [`SymmetricProfile`] stores, for each row `i`, the lower-triangle entries from its first non-zero column
//! `first[i]` to the diagonal. Assembly from element cliques fixes the profile once, and every matrix built from
//! the same cliques (stiffness, mass, a shifted pencil `K − σM`) shares it.
//!
//! [`ldlt`] factors without pivoting. Fill-in stays inside the profile, so the cost is `Σᵢ (i − first[i])²` and
//! the memory is the profile itself. The factor carries two certificates:
//!
//! - **definiteness**: every pivot positive ⇔ the matrix is symmetric positive definite (Sylvester's criterion on
//!   the leading minors, since `det Aₖ = Π_{i<k} dᵢ`), so [`LdltFactor::spd`] is a witness and not a test;
//! - **inertia**: by Sylvester's law of inertia, `A = L D Lᵀ` has as many negative eigenvalues as `D` has
//!   negative entries. For a pencil `K − τM` with `M` positive definite this counts the generalised eigenvalues
//!   below `τ` (the Sturm-sequence count of structural dynamics).
//!
//! A pivot whose magnitude falls below the rounding floor of its row refuses ([`ProfileRefuse::NearZeroPivot`]):
//! the inertia it would report cannot be trusted, and the caller moves the shift.

use crate::linear_operator::{LinearOperator, OpError};

/// Why a profile matrix or its factor was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileRefuse {
    /// The matrix has no rows.
    Empty,
    /// A clique names a degree of freedom at or beyond the dimension.
    IndexOutOfRange {
        /// The offending index.
        index: usize,
    },
    /// A clique's block is not `k × k` for its `k` indices, or a vector length disagrees.
    DimMismatch,
    /// An entry, a right-hand side or a result is NaN or infinite.
    NonFinite,
    /// Two matrices combined do not share one profile.
    PatternMismatch,
    /// A pivot is below the rounding floor of its row.
    NearZeroPivot {
        /// Row of the pivot.
        row: usize,
    },
    /// A pivot is not positive where a positive definite factor was required.
    NotPositiveDefinite {
        /// Row of the first non-positive pivot.
        row: usize,
    },
}

/// One element's contribution: global indices and a dense symmetric block in row-major order.
#[derive(Clone, Copy, Debug)]
pub struct Clique<'a> {
    /// Global degree-of-freedom indices, one per block row.
    pub dofs: &'a [usize],
    /// Row-major `k × k` block, `k = dofs.len()`. Only the lower triangle is read.
    pub block: &'a [f64],
}

/// A symmetric matrix in row-profile storage.
#[derive(Clone, Debug, PartialEq)]
pub struct SymmetricProfile {
    first: Vec<usize>,
    start: Vec<usize>,
    values: Vec<f64>,
}

/// Profile of `n` rows fixed by a set of cliques: row `i` begins at the smallest index sharing a clique with `i`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfilePattern {
    first: Vec<usize>,
    start: Vec<usize>,
    len: usize,
}

impl ProfilePattern {
    /// The profile spanned by `cliques` over `n` rows. A row in no clique keeps only its diagonal.
    ///
    /// # Errors
    /// [`ProfileRefuse::Empty`] for `n = 0`; [`ProfileRefuse::IndexOutOfRange`] for an index `≥ n`.
    pub fn from_cliques<'a, I>(n: usize, cliques: I) -> Result<Self, ProfileRefuse>
    where
        I: IntoIterator<Item = &'a [usize]>,
    {
        if n == 0 {
            return Err(ProfileRefuse::Empty);
        }
        let first = cliques.into_iter().try_fold((0..n).collect::<Vec<usize>>(), |mut first, dofs| {
            if let Some(&index) = dofs.iter().find(|&&d| d >= n) {
                return Err(ProfileRefuse::IndexOutOfRange { index });
            }
            if let Some(lowest) = dofs.iter().copied().min() {
                dofs.iter().for_each(|&d| first[d] = first[d].min(lowest));
            }
            Ok(first)
        })?;
        let start: Vec<usize> = first
            .iter()
            .enumerate()
            .scan(0_usize, |offset, (i, &f)| {
                let here = *offset;
                *offset += i - f + 1;
                Some(here)
            })
            .collect();
        let len = start[n - 1] + (n - first[n - 1]);
        Ok(Self { first, start, len })
    }

    /// Number of rows.
    #[must_use]
    pub fn n(&self) -> usize {
        self.first.len()
    }

    /// Stored entries (lower triangle within the profile, diagonal included).
    #[must_use]
    pub fn stored(&self) -> usize {
        self.len
    }

    /// Sum the cliques into a matrix on this profile.
    ///
    /// # Errors
    /// [`ProfileRefuse::DimMismatch`] for a block of the wrong size, [`ProfileRefuse::IndexOutOfRange`] for an
    /// index outside the pattern's rows or its profile, and [`ProfileRefuse::NonFinite`] for a NaN or infinite
    /// entry.
    pub fn assemble<'a, I>(&self, cliques: I) -> Result<SymmetricProfile, ProfileRefuse>
    where
        I: IntoIterator<Item = Clique<'a>>,
    {
        let values = cliques.into_iter().try_fold(vec![0.0_f64; self.len], |mut values, clique| {
            let k = clique.dofs.len();
            if clique.block.len() != k * k {
                return Err(ProfileRefuse::DimMismatch);
            }
            for (a, &ra) in clique.dofs.iter().enumerate() {
                for (b, &rb) in clique.dofs.iter().enumerate().take(a + 1) {
                    let v = clique.block[a * k + b];
                    if !v.is_finite() {
                        return Err(ProfileRefuse::NonFinite);
                    }
                    let (row, col) = if ra >= rb { (ra, rb) } else { (rb, ra) };
                    let slot = self.slot(row, col).ok_or(ProfileRefuse::IndexOutOfRange { index: row })?;
                    // A diagonal pair (a, a) and an off-diagonal pair landing on the same global diagonal both add.
                    values[slot] += if a != b && ra == rb { v + v } else { v };
                }
            }
            Ok(values)
        })?;
        Ok(SymmetricProfile { first: self.first.clone(), start: self.start.clone(), values })
    }

    fn slot(&self, row: usize, col: usize) -> Option<usize> {
        (row < self.first.len() && col >= self.first[row] && col <= row)
            .then(|| self.start[row] + (col - self.first[row]))
    }
}

impl SymmetricProfile {
    /// Number of rows.
    #[must_use]
    pub fn n(&self) -> usize {
        self.first.len()
    }

    /// Entry `(i, j)`; zero outside the profile.
    #[must_use]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        let (row, col) = if i >= j { (i, j) } else { (j, i) };
        if row >= self.n() || col < self.first[row] {
            return 0.0;
        }
        self.values[self.start[row] + (col - self.first[row])]
    }

    /// The diagonal.
    #[must_use]
    pub fn diagonal(&self) -> Vec<f64> {
        (0..self.n()).map(|i| self.values[self.start[i] + (i - self.first[i])]).collect()
    }

    /// `a·self + b·other` on the shared profile (for example `K − σM` with `a = 1`, `b = −σ`).
    ///
    /// # Errors
    /// [`ProfileRefuse::PatternMismatch`] when the profiles differ; [`ProfileRefuse::NonFinite`] when a scale or a
    /// result is not finite.
    pub fn combine(&self, a: f64, other: &Self, b: f64) -> Result<Self, ProfileRefuse> {
        if self.first != other.first {
            return Err(ProfileRefuse::PatternMismatch);
        }
        if !(a.is_finite() && b.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        let values: Vec<f64> = self.values.iter().zip(&other.values).map(|(x, y)| a * x + b * y).collect();
        if values.iter().any(|v| !v.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        Ok(Self { first: self.first.clone(), start: self.start.clone(), values })
    }

    /// `|x|ᵀ |A| |x|`, the magnitude that bounds the rounding error of a computed `xᵀ A x`.
    #[must_use]
    pub fn abs_quadratic(&self, x: &[f64]) -> f64 {
        (0..self.n().min(x.len()))
            .map(|i| {
                let f = self.first[i];
                let row = &self.values[self.start[i]..=self.start[i] + (i - f)];
                let off: f64 = row[..i - f].iter().zip(&x[f..i]).map(|(a, xj)| a.abs() * xj.abs()).sum();
                let diag = row[i - f].abs() * x[i].abs();
                x[i].abs() * (diag + off + off)
            })
            .sum()
    }

    /// The widest stored row (profile bandwidth plus one).
    #[must_use]
    pub fn max_row_width(&self) -> usize {
        self.first.iter().enumerate().map(|(i, f)| i - f + 1).max().unwrap_or(0)
    }

    /// `y = A x` as a new vector.
    ///
    /// # Errors
    /// [`ProfileRefuse::DimMismatch`] for a wrong length; [`ProfileRefuse::NonFinite`] for a NaN or infinite
    /// input or product.
    pub fn mul(&self, x: &[f64]) -> Result<Vec<f64>, ProfileRefuse> {
        let n = self.n();
        if x.len() != n {
            return Err(ProfileRefuse::DimMismatch);
        }
        if x.iter().any(|v| !v.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        let y = (0..n).fold(vec![0.0_f64; n], |mut y, i| {
            let f = self.first[i];
            let row = &self.values[self.start[i]..=self.start[i] + (i - f)];
            // Lower part of row i contributes to y[i]; its transpose contributes to y[j] for j < i.
            y[i] += row.iter().zip(&x[f..=i]).map(|(a, xj)| a * xj).sum::<f64>();
            row[..i - f].iter().enumerate().for_each(|(k, a)| y[f + k] += a * x[i]);
            y
        });
        if y.iter().any(|v| !v.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        Ok(y)
    }
}

impl LinearOperator for SymmetricProfile {
    fn n(&self) -> usize {
        self.first.len()
    }

    fn apply(&self, x: &[f64], y: &mut [f64]) -> Result<(), OpError> {
        if y.len() != self.first.len() {
            return Err(OpError::DimMismatch);
        }
        let product = self.mul(x).map_err(|e| match e {
            ProfileRefuse::NonFinite => OpError::NonFinite,
            _ => OpError::DimMismatch,
        })?;
        y.copy_from_slice(&product);
        Ok(())
    }
}

/// Counts of negative and positive pivots of an `L D Lᵀ` factor (no zero pivot is ever admitted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inertia {
    /// Negative eigenvalues of the factored matrix.
    pub negative: usize,
    /// Positive eigenvalues of the factored matrix.
    pub positive: usize,
}

/// `A = L D Lᵀ` with unit lower `L` on the profile of `A`.
#[derive(Clone, Debug, PartialEq)]
pub struct LdltFactor {
    l: SymmetricProfile,
    d: Vec<f64>,
}

/// A factor whose pivots are all positive: the factored matrix is symmetric positive definite.
#[derive(Clone, Debug, PartialEq)]
pub struct SpdFactor {
    inner: LdltFactor,
}

/// Factor `a` as `L D Lᵀ` without pivoting.
///
/// The rounding floor of row `i` is `ε · n · max_j |a_ij|`, with `ε` the unit roundoff of `f64`. A pivot inside
/// the floor refuses.
///
/// # Errors
/// [`ProfileRefuse::NearZeroPivot`] for a pivot inside its row's rounding floor; [`ProfileRefuse::NonFinite`] when
/// the elimination overflows.
pub fn ldlt(a: &SymmetricProfile) -> Result<LdltFactor, ProfileRefuse> {
    let n = a.n();
    let floor_scale = f64::EPSILON * usize_to_f64(n);
    let mut lv = a.values.clone();
    let mut d = vec![0.0_f64; n];
    for i in 0..n {
        let fi = a.first[i];
        let si = a.start[i];
        let row_max = a.values[si..=si + (i - fi)].iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        // g_j = a_ij − Σ_k g_k L_jk over the shared profile, then L_ij = g_j / d_j; the g_k stay in lv.
        for j in fi..i {
            let fj = a.first[j];
            let sj = a.start[j];
            let k0 = fi.max(fj);
            let acc: f64 = (k0..j).map(|k| lv[si + (k - fi)] * lv[sj + (k - fj)]).sum();
            lv[si + (j - fi)] -= acc;
        }
        let pivot = (fi..i).fold(lv[si + (i - fi)], |p, k| {
            let g = lv[si + (k - fi)];
            let l = g / d[k];
            lv[si + (k - fi)] = l;
            p - g * l
        });
        if !pivot.is_finite() {
            return Err(ProfileRefuse::NonFinite);
        }
        if pivot.abs() <= floor_scale * row_max {
            return Err(ProfileRefuse::NearZeroPivot { row: i });
        }
        d[i] = pivot;
        lv[si + (i - fi)] = 1.0;
    }
    Ok(LdltFactor { l: SymmetricProfile { first: a.first.clone(), start: a.start.clone(), values: lv }, d })
}

/// Factor `a` and require every pivot positive.
///
/// # Errors
/// As [`ldlt`], and [`ProfileRefuse::NotPositiveDefinite`] at the first non-positive pivot.
pub fn spd_factor(a: &SymmetricProfile) -> Result<SpdFactor, ProfileRefuse> {
    let f = ldlt(a)?;
    match f.d.iter().position(|&p| p <= 0.0) {
        Some(row) => Err(ProfileRefuse::NotPositiveDefinite { row }),
        None => Ok(SpdFactor { inner: f }),
    }
}

impl LdltFactor {
    /// Number of rows.
    #[must_use]
    pub fn n(&self) -> usize {
        self.d.len()
    }

    /// Negative and positive pivot counts (Sylvester's law of inertia).
    #[must_use]
    pub fn inertia(&self) -> Inertia {
        let negative = self.d.iter().filter(|&&p| p < 0.0).count();
        Inertia { negative, positive: self.d.len() - negative }
    }

    /// The pivots `D`.
    #[must_use]
    pub fn pivots(&self) -> &[f64] {
        &self.d
    }

    /// Solve `A x = b`.
    ///
    /// # Errors
    /// [`ProfileRefuse::DimMismatch`] for a wrong length; [`ProfileRefuse::NonFinite`] for a NaN or infinite
    /// input or result.
    pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, ProfileRefuse> {
        let n = self.n();
        if b.len() != n {
            return Err(ProfileRefuse::DimMismatch);
        }
        if b.iter().any(|v| !v.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        let first = &self.l.first;
        let start = &self.l.start;
        let lv = &self.l.values;
        // Forward: L y = b, row by row.
        let y = (0..n).fold(Vec::with_capacity(n), |mut y: Vec<f64>, i| {
            let fi = first[i];
            let s: f64 = (fi..i).map(|k| lv[start[i] + (k - fi)] * y[k]).sum();
            y.push(b[i] - s);
            y
        });
        // Diagonal, then backward Lᵀ x = z column by column (row i of L is column i of Lᵀ).
        let z: Vec<f64> = y.iter().zip(&self.d).map(|(yi, di)| yi / di).collect();
        let x = (0..n).rev().fold(z, |mut x, i| {
            let fi = first[i];
            let xi = x[i];
            (fi..i).for_each(|k| x[k] -= lv[start[i] + (k - fi)] * xi);
            x
        });
        if x.iter().any(|v| !v.is_finite()) {
            return Err(ProfileRefuse::NonFinite);
        }
        Ok(x)
    }
}

impl SpdFactor {
    /// The underlying `L D Lᵀ` factor.
    #[must_use]
    pub fn factor(&self) -> &LdltFactor {
        &self.inner
    }

    /// Solve `A x = b`.
    ///
    /// # Errors
    /// As [`LdltFactor::solve`].
    pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, ProfileRefuse> {
        self.inner.solve(b)
    }

    /// `rᵀ A⁻¹ r`, the squared `A⁻¹`-norm (non-negative because `A` is positive definite).
    ///
    /// # Errors
    /// As [`LdltFactor::solve`].
    pub fn inverse_norm_sq(&self, r: &[f64]) -> Result<f64, ProfileRefuse> {
        let x = self.inner.solve(r)?;
        Ok(r.iter().zip(&x).map(|(a, b)| a * b).sum::<f64>().max(0.0))
    }
}

/// `usize` to `f64` for counts below 2⁵³, which every dimension here is.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub(crate) fn usize_to_f64(n: usize) -> f64 {
    n as f64
}

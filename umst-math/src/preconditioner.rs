// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Preconditioners as matrix-free maps \(z \leftarrow M^{-1} v\) (design §2.1 companion surface).

use crate::linear_operator::OpError;

/// Matrix-free preconditioner application on \(\mathbb{R}^n\).
pub trait Preconditioner {
    /// Apply the preconditioner: `out[i]` receives the action of \(M^{-1}\) on `v`.
    fn apply(&self, v: &[f64], out: &mut [f64]) -> Result<(), OpError>;
}

/// Identity preconditioner \(M = I\).
pub struct IdentityPreconditioner {
    n: usize,
}

impl IdentityPreconditioner {
    /// Build the identity preconditioner on \(\mathbb{R}^n\).
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl Preconditioner for IdentityPreconditioner {
    fn apply(&self, v: &[f64], out: &mut [f64]) -> Result<(), OpError> {
        let n = self.n;
        if v.len() != n || out.len() != n {
            return Err(OpError::DimMismatch);
        }
        for i in 0..n {
            let vi = v[i];
            if !vi.is_finite() {
                return Err(OpError::NonFinite);
            }
            out[i] = vi;
        }
        Ok(())
    }
}

/// Jacobi / diagonal preconditioner \(M^{-1} = \mathrm{diag}(1/d_i)\).
pub struct DiagonalPreconditioner {
    inv_diag: Vec<f64>,
}

impl DiagonalPreconditioner {
    /// Build \(M^{-1}\) from a strictly positive finite diagonal of \(M\).
    pub fn from_diagonal_positive(diag: Vec<f64>) -> Result<Self, OpError> {
        let mut inv_diag = Vec::with_capacity(diag.len());
        for &d in &diag {
            if !d.is_finite() || d <= 0.0 {
                return Err(OpError::NonFinite);
            }
            let inv = 1.0 / d;
            if !inv.is_finite() {
                return Err(OpError::NonFinite);
            }
            inv_diag.push(inv);
        }
        Ok(Self { inv_diag })
    }

    /// Dimension of the preconditioned space.
    pub fn n(&self) -> usize {
        self.inv_diag.len()
    }
}

impl Preconditioner for DiagonalPreconditioner {
    fn apply(&self, v: &[f64], out: &mut [f64]) -> Result<(), OpError> {
        let n = self.inv_diag.len();
        if v.len() != n || out.len() != n {
            return Err(OpError::DimMismatch);
        }
        for i in 0..n {
            let vi = v[i];
            if !vi.is_finite() {
                return Err(OpError::NonFinite);
            }
            let val = self.inv_diag[i] * vi;
            if !val.is_finite() {
                return Err(OpError::NonFinite);
            }
            out[i] = val;
        }
        Ok(())
    }
}

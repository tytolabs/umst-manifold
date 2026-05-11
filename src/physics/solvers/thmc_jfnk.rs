// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Matrix-free Krylov helpers for THMC Newton (experimental).

/// GMRES placeholder: returns `Err` so callers fall back to dense Jacobian assembly.
pub fn gmres_f32<F>(_matvec: F, _rhs: &[f32], _dim: usize, _max_iter: usize, _tol: f32) -> Result<Vec<f32>, &'static str>
where
    F: Fn(&[f32]) -> Vec<f32>,
{
    Err("gmres_f32 placeholder: use dense Jacobian path")
}

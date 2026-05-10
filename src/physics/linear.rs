// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Masked inner products for Krylov solves on constrained DOFs (Dirichlet via mask).
//!
//! Shared by mechanics today; future solvers (THMC implicit steps, fracture) should reuse
//! these primitives instead of duplicating `mask *` reduction patterns.

use burn::tensor::{backend::Backend, Tensor};

/// \(\sum_i (a_i m_i)^2\) — masked squared norm.
pub fn masked_norm_sq<B: Backend<FloatElem = f32>>(
    a: &Tensor<B, 3>,
    mask: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let batch = a.dims()[0];
    let n = a.dims()[1];
    let am = a.clone().mul(mask.clone());
    am.clone()
        .mul(am)
        .reshape([batch, n * 3])
        .sum_dim(1)
        .reshape([batch])
}

/// \(\sum_i a_i b_i m_i\) — masked dot product.
pub fn masked_dot<B: Backend<FloatElem = f32>>(
    a: &Tensor<B, 3>,
    b: &Tensor<B, 3>,
    mask: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let batch = a.dims()[0];
    let n = a.dims()[1];
    a.clone()
        .mul(b.clone())
        .mul(mask.clone())
        .reshape([batch, n * 3])
        .sum_dim(1)
        .reshape([batch])
}

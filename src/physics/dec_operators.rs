// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! DEC maps between nodal samples and edge values (Refinement #3).
//!
//! **Naming:** edge-wise reductions are grouped on [`DecEdgeOperators`]; there is no `DecOperator`
//! type in this crate (see repository markdown `docs/FP_CATEGORICAL_DEC.md`).
//!
//! These reductions are **nonlinear** maps \(C^0\to C^1\) on the same primal skeleton as
//! [`super::dec_primal`]’s linear \(d_0\): they average intensive data at endpoints for edge-wise
//! material models. Functorially: they post-compose with gather \((\mathrm{id}\otimes\mathrm{id})^*: C^0\times C^0\to\) edge slots;
//! naturality is with respect to **relabeling / restriction of vertices** that commute with `edges_b1`.
//!
//! - **Arithmetic mean** on edges: intensive mechanical moduli (Young’s modulus) on the primal 1-skeleton.
//! - **Harmonic mean** on edges: flux-consistent reduction for positive transport coefficients.

use burn::tensor::{backend::Backend, Int, Tensor};

use super::topology::EdgeTopology;

pub struct DecEdgeOperators;

impl DecEdgeOperators {
    /// Arithmetic mean at edge endpoints. `[B, N, C]` → `[B, E, C]`.
    pub fn arithmetic_mean_on_edges<B: Backend>(
        nodal: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> Tensor<B, 3> {
        let topo = EdgeTopology::new(edges_b1);
        let (srcv, tgtv) = topo.gather_endpoints(nodal);
        srcv.add(tgtv).mul_scalar(0.5_f32)
    }

    /// Harmonic mean with epsilon floor: `2(a+\epsilon)(b+\epsilon) / (a+b+2\epsilon)`.
    pub fn harmonic_mean_on_edges<B: Backend>(
        nodal: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        eps: f32,
    ) -> Tensor<B, 3> {
        let topo = EdgeTopology::new(edges_b1);
        let (srcv, tgtv) = topo.gather_endpoints(nodal);
        let sa = srcv.clone().add_scalar(eps);
        let sb = tgtv.clone().add_scalar(eps);
        sa.clone()
            .mul(sb.clone())
            .mul_scalar(2.0_f32)
            .div(sa.add(sb))
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Primal-chain DEC primitives: incidence-style scatter without material laws.
//!
//! ## Invariants
//! - Topology follows [`super::topology::EdgeTopology`] (`edges_b1` shape `[2, E]`).
//! - For constant nodal data, `primal_divergence_from_edge_flux(d_0 x, …)` has **zero row-sum**
//!   per channel (closed incidence), matching mass conservation in [`super::laplacian::TopologicalLaplacian`].

use burn::tensor::{backend::Backend, Int, Tensor};

use super::topology::EdgeTopology;

/// Primal **d₀** on nodal 0-cochains: oriented increment `tgt − src` per edge, shape `[B, E, C]`.
#[inline]
pub fn primal_scalar_edge_increment<B: Backend>(
    nodal: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
) -> Tensor<B, 3> {
    let (src, tgt) = topo.gather_endpoints(nodal);
    tgt.sub(src)
}

/// Weak divergence \(B_1^\top\): oriented edge flux `[B, E, C]` → nodal accumulation `[B, N, C]`.
#[inline]
pub fn primal_divergence_from_edge_flux<B: Backend>(
    edge_flux: Tensor<B, 3>,
    src_indices: Tensor<B, 3, Int>,
    tgt_indices: Tensor<B, 3, Int>,
    nodal_zeros_template: &Tensor<B, 3>,
) -> Tensor<B, 3> {
    Tensor::<B, 3>::zeros_like(nodal_zeros_template)
        .scatter(1, src_indices, edge_flux.clone())
        .scatter(1, tgt_indices, edge_flux.neg())
}

/// Same as [`primal_divergence_from_edge_flux`], but indices are derived from `topo` and channel count from `edge_flux`.
#[inline]
pub fn primal_divergence_from_edge_flux_topo<B: Backend>(
    edge_flux: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
    nodal_shape_template: &Tensor<B, 3>,
) -> Tensor<B, 3> {
    let batch = nodal_shape_template.dims()[0];
    let channels = edge_flux.dims()[2];
    let src_ix = topo.expand_src_gather_indices(batch, channels);
    let tgt_ix = topo.expand_tgt_gather_indices(batch, channels);
    primal_divergence_from_edge_flux(edge_flux, src_ix, tgt_ix, nodal_shape_template)
}

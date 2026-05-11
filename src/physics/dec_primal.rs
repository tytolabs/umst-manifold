// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Primal-chain DEC primitives: incidence-style scatter without material laws.
//!
//! ## Invariants
//! - Topology follows [`super::topology::EdgeTopology`] (`edges_b1` shape `[2, E]`).
//! - For constant nodal data, `primal_divergence_from_edge_flux(d_0 x, …)` has **zero row-sum**
//!   per channel (closed incidence), matching mass conservation in [`super::laplacian::TopologicalLaplacian`].
//!
//! ## `faces_b2` (primal \(d_1\) / discrete curl onto 2-cells)
//! [`crate::core::tensors::UnifiedMaterialStateTensor::faces_b2`] uses shape `[2, K]` as **signed COO**
//! columns: **row 0** = global edge index in `0 … E-1`, **row 1** = incidence sign in `{-1, +1}`.
//! Partition columns into faces with [`primal_d1_edge_flux_to_faces`]'s `face_column_ranges`
//! (half-open column slices). Metric/Hodge weights on 2-cells are **not** applied here — topology only.

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
    let idx_cat = Tensor::cat(vec![src_indices, tgt_indices], 1);
    let val_cat = Tensor::cat(vec![edge_flux.clone(), edge_flux.neg()], 1);
    Tensor::<B, 3>::zeros_like(nodal_zeros_template).scatter(1, idx_cat, val_cat)
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

/// Primal **\(d_1\)**: oriented boundary sum of edge 1-cochains into one scalar per face per channel.
///
/// `edge_vals` has shape `[B, E, C]`. `faces_b2` has shape `[2, K]` (see module docs). For each
/// `(start, end)` in `face_column_ranges`, columns `[start, end)` contribute
/// \(\sum_j \sigma_j \, u_{e_j}\) into the next face slot (output order follows the slice order).
///
/// Returns `[B, F, C]` with `F = face_column_ranges.len()`.
pub fn primal_d1_edge_flux_to_faces<B: Backend>(
    edge_vals: Tensor<B, 3>,
    faces_b2: Tensor<B, 2, Int>,
    face_column_ranges: &[(usize, usize)],
) -> Tensor<B, 3> {
    let dims_e = edge_vals.dims();
    let batch = dims_e[0];
    let channels = dims_e[2];
    let device = edge_vals.device();

    let fd = faces_b2.dims();
    debug_assert_eq!(fd[0], 2, "faces_b2: expected shape [2, K]");
    let k = fd[1];

    let mut face_tensors: Vec<Tensor<B, 3>> = Vec::with_capacity(face_column_ranges.len());
    for &(start, end) in face_column_ranges {
        debug_assert!(
            start < end && end <= k,
            "faces_b2: invalid column range [{start}, {end}) for K={k}"
        );
        let len = end - start;
        if len == 0 {
            face_tensors.push(Tensor::zeros([batch, 1, channels], &device));
            continue;
        }

        let edge_ix = faces_b2.clone().slice([0..1, start..end]);
        let signs = faces_b2.clone().slice([1..2, start..end]).float();

        let gather_ix = edge_ix.reshape([1, len, 1]).expand([batch, len, channels]);
        let contrib = edge_vals.clone().gather(1, gather_ix);
        let signs_b = signs.reshape([1, len, 1]).expand([batch, len, channels]);
        let face_sum = contrib
            .mul(signs_b)
            .sum_dim(1)
            .reshape([batch, 1, channels]);
        face_tensors.push(face_sum);
    }

    Tensor::cat(face_tensors, 1)
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Primal 1-skeleton topology for DEC / graph operators.
//!
//! Centralizes `edges_b1` layout `[2, E]` (source row, target row) and the repeated
//! `slice → reshape → expand` gather-index pattern so solvers stay thin.

use burn::tensor::{backend::Backend, Int, Tensor};

/// COO-style edge list on the primal graph: row 0 = source node id, row 1 = target node id.
#[derive(Clone, Debug)]
pub struct EdgeTopology<B: Backend> {
    pub edges_b1: Tensor<B, 2, Int>,
}

impl<B: Backend> EdgeTopology<B> {
    #[inline]
    pub fn new(edges_b1: Tensor<B, 2, Int>) -> Self {
        Self { edges_b1 }
    }

    #[inline]
    pub fn n_edges(&self) -> usize {
        self.edges_b1.dims()[1]
    }

    /// Indices for `gather` along node dim: `[batch, E, channels]` — source endpoints.
    pub fn expand_src_gather_indices(
        &self,
        batch_size: usize,
        channels: usize,
    ) -> Tensor<B, 3, Int> {
        let n_edges = self.n_edges();
        self.edges_b1
            .clone()
            .slice([0..1])
            .reshape([1, n_edges, 1])
            .expand([batch_size, n_edges, channels])
    }

    /// Indices for `gather` along node dim: `[batch, E, channels]` — target endpoints.
    pub fn expand_tgt_gather_indices(
        &self,
        batch_size: usize,
        channels: usize,
    ) -> Tensor<B, 3, Int> {
        let n_edges = self.n_edges();
        self.edges_b1
            .clone()
            .slice([1..2])
            .reshape([1, n_edges, 1])
            .expand([batch_size, n_edges, channels])
    }

    /// Gather nodal `[B, N, C]` values at edge endpoints → `(src_on_edge, tgt_on_edge)` each `[B, E, C]`.
    pub fn gather_endpoints(&self, nodal: Tensor<B, 3>) -> (Tensor<B, 3>, Tensor<B, 3>) {
        let batch_size = nodal.dims()[0];
        let channels = nodal.dims()[2];
        let src_ix = self.expand_src_gather_indices(batch_size, channels);
        let tgt_ix = self.expand_tgt_gather_indices(batch_size, channels);
        let srcv = nodal.clone().gather(1, src_ix.clone());
        let tgtv = nodal.gather(1, tgt_ix);
        (srcv, tgtv)
    }
}

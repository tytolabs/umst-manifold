// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Primal 1-skeleton topology for DEC / graph operators.
//!
//! Centralizes `edges_b1` layout `[2, E]` (row `0` = all source ids, row `1` = all targets) and the
//! repeated `slice → reshape → expand` gather-index pattern so solvers stay thin.
//!
//! **Storage:** Burn’s ndarray backend uses **row-major** contiguous layout: flattened `value` is
//! `[src_0 … src_{E-1}, tgt_0 … tgt_{E-1}]`, not interleaved `[src_0,tgt_0,…]`.
//!
//! # Honest boundary (W29-092)
//!
//! [`EdgeTopology`] is the **primal edge-list SSOT** for gather indices and endpoint gathers used by
//! DEC / bar operators / sensitivity filters. Unit contracts:
//! `cargo test -p umst-manifold topology`. Not physics GREEN, not `PRODUCTION_WIRED`, not
//! `MASTER` / OP-5.

/// W29 deepen cell — primal EdgeTopology honest fence bundle.
pub const W29_TOPOLOGY_DEEPEN_CELL: &str = "W29-092-TOPOLOGY";

/// Honest posture tag — primal edge-list gather SSOT research lane.
pub const TOPOLOGY_POSTURE_TAG: &str = "honest-primal-edge-topology-gather-research-lane";

/// Honest physics posture — gather contracts pass unit tests; does not certify fleet physics GREEN.
pub const TOPOLOGY_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the edge-list SSOT alone.
pub const TOPOLOGY_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const TOPOLOGY_MASTER: bool = false;

/// OP-5 pin — not claimed by this module.
pub const TOPOLOGY_OP5: bool = false;

/// Whether `edges_b1` `[2, E]` + src/tgt gather expand + endpoint gather are landed.
pub const TOPOLOGY_EDGE_GATHER_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const TOPOLOGY_HONEST_FENCE: &str =
    "edge_gather_landed=true src_tgt_expand_wired=true gather_endpoints_wired=true production_wired=false master_composition_wired=false physics_green=false op5=false";

const _: () = assert!(!TOPOLOGY_PHYSICS_GREEN);
const _: () = assert!(!TOPOLOGY_PRODUCTION_WIRED);
const _: () = assert!(!TOPOLOGY_MASTER);
const _: () = assert!(!TOPOLOGY_OP5);
const _: () = assert!(TOPOLOGY_EDGE_GATHER_LANDED);

/// Typed probe for primal EdgeTopology posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeTopologyPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub edge_gather_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for primal EdgeTopology.
#[must_use]
pub fn edge_topology_honest_posture_bundle() -> EdgeTopologyPostureProbe {
    EdgeTopologyPostureProbe {
        physics_green: TOPOLOGY_PHYSICS_GREEN,
        production_wired: TOPOLOGY_PRODUCTION_WIRED,
        master: TOPOLOGY_MASTER,
        op5: TOPOLOGY_OP5,
        edge_gather_landed: TOPOLOGY_EDGE_GATHER_LANDED,
        honest_fence: TOPOLOGY_HONEST_FENCE,
        posture_tag: TOPOLOGY_POSTURE_TAG,
        deepen_cell: W29_TOPOLOGY_DEEPEN_CELL,
    }
}

/// Edge-gather SSOT landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn edge_topology_posture_honest(probe: &EdgeTopologyPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.edge_gather_landed
        && probe.honest_fence.contains("edge_gather_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("op5=false")
        && probe.deepen_cell == W29_TOPOLOGY_DEEPEN_CELL
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 claims on the EdgeTopology surface.
#[must_use]
pub fn edge_topology_refuse_overclaim(
    probe: &EdgeTopologyPostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("TOPOLOGY_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("TOPOLOGY_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("TOPOLOGY_MASTER must stay false — not claimed by edge-list SSOT alone");
    }
    if probe.op5 {
        return Err("TOPOLOGY_OP5 must stay false — not claimed by this module");
    }
    if !edge_topology_posture_honest(probe) {
        return Err("topology posture fence inconsistent");
    }
    Ok(())
}

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

    /// Row count of `edges_b1` (must be 2 for oriented incidence).
    #[inline]
    #[must_use]
    pub fn n_incidence_rows(&self) -> usize {
        self.edges_b1.dims()[0]
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

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn topology_honest_posture_refuses_green_production_master_op5() {
        let probe = edge_topology_honest_posture_bundle();
        assert!(edge_topology_posture_honest(&probe));
        edge_topology_refuse_overclaim(&probe).expect("refuse overclaim");
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(probe.edge_gather_landed);
        assert_eq!(probe.deepen_cell, W29_TOPOLOGY_DEEPEN_CELL);
        assert!(TOPOLOGY_HONEST_FENCE.contains("production_wired=false"));
        assert!(TOPOLOGY_HONEST_FENCE.contains("physics_green=false"));
        assert!(TOPOLOGY_HONEST_FENCE.contains("master_composition_wired=false"));
        assert!(TOPOLOGY_HONEST_FENCE.contains("op5=false"));
    }

    #[test]
    fn topology_edge_list_n_edges_and_incidence_rows() {
        let device = Default::default();
        // Two edges: 0→1, 1→2 — row-major [src…, tgt…]
        let edges = Tensor::<B, 2, Int>::from_data(
            Data::new(vec![0i64, 1, 1, 2], [2, 2].into()),
            &device,
        );
        let topo = EdgeTopology::new(edges);
        assert_eq!(topo.n_incidence_rows(), 2);
        assert_eq!(topo.n_edges(), 2);
    }

    #[test]
    fn topology_expand_src_tgt_gather_shapes_and_values() {
        let device = Default::default();
        let edges = Tensor::<B, 2, Int>::from_data(
            Data::new(vec![0i64, 1, 1, 2], [2, 2].into()),
            &device,
        );
        let topo = EdgeTopology::new(edges);
        let src_ix = topo.expand_src_gather_indices(1, 3);
        let tgt_ix = topo.expand_tgt_gather_indices(1, 3);
        assert_eq!(src_ix.dims(), [1, 2, 3]);
        assert_eq!(tgt_ix.dims(), [1, 2, 3]);
        let src_v = src_ix.into_data().value;
        let tgt_v = tgt_ix.into_data().value;
        // Each edge index is expanded across channels.
        assert_eq!(&src_v[..3], &[0, 0, 0]);
        assert_eq!(&src_v[3..6], &[1, 1, 1]);
        assert_eq!(&tgt_v[..3], &[1, 1, 1]);
        assert_eq!(&tgt_v[3..6], &[2, 2, 2]);
    }

    #[test]
    fn topology_gather_endpoints_reads_nodal_at_edge_ends() {
        let device = Default::default();
        let edges = Tensor::<B, 2, Int>::from_data(
            Data::new(vec![0i64, 1, 1, 2], [2, 2].into()),
            &device,
        );
        let topo = EdgeTopology::new(edges);
        // Nodal scalar values [1, 4, 9] at nodes 0,1,2.
        let nodal = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 4.0, 9.0], [1, 3, 1].into()),
            &device,
        );
        let (srcv, tgtv) = topo.gather_endpoints(nodal);
        assert_eq!(srcv.dims(), [1, 2, 1]);
        assert_eq!(tgtv.dims(), [1, 2, 1]);
        let src = srcv.into_data().value;
        let tgt = tgtv.into_data().value;
        assert!((src[0] - 1.0).abs() < 1e-6 && (src[1] - 4.0).abs() < 1e-6);
        assert!((tgt[0] - 4.0).abs() < 1e-6 && (tgt[1] - 9.0).abs() < 1e-6);
    }
}

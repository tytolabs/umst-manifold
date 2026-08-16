// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
//! Vendored from tytolabs/umst-prototype-2a@9c0434d
//! `prototype/src/rust/core/src/tensors/hyper_graph_tensor.rs`
//! SPDX-License-Identifier: MIT
//!
//! Adaptation: unweighted, single-threaded [`petgraph::graph::UnGraph`] with
//! external `usize` ids; **not** the full `TensorNode` / `Material` / `petgraph` directed
//! semantics (SPDX: structural reduction; closure lives in the sibling module).
//!
//! Public surface: [`HyperGraphTensor`], `add_edge`, `degree` (vendored shape).

use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::prelude::UnGraph;

use super::closure::clique_transitive_close;

/// ZCI-EXEMPT: vendored `UnGraph` tensor adapter; `hypergraph_e_bisim` obligations, not a Lean def (G8)
/// Unified hypergraph tensor (combinatorial adjacency; undirected, simple, no self-loops in storage).
pub struct HyperGraphTensor {
    g: UnGraph<(), ()>,
    id_to_node: HashMap<usize, NodeIndex>,
}

impl Default for HyperGraphTensor {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperGraphTensor {
    /// ZCI-EXEMPT: empty `UnGraph` + id map; structural init (G8)
    /// Empty undirected multigraph (simple after insert).
    #[must_use]
    pub fn new() -> Self {
        Self {
            g: UnGraph::new_undirected(),
            id_to_node: HashMap::new(),
        }
    }

    /// ZCI-EXEMPT: external `usize` → `NodeIndex` map; `petgraph` insert only (G8)
    /// Ensure a vertex exists for the external `id` and return its `NodeIndex`.
    pub fn node_index_of(&mut self, id: usize) -> NodeIndex {
        *self
            .id_to_node
            .entry(id)
            .or_insert_with(|| self.g.add_node(()))
    }

    /// ZCI-EXEMPT: simple undirected edge insert; skip self-loops; monotonic in degree tests (G8)
    /// Add an (undirected) link between the two **external** ids, creating
    /// incident vertices if needed.
    pub fn add_edge(&mut self, a: usize, b: usize) {
        if a == b {
            self.node_index_of(a);
            return;
        }
        let ua = self.node_index_of(a);
        let ub = self.node_index_of(b);
        if self.g.find_edge(ua, ub).is_none() {
            let _ = self.g.add_edge(ua, ub, ());
        }
    }

    /// MEASUREMENT: `petgraph` neighbor count for external id (0 if absent) (G8 hypergraph_e_bisim)
    /// Degree of external id (number of simple edges incident) — 0 if unknown.
    #[must_use]
    pub fn degree(&self, id: usize) -> usize {
        let Some(ua) = self.id_to_node.get(&id) else {
            return 0;
        };
        self.g.neighbors(*ua).count()
    }

    /// ZCI-EXEMPT: `id_to_node` cardinality; bookkeeping stat (G8)
    /// Number of **external** ids with materialised vertices.
    #[must_use]
    pub fn external_id_count(&self) -> usize {
        self.id_to_node.len()
    }

    /// ZCI-EXEMPT: borrow internal `UnGraph` for tests/composers; no new math (G8)
    /// Internal graph — read-only; for advanced composition / tests.
    #[must_use]
    pub fn graph(&self) -> &UnGraph<(), ()> {
        &self.g
    }

    /// ZCI-EXEMPT: calls `clique_transitive_close`; `prop_closure_idempotent` in `hypergraph_e_bisim` (G8)
    /// `closure(closure(h)) = closure(h)` (each component a clique) — `closure.rs`
    /// implements the Warshall / completion step.
    pub fn closure(&mut self) {
        clique_transitive_close(&mut self.g);
    }
}

// PROOF-OBLIGATION: `prop_closure_idempotent` / `prop_add_edge_monotonic_in_degree` in
// `umst-math/tests/hypergraph_e_bisim.rs`.

#[cfg(test)]
mod h2_tensor_lib {
    use super::HyperGraphTensor;

    #[test]
    fn add_edge_increases_degree() {
        let mut g = HyperGraphTensor::new();
        let d0 = g.degree(0);
        g.add_edge(0, 1);
        assert!(g.degree(0) > d0);
    }

    #[test]
    fn closure_idempotent_on_triangle() {
        let mut g = HyperGraphTensor::new();
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        g.closure();
        let n = g.graph().node_count();
        g.closure();
        assert_eq!(g.graph().node_count(), n);
    }

    #[test]
    fn external_id_tracks_on_edge() {
        let mut g = HyperGraphTensor::new();
        g.add_edge(5, 7);
        assert_eq!(g.external_id_count(), 2);
    }
}

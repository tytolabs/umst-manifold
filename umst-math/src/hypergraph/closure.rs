// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Hypergraph (undirected) **clique** closure: within each connected component, add
//! the missing simple edges; idempotent. Specialises **RED §0.8** to an algebraic
//! idempotent (projective) operator on a finite unweighted `UnGraph`.
//!
//! # PROOF-OBLIGATION
//! - `prop_closure_idempotent` — `umst-math/tests/hypergraph_e_bisim.rs`
//! - `prop_add_edge_monotonic_in_degree` — as above
//!
//! SPDX: adaptation under MIT (see `tensor.rs` vendoring header)

use std::collections::{HashSet, VecDeque};

use petgraph::graph::NodeIndex;
use petgraph::prelude::UnGraph;

/// For each **connected** component, close under **clique** (all pairs linked).
/// Second application is a no-op on a simple unweighted `UnGraph`.
pub fn clique_transitive_close(g: &mut UnGraph<(), ()>) {
    if g.node_count() < 2 {
        return;
    }
    let mut remaining: HashSet<NodeIndex> = g.node_indices().collect();
    while let Some(s) = remaining.iter().next().copied() {
        let comp = bfs_reachable(g, s);
        for n in &comp {
            remaining.remove(n);
        }
        for a in 0..comp.len() {
            for b in a + 1..comp.len() {
                if g.find_edge(comp[a], comp[b]).is_none() {
                    let _ = g.add_edge(comp[a], comp[b], ());
                }
            }
        }
    }
}

fn bfs_reachable(g: &UnGraph<(), ()>, start: NodeIndex) -> Vec<NodeIndex> {
    let mut q = VecDeque::new();
    let mut vis: HashSet<NodeIndex> = HashSet::new();
    let mut out: Vec<NodeIndex> = vec![];
    vis.insert(start);
    out.push(start);
    q.push_back(start);
    while let Some(ni) = q.pop_front() {
        for nbr in g.neighbors(ni) {
            if vis.insert(nbr) {
                out.push(nbr);
                q.push_back(nbr);
            }
        }
    }
    out
}

// —— local smoke (e-bisim: `tests/hypergraph_e_bisim.rs`)
#[cfg(test)]
mod tests {
    use super::clique_transitive_close;
    use petgraph::prelude::UnGraph;

    fn three_path() -> UnGraph<(), ()> {
        let mut g = UnGraph::new_undirected();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, ());
        g.add_edge(b, c, ());
        g
    }

    #[test]
    fn path3_becomes_clique3() {
        let mut g = three_path();
        clique_transitive_close(&mut g);
        let nodes: Vec<_> = g.node_indices().collect();
        let u = nodes[0];
        let w = nodes[2];
        assert!(g.find_edge(u, w).is_some());
    }
}

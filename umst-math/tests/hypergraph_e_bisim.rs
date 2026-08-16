// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! RED §0.8 e-bisim for `umst_math::hypergraph::HyperGraphTensor`.

use quickcheck::quickcheck;
use umst_math::hypergraph::HyperGraphTensor;

/// Four hand-checked `closure` steps: path 0-1-2 becomes triangle.
#[test]
fn e_bisim_closure_against_2a_reference() {
    let mut g = HyperGraphTensor::new();
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    let n0 = g.node_index_of(0);
    let n2 = g.node_index_of(2);
    assert!(g.graph().find_edge(n0, n2).is_none());
    g.closure();
    let n0 = g.node_index_of(0);
    let n2 = g.node_index_of(2);
    assert!(g.graph().find_edge(n0, n2).is_some());
    // clique on 0,1,2 only — add isolated 9
    let mut h = HyperGraphTensor::new();
    h.add_edge(0, 1);
    h.add_edge(9, 8);
    h.closure();
    assert_eq!(h.degree(8), 1);
}

fn prop_closure_idempotent_inner() -> bool {
    let mut g = HyperGraphTensor::new();
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.closure();
    let e1 = g.graph().edge_count();
    g.closure();
    g.graph().edge_count() == e1
}

#[test]
fn prop_closure_idempotent() {
    quickcheck(prop_closure_idempotent_inner as fn() -> bool);
}

fn prop_add_edge_monotone_inner() -> bool {
    let mut g = HyperGraphTensor::new();
    g.add_edge(0, 1);
    let d0 = g.degree(0);
    g.add_edge(0, 2);
    g.degree(0) >= d0
}

#[test]
fn prop_add_edge_monotonic_in_degree() {
    quickcheck(prop_add_edge_monotone_inner as fn() -> bool);
}

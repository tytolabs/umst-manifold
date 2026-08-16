// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Finite hypergraphs + **monoidal** edge products (Phase K2).
//!
//! Distinct from [`crate::tensor`] (Kronecker / `DensityDiag` bridges) — this module is combinatorial
//! incidence + algebraic composition for multi-agent / credit **geometry** prototypes.
//!
//! # Portability (§0.3 I-A / rule 4)
//! All kernels are **scalar `f64` loops** — no `f32x8` AVX, no CUDA. If SIMD is ever needed, gate it
//! behind `feature = "simd"` using portable `wide` (already optional in this crate) or `core::simd`,
//! and keep this path as the **fallback** reference implementation.
//!
//! Proof: `GraphProperties` (finite hypergraph incidence); `Naturality` (relabeling commutes with
//! edge maps); `MonoidalState` (associative tensor product monoid on diagonal factors).
//! DOI: 10.5281/zenodo.19159660

mod closure;
mod functor;
mod graph;
mod tensor;

pub use closure::clique_transitive_close;
pub use functor::{compose_scalar_maps, edge_tensor_mul, relabel_edge_vertices};
pub use graph::Hypergraph;
pub use tensor::HyperGraphTensor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hypergraph_zero_edges() {
        let g = Hypergraph::new(0);
        assert_eq!(g.vertex_count(), 0);
        assert!(g.edges().is_empty());
    }

    #[test]
    fn add_hyperedge_normalizes_order_dedup() {
        let mut g = Hypergraph::new(3);
        g.add_hyperedge(&[2, 0, 1, 1]).expect("ok");
        assert_eq!(g.edges(), &[vec![0, 1, 2]]);
    }

    #[test]
    fn add_hyperedge_idempotent() {
        let mut g = Hypergraph::new(2);
        g.add_hyperedge(&[0, 1]).expect("ok");
        g.add_hyperedge(&[1, 0]).expect("ok");
        assert_eq!(g.edges().len(), 1);
    }

    #[test]
    fn edge_tensor_mul_associative_three_verts() {
        let labels = [2.0_f64, 3.0, 5.0];
        let edge = [0_usize, 1, 2];
        let p = edge_tensor_mul(&labels, &edge).expect("p");
        assert!(((labels[0] * labels[1]) * labels[2] - p).abs() < 1e-12);
        assert!((labels[0] * (labels[1] * labels[2]) - p).abs() < 1e-12);
    }

    #[test]
    fn functor_compose_identity_on_scalar_map() {
        let f = |x: f64| x + 1.0;
        let id = |x: f64| x;
        let c = compose_scalar_maps(f, id);
        assert!((c(2.0) - 3.0).abs() < 1e-12);
        let c2 = compose_scalar_maps(id, f);
        assert!((c2(2.0) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn degenerate_single_vertex_edge() {
        let labels = [4.2_f64];
        let p = edge_tensor_mul(&labels, &[0]).expect("p");
        assert!((p - 4.2).abs() < 1e-12);
    }

    #[test]
    fn relabel_identity_preserves_edge() {
        let e = [0_usize, 2];
        let id = [0, 1, 2];
        let out = relabel_edge_vertices(&e, &id).expect("rel");
        assert_eq!(out, vec![0, 2]);
    }

    #[test]
    fn relabel_composes_with_functor_on_labels() {
        let edge = [0_usize, 1];
        let labels = [2.0_f64, 3.0];
        let p0 = edge_tensor_mul(&labels, &edge).expect("p0");
        // permutation: swap 0 and 1
        let perm = [1, 0];
        let new_edge = relabel_edge_vertices(&edge, &perm).expect("ne");
        let swapped = [3.0_f64, 2.0];
        let p1 = edge_tensor_mul(&swapped, &new_edge).expect("p1");
        assert!((p0 - p1).abs() < 1e-12);
    }

    #[test]
    fn out_of_range_vertex_rejected() {
        let mut g = Hypergraph::new(2);
        assert!(g.add_hyperedge(&[0, 3]).is_err());
    }
}

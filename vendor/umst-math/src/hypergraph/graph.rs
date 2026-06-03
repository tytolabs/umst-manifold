//! Mutable hypergraph with sorted-unique hyperedges.
//!
//! Proof: `GraphProperties` — finite vertex set + set of hyperedges (incidence structure).
//! DOI: 10.5281/zenodo.19159660

/// Hypergraph \(H=(V,E)\) with \(V=\{0,\dots,n-1\}\) and each edge a nonempty subset of \(V\)
/// stored as a sorted duplicate-free `Vec<usize>`.
///
/// Proof: edges are finite subsets; representation normalises set equality.
/// DOI: 10.5281/zenodo.19159660
/// ZCI-EXEMPT: finite-hypergraph carrier matching `GraphProperties` doc proof sketch; not a new Lean def name (G8)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hypergraph {
    n_vertices: usize,
    edges: Vec<Vec<usize>>,
}

impl Hypergraph {
    /// ZCI-EXEMPT: empty edge list on `0..n_vertices`; `GraphProperties` object (G8)
    /// Empty edge set on `n_vertices` nodes (may be `0`).
    ///
    /// Proof: empty hypergraph is a valid object in `GraphProperties`.
    /// DOI: 10.5281/zenodo.19159660
    #[must_use]
    pub fn new(n_vertices: usize) -> Self {
        Hypergraph {
            n_vertices,
            edges: Vec::new(),
        }
    }

    /// ZCI-EXEMPT: `|V|` readout; incident to `GraphProperties` (G8)
    /// \(|V|\).
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        self.n_vertices
    }

    /// ZCI-EXEMPT: read-only `E` as sorted-unique `Vec<usize>` rows (G8)
    /// Read-only view of edges (each inner vec sorted, length ≥ 1).
    #[must_use]
    pub fn edges(&self) -> &[Vec<usize>] {
        &self.edges
    }

    /// ZCI-EXEMPT: normalise + dedup hyperedge; idempotent add per `GraphProperties/union_closed` (G8)
    /// Insert a hyperedge; `verts` is sorted, deduplicated, and checked against `n_vertices`.
    /// Duplicate edge sets are ignored (idempotent).
    ///
    /// Proof: `GraphProperties/union_closed` — closure under adding an edge from the edge space.
    /// DOI: 10.5281/zenodo.19159660
    pub fn add_hyperedge(&mut self, verts: &[usize]) -> Result<(), &'static str> {
        if verts.is_empty() {
            return Err("empty hyperedge");
        }
        let mut v = verts.to_vec();
        v.sort_unstable();
        v.dedup();
        for &i in &v {
            if i >= self.n_vertices {
                return Err("vertex out of range");
            }
        }
        if self.edges.iter().any(|e| e == &v) {
            return Ok(());
        }
        self.edges.push(v);
        Ok(())
    }
}

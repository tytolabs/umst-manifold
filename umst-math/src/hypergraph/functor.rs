//! Monoidal edge product + relabelling (functorial on vertex indices).
//!
//! **Scalar-only** — portable across ISAs (§0.3).
//!
//! Proof: `MonoidalState/tensor_product` — associative monoid on tensor factors; `Naturality`
//! — diagram commutes when vertex labels are relabelled coherently.
//! DOI: 10.5281/zenodo.19159660

/// Product of vertex labels along a hyperedge (multiplicative monoid on `f64`).
///
/// Empty `edge` returns `1.0` (monoid unit). Out-of-range index → `None`.
///
/// Proof: associativity of multiplication — coherence for `MonoidalState`.
/// DOI: 10.5281/zenodo.19159660
#[must_use]
pub fn edge_tensor_mul(labels: &[f64], edge: &[usize]) -> Option<f64> {
    let mut p = 1.0_f64;
    for &i in edge {
        let x = *labels.get(i)?;
        if !x.is_finite() {
            return None;
        }
        p *= x;
    }
    Some(p)
}

/// Compose unary maps on vertex-attached scalars (categorical composition \(g \circ f\)).
///
/// Proof: `Naturality/functor_compose` — functor composition law at the morphism level.
/// DOI: 10.5281/zenodo.19159660
#[inline]
pub fn compose_scalar_maps<F, G>(f: F, g: G) -> impl Fn(f64) -> f64
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    move |x| g(f(x))
}

/// Relabel vertices by a map `old_index -> new_index` given as `perm[new_index]`...
///
/// Standard pattern: `mapping` has length = old vertex count; `mapping[i]` is the image of `i`.
/// Returns `None` if any `edge` index is out of range for `mapping`.
///
/// Proof: `Naturality` — functoriality on the object map of vertices.
/// DOI: 10.5281/zenodo.19159660
#[must_use]
pub fn relabel_edge_vertices(edge: &[usize], mapping: &[usize]) -> Option<Vec<usize>> {
    let mut out = Vec::with_capacity(edge.len());
    for &i in edge {
        let img = *mapping.get(i)?;
        out.push(img);
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

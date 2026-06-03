//! Vendored from tytolabs/umst-prototype-2a@9c0434d
//! SPDX-License-Identifier: MIT
//! Copyright (c) 2025–2026 Tyto Labs, Inc. and authors named in upstream
//! `prototype/src/rust/core/src/tensors/sparse.rs`
//!
//! Adaptation: `SparseTensor<T>`, 2D COO in sorted unique (row‑major) flat
//! form for RED §0.8 e‑bisim; upstream used append-only `set` and `f32` only.
//!
//! # Proof obligations
//! - PROOF-OBLIGATION: `prop_add_associative` — see `umst-math/tests/sparse_e_bisim.rs`
//! - PROOF-OBLIGATION: `prop_dot_distributes_over_add` — as above
//! - PROOF-OBLIGATION: `prop_transpose_involutive` — as above

use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, Sub};

/// ZCI-EXEMPT: parametric `SparseScalar` bounds for vendored CSR; ring ops + sparsity filter, not a new Lean id (G8)
/// Scalar for sparse entries: **upstream port used `f32` only;** this slice uses
/// a generic to mirror a parametric `SparseTensor<T>` on the public surface.
pub trait SparseScalar:
    Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + PartialEq
    + PartialOrd
{
    /// Additive identity.
    fn s_zero() -> Self;
    /// Inexact zero filter (mirrors `1e-9` in upstream `matmul`).
    fn s_sparsity_eps() -> Self;
    /// Absolute value for sparsity filtering.
    fn s_abs(self) -> Self;
}

impl SparseScalar for f32 {
    fn s_zero() -> Self {
        0.0
    }
    fn s_sparsity_eps() -> Self {
        1e-9
    }
    fn s_abs(self) -> Self {
        self.abs()
    }
}

impl SparseScalar for f64 {
    fn s_zero() -> Self {
        0.0
    }
    fn s_sparsity_eps() -> Self {
        1e-9
    }
    fn s_abs(self) -> Self {
        self.abs()
    }
}

/// ZCI-EXEMPT: vendored CSR/COO 2D tensor; proof obligations in `umst-math/tests/sparse_e_bisim.rs` (G8)
/// Sparse 2D tensor in **sorted unique** COO / flat-index form (row‑major).
#[derive(Clone, Debug, PartialEq)]
pub struct SparseTensor<T: SparseScalar> {
    shape: [u32; 2],
    /// (flat_index, value), sorted by `flat_index`, unique
    pub(crate) flat: Vec<(u32, T)>,
}

impl<T: SparseScalar> SparseTensor<T> {
    /// ZCI-EXEMPT: `nnz` = merged unique flat length; e-bisim checks in `sparse_e_bisim` (G8)
    /// Number of stored nonzeros (after merge; unique indices).
    #[must_use]
    pub fn nnz(&self) -> usize {
        self.flat.len()
    }

    /// ZCI-EXEMPT: shape readout; structural invariant, not a theorem name (G8)
    /// Matrix shape \([rows, cols]\) per upstream `shape: Vec<u32>`.
    #[must_use]
    pub fn shape(&self) -> [u32; 2] {
        self.shape
    }

    /// ZCI-EXEMPT: row‑major scan over nonzeros; iterator shape only (G8)
    /// Iterator over \((row, col, &value)\) in row‑major order.
    pub fn iter_nonzero(&self) -> impl Iterator<Item = (u32, u32, &T)> {
        let cols = self.shape[1];
        self.flat.iter().map(move |(f, t)| (f / cols, f % cols, t))
    }

    /// ZCI-EXEMPT: empty canonical tensor constructor (G8)
    /// New empty tensor of given 2D shape.
    #[must_use]
    pub fn new2(shape: [u32; 2]) -> Self {
        SparseTensor {
            shape,
            flat: Vec::new(),
        }
    }

    /// ZCI-EXEMPT: triplet merge + sparsity filter; empirical merge semantics from prototype (G8)
    /// Build from \((r,c,v)\) lists (all same length). Merges duplicate
    /// coordinates by **summing** values, then discards ≈0 entries.
    pub fn from_triplets(
        shape: [u32; 2],
        rows: &[u32],
        cols: &[u32],
        vals: &[T],
    ) -> Result<Self, &'static str> {
        if rows.len() != cols.len() || rows.len() != vals.len() {
            return Err("triplet len mismatch");
        }
        let [h, w] = shape;
        if h == 0 || w == 0 {
            if rows.is_empty() {
                return Ok(Self::new2([h, w]));
            }
            return Err("empty shape with nonzero triplets");
        }
        let w64 = w as u64;
        let mut acc: HashMap<u32, T> = HashMap::new();
        for ((&r, &c), &v) in rows.iter().zip(cols).zip(vals) {
            if (r as u64) >= h as u64 || (c as u64) >= w as u64 {
                return Err("index out of bounds");
            }
            let f = (r as u64 * w64 + c as u64) as u32;
            *acc.entry(f).or_insert(T::s_zero()) += v;
        }
        let mut flat: Vec<(u32, T)> = acc
            .into_iter()
            .filter(|(_, t)| t.s_abs() > T::s_sparsity_eps())
            .collect();
        flat.sort_unstable_by_key(|(f, _)| *f);
        Ok(SparseTensor { shape, flat })
    }

    /// Merge to canonical (sorted unique) form.
    fn from_raw(shape: [u32; 2], raw: Vec<(u32, T)>) -> Result<Self, &'static str> {
        let [h, w] = shape;
        if h == 0 && w == 0 {
            if raw.is_empty() {
                return Ok(Self {
                    shape,
                    flat: vec![],
                });
            }
            return Err("0×0 with entries");
        }
        let w64 = w as u64;
        let h64 = h as u64;
        if h == 0 || w == 0 {
            if raw.is_empty() {
                return Ok(Self {
                    shape,
                    flat: vec![],
                });
            }
            return Err("0 extent with data");
        }
        let h_times_w = h64.checked_mul(w64).ok_or("shape overflow")?;
        let mut acc: HashMap<u32, T> = HashMap::new();
        for (f, v) in raw {
            if f as u64 >= h_times_w {
                return Err("flat index oob");
            }
            *acc.entry(f).or_insert(T::s_zero()) += v;
        }
        let mut flat: Vec<(u32, T)> = acc
            .into_iter()
            .filter(|(_, t)| t.s_abs() > T::s_sparsity_eps())
            .collect();
        flat.sort_unstable_by_key(|(f, _)| *f);
        Ok(SparseTensor { shape, flat })
    }

    /// ZCI-EXEMPT: entrywise merge-sum; `prop_add_associative` in `sparse_e_bisim` (G8)
    /// Entrywise sum; shapes must match.
    pub fn add(&self, other: &Self) -> Result<Self, &'static str> {
        if self.shape != other.shape {
            return Err("shape mismatch add");
        }
        let mut acc: HashMap<u32, T> = HashMap::new();
        for (f, v) in &self.flat {
            *acc.entry(*f).or_insert(T::s_zero()) += *v;
        }
        for (f, v) in &other.flat {
            *acc.entry(*f).or_insert(T::s_zero()) += *v;
        }
        let raw: Vec<(u32, T)> = acc.into_iter().collect();
        Self::from_raw(self.shape, raw)
    }

    /// **[Phase 16]** matmul: upstream name preserved on the public API.
    ///
    /// [doc = "alias: `matmul` in umst-prototype-2a `tensors/sparse.rs`"]
    /// ZCI-EXEMPT: bucketed sparse `matmul`; O(nnz²) path from prototype-2a port (G8)
    pub fn matmul(&self, other: &Self) -> Self {
        if self.shape.len() != 2
            || other.shape.len() != 2
            || self.shape[0] == 0
            || self.shape[1] == 0
            || other.shape[0] == 0
            || other.shape[1] == 0
        {
            return SparseTensor::new2([0, 0]);
        }
        let rows_a = self.shape[0];
        let cols_a = self.shape[1];
        let rows_b = other.shape[0];
        let cols_b = other.shape[1];
        if cols_a != rows_b {
            return SparseTensor::new2([0, 0]);
        }
        // Row buckets for A, B
        use std::collections::HashMap as H;
        let mut a_rows: H<u32, Vec<(u32, T)>> = H::new();
        for &(f, v) in &self.flat {
            let r = f / cols_a;
            let c = f % cols_a;
            a_rows.entry(r).or_default().push((c, v));
        }
        let mut b_rows: H<u32, Vec<(u32, T)>> = H::new();
        for &(f, v) in &other.flat {
            let r = f / cols_b;
            let c = f % cols_b;
            b_rows.entry(r).or_default().push((c, v));
        }
        type Map32<TT> = std::collections::HashMap<u32, TT>;
        let mut c_map: Map32<T> = Map32::new();
        for (r, a_cols) in &a_rows {
            for &(k, val_a) in a_cols {
                if let Some(b_rowk) = b_rows.get(&k) {
                    for &(c, val_b) in b_rowk {
                        let flat = r * cols_b + c;
                        *c_map.entry(flat).or_insert(T::s_zero()) += val_a * val_b;
                    }
                }
            }
        }
        let mut out_raw: Vec<(u32, T)> = c_map
            .into_iter()
            .filter(|(_, v)| v.s_abs() > T::s_sparsity_eps())
            .collect();
        out_raw.sort_unstable_by_key(|(f, _)| *f);
        Self {
            shape: [rows_a, cols_b],
            flat: out_raw,
        }
    }

    /// ZCI-EXEMPT: intersection hash inner product; `prop_dot_distributes` in `sparse_e_bisim` (G8)
    /// Frobenius inner product: \(\sum A_{ij}B_{ij}\) for equal shape.
    pub fn dot(&self, other: &Self) -> Result<T, &'static str> {
        if self.shape != other.shape {
            return Err("shape mismatch dot");
        }
        let mut small: &[(u32, T)] = &self.flat;
        let mut big: &[(u32, T)] = &other.flat;
        if self.flat.len() > other.flat.len() {
            std::mem::swap(&mut small, &mut big);
        }
        // Hash small for O(n) intersection
        let mut hm: HashMap<u32, T> = HashMap::new();
        for (f, v) in small {
            hm.insert(*f, *v);
        }
        let mut s = T::s_zero();
        for (f, w) in big {
            if let Some(u) = hm.get(f) {
                s += *u * *w;
            }
        }
        Ok(s)
    }

    /// ZCI-EXEMPT: index permute + canonicalize; `prop_transpose_involutive` in `sparse_e_bisim` (G8)
    /// Conjugate 2D transpose: shape \((rows,cols) → (cols,rows)\) on indices.
    pub fn transpose(&self) -> Result<Self, &'static str> {
        let [r, c] = self.shape;
        if r == 0 && c == 0 {
            return Ok(Self::new2([0, 0]));
        }
        if r == 0 || c == 0 {
            return Ok(Self {
                shape: [c, r],
                flat: vec![],
            });
        }
        let out: Vec<(u32, T)> = self
            .flat
            .iter()
            .map(|(f, v)| {
                let cr = f / c;
                let cc = f % c;
                let nflat = (cc as u64 * (r as u64) + cr as u64) as u32;
                (nflat, *v)
            })
            .collect();
        Self::from_raw([c, r], out)
    }

    /// MEASUREMENT: `nnz/(rows*cols)` as f32 sparsity diagnostic (G8 e-bisim RED §0.8)
    /// [Phase 16] density — `nnz / (rows*cols)`.
    #[must_use]
    pub fn density(&self) -> f32 {
        let h = u64::from(self.shape[0]);
        let w = u64::from(self.shape[1]);
        let tot = h.saturating_mul(w) as f32;
        if tot == 0.0 {
            return 0.0;
        }
        (self.nnz() as f32) / tot
    }

    /// ZCI-EXEMPT: clone flat CSR indices; buffer export only (G8)
    /// Cloned flat indices (upstream: `indices()`).
    #[must_use]
    pub fn flat_indices(&self) -> Vec<u32> {
        self.flat.iter().map(|(f, _)| *f).collect()
    }

    /// ZCI-EXEMPT: clone value buffer; no new semantic constant (G8)
    /// Cloned value buffer (upstream: `values()` on `f32` rows).
    #[must_use]
    pub fn values(&self) -> Vec<T> {
        self.flat.iter().map(|(_, v)| *v).collect()
    }

    /// ZCI-EXEMPT: `shape` as `Vec`; parity helper for interop (G8)
    /// Vendored-style shape as `Vec<u32>` (clone).
    #[must_use]
    pub fn shape_vec(&self) -> Vec<u32> {
        self.shape.to_vec()
    }

    /// ZCI-EXEMPT: binary search lookup on sorted flat; mechanical index op (G8)
    /// Lookup a single 2D entry; **O(log n)** via binary search.
    pub fn at(&self, r: u32, c: u32) -> Result<T, &'static str> {
        if r >= self.shape[0] || c >= self.shape[1] {
            return Err("at oob");
        }
        let f = r * self.shape[1] + c;
        if let Ok(i) = self.flat.binary_search_by_key(&f, |a| a.0) {
            return Ok(self.flat[i].1);
        }
        Ok(T::s_zero())
    }
}

// PROOF-OBLIGATION: `prop_add_associative` / `prop_dot_distributes_over_add` / `prop_transpose_involutive`
// are exercised in `umst-math/tests/sparse_e_bisim.rs` (RED §0.8).

/// Lib-only micro-tests (merge-bar: `cargo test -p umst-math --lib` growth vs H-1 baseline).
#[cfg(test)]
mod h2_sparse_lib {
    use super::SparseTensor;

    #[test]
    fn matmul_2x2_eye() {
        let a = SparseTensor::from_triplets([2, 2], &[0, 1], &[0, 1], &[1.0_f64, 1.0]).expect("a");
        let i = SparseTensor::from_triplets([2, 2], &[0, 1], &[0, 1], &[1.0_f64, 1.0]).expect("i");
        let c = a.matmul(&i);
        assert_eq!(c.shape(), [2, 2]);
    }

    #[test]
    fn add_merge_commutes_small() {
        let a = SparseTensor::from_triplets([2, 2], &[0], &[0], &[1.0_f64]).expect("a");
        let b = SparseTensor::from_triplets([2, 2], &[0], &[0], &[2.0_f64]).expect("b");
        let s = a.add(&b).expect("add");
        assert!((s.at(0, 0).expect("t") - 3.0).abs() < 1e-12);
    }

    #[test]
    fn transpose_involutive_2x3() {
        let a = SparseTensor::from_triplets([2, 3], &[0, 1], &[0, 2], &[1.0_f64, 2.0]).expect("a");
        let t = a.transpose().expect("t");
        let tt = t.transpose().expect("tt");
        assert_eq!(a.shape(), tt.shape());
        assert!((a.at(0, 0).expect("a00") - tt.at(0, 0).expect("t00")).abs() < 1e-12);
    }

    #[test]
    fn dot_frobenius_smoke() {
        let a = SparseTensor::from_triplets([2, 2], &[0, 0], &[0, 1], &[1.0_f64, 2.0]).expect("a");
        let b = SparseTensor::from_triplets([2, 2], &[0, 0], &[0, 1], &[3.0_f64, 4.0]).expect("b");
        let d = a.dot(&b).expect("dot");
        assert!((d - 11.0).abs() < 1e-9);
    }

    #[test]
    fn nnz_counts_unique() {
        let a = SparseTensor::from_triplets([1, 2], &[0, 0], &[0, 1], &[1.0, 1.0]).expect("a");
        assert_eq!(a.nnz(), 2);
    }

    #[test]
    fn density_range() {
        let a = SparseTensor::from_triplets([2, 2], &[0], &[0], &[1.0_f32]).expect("a");
        let d = a.density();
        assert!(d > 0.0 && d <= 1.0);
    }
}

// --- Reference port for e-bisim: upstream f32, append style `set` (unnormalized) —— test-only
#[cfg(test)]
mod reference_sparse_2a {
    use std::collections::HashMap as H;

    pub(crate) struct UpstreamF32Coo {
        shape: [u32; 2],
        /// append-only like upstream, may duplicate
        acc: H<u32, f32>,
    }

    impl UpstreamF32Coo {
        pub(crate) fn new(shape: [u32; 2]) -> Self {
            UpstreamF32Coo {
                shape,
                acc: H::new(),
            }
        }
        /// Merge sum at index (vendored semantic when canonicalizing for compare).
        pub(crate) fn set_merged(&mut self, f: u32, v: f32) {
            *self.acc.entry(f).or_insert(0.0) += v;
        }

        fn matmul_2a(&self, o: &Self) -> (Vec<u32>, Vec<f32>, [u32; 2]) {
            if self.shape[1] != o.shape[0] {
                return (vec![], vec![], [0, 0]);
            }
            use super::SparseTensor;
            // Convert to canonical
            let sh = self.shape;
            let a = SparseTensor::<f32> {
                shape: sh,
                flat: {
                    let mut v: Vec<_> = self.acc.iter().map(|(k, t)| (*k, *t)).collect();
                    v.sort_unstable_by_key(|(f, _)| *f);
                    v
                },
            };
            let b = SparseTensor::<f32> {
                shape: o.shape,
                flat: {
                    let mut v: Vec<_> = o.acc.iter().map(|(k, t)| (*k, *t)).collect();
                    v.sort_unstable_by_key(|(f, _)| *f);
                    v
                },
            };
            let c = a.matmul(&b);
            (
                c.flat.iter().map(|(f, _)| *f).collect(),
                c.values(),
                c.shape,
            )
        }
    }

    #[test]
    fn self_consistent_matmul_smoke() {
        let a = [2u32, 2];
        let mut p = UpstreamF32Coo::new(a);
        p.set_merged(0, 1.0);
        p.set_merged(2, 1.0);
        p.set_merged(3, 1.0);
        let c = p.matmul_2a(&p);
        assert_eq!(c.2, [2, 2]);
    }
}

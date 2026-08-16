// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![allow(clippy::single_range_in_vec_init)]

//! Primal-chain DEC primitives: incidence-style scatter without material laws.
//!
//! ## Functoriality / naturality (discrete spaces)
//!
//! Hold the oriented **primal** skeleton \((V,E,F,\ldots)\) fixed (`edges_b1`, optional `faces_b2`).
//! Nodal tensors are 0-cochains \(C^0\); edge tensors are 1-cochains \(C^1\); face tensors are 2-cochains
//! \(C^2\). Each primitive here is a **linear** map between these finite-dimensional section spaces
//! determined only by topology and orientation — a morphism in the diagram of chain groups. Varying the
//! input cochain while keeping indices fixed is therefore the action of a **natural** family (identity
//! on morphisms in the usual “only indices change” sense for DEC software). [`primal_d1_edge_flux_to_faces`]
//! and [`primal_d1_transpose_face_flux_to_edges`] are adjoints under unweighted Frobenius pairings; see
//! `tests/dec_identities.rs` for Burn-level checks. Metric / Hodge post-scaling belongs in physics callers,
//! not in this topology-only layer.
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
//! The transpose [`primal_d1_transpose_face_flux_to_edges`] scatters face potentials back to edges with
//! the same signs (Euclidean-weight adjoint of \(d_1\) for unweighted inner products).
//!
//! ## Volumetric 3D topology hook (matrix **#6** slice)
//! [`canonical_tetrahedron_boundary_dec_coo`] materialises the **oriented boundary 2-chain** (four
//! triangles) of a single canonical **3-simplex** with vertices `0…3` and six globally indexed
//! directed edges — the same **`faces_b2` / `edges_b1`** COO contract as material tensors. This is
//! **surface-only** (the **skin** of one tet); it does **not** assemble interior facets of a volume
//! mesh or call photonics — it exists so tests and future mesh loaders can share one **closed**
//! volumetric **primal** boundary pattern without duplicating hand-authored COO.
//!
//! # Honest boundary (W29-050)
//!
//! Linear primal incidence (`d_0`, \(B_1^\top\), \(d_1\), \(d_1^\top\)) plus the canonical tet
//! boundary COO and \(d_1\!\circ\!d_0\) witness are **topology-only** maps. Burn identity checks live
//! in `tests/dec_identities.rs` (`cargo test -p umst-manifold dec_primal`). Not physics GREEN, not
//! `PRODUCTION_WIRED`, not `MASTER`.

/// W29 deepen cell — primal DEC honest fence bundle.
pub const W29_DEC_PRIMAL_DEEPEN_CELL: &str = "W29-050-DEC_PRIMAL";

/// Honest posture tag — primal incidence landed; fleet production wiring refused.
pub const DEC_PRIMAL_POSTURE_TAG: &str = "honest-dec-primal-incidence-research-lane";

/// Honest physics posture — topology identities pass Burn tests; does not certify fleet physics GREEN.
pub const DEC_PRIMAL_PHYSICS_GREEN: bool = false;

/// R14-5 — `d₁∘d₀ = 0` measured exactly on canonical tet boundary DEC complex.
pub const DEC_DD_ZERO_EXACT_MEASURED: bool = true;

/// Production wiring — not claimed by primal incidence alone.
pub const DEC_PRIMAL_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const DEC_PRIMAL_MASTER: bool = false;

/// Whether linear \(d_0\) / divergence / \(d_1\) / \(d_1^\top\) Burn maps are landed.
pub const DEC_PRIMAL_INCIDENCE_LANDED: bool = true;

/// Whether [`canonical_tetrahedron_boundary_dec_coo`] closed-surface hook is landed.
pub const DEC_PRIMAL_TET_BOUNDARY_COO_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const DEC_PRIMAL_HONEST_FENCE: &str =
    "dec_primal_incidence_landed=true d0_wired=true d1_wired=true d1_transpose_wired=true tet_boundary_coo_wired=true production_wired=false master_composition_wired=false physics_green=false";

/// Typed probe for primal DEC posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecPrimalPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub incidence_landed: bool,
    pub tet_boundary_coo_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for primal DEC.
#[must_use]
pub fn dec_primal_honest_posture_bundle() -> DecPrimalPostureProbe {
    DecPrimalPostureProbe {
        physics_green: DEC_PRIMAL_PHYSICS_GREEN,
        production_wired: DEC_PRIMAL_PRODUCTION_WIRED,
        master: DEC_PRIMAL_MASTER,
        incidence_landed: DEC_PRIMAL_INCIDENCE_LANDED,
        tet_boundary_coo_landed: DEC_PRIMAL_TET_BOUNDARY_COO_LANDED,
        honest_fence: DEC_PRIMAL_HONEST_FENCE,
        posture_tag: DEC_PRIMAL_POSTURE_TAG,
        deepen_cell: W29_DEC_PRIMAL_DEEPEN_CELL,
    }
}

/// Primal DEC SSOT landed with production/master composition honestly open.
#[must_use]
pub fn dec_primal_posture_honest(probe: &DecPrimalPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.incidence_landed
        && probe.tet_boundary_coo_landed
        && probe
            .honest_fence
            .contains("dec_primal_incidence_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

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

/// Primal **\(d_1^\top\)**: scatter oriented face 2-cochains onto incident edges.
///
/// For each face `f` with column range `(start, end)` in `faces_b2`, adds
/// \(\sigma_j \, \phi_f\) to global edge `e_j` for every column `j` in `[start, end)`,
/// where \(\sigma_j\) is the stored incidence sign. This matches the transpose of
/// [`primal_d1_edge_flux_to_faces`] under componentwise dot products on `[B, E, C]` and `[B, F, C]`
/// (no Hodge / metric weighting).
///
/// `face_vals` must have shape `[B, F, C]` with `F = face_column_ranges.len()`. Empty ranges contribute
/// nothing (consistent with a zero row in \(d_1\)).
pub fn primal_d1_transpose_face_flux_to_edges<B: Backend>(
    face_vals: Tensor<B, 3>,
    faces_b2: Tensor<B, 2, Int>,
    face_column_ranges: &[(usize, usize)],
    edge_accum_template: &Tensor<B, 3>,
) -> Tensor<B, 3> {
    let dims_f = face_vals.dims();
    let batch = dims_f[0];
    let f_count = dims_f[1];
    let channels = dims_f[2];
    debug_assert_eq!(
        f_count,
        face_column_ranges.len(),
        "face_vals: dim 1 must equal face_column_ranges.len()"
    );

    let fd = faces_b2.dims();
    debug_assert_eq!(fd[0], 2, "faces_b2: expected shape [2, K]");
    let k = fd[1];

    let mut acc = Tensor::zeros_like(edge_accum_template);
    for (face_idx, &(start, end)) in face_column_ranges.iter().enumerate() {
        debug_assert!(
            start <= end && end <= k,
            "faces_b2: invalid column range [{start}, {end}) for K={k}"
        );
        if start >= end {
            continue;
        }
        let len = end - start;
        let fv = face_vals
            .clone()
            .slice([0..batch, face_idx..face_idx + 1, 0..channels]);
        let edge_ix = faces_b2.clone().slice([0..1, start..end]);
        let signs = faces_b2.clone().slice([1..2, start..end]).float();

        for j in 0..len {
            let e_idx = edge_ix.clone().slice([0..1, j..j + 1]);
            let s = signs.clone().slice([0..1, j..j + 1]);
            let gather_ix = e_idx.reshape([1, 1, 1]).expand([batch, 1, channels]);
            let contrib = fv
                .clone()
                .mul(s.reshape([1, 1, 1]).expand([batch, 1, channels]));
            acc = acc.scatter(1, gather_ix, contrib);
        }
    }
    acc
}

/// Maximum absolute face value of \(d_1(d_0 \omega)\) for a scalar \(\omega\) on vertices (primal DEC).
///
/// For a **closed** oriented 2-chain encoded in `faces_b2` / `face_column_ranges` consistent with
/// `edges_b1` (same COO contract as [`crate::core::tensors::UnifiedMaterialStateTensor::faces_b2`]),
/// this should be **≈ 0** up to float noise — see Burn checks in `tests/dec_identities.rs`
/// (`dec_curl_d1_annihilates_gradient_*`). The photonics `faces_b2` patch path uses this as a
/// **cheap incidence witness** before attempting a patch curl–curl solve.
#[must_use]
pub fn dec_primal_max_abs_d1_of_scalar_gradient<B: Backend<FloatElem = f32>>(
    nodal_omega: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
    faces_b2: Tensor<B, 2, Int>,
    face_column_ranges: &[(usize, usize)],
) -> f32 {
    let g = primal_scalar_edge_increment(nodal_omega, topo);
    let d1g = primal_d1_edge_flux_to_faces(g, faces_b2, face_column_ranges);
    d1g.abs().max().into_scalar()
}

/// Host-side **`edges_b1` / `faces_b2`** for the **closed oriented boundary** of one canonical tetrahedron.
///
/// Vertices are **`0,1,2,3`**. Six directed edges (global ids **0…5**):
/// `0→1`, `0→2`, `0→3`, `1→2`, `1→3`, `2→3`. Four triangular faces (boundary of the 3-simplex) use
/// column ranges **`(0,3)`, `(3,6)`, `(6,9)`, `(9,12)`** — three signed edge columns per face, same
/// storage as [`crate::core::tensors::UnifiedMaterialStateTensor::faces_b2`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalTetrahedronBoundaryDecCoo {
    /// Row-major **`[2, 6]`** data: six sources then six targets (`edges_b1` contract).
    pub edges_b1_flat: [i64; 12],
    /// Row-major **`[2, 12]`** data: twelve edge indices then twelve signs in **`{-1, +1}`**.
    pub faces_b2_flat: [i64; 24],
    /// Half-open column slices into **`faces_b2_flat`** — one per boundary triangle.
    pub face_column_ranges: [(usize, usize); 4],
}

/// Returns fixed **topology-only** COO for the **skin** of a single **positively oriented** tet
/// (`\det(v_1-v_0,v_2-v_0,v_3-v_0)>0` in \(\mathbb{R}^3\) when embedded with that vertex order).
///
/// Boundary walks are chosen so **`dec_primal_max_abs_d1_of_scalar_gradient`** is ~**0** on random
/// nodal data (closed simplicial surface — discrete **`d_1\!\circ\!d_0=0`** witness). Does **not**
/// allocate; safe to call from tests or mesh glue code.
#[must_use]
pub fn canonical_tetrahedron_boundary_dec_coo() -> CanonicalTetrahedronBoundaryDecCoo {
    CanonicalTetrahedronBoundaryDecCoo {
        edges_b1_flat: [
            0, 0, 0, 1, 1, 2, //
            1, 2, 3, 2, 3, 3,
        ],
        // Face opposite 0: 1→2 (+e3), 2→3 (+e5), 3→1 (−e4); opposite 1: 0→2, 2→3, 3→0; …
        faces_b2_flat: [
            3, 5, 4, 1, 5, 2, 0, 4, 2, 0, 3, 1, //
            1, 1, -1, 1, 1, -1, 1, 1, -1, 1, 1, -1,
        ],
        face_column_ranges: [(0, 3), (3, 6), (6, 9), (9, 12)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn dec_primal_honest_posture_refuses_green_and_production() {
        let probe = dec_primal_honest_posture_bundle();
        assert!(dec_primal_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.incidence_landed);
        assert!(probe.tet_boundary_coo_landed);
        assert_eq!(probe.deepen_cell, W29_DEC_PRIMAL_DEEPEN_CELL);
        assert!(DEC_PRIMAL_HONEST_FENCE.contains("production_wired=false"));
        assert!(DEC_PRIMAL_HONEST_FENCE.contains("physics_green=false"));
        assert!(DEC_PRIMAL_HONEST_FENCE.contains("master_composition_wired=false"));
    }

    #[test]
    fn dec_primal_canonical_tet_coo_shape_and_ranges() {
        let coo = canonical_tetrahedron_boundary_dec_coo();
        assert_eq!(coo.edges_b1_flat.len(), 12);
        assert_eq!(coo.faces_b2_flat.len(), 24);
        assert_eq!(coo.face_column_ranges, [(0, 3), (3, 6), (6, 9), (9, 12)]);
        // Six directed edges: src row then tgt row.
        assert_eq!(&coo.edges_b1_flat[0..6], &[0, 0, 0, 1, 1, 2]);
        assert_eq!(&coo.edges_b1_flat[6..12], &[1, 2, 3, 2, 3, 3]);
        // Incidence signs are ±1.
        for &s in &coo.faces_b2_flat[12..24] {
            assert!(s == 1 || s == -1, "sign must be ±1, got {s}");
        }
        // Edge indices in faces_b2 row 0 stay in 0…5.
        for &e in &coo.faces_b2_flat[0..12] {
            assert!((0..6).contains(&e), "edge index out of range: {e}");
        }
    }

    #[test]
    fn dec_primal_d0_constant_nodal_is_zero() {
        let device = Default::default();
        let edges =
            Tensor::<B, 2, Int>::from_data(Data::new(vec![0i64, 1, 1, 2], [2, 2].into()), &device);
        let topo = EdgeTopology::new(edges);
        let nodal =
            Tensor::<B, 3>::from_data(Data::new(vec![3.0_f32; 3], [1, 3, 1].into()), &device);
        let inc = primal_scalar_edge_increment(nodal, &topo);
        for x in inc.into_data().value {
            assert!(x.abs() < 1e-6, "constant field → zero d0, got {x}");
        }
    }

    #[test]
    fn dec_primal_divergence_of_d0_row_sum_zero_on_chain() {
        let device = Default::default();
        let edges =
            Tensor::<B, 2, Int>::from_data(Data::new(vec![0i64, 1, 1, 2], [2, 2].into()), &device);
        let topo = EdgeTopology::new(edges);
        let nodal = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0_f32, 4.0, 9.0], [1, 3, 1].into()),
            &device,
        );
        let d0 = primal_scalar_edge_increment(nodal.clone(), &topo);
        let div = primal_divergence_from_edge_flux_topo(d0, &topo, &nodal);
        let row_sum: f32 = div.into_data().value.iter().sum();
        assert!(
            row_sum.abs() < 1e-5,
            "closed incidence: row-sum of B1^T d0 x ≈ 0, got {row_sum}"
        );
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::needless_range_loop)]

//! Discrete Exterior Calculus identities on the 1-skeleton.
//!
//! These tests assert the two foundational identities the manifold
//! relies on for conservation of mass and energy:
//!
//!   - `d ∘ d = 0` on a closed chain (verified via the Hodge Laplacian
//!     row-sum being zero).
//!   - The Hodge Laplacian is symmetric: `Δ = Δᵀ`.
//!   - The discrete Stokes identity holds on a triangle.
//!   - **Photonics / Track 15 prerequisite:** on one oriented triangle, the discrete edge curl
//!     \(d_1\) applied to the gradient \(d_0 \omega\) vanishes (\(d_1 \circ d_0 = 0\)), matching
//!     [`docs/Solver-Status.md`](../docs/Solver-Status.md) DEFERRAL — Photonics (single-triangle
//!     DEC curl sanity; [`dec_curl_d1_annihilates_gradient_on_triangle_faces_b2_burn`] uses
//!     [`umst_manifold::physics::dec_primal::primal_d1_edge_flux_to_faces`] with production-shaped
//!     [`faces_b2`](umst_manifold::core::tensors::UnifiedMaterialStateTensor::faces_b2).
//!     [`dec_primal_d1_adjoint_identity_single_triangle_burn`] locks the unweighted discrete adjoint
//!     via [`umst_manifold::physics::dec_primal::primal_d1_transpose_face_flux_to_edges`].

use approx::assert_abs_diff_eq;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::dec_primal::{
    primal_d1_edge_flux_to_faces, primal_d1_transpose_face_flux_to_edges,
    primal_scalar_edge_increment,
};
use umst_manifold::physics::topology::EdgeTopology;

type NdB = NdArray<f32>;

/// Build a signed vertex-edge incidence matrix for a ring on `n` vertices.
/// Edge `e` connects vertex `e` (tail) to vertex `(e + 1) mod n` (head).
/// Returns a row-major `n × n` matrix.
fn ring_b1(n: usize) -> Vec<Vec<f32>> {
    let mut b1 = vec![vec![0.0_f32; n]; n];
    for e in 0..n {
        let tail = e;
        let head = (e + 1) % n;
        b1[head][e] = 1.0;
        b1[tail][e] = -1.0;
    }
    b1
}

fn matmul(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let m = a.len();
    let k = a[0].len();
    let n = b[0].len();
    assert_eq!(k, b.len());
    let mut out = vec![vec![0.0_f32; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut s = 0.0;
            for p in 0..k {
                s += a[i][p] * b[p][j];
            }
            out[i][j] = s;
        }
    }
    out
}

fn transpose(a: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let m = a.len();
    let n = a[0].len();
    let mut out = vec![vec![0.0_f32; m]; n];
    for i in 0..m {
        for j in 0..n {
            out[j][i] = a[i][j];
        }
    }
    out
}

#[test]
fn d_squared_zero() {
    // On a ring graph, d ∘ d acts as a degree-2 boundary operator and
    // must annihilate any 0-cochain. We test it on a unit impulse.
    let n = 8;
    let b1 = ring_b1(n);
    let b1t = transpose(&b1);

    let mut omega = vec![0.0_f32; n];
    omega[0] = 1.0;

    // d.omega = B1^T . omega
    let mut d_omega = vec![0.0_f32; n];
    for e in 0..n {
        for v in 0..n {
            d_omega[e] += b1t[e][v] * omega[v];
        }
    }

    // d. (d.omega) on a ring is zero because every edge is in exactly
    // one closed loop.
    let total: f32 = d_omega.iter().sum();
    assert_abs_diff_eq!(total, 0.0, epsilon = 1.0e-6);
}

#[test]
fn laplacian_symmetric() {
    let n = 8;
    let b1 = ring_b1(n);
    let b1t = transpose(&b1);

    // Δ = B1 . B1^T  (Hodge Laplacian on 0-cochains)
    let lap = matmul(&b1, &b1t);
    let lap_t = transpose(&lap);

    for i in 0..n {
        for j in 0..n {
            assert_abs_diff_eq!(lap[i][j], lap_t[i][j], epsilon = 1.0e-6);
        }
    }
}

#[test]
fn stokes_triangle() {
    // Triangle: 3 vertices, 3 directed edges forming a closed 1-cycle.
    // Σ d.omega over the closed cycle = 0 for any 0-cochain.
    let n = 3;
    let b1 = ring_b1(n);
    let b1t = transpose(&b1);

    let omega = [1.5_f32, 2.7, -0.8];
    let mut d_omega = vec![0.0_f32; n];
    for e in 0..n {
        for v in 0..n {
            d_omega[e] += b1t[e][v] * omega[v];
        }
    }

    let total: f32 = d_omega.iter().sum();
    assert_abs_diff_eq!(total, 0.0, epsilon = 1.0e-6);
}

/// Discrete \(d_1\) on the sole triangular face: CCW boundary walk uses all three ring edges with
/// coefficient \(+1\) ([`ring_b1`] orients edges \(0\!\to\!1\), \(1\!\to\!2\), \(2\!\to\!0\)).
fn triangle_d1_times_edge(edge_vals: &[f32; 3]) -> f32 {
    edge_vals[0] + edge_vals[1] + edge_vals[2]
}

#[test]
fn dec_curl_d1_annihilates_gradient_on_triangle() {
    // d_1(d_0 ω) = 0 for scalar ω on vertices — discrete analogue of curl(grad f) = 0.
    let n = 3;
    let b1 = ring_b1(n);
    let b1t = transpose(&b1);

    let omega = [1.2_f32, -0.5, 3.1];
    let mut grad_on_edges = vec![0.0_f32; n];
    for e in 0..n {
        for v in 0..n {
            grad_on_edges[e] += b1t[e][v] * omega[v];
        }
    }

    let curl_sum = triangle_d1_times_edge(&[grad_on_edges[0], grad_on_edges[1], grad_on_edges[2]]);
    assert_abs_diff_eq!(curl_sum, 0.0, epsilon = 1.0e-5);
}

#[test]
fn dec_curl_d1_annihilates_gradient_on_triangle_faces_b2_burn() {
    // Same identity as `dec_curl_d1_annihilates_gradient_on_triangle`, but d₀ and d₁ go through
    // `edges_b1` + `faces_b2` tensors and Burn gather/scatter (`DEFERRAL — Photonics`, Next PR (2)).
    let dev = NdArrayDevice::default();
    let edges_b1: Tensor<NdB, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, // sources
                1, 2, 0, // targets — CCW ring 0→1→2→0
            ],
            Shape::new([2, 3]),
        ),
        &dev,
    );
    // One triangular face: boundary walk uses edges 0,1,2 with orientation matching `edges_b1`.
    let faces_b2: Tensor<NdB, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, // edge ids
                1, 1, 1, // signs (+1 each)
            ],
            Shape::new([2, 3]),
        ),
        &dev,
    );
    let topo = EdgeTopology::new(edges_b1);
    let omega = [1.2_f32, -0.5, 3.1];
    let nodal = Tensor::from_data(
        Data::new(vec![omega[0], omega[1], omega[2]], Shape::new([1, 3, 1])),
        &dev,
    );
    let grad_on_edges = primal_scalar_edge_increment(nodal, &topo);
    let d1_grad = primal_d1_edge_flux_to_faces(grad_on_edges, faces_b2, &[(0, 3)]);
    let v: Vec<f32> = d1_grad.into_data().value;
    assert_eq!(v.len(), 1);
    assert_abs_diff_eq!(v[0], 0.0, epsilon = 1.0e-4);
}

/// Unweighted Frobenius inner product \(\langle a, b\rangle = \sum_{b,n,c} a\,b\) on matching `[B,N,C]`.
fn tensor_inner(a: Tensor<NdB, 3>, b: Tensor<NdB, 3>) -> f32 {
    a.mul(b).sum().into_scalar()
}

#[test]
fn dec_primal_d1_adjoint_identity_single_triangle_burn() {
    // ⟨ d₁ u , w ⟩ = ⟨ u , d₁ᵀ w ⟩ on one CCW triangle (same `faces_b2` as the Burn annihilation test).
    let dev = NdArrayDevice::default();
    let faces_b2: Tensor<NdB, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, //
                1, 1, 1,
            ],
            Shape::new([2, 3]),
        ),
        &dev,
    );
    let ranges = [(0usize, 3usize)];
    let u = Tensor::from_data(
        Data::new(vec![0.7_f32, -1.1, 0.25], Shape::new([1, 3, 1])),
        &dev,
    );
    let w = Tensor::from_data(Data::new(vec![-0.33_f32], Shape::new([1, 1, 1])), &dev);
    let d1u = primal_d1_edge_flux_to_faces(u.clone(), faces_b2.clone(), &ranges);
    let lhs = tensor_inner(d1u, w.clone());
    let d1t_w = primal_d1_transpose_face_flux_to_edges(w, faces_b2, &ranges, &u);
    let rhs = tensor_inner(u.clone(), d1t_w);
    assert_abs_diff_eq!(lhs, rhs, epsilon = 1.0e-5);
}

#[test]
fn laplacian_row_sum_zero() {
    // Mass-conservation invariant: the Laplacian's row sum is zero, so
    // applying it to a constant 0-cochain yields the zero vector.
    let n = 16;
    let b1 = ring_b1(n);
    let lap = matmul(&b1, &transpose(&b1));

    for row in lap.iter() {
        let s: f32 = row.iter().sum();
        assert_abs_diff_eq!(s, 0.0, epsilon = 1.0e-6);
    }
}

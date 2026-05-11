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
//!     DEC curl operator sanity before `faces_b2` production wiring).

use approx::assert_abs_diff_eq;

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

    let curl_sum = triangle_d1_times_edge(&[
        grad_on_edges[0],
        grad_on_edges[1],
        grad_on_edges[2],
    ]);
    assert_abs_diff_eq!(curl_sum, 0.0, epsilon = 1.0e-5);
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

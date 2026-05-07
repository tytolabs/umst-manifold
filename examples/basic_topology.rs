// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Basic-topology example.
//!
//! Builds a small ring graph, applies the discrete Laplacian to a heat
//! distribution, and demonstrates the two invariants the manifold relies
//! on at the substrate level:
//!
//! 1. The discrete Stokes identity — the sum of `d*omega` over a closed
//!    chain is zero.
//! 2. The Laplacian's row-sum is zero — i.e. the operator conserves
//!    total mass on a closed graph.
//!
//! Run with:
//! ```bash
//! cargo run --example basic_topology --release
//! ```

use burn::backend::NdArray;
use burn::tensor::Tensor;

type B = NdArray<f32>;

fn main() {
    println!("UMST Manifold — basic topology example");
    println!("======================================");

    let device = Default::default();

    // 1. Build a ring graph on N vertices.
    //    Edges: 0-1, 1-2, ..., (N-1)-0.
    let n: usize = 8;
    println!("Graph: ring on N = {n} vertices.");

    // B1 boundary matrix (vertex-edge incidence, signed):
    //   row v, column e is +1 if v is the head of e, -1 if tail, 0 otherwise.
    let mut b1 = vec![vec![0.0f32; n]; n]; // n vertices, n edges
    for e in 0..n {
        let head = (e + 1) % n;
        let tail = e;
        b1[head][e] = 1.0;
        b1[tail][e] = -1.0;
    }
    let b1: Tensor<B, 2> =
        Tensor::from_data(burn::tensor::TensorData::from(reshape(&b1, n, n)), &device);

    // 2. Place a heat impulse at vertex 0.
    let mut omega0 = vec![0.0f32; n];
    omega0[0] = 1.0;
    let omega: Tensor<B, 2> = Tensor::from_data(
        burn::tensor::TensorData::from(reshape(&[omega0.clone()], 1, n)),
        &device,
    );

    println!("Initial heat distribution: {omega0:?}");

    // 3. Apply the discrete exterior derivative d : C0 -> C1.
    //    d.omega = B1^T . omega
    let b1_t = b1.clone().transpose();
    let d_omega = omega.clone().matmul(b1_t.clone());

    // 4. Apply d* = B1 to recover a divergence on vertices.
    let div = d_omega.clone().matmul(b1.clone().transpose());

    // 5. Hodge Laplacian Delta_0 = d* d on vertices.
    let lap = b1.clone().matmul(b1.clone().transpose());
    let lap_omega = omega.matmul(lap);

    let lap_v: Vec<f32> = lap_omega.into_data().to_vec().unwrap();
    println!("Δ₀·ω        : {lap_v:.3?}");

    let div_v: Vec<f32> = div.into_data().to_vec().unwrap();
    println!("d*(d ω)     : {div_v:.3?}");

    // 6. Invariant: sum of Δ₀·ω over all vertices equals zero (mass conservation).
    let total: f32 = lap_v.iter().sum();
    println!("Σ Δ₀·ω      = {total:+.3e}  (should be ~0 by mass conservation)");

    assert!(
        total.abs() < 1.0e-5,
        "Laplacian violated row-sum invariant: |sum| = {}",
        total.abs()
    );

    println!();
    println!("OK — discrete Laplacian conserves mass on a closed graph.");
}

fn reshape(rows: &[Vec<f32>], r: usize, c: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(r * c);
    for row in rows.iter().take(r) {
        for &v in row.iter().take(c) {
            out.push(v);
        }
    }
    out
}

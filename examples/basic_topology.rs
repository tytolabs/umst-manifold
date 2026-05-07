// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::needless_range_loop)]

//! Basic-topology example.
//!
//! Builds a small ring graph, applies the discrete Laplacian to a heat
//! impulse, and demonstrates the two invariants the manifold relies on
//! at the substrate level:
//!
//! 1. The discrete Stokes identity — `Σ d ω = 0` over a closed cycle.
//! 2. Mass conservation — the Laplacian's row-sum is zero.
//!
//! No `burn` backend is needed: the example uses pure-stdlib matrix
//! arithmetic so it runs on any platform without GPU drivers.
//!
//! Run with:
//! ```bash
//! cargo run --example basic_topology --release
//! ```

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

fn matvec(m: &[Vec<f32>], v: &[f32]) -> Vec<f32> {
    m.iter()
        .map(|row| row.iter().zip(v).map(|(a, b)| a * b).sum())
        .collect()
}

fn matmul(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let m = a.len();
    let k = a[0].len();
    let n = b[0].len();
    let mut out = vec![vec![0.0_f32; n]; m];
    for i in 0..m {
        for j in 0..n {
            for p in 0..k {
                out[i][j] += a[i][p] * b[p][j];
            }
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

fn main() {
    println!("UMST Manifold — basic topology example");
    println!("======================================");

    let n: usize = 8;
    println!("Graph: ring on N = {n} vertices.");

    let b1 = ring_b1(n);
    let b1t = transpose(&b1);

    let mut omega = vec![0.0_f32; n];
    omega[0] = 1.0;
    println!("Initial 0-cochain ω : {omega:?}");

    let d_omega = matvec(&b1t, &omega);
    println!("d ω                : {d_omega:?}");

    let lap = matmul(&b1, &b1t);
    let lap_omega = matvec(&lap, &omega);
    println!("Δ₀ ω = d* d ω      : {lap_omega:?}");

    let total: f32 = lap_omega.iter().sum();
    println!("Σ Δ₀ ω             = {total:+.3e}  (mass-conservation invariant)");

    assert!(
        total.abs() < 1.0e-5,
        "Laplacian row-sum invariant violated: |sum| = {}",
        total.abs()
    );

    let stokes: f32 = d_omega.iter().sum();
    println!("Σ d ω over cycle    = {stokes:+.3e}  (discrete Stokes invariant)");
    assert!(
        stokes.abs() < 1.0e-5,
        "Discrete Stokes violated: |sum| = {}",
        stokes.abs()
    );

    println!();
    println!("OK — both DEC invariants hold to within 1e-5.");
}

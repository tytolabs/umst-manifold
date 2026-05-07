// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Mass conservation under topology mutation.
//!
//! Verifies that severing an edge (the discrete analogue of fracture)
//! preserves total mass — i.e. the Laplacian's row-sum stays zero on
//! the mutated graph.

use approx::assert_abs_diff_eq;

fn line_b1(n: usize) -> Vec<Vec<f32>> {
    // Open chain on n vertices, n-1 edges.
    let m = n - 1;
    let mut b1 = vec![vec![0.0_f32; m]; n];
    for e in 0..m {
        b1[e + 1][e] = 1.0;
        b1[e][e] = -1.0;
    }
    b1
}

fn sever_edge(mut b1: Vec<Vec<f32>>, edge: usize) -> Vec<Vec<f32>> {
    for row in b1.iter_mut() {
        row[edge] = 0.0;
    }
    b1
}

fn laplacian_row_sums(b1: &[Vec<f32>]) -> Vec<f32> {
    let n = b1.len();
    let m = b1[0].len();
    let mut sums = vec![0.0_f32; n];
    for i in 0..n {
        for j in 0..n {
            let mut x = 0.0_f32;
            for e in 0..m {
                x += b1[i][e] * b1[j][e];
            }
            sums[i] += x;
        }
    }
    sums
}

#[test]
fn mass_conserved_on_open_chain() {
    let n = 32;
    let b1 = line_b1(n);
    for s in laplacian_row_sums(&b1) {
        assert_abs_diff_eq!(s, 0.0, epsilon = 1.0e-6);
    }
}

#[test]
fn mass_conserved_under_severing() {
    let n = 32;
    let b1 = line_b1(n);
    let severed = sever_edge(b1, 7);
    for s in laplacian_row_sums(&severed) {
        assert_abs_diff_eq!(s, 0.0, epsilon = 1.0e-6);
    }
}

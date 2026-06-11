// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Permanent property: `k_char` nondim is an exact change of variables on the original PCG loop.
//!
//! Bisection arm B must be trajectory-identical to arm A (same iter count, `|Δu|∞` within f32 slack).

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::q1_hex_elasticity::{
    hex_solve_pcg_bisect, HexPcgBisectConfig, HexPcgLoopKind, HEX_PCG_REL_TOL_F32,
};

fn harness_pin_bottom_perimeter(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut bm = vec![1.0_f32; n * 3];
    let mut pin = |ix: usize, iy: usize| {
        let nid = ix + iy * nx1;
        bm[nid * 3] = 0.0;
        bm[nid * 3 + 1] = 0.0;
        bm[nid * 3 + 2] = 0.0;
    };
    for ix in 0..=nx {
        pin(ix, 0);
        pin(ix, ny);
    }
    for iy in 0..=ny {
        pin(0, iy);
        pin(nx, iy);
    }
    let _ = nz;
    bm
}

fn run_original(
    nondim: bool,
    nx: usize,
    ny: usize,
    nz: usize,
    lx: f32,
    ly: f32,
    lz: f32,
) -> umst_manifold::physics::q1_hex_elasticity::HexPcgBisectReport {
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: lx / nx as f32,
        dy: ly / ny as f32,
        dz: lz / nz as f32,
    };
    let n = plate.n_nodes();
    let e = 0.5_f32.powf(3.0) * (200e6_f32 - 1.0) + 1.0;
    let e_cell = vec![e; nx * ny * nz];
    let bf = plate.body_force_top_uniform_pressure(50.0);
    let bm = harness_pin_bottom_perimeter(nx, ny, nz);
    let mut diag = vec![0.0_f32; n * 3];
    let mut scratch = vec![0.0_f32; n * 3];
    hex_solve_pcg_bisect(
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        0.2,
        &e_cell,
        &bf,
        &bm,
        &mut diag,
        &mut scratch,
        2000,
        true,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::Original,
            nondim,
            stop_on_true_residual: false,
        },
    )
}

fn assert_a_equiv_b(nx: usize, ny: usize, nz: usize, lx: f32, ly: f32, lz: f32) {
    let a = run_original(false, nx, ny, nz, lx, ly, lz);
    let b = run_original(true, nx, ny, nz, lx, ly, lz);
    let u_scale = a.u.iter().map(|x| x.abs()).fold(1.0_f32, f32::max);
    let u_diff =
        a.u.iter()
            .zip(&b.u)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0_f32, f32::max);
    assert!(
        a.rel_residual_true.is_finite() && a.rel_residual_true <= HEX_PCG_REL_TOL_F32,
        "arm A must converge at {nx}x{ny}x{nz}: r_true={}",
        a.rel_residual_true
    );
    assert!(
        a.iterations.abs_diff(b.iterations) <= 1,
        "nondim must not materially change PCG iteration count at {nx}x{ny}x{nz}: a={} b={}",
        a.iterations,
        b.iterations
    );
    assert!(
        u_diff < 1e-4 * u_scale,
        "nondim must be trajectory-identical at {nx}x{ny}x{nz}: |du|_inf={u_diff}"
    );
    assert!(
        b.rel_residual_true.is_finite(),
        "arm B must finish finite at {nx}x{ny}x{nz}: r_true={}",
        b.rel_residual_true
    );
}

#[test]
fn q1_hex_nondim_trajectory_identity_quick_9x8x2() {
    assert_a_equiv_b(9, 8, 2, 0.8, 0.8, 0.1);
}

#[test]
fn q1_hex_nondim_trajectory_identity_coarse_4x4x1() {
    assert_a_equiv_b(4, 4, 1, 0.4, 0.4, 0.05);
}

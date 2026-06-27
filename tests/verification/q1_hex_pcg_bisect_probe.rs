// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! 2×2 bisection at 9×8×2, tol [`HEX_PCG_REL_TOL_F32`]: loop × nondim.

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::q1_hex_elasticity::{
    hex_equilibrium_residual_parts, hex_precond_from_use_preconditioner, hex_solve_pcg_bisect,
    HexPcgBisectConfig, HexPcgLoopKind, HEX_PCG_REL_TOL_F32,
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

fn uniform_e_cell(nx: usize, ny: usize, nz: usize, e: f32) -> Vec<f32> {
    vec![e; nx * ny * nz]
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

#[allow(clippy::too_many_arguments)]
fn run_arm(
    label: &str,
    nx: usize,
    ny: usize,
    nz: usize,
    lx: f32,
    ly: f32,
    lz: f32,
    max_cg: usize,
    tol: f32,
    cfg: HexPcgBisectConfig,
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
    let e_cell = uniform_e_cell(nx, ny, nz, e);
    let bf = plate.body_force_top_uniform_pressure(50.0);
    let bm = harness_pin_bottom_perimeter(nx, ny, nz);
    let mut diag = vec![0.0_f32; n * 3];
    let mut scratch = vec![0.0_f32; n * 3];
    let report = hex_solve_pcg_bisect(
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
        max_cg,
        hex_precond_from_use_preconditioner(true),
        tol,
        cfg,
    );
    let parts = hex_equilibrium_residual_parts(
        nx, ny, nz, plate.dx, plate.dy, plate.dz, 0.2, &e_cell, &bf, &bm, &report.u,
    );
    eprintln!(
        "Q1_HEX_BISECT {label}: iters={} |Pf|={:.3e}N |Pr|={:.3e}N rel=|Pr|/|Pf|={:.3e} r_recursive={:.3e} k_char={:.3e} loop={:?} nondim={}",
        report.iterations,
        parts.abs_rhs,
        parts.abs_residual,
        parts.rel_residual,
        report.rel_residual_recursive,
        report.stiffness_scale,
        cfg.loop_kind,
        cfg.nondim,
    );
    assert!(
        (parts.rel_residual - report.rel_residual_true).abs() < 1e-6 * parts.rel_residual.max(1.0),
        "{label}: r_true must match |Pr|/|Pf|"
    );
    report
}

#[test]
fn q1_hex_pcg_bisect_2x2_at_quick_scale() {
    let quick = (
        9_usize, 8_usize, 2_usize, 0.8_f32, 0.8_f32, 0.1_f32, 2000_usize,
    );
    let a = run_arm(
        "A",
        quick.0,
        quick.1,
        quick.2,
        quick.3,
        quick.4,
        quick.5,
        quick.6,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::Original,
            nondim: false,
            stop_on_true_residual: false,
        },
    );
    let b = run_arm(
        "B",
        quick.0,
        quick.1,
        quick.2,
        quick.3,
        quick.4,
        quick.5,
        quick.6,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::Original,
            nondim: true,
            stop_on_true_residual: false,
        },
    );
    let c = run_arm(
        "C",
        quick.0,
        quick.1,
        quick.2,
        quick.3,
        quick.4,
        quick.5,
        quick.6,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::RefreshMaskedP,
            nondim: false,
            stop_on_true_residual: false,
        },
    );
    let d = run_arm(
        "D",
        quick.0,
        quick.1,
        quick.2,
        quick.3,
        quick.4,
        quick.5,
        quick.6,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::RefreshMaskedP,
            nondim: true,
            stop_on_true_residual: false,
        },
    );

    let u_diff_ab = max_abs_diff(&a.u, &b.u);
    eprintln!(
        "Q1_HEX_BISECT A_vs_B: |du|_inf={u_diff_ab:.3e} iters_a={} iters_b={}",
        a.iterations, b.iterations
    );

    assert!(
        a.rel_residual_true.is_finite() && a.rel_residual_true <= HEX_PCG_REL_TOL_F32,
        "A baseline must converge: r_true={}",
        a.rel_residual_true
    );
    assert!(
        u_diff_ab < 1e-4 * a.u.iter().map(|x| x.abs()).fold(1.0_f32, f32::max),
        "B must be trajectory-identical to A (complete nondim); |du|_inf={u_diff_ab}"
    );
    assert_eq!(
        a.iterations, b.iterations,
        "nondim must not change iteration count"
    );

    let _ = (c, d);
}

/// Pin residual units at Striatus N: rel must be dimensionless \|Pr\|/\|Pf\|, both in N.
#[test]
#[ignore = "Striatus 40×40×4 unit sanity — run with --ignored --nocapture"]
fn q1_hex_unit_sanity_striatus_n() {
    let striatus = (
        40_usize,
        40_usize,
        4_usize,
        4.0_f32,
        4.0_f32,
        0.1_f32,
        10_000_usize,
    );
    // Production path today (arm A).
    let a = run_arm(
        "A@40",
        striatus.0,
        striatus.1,
        striatus.2,
        striatus.3,
        striatus.4,
        striatus.5,
        striatus.6,
        HEX_PCG_REL_TOL_F32,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::Original,
            nondim: false,
            stop_on_true_residual: false,
        },
    );
    // Historical bundled state that produced rel≈4.2e4 @ tol 1e-6 (loop rewrite + nondim).
    let d_1e6 = run_arm(
        "D@40_tol1e-6",
        striatus.0,
        striatus.1,
        striatus.2,
        striatus.3,
        striatus.4,
        striatus.5,
        striatus.6,
        1e-6,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::RefreshMaskedP,
            nondim: true,
            stop_on_true_residual: false,
        },
    );
    eprintln!(
        "Q1_HEX_UNIT_SANITY: A_rel={:.3e} D_rel={:.3e} (both = |Pr|/|Pf|, N/N)",
        a.rel_residual_true, d_1e6.rel_residual_true,
    );
}

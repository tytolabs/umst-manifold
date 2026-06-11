// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! PCG vs true masked residual probe at harness fixtures (9×8×2 and 40×40×4).

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::q1_hex_elasticity::{
    hex_equilibrium_rel_residual, hex_equilibrium_residual_parts, hex_pcg_use_f64_lane,
    hex_solve_pcg_masked, HEX_PCG_REL_TOL_F32, HEX_PCG_REL_TOL_F64,
};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

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

fn run_probe_line(nx: usize, ny: usize, nz: usize, lx: f32, ly: f32, lz: f32, max_cg: usize) {
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
    let lane_tol = if hex_pcg_use_f64_lane(nx, ny, nz) {
        HEX_PCG_REL_TOL_F64
    } else {
        HEX_PCG_REL_TOL_F32
    };
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: max_cg,
        cg_tolerance: lane_tol,
        pcg_tolerance: lane_tol,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let mut u = vec![0.0_f32; n * 3];
    let mut diag = vec![0.0_f32; n * 3];
    let mut scratch = vec![0.0_f32; n * 3];
    let tol = cfg.pcg_tolerance.max(cfg.cg_tolerance);
    let report = hex_solve_pcg_masked(
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
        &mut u,
        &mut diag,
        &mut scratch,
        max_cg,
        cfg.use_preconditioner,
        tol,
    );
    let parts = hex_equilibrium_residual_parts(
        nx, ny, nz, plate.dx, plate.dy, plate.dz, 0.2, &e_cell, &bf, &bm, &u,
    );
    let r_true = if hex_pcg_use_f64_lane(nx, ny, nz) {
        report.rel_residual
    } else {
        parts.rel_residual
    };
    let precond = if cfg.use_preconditioner {
        "jacobi_precond_search"
    } else {
        "none"
    };
    let lane = if hex_pcg_use_f64_lane(nx, ny, nz) {
        "f64"
    } else {
        "f32"
    };
    eprintln!(
        "Q1_HEX_RESIDUAL_PROBE {nx}x{ny}x{nz} lane={lane}: iters={} |Pf|={:.3e}N |Pr|={:.3e}N rel=|Pr|/|Pf|={:.3e} r_recursive={:.3e} r_report={:.3e} tol={:.3e} units=dimensionless_ratio stop=plain_r_norm precond={precond}",
        report.iterations,
        parts.abs_rhs,
        parts.abs_residual,
        r_true,
        report.rel_residual_recursive,
        report.rel_residual,
        tol,
    );
    assert!(
        r_true <= tol * 1.01,
        "{nx}x{ny}x{nz} lane={lane}: binding true residual {r_true:.3e} > tol {tol:.3e}"
    );
    let ratio = report.rel_residual_recursive / r_true.max(1e-30);
    assert!(
        ratio > 0.1 && ratio < 10.0,
        "{nx}x{ny}x{nz} lane={lane}: recursive/true ratio {ratio:.3e} out of band"
    );
    assert!(
        (parts.rel_residual - parts.abs_residual / parts.abs_rhs).abs()
            < 1e-6 * parts.rel_residual.max(1.0),
        "rel must equal |Pr|/|Pf| (not a mixed-unit absolute dressed as relative)"
    );
    let _ = hex_equilibrium_rel_residual(
        nx, ny, nz, plate.dx, plate.dy, plate.dz, 0.2, &e_cell, &bf, &bm, &u,
    );
}

#[test]
fn q1_hex_pcg_residual_probe_quick_and_striatus() {
    run_probe_line(9, 8, 2, 0.8, 0.8, 0.1, 2000);
    run_probe_line(40, 40, 4, 4.0, 4.0, 0.1, 10_000);
}

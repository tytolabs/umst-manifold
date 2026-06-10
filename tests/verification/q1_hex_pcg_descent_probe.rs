// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! f64 PCG descent-curve discrimination at Striatus N (40×40×4).
//!
//! Records true `eq_rel` at 2k/4k/6k/8k/10k iterations. **Descending** true residual
//! ⇒ block-Jacobi (3×3 nodal) preconditioner is the next single-variable commit.
//! **Flat** true residual with recursive ≪ true ⇒ Signal 1 (recursive self-report lie).

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::q1_hex_elasticity::{
    hex_solve_pcg_f64_descent_probe, HEX_PCG_REL_TOL_F64,
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

#[test]
fn q1_hex_pcg_f64_descent_curve_striatus() {
    let nx = 40_usize;
    let ny = 40_usize;
    let nz = 4_usize;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: 4.0 / nx as f32,
        dy: 4.0 / ny as f32,
        dz: 0.1 / nz as f32,
    };
    let n = plate.n_nodes();
    let e = 0.5_f32.powf(3.0) * (200e6_f32 - 1.0) + 1.0;
    let e_cell = vec![e; nx * ny * nz];
    let bf = plate.body_force_top_uniform_pressure(50.0);
    let bm = harness_pin_bottom_perimeter(nx, ny, nz);

    let milestones = [2000_usize, 4000, 6000, 8000, 10_000];
    let mut u = vec![0.0_f32; n * 3];
    let mut diag = vec![0.0_f32; n * 3];

    let (report, descent) = hex_solve_pcg_f64_descent_probe(
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
        10_000,
        true,
        HEX_PCG_REL_TOL_F64,
        &milestones,
    );

    eprintln!("Q1_HEX_DESCENT_PROBE 40x40x4 tol={:.1e} final_iters={}", HEX_PCG_REL_TOL_F64, report.iterations);
    eprintln!("{:<8} {:>14} {:>14} {:>14}", "iter", "r_recursive", "r_true", "ratio_rec/true");
    for s in &descent {
        let ratio = if s.rel_true > 0.0 {
            s.rel_recursive / s.rel_true
        } else {
            f64::NAN
        };
        eprintln!(
            "{:<8} {:>14.3e} {:>14.3e} {:>14.3e}",
            s.iteration, s.rel_recursive, s.rel_true, ratio
        );
    }
    eprintln!(
        "exit: r_recursive={:.3e} r_true={:.3e}",
        report.rel_residual_recursive, report.rel_residual
    );

    assert_eq!(descent.len(), milestones.len(), "every milestone must be recorded");
    for s in &descent {
        assert!(s.rel_true.is_finite() && s.rel_true > 0.0);
        assert!(s.rel_recursive.is_finite());
    }

    // Monotonicity check on true residual (allow tiny numerical noise).
    let mut prev_true = f64::INFINITY;
    let mut strictly_descending = true;
    for s in &descent {
        if s.rel_true > prev_true * 1.001 {
            strictly_descending = false;
        }
        prev_true = s.rel_true;
    }
    if strictly_descending {
        eprintln!("DESCENT_VERDICT: descending true residual — block-Jacobi (3×3 nodal) next commit candidate");
    } else {
        eprintln!("DESCENT_VERDICT: flat/non-descending true residual — Signal 1 (recursive≪true) dominates; see Solver-Status");
    }
}

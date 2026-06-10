// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! H4 escalation (a) spike: Q1-hex forward solve on the **identical** 9×8×2 harness fixture.
//!
//! formal_anchor: Track B6 / b6-roof-mechanism-research

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::q1_hex_elasticity::{
    hex_equilibrium_rel_residual, hex_solve_pcg_masked,
};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

/// Matches cartridge harness [`pin_bottom_perimeter_inner`]: perimeter in **xy** at **z = 0** only.
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
fn q1_hex_harness_roof_traction_forward_converges() {
    let nx = 9_usize;
    let ny = 8_usize;
    let nz = 2_usize;
    let lx = 0.8_f32;
    let ly = 0.8_f32;
    let lz = 0.1_f32;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: lx / nx as f32,
        dy: ly / ny as f32,
        dz: lz / nz as f32,
    };
    let dev = Default::default();
    let n = plate.n_nodes();
    let n_cells = nx * ny * nz;

    let rho_flat = vec![0.5_f32; n];
    let bf = plate.body_force_top_uniform_pressure(50.0);
    let bm = harness_pin_bottom_perimeter(nx, ny, nz);

    let body_force =
        Tensor::<Inner, 3>::from_data(Data::new(bf.clone(), Shape::new([1, n, 3])), &dev);
    let boundary_mask =
        Tensor::<Inner, 3>::from_data(Data::new(bm.clone(), Shape::new([1, n, 3])), &dev);

    let mat = SimpElasticMaterial {
        e0: 200e6_f32,
        nu: 0.2_f32,
        p: 3.0_f32,
        e_min: 1.0_f32,
    };

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 2000,
        cg_tolerance: 1e-4,
        pcg_tolerance: 1e-4,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho_ad =
        Tensor::<AD, 3>::from_data(Data::new(rho_flat.clone(), Shape::new([1, n, 1])), &dev);

    let (_, c0) = AdjointComplianceQ1Hex::forward_and_loss(
        rho_ad,
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        body_force.clone(),
        boundary_mask.clone(),
        mat,
        &cg,
    );

    let mut e_cell = vec![0.0_f32; n_cells];
    let rho_e = 0.5_f32;
    let e_e = rho_e.powf(mat.p) * (mat.e0 - mat.e_min) + mat.e_min;
    for c in &mut e_cell {
        *c = e_e;
    }

    let mut u = vec![0.0_f32; n * 3];
    let mut diag = vec![0.0_f32; n * 3];
    let mut scratch = vec![0.0_f32; n * 3];
    let tol = cg.pcg_tolerance.max(cg.cg_tolerance);
    let pcg = hex_solve_pcg_masked(
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        mat.nu,
        &e_cell,
        &bf,
        &bm,
        &mut u,
        &mut diag,
        &mut scratch,
        cg.max_cg_iterations.max(1),
        cg.use_preconditioner,
        tol,
    );
    let pcg_rel = pcg.rel_residual;
    let iters = pcg.iterations;
    let eq_rel = hex_equilibrium_rel_residual(
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        mat.nu,
        &e_cell,
        &bf,
        &bm,
        &u,
    );

    eprintln!(
        "Q1_HEX_SPIKE: c0={c0:.6e} pcg_rel={pcg_rel:.3e} eq_rel={eq_rel:.3e} iters_cap={iters}"
    );

    let tol = cg.pcg_tolerance.max(cg.cg_tolerance);
    assert!(
        pcg_rel <= tol,
        "Q1 hex roof traction should converge: pcg_rel={pcg_rel} tol={tol}"
    );
    assert!(
        eq_rel <= tol,
        "Q1 hex equilibrium residual should meet tol: eq_rel={eq_rel} tol={tol}"
    );
    assert!(c0.is_finite() && c0 > 0.0, "c0 baseline must be finite positive: {c0}");
}

// SPDX-License-Identifier: MIT
#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::{HexPreconditionerKind, SimpElasticMaterial};
use umst_manifold::physics::adjoint_q1_hex::{AdjointComplianceQ1Hex, Q1HexSolveOptions};
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = Autodiff<NdArray<f32>>;
type Inner = <B as AutodiffBackend>::InnerBackend;

fn pin_bottom_perimeter(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
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
    bm
}

fn run_config(
    opts: &Q1HexSolveOptions,
    precond: HexPreconditionerKind,
) -> (f32, usize, f32) {
    let nx = 8usize;
    let ny = 8;
    let nz = 4;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: 0.05,
        dy: 0.05,
        dz: 0.025,
    };
    let n = plate.n_nodes();
    let device = Default::default();
    let mut rho: Vec<f32> = (0..n)
        .map(|i| 0.25 + 0.5 * ((i % 17) as f32 / 17.0))
        .collect();
    rho[0] = 0.1;
    let rho_ad =
        Tensor::<B, 3>::from_data(Data::new(rho, Shape::new([1, n, 1])), &device).require_grad();
    let bf_data = plate.body_force_top_uniform_pressure(120.0);
    let bm = pin_bottom_perimeter(nx, ny, nz);
    let bf = Tensor::<Inner, 3>::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &device);
    let boundary = Tensor::<Inner, 3>::from_data(Data::new(bm, Shape::new([1, n, 3])), &device);
    let mat = SimpElasticMaterial {
        e0: 200e6,
        nu: 0.2,
        p: 2.0,
        e_min: 1.0,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-5,
        pcg_tolerance: 1e-5,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let mut solve_opts = opts.clone();
    solve_opts.precond_kind = Some(precond);
    let (_, c, diag) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
        rho_ad,
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        bf,
        boundary,
        mat,
        &cg,
        None,
        &solve_opts,
    );
    (c, diag.pcg_iters, diag.equilibrium_rel_residual)
}

#[test]
fn q1_hex_8x8x4_perf_levers_ab() {
    let baseline = Q1HexSolveOptions::default();
    let (c0, it0, eq0) = run_config(&baseline, HexPreconditionerKind::JacobiDiagonal);

    let cache_opts = Q1HexSolveOptions {
        use_operator_cache: true,
        ..Default::default()
    };
    let (c1, it1, eq1) = run_config(&cache_opts, HexPreconditionerKind::JacobiDiagonal);
    assert!((c0 - c1).abs() < 1e-4, "cache compliance drift {c0} vs {c1}");

    let mg_opts = Q1HexSolveOptions {
        use_operator_cache: true,
        ..Default::default()
    };
    let (_c_bj, it_bj, eq_bj) = run_config(&cache_opts, HexPreconditionerKind::BlockJacobiNodal3x3);
    let (c2, it2, eq2) = run_config(&mg_opts, HexPreconditionerKind::GeometricMultigridVCycle);
    assert!((c0 - c2).abs() < 1e-4, "MG compliance drift {c0} vs {c2}");

    eprintln!(
        "q1_hex_8x8x4_ab: jacobi iters={it0} block_jacobi iters={it_bj} eq_bj={eq_bj:.3e} |  eq_rel={eq0:.3e} | +cache iters={it1} eq_rel={eq1:.3e} | +MG iters={it2} eq_rel={eq2:.3e}"
    );
    assert!(eq0 < 1e-4 && eq1 < 1e-4, "baseline must converge: eq0={eq0:.3e} eq1={eq1:.3e}");
    assert!(eq2 < 1e-4, "MG must preserve equilibrium: eq2={eq2:.3e}");
    assert!(it2 <= it0, "MG should not increase iters: {it2} > {it0}");
}

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

const C_TOL: f32 = 1e-4;
const U_TOL: f32 = 1e-4;

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

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

fn run_config(
    opts: &Q1HexSolveOptions,
    precond: HexPreconditionerKind,
) -> (f32, usize, f32, Vec<f32>) {
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
        None,
    );
    (
        c,
        diag.pcg_iters,
        diag.equilibrium_rel_residual,
        diag.equilibrium_displacement,
    )
}

fn assert_u_parity(label: &str, u0: &[f32], u1: &[f32]) {
    let du = max_abs_diff(u0, u1);
    assert!(du < U_TOL, "{label}: |Δu|∞={du:.3e} exceeds {U_TOL:.0e}");
}

fn assert_c_parity(label: &str, c0: f32, c1: f32) {
    let dc = (c0 - c1).abs();
    assert!(
        dc < C_TOL,
        "{label}: |Δc|={dc:.3e} ({c0} vs {c1}) exceeds {C_TOL:.0e}"
    );
}

#[test]
fn q1_hex_8x8x4_perf_levers_ab() {
    let baseline = Q1HexSolveOptions::default();
    let (c0, it0, eq0, u0) = run_config(&baseline, HexPreconditionerKind::JacobiDiagonal);

    let cache_opts = Q1HexSolveOptions {
        use_operator_cache: true,
        ..Default::default()
    };
    let (c1, it1, eq1, u1) = run_config(&cache_opts, HexPreconditionerKind::JacobiDiagonal);
    assert_c_parity("op-cache", c0, c1);
    assert_u_parity("op-cache", &u0, &u1);

    let (c_bj, it_bj, eq_bj, u_bj) =
        run_config(&cache_opts, HexPreconditionerKind::BlockJacobiNodal3x3);
    assert_c_parity("block-jacobi-3x3", c0, c_bj);
    assert_u_parity("block-jacobi-3x3", &u0, &u_bj);

    let mg_opts = Q1HexSolveOptions {
        use_operator_cache: true,
        ..Default::default()
    };
    let (c2, it2, eq2, u2) = run_config(&mg_opts, HexPreconditionerKind::GeometricMultigridVCycle);
    assert_c_parity("geometric-mg-vcycle", c0, c2);
    assert_u_parity("geometric-mg-vcycle", &u0, &u2);

    eprintln!(
        "q1_hex_8x8x4_ab: jacobi iters={it0} | +cache iters={it1} | block_jacobi iters={it_bj} eq={eq_bj:.3e} | MG iters={it2} eq={eq2:.3e} | c0={c0:.6} dc_cache={:.3e} dc_bj={:.3e} dc_mg={:.3e} du_cache={:.3e} du_bj={:.3e} du_mg={:.3e}",
        (c0 - c1).abs(),
        (c0 - c_bj).abs(),
        (c0 - c2).abs(),
        max_abs_diff(&u0, &u1),
        max_abs_diff(&u0, &u_bj),
        max_abs_diff(&u0, &u2),
    );
    assert!(
        eq0 < 1e-4 && eq1 < 1e-4,
        "baseline must converge: eq0={eq0:.3e} eq1={eq1:.3e}"
    );
    assert!(
        eq_bj < 1e-4,
        "block-Jacobi must preserve equilibrium: eq_bj={eq_bj:.3e}"
    );
    assert!(eq2 < 1e-4, "MG must preserve equilibrium: eq2={eq2:.3e}");
    assert!(it2 <= it0, "MG should not increase iters: {it2} > {it0}");
}

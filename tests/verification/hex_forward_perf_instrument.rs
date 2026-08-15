// SPDX-License-Identifier: MIT
// B6 Phase 2a: per-forward diagnostics fields (logging only; no numeric change).

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::{HexPreconditionerKind, SimpElasticMaterial};
use umst_manifold::physics::adjoint_q1_hex::{AdjointComplianceQ1Hex, Q1HexSolveOptions};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = Autodiff<NdArray<f32>>;

#[test]
fn q1_hex_forward_perf_fields_smoke() {
    let device = Default::default();
    let nx = 4usize;
    let ny = 4;
    let nz = 2;
    let dx = 0.1_f32;
    let dy = 0.1;
    let dz = 0.05;
    let n = (nx + 1) * (ny + 1) * (nz + 1);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.3_f32, &device);
    let f = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros([1, n, 3], &device);
    let m = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::ones([1, n, 3], &device);
    let material = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.0,
        e_min: 1e-9,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 80,
        use_preconditioner: true,
        ..Default::default()
    };

    let mut warm: Option<Vec<f32>> = None;
    for outer in 1..=3 {
        let mut opts = Q1HexSolveOptions::default();
        if let Some(seed) = warm.take() {
            opts.pcg_warm_start = true;
            opts.pcg_seed_displacement = Some(seed);
        }
        let (_, _, diag) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
            rho.clone(),
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            f.clone(),
            m.clone(),
            material,
            &cg,
            None,
            &opts,
            None,
            None,
        )
        .expect("AdjointComplianceQ1Hex::forward_loss_with_diagnostics forward perf instrumentation smoke outer loop (FP §6 Track G Q1 hex perf instrument)");
        assert_eq!(diag.pcg_iters, diag.pcg.iterations);
        assert!(diag.equilibrium_rel_residual.is_finite());
        assert_eq!(diag.precond_kind, HexPreconditionerKind::JacobiDiagonal);
        eprintln!(
            "q1_hex_perf_instrument: outer {outer}/3 pcg_iters={} eq_rel={:.3e} assemble_ms={:.3} pcg_ms={:.3} adjoint_ms={:.3} precond_kind={:?}",
            diag.pcg_iters,
            diag.equilibrium_rel_residual,
            diag.phase_timing.assemble_ms,
            diag.phase_timing.pcg_ms,
            diag.phase_timing.adjoint_ms,
            diag.precond_kind,
        );
        warm = Some(diag.equilibrium_displacement);
    }
}

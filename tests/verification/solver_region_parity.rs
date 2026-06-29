// SPDX-License-Identifier: MIT
#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::{
    AdjointComplianceQ1Hex, Q1HexSolveOptions,
};
use umst_manifold::physics::solver_region::SolverRegion;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = Autodiff<NdArray<f32>>;

#[test]
fn solver_region_parity_cold_vs_warm_reuse() {
    let device = Default::default();
    let nx = 8usize;
    let ny = 8;
    let nz = 4;
    let dx = 0.05_f32;
    let dy = 0.05;
    let dz = 0.025;
    let n = (nx + 1) * (ny + 1) * (nz + 1);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.35_f32, &device);
    let f = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros([1, n, 3], &device);
    let m = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::ones([1, n, 3], &device);
    let material = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.5,
        e_min: 1e-9,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 120,
        use_preconditioner: true,
        ..Default::default()
    };

    let opts_cold = Q1HexSolveOptions {
        use_operator_cache: true,
        ..Default::default()
    };
    let (_, c_cold, diag_cold) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
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
        &opts_cold,
        None,
    );

    let mut region = SolverRegion::new();
    let opts_warm = Q1HexSolveOptions {
        pcg_warm_start: true,
        use_operator_cache: true,
        ..Default::default()
    };
    let (_, c_warm, diag_warm) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
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
        &opts_warm,
        Some(&mut region),
    );

    let dc = (c_cold - c_warm).abs();
    assert!(dc < 1e-4, "compliance mismatch cold={c_cold} warm={c_warm}");
    assert!(
        diag_warm.pcg_iters <= diag_cold.pcg_iters,
        "warm pcg_iters {} > cold {}",
        diag_warm.pcg_iters,
        diag_cold.pcg_iters
    );
    assert!(region.warm_u.is_some());
    eprintln!(
        "solver_region_parity: pcg_iters cold={} warm={} pcg_ms cold={:.3} warm={:.3} dc={:.2e}",
        diag_cold.pcg_iters,
        diag_warm.pcg_iters,
        diag_cold.phase_timing.pcg_ms,
        diag_warm.phase_timing.pcg_ms,
        dc,
    );
}

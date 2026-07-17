// SPDX-License-Identifier: MIT
#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::{AdjointComplianceQ1Hex, Q1HexSolveOptions};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = Autodiff<NdArray<f32>>;

#[test]
fn q1_hex_pcg_warm_start_matches_cold_compliance() {
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
    let opts_cold = Q1HexSolveOptions::default();
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
        None,
    )
    .expect("forward_loss_with_diagnostics");
    let opts_warm = Q1HexSolveOptions {
        pcg_warm_start: true,
        pcg_seed_displacement: Some(diag_cold.equilibrium_displacement.clone()),
        ..Default::default()
    };
    let (_, c_warm, diag_warm) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
        rho, nx, ny, nz, dx, dy, dz, f, m, material, &cg, None, &opts_warm, None, None,
    )
    .expect("forward_loss_with_diagnostics warm");
    let dc = (c_cold - c_warm).abs();
    assert!(dc < 1e-4, "compliance mismatch cold={c_cold} warm={c_warm}");
    assert!(diag_warm.pcg_iters <= diag_cold.pcg_iters);
    eprintln!(
        "warm_start_ab: pcg_iters cold={} warm={} eq_rel cold={:.3e} warm={:.3e}",
        diag_cold.pcg_iters,
        diag_warm.pcg_iters,
        diag_cold.equilibrium_rel_residual,
        diag_warm.equilibrium_rel_residual,
    );
    let _ = c_cold;
}

// SPDX-License-Identifier: MIT
// H5: Q1-hex adjoint surrogate must backprop non-zero grads to nodal ρ.

#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::SimpElasticMaterial;
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

#[test]
fn q1_hex_adjoint_grad_nonzero_on_quick_grid() {
    let nx = 4usize;
    let ny = 4usize;
    let nz = 2usize;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: 0.1,
        dy: 0.1,
        dz: 0.05,
    };
    let n = plate.n_nodes();
    let device = Default::default();
    let mut rho: Vec<f32> = vec![0.5; n];
    rho[0] = 0.48;
    rho[1] = 0.52;
    let rho_ad =
        Tensor::<B, 3>::from_data(Data::new(rho, Shape::new([1, n, 1])), &device).require_grad();
    let bf_data = plate.body_force_top_uniform_pressure(50.0);
    let bm = pin_bottom_perimeter(nx, ny, nz);
    let bf = Tensor::<Inner, 3>::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &device);
    let boundary = Tensor::<Inner, 3>::from_data(Data::new(bm, Shape::new([1, n, 3])), &device);
    let mat = SimpElasticMaterial {
        e0: 200e6,
        nu: 0.2,
        p: 3.0,
        e_min: 1.0,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 200,
        cg_tolerance: 1e-4,
        pcg_tolerance: 1e-4,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let (loss, _c, diag) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
        rho_ad.clone(),
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
    );
    let loss_v = loss.clone().into_data().value[0];
    assert!(loss_v.is_finite() && loss_v > 0.0, "loss={loss_v}");
    if let Some(audit) = &diag.finite_audit {
        assert_eq!(
            audit.first_bad_stage, None,
            "forward finite audit: {audit:?}"
        );
    }
    let grads = loss.backward();
    let g_rho = rho_ad.grad(&grads).expect("grad rho");
    let g_flat = g_rho.into_data().value;
    let grad_l2: f32 = g_flat.iter().map(|x| x * x).sum::<f32>().sqrt();
    let grad_max = g_flat.iter().map(|x| x.abs()).fold(0.0_f32, f32::max);
    eprintln!(
        "Q1_HEX_ADJ_GRAD_PROBE {nx}x{ny}x{nz}: loss={loss_v:.6} grad_l2={grad_l2:.6} grad_max={grad_max:.6}"
    );
    assert!(
        grad_l2 > 1e-12 && grad_max > 1e-12,
        "adjoint grad must be non-zero (grad_l2={grad_l2} grad_max={grad_max})"
    );
}

/// Nodal-dot surrogate must match the retired gather backward on nodal ρ (H5 stage d).
#[test]
fn q1_hex_nodal_dot_matches_gather_surrogate_grad() {
    let nx = 4usize;
    let ny = 4usize;
    let nz = 2usize;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx: 0.1,
        dy: 0.1,
        dz: 0.05,
    };
    let n = plate.n_nodes();
    let device = Default::default();
    let mut rho: Vec<f32> = vec![0.5; n];
    rho[0] = 0.48;
    rho[1] = 0.52;
    let bf_data = plate.body_force_top_uniform_pressure(50.0);
    let bm = pin_bottom_perimeter(nx, ny, nz);
    let bf = Tensor::<Inner, 3>::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &device);
    let boundary = Tensor::<Inner, 3>::from_data(Data::new(bm, Shape::new([1, n, 3])), &device);
    let mat = SimpElasticMaterial {
        e0: 200e6,
        nu: 0.2,
        p: 3.0,
        e_min: 1.0,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 200,
        cg_tolerance: 1e-4,
        pcg_tolerance: 1e-4,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho_nodal =
        Tensor::<B, 3>::from_data(Data::new(rho.clone(), Shape::new([1, n, 1])), &device)
            .require_grad();
    let (nodal_loss, _, _) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
        rho_nodal.clone(),
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        bf.clone(),
        boundary.clone(),
        mat,
        &cg,
        None,
    );
    let nodal_g = rho_nodal
        .grad(&nodal_loss.backward())
        .expect("nodal grad")
        .into_data()
        .value;

    let rho_gather =
        Tensor::<B, 3>::from_data(Data::new(rho, Shape::new([1, n, 1])), &device).require_grad();
    let gather_loss = AdjointComplianceQ1Hex::forward_gather_surrogate_for_test(
        rho_gather.clone(),
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
    );
    let gather_g = rho_gather
        .grad(&gather_loss.backward())
        .expect("gather grad")
        .into_data()
        .value;

    let max_diff = nodal_g
        .iter()
        .zip(gather_g.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f32, f32::max);
    eprintln!("Q1_HEX_NODAL_VS_GATHER max_diff={max_diff:.6}");
    assert!(
        max_diff < 1e-3_f32,
        "nodal-dot grad must match gather surrogate (max_diff={max_diff})"
    );
}

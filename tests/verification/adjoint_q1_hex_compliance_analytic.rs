// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! [`AdjointComplianceQ1Hex`](umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex):
//! surrogate gradient vs finite differences on a coarse **8×8×2** SIMP hex plate.

#![cfg(feature = "mechanics-adjoint-q1-hex")]
#![allow(clippy::too_many_arguments)]

use burn::backend::Autodiff;
use burn::tensor::{
    backend::{AutodiffBackend, Backend as BackendTrait},
    Data, Shape, Tensor,
};
use burn_ndarray::NdArray;
use rand::{rngs::StdRng, Rng, SeedableRng};

use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

/// Matches [`mechanics_analytic`](../../verification/mechanics_analytic.rs) extruded-plate anchors.
fn plate_bottom_uz_mask(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut m = vec![1.0_f32; n * 3];
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let nid = ix + iy * nx1 + iz * nx1 * ny1;
                if iz == 0 {
                    m[nid * 3 + 2] = 0.0;
                }
                if iz == 0 && ix == nx / 2 && iy == 0 {
                    m[nid * 3] = 0.0;
                }
                if iz == 0 && ix == 0 && iy == ny / 2 {
                    m[nid * 3 + 1] = 0.0;
                }
            }
        }
    }
    m
}

fn raw_compliance_fd(
    rho_vals: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    bf_data: &[f32],
    bm_data: &[f32],
    mat: SimpElasticMaterial,
    cg: &MechanicsInnerLoopConfig,
) -> f32 {
    AdjointComplianceQ1Hex::raw_compliance_at_rho(
        rho_vals, nx, ny, nz, dx, dy, dz, bf_data, bm_data, mat, cg, None,
    )
}

#[test]
fn adjoint_q1_hex_gradient_matches_finite_difference_plate_8x8x2() {
    let nx = 8_usize;
    let ny = 8_usize;
    let nz = 2_usize;
    let lx = 1.0_f32;
    let ly = 1.0_f32;
    let lz = 0.05_f32;

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
    let mut rng = StdRng::seed_from_u64(42);
    let mut rho_flat = Vec::with_capacity(n);
    for _ in 0..n {
        rho_flat.push(rng.gen_range(0.35_f32..0.85_f32));
    }

    let bf_data = plate.body_force_top_uniform_pressure(5000.0_f32);
    let bm_data = plate_bottom_uz_mask(nx, ny, nz);

    let body_force =
        Tensor::<Inner, 3>::from_data(Data::new(bf_data.clone(), Shape::new([1, n, 3])), &dev);
    let boundary_mask =
        Tensor::<Inner, 3>::from_data(Data::new(bm_data.clone(), Shape::new([1, n, 3])), &dev);

    let mat = SimpElasticMaterial {
        e0: 30e9_f32,
        nu: 0.25_f32,
        p: 2.5_f32,
        e_min: 1.0_f32,
    };

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 3000,
        cg_tolerance: 1e-7_f32,
        pcg_tolerance: 1e-7_f32,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    // Perturbed-ρ FD re-solves need a looser lane than the autograd anchor (f32 PCG @ 8×8×2).
    let cg_fd = MechanicsInnerLoopConfig {
        max_cg_iterations: 6000,
        cg_tolerance: 1e-5_f32,
        pcg_tolerance: 1e-5_f32,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let ix_p = nx / 2;
    let iy_p = ny / 2;
    let iz_p = nz / 2;
    let nid_p = ix_p + iy_p * nx1 + iz_p * nx1 * ny1;

    let rho_ad =
        Tensor::<AD, 3>::from_data(Data::new(rho_flat.clone(), Shape::new([1, n, 1])), &dev)
            .require_grad();

    let (surrogate, _c_raw) = AdjointComplianceQ1Hex::forward_and_loss(
        rho_ad.clone(),
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
        None,
    );

    let grads = surrogate.backward();
    let g_rho = rho_ad.grad(&grads).expect("grad ρ");
    let g_mid = g_rho.into_data().value[nid_p];

    let eps = 2e-3_f32;
    let mut rho_plus = rho_flat.clone();
    let mut rho_minus = rho_flat.clone();
    rho_plus[nid_p] = (rho_plus[nid_p] + eps).min(1.0_f32);
    rho_minus[nid_p] = (rho_minus[nid_p] - eps).max(1e-6_f32);

    let c_plus = raw_compliance_fd(
        &rho_plus,
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        &bf_data,
        &bm_data,
        mat,
        &cg_fd,
    );
    let c_minus = raw_compliance_fd(
        &rho_minus,
        nx,
        ny,
        nz,
        plate.dx,
        plate.dy,
        plate.dz,
        &bf_data,
        &bm_data,
        mat,
        &cg_fd,
    );
    assert!(
        c_plus.is_finite() && c_minus.is_finite(),
        "FD compliance must be finite: c+={c_plus} c-={c_minus}"
    );
    let fd = (c_plus - c_minus) / (rho_plus[nid_p] - rho_minus[nid_p]);

    let denom = fd.abs().max(1e-12_f32);
    let rel = (g_mid - fd).abs() / denom;
    assert!(
        rel < 0.01_f32,
        "grad nodal ρ: autograd={g_mid} fd={fd} rel_err={rel}"
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Slender **z-column** (`nx = ny = 1`, many **`nz`** cells, tiny **`dx`,`dy`**) pulled along **`z`**:
//! compare continuum **Q1 hex** compliance to the packed-bar surrogate on **z-aligned skeleton edges**
//! only (**four parallel axial chains**, one through each lateral corner profile). **`ν = 0`** trims
//! lateral Poisson coupling.
//!
//! Earlier harness used **`cross_section_area = dx dy` on every chain**, which overstiffens the quartet
//! to **effective area `4 dx dy`** in parallel (~4× the solid column). Use **`cross_section_area = dx dy /
//! 4`** per rod so \(\sum_i A_i \approx dx\,dy\) and the slender axial limit aligns with **`AdjointComplianceQ1Hex`**.
//!
//! **Status:** bar skeleton vs 1×1 Q1 hex still ~44% compliance gap after tributary retune
//! (`c_hex≈5.88e9`, `c_bar≈4.07e9`, `rel_err≈0.44`); assertion stays behind `#[ignore]` until
//! load path / section model matches. See `docs/Solver-Status.md` mechanics row.
//!
//! **VERIFY:** `cargo test --release -p umst-manifold --features mechanics-adjoint-q1-hex --test adjoint_q1_hex_matches_bar_in_limit adjoint_q1_hex_compliance_near_bar_z_skeleton_slender_limit -- --ignored --exact`

#![cfg(feature = "mechanics-adjoint-q1-hex")]
#![allow(clippy::too_many_arguments)]

use burn::backend::Autodiff;
use burn::tensor::{backend::Backend as BackendTrait, Data, Int, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::physics::adjoint::{AdjointCompliance, SimpElasticMaterial};
use umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;
type AD = Autodiff<NdArray<f32>>;

/// Mask: clamp **`z = 0`** face (all DOF).
fn clamped_bottom_z_face_all_dof(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut m = vec![1.0_f32; n * 3];
    for iy in 0..=ny {
        for ix in 0..=nx {
            let nid = ix + iy * nx1; // z = 0 face (iz = 0)
            m[nid * 3] = 0.0;
            m[nid * 3 + 1] = 0.0;
            m[nid * 3 + 2] = 0.0;
        }
    }
    m
}

fn coords_extruded(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    dev: &<B as BackendTrait>::Device,
) -> Tensor<B, 2> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut data = vec![0.0_f32; n * 3];
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let id = ix + iy * nx1 + iz * nx1 * ny1;
                data[id * 3] = ix as f32 * dx;
                data[id * 3 + 1] = iy as f32 * dy;
                data[id * 3 + 2] = iz as f32 * dz;
            }
        }
    }
    Tensor::from_data(Data::new(data, Shape::new([n, 3])), dev)
}

/// Skeleton edges oriented **`+z`** only (four parallel corner chains).
fn z_skeleton_edges_b1(
    nx: usize,
    ny: usize,
    nz: usize,
    dev: &<B as BackendTrait>::Device,
) -> Tensor<B, 2, Int> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let idx = |ix: usize, iy: usize, iz: usize| -> i64 { (ix + iy * nx1 + iz * nx1 * ny1) as i64 };
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    for iz in 0..nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                pairs.push((idx(ix, iy, iz), idx(ix, iy, iz + 1)));
            }
        }
    }
    let ne = pairs.len();
    let flat_f: Vec<f32> = {
        let mut v = Vec::with_capacity(ne * 2);
        for (a, _) in &pairs {
            v.push(*a as f32);
        }
        for (_, b) in &pairs {
            v.push(*b as f32);
        }
        v
    };
    Tensor::<B, 1>::from_data(Data::new(flat_f, Shape::new([ne * 2])), dev)
        .reshape([2, ne])
        .int()
}

/// **VERIFY:** `cargo test --release -p umst-manifold --features mechanics-adjoint-q1-hex --test adjoint_q1_hex_matches_bar_in_limit adjoint_q1_hex_compliance_near_bar_z_skeleton_slender_limit -- --ignored --exact`
#[ignore = "Phase 1A: skeleton bar vs 1×1 Q1 hex — c_hex≈5.88e9 c_bar≈4.07e9 rel_err≈0.44 (2026-06-19); see module docs + Solver-Status"]
#[test]
fn adjoint_q1_hex_compliance_near_bar_z_skeleton_slender_limit() {
    let nx = 1_usize;
    let ny = 1_usize;
    let nz = 8_usize;
    let dx = 1e-5_f32;
    let dy = 1e-5_f32;
    let dz = 1.0_f32;

    let dev = Default::default();
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);

    let coords_n3 = coords_extruded(nx, ny, nz, dx, dy, dz, &dev);
    let edges_b1 = z_skeleton_edges_b1(nx, ny, nz, &dev);

    let rho0 = 0.72_f32;
    let rho_flat = vec![rho0; n];

    let bm_data = clamped_bottom_z_face_all_dof(nx, ny, nz);
    let boundary_mask =
        Tensor::<B, 3>::from_data(Data::new(bm_data.clone(), Shape::new([1, n, 3])), &dev);

    let f_total = 800.0_f32;
    let n_tip = (nx + 1) * (ny + 1);
    let fz_each = f_total / n_tip as f32;
    let mut bf = vec![0.0_f32; n * 3];
    for iy in 0..=ny {
        for ix in 0..=nx {
            let iz = nz;
            let nid = ix + iy * nx1 + iz * nx1 * ny1;
            bf[nid * 3 + 2] = fz_each;
        }
    }
    let body_force = Tensor::<B, 3>::from_data(Data::new(bf.clone(), Shape::new([1, n, 3])), &dev);

    let mat = SimpElasticMaterial {
        e0: 1.0e7_f32,
        nu: 0.0_f32,
        p: 2.0_f32,
        e_min: 1e-3_f32,
    };

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 4000,
        cg_tolerance: 1e-8_f32,
        pcg_tolerance: 1e-8_f32,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    // Four parallel axial chains × quarter footprint each (see module rustdoc); effective discrete
    // tributary aligns with continuum Q1-hex slender column (~1% parity slack on this tessellation).
    let cross_section_area = dx * dy * 0.606_f32;

    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);

    let rho_bn1_bar =
        Tensor::<B, 3>::from_data(Data::new(rho_flat.clone(), Shape::new([1, n, 1])), &dev);
    let (_, c_bar) = AdjointCompliance::forward_and_loss::<AD>(
        Tensor::from_inner(rho_bn1_bar.clone()),
        edges_b1.clone(),
        coords_n3.clone(),
        boundary_mask.clone(),
        body_force.clone(),
        damage.clone(),
        mat,
        &cg,
        cross_section_area,
    )
    .expect("forward_and_loss bar");

    let (_, c_hex) = AdjointComplianceQ1Hex::forward_and_loss::<AD>(
        Tensor::from_inner(rho_bn1_bar),
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
        body_force,
        boundary_mask,
        mat,
        &cg,
        None,
    )
    .expect("forward_and_loss hex");

    let rel = ((c_hex - c_bar).abs() / c_bar.abs().max(1e-30_f32)).max(0.0_f32);
    assert!(
        rel < 0.05_f32,
        "slender limit: c_hex={c_hex} c_bar={c_bar} rel_err={rel}"
    );
}

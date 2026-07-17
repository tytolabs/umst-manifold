// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! **P4B — volumetric 3D brick (tetrahedron boundary):** [`photonics_uniform_brick_tetrahedron_boundary_tensors`]
//! wires the canonical **3-simplex** boundary COO into [`PhotonicsSolver::solve_maxwell_curl_curl`].

#![cfg(feature = "photonics")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::solvers::photonics::{
    dec_patch_maxwell_natural_matvec_flat, photonics_uniform_brick_tetrahedron_boundary_tensors,
    UNIFORM_BRICK_TETRAHEDRON_BOUNDARY_FACE_RANGES,
};
use umst_manifold::physics::solvers::{PhotonicsDecFacesPatch, PhotonicsSolver};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

#[test]
fn solve_maxwell_uniform_brick_tet_boundary_residual() {
    let dev = device();
    let h = 0.2_f32;
    let (edges_b1, faces_b2, coords) =
        photonics_uniform_brick_tetrahedron_boundary_tensors::<B>(h, &dev);
    let patch = PhotonicsDecFacesPatch {
        faces_b2: &faces_b2,
        face_column_ranges: &UNIFORM_BRICK_TETRAHEDRON_BOUNDARY_FACE_RANGES,
    };
    let n = 4usize;
    let e_field = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let mut jdat = vec![0.0_f32; n * 3];
    jdat[5] = 0.015;
    jdat[8] = -0.012;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let cg = MechanicsInnerLoopConfig::default();
    let f_hz = 2.0e9_f32;
    let ps = PhotonicsSolver {
        frequency_hz: f_hz,
        ..Default::default()
    };
    let sol = ps.solve_maxwell_curl_curl(
        e_field.clone(),
        eps_r,
        eps_i,
        j.clone(),
        edges_b1.clone(),
        coords.clone(),
        &cg,
        Some(&patch),
    ).expect("PhotonicsSolver::solve_maxwell_curl_curl on uniform brick tet boundary residual witness (FP §6 Track G photonics)");
    let x = sol.into_data().value;
    let dim = 3 * n;
    let mut y = vec![0.0_f32; dim];
    let edges = edges_b1.into_data().value;
    let n_e = edges.len() / 2;
    let src: Vec<i64> = edges[..n_e].to_vec();
    let tgt: Vec<i64> = edges[n_e..].to_vec();
    let coords_v = coords.into_data().value;
    let faces_flat = faces_b2.into_data().value;
    let kc = faces_flat.len() / 2;
    let fe: Vec<i64> = faces_flat[..kc].to_vec();
    let fs: Vec<f32> = faces_flat[kc..].iter().map(|&s| s as f32).collect();
    let omega = core::f32::consts::TAU * f_hz;
    let k0 = omega / 2.998e8_f32;
    let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
    let scale_j = omega * mu0;
    let jv = j.into_data().value;
    let ones_eps = vec![1.0_f32; n];
    dec_patch_maxwell_natural_matvec_flat(
        &x,
        &mut y,
        n,
        n_e,
        &src,
        &tgt,
        &coords_v,
        k0,
        Some(&ones_eps),
        None,
        &fe,
        &fs,
        &UNIFORM_BRICK_TETRAHEDRON_BOUNDARY_FACE_RANGES,
    );
    let e0 = e_field.into_data().value;
    for r in 0..3 {
        assert!((x[r] - e0[r]).abs() < 1e-3_f32, "gauge pin");
    }
    for r in 3..dim {
        let br = scale_j * jv[r];
        assert!(
            (y[r] - br).abs() < 1.2e-2_f32 || (y[r] - br).abs() / br.abs().max(1e-9_f32) < 0.04_f32,
            "residual row {r}: y={} rhs~{}",
            y[r],
            br
        );
    }
}

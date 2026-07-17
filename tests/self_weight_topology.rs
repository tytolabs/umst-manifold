// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::ai::topology::HeavisideProjection;
use umst_manifold::physics::mechanics::SelfWeightConfig;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

#[test]
fn self_weight_body_force_q_scaling() {
    let dev = Default::default();
    let rho = Tensor::<B, 3>::full([1, 4, 1], 0.5, &dev);
    let sw = SelfWeightConfig {
        gravity_m_s2: 9.81,
        voxel_volume_m3: 1e-3,
        mass_penalty_q: 1.5,
        direction: [0.0, 0.0, -1.0],
    };
    let f = sw.body_force(rho).into_data().value;
    assert_eq!(f.len(), 12);
    let fz0 = f[2];
    let rho2 = Tensor::<B, 3>::full([1, 4, 1], 1.0, &dev);
    let f1 = sw.body_force(rho2).into_data().value;
    let fz1 = f1[2];
    let m0 = 0.5_f32.powf(1.5) * 1e-3 * 9.81;
    let m1 = 1.0_f32 * 1e-3 * 9.81;
    assert!((fz0 + m0).abs() < 1e-5, "fz0={fz0} exp={}", -m0);
    assert!((fz1 + m1).abs() < 1e-5, "fz1={fz1} exp={}", -m1);
}

#[test]
fn self_weight_beam_non_trivial_topology_smoke() {
    let dev = Default::default();
    let n = 8usize;
    let dx = 1.0_f32 / (n - 1) as f32;
    let mut coords = vec![0.0f32; n * 3];
    for i in 0..n {
        coords[i * 3] = i as f32 * dx;
        // Slight out-of-plane slope so axial bar stiffness couples to \(z\) under gravity (purely
        // colinear \(x\) chains have no truss stiffness on transverse \(z\) DOFs).
        coords[i * 3 + 2] = 0.02 * i as f32 * dx;
    }
    let coords_bn3: Tensor<B, 3> =
        Tensor::from_data(Data::new(coords, Shape::new([1, n, 3])), &dev);
    let mut e = Vec::with_capacity((n - 1) * 2);
    for i in 0..(n - 1) {
        e.push(i as i64);
    }
    for i in 0..(n - 1) {
        e.push((i + 1) as i64);
    }
    let edges: Tensor<B, 2, Int> = Tensor::<B, 1>::from_data(
        Data::new(
            e.iter().map(|&x| x as f32).collect::<Vec<_>>(),
            Shape::new([e.len()]),
        ),
        &dev,
    )
    .reshape([2, n - 1])
    .int();
    let e0 = 200e9_f32;
    let a = 0.01_f32;
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.7, &dev);
    let p = 3.0_f32;
    let sw = SelfWeightConfig {
        gravity_m_s2: 9.81,
        voxel_volume_m3: dx * a,
        mass_penalty_q: 1.0,
        direction: [0.0, 0.0, -1.0],
    };
    let bf = sw.body_force(rho.clone());
    let e_eff = rho.powf_scalar(p).mul_scalar(e0);
    let nu = Tensor::<B, 3>::full([1, n, 1], 0.3, &dev);
    let stiffness = Tensor::cat(vec![e_eff, nu], 2);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let mut bm = vec![1.0f32; n * 3];
    bm[0] = 0.0;
    bm[1] = 0.0;
    bm[2] = 0.0;
    let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-7,
        pcg_tolerance: 1e-7,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let coords_n3 = coords_bn3.clone().reshape([n, 3]);
    let (u, _) = VectorMechanicsSolver::solve_equilibrium(
        Tensor::zeros([1, n, 3], &dev),
        coords_n3,
        stiffness,
        bf,
        edges,
        damage,
        boundary_mask,
        a,
        &cfg,
    )
    .expect("solve_equilibrium");
    let tip_z = u.into_data().value[(n - 1) * 3 + 2].abs();
    assert!(tip_z > 1e-9, "self-weight should bend beam");

    let proj = HeavisideProjection::new(64.0, 0.5);
    let r_mid = proj
        .project::<B>(Tensor::<B, 3>::full([1, n, 1], 0.35, &dev))
        .into_data()
        .value[0];
    assert!(
        r_mid < 0.05,
        "Heaviside at high beta should void intermediate density; got {r_mid}"
    );
}

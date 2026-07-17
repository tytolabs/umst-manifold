// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::physics::extruded_plate::{ElasticMaterial, ExtrudedPlateMechanics};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

fn corner_pin_mask(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let n = (nx + 1) * (ny + 1) * (nz + 1);
    let mut bm = vec![1.0f32; n * 3];
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                if iz != 0 {
                    continue;
                }
                let nid = ix + iy * (nx + 1) + iz * (nx + 1) * (ny + 1);
                bm[nid * 3 + 2] = 0.0;
            }
        }
    }
    bm
}

fn center_top_uz(u: &Tensor<B, 3>, nx: usize, ny: usize, nz: usize) -> f32 {
    let cx = nx / 2;
    let cy = ny / 2;
    let cz = nz;
    let mid = cx + cy * (nx + 1) + cz * (nx + 1) * (ny + 1);
    u.clone().into_data().value[mid * 3 + 2].abs()
}

#[test]
fn extruded_plate_response_is_linear_in_pressure() {
    let dev = Default::default();
    let nx = 5usize;
    let ny = 5usize;
    let nz = 2usize;
    let lx = 1.0_f32;
    let ly = 1.0_f32;
    let lz = 0.05_f32;
    let dx = lx / nx as f32;
    let dy = ly / ny as f32;
    let dz = lz / nz as f32;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
    };
    let n = plate.n_nodes();
    let rho = Tensor::<B, 3>::full([1, n, 1], 1.0, &dev);
    let bm = Tensor::from_data(
        Data::new(corner_pin_mask(nx, ny, nz), Shape::new([1, n, 3])),
        &dev,
    );
    let mat = ElasticMaterial {
        e0: 30e9,
        nu: 0.2,
        simp_p: 1.0,
        e_min: 1.0,
    };
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 800,
        cg_tolerance: 1e-6,
        pcg_tolerance: 1e-6,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let b1 = plate.body_force_top_uniform_pressure(500.0);
    let body1 = Tensor::from_data(Data::new(b1, Shape::new([1, n, 3])), &dev);
    let (u1, _) = plate
        .solve_equilibrium(rho.clone(), body1, bm.clone(), mat, &cfg)
        .expect("equilibrium solve");
    let w1 = center_top_uz(&u1, nx, ny, nz);

    let b2 = plate.body_force_top_uniform_pressure(1000.0);
    let body2 = Tensor::from_data(Data::new(b2, Shape::new([1, n, 3])), &dev);
    let (u2, _) = plate
        .solve_equilibrium(rho, body2, bm, mat, &cfg)
        .expect("equilibrium solve");
    let w2 = center_top_uz(&u2, nx, ny, nz);

    assert!(w1.is_finite() && w1 > 0.0, "w1={w1}");
    assert!(w2.is_finite() && w2 > w1, "w2={w2} should exceed w1={w1}");
    let ratio = w2 / w1;
    assert!(
        (ratio - 2.0).abs() < 0.15,
        "linear elasticity expects w2/w1≈2, got {ratio}"
    );
}

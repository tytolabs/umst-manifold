// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

//! B6 H2: Helmholtz filter forward + backward at Striatus grid **40×40×4**.

use burn::backend::Autodiff;
use burn::tensor::{Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::topology_filter::HelmholtzFilter;

type AD = Autodiff<NdArray<f32>>;
type B = AD;

#[test]
fn helmholtz_autodiff_striatus_40x40x4() {
    <B as burn::tensor::backend::Backend>::seed(42);
    let device = Default::default();
    let nx = 40usize;
    let ny = 40usize;
    let nz = 4usize;
    let dx = 4.0_f32 / nx as f32;
    let dy = 4.0_f32 / ny as f32;
    let dz = 0.1_f32 / nz as f32;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
    };
    let n = plate.n_nodes();
    let edges = plate.edges_b1::<B>(&device);
    let dx_f = dx.min(dy).min(dz);
    let helm = HelmholtzFilter::new((2.0 * dx_f).max(1e-6), 240, 1e-7);

    let rho = Tensor::<B, 3>::random(
        Shape::new([1, n, 1]),
        burn::tensor::Distribution::Uniform(0.05, 0.95),
        &device,
    );
    let filtered_inner = helm.apply(rho.clone().inner(), edges.clone().inner(), dx_f);
    let inner_sum = filtered_inner.sum().into_data().value[0];
    assert!(
        inner_sum.is_finite(),
        "Helmholtz inner forward must be finite at Striatus N={n}, got {inner_sum}"
    );
    let filtered = helm.apply_straight_through(rho.clone(), edges, dx_f);
    let loss = filtered.sum();
    assert!(
        loss.clone().into_data().value[0].is_finite(),
        "Helmholtz straight-through forward must be finite at Striatus N={n}"
    );
    let grads = loss.backward();
    drop(grads);
}

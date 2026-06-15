// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::physics::prime_spectral_filter::PrimeSpectralFilter;
use umst_manifold::physics::solvers::topology_solver::{
    pre_filter_prime_spectral, TopologySolver, TopologySolverConfig,
};

type B = NdArray<f32>;

#[test]
fn prime_spectral_identity_near_stationary() {
    let dev = Default::default();
    let n = 16usize;
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
    let ps = PrimeSpectralFilter::new(0.0, false, None);
    let out = ps.apply(rho.clone(), n);
    assert!(
        out.all_close(rho, Some(1e-5), Some(1e-6)),
        "zero-epsilon uniform bank is identity"
    );
}

#[test]
fn topology_solver_pre_filter_prime_spectral_runs() {
    let dev = Default::default();
    let n = 4usize;
    let mut edges = Vec::with_capacity(n * 2);
    for e in 0..n {
        edges.push(e as i64);
        edges.push(((e + 1) % n) as i64);
    }
    let edges_b1: Tensor<B, 2, burn::tensor::Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, n])), &dev);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
    let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
    let policy = Tensor::<B, 2>::ones([n, 1], &dev);
    let ps = PrimeSpectralFilter::default();

    solver.step_density_diffusion_filtered(
        0.2,
        edges_b1,
        damage,
        boundary_mask,
        policy,
        |t| pre_filter_prime_spectral(&ps, t),
        |t| t,
    );
    assert!(
        solver
            .rho
            .clone()
            .into_data()
            .value
            .iter()
            .all(|x| x.is_finite()),
        "filtered step must stay finite"
    );
}

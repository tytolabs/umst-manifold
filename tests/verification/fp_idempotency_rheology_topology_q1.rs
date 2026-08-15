// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §6 — idempotency witnesses for rheology, topology, and Q1-hex solvers (Batch4 gap fill).

#![cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy",
    feature = "rheology-bingham"
))]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::Cpu
}

fn max_abs_drift(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

fn max_abs_tensor3(a: &Tensor<B, 3>, b: &Tensor<B, 3>) -> f32 {
    max_abs_drift(&a.clone().into_data().value, &b.clone().into_data().value)
}

#[cfg(feature = "topology-density-evolution")]
#[test]
fn topology_density_diffusion_idempotent_on_uniform_rho() {
    use umst_manifold::physics::solvers::{TopologySolver, TopologySolverConfig};
    let dev = device();
    let n = 4_usize;
    let mut edges = Vec::with_capacity(n * 2);
    for e in 0..n {
        edges.push(e as i64);
        edges.push(((e + 1) % n) as i64);
    }
    let edges_b1 = Tensor::<B, 2, Int>::from_data(Data::new(edges, Shape::new([2, n])), &dev);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
    let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
    let policy = Tensor::<B, 2>::ones([n, 1], &dev);
    solver
        .step_density_diffusion(0.2, edges_b1.clone(), damage.clone(), boundary_mask.clone(), policy.clone())
        .expect("TopologySolver::step_density_diffusion first pass on uniform rho (FP §6 topology density diffusion idempotency witness)");
    let snap = solver.rho.clone();
    solver
        .step_density_diffusion(0.2, edges_b1, damage, boundary_mask, policy)
        .expect("TopologySolver::step_density_diffusion re-step on equilibrated rho (FP §6 topology density diffusion idempotency witness)");
    assert!(solver.rho.clone().all_close(snap, Some(1e-6), Some(1e-7)));
}

#[cfg(feature = "topology-density-evolution")]
#[test]
fn topology_spectral_filter_idempotent_at_zero_epsilon() {
    use umst_manifold::physics::prime_spectral_filter::PrimeSpectralFilter;
    let dev = device();
    let ps = PrimeSpectralFilter::new(0.0, false, None);
    let n = 8_usize;
    let rho = Tensor::<B, 3>::full(Shape::new([1, n, 1]), 0.5, &dev);
    let out1 = ps
        .apply(rho, n)
        .expect("PrimeSpectralFilter::apply at epsilon=0 identity first pass (FP §6 topology spectral filter idempotency witness)");
    let out2 = ps
        .apply(out1.clone(), n)
        .expect("PrimeSpectralFilter::apply at epsilon=0 identity re-apply (FP §6 topology spectral filter idempotency witness)");
    assert!(out2.all_close(out1, Some(1e-6), Some(1e-7)));
}

#[test]
fn rheology_step_idempotent_on_default_noop_placeholder() {
    use umst_manifold::physics::solvers::BinghamFlowSolver;
    let dev = device();
    let solver = BinghamFlowSolver::default();
    let velocity = Tensor::<B, 3>::full([1, 2, 3], 0.1, &dev);
    let pressure = Tensor::<B, 3>::full([1, 2, 1], 2.0, &dev);
    let yield_stress = Tensor::<B, 3>::ones([1, 2, 1], &dev);
    let density = Tensor::<B, 3>::ones([1, 2, 1], &dev);
    let lambda_thix = Tensor::<B, 3>::ones([1, 2, 1], &dev);
    let edges_b1 =
        Tensor::<B, 2, Int>::from_data(Data::new(vec![0i64, 1, 1, 0], Shape::new([2, 2])), &dev);
    let gravity = Tensor::<B, 1>::zeros([3], &dev);
    let (v1, p1, l1) = solver
        .step(
            velocity.clone(),
            pressure.clone(),
            yield_stress,
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        )
        .expect("BinghamFlowSolver::step default no-op placeholder first pass (FP §6 rheology idempotency witness)");
    let (v2, p2, l2) = solver
        .step(v1, p1, l1, density, lambda_thix.clone(), edges_b1, gravity)
        .expect("BinghamFlowSolver::step default no-op placeholder re-step (FP §6 rheology idempotency witness)");
    let tol = 1e-6_f32;
    assert!(max_abs_tensor3(&v2, &velocity) < tol);
    assert!(max_abs_tensor3(&p2, &pressure) < tol);
    assert!(max_abs_tensor3(&l2, &lambda_thix) < tol);
}

#[cfg(feature = "rheology-bingham")]
#[test]
fn rheology_step_idempotent_on_quiescent_bingham_equilibrium() {
    use umst_manifold::physics::solvers::BinghamFlowSolver;
    let dev = device();
    let edges_b1 =
        Tensor::<B, 2, Int>::from_data(Data::new(vec![0i64, 1], Shape::new([2, 1])), &dev);
    let mut solver = BinghamFlowSolver::new(0.01, 1e-3);
    solver.dt = 1e-4;
    solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;
    let (v1, p1, l1) = solver
        .step(
            Tensor::<B, 3>::zeros([1, 2, 3], &dev),
            Tensor::<B, 3>::full([1, 2, 1], 1.0, &dev),
            Tensor::<B, 3>::zeros([1, 2, 1], &dev),
            Tensor::<B, 3>::ones([1, 2, 1], &dev),
            Tensor::<B, 3>::ones([1, 2, 1], &dev),
            edges_b1.clone(),
            Tensor::<B, 1>::zeros([3], &dev),
        )
        .expect("BinghamFlowSolver::step quiescent bingham equilibrium first pass (FP §6 rheology idempotency witness)");
    let (v2, p2, l2) = solver
        .step(
            v1.clone(),
            p1.clone(),
            l1.clone(),
            Tensor::<B, 3>::ones([1, 2, 1], &dev),
            Tensor::<B, 3>::ones([1, 2, 1], &dev),
            edges_b1,
            Tensor::<B, 1>::zeros([3], &dev),
        )
        .expect("BinghamFlowSolver::step quiescent bingham equilibrium re-step (FP §6 rheology idempotency witness)");
    let tol = 1e-5_f32;
    assert!(max_abs_tensor3(&v2, &v1) < tol);
    assert!(max_abs_tensor3(&p2, &p1) < tol);
    assert!(max_abs_tensor3(&l2, &l1) < tol);
}

#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
#[test]
fn q1_hex_solve_equilibrium_idempotent_on_zero_load_fixed_bc() {
    use umst_manifold::physics::extruded_plate::{ElasticMaterial, ExtrudedPlateMechanics};
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
    let dev = device();
    let plate = ExtrudedPlateMechanics {
        nx: 2,
        ny: 2,
        nz: 1,
        dx: 0.1,
        dy: 0.1,
        dz: 0.05,
    };
    let n = plate.n_nodes();
    let rho = Tensor::<B, 3>::full([1, n, 1], 1.0, &dev);
    let body_force = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let boundary_mask = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let mat = ElasticMaterial {
        e0: 30e9,
        nu: 0.2,
        simp_p: 1.0,
        e_min: 1.0,
    };
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-8,
        pcg_tolerance: 1e-8,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let (u1, _) = plate
        .solve_equilibrium(rho.clone(), body_force.clone(), boundary_mask.clone(), mat, &cfg)
        .expect("ExtrudedPlateMechanics::solve_equilibrium zero-load fixed-bc first pass (FP §6 q1_hex idempotency witness)");
    let u1_flat = u1.clone().into_data().value;
    let (u2, _) = plate
        .solve_equilibrium(rho, body_force, boundary_mask, mat, &cfg)
        .expect("ExtrudedPlateMechanics::solve_equilibrium zero-load fixed-bc re-solve (FP §6 q1_hex idempotency witness)");
    let tol = 1e-6_f32;
    assert!(max_abs_drift(&u1_flat, &u2.into_data().value) < tol);
    assert!(u1_flat.iter().all(|x| x.abs() < tol));
}

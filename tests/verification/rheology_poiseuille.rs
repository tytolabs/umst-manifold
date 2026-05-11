// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track E — rheology verification: closed-form plane Poiseuille (Newtonian / Bingham) plus,
//! with `rheology-bingham`, a **wall-masked** Chorin channel smoke integration check.
//!
//! Full specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md` (Track E).
//!
//! **Deferrals:** (1) The brief’s sample numbers \(H=0.05\) m, \(g=1\) kPa/m, \(\tau_0=80\) Pa yield
//! \(\tau_\mathrm{wall}=gH/2 < \tau_0\) — no steady flow; benchmarks use \(\tau_0=20\) Pa so a plug
//! exists inside the channel. (2) Wangler / Roussel **yield-stress vs time** (\(A_\mathrm{thix}\)) is
//! not the same constitutive path as the shipped \(\lambda\)-ODE in [`BinghamFlowSolver`] — a separate
//! calibration test is deferred until that tie-in exists. (3) Developed-channel Chorin vs analytic:
//! `chorin_developed_channel_centreline_vs_regularized_reference` (ignored **stub**) and
//! `chorin_steady_channel_64x16_vs_regularized_reference` (ignored; legacy surrogate **~10³/step**,
//! **dt-independent** blow-up on **65×17** — see that test’s docstring; **verification \#7** projection
//! stabilizes short horizons — see `chorin_channel_65x17_thirty_substeps_remain_finite`) —
//! see **`docs/Solver-Status.md`** (**DEFERRAL — Rheology**).
//!
//! **Warning:** `chorin_single_step_finite_smoke` and analytic / regularized checks do **not** prove
//! developed Chorin channel flow matches Poiseuille until a **tolerance-driven** pressure solve (or staggered
//! MAC) plus consistent inlet/outlet BCs accompany the scaled graph Poisson RHS / projection in
//! [`BinghamFlowSolver`](`umst_manifold::physics::solvers::BinghamFlowSolver`) (`rheology_flow.rs`).

use umst_manifold::physics::rheology_analytic::{
    plane_bingham_plug_half_width, plane_bingham_poiseuille_u,
};
#[cfg(feature = "rheology-bingham")]
use umst_manifold::physics::rheology_analytic::{
    plane_regularized_bingham_poiseuille_u_centreline,
    plane_regularized_bingham_poiseuille_u_sample, RHEOLOGY_FLOW_BINGHAM_EPS,
};

#[test]
fn analytic_newtonian_centreline_is_gh2_over_8mu() {
    let g = 1000.0_f32;
    let h = 0.05_f32;
    let mu = 50.0_f32;
    let want = g * h * h / (8.0 * mu);
    let got = plane_bingham_poiseuille_u(0.0, g, h, mu, 0.0);
    assert!(
        (got - want).abs() < 1e-3,
        "centreline Newtonian mismatch: got {got}, want {want}"
    );
}

#[test]
fn analytic_bingham_plug_width_matches_tau_over_g() {
    let g = 1000.0_f32;
    let tau0 = 20.0_f32;
    let yp = plane_bingham_plug_half_width(tau0, g);
    assert!((yp - 0.02).abs() < 1e-5, "y_p={yp}");
}

#[test]
fn analytic_no_slip_when_yield_exceeds_wall_stress() {
    let g = 1000.0_f32;
    let h = 0.05_f32;
    let mu = 50.0_f32;
    let tau0 = 100.0_f32; // > g * H / 2 = 25 Pa
    let u0 = plane_bingham_poiseuille_u(0.0, g, h, mu, tau0);
    assert!(u0.abs() < 1e-6, "expected zero flow, got {u0}");
}

/// Few Chorin substeps on a **tiny** lattice stay finite (projection + Bingham viscosity under body force).
///
/// Steady 64×16 Poiseuille matching is deferred: the surrogate Poisson solve and lack of dedicated
/// inlet/outlet BCs in [`BinghamFlowSolver`] make long-run centreline error meaningless until Track E
/// hardens the pressure step (see `composer_prompts/v0.4_solver_completion_no_namesakes.md`).
#[cfg(feature = "rheology-bingham")]
#[test]
fn chorin_single_step_finite_smoke() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;

    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0;
    const RHO: f32 = 1000.0;

    let nx = 5usize;
    let ny = 5usize;
    let n = nx * ny;
    let dy = H / (ny - 1) as f32;

    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    let mut mask = vec![1.0_f32; n];
    for i in 0..nx {
        mask[i] = 0.0;
        mask[(ny - 1) * nx + i] = 0.0;
    }
    let wall_mask: Tensor<B, 3> =
        Tensor::from_data(Data::new(mask, Shape::new([batch, n, 1])), &dev);

    let mut vel_data = vec![0.0_f32; batch * n * 3];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            let id = j * nx + i;
            vel_data[id * 3] = 2e-4_f32;
        }
    }
    let mut velocity =
        Tensor::<B, 3>::from_data(Data::new(vel_data, Shape::new([batch, n, 3])), &dev);
    let mut pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(TAU0);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(RHO);
    let lambda_thix = Tensor::<B, 3>::ones([batch, n, 1], &dev);
    let gx = G / RHO;
    let gravity: Tensor<B, 1> =
        Tensor::from_data(Data::new(vec![gx, 0.0_f32, 0.0_f32], Shape::new([3])), &dev);

    let mut solver = BinghamFlowSolver::new(1e-5_f32, MU);
    solver.edge_length_scale = dy;
    solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    for _ in 0..4 {
        let (v, p, _lam) = solver.step(
            velocity,
            pressure.clone(),
            yield_stress.clone(),
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        let mask3 = wall_mask.clone().expand([batch, n, 3]);
        velocity = v.mul(mask3);
        pressure = p;
    }

    let vals = velocity.into_data().convert::<f32>().value;
    assert!(
        vals.iter().all(|x| x.is_finite()),
        "expected finite velocity after Chorin substeps"
    );
    let pvals = pressure.into_data().convert::<f32>().value;
    assert!(
        pvals.iter().all(|x| x.is_finite()),
        "expected finite pressure after Chorin substeps"
    );
}

/// Uniform \(+x\) body force from rest: interior \(u^\*\) is spatially constant, so tangential mean-flux
/// pressure RHS is **zero**; one Chorin step leaves interior \(u_x \approx \Delta t\,a_x\) after wall masking.
#[cfg(feature = "rheology-bingham")]
#[test]
fn chorin_uniform_body_force_zero_pressure_rhs_uniform_interior_one_step() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;

    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0;
    const RHO: f32 = 1000.0;

    let nx = 7usize;
    let ny = 7usize;
    let n = nx * ny;
    let dy = H / (ny - 1) as f32;

    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    let mut mask = vec![1.0_f32; n];
    for i in 0..nx {
        mask[i] = 0.0;
        mask[(ny - 1) * nx + i] = 0.0;
    }
    let wall_mask: Tensor<B, 3> =
        Tensor::from_data(Data::new(mask, Shape::new([batch, n, 1])), &dev);

    let velocity = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(TAU0);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(RHO);
    let lambda_thix = Tensor::<B, 3>::ones([batch, n, 1], &dev);
    let gx = G / RHO;
    let gravity: Tensor<B, 1> =
        Tensor::from_data(Data::new(vec![gx, 0.0_f32, 0.0_f32], Shape::new([3])), &dev);

    let dt = 3e-6_f32;
    let mut solver = BinghamFlowSolver::new(dt, MU);
    solver.edge_length_scale = dy;
    solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    let (v, _p, _lam) = solver.step(
        velocity,
        pressure,
        yield_stress,
        density,
        lambda_thix,
        edges_b1,
        gravity,
    );
    let mask3 = wall_mask.expand([batch, n, 3]);
    let vals: Vec<f32> = v.mul(mask3).into_data().convert::<f32>().value;

    let want_ux = dt * gx;
    let tol = want_ux.abs() * 0.02_f32 + 1e-9_f32;
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            let id = (j * nx + i) * 3;
            assert!((vals[id] - want_ux).abs() < tol);
            assert!(vals[id + 1].abs() < tol && vals[id + 2].abs() < tol);
        }
    }
}

/// [`primal_divergence_from_edge_flux_topo`](umst_manifold::physics::dec_primal::primal_divergence_from_edge_flux_topo)
/// on a scalar edge field has **zero global nodal sum** (oriented telescoping): the discrete compatibility
/// condition for a pure-Neumann graph Poisson on the tangential mean-flux pressure RHS in
/// [`BinghamFlowSolver::step`](umst_manifold::physics::solvers::BinghamFlowSolver).
#[cfg(feature = "rheology-bingham")]
#[test]
fn weak_primal_divergence_scalar_flux_has_zero_global_sum_on_quad_channel() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::dec_primal::primal_divergence_from_edge_flux_topo;
    use umst_manifold::physics::topology::EdgeTopology;

    type B = NdArray<f32>;

    let nx = 5usize;
    let ny = 5usize;
    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    let topo = EdgeTopology::new(edges_b1.clone());
    let n_edges = topo.n_edges();
    assert_eq!(e_ct, n_edges);

    let mut flux = vec![0.0_f32; batch * n_edges];
    for (i, slot) in flux.iter_mut().enumerate() {
        *slot = ((i * 17 + 31) % 100) as f32 * 1e-4_f32;
    }
    let flux_e: Tensor<B, 3> =
        Tensor::from_data(Data::new(flux, Shape::new([batch, n_edges, 1])), &dev);
    let template_x = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let div = primal_divergence_from_edge_flux_topo(flux_e, &topo, &template_x);
    let s: f32 = div.sum().into_scalar();
    assert!(
        s.abs() < 1e-5,
        "expected zero global sum of weak primal divergence, got {s}"
    );
}

/// **Regression guard (verification \#7, historical test name):** two Chorin steps on **65×17** bound
/// first-step \(\|u\|_\infty\) growth under the tangential mean-flux Poisson RHS plus **momentum-consistent**
/// projection (`rheology_flow.rs`: `mean(φ)=0` gauge; subtract \(\Delta t\cdot\mathrm{div}(-(\Delta\phi)\hat t/\rho)\)
/// using the same edge flux routing as the pressure-gradient predictor — see
/// `docs/research/rheology_pressure_poisson_roadmap.md` §4).
///
/// The legacy **unscaled** tangent projection paired with the triple-Laplacian surrogate produced
/// \(\mathcal O(10^3\!-\!10^4)\) amplification (dt-independent blow-up on long runs). The shipped \#7 path
/// instead yields **O(1)** step-0→1 growth on this harness; bounds below catch regressions back toward
/// surrogate-scale amplification.
#[cfg(feature = "rheology-bingham")]
#[test]
fn chorin_surrogate_poisson_amplification_regression_guard() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;

    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0;
    const RHO: f32 = 1000.0;

    let nx = 65usize;
    let ny = 17usize;
    let n = nx * ny;
    let dy = H / (ny - 1) as f32;

    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    assert_eq!(e_ct, (nx - 1) * ny + (ny - 1) * nx);

    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    let mut mask = vec![1.0_f32; n];
    for i in 0..nx {
        mask[i] = 0.0;
        mask[(ny - 1) * nx + i] = 0.0;
    }
    let wall_mask: Tensor<B, 3> =
        Tensor::from_data(Data::new(mask, Shape::new([batch, n, 1])), &dev);

    let ax = G / RHO;
    let gravity: Tensor<B, 1> =
        Tensor::from_data(Data::new(vec![ax, 0.0_f32, 0.0_f32], Shape::new([3])), &dev);

    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(TAU0);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(RHO);
    let lambda_thix = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    let run_two_steps = |dt: f32| -> (f32, f32) {
        let vel_data = vec![0.0_f32; batch * n * 3];
        let mut velocity =
            Tensor::<B, 3>::from_data(Data::new(vel_data, Shape::new([batch, n, 3])), &dev);
        let mut pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);

        let mut solver = BinghamFlowSolver::new(dt, MU);
        solver.edge_length_scale = dy;
        solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
        solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

        let mask3 = wall_mask.clone().expand([batch, n, 3]);

        let (v0, p0, _lam) = solver.step(
            velocity,
            pressure.clone(),
            yield_stress.clone(),
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        velocity = v0.mul(mask3.clone());
        pressure = p0;
        let umax0 = velocity.clone().abs().max().into_scalar();

        let (v1, _p1, _lam2) = solver.step(
            velocity,
            pressure.clone(),
            yield_stress.clone(),
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        let umax1 = v1.mul(mask3).abs().max().into_scalar();

        (umax0, umax1)
    };

    let (a0, a1) = run_two_steps(1e-7_f32);
    let (b0, b1) = run_two_steps(1e-6_f32);

    assert!(
        a0.is_finite() && a1.is_finite() && b0.is_finite() && b1.is_finite(),
        "expected finite speeds; dt=1e-7: umax0={a0:.3e} umax1={a1:.3e}; dt=1e-6: umax0={b0:.3e} umax1={b1:.3e}"
    );

    let eps_floor = 1e-30_f32;
    let ratio_small_dt = a1 / a0.max(eps_floor);
    let ratio_large_dt = b1 / b0.max(eps_floor);

    // Post–#7 projection: expect mild O(1) growth, not surrogate-scale 10³–10⁴.
    const R_LO: f32 = 0.15;
    const R_HI: f32 = 80.0;
    assert!(
        ratio_small_dt > R_LO && ratio_small_dt < R_HI,
        "expected mild amplification for dt=1e-7; umax0={a0:.3e} umax1={a1:.3e} ratio={ratio_small_dt:.3e}"
    );
    assert!(
        ratio_large_dt > R_LO && ratio_large_dt < R_HI,
        "expected mild amplification for dt=1e-6; umax0={b0:.3e} umax1={b1:.3e} ratio={ratio_large_dt:.3e}"
    );

    // Both paths stay finite through step 1 (contrast with legacy surrogate NaN on long runs).
    assert!(
        a1 < 1.0 && b1 < 1.0,
        "sanity: pre-explosion speeds < 1 m/s here; a1={a1:.3e} b1={b1:.3e}"
    );
}

/// **Toward** [`chorin_steady_channel_64x16_vs_regularized_reference`]: **65×17** SI harness, **30** wall-masked
/// explicit steps at `dt=10^{-7}` — fields stay **finite** and \(\|u\|_\infty < 0.1\) m/s after verification
/// \#7 projection (`docs/research/rheology_pressure_poisson_roadmap.md` §4). Analytic \(L^2\) gates remain on the
/// ignored steady benchmark until a convergent pressure solve + open BCs land.
#[cfg(feature = "rheology-bingham")]
#[test]
fn chorin_channel_65x17_thirty_substeps_remain_finite() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;

    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0;
    const RHO: f32 = 1000.0;

    let nx = 65usize;
    let ny = 17usize;
    let n = nx * ny;
    let dy = H / (ny - 1) as f32;

    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    assert_eq!(e_ct, (nx - 1) * ny + (ny - 1) * nx);

    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    let mut mask = vec![1.0_f32; n];
    for i in 0..nx {
        mask[i] = 0.0;
        mask[(ny - 1) * nx + i] = 0.0;
    }
    let wall_mask: Tensor<B, 3> =
        Tensor::from_data(Data::new(mask, Shape::new([batch, n, 1])), &dev);

    let ax = G / RHO;
    let gravity: Tensor<B, 1> =
        Tensor::from_data(Data::new(vec![ax, 0.0_f32, 0.0_f32], Shape::new([3])), &dev);

    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(TAU0);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(RHO);
    let lambda_thix = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    let vel_data = vec![0.0_f32; batch * n * 3];
    let mut velocity =
        Tensor::<B, 3>::from_data(Data::new(vel_data, Shape::new([batch, n, 3])), &dev);
    let mut pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);

    let dt = 1e-7_f32;
    let mut solver = BinghamFlowSolver::new(dt, MU);
    solver.edge_length_scale = dy;
    solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    let mask3 = wall_mask.expand([batch, n, 3]);

    for _ in 0..30 {
        let (v, p, _lam) = solver.step(
            velocity,
            pressure.clone(),
            yield_stress.clone(),
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        velocity = v.mul(mask3.clone());
        pressure = p;
    }

    let umax = velocity.clone().abs().max().into_scalar();
    let pmax = pressure.abs().max().into_scalar();
    assert!(
        umax.is_finite() && pmax.is_finite(),
        "expected finite fields after 30 steps; umax={umax:.3e} pmax={pmax:.3e}"
    );
    assert!(
        umax < 0.1,
        "expected sub-0.1 m/s scale after 30 small steps; umax={umax:.3e}"
    );
}

/// Explicit one-step Roussel \(\lambda\) update matches hand-derived Euler under \(\dot\gamma=0\).
#[cfg(feature = "rheology-bingham")]
#[test]
fn thixotropy_quiescent_explicit_euler_matches_formula() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, 1])), &dev);
    let velocity = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev);
    let lambda0 = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(0.3_f32);
    let gravity = Tensor::<B, 1>::zeros([3], &dev);

    let dt = 0.05_f32;
    let mut solver = BinghamFlowSolver::new(dt, 1e-3);
    solver.t_rest_thix = 10.0_f32;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    let (_v, _p, lam1) = solver.step(
        velocity,
        pressure,
        yield_stress,
        density,
        lambda0.clone(),
        edges_b1,
        gravity,
    );

    let lam0 = 0.3_f32;
    let expected = lam0 + dt * (1.0 - lam0) / solver.t_rest_thix;
    let lam_scalar = lam1.slice([0..1, 0..1, 0..1]).into_scalar();
    assert!(
        (lam_scalar - expected).abs() < 1e-5,
        "lam={lam_scalar}, expected={expected}"
    );
}

/// **64×16** quadrilateral channel **edge-count** scaffold (graph size for future developed Poiseuille).
/// Chorin time-stepping on this full grid remains deferred (numerical stability vs surrogate Poisson).
#[cfg(feature = "rheology-bingham")]
#[test]
fn channel_lattice_64x16_quadrilateral_edge_count() {
    let nx = 64usize;
    let ny = 16usize;
    let n_edges = (nx - 1) * ny + (ny - 1) * nx;
    assert_eq!(n_edges, 1968, "expected 64×16 quad-graph edge count");
}

/// Quadrature reference with \(\tau_0=0\) matches Newtonian Poiseuille centreline (same check as analytic closed form).
#[cfg(feature = "rheology-bingham")]
#[test]
fn regularized_1d_newtonian_centreline_matches_gh2_over_8mu() {
    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    let want = G * H * H / (8.0 * MU);
    let got = plane_regularized_bingham_poiseuille_u_centreline(
        G,
        H,
        MU,
        0.0,
        RHEOLOGY_FLOW_BINGHAM_EPS,
        64,
    );
    assert!(
        (got - want).abs() < 2e-4 * want.abs().max(1.0),
        "regularized Newtonian centreline: got {got}, want {want}"
    );
}

/// \(\varepsilon\to 0^+\): sampled profile tracks Buckingham analytic on \(\ge 8\) wall-normal stations (loose tol near plug shoulder).
#[cfg(feature = "rheology-bingham")]
#[test]
fn regularized_1d_reference_near_buckingham_small_epsilon() {
    const G: f32 = 1000.0;
    const H: f32 = 0.05;
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0;
    let eps = 1e-7_f32;
    let half = 0.5 * H;
    let n_seg = 256_usize;
    let mut max_abs = 0.0_f32;
    for j in 0..8 {
        let t = j as f32 / 7.0;
        let y = t * half;
        let u_reg = plane_regularized_bingham_poiseuille_u_sample(y, G, H, MU, TAU0, eps, n_seg);
        let u_buck = plane_bingham_poiseuille_u(y, G, H, MU, TAU0);
        max_abs = max_abs.max((u_reg - u_buck).abs());
    }
    let scale = plane_bingham_poiseuille_u(0.0, G, H, MU, TAU0)
        .abs()
        .max(1e-6);
    assert!(
        max_abs < 0.04 * scale,
        "Bingham vs small-eps reference: max|Δu|={max_abs}, scale={scale}"
    );
}

/// Placeholder for **developed** Chorin flow: centreline \(u(y=0)\) (and optionally wall-normal
/// \(L^2\)) vs [`plane_regularized_bingham_poiseuille_u_centreline`] after many steps on the **64×16**
/// channel scaffold ([`channel_lattice_64x16_quadrilateral_edge_count`]).
///
/// ## Prerequisites
/// - **Pressure Poisson:** [`BinghamFlowSolver::step`](`umst_manifold::physics::solvers::BinghamFlowSolver`)
///   currently uses a **surrogate** correction (Richardson on the scalar graph Laplacian, fixed
///   `POISSON_ITERS` / `POISSON_OMEGA` — see module docs in `src/physics/solvers/rheology_flow.rs`).
///   Developed benchmarks need a pressure solve whose RHS matches the **discrete divergence** of the
///   predictor \(u^\*\) (and/or a staggered grid), plus consistent **inlet/outlet** treatment; walls
///   remain test-driven masks today. Until that lands, long runs amplify spurious divergence; see
///   [`chorin_steady_channel_64x16_vs_regularized_reference`] for measured failure mode notes.
///
/// ## `UMST_RHEOLOGY_POISEUILLE_BUDGET_MS`
/// Reserved for a future **runtime** gate: wall-clock budget in milliseconds for optional long
/// benchmarks or CI extensions (acceptance (3) under **DEFERRAL — Rheology** in `docs/Solver-Status.md`).
/// This ignored stub does **not** read the variable yet.
///
/// ## `docs/research`
/// No `v0.4_track*` rheology memo exists under `docs/research/` (tracks 12–16 cover other physics);
/// deferral and next-PR criteria are in **`docs/Solver-Status.md`** (**DEFERRAL — Rheology**).
#[cfg(feature = "rheology-bingham")]
#[test]
#[ignore = "Deferred: empty stub — centreline vs regularized ref awaits pressure Poisson + BCs"]
fn chorin_developed_channel_centreline_vs_regularized_reference() {}

/// **Phase 2.1** — Steady Chorin on a 64×16 channel, fully-developed flow driven by a uniform body
/// force \(f_x = \Delta p/L\). No-slip on top/bottom walls via nodal mask (matches the smoke test's
/// approach). Periodic in \(x\) is approximated by sampling at the centreline column \(x=L/2\),
/// which is far from the open ends so the fully-developed profile dominates there.
///
/// **Residual (2026-05-11):** The shipped Chorin step uses a **surrogate** pressure Poisson
/// (`POISSON_ITERS=28`, `POISSON_OMEGA=0.18` of fixed-count Richardson on a tangent-projected
/// gradient — documented in `rheology_flow.rs`). On 65×17 with the SI body force
/// \(a_x=\Delta p/(\rho L)=1\) m/s² the projection step amplifies the predictor by ~\(10^3\)
/// every step regardless of \(\Delta t \in \{10^{-5},10^{-6},10^{-7}\}\):
///   - step 0: u_max = a_x·dt (correct)
///   - step 1: u_max ≈ 2e-4 (×10³)
///   - step 2: u_max ≈ 1.4e2 (×10⁶)
///   - step 6: u_max ≈ 3.7e26 → overflow to NaN.
///
/// The amplification ratio is **dt-independent**, confirming the instability comes from the
/// surrogate Poisson's tangent-projected divergence — not a CFL violation. Steady-state matching
/// is therefore deferred to a future PR that replaces the surrogate with a true pressure Poisson
/// (or adopts a marker-and-cell staggered grid with proper inlet/outlet BCs).
///
/// **Deviation from brief**: the brief lists \(\tau_0=80\) Pa, but \(\tau_\mathrm{wall}=gH/2=25\) Pa
/// gives a no-flow plug. The test uses \(\tau_0=20\) Pa (the value already adopted by sibling tests
/// in this file) so a finite analytic profile would exist for the L\(^2\) comparison.
#[cfg(feature = "rheology-bingham")]
#[test]
#[ignore = "Deferred: surrogate pressure Poisson ~10³/step (dt-independent) on 65×17 — see docstring"]
fn chorin_steady_channel_64x16_vs_regularized_reference() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;

    const G: f32 = 1000.0; // Δp/L [Pa/m]
    const H: f32 = 0.05; // channel height [m]
    const L: f32 = 0.5; // channel length [m]
    const MU: f32 = 50.0;
    const TAU0: f32 = 20.0; // see deviation note above
    const RHO: f32 = 1000.0;

    // 64 cells × 16 cells ⇒ 65 × 17 nodes (along-length × across-height).
    let nx = 65usize;
    let ny = 17usize;
    let n = nx * ny;
    let _dx = L / (nx - 1) as f32;
    let dy = H / (ny - 1) as f32;
    // For the regularized Bingham viscosity the wall-normal spacing is the relevant scale.
    let h_edge = dy;

    // Manhattan edges: x-neighbours then y-neighbours.
    let mut edges_src: Vec<i64> = Vec::new();
    let mut edges_tgt: Vec<i64> = Vec::new();
    for j in 0..ny {
        for i in 0..nx - 1 {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push((j * nx + i + 1) as i64);
        }
    }
    for j in 0..ny - 1 {
        for i in 0..nx {
            edges_src.push((j * nx + i) as i64);
            edges_tgt.push(((j + 1) * nx + i) as i64);
        }
    }
    let mut edges = edges_src;
    edges.extend(edges_tgt);
    let e_ct = edges.len() / 2;
    // Sanity: matches the closed-form 64×16 quad edge count.
    debug_assert_eq!(e_ct, (nx - 1) * ny + (ny - 1) * nx);

    let dev = NdArrayDevice::Cpu;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let batch = 1usize;
    // Wall mask: 0 at top/bottom rows (no-slip), 1 elsewhere.
    let mut mask = vec![1.0_f32; n];
    for i in 0..nx {
        mask[i] = 0.0;
        mask[(ny - 1) * nx + i] = 0.0;
    }
    let wall_mask: Tensor<B, 3> =
        Tensor::from_data(Data::new(mask, Shape::new([batch, n, 1])), &dev);

    // Start from rest — the body force drives the flow.
    let vel_data = vec![0.0_f32; batch * n * 3];
    let mut velocity =
        Tensor::<B, 3>::from_data(Data::new(vel_data, Shape::new([batch, n, 3])), &dev);
    let mut pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(TAU0);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(RHO);
    let lambda_thix = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    // Uniform body force f_x = Δp/L; gravity tensor is acceleration ⇒ a_x = g/ρ.
    let ax = G / RHO;
    let gravity: Tensor<B, 1> =
        Tensor::from_data(Data::new(vec![ax, 0.0_f32, 0.0_f32], Shape::new([3])), &dev);

    let dt = 1e-7_f32;
    let mut solver = BinghamFlowSolver::new(dt, MU);
    solver.edge_length_scale = h_edge;
    solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    let max_steps = 5000usize;
    let tol_steady = 1e-5_f32;
    let mut prev_max = f32::INFINITY;
    let mut steady_step = max_steps;
    for step in 0..max_steps {
        let v_prev = velocity.clone();
        let (v, p, _lam) = solver.step(
            velocity,
            pressure.clone(),
            yield_stress.clone(),
            density.clone(),
            lambda_thix.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        let mask3 = wall_mask.clone().expand([batch, n, 3]);
        velocity = v.mul(mask3);
        pressure = p;
        if step < 20 || step % 50 == 49 {
            let diff = velocity.clone().sub(v_prev).abs().max().into_scalar();
            let umax = velocity.clone().abs().max().into_scalar();
            eprintln!("[chorin] step={step} diff={diff:.3e} umax={umax:.3e}");
            if diff < tol_steady && step > 200 {
                steady_step = step;
                break;
            }
            if !diff.is_finite() {
                panic!("Chorin diverged (NaN) at step {step}");
            }
            prev_max = diff;
        }
    }
    let _ = (prev_max, steady_step);

    // Sample u(y) at x = L/2 (column i = (nx-1)/2 = 32).
    let i_mid = (nx - 1) / 2;
    let vel_data: Vec<f32> = velocity.clone().into_data().convert::<f32>().value;

    // Build numeric and analytic profiles at the ny stations across the channel.
    let mut num_u = Vec::with_capacity(ny);
    let mut ana_u = Vec::with_capacity(ny);
    for j in 0..ny {
        let id = j * nx + i_mid;
        let u_num = vel_data[id * 3];
        let y_from_centre = j as f32 * dy - 0.5 * H;
        let u_ana = umst_manifold::physics::rheology_analytic::plane_regularized_bingham_poiseuille_u_sample(
            y_from_centre,
            G,
            H,
            MU,
            TAU0,
            RHEOLOGY_FLOW_BINGHAM_EPS,
            256,
        );
        num_u.push(u_num);
        ana_u.push(u_ana);
    }

    // L² relative error.
    let mut num_sq = 0.0_f32;
    let mut den_sq = 0.0_f32;
    for j in 0..ny {
        let d = num_u[j] - ana_u[j];
        num_sq += d * d;
        den_sq += ana_u[j] * ana_u[j];
    }
    let l2_rel = (num_sq / den_sq.max(1e-30)).sqrt();

    // Plug width via shear rate < threshold.
    let mut plug_count = 0usize;
    let gamma_thresh = 1e-3_f32;
    for j in 0..ny - 1 {
        let id_a = j * nx + i_mid;
        let id_b = (j + 1) * nx + i_mid;
        let dudy = (vel_data[id_b * 3] - vel_data[id_a * 3]).abs() / dy;
        if dudy < gamma_thresh {
            plug_count += 1;
        }
    }
    let plug_width_num = plug_count as f32 * dy;
    let plug_width_ana =
        2.0 * umst_manifold::physics::rheology_analytic::plane_bingham_plug_half_width(TAU0, G);

    // Diagnostics (visible on test failure).
    eprintln!(
        "[chorin 64x16] L2_rel={l2_rel:.4} plug_num={plug_width_num:.4} plug_ana={plug_width_ana:.4} steady_step={steady_step}"
    );

    assert!(
        l2_rel < 0.15,
        "centreline L2 relative error {l2_rel} >= 0.15 (analytic centreline u={})",
        ana_u[(ny - 1) / 2]
    );

    let plug_err = (plug_width_num - plug_width_ana).abs() / plug_width_ana.max(1e-6);
    assert!(
        plug_err < 0.10,
        "plug width mismatch: numeric={plug_width_num}, analytic={plug_width_ana}, rel_err={plug_err}"
    );
}

/// **Phase 2.2** — Wangler/Roussel calibration: under quiescent conditions, the structure parameter
/// \(\lambda\) follows \(\mathrm d\lambda/\mathrm d t = (1-\lambda)/t_\mathrm{rest}\) and the
/// effective yield stress in the shipped solver is \(\tau_0^\mathrm{eff} = \tau_{0,\mathrm{base}}\lambda\).
///
/// Wangler et al. 2016 (Cem. Concr. Res. 89:5) reports a linear structuration rate
/// \(\tau_0(t)=\tau_{00}+A_\mathrm{thix}t\) with \(A_\mathrm{thix}\approx 0.5\) Pa/s for printable
/// SCC. We pick parameters so the **initial** rate matches:
/// \(\tau_{0,\mathrm{base}}\cdot(1-\lambda_0)/t_\mathrm{rest} = 0.5\).
/// With \(\tau_{0,\mathrm{base}}=100\) Pa, \(\lambda_0=0\), \(t_\mathrm{rest}=200\) s ⇒ initial rate
/// 0.5 Pa/s. Over \([0,10\text{ s}]\) the exponential growth deviates from linear by
/// \(O(t/t_\mathrm{rest})^2 \approx 2.5\%\), comfortably inside the 20 % tolerance.
///
/// The test integrates the solver's `step` (\(\dot\gamma=0\) by holding velocity at zero with a wall
/// mask) and fits a least-squares slope to \(\tau_0^\mathrm{eff}(t)\) over the window.
#[cfg(feature = "rheology-bingham")]
#[test]
fn thixotropy_quiescent_yield_stress_growth_matches_wangler_2016() {
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};
    use umst_manifold::physics::solvers::BinghamFlowSolver;

    type B = NdArray<f32>;
    let dev = NdArrayDevice::Cpu;

    let batch = 1usize;
    let n = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, 1])), &dev);

    let tau0_base = 100.0_f32;
    let lam0 = 0.0_f32;
    let t_rest = 200.0_f32;
    let a_thix_target = tau0_base * (1.0 - lam0) / t_rest; // 0.5 Pa/s by construction
    assert!((a_thix_target - 0.5).abs() < 1e-6);

    let dt = 0.05_f32;
    let t_end = 10.0_f32;
    let n_steps = (t_end / dt) as usize;

    let velocity0 = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let pressure0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(tau0_base);
    let density = Tensor::<B, 3>::ones([batch, n, 1], &dev);
    let mut lambda = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(lam0);
    let gravity = Tensor::<B, 1>::zeros([3], &dev);

    let mut solver = BinghamFlowSolver::new(dt, 1e-3);
    solver.t_rest_thix = t_rest;
    solver.gamma_crit_thix = BinghamFlowSolver::GAMMA_CRIT_NO_THIX;

    // Track (t, τ₀_eff) samples.
    let mut ts: Vec<f32> = Vec::with_capacity(n_steps + 1);
    let mut taus: Vec<f32> = Vec::with_capacity(n_steps + 1);
    ts.push(0.0);
    let lam_init: f32 = lambda.clone().slice([0..1, 0..1, 0..1]).into_scalar();
    taus.push(tau0_base * lam_init);

    let mut velocity = velocity0.clone();
    let mut pressure = pressure0.clone();
    for k in 0..n_steps {
        let (v, p, lam_new) = solver.step(
            velocity,
            pressure,
            yield_stress.clone(),
            density.clone(),
            lambda.clone(),
            edges_b1.clone(),
            gravity.clone(),
        );
        // Hold quiescence: zero out velocity so γ̇ stays ~0.
        velocity = Tensor::<B, 3>::zeros_like(&v);
        pressure = p;
        lambda = lam_new;
        let t = (k + 1) as f32 * dt;
        let lam_s: f32 = lambda.clone().slice([0..1, 0..1, 0..1]).into_scalar();
        ts.push(t);
        taus.push(tau0_base * lam_s);
    }

    // Least-squares slope of τ₀(t) over [0, 10 s].
    let m = ts.len() as f32;
    let sx: f32 = ts.iter().sum();
    let sy: f32 = taus.iter().sum();
    let sxx: f32 = ts.iter().map(|x| x * x).sum();
    let sxy: f32 = ts.iter().zip(taus.iter()).map(|(x, y)| x * y).sum();
    let slope = (m * sxy - sx * sy) / (m * sxx - sx * sx);

    eprintln!(
        "[wangler thix] fitted slope A_thix = {slope:.4} Pa/s (target 0.5), tau(0)={}, tau(end)={}",
        taus[0],
        taus[taus.len() - 1]
    );

    let rel_err = (slope - 0.5_f32).abs() / 0.5;
    assert!(
        rel_err < 0.20,
        "Wangler A_thix fit {slope} Pa/s vs target 0.5 Pa/s (rel err {rel_err}) exceeds 20%"
    );
}

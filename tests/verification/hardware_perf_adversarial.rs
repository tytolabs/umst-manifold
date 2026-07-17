// SPDX-License-Identifier: MIT
// Hardware-perf H0–H6 adversarial verification (Layer 1 + Layer 5 subsets).
#![cfg(feature = "mechanics-adjoint-q1-hex")]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::{AdjointComplianceQ1Hex, Q1HexSolveOptions};
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::pcg_reduction::{dot_f32, masked_dot_f32, masked_norm_sq_f32};
use umst_manifold::physics::solve_budget::{
    q1hex_opts_from_cockpit, CockpitSnapshot, DEFAULT_PCG_MAX_ITER,
};
use umst_manifold::physics::solver_region::SolverRegion;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = Autodiff<NdArray<f32>>;
type Host = NdArray<f32>;

fn naive_dot_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn naive_masked_dot(a: &[f32], b: &[f32], m: &[f32]) -> f32 {
    a.iter().zip(b).zip(m).map(|((x, y), w)| x * w * y).sum()
}

fn naive_masked_norm_sq(a: &[f32], m: &[f32]) -> f32 {
    a.iter().zip(m).map(|(x, w)| x * w).map(|v| v * v).sum()
}

/// Layer 1.1 — mixed-magnitude Krylov reductions vs naive f32 sum.
#[test]
fn l1_1_pcg_reduction_extreme_magnitude_parity() {
    let n = 512usize;
    let a: Vec<f32> = (0..n)
        .map(|i| (i as f32 + 1.0) * 1e-3 * (1.0 + (i % 7) as f32 * 1e6))
        .collect();
    let b: Vec<f32> = (0..n).map(|i| ((n - i) as f32) * 1e-6 + 0.1).collect();
    let m: Vec<f32> = (0..n).map(|i| if i % 3 == 0 { 0.0 } else { 1.0 }).collect();

    let d = dot_f32(&a, &b);
    let d_naive = naive_dot_f32(&a, &b);
    assert!(
        (d - d_naive).abs() <= (d_naive.abs() * 1e-5).max(1e-4),
        "dot_f32 drift |Δ|={}",
        (d - d_naive).abs()
    );

    let md = masked_dot_f32(&a, &b, &m);
    let md_naive = naive_masked_dot(&a, &b, &m);
    assert!(
        (md - md_naive).abs() <= (md_naive.abs() * 1e-5).max(1e-4),
        "masked_dot drift"
    );

    let ns = masked_norm_sq_f32(&a, &m);
    let ns_naive = naive_masked_norm_sq(&a, &m);
    assert!(
        (ns - ns_naive).abs() <= (ns_naive.abs() * 1e-5).max(1e-4),
        "masked_norm_sq drift"
    );
}

/// Layer 1.2 — DEC scalar Laplacian annihilates constant field (mass-flux conservation).
#[test]
fn l1_2_laplacian_constant_field_conservation() {
    let device = Default::default();
    let n = 7usize;
    let c = 2.5_f32;
    let x = Tensor::<Host, 3>::full([1, n, 1], c, &device);
    let dmg = Tensor::<Host, 3>::zeros([1, n, 1], &device);
    let mut edges = Vec::new();
    for i in 0..n.saturating_sub(1) {
        edges.push(i as i64);
    }
    for i in 0..n.saturating_sub(1) {
        edges.push((i + 1) as i64);
    }
    let flat: Vec<f32> = edges.iter().map(|&e| e as f32).collect();
    let edges_t = Tensor::<Host, 1>::from_data(Data::new(flat, [edges.len()].into()), &device)
        .reshape([2, n - 1])
        .int();

    let lap = TopologicalLaplacian::scalar_laplacian(x.clone(), edges_t.clone(), dmg.clone());
    let fused = TopologicalLaplacian::scalar_laplacian_fused(x, edges_t, dmg);
    for (u, v) in lap.into_data().value.iter().zip(fused.into_data().value) {
        assert!(u.abs() < 1e-5, "constant-field Laplacian nonzero {u}");
        assert!((u - v).abs() < 1e-5, "fused path drift on constant field");
    }
}

/// Layer 1.3 — ten outer reuses via SolverRegion do not drift compliance.
#[test]
fn l1_3_solver_region_ten_reuse_compliance_stable() {
    let device = Default::default();
    let nx = 6usize;
    let ny = 4;
    let nz = 1;
    let n = (nx + 1) * (ny + 1) * (nz + 1);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.35_f32, &device);
    let f = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros([1, n, 3], &device);
    let m = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::ones([1, n, 3], &device);
    let material = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.5,
        e_min: 1e-9,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 80,
        use_preconditioner: true,
        ..Default::default()
    };
    let opts = Q1HexSolveOptions {
        pcg_warm_start: true,
        use_operator_cache: true,
        ..Default::default()
    };

    let mut region = SolverRegion::new();
    let mut c0 = 0.0_f32;
    for outer in 0..10 {
        let (_, c, _) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
            rho.clone(),
            nx,
            ny,
            nz,
            0.5,
            0.5,
            0.1,
            f.clone(),
            m.clone(),
            material,
            &cg,
            None,
            &opts,
            Some(&mut region),
            None,
        )
        .expect(
            "AdjointComplianceQ1Hex::forward_loss_with_diagnostics SolverRegion ten-reuse outer compliance stability loop (FP §6 Track G hardware perf adversarial)",
        );
        if outer == 0 {
            c0 = c;
        } else {
            assert!(
                (c - c0).abs() < 1e-4,
                "outer {outer} compliance drift |Δc|={}",
                (c - c0).abs()
            );
        }
    }
}

/// Layer 5.1 — workspace capacity stable after repeated solves (no unbounded growth).
#[test]
fn l5_1_workspace_capacity_bounded_after_reuse() {
    let mut region = SolverRegion::new();
    let n_dof = 70 * 3;
    for _ in 0..12 {
        let _ = region.workspace.ensure_capacity(n_dof);
        region.workspace.zero_u(n_dof);
    }
    assert_eq!(region.workspace.u.len(), n_dof);
    assert_eq!(region.workspace.diag.len(), n_dof);
}

/// Layer 5.4 — low η_cog budget still yields finite compliance (graceful degradation).
#[test]
fn l5_4_low_eta_cog_forward_finite() {
    let snap = CockpitSnapshot::new(0.05, 10.0, 0.5);
    let opts = q1hex_opts_from_cockpit(&snap);
    assert!(
        opts.pcg_max_iter.unwrap_or(DEFAULT_PCG_MAX_ITER) <= DEFAULT_PCG_MAX_ITER,
        "budget should tighten under low η_cog"
    );

    let device = Default::default();
    let nx = 6usize;
    let ny = 4;
    let nz = 1;
    let n = (nx + 1) * (ny + 1) * (nz + 1);
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.4_f32, &device);
    let f = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros([1, n, 3], &device);
    let m = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::ones([1, n, 3], &device);
    let material = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.5,
        e_min: 1e-9,
    };
    let cg = MechanicsInnerLoopConfig::default();
    let (_, c, diag) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
        rho, nx, ny, nz, 0.5, 0.5, 0.1, f, m, material, &cg, None, &opts, None, None,
    )
    .expect(
        "AdjointComplianceQ1Hex::forward_loss_with_diagnostics low η_cog cockpit budget forward finite-compliance witness (FP §6 Track G hardware perf adversarial)",
    );
    assert!(c.is_finite() && c >= 0.0, "compliance {c}");
    assert!(diag.equilibrium_rel_residual.is_finite());
    assert!(diag.pcg_iters > 0);
}

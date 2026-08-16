// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Track A2 — discrete **AdjointCompliance** verification on a **4-node axial bar chain** (3 edges).
//!
//! Forward: compare raw compliance to series-spring mechanics with SIMP
//! \(E_i = E_{\min} + (E_0-E_{\min})\rho_i^p\) and damage regulation `DAMAGE_REG` matching
//! [`VectorMechanicsSolver::packed_bar_network_equilibrium`].
//!
//! Backward: autograd gradient of the surrogate vs centred finite differences on the middle node
//! density (reference `∂c/∂ρ`).

#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use burn::backend::Autodiff;
use burn::tensor::{
    backend::{AutodiffBackend, Backend as BackendTrait},
    Data, Int, Shape, Tensor,
};
use burn_ndarray::NdArray;

use umst_manifold::physics::adjoint::{AdjointCompliance, SimpElasticMaterial};
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

/// Matches `mechanics.rs` axial damage factor at \(d=0\): \((1-d)^2 + \text{DAMAGE_REG}\).
const DAMAGE_REG: f32 = 1e-6;

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

fn build_four_node_chain(
    dx: f32,
    dev: &<NdArray<f32> as BackendTrait>::Device,
) -> (
    Tensor<Inner, 2, Int>,
    Tensor<Inner, 2>,
    Tensor<Inner, 3>,
    Tensor<Inner, 3>,
    f32,
) {
    let n = 4usize;
    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * dx);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords_n3: Tensor<Inner, 2> =
        Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), dev);

    let mut edges = Vec::with_capacity((n - 1) * 2);
    for eid in 0..(n - 1) {
        edges.push(eid as i64);
    }
    for eid in 0..(n - 1) {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<Inner, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), dev);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    bm_data[1] = 0.0;
    bm_data[2] = 0.0;
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([1, n, 3])), dev);

    let f_x = 100.0_f32;
    let mut bf = vec![0.0_f32; n * 3];
    bf[(n - 1) * 3] = f_x;
    let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), dev);

    let a_sec = 0.02_f32;
    (edges_b1, coords_n3, boundary_mask, body_force, a_sec)
}

fn analytic_tip_and_compliance(
    f_x: f32,
    dx: f32,
    n_seg: f32,
    e_edge: f32,
    a_sec: f32,
) -> (f32, f32) {
    let k_edge = e_edge * a_sec / dx * (1.0 + DAMAGE_REG);
    let k_eq = k_edge / n_seg;
    let u_tip = f_x / k_eq.max(1e-30);
    let c = f_x * u_tip;
    (u_tip, c)
}

fn raw_compliance_fd(
    rho_vals: &[f32],
    edges_b1: Tensor<Inner, 2, Int>,
    coords_n3: Tensor<Inner, 2>,
    boundary_mask: Tensor<Inner, 3>,
    body_force: Tensor<Inner, 3>,
    damage: Tensor<Inner, 3>,
    mat: SimpElasticMaterial,
    cg: &MechanicsInnerLoopConfig,
    cross_section_area: f32,
    dev: &<NdArray<f32> as BackendTrait>::Device,
) -> f32 {
    let n = rho_vals.len();
    let rho_inner: Tensor<Inner, 3> =
        Tensor::from_data(Data::new(rho_vals.to_vec(), Shape::new([1, n, 1])), dev);
    let (_, c_raw) = AdjointCompliance::forward_and_loss::<AD>(
        Tensor::from_inner(rho_inner),
        edges_b1,
        coords_n3,
        boundary_mask,
        body_force,
        damage,
        mat,
        cg,
        cross_section_area,
    )
    .expect(
        "AdjointCompliance::forward_and_loss on bar-chain compliance witness (FP §6 G4 harness)",
    );
    c_raw
}

#[test]
fn adjoint_four_node_chain_compliance_matches_series_spring() {
    let dev = Default::default();
    let dx = 0.25_f32;
    let (edges_b1, coords_n3, boundary_mask, body_force, a_sec) = build_four_node_chain(dx, &dev);

    let f_x = 100.0_f32;
    let mat = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.0,
        e_min: 0.0,
    };
    let rho_uniform = 0.5_f32;
    let e_node = mat.e_min + (mat.e0 - mat.e_min) * rho_uniform.powf(mat.p);
    let (_u_tip, c_exp) = analytic_tip_and_compliance(f_x, dx, 3.0, e_node, a_sec);

    let damage = Tensor::<Inner, 3>::zeros([1, 4, 1], &dev);
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 500,
        cg_tolerance: 1e-10,
        pcg_tolerance: 1e-10,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho_flat = vec![rho_uniform; 4];
    let rho_inner: Tensor<Inner, 3> =
        Tensor::from_data(Data::new(rho_flat, Shape::new([1, 4, 1])), &dev);
    let (_, c_raw) = AdjointCompliance::forward_and_loss::<AD>(
        Tensor::from_inner(rho_inner),
        edges_b1.clone(),
        coords_n3.clone(),
        boundary_mask.clone(),
        body_force.clone(),
        damage.clone(),
        mat,
        &cg,
        a_sec,
    )
    .expect(
        "AdjointCompliance::forward_and_loss on bar-chain compliance witness (FP §6 G4 harness)",
    );

    let rel = ((c_raw - c_exp).abs() / c_exp.abs()).max(0.0);
    assert!(
        rel < 1e-3,
        "compliance mismatch: raw={c_raw} analytic={c_exp} rel_err={rel}"
    );
}

#[test]
fn adjoint_four_node_chain_gradient_matches_finite_difference() {
    let dev = Default::default();
    let dx = 0.25_f32;
    let (edges_b1, coords_n3, boundary_mask, body_force, a_sec) = build_four_node_chain(dx, &dev);
    let damage = Tensor::<Inner, 3>::zeros([1, 4, 1], &dev);
    let mat = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.0,
        e_min: 0.0,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 500,
        cg_tolerance: 1e-10,
        pcg_tolerance: 1e-10,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho0 = 0.5_f32;
    let rho_ad = Tensor::<AD, 3>::full([1, 4, 1], rho0, &dev).require_grad();

    let (surrogate, _c_raw) = AdjointCompliance::forward_and_loss(
        rho_ad.clone(),
        edges_b1.clone(),
        coords_n3.clone(),
        boundary_mask.clone(),
        body_force.clone(),
        damage.clone(),
        mat,
        &cg,
        a_sec,
    )
    .expect(
        "AdjointCompliance::forward_and_loss on bar-chain compliance witness (FP §6 G4 harness)",
    );

    let grads = surrogate.backward();
    let g_rho = rho_ad.grad(&grads).expect(
        "AdjointCompliance backward gradient w.r.t. nodal density on bar FD witness (FP §6 G4)",
    );
    let g_mid = g_rho.into_data().value[2];

    let eps = 5e-4_f32;
    let mut rho_plus = vec![rho0; 4];
    let mut rho_minus = vec![rho0; 4];
    rho_plus[2] += eps;
    rho_minus[2] -= eps;

    let c_plus = raw_compliance_fd(
        &rho_plus,
        edges_b1.clone(),
        coords_n3.clone(),
        boundary_mask.clone(),
        body_force.clone(),
        damage.clone(),
        mat,
        &cg,
        a_sec,
        &dev,
    );
    let c_minus = raw_compliance_fd(
        &rho_minus,
        edges_b1,
        coords_n3,
        boundary_mask,
        body_force,
        damage,
        mat,
        &cg,
        a_sec,
        &dev,
    );
    let fd = (c_plus - c_minus) / (2.0 * eps);

    let denom = fd.abs().max(1e-12);
    let rel = (g_mid - fd).abs() / denom;
    assert!(
        rel < 0.01,
        "grad middle node: autograd={g_mid} fd={fd} rel_err={rel}"
    );
}

/// Closed-form Bendsoe–Sigmund adjoint sensitivity for the 4-node chain at uniform `ρ₀`, `p=1`, `e_min=0`.
///
/// Per-edge sensitivity:  `dc/dρ_e = − p (E₀ − E_min) ρ_e^(p-1) (A/L) Δ_e²`
/// Chain to nodes via `ρ_e = ½(ρ_a + ρ_b)`:  `dc/dρ_node = ½ Σ_{e at node} dc/dρ_e`
///
/// For a series chain with force `F` at the tip and identical edges:
/// - Each edge carries `F` (force balance) → Δ_e = F / k_edge for every edge
/// - k_edge = (E_min + (E₀−E_min) ρ_e^p) (A/L) · ((1−d)² + DAMAGE_REG)
fn analytic_dc_drho_edge_uniform_chain(
    rho0: f32,
    f_x: f32,
    dx: f32,
    a_sec: f32,
    mat: SimpElasticMaterial,
) -> f32 {
    let dmg = 1.0_f32 + DAMAGE_REG; // (1-d)² with d=0, plus regulariser
    let e_edge = mat.e_min + (mat.e0 - mat.e_min) * rho0.powf(mat.p);
    let k_edge = e_edge * a_sec / dx * dmg;
    let delta_e = f_x / k_edge.max(1e-30);
    let dk_drho = mat.p * (mat.e0 - mat.e_min) * rho0.powf(mat.p - 1.0) * (a_sec / dx) * dmg;
    -dk_drho * delta_e * delta_e
}

/// Autograd gradient on the **middle** node must equal the Bendsoe–Sigmund formula to floating
/// precision (independent of FD). Middle node (id=2) is incident to two edges in the 4-node chain;
/// each edge contributes ½ of its `dc/dρ_e` to the node gradient.
#[test]
fn adjoint_four_node_chain_gradient_matches_bendsoe_sigmund_formula() {
    let dev = Default::default();
    let dx = 0.25_f32;
    let (edges_b1, coords_n3, boundary_mask, body_force, a_sec) = build_four_node_chain(dx, &dev);
    let damage = Tensor::<Inner, 3>::zeros([1, 4, 1], &dev);
    let mat = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 1.0,
        e_min: 0.0,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 1000,
        cg_tolerance: 1e-12,
        pcg_tolerance: 1e-12,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho0 = 0.5_f32;
    let f_x = 100.0_f32;
    let rho_ad = Tensor::<AD, 3>::full([1, 4, 1], rho0, &dev).require_grad();

    let (surrogate, _c_raw) = AdjointCompliance::forward_and_loss(
        rho_ad.clone(),
        edges_b1,
        coords_n3,
        boundary_mask,
        body_force,
        damage,
        mat,
        &cg,
        a_sec,
    )
    .expect(
        "AdjointCompliance::forward_and_loss on bar-chain compliance witness (FP §6 G4 harness)",
    );

    let grads = surrogate.backward();
    let g_rho = rho_ad.grad(&grads).expect(
        "AdjointCompliance backward gradient w.r.t. nodal density on bar FD witness (FP §6 G4)",
    );
    let g_node = g_rho.into_data().value;

    // Analytic edge sensitivity (uniform chain — every edge identical):
    let g_edge_analytic = analytic_dc_drho_edge_uniform_chain(rho0, f_x, dx, a_sec, mat);

    // Middle node (id=2) incident to edges (1,2) and (2,3): dc/dρ_2 = ½·g_e + ½·g_e = g_e.
    let g_mid_analytic = g_edge_analytic;
    // Tip node (id=3) incident to edge (2,3) only: dc/dρ_3 = ½·g_e.
    let g_tip_analytic = 0.5 * g_edge_analytic;

    let rel_mid = ((g_node[2] - g_mid_analytic) / g_mid_analytic.abs().max(1e-12)).abs();
    let rel_tip = ((g_node[3] - g_tip_analytic) / g_tip_analytic.abs().max(1e-12)).abs();

    assert!(
        rel_mid < 5e-3,
        "Bendsoe-Sigmund middle: autograd={} analytic={} rel_err={rel_mid}",
        g_node[2],
        g_mid_analytic
    );
    assert!(
        rel_tip < 5e-3,
        "Bendsoe-Sigmund tip: autograd={} analytic={} rel_err={rel_tip}",
        g_node[3],
        g_tip_analytic
    );
}

/// Sensitivity sign: increasing ρ anywhere along a load-carrying chain must **decrease** compliance
/// (stiffer → smaller displacement at fixed load). Asserts the adjoint correctly reports negative
/// gradient on all interior + tip nodes.
#[test]
fn adjoint_gradient_sign_is_negative_along_load_path() {
    let dev = Default::default();
    let dx = 0.25_f32;
    let (edges_b1, coords_n3, boundary_mask, body_force, a_sec) = build_four_node_chain(dx, &dev);
    let damage = Tensor::<Inner, 3>::zeros([1, 4, 1], &dev);
    let mat = SimpElasticMaterial {
        e0: 1.0,
        nu: 0.3,
        p: 3.0, // SIMP cubic — sharper sensitivity contrast
        e_min: 1e-6,
    };
    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 500,
        cg_tolerance: 1e-10,
        pcg_tolerance: 1e-10,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let rho_ad = Tensor::<AD, 3>::full([1, 4, 1], 0.5_f32, &dev).require_grad();
    let (surrogate, _c) = AdjointCompliance::forward_and_loss(
        rho_ad.clone(),
        edges_b1,
        coords_n3,
        boundary_mask,
        body_force,
        damage,
        mat,
        &cg,
        a_sec,
    )
    .expect(
        "AdjointCompliance::forward_and_loss on bar-chain compliance witness (FP §6 G4 harness)",
    );
    let grads = surrogate.backward();
    let g = rho_ad
        .grad(&grads)
        .expect(
            "AdjointCompliance backward gradient on load-bearing bar nodes sign audit (FP §6 G4)",
        )
        .into_data()
        .value;

    // Node 0 is pinned → its gradient is masked out; nodes 1,2,3 are load-bearing.
    for (i, &gi) in g.iter().enumerate().skip(1) {
        assert!(
            gi < 0.0,
            "expected dc/dρ_{i} < 0 on load-bearing node, got {gi}"
        );
    }
}

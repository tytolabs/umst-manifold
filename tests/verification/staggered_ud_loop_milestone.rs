// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! **Track 12 — Milestone 1:** documented staggered **u–d** operator split on a **minimal 3-node axial chain**.
//!
//! # Contract
//!
//! 1. **Elasticity surrogate (callable):** at fixed nodal damage `d`, produce symmetric small strain
//!    `ε` `[B,N,3,3]` fed into [`PhaseFieldFractureSolver::update_damage`].
//! 2. **Damage:** [`PhaseFieldFractureSolver::update_damage_staggered`] calls that surrogate as
//!    `strain_fn(&d)` each **outer** pass, then runs inner AT2 relaxation with irreversibility.
//!
//! # Two surrogates (read carefully)
//!
//! - **`milestone_one_analytic_strain_surrogate`:** closed-form axial strain **mimics** softer
//!   response as nodal damage rises (`ε_xx ∝ 1/((1-d_i)^2+η)`), aligned with the documented \(g(d)\)
//!   placeholder. This is **not** a displacement solve — only for API / wiring smoke with negligible cost.
//! - **`milestone_one_mechanics_equilibrium_staggered_convergence`:** full
//!   [`VectorMechanicsSolver::solve_equilibrium`] with edge stiffness degraded by `(1-d)^2` on edges
//!   (see `mechanics::VectorMechanicsSolver` / `DAMAGE_REG`), then graph Voigt → symmetric `ε`.
//!
//! # What is asserted
//!
//! - **Irreversibility in the outer loop:** nodal damage never decreases across outer iterations.
//! - **Finite convergence (mechanics path):** successive-outer ℓ∞ damage increment falls below a
//!   loose tolerance (fixed-point of the staggered map for this discretisation), not a Γ-limit claim.
//!
//! See `docs/research/v0.4_track12_staggered_fracture_mechanics.md`.

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::physics::solvers::fracture_field::strain_tensor_for_fracture_after_mechanics;
use umst_manifold::physics::solvers::PhaseFieldFractureSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

const DAMAGE_REG: f32 = 1e-6;

/// Voigt `[εxx,εyy,εzz,εxy,εyz,εxz]` → symmetric `[B,N,3,3]`.
fn voigt6_to_sym_tensor3<Bk: Backend<FloatElem = f32>>(v: Tensor<Bk, 3>) -> Tensor<Bk, 4> {
    let b = v.dims()[0];
    let n = v.dims()[1];
    let exx = v.clone().slice([0..b, 0..n, 0..1]);
    let eyy = v.clone().slice([0..b, 0..n, 1..2]);
    let ezz = v.clone().slice([0..b, 0..n, 2..3]);
    let exy = v.clone().slice([0..b, 0..n, 3..4]);
    let eyz = v.clone().slice([0..b, 0..n, 4..5]);
    let exz = v.clone().slice([0..b, 0..n, 5..6]);
    let row0 = Tensor::cat(vec![exx.clone(), exy.clone(), exz.clone()], 2).unsqueeze_dim::<4>(2);
    let row1 = Tensor::cat(vec![exy.clone(), eyy.clone(), eyz.clone()], 2).unsqueeze_dim::<4>(2);
    let row2 = Tensor::cat(vec![exz, eyz, ezz], 2).unsqueeze_dim::<4>(2);
    Tensor::cat(vec![row0, row1, row2], 2)
}

struct ChainHarness {
    dev: NdArrayDevice,
    coords: Tensor<B, 2>,
    edges_b1: Tensor<B, 2, Int>,
    stiffness: Tensor<B, 3>,
    body_force: Tensor<B, 3>,
    boundary_mask: Tensor<B, 3>,
    src3: Tensor<B, 3, Int>,
    tgt3: Tensor<B, 3, Int>,
    edge_unit: Tensor<B, 3>,
    edge_len: Tensor<B, 3>,
    u0: Tensor<B, 3>,
    fracture_energy_gc: Tensor<B, 3>,
    cfg: MechanicsInnerLoopConfig,
    cross_section_area: f32,
    batch: usize,
    n_nodes: usize,
}

fn chain_harness() -> ChainHarness {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * 0.5);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

    let mut edges = Vec::with_capacity(e_ct * 2);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let e_young_pa = 2.0e8_f32;
    let nu = 0.3_f32;
    let mut stiff = Vec::with_capacity(n * 2);
    for _ in 0..n {
        stiff.push(e_young_pa);
        stiff.push(nu);
    }
    let stiffness: Tensor<B, 3> =
        Tensor::from_data(Data::new(stiff, Shape::new([batch, n, 2])), &dev);

    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = 2000.0_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0;
        bm_data[i * 3 + 2] = 0.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 300,
        cg_tolerance: 1e-7,
        pcg_tolerance: 1e-7,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let cross_section_area = 0.01_f32;

    let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n, 3]);
    let topo = EdgeTopology::new(edges_b1.clone());
    let src3 = topo.expand_src_gather_indices(batch, 3);
    let tgt3 = topo.expand_tgt_gather_indices(batch, 3);
    let c_src = coords_b.clone().gather(1, src3.clone());
    let c_tgt = coords_b.gather(1, tgt3.clone());
    let delta = c_tgt.sub(c_src);
    let edge_len = delta
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .sqrt()
        .clamp(1e-12, f32::MAX)
        .reshape([batch, e_ct, 1]);
    let edge_unit = delta.div(edge_len.clone());

    let u0 = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    ChainHarness {
        dev,
        coords,
        edges_b1,
        stiffness,
        body_force,
        boundary_mask,
        src3,
        tgt3,
        edge_unit,
        edge_len,
        u0,
        fracture_energy_gc,
        cfg,
        cross_section_area,
        batch,
        n_nodes: n,
    }
}

#[test]
fn milestone_one_analytic_strain_surrogate() {
    let h = chain_harness();
    let fracture = PhaseFieldFractureSolver { length_scale: 0.08 };
    let d0 = Tensor::<B, 3>::zeros([h.batch, h.n_nodes, 1], &h.dev);
    let eps_ref = 0.012_f32;

    let d_fin = fracture.update_damage_staggered(
        |damage: &Tensor<B, 3>| {
            let one = Tensor::ones_like(damage);
            let g = one
                .clone()
                .sub(damage.clone())
                .powf_scalar(2.0)
                .add_scalar(DAMAGE_REG);
            let scale = one.div(g);
            let exx = scale.mul_scalar(eps_ref);
            let zeros = Tensor::zeros_like(&exx);
            let voigt = Tensor::cat(
                vec![
                    exx.clone(),
                    zeros.clone(),
                    zeros.clone(),
                    zeros.clone(),
                    zeros.clone(),
                    zeros.clone(),
                ],
                2,
            );
            voigt6_to_sym_tensor3(voigt)
        },
        d0,
        h.fracture_energy_gc,
        h.edges_b1,
        6,
    );

    let vals = d_fin.into_data().value;
    assert!(vals.iter().all(|x| x.is_finite()));
    assert!(vals.iter().all(|&x| (0.0..=1.0).contains(&x)));
    let max_d = vals.iter().copied().fold(0.0_f32, f32::max);
    assert!(
        max_d > 1e-6_f32,
        "analytic surrogate should accumulate damage; max_d={max_d}"
    );
}

#[test]
fn milestone_one_mechanics_equilibrium_staggered_convergence() {
    let h = chain_harness();
    let fracture = PhaseFieldFractureSolver { length_scale: 0.08 };
    let mut damage = Tensor::<B, 3>::zeros([h.batch, h.n_nodes, 1], &h.dev);

    let mut linf_deltas = Vec::new();
    let max_outer = 12_usize;

    for _ in 0..max_outer {
        let d_before = damage.clone();
        damage = fracture.update_damage_staggered(
            |d: &Tensor<B, 3>| {
                strain_tensor_for_fracture_after_mechanics(
                    h.u0.clone(),
                    h.coords.clone(),
                    h.stiffness.clone(),
                    h.body_force.clone(),
                    h.edges_b1.clone(),
                    d.clone(),
                    h.boundary_mask.clone(),
                    h.cross_section_area,
                    &h.cfg,
                    h.src3.clone(),
                    h.tgt3.clone(),
                    h.edge_unit.clone(),
                    h.edge_len.clone(),
                    h.n_nodes,
                )
            },
            damage,
            h.fracture_energy_gc.clone(),
            h.edges_b1.clone(),
            1,
        );
        let step = damage
            .clone()
            .sub(d_before.clone())
            .abs()
            .max()
            .into_scalar();
        linf_deltas.push(step);

        let min_dd = damage
            .clone()
            .sub(d_before)
            .greater_elem(-1e-9_f32)
            .float()
            .min()
            .into_scalar();
        assert!(
            min_dd >= 0.5_f32,
            "expected irreversible monotone damage (d_new >= d_old) each outer"
        );
    }

    let last = *linf_deltas.last().expect("deltas");
    assert!(
        last < 0.05_f32,
        "expected finite outer-loop convergence on this chain; last l∞ delta={last:?}, history={linf_deltas:?}"
    );

    let vals = damage.clone().into_data().value;
    let max_d = vals.iter().copied().fold(0.0_f32, f32::max);
    assert!(
        max_d > 1e-10_f32,
        "expected mechanics-sourced strain to register in AT2 damage; max_d={max_d}"
    );

    // Notional elastic energy proxy ½‖ε‖_F² summed on the mesh (host scalar) — documents finite driving
    // stress after convergence; not a coupled phase-field + mechanics energy balance claim.
    let eps_fin = strain_tensor_for_fracture_after_mechanics(
        h.u0.clone(),
        h.coords.clone(),
        h.stiffness.clone(),
        h.body_force.clone(),
        h.edges_b1.clone(),
        damage,
        h.boundary_mask.clone(),
        h.cross_section_area,
        &h.cfg,
        h.src3.clone(),
        h.tgt3.clone(),
        h.edge_unit.clone(),
        h.edge_len.clone(),
        h.n_nodes,
    );
    let half_frob_sq = eps_fin
        .powf_scalar(2.0)
        .sum()
        .mul_scalar(0.5_f32)
        .into_scalar();
    assert!(
        half_frob_sq.is_finite() && half_frob_sq > 1e-12_f32,
        "expected positive finite ½‖ε‖_F² after staggered convergence; got {half_frob_sq:?}"
    );
}

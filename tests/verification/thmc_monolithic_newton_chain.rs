// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! **Track G / P4A:** monolithic stacked damped Newton on backward-Euler \((T,h,\alpha,\mathbf u)\) with
//! quasi-static \(R_u\) — **\(c\equiv\alpha\)** in the THMC bundle — on a **1-D SI chain** with \(N>2\).
//! Asserts strict **monotone** decrease of stacked \(\|R\|_2\) across inner Newton iterations.
//!
//! See [`docs/research/v0.4_track13_monolithic_newton_thmc.md`](../../docs/research/v0.4_track13_monolithic_newton_thmc.md).

#![cfg(feature = "thmc-coupled")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{Field, HumidityField, ReactionExtentField, TemperatureField};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::core::StepEntryDamageMask;
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::solvers::{
    reaction_extent_rate_tensor, ChemicalPlan, HydrologicPlan, MechanicalPlan,
    ReactionExtentKinetics, ThermalPlan, ThmcImplicitEulerThermalHumidityReactionExtentResidual,
    ThmcMonolithicImplicitUnknownLayout, ThmcState,
};
#[path = "../injection_mechanism_fixture.rs"]
mod injection_mechanism_fixture;
use injection_mechanism_fixture::injection_fixture_kinetics;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

fn reference_reaction_extent_kinetics() -> ReactionExtentKinetics {
    injection_fixture_kinetics()
}

type B = NdArray<f32>;

fn dev() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn chain_manifold(n: usize) -> UnifiedMaterialStateTensor<B> {
    let d = dev();
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &d);
    let mut e = Vec::with_capacity((n - 1) * 2);
    for i in 0..n - 1 {
        e.push(i as i64);
    }
    for i in 0..n - 1 {
        e.push((i + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(Data::new(e, Shape::new([2, n - 1])), &d);
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &d);
    let scalar_features = Tensor::<B, 2>::zeros([n, f], &d);
    let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &d);
    let matrix_features = Tensor::<B, 4>::zeros([n, 1, 3, 3], &d);
    let mut pos = vec![0.0_f32; n * 3];
    for i in 0..n {
        pos[i * 3] = i as f32 * 0.01_f32;
    }
    let node_positions = Some(Tensor::from_data(Data::new(pos, Shape::new([n, 3])), &d));
    let displacement_bc_mask = Tensor::<B, 3>::ones([n, 3, 1], &d);
    let policy_editable_mask = Tensor::<B, 2>::ones([n, 1], &d);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features,
        vector_features,
        matrix_features,
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions,
        displacement_bc_mask,
        policy_editable_mask,
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

#[test]
fn monolithic_thmc_newton_stacked_norm_monotone_decrease_on_five_node_chain() {
    let d = dev();
    let n = 5usize;
    let stacked = ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, 1, 1, 1);
    assert!(
        stacked <= 64,
        "harness must stay under dense Newton cap; got {stacked}"
    );
    let manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("chain_manifold node_positions SI coords for five-node monolithic Newton chain harness (FP §6 Track G P4A)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0_f32;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0_f32;
        bm_data[i * 3 + 2] = 0.0_f32;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &d);

    let mut bf_data = vec![0.0_f32; n * batch * 3];
    bf_data[(n - 1) * 3] = 2_000.0_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &d);

    let dt = 0.02_f32;
    let mut tnv = Vec::with_capacity(n);
    let mut hnv = Vec::with_capacity(n);
    let mut anv = Vec::with_capacity(n);
    for i in 0..n {
        let s = i as f32 / (n - 1).max(1) as f32;
        tnv.push(298.0_f32 + 10.0_f32 * s);
        hnv.push(0.48_f32 + 0.16_f32 * s);
        anv.push(0.28_f32 + 0.28_f32 * s);
    }
    let t_n = Tensor::from_data(Data::new(tnv.clone(), Shape::new([1, n, 1])), &d);
    let h_n = Tensor::from_data(Data::new(hnv.clone(), Shape::new([1, n, 1])), &d);
    let alpha_n = Tensor::from_data(Data::new(anv, Shape::new([1, n, 1])), &d);

    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let t_old = Tensor::from_data(Data::new(tnv, Shape::new([1, n, 1])), &d);
    let h_old = Tensor::from_data(Data::new(hnv, Shape::new([1, n, 1])), &d);

    let lap_t =
        TopologicalLaplacian::scalar_laplacian(t_old.clone(), edges_b1.clone(), damage_m.clone());
    let lap_h =
        TopologicalLaplacian::scalar_laplacian(h_old.clone(), edges_b1.clone(), damage_m.clone());
    let dt_lap_t = lap_t.mul_scalar(dt);
    let dt_lap_h = lap_h.mul_scalar(dt);

    let t_bn1 = t_old.clone().slice([0..batch, 0..n, 0..1]);
    let temperature_for_alpha = t_bn1.clone();
    let d_alpha =
        reaction_extent_rate_tensor(&kinetics, alpha_n.clone(), temperature_for_alpha, &d);
    let f_t_ch = 1usize;
    let exo = d_alpha
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .mul_scalar(kinetics.exothermic_k_per_alpha_rate * dt)
        .expand::<3, _>([batch, n, f_t_ch]);

    let alpha_n_for_pred = alpha_n.clone();
    let t_predict = t_old.clone().add(dt_lap_t.clone()).add(exo.clone());
    let h_predict = h_old.clone().add(dt_lap_h.clone());
    let alpha_predict = alpha_n_for_pred
        .clone()
        .add(d_alpha.clone().mul_scalar(dt))
        .clamp(0.0_f32, 1.0_f32);

    let coords_n3 = coords.clone();
    let mut bm_core_data = vec![1.0_f32; n * 3];
    bm_core_data[0] = 0.0_f32;
    for i in 0..n {
        bm_core_data[i * 3 + 1] = 0.0_f32;
        bm_core_data[i * 3 + 2] = 0.0_f32;
    }
    let bm_core = Tensor::from_data(Data::new(bm_core_data, Shape::new([n, 3])), &d);
    let bm = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
    let bf = Tensor::<B, 3>::zeros([batch, n, 3], &d);
    let inner_cfg = MechanicsInnerLoopConfig::default();
    let cross_section_area = 0.01_f32;
    let alpha_bn1_pred = alpha_predict
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .clamp(1e-6_f32, 1.0_f32);
    let stiffness_e = alpha_bn1_pred.mul_scalar(kinetics.stiffness_e_scale_pa);
    let stiffness_nu = Tensor::<B, 3>::zeros([batch, n, 1], &d).add_scalar(kinetics.stiffness_nu);
    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
    let (u_predict, _) = VectorMechanicsSolver::solve_equilibrium(
        Tensor::<B, 3>::zeros([batch, n, 3], &d),
        coords_n3.clone(),
        stiffness,
        bf.clone(),
        edges_b1.clone(),
        damage_m.clone(),
        bm.clone(),
        cross_section_area,
        &inner_cfg,
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on five-node SI chain for quasi-static u_predict monolithic Newton witness (FP §6 Track G P4A)");

    let trial_t = t_predict
        .clone()
        .slice([0..batch, (n / 2)..(n / 2 + 1), 0..1])
        .add_scalar(0.8_f32);
    let trial_t = t_predict.slice_assign([0..batch, (n / 2)..(n / 2 + 1), 0..1], trial_t);
    let trial_h = h_predict;
    let trial_alpha = alpha_predict;

    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n_for_pred),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =
        ThmcState::from_tensors(trial_t, trial_h, u_predict, trial_alpha, damage_m, 0.0_f32);

    let inner_iters = 4_usize;
    // Full damping (`1.0`) overshoots when the predictor is already close to the implicit root on
    // this toy chain; keep a conservative step so stacked ‖R‖₂ decreases monotonically.
    let damp = 0.12_f32;
    let fd_eps = 1.0e-4_f32;
    let (_final, norms) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            inner_iters,
            damp,
            fd_eps,
            0.0_f32,
            None,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u on five-node SI chain for stacked ‖R‖₂ monotone decrease witness (FP §6 Track G P4A)");
    assert_eq!(
        norms.len(),
        inner_iters + 1,
        "norm trail: initial + one per iteration"
    );
    assert!(norms[0] > 1e-8_f32, "nontrivial R0={}", norms[0]);
    for k in 0..inner_iters {
        assert!(
            norms[k + 1] < norms[k] * 0.999_f32,
            "stacked ||R|| should strictly drop (chain step {}): {} -> {}",
            k,
            norms[k],
            norms[k + 1]
        );
    }
}

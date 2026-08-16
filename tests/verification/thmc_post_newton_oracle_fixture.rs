// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! **Solver Wave S1 prep** — post-Newton stacked residual oracle fixture (skeleton).
//!
//! Wave 1 scope ([`outputs/.plans/archive/waves/solver-quality-wave-plan.md`](../../../outputs/.plans/archive/waves/solver-quality-wave-plan.md)):
//! wire `tol` to stacked-\(R\) exit, post-step diagnostics on the implicit functional, brute-force
//! oracle at Newton exit. Production hook deferred — this file pins the **oracle contract** only.
//!
//! See [`docs/Solver-Status.md`](../../docs/Solver-Status.md) §THMC — Wave 1 prep row.

#![cfg(feature = "thmc-coupled")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{Field, HumidityField, ReactionExtentField, TemperatureField};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::core::StepEntryDamageMask;
use umst_manifold::physics::solvers::{
    ChemicalPlan, HydrologicPlan, MechanicalPlan, ReactionExtentKinetics, ThermalPlan,
    ThmcImplicitEulerThermalHumidityReactionExtentResidual, ThmcState,
};
#[path = "../injection_mechanism_fixture.rs"]
mod injection_mechanism_fixture;
use injection_mechanism_fixture::injection_fixture_kinetics;

type B = NdArray<f32>;

fn dev() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn reference_reaction_extent_kinetics() -> ReactionExtentKinetics {
    injection_fixture_kinetics()
}

fn two_node_chain_manifold(n: usize) -> UnifiedMaterialStateTensor<B> {
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

/// Wave S1 oracle snapshot — fields a production `ThmcPostNewtonDiagnostic` hook will expose.
#[derive(Clone, Debug, PartialEq)]
struct PostNewtonOracleSnapshot {
    stacked_residual_l2: f32,
    newton_steps_taken: usize,
}

/// Brute-force independent recompute of stacked ‖R‖₂ (oracle at Newton exit).
fn oracle_stacked_residual_l2(
    assembler: &ThmcImplicitEulerThermalHumidityReactionExtentResidual<B>,
    trial: &ThmcState<B>,
    coords: &Tensor<B, 2>,
    boundary_mask: &Tensor<B, 3>,
    body_force: &Tensor<B, 3>,
    cross_section_area: f32,
) -> f32 {
    assembler
        .residual_l2_including_quasi_static_r_u(
            trial,
            coords,
            boundary_mask,
            body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2_including_quasi_static_r_u on post-Newton oracle two-node chain (FP §6 Track G Wave S1 prep)")
}

/// Skeleton: one damped Newton step → post-step ‖R‖₂ matches independent oracle recompute.
#[test]
fn post_newton_stacked_residual_oracle_matches_independent_recompute_two_nodes() {
    let d = dev();
    let n = 2usize;
    let manifold = two_node_chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("two_node_chain_manifold node_positions SI coords for post-Newton oracle fixture (FP §6 Track G Wave S1 prep)")
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
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let h_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.50_f32, 0.62_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial = ThmcState::from_tensors(
        Tensor::<B, 3>::from_data(
            Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
            &d,
        ),
        Tensor::<B, 3>::from_data(
            Data::new(vec![0.51_f32, 0.63_f32], Shape::new([1, n, 1])),
            &d,
        ),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::from_data(
            Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
            &d,
        ),
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        0.0_f32,
    );
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(Tensor::<B, 3>::zeros([1, n, 1], &d)),
        kinetics,
    };

    let cross_section_area = 0.01_f32;
    let (post_trial, norm_before, norm_after) = assembler
        .one_damped_newton_step_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            1.0_f32,
            1.0e-5_f32,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::one_damped_newton_step_with_quasi_static_r_u on two-node chain (FP §6 Track G Wave S1 prep)");

    let snapshot = PostNewtonOracleSnapshot {
        stacked_residual_l2: norm_after,
        newton_steps_taken: 1,
    };

    let oracle_l2 = oracle_stacked_residual_l2(
        &assembler,
        &post_trial,
        &coords,
        &boundary_mask,
        &body_force,
        cross_section_area,
    );

    assert!(
        (snapshot.stacked_residual_l2 - oracle_l2).abs() < 1e-4_f32,
        "post-Newton diagnostic must match brute-force oracle: diag={} oracle={}",
        snapshot.stacked_residual_l2,
        oracle_l2
    );
    assert!(
        norm_after <= norm_before + 1e-6_f32,
        "Newton step should not increase stacked ‖R‖₂: before={norm_before} after={norm_after}"
    );
}

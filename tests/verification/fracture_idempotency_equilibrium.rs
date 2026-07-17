// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §6 — phase-field fracture idempotency at AT2 equilibrium (RW-FP-P41).

#![cfg(feature = "fracture-at2")]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::core::field::{DamageField, Field, FractureEnergyField, SmallStrainField};
use umst_manifold::physics::solvers::PhaseFieldFractureSolver;

type B = NdArray<f32>;

fn strain_field(t: Tensor<B, 4>) -> SmallStrainField<B> {
    SmallStrainField::from_tensor(t)
}

fn damage_field(t: Tensor<B, 3>) -> DamageField<B> {
    Field::new(t)
}

fn gc_field(t: Tensor<B, 3>) -> FractureEnergyField<B> {
    FractureEnergyField::from_tensor(t)
}

fn max_abs_drift(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).abs())
        .fold(0.0_f32, f32::max)
}

fn tiny_chain_fixture() -> (Tensor<B, 2, Int>, Tensor<B, 4>, Tensor<B, 3>, Tensor<B, 3>) {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let strain = Tensor::<B, 4>::zeros([batch, n, 3, 3], &dev);
    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    (edges_b1, strain, damage, fracture_energy_gc)
}

#[test]
fn update_damage_idempotent_at_zero_strain_equilibrium() {
    let (edges_b1, strain, damage, gc) = tiny_chain_fixture();
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let tol = 1e-6_f32;

    let d1 = solver.update_damage(
        strain_field(strain.clone()),
        damage_field(damage),
        gc_field(gc.clone()),
        edges_b1.clone(),
    ).expect("update_damage");
    let d1_vals = d1.clone().into_tensor().into_data().value;

    let d2 = solver.update_damage(strain_field(strain), d1, gc_field(gc), edges_b1).expect("update_damage");
    let d2_vals = d2.into_tensor().into_data().value;

    assert!(max_abs_drift(&d1_vals, &d2_vals) < tol);
    assert!(d1_vals.iter().all(|x| x.abs() < tol));
}

#[test]
fn update_damage_staggered_idempotent_at_converged_outer_equilibrium() {
    let (edges_b1, strain, damage, gc) = tiny_chain_fixture();
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let tol = 1e-6_f32;
    let outer_iters = 4usize;

    let strain_snapshot = strain.clone();
    let mut strain_fn = move |_d: &DamageField<B>| strain_field(strain_snapshot.clone());

    let d_conv = solver.update_damage_staggered(
        &mut strain_fn,
        damage_field(damage),
        gc.clone(),
        edges_b1.clone(),
        outer_iters,
    ).expect("update_damage_staggered");
    let conv_vals = d_conv.clone().into_tensor().into_data().value;

    let d_repeat = solver.update_damage_staggered(
        &mut strain_fn,
        d_conv,
        gc,
        edges_b1,
        outer_iters,
    ).expect("update_damage_staggered");
    let repeat_vals = d_repeat.into_tensor().into_data().value;

    assert!(max_abs_drift(&conv_vals, &repeat_vals) < tol);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::ai::topology::HeavisideProjection;

type B = NdArray<f32>;

#[test]
fn heaviside_bounds_midpoint_and_stiff_limits() {
    let dev = Default::default();
    let h = HeavisideProjection::new(1.0, 0.5);
    let xs = [0.05f32, 0.2, 0.4, 0.5, 0.6, 0.8, 0.95];
    for &x in &xs {
        let t: Tensor<B, 3> = Tensor::from_data(Data::new(vec![x], Shape::new([1, 1, 1])), &dev);
        let y = h.project(t).into_data().value[0];
        assert!((-1e-5..=1.0 + 1e-5).contains(&y), "out of [0,1]: {y}");
    }
    let half: Tensor<B, 3> =
        Tensor::from_data(Data::new(vec![0.5_f32], Shape::new([1, 1, 1])), &dev);
    let mid = h.project(half).into_data().value[0];
    assert!(
        (mid - 0.5).abs() < 1e-3,
        "rho=0.5,eta=0.5 -> 0.5, got {mid}"
    );

    let h_hi = HeavisideProjection::new(64.0, 0.5);
    let lo: Tensor<B, 3> = Tensor::from_data(Data::new(vec![0.4_f32], Shape::new([1, 1, 1])), &dev);
    let hi: Tensor<B, 3> = Tensor::from_data(Data::new(vec![0.6_f32], Shape::new([1, 1, 1])), &dev);
    assert!(h_hi.project(lo).into_data().value[0] < 0.05);
    assert!(h_hi.project(hi).into_data().value[0] > 0.95);
}

#[test]
fn continuation_doubles_beta() {
    let mut h = HeavisideProjection::new(1.0, 0.5);
    assert_eq!(h.beta(), 1.0);
    h.step_continuation(30, 30, 64.0);
    assert!((h.beta() - 2.0).abs() < 1e-5);
    h.step_continuation(30, 60, 64.0);
    assert!((h.beta() - 4.0).abs() < 1e-5);
}

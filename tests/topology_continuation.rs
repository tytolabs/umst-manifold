// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(any(
    feature = "topology-density-evolution",
    feature = "solver-experimental"
))]

//! Integration smoke tests for continuation / projection helpers (requires
//! `topology-density-evolution` or meta `solver-experimental`).

use burn::tensor::{Shape, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::ai::topology::{ContinuationSchedule, VolumeProjection};

type B = NdArray<f32>;

#[test]
fn continuation_and_volume_smoke_with_solver_experimental_feature() {
    let dev = Default::default();
    assert!((ContinuationSchedule::value(15, 100) - 2.0).abs() < 1e-5);
    let rho = Tensor::<B, 3>::ones(Shape::new([1, 3, 1]), &dev).mul_scalar(0.1);
    let out = VolumeProjection::new(0.4_f32, 40).project(rho);
    let mean = out.sum().div_scalar(3.0_f32).into_scalar();
    assert!((mean - 0.4).abs() < 1e-4, "mean {mean}");
}

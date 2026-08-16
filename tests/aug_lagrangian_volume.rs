// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![cfg(feature = "solver-experimental")]

use burn::tensor::Tensor;
use burn_ndarray::NdArray;

use umst_manifold::ai::topology::AugmentedLagrangianVolume;

type B = NdArray<f32>;

#[test]
fn aug_lagrangian_loss_vanishes_at_target_mean() {
    let dev = Default::default();
    let mut aug = AugmentedLagrangianVolume::new(0.35);
    aug.mu = 10.0;
    aug.lambda = 0.5;
    let rho = Tensor::<B, 3>::full([1, 24, 1], 0.35, &dev);
    let l = aug.loss_term(rho).into_data().value[0];
    assert!(l.abs() < 1e-4, "loss at exact mean {l}");
}

#[test]
fn aug_lagrangian_lambda_increases_when_volume_high() {
    let mut aug = AugmentedLagrangianVolume::new(0.3);
    aug.mu = 4.0;
    aug.lambda = 0.0;
    for _ in 0..aug.update_period {
        aug.update_multipliers(0.5);
    }
    assert!(
        aug.lambda > 0.4,
        "λ should accumulate violation, got {}",
        aug.lambda
    );
}

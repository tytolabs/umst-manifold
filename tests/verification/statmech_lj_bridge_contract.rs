// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Lennard-Jones → continuum **placeholder bridge** contract (`statistical_mechanics::upscale_potentials`).
//!
//! This is **not** a virial or coexistence-derived EOS; it locks the analytic \(\varepsilon/\sigma^n\)
//! scaling verified in-crate. Stable lane **Vinet** checks live in `tests/verification/statmech_vinet_eos.rs`
//! (`statistical-mechanics-vinet`).

use approx::assert_abs_diff_eq;
use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::solvers::statistical_mechanics::{
    upscale_potentials, ANALYTIC_BULK_MODULUS_SCALE, ANALYTIC_SURFACE_ENERGY_SCALE,
};

type B = NdArray<f32>;

#[test]
fn lj_bridge_scaling_non_regression() {
    let dev = NdArrayDevice::Cpu;
    let lj: Tensor<B, 2> = Tensor::from_data(
        Data::new(
            vec![0.15_f32, 0.25_f32, 0.45_f32, 0.35_f32],
            Shape::new([2, 2]),
        ),
        &dev,
    );
    let (k, gamma) = upscale_potentials(lj);
    let kv = k.into_data().value;
    let gv = gamma.into_data().value;
    let c_k = ANALYTIC_BULK_MODULUS_SCALE;
    let c_g = ANALYTIC_SURFACE_ENERGY_SCALE;
    assert_abs_diff_eq!(kv[0], c_k * 0.15_f32 / 0.25_f32.powi(3), epsilon = 1e-5);
    assert_abs_diff_eq!(gv[0], c_g * 0.15_f32 / 0.25_f32.powi(2), epsilon = 1e-5);
    assert_abs_diff_eq!(kv[1], c_k * 0.45_f32 / 0.35_f32.powi(3), epsilon = 1e-5);
    assert_abs_diff_eq!(gv[1], c_g * 0.45_f32 / 0.35_f32.powi(2), epsilon = 1e-5);
}

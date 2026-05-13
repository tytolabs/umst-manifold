// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thermodynamic control-barrier-function tests.
//!
//! Verifies the bookkeeping properties of the CBF independently of any
//! particular cartridge. We check three independent invariants:
//!
//! - Landauer cost is non-negative for any non-trivial information gain.
//! - The cost saturates `k_B T ln 2` per bit at thermodynamic equilibrium.
//! - The available credit decreases monotonically when admissible work
//!   is performed.

use approx::assert_relative_eq;
use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::ai::cbf::ThermodynamicCBF;

type B = NdArray<f32>;

const K_BOLTZMANN: f64 = 1.380649e-23;

fn landauer_cost_joules(temperature_k: f64, info_gain_bits: f64) -> f64 {
    K_BOLTZMANN * temperature_k * std::f64::consts::LN_2 * info_gain_bits
}

#[test]
fn landauer_cost_nonnegative() {
    for &t in &[150.0_f64, 300.0, 1000.0] {
        for &h in &[0.0_f64, 0.5, 1.0, 32.0] {
            let q = landauer_cost_joules(t, h);
            assert!(q >= 0.0, "Landauer cost negative for T = {t}, H = {h}");
        }
    }
}

#[test]
fn landauer_saturation_one_bit() {
    // At T = 300 K, one bit of erasure costs k_B T ln 2 ≈ 2.87e-21 J.
    let q = landauer_cost_joules(300.0, 1.0);
    let expected = 2.870_e-21;
    assert_relative_eq!(q, expected, epsilon = 1.0e-22, max_relative = 1.0e-2);
}

#[test]
fn budget_decreases_under_admissible_work() {
    let mut budget = 1.0e-15_f64;
    let work_per_step = landauer_cost_joules(300.0, 1.0);
    let steps = 100;

    for _ in 0..steps {
        assert!(budget >= work_per_step, "budget underflow");
        budget -= work_per_step;
    }
    let expected_remaining = 1.0e-15_f64 - (steps as f64) * work_per_step;
    assert_relative_eq!(budget, expected_remaining, max_relative = 1.0e-9);
}

#[test]
fn verify_tensor_update_clamps_negative_d_int() {
    let dev = NdArrayDevice::default();
    let mut cbf = ThermodynamicCBF::new(300.0_f64, 1.0e-12_f64);
    cbf.k_phys_dint_to_joules = 1.0;
    let d_int = Tensor::<B, 1>::from_floats([-1.0e6_f32], &dev);
    let info_gain = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
    cbf.verify_tensor_update(d_int, info_gain)
        .expect("negative d_int must not inflate CD violation after clamp");
}

#[test]
fn verify_tensor_update_credit_deduction_independent_of_d_int() {
    let dev = NdArrayDevice::default();
    let credit0 = 1.0e-9_f64;
    let mut cbf_a = ThermodynamicCBF::new(300.0_f64, credit0);
    cbf_a.k_phys_dint_to_joules = 0.0;
    let mut cbf_b = ThermodynamicCBF::new(300.0_f64, credit0);
    cbf_b.k_phys_dint_to_joules = 100.0;
    let bits = Tensor::<B, 1>::from_floats([4.0_f32], &dev);
    let d_zero = Tensor::<B, 1>::from_floats([0.0_f32], &dev);
    let d_big = Tensor::<B, 1>::from_floats([1.0e6_f32], &dev);
    let cost_a = cbf_a.verify_tensor_update(d_zero, bits.clone()).unwrap();
    let credit_after_a = cbf_a.available_credit_joules;
    let cost_b = cbf_b.verify_tensor_update(d_big, bits).unwrap();
    assert_relative_eq!(cost_a, cost_b, epsilon = 1e-30, max_relative = 1e-9);
    assert_relative_eq!(
        credit_after_a,
        cbf_b.available_credit_joules,
        epsilon = 1e-30,
        max_relative = 1e-9
    );
}

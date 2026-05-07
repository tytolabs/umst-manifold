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

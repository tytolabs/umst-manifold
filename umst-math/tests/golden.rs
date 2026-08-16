// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Golden qubit fixtures — parity with `fixtures.rs`.

use umst_math::density::DensityDiag;
use umst_math::fixtures::{qubit_one, qubit_plus, qubit_zero};
use umst_math::tensor::tensor_diagonal;

#[test]
fn qubit_pure_states_are_trace_one() {
    assert!((qubit_zero().trace().into_inner() - 1.0).abs() < 1e-12);
    assert!((qubit_one().trace().into_inner() - 1.0).abs() < 1e-12);
    assert!((qubit_plus().trace().into_inner() - 1.0).abs() < 1e-12);
}

#[test]
fn tensor_two_zeros() {
    let z = qubit_zero();
    let zz: DensityDiag<4> = tensor_diagonal(&z, &z).expect("tensor");
    assert!((zz.trace().into_inner() - 1.0).abs() < 1e-12);
}

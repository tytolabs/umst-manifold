// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! constants reexport reaches one table: `umst_math::umst_constants` is the same SSOT as `umst-constants`.

#[test]
fn constants_reexport_reaches_one_table() {
    let direct = umst_constants::BOLTZMANN;
    let via_math = umst_math::umst_constants::BOLTZMANN;
    assert_eq!(direct, via_math);
    let direct_ref: &f64 = &umst_constants::BOLTZMANN;
    let via_math_ref: &f64 = &umst_math::umst_constants::BOLTZMANN;
    assert!(
        std::ptr::eq(direct_ref, via_math_ref),
        "re-export must be the same const item (one table), not a copy"
    );
}

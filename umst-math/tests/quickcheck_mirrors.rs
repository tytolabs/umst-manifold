// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Property tests — Englert disk sanity (QuickCheck).

use ordered_float::NotNan;
use quickcheck::quickcheck;
use umst_math::englert::englert_bound_holds;

quickcheck! {
    fn englert_holds_on_unit_disk(r: f64, theta: f64) -> bool {
        if !r.is_finite() || !theta.is_finite() {
            return true;
        }
        let r = r.abs() % 1.0;
        let theta = theta % (std::f64::consts::PI * 2.0);
        let vc = r * theta.cos();
        let ic = r * theta.sin();
        if !vc.is_finite() || !ic.is_finite() {
            return true;
        }
        let v = NotNan::new(vc).expect("v");
        let i = NotNan::new(ic).expect("i");
        englert_bound_holds(v, i)
    }
}

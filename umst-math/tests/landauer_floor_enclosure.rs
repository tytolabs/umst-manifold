// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The Landauer floor of `unfold` from the rational enclosure of ln 2 (cell LANDAUER-FLOOR-LN2-ENCLOSURE).
//!
//! `landauer_step_joules` returns `k_B · T · ln2_lo · bits` rounded down, with `ln2_lo = 6931471803 / 10^10` the lower
//! end of the enclosure `(ln2_lo, ln2_hi)` that Lean proves contains ln 2 (`UnfoldTermination.log_two_mem_enclosure`,
//! from Mathlib `Real.log_two_gt_d9` and `Real.log_two_lt_d9`). A step that spends less than that floor is refused
//! (`MeasuredBelowFloor`), and every such spend lies below the real floor `k_B · T · ln 2 · bits`; a spend between
//! the enclosed floor and the binary64 `ln 2` floor is admitted.

use ordered_float::NotNan;
use umst_math::solve_combinator::{
    landauer_step_joules, unfold, CombinatorRefuse, EnergyBudget, FixedJouleMeter,
    ProblemProgressWindow, ProblemTolerance, UnfoldStop,
};
use umst_math::umst_constants::{
    BOLTZMANN, LOG_TWO_ENCLOSURE_LOWER, LOG_TWO_ENCLOSURE_UPPER, ROWS,
};

fn nn(x: f64) -> NotNan<f64> {
    NotNan::new(x).expect("finite")
}

fn floor_joules(t: f64, bits: f64) -> f64 {
    landauer_step_joules(nn(t), nn(bits))
        .expect("floor")
        .ucrs_ledger_joules()
        .expect("finite")
        .into_inner()
}

/// One unfold step whose meter posts `joules`, at 300 K and one bit per step.
fn one_step(joules: f64) -> Result<(), CombinatorRefuse> {
    let tolerance = ProblemTolerance::from_problem(1.0, 1e-30).expect("tolerance");
    let budget = EnergyBudget::from_joules_at(1.0, 300.0).expect("budget");
    let window = ProblemProgressWindow::from_krylov_restart(1).expect("window");
    let stop = UnfoldStop::new(tolerance, budget, window, 1.0).expect("stop");
    let mut calls = 0_u32;
    let out = unfold(
        1.0_f64,
        |x| NotNan::new(*x).map_err(|_| CombinatorRefuse::NonFiniteQuantity),
        |x, _| {
            calls += 1;
            if calls > 1 {
                return Err(CombinatorRefuse::NonFiniteQuantity);
            }
            Ok(x * 0.5)
        },
        stop,
        &FixedJouleMeter::from_joules(joules).expect("meter"),
    );
    match out {
        Err(CombinatorRefuse::MeasuredBelowFloor) => Err(CombinatorRefuse::MeasuredBelowFloor),
        _ => Ok(()),
    }
}

#[test]
fn enclosure_rows_are_the_pinned_rationals() {
    let lo = ROWS
        .iter()
        .find(|r| r.id == "logTwoEnclosureLower")
        .expect("lower row");
    let hi = ROWS
        .iter()
        .find(|r| r.id == "logTwoEnclosureUpper")
        .expect("upper row");
    // The table stores each rational in lowest terms; compare by cross-multiplication.
    let is = |num: &str, den: &str, n: u128| {
        num.parse::<u128>().expect("num") * 10_000_000_000 == n * den.parse::<u128>().expect("den")
    };
    assert!(is(lo.exact_num, lo.exact_den, 6_931_471_803));
    assert!(is(hi.exact_num, hi.exact_den, 6_931_471_808));
    assert!(lo.value < core::f64::consts::LN_2);
    assert!(core::f64::consts::LN_2 < hi.value);
    assert_eq!(
        (lo.value, hi.value),
        (LOG_TWO_ENCLOSURE_LOWER, LOG_TWO_ENCLOSURE_UPPER)
    );
}

#[test]
fn floor_is_the_enclosure_lower_end_rounded_down() {
    for &(t, bits) in &[(300.0, 1.0), (293.15, 1.0), (4.2, 64.0), (1.0e4, 0.5)] {
        let j = floor_joules(t, bits);
        let real = BOLTZMANN * t * core::f64::consts::LN_2 * bits;
        // ln2_lo / ln 2 = 1 − 3.7e-10: the floor sits that far below the real floor, far beyond binary64 rounding.
        assert!(j < real * (1.0 - 3.0e-10), "T = {t}, bits = {bits}");
        assert!(j > real * (1.0 - 5.0e-10), "T = {t}, bits = {bits}");
        assert!(
            j <= BOLTZMANN * t * LOG_TWO_ENCLOSURE_LOWER * bits,
            "T = {t}, bits = {bits}"
        );
    }
}

#[test]
fn refusal_decides_about_the_real_floor() {
    let kt = BOLTZMANN * 300.0;
    // Between the enclosed floor and the binary64 ln 2 floor: admitted (the binary64 floor refused it).
    assert_eq!(one_step(kt * (LOG_TWO_ENCLOSURE_LOWER + 2.0e-10)), Ok(()));
    // Below the enclosure's lower end, hence below the real floor: refused.
    assert_eq!(
        one_step(kt * (LOG_TWO_ENCLOSURE_LOWER - 1.0e-10)),
        Err(CombinatorRefuse::MeasuredBelowFloor)
    );
    // At the real floor and above: admitted.
    assert_eq!(one_step(kt * core::f64::consts::LN_2), Ok(()));
}

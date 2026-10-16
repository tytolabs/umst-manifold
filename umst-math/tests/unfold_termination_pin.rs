// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The unfold termination pin: law L-E for the plain `unfold` in Rust (cell FORMAL-UNFOLD-TERMINATION-LANDAUER).
//!
//! Lean `UnfoldTermination.unfold_terminates_of_secondLaw`: steps that each spend at least the enclosed Landauer floor
//! `ε = k_B · T · ln2_lo · b` and whose spends sum to at most `B` number at most `⌊B / ε⌋`. `UnfoldStop::max_steps`
//! exposes that bound; a run priced at the floor itself, on a residual that falls forever and never meets its
//! tolerance, debits exactly `⌊B / ε⌋` steps and then reports `BudgetSpent`.

use core::cell::Cell;
use ordered_float::NotNan;
use umst_math::solve_combinator::{
    landauer_step_joules, unfold, CombinatorRefuse, EnergyBudget, FixedJouleMeter,
    ProblemProgressWindow, ProblemTolerance, SolveOutcome, UnfoldStop,
};

fn nn(x: f64) -> NotNan<f64> {
    NotNan::new(x).expect("finite")
}

/// Run `unfold` at the floor price on budget `budget_floors · ε`; the number of debited steps and the stop's bound.
fn floor_priced_run(t: f64, bits: f64, budget_floors: f64) -> (u64, u64) {
    let eps = landauer_step_joules(nn(t), nn(bits))
        .expect("floor")
        .ucrs_ledger_joules()
        .expect("finite")
        .into_inner();
    let budget = EnergyBudget::from_joules_at(budget_floors * eps, t).expect("budget");
    let tolerance = ProblemTolerance::from_problem(1.0, 1e-300).expect("tolerance");
    let window = ProblemProgressWindow::from_krylov_restart(4).expect("window");
    let stop = UnfoldStop::new(tolerance, budget, window, bits).expect("stop");
    let bound = stop.max_steps().expect("bound");
    let calls = Cell::new(0_u64);
    let out = unfold(
        0_u64,
        |k: &u64| {
            NotNan::new(1.0 / (*k as f64 + 1.0)).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        },
        |k, _| {
            calls.set(calls.get() + 1);
            Ok(k + 1)
        },
        stop,
        &FixedJouleMeter::from_joules(eps).expect("meter"),
    )
    .expect("outcome");
    assert!(
        matches!(out, SolveOutcome::BudgetSpent { .. }),
        "the run ends on its budget"
    );
    // The last measured step is the one the budget cannot pay; every other step was debited.
    (calls.get() - 1, bound)
}

#[test]
fn max_steps_is_the_floor_of_budget_over_enclosed_floor() {
    for &(t, bits, floors, expected) in &[
        (300.0, 1.0, 10.5, 10_u64),
        (293.15, 8.0, 7.25, 7),
        (4.2, 64.0, 3.0, 3),
        (1.0e4, 0.5, 40.9, 40),
    ] {
        let eps = landauer_step_joules(nn(t), nn(bits)).expect("floor");
        let eps = eps.ucrs_ledger_joules().expect("finite").into_inner();
        let stop = UnfoldStop::new(
            ProblemTolerance::from_problem(1.0, 1e-3).expect("tolerance"),
            EnergyBudget::from_joules_at(floors * eps, t).expect("budget"),
            ProblemProgressWindow::from_krylov_restart(1).expect("window"),
            bits,
        )
        .expect("stop");
        assert_eq!(
            stop.max_steps().expect("bound"),
            expected,
            "T = {t}, bits = {bits}"
        );
    }
}

#[test]
fn floor_priced_unfold_stops_within_the_bound() {
    for &(t, bits, floors) in &[(300.0, 1.0, 10.5), (293.15, 8.0, 7.25), (4.2, 64.0, 25.5)] {
        let (debited, bound) = floor_priced_run(t, bits, floors);
        assert!(
            debited <= bound,
            "T = {t}, bits = {bits}: {debited} > {bound}"
        );
        // With a budget strictly between two multiples of the floor, the bound is met exactly.
        assert_eq!(debited, bound, "a floor-priced run meets the bound");
    }
}

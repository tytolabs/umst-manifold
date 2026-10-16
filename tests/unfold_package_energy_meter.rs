// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! The host package energy counter as the unfold step meter (cell UNFOLD-PACKAGE-ENERGY-METER).
//!
//! A step with a counter reading carries a `Measured` spend; a step without one still runs and
//! carries the typed absence. `unfold` debits an absent step at the Landauer floor, so the
//! step count stays within `UnfoldStop::max_steps` (law L-E), and the grade of a sum is the
//! meet, so honest_spend on an absent spend has no value.

use std::cell::Cell;

use ordered_float::NotNan;
use umst_manifold::solve::PackageEnergyMeter;
use umst_math::dignity::core::honest_spend;
use umst_math::solve_combinator::{
    landauer_step_joules, unfold, CombinatorRefuse, EnergyBudget, EnergySpendProvenance,
    EnergySpent, ProblemProgressWindow, ProblemTolerance, SolveOutcome, StepEnergyMeter,
    StrategyRung, UnfoldStop,
};
use umst_math::solve_kleisli::nested_unfold;
use umst_ucrs::rapl::read_package_energy;

const T_K: f64 = 293.15;

/// A meter for a host with no counter: every step runs and its spend is absent.
struct NoCounter;

impl StepEnergyMeter for NoCounter {
    fn measure<F, X>(&self, step: F) -> Result<(X, EnergySpent), CombinatorRefuse>
    where
        F: FnOnce() -> Result<X, CombinatorRefuse>,
    {
        Ok((step()?, EnergySpent::absent()))
    }
}

fn floor_joules() -> f64 {
    landauer_step_joules(
        NotNan::new(T_K).expect("T"),
        NotNan::new(1.0).expect("bits"),
    )
    .expect("floor")
    .ucrs_ledger_joules()
    .expect("floor joules")
    .into_inner()
}

fn stop(tolerance: f64, joules: f64) -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, tolerance).expect("tolerance"),
        EnergyBudget::from_joules_at(joules, T_K).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn abs(x: &f64) -> Result<NotNan<f64>, CombinatorRefuse> {
    NotNan::new(x.abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
}

#[test]
fn the_package_meter_grades_each_step_by_the_host_counter() {
    let solved = nested_unfold(
        1.0_f64,
        abs,
        |x: &f64, _rung: StrategyRung| Ok(x / 2.0),
        stop(1e-6, 1.0),
        &PackageEnergyMeter,
    )
    .expect("halving converges");
    let grade = solved.ledger().provenance();
    if read_package_energy().is_ok() {
        assert_eq!(grade, EnergySpendProvenance::Measured);
        assert!(solved.ledger().total().is_ok());
    } else {
        assert_eq!(grade, EnergySpendProvenance::Absent);
        assert_eq!(solved.ledger().total(), Err(CombinatorRefuse::Unmeasured));
    }
}

#[test]
fn an_absent_step_is_debited_at_the_landauer_floor() {
    // Budget of 3.5 floors and a residual that shrinks too slowly to converge: the run spends
    // its budget after three floor debits, within max_steps.
    let budget = 3.5 * floor_joules();
    let steps = Cell::new(0_u64);
    let policy = stop(1e-300, budget);
    let out = unfold(
        1.0_f64,
        abs,
        |x: &f64, _rung: StrategyRung| {
            steps.set(steps.get() + 1);
            Ok(x * (1.0 - 1e-9))
        },
        policy,
        &NoCounter,
    )
    .expect("runs without a counter");
    match out {
        SolveOutcome::BudgetSpent { spent, .. } => {
            assert_eq!(spent.provenance(), EnergySpendProvenance::Absent);
            assert_eq!(
                spent.ucrs_ledger_joules(),
                Err(CombinatorRefuse::Unmeasured)
            );
        }
        other => panic!("expected BudgetSpent, got {other:?}"),
    }
    assert_eq!(steps.get(), 4);
    assert!(steps.get() <= policy.max_steps().expect("L-E bound") + 1);
}

#[test]
fn the_grade_of_a_sum_is_the_meet_and_honest_spend_has_no_value_on_absence() {
    let measured = EnergySpent::measured(1.0).expect("measured");
    let absent = EnergySpent::absent();
    let sum = measured.plus(absent).expect("sum");
    assert_eq!(sum.provenance(), EnergySpendProvenance::Absent);
    assert_eq!(sum.ucrs_ledger_joules(), Err(CombinatorRefuse::Unmeasured));
    assert_eq!(
        measured.plus(measured).expect("sum").provenance(),
        EnergySpendProvenance::Measured
    );
    // honest_spend takes joules; an absent spend has none to give it, so the predicate's
    // input is the typed refusal and the decision is absent, through the meet.
    let decide = |spend: EnergySpent| {
        spend
            .ucrs_ledger_joules()
            .map(|j| honest_spend(T_K, 1.0, j.into_inner()))
    };
    assert_eq!(decide(measured), Ok(true));
    assert_eq!(
        decide(EnergySpent::measured(floor_joules() / 2.0).expect("half floor")),
        Ok(false)
    );
    assert_eq!(decide(absent), Err(CombinatorRefuse::Unmeasured));
    assert_eq!(decide(sum), Err(CombinatorRefuse::Unmeasured));
}

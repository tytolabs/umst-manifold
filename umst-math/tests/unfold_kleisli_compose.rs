// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Nested solves compose under `unfold` as Kleisli arrows (cell UNFOLD-KLEISLI-COMPOSE).
//!
//! Each nested solve is a Newton square root run by `unfold` under its own budget share
//! and sub-meter. Step spends and budget shares are powers of two, so every sum is exact
//! and the enclosures collapse to points.

use std::cell::Cell;

use ordered_float::NotNan;
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow, ProblemTolerance,
    StepEnergyMeter, StrategyRung, UnfoldStop, UnmeasuredMeter,
};
use umst_math::solve_kleisli::{
    and_then, compose, nested_unfold, Certified, Nested, NestedRefuse, SumEnclosure,
};

/// Method form of `and_then`, so the composites below read left to right.
trait AndThenNested<X> {
    fn and_then_nested<Y>(self, f: impl FnOnce(X) -> Nested<Y>) -> Nested<Y>;
}

impl<X> AndThenNested<X> for Nested<X> {
    fn and_then_nested<Y>(self, f: impl FnOnce(X) -> Nested<Y>) -> Nested<Y> {
        and_then(self, f)
    }
}

const T_K: f64 = 293.15;
/// 2⁻⁶⁰ J per step: above the Landauer floor k_B·T·ln 2 ≈ 2.8·10⁻²¹ J at 293.15 K.
const STEP_J: f64 = 1.0 / (1u64 << 60) as f64;
/// 2⁻⁴⁰ J: the outer budget.
const OUTER_J: f64 = 1.0 / (1u64 << 40) as f64;

fn stop(budget: EnergyBudget) -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, 1e-12).expect("tolerance"),
        budget,
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

fn meter() -> FixedJouleMeter {
    FixedJouleMeter::from_joules(STEP_J).expect("meter")
}

fn share() -> EnergyBudget {
    EnergyBudget::from_joules_at(OUTER_J / 4.0, T_K).expect("share")
}

/// A nested solve: Newton for `x² = target`, started at `target`, counting its steps.
fn sqrt_solve<M: StepEnergyMeter>(
    target: f64,
    budget: EnergyBudget,
    meter: &M,
    steps: &Cell<u32>,
) -> Nested<f64> {
    nested_unfold(
        target,
        move |x: &f64| {
            NotNan::new((x * x - target).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        },
        |x: &f64, _rung: StrategyRung| {
            steps.set(steps.get() + 1);
            Ok(x - (x * x - target) / (2.0 * x))
        },
        stop(budget),
        meter,
    )
}

#[test]
fn bind_is_associative_and_concatenates_ledgers_and_chains() {
    let m = meter();
    let n = Cell::new(0);
    let f = |a: f64| sqrt_solve(a, share(), &m, &n);
    let g = |y: f64| sqrt_solve(y + 7.0, share(), &m, &n);
    let h = |w: f64| sqrt_solve(3.0 * w, share(), &m, &n);
    let left = compose(compose(f, g), h)(2.0);
    let right = compose(f, compose(g, h))(2.0);
    assert_eq!(left, right);
    let c = left.expect("all three converge");
    assert_eq!(c.ledger().entries().len(), 3);
    assert_eq!(c.chain().links().len(), 3);
    assert!(c.chain().meets());
    let expected = 3.0 * (2.0_f64.sqrt() + 7.0).sqrt();
    assert!((c.value() - expected.sqrt()).abs() < 1e-9);
}

#[test]
fn unit_is_left_and_right_identity() {
    let m = meter();
    let n = Cell::new(0);
    let f = |a: f64| sqrt_solve(a, share(), &m, &n);
    assert_eq!(and_then(Ok(Certified::unit(5.0)), f), f(5.0));
    assert_eq!(and_then(f(5.0), |x| Ok(Certified::unit(x))), f(5.0));
}

#[test]
fn budget_is_an_exact_monoid_with_a_split() {
    let outer = EnergyBudget::from_joules_at(OUTER_J, T_K).expect("outer");
    let (first, rest) = outer.split(OUTER_J / 4.0).expect("exact split");
    assert_eq!(first.plus(rest).expect("sum"), outer);
    let (second, third) = rest.split(OUTER_J / 2.0).expect("exact split");
    let (zero, same) = outer.split(0.0).expect("zero share");
    assert_eq!(same, outer);
    assert_eq!(first.plus(zero).expect("right id"), first);
    assert_eq!(zero.plus(first).expect("left id"), first);
    let l = first
        .plus(second)
        .expect("a+b")
        .plus(third)
        .expect("(a+b)+c");
    let r = first
        .plus(second.plus(third).expect("b+c"))
        .expect("a+(b+c)");
    assert_eq!(l, r);
    assert_eq!(l, outer);
    assert_eq!(
        first.plus(second).expect("a+b"),
        second.plus(first).expect("b+a")
    );
    // 1 − 10⁻²⁰ rounds to 1: the shares would not add back exactly.
    let one = EnergyBudget::from_joules_at(1.0, T_K).expect("one");
    assert_eq!(one.split(1e-20), Err(CombinatorRefuse::InexactBudgetSum));
    assert_eq!(one.split(2.0), Err(CombinatorRefuse::NonPositiveBudget));
    let warm = EnergyBudget::from_joules_at(1.0, 300.0).expect("warm");
    assert_eq!(one.plus(warm), Err(CombinatorRefuse::TemperatureMismatch));
}

#[test]
fn outer_spend_is_the_sum_of_sub_meters_within_the_outer_budget() {
    let m = meter();
    let n = Cell::new(0);
    let outer = EnergyBudget::from_joules_at(OUTER_J, T_K).expect("outer");
    let (b1, rest) = outer.split(OUTER_J / 4.0).expect("split");
    let (b2, b3) = rest.split(OUTER_J / 4.0).expect("split");
    let c = sqrt_solve(2.0, b1, &m, &n)
        .and_then_nested(|y| sqrt_solve(y + 7.0, b2, &m, &n))
        .and_then_nested(|w| sqrt_solve(3.0 * w, b3, &m, &n))
        .expect("converges");
    let total = c.ledger().total().expect("total");
    let all_steps = f64::from(n.get()) * STEP_J;
    assert_eq!(total, SumEnclosure::point(all_steps).expect("point"));
    let per_solve: f64 = c.ledger().entries().iter().map(|e| e.joules.hi()).sum();
    assert_eq!(per_solve, all_steps);
    assert!(total.hi() <= outer.remaining_joules().into_inner());
    for (entry, share) in c.ledger().entries().iter().zip([b1, b2, b3]) {
        assert!(entry.joules.hi() <= share.remaining_joules().into_inner());
    }
}

#[test]
fn an_inner_refusal_is_the_outer_refusal() {
    let m = meter();
    let absent = UnmeasuredMeter::default();
    let n = Cell::new(0);
    let ran_h = Cell::new(false);
    let out = sqrt_solve(2.0, share(), &m, &n)
        .and_then_nested(|y| sqrt_solve(y + 7.0, share(), &absent, &n))
        .and_then_nested(|w| {
            ran_h.set(true);
            sqrt_solve(w, share(), &m, &n)
        });
    match out {
        Err(NestedRefuse::Refused {
            cause,
            ledger,
            chain,
        }) => {
            assert_eq!(cause, CombinatorRefuse::Unmeasured);
            assert_eq!(ledger.entries().len(), 2);
            assert_eq!(chain.links().len(), 1);
        }
        other => panic!("expected the inner refusal, got {other:?}"),
    }
    assert!(!ran_h.get());
}

#[test]
fn an_inner_budget_spent_stops_the_composite() {
    let m = meter();
    let n = Cell::new(0);
    let starved = EnergyBudget::from_joules_at(STEP_J / 2.0, T_K).expect("starved");
    let ran_h = Cell::new(false);
    let out = sqrt_solve(2.0, share(), &m, &n)
        .and_then_nested(|y| sqrt_solve(y + 7.0, starved, &m, &n))
        .and_then_nested(|w| {
            ran_h.set(true);
            Ok(Certified::unit(w))
        });
    match out {
        Err(NestedRefuse::BudgetSpent { ledger, chain, .. }) => {
            assert_eq!(ledger.entries().len(), 2);
            assert_eq!(chain.links().len(), 1);
        }
        other => panic!("expected BudgetSpent, got {other:?}"),
    }
    assert!(!ran_h.get());
}

#[test]
fn an_inner_stall_stops_the_composite() {
    let m = meter();
    let n = Cell::new(0);
    let stalled = |y: f64| -> Nested<f64> {
        nested_unfold(
            y,
            |_x: &f64| Ok(NotNan::new(1.0).expect("finite")),
            |x: &f64, _rung: StrategyRung| Ok(*x),
            stop(share()),
            &m,
        )
    };
    let out = compose(|a: f64| sqrt_solve(a, share(), &m, &n), stalled)(2.0);
    match out {
        Err(NestedRefuse::Stalled {
            evidence,
            ledger,
            chain,
        }) => {
            assert_eq!(evidence.rung, StrategyRung::SparseDirect);
            assert_eq!(ledger.entries().len(), 2);
            assert_eq!(chain.links().len(), 1);
            assert!(chain.meets());
        }
        other => panic!("expected Stalled, got {other:?}"),
    }
}

#[test]
fn sum_enclosure_contains_an_inexact_sum() {
    let e = SumEnclosure::point(0.1)
        .and_then(|a| a.plus(SumEnclosure::point(0.2)?))
        .expect("sum");
    assert!(e.lo() < e.hi());
    assert!(e.contains(0.1 + 0.2));
    assert_eq!(
        SumEnclosure::point(-1.0),
        Err(CombinatorRefuse::NonPositiveBudget)
    );
    assert_eq!(
        SumEnclosure::point(f64::INFINITY),
        Err(CombinatorRefuse::NonFiniteQuantity)
    );
}

#[test]
fn tolerance_chain_carries_each_certificate() {
    let m = meter();
    let n = Cell::new(0);
    let c = sqrt_solve(2.0, share(), &m, &n)
        .and_then_nested(|y| sqrt_solve(y + 7.0, share(), &m, &n))
        .expect("converges");
    let chain = c.chain();
    let tol = chain.tolerance_sum().expect("tolerance sum");
    let res = chain.residual_sum().expect("residual sum");
    assert!(tol.contains(2e-12));
    assert!(res.hi() <= tol.hi());
}

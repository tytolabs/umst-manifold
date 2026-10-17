// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Proof for cell UNFOLD-KLEISLI-LAWS-PIN: nested solves composed by `solve_kleisli` satisfy, on
//! generated inputs, the statements umst-formal `UnfoldKleisli` proves in four languages:
//! bind_assoc (L1: bind is associative with unit as identity, an inner refusal is the outer
//! refusal, an outer spend is the sum of its inner spends), bind_honest (L2: honest spends,
//! floor ≤ measured, bind to an honest spend, so the summed measured energy is at least the
//! summed Landauer floors) and error_chain (an inner error within τ_in through an L-Lipschitz
//! map with an outer error within τ_out stays within τ_out + L·τ_in). The last test reads the
//! three parity rows of formal_parity.json and the Lean theorems they name.

use std::path::Path;

use ordered_float::NotNan;
use umst_math::solve_combinator::{
    landauer_step_joules, CombinatorRefuse, EnergyBudget, FixedJouleMeter, ProblemProgressWindow,
    ProblemTolerance, StepEnergyMeter, StrategyRung, UnfoldStop, UnmeasuredMeter,
};
use umst_math::solve_kleisli::{and_then, compose, nested_unfold, Converged, Nested, NestedRefuse};

const T_K: f64 = 293.15;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, n: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) % n
    }
}

fn floor() -> f64 {
    landauer_step_joules(
        NotNan::new(T_K).expect("T"),
        NotNan::new(1.0).expect("bits"),
    )
    .expect("floor")
    .ucrs_ledger_joules()
    .expect("joules")
    .into_inner()
}

fn stop(tolerance: f64) -> UnfoldStop {
    UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, tolerance).expect("tolerance"),
        EnergyBudget::from_joules_at(1e-9, T_K).expect("budget"),
        ProblemProgressWindow::from_krylov_restart(1).expect("window"),
        1.0,
    )
    .expect("stop")
}

/// A nested solve of `x = target` by halving the gap, to `tolerance`.
fn approach<M: StepEnergyMeter>(start: f64, target: f64, tolerance: f64, meter: &M) -> Nested<f64> {
    nested_unfold(
        start,
        move |x: &f64| {
            NotNan::new((x - target).abs()).map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        },
        move |x: &f64, _rung: StrategyRung| Ok(x - (x - target) / 2.0),
        stop(tolerance),
        meter,
    )
}

#[test]
fn bind_is_associative_on_generated_nested_solves() {
    let mut g = Lcg(11);
    for _ in 0..60 {
        let m = FixedJouleMeter::from_joules(floor() * (1 + g.next(50)) as f64).expect("meter");
        let (a, b, c) = (
            g.next(9) as f64,
            g.next(9) as f64 + 1.0,
            g.next(9) as f64 + 2.0,
        );
        let f = |x: f64| approach(x, x + a, 1e-6, &m);
        let gg = |x: f64| approach(x, b * x, 1e-6, &m);
        let h = |x: f64| approach(x, x - c, 1e-6, &m);
        let start = g.next(5) as f64;
        let left = compose(compose(f, gg), h)(start);
        let right = compose(f, compose(gg, h))(start);
        assert_eq!(left, right);
        // unit_bind and bind_unit.
        assert_eq!(and_then(Ok(Converged::unit(start)), f), f(start));
        assert_eq!(and_then(f(start), |x| Ok(Converged::unit(x))), f(start));
        // bind_spend: the outer ledger is the concatenation of the inner ledgers.
        let c3 = left.expect("converges");
        assert_eq!(c3.ledger().entries().len(), 3);
    }
}

#[test]
fn an_inner_refusal_is_the_outer_refusal_at_every_stage() {
    let mut g = Lcg(5);
    let paid = FixedJouleMeter::from_joules(floor() * 4.0).expect("meter");
    let absent = UnmeasuredMeter::default();
    for _ in 0..30 {
        let refusing = g.next(3) as usize;
        let stage = |k: usize, x: f64| {
            if k == refusing {
                approach(x, x + 1.0, 1e-6, &absent)
            } else {
                approach(x, x + 1.0, 1e-6, &paid)
            }
        };
        let out = and_then(and_then(stage(0, 0.0), |x| stage(1, x)), |x| stage(2, x));
        match out {
            Err(NestedRefuse::Refused {
                cause,
                ledger,
                chain,
            }) => {
                assert_eq!(cause, CombinatorRefuse::Unmeasured);
                assert_eq!(ledger.entries().len(), refusing + 1);
                assert_eq!(chain.links().len(), refusing);
            }
            other => panic!("expected the stage-{refusing} refusal, got {other:?}"),
        }
    }
}

#[test]
fn honest_spends_bind_to_an_honest_spend() {
    let mut g = Lcg(23);
    let floor = floor();
    for _ in 0..60 {
        let stages = 1 + g.next(4) as usize;
        let mut out: Nested<f64> = Ok(Converged::unit(0.0));
        for _ in 0..stages {
            // A measured spend of at least the floor per step: each step honest.
            let joules = floor * (1 + g.next(1000)) as f64;
            let m = FixedJouleMeter::from_joules(joules).expect("meter");
            let target = g.next(7) as f64;
            out = and_then(out, |x| approach(x, target, 1e-6, &m));
        }
        let c = out.expect("converges");
        let (mut measured_lo, mut floors_hi) = (0.0, 0.0);
        for e in c.ledger().entries() {
            assert!(e.floor.hi() <= e.joules.lo(), "entry {e:?} not honest");
            assert!(e.floor.lo() > 0.0 || e.joules.hi() == 0.0);
            measured_lo += e.joules.lo();
            floors_hi += e.floor.hi();
        }
        // honest_sum: the summed measured energy is at least the summed floors.
        assert!(c.ledger().total().expect("total").lo() >= floors_hi * (1.0 - 1e-12));
        assert!(measured_lo >= floors_hi * (1.0 - 1e-12));
    }
}

#[test]
fn errors_chain_within_tau_out_plus_l_tau_in() {
    let mut g = Lcg(31);
    let meter = FixedJouleMeter::from_joules(floor() * 2.0).expect("meter");
    for _ in 0..60 {
        let target = g.next(20) as f64 - 10.0;
        let lipschitz = (1 + g.next(5)) as f64;
        let tau_in = 10f64.powi(-(3 + g.next(5) as i32));
        let tau_out = 10f64.powi(-(3 + g.next(5) as i32));
        // Inner: x ≈ target within τ_in. Map: y = L·x. Outer: z ≈ y within τ_out.
        let composite = and_then(approach(0.0, target, tau_in, &meter), |x| {
            approach(0.0, lipschitz * x, tau_out, &meter)
        })
        .expect("both converge");
        let z = *composite.value();
        let links = composite.chain().links();
        assert_eq!(links.len(), 2);
        let (t_in, t_out) = (
            links[0].tolerance().residual().into_inner(),
            links[1].tolerance().residual().into_inner(),
        );
        assert!((z - lipschitz * target).abs() <= t_out + lipschitz * t_in);
    }
}

#[test]
fn the_parity_rows_pin_the_three_laws() {
    let formal = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../umst-formal");
    let parity: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(formal.join("formal_parity.json")).expect("formal_parity.json"),
    )
    .expect("json");
    let rows = parity["statements"].as_array().expect("statements");
    for (id, short) in [
        ("unfold.bind_assoc", "bind_assoc"),
        ("unfold.bind_honest", "bind_honest"),
        ("unfold.error_chain", "error_chain"),
    ] {
        let row = rows
            .iter()
            .find(|r| r["id"] == id)
            .unwrap_or_else(|| panic!("no parity row {id}"));
        assert_eq!(row["lean"]["name"], format!("UMST.UnfoldKleisli.{short}"));
        assert_eq!(row["coq"]["name"], short);
        assert_eq!(row["agda"]["name"], short.replace('_', "-"));
        assert_eq!(row["haskell"]["name"], format!("prop_{short}"));
        let file = row["lean"]["file"].as_str().expect("lean file");
        let text = std::fs::read_to_string(formal.join(file)).expect("lean source");
        assert!(
            text.contains(&format!("theorem {short}")),
            "{file} lacks {short}"
        );
    }
}

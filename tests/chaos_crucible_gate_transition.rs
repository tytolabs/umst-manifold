// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Chaos crucible: the transition gate fails closed on malformed snapshots.
//!
//! Ported from branch `engine-slices` @ `64da191` (tag `backup/engine-slices-20260624`) onto the
//! current gate surface. The branch guarded `ThermodynamicMixFilter::check_transition` directly; on
//! `main` the guard lives in the pure evaluator [`transition_outcome`], which the filter wraps, and in
//! [`thermodynamic_transition_admissible_tol`]. These tests drive every snapshot field and the step
//! `dt` through each non-finite value, both through the stateful filter (whose rejection counter must
//! move) and through the pure predicate, so removing any one guard clause fails a named case.

use umst_manifold::gate::verdict::{ConjunctVerdict, GateRejectReason};
use umst_manifold::gate::{
    thermodynamic_transition_admissible_tol, transition_outcome, ThermodynamicMixFilter,
    ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
};

/// Forward hydration step the gate accepts: the phase 0e mix-calibrated pair, with the free energy
/// lowered by 1 J/kg across the step so the Clausius–Duhem dissipation is strictly positive. (The
/// default substrate witness supplies no reaction enthalpy, so its closure free energy is 0 J/kg at
/// every extent.)
fn clean_pair() -> (ThermodynamicStateSnapshot, ThermodynamicStateSnapshot, f64) {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    new.free_energy = old.free_energy - 1.0;
    (old, new, 28.0 * 24.0 * 3600.0)
}

type FieldSetter = fn(&mut ThermodynamicStateSnapshot, f64);

const FIELDS: [(&str, FieldSetter); 6] = [
    ("density", |s, v| s.density = v),
    ("temperature", |s, v| s.temperature = v),
    ("free_energy", |s, v| s.free_energy = v),
    ("entropy", |s, v| s.entropy = v),
    ("reaction_extent", |s, v| s.reaction_extent = v),
    ("strength", |s, v| s.strength = v),
];

const NON_FINITE: [f64; 3] = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

fn assert_malformed_reject(
    filter: &mut ThermodynamicMixFilter,
    old: &ThermodynamicStateSnapshot,
    new: &ThermodynamicStateSnapshot,
    dt: f64,
    case: &str,
) {
    let before = filter.rejections();
    let outcome = filter.check_transition(old, new, dt);
    assert!(!outcome.is_accepted(), "{case}: must reject");
    assert_eq!(
        outcome.conjunct_verdict(),
        ConjunctVerdict::Rejected(GateRejectReason::MalformedInput),
        "{case}: must reject as malformed input, not on a physics conjunct"
    );
    assert_eq!(
        filter.rejections(),
        before + 1,
        "{case}: filter must count the rejection"
    );
}

#[test]
fn clean_forward_hydration_accepts_with_positive_dissipation() {
    let (old, new, dt) = clean_pair();
    let mut filter = ThermodynamicMixFilter::new();
    let outcome = filter.check_transition(&old, &new, dt);
    assert!(outcome.is_accepted(), "clean forward step must accept");
    assert!(outcome.dissipation > 0.0);
    assert_eq!(filter.acceptances(), 1);
    assert_eq!(filter.rejections(), 0);
}

#[test]
fn every_non_finite_snapshot_field_rejects_on_either_side() {
    let (old, new, dt) = clean_pair();
    let mut filter = ThermodynamicMixFilter::new();
    for (name, set) in FIELDS {
        for v in NON_FINITE {
            let mut bad_new = new;
            set(&mut bad_new, v);
            assert_malformed_reject(&mut filter, &old, &bad_new, dt, &format!("new.{name}={v}"));
            let mut bad_old = old;
            set(&mut bad_old, v);
            assert_malformed_reject(&mut filter, &bad_old, &new, dt, &format!("old.{name}={v}"));
        }
    }
    assert_eq!(filter.acceptances(), 0);
}

#[test]
fn non_positive_temperature_rejects() {
    let (old, new, dt) = clean_pair();
    let mut filter = ThermodynamicMixFilter::new();
    for t in [0.0, -1.0, -273.15] {
        let mut bad = new;
        bad.temperature = t;
        assert_malformed_reject(&mut filter, &old, &bad, dt, &format!("new.temperature={t}"));
    }
}

#[test]
fn non_finite_or_non_positive_step_rejects() {
    let (old, new, _) = clean_pair();
    let mut filter = ThermodynamicMixFilter::new();
    for dt in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_malformed_reject(&mut filter, &old, &new, dt, &format!("dt={dt}"));
        let pure = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(!pure.is_accepted(), "pure evaluator dt={dt} must reject");
    }
}

#[test]
fn pure_predicate_rejects_each_non_finite_argument() {
    let (old, new, dt) = clean_pair();
    let args = [
        old.density,
        old.free_energy,
        old.reaction_extent,
        old.strength,
        new.density,
        new.free_energy,
        new.reaction_extent,
        new.strength,
        new.strength,
        dt,
    ];
    let eval = |a: &[f64; 10]| {
        thermodynamic_transition_admissible_tol(
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
            a[6],
            a[7],
            a[8],
            a[9],
            TRANSITION_TOLERANCE,
        )
    };
    assert!(eval(&args), "clean arguments must be admissible");
    for i in 0..args.len() {
        for v in NON_FINITE {
            let mut bad = args;
            bad[i] = v;
            assert!(!eval(&bad), "argument {i} = {v} must be inadmissible");
        }
    }
}

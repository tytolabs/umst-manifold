// SPDX-FileCopyrightText: 2026 Santhosh Shyamsundar and Santosh Prabhu Shenbagamoorthy
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Proof for cell BARRIER-CBF-ONE-LAW (umst-manifold part), an exact parity test: the f64 credit gate `gate::cbf`
//! is the `γ = 1` case of umst-algebra's exact discrete barrier. The gate admits an update when its Landauer cost `k_B T ln 2 · bits` does
//! not exceed the credit; with the credit `m · k_B T` the barrier value after the update, in units of `k_B T`, is
//! `h = m − ln 2 · bits`, and at `γ = 1` the decay floor is 0, so `discrete_cbf` admits exactly when `h ≥ 0`.
//!
//! `ln 2` is umst-algebra's rational enclosure, so the exact side encloses `h` and decides when both ends agree.
//! Error bound at the f64 edge: the gate forms `credit = m · (k_B T ln 2) / ln 2` and `cost = (k_B T ln 2) · bits`
//! with at most four roundings each, a relative error below `2^-50`; every case keeps `|m − ln 2 · bits|` above
//! `10^-4 · ln 2 · bits`, far outside it, so the f64 verdict must equal the exact one.

use umst_algebra::discrete_cbf::{discrete_cbf, BarrierRate, ExactRat};
use umst_algebra::ln_two_enclosure;
use umst_manifold::ai::cbf::CbfReject;
use umst_manifold::gate::cbf::GateThermodynamicCBF;

const TEMPERATURE_K: f64 = 300.0; // Policy: the reference temperature of the gate's own tests

/// The exact verdict on `h = m − ln 2 · bits` at `γ = 1`, or None when the `ln 2` enclosure leaves it undecided.
fn exact_admits(m: (i128, i128), bits: i128) -> Option<bool> {
    let ln2 = ln_two_enclosure().expect("ln 2 enclosure");
    let q = |n: i64, d: i64| ExactRat::new(i128::from(n), i128::from(d)).expect("rational");
    let m = ExactRat::new(m.0, m.1).expect("rational");
    let bits = ExactRat::new(bits, 1).expect("rational");
    let rate = BarrierRate::try_new(ExactRat::ONE).expect("gamma 1");
    let at = |ln2: ExactRat| {
        let h = m
            .checked_sub(ln2.checked_mul(bits).expect("exact"))
            .expect("exact");
        discrete_cbf(Some(m), h, rate).expect("exact").admits()
    };
    // the smaller ln 2 gives the larger h
    let (high, low) = (
        at(q(ln2.lower.num(), ln2.lower.den())),
        at(q(ln2.upper.num(), ln2.upper.den())),
    );
    (high == low).then_some(high)
}

/// The f64 gate's verdict for credit `m · k_B T` and `bits` resolved, with entropy production high enough that only
/// the credit can refuse.
fn gate_admits(m: f64, bits: f64) -> bool {
    let bit_j = umst_manifold::constants::landauer_bit_energy_joules(TEMPERATURE_K);
    let credit = m * bit_j / std::f64::consts::LN_2;
    let mut gate = GateThermodynamicCBF::new(TEMPERATURE_K, credit);
    match gate.verify_and_deduct_update(1.0e3 * (credit + bit_j * bits), bits) {
        Ok(_) => true,
        Err(CbfReject::InsufficientGlobalEnergyCredit { .. }) => false,
        Err(other) => panic!("only the credit may refuse here: {other:?}"),
    }
}

#[test]
fn the_credit_gate_is_the_gamma_one_barrier() {
    // (m as p/q, bits): each m sits at least 10^-4 · ln 2 · bits away from ln 2 · bits
    let cases: [((i128, i128), i128); 12] = [
        ((69, 100), 1),
        ((70, 100), 1),
        ((1, 2), 1),
        ((1, 1), 1),
        ((138, 100), 2),
        ((139, 100), 2),
        ((693, 100), 10),
        ((694, 100), 10),
        ((6930, 10), 1000),
        ((6933, 10), 1000),
        ((0, 1), 1),
        ((7000, 1), 10_000),
    ];
    let mut admitted = 0;
    for (m, bits) in cases {
        let exact = exact_admits(m, bits).expect("the ln 2 enclosure decides every case");
        let gate = gate_admits(m.0 as f64 / m.1 as f64, bits as f64);
        assert_eq!(gate, exact, "m {}/{} bits {bits}", m.0, m.1);
        admitted += usize::from(exact);
    }
    // both verdicts occur, so the agreement is not one-sided
    assert_eq!(admitted, 6);
}

#[test]
fn zero_bits_cost_nothing_on_either_side() {
    assert_eq!(exact_admits((0, 1), 0), Some(true));
    assert!(gate_admits(0.0, 0.0));
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! A3 consumer pilot (F25-R06 L3) — HAL-relevant surfaces via `umst_runtime::`.
//!
//! Proves the alias path exposes the ThermodynamicCBF + open-system gate bridge that
//! `umst-hal` `cbf-therm` consumes today via `umst_manifold::` (A3-CONSUMER-PILOT-HAL).

use umst_runtime::ai::cbf::ThermodynamicCBF;
use umst_runtime::core::error_boundary::CbfReject;
use umst_runtime::gate::{cbf_open_system_admissible, TRANSITION_TOLERANCE};

#[test]
fn runtime_alias_surfaces_hal_cbf_and_open_system_gate() {
    let temperature_k = 300.0;
    let credit_joules = 1.0e-9;
    let bits = 1.0;
    let erasure = ThermodynamicCBF::new(temperature_k, credit_joules).calculate_landauer_cost(bits);

    assert!(cbf_open_system_admissible(
        erasure,
        bits,
        temperature_k,
        credit_joules,
        TRANSITION_TOLERANCE,
    ));

    let mut cbf = ThermodynamicCBF::new(temperature_k, credit_joules);
    let cost = cbf
        .verify_and_deduct_update(erasure, bits)
        .expect("admissible servo step via alias path");
    assert!(cost > 0.0);
    assert!(cbf.available_credit_joules < credit_joules);
}

#[test]
fn runtime_alias_hal_cbf_reject_surfaces_match_manifold() {
    let mut cbf = ThermodynamicCBF::new(300.0, 0.0);
    let err = cbf
        .verify_and_deduct_update(0.0, 1.0)
        .expect_err("zero credit must reject");
    assert!(matches!(
        err,
        CbfReject::InsufficientGlobalEnergyCredit { .. }
    ));
}

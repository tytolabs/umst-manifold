// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! A3 runtime alias witness — `umst-runtime` re-exports manifold gate SSOT unchanged.

use umst_runtime::gate::{
    GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX, OPEN_RECONCILIATION_DELTAS,
};

#[test]
fn runtime_alias_surfaces_gate_parity_anchor() {
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "149081fa81a6525f");
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"
    );
}

#[test]
fn runtime_alias_preserves_phase0f_reconciliation_closure() {
    assert!(
        OPEN_RECONCILIATION_DELTAS.is_empty(),
        "phase 0f parity lock — no open reconciliation deltas via alias path"
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! A3 runtime alias witness — `umst-runtime` re-exports manifold gate SSOT unchanged.

use umst_runtime::gate::{
    GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX, OPEN_RECONCILIATION_DELTAS,
};

#[test]
fn runtime_alias_surfaces_gate_parity_anchor() {
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "d5608148e29eeabd");
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "d5608148e29eeabd83935988699d08ce1233c3e87f2cd217d658e0c71c7a841e"
    );
}

#[test]
fn runtime_alias_preserves_phase0f_reconciliation_closure() {
    assert!(
        OPEN_RECONCILIATION_DELTAS.is_empty(),
        "phase 0f parity lock — no open reconciliation deltas via alias path"
    );
}

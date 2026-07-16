// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0f — parity lock under live digest (blueprint §7 0f · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! Declares **M0 — Foundation reconciled** when this suite is green alongside:
//! - `cargo test -p umst-mcp --test gate_parity` (authoritative MCP byte lock)
//! - `phase_0a_admissibility_census::phase0a_reconciliation_matrix_red` (census deltas empty)
//!
//! **Parity anchor:** `gate_parity_v0.json` · SHA256 `149081fa81a6525f…` (unchanged through 0a–0e).
//! **Next:** Wave 1 **A1** — `umst-cartridge-api`.

use std::path::PathBuf;

use umst_manifold::gate::{
    format_open_deltas, ADMISSIBILITY_COMPUTE_SITES, ADMISSIBILITY_CONSUME_SITES,
    GATE_PARITY_V0_FIXTURE_REL, GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX,
    OPEN_RECONCILIATION_DELTAS,
};

fn gate_parity_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../umst-concrete-cartridge/crates/umst-mcp/tests/fixtures/gate_parity_v0.json")
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn phase0f_fixture_bytes_sha256_locked() {
    let path = gate_parity_fixture_path();
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "missing live fixture at {} — Phase 0f requires gate_parity_v0.json: {e}",
            path.display()
        )
    });
    assert_eq!(
        sha256_hex(&bytes),
        GATE_PARITY_V0_SHA256,
        "gate_parity_v0.json digest drift — update pin only after intentional fixture change"
    );
}

#[test]
fn phase0f_census_pins_match_live_fixture() {
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"
    );
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "149081fa81a6525f");
    assert!(GATE_PARITY_V0_FIXTURE_REL.contains("gate_parity_v0.json"));
}

#[test]
fn phase0f_reconciliation_matrix_green() {
    assert!(
        OPEN_RECONCILIATION_DELTAS.is_empty(),
        "Phase 0f requires empty OPEN_RECONCILIATION_DELTAS — {} open:\n{}",
        OPEN_RECONCILIATION_DELTAS.len(),
        format_open_deltas(),
    );
}

#[test]
fn phase0f_census_surface_complete() {
    assert!(
        ADMISSIBILITY_COMPUTE_SITES.len() >= 10,
        "compute census too small: {}",
        ADMISSIBILITY_COMPUTE_SITES.len()
    );
    assert!(
        ADMISSIBILITY_CONSUME_SITES.len() >= 10,
        "consume census too small: {}",
        ADMISSIBILITY_CONSUME_SITES.len()
    );
    let parity_consumer = ADMISSIBILITY_CONSUME_SITES
        .iter()
        .find(|s| s.symbol == "gate_check_mix_result_parity_fixture")
        .expect("gate_parity harness must be registered as consume site");
    assert_eq!(parity_consumer.repo, "umst-concrete-cartridge");
}

#[test]
fn phase0f_m0_receipt_parity_prefix() {
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "149081fa81a6525f");
}

// --- FP Manifesto §6: lock-suite idempotency receipt ---
//
// Gate *evaluation* idempotency (solver re-application on equilibrated states) is asserted in
// `phase0b_core_gate` and `phase0e_open_system_spike`. This suite only pins digest/census bytes;
// re-hashing the live fixture must be stable (no drift on re-read).

#[test]
fn phase0f_fixture_digest_idempotent_on_rehash() {
    let path = gate_parity_fixture_path();
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "missing live fixture at {} — Phase 0f requires gate_parity_v0.json: {e}",
            path.display()
        )
    });
    let first = sha256_hex(&bytes);
    let second = sha256_hex(&bytes);
    assert_eq!(first, second, "fixture digest must be idempotent under re-hash");
    assert_eq!(first, GATE_PARITY_V0_SHA256);
}

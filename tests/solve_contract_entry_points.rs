// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! D1 adoption ladder skeleton — enumerates solver entry points cited in the solver-quality audit
//! (findings **#3**, **#5**, **#12**, **#19**). Wave 2 executes ignored envelopes; this test only
//! pins the inventory.

use umst_manifold::solve_report::{SolverEntryPoint, SOLVER_ENTRY_POINTS};

fn finding(entry: &SolverEntryPoint) -> u8 {
    entry.audit_finding
}

#[test]
fn solver_entry_point_inventory_covers_audit_findings() {
    let findings: Vec<u8> = SOLVER_ENTRY_POINTS.iter().map(finding).collect();
    for required in [3u8, 5, 12, 19] {
        assert!(
            findings.contains(&required),
            "SOLVER_ENTRY_POINTS must cite audit finding #{required}"
        );
    }
    assert!(
        SOLVER_ENTRY_POINTS.len() >= 6,
        "expected ≥6 entry points, got {}",
        SOLVER_ENTRY_POINTS.len()
    );
}

/// Wave 2: run each `#[ignore]` verification envelope once (`--ignored --release`) and record in ledger.
#[test]
#[ignore = "Wave 2: execute never-run #[ignore] verification tests; see tests/verification/MANIFEST.toml NEVER-RUN rows"]
fn wave2_execute_never_run_ignored_envelopes() {
    panic!(
        "Skeleton only. Wave 2 command: cargo test -p umst-manifold --features solver-experimental --release -- --ignored"
    );
}

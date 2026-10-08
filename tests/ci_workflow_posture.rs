// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Workflow posture of the public repository (worklist W-63): the setup jobs that reach private sibling
//! repositories do not block the public default branch.
//!
//! This test replaces three steer-wave receipt modules (`b_public_ci_wave25`, `b_public_ci_wave26`,
//! `public_ci_red_pool_receipt_pins`), which pinned steer receipt ids and wave numbers beside this one check.

#[test]
fn private_sibling_setup_does_not_block_the_public_branch() {
    let rust_yml = include_str!("../.github/workflows/rust.yml");
    assert!(
        rust_yml.contains("continue-on-error: true"),
        "W-63: a private-sibling setup step must carry continue-on-error"
    );
}

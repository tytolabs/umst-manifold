// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![cfg(feature = "solver-experimental")]

use umst_manifold::ai::topology::{BetaAlHandshake, PlateauBetaContinuation};

#[test]
fn beta_al_handshake_blocks_step_when_vf_unsettled() {
    let plateau = PlateauBetaContinuation::new(3, 0.01);
    let mut hs = BetaAlHandshake::new(1.0, 0.02, 0.05);
    for lam in [0.0, 0.1, 0.11, 0.12] {
        hs.record_lambda(lam);
    }
    let greys = [0.5_f32, 0.49, 0.48];
    let (beta, stepped, settled) =
        hs.effective_beta(&plateau, 4.0, &greys, 32.0, 0.05, 0.12, false);
    assert!(!settled, "vf_err=0.05 exceeds vf_settle_tol=0.02");
    assert!(!stepped);
    assert!((beta - 1.0).abs() < 1e-6);
}

#[test]
fn beta_al_handshake_steps_when_settled() {
    let plateau = PlateauBetaContinuation::new(3, 0.01);
    let mut hs = BetaAlHandshake::new(1.0, 0.02, 0.05);
    for lam in [0.5, 0.5001, 0.5002, 0.5003] {
        hs.record_lambda(lam);
    }
    let greys = [0.5_f32, 0.49, 0.48];
    let (beta, stepped, settled) =
        hs.effective_beta(&plateau, 4.0, &greys, 32.0, 0.01, 0.5003, false);
    assert!(settled);
    assert!(stepped);
    assert!(beta > 1.0);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R14-0 — rank-indexed parity gate integration tests.

use umst_algebra_burn::parity_rank::{
    close, Evidence, ParityClaim, ParityRank,
};
use umst_algebra_burn::{compare_host_scalar, rank1::RANK1_PLUS_COMPARISON_EPS, GoldenVerdict};

#[test]
fn r14_rank0_cannot_inhabit_rank1_field_gate() {
    let claim = ParityClaim {
        rank: ParityRank::Rank0Pointwise,
        verdict: GoldenVerdict::Equal,
        evidence: Evidence {
            probe_id: "constitutive_scalar",
            host_scalar: Some(35.689_57),
        },
    };
    assert!(close(ParityRank::Rank0Pointwise, &claim).is_ok());
    assert!(close(ParityRank::Rank1Field, &claim).is_err());
    assert!(close(ParityRank::Rank2Complex, &claim).is_err());
    assert!(close(ParityRank::Coupled, &claim).is_err());
}

#[test]
fn r14_perturbation_verdict_blocks_close_even_at_matching_rank() {
    let eps = RANK1_PLUS_COMPARISON_EPS;
    let claim = ParityClaim {
        rank: ParityRank::Rank0Pointwise,
        verdict: compare_host_scalar(2.5, 2.0, eps),
        evidence: Evidence {
            probe_id: "perturbation",
            host_scalar: Some(2.5),
        },
    };
    assert!(matches!(
        compare_host_scalar(2.5, 2.0, eps),
        GoldenVerdict::Differs { .. }
    ));
    assert!(close(ParityRank::Rank0Pointwise, &claim).is_err());
}

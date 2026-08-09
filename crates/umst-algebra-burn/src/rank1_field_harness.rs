// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R15-B1 — per-surface rank-1 field comparison instrument.
//!
//! Lattice field evaluation (host projection) vs monolith field golden. B2+ surfaces call
//! [`rank1_field_parity_claim`] + [`rank1_field_gate_closes`] — do not flip surface flags here.

use crate::golden_harness::{compare_host_relative, GoldenVerdict};
use crate::parity_rank::{close, Evidence, ParityClaim, ParityRank};
use crate::rank1::RANK1_PLUS_COMPARISON_EPS;

/// Default relative tolerance for rank-1 field golden parity (provenanced f32 cold boundary).
pub const RANK1_FIELD_DEFAULT_RTOL: f64 = RANK1_PLUS_COMPARISON_EPS;

/// Typed verdict — lattice host projection vs monolith golden.
#[must_use]
pub fn rank1_field_parity_verdict(actual: f64, golden: f64, rtol: f64) -> GoldenVerdict {
    compare_host_relative(actual, golden, rtol)
}

/// Whether lattice field host closes against monolith golden within `rtol`.
#[must_use]
pub fn rank1_field_parity_closes(actual: f64, golden: f64, rtol: f64) -> bool {
    rank1_field_parity_verdict(actual, golden, rtol).closes_deferred()
}

/// Build a rank-1 [`ParityClaim`] for gate close attempts.
#[must_use]
pub fn rank1_field_parity_claim(
    actual: f64,
    golden: f64,
    rtol: f64,
    probe_id: &'static str,
) -> ParityClaim {
    ParityClaim {
        rank: ParityRank::Rank1Field,
        verdict: rank1_field_parity_verdict(actual, golden, rtol),
        evidence: Evidence {
            probe_id,
            host_scalar: Some(actual),
        },
    }
}

/// Close the rank-1 field gate — rank-0 claims are refused by rank index.
#[must_use]
pub fn rank1_field_gate_closes(claim: &ParityClaim) -> Result<(), crate::parity_rank::CloseError> {
    close(ParityRank::Rank1Field, claim)
}

/// Perturbation witness: matching hosts close; perturbed host reports `Differs`.
#[must_use]
pub fn rank1_field_perturbation_witness() -> bool {
    let rtol = RANK1_FIELD_DEFAULT_RTOL;
    let base = rank1_field_parity_closes(2.0, 2.0, rtol);
    let pert = rank1_field_parity_closes(2.5, 2.0, rtol);
    base && !pert
}

#[cfg(test)]
mod rank1_field_harness {
    use super::*;
    use crate::golden_harness::compare_host_scalar;
    use crate::parity_rank::{close, CloseError, ParityRank};

    #[test]
    fn rank1_field_harness_parity_closes_on_match() {
        let rtol = RANK1_FIELD_DEFAULT_RTOL;
        assert!(rank1_field_parity_closes(35.689_57, 35.689_57, rtol));
        let v = rank1_field_parity_verdict(35.689_57, 35.689_58, rtol);
        assert!(v.closes_deferred());
    }

    #[test]
    fn rank1_field_harness_perturbation_witness_measured() {
        assert!(rank1_field_perturbation_witness());
        let rtol = RANK1_FIELD_DEFAULT_RTOL;
        assert!(matches!(
            rank1_field_parity_verdict(2.5, 2.0, rtol),
            GoldenVerdict::Differs { .. }
        ));
    }

    #[test]
    fn rank1_field_harness_rank0_claim_rejected_at_rank1_gate() {
        let rank0 = ParityClaim {
            rank: ParityRank::Rank0Pointwise,
            verdict: compare_host_scalar(2.0, 2.0, RANK1_FIELD_DEFAULT_RTOL),
            evidence: Evidence {
                probe_id: "constitutive_scalar",
                host_scalar: Some(2.0),
            },
        };
        assert!(close(ParityRank::Rank0Pointwise, &rank0).is_ok());
        assert_eq!(
            rank1_field_gate_closes(&rank0),
            Err(CloseError::RankMismatch {
                gate: ParityRank::Rank1Field,
                claim: ParityRank::Rank0Pointwise,
            })
        );
    }

    #[test]
    fn rank1_field_harness_rank1_claim_closes_gate() {
        let claim = rank1_field_parity_claim(4.0, 4.0, RANK1_FIELD_DEFAULT_RTOL, "field_probe");
        assert!(rank1_field_gate_closes(&claim).is_ok());
    }

    /// B2 continuum strength — example surface call pattern (flags flip in B2, not B1).
    #[test]
    fn rank1_field_harness_b2_strength_surface_call_pattern() {
        let golden_fc_mpa = 35.689_57;
        let lattice_host = golden_fc_mpa;
        let rtol = 1e-4;
        let claim = rank1_field_parity_claim(
            lattice_host,
            golden_fc_mpa,
            rtol,
            "strength_fc_mpa",
        );
        assert!(rank1_field_gate_closes(&claim).is_ok());
        assert!(!rank1_field_parity_closes(lattice_host + 1.0, golden_fc_mpa, rtol));
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Rank-indexed parity claims — R14-0 SSOT.
//!
//! A rank-0 measurement can never inhabit a rank-1+ gate.

use crate::golden_harness::GoldenVerdict;

/// Parity is indexed by the rank at which it was measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParityRank {
    /// Scalar constitutive law at a material point (ψ, 𝒟 @ point).
    Rank0Pointwise,
    /// Rank-1 field carrier over nodal/edge DOFs.
    Rank1Field,
    /// Rank-2 complex (DEC 2-forms, stress tensors).
    Rank2Complex,
    /// Fully coupled THMC / conservation-glued system.
    Coupled,
}

/// Minimal evidence bundle for receipts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Evidence {
    /// Probe or fixture id for audit.
    pub probe_id: &'static str,
    /// Optional host scalar when rank-0.
    pub host_scalar: Option<f64>,
}

/// A measured parity claim at a specific rank.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParityClaim {
    /// Rank at which this claim was measured.
    pub rank: ParityRank,
    /// Typed verdict from golden harness.
    pub verdict: GoldenVerdict,
    /// Audit evidence.
    pub evidence: Evidence,
}

/// Why a gate refused to close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseError {
    /// Claim rank does not match gate rank.
    RankMismatch {
        /// Gate rank required.
        gate: ParityRank,
        /// Claim rank offered.
        claim: ParityRank,
    },
    /// Verdict does not license close.
    VerdictNotClosing,
}

/// Close a gate **only** when `claim.rank == gate` and verdict closes.
#[must_use]
pub fn close(gate: ParityRank, claim: &ParityClaim) -> Result<(), CloseError> {
    if claim.rank != gate {
        return Err(CloseError::RankMismatch {
            gate,
            claim: claim.rank,
        });
    }
    if claim.verdict.closes_deferred() {
        Ok(())
    } else {
        Err(CloseError::VerdictNotClosing)
    }
}

/// Human-readable rank label for receipts.
#[must_use]
pub const fn rank_label(rank: ParityRank) -> &'static str {
    match rank {
        ParityRank::Rank0Pointwise => "rank0_pointwise",
        ParityRank::Rank1Field => "rank1_field",
        ParityRank::Rank2Complex => "rank2_complex",
        ParityRank::Coupled => "coupled",
    }
}

/// Exhaustive rank dispatch — no default arm.
#[must_use]
pub fn rank_ordinal(rank: ParityRank) -> u8 {
    match rank {
        ParityRank::Rank0Pointwise => 0,
        ParityRank::Rank1Field => 1,
        ParityRank::Rank2Complex => 2,
        ParityRank::Coupled => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::golden_harness::{compare_host_scalar, rank1_eps};

    fn rank0_equal_claim() -> ParityClaim {
        ParityClaim {
            rank: ParityRank::Rank0Pointwise,
            verdict: compare_host_scalar(1.0, 1.0, rank1_eps()),
            evidence: Evidence {
                probe_id: "rank0_unit",
                host_scalar: Some(1.0),
            },
        }
    }

    #[test]
    fn rank0_claim_closes_rank0_gate() {
        let claim = rank0_equal_claim();
        assert!(close(ParityRank::Rank0Pointwise, &claim).is_ok());
    }

    #[test]
    fn rank0_claim_rejected_at_rank1_gate() {
        let claim = rank0_equal_claim();
        let err = close(ParityRank::Rank1Field, &claim).unwrap_err();
        assert_eq!(
            err,
            CloseError::RankMismatch {
                gate: ParityRank::Rank1Field,
                claim: ParityRank::Rank0Pointwise,
            }
        );
    }

    #[test]
    fn rank_ordinal_exhaustive() {
        assert_eq!(rank_ordinal(ParityRank::Rank0Pointwise), 0);
        assert_eq!(rank_ordinal(ParityRank::Rank1Field), 1);
        assert_eq!(rank_ordinal(ParityRank::Rank2Complex), 2);
        assert_eq!(rank_ordinal(ParityRank::Coupled), 3);
    }

    #[test]
    fn diff_verdict_does_not_close() {
        let claim = ParityClaim {
            rank: ParityRank::Rank0Pointwise,
            verdict: compare_host_scalar(2.0, 3.0, rank1_eps()),
            evidence: Evidence {
                probe_id: "divergent",
                host_scalar: Some(2.0),
            },
        };
        assert_eq!(
            close(ParityRank::Rank0Pointwise, &claim),
            Err(CloseError::VerdictNotClosing)
        );
    }
}

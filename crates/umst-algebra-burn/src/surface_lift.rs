// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Atom-surface burn scalar lift measurement — R13 burn backend closure.

use crate::golden_harness::{compare_burn_lift_to_golden, rank1_eps, GoldenVerdict};

/// Whether burn tensor lift of `host` closes against CON golden within rank-1+ eps.
#[must_use]
pub fn burn_scalar_lift_closes(host: f64, golden: f64) -> bool {
    compare_burn_lift_to_golden(host, golden, rank1_eps()).closes_deferred()
}

/// Raw verdict for receipts.
#[must_use]
pub fn burn_scalar_lift_verdict(host: f64, golden: f64) -> GoldenVerdict {
    compare_burn_lift_to_golden(host, golden, rank1_eps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_scalar_closes() {
        assert!(burn_scalar_lift_closes(35.689_57, 35.689_57));
    }

    #[test]
    fn perturbation_does_not_close() {
        assert!(!burn_scalar_lift_closes(35.689_57, 40.0));
    }
}

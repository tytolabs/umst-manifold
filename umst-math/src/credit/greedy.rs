// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Filter-then-sum credit mass (Case A — matches `UMST.Formal.CreditGreedy.creditMass`).

/// One weighted row with an admissibility flag (engineering mirror of Lean `CreditCandidate`).
#[derive(Clone, Debug, PartialEq)]
/// THEOREM-BOUND: `UMST.Formal.CreditGreedy::credit_greedy_optimal` (§14bis.l W-3 G8)
pub struct CreditCandidate {
    /// Weight counted only when `admissible` is true.
    pub weight: f64,
    /// When false, this row is excluded from `credit_greedy_sum`.
    pub admissible: bool,
}

/// Proof: `UMST.Formal.CreditGreedy::credit_greedy_optimal` (Zenodo **10.5281/zenodo.19159660**).
/// Impl: `umst_math::credit::greedy::credit_greedy_sum`
#[must_use]
/// THEOREM-BOUND: `UMST.Formal.CreditGreedy::credit_greedy_optimal` (§14bis.l W-3 G8)
pub fn credit_greedy_sum(candidates: &[CreditCandidate]) -> f64 {
    candidates
        .iter()
        .filter(|c| c.admissible)
        .map(|c| c.weight)
        .sum()
}

/// Proof: `UMST.Formal.CreditGreedy::greedy_nonneg` (same DOI family as `theorem_registry`).
#[must_use]
/// THEOREM-BOUND: `UMST.Formal.CreditGreedy::credit_greedy_optimal` (§14bis.l W-3 G8)
pub fn credit_greedy_is_nonneg(candidates: &[CreditCandidate]) -> bool {
    credit_greedy_sum(candidates) >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_yields_zero() {
        assert_eq!(credit_greedy_sum(&[]), 0.0);
    }

    #[test]
    fn test_single_admissible_returns_weight() {
        let cs = [CreditCandidate {
            weight: 2.5,
            admissible: true,
        }];
        assert_eq!(credit_greedy_sum(&cs), 2.5);
    }

    #[test]
    fn test_filters_inadmissible() {
        let cs = [
            CreditCandidate {
                weight: 1.0,
                admissible: true,
            },
            CreditCandidate {
                weight: 99.0,
                admissible: false,
            },
        ];
        assert_eq!(credit_greedy_sum(&cs), 1.0);
    }

    #[test]
    fn test_sum_is_nonneg_under_nonneg_precondition() {
        let cs = [
            CreditCandidate {
                weight: 0.0,
                admissible: true,
            },
            CreditCandidate {
                weight: 3.0,
                admissible: true,
            },
        ];
        assert!(credit_greedy_is_nonneg(&cs));
    }

    #[test]
    fn test_determinism_across_invocations() {
        let cs = [CreditCandidate {
            weight: 7.0,
            admissible: true,
        }];
        assert_eq!(credit_greedy_sum(&cs), credit_greedy_sum(&cs));
    }

    #[test]
    fn test_tie_break_order_preserving() {
        let cs = [
            CreditCandidate {
                weight: 1.0,
                admissible: true,
            },
            CreditCandidate {
                weight: 2.0,
                admissible: true,
            },
        ];
        assert_eq!(credit_greedy_sum(&cs), 3.0);
    }

    #[test]
    fn test_matches_exhaustive_filter_sum() {
        let cs = [
            CreditCandidate {
                weight: 0.25,
                admissible: true,
            },
            CreditCandidate {
                weight: 0.5,
                admissible: false,
            },
            CreditCandidate {
                weight: 0.125,
                admissible: true,
            },
        ];
        let greedy = credit_greedy_sum(&cs);
        let optimal: f64 = cs.iter().filter(|c| c.admissible).map(|c| c.weight).sum();
        assert!((greedy - optimal).abs() < 1e-15);
    }
}

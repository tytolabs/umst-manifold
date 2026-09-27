// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// registration_request: pub mod problem_size_escalation;
//! Problem-size escalation evidence for unfold-driven solves.
//!
//! `n_unknowns` counts toward escalation witnesses; it is **not** an unfold stop condition.
//! The coalgebraic unfold in [`crate::solve_combinator`] stops only for
//! [`SolveOutcome::Converged`](crate::solve_combinator::SolveOutcome::Converged),
//! [`SolveOutcome::Stalled`](crate::solve_combinator::SolveOutcome::Stalled), or
//! [`SolveOutcome::BudgetSpent`](crate::solve_combinator::SolveOutcome::BudgetSpent)
//! (see `unfold` in `solve_combinator.rs`). This module does not run a solver loop and does
//! not call excitement selection.
//!
//! [`Escalation`] is typed evidence for a caller to choose `Refine(f64)` or a restart — not
//! a substitute for those unfold outcomes.

/// Witness that iteration count has crossed the problem-size threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemSizeEvidence {
    /// Degrees of freedom (or analogous problem size) used as the orthogonality threshold.
    pub n_unknowns: u64,
    /// Unfold or outer-loop steps already taken when the witness was recorded.
    pub steps_taken: u64,
}

/// True when at least one unknown is declared and steps have reached or passed that count.
pub fn steps_past_unknowns(evidence: &ProblemSizeEvidence) -> bool {
    evidence.n_unknowns > 0 && evidence.steps_taken >= evidence.n_unknowns
}

/// Escalation kinds derived from problem-size witnesses (not unfold terminal outcomes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Escalation {
    /// Iteration count has reached the unknown count; orthogonality may be lost — refine or restart.
    OrthogonalityLoss {
        /// Size and step count at the crossing.
        evidence: ProblemSizeEvidence,
    },
}

/// Returns [`Escalation::OrthogonalityLoss`] only when [`steps_past_unknowns`] holds.
pub fn escalation_from_evidence(evidence: ProblemSizeEvidence) -> Option<Escalation> {
    if steps_past_unknowns(&evidence) {
        Some(Escalation::OrthogonalityLoss { evidence })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_below_unknown_count_is_not_escalation() {
        let evidence = ProblemSizeEvidence {
            n_unknowns: 4,
            steps_taken: 3,
        };
        assert!(!steps_past_unknowns(&evidence));
        assert_eq!(escalation_from_evidence(evidence), None);
    }

    #[test]
    fn steps_at_unknown_count_is_orthogonality_loss() {
        let evidence = ProblemSizeEvidence {
            n_unknowns: 4,
            steps_taken: 4,
        };
        assert!(steps_past_unknowns(&evidence));
        assert_eq!(
            escalation_from_evidence(evidence),
            Some(Escalation::OrthogonalityLoss { evidence })
        );
    }

    #[test]
    fn zero_unknowns_never_escalates() {
        let evidence = ProblemSizeEvidence {
            n_unknowns: 0,
            steps_taken: 100,
        };
        assert!(!steps_past_unknowns(&evidence));
        assert_eq!(escalation_from_evidence(evidence), None);
    }
}

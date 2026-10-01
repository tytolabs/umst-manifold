// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Budgeted control-flow iteration via fold — explicit `max_iters` cap with typed certificate.

use core::ops::ControlFlow;

/// Explicit iteration budget (cap is not renamed or hidden).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlFlowIterateBudget {
    pub max_iters: usize,
}

impl ControlFlowIterateBudget {
    #[must_use]
    pub const fn new(max_iters: usize) -> Self {
        Self { max_iters }
    }
}

/// Stop certificate for a bounded control-flow unfold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlowIterateCertificate {
    Converged {
        completed: usize,
        residual: f64,
    },
    Stalled {
        completed: usize,
        residual: f64,
    },
    BudgetSpent {
        completed: usize,
        residual: f64,
    },
}

impl ControlFlowIterateCertificate {
    #[must_use]
    pub const fn completed(self) -> usize {
        match self {
            Self::Converged { completed, .. }
            | Self::Stalled { completed, .. }
            | Self::BudgetSpent { completed, .. } => completed,
        }
    }
}

/// Run at most `budget.max_iters` steps; stop early on [`ControlFlow::Break`].
#[must_use]
pub fn control_flow_iterate_budget<S>(
    budget: ControlFlowIterateBudget,
    state: &mut S,
    mut step: impl FnMut(&mut S) -> ControlFlow<(), ()>,
    mut residual_of: impl FnMut(&S) -> f64,
    mut converged_predicate: impl FnMut(&S) -> bool,
) -> ControlFlowIterateCertificate {
    let max_iters = budget.max_iters;
    if max_iters == 0 {
        return ControlFlowIterateCertificate::BudgetSpent {
            completed: 0,
            residual: residual_of(state),
        };
    }

    #[derive(Copy, Clone)]
    enum Early {
        Converged(usize),
        Stalled(usize),
    }

    let folded = (0..max_iters).try_fold((), |(), i| {
        match step(state) {
            ControlFlow::Break(()) => {
                if converged_predicate(state) {
                    Err(Early::Converged(i + 1))
                } else {
                    Err(Early::Stalled(i + 1))
                }
            }
            ControlFlow::Continue(()) => Ok(()),
        }
    });

    match folded {
        Err(Early::Converged(completed)) => ControlFlowIterateCertificate::Converged {
            completed,
            residual: residual_of(state),
        },
        Err(Early::Stalled(completed)) => ControlFlowIterateCertificate::Stalled {
            completed,
            residual: residual_of(state),
        },
        Ok(()) => ControlFlowIterateCertificate::BudgetSpent {
            completed: max_iters,
            residual: residual_of(state),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_flow_iterate_budget_certificate_converged_and_budget_spent() {
        let mut acc = 0_u32;
        let spent = control_flow_iterate_budget(
            ControlFlowIterateBudget::new(5),
            &mut acc,
            |s| {
                *s += 1;
                ControlFlow::Continue(())
            },
            |s| f64::from(*s),
            |_| false,
        );
        assert!(matches!(
            spent,
            ControlFlowIterateCertificate::BudgetSpent {
                completed: 5,
                residual: 5.0
            }
        ));

        let mut x = 0_i32;
        let cert = control_flow_iterate_budget(
            ControlFlowIterateBudget::new(20),
            &mut x,
            |v| {
                *v += 1;
                if *v >= 4 {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            },
            |v| f64::from(*v),
            |v| *v >= 4,
        );
        assert!(matches!(
            cert,
            ControlFlowIterateCertificate::Converged {
                completed: 4,
                residual: 4.0
            }
        ));
    }
}

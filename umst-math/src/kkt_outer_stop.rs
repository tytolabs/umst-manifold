// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// registration_request: pub mod kkt_outer_stop;
//! Outer-loop stop predicate under a second-law budget: termination is KKT/optimality or budget spent.
//!
//! Outer optimisation loops stop only on an optimality certificate ([`OuterStop::Optimal`]) or when
//! the budget is exhausted ([`OuterStop::BudgetSpent`]). A cell count is not a stop condition.
//! This module does not run a solver loop and does not call excitement selection.

/// Refusal when inputs to [`outer_stop`] are not admissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OuterStopRefuse {
    /// KKT residual is not finite.
    NonFiniteResidual,
    /// Tolerance is not finite or is not strictly positive.
    NonPositiveTolerance,
    /// Budget progress witness is not finite.
    NonFiniteProgress,
}

/// KKT / optimality certificate: residual at or below the declared tolerance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KktCertificate {
    /// Measured KKT (or equivalent optimality) residual.
    pub residual: f64,
    /// Positive finite tolerance against which residual was certified.
    pub tolerance: f64,
}

/// Terminal outer-loop outcome, when [`outer_stop`] returns `Ok(Some(_))`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OuterStop {
    /// Optimality: residual within tolerance.
    Optimal {
        /// Certificate fields at stop.
        certificate: KktCertificate,
    },
    /// Budget exhausted before optimality; progress records spend fraction or analogous witness.
    BudgetSpent {
        /// Finite progress witness at budget stop.
        progress: f64,
    },
}

/// Decide whether an outer loop should stop, continue, or refuse invalid inputs.
///
/// Checks residual finiteness, then tolerance, then progress finiteness. If
/// `kkt_residual <= tolerance`, returns optimality; else if `budget_exhausted`, returns
/// budget stop; else returns `Ok(None)` to continue the loop.
pub fn outer_stop(
    kkt_residual: f64,
    tolerance: f64,
    budget_exhausted: bool,
    progress: f64,
) -> Result<Option<OuterStop>, OuterStopRefuse> {
    if !kkt_residual.is_finite() {
        return Err(OuterStopRefuse::NonFiniteResidual);
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(OuterStopRefuse::NonPositiveTolerance);
    }
    if !progress.is_finite() {
        return Err(OuterStopRefuse::NonFiniteProgress);
    }
    if kkt_residual <= tolerance {
        return Ok(Some(OuterStop::Optimal {
            certificate: KktCertificate {
                residual: kkt_residual,
                tolerance,
            },
        }));
    }
    if budget_exhausted {
        return Ok(Some(OuterStop::BudgetSpent { progress }));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric_tolerance::transition_tolerance_f64;

    #[test]
    fn residual_within_tolerance_is_optimal() {
        assert_eq!(
            outer_stop(1e-8, 1e-6, false, 0.2),
            Ok(Some(OuterStop::Optimal {
                certificate: KktCertificate {
                    residual: 1e-8,
                    tolerance: transition_tolerance_f64(),
                },
            }))
        );
    }

    #[test]
    fn residual_above_tolerance_and_budget_remaining_continues() {
        let out = outer_stop(1.0, 1e-6, false, 0.2);
        assert_eq!(out, Ok(None));
    }

    #[test]
    fn budget_exhausted_before_optimality_stops_with_progress() {
        let out = outer_stop(1.0, 1e-6, true, 0.4);
        assert_eq!(
            out,
            Ok(Some(OuterStop::BudgetSpent { progress: 0.4 }))
        );
    }

    #[test]
    fn zero_tolerance_is_refused() {
        let out = outer_stop(1.0, 0.0, false, 0.2);
        assert_eq!(out, Err(OuterStopRefuse::NonPositiveTolerance));
    }

    #[test]
    fn nan_residual_is_refused() {
        let out = outer_stop(f64::NAN, 1e-6, false, 0.2);
        assert_eq!(out, Err(OuterStopRefuse::NonFiniteResidual));
    }
}

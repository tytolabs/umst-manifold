// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Rank-1+ golden comparison harness — typed verdicts for CON Burn parity.

use crate::rank1::RANK1_PLUS_COMPARISON_EPS;
use crate::tensor::{BurnNdArrayAlgebra, BurnTensorField, DefaultBackend};

/// Typed comparison verdict (R13-2 SSOT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GoldenVerdict {
    /// Exact equality (rank-0 only).
    Equal,
    /// Within declared tolerance.
    WithinTolerance {
        /// Metric name for receipts.
        metric: &'static str,
        /// Provenanced epsilon.
        eps: f64,
        /// Measured |actual − golden|.
        delta: f64,
    },
    /// Measured divergence beyond tolerance.
    Differs {
        /// |actual − golden|.
        delta: f64,
    },
    /// Comparison not defined for this pair.
    NotComparable {
        /// Reason string for receipts.
        reason: &'static str,
    },
}

impl GoldenVerdict {
    /// Whether this verdict licenses a deferred-flag CLOSE.
    #[must_use]
    pub const fn closes_deferred(self) -> bool {
        matches!(self, Self::Equal | Self::WithinTolerance { .. })
    }
}

/// Compare host scalars with relative tolerance (`|actual − golden| / max(|golden|, 1e-12)`).
#[must_use]
pub fn compare_host_relative(actual: f64, golden: f64, rtol: f64) -> GoldenVerdict {
    if !(actual.is_finite() && golden.is_finite() && rtol.is_finite() && rtol >= 0.0) {
        return GoldenVerdict::NotComparable {
            reason: "non-finite or negative rtol",
        };
    }
    if actual == golden {
        return GoldenVerdict::Equal;
    }
    let denom = golden.abs().max(1e-12);
    let rel_delta = (actual - golden).abs() / denom;
    if rel_delta <= rtol {
        GoldenVerdict::WithinTolerance {
            metric: "relative",
            eps: rtol,
            delta: rel_delta,
        }
    } else {
        GoldenVerdict::Differs { delta: rel_delta }
    }
}

/// Compare host scalars with provenanced rank-1+ epsilon.
#[must_use]
pub fn compare_host_scalar(actual: f64, golden: f64, eps: f64) -> GoldenVerdict {
    if !(actual.is_finite() && golden.is_finite() && eps.is_finite() && eps >= 0.0) {
        return GoldenVerdict::NotComparable {
            reason: "non-finite or negative eps",
        };
    }
    if actual == golden {
        return GoldenVerdict::Equal;
    }
    let delta = (actual - golden).abs();
    if delta <= eps {
        GoldenVerdict::WithinTolerance {
            metric: "abs",
            eps,
            delta,
        }
    } else {
        GoldenVerdict::Differs { delta }
    }
}

/// Project a burn tensor atom field and compare to CON golden.
#[must_use]
pub fn compare_burn_tensor_scalar_projection(
    field: &BurnTensorField<DefaultBackend>,
    golden: f64,
    eps: f64,
) -> GoldenVerdict {
    compare_host_scalar(field.to_host_scalar(), golden, eps)
}

/// Lift host scalar through burn tensor carrier and compare roundtrip to golden.
#[must_use]
pub fn compare_burn_lift_to_golden(host: f64, golden: f64, eps: f64) -> GoldenVerdict {
    let device = Default::default();
    let field = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, host);
    compare_burn_tensor_scalar_projection(&field, golden, eps)
}

/// Default provenanced epsilon for rank-1+ comparisons.
#[must_use]
pub const fn rank1_eps() -> f64 {
    RANK1_PLUS_COMPARISON_EPS
}

/// Whether rank-1+ tensor path matches scalar at a probe via `mul` monomorphization.
#[must_use]
pub fn rank1_mul_path_matches_scalar(lhs: f64, rhs: f64, eps: f64) -> GoldenVerdict {
    use umst_cartridge_api::{ScalarAlgebra, TensorAlgebra};

    let scalar_product = ScalarAlgebra::mul(lhs, rhs);
    let device = Default::default();
    let l = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, lhs);
    let r = BurnTensorField::<DefaultBackend>::from_host_scalar(&device, rhs);
    let burn_product = <BurnNdArrayAlgebra as TensorAlgebra>::mul(l, r);
    compare_host_scalar(burn_product.to_host_scalar(), scalar_product, eps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_equal_and_within_tolerance() {
        assert_eq!(
            compare_host_scalar(2.0, 2.0, rank1_eps()),
            GoldenVerdict::Equal
        );
        let v = compare_host_scalar(2.0, 2.0005, 1e-3);
        assert!(matches!(v, GoldenVerdict::WithinTolerance { .. }));
        let d = compare_host_scalar(2.0, 3.0, 1e-3);
        assert!(matches!(d, GoldenVerdict::Differs { .. }));
    }

    #[test]
    fn perturbation_reports_differs() {
        let base = compare_host_scalar(1.0, 1.0, rank1_eps());
        let pert = compare_host_scalar(1.5, 1.0, rank1_eps());
        assert_eq!(base, GoldenVerdict::Equal);
        assert!(matches!(pert, GoldenVerdict::Differs { .. }));
    }
}

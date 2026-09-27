// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Typed structural refusals before any iterative solve (SOLVER_COMPOSITION_DESIGN §2.2).
//!
//! Each variant carries a witness — never a bare string — so model defects are recorded
//! as typed absences and never masquerade as divergence.

/// Why a refusal witness could not be certified from raw coefficients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolverRefusalCertifyRefuse {
    /// A scalar or vector entry was NaN or infinite.
    NonFinite,
    /// A required scalar was not strictly positive.
    NonPositive,
    /// Least-squares residual was negative (not a valid residual witness).
    NegativeResidual,
    /// Working precision can certify `tol` (`κ_hi · ε_work < tol`); not insufficient.
    PrecisionAdequate,
}

/// Mechanism modes with the load component lying in the mechanism space (Maxwell–Calladine).
#[derive(Clone, Debug, PartialEq)]
pub struct Mechanism {
    /// Independent mechanism mode shapes (e.g. bar axial motions).
    pub modes: Vec<f64>,
    /// Load projected onto the mechanism space (non-orthogonal witness).
    pub load_component: Vec<f64>,
}

impl Mechanism {
    /// Certifies a mechanism refusal when every mode and load entry is finite.
    pub fn certified(
        modes: Vec<f64>,
        load_component: Vec<f64>,
    ) -> Result<Self, SolverRefusalCertifyRefuse> {
        certify_finite_slice(&modes)?;
        certify_finite_slice(&load_component)?;
        Ok(Self {
            modes,
            load_component,
        })
    }
}

/// Rigid-body or support modes not removed by the support set.
#[derive(Clone, Debug, PartialEq)]
pub struct Underconstrained {
    /// Mode shapes spanning the missing constraint space (e.g. six RBMs in 3D).
    pub modes: Vec<f64>,
}

impl Underconstrained {
    /// Certifies an underconstrained refusal when every mode entry is finite.
    pub fn certified(modes: Vec<f64>) -> Result<Self, SolverRefusalCertifyRefuse> {
        certify_finite_slice(&modes)?;
        Ok(Self { modes })
    }
}

/// Load component outside the range of a semidefinite stiffness operator.
#[derive(Clone, Debug, PartialEq)]
pub struct Inconsistent {
    /// CGLS/LSQR least-squares residual (stall value a capped CG can hide).
    pub least_squares_residual: f64,
    /// Witness direction in the nullspace of the range projection.
    pub nullspace_witness: Vec<f64>,
}

impl Inconsistent {
    /// Certifies inconsistency when the residual is finite, non-negative, and the witness is finite.
    pub fn certified(
        least_squares_residual: f64,
        nullspace_witness: Vec<f64>,
    ) -> Result<Self, SolverRefusalCertifyRefuse> {
        if !least_squares_residual.is_finite() {
            return Err(SolverRefusalCertifyRefuse::NonFinite);
        }
        if least_squares_residual < 0.0 {
            return Err(SolverRefusalCertifyRefuse::NegativeResidual);
        }
        certify_finite_slice(&nullspace_witness)?;
        Ok(Self {
            least_squares_residual,
            nullspace_witness,
        })
    }
}

/// Working precision cannot certify the requested tolerance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrecisionInsufficient {
    /// Upper conditioning bound used in the precision need inequality.
    pub kappa_hi: f64,
    /// Working precision ε_work.
    pub eps_work: f64,
    /// Target tolerance that cannot be certified at this precision.
    pub tol: f64,
}

impl PrecisionInsufficient {
    /// Certifies insufficient precision when `κ_hi · ε_work ≥ tol` and all scalars are finite and positive.
    ///
    /// Returns [`SolverRefusalCertifyRefuse::PrecisionAdequate`] when `κ_hi · ε_work < tol`.
    pub fn certified(
        kappa_hi: f64,
        eps_work: f64,
        tol: f64,
    ) -> Result<Self, SolverRefusalCertifyRefuse> {
        let k = certify_positive_finite(kappa_hi)?;
        let e = certify_positive_finite(eps_work)?;
        let t = certify_positive_finite(tol)?;
        let product = k * e;
        if !product.is_finite() {
            return Err(SolverRefusalCertifyRefuse::NonFinite);
        }
        if product < t {
            return Err(SolverRefusalCertifyRefuse::PrecisionAdequate);
        }
        Ok(Self {
            kappa_hi: k,
            eps_work: e,
            tol: t,
        })
    }
}

/// Structural refusal taxonomy for pre-iteration checks (§2.2).
#[derive(Clone, Debug, PartialEq)]
pub enum SolverRefusal {
    /// Pin-jointed network has mechanisms and load is not orthogonal to them.
    Mechanism(Mechanism),
    /// Supports do not remove all rigid-body or free modes.
    Underconstrained(Underconstrained),
    /// Load is inconsistent with the operator range (semidefinite / rank-deficient K).
    Inconsistent(Inconsistent),
    /// `κ_hi · ε_work` cannot certify `tol` at the working precision.
    PrecisionInsufficient(PrecisionInsufficient),
}

fn certify_finite_slice(v: &[f64]) -> Result<(), SolverRefusalCertifyRefuse> {
    for &x in v {
        if !x.is_finite() {
            return Err(SolverRefusalCertifyRefuse::NonFinite);
        }
    }
    Ok(())
}

fn certify_positive_finite(x: f64) -> Result<f64, SolverRefusalCertifyRefuse> {
    if !x.is_finite() {
        return Err(SolverRefusalCertifyRefuse::NonFinite);
    }
    if x <= 0.0 {
        return Err(SolverRefusalCertifyRefuse::NonPositive);
    }
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Precondition: finite mode and load vectors with a non-zero load component witness.
    #[test]
    fn mechanism_certified_from_finite_vectors() {
        let m = Mechanism::certified(vec![1.0, 0.0], vec![0.1, 0.2]).expect("finite witness");
        let refusal = SolverRefusal::Mechanism(m);
        assert!(matches!(refusal, SolverRefusal::Mechanism(_)));
    }

    /// Precondition: finite mode basis for missing supports.
    #[test]
    fn underconstrained_certified_from_finite_modes() {
        let u = Underconstrained::certified(vec![0.0, 1.0, 0.0]).expect("finite modes");
        let refusal = SolverRefusal::Underconstrained(u);
        assert!(matches!(refusal, SolverRefusal::Underconstrained(_)));
    }

    /// Precondition: non-negative LS residual and finite nullspace witness.
    #[test]
    fn inconsistent_certified_from_residual_and_witness() {
        let i = Inconsistent::certified(0.778, vec![1.0, -1.0]).expect("consistent witness");
        let refusal = SolverRefusal::Inconsistent(i);
        assert!(matches!(refusal, SolverRefusal::Inconsistent(_)));
        if let SolverRefusal::Inconsistent(Inconsistent {
            least_squares_residual,
            ..
        }) = refusal
        {
            assert!((least_squares_residual - 0.778).abs() < 1e-12);
        }
    }

    /// Precondition: κ_hi · ε_work ≥ tol (product 1e-6 ≥ 1e-9).
    #[test]
    fn precision_insufficient_certified_when_product_meets_tol() {
        let p = PrecisionInsufficient::certified(1e3, 1e-9, 1e-9).expect("insufficient precision");
        let refusal = SolverRefusal::PrecisionInsufficient(p);
        assert!(matches!(refusal, SolverRefusal::PrecisionInsufficient(_)));
    }

    /// Precondition: κ_hi · ε_work < tol — precision is adequate; witness must be refused.
    #[test]
    fn precision_insufficient_refuses_when_kappa_eps_below_tol() {
        let err = PrecisionInsufficient::certified(1.0, 1e-12, 1e-9);
        assert_eq!(err, Err(SolverRefusalCertifyRefuse::PrecisionAdequate));
    }

    /// Precondition: mode vector contains NaN — certification must not succeed.
    #[test]
    fn non_finite_mode_vector_is_refused() {
        let err = Underconstrained::certified(vec![1.0, f64::NAN]);
        assert_eq!(err, Err(SolverRefusalCertifyRefuse::NonFinite));
        let err_mech = Mechanism::certified(vec![1.0], vec![f64::INFINITY]);
        assert_eq!(err_mech, Err(SolverRefusalCertifyRefuse::NonFinite));
    }
}

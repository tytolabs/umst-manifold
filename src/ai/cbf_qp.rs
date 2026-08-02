// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! CBF-QP steering filter (F2): project agent proposals onto the admissible half-space.
//!
//! Warm/agent layer only — no serde, clock, or tape through PCG.
//!
//! **Honest boundary:** 1-D closed-form CBF-QP over [`AdmissibilityMargin`] is landed;
//! coupling to hybrid MD-oracle trajectories, multi-D QP, and production PPO hot-bind is **open**
//! (see P-24 capability audit — thermodynamic margin filter, not MD-oracle coupled).

use crate::runtime::gate::AdmissibilityMargin;

/// W29 wave step — CBF-QP deepen landed at 1-D steering slice.
pub const W29_CBF_QP_DEEPEN_STEP: &str = "W29-006-CBF_QP";

/// Morphism F2 — differentiable 1-D CBF-QP steering projection.
pub const MORPHISM_F2_CBF_QP: &str = "F2-CBF-QP";

/// 1-D CBF-QP closed form landed (scalar surrogate ∂h/∂u).
pub const CBF_QP_STEER_LANDED: bool = true;

/// Honest physics posture — margin filter only; not full coupled active-matter GREEN.
pub const CBF_QP_PHYSICS_GREEN: bool = false;

/// Honest refusal — not production-wired to `umst-cartridge-active-steer-sim` or MD oracle.
pub const CBF_QP_PRODUCTION_WIRED: bool = false;

/// Hybrid MD-oracle trajectory envelope — **not** coupled at this slice.
pub const CBF_QP_MD_ORACLE_COUPLED: bool = false;

/// Operator master retick — **not** authorized from this warm-layer slice.
pub const CBF_QP_MASTER_RETICK: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const HONEST_FENCE: &str =
    "steer_landed=true md_oracle_coupled=false production_wired=false physics_green=false";

/// Numerical floor on `∂h/∂u` below which the half-space constraint is treated as inactive.
pub const H_DOT_COEFF_EPS: f32 = 1e-12;

/// Typed probe for W29 CBF-QP posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CbfQpPostureProbe {
    pub deepen_step: &'static str,
    pub morphism_id: &'static str,
    pub steer_landed: bool,
    pub physics_green: bool,
    pub production_wired: bool,
    pub md_oracle_coupled: bool,
    pub master_retick: bool,
    pub honest_fence: &'static str,
}

/// Why a 1-D projection returned its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CbfQpProjectReason {
    /// Agent proposal already satisfies `ḣ ≥ −α(h)`.
    PassThrough,
    /// Closed-form projection onto the admissible half-space boundary.
    Projected,
    /// `∂h/∂u` below [`H_DOT_COEFF_EPS`] — constraint inactive.
    DegenerateCoeff,
}

/// Witness for a single 1-D CBF-QP solve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CbfQpProjectOutcome {
    pub u: f32,
    pub u_agent: f32,
    pub reason: CbfQpProjectReason,
    /// Post-projection constraint slack `ḣ + α(h)` (≥ 0 when satisfied).
    pub margin_after: f32,
}

/// Batch length mismatch on the CBF-QP steering path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CbfQpBatchError {
    LengthMismatch {
        u_agent_len: usize,
        margins_len: usize,
        h_dot_len: usize,
    },
}

/// Honest production posture — **false** until steer-sim + MD-oracle wire measured.
#[must_use]
pub const fn cbf_qp_production_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at this slice.
const _: () = assert!(!cbf_qp_production_wired());

/// Build introspection probe for CBF-QP posture done-when checks.
#[must_use]
pub const fn cbf_qp_posture_probe() -> CbfQpPostureProbe {
    CbfQpPostureProbe {
        deepen_step: W29_CBF_QP_DEEPEN_STEP,
        morphism_id: MORPHISM_F2_CBF_QP,
        steer_landed: CBF_QP_STEER_LANDED,
        physics_green: CBF_QP_PHYSICS_GREEN,
        production_wired: CBF_QP_PRODUCTION_WIRED,
        md_oracle_coupled: CBF_QP_MD_ORACLE_COUPLED,
        master_retick: CBF_QP_MASTER_RETICK,
        honest_fence: HONEST_FENCE,
    }
}

/// CBF-QP scaffold landed with production / MD-oracle paths honestly open.
#[must_use]
pub fn cbf_qp_posture_honest(probe: &CbfQpPostureProbe) -> bool {
    probe.deepen_step == W29_CBF_QP_DEEPEN_STEP
        && probe.morphism_id == MORPHISM_F2_CBF_QP
        && probe.steer_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.md_oracle_coupled
        && !probe.master_retick
        && probe.honest_fence.contains("steer_landed=true")
        && probe.honest_fence.contains("md_oracle_coupled=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Validate CBF-QP posture honesty — fail closed on fake production / GREEN claims.
pub fn validate_cbf_qp_posture_honesty() -> Result<(), &'static str> {
    let probe = cbf_qp_posture_probe();
    if probe.production_wired || cbf_qp_production_wired() {
        return Err("cbf_qp_production_wired must stay false until steer-sim MD-oracle wire");
    }
    if probe.physics_green {
        return Err("CBF_QP_PHYSICS_GREEN must stay false at warm-layer slice");
    }
    if probe.md_oracle_coupled {
        return Err("CBF_QP_MD_ORACLE_COUPLED must stay false until P-24 testbed");
    }
    if probe.master_retick {
        return Err("CBF_QP_MASTER_RETICK must stay false — operator-owned");
    }
    if !cbf_qp_posture_honest(&probe) {
        return Err("cbf_qp_posture_honest failed");
    }
    Ok(())
}

/// CBF constraint RHS: `−α(h)` for barrier `h = margin.value()`.
#[must_use]
pub fn cbf_qp_constraint_rhs(margin: AdmissibilityMargin, alpha: f32) -> f32 {
    -alpha * margin.value()
}

/// Post-projection slack `ḣ + α(h)` for scalar surrogate `ḣ = (∂h/∂u)·u`.
#[must_use]
pub fn cbf_qp_constraint_margin(u: f32, margin: AdmissibilityMargin, h_dot_coeff: f32, alpha: f32) -> f32 {
    h_dot_coeff * u + alpha * margin.value()
}

/// Whether `u` satisfies `ḣ ≥ −α(h)` for the scalar surrogate.
#[must_use]
pub fn cbf_qp_constraint_satisfied(
    u: f32,
    margin: AdmissibilityMargin,
    h_dot_coeff: f32,
    alpha: f32,
) -> bool {
    cbf_qp_constraint_margin(u, margin, h_dot_coeff, alpha) >= -H_DOT_COEFF_EPS
}

/// Differentiable 1-D CBF-QP with explicit outcome witness.
///
/// `min (u − u_agent)²` s.t. `(∂h/∂u)·u ≥ −α(h)`.
#[must_use]
pub fn cbf_qp_project_1d_outcome(
    u_agent: f32,
    margin: AdmissibilityMargin,
    h_dot_coeff: f32,
    alpha: f32,
) -> CbfQpProjectOutcome {
    if h_dot_coeff.abs() < H_DOT_COEFF_EPS {
        let margin_after = cbf_qp_constraint_margin(u_agent, margin, h_dot_coeff, alpha);
        return CbfQpProjectOutcome {
            u: u_agent,
            u_agent,
            reason: CbfQpProjectReason::DegenerateCoeff,
            margin_after,
        };
    }

    let rhs = cbf_qp_constraint_rhs(margin, alpha);
    let lhs = h_dot_coeff * u_agent;
    if lhs >= rhs {
        let margin_after = cbf_qp_constraint_margin(u_agent, margin, h_dot_coeff, alpha);
        return CbfQpProjectOutcome {
            u: u_agent,
            u_agent,
            reason: CbfQpProjectReason::PassThrough,
            margin_after,
        };
    }

    let u = rhs / h_dot_coeff;
    let margin_after = cbf_qp_constraint_margin(u, margin, h_dot_coeff, alpha);
    CbfQpProjectOutcome {
        u,
        u_agent,
        reason: CbfQpProjectReason::Projected,
        margin_after,
    }
}

/// Differentiable 1-D CBF-QP: `min (u − u_agent)²` s.t. `ḣ ≥ −α(h)`.
///
/// `h_dot_coeff` is ∂h/∂u along the proposed direction (scalar surrogate).
#[must_use]
pub fn cbf_qp_project_1d(
    u_agent: f32,
    margin: AdmissibilityMargin,
    h_dot_coeff: f32,
    alpha: f32,
) -> f32 {
    cbf_qp_project_1d_outcome(u_agent, margin, h_dot_coeff, alpha).u
}

/// Batch wrapper for policy tensors (same semantics as [`cbf_qp_project_1d`]).
#[must_use]
pub fn cbf_qp_project_batch(
    u_agent: &[f32],
    margins: &[AdmissibilityMargin],
    h_dot: &[f32],
    alpha: f32,
) -> Vec<f32> {
    u_agent
        .iter()
        .zip(margins.iter().zip(h_dot.iter()))
        .map(|(&u, (&m, &hd))| cbf_qp_project_1d(u, m, hd, alpha))
        .collect()
}

/// Fallible batch wrapper — rejects length mismatch instead of silent zip truncation.
pub fn cbf_qp_project_batch_checked(
    u_agent: &[f32],
    margins: &[AdmissibilityMargin],
    h_dot: &[f32],
    alpha: f32,
) -> Result<Vec<f32>, CbfQpBatchError> {
    let n = u_agent.len();
    if margins.len() != n || h_dot.len() != n {
        return Err(CbfQpBatchError::LengthMismatch {
            u_agent_len: n,
            margins_len: margins.len(),
            h_dot_len: h_dot.len(),
        });
    }
    Ok(cbf_qp_project_batch(u_agent, margins, h_dot, alpha))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbf_qp_posture_metadata_locked() {
        assert_eq!(W29_CBF_QP_DEEPEN_STEP, "W29-006-CBF_QP");
        assert_eq!(MORPHISM_F2_CBF_QP, "F2-CBF-QP");
        assert!(CBF_QP_STEER_LANDED);
        assert!(!CBF_QP_PHYSICS_GREEN);
        assert!(!CBF_QP_PRODUCTION_WIRED);
        assert!(!CBF_QP_MD_ORACLE_COUPLED);
        assert!(!CBF_QP_MASTER_RETICK);
        assert!(!cbf_qp_production_wired());
        assert_eq!(
            HONEST_FENCE,
            "steer_landed=true md_oracle_coupled=false production_wired=false physics_green=false"
        );
    }

    #[test]
    fn cbf_qp_posture_probe_honest_not_green() {
        let probe = cbf_qp_posture_probe();
        assert!(cbf_qp_posture_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.md_oracle_coupled);
        assert!(!probe.master_retick);
        assert!(validate_cbf_qp_posture_honesty().is_ok());
    }

    #[test]
    fn cbf_qp_projects_inside_margin() {
        let margin = AdmissibilityMargin(-2.0);
        let u_agent = 1.0_f32;
        let h_dot = 1.0_f32;
        let alpha = 1.0_f32;
        let outcome = cbf_qp_project_1d_outcome(u_agent, margin, h_dot, alpha);
        assert_eq!(outcome.reason, CbfQpProjectReason::Projected);
        assert!(
            outcome.u > u_agent,
            "negative margin should push u outward to satisfy ḣ ≥ −αh"
        );
        assert!(cbf_qp_constraint_satisfied(outcome.u, margin, h_dot, alpha));
        // On-boundary projection ⇒ post-projection slack ḣ+αh ≈ 0 (not the RHS −αh).
        assert!(
            outcome.margin_after.abs() < 1e-5,
            "projected boundary slack should be ≈0, got {}",
            outcome.margin_after
        );
        assert!((outcome.u - (-alpha * margin.value()) / h_dot).abs() < 1e-5);
        assert!((cbf_qp_project_1d(u_agent, margin, h_dot, alpha) - outcome.u).abs() < 1e-6);
    }

    #[test]
    fn cbf_qp_passes_through_when_already_safe() {
        let margin = AdmissibilityMargin(1.0);
        let u_agent = 0.25_f32;
        let outcome = cbf_qp_project_1d_outcome(u_agent, margin, 1.0, 1.0);
        assert_eq!(outcome.reason, CbfQpProjectReason::PassThrough);
        assert!((outcome.u - u_agent).abs() < 1e-6);
        assert!(cbf_qp_constraint_satisfied(outcome.u, margin, 1.0, 1.0));
    }

    #[test]
    fn cbf_qp_degenerate_coeff_passes_through() {
        let margin = AdmissibilityMargin(-5.0);
        let u_agent = 0.5_f32;
        let outcome = cbf_qp_project_1d_outcome(u_agent, margin, 0.0, 1.0);
        assert_eq!(outcome.reason, CbfQpProjectReason::DegenerateCoeff);
        assert!((outcome.u - u_agent).abs() < 1e-6);
    }

    #[test]
    fn cbf_qp_negative_h_dot_projects_opposite_direction() {
        let margin = AdmissibilityMargin(-1.0);
        let u_agent = 2.0_f32;
        let h_dot = -1.0_f32;
        let alpha = 1.0_f32;
        let outcome = cbf_qp_project_1d_outcome(u_agent, margin, h_dot, alpha);
        assert_eq!(outcome.reason, CbfQpProjectReason::Projected);
        assert!(outcome.u < u_agent);
        assert!(cbf_qp_constraint_satisfied(outcome.u, margin, h_dot, alpha));
    }

    #[test]
    fn cbf_qp_batch_matches_scalar_projection() {
        let u_agent = [1.0_f32, 0.25, 3.0];
        let margins = [
            AdmissibilityMargin(-2.0),
            AdmissibilityMargin(1.0),
            AdmissibilityMargin(-0.5),
        ];
        let h_dot = [1.0_f32, 1.0, -0.5];
        let alpha = 1.0_f32;
        let batch = cbf_qp_project_batch(&u_agent, &margins, &h_dot, alpha);
        assert_eq!(batch.len(), 3);
        for ((&u, m), (&hd, &proj)) in u_agent
            .iter()
            .zip(margins.iter().copied())
            .zip(h_dot.iter().zip(batch.iter()))
        {
            let expected = cbf_qp_project_1d(u, m, hd, alpha);
            assert!((proj - expected).abs() < 1e-6);
            assert!(cbf_qp_constraint_satisfied(proj, m, hd, alpha));
        }
    }

    #[test]
    fn cbf_qp_batch_checked_rejects_length_mismatch() {
        let err = cbf_qp_project_batch_checked(&[1.0], &[AdmissibilityMargin(1.0)], &[], 1.0)
            .unwrap_err();
        assert_eq!(
            err,
            CbfQpBatchError::LengthMismatch {
                u_agent_len: 1,
                margins_len: 1,
                h_dot_len: 0,
            }
        );
    }

    #[test]
    fn cbf_qp_constraint_rhs_and_margin_witness() {
        let margin = AdmissibilityMargin(-3.0);
        let alpha = 2.0_f32;
        let rhs = cbf_qp_constraint_rhs(margin, alpha);
        assert!((rhs - 6.0).abs() < 1e-6);
        let u = rhs;
        let slack = cbf_qp_constraint_margin(u, margin, 1.0, alpha);
        assert!(slack >= -H_DOT_COEFF_EPS);
    }
}

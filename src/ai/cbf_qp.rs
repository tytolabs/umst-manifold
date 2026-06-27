// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! CBF-QP steering filter (F2): project agent proposals onto the admissible half-space.
//!
//! Warm/agent layer only — no serde, clock, or tape through PCG.

use crate::runtime::gate::AdmissibilityMargin;

/// Differentiable 1-D CBF-QP: `min (u − u_agent)²` s.t. `ḣ ≥ −α(h)`.
///
/// `h_dot_coeff` is ∂h/∂u along the proposed direction (scalar surrogate).
#[must_use]
pub fn cbf_qp_project_1d(u_agent: f32, margin: AdmissibilityMargin, h_dot_coeff: f32, alpha: f32) -> f32 {
    let h = margin.value();
    let rhs = -alpha * h;
    let lhs = h_dot_coeff * u_agent;
    if h_dot_coeff.abs() < 1e-12 || lhs >= rhs {
        return u_agent;
    }
    rhs / h_dot_coeff
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbf_qp_projects_inside_margin() {
        let margin = AdmissibilityMargin(-2.0);
        let u_agent = 1.0_f32;
        let h_dot = 1.0_f32;
        let alpha = 1.0_f32;
        let u = cbf_qp_project_1d(u_agent, margin, h_dot, alpha);
        assert!(u > u_agent, "negative margin should push u outward to satisfy ḣ ≥ −αh");
        assert!((h_dot * u - (-alpha * margin.value())).abs() < 1e-5);
    }

    #[test]
    fn cbf_qp_passes_through_when_already_safe() {
        let margin = AdmissibilityMargin(1.0);
        let u_agent = 0.25_f32;
        let u = cbf_qp_project_1d(u_agent, margin, 1.0, 1.0);
        assert!((u - u_agent).abs() < 1e-6);
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Numeric tolerance SSOT for solver floors and regression approx bounds.
//!
//! Literal `tol` / `eps` / `epsilon` assignments belong here (or [`super::registry`]
//! rows for gate constants). Call sites derive relative floors from [`ProblemScale`].

/// Registry row `transition_tolerance` — admissibility ε (formal Gate.transitionTolerance).
#[must_use]
pub const fn transition_tolerance_f64() -> f64 {
    1e-6
}

/// Registry row `admissibility_margin_eps` — hard token floor (Gate.gateCheckSound).
#[must_use]
pub const fn admissibility_margin_eps_f64() -> f64 {
    1e-4
}

/// Characteristic positive scale (mesh length, modulus, residual norm, etc.).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProblemScale {
    /// Representative magnitude; non-finite or ≤0 falls back to 1.0 when deriving floors.
    pub characteristic: f64,
}

impl ProblemScale {
    #[must_use]
    pub const fn new(characteristic: f64) -> Self {
        Self { characteristic }
    }

    #[must_use]
    pub const fn unit() -> Self {
        Self { characteristic: 1.0 }
    }

    #[must_use]
    fn effective(self) -> f64 {
        if self.characteristic.is_finite() && self.characteristic > 0.0 {
            self.characteristic
        } else {
            1.0
        }
    }
}

/// Relative residual tier for projected CG / PCG loops (f32 mechanics vs tighter checks).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinearSolveRelativeTier {
    /// Default bar-network PCG headroom (~1e-6 relative @ unit scale).
    BarNetworkF32Default,
    /// Tighter equilibrium / hex regression (~1e-8 relative @ unit scale).
    MechanicsTightF32,
    /// Near-analytic adjoint / operator identity checks (~1e-10 relative @ unit scale).
    AdjointReferenceF64,
}

/// Relative residual tolerance from scale × tier (never below `f32::EPSILON`).
#[must_use]
pub fn linear_solve_relative_tol_f32(
    scale: ProblemScale,
    tier: LinearSolveRelativeTier,
) -> f32 {
    let base = match tier {
        LinearSolveRelativeTier::BarNetworkF32Default => 1e-6_f64,
        LinearSolveRelativeTier::MechanicsTightF32 => 1e-8_f64,
        LinearSolveRelativeTier::AdjointReferenceF64 => 1e-10_f64,
    };
    let rel = base * scale.effective();
    (rel.max(f64::from(f32::EPSILON))).min(1.0) as f32
}

/// Unit-scale bar-network CG/PCG relative tolerance (f32).
#[must_use]
pub fn bar_network_cg_tol_f32() -> f32 {
    linear_solve_relative_tol_f32(
        ProblemScale::unit(),
        LinearSolveRelativeTier::BarNetworkF32Default,
    )
}

/// Unit-scale tight mechanics CG/PCG relative tolerance (f32).
#[must_use]
pub fn mechanics_tight_cg_tol_f32() -> f32 {
    linear_solve_relative_tol_f32(
        ProblemScale::unit(),
        LinearSolveRelativeTier::MechanicsTightF32,
    )
}

/// Mid tier between default bar network and tight mechanics (~1e-7).
#[must_use]
pub fn mechanics_mid_cg_tol_f32() -> f32 {
    linear_solve_relative_tol_f32(ProblemScale::new(0.1), LinearSolveRelativeTier::BarNetworkF32Default)
}

/// Unit-scale adjoint / analytic reference relative tolerance (f64).
#[must_use]
pub fn adjoint_reference_tol_f64() -> f64 {
    linear_solve_relative_tol_f64(
        ProblemScale::unit(),
        LinearSolveRelativeTier::AdjointReferenceF64,
    )
}

/// Same tiers for f64 reference solves and adjoint witnesses.
#[must_use]
pub fn linear_solve_relative_tol_f64(
    scale: ProblemScale,
    tier: LinearSolveRelativeTier,
) -> f64 {
    let base = match tier {
        LinearSolveRelativeTier::BarNetworkF32Default => 1e-6_f64,
        LinearSolveRelativeTier::MechanicsTightF32 => 1e-8_f64,
        LinearSolveRelativeTier::AdjointReferenceF64 => 1e-10_f64,
    };
    let rel = base * scale.effective();
    rel.max(f64::EPSILON).min(1.0)
}

/// Symmetric finite-difference step from problem scale (central difference on f32 fields).
#[must_use]
pub fn finite_difference_step_f32(scale: ProblemScale) -> f32 {
    let h = scale.effective().sqrt() * 5e-4_f64;
    (h.max(1e-6_f64).min(1e-2_f64)) as f32
}

/// Denominator guard for relative error ratios (avoids division by exact zero).
#[must_use]
pub fn relative_error_denominator_f32(reference: f32) -> f32 {
    reference.abs().max(f32::EPSILON)
}

/// Denominator guard for f64 relative error ratios.
#[must_use]
pub fn relative_error_denominator_f64(reference: f64) -> f64 {
    reference.abs().max(f64::EPSILON)
}

/// `approx` / unit-test absolute epsilon for f64 Landauer and credit parity.
pub const APPROX_EPSILON_F64: f64 = 1.0e-30;

/// Default `max_relative` for credit / Landauer debit parity tests.
pub const APPROX_MAX_RELATIVE_DEFAULT: f64 = 1.0e-9;

/// Loose f32 component checks (tensor spot tests).
pub const APPROX_EPSILON_F32_LOOSE: f32 = 1.0e-6;

/// Mid f32 component checks (DEC / bar-network spot tests).
pub const APPROX_EPSILON_F32_MID: f32 = 1.0e-5;

/// Loose f64 checks where f32 noise is absent.
pub const APPROX_EPSILON_F64_LOOSE: f64 = 1.0e-18;

/// Gate / optim comparison floor (dimensionless).
#[must_use]
pub const fn gate_comparison_tol_f64() -> f64 {
    transition_tolerance_f64()
}

/// Relative tolerance for uniform grid spacing inference (photonics chain probes).
#[must_use]
pub const fn uniform_grid_spacing_rtol_f32() -> f32 {
    1e-2
}

/// Loose relative tolerance for rank-1 / field algebra regression checks.
#[must_use]
pub const fn field_algebra_rtol_f64() -> f64 {
    1e-4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_scale_default_tier_matches_bar_network_headroom() {
        let tol = linear_solve_relative_tol_f32(
            ProblemScale::unit(),
            LinearSolveRelativeTier::BarNetworkF32Default,
        );
        assert!((tol - 1e-6_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn scale_scales_relative_floor_linearly() {
        let s = ProblemScale::new(10.0);
        let tol = linear_solve_relative_tol_f32(s, LinearSolveRelativeTier::BarNetworkF32Default);
        assert!((tol - 1e-5_f32).abs() < 1e-7_f32);
    }
}

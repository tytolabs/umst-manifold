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

/// Registry row `gate_mass_tolerance_kg_m3` — bulk density jump band (Concrete.Gate.δMass_val).
#[must_use]
pub const fn gate_mass_tolerance_kg_m3_f64() -> f64 {
    100.0
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

/// Default outer Newton tolerance for coupled THMC stepping (dimensionless).
#[must_use]
pub const fn thmc_outer_newton_tol_f32() -> f32 {
    1e-3
}

/// Orchestrator smoke-test THMC tolerance (tighter than production default).
#[must_use]
pub const fn thmc_orchestrator_smoke_tol_f32() -> f32 {
    1e-4
}

/// Residual floor for implicit THMC Newton / CG inner solves.
#[must_use]
pub const fn thmc_newton_residual_tol_f32() -> f32 {
    1e-6
}

/// Finite-difference step for THMC Newton Jacobian probes (thermal block).
#[must_use]
pub const fn thmc_newton_fd_eps_f32() -> f32 {
    1e-6
}

/// Default outer Newton iteration budget for implicit THMC thermal block (registry SSOT).
#[must_use]
pub const fn thmc_newton_default_iteration_budget() -> usize {
    25 + 25
}

/// Default damped Newton iteration budget for PNP chain implicit solves (registry SSOT).
#[must_use]
pub const fn newton_pnp_default_iteration_budget() -> usize {
    1usize << 5
}

/// Default GMRES restart budget for acoustic implicit Newmark solves (registry SSOT).
#[must_use]
pub const fn acoustic_gmres_default_iteration_budget() -> usize {
    1usize << 8
}

/// FD step for damped Newton on stacked THMC implicit blocks.
#[must_use]
pub const fn thmc_damped_newton_fd_eps_f32() -> f32 {
    1e-5
}

/// Default GMRES relative tolerance for acoustic implicit Newmark solves.
#[must_use]
pub const fn acoustic_gmres_rel_tol_f32() -> f32 {
    1e-4
}

/// Tight GMRES relative tolerance (short-chain regression).
#[must_use]
pub const fn acoustic_gmres_rel_tol_tight_f32() -> f32 {
    1e-7
}

/// Default Poisson CG relative tolerance for Bingham flow pressure solve.
#[must_use]
pub const fn rheology_poisson_cg_rel_tol_f32() -> f32 {
    2e-5
}

/// Absolute term in mixed DEC matvec operator parity checks.
#[must_use]
pub const fn dec_matvec_abs_tol_f32() -> f32 {
    1e-4
}

/// Relative coefficient in mixed DEC matvec operator parity checks.
#[must_use]
pub const fn dec_matvec_rel_coeff_f32() -> f32 {
    1e-3
}

/// Virial closed-form regression (f32).
#[must_use]
pub const fn virial_closed_form_abs_tol_f32() -> f32 {
    2e-5
}

/// Analytic bulk-modulus tensor parity vs Johnson reference (f64).
#[must_use]
pub const fn statmech_bulk_modulus_rel_tol_f64() -> f64 {
    5e-4
}

/// FD bulk-modulus parity for LJ virial path (f32).
#[must_use]
pub const fn statmech_fd_bulk_modulus_abs_tol_f32() -> f32 {
    2e-3
}

/// Regularized Bingham FD regression strain scale (f64).
#[must_use]
pub const fn rheology_analytic_fd_strain_eps_f64() -> f64 {
    1e-4
}

/// Spectral tensile ψ surrogate probe strain (f32).
#[must_use]
pub const fn fracture_psi_probe_strain_f32() -> f32 {
    1e-3
}

/// Edge-length divisor floor for axial strain (`elong / edge_len`) in bar networks.
pub const EDGE_LENGTH_DIVISOR_FLOOR_F32: f32 = 1e-30;

/// Sentinel non-positive rel-tol for refusal / precondition tests (must stay ≤ 0).
pub const REFUSAL_NONPOSITIVE_REL_TOL_F32: f32 = 0.0_f32;

/// Default equilibrium inner iterations per mechanics pass (single pass today).
pub const DEFAULT_EQUILIBRIUM_SUB_ITERS: u32 = 1;

/// Hard cap on mechanics inner iterations per outer chemistry step (orchestration clocks).
pub const DEFAULT_MECH_SUB_ITERS_PER_CHEM_CAP: u32 = 10_000;

/// Bar-network / adjoint regression PCG iteration budget (fixture SSOT — not a solver-src literal cap).
#[must_use]
pub const fn bar_network_regression_cg_iteration_budget() -> usize {
    500
}

/// Single-Newton electrochemistry verification fixture (linearized Jacobian smoke).
#[must_use]
pub const fn newton_pnp_single_iter_fixture_budget() -> usize {
    1
}

/// Extended Newton budget for stiff PNP verification grids (above default fifty).
#[must_use]
pub const fn newton_pnp_verification_extended_iteration_budget() -> usize {
    60
}

/// AT2 staggered outer-loop regression budget (phase-field damage outer witness).
#[must_use]
pub const fn fracture_at2_outer_regression_iteration_budget() -> usize {
    40
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

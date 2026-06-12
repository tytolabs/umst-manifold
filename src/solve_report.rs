// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Unified linear / equilibrium solve telemetry (integration-contracts D1).
//!
//! **Objects:** [`SolveReport`] (immutable witness), [`PrecisionLane`] (numeric path tag).
//! **Morphisms:** [`ReportedSolve::into_solve_report`], [`from_bar_network_pcg`].
//! **Law:** `converged()` iff `rel_tol > 0` and `rel_residual <= rel_tol` (same scale as
//! [`crate::physics::mechanics::VectorMechanicsSolver::packed_bar_network_equilibrium`] PCG exit).

use serde::{Deserialize, Serialize};

/// Numeric lane for an inner equilibrium / Krylov solve (audit finding **#3** THMC honesty ladder).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrecisionLane {
    /// Default Burn f32 projected CG on the bar network.
    F32BurnBarPcg,
    /// f64 matvec PCG (`mechanics-adjoint` forward pass).
    F64AdjointBarPcg,
    /// Q1-hex matrix-free PCG (`q1_hex_elasticity`).
    HexQ1Pcg,
    /// Small-graph dense Newton / direct factorisation.
    DenseDirect,
    /// Host Krylov (`krylov_host`, THMC JFNK smoke).
    HostKrylov,
}

/// Immutable solve witness returned at solver boundaries (serde for ledger / CI fixtures).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolveReport {
    pub iterations: usize,
    /// \(\|Pr\|_2 / \|Pf\|_2\) or lane-equivalent relative residual at exit.
    pub rel_residual: f32,
    pub stiffness_scale: f32,
    pub e_ref: f32,
    pub dx_char: f32,
    /// Relative tolerance used for the converged predicate (max of CG / PCG aliases).
    pub rel_tol: f32,
    pub lane: PrecisionLane,
}

impl SolveReport {
    /// **Law:** converged when a positive relative tolerance was requested and the exit residual meets it.
    #[must_use]
    pub fn converged(&self) -> bool {
        self.rel_tol > 0.0 && self.rel_residual.is_finite() && self.rel_residual <= self.rel_tol
    }
}

/// Morphism from lane-specific telemetry into the unified contract.
pub trait ReportedSolve {
    fn into_solve_report(self, rel_tol: f32, lane: PrecisionLane) -> SolveReport;
}

/// Lift bar-network PCG telemetry (model: [`crate::physics::mechanics::VectorMechanicsSolver::solve_equilibrium_with_pcg_report`]).
#[must_use]
pub fn from_bar_network_pcg(
    pcg: crate::physics::mechanics::BarNetworkPcgReport,
    rel_tol: f32,
    lane: PrecisionLane,
) -> SolveReport {
    SolveReport {
        iterations: pcg.iterations,
        rel_residual: pcg.rel_residual,
        stiffness_scale: pcg.stiffness_scale,
        e_ref: pcg.e_ref,
        dx_char: pcg.dx_char,
        rel_tol,
        lane,
    }
}

impl ReportedSolve for crate::physics::mechanics::BarNetworkPcgReport {
    fn into_solve_report(self, rel_tol: f32, lane: PrecisionLane) -> SolveReport {
        from_bar_network_pcg(self, rel_tol, lane)
    }
}

/// Catalog of public equilibrium entry points awaiting [`SolveReport`] adoption (audit **#3**, **#5**, **#12**, **#19**).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolverEntryPoint {
    pub module: &'static str,
    pub symbol: &'static str,
    pub audit_finding: u8,
    pub notes: &'static str,
}

/// Static inventory — Wave 0 ledger honesty; Wave 1+ wire `SolveReport` at each site.
pub const SOLVER_ENTRY_POINTS: &[SolverEntryPoint] = &[
    SolverEntryPoint {
        module: "physics::mechanics",
        symbol: "VectorMechanicsSolver::solve_equilibrium",
        audit_finding: 1,
        notes: "bar network; THMC/fracture/adjoint load-bearing (P0 #1)",
    },
    SolverEntryPoint {
        module: "physics::mechanics",
        symbol: "VectorMechanicsSolver::solve_equilibrium_with_pcg_report",
        audit_finding: 3,
        notes: "H4 telemetry; model for SolveReport adoption",
    },
    SolverEntryPoint {
        module: "physics::solvers::thmc",
        symbol: "ThmcSolver::step",
        audit_finding: 3,
        notes: "operator-split; stacked-R exit open (Wave 1)",
    },
    SolverEntryPoint {
        module: "physics::solvers::thmc_residual",
        symbol: "ThmcImplicitEulerThermalHumidityHydrationResidual::damped_newton_iterations_with_quasi_static_r_u",
        audit_finding: 4,
        notes: "dense cap 64 DOF; ‖R‖ honesty (Wave 1)",
    },
    SolverEntryPoint {
        module: "physics::solvers::fracture_field",
        symbol: "PhaseFieldFractureSolver::update_damage_staggered",
        audit_finding: 12,
        notes: "staggered u↔d; within-step coupling open",
    },
    SolverEntryPoint {
        module: "physics::solvers::electrochemistry",
        symbol: "ElectroChemicalSolver::solve_pnp_step",
        audit_finding: 5,
        notes: "explicit Picard default; scale / graph generality",
    },
    SolverEntryPoint {
        module: "physics::extruded_plate",
        symbol: "ExtrudedPlateMechanics::solve_equilibrium",
        audit_finding: 2,
        notes: "Q1 hex; 9×8×2 roof PCG stall (P0 #2)",
    },
    SolverEntryPoint {
        module: "physics::adjoint",
        symbol: "AdjointCompliance::compliance_and_sensitivity",
        audit_finding: 19,
        notes: "inner f64 PCG; discrete adjoint surrogate",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_report_construction_and_converged() {
        let ok = SolveReport {
            iterations: 42,
            rel_residual: 1e-7,
            stiffness_scale: 1.5e6,
            e_ref: 30e9,
            dx_char: 0.1,
            rel_tol: 1e-6,
            lane: PrecisionLane::F32BurnBarPcg,
        };
        assert!(ok.converged());

        let stall = SolveReport {
            rel_residual: 0.94,
            rel_tol: 1e-4,
            ..ok
        };
        assert!(!stall.converged());

        let no_tol = SolveReport { rel_tol: 0.0, ..ok };
        assert!(!no_tol.converged());
    }

    #[test]
    fn solve_report_serde_roundtrip() {
        let report = SolveReport {
            iterations: 3,
            rel_residual: 2e-8,
            stiffness_scale: 1.0,
            e_ref: 210e9,
            dx_char: 0.05,
            rel_tol: 1e-6,
            lane: PrecisionLane::F64AdjointBarPcg,
        };
        let json = serde_json::to_string(&report).expect("serialize");
        let back: SolveReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(report, back);
    }

    #[test]
    fn from_bar_network_pcg_matches_trait() {
        let pcg = crate::physics::mechanics::BarNetworkPcgReport {
            iterations: 10,
            rel_residual: 1e-8,
            stiffness_scale: 2.0,
            e_ref: 1.0,
            dx_char: 0.25,
        };
        let via_fn = from_bar_network_pcg(pcg, 1e-6, PrecisionLane::F32BurnBarPcg);
        let via_trait = pcg.into_solve_report(1e-6, PrecisionLane::F32BurnBarPcg);
        assert_eq!(via_fn, via_trait);
        assert!(via_fn.converged());
    }
}

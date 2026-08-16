// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Unified linear / equilibrium solve telemetry (integration-contracts D1).
//!
//! **Objects:** [`SolveReport`] (immutable witness), [`PrecisionLane`] (numeric path tag).
//! **Morphisms:** [`ReportedSolve::into_solve_report`], [`from_bar_network_pcg`].
//! **Law:** `converged()` iff `rel_tol > 0` and `rel_residual <= rel_tol` (same scale as
//! [`crate::physics::mechanics::VectorMechanicsSolver::packed_bar_network_equilibrium`] PCG exit).
//!
//! # Honest boundary (W29-130)
//!
//! Contract + bar-PCG lift + entry-point inventory are landed. Full solver-site adoption,
//! fleet physics GREEN, production wire, MASTER retick, and OP-5 remain **open / refused**.
//! Unit contracts: `cargo test -p umst-manifold solve_report`.

/// W29 deepen cell — solve_report honest fence bundle.
pub const W29_SOLVE_REPORT_DEEPEN_CELL: &str = "W29-130-SOLVE_REPORT";

/// Honest posture tag — unified witness + bar lift landed; multi-site adoption deferred.
pub const SOLVE_REPORT_POSTURE_TAG: &str = "honest-solve-report-contract-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const SOLVE_REPORT_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the SolveReport contract alone.
pub const SOLVE_REPORT_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const SOLVE_REPORT_MASTER: bool = false;

/// OP-5 claim — always refused at this module (no invent OP-5).
pub const SOLVE_REPORT_OP5_CLAIMED: bool = false;

/// GREEN invent blocked while physics GREEN stays false.
pub const SOLVE_REPORT_GREEN_CLAIM_BLOCKED: bool = true;

/// Contract types + `from_bar_network_pcg` lift landed.
pub const SOLVE_REPORT_CONTRACT_LANDED: bool = true;

/// Entry-point inventory is Wave-0 honesty (catalog), not a GREEN certificate.
pub const SOLVE_REPORT_INVENTORY_CATALOGUED: bool = true;

/// Multi-site SolveReport adoption (THMC / fracture / electrochem / Q1 / adjoint) — open.
pub const SOLVE_REPORT_ALL_SITES_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const SOLVE_REPORT_HONEST_FENCE: &str = "contract_landed=true inventory_catalogued=true bar_pcg_lift=true all_sites_wired=false production_wired=false master_composition_wired=false physics_green=false op5_claimed=false";

const _: () = assert!(!SOLVE_REPORT_PHYSICS_GREEN);
const _: () = assert!(!SOLVE_REPORT_PRODUCTION_WIRED);
const _: () = assert!(!SOLVE_REPORT_MASTER);
const _: () = assert!(!SOLVE_REPORT_OP5_CLAIMED);
const _: () = assert!(SOLVE_REPORT_GREEN_CLAIM_BLOCKED);
const _: () = assert!(SOLVE_REPORT_CONTRACT_LANDED);
const _: () = assert!(SOLVE_REPORT_INVENTORY_CATALOGUED);
const _: () = assert!(!SOLVE_REPORT_ALL_SITES_WIRED);

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

impl PrecisionLane {
    /// Static lane catalog (order stable for serde / inventory probes).
    pub const ALL: [PrecisionLane; 5] = [
        PrecisionLane::F32BurnBarPcg,
        PrecisionLane::F64AdjointBarPcg,
        PrecisionLane::HexQ1Pcg,
        PrecisionLane::DenseDirect,
        PrecisionLane::HostKrylov,
    ];

    /// Audit label for ledger / CI fixtures (not a GREEN claim).
    #[must_use]
    pub const fn audit_label(self) -> &'static str {
        match self {
            PrecisionLane::F32BurnBarPcg => "f32_burn_bar_pcg",
            PrecisionLane::F64AdjointBarPcg => "f64_adjoint_bar_pcg",
            PrecisionLane::HexQ1Pcg => "hex_q1_pcg",
            PrecisionLane::DenseDirect => "dense_direct",
            PrecisionLane::HostKrylov => "host_krylov",
        }
    }
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
    /// **Law:** converged when residual is admissible (finite, ≥0) and a positive relative
    /// tolerance was requested and the exit residual meets it.
    ///
    /// Negative residuals must not count as converged (`-ε ≤ tol` is true in IEEE but is not a witness).
    #[must_use]
    pub fn converged(&self) -> bool {
        self.residual_is_admissible() && self.rel_tol > 0.0 && self.rel_residual <= self.rel_tol
    }

    /// Residual is a finite non-negative scalar (NaN / ±∞ / negative refuse as non-witness).
    #[must_use]
    pub fn residual_is_admissible(&self) -> bool {
        self.rel_residual.is_finite() && self.rel_residual >= 0.0
    }

    /// Stall ratio `rel_residual / rel_tol` when both are positive and finite; `None` otherwise.
    ///
    /// Values `> 1.0` mean the exit residual missed the tolerance (honest stall / divergence).
    #[must_use]
    pub fn stall_ratio(&self) -> Option<f32> {
        if self.rel_tol > 0.0 && self.rel_residual.is_finite() && self.rel_residual >= 0.0 {
            Some(self.rel_residual / self.rel_tol)
        } else {
            None
        }
    }

    /// Fail-closed when the witness does not meet the converged predicate.
    pub fn ensure_converged(&self) -> Result<(), &'static str> {
        if self.converged() {
            Ok(())
        } else if !self.residual_is_admissible() {
            Err("SolveReport residual not admissible (NaN/Inf/negative)")
        } else if !(self.rel_tol > 0.0) {
            Err("SolveReport rel_tol must be positive for converged predicate")
        } else {
            Err("SolveReport did not meet rel_tol (stall / divergence)")
        }
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
    /// `true` when this site currently emits [`SolveReport`] at its boundary.
    pub report_wired: bool,
}

/// Static inventory — Wave 0 ledger honesty; Wave 1+ wire `SolveReport` at each site.
///
/// Only `solve_equilibrium_with_pcg_report` is marked `report_wired` (bar PCG lift via
/// [`from_bar_network_pcg`] / mechanics solve port). Remaining sites stay catalogued.
pub const SOLVER_ENTRY_POINTS: &[SolverEntryPoint] = &[
    SolverEntryPoint {
        module: "physics::mechanics",
        symbol: "VectorMechanicsSolver::solve_equilibrium",
        audit_finding: 1,
        notes: "bar network; THMC/fracture/adjoint load-bearing (P0 #1)",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::mechanics",
        symbol: "VectorMechanicsSolver::solve_equilibrium_with_pcg_report",
        audit_finding: 3,
        notes: "H4 telemetry; model for SolveReport adoption",
        report_wired: true,
    },
    SolverEntryPoint {
        module: "physics::solvers::thmc",
        symbol: "ThmcSolver::step",
        audit_finding: 3,
        notes: "operator-split; stacked-R exit open (Wave 1)",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::solvers::thmc_residual",
        symbol: "ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u",
        audit_finding: 4,
        notes: "dense cap 64 DOF; ‖R‖ honesty (Wave 1)",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::solvers::fracture_field",
        symbol: "PhaseFieldFractureSolver::update_damage_staggered",
        audit_finding: 12,
        notes: "staggered u↔d; within-step coupling open",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::solvers::electrochemistry",
        symbol: "ElectroChemicalSolver::solve_pnp_step",
        audit_finding: 5,
        notes: "explicit Picard default; scale / graph generality",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::extruded_plate",
        symbol: "ExtrudedPlateMechanics::solve_equilibrium",
        audit_finding: 2,
        notes: "Q1 hex; 9×8×2 roof PCG stall (P0 #2)",
        report_wired: false,
    },
    SolverEntryPoint {
        module: "physics::adjoint",
        symbol: "AdjointCompliance::compliance_and_sensitivity",
        audit_finding: 19,
        notes: "inner f64 PCG; discrete adjoint surrogate",
        report_wired: false,
    },
];

/// Count of catalogued solver entry points.
#[must_use]
pub const fn solver_entry_point_count() -> usize {
    SOLVER_ENTRY_POINTS.len()
}

/// Count of entry points that currently emit [`SolveReport`].
#[must_use]
pub fn solver_entry_points_wired_count() -> usize {
    SOLVER_ENTRY_POINTS
        .iter()
        .filter(|e| e.report_wired)
        .count()
}

/// Count of catalogued-but-unwired entry points (honest Wave-1 backlog).
#[must_use]
pub fn solver_entry_points_open_count() -> usize {
    SOLVER_ENTRY_POINTS
        .iter()
        .filter(|e| !e.report_wired)
        .count()
}

/// Typed probe for solve_report posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolveReportPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5_claimed: bool,
    pub green_claim_blocked: bool,
    pub contract_landed: bool,
    pub inventory_catalogued: bool,
    pub all_sites_wired: bool,
    pub wired_entry_points: usize,
    pub open_entry_points: usize,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the solve_report surface.
#[must_use]
pub fn solve_report_honest_posture_bundle() -> SolveReportPostureProbe {
    SolveReportPostureProbe {
        physics_green: SOLVE_REPORT_PHYSICS_GREEN,
        production_wired: SOLVE_REPORT_PRODUCTION_WIRED,
        master: SOLVE_REPORT_MASTER,
        op5_claimed: SOLVE_REPORT_OP5_CLAIMED,
        green_claim_blocked: SOLVE_REPORT_GREEN_CLAIM_BLOCKED,
        contract_landed: SOLVE_REPORT_CONTRACT_LANDED,
        inventory_catalogued: SOLVE_REPORT_INVENTORY_CATALOGUED,
        all_sites_wired: SOLVE_REPORT_ALL_SITES_WIRED,
        wired_entry_points: solver_entry_points_wired_count(),
        open_entry_points: solver_entry_points_open_count(),
        honest_fence: SOLVE_REPORT_HONEST_FENCE,
        posture_tag: SOLVE_REPORT_POSTURE_TAG,
        deepen_cell: W29_SOLVE_REPORT_DEEPEN_CELL,
    }
}

/// Contract + inventory landed with production/master/GREEN/OP-5 / all-sites honestly open.
#[must_use]
pub fn solve_report_posture_honest(probe: &SolveReportPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5_claimed
        && probe.green_claim_blocked
        && probe.contract_landed
        && probe.inventory_catalogued
        && !probe.all_sites_wired
        && probe.wired_entry_points >= 1
        && probe.open_entry_points >= 1
        && probe.honest_fence.contains("contract_landed=true")
        && probe.honest_fence.contains("all_sites_wired=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("op5_claimed=false")
        && probe.deepen_cell == W29_SOLVE_REPORT_DEEPEN_CELL
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 / fake all-sites claims on this surface.
#[must_use]
pub fn solve_report_refuse_overclaim(probe: &SolveReportPostureProbe) -> Result<(), &'static str> {
    if probe.physics_green || SOLVE_REPORT_PHYSICS_GREEN {
        return Err("SOLVE_REPORT_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired || SOLVE_REPORT_PRODUCTION_WIRED {
        return Err("SOLVE_REPORT_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master || SOLVE_REPORT_MASTER {
        return Err("SOLVE_REPORT_MASTER must stay false — not claimed by SolveReport alone");
    }
    if probe.op5_claimed || SOLVE_REPORT_OP5_CLAIMED {
        return Err("SOLVE_REPORT_OP5_CLAIMED must stay false — no invent OP-5");
    }
    if !probe.green_claim_blocked || !SOLVE_REPORT_GREEN_CLAIM_BLOCKED {
        return Err("SOLVE_REPORT_GREEN_CLAIM_BLOCKED must stay true while GREEN is refused");
    }
    if probe.all_sites_wired || SOLVE_REPORT_ALL_SITES_WIRED {
        return Err("SOLVE_REPORT_ALL_SITES_WIRED must stay false until Wave-1 adoption closes");
    }
    if !solve_report_posture_honest(probe) {
        return Err("solve_report posture fence inconsistent");
    }
    Ok(())
}

/// Honesty probe — fence holds; bar PCG lift converges under a unit residual.
#[must_use]
pub fn solve_report_honesty_probe() -> bool {
    let probe = solve_report_honest_posture_bundle();
    if solve_report_refuse_overclaim(&probe).is_err() {
        return false;
    }
    let pcg = crate::physics::mechanics::BarNetworkPcgReport {
        iterations: 1,
        rel_residual: 1e-9,
        stiffness_scale: 1.0,
        e_ref: 1.0,
        dx_char: 1.0,
    };
    let report = from_bar_network_pcg(pcg, 1e-6, PrecisionLane::F32BurnBarPcg);
    report.converged()
        && report.ensure_converged().is_ok()
        && report.residual_is_admissible()
        && report.stall_ratio() == Some(1e-9 / 1e-6)
        && solver_entry_point_count() == SOLVER_ENTRY_POINTS.len()
        && solver_entry_points_wired_count() == 1
        && !SOLVE_REPORT_ALL_SITES_WIRED
}

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
        assert!(ok.ensure_converged().is_ok());
        assert!(ok.residual_is_admissible());
        assert!((ok.stall_ratio().expect("stall_ratio") - 0.1).abs() < 1e-6);

        let stall = SolveReport {
            rel_residual: 0.94,
            rel_tol: 1e-4,
            ..ok
        };
        assert!(!stall.converged());
        assert!(stall.ensure_converged().is_err());
        assert!(stall.stall_ratio().expect("stall") > 1.0);

        let no_tol = SolveReport { rel_tol: 0.0, ..ok };
        assert!(!no_tol.converged());
        assert!(no_tol.stall_ratio().is_none());
        assert_eq!(
            no_tol.ensure_converged().expect_err("zero tol"),
            "SolveReport rel_tol must be positive for converged predicate"
        );
    }

    #[test]
    fn solve_report_refuses_nan_inf_residual() {
        let base = SolveReport {
            iterations: 1,
            rel_residual: f32::NAN,
            stiffness_scale: 1.0,
            e_ref: 1.0,
            dx_char: 1.0,
            rel_tol: 1e-6,
            lane: PrecisionLane::HostKrylov,
        };
        assert!(!base.converged());
        assert!(!base.residual_is_admissible());
        assert_eq!(
            base.ensure_converged().expect_err("nan"),
            "SolveReport residual not admissible (NaN/Inf/negative)"
        );

        let inf = SolveReport {
            rel_residual: f32::INFINITY,
            ..base
        };
        assert!(!inf.converged());
        assert!(!inf.residual_is_admissible());

        let neg = SolveReport {
            rel_residual: -1e-3,
            ..base
        };
        assert!(!neg.residual_is_admissible());
        assert!(!neg.converged());
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
        assert!(via_fn.ensure_converged().is_ok());
    }

    #[test]
    fn precision_lane_catalog_stable() {
        assert_eq!(PrecisionLane::ALL.len(), 5);
        let labels: Vec<_> = PrecisionLane::ALL.iter().map(|l| l.audit_label()).collect();
        assert_eq!(
            labels,
            [
                "f32_burn_bar_pcg",
                "f64_adjoint_bar_pcg",
                "hex_q1_pcg",
                "dense_direct",
                "host_krylov",
            ]
        );
        // Labels unique.
        let mut sorted = labels.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), labels.len());
    }

    #[test]
    fn solver_entry_inventory_honest_wiring() {
        assert_eq!(solver_entry_point_count(), 8);
        assert_eq!(solver_entry_points_wired_count(), 1);
        assert_eq!(solver_entry_points_open_count(), 7);
        assert!(!SOLVE_REPORT_ALL_SITES_WIRED);

        let wired: Vec<_> = SOLVER_ENTRY_POINTS
            .iter()
            .filter(|e| e.report_wired)
            .map(|e| e.symbol)
            .collect();
        assert_eq!(
            wired,
            ["VectorMechanicsSolver::solve_equilibrium_with_pcg_report"]
        );

        // Symbols unique.
        let mut symbols: Vec<_> = SOLVER_ENTRY_POINTS.iter().map(|e| e.symbol).collect();
        let before = symbols.len();
        symbols.sort_unstable();
        symbols.dedup();
        assert_eq!(symbols.len(), before);
    }

    #[test]
    fn solve_report_honest_fence_blocks_production_master_green_op5() {
        let probe = solve_report_honest_posture_bundle();
        assert_eq!(probe.deepen_cell, W29_SOLVE_REPORT_DEEPEN_CELL);
        assert!(solve_report_posture_honest(&probe));
        solve_report_refuse_overclaim(&probe).expect("honest refuse");
        assert!(SOLVE_REPORT_HONEST_FENCE.contains("production_wired=false"));
        assert!(SOLVE_REPORT_HONEST_FENCE.contains("physics_green=false"));
        assert!(SOLVE_REPORT_HONEST_FENCE.contains("op5_claimed=false"));
        assert!(SOLVE_REPORT_HONEST_FENCE.contains("all_sites_wired=false"));
        assert!(!SOLVE_REPORT_PHYSICS_GREEN);
        assert!(!SOLVE_REPORT_PRODUCTION_WIRED);
        assert!(!SOLVE_REPORT_MASTER);
        assert!(!SOLVE_REPORT_OP5_CLAIMED);
        assert!(SOLVE_REPORT_GREEN_CLAIM_BLOCKED);
        assert!(!SOLVE_REPORT_ALL_SITES_WIRED);
    }

    #[test]
    fn solve_report_refuse_overclaim_detects_fake_green() {
        let mut probe = solve_report_honest_posture_bundle();
        probe.physics_green = true;
        let err = solve_report_refuse_overclaim(&probe).expect_err("fake green");
        assert!(err.contains("PHYSICS_GREEN"));
    }

    #[test]
    fn solve_report_honesty_probe_holds() {
        assert!(solve_report_honesty_probe());
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
pub mod ai;
pub mod constants;
pub mod constants_registry;
#[cfg(feature = "math-constants")]
pub use constants::landauer_bit_energy_joules;
pub use constants_registry::GroundedConst;
pub mod core;
#[cfg(feature = "design-query")]
pub mod design;
pub mod embodied;
pub mod gate;
pub mod gate_server_router;
pub mod manifest;
pub mod physics;
pub use physics::hertz_contact::{HertzImpactOutcome, HertzImpactParams};
/// Production Hertz / Hunt–Crossley sphere-on-plane impact (PB-B10).
///
/// Body calls [`physics::hertz_contact::admit_hertz_sphere_plane_impact`] so the
/// contact surface has a non-test reader (re-export alone is refused by wire).
#[must_use]
pub fn hertz_sphere_plane_impact(
    params: HertzImpactParams,
    modal: &[physics::modal_loss::ImpactSoundMode],
    dt_s: f64,
    energy_rel_tol: f64,
) -> Result<HertzImpactOutcome, physics::PhysicsError> {
    let outcome =
        physics::hertz_contact::admit_hertz_sphere_plane_impact(params, modal, dt_s, energy_rel_tol)?;
    if outcome.johnson_duration_citation
        != physics::hertz_contact::JOHNSON_1985_DURATION_CITATION
    {
        return Err(physics::PhysicsError::Domain {
            detail: "hertz: outcome citation pin drifted from JOHNSON_1985_DURATION_CITATION"
                .into(),
        });
    }
    Ok(outcome)
}
pub use physics::orchestration::{
    OrchestrationPostureProbe, TopologyPhysicsOrchestrator, TopologyPlanIntent,
};
pub use physics::time_orchestration::{
    ClockValidationError, MechanicsInnerLoopConfig, SimulationClocks, TimeOrchestrationPostureProbe,
};
/// Finite-cell hexahedral elasticity over an occupancy field (PB-B8): field, materials, grid, rule, elements, assembly
/// and analysis types.
pub use physics::solvers::finite_cell::analysis::{ApparentMass, InertiaRelief, ModeRequest};
pub use physics::solvers::finite_cell::assembly::{
    Diagnostics as FiniteCellDiagnostics, Discretisation, DiscretisationSpec, MassProperties,
    RawMoments as FiniteCellRawMoments, Springs as FiniteCellSprings, System as FiniteCellSystem,
};
pub use physics::solvers::finite_cell::element::{ElementKind, ElementMatrices};
pub use physics::solvers::finite_cell::grid::TensorGrid;
pub use physics::solvers::finite_cell::quadrature::{
    CellQuadrature, Exactness, QPoint, QuadratureSpec,
};
pub use physics::solvers::finite_cell::{FiniteCellRefuse, MaterialTable, OccupancyField, Voigt6};
pub use physics::solvers::fracture_field::PhaseFieldFractureSolver;
pub use physics::solvers::krylov_host::KrylovHostPostureProbe;
pub use physics::solvers::photonics::{
    DecPatchCsrInnerMode, DecPatchCurlConstitutive, PhotonicsDecFacesPatch,
    PhotonicsDecPatchConfig, PhotonicsHelmholtzSolver, PhotonicsLaneHonesty, PhotonicsSolver,
};
pub mod pnp_bridge;
#[cfg(feature = "ros2-contract")]
pub mod ros;

pub mod ci_research_gate;
pub mod detect_whether_workflow_job;
pub use ci_research_gate::ResearchCiPosture;
pub mod cargo_test_gap_census;
pub mod cartridge_migration_stub;
pub mod migration;
pub mod nested_drift_census;
pub mod night_residual_deepen;
pub mod runtime;
pub mod solve;
pub use solve::{CapMigrationDisposition, CapMigrationSite, CAP_MIGRATION_SITES};
pub mod solve_report;
pub mod swarm_manifold_deepen;
pub mod web_constitutive;

#[cfg(feature = "ucrs-provenance")]
pub use runtime::gate::TransitionEvidenceWire;

#[allow(deprecated)]
pub use cartridge_migration_stub::*;
pub use migration::{
    W9CartridgeInjectAbsence, W9MigrationPostureProbe, W9PhaseASurfaceKind, W9PhaseASurfaceRow,
};

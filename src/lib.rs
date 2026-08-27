// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
pub mod ai;
pub mod constants;
pub mod constants_registry;
#[cfg(feature = "math-constants")]
pub use constants::landauer_bit_energy_joules;
pub mod core;
#[cfg(feature = "design-query")]
pub mod design;
pub mod embodied;
pub mod gate;
pub mod gate_server_router;
pub mod manifest;
pub mod physics;
pub use physics::orchestration::{
    OrchestrationPostureProbe, TopologyPlanIntent, TopologyPhysicsOrchestrator,
};
pub mod pnp_bridge;
#[cfg(feature = "ros2-contract")]
pub mod ros;

pub mod cargo_test_gap_census;
pub mod cartridge_migration_stub;
pub mod nested_drift_census;
pub mod night_residual_deepen;
pub mod runtime;
pub mod solve_report;
pub mod swarm_manifold_deepen;
pub mod web_constitutive;

#[cfg(feature = "ucrs-provenance")]
pub use runtime::gate::TransitionEvidenceWire;

#[allow(deprecated)]
pub use cartridge_migration_stub::*;

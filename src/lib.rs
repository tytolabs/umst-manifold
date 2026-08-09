// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

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
pub mod pnp_bridge;
#[cfg(feature = "ros2-contract")]
pub mod ros;

pub mod runtime;
pub mod solve_report;
pub mod cargo_test_gap_census;
pub mod nested_drift_census;
pub mod night_residual_deepen;
pub mod swarm_manifold_deepen;
pub mod web_constitutive;
pub mod cartridge_migration_stub;

#[allow(deprecated)]
pub use cartridge_migration_stub::*;

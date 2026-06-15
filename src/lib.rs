// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod ai;
pub mod constants;
pub mod constants_registry;
#[cfg(feature = "math-constants")]
pub use constants::landauer_bit_energy_joules;
pub mod core;
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
pub mod W9_MIGRATION;

#[allow(deprecated)]
pub use W9_MIGRATION::*;

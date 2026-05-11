// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod adjoint;
pub mod dec_operators;
pub mod dec_primal;
#[cfg(feature = "topology-density-evolution")]
pub mod extruded_plate;
pub mod framework;
pub mod laplacian;
pub mod linear;
pub mod mechanics;
pub mod orchestration;
pub mod protocols;
#[cfg(feature = "topology-density-evolution")]
pub mod q1_hex_elasticity;
pub mod rheology_analytic;
pub mod solvers;
pub mod time_orchestration;
pub mod topology;
#[cfg(feature = "topology-density-evolution")]
pub mod topology_filter;

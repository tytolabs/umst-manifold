// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod adjoint;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod adjoint_q1_hex;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod compliance_functional;
pub mod dec_operators;
pub mod dec_primal;
pub mod error;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod device_sheet;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod extruded_plate;
pub mod framework;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod grid_witness;
pub mod laplacian;
pub mod linear;
pub mod mechanics;
pub mod mechanics_operator;
#[cfg(feature = "mechanics-adjoint")]
pub mod mechanics_solve_port;
pub mod operator;
pub mod orchestration;
pub(crate) mod pipeline;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod pcg_reduction;
#[cfg(feature = "topology-density-evolution")]
pub mod prime_spectral_filter;
pub mod protocols;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod q1_hex_elasticity;
pub mod rheology_analytic;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod solve_budget;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod solver_region;
pub mod solvers;
pub mod thmc_umst_sync;
pub mod time_orchestration;
pub mod topology;
#[cfg(feature = "topology-density-evolution")]
pub mod topology_filter;

pub use error::PhysicsError;
pub(crate) use pipeline::{and_then_state, map_state};

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
pub mod adjoint;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod adjoint_q1_hex;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod compliance_functional;
pub mod dec_operators;
pub mod dec_primal;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod device_sheet;
pub mod error;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod extruded_plate;
pub mod framework;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod grid_witness;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod hex_elasticity;
pub mod laplacian;
pub mod linear;
pub mod mechanics;
pub mod mechanics_operator;
#[cfg(feature = "mechanics-adjoint")]
pub mod mechanics_solve_port;
pub mod operator;
pub mod operators;
pub mod orchestration;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod pcg_reduction;
#[cfg(feature = "thmc-coupled")]
pub(crate) mod pipeline;
#[cfg(feature = "topology-density-evolution")]
pub mod prime_spectral_filter;
pub mod protocols;
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

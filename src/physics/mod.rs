// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod adjoint;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod adjoint_q1_hex;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod compliance_functional;
pub mod dec_operators;
pub mod dec_primal;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod extruded_plate;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod grid_witness;
pub mod framework;
pub mod laplacian;
pub mod linear;
pub mod mechanics;
pub mod mechanics_operator;
#[cfg(feature = "mechanics-adjoint")]
pub mod mechanics_solve_port;
pub mod operator;
pub mod orchestration;
#[cfg(feature = "topology-density-evolution")]
pub mod prime_spectral_filter;
pub mod protocols;
#[cfg(feature = "rheology-bingham")]
pub mod rheology_analytic;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod pcg_reduction;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod q1_hex_elasticity;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod device_sheet;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod solver_region;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod solve_budget;
pub mod solvers;
pub mod time_orchestration;
pub mod topology;
#[cfg(feature = "topology-density-evolution")]
pub mod topology_filter;

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod adjoint;
#[cfg(feature = "mechanics-adjoint-q1-hex")]
pub mod adjoint_q1_hex;
pub mod dec_operators;
pub mod dec_primal;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod extruded_plate;
pub mod framework;
pub mod laplacian;
pub mod linear;
pub mod mechanics;
pub mod mechanics_operator;
pub mod operator;
pub mod orchestration;
#[cfg(feature = "topology-density-evolution")]
pub mod prime_spectral_filter;
pub mod protocols;
#[cfg(any(
    feature = "topology-density-evolution",
    feature = "mechanics-voigt-cauchy"
))]
pub mod q1_hex_elasticity;
pub mod rheology_analytic;
pub mod solvers;
pub mod time_orchestration;
pub mod topology;
#[cfg(feature = "topology-density-evolution")]
pub mod topology_filter;

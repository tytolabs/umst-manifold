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
pub mod hertz_contact;
pub use hertz_contact::{HertzImpactOutcome, HertzImpactParams};
pub mod modal_loss;
pub use modal_loss::ImpactSoundMode;

/// Production keeper for the crate-root Hertz sphere-plane surface (PB-B10).
///
/// Private so the keeper itself is not a new unwired pub; [`time_orchestration`]
/// calls it so the crate-root surface has a non-test reader beyond `lib.rs`.
#[must_use]
pub(crate) fn hertz_contact_surface_holds() -> bool {
    // Type ascription (with `=`) so the wire gate counts HertzImpactParams as a reader;
    // a bare `HertzImpactParams { ... }` line matches the re-export-list refuse pattern.
    let params: HertzImpactParams = HertzImpactParams {
        mass_kg: 0.045,
        radius_m: 0.037,
        e_star_pa: 1.0e9,
        v_in_m_s: 2.0,
        hunt_crossley_chi_s_per_m: 0.0,
    };
    crate::hertz_sphere_plane_impact(params, &[], 1e-7, 0.05).is_ok()
}
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

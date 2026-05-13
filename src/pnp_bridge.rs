// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Surrogate / reduced-order hooks for Poisson–Nernst–Planck (PNP), gated on **`electrochemistry-pnp`**.
//!
//! The canonical physics implementation lives in [`crate::physics::solvers::electrochemistry`]. This
//! module is intentionally thin: same tensor contract as
//! [`crate::physics::solvers::electrochemistry::ElectroChemicalSolver::solve_pnp_step`],
//! with a no-op passthrough until a learned surrogate is wired in.

#[cfg(feature = "electrochemistry-pnp")]
use burn::tensor::{backend::Backend, Int, Tensor};
#[cfg(feature = "electrochemistry-pnp")]
use crate::physics::solvers::electrochemistry::ElectroChemicalSolver;

/// Placeholder **PNP surrogate** step: same arguments and rank-3 tensor contract as
/// [`ElectroChemicalSolver::solve_pnp_step`](crate::physics::solvers::electrochemistry::ElectroChemicalSolver::solve_pnp_step);
/// currently returns inputs unchanged (explicit no-op).
///
/// # Tensor shapes (must match `solve_pnp_step`)
/// - `electric_potential`: **`[B, N, 1]`**
/// - `ion_concentration`: **`[B, N, 2]`** (e.g. two species channels)
/// - `edges_b1`: **`[2, E]`** (`Int` topology)
/// - `permittivity`: **`[B, N, 1]`**
/// - `diffusivity`: **`[B, N, 2]`**
///
/// Returns **`(electric_potential, ion_concentration)`** with the same shapes as the inputs.
#[cfg(feature = "electrochemistry-pnp")]
#[allow(unused_variables)]
pub fn pnp_surrogate_step<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let _ = (solver, dt, edges_b1, permittivity, diffusivity);
    (electric_potential, ion_concentration)
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 6 — Poisson–Nernst–Planck (PNP) and electrostatic **scaffold** on the DEC 1-skeleton.
//!
//! ## Physics intent
//! Model coupled ion transport and electric fields in electrolytes / interfaces (e.g. supercap
//! electrodes): **Poisson** for potential \(\Phi\),
//! \(\nabla\cdot(\varepsilon \nabla\Phi) = -\rho_e\) with net charge density \(\rho_e\) from species
//! concentrations, and **Nernst–Planck** balances \(\partial_t c_i + \nabla\cdot \mathbf{J}_i = 0\) with
//! drift–diffusion flux \(\mathbf{J}_i = -D_i \nabla c_i - z_i (F/RT)\, D_i c_i \nabla\Phi\).
//! Production builds will thread spatially varying \(\varepsilon\), mobilities, and
//! Scharfetter–Gummel edge fluxes; this module pins tensor ranks and a differentiable **stub** graph
//! behind `solver-experimental`.
//!
//! ## Gaps vs full PNP / Scharfetter–Gummel (experimental path)
//! - **Poisson**: one explicit relaxation step toward a discrete Poisson residual
//!   \(\Delta\Phi + \rho_e/\varepsilon \approx 0\) (constant-\(\varepsilon\) surrogate on the primal
//!   Laplacian), not an implicit linear solve; no \(\nabla\varepsilon\cdot\nabla\Phi\) term; \(\rho_e\)
//!   is the minimal **monovalent** net charge \(F(c^+ - c^-)\) from nodal concentrations only (no
//!   fixed background charge, no multiply-charged species).
//! - **Nernst–Planck**: Fickian diffusion \(\partial_t c \approx D\,\Delta c\) plus a **nodal drift surrogate**
//!   \(\Delta c_{\mathrm{drift}} \approx -z_i \frac{F}{RT} D_i c_i \,\Delta\Phi\) using the same discrete
//!   scalar Laplacian of \(\Phi\) as for diffusion (not edge SG fluxes); \(z=\pm1\) on channels `0`/`1`.
//!   No Stefan–Maxwell coupling; [**Scharfetter–Gummel**](https://en.wikipedia.org/wiki/Scharfetter%E2%80%93Gummel_method)
//!   edge stabilization (**Note — SG:** Peclet-aware upwinding on graph edges) remains deferred.
//! - **Coupling**: \(\Phi\) and \(c\) updates are split; there is no Newton–Raphson block solve or
//!   consistent time-splitting analysis—suitable for autodiff smoke tests, not production accuracy.

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::laplacian::TopologicalLaplacian;

/// Drift–diffusion scaling: `faraday_const` is \(F\) (C/mol); `gas_const` must be **\(R\,T\)** (J/(mol·K)×K)
/// so that \(F/(RT) =\,\) `faraday_const / gas_const` in the Nernst–Planck drift term.
pub struct ElectroChemicalSolver {
    pub faraday_const: f32,
    pub gas_const: f32,
}

impl ElectroChemicalSolver {
    /// One explicit coupled **surrogate** PNP step (`dt` is the explicit time increment; full implicit
    /// Poisson / SG is still out of scope—see module **Gaps** when `solver-experimental` is on).
    ///
    /// # Shapes (contract, `[Batch, N, …]`)
    /// - `electric_potential`: `[B, N, 1]`
    /// - `ion_concentration`: `[B, N, 2]` (e.g. two species channels)
    /// - `permittivity`: `[B, N, 1]`
    /// - `diffusivity`: `[B, N, 2]`
    /// - `edges_b1`: `[2, E]`
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns `(electric_potential, ion_concentration)` unchanged.
    ///
    /// ## `--features solver-experimental`
    /// Applies [`TopologicalLaplacian::scalar_laplacian`] on \(\Phi\) and `c`, then:
    /// - **Poisson surrogate**: one relaxation step using net charge density
    ///   \(\rho_e = F(c^+ - c^-)\) (channels `0` / `1`) so the graph residual
    ///   \(\Delta\Phi + \rho_e/\varepsilon\) drives \(\Phi\) (see module **Gaps** for limitations).
    /// - **NP surrogate**: explicit diffusion plus drift `c ← c + dt\,(D \odot \Delta c + J_{\mathrm{drift}})` with
    ///   \(J_{\mathrm{drift},i} = -z_i (F/RT) D_i c_i \Delta\Phi\) (channels `0`/`1`: \(z=+1/-1\)).
    ///   **Note — SG:** replace nodal drift/diffusion surrogates with Scharfetter–Gummel edge fluxes.
    #[allow(unused_variables)]
    pub fn solve_pnp_step<B: Backend<FloatElem = f32>>(
        &self,
        dt: f32,
        electric_potential: Tensor<B, 3>,
        ion_concentration: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        permittivity: Tensor<B, 3>,
        diffusivity: Tensor<B, 3>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            let _ = (dt, edges_b1, permittivity, diffusivity);
            (electric_potential, ion_concentration)
        }

        #[cfg(feature = "electrochemistry-mvp")]
        {
            solve_pnp_step_experimental(
                self,
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }
}

/// Placeholder hook for **Scharfetter–Gummel** edge flux assembly (drift–diffusion upwinding).
///
/// Full PNP on the DEC graph should accumulate conservative fluxes on primal edges with
/// Peclet-dependent weighting between adjacent nodal values and electrostatic differences.
/// The experimental [`solve_pnp_step_experimental`] path still uses nodal Laplacian surrogates;
/// this function exists only to anchor the future API and docs. It is not called yet.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(dead_code)]
fn sg_flux_placeholder() {}

#[cfg(feature = "electrochemistry-mvp")]
/// Dimensionless multiplier for explicit Poisson relaxation (stability vs `dt`).
const POISSON_RELAX_SCALE: f32 = 1e-2;

#[cfg(feature = "electrochemistry-mvp")]
fn solve_pnp_step_experimental<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let mask_phi = Tensor::<B, 3>::ones_like(&electric_potential);
    let lap_phi = TopologicalLaplacian::scalar_laplacian(
        electric_potential.clone(),
        edges_b1.clone(),
        mask_phi,
    );

    let mask_c = Tensor::<B, 3>::ones_like(&ion_concentration);
    let lap_c = TopologicalLaplacian::scalar_laplacian(ion_concentration.clone(), edges_b1, mask_c);

    // Gauss / Poisson: net charge from monovalent cation (ch 0) and anion (ch 1), ρ_e = F (c+ − c−).
    let c_plus = ion_concentration.clone().narrow(2, 0, 1);
    let c_minus = ion_concentration.clone().narrow(2, 1, 1);
    let rho_e = c_plus.sub(c_minus).mul_scalar(solver.faraday_const);

    let eps_safe = permittivity.clamp_min(1e-30_f32);
    let rho_over_eps = rho_e.div(eps_safe);

    // One Jacobi-like relaxation step: Φ ← Φ − η (ΔΦ + ρ_e/ε), η = POISSON_RELAX_SCALE * dt.
    let poisson_residual = lap_phi.clone().add(rho_over_eps);
    let relax = POISSON_RELAX_SCALE * dt;
    let phi_next = electric_potential.sub(poisson_residual.mul_scalar(relax));

    // Note — SG: Scharfetter–Gummel edge fluxes for drift–diffusion; Fickian Δc is a nodal surrogate only.
    // Explicit Fickian diffusion on concentrations.
    let mut c_next = ion_concentration
        .clone()
        .add(lap_c.mul(diffusivity.clone()).mul_scalar(dt));

    // Note — SG: Replace drift surrogate (nodal lap_phi) with SG-upwinded edge flux divergence.
    // Nernst–Planck drift surrogate: J_drift,i = -z_i (F/RT) D_i c_i lap_phi (lap_phi ≡ discrete ΔΦ).
    let rt_safe = solver.gas_const.max(1e-30_f32);
    let f_over_rt = solver.faraday_const / rt_safe;
    let batch = ion_concentration.dims()[0];
    let n_nodes = ion_concentration.dims()[1];
    let device = ion_concentration.device();
    // Per channel: -z * (F/RT) for z = +1, -1  =>  [-F/RT, +F/RT]
    let drift_scale = Tensor::<B, 3>::from_floats([[[-f_over_rt, f_over_rt]]], &device)
        .reshape([1, 1, 2])
        .expand([batch, n_nodes, 2]);
    let j_drift = drift_scale
        .mul(diffusivity)
        .mul(ion_concentration.clone())
        .mul(lap_phi);
    c_next = c_next.add(j_drift.mul_scalar(dt));

    (phi_next, c_next)
}

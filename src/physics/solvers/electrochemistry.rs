// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg_attr(
    feature = "electrochemistry-mvp",
    allow(dead_code, clippy::doc_lazy_continuation, clippy::needless_range_loop)
)]

//! Phase 6 — Poisson–Nernst–Planck (PNP) and electrostatic **scaffold** on the DEC 1-skeleton.
//!
//! ## Physics intent
//! Model coupled ion transport and electric fields in electrolytes / interfaces (e.g. supercap
//! electrodes): **Poisson** for potential \(\Phi\),
//! \(\nabla\cdot(\varepsilon \nabla\Phi) = -\rho_e\) with net charge density \(\rho_e\) from species
//! concentrations, and **Nernst–Planck** balances \(\partial_t c_i + \nabla\cdot \mathbf{J}_i = 0\) with
//! drift–diffusion flux \(\mathbf{J}_i = -D_i \nabla c_i - z_i (F/RT)\, D_i c_i \nabla\Phi\).
//! Production builds will thread spatially varying \(\varepsilon\), mobilities, and
//! higher-order species coupling; this module pins tensor ranks and a **research** PNP graph
//! behind `electrochemistry-mvp` / `solver-research`.
//!
//! ## Gaps vs full PNP / Scharfetter–Gummel (experimental path)
//! - **Poisson**: on a **simple path chain** (contiguous nodes `0..N-1`, `N-1` edges matching the
//!   `edges_b1`), each sub-step solves **\(\nabla\cdot(\varepsilon\nabla\Phi)=-\rho_e\)** on interior
//!   nodes with harmonic edge weights \(\varepsilon_{i+\frac12}=\tfrac12(\varepsilon_i+\varepsilon_{i+1})\)
//!   via `poisson_chain_net_charge_variable_eps_thomas` + **Thomas**; endpoint Dirichlet values come
//!   from `electric_potential`. `try_solve_poisson_chain_thomas` accepts a **`rho_over_eps`** tensor
//!   and [`ElectroChemicalSolver::mesh_spacing`] \(h\): it reconstructs \(\rho_{e,i}=(\rho/\varepsilon)_i\,\varepsilon_i\)
//!   and solves \(\mathcal{L}_{\mathrm{idx}}\Phi=-h^2\rho_e\) on interior nodes (same \(\mathcal{L}_{\mathrm{idx}}\) as
//!   [`crate::physics::laplacian::TopologicalLaplacian`] on the chain). Non-chain graphs use **Jacobi-preconditioned CG** on the graph Laplacian
//!   (`poisson_graph_uniform_laplacian_jacobi_pcg`). Minimal
//!   **monovalent** \(\rho_e\) (no fixed background charge, no multiply-charged species).
//! - **Matrix-free / Krylov:** `pnp_be_full_sg_jacobian_matvec_nm_f64` applies a node-major
//!   finite-difference \(J_{\mathrm{nm}} v\) for the same full-SG backward-Euler residual as the band FD
//!   Jacobian. When [`NewtonPnpContext::full_sg_correction_use_gmres`] is **`true`** (full SG, no frozen-**J**
//!   inners), [`try_solve_pnp_be_newton_chain_host`] uses host [`super::krylov_host::gmres_f32_try`] for each
//!   Newton correction instead of band assembly + dense expand. Regression tests
//!   `pnp3d_full_sg_gmres_delta_matches_dense_expand_small_chain` / `_chain_n17` lock the matvec + GMRES stack;
//!   `try_solve_pnp_be_newton_chain_host_full_sg_gmres_matches_dense_smoke` locks the full host Newton path.
//! - **Nernst–Planck**: **Scharfetter–Gummel** conservative edge flux (see `solve_pnp_split_step_experimental_with_refs` in this module)
//!   \(J_e = (D_e/h_e)\,[c_a B(z\Delta\phi_{ba}) - c_b B(z\Delta\phi_{ab})]\) with \(\Delta\phi_{ba}=\phi_b-\phi_a\),
//!   Bernoulli \(B(x)=x/(e^x-1)\) (Taylor form for \(|x|\ll 1\)), assembled with
//!   [`crate::physics::dec_primal::primal_divergence_from_edge_flux_topo`]. Channels `0`/`1` use \(z=\pm1\).
//!   No Stefan–Maxwell multiply-charged species yet.
//! - **Coupling**: \(\Phi\) and \(c\) updates are **split** (Poisson: Thomas on a path chain or explicit
//!   relaxation otherwise, then SG NP flux and explicit mass update). [`ElectroChemicalSolver::coupling_picard_iters`] \(>1\) runs **Picard**
//!   outer sweeps **within one explicit step**: each sweep solves Poisson with \(\rho_e(c^{(k)})\), then
//!   \(c^{n+1}=c^n-\Delta t\,\nabla\!\cdot J(\Phi^{(k+1)},c^{(k)})\) (fixed \(c^n\) across sweeps). Optional
//!   Picard early stop: [`ElectroChemicalSolver::coupling_picard_tol_linf`] on \((\Phi,c)\), and/or
//!   [`ElectroChemicalSolver::coupling_picard_tol_delta_phi_linf`] / [`ElectroChemicalSolver::coupling_picard_tol_delta_phi_l2`]
//!   on successive \(\Phi\) alone. Implementation: `solve_pnp_step_experimental` — **Picard only**;
//!   it does **not** invoke [`NewtonPnpContext`] or the host Newton kernel. For implicit BE + Newton
//!   on contiguous path-chain graphs, call [`ElectroChemicalSolver::try_solve_pnp_backward_euler_newton_chain`] directly,
//!   or set [`ElectroChemicalSolver::pnp_implicit_newton_chain`] and use [`ElectroChemicalSolver::solve_pnp_step_dispatch`]
//!   (same `None` default as today: explicit Picard path only).
//! - **Track 14 (chain-only implicit BE):** [`ElectroChemicalSolver::try_solve_pnp_backward_euler_newton_chain`]
//!   solves the **fully implicit backward Euler** residual with the same **variable-\(\varepsilon\)** chain
//!   Poisson block as the host helper `pnp_be_residual_vector_f64` (harmonic \(\varepsilon\) on edges) plus SG NP rows,
//!   on a **contiguous path** using host `f64` Newton: when **`linearize_sg_fickian` is `true`**, the Jacobian is the
//!   affine Fickian / Debye–Hückel model: concentration blocks are **uncoupled from \(\Phi\)** and each
//!   species row is a **chain Laplacian \(+\,1/\Delta t\)**, while \(\Phi\) rows couple to \(c^\pm\)
//!   only through \(\rho_e\) — the Newton correction uses **three Thomas solves** per iteration
//!   (\(O(N)\) work). **Full nonlinear SG** (`linearize_sg_fickian: false`) uses **column finite differences**
//!   on the same physics in **node-major** \((\phi_i,c^+_i,c^-_i)\) order, stored in a **fixed row band**
//!   (\(kl,ku\) from nearest-neighbour coupling). Each Newton **correction** uses **`solve_newton_correction_full_sg_row_band_band_lu_or_dense_expand`**
//!   (**`solve_newton_correction_full_sg_row_band_via_band_lu`** then dense expand fallback **`solve_newton_correction_full_sg_row_band_via_dense_expand`**), which [`ElectroChemicalSolver::try_solve_pnp_backward_euler_newton_chain`] ships. When
//!   [`NewtonPnpContext::full_sg_frozen_jacobian_inner_iters`] is **`>1`**, inners **reuse the same frozen band**
//!   entries while re-solving each inner (**no extra column-FD probes** between inners).
//!   **`solve_newton_correction_full_sg_row_band_via_band_lu`** runs in-place `row_band_lu_factorize_partial_pivot`
//!   + `row_band_lu_solve_factored` on the assembled band Jacobian. The static LU envelope `PNP_CHAIN_FULL_SG_JAC_KL_LU`
//!     / `PNP_CHAIN_FULL_SG_JAC_KU_LU` is widened to **`3·17−1`** so pivot search + Schur fill match `solve_dense_linear`
//!     on the **N=17** CI fixture (`dim=51`; **dense expand** satisfies the pivot-search fallback once **`dim`** exceeds the LU slice).
//!     Default-CI `full_sg_newton_band_expand_dense_matches_dense_column_fd_reference` checks band assembly + dense-expand **δ** vs all-dense column FD on **N=17**; `full_sg_newton_dense_expand_matches_direct_gaussian_multi_n` locks dense-expand **δ** vs direct Gaussian on the expanded Jacobian at **N ∈ {17,33,49,65,81}**; `full_sg_newton_band_lu_matches_dense_expand_n17_fixture` asserts the band-LU entry point matches dense-expand on the same fixture. **`#[ignore]`** `full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity` prints timings (at **N=256** the static envelope is **narrower** than **`dim−1`**, so printed **`max|δ_lu−δ_de|`** is **not** expected to vanish — use for assembly / dense-expand wall-clock triage).
//!     **Still open (research):** band LU vs dense parity for **arbitrary** chain **`dim`** under a production-tight static **`(kl,ku)`** without the **`(3N)²`** scratch; general graphs (**Ring 2 R2.3**). Matrix-free matvec + GMRES (`pnp3d_full_sg_gmres_delta_matches_dense_expand_*`,
//!     optional [`NewtonPnpContext::full_sg_correction_use_gmres`] on the host Newton path) validates
//!     `pnp_be_full_sg_jacobian_matvec_nm_f64` + [`crate::physics::solvers::krylov_host::gmres_f32_try`] against dense-expand
//!     on path chains; otherwise default remains band-LU-first with dense fallback when the GMRES flag is **`false`**.
//!     The default [`ElectroChemicalSolver::solve_pnp_step`] path remains explicit split.
//! - **Mesh / extreme \(\Delta\phi\):** edge factor `h_inv` is now sourced from [`ElectroChemicalSolver::mesh_spacing`] (default `1.0` preserves the legacy dimensionless chain);
//!   Bernoulli uses `exp` on \(|z F\Delta\phi/(RT)|\); very large \(|pe|\) can **saturate** `f32::exp`
//!   before the ratio stabilises—tight \(\Delta t\) / smaller drift or double precision are the
//!   practical mitigations until a log-flux or exponential fitting formulation is added.
//! - **`mesh_spacing` on the explicit split path:** SG flux uses \(J\propto D/h\) with
//!   \(h=\) [`ElectroChemicalSolver::mesh_spacing`]. **Poisson:** the chain **Thomas** interior system is
//!   the same harmonic-\(\varepsilon\) index stencil as [`crate::physics::laplacian::TopologicalLaplacian`], with interior RHS
//!   scaled by **`h²`** so \(\mathcal{L}_{\mathrm{idx}}\Phi = -h^2\rho_e\) matches
//!   \(\nabla\!\cdot(\varepsilon\nabla\Phi)=-\rho_e\) on a uniform spacing \(h\); the non-chain **graph PCG**
//!   surrogate solves \(\mathcal{L}\phi=-h^2\rho_e/\varepsilon\) with `poisson_graph_uniform_laplacian_jacobi_pcg`
//!   (same \(\mathcal{L}\) as [`crate::physics::laplacian::TopologicalLaplacian`] with damage `1`; mean gauge). The implicit BE residual
//!   applies **`1/h²`** to the same \(\Phi\)-stencil in **`pnp_be_residual_vector_f64`** and
//!   **`fill_jacobian_linearized_sg_fickian`**. **`h = 1`** recovers legacy unit-edge behaviour.
//!   `tests/verification/pnp_debye_layer.rs::sg_flux_drift_scales_with_mesh_spacing_inverse` still
//!   isolates SG **`J\propto 1/h`** at \(\rho=0\).

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "electrochemistry-mvp")]
use burn::tensor::{Bool, Data, Shape};

#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::PhysicsError;
#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::dec_primal::primal_divergence_from_edge_flux_topo;
#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::dec_primal::primal_scalar_edge_increment;
#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "electrochemistry-mvp")]
use crate::physics::topology::EdgeTopology;

/// Drift–diffusion scaling: `faraday_const` is \(F\) (C/mol); `gas_const` must be **\(R\,T\)** (J/(mol·K)×K)
/// so that \(F/(RT) =\,\) `faraday_const / gas_const` in the Nernst–Planck drift term.
pub struct ElectroChemicalSolver {
    pub faraday_const: f32,
    pub gas_const: f32,
    /// Picard outer sweeps per call to [`Self::solve_pnp_step`] (≥1). Each sweep: Poisson surrogate
    /// on \(\Phi\) with \(\rho_e(c)\), then one SG NP update advancing \(c\) in place.
    pub coupling_picard_iters: usize,
    /// When **> 0**, stop Picard early once an outer sweep changes \((\Phi,c)\) by less than this
    /// **L∞** threshold (element-wise max of \(|\Delta\Phi|\) and \(|\Delta c|\)). `0` disables this
    /// criterion. Early exit triggers if **any** enabled Picard tolerance (this field,
    /// [`Self::coupling_picard_tol_delta_phi_linf`], [`Self::coupling_picard_tol_delta_phi_l2`]) is met.
    pub coupling_picard_tol_linf: f32,
    /// When **> 0**, stop Picard early when **`max |\Delta\Phi|`** alone (successive outer iterates)
    /// falls below this threshold. `0` disables.
    pub coupling_picard_tol_delta_phi_linf: f32,
    /// When **> 0**, stop Picard early when **\(\|\Delta\Phi\|_2\)** (Euclidean norm of \(\Phi^{(k)}-\Phi^{(k-1)}\)
    /// over all nodes) falls below this threshold. `0` disables.
    pub coupling_picard_tol_delta_phi_l2: f32,
    /// Uniform mesh spacing `h` for SG-flux edge scaling: the Scharfetter–Gummel flux carries
    /// dimension `[D]·[c]/[h]`, so the divergence has dimension `[D]·[c]/[h]²` — matching `∇·J`.
    /// Setting `mesh_spacing = 1.0` (default) reproduces the legacy unit-edge convention used by
    /// existing tests; physical-units callers should pass actual edge length. Non-uniform meshes
    /// (variable `h` per edge) are open to the implicit-Newton step (Phase 3.3).
    ///
    /// **Coupled note:** the **Poisson path-chain Thomas** solve uses the same harmonic-\(\varepsilon\)
    /// stencil in **index space**, with interior RHS scaled by **`h²`** and the implicit BE **Poisson
    /// residual / Jacobian** Laplacian scaled by **`1/h²`** (`h` = this field) so \(\nabla\!\cdot(\varepsilon\nabla\phi)\)
    /// matches the SG **`J\propto D/h`** graph. Default **`1.0`** preserves legacy unit-edge tests.
    /// formal_anchor: Literature
    /// formal_citation: Scharfetter & Gummel 1969, IEEE TED 16:64
    /// formal_form: "J_e = (D_e/h) [c_s B(z F Δφ/RT) − c_t B(−z F Δφ/RT)]"
    pub mesh_spacing: f32,
    /// When **`Some`**, [`ElectroChemicalSolver::solve_pnp_step_dispatch`] first attempts Track 14
    /// **implicit backward Euler + damped Newton** on path-chain graphs (same contract as
    /// [`Self::try_solve_pnp_backward_euler_newton_chain`]); on failure (`None` from that helper —
    /// non-chain graph, `batch≠1`, etc.) it falls back to explicit [`Self::solve_pnp_step`].
    /// Default **`None`**: dispatch matches the historical **Picard / split-explicit** behaviour only.
    pub pnp_implicit_newton_chain: Option<NewtonPnpContext>,
}

impl Default for ElectroChemicalSolver {
    fn default() -> Self {
        Self {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            coupling_picard_iters: 1_usize,
            coupling_picard_tol_linf: 0.0_f32,
            coupling_picard_tol_delta_phi_linf: 0.0_f32,
            coupling_picard_tol_delta_phi_l2: 0.0_f32,
            mesh_spacing: 1.0_f32,
            pnp_implicit_newton_chain: None,
        }
    }
}

/// Track 14 — implicit backward Euler Newton on a 1D chain: damping, tolerances, optional Fickian
/// linearisation. **`linearize_sg_fickian: false`:** column finite-difference Jacobian for full SG in
/// **node-major** band storage, then **dense expand + Gaussian elimination** per Newton correction
/// (**\(O((3N)^3)\)**; optional [`NewtonPnpContext::full_sg_frozen_jacobian_inner_iters`] **`>1`** reuses one
/// assembled band across inner corrections without extra column FD, still re-expanding into dense scratch each inner).
/// Alternatively, [`NewtonPnpContext::full_sg_correction_use_gmres`] **`true`** (with **`full_sg_frozen_jacobian_inner_iters` == `1`**)
/// skips band assembly and solves each correction with matrix-free GMRES on \(J_{\mathrm{nm}}\).
/// **`linearize_sg_fickian: true`:** analytic Jacobian structure with **three Thomas solves** per Newton step
/// (\(O(N)\)), no dense \((3N)^2\) solve on the concentration–Poisson block.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NewtonPnpContext {
    /// Maximum damped Newton iterations per call.
    pub max_newton_iters: usize,
    /// Stop when \(\|R\|_2\) falls below this threshold (host `f64`).
    pub residual_tol_l2: f64,
    /// Under-relaxation on the Newton correction: `U ← U + damping · δ`.
    pub damping: f64,
    /// Relative finite-difference increment scale for Jacobian columns (full SG band FD and tests).
    pub fd_step: f64,
    /// Refuse the host solve when `N_nodes` exceeds this cap (safety / CI guard).
    ///
    /// **Perf:** implicit Newton work is **\(O(\texttt{max\_newton\_iters}\cdot N)\)** per call when
    /// **`linearize_sg_fickian`** is **`true`** (three Thomas solves per iteration). When **`false`**, each
    /// outer iteration assembles a **band** Jacobian (**\(O(N^2)\)** column-FD probes when **J** is refreshed);
    /// each Newton correction **expands the band to dense** and eliminates (**\(O((3N)^3)\)**). When
    /// [`NewtonPnpContext::full_sg_frozen_jacobian_inner_iters`] is **`>1`**, inner sweeps **reuse the same
    /// frozen band** between assemblies (fewer FD assemblies, same dense-expand cost per inner correction). Keep **`N`**
    /// within this cap for interactive use.
    pub max_chain_nodes: usize,
    /// Use `B(z\Delta\phi)\equiv 1` in the SG flux (Fickian / **linear in \(c\)**) inside the implicit residual only.
    ///
    /// **Debye \(\lambda_D\) gates:** set **`true`** so the implicit residual matches the **Debye–Hückel**
    /// linear transport limit (same limit as continuum **`λ_D = \sqrt{\varepsilon/(2 z^2 c_0)}\)** in the
    /// reference) and so Newton can use the **analytic** Jacobian (`fill_jacobian_linearized_sg_fickian`
    /// in unit tests; production applies **three Thomas solves** per Newton iter on the same sparse
    /// block structure). **Poisson / LS calibration:** the chain Thomas interior RHS carries **`mesh_spacing²`**
    /// on the unit-index stencil — see **`poisson_chain_uniform_rho_matches_h_squared_rhs_scaling`** in
    /// `tests/verification/pnp_debye_layer.rs`. The long-horizon **`debye_screening_256_cells_*`** gates set
    /// **`mesh_spacing = h = L/(N-1)`** so the LS abscissa **`x_i = i\,h`** matches that spacing (same file:
    /// **`fit_phi_screening_decay_length_ls`**). **`debye_ls_decay_length_miscalibrated_unit_index_abscissa_rescales_by_physical_h`**
    /// shows that fitting the same |\phi| samples with **`h_{\mathrm{fit}} = 1`** biases **`λ_eff`** vs continuum **`λ_D`**
    /// (coordinate mismatch). Shipped bands remain **±11%** / **±15%**; see `pnp_debye_layer` module docs and `docs/Solver-Status.md`.
    pub linearize_sg_fickian: bool,
    /// **Full SG only** (`linearize_sg_fickian: false`): after each band Jacobian **assembly**, allow up to
    /// this many damped Newton **corrections** before the next assembly (still within the same outer Newton
    /// iteration budget [`Self::max_newton_iters`]). Each inner correction **re-expands** the frozen band to
    /// dense and re-eliminates (**\(O((3N)^3)\)**). **`1`** (default): classical Newton (one correction per
    /// assembly). Values **`>1`**: frozen-**J** inner sweep — **no extra column FD** between inners on the
    /// same outer iteration. Clamped to **`32`** in the host kernel.
    pub full_sg_frozen_jacobian_inner_iters: u8,
    /// **Full SG only** (`linearize_sg_fickian: false`): when **`true`**, each Newton correction solves
    /// \(J_{\mathrm{nm}}\,\delta=-R_{\mathrm{nm}}\) with host [`super::krylov_host::gmres_f32_try`] and
    /// matrix-free [`pnp_be_full_sg_jacobian_matvec_nm_f64`] (no band assembly / dense expand). Ignored when
    /// [`Self::full_sg_frozen_jacobian_inner_iters`] **`> 1`** (frozen-**J** inners keep the dense-expand path).
    /// Default **`false`**: band Jacobian + dense Gaussian (legacy).
    pub full_sg_correction_use_gmres: bool,
}

impl Default for NewtonPnpContext {
    fn default() -> Self {
        Self {
            max_newton_iters: 32,
            residual_tol_l2: 1e-10,
            damping: 1.0,
            fd_step: 1e-6,
            max_chain_nodes: 128,
            linearize_sg_fickian: false,
            full_sg_frozen_jacobian_inner_iters: 1,
            full_sg_correction_use_gmres: false,
        }
    }
}

impl ElectroChemicalSolver {
    /// One explicit coupled PNP sub-step (`dt` is the explicit time increment). Path-chain Poisson
    /// uses a **Thomas** solve when `edges_b1` matches the **contiguous** path chain layout; otherwise Jacobi
    /// substeps. [`ElectroChemicalSolver::coupling_picard_iters`] \(>1\) runs **Picard** outer sweeps
    /// with optional L∞ / \(\max|\Delta\Phi|\) / \(\|\Delta\Phi\|_2\) early stop (see `solve_pnp_step_experimental`)
    /// — still not a monolithic implicit Newton step (see module **Gaps**).
    ///
    /// # Shapes (contract, `[Batch, N, …]`)
    /// - `electric_potential`: `[B, N, 1]`
    /// - `ion_concentration`: `[B, N, 2]` (e.g. two species channels)
    /// - `permittivity`: `[B, N, 1]`
    /// - `diffusivity`: `[B, N, 2]`
    /// - `edges_b1`: `[2, E]`
    ///
    /// ## Default builds (`electrochemistry-mvp` **off**)
    /// Returns `(electric_potential, ion_concentration)` unchanged.
    ///
    /// ## `--features electrochemistry-mvp` / `solver-experimental`
    /// - **Poisson:** on a **path chain** (`0\!-\!1\!-\!\cdots\!-\!(N-1)\)` with `N-1` edges),
    ///   one **Thomas** solve of \(\mathcal{L}\Phi=-\rho_e/\varepsilon\) with endpoint values taken from
    ///   `electric_potential`; otherwise Jacobi-like relaxation substeps (see module **Gaps**).
    /// - **NP (Scharfetter–Gummel):** conservative edge flux with Bernoulli stabilisation; explicit Euler
    ///   \(c \leftarrow c - \Delta t\,\nabla\!\cdot J\) (sign matches the graph Laplacian for \(\Phi=0\)).
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

    /// Single entry point that respects [`Self::pnp_implicit_newton_chain`]: when it is **`Some`**
    /// and `--features electrochemistry-mvp` (alias `electrochemistry-pnp`) is enabled, tries
    /// [`Self::try_solve_pnp_backward_euler_newton_chain`] first; otherwise returns the same result
    /// as [`Self::solve_pnp_step`] (explicit split + Picard via `solve_pnp_step_experimental`).
    ///
    /// Without the **`electrochemistry-mvp`** feature flag enabled, tensors are returned unchanged (same as [`Self::solve_pnp_step`]).
    #[allow(unused_variables)]
    pub fn solve_pnp_step_dispatch<B: Backend<FloatElem = f32>>(
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
            if let Some(newton) = self.pnp_implicit_newton_chain {
                if let Some(out) = self.try_solve_pnp_backward_euler_newton_chain(
                    &newton,
                    dt,
                    electric_potential.clone(),
                    ion_concentration.clone(),
                    edges_b1.clone(),
                    permittivity.clone(),
                    diffusivity.clone(),
                ) {
                    return out;
                }
            }
            self.solve_pnp_step(
                dt,
                electric_potential,
                ion_concentration,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }

    /// Fully implicit **backward Euler** step on \((\Phi,c^\pm)\) via **damped Newton** with a host
    /// `f64` Jacobian: **node-major band column finite differences**, then **dense expand + elimination**
    /// when `linearize_sg_fickian` is **false** (including frozen-**J** inner sweeps when
    /// [`NewtonPnpContext::full_sg_frozen_jacobian_inner_iters`] **`>1`** — same **\(O(dim^3)\)** work per inner, fewer FD assemblies).
    /// When **`linearize_sg_fickian` is `true`** (Fickian-linearised SG / Debye–Hückel residual), each
    /// Newton correction uses **three Thomas solves** on the **sparse block structure** (no dense expand).
    /// **Only** when `edges_b1` is the **contiguous** path
    /// `0\!-\!1\!-\!\cdots\!-\!(N-1)\` and `batch=1`; otherwise returns `None` (caller keeps split
    /// [`Self::solve_pnp_step`]).
    ///
    /// Dirichlet \(\Phi\) at the two endpoints is read from `electric_potential_n` at nodes `0` and
    /// `N-1` and enforced as algebraic rows in the residual. The discrete operators match
    /// `poisson_path_dirichlet_thomas` (interior Laplacian) and the SG edge assembly in
    /// `solve_pnp_split_step_experimental_with_refs`, with backward Euler \(R_c=(c-c^n)/\Delta t+\nabla\!\cdot J\).
    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn try_solve_pnp_backward_euler_newton_chain<B: Backend<FloatElem = f32>>(
        &self,
        newton: &NewtonPnpContext,
        dt: f32,
        electric_potential_n: Tensor<B, 3>,
        ion_concentration_n: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        permittivity: Tensor<B, 3>,
        diffusivity: Tensor<B, 3>,
    ) -> Option<(Tensor<B, 3>, Tensor<B, 3>)> {
        #[cfg(not(feature = "electrochemistry-mvp"))]
        {
            None
        }
        #[cfg(feature = "electrochemistry-mvp")]
        {
            try_solve_pnp_be_newton_chain_host(
                self,
                newton,
                dt,
                electric_potential_n,
                ion_concentration_n,
                edges_b1,
                permittivity,
                diffusivity,
            )
        }
    }
}

#[cfg(feature = "electrochemistry-mvp")]
const POISSON_GRAPH_PCG_REL_TOL: f32 = 5e-5_f32;
#[cfg(feature = "electrochemistry-mvp")]
const POISSON_GRAPH_PCG_MAX_IT: usize = 4096;
#[cfg(feature = "electrochemistry-mvp")]
/// Host read for relative-residual early exit inside [`poisson_graph_uniform_laplacian_jacobi_pcg`]
/// (not after every inner matvec / Saxpy).
const POISSON_GRAPH_PCG_INNER_HOST_CHECK_EVERY: usize = 10;

/// Single-element rank-1 float tensor → host `f32` via [`Tensor::into_data`] (avoids the Burn one-element float extraction helper).
#[cfg(feature = "electrochemistry-mvp")]
#[inline]
fn tensor1_f32<B: Backend<FloatElem = f32>>(t: Tensor<B, 1>) -> f32 {
    t.into_data().value[0]
}

/// Single-element rank-1 bool tensor → host `bool` via [`Tensor::into_data`].
#[cfg(feature = "electrochemistry-mvp")]
#[inline]
fn tensor1_bool<B: Backend<FloatElem = f32>>(t: Tensor<B, 1, Bool>) -> bool {
    t.into_data().value[0]
}

/// Jacobi–PCG on the graph Laplacian for non-chain Poisson surrogates.
///
/// Inner iterations keep \(lpha\), \(eta\), and Saxpy scaling on the Burn graph by reshaping
/// rank‑1 reductions to \([1,1,1]\) for broadcasting (same pattern as masked bar CG in
/// [`crate::physics::mechanics`]). Relative \(\|r\|_2\) early exit uses [`Tensor::lower_elem`] +
/// [`Tensor::all`] with a host boolean read at most every [`POISSON_GRAPH_PCG_INNER_HOST_CHECK_EVERY`]
/// iterations (and on the last inner iteration), avoiding [`tensor1_f32`] on every matvec step.
#[cfg(feature = "electrochemistry-mvp")]
pub(crate) fn poisson_graph_uniform_laplacian_jacobi_pcg<B: Backend<FloatElem = f32>>(
    phi_initial: Tensor<B, 3>,
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
) -> Tensor<B, 3> {
    let rhs_abs_max = tensor1_f32(rhs.clone().abs().max());
    if !rhs_abs_max.is_finite() {
        return phi_initial;
    }
    let rhs_norm = tensor1_f32(rhs.clone().powf_scalar(2.0).sum().sqrt()).max(1e-30_f32);
    if !rhs_norm.is_finite() || rhs_norm < 1e-24_f32 {
        return phi_initial;
    }
    let rhs_tol = rhs_norm * POISSON_GRAPH_PCG_REL_TOL;

    let diag_a =
        TopologicalLaplacian::scalar_laplacian_neg_opposite_diag(edges_b1.clone(), damage.clone());
    let diag_inv = diag_a.clamp_min(1e-14_f32).recip();

    let mut phi = phi_initial;
    let lap0 =
        TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
    let mut r = lap0.sub(rhs.clone());

    let mut z = r.clone().mul(diag_inv.clone());
    let mut p = z.clone();
    let mut rz_old_t = r
        .clone()
        .mul(z.clone())
        .sum()
        .reshape([1, 1, 1])
        .clamp_min(1e-40_f32);

    let max_it = n.saturating_mul(10).clamp(256, POISSON_GRAPH_PCG_MAX_IT);

    for it in 0..max_it {
        let lp =
            TopologicalLaplacian::scalar_laplacian(p.clone(), edges_b1.clone(), damage.clone());
        let ap = lp.neg();
        let p_ap_t = p
            .clone()
            .mul(ap.clone())
            .sum()
            .reshape([1, 1, 1])
            .clamp_min(1e-40_f32);
        let alpha_t = rz_old_t.clone().div(p_ap_t).clamp(-1e4_f32, 1e4_f32);

        phi = phi.add(p.clone().mul(alpha_t.clone()));
        r = r.sub(ap.mul(alpha_t));

        let should_sync = (it % POISSON_GRAPH_PCG_INNER_HOST_CHECK_EVERY
            == POISSON_GRAPH_PCG_INNER_HOST_CHECK_EVERY - 1)
            || it + 1 == max_it;
        if should_sync {
            let r_l2 = r.clone().powf_scalar(2.0).sum().sqrt();
            if tensor1_bool(r_l2.lower_elem(rhs_tol).all()) {
                break;
            }
        }

        z = r.clone().mul(diag_inv.clone());
        let rz_new_t = r
            .clone()
            .mul(z.clone())
            .sum()
            .reshape([1, 1, 1])
            .clamp_min(1e-40_f32);
        let beta_t = rz_new_t
            .clone()
            .div(rz_old_t.clone().clamp_min(1e-40_f32))
            .clamp(0.0_f32, 1e6_f32);
        p = z.clone().add(p.mul(beta_t));
        rz_old_t = rz_new_t;
    }

    let phi_mean = phi
        .clone()
        .sum_dim(1)
        .div_scalar(n as f32)
        .reshape([batch, 1, 1]);
    phi.sub(phi_mean)
}

/// Bernoulli function \(B(x)=x/(e^x-1)\) for Scharfetter–Gummel fluxes (symmetrised for \(x<0\) via \(B(x)=B(|x|)+\min(0,x)\)).
/// Uses `f64` exponentials so large \(|z F\Delta\phi/(RT)|\) does not saturate `f32::exp` before the ratio stabilises.
#[cfg(feature = "electrochemistry-mvp")]
fn bernoulli_b_elem_f32(x: f32) -> f32 {
    let xd = x as f64;
    let u = xd.abs().max(0.0);
    let b_pos = if u < 1e-4_f64 {
        1.0_f64 - u * 0.5_f64 + u * u / 12.0_f64 - u.powi(4) / 720.0_f64
    } else {
        let exp_u = u.exp();
        let denom = (exp_u - 1.0_f64).max(1e-30_f64);
        u / denom
    };
    (b_pos + xd.min(0.0_f64)) as f32
}

#[cfg(feature = "electrochemistry-mvp")]
fn bernoulli_b<B: Backend<FloatElem = f32>>(x: Tensor<B, 3>) -> Tensor<B, 3> {
    let device = x.device();
    let dims = x.dims();
    let v = x.into_data().value;
    let out: Vec<f32> = v.into_iter().map(bernoulli_b_elem_f32).collect();
    Tensor::from_data(Data::new(out, Shape::new(dims)), &device)
}

/// Flattened `edges_b1` host layout `[src_0…src_{E-1}, tgt_0…tgt_{E-1}]` for shape `[2, E]`.
#[cfg(feature = "electrochemistry-mvp")]
fn is_contiguous_unit_path(n: usize, layout: &[i64]) -> bool {
    let e = match n.checked_sub(1) {
        Some(x) => x,
        None => return false,
    };
    if layout.len() != 2 * e {
        return false;
    }
    if n < 2 {
        return e == 0 && layout.is_empty();
    }
    let src = &layout[0..e];
    let tgt = &layout[e..2 * e];
    let mut deg = vec![0u8; n];
    for i in 0..e {
        let s = src[i] as usize;
        let t = tgt[i] as usize;
        if s >= n || t >= n || s == t {
            return false;
        }
        deg[s] = deg[s].saturating_add(1);
        if deg[s] > 2 {
            return false;
        }
        deg[t] = deg[t].saturating_add(1);
        if deg[t] > 2 {
            return false;
        }
    }
    deg.iter().filter(|&&d| d == 1).count() == 2 && deg.iter().all(|&d| d > 0)
}

/// Thomas tridiagonal solve: row `i` has `a[i]*u[i-1] + b[i]*u[i] + c[i]*u[i+1] = r[i]` with `a[0]=0`, `c[m-1]=0`.
#[cfg(feature = "electrochemistry-mvp")]
fn thomas_tridiagonal_solve(a: &[f32], b: &mut [f32], c: &[f32], r: &mut [f32], u: &mut [f32]) {
    let m = b.len();
    if m == 0 {
        return;
    }
    debug_assert_eq!(a.len(), m);
    debug_assert_eq!(c.len(), m);
    debug_assert_eq!(r.len(), m);
    debug_assert_eq!(u.len(), m);
    for i in 1..m {
        let w = a[i] / b[i - 1];
        b[i] -= w * c[i - 1];
        r[i] -= w * r[i - 1];
    }
    u[m - 1] = r[m - 1] / b[m - 1];
    for i in (0..m - 1).rev() {
        u[i] = (r[i] - c[i] * u[i + 1]) / b[i];
    }
}

/// Discrete Poisson \(\mathcal{L}\phi = -\rho\) on a path with \(\phi_0=g_0\), \(\phi_{n-1}=g_1\) (same \(\mathcal{L}\) as [`crate::physics::laplacian::TopologicalLaplacian`] on the chain, damage `1`).
#[cfg(feature = "electrochemistry-mvp")]
#[allow(dead_code)] // Superseded by [`poisson_chain_net_charge_variable_eps_thomas`] for uniform-\(\varepsilon\) parity checks.
fn poisson_path_dirichlet_thomas(n: usize, g0: f32, g1: f32, rho: &[f32], out: &mut [f32]) {
    debug_assert_eq!(rho.len(), n);
    debug_assert_eq!(out.len(), n);
    out[0] = g0;
    if n <= 1 {
        return;
    }
    out[n - 1] = g1;
    if n == 2 {
        return;
    }
    let m = n - 2;
    let mut a = vec![0.0_f32; m];
    let mut b = vec![-2.0_f32; m];
    let mut c = vec![1.0_f32; m];
    let mut r = vec![0.0_f32; m];
    c[m - 1] = 0.0_f32;
    for a_i in a.iter_mut().take(m).skip(1) {
        *a_i = 1.0_f32;
    }
    r[0] = -rho[1] - g0;
    for k in 1..m - 1 {
        r[k] = -rho[k + 1];
    }
    if m == 1 {
        r[0] = -rho[1] - g0 - g1;
    } else {
        r[m - 1] = -rho[n - 2] - g1;
    }
    let mut u = vec![0.0_f32; m];
    thomas_tridiagonal_solve(&a, &mut b, &c, &mut r, &mut u);
    out[1..(m + 1)].copy_from_slice(&u[..m]);
}

/// Same discrete operator as [`poisson_chain_net_charge_variable_eps_thomas_f64`] in `f32`.
#[cfg(feature = "electrochemistry-mvp")]
fn poisson_chain_net_charge_variable_eps_thomas(
    n: usize,
    g0: f32,
    g1: f32,
    eps: &[f32],
    rho_net: &[f32],
    interior_rhs_h_sq: f32,
    out: &mut [f32],
) {
    debug_assert_eq!(eps.len(), n);
    debug_assert_eq!(rho_net.len(), n);
    debug_assert_eq!(out.len(), n);
    out[0] = g0;
    if n <= 1 {
        return;
    }
    out[n - 1] = g1;
    if n == 2 {
        return;
    }
    let mut eps_half = vec![0.0_f32; n - 1];
    for i in 0..n - 1 {
        eps_half[i] = 0.5_f32 * (eps[i] + eps[i + 1]);
    }
    let m = n - 2;
    let mut a = vec![0.0_f32; m];
    let mut b = vec![0.0_f32; m];
    let mut c = vec![0.0_f32; m];
    let mut rhs = vec![0.0_f32; m];
    if m == 1 {
        b[0] = -(eps_half[0] + eps_half[1]);
        rhs[0] = -rho_net[1] - eps_half[0] * g0 - eps_half[1] * g1;
    } else {
        b[0] = -(eps_half[0] + eps_half[1]);
        c[0] = eps_half[1];
        rhs[0] = -rho_net[1] - eps_half[0] * g0;
        for k in 1..m - 1 {
            a[k] = eps_half[k];
            b[k] = -(eps_half[k] + eps_half[k + 1]);
            c[k] = eps_half[k + 1];
            rhs[k] = -rho_net[k + 1];
        }
        a[m - 1] = eps_half[m - 2];
        b[m - 1] = -(eps_half[m - 2] + eps_half[m - 1]);
        rhs[m - 1] = -rho_net[n - 2] - eps_half[m - 1] * g1;
    }
    let scale = interior_rhs_h_sq.max(0.0_f32);
    for r in rhs.iter_mut() {
        *r *= scale;
    }
    c[m - 1] = 0.0_f32;
    let mut u = vec![0.0_f32; m];
    thomas_tridiagonal_solve(&a, &mut b, &c, &mut rhs, &mut u);
    out[1..(m + 1)].copy_from_slice(&u[..m]);
}

/// If `edges_b1` is a contiguous path on `0..n-1`, returns Poisson solution tensor; otherwise `None`.
#[cfg(feature = "electrochemistry-mvp")]
fn try_solve_poisson_chain_thomas<B: Backend<FloatElem = f32>>(
    electric_potential: Tensor<B, 3>,
    rho_over_eps: Tensor<B, 3>,
    permittivity: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    mesh_spacing: f32,
) -> Option<Tensor<B, 3>> {
    let device = electric_potential.device();
    let pd = electric_potential.dims();
    let batch = pd[0];
    let n = pd[1];
    let fc = pd[2];
    if fc != 1 {
        return None;
    }
    let ed = edges_b1.dims();
    if ed[0] != 2 {
        return None;
    }
    let e_ct = ed[1];
    if n < 2 || e_ct != n - 1 {
        return None;
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return None;
    }
    let phi_h = electric_potential.into_data().value;
    let rho_h = rho_over_eps.into_data().value;
    let eps_h = permittivity.into_data().value;
    let mut out = vec![0.0_f32; batch * n * fc];
    let stride = n * fc;
    for b in 0..batch {
        let off = b * stride;
        let g0 = phi_h[off];
        let g1 = phi_h[off + (n - 1) * fc];
        let mut rho_net = vec![0.0_f32; n];
        for i in 0..n {
            rho_net[i] = rho_h[off + i] * eps_h[off + i].max(1e-30_f32);
        }
        let h_sq = mesh_spacing.max(1e-30_f32).powi(2);
        poisson_chain_net_charge_variable_eps_thomas(
            n,
            g0,
            g1,
            &eps_h[off..off + stride],
            &rho_net,
            h_sq,
            &mut out[off..off + stride],
        );
    }
    Some(Tensor::from_data(
        Data::new(out, Shape::new([batch, n, fc])),
        &device,
    ))
}

/// One Picard outer sweep for the explicit split PNP step: Poisson with \(\rho_e(c^{(k)})\), then SG
/// NP with fixed step anchor \(c^n\) in the mass update (see [`solve_pnp_split_step_experimental_with_refs`]).
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn one_picard_pnp_pass<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration_n: Tensor<B, 3>,
    ion_concentration_trial: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    solve_pnp_split_step_experimental_with_refs(
        solver,
        dt,
        electric_potential,
        ion_concentration_n,
        ion_concentration_trial,
        edges_b1,
        permittivity,
        diffusivity,
    )
}

/// Experimental explicit split PNP step: **Picard** outer repeats of [`one_picard_pnp_pass`] (Poisson
/// with \(\rho_e(c^{(k)})\), then SG NP anchored on \(c^n\)), bounded by [`ElectroChemicalSolver::coupling_picard_iters`].
/// Does **not** call [`NewtonPnpContext`] or implicit backward Euler; those live in
/// [`ElectroChemicalSolver::try_solve_pnp_backward_euler_newton_chain`] / [`ElectroChemicalSolver::solve_pnp_step_dispatch`].
/// Early exit when **any** enabled tolerance is satisfied: \((\Phi,c)\) L∞
/// ([`ElectroChemicalSolver::coupling_picard_tol_linf`]), **`max|\Delta\Phi|`**
/// ([`ElectroChemicalSolver::coupling_picard_tol_delta_phi_linf`]), or **\(\|\Delta\Phi\|_2\)**
/// ([`ElectroChemicalSolver::coupling_picard_tol_delta_phi_l2`]). L∞ checks use [`picard_delta_linf_tol_met`].
///
/// **Driver:** explicit `for` (bounded by [`ElectroChemicalSolver::coupling_picard_iters`]), not
/// `repeat_controlled` from `fixed_point`: each pass consumes owned [`Tensor`]s, and a stateful `FnMut`
/// closure that alternates [`Option::take`] with [`one_picard_pnp_pass`] runs into **E0507** (use of moved
/// value) when the tensors are also direct captures.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn solve_pnp_step_experimental<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let iters = solver.coupling_picard_iters.max(1);
    let tol = solver.coupling_picard_tol_linf;
    let tol_dphi_linf = solver.coupling_picard_tol_delta_phi_linf;
    let tol_dphi_l2 = solver.coupling_picard_tol_delta_phi_l2;
    let any_tol = tol > 0.0_f32 || tol_dphi_linf > 0.0_f32 || tol_dphi_l2 > 0.0_f32;
    let mut phi = electric_potential;
    let c_n = ion_concentration.clone();
    let mut c_work = ion_concentration;
    for k in 0..iters {
        let track = any_tol && k + 1 < iters;
        let phi_prev = if track { Some(phi.clone()) } else { None };
        let c_prev = if track { Some(c_work.clone()) } else { None };
        let (p, c) = one_picard_pnp_pass(
            solver,
            dt,
            phi,
            c_n.clone(),
            c_work,
            edges_b1.clone(),
            permittivity.clone(),
            diffusivity.clone(),
        );
        phi = p;
        c_work = c;
        if let (Some(pp), Some(cp)) = (phi_prev, c_prev) {
            let diff_phi = phi.clone().sub(pp);
            let diff_c = c_work.clone().sub(cp);
            let ok_linf = tol > 0.0_f32
                && picard_delta_linf_tol_met(diff_phi.clone(), tol)
                && picard_delta_linf_tol_met(diff_c, tol);
            let ok_phi_linf = tol_dphi_linf > 0.0_f32
                && picard_delta_linf_tol_met(diff_phi.clone(), tol_dphi_linf);
            let ok_phi_l2 = tol_dphi_l2 > 0.0_f32
                && tensor1_bool(
                    diff_phi
                        .powf_scalar(2.0)
                        .sum()
                        .sqrt()
                        .lower_elem(tol_dphi_l2)
                        .all(),
                );
            if ok_linf || ok_phi_linf || ok_phi_l2 {
                break;
            }
        }
    }
    (phi, c_work)
}

/// Picard L∞ gate: **`max |\Delta| < \texttt{tol}`** iff every component satisfies **`|\Delta_i| < \texttt{tol}`**
/// (strict inequality, same as the legacy global max reduction).
///
/// Implemented as [`Tensor::lower_elem`] + [`Tensor::all`] so the reduction stays on the Burn graph until
/// the single host boolean read via [`tensor1_bool`].
#[cfg(feature = "electrochemistry-mvp")]
fn picard_delta_linf_tol_met<B: Backend<FloatElem = f32>>(delta: Tensor<B, 3>, tol: f32) -> bool {
    if tol <= 0.0_f32 {
        return false;
    }
    tensor1_bool(delta.abs().lower_elem(tol).all())
}

/// One split PNP sub-step: Poisson uses \(\rho_e(c_{\mathrm{trial}})\); mass update uses fixed \(c^n\):
/// \(c^{n+1}=c^n-\Delta t\,\nabla\!\cdot J(\Phi,c_{\mathrm{trial}})\).
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn solve_pnp_split_step_experimental_with_refs<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    dt: f32,
    electric_potential: Tensor<B, 3>,
    ion_concentration_n: Tensor<B, 3>,
    ion_concentration_trial: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let mask_phi = Tensor::<B, 3>::ones_like(&electric_potential);
    let rt_safe = solver.gas_const.max(1e-30_f32);
    let f_over_rt = solver.faraday_const / rt_safe;

    // Gauss / Poisson: net charge from monovalent cation (ch 0) and anion (ch 1), ρ_e = F (c+ − c−).
    let c_plus = ion_concentration_trial.clone().narrow(2, 0, 1);
    let c_minus = ion_concentration_trial.clone().narrow(2, 1, 1);
    let rho_e = c_plus.sub(c_minus).mul_scalar(solver.faraday_const);

    let eps_safe = permittivity.clone().clamp_min(1e-30_f32);
    let rho_over_eps = rho_e.div(eps_safe);

    let phi_next = if let Some(phi_t) = try_solve_poisson_chain_thomas(
        electric_potential.clone(),
        rho_over_eps.clone(),
        permittivity.clone(),
        edges_b1.clone(),
        solver.mesh_spacing,
    ) {
        phi_t
    } else {
        let h_sq = solver.mesh_spacing.max(1e-30_f32).powi(2);
        let rhs_lap = rho_over_eps.neg().mul_scalar(h_sq);
        let batch = electric_potential.dims()[0];
        let n = electric_potential.dims()[1];
        poisson_graph_uniform_laplacian_jacobi_pcg(
            electric_potential.clone(),
            rhs_lap,
            edges_b1.clone(),
            mask_phi.clone(),
            batch,
            n,
        )
    };

    // Scharfetter–Gummel drift–diffusion flux on each edge, per species channel.
    let batch = ion_concentration_trial.dims()[0];
    let n_e = edges_b1.dims()[1];
    let device = ion_concentration_trial.device();
    let topo = EdgeTopology::new(edges_b1.clone());
    let grad_phi_e = primal_scalar_edge_increment(phi_next.clone(), &topo);
    // Track F refinement (v0.4): uniform-mesh edge length factor `h_inv = 1/h` so the SG flux
    // `J_e = (D_e/h) [...]` has correct physical units (concentration / time).
    let h_safe = solver.mesh_spacing.max(1e-30_f32);
    let h_inv = Tensor::<B, 3>::full([batch, n_e, 1], 1.0_f32 / h_safe, &device);

    let mut flux_channels: Vec<Tensor<B, 3>> = Vec::with_capacity(2);
    for ch in 0..2usize {
        let z = if ch == 0 { 1.0_f32 } else { -1.0_f32 };
        let c_ch = ion_concentration_trial.clone().narrow(2, ch, 1);
        let d_ch = diffusivity.clone().narrow(2, ch, 1);
        let (c_src, c_tgt) = topo.gather_endpoints(c_ch);
        let (d_src, d_tgt) = topo.gather_endpoints(d_ch);
        let d_e = d_src.add(d_tgt).mul_scalar(0.5_f32);
        let c_src = c_src.clamp_min(1e-30_f32);
        let c_tgt = c_tgt.clamp_min(1e-30_f32);
        let pe = grad_phi_e.clone().mul_scalar(z * f_over_rt);
        let b_pe = bernoulli_b(pe.clone());
        let b_mpe = bernoulli_b(pe.neg());
        let j_e = d_e
            .mul(h_inv.clone())
            .mul(c_src.mul(b_pe).sub(c_tgt.mul(b_mpe)));
        flux_channels.push(j_e);
    }
    let flux_stack = Tensor::cat(flux_channels, 2);
    let div_j = primal_divergence_from_edge_flux_topo(flux_stack, &topo, &ion_concentration_trial);
    // ∂c/∂t + ∇·J = 0 with this primal divergence convention ⇒ explicit Euler c ← c^n − Δt ∇·J (φ = 0 recovers Fickian graph Laplacian sign).
    let c_next = ion_concentration_n.sub(div_j.mul_scalar(dt));

    (phi_next, c_next)
}

// --- Track 14: backward Euler + damped Newton (host `f64`, contiguous path chain only) ---

#[cfg(feature = "electrochemistry-mvp")]
fn bernoulli_b_f64(x: f64) -> f64 {
    // B(x)=x/(e^x-1) with symmetrisation B(x)=B(|x|)+min(0,x) (Scharfetter–Gummel).
    // Use f64 throughout; for large positive u, u/(e^u-1) → 0 — avoid exp overflow.
    let u = x.abs().max(0.0);
    let b_pos = if u < 1e-5_f64 {
        1.0_f64 - u * 0.5_f64 + u * u / 12.0_f64 - u * u * u * u / 720.0_f64
    } else if u > 60.0_f64 {
        // u/(e^u-1) ≈ u·e^{-u}/(1-e^{-u}) — stable for large u
        let eu = (-u).exp();
        (u * eu / (1.0_f64 - eu).max(1e-300_f64)).min(1.0_f64)
    } else {
        let exp_u = u.exp();
        let denom = (exp_u - 1.0_f64).max(1e-300_f64);
        u / denom
    };
    b_pos + x.min(0.0_f64)
}

#[cfg(feature = "electrochemistry-mvp")]
fn thomas_tridiagonal_solve_f64(a: &[f64], b: &mut [f64], c: &[f64], r: &mut [f64], u: &mut [f64]) {
    let m = b.len();
    if m == 0 {
        return;
    }
    for i in 1..m {
        let w = a[i] / b[i - 1];
        b[i] -= w * c[i - 1];
        r[i] -= w * r[i - 1];
    }
    u[m - 1] = r[m - 1] / b[m - 1];
    for i in (0..m - 1).rev() {
        u[i] = (r[i] - c[i] * u[i + 1]) / b[i];
    }
}

#[cfg(feature = "electrochemistry-mvp")]
#[allow(dead_code)] // Legacy \(\nabla^2\phi=-\rho\); chain Poisson uses [`poisson_chain_net_charge_variable_eps_thomas_f64`].
fn poisson_path_dirichlet_thomas_f64(n: usize, g0: f64, g1: f64, rho: &[f64], out: &mut [f64]) {
    debug_assert_eq!(rho.len(), n);
    debug_assert_eq!(out.len(), n);
    out[0] = g0;
    if n <= 1 {
        return;
    }
    out[n - 1] = g1;
    if n == 2 {
        return;
    }
    let m = n - 2;
    let mut a = vec![0.0_f64; m];
    let mut b = vec![-2.0_f64; m];
    let mut c = vec![1.0_f64; m];
    let mut r = vec![0.0_f64; m];
    c[m - 1] = 0.0_f64;
    for a_i in a.iter_mut().take(m).skip(1) {
        *a_i = 1.0_f64;
    }
    r[0] = -rho[1] - g0;
    for k in 1..m - 1 {
        r[k] = -rho[k + 1];
    }
    if m == 1 {
        r[0] = -rho[1] - g0 - g1;
    } else {
        r[m - 1] = -rho[n - 2] - g1;
    }
    let mut u = vec![0.0_f64; m];
    thomas_tridiagonal_solve_f64(&a, &mut b, &c, &mut r, &mut u);
    out[1..(m + 1)].copy_from_slice(&u[..m]);
}

/// Dirichlet Poisson on a unit-spaced chain with **spatially varying** nodal \(\varepsilon\):
/// \(\nabla\cdot(\varepsilon\nabla\phi)= -\rho_{\mathrm{net}}\) with \(\rho_{\mathrm{net}} = F(c^+-c^-)\).
/// Edge halves \(\varepsilon_{i+1/2}=\tfrac12(\varepsilon_i+\varepsilon_{i+1})\). When \(\varepsilon\) is
/// uniform, this matches the legacy stencil \(\nabla^2\phi=-\rho/\varepsilon\) with \(\rho_{\mathrm{net}}=\rho_e\).
#[cfg(feature = "electrochemistry-mvp")]
fn poisson_chain_net_charge_variable_eps_thomas_f64(
    n: usize,
    g0: f64,
    g1: f64,
    eps: &[f64],
    rho_net: &[f64],
    interior_rhs_h_sq: f64,
    out: &mut [f64],
) {
    debug_assert_eq!(eps.len(), n);
    debug_assert_eq!(rho_net.len(), n);
    debug_assert_eq!(out.len(), n);
    out[0] = g0;
    if n <= 1 {
        return;
    }
    out[n - 1] = g1;
    if n == 2 {
        return;
    }
    let mut eps_half = vec![0.0_f64; n - 1];
    for i in 0..n - 1 {
        eps_half[i] = 0.5_f64 * (eps[i] + eps[i + 1]);
    }
    let m = n - 2;
    let mut a = vec![0.0_f64; m];
    let mut b = vec![0.0_f64; m];
    let mut c = vec![0.0_f64; m];
    let mut rhs = vec![0.0_f64; m];
    if m == 1 {
        b[0] = -(eps_half[0] + eps_half[1]);
        rhs[0] = -rho_net[1] - eps_half[0] * g0 - eps_half[1] * g1;
    } else {
        b[0] = -(eps_half[0] + eps_half[1]);
        c[0] = eps_half[1];
        rhs[0] = -rho_net[1] - eps_half[0] * g0;
        for k in 1..m - 1 {
            a[k] = eps_half[k];
            b[k] = -(eps_half[k] + eps_half[k + 1]);
            c[k] = eps_half[k + 1];
            rhs[k] = -rho_net[k + 1];
        }
        a[m - 1] = eps_half[m - 2];
        b[m - 1] = -(eps_half[m - 2] + eps_half[m - 1]);
        rhs[m - 1] = -rho_net[n - 2] - eps_half[m - 1] * g1;
    }
    let scale = interior_rhs_h_sq.max(0.0_f64);
    for r in rhs.iter_mut() {
        *r *= scale;
    }
    c[m - 1] = 0.0_f64;
    let mut u = vec![0.0_f64; m];
    thomas_tridiagonal_solve_f64(&a, &mut b, &c, &mut rhs, &mut u);
    out[1..(m + 1)].copy_from_slice(&u[..m]);
}

/// Scatter-style divergence on a path: edge `e` connects `e → e+1`, `div[src]+=J`, `div[tgt]-=J`.
#[cfg(feature = "electrochemistry-mvp")]
fn chain_divergence_from_edge_flux_f64(j_edge: &[f64], n: usize) -> Vec<f64> {
    let mut div = vec![0.0_f64; n];
    for e in 0..n - 1 {
        let j = j_edge[e];
        div[e] += j;
        div[e + 1] -= j;
    }
    div
}

#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn pnp_be_residual_vector_f64(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f64,
    phi: &[f64],
    c_plus: &[f64],
    c_minus: &[f64],
    c_plus_n: &[f64],
    c_minus_n: &[f64],
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    g0: f64,
    g1: f64,
) -> Vec<f64> {
    let n = phi.len();
    let f = solver.faraday_const as f64;
    let rt = solver.gas_const.max(1e-30_f32) as f64;
    let f_over_rt = f / rt;
    let h_inv = 1.0_f64 / solver.mesh_spacing.max(1e-30_f32) as f64;
    let inv_h_sq = h_inv * h_inv;
    let mut eps_half = vec![0.0_f64; n.saturating_sub(1)];
    for i in 0..eps_half.len() {
        eps_half[i] = 0.5_f64 * (eps[i] + eps[i + 1]);
    }
    let mut r = vec![0.0_f64; 3 * n];
    for i in 0..n {
        if i == 0 || i + 1 == n {
            r[i] = phi[i] - if i == 0 { g0 } else { g1 };
        } else {
            let lap_eps =
                eps_half[i] * (phi[i + 1] - phi[i]) - eps_half[i - 1] * (phi[i] - phi[i - 1]);
            let rho_net = f * (c_plus[i] - c_minus[i]);
            r[i] = lap_eps * inv_h_sq + rho_net;
        }
    }
    let mut j_plus_e = vec![0.0_f64; n.saturating_sub(1)];
    let mut j_minus_e = vec![0.0_f64; n.saturating_sub(1)];
    for e in 0..n - 1 {
        let dphi = phi[e + 1] - phi[e];
        let (cps, cpt, cms, cmt) = if newton.linearize_sg_fickian {
            (c_plus[e], c_plus[e + 1], c_minus[e], c_minus[e + 1])
        } else {
            (
                c_plus[e].max(1e-30_f64),
                c_plus[e + 1].max(1e-30_f64),
                c_minus[e].max(1e-30_f64),
                c_minus[e + 1].max(1e-30_f64),
            )
        };
        let dpe = 0.5_f64 * (d_plus[e] + d_plus[e + 1]);
        let dme = 0.5_f64 * (d_minus[e] + d_minus[e + 1]);
        let flux_edge = |z: f64, c_s: f64, c_t: f64, d_e: f64| -> f64 {
            let pe = z * f_over_rt * dphi;
            let (b_pe, b_mpe) = if newton.linearize_sg_fickian {
                (1.0_f64, 1.0_f64)
            } else {
                (bernoulli_b_f64(pe), bernoulli_b_f64(-pe))
            };
            d_e * h_inv * (c_s * b_pe - c_t * b_mpe)
        };
        j_plus_e[e] = flux_edge(1.0_f64, cps, cpt, dpe);
        j_minus_e[e] = flux_edge(-1.0_f64, cms, cmt, dme);
    }
    let div_p = chain_divergence_from_edge_flux_f64(&j_plus_e, n);
    let div_m = chain_divergence_from_edge_flux_f64(&j_minus_e, n);
    for i in 0..n {
        r[n + i] = (c_plus[i] - c_plus_n[i]) / dt + div_p[i];
        r[2 * n + i] = (c_minus[i] - c_minus_n[i]) / dt + div_m[i];
    }
    r
}

#[cfg(feature = "electrochemistry-mvp")]
fn vec_l2(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Dense linear solve: destroys `a` (LU-style forward elimination); solution written into `b`.
#[cfg(feature = "electrochemistry-mvp")]
fn solve_dense_linear(dim: usize, a: &mut [f64], b: &mut [f64]) -> bool {
    for k in 0..dim {
        let mut piv = k;
        let mut best = a[k * dim + k].abs();
        for r in (k + 1)..dim {
            let v = a[r * dim + k].abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        if best < 1e-18_f64 {
            return false;
        }
        if piv != k {
            for c in 0..dim {
                a.swap(k * dim + c, piv * dim + c);
            }
            b.swap(k, piv);
        }
        let akk = a[k * dim + k];
        for r in (k + 1)..dim {
            let f = a[r * dim + k] / akk;
            if f == 0.0 {
                continue;
            }
            for c in k..dim {
                a[r * dim + c] -= f * a[k * dim + c];
            }
            b[r] -= f * b[k];
        }
    }
    for r in (0..dim).rev() {
        let mut sum = b[r];
        for c in (r + 1)..dim {
            sum -= a[r * dim + c] * b[c];
        }
        let arr = a[r * dim + r];
        if arr.abs() < 1e-18_f64 {
            return false;
        }
        b[r] = sum / arr;
    }
    true
}

/// Unknowns \((\phi_i,c^+_i,c^-_i)\) per spatial node ⇒ Jacobian half-bandwidth **5** in node-major order.
#[cfg(feature = "electrochemistry-mvp")]
const PNP_CHAIN_FULL_SG_JAC_KL_PHYS: usize = 5;
#[cfg(feature = "electrochemistry-mvp")]
const PNP_CHAIN_FULL_SG_JAC_KU_PHYS: usize = 5;

/// Wider row envelope for band Jacobian assembly / scratch layout (same row-major layout as physics).
///
/// Set to **`3·17−1`** so that for the **N=17** full-SG FP fixture (**`dim=3·17`**), the static band is a full
/// strip and `row_band_lu_factorize_partial_pivot` + `row_band_lu_solve_factored` match `solve_dense_linear`
/// on the expanded Jacobian (same pivot + fill story as ladder **`fp001_kl_ge_dim_minus_one_full_envelope_matches_dense_gaussian`**).
/// Together with the corrected **forward** application of pivot swaps on the RHS in `row_band_l_forward_swapped_rhs`,
/// this locks CI parity for **`full_sg_newton_band_lu_matches_dense_expand_n17_fixture`**. **Larger** **`dim`**
/// (e.g. ignored **N=256** harness) exceeds this envelope vs dense Gaussian; production inner solves remain
/// `solve_newton_correction_full_sg_row_band_via_dense_expand` (see [`FP_GAP_BACKLOG.md`](../../docs/FP_GAP_BACKLOG.md)).
#[cfg(feature = "electrochemistry-mvp")]
const PNP_CHAIN_FULL_SG_JAC_KL_LU: usize = 3 * 17 - 1;
#[cfg(feature = "electrochemistry-mvp")]
const PNP_CHAIN_FULL_SG_JAC_KU_LU: usize = 3 * 17 - 1;
#[cfg(feature = "electrochemistry-mvp")]
const PNP_CHAIN_FULL_SG_BW_LU: usize =
    PNP_CHAIN_FULL_SG_JAC_KL_LU + PNP_CHAIN_FULL_SG_JAC_KU_LU + 1;

#[cfg(feature = "electrochemistry-mvp")]
#[inline]
fn pnp_nm_index_to_fm(nm: usize, n: usize) -> usize {
    let node = nm / 3;
    match nm % 3 {
        0 => node,
        1 => n + node,
        2 => 2 * n + node,
        rem => {
            debug_assert!(rem < 3, "pnp_nm_index_to_fm: nm % 3 must be 0..2");
            node
        }
    }
}

#[cfg(feature = "electrochemistry-mvp")]
fn pnp_residual_fm_to_nm(r_fm: &[f64], n: usize, r_nm: &mut [f64]) {
    let dim = 3 * n;
    debug_assert_eq!(r_fm.len(), dim);
    debug_assert_eq!(r_nm.len(), dim);
    for node in 0..n {
        r_nm[3 * node] = r_fm[node];
        r_nm[3 * node + 1] = r_fm[n + node];
        r_nm[3 * node + 2] = r_fm[2 * n + node];
    }
}

#[cfg(feature = "electrochemistry-mvp")]
fn pnp_delta_nm_to_fm(dx_nm: &[f64], n: usize, dx_fm: &mut [f64]) {
    let dim = 3 * n;
    debug_assert_eq!(dx_nm.len(), dim);
    debug_assert_eq!(dx_fm.len(), dim);
    for node in 0..n {
        dx_fm[node] = dx_nm[3 * node];
        dx_fm[n + node] = dx_nm[3 * node + 1];
        dx_fm[2 * n + node] = dx_nm[3 * node + 2];
    }
}

/// Matrix-free \(J_{\mathrm{nm}} v\) for the full-SG backward-Euler residual in **node-major** unknown
/// order \((\phi_i,c^+_i,c^-_i)\) per node, via a scaled finite difference on the FM stack used by
/// [`pnp_be_residual_vector_f64`].
///
/// `r0_fm` must equal `pnp_be_residual_vector_f64(...)` at the same `u_fm`. Used by
/// [`full_sg_newton_correction_gmres_nm_f64`] when [`NewtonPnpContext::full_sg_correction_use_gmres`] is set;
/// the default full-SG Newton path still assembles a band Jacobian and uses dense expand + Gaussian elimination.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn pnp_be_full_sg_jacobian_matvec_nm_f64(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f64,
    u_fm: &[f64],
    c_plus_n: &[f64],
    c_minus_n: &[f64],
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    g0: f64,
    g1: f64,
    r0_fm: &[f64],
    v_nm: &[f64],
    scratch_u_fm: &mut [f64],
    diff_fm: &mut [f64],
    out_nm: &mut [f64],
) {
    let n = c_plus_n.len();
    let dim = 3 * n;
    debug_assert_eq!(u_fm.len(), dim);
    debug_assert_eq!(r0_fm.len(), dim);
    debug_assert_eq!(v_nm.len(), dim);
    debug_assert_eq!(scratch_u_fm.len(), dim);
    debug_assert_eq!(diff_fm.len(), dim);
    debug_assert_eq!(out_nm.len(), dim);

    let vnorm = v_nm.iter().map(|x| *x * *x).sum::<f64>().sqrt();
    let eps_fd = if vnorm < 1e-28_f64 {
        newton.fd_step
    } else {
        newton.fd_step / vnorm
    };

    pnp_delta_nm_to_fm(v_nm, n, diff_fm);
    for i in 0..dim {
        scratch_u_fm[i] = u_fm[i] + eps_fd * diff_fm[i];
    }
    let r1 = pnp_be_residual_vector_f64(
        solver,
        newton,
        dt,
        &scratch_u_fm[0..n],
        &scratch_u_fm[n..2 * n],
        &scratch_u_fm[2 * n..3 * n],
        c_plus_n,
        c_minus_n,
        eps,
        d_plus,
        d_minus,
        g0,
        g1,
    );
    for i in 0..dim {
        diff_fm[i] = (r1[i] - r0_fm[i]) / eps_fd;
    }
    pnp_residual_fm_to_nm(diff_fm, n, out_nm);
}

/// Matrix-free Newton correction \(\delta_{\mathrm{nm}}\) for full-SG BE via GMRES on \(J_{\mathrm{nm}}\).
#[cfg(feature = "electrochemistry-mvp")]
#[cfg_attr(feature = "electrochemistry-mvp", allow(clippy::too_many_arguments))]
fn full_sg_newton_correction_gmres_nm_f64(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f64,
    u_fm: &[f64],
    c_plus_n: &[f64],
    c_minus_n: &[f64],
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    g0: f64,
    g1: f64,
    r0_fm: &[f64],
    rhs_nm: &[f64],
    n: usize,
) -> Result<Vec<f64>, PhysicsError> {
    use super::krylov_host::gmres_f32_try;

    let dim = 3 * n;
    debug_assert_eq!(u_fm.len(), dim);
    debug_assert_eq!(r0_fm.len(), dim);
    debug_assert_eq!(rhs_nm.len(), dim);

    let b_f32: Vec<f32> = rhs_nm.iter().map(|&x| x as f32).collect();
    let beta: f32 = b_f32.iter().map(|x| x * x).sum::<f32>().sqrt();
    if beta < 1e-30_f32 {
        return Ok(vec![0.0_f64; dim]);
    }

    let max_iter = (dim + 96).min(512).max(dim);
    const GMRES_REL_TOL: f32 = 5e-4_f32;

    let u_fm = u_fm.to_vec();
    let r0_fm = r0_fm.to_vec();
    let c_plus_n = c_plus_n.to_vec();
    let c_minus_n = c_minus_n.to_vec();
    let eps = eps.to_vec();
    let d_plus = d_plus.to_vec();
    let d_minus = d_minus.to_vec();
    let solver_a = std::sync::Arc::new(ElectroChemicalSolver {
        faraday_const: solver.faraday_const,
        gas_const: solver.gas_const,
        coupling_picard_iters: solver.coupling_picard_iters,
        coupling_picard_tol_linf: solver.coupling_picard_tol_linf,
        coupling_picard_tol_delta_phi_linf: solver.coupling_picard_tol_delta_phi_linf,
        coupling_picard_tol_delta_phi_l2: solver.coupling_picard_tol_delta_phi_l2,
        mesh_spacing: solver.mesh_spacing,
        pnp_implicit_newton_chain: solver.pnp_implicit_newton_chain,
    });
    let newton = *newton;

    let matvec = move |v: &[f32]| -> Result<Vec<f32>, PhysicsError> {
        let mut v_nm_loc = vec![0.0_f64; dim];
        for i in 0..dim {
            v_nm_loc[i] = v[i] as f64;
        }
        let mut out_nm = vec![0.0_f64; dim];
        let mut scratch = vec![0.0_f64; dim];
        let mut d_fm = vec![0.0_f64; dim];
        pnp_be_full_sg_jacobian_matvec_nm_f64(
            solver_a.as_ref(),
            &newton,
            dt,
            &u_fm,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r0_fm,
            &v_nm_loc,
            &mut scratch,
            &mut d_fm,
            &mut out_nm,
        );
        Ok(out_nm.iter().map(|x| *x as f32).collect())
    };

    let x_g = gmres_f32_try(matvec, &b_f32, dim, max_iter, GMRES_REL_TOL)?;
    Ok(x_g.into_iter().map(|x| x as f64).collect())
}

#[cfg(feature = "electrochemistry-mvp")]
#[inline]
fn row_band_linear_index(bw: usize, i: usize, j: usize, kl: usize) -> usize {
    i * bw + (j + kl - i)
}

#[cfg(feature = "electrochemistry-mvp")]
fn row_band_get(mat: &[f64], kl: usize, ku: usize, bw: usize, i: usize, j: usize) -> f64 {
    if j + kl < i || j > i + ku {
        0.0_f64
    } else {
        mat[row_band_linear_index(bw, i, j, kl)]
    }
}

#[cfg(feature = "electrochemistry-mvp")]
fn row_band_set(mat: &mut [f64], kl: usize, ku: usize, bw: usize, i: usize, j: usize, v: f64) {
    debug_assert!(j + kl >= i && j <= i + ku);
    let ix = row_band_linear_index(bw, i, j, kl);
    mat[ix] = v;
}

/// Exchanges **matrix rows** `i` and `p` while preserving band semantics: for each column `j` the
/// entry at `(i,j)` is swapped with `(p,j)` via [`row_band_get`] / [`row_band_set`], rather than
/// swapping fixed-length row buffers (which would mis-map indices under skewed row-major packing).
#[cfg(feature = "electrochemistry-mvp")]
fn row_band_swap_rows(
    mat: &mut [f64],
    n: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    i: usize,
    p: usize,
) {
    if i == p {
        return;
    }
    for j in 0..n {
        let vij = row_band_get(mat, kl, ku, bw, i, j);
        let vpj = row_band_get(mat, kl, ku, bw, p, j);
        let in_i = j + kl >= i && j <= i + ku;
        let in_p = j + kl >= p && j <= p + ku;
        if in_i {
            row_band_set(mat, kl, ku, bw, i, j, vpj);
        }
        if in_p {
            row_band_set(mat, kl, ku, bw, p, j, vij);
        }
    }
}

/// Band LU **factorisation** with partial pivot on row-major band storage; overwrites `a` with packed
/// `L` (unit diagonal implicit) and `U`. Row interchanges are recorded in `swap_pairs` as `(k, piv)` in
/// elimination order (apply the same swaps to any new RHS before forward substitution).
///
/// **Pivot scope:** only rows `k+1..=min(k+kl, n−1)` are considered in column `k`, unlike
/// [`solve_dense_linear`] which searches the **entire** column — so this factorisation is **not**
/// guaranteed to match dense Gaussian on the same matrix unless `kl` is large enough for the active
/// Schur complement (and all fill fits the `[i−kl, i+ku]` envelope).
///
/// # LAPACK / Netlib correspondence (packing and fill)
///
/// Reference: Netlib [DGBTRF](https://www.netlib.org/lapack/explore-html/d7/db8/dgbtrf_8f.html) /
/// [DGBTRS](https://www.netlib.org/lapack/explore-html/da/d46/dgbtrs_8f.html) factor and solve a general
/// band matrix stored in **column-major** `AB` with leading dimension `LDAB`. For partial pivoting,
/// **`LDAB ≥ 2·KL + KU + 1`** is required so the working band can grow downward as elimination proceeds
/// without overwriting untouched columns — the “extra” rows are the LAPACK analogue of **widening the
/// envelope** beyond a minimal `(KL+KU+1)`-tall strip.
///
/// Here each logical matrix row occupies a **fixed** slice of length `bw = kl + ku + 1` (`bw` in
/// [`row_band_linear_index`]); Schur updates that would land outside `[i−kl, i+ku]` are dropped, so parity
/// with dense Gaussian may require larger `kl`/`ku` (see [`PNP_CHAIN_FULL_SG_JAC_KL_LU`] /
/// [`PNP_CHAIN_FULL_SG_JAC_KU_LU`] and FP-001 notes in [`FP_GAP_BACKLOG.md`](../../docs/FP_GAP_BACKLOG.md)).
#[cfg(feature = "electrochemistry-mvp")]
fn row_band_lu_factorize_partial_pivot(
    a: &mut [f64],
    n: usize,
    kl: usize,
    ku: usize,
    swap_pairs: &mut Vec<(usize, usize)>,
) -> bool {
    swap_pairs.clear();
    let bw = kl + ku + 1;
    debug_assert_eq!(a.len(), n * bw);
    for k in 0..n {
        let mut piv = k;
        let mut best = row_band_get(a, kl, ku, bw, k, k).abs();
        // Pivot search: only rows `i <= k + kl` can be nonzero below the diagonal in column `k` for a
        // matrix whose **original** lower bandwidth is `kl` (fill can widen `kl` during elimination; the
        // caller must size `kl`/`ku` accordingly — see [`PNP_CHAIN_FULL_SG_JAC_KL_LU`]).
        let p1 = (k + kl).min(n - 1);
        for p in (k + 1)..=p1 {
            let v = row_band_get(a, kl, ku, bw, p, k).abs();
            if v > best {
                best = v;
                piv = p;
            }
        }
        if best < 1e-18_f64 {
            return false;
        }
        if piv != k {
            row_band_swap_rows(a, n, kl, ku, bw, k, piv);
            swap_pairs.push((k, piv));
        }
        let akk = row_band_get(a, kl, ku, bw, k, k);
        let i1 = (k + kl).min(n - 1);
        for i in (k + 1)..=i1 {
            let aik = row_band_get(a, kl, ku, bw, i, k);
            if aik == 0.0_f64 {
                continue;
            }
            let m = aik / akk;
            row_band_set(a, kl, ku, bw, i, k, m);
            let j_hi = (k + ku).min(i + ku).min(n - 1);
            for j in (k + 1)..=j_hi {
                let v_ij =
                    row_band_get(a, kl, ku, bw, i, j) - m * row_band_get(a, kl, ku, bw, k, j);
                if j + kl >= i && j <= i + ku {
                    row_band_set(a, kl, ku, bw, i, j, v_ij);
                }
            }
        }
    }
    true
}

/// Apply row interchanges from `row_band_lu_factorize_partial_pivot` to **`rhs`**, then forward substitution
/// with the unit-diagonal **L** factors stored below the diagonal in **`a`**.
#[cfg(feature = "electrochemistry-mvp")]
fn row_band_l_forward_swapped_rhs(
    a: &[f64],
    n: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    swap_pairs: &[(usize, usize)],
    rhs: &mut [f64],
) {
    debug_assert_eq!(a.len(), n * bw);
    // `swap_pairs` records row swaps in elimination order (same order they were applied to `a`).
    // With `P A = L U`, forward substitution solves `L y = P b`; build `P b` by applying the same swaps
    // to `rhs` in **that same order** (first recorded swap first).
    for &(k, piv) in swap_pairs.iter() {
        rhs.swap(k, piv);
    }
    for k in 0..n {
        let i1 = (k + kl).min(n - 1);
        for i in (k + 1)..=i1 {
            let m = row_band_get(a, kl, ku, bw, i, k);
            if m != 0.0_f64 {
                rhs[i] -= m * rhs[k];
            }
        }
    }
}

#[cfg(feature = "electrochemistry-mvp")]
fn row_band_u_back_substitution(
    a: &[f64],
    n: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    rhs: &mut [f64],
) -> bool {
    debug_assert_eq!(a.len(), n * bw);
    for i in (0..n).rev() {
        let mut sum = rhs[i];
        let j0 = (i + 1).min(n);
        // Only j <= i + k_u are stored in the upper triangle; row_band_get is zero beyond that envelope.
        let j1 = (i + ku).min(n - 1);
        for j in j0..=j1 {
            sum -= row_band_get(a, kl, ku, bw, i, j) * rhs[j];
        }
        let uii = row_band_get(a, kl, ku, bw, i, i);
        if uii.abs() < 1e-18_f64 {
            return false;
        }
        rhs[i] = sum / uii;
    }
    true
}

/// Triangular solve after [`row_band_lu_factorize_partial_pivot`]; overwrites `rhs` with the solution.
#[cfg(feature = "electrochemistry-mvp")]
fn row_band_lu_solve_factored(
    a: &[f64],
    n: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    swap_pairs: &[(usize, usize)],
    rhs: &mut [f64],
) -> bool {
    row_band_l_forward_swapped_rhs(a, n, kl, ku, bw, swap_pairs, rhs);
    row_band_u_back_substitution(a, n, kl, ku, bw, rhs)
}

/// Column FD Jacobian for full SG (`linearize_sg_fickian: false`) in **node-major** ordering, row-major band
/// storage (`kl_lu` / `ku_lu` envelope). Only entries with \(|i-j|\le 5\) receive physics; the wider band
/// carries LU fill-in.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn newton_fd_jacobian_full_sg_node_major_row_band(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f64,
    u_fm: &[f64],
    c_plus_n: &[f64],
    c_minus_n: &[f64],
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    g0: f64,
    g1: f64,
    r0_fm: &[f64],
    jac_lu: &mut [f64],
) {
    let n = c_plus_n.len();
    let dim = 3 * n;
    let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
    let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
    let bw = PNP_CHAIN_FULL_SG_BW_LU;
    let kl0 = PNP_CHAIN_FULL_SG_JAC_KL_PHYS;
    let ku0 = PNP_CHAIN_FULL_SG_JAC_KU_PHYS;
    debug_assert_eq!(jac_lu.len(), dim * bw);
    jac_lu.fill(0.0_f64);
    let mut r0_nm = vec![0.0_f64; dim];
    pnp_residual_fm_to_nm(r0_fm, n, &mut r0_nm);
    let mut r_pert_nm = vec![0.0_f64; dim];
    let mut u_pert = vec![0.0_f64; dim];
    for j_nm in 0..dim {
        let fm_j = pnp_nm_index_to_fm(j_nm, n);
        let h = newton.fd_step * (1.0_f64 + u_fm[fm_j].abs());
        u_pert.copy_from_slice(u_fm);
        u_pert[fm_j] += h;
        let phi = &u_pert[0..n];
        let cp = &u_pert[n..2 * n];
        let cm = &u_pert[2 * n..3 * n];
        let r_p = pnp_be_residual_vector_f64(
            solver, newton, dt, phi, cp, cm, c_plus_n, c_minus_n, eps, d_plus, d_minus, g0, g1,
        );
        pnp_residual_fm_to_nm(&r_p, n, &mut r_pert_nm);
        for i_nm in 0..dim {
            if i_nm + kl0 < j_nm || i_nm > j_nm + ku0 {
                continue;
            }
            let val = (r_pert_nm[i_nm] - r0_nm[i_nm]) / h;
            row_band_set(jac_lu, kl_lu, ku_lu, bw, i_nm, j_nm, val);
        }
    }
}

/// Full-SG Newton correction: in-place band LU on **`jac_fact_scratch`** (copy of `jac_band`), then triangular
/// solve into **`rhs_nm`**.
///
/// Parity with `solve_newton_correction_full_sg_row_band_via_dense_expand` / `solve_dense_linear` holds when
/// `PNP_CHAIN_FULL_SG_JAC_KL_LU` and `PNP_CHAIN_FULL_SG_JAC_KU_LU` are large enough for the active **`dim`**
/// (see module rustdoc — **N=17** fixture is the CI-sized lock). **Production** Newton uses
/// **`solve_newton_correction_full_sg_row_band_band_lu_or_dense_expand`** (band LU first, dense fallback).
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn solve_newton_correction_full_sg_row_band_via_band_lu(
    jac_band: &[f64],
    dim: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    jac_fact_scratch: &mut [f64],
    swap_pairs: &mut Vec<(usize, usize)>,
    rhs_nm: &mut [f64],
) -> bool {
    debug_assert_eq!(jac_band.len(), dim * bw);
    debug_assert_eq!(jac_fact_scratch.len(), dim * bw);
    debug_assert_eq!(kl + ku + 1, bw);
    jac_fact_scratch.copy_from_slice(jac_band);
    if !row_band_lu_factorize_partial_pivot(jac_fact_scratch, dim, kl, ku, swap_pairs) {
        return false;
    }
    row_band_lu_solve_factored(jac_fact_scratch, dim, kl, ku, bw, swap_pairs, rhs_nm)
}

/// Try **band LU** on the assembled Jacobian (no \((3N)^2\) dense expand); fall back to
/// [`solve_newton_correction_full_sg_row_band_via_dense_expand`] if factorisation fails (envelope too
/// narrow for **`dim`**, or pivot breakdown).
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn solve_newton_correction_full_sg_row_band_band_lu_or_dense_expand(
    jac_band: &[f64],
    dim: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    jac_lu_scratch: &mut [f64],
    swap_pairs: &mut Vec<(usize, usize)>,
    jac_dense_scratch: &mut [f64],
    rhs_nm: &mut [f64],
) -> bool {
    if solve_newton_correction_full_sg_row_band_via_band_lu(
        jac_band,
        dim,
        kl,
        ku,
        bw,
        jac_lu_scratch,
        swap_pairs,
        rhs_nm,
    ) {
        return true;
    }
    solve_newton_correction_full_sg_row_band_via_dense_expand(
        jac_band,
        dim,
        kl,
        ku,
        bw,
        jac_dense_scratch,
        rhs_nm,
    )
}

/// Expand node-major row band storage to dense row-major `dim×dim`, then Gaussian-eliminate in place
/// (same `solve_dense_linear` as Jacobian unit tests). **Perf:** **\(O(dim^3)\)** — **fallback when**
/// [`solve_newton_correction_full_sg_row_band_via_band_lu`] fails (`dim` exceeds static LU envelope
/// pivot search, pivot breakdown).
#[cfg(feature = "electrochemistry-mvp")]
fn solve_newton_correction_full_sg_row_band_via_dense_expand(
    jac_band: &[f64],
    dim: usize,
    kl: usize,
    ku: usize,
    bw: usize,
    jac_dense_scratch: &mut [f64],
    rhs_nm: &mut [f64],
) -> bool {
    debug_assert_eq!(jac_band.len(), dim * bw);
    debug_assert_eq!(jac_dense_scratch.len(), dim * dim);
    debug_assert_eq!(rhs_nm.len(), dim);
    jac_dense_scratch.fill(0.0_f64);
    for i in 0..dim {
        let j0 = i.saturating_sub(kl);
        let j1 = (i + ku).min(dim - 1);
        for j in j0..=j1 {
            jac_dense_scratch[i * dim + j] = row_band_get(jac_band, kl, ku, bw, i, j);
        }
    }
    solve_dense_linear(dim, jac_dense_scratch, rhs_nm)
}

#[cfg(feature = "electrochemistry-mvp")]
#[cfg_attr(not(test), allow(dead_code))] // dense assembly retained for unit tests / dense–sparse parity
#[allow(clippy::too_many_arguments)]
fn fill_jacobian_linearized_sg_fickian(
    jac: &mut [f64],
    dim: usize,
    n: usize,
    dt: f64,
    f: f64,
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    h_inv: f64,
) {
    jac.fill(0.0);
    let inv_h_sq = h_inv * h_inv;
    let ip = |i: usize| i;
    let icp = |i: usize| n + i;
    let icm = |i: usize| 2 * n + i;
    for i in 0..n {
        let r = ip(i);
        if i == 0 || i + 1 == n {
            jac[r * dim + r] = 1.0;
        } else {
            let eh_l = 0.5_f64 * (eps[i - 1] + eps[i]);
            let eh_r = 0.5_f64 * (eps[i] + eps[i + 1]);
            jac[r * dim + ip(i - 1)] = eh_l * inv_h_sq;
            jac[r * dim + r] = -(eh_l + eh_r) * inv_h_sq;
            jac[r * dim + ip(i + 1)] = eh_r * inv_h_sq;
            jac[r * dim + icp(i)] = f;
            jac[r * dim + icm(i)] = -f;
        }
    }
    for ch in 0..2usize {
        let off = if ch == 0 { n } else { 2 * n };
        let dloc = if ch == 0 { d_plus } else { d_minus };
        for i in 0..n {
            jac[(off + i) * dim + (off + i)] += 1.0 / dt;
        }
        for e in 0..n - 1 {
            let k = h_inv * 0.5_f64 * (dloc[e] + dloc[e + 1]);
            let ie = off + e;
            let i1 = off + e + 1;
            jac[ie * dim + ie] += k;
            jac[ie * dim + i1] -= k;
            jac[i1 * dim + ie] -= k;
            jac[i1 * dim + i1] += k;
        }
    }
}

/// Fickian-linearised SG chain block \((1/\Delta t)\,I + L_{\mathrm{chain}}(D)\) (same `k` pattern as
/// [`fill_jacobian_linearized_sg_fickian`]) for one species on `n` nodes — **symmetric** tridiagonal.
#[cfg(feature = "electrochemistry-mvp")]
fn fill_fickian_species_tridiagonal_chain_f64(
    n: usize,
    dt: f64,
    d_nodal: &[f64],
    h_inv: f64,
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
) {
    debug_assert_eq!(d_nodal.len(), n);
    debug_assert_eq!(a.len(), n);
    debug_assert_eq!(b.len(), n);
    debug_assert_eq!(c.len(), n);
    let inv_dt = 1.0_f64 / dt;
    for i in 0..n {
        a[i] = 0.0_f64;
        b[i] = inv_dt;
        c[i] = 0.0_f64;
    }
    for e in 0..n.saturating_sub(1) {
        let k = h_inv * 0.5_f64 * (d_nodal[e] + d_nodal[e + 1]);
        b[e] += k;
        b[e + 1] += k;
        c[e] -= k;
        a[e + 1] -= k;
    }
}

/// Newton correction for one linearised-SG iteration: \(J\) is block-lower (no \(\partial R_c/\partial\phi\)),
/// so solve \(L_\pm\,\delta c^\pm=-R_{c^\pm}\) with Thomas, then the \(\Phi\) tridiagonal with RHS
/// \(-R_\Phi - F\delta c^+ + F\delta c^-\) on interior rows and Dirichlet identity on the two endpoints.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn solve_newton_correction_linearized_sg_chain_f64(
    n: usize,
    dt: f64,
    f: f64,
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    h_inv: f64,
    r: &[f64],
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
    rhs: &mut [f64],
    u: &mut [f64],
    x: &mut [f64],
) -> bool {
    debug_assert_eq!(r.len(), 3 * n);
    debug_assert_eq!(x.len(), 3 * n);
    debug_assert_eq!(a.len(), n);
    debug_assert_eq!(b.len(), n);
    debug_assert_eq!(c.len(), n);
    debug_assert_eq!(rhs.len(), n);
    debug_assert_eq!(u.len(), n);

    let inv_h_sq = h_inv * h_inv;

    // δc+
    fill_fickian_species_tridiagonal_chain_f64(n, dt, d_plus, h_inv, a, b, c);
    rhs.copy_from_slice(&r[n..2 * n]);
    for v in rhs.iter_mut() {
        *v = -*v;
    }
    let mut bp = b.to_vec();
    let mut rp = rhs.to_vec();
    thomas_tridiagonal_solve_f64(a, &mut bp, c, &mut rp, u);
    x[n..2 * n].copy_from_slice(u);

    // δc−
    fill_fickian_species_tridiagonal_chain_f64(n, dt, d_minus, h_inv, a, b, c);
    rhs.copy_from_slice(&r[2 * n..3 * n]);
    for v in rhs.iter_mut() {
        *v = -*v;
    }
    let mut bm = b.to_vec();
    let mut rm = rhs.to_vec();
    thomas_tridiagonal_solve_f64(a, &mut bm, c, &mut rm, u);
    x[2 * n..3 * n].copy_from_slice(u);

    // δφ — Dirichlet rows 0 and n−1 match [`fill_jacobian_linearized_sg_fickian`].
    let dcp = &x[n..2 * n];
    let dcm = &x[2 * n..3 * n];
    for i in 0..n {
        if i == 0 || i + 1 == n {
            a[i] = 0.0_f64;
            b[i] = 1.0_f64;
            c[i] = 0.0_f64;
            rhs[i] = -r[i];
        } else {
            let eh_l = 0.5_f64 * (eps[i - 1] + eps[i]);
            let eh_r = 0.5_f64 * (eps[i] + eps[i + 1]);
            a[i] = eh_l * inv_h_sq;
            b[i] = -(eh_l + eh_r) * inv_h_sq;
            c[i] = eh_r * inv_h_sq;
            rhs[i] = -r[i] - f * dcp[i] + f * dcm[i];
        }
    }
    let mut bf = b.to_vec();
    let mut rf = rhs.to_vec();
    thomas_tridiagonal_solve_f64(a, &mut bf, c, &mut rf, u);
    x[0..n].copy_from_slice(u);

    x.iter().all(|v| v.is_finite())
}

#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn newton_dense_column_f64(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f64,
    u0: &[f64],
    c_plus_n: &[f64],
    c_minus_n: &[f64],
    eps: &[f64],
    d_plus: &[f64],
    d_minus: &[f64],
    g0: f64,
    g1: f64,
    r0: &[f64],
    jac: &mut [f64],
) {
    let dim = u0.len();
    let n = dim / 3;
    debug_assert_eq!(r0.len(), dim);
    for j in 0..dim {
        let h = newton.fd_step * (1.0_f64 + u0[j].abs());
        let mut u_pert = u0.to_vec();
        u_pert[j] += h;
        let phi = &u_pert[0..n];
        let cp = &u_pert[n..2 * n];
        let cm = &u_pert[2 * n..3 * n];
        let rp = pnp_be_residual_vector_f64(
            solver, newton, dt, phi, cp, cm, c_plus_n, c_minus_n, eps, d_plus, d_minus, g0, g1,
        );
        for i in 0..dim {
            jac[i * dim + j] = (rp[i] - r0[i]) / h;
        }
    }
}

#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
fn try_solve_pnp_be_newton_chain_host<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f32,
    electric_potential_n: Tensor<B, 3>,
    ion_concentration_n: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    permittivity: Tensor<B, 3>,
    diffusivity: Tensor<B, 3>,
) -> Option<(Tensor<B, 3>, Tensor<B, 3>)> {
    let pd = electric_potential_n.dims();
    if pd[0] != 1 || pd[2] != 1 {
        return None;
    }
    let n = pd[1];
    if n < 2 || n > newton.max_chain_nodes {
        return None;
    }
    let ed = edges_b1.dims();
    if ed[0] != 2 || ed[1] != n - 1 {
        return None;
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return None;
    }
    let dt64 = dt as f64;
    if !dt64.is_finite() || dt64 <= 0.0 {
        return None;
    }
    let device = ion_concentration_n.device();
    let phi_h = electric_potential_n.clone().into_data().value;
    let c_h = ion_concentration_n.clone().into_data().value;
    let eps_h = permittivity.into_data().value;
    let d_h = diffusivity.into_data().value;
    let g0 = phi_h[0] as f64;
    let g1 = phi_h[n - 1] as f64;
    let mut c_plus_n = vec![0.0_f64; n];
    let mut c_minus_n = vec![0.0_f64; n];
    let mut eps = vec![0.0_f64; n];
    let mut d_plus = vec![0.0_f64; n];
    let mut d_minus = vec![0.0_f64; n];
    for i in 0..n {
        c_plus_n[i] = c_h[i * 2] as f64;
        c_minus_n[i] = c_h[i * 2 + 1] as f64;
        eps[i] = eps_h[i] as f64;
        d_plus[i] = d_h[i * 2] as f64;
        d_minus[i] = d_h[i * 2 + 1] as f64;
    }
    let f = solver.faraday_const as f64;
    let h_inv = 1.0_f64 / solver.mesh_spacing.max(1e-30_f32) as f64;
    let mut rho_net = vec![0.0_f64; n];
    for i in 0..n {
        rho_net[i] = f * (c_plus_n[i] - c_minus_n[i]);
    }
    let mut phi = vec![0.0_f64; n];
    let h_sq = (solver.mesh_spacing.max(1e-30_f32) as f64).powi(2);
    poisson_chain_net_charge_variable_eps_thomas_f64(n, g0, g1, &eps, &rho_net, h_sq, &mut phi);
    let mut u = vec![0.0_f64; 3 * n];
    u[0..n].copy_from_slice(&phi);
    u[n..2 * n].copy_from_slice(&c_plus_n);
    u[2 * n..3 * n].copy_from_slice(&c_minus_n);
    let dim = 3 * n;
    let inner_cap_sg = (newton.full_sg_frozen_jacobian_inner_iters as usize).clamp(1, 32);
    let alloc_full_sg_band_dense =
        !newton.linearize_sg_fickian && (inner_cap_sg > 1 || !newton.full_sg_correction_use_gmres);
    let mut jac_band =
        alloc_full_sg_band_dense.then(|| vec![0.0_f64; dim * PNP_CHAIN_FULL_SG_BW_LU]);
    let mut jac_lu_scratch =
        alloc_full_sg_band_dense.then(|| vec![0.0_f64; dim * PNP_CHAIN_FULL_SG_BW_LU]);
    let mut jac_dense_scratch = alloc_full_sg_band_dense.then(|| vec![0.0_f64; dim * dim]);
    let mut band_lu_swaps = alloc_full_sg_band_dense.then(Vec::<(usize, usize)>::new);
    let mut rhs_nm = vec![0.0_f64; dim];
    let mut thomas_a = vec![0.0_f64; n];
    let mut thomas_b = vec![0.0_f64; n];
    let mut thomas_c = vec![0.0_f64; n];
    let mut thomas_r = vec![0.0_f64; n];
    let mut thomas_u = vec![0.0_f64; n];
    let mut x = vec![0.0_f64; dim];
    for _it in 0..newton.max_newton_iters {
        let phi_s = &u[0..n];
        let cp_s = &u[n..2 * n];
        let cm_s = &u[2 * n..3 * n];
        let r = pnp_be_residual_vector_f64(
            solver, newton, dt64, phi_s, cp_s, cm_s, &c_plus_n, &c_minus_n, &eps, &d_plus,
            &d_minus, g0, g1,
        );
        let nr = vec_l2(&r);
        if nr < newton.residual_tol_l2 {
            break;
        }
        let (ok, u_frozen_inner) = if newton.linearize_sg_fickian {
            let o = solve_newton_correction_linearized_sg_chain_f64(
                n,
                dt64,
                f,
                &eps,
                &d_plus,
                &d_minus,
                h_inv,
                &r,
                &mut thomas_a,
                &mut thomas_b,
                &mut thomas_c,
                &mut thomas_r,
                &mut thomas_u,
                &mut x,
            );
            (o, false)
        } else if inner_cap_sg <= 1 {
            let use_gmres_here =
                newton.full_sg_correction_use_gmres && !newton.linearize_sg_fickian;
            if use_gmres_here {
                pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
                for v in rhs_nm.iter_mut() {
                    *v = -*v;
                }
                let ok = match full_sg_newton_correction_gmres_nm_f64(
                    solver, newton, dt64, &u, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus, g0,
                    g1, &r, &rhs_nm, n,
                ) {
                    Ok(dn) => {
                        pnp_delta_nm_to_fm(&dn, n, &mut x);
                        true
                    }
                    Err(_) => false,
                };
                (ok, false)
            } else {
                let Some(jac) = jac_band.as_mut() else {
                    return None;
                };
                newton_fd_jacobian_full_sg_node_major_row_band(
                    solver, newton, dt64, &u, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus, g0,
                    g1, &r, jac,
                );
                pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
                for v in rhs_nm.iter_mut() {
                    *v = -*v;
                }
                let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                    return None;
                };
                let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                    return None;
                };
                let Some(swaps) = band_lu_swaps.as_mut() else {
                    return None;
                };
                swaps.clear();
                let ok = solve_newton_correction_full_sg_row_band_band_lu_or_dense_expand(
                    jac,
                    dim,
                    PNP_CHAIN_FULL_SG_JAC_KL_LU,
                    PNP_CHAIN_FULL_SG_JAC_KU_LU,
                    PNP_CHAIN_FULL_SG_BW_LU,
                    lu_buf,
                    swaps,
                    dense_buf,
                    &mut rhs_nm,
                );
                if ok {
                    pnp_delta_nm_to_fm(&rhs_nm, n, &mut x);
                }
                (ok, false)
            }
        } else {
            let Some(jac) = jac_band.as_mut() else {
                return None;
            };
            let Some(lu_buf) = jac_lu_scratch.as_mut() else {
                return None;
            };
            let Some(dense_buf) = jac_dense_scratch.as_mut() else {
                return None;
            };
            let Some(swaps) = band_lu_swaps.as_mut() else {
                return None;
            };
            newton_fd_jacobian_full_sg_node_major_row_band(
                solver, newton, dt64, &u, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus, g0, g1,
                &r, jac,
            );
            let mut r_work = r;
            let mut ok_all = true;
            let mut frozen_used = false;
            for inner_i in 0..inner_cap_sg {
                if vec_l2(&r_work) < newton.residual_tol_l2 {
                    break;
                }
                frozen_used = true;
                pnp_residual_fm_to_nm(&r_work, n, &mut rhs_nm);
                for v in rhs_nm.iter_mut() {
                    *v = -*v;
                }
                swaps.clear();
                if !solve_newton_correction_full_sg_row_band_band_lu_or_dense_expand(
                    jac,
                    dim,
                    PNP_CHAIN_FULL_SG_JAC_KL_LU,
                    PNP_CHAIN_FULL_SG_JAC_KU_LU,
                    PNP_CHAIN_FULL_SG_BW_LU,
                    lu_buf,
                    swaps,
                    dense_buf,
                    &mut rhs_nm,
                ) {
                    ok_all = false;
                    break;
                }
                pnp_delta_nm_to_fm(&rhs_nm, n, &mut x);
                for i in 0..dim {
                    u[i] += newton.damping * x[i];
                }
                u[0] = g0;
                u[n - 1] = g1;
                if inner_i + 1 >= inner_cap_sg {
                    break;
                }
                r_work = pnp_be_residual_vector_f64(
                    solver,
                    newton,
                    dt64,
                    &u[0..n],
                    &u[n..2 * n],
                    &u[2 * n..3 * n],
                    &c_plus_n,
                    &c_minus_n,
                    &eps,
                    &d_plus,
                    &d_minus,
                    g0,
                    g1,
                );
            }
            (ok_all, frozen_used)
        };
        if !ok {
            return None;
        }
        if !u_frozen_inner {
            for i in 0..dim {
                u[i] += newton.damping * x[i];
            }
            u[0] = g0;
            u[n - 1] = g1;
        }
    }
    let phi_s = &u[0..n];
    let cp_s = &u[n..2 * n];
    let cm_s = &u[2 * n..3 * n];
    let r_pre = pnp_be_residual_vector_f64(
        solver, newton, dt64, phi_s, cp_s, cm_s, &c_plus_n, &c_minus_n, &eps, &d_plus, &d_minus,
        g0, g1,
    );
    let n_pre = vec_l2(&r_pre);
    if !n_pre.is_finite() || n_pre > 1e-6_f64 {
        return None;
    }
    let phi_out: Vec<f32> = u[0..n].iter().map(|&x| x as f32).collect();
    let mut c_out = vec![0.0_f32; n * 2];
    for i in 0..n {
        c_out[i * 2] = u[n + i] as f32;
        c_out[i * 2 + 1] = u[2 * n + i] as f32;
    }
    let phi_t = Tensor::from_data(Data::new(phi_out, Shape::new([1, n, 1])), &device);
    let c_t = Tensor::from_data(Data::new(c_out, Shape::new([1, n, 2])), &device);
    Some((phi_t, c_t))
}

/// L2 norm of the fully implicit backward Euler residual on a chain (`batch=1`), for verification.
#[cfg(feature = "electrochemistry-mvp")]
#[allow(clippy::too_many_arguments)]
pub fn pnp_backward_euler_residual_l2_chain_host_f64<B: Backend<FloatElem = f32>>(
    solver: &ElectroChemicalSolver,
    newton: &NewtonPnpContext,
    dt: f32,
    phi: &Tensor<B, 3>,
    c: &Tensor<B, 3>,
    ion_concentration_n: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    permittivity: &Tensor<B, 3>,
    diffusivity: &Tensor<B, 3>,
) -> Option<f64> {
    let pd = phi.dims();
    if pd[0] != 1 || pd[2] != 1 {
        return None;
    }
    let n = pd[1];
    let ed = edges_b1.dims();
    if ed[0] != 2 || ed[1] != n - 1 {
        return None;
    }
    let layout_raw = edges_b1.clone().float().into_data().value;
    let layout: Vec<i64> = layout_raw.iter().map(|&x| x as i64).collect();
    if !is_contiguous_unit_path(n, &layout) {
        return None;
    }
    let phi_h = phi.clone().into_data().value;
    let c_h = c.clone().into_data().value;
    let cn_h = ion_concentration_n.clone().into_data().value;
    let eps_h = permittivity.clone().into_data().value;
    let d_h = diffusivity.clone().into_data().value;
    let g0 = phi_h[0] as f64;
    let g1 = phi_h[n - 1] as f64;
    let mut c_plus_n = vec![0.0_f64; n];
    let mut c_minus_n = vec![0.0_f64; n];
    let mut eps = vec![0.0_f64; n];
    let mut d_plus = vec![0.0_f64; n];
    let mut d_minus = vec![0.0_f64; n];
    let mut phi_v = vec![0.0_f64; n];
    let mut cp = vec![0.0_f64; n];
    let mut cm = vec![0.0_f64; n];
    for i in 0..n {
        phi_v[i] = phi_h[i] as f64;
        cp[i] = c_h[i * 2] as f64;
        cm[i] = c_h[i * 2 + 1] as f64;
        c_plus_n[i] = cn_h[i * 2] as f64;
        c_minus_n[i] = cn_h[i * 2 + 1] as f64;
        eps[i] = eps_h[i] as f64;
        d_plus[i] = d_h[i * 2] as f64;
        d_minus[i] = d_h[i * 2 + 1] as f64;
    }
    let r = pnp_be_residual_vector_f64(
        solver, newton, dt as f64, &phi_v, &cp, &cm, &c_plus_n, &c_minus_n, &eps, &d_plus,
        &d_minus, g0, g1,
    );
    Some(vec_l2(&r))
}

#[cfg(all(test, feature = "electrochemistry-mvp"))]
mod newton_chain_tests {
    use super::*;

    #[test]
    fn band_lu_identity_two_by_two_full_envelope_matches_dense() {
        let n = 2_usize;
        let kl = n - 1;
        let ku = n - 1;
        let bw = kl + ku + 1;
        let mut band = vec![0.0_f64; n * bw];
        row_band_set(&mut band, kl, ku, bw, 0, 0, 1.0);
        row_band_set(&mut band, kl, ku, bw, 1, 1, 1.0);
        let mut fac = band.clone();
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac, n, kl, ku, &mut swaps
        ));
        let mut rhs_band = vec![3.0_f64, 4.0_f64];
        assert!(row_band_lu_solve_factored(
            &fac,
            n,
            kl,
            ku,
            bw,
            &swaps,
            &mut rhs_band
        ));
        let mut a_dense = vec![0.0_f64; n * n];
        a_dense[0] = 1.0;
        a_dense[3] = 1.0;
        let mut rhs_dense = vec![3.0_f64, 4.0_f64];
        assert!(solve_dense_linear(n, &mut a_dense, &mut rhs_dense));
        let mx: f64 = (0..n)
            .map(|i| (rhs_band[i] - rhs_dense[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(mx < 1e-12_f64, "2x2 I: max|Δ|={mx:.3e}");
    }

    #[test]
    fn band_lu_two_by_two_needs_pivot_full_envelope_matches_dense() {
        let n = 2_usize;
        let kl = n - 1;
        let ku = n - 1;
        let bw = kl + ku + 1;
        let mut band = vec![0.0_f64; n * bw];
        // A = [[0, 1], [1, 0]] — zero pivot without interchange.
        row_band_set(&mut band, kl, ku, bw, 0, 0, 0.0);
        row_band_set(&mut band, kl, ku, bw, 0, 1, 1.0);
        row_band_set(&mut band, kl, ku, bw, 1, 0, 1.0);
        row_band_set(&mut band, kl, ku, bw, 1, 1, 0.0);
        let mut fac = band.clone();
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac, n, kl, ku, &mut swaps
        ));
        let mut rhs_band = vec![1.0_f64, 0.0_f64];
        assert!(row_band_lu_solve_factored(
            &fac,
            n,
            kl,
            ku,
            bw,
            &swaps,
            &mut rhs_band
        ));
        let mut a_dense = vec![0.0_f64; n * n];
        // Row-major 2×2: [[0, 1], [1, 0]]
        a_dense[0] = 0.0;
        a_dense[1] = 1.0;
        a_dense[2] = 1.0;
        a_dense[3] = 0.0;
        let mut rhs_dense = vec![1.0_f64, 0.0_f64];
        assert!(solve_dense_linear(n, &mut a_dense, &mut rhs_dense));
        let mx: f64 = (0..n)
            .map(|i| (rhs_band[i] - rhs_dense[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(mx < 1e-12_f64, "2x2 swap: max|Δ|={mx:.3e}");
    }

    // --- FP-001 band-LU ladder (1): isolated fixtures for pivot scope, truncated Schur / envelope, and
    // row-swap packing semantics (see `FP_GAP_BACKLOG.md` § end-condition gap #2). ---

    fn expand_row_band_to_dense_row_major(
        mat: &[f64],
        n: usize,
        kl: usize,
        ku: usize,
        bw: usize,
    ) -> Vec<f64> {
        let mut d = vec![0.0_f64; n * n];
        for i in 0..n {
            for j in 0..n {
                d[i * n + j] = row_band_get(mat, kl, ku, bw, i, j);
            }
        }
        d
    }

    /// First-column partial-pivot row for `k = 0`: dense scans all rows; band-style window uses only
    /// `1..=min(kl, n−1)` (same indexing as [`row_band_lu_factorize_partial_pivot`]).
    fn first_col_partial_pivot_row_dense(jac: &[f64], dim: usize) -> usize {
        let mut piv = 0_usize;
        let mut best = jac[0].abs();
        for r in 1..dim {
            let v = jac[r * dim].abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        piv
    }

    fn first_col_partial_pivot_row_band_window(jac: &[f64], dim: usize, kl: usize) -> usize {
        let mut piv = 0_usize;
        let mut best = jac[0].abs();
        let p1 = kl.min(dim - 1);
        for p in 1..=p1 {
            let v = jac[p * dim].abs();
            if v > best {
                best = v;
                piv = p;
            }
        }
        piv
    }

    /// FP-001 / ladder (1): **pivot scope** — on a dense `4×4` system, [`solve_dense_linear`] can pick a
    /// different first-column pivot than a band-restricted search (`kl = 1` window on the **same** matrix).
    #[test]
    fn fp001_band_style_pivot_window_differs_from_full_column_dense_pivot() {
        let dim = 4_usize;
        let kl_win = 1_usize;
        let mut jac = vec![0.0_f64; dim * dim];
        jac[0] = 1e-12;
        jac[dim] = 1.0;
        jac[2 * dim] = 10.0;
        jac[3 * dim] = 100.0;
        jac[dim + 1] = 1.0;
        jac[2 * dim + 2] = 1.0;
        jac[3 * dim + 3] = 1.0;
        let p_full = first_col_partial_pivot_row_dense(&jac, dim);
        let p_win = first_col_partial_pivot_row_band_window(&jac, dim, kl_win);
        assert_eq!(p_full, 3, "dense should pivot row 3 into row 0");
        assert_eq!(
            p_win, 1,
            "band-window (kl=1) only sees row 1 below the diagonal"
        );
        assert_ne!(p_full, p_win);
    }

    /// FP-001 / ladder (1c): **`kl ≥ dim − 1`** (here `kl = ku = dim − 1`) gives a full **row-band**
    /// envelope — the pivot window in [`row_band_lu_factorize_partial_pivot`] matches a **full-column**
    /// search on the **initial** pattern, and Schur fill stays **inside** the strip — so band LU + solve
    /// matches [`solve_dense_linear`] (research slice toward LAPACK pivot semantics without widening `LDAB`
    /// in production).
    #[test]
    fn fp001_kl_ge_dim_minus_one_full_envelope_matches_dense_gaussian() {
        let n = 6_usize;
        let kl = n - 1;
        let ku = n - 1;
        let bw = kl + ku + 1;
        let mut band = vec![0.0_f64; n * bw];
        // Strictly diagonally dominant dense pattern (stable for partial pivot vs reference).
        let mut dense = vec![0.0_f64; n * n];
        for i in 0..n {
            for j in 0..n {
                let v = if i == j {
                    40.0_f64 + i as f64
                } else {
                    let h = (i.wrapping_mul(31) ^ j.wrapping_mul(17)).wrapping_rem(7) as i32 - 3;
                    0.5_f64 * h as f64
                };
                dense[i * n + j] = v;
                row_band_set(&mut band, kl, ku, bw, i, j, v);
            }
        }
        let rhs0: Vec<f64> = (1..=n).map(|k| k as f64 * 0.25 - 0.5).collect();

        let mut jac_dense = dense.clone();
        let mut rhs_d = rhs0.clone();
        assert!(solve_dense_linear(n, &mut jac_dense, &mut rhs_d));

        let mut fac = band.clone();
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac, n, kl, ku, &mut swaps
        ));
        let mut rhs_b = rhs0.clone();
        assert!(row_band_lu_solve_factored(
            &fac, n, kl, ku, bw, &swaps, &mut rhs_b
        ));

        let mx: f64 = (0..n)
            .map(|i| (rhs_b[i] - rhs_d[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx < 1e-10_f64,
            "full envelope kl=ku=dim-1: expected band-LU parity with dense Gaussian, max|Δ|={mx:.3e}"
        );
    }

    /// FP-001 / ladder (1d): **true narrow bandwidth** — random symmetric **strictly diagonally dominant**
    /// band matrices (nonzeros only for `|i−j| ≤ w`) are SPD. Partial pivot can widen fill; Netlib
    /// `DGBTRF` column-major `AB` therefore uses `LDAB ≥ 2·KL + KU + 1` for static half-bandwidths
    /// `(KL, KU) = (w, w)`. This test stores the same pattern in row-band form with `kl = ku = 2w`, so
    /// `bw = 4w + 1 ≥ 3w + 1`, documenting a conservative envelope toward **production** `(KL_LU, KU_LU)`
    /// sizing (still not the full-SG Jacobian lane — see [`FP_GAP_BACKLOG.md`](../../docs/FP_GAP_BACKLOG.md)).
    #[test]
    fn fp001_random_ssdd_symmetric_band_matches_dense_under_documented_wide_envelope() {
        let n = 15_usize;
        let w = 2_usize;
        let kl = 2 * w;
        let ku = 2 * w;
        let bw = kl + ku + 1;
        assert!(
            bw > 3 * w,
            "bw={bw} should admit the LAPACK LDAB lower-bound narrative for static KL=KU=w ({lb})",
            lb = 3 * w + 1
        );

        let seeds: [f64; 9] = [0.11, -0.23, 0.37, -0.19, 0.29, -0.31, 0.17, -0.09, 0.41];
        let mut dense = vec![0.0_f64; n * n];
        for i in 0..n {
            let j1 = (i + w).min(n - 1);
            for j in (i + 1)..=j1 {
                let v = 0.04_f64 * seeds[(i + j) % seeds.len()];
                dense[i * n + j] = v;
                dense[j * n + i] = v;
            }
        }
        for i in 0..n {
            let mut off = 0.0_f64;
            for j in 0..n {
                if i != j {
                    off += dense[i * n + j].abs();
                }
            }
            dense[i * n + i] = 8.0_f64 + off;
        }

        let mut band = vec![0.0_f64; n * bw];
        for i in 0..n {
            for j in 0..n {
                let v = dense[i * n + j];
                if j + kl >= i && j <= i + ku {
                    row_band_set(&mut band, kl, ku, bw, i, j, v);
                }
            }
        }

        let rhs0: Vec<f64> = (0..n).map(|k| (k as f64 + 1.0).cos() * 0.7).collect();

        let mut jac_dense = dense.clone();
        let mut rhs_d = rhs0.clone();
        assert!(solve_dense_linear(n, &mut jac_dense, &mut rhs_d));

        let mut fac = band.clone();
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac, n, kl, ku, &mut swaps
        ));
        let mut rhs_b = rhs0.clone();
        assert!(row_band_lu_solve_factored(
            &fac, n, kl, ku, bw, &swaps, &mut rhs_b
        ));

        let mx: f64 = (0..n)
            .map(|i| (rhs_b[i] - rhs_d[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx < 1e-9_f64,
            "narrow-band SPD with widened envelope: band-LU vs dense Gaussian max|Δ|={mx:.3e}"
        );
    }

    /// FP-001 / ladder (1): **truncated Schur / envelope** — same sparse pattern expanded from band
    /// storage with a **tight** `(kl, ku) = (1, 1)` envelope yields a band-LU solve that disagrees with
    /// [`solve_dense_linear`] on the expanded matrix; widening to `(2, 2)` restores parity (next ladder
    /// rung: LAPACK-style `LDAB` / fill envelope).
    #[test]
    fn fp001_tight_envelope_band_lu_differs_from_dense_then_wide_envelope_matches() {
        let n = 4_usize;
        let kl_t = 1_usize;
        let ku_t = 1_usize;
        let bw_t = kl_t + ku_t + 1;
        let mut band_tight = vec![0.0_f64; n * bw_t];
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 0, 0, 3.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 0, 1, 1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 1, 0, 3.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 1, 1, 1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 1, 2, -3.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 2, 1, -1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 2, 2, 1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 2, 3, 1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 3, 2, 1.0);
        row_band_set(&mut band_tight, kl_t, ku_t, bw_t, 3, 3, 3.0);

        let mut jac_dense = expand_row_band_to_dense_row_major(&band_tight, n, kl_t, ku_t, bw_t);
        let rhs0 = [1.0_f64, -1.0, 2.0, 1.0];
        let mut rhs_d = rhs0.to_vec();
        assert!(solve_dense_linear(n, &mut jac_dense, &mut rhs_d));

        let mut fac = band_tight.clone();
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac, n, kl_t, ku_t, &mut swaps
        ));
        let mut rhs_b = rhs0.to_vec();
        assert!(row_band_lu_solve_factored(
            &fac, n, kl_t, ku_t, bw_t, &swaps, &mut rhs_b
        ));

        let mx_tight: f64 = (0..n)
            .map(|i| (rhs_b[i] - rhs_d[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_tight > 0.25_f64,
            "expected band-LU vs dense mismatch on tight envelope, max|Δ|={mx_tight:.3e}"
        );

        let kl_w = 2_usize;
        let ku_w = 2_usize;
        let bw_w = kl_w + ku_w + 1;
        let mut band_wide = vec![0.0_f64; n * bw_w];
        for i in 0..n {
            for j in 0..n {
                let v = row_band_get(&band_tight, kl_t, ku_t, bw_t, i, j);
                if j + kl_w >= i && j <= i + ku_w {
                    row_band_set(&mut band_wide, kl_w, ku_w, bw_w, i, j, v);
                }
            }
        }
        let mut jac_wide = expand_row_band_to_dense_row_major(&band_wide, n, kl_w, ku_w, bw_w);
        let mut rhs_dw = rhs0.to_vec();
        assert!(solve_dense_linear(n, &mut jac_wide, &mut rhs_dw));

        let mut fac_w = band_wide.clone();
        swaps.clear();
        assert!(row_band_lu_factorize_partial_pivot(
            &mut fac_w, n, kl_w, ku_w, &mut swaps
        ));
        let mut rhs_bw = rhs0.to_vec();
        assert!(row_band_lu_solve_factored(
            &fac_w,
            n,
            kl_w,
            ku_w,
            bw_w,
            &swaps,
            &mut rhs_bw
        ));

        let mx_wide: f64 = (0..n)
            .map(|i| (rhs_bw[i] - rhs_d[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_wide < 1e-9_f64,
            "widened envelope should recover dense parity, max|Δ|={mx_wide:.3e}"
        );
    }

    /// FP-001 / ladder (1): **row swaps** — naive contiguous `bw`-slice exchange mis-maps skewed
    /// row-major packing vs [`row_band_swap_rows`]; band column-wise interchange also differs from a full
    /// dense `n×n` row swap when the interchange would place entries outside the declared envelope.
    #[test]
    fn fp001_naive_band_row_slice_swap_misindexes_skewed_storage() {
        let n = 3_usize;
        let kl = 1_usize;
        let ku = 1_usize;
        let bw = kl + ku + 1;
        let mut semantic = vec![0.0_f64; n * bw];
        row_band_set(&mut semantic, kl, ku, bw, 0, 0, 1.0);
        row_band_set(&mut semantic, kl, ku, bw, 0, 1, 2.0);
        row_band_set(&mut semantic, kl, ku, bw, 1, 0, 3.0);
        row_band_set(&mut semantic, kl, ku, bw, 1, 1, 4.0);
        row_band_set(&mut semantic, kl, ku, bw, 1, 2, 5.0);
        row_band_set(&mut semantic, kl, ku, bw, 2, 1, 6.0);
        row_band_set(&mut semantic, kl, ku, bw, 2, 2, 7.0);

        let dense_before = expand_row_band_to_dense_row_major(&semantic, n, kl, ku, bw);

        let mut naive = semantic.clone();
        let row_a = 0_usize;
        let row_b = 2_usize;
        let s0 = row_a * bw;
        let s1 = row_b * bw;
        for off in 0..bw {
            naive.swap(s0 + off, s1 + off);
        }

        let mut swapped_sem = semantic.clone();
        row_band_swap_rows(&mut swapped_sem, n, kl, ku, bw, row_a, row_b);

        let dense_naive = expand_row_band_to_dense_row_major(&naive, n, kl, ku, bw);
        let dense_sem = expand_row_band_to_dense_row_major(&swapped_sem, n, kl, ku, bw);

        let max_naive_vs_semantic: f64 = (0..(n * n))
            .map(|k| (dense_naive[k] - dense_sem[k]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            max_naive_vs_semantic > 0.5_f64,
            "naive bw-slice swap should diverge from row_band_swap_rows, max|Δ|={max_naive_vs_semantic:.3e}"
        );

        // `row_band_swap_rows` exchanges band-stored entries column-wise; it need not coincide with a
        // full `n×n` dense row swap when the interchange would place mass outside either row envelope.
        let mut dense_ref = dense_before.clone();
        for j in 0..n {
            dense_ref.swap(row_a * n + j, row_b * n + j);
        }
        let max_sem_vs_dense_full_row: f64 = (0..(n * n))
            .map(|k| (dense_sem[k] - dense_ref[k]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            max_sem_vs_dense_full_row > 0.5_f64,
            "band row swap vs naive dense row swap should differ when fill would leave the envelope, max|Δ|={max_sem_vs_dense_full_row:.3e}"
        );
    }

    #[test]
    fn linearized_jacobian_one_newton_nears_zero_residual() {
        let n = 9_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            max_newton_iters: 1,
            residual_tol_l2: 1e-20,
            linearize_sg_fickian: true,
            ..Default::default()
        };
        let dt = 1e-7_f64;
        let g0 = 0.015_f64;
        let g1 = 0.0_f64;
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        let eps: Vec<f64> = vec![1.0_f64; n];
        let d_plus: Vec<f64> = vec![0.04_f64; n];
        let d_minus: Vec<f64> = vec![0.04_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.02 * x;
            c_minus_n[i] = 1.0 - 0.02 * x;
        }
        let f = solver.faraday_const as f64;
        let h_inv = 1.0_f64 / solver.mesh_spacing as f64;
        let mut rho_net = vec![0.0_f64; n];
        for i in 0..n {
            rho_net[i] = f * (c_plus_n[i] - c_minus_n[i]);
        }
        let mut phi = vec![0.0_f64; n];
        poisson_chain_net_charge_variable_eps_thomas_f64(
            n,
            g0,
            g1,
            &eps,
            &rho_net,
            (solver.mesh_spacing as f64).powi(2),
            &mut phi,
        );
        let mut u = vec![0.0_f64; 3 * n];
        u[0..n].copy_from_slice(&phi);
        u[n..2 * n].copy_from_slice(&c_plus_n);
        u[2 * n..3 * n].copy_from_slice(&c_minus_n);
        let dim = 3 * n;
        let r0 = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let n0 = vec_l2(&r0);
        let mut jac = vec![0.0_f64; dim * dim];
        fill_jacobian_linearized_sg_fickian(
            &mut jac, dim, n, dt, f, &eps, &d_plus, &d_minus, h_inv,
        );
        let mut a_work = jac.clone();
        let mut x: Vec<f64> = r0.iter().map(|v| -v).collect();
        assert!(solve_dense_linear(dim, &mut a_work, &mut x), "linear solve");
        for i in 0..dim {
            u[i] += x[i];
        }
        u[0] = g0;
        u[n - 1] = g1;
        let r1 = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let n1 = vec_l2(&r1);
        assert!(
            n1 < 1e-9,
            "expected one Newton to nearly zero affine residual, n0={n0:.3e} n1={n1:.3e}"
        );
    }

    /// Block-Thomas Newton correction for `linearize_sg_fickian` must match the assembled dense Jacobian.
    #[test]
    fn linearized_sg_newton_correction_block_matches_dense_jacobian() {
        let n = 23_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.07_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: true,
            ..Default::default()
        };
        let dt = 2.3e-4_f64;
        let f = solver.faraday_const as f64;
        let h_inv = 1.0_f64 / solver.mesh_spacing as f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.12 * (x - 0.5).powi(2);
            d_plus[i] = 0.03 + 0.01 * x.sin();
            d_minus[i] = 0.028 + 0.009 * (x * 1.7).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.05 * x;
            c_minus_n[i] = 1.0 - 0.04 * x * x;
        }
        let g0 = 0.02_f64;
        let g1 = -0.01_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.015 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.002 * ((i % 3) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.001 * ((i % 2) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;
        let r = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let mut jac = vec![0.0_f64; dim * dim];
        fill_jacobian_linearized_sg_fickian(
            &mut jac, dim, n, dt, f, &eps, &d_plus, &d_minus, h_inv,
        );
        let mut x_dense: Vec<f64> = r.iter().map(|v| -v).collect();
        let mut a_work = jac.clone();
        assert!(
            solve_dense_linear(dim, &mut a_work, &mut x_dense),
            "dense ref solve"
        );

        let mut th_a = vec![0.0_f64; n];
        let mut th_b = vec![0.0_f64; n];
        let mut th_c = vec![0.0_f64; n];
        let mut th_r = vec![0.0_f64; n];
        let mut th_u = vec![0.0_f64; n];
        let mut x_block = vec![0.0_f64; dim];
        assert!(solve_newton_correction_linearized_sg_chain_f64(
            n,
            dt,
            f,
            &eps,
            &d_plus,
            &d_minus,
            h_inv,
            &r,
            &mut th_a,
            &mut th_b,
            &mut th_c,
            &mut th_r,
            &mut th_u,
            &mut x_block,
        ));
        let max_abs: f64 = (0..dim)
            .map(|i| (x_dense[i] - x_block[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            max_abs < 1e-10_f64,
            "block Thomas vs dense Jacobian solve: max_abs={max_abs:.3e}"
        );
    }

    /// Full-SG (`linearize_sg_fickian: false`) **node-major band** FD Jacobian matches dense column FD;
    /// Newton correction from **band expanded to dense** matches the all-dense column-FD linear solve on **N=17**.
    /// **Production** [`try_solve_pnp_backward_euler_newton_chain`] uses the same
    /// [`solve_newton_correction_full_sg_row_band_via_dense_expand`] path as this test.
    #[test]
    fn full_sg_newton_band_expand_dense_matches_dense_column_fd_reference() {
        let n = 17_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-6,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
            d_plus[i] = 0.031 + 0.008 * x.sin();
            d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.06 * x;
            c_minus_n[i] = 1.0 - 0.05 * x * x;
        }
        let g0 = 0.018_f64;
        let g1 = -0.012_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.012 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;
        let r = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;
        let kl0 = PNP_CHAIN_FULL_SG_JAC_KL_PHYS;
        let ku0 = PNP_CHAIN_FULL_SG_JAC_KU_PHYS;
        let mut jac_dense = vec![0.0_f64; dim * dim];
        newton_dense_column_f64(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r,
            &mut jac_dense,
        );
        let mut jac_band = vec![0.0_f64; dim * bw_lu];
        newton_fd_jacobian_full_sg_node_major_row_band(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r,
            &mut jac_band,
        );
        for i_nm in 0..dim {
            for j_nm in 0..dim {
                if i_nm + kl0 < j_nm || i_nm > j_nm + ku0 {
                    continue;
                }
                let i_fm = pnp_nm_index_to_fm(i_nm, n);
                let j_fm = pnp_nm_index_to_fm(j_nm, n);
                let jd = jac_dense[i_fm * dim + j_fm];
                let jb = row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i_nm, j_nm);
                let diff = (jd - jb).abs();
                assert!(
                    diff < 5e-7_f64,
                    "J mismatch nm({i_nm},{j_nm}) fm({i_fm},{j_fm}): dense={jd:.3e} band={jb:.3e}"
                );
            }
        }
        // Node-major band expanded to dense must match the permuted dense Jacobian everywhere
        // (zeros outside the physics band).
        let mut jac_nm_dense = vec![0.0_f64; dim * dim];
        for i_nm in 0..dim {
            for j_nm in 0..dim {
                jac_nm_dense[i_nm * dim + j_nm] =
                    row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i_nm, j_nm);
            }
        }
        for i_nm in 0..dim {
            for j_nm in 0..dim {
                let i_fm = pnp_nm_index_to_fm(i_nm, n);
                let j_fm = pnp_nm_index_to_fm(j_nm, n);
                let diff = (jac_nm_dense[i_nm * dim + j_nm] - jac_dense[i_fm * dim + j_fm]).abs();
                assert!(
                    diff < 1e-15_f64,
                    "nm dense vs fm dense mismatch at nm({i_nm},{j_nm}): diff={diff:.3e}"
                );
            }
        }
        let mut x_dense: Vec<f64> = r.iter().map(|v| -*v).collect();
        let mut a_work = jac_dense.clone();
        assert!(
            solve_dense_linear(dim, &mut a_work, &mut x_dense),
            "dense solve"
        );
        let mut rhs_nm = vec![0.0_f64; dim];
        pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
        for v in rhs_nm.iter_mut() {
            *v = -*v;
        }
        let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
        assert!(
            solve_newton_correction_full_sg_row_band_via_dense_expand(
                &jac_band,
                dim,
                kl_lu,
                ku_lu,
                bw_lu,
                &mut jac_dense_scratch,
                &mut rhs_nm,
            ),
            "dense expand from band"
        );
        let mut x_band_fm = vec![0.0_f64; dim];
        pnp_delta_nm_to_fm(&rhs_nm, n, &mut x_band_fm);
        let max_dx: f64 = (0..dim)
            .map(|i| (x_dense[i] - x_band_fm[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            max_dx < 1e-9_f64,
            "Newton dx dense vs band-expand-dense: max_abs={max_dx:.3e}"
        );
        let mut mx = vec![0.0_f64; dim];
        for i in 0..dim {
            let mut s = 0.0_f64;
            for j in 0..dim {
                s += jac_dense[i * dim + j] * x_band_fm[j];
            }
            mx[i] = s;
        }
        let lin_err: f64 = (0..dim)
            .map(|i| (mx[i] + r[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            lin_err < 1e-7_f64,
            "expected J*dx ≈ -R (full SG), max|Jx+R|={lin_err:.3e}"
        );
    }

    /// Multi-**N** regression: [`solve_newton_correction_full_sg_row_band_via_dense_expand`] matches
    /// [`solve_dense_linear`] on the **fully expanded** band Jacobian (same pivot order as production inner solve).
    #[test]
    fn full_sg_newton_dense_expand_matches_direct_gaussian_multi_n() {
        let ns = [17_usize, 33, 49, 65, 81];
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-6,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;

        for &n in &ns {
            let mut eps = vec![0.0_f64; n];
            let mut d_plus = vec![0.0_f64; n];
            let mut d_minus = vec![0.0_f64; n];
            for i in 0..n {
                let x = i as f64 / (n - 1) as f64;
                eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
                d_plus[i] = 0.031 + 0.008 * x.sin();
                d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
            }
            let mut c_plus_n = vec![0.0_f64; n];
            let mut c_minus_n = vec![0.0_f64; n];
            for i in 0..n {
                let x = i as f64 / (n - 1) as f64;
                c_plus_n[i] = 1.0 + 0.06 * x;
                c_minus_n[i] = 1.0 - 0.05 * x * x;
            }
            let g0 = 0.018_f64;
            let g1 = -0.012_f64;
            let mut u = vec![0.0_f64; 3 * n];
            for i in 0..n {
                u[i] = 0.012 * (i as f64 / n as f64).sin();
                u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
                u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
            }
            u[0] = g0;
            u[n - 1] = g1;
            let r = pnp_be_residual_vector_f64(
                &solver,
                &newton,
                dt,
                &u[0..n],
                &u[n..2 * n],
                &u[2 * n..3 * n],
                &c_plus_n,
                &c_minus_n,
                &eps,
                &d_plus,
                &d_minus,
                g0,
                g1,
            );
            let dim = 3 * n;
            let mut jac_band = vec![0.0_f64; dim * bw_lu];
            newton_fd_jacobian_full_sg_node_major_row_band(
                &solver,
                &newton,
                dt,
                &u,
                &c_plus_n,
                &c_minus_n,
                &eps,
                &d_plus,
                &d_minus,
                g0,
                g1,
                &r,
                &mut jac_band,
            );

            let mut rhs_nm = vec![0.0_f64; dim];
            pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
            for v in rhs_nm.iter_mut() {
                *v = -*v;
            }
            let rhs_gauss = rhs_nm.clone();

            let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
            let mut rhs_de = rhs_nm;
            assert!(
                solve_newton_correction_full_sg_row_band_via_dense_expand(
                    &jac_band,
                    dim,
                    kl_lu,
                    ku_lu,
                    bw_lu,
                    &mut jac_dense_scratch,
                    &mut rhs_de,
                ),
                "dense-expand Newton correction N={n}"
            );

            let mut a_full = vec![0.0_f64; dim * dim];
            for i_nm in 0..dim {
                for j_nm in 0..dim {
                    a_full[i_nm * dim + j_nm] =
                        row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i_nm, j_nm);
                }
            }
            let mut x_gauss = rhs_gauss;
            assert!(
                solve_dense_linear(dim, &mut a_full, &mut x_gauss),
                "direct Gaussian on expanded Jacobian N={n}"
            );

            let max_dx: f64 = (0..dim)
                .map(|i| (rhs_de[i] - x_gauss[i]).abs())
                .fold(0.0_f64, f64::max);
            assert!(
                max_dx < 1e-11_f64,
                "N={n}: dense-expand δ vs direct Gaussian max_abs={max_dx:.3e}"
            );
        }
    }

    #[test]
    fn row_band_lu_matches_dense_random_well_conditioned_51() {
        let n = 51_usize;
        let kl = n - 1;
        let ku = n - 1;
        let bw = kl + ku + 1;
        let mut dense = vec![0.0_f64; n * n];
        for i in 0..n {
            dense[i * n + i] = 10.0_f64;
        }
        let seeds: [f64; 8] = [0.31, -0.17, 0.52, 0.09, -0.44, 0.28, -0.06, 0.33];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                dense[i * n + j] =
                    0.03_f64 * seeds[(i + j) % 8] / (1 + (i as isize - j as isize).abs()) as f64;
            }
        }
        let mut band = vec![0.0_f64; n * bw];
        for i in 0..n {
            for j in 0..n {
                row_band_set(&mut band, kl, ku, bw, i, j, dense[i * n + j]);
            }
        }
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        let mut fac = band.clone();
        assert!(
            row_band_lu_factorize_partial_pivot(&mut fac, n, kl, ku, &mut swaps),
            "factor n=51"
        );
        let mut rhs_b = vec![0.0_f64; n];
        for i in 0..n {
            rhs_b[i] = (i as f64 + 1.0).sin();
        }
        let rhs0 = rhs_b.clone();
        let mut rhs_dense = rhs_b.clone();
        let mut a_dense = dense.clone();
        assert!(solve_dense_linear(n, &mut a_dense, &mut rhs_dense), "dense");
        let mut rhs_band = rhs0.clone();
        assert!(row_band_lu_solve_factored(
            &fac,
            n,
            kl,
            ku,
            bw,
            &swaps,
            &mut rhs_band
        ));
        let mx: f64 = (0..n)
            .map(|i| (rhs_band[i] - rhs_dense[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(mx < 1e-9_f64, "band vs dense solve n=51 max|Δ|={mx:.3e}");
    }

    #[test]
    fn row_band_lu_matches_dense_random_well_conditioned_8() {
        let n = 8_usize;
        let kl = n - 1;
        let ku = n - 1;
        let bw = kl + ku + 1;
        let mut dense = vec![0.0_f64; n * n];
        for i in 0..n {
            dense[i * n + i] = 10.0_f64;
        }
        let seeds: [f64; 8] = [0.31, -0.17, 0.52, 0.09, -0.44, 0.28, -0.06, 0.33];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                dense[i * n + j] =
                    0.03_f64 * seeds[(i + j) % n] / (1 + (i as isize - j as isize).abs()) as f64;
            }
        }
        let mut band = vec![0.0_f64; n * bw];
        for i in 0..n {
            for j in 0..n {
                row_band_set(&mut band, kl, ku, bw, i, j, dense[i * n + j]);
            }
        }
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        let mut fac = band.clone();
        assert!(
            row_band_lu_factorize_partial_pivot(&mut fac, n, kl, ku, &mut swaps),
            "factor"
        );
        let mut rhs_b = vec![0.0_f64; n];
        for i in 0..n {
            rhs_b[i] = (i as f64 + 1.0).sin();
        }
        let rhs0 = rhs_b.clone();
        let mut rhs_dense = rhs_b.clone();
        let mut a_dense = dense.clone();
        assert!(solve_dense_linear(n, &mut a_dense, &mut rhs_dense), "dense");
        let mut rhs_band = rhs0.clone();
        assert!(row_band_lu_solve_factored(
            &fac,
            n,
            kl,
            ku,
            bw,
            &swaps,
            &mut rhs_band
        ));
        let mx: f64 = (0..n)
            .map(|i| (rhs_band[i] - rhs_dense[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(mx < 1e-10_f64, "band vs dense solve max|Δ|={mx:.3e}");
    }

    #[test]
    fn row_band_lu_identity_smoke() {
        let n = 7_usize;
        let kl = 3_usize;
        let ku = 3_usize;
        let bw = kl + ku + 1;
        let mut a = vec![0.0_f64; n * bw];
        for i in 0..n {
            row_band_set(&mut a, kl, ku, bw, i, i, 1.0_f64);
        }
        let mut swaps: Vec<(usize, usize)> = Vec::new();
        assert!(
            row_band_lu_factorize_partial_pivot(&mut a, n, kl, ku, &mut swaps),
            "I factor"
        );
        let mut rhs = vec![1.0_f64; n];
        assert!(row_band_lu_solve_factored(
            &a, n, kl, ku, bw, &swaps, &mut rhs
        ));
        for i in 0..n {
            assert!(
                (rhs[i] - 1.0_f64).abs() < 1e-12_f64,
                "I x = 1, rhs[{i}]={}",
                rhs[i]
            );
        }
    }

    /// FP-001: **`solve_newton_correction_full_sg_row_band_via_band_lu`** (in-place `row_band_lu_factorize_partial_pivot`
    /// + `row_band_lu_solve_factored`) must match **dense expand + `solve_dense_linear`**
    /// on the canonical **N=17** full-SG fixture (same Jacobian/RHS as
    /// [`full_sg_newton_band_expand_dense_matches_dense_column_fd_reference`]).
    #[test]
    fn full_sg_newton_band_lu_matches_dense_expand_n17_fixture() {
        let n = 17_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-6,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
            d_plus[i] = 0.031 + 0.008 * x.sin();
            d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.06 * x;
            c_minus_n[i] = 1.0 - 0.05 * x * x;
        }
        let g0 = 0.018_f64;
        let g1 = -0.012_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.012 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;
        let r = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;
        let mut jac_band = vec![0.0_f64; dim * bw_lu];
        newton_fd_jacobian_full_sg_node_major_row_band(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r,
            &mut jac_band,
        );
        let mut jac_nm_dense = vec![0.0_f64; dim * dim];
        for i_nm in 0..dim {
            for j_nm in 0..dim {
                jac_nm_dense[i_nm * dim + j_nm] =
                    row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i_nm, j_nm);
            }
        }

        let mut rhs0 = vec![0.0_f64; dim];
        pnp_residual_fm_to_nm(&r, n, &mut rhs0);
        for v in rhs0.iter_mut() {
            *v = -*v;
        }
        let mut rhs_lu = rhs0.clone();
        let mut jac_lu_fact = vec![0.0_f64; dim * bw_lu];
        let mut lu_swaps: Vec<(usize, usize)> = Vec::new();
        assert!(
            solve_newton_correction_full_sg_row_band_via_band_lu(
                &jac_band,
                dim,
                kl_lu,
                ku_lu,
                bw_lu,
                &mut jac_lu_fact,
                &mut lu_swaps,
                &mut rhs_lu,
            ),
            "band LU linear solve N=17"
        );
        let mut rhs_de = rhs0.clone();
        let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
        assert!(
            solve_newton_correction_full_sg_row_band_via_dense_expand(
                &jac_band,
                dim,
                kl_lu,
                ku_lu,
                bw_lu,
                &mut jac_dense_scratch,
                &mut rhs_de,
            ),
            "dense-expand linear solve N=17"
        );
        let max_nm: f64 = (0..dim)
            .map(|i| (rhs_lu[i] - rhs_de[i]).abs())
            .fold(0.0_f64, f64::max);
        let lin_res = |dx_nm: &[f64]| -> f64 {
            let mut mx = vec![0.0_f64; dim];
            for i in 0..dim {
                let mut s = 0.0_f64;
                for j in 0..dim {
                    s += jac_nm_dense[i * dim + j] * dx_nm[j];
                }
                mx[i] = s;
            }
            (0..dim)
                .map(|i| (mx[i] - rhs0[i]).abs())
                .fold(0.0_f64, f64::max)
        };
        let err_lu = lin_res(&rhs_lu);
        let err_de = lin_res(&rhs_de);
        assert!(
            err_de < 1e-9_f64,
            "dense-expand residual max|J δ − (−R)|={err_de:.3e}"
        );
        assert!(
            max_nm < 1e-9_f64 && err_lu < 1e-9_f64,
            "band LU vs dense-expand: max|δ_lu−δ_de|={max_nm:.3e} max|Jδ+R|_lu={err_lu:.3e} _de={err_de:.3e}"
        );
    }

    /// **Exec §1 — 3×N PNP stack:** matrix-free \(J_{\mathrm{nm}} v\) via `pnp_be_full_sg_jacobian_matvec_nm_f64`
    /// vs expanded band multiply on a fixed `v`, then host **GMRES** ([`crate::physics::solvers::krylov_host::gmres_f32_try`]) vs
    /// `solve_newton_correction_full_sg_row_band_via_dense_expand` on a **small** (`N=6`) path graph.
    /// With [`NewtonPnpContext::full_sg_correction_use_gmres`], the host Newton path uses the same matvec
    /// (see `try_solve_pnp_be_newton_chain_host_full_sg_gmres_matches_dense_smoke`).
    #[test]
    fn pnp3d_full_sg_gmres_delta_matches_dense_expand_small_chain() {
        use crate::physics::solvers::krylov_host::gmres_f32_try;

        let n = 6_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-5,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
            d_plus[i] = 0.031 + 0.008 * x.sin();
            d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.06 * x;
            c_minus_n[i] = 1.0 - 0.05 * x * x;
        }
        let g0 = 0.018_f64;
        let g1 = -0.012_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.012 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;

        let r0_fm = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;
        let mut jac_band = vec![0.0_f64; dim * bw_lu];
        newton_fd_jacobian_full_sg_node_major_row_band(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r0_fm,
            &mut jac_band,
        );

        let v_nm: Vec<f64> = (0..dim)
            .map(|i| 0.07_f64 * ((i % 5) as i64 - 2) as f64)
            .collect();
        let mut jv_band = vec![0.0_f64; dim];
        for i in 0..dim {
            let j0 = i.saturating_sub(kl_lu);
            let j1 = (i + ku_lu).min(dim - 1);
            let mut s = 0.0_f64;
            for j in j0..=j1 {
                s += row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i, j) * v_nm[j];
            }
            jv_band[i] = s;
        }
        let mut scratch_u = vec![0.0_f64; dim];
        let mut diff_fm = vec![0.0_f64; dim];
        let mut jv_fd = vec![0.0_f64; dim];
        pnp_be_full_sg_jacobian_matvec_nm_f64(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r0_fm,
            &v_nm,
            &mut scratch_u,
            &mut diff_fm,
            &mut jv_fd,
        );
        let mx_fd: f64 = (0..dim)
            .map(|i| (jv_fd[i] - jv_band[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_fd < 2e-4_f64,
            "matrix-free J·v vs band matvec: max|Δ|={mx_fd:.3e}"
        );

        let mut rhs_nm = vec![0.0_f64; dim];
        pnp_residual_fm_to_nm(&r0_fm, n, &mut rhs_nm);
        for v in rhs_nm.iter_mut() {
            *v = -*v;
        }
        let mut rhs_de = rhs_nm.clone();
        let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
        assert!(
            solve_newton_correction_full_sg_row_band_via_dense_expand(
                &jac_band,
                dim,
                kl_lu,
                ku_lu,
                bw_lu,
                &mut jac_dense_scratch,
                &mut rhs_de,
            ),
            "dense-expand reference solve"
        );

        let b_f32: Vec<f32> = rhs_nm.iter().map(|x| *x as f32).collect();
        let u_fm = u.clone();
        let cpn = c_plus_n.clone();
        let cmn = c_minus_n.clone();
        let eps_c = eps.clone();
        let dp = d_plus.clone();
        let dm = d_minus.clone();
        let r0c = r0_fm.clone();
        let solver_a = std::sync::Arc::new(solver);

        let matvec = {
            let solver_a = std::sync::Arc::clone(&solver_a);
            move |v: &[f32]| -> Result<Vec<f32>, PhysicsError> {
                let mut v_nm_loc = vec![0.0_f64; dim];
                for i in 0..dim {
                    v_nm_loc[i] = v[i] as f64;
                }
                let mut out_nm = vec![0.0_f64; dim];
                let mut scratch = vec![0.0_f64; dim];
                let mut d_fm = vec![0.0_f64; dim];
                pnp_be_full_sg_jacobian_matvec_nm_f64(
                    solver_a.as_ref(),
                    &newton,
                    dt,
                    &u_fm,
                    &cpn,
                    &cmn,
                    &eps_c,
                    &dp,
                    &dm,
                    g0,
                    g1,
                    &r0c,
                    &v_nm_loc,
                    &mut scratch,
                    &mut d_fm,
                    &mut out_nm,
                );
                Ok(out_nm.iter().map(|x| *x as f32).collect())
            }
        };

        let x_g = gmres_f32_try(matvec, &b_f32, dim, dim, 1e-5_f32).expect("GMRES solve");

        let mx_sol: f64 = (0..dim)
            .map(|i| (x_g[i] as f64 - rhs_de[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_sol < 5e-3_f64,
            "GMRES δ vs dense-expand δ: max|Δ|={mx_sol:.3e}"
        );
    }

    /// Same stack as [`pnp3d_full_sg_gmres_delta_matches_dense_expand_small_chain`] on **`N=17`**
    /// (`dim=51`): larger matrix-free + GMRES smoke vs band dense-expand reference.
    #[test]
    fn pnp3d_full_sg_gmres_delta_matches_dense_expand_chain_n17() {
        use crate::physics::solvers::krylov_host::gmres_f32_try;

        let n = 17_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-5,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
            d_plus[i] = 0.031 + 0.008 * x.sin();
            d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.06 * x;
            c_minus_n[i] = 1.0 - 0.05 * x * x;
        }
        let g0 = 0.018_f64;
        let g1 = -0.012_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.012 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;

        let r0_fm = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;
        let mut jac_band = vec![0.0_f64; dim * bw_lu];
        newton_fd_jacobian_full_sg_node_major_row_band(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r0_fm,
            &mut jac_band,
        );

        let v_nm: Vec<f64> = (0..dim)
            .map(|i| 0.05_f64 * (((i * 7) % 11) as f64 - 5.0))
            .collect();
        let mut jv_band = vec![0.0_f64; dim];
        for i in 0..dim {
            let j0 = i.saturating_sub(kl_lu);
            let j1 = (i + ku_lu).min(dim - 1);
            let mut s = 0.0_f64;
            for j in j0..=j1 {
                s += row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i, j) * v_nm[j];
            }
            jv_band[i] = s;
        }
        let mut scratch_u = vec![0.0_f64; dim];
        let mut diff_fm = vec![0.0_f64; dim];
        let mut jv_fd = vec![0.0_f64; dim];
        pnp_be_full_sg_jacobian_matvec_nm_f64(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r0_fm,
            &v_nm,
            &mut scratch_u,
            &mut diff_fm,
            &mut jv_fd,
        );
        let mx_fd: f64 = (0..dim)
            .map(|i| (jv_fd[i] - jv_band[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_fd < 5e-3_f64,
            "matrix-free J·v vs band matvec (N=17): max|Δ|={mx_fd:.3e}"
        );

        let mut rhs_nm = vec![0.0_f64; dim];
        pnp_residual_fm_to_nm(&r0_fm, n, &mut rhs_nm);
        for v in rhs_nm.iter_mut() {
            *v = -*v;
        }
        let mut rhs_de = rhs_nm.clone();
        let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
        assert!(
            solve_newton_correction_full_sg_row_band_via_dense_expand(
                &jac_band,
                dim,
                kl_lu,
                ku_lu,
                bw_lu,
                &mut jac_dense_scratch,
                &mut rhs_de,
            ),
            "dense-expand reference solve N=17"
        );

        let b_f32: Vec<f32> = rhs_nm.iter().map(|x| *x as f32).collect();
        let u_fm = u.clone();
        let cpn = c_plus_n.clone();
        let cmn = c_minus_n.clone();
        let eps_c = eps.clone();
        let dp = d_plus.clone();
        let dm = d_minus.clone();
        let r0c = r0_fm.clone();
        let solver_a = std::sync::Arc::new(solver);

        let matvec = {
            let solver_a = std::sync::Arc::clone(&solver_a);
            move |v: &[f32]| -> Result<Vec<f32>, PhysicsError> {
                let mut v_nm_loc = vec![0.0_f64; dim];
                for i in 0..dim {
                    v_nm_loc[i] = v[i] as f64;
                }
                let mut out_nm = vec![0.0_f64; dim];
                let mut scratch = vec![0.0_f64; dim];
                let mut d_fm = vec![0.0_f64; dim];
                pnp_be_full_sg_jacobian_matvec_nm_f64(
                    solver_a.as_ref(),
                    &newton,
                    dt,
                    &u_fm,
                    &cpn,
                    &cmn,
                    &eps_c,
                    &dp,
                    &dm,
                    g0,
                    g1,
                    &r0c,
                    &v_nm_loc,
                    &mut scratch,
                    &mut d_fm,
                    &mut out_nm,
                );
                Ok(out_nm.iter().map(|x| *x as f32).collect())
            }
        };

        let max_iter = (dim + 120).min(512);
        let x_g = gmres_f32_try(matvec, &b_f32, dim, max_iter, 5e-4_f32).expect("GMRES solve N=17");

        let mx_sol: f64 = (0..dim)
            .map(|i| (x_g[i] as f64 - rhs_de[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            mx_sol < 1.2e-2_f64,
            "GMRES δ vs dense-expand δ (N=17): max|Δ|={mx_sol:.3e}"
        );
    }

    /// Full host Newton: [`NewtonPnpContext::full_sg_correction_use_gmres`] matches dense-expand
    /// correction path on a small chain (tensor I/O).
    #[test]
    fn try_solve_pnp_be_newton_chain_host_full_sg_gmres_matches_dense_smoke() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 11_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let newton_dense = NewtonPnpContext {
            max_newton_iters: 50,
            residual_tol_l2: 1e-10,
            linearize_sg_fickian: false,
            full_sg_correction_use_gmres: false,
            ..Default::default()
        };
        let newton_gmres = NewtonPnpContext {
            full_sg_correction_use_gmres: true,
            ..newton_dense
        };
        let dt = 1e-7_f32;
        let out_d = try_solve_pnp_be_newton_chain_host(
            &solver,
            &newton_dense,
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        )
        .expect("dense full-SG Newton");
        let out_g = try_solve_pnp_be_newton_chain_host(
            &solver,
            &newton_gmres,
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        )
        .expect("GMRES full-SG Newton");
        let pd = out_d.0.into_data().value;
        let pg = out_g.0.into_data().value;
        let cd = out_d.1.into_data().value;
        let cg = out_g.1.into_data().value;
        let mut mx = 0.0_f32;
        for i in 0..n {
            mx = mx.max((pd[i] - pg[i]).abs());
        }
        for i in 0..(n * 2) {
            mx = mx.max((cd[i] - cg[i]).abs());
        }
        assert!(
            mx < 2e-3_f32,
            "dense vs GMRES Newton export: max|Δ|={mx:.3e}"
        );
    }

    #[test]
    fn try_solve_host_tensor_path_converges() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 9_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let dt: f32 = 1e-7;
        let dt64 = dt as f64;
        let newton = NewtonPnpContext {
            max_newton_iters: 50,
            residual_tol_l2: 1e-11,
            linearize_sg_fickian: true,
            ..Default::default()
        };
        let out = try_solve_pnp_be_newton_chain_host(
            &solver,
            &newton,
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (phi_t, c_t) = out.expect("tensor host Newton returned None");
        let g0 = phi_n.clone().into_data().value[0] as f64;
        let g1 = phi_n.clone().into_data().value[n - 1] as f64;
        let mut phi_v = vec![0.0_f64; n];
        let mut cp = vec![0.0_f64; n];
        let mut cm = vec![0.0_f64; n];
        let ph = phi_t.into_data().value;
        let ch = c_t.into_data().value;
        for i in 0..n {
            phi_v[i] = ph[i] as f64;
            cp[i] = ch[i * 2] as f64;
            cm[i] = ch[i * 2 + 1] as f64;
        }
        let mut cpn = vec![0.0_f64; n];
        let mut cmn = vec![0.0_f64; n];
        let cnh = c_n.into_data().value;
        for i in 0..n {
            cpn[i] = cnh[i * 2] as f64;
            cmn[i] = cnh[i * 2 + 1] as f64;
        }
        let mut epsv = vec![0.0_f64; n];
        let mut dp = vec![0.0_f64; n];
        let mut dm = vec![0.0_f64; n];
        let eh = eps.into_data().value;
        let dh = d.into_data().value;
        for i in 0..n {
            epsv[i] = eh[i] as f64;
            dp[i] = dh[i * 2] as f64;
            dm[i] = dh[i * 2 + 1] as f64;
        }
        let r = pnp_be_residual_vector_f64(
            &solver, &newton, dt64, &phi_v, &cp, &cm, &cpn, &cmn, &epsv, &dp, &dm, g0, g1,
        );
        let nrf = vec_l2(&r);
        // Re-evaluating R on f32 tensors amplifies the (c−cⁿ)/Δt term when Δt is tiny; the host
        // solve already enforced ‖R‖₂ ≪ 1e-6 in f64 before export.
        assert!(nrf < 1e-3_f64, "post-f32 export BE residual, nrf={nrf:.3e}");
    }

    /// Full SG (`linearize_sg_fickian: false`): band FD Jacobian + dense-expand Newton host path converges on the same
    /// small-chain tensor harness as [`try_solve_host_tensor_path_converges`].
    #[test]
    fn try_solve_host_tensor_full_sg_banded_newton_converges() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 9_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let dt: f32 = 1e-7;
        let dt64 = dt as f64;
        let newton = NewtonPnpContext {
            max_newton_iters: 60,
            residual_tol_l2: 1e-11,
            linearize_sg_fickian: false,
            ..Default::default()
        };
        let out = try_solve_pnp_be_newton_chain_host(
            &solver,
            &newton,
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (phi_t, c_t) = out.expect("full-SG band Newton host returned None");
        let g0 = phi_n.clone().into_data().value[0] as f64;
        let g1 = phi_n.clone().into_data().value[n - 1] as f64;
        let mut phi_v = vec![0.0_f64; n];
        let mut cp = vec![0.0_f64; n];
        let mut cm = vec![0.0_f64; n];
        let ph = phi_t.into_data().value;
        let ch = c_t.into_data().value;
        for i in 0..n {
            phi_v[i] = ph[i] as f64;
            cp[i] = ch[i * 2] as f64;
            cm[i] = ch[i * 2 + 1] as f64;
        }
        let mut cpn = vec![0.0_f64; n];
        let mut cmn = vec![0.0_f64; n];
        let cnh = c_n.into_data().value;
        for i in 0..n {
            cpn[i] = cnh[i * 2] as f64;
            cmn[i] = cnh[i * 2 + 1] as f64;
        }
        let mut epsv = vec![0.0_f64; n];
        let mut dp = vec![0.0_f64; n];
        let mut dm = vec![0.0_f64; n];
        let eh = eps.into_data().value;
        let dh = d.into_data().value;
        for i in 0..n {
            epsv[i] = eh[i] as f64;
            dp[i] = dh[i * 2] as f64;
            dm[i] = dh[i * 2 + 1] as f64;
        }
        let r = pnp_be_residual_vector_f64(
            &solver, &newton, dt64, &phi_v, &cp, &cm, &cpn, &cmn, &epsv, &dp, &dm, g0, g1,
        );
        let nrf = vec_l2(&r);
        assert!(
            nrf < 1e-3_f64,
            "post-f32 export BE residual (full SG), nrf={nrf:.3e}"
        );
    }

    /// [`NewtonPnpContext::full_sg_frozen_jacobian_inner_iters`] **`>1`**: extra inners reuse a frozen band Jacobian
    /// (band LU each inner) on the same small-chain harness; must still return a finite low-residual state.
    #[test]
    fn try_solve_full_sg_frozen_jacobian_inner_iters_converges() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 9_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let dt: f32 = 1e-7;
        let dt64 = dt as f64;
        let newton = NewtonPnpContext {
            max_newton_iters: 50,
            residual_tol_l2: 1e-11,
            linearize_sg_fickian: false,
            full_sg_frozen_jacobian_inner_iters: 4,
            ..Default::default()
        };
        let out = try_solve_pnp_be_newton_chain_host(
            &solver,
            &newton,
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (phi_t, c_t) = out.expect("full-SG frozen-inner Newton should succeed");
        let g0 = phi_n.clone().into_data().value[0] as f64;
        let g1 = phi_n.clone().into_data().value[n - 1] as f64;
        let mut phi_v = vec![0.0_f64; n];
        let mut cp = vec![0.0_f64; n];
        let mut cm = vec![0.0_f64; n];
        let ph = phi_t.into_data().value;
        let ch = c_t.into_data().value;
        for i in 0..n {
            phi_v[i] = ph[i] as f64;
            cp[i] = ch[i * 2] as f64;
            cm[i] = ch[i * 2 + 1] as f64;
        }
        let mut cpn = vec![0.0_f64; n];
        let mut cmn = vec![0.0_f64; n];
        let cnh = c_n.into_data().value;
        for i in 0..n {
            cpn[i] = cnh[i * 2] as f64;
            cmn[i] = cnh[i * 2 + 1] as f64;
        }
        let mut epsv = vec![0.0_f64; n];
        let mut dp = vec![0.0_f64; n];
        let mut dm = vec![0.0_f64; n];
        let eh = eps.into_data().value;
        let dh = d.into_data().value;
        for i in 0..n {
            epsv[i] = eh[i] as f64;
            dp[i] = dh[i * 2] as f64;
            dm[i] = dh[i * 2 + 1] as f64;
        }
        let r = pnp_be_residual_vector_f64(
            &solver, &newton, dt64, &phi_v, &cp, &cm, &cpn, &cmn, &epsv, &dp, &dm, g0, g1,
        );
        let nrf = vec_l2(&r);
        assert!(
            nrf < 1e-3_f64,
            "post-f32 export BE residual (full SG frozen-inner), nrf={nrf:.3e}"
        );
    }

    /// Regression: dense Newton must satisfy the **linear model equations** on tensor-extracted data.
    #[test]
    fn linearized_newton_single_step_satisfies_linear_model_on_tensor_layout() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 9_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps_t = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d_t = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: true,
            ..Default::default()
        };
        let dt: f32 = 1e-7;
        let res_chain = pnp_backward_euler_residual_l2_chain_host_f64(
            &solver, &newton, dt, &phi_n, &c_n, &c_n, &edges_b1, &eps_t, &d_t,
        )
        .expect("BE residual on chain with edges_b1");
        assert!(res_chain.is_finite(), "BE residual L2={res_chain}");
        let dt64 = dt as f64;
        let phi_h = phi_n.into_data().value;
        let c_h = c_n.into_data().value;
        let eps_h = eps_t.into_data().value;
        let d_h = d_t.into_data().value;
        let g0 = phi_h[0] as f64;
        let g1 = phi_h[n - 1] as f64;
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            c_plus_n[i] = c_h[i * 2] as f64;
            c_minus_n[i] = c_h[i * 2 + 1] as f64;
            eps[i] = eps_h[i] as f64;
            d_plus[i] = d_h[i * 2] as f64;
            d_minus[i] = d_h[i * 2 + 1] as f64;
        }
        let f = solver.faraday_const as f64;
        let h_inv = 1.0_f64 / solver.mesh_spacing as f64;
        let mut rho_net = vec![0.0_f64; n];
        for i in 0..n {
            rho_net[i] = f * (c_plus_n[i] - c_minus_n[i]);
        }
        let mut phi = vec![0.0_f64; n];
        poisson_chain_net_charge_variable_eps_thomas_f64(
            n,
            g0,
            g1,
            &eps,
            &rho_net,
            (solver.mesh_spacing as f64).powi(2),
            &mut phi,
        );
        let mut u = vec![0.0_f64; 3 * n];
        u[0..n].copy_from_slice(&phi);
        u[n..2 * n].copy_from_slice(&c_plus_n);
        u[2 * n..3 * n].copy_from_slice(&c_minus_n);
        let dim = 3 * n;
        let r = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt64,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let mut jac = vec![0.0_f64; dim * dim];
        fill_jacobian_linearized_sg_fickian(
            &mut jac, dim, n, dt64, f, &eps, &d_plus, &d_minus, h_inv,
        );
        let mut x: Vec<f64> = r.iter().map(|v| -v).collect();
        let mut a = jac.clone();
        assert!(solve_dense_linear(dim, &mut a, &mut x), "solve");
        let mut mx = vec![0.0_f64; dim];
        for i in 0..dim {
            let mut s = 0.0_f64;
            for j in 0..dim {
                s += jac[i * dim + j] * x[j];
            }
            mx[i] = s;
        }
        let lin_err: f64 = (0..dim)
            .map(|i| (mx[i] + r[i]).abs())
            .fold(0.0_f64, f64::max);
        assert!(
            lin_err < 1e-8_f64,
            "expected J*delta = -R (linear model), max|Jx+R|={lin_err:.3e}"
        );
    }

    #[test]
    fn solve_pnp_step_dispatch_defaults_match_solve_pnp_step() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 11_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let c = Tensor::<B, 3>::full([1, n, 2], 1.0_f32, &dev);
        let phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver::default();
        let dt = 1e-4_f32;
        let (p0, c0) = solver.solve_pnp_step(
            dt,
            phi.clone(),
            c.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (p1, c1) = solver.solve_pnp_step_dispatch(dt, phi, c, edges, eps, d);
        assert!(
            tensor1_bool(p0.sub(p1).abs().lower_elem(1e-6_f32).all())
                && tensor1_bool(c0.sub(c1).abs().lower_elem(1e-6_f32).all()),
            "dispatch default should equal explicit path"
        );
    }

    #[test]
    fn solve_pnp_step_dispatch_implicit_config_matches_try_solve_chain() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::default();
        let n = 9_usize;
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), &dev);
        let mut c_flat = vec![0.0_f32; n * 2];
        for i in 0..n {
            let x = i as f32 / (n - 1) as f32;
            c_flat[i * 2] = 1.0 + 0.02 * x;
            c_flat[i * 2 + 1] = 1.0 - 0.02 * x;
        }
        let c_n = Tensor::<B, 3>::from_data(Data::new(c_flat, Shape::new([1, n, 2])), &dev);
        let mut ph = vec![0.0_f32; n];
        ph[0] = 0.015;
        ph[n - 1] = 0.0;
        let phi_n = Tensor::<B, 3>::from_data(Data::new(ph, Shape::new([1, n, 1])), &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let newton = NewtonPnpContext {
            max_newton_iters: 50,
            residual_tol_l2: 1e-11,
            linearize_sg_fickian: true,
            ..Default::default()
        };
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0e9_f32,
            mesh_spacing: 1.0_f32,
            pnp_implicit_newton_chain: Some(newton),
            ..Default::default()
        };
        let dt: f32 = 1e-7;
        let (phi_d, c_d) = solver.solve_pnp_step_dispatch(
            dt,
            phi_n.clone(),
            c_n.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (phi_t, c_t) = solver
            .try_solve_pnp_backward_euler_newton_chain(&newton, dt, phi_n, c_n, edges, eps, d)
            .expect("try_solve chain");
        assert!(
            tensor1_bool(phi_d.sub(phi_t).abs().lower_elem(1e-5_f32).all())
                && tensor1_bool(c_d.sub(c_t).abs().lower_elem(1e-5_f32).all()),
            "dispatch implicit branch should match try_solve"
        );
    }

    /// **Chain full-SG (`linearize_sg_fickian: false`)** at **`N=256`**: assemble the band Jacobian, time
    /// **dense expand + Gauss** vs the **`solve_newton_correction_full_sg_row_band_via_band_lu`** entry point
    /// (in-place band LU with the **static** [`PNP_CHAIN_FULL_SG_JAC_KL_LU`] envelope — **`max|δ_lu−δ_de|`** is
    /// **not** expected to match dense at this **`dim`**) and **print** timings.
    /// Run with **`--release`** for meaningful assembly/solve times:
    ///
    /// ```text
    /// cargo test -p umst-manifold --features electrochemistry-pnp,solver-experimental \
    ///   full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity \
    ///   --release -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "slow — full-SG band FD Jacobian assembly at N=256; prints assembly/solve wall-clock (band-LU uses static envelope; δ may differ from dense-expand)"]
    fn full_sg_chain_n256_band_lu_vs_dense_expand_wall_clock_and_residual_parity() {
        use std::time::Instant;

        let n = 256_usize;
        let solver = ElectroChemicalSolver {
            faraday_const: 1.0_f32,
            gas_const: 1.0_f32,
            mesh_spacing: 0.11_f32,
            ..Default::default()
        };
        let newton = NewtonPnpContext {
            linearize_sg_fickian: false,
            fd_step: 1e-6,
            max_chain_nodes: 512,
            ..Default::default()
        };
        let dt = 1.7e-4_f64;
        let mut eps = vec![0.0_f64; n];
        let mut d_plus = vec![0.0_f64; n];
        let mut d_minus = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            eps[i] = 1.0 + 0.09 * (x - 0.4).powi(2);
            d_plus[i] = 0.031 + 0.008 * x.sin();
            d_minus[i] = 0.029 + 0.007 * (x * 1.9).cos();
        }
        let mut c_plus_n = vec![0.0_f64; n];
        let mut c_minus_n = vec![0.0_f64; n];
        for i in 0..n {
            let x = i as f64 / (n - 1) as f64;
            c_plus_n[i] = 1.0 + 0.06 * x;
            c_minus_n[i] = 1.0 - 0.05 * x * x;
        }
        let g0 = 0.018_f64;
        let g1 = -0.012_f64;
        let mut u = vec![0.0_f64; 3 * n];
        for i in 0..n {
            u[i] = 0.012 * (i as f64 / n as f64).sin();
            u[n + i] = c_plus_n[i] + 0.003 * ((i % 4) as f64);
            u[2 * n + i] = c_minus_n[i] - 0.002 * ((i % 3) as f64);
        }
        u[0] = g0;
        u[n - 1] = g1;
        let r = pnp_be_residual_vector_f64(
            &solver,
            &newton,
            dt,
            &u[0..n],
            &u[n..2 * n],
            &u[2 * n..3 * n],
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
        );
        let dim = 3 * n;
        let kl_lu = PNP_CHAIN_FULL_SG_JAC_KL_LU;
        let ku_lu = PNP_CHAIN_FULL_SG_JAC_KU_LU;
        let bw_lu = PNP_CHAIN_FULL_SG_BW_LU;
        let mut jac_band = vec![0.0_f64; dim * bw_lu];
        let t_asm = Instant::now();
        newton_fd_jacobian_full_sg_node_major_row_band(
            &solver,
            &newton,
            dt,
            &u,
            &c_plus_n,
            &c_minus_n,
            &eps,
            &d_plus,
            &d_minus,
            g0,
            g1,
            &r,
            &mut jac_band,
        );
        let asm_s = t_asm.elapsed().as_secs_f64();

        let mut jac_nm_dense = vec![0.0_f64; dim * dim];
        for i_nm in 0..dim {
            for j_nm in 0..dim {
                jac_nm_dense[i_nm * dim + j_nm] =
                    row_band_get(&jac_band, kl_lu, ku_lu, bw_lu, i_nm, j_nm);
            }
        }

        let rhs0 = {
            let mut rhs_nm = vec![0.0_f64; dim];
            pnp_residual_fm_to_nm(&r, n, &mut rhs_nm);
            for v in rhs_nm.iter_mut() {
                *v = -*v;
            }
            rhs_nm
        };

        let mut rhs_lu = rhs0.clone();
        let mut jac_lu_fact = vec![0.0_f64; dim * bw_lu];
        let mut lu_swaps: Vec<(usize, usize)> = Vec::new();
        let t_lu = Instant::now();
        let ok_lu = solve_newton_correction_full_sg_row_band_via_band_lu(
            &jac_band,
            dim,
            kl_lu,
            ku_lu,
            bw_lu,
            &mut jac_lu_fact,
            &mut lu_swaps,
            &mut rhs_lu,
        );
        let lu_s = t_lu.elapsed().as_secs_f64();
        assert!(ok_lu, "band LU linear solve");

        let mut rhs_de = rhs0.clone();
        let mut jac_dense_scratch = vec![0.0_f64; dim * dim];
        let t_de = Instant::now();
        let ok_de = solve_newton_correction_full_sg_row_band_via_dense_expand(
            &jac_band,
            dim,
            kl_lu,
            ku_lu,
            bw_lu,
            &mut jac_dense_scratch,
            &mut rhs_de,
        );
        let de_s = t_de.elapsed().as_secs_f64();
        assert!(ok_de, "dense-expand linear solve");

        let max_nm: f64 = (0..dim)
            .map(|i| (rhs_lu[i] - rhs_de[i]).abs())
            .fold(0.0_f64, f64::max);

        let lin_res = |dx_nm: &[f64]| -> f64 {
            let mut mx = vec![0.0_f64; dim];
            for i in 0..dim {
                let mut s = 0.0_f64;
                for j in 0..dim {
                    s += jac_nm_dense[i * dim + j] * dx_nm[j];
                }
                mx[i] = s;
            }
            (0..dim)
                .map(|i| (mx[i] - rhs0[i]).abs())
                .fold(0.0_f64, f64::max)
        };
        let err_lu = lin_res(&rhs_lu);
        let err_de = lin_res(&rhs_de);
        eprintln!(
            "full-SG N={n}: Jacobian assembly {asm_s:.3}s; band-LU solve {lu_s:.4}s; dense-expand solve {de_s:.4}s (dim={dim}); max|δ_lu-δ_de|={max_nm:.3e}; max|Jδ+R|_lu={err_lu:.3e} _de={err_de:.3e}"
        );
        assert!(
            err_de < 1e-6_f64,
            "dense-expand linear model max|J δ − (−R)|={err_de:.3e}"
        );
    }

    /// **Smallest non-chain tree** (4 nodes, 3 edges): Jacobi–PCG on [`crate::physics::laplacian::TopologicalLaplacian`] reduces
    /// \(\|\mathcal{L}\phi-\text{rhs}\|_2/\|\text{rhs}\|_2\) for a manufactured harmonic \(\phi^\star\).
    #[test]
    fn poisson_graph_pcg_four_node_star_reduces_laplacian_residual() {
        use burn::tensor::{Data, Int, Shape, Tensor};
        use burn_ndarray::{NdArray, NdArrayDevice};
        type B = NdArray<f32>;
        let dev = NdArrayDevice::Cpu;
        let n = 4_usize;
        let batch = 1_usize;
        let ev = vec![
            0_i64, 0, 0, //
            1, 2, 3,
        ];
        let edges = Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, 3])), &dev);
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let phi_star = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.0_f32, 1.0_f32, -1.0_f32, 0.0_f32],
                Shape::new([batch, n, 1]),
            ),
            &dev,
        );
        let rhs =
            TopologicalLaplacian::scalar_laplacian(phi_star.clone(), edges.clone(), damage.clone());
        let phi0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let phi_sol = super::poisson_graph_uniform_laplacian_jacobi_pcg(
            phi0,
            rhs.clone(),
            edges.clone(),
            damage.clone(),
            batch,
            n,
        );
        let lap_sol =
            TopologicalLaplacian::scalar_laplacian(phi_sol.clone(), edges.clone(), damage.clone());
        let rn = lap_sol.sub(rhs.clone()).powf_scalar(2.0).sum().sqrt();
        let bn = rhs.powf_scalar(2.0).sum().sqrt().clamp_min(1e-30_f32);
        let rel = rn.div(bn);
        assert!(
            tensor1_bool(rel.lower_elem(2e-3_f32).all()),
            "PCG relative Laplacian residual too large"
        );
        let mean_star = phi_star.clone().sum_dim(1).div_scalar(n as f32);
        let mean_sol = phi_sol.clone().sum_dim(1).div_scalar(n as f32);
        let d = phi_sol.sub(phi_star.clone()).sub(mean_sol.sub(mean_star));
        assert!(
            tensor1_bool(d.abs().lower_elem(5e-2_f32).all()),
            "PCG solution should match manufactured φ up to gauge"
        );
    }
}

#[cfg(all(test, feature = "electrochemistry-mvp"))]
mod physics_idempotency_tests {
    use super::*;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn path_chain_edges(dev: &NdArrayDevice, n: usize) -> Tensor<B, 2, Int> {
        let e = n - 1;
        let mut ev = Vec::with_capacity(2 * e);
        for i in 0..e {
            ev.push(i as i64);
        }
        for i in 0..e {
            ev.push((i + 1) as i64);
        }
        Tensor::<B, 2, Int>::from_data(Data::new(ev, Shape::new([2, e])), dev)
    }

    /// FP Manifesto §6: uniform electroneutral concentrations with zero potential are a split-PNP
    /// fixed point — re-applying [`ElectroChemicalSolver::solve_pnp_step`] must not drift.
    #[test]
    fn solve_pnp_step_idempotent_on_uniform_electroneutral_equilibrium() {
        let dev = NdArrayDevice::default();
        let n = 11_usize;
        let edges = path_chain_edges(&dev, n);
        let phi = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let c = Tensor::<B, 3>::full([1, n, 2], 1.0_f32, &dev);
        let eps = Tensor::<B, 3>::ones([1, n, 1], &dev);
        let d = Tensor::<B, 3>::full([1, n, 2], 0.04_f32, &dev);
        let solver = ElectroChemicalSolver::default();
        let dt = 1e-4_f32;

        let (phi1, c1) = solver.solve_pnp_step(
            dt,
            phi.clone(),
            c.clone(),
            edges.clone(),
            eps.clone(),
            d.clone(),
        );
        let (phi2, c2) = solver.solve_pnp_step(dt, phi1.clone(), c1.clone(), edges, eps, d);

        let tol = 1e-6_f32;
        assert!(
            tensor1_bool(phi1.clone().sub(phi2).abs().lower_elem(tol).all())
                && tensor1_bool(c1.clone().sub(c2).abs().lower_elem(tol).all()),
            "re-application on equilibrated PNP state must not drift"
        );
        assert!(
            tensor1_bool(phi1.sub(phi).abs().lower_elem(tol).all())
                && tensor1_bool(c1.sub(c).abs().lower_elem(tol).all()),
            "uniform electroneutral equilibrium must be a fixed point"
        );
    }
}

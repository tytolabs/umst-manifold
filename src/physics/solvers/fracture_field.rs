// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! AT2 variational phase-field fracture (Phase 2) — **minimal working path** behind cargo feature
//! **`fracture-at2`** (included in **`solver-experimental`** / **`solver-research`**).
//!
//! - **Tensile strain energy (spectral)**: eigenvalues \(\lambda_i\) of the symmetric small
//!   strain tensor \(\varepsilon\). Positive spectral part
//!   \(\langle\varepsilon\rangle_+ = \sum_i \langle\lambda_i\rangle_+ \, \mathbf{n}_i\otimes\mathbf{n}_i\)
//!   (Macaulay \(\langle x\rangle_+ = \max(0,x)\)). We use the scalar surrogate
//!   \(\psi^+ = \tfrac{1}{2}\,\|\langle\varepsilon\rangle_+\|_F^2
//!   = \tfrac{1}{2}\sum_i \langle\lambda_i\rangle_+^2\), i.e. half the squared Frobenius norm of
//!   the tensile spectral projection (identity stiffness in the principal frame — document any
//!   rescaling if you later couple \(\lambda,\mu\) from the mechanical kernel).
//! - **Eigenvalues on tensor**: Burn 0.13 has no public `acos` / symmetric eigendecomposition on
//!   `Tensor`. We use **fixed-step cyclic Jacobi** diagonalization (only `sqrt`, `add`, `mul`,
//!   `sign`, `mask_where`, …) on each \(3\times3\) block, so the diagonals converge to the
//!   eigenvalue multiset. Same \(\psi^+\) as from sorted \(\lambda_i\) because it is symmetric in
//!   the three eigenvalues.
//! - Degradation (for documentation / future tight coupling with mechanics):
//!   \(g(d) = (1-d)^2 + \eta\).
//! - AT2-style nodal field: `Gc/l · d − Gc · l · Δ d ≈ 2(1-d) ψ⁺` with `Δ` from
//!   [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`] on `edges_b1`.
//! - Irreversibility `max(d_old, d_{trial})`, then clamp to `[0, 1]`.
//!
//! ## Inner damage relaxation (Jacobi + graph Laplacian)
//!
//! A plain Jacobi step on \((Gc/l - Gc\,l\,\Delta)\,d \approx 2(1-d)\psi^+\) can **checkerboard**
//! on 1D chains (odd/even mode), which shows up as alternating **global sums** in `f32` smoke tests.
//! We combine **smaller** \(\omega\), **node-parity red–black** half-steps, a **`[0,1]` clamp once
//! per outer pair**, and an **odd** number of outer passes on short chains (see `DAMAGE_RELAXATION_ITERS`)
//! so a terminal near-checkerboard state does not cancel the integrated damage to **0** in `f32`.
//!
//! Default builds (no `fracture-at2`): [`PhaseFieldFractureSolver::update_damage`] is a **documented
//! no-op** — returns `damage` unchanged so `cargo test` stays green.
//!
//! ## Intended staggered coupling (`fracture-at2`)
//!
//! Each call to [`PhaseFieldFractureSolver::update_damage`] treats the supplied strain **ε** as
//! fixed for that relaxation. In a coupled elasticity–damage problem the standard **staggered**
//! (operator-split) outer scheme is:
//!
//! ```text
//! Initialize damage d^0.
//! For k = 0 … K−1 (or until a coupled residual tolerance):
//!   (1) Elasticity (hook): fix d^k; solve for displacement u^{k+1}
//!       — e.g. [`crate::physics::mechanics::VectorMechanicsSolver`] with stiffness degraded by g(d^k) —
//!       and recover symmetric small strain ε^{k+1} per node.
//!   (2) Damage: d^{k+1} ← AT2_inner( ε^{k+1}, d^k, Gc, l, edges )
//!       — implemented here as the spectral ψ⁺ relaxation in [`PhaseFieldFractureSolver::update_damage`].
//! ```
//!
//! This module implements **step (2) only**. Step (1) is a **call-site / orchestrator** concern
//! ([`crate::physics::solvers::ThmcSolver`] supplies one such wiring). Operator-split **outer**
//! damage passes are exposed as [`PhaseFieldFractureSolver::update_damage_staggered`].
//!
//! ## Known limitations (v0.4 / Track C)
//!
//! - **No in-module elasticity:** [`PhaseFieldFractureSolver::update_damage`] does not call mechanics;
//!   packaged **post-mechanics** strain for orchestrators is [`strain_tensor_for_fracture_after_mechanics`]
//!   / [`strain_tensor_from_bar_network_displacement`] (feature `fracture-at2`); full staggered outer
//!   loops outside THMC still require callers to refresh **ε** after each equilibrium solve when they
//!   do not use [`crate::physics::solvers::ThmcSolver::step`].
//! - **Stiffness degradation** \(g(d) = (1-d)^2 + \eta\) is documented for future mechanical
//!   coupling but is **not** consumed inside [`PhaseFieldFractureSolver::update_damage`], which
//!   updates **d** from a fixed **ε** snapshot only.
//! - [`PhaseFieldFractureSolver::update_damage_staggered`] does **not** run mechanics solves; it
//!   only composes multiple [`PhaseFieldFractureSolver::update_damage`] calls with a **strain
//!   provider** `FnMut(&Tensor<B,3>) -> Tensor<B,4>` so call sites can inject refreshed **ε(d)**.
//! - **Repo status (one place):** implemented vs multi-\(l_0\) Γ-limit / full staggered deferrals —
//!   `docs/Solver-Status.md` → **DEFERRAL — Fracture** and table row `solvers::fracture_field`.
//!
//! ## `update_damage_staggered` backward compatibility
//!
//! For `outer_iterations == 1`, a provider that **ignores** the current damage and returns the
//! same strain tensor `ε` on every call is equivalent to a single call to
//! [`PhaseFieldFractureSolver::update_damage`] with strain `ε` and initial damage `d0` (bit-for-bit in
//! default builds; with `fracture-at2`, same floating pipeline). For `outer_iterations > 1`, each outer pass feeds the
//! **output** damage of the previous pass into the provider, then runs another full inner
//! relaxation — this is **not** the same as one pass unless **ε** is unchanged and the inner map
//! happens to be idempotent (not assumed).

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "fracture-at2")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "fracture-at2")]
use crate::physics::mechanics::VectorMechanicsSolver;
#[cfg(feature = "fracture-at2")]
use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
#[cfg(feature = "fracture-at2")]
use crate::physics::topology::EdgeTopology;

#[cfg(feature = "fracture-at2")]
use burn::tensor::{Data, Shape};

/// Configuration for the staggered elasticity–damage outer loop in
/// [`PhaseFieldFractureSolver::solve_staggered_with_mechanics`].
#[derive(Clone, Copy, Debug)]
pub struct StaggeredFractureConfig {
    /// Number of outer staggered alternations (`u_k` ↔ `d_k`).
    pub outer_iters: usize,
    /// Inner AT2 relaxation passes per outer iteration; reserved for callers that wrap
    /// [`PhaseFieldFractureSolver::update_damage`] in their own loop. The current implementation
    /// performs one [`PhaseFieldFractureSolver::update_damage`] call per outer pass (which itself
    /// runs `DAMAGE_RELAXATION_ITERS` red–black sweeps); this field is preserved for API stability.
    pub damage_relaxation_passes: usize,
    /// Critical fracture-energy release rate `Gc` (uniform).
    pub gc: f32,
    /// Phase-field length scale `l₀`.
    pub length_scale: f32,
    /// Regularization floor on degraded stiffness `g(d) = (1−d)² + k_reg`.
    pub kappa_reg: f32,
}

/// Maps graph Voigt strain `[B, N, 6]` from [`VectorMechanicsSolver::voigt_strain_from_edge_displacement`]
/// to symmetric small strain `[B, N, 3, 3]` for [`PhaseFieldFractureSolver::update_damage`].
#[cfg(feature = "fracture-at2")]
fn symmetric_strain_tensor_from_graph_voigt6<B: Backend<FloatElem = f32>>(
    eps_v: Tensor<B, 3>,
) -> Tensor<B, 4> {
    let dims = eps_v.dims();
    let batch = dims[0];
    let n = dims[1];
    let exx = eps_v.clone().slice([0..batch, 0..n, 0..1]);
    let eyy = eps_v.clone().slice([0..batch, 0..n, 1..2]);
    let ezz = eps_v.clone().slice([0..batch, 0..n, 2..3]);
    let exy = eps_v.clone().slice([0..batch, 0..n, 3..4]);
    let eyz = eps_v.clone().slice([0..batch, 0..n, 4..5]);
    let exz = eps_v.slice([0..batch, 0..n, 5..6]);
    let row0 = Tensor::cat(vec![exx.clone(), exy.clone(), exz.clone()], 2).unsqueeze_dim::<4>(2);
    let row1 = Tensor::cat(vec![exy.clone(), eyy.clone(), eyz.clone()], 2).unsqueeze_dim::<4>(2);
    let row2 = Tensor::cat(vec![exz, eyz, ezz], 2).unsqueeze_dim::<4>(2);
    Tensor::cat(vec![row0, row1, row2], 2)
}

/// Symmetric small strain `[B, N, 3, 3]` from a **bar-network** displacement field and the same
/// embedded vertex coordinates used in [`VectorMechanicsSolver::voigt_strain_from_edge_displacement`].
///
/// Edge tangents and rest lengths are taken from `coords_n3` and `edges_b1` (via [`EdgeTopology`]);
/// this matches the Track 12 milestone convention when gathers are built from SI `coords_n3`.
#[cfg(feature = "fracture-at2")]
pub fn strain_tensor_from_bar_network_displacement<B: Backend<FloatElem = f32>>(
    u: Tensor<B, 3>,
    coords_n3: Tensor<B, 2>,
    edges_b1: Tensor<B, 2, Int>,
    n_nodes: usize,
) -> Tensor<B, 4> {
    let batch = u.dims()[0];
    let topo = EdgeTopology::new(edges_b1.clone());
    let n_edges = topo.n_edges();
    let coords_b = coords_n3
        .clone()
        .unsqueeze_dim::<3>(0)
        .expand([batch, n_nodes, 3]);
    let src3 = topo.expand_src_gather_indices(batch, 3);
    let tgt3 = topo.expand_tgt_gather_indices(batch, 3);
    let c_src = coords_b.clone().gather(1, src3.clone());
    let c_tgt = coords_b.gather(1, tgt3.clone());
    let delta = c_tgt.sub(c_src);
    let edge_len = delta
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .sqrt()
        .clamp(1e-12_f32, f32::MAX)
        .reshape([batch, n_edges, 1]);
    let edge_unit = delta.div(edge_len.clone());
    let u_src = u.clone().gather(1, src3);
    let u_tgt = u.gather(1, tgt3);
    let edge_disp = u_tgt.sub(u_src);
    let eps_v = VectorMechanicsSolver::voigt_strain_from_edge_displacement(
        edge_disp,
        edge_unit,
        edge_len,
        edges_b1,
        n_nodes,
    );
    symmetric_strain_tensor_from_graph_voigt6(eps_v)
}

/// One bar-network equilibrium solve at fixed `damage`, then symmetric strain `[B, N, 3, 3]` from edge
/// kinematics for [`PhaseFieldFractureSolver::update_damage`] / staggered outer loops.
///
/// Call sites may still supply **pre-expanded** gather indices and edge frames (`src3`, `tgt3`,
/// `edge_unit`, `edge_len`) for API compatibility with Track 12 harnesses; strain is assembled from
/// the equilibrium **`u`** using [`strain_tensor_from_bar_network_displacement`] and the same
/// `coords_n3` / `edges_b1` (re-derived edge geometry from coordinates).
///
/// **Scope:** documented **fresh \(\varepsilon(\mathbf u)\)** after one mechanics solve
/// (see `docs/research/v0.4_track12_staggered_fracture_mechanics.md`). [`ThmcSolver::step`] uses
/// [`strain_tensor_from_bar_network_displacement`] on `state.mechanical.displacement` when SI
/// `node_positions` are present (see `thmc` module docs).
#[cfg(feature = "fracture-at2")]
#[allow(clippy::too_many_arguments)]
pub fn strain_tensor_for_fracture_after_mechanics<B: Backend<FloatElem = f32>>(
    u0: Tensor<B, 3>,
    coords_n3: Tensor<B, 2>,
    stiffness: Tensor<B, 3>,
    body_force: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    boundary_mask: Tensor<B, 3>,
    cross_section_area: f32,
    cg: &MechanicsInnerLoopConfig,
    _src3: Tensor<B, 3, Int>,
    _tgt3: Tensor<B, 3, Int>,
    _edge_unit: Tensor<B, 3>,
    _edge_len: Tensor<B, 3>,
    n_nodes: usize,
) -> Tensor<B, 4> {
    let (u, _) = VectorMechanicsSolver::solve_equilibrium(
        u0,
        coords_n3.clone(),
        stiffness,
        body_force,
        edges_b1.clone(),
        damage,
        boundary_mask,
        cross_section_area,
        cg,
    );
    strain_tensor_from_bar_network_displacement(u, coords_n3, edges_b1, n_nodes)
}

#[cfg(feature = "fracture-at2")]
/// Relaxation **outer** passes; each pass is one even-index half-step plus one odd-index half-step.
/// Use an **odd** count on short path graphs: with red–black + per-pass clamp, an **even** total
/// can align the terminal iterate with a near-checkerboard mode whose **global sum** underflows to
/// 0 in `f32` while nodal values are not converged.
const DAMAGE_RELAXATION_ITERS: usize = 17;

#[cfg(feature = "fracture-at2")]
/// Under-relaxation \(\omega\) on **each** parity half-step.
const RELAXATION_OMEGA: f32 = 0.055;

#[cfg(feature = "fracture-at2")]
/// Cyclic Jacobi sweeps \((0,1)\to(0,2)\to(1,2)\) per sweep; enough for `f32` diagonal drift \(\ll 10^{-4}\|\varepsilon\|\) in typical strain ranges.
const JACOBI_SWEEPS: usize = 18;

#[cfg(feature = "fracture-at2")]
/// Upper-triangle packing of symmetric strain per node (`[B,N,1]` each).
type SymStrainPackBn1<B> = (
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
);

/// Phase-field length scale \(l\) (AT2 gradient regularization strength).
pub struct PhaseFieldFractureSolver {
    pub length_scale: f32,
}

impl PhaseFieldFractureSolver {
    /// Update continuum damage from strain energy and fracture toughness.
    ///
    /// # Shapes (contract)
    /// - `strain`: `[B, N, 3, 3]` symmetric strain tensor.
    /// - `damage`, `fracture_energy_gc`: `[B, N, 1]`.
    /// - `edges_b1`: `[2, E]`.
    /// - Returns updated `damage` `[B, N, 1]`.
    ///
    /// ## Default builds (**no `fracture-at2`**)
    /// Returns `damage` unchanged (documented no-op / Phase 2 stub for downstream wiring tests).
    ///
    /// ## Feature `fracture-at2` (e.g. `--features solver-experimental`)
    /// Runs the minimal AT2 relaxation documented in this module (spectral tensile \(\psi^+\)).
    #[allow(unused_variables)]
    pub fn update_damage<B: Backend<FloatElem = f32>>(
        &self,
        strain: Tensor<B, 4>,
        damage: Tensor<B, 3>,
        fracture_energy_gc: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
    ) -> Tensor<B, 3> {
        #[cfg(not(feature = "fracture-at2"))]
        {
            damage
        }

        #[cfg(feature = "fracture-at2")]
        {
            update_damage_experimental(self, strain, damage, fracture_energy_gc, edges_b1)
        }
    }

    /// Outer **staggered** damage passes: for each `k` in `0..outer_iterations`, replaces `d` with
    /// the result of [`Self::update_damage`] using `strain = strain_fn(&d)` and the current
    /// `d`, `fracture_energy_gc`, and `edges_b1`.
    ///
    /// **Milestone integration tests** (feature `fracture-at2`): `tests/verification/staggered_ud_loop_milestone`
    /// (analytic vs mechanics strain providers; outer-loop irreversibility + mechanics-side ℓ∞ convergence);
    /// `tests/verification/staggered_fracture_mechanics_chain` (single-outer mechanics wiring smoke).
    ///
    /// Use `outer_iterations == 0` to return `damage` unchanged (no provider call).
    ///
    /// **Backward compatibility:** `outer_iterations == 1` with a provider that returns the
    /// same fixed strain regardless of `d` matches one call to [`Self::update_damage`] with that
    /// strain and the initial damage. See module section **update_damage_staggered backward compatibility**.
    pub fn update_damage_staggered<B, F>(
        &self,
        mut strain_fn: F,
        mut damage: Tensor<B, 3>,
        fracture_energy_gc: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        outer_iterations: usize,
    ) -> Tensor<B, 3>
    where
        B: Backend<FloatElem = f32>,
        F: FnMut(&Tensor<B, 3>) -> Tensor<B, 4>,
    {
        for _ in 0..outer_iterations {
            let strain_k = strain_fn(&damage);
            damage = self.update_damage(
                strain_k,
                damage,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            );
        }
        damage
    }

    /// Staggered elasticity–damage alternation that owns the mechanics solve internally.
    ///
    /// Performs Miehe-style operator splitting:
    /// `u_{k+1} = arg min_u E(u, d_k)` via [`VectorMechanicsSolver::solve_equilibrium`];
    /// `d_{k+1} = arg min_d E(u_{k+1}, d)` via this struct's [`Self::update_damage`].
    ///
    /// formal_anchor: Literature
    /// formal_citation: Miehe, Welschinger, Hofacker 2010 IJNME 83:1273
    /// formal_form: "alternation: u_k = arg min_u E(u, d_k); d_k+1 = arg min_d E(u_k, d)"
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "fracture-at2")]
    pub fn solve_staggered_with_mechanics<B: Backend<FloatElem = f32>>(
        coords_n3: Tensor<B, 2>,
        edges_b1: Tensor<B, 2, Int>,
        body_force: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        rho_node: Tensor<B, 3>,
        e0: f32,
        cross_section_area: f32,
        cg: &MechanicsInnerLoopConfig,
        config: StaggeredFractureConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        let _ = config.damage_relaxation_passes; // see field doc — preserved for API stability.
        let dev = body_force.device();
        let batch = body_force.dims()[0];
        let n = body_force.dims()[1];

        // E_node = E0 * rho^p (p = 1) as `[B,N,1]`; pair with ν = 0.2 for an isotropic bar network.
        let e_node = rho_node.clone().mul_scalar(e0);
        let nu_node = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(0.2_f32);

        // Edge geometry shared across outer passes.
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();
        let coords_b = coords_n3
            .clone()
            .unsqueeze_dim::<3>(0)
            .expand([batch, n, 3]);
        let src3 = topo.expand_src_gather_indices(batch, 3);
        let tgt3 = topo.expand_tgt_gather_indices(batch, 3);
        let c_src = coords_b.clone().gather(1, src3.clone());
        let c_tgt = coords_b.gather(1, tgt3.clone());
        let delta = c_tgt.sub(c_src);
        let edge_len = delta
            .clone()
            .powf_scalar(2.0)
            .sum_dim(2)
            .sqrt()
            .clamp(1e-12, f32::MAX)
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta.div(edge_len.clone());

        let gc_field = Tensor::<B, 3>::ones([batch, n, 1], &dev).mul_scalar(config.gc);
        let solver = PhaseFieldFractureSolver {
            length_scale: config.length_scale,
        };

        let mut d = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let mut u = Tensor::<B, 3>::zeros([batch, n, 3], &dev);

        for _outer in 0..config.outer_iters {
            // Multiplicative degradation g(d) = (1-d)^2 + k_reg applied to per-node E_young.
            // VectorMechanicsSolver also applies its own internal degradation via the `damage`
            // argument; to avoid double counting we pass damage=0 to mechanics and instead bake
            // g(d) into the effective stiffness tensor.
            let one_minus_d = Tensor::<B, 3>::ones_like(&d).sub(d.clone());
            let g_of_d = one_minus_d
                .clone()
                .mul(one_minus_d)
                .add_scalar(config.kappa_reg);
            let e_eff = e_node.clone().mul(g_of_d);
            let stiffness = Tensor::cat(vec![e_eff, nu_node.clone()], 2);

            let zero_damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
            let (u_k, _stress) = VectorMechanicsSolver::solve_equilibrium(
                u.clone(),
                coords_n3.clone(),
                stiffness,
                body_force.clone(),
                edges_b1.clone(),
                zero_damage,
                boundary_mask.clone(),
                cross_section_area,
                cg,
            );
            u = u_k;

            // Per-edge axial strain -> nodal symmetric strain tensor via Voigt scatter.
            let u_src = u.clone().gather(1, src3.clone());
            let u_tgt = u.clone().gather(1, tgt3.clone());
            let edge_disp = u_tgt.sub(u_src);
            let eps_v = VectorMechanicsSolver::voigt_strain_from_edge_displacement(
                edge_disp,
                edge_unit.clone(),
                edge_len.clone(),
                edges_b1.clone(),
                n,
            );
            let strain4 = symmetric_strain_tensor_from_graph_voigt6(eps_v);

            d = solver.update_damage(strain4, d, gc_field.clone(), edges_b1.clone());
        }

        (u, d)
    }
}

#[cfg(feature = "fracture-at2")]
fn node_parity_masks_b_n1<B: Backend<FloatElem = f32>>(
    batch: usize,
    n: usize,
    device: &B::Device,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let mut even = vec![0.0_f32; batch * n];
    for b in 0..batch {
        for j in 0..n {
            even[b * n + j] = if (j & 1) == 0 { 1.0 } else { 0.0 };
        }
    }
    let mask_even = Tensor::from_data(Data::new(even, Shape::new([batch, n, 1])), device);
    let mask_odd = Tensor::<B, 3>::ones_like(&mask_even).sub(mask_even.clone());
    (mask_even, mask_odd)
}

#[cfg(feature = "fracture-at2")]
fn update_damage_experimental<B: Backend<FloatElem = f32>>(
    solver: &PhaseFieldFractureSolver,
    strain: Tensor<B, 4>,
    damage_old: Tensor<B, 3>,
    fracture_energy_gc: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
) -> Tensor<B, 3> {
    let l = solver.length_scale.max(1e-12);
    let gc = fracture_energy_gc.clone().clamp_min(1e-30_f32);

    let psi_plus = tensile_strain_energy_density_spectral_jacobi(strain);

    let [batch, n, _one] = damage_old.dims();
    let (mask_even, mask_odd) = node_parity_masks_b_n1::<B>(batch, n, &damage_old.device());

    let mut d = damage_old.clone();
    for _ in 0..DAMAGE_RELAXATION_ITERS {
        let lap_d = TopologicalLaplacian::scalar_laplacian(d.clone(), edges_b1.clone(), d.clone());
        let one_minus_d = Tensor::<B, 3>::ones_like(&d).sub(d.clone());
        let drive = one_minus_d.mul(psi_plus.clone()).mul_scalar(2.0);
        let lin = gc.clone().div_scalar(l).mul(d.clone());
        let grad_term = gc.clone().mul_scalar(l).mul(lap_d);
        let residual = lin.sub(drive).sub(grad_term);
        d = d.sub(residual.mul_scalar(RELAXATION_OMEGA).mul(mask_even.clone()));

        let lap_d = TopologicalLaplacian::scalar_laplacian(d.clone(), edges_b1.clone(), d.clone());
        let one_minus_d = Tensor::<B, 3>::ones_like(&d).sub(d.clone());
        let drive = one_minus_d.mul(psi_plus.clone()).mul_scalar(2.0);
        let lin = gc.clone().div_scalar(l).mul(d.clone());
        let grad_term = gc.clone().mul_scalar(l).mul(lap_d);
        let residual = lin.sub(drive).sub(grad_term);
        d = d.sub(residual.mul_scalar(RELAXATION_OMEGA).mul(mask_odd.clone()));

        d = d.clamp(0.0_f32, 1.0_f32);
    }

    let out = d.max_pair(damage_old.clone()).clamp(0.0_f32, 1.0_f32);
    let elem_fin = out
        .clone()
        .equal(out.clone())
        .float()
        .mul(out.clone().abs().lower_elem(f32::INFINITY).float())
        .greater_elem(0.5_f32);
    damage_old.mask_where(elem_fin, out)
}

/// Extract upper-triangle entries of symmetric strain, each `[B, N, 1]`.
#[cfg(feature = "fracture-at2")]
fn strain_sym_components_bn1<B: Backend<FloatElem = f32>>(
    strain: Tensor<B, 4>,
) -> SymStrainPackBn1<B> {
    let [b, n, _, _] = strain.dims();
    let reshape_bn1 = [b, n, 1];
    let e00 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 0, 1)
        .reshape(reshape_bn1);
    let e01 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 1, 1)
        .reshape(reshape_bn1);
    let e02 = strain
        .clone()
        .narrow(2, 0, 1)
        .narrow(3, 2, 1)
        .reshape(reshape_bn1);
    let e11 = strain
        .clone()
        .narrow(2, 1, 1)
        .narrow(3, 1, 1)
        .reshape(reshape_bn1);
    let e12 = strain
        .clone()
        .narrow(2, 1, 1)
        .narrow(3, 2, 1)
        .reshape(reshape_bn1);
    let e22 = strain.narrow(2, 2, 1).narrow(3, 2, 1).reshape(reshape_bn1);
    (e00, e01, e02, e11, e12, e22)
}

/// Jacobi tangent \(t=\tan\theta\) for annihilating \((p,q)\) off-diagonal (Golub–Van Loan stable form).
#[cfg(feature = "fracture-at2")]
fn jacobi_t_bn1<B: Backend<FloatElem = f32>>(
    app: Tensor<B, 3>,
    aqq: Tensor<B, 3>,
    apq: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let apq_active = apq.clone().abs().greater_elem(1e-20_f32);
    let apq_tiny = apq.clone().abs().lower_elem(1e-20_f32);
    // When |apq|≈0 the Golub–Van Loan ratio is undefined; use a harmless denom so `rho` stays finite;
    // the returned tangent is zeroed wherever `apq_active` is false (see last line).
    let denom = apq
        .clone()
        .mul_scalar(2.0)
        .mask_where(apq_tiny, Tensor::<B, 3>::ones_like(&apq));
    let rho = app.clone().sub(aqq.clone()).div(denom);
    let sqrt_one_rho2 = rho.clone().mul(rho.clone()).add_scalar(1.0_f32).sqrt();
    let t_unequal = rho.clone().sign().div(rho.abs().add(sqrt_one_rho2));
    let t_equal_diag = apq.clone().sign();
    let t_branch = t_unequal.mask_where(app.sub(aqq).abs().lower_elem(1e-12_f32), t_equal_diag);
    Tensor::<B, 3>::zeros_like(&apq).mask_where(apq_active, t_branch)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_cs_from_t_bn1<B: Backend<FloatElem = f32>>(
    t: Tensor<B, 3>,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let c = t
        .clone()
        .powf_scalar(2.0)
        .add_scalar(1.0_f32)
        .sqrt()
        .recip();
    let s = t.mul(c.clone());
    (c, s)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_01<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e00.clone(), e11.clone(), e01.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e00_new = c2
        .clone()
        .mul(e00.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e01.clone()))
        .add(s2.clone().mul(e11.clone()));
    let e11_new = s2
        .clone()
        .mul(e00.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e01.clone()))
        .add(c2.clone().mul(e11.clone()));
    let e01_new = c2.sub(s2.clone()).mul(e01).add(cs.mul(e00.sub(e11)));
    let e02_new = c.clone().mul(e02.clone()).sub(s.clone().mul(e12.clone()));
    let e12_new = s.mul(e02).add(c.mul(e12));
    (e00_new, e01_new, e02_new, e11_new, e12_new, e22)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_02<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e00.clone(), e22.clone(), e02.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e00_new = c2
        .clone()
        .mul(e00.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e02.clone()))
        .add(s2.clone().mul(e22.clone()));
    let e22_new = s2
        .clone()
        .mul(e00.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e02.clone()))
        .add(c2.clone().mul(e22.clone()));
    let e02_new = c2
        .sub(s2.clone())
        .mul(e02)
        .add(cs.mul(e00.clone().sub(e22)));
    let e01_new = c.clone().mul(e01.clone()).sub(s.clone().mul(e12.clone()));
    let e12_new = s.mul(e01).add(c.mul(e12));
    (e00_new, e01_new, e02_new, e11, e12_new, e22_new)
}

#[cfg(feature = "fracture-at2")]
fn jacobi_sweep_12<B: Backend<FloatElem = f32>>(
    e00: Tensor<B, 3>,
    e01: Tensor<B, 3>,
    e02: Tensor<B, 3>,
    e11: Tensor<B, 3>,
    e12: Tensor<B, 3>,
    e22: Tensor<B, 3>,
) -> SymStrainPackBn1<B> {
    let t = jacobi_t_bn1(e11.clone(), e22.clone(), e12.clone());
    let (c, s) = jacobi_cs_from_t_bn1(t);
    let c2 = c.clone().mul(c.clone());
    let s2 = s.clone().mul(s.clone());
    let cs = c.clone().mul(s.clone());
    let e11_new = c2
        .clone()
        .mul(e11.clone())
        .sub(cs.clone().mul_scalar(2.0).mul(e12.clone()))
        .add(s2.clone().mul(e22.clone()));
    let e22_new = s2
        .clone()
        .mul(e11.clone())
        .add(cs.clone().mul_scalar(2.0).mul(e12.clone()))
        .add(c2.clone().mul(e22.clone()));
    let e12_new = c2
        .sub(s2.clone())
        .mul(e12)
        .add(cs.mul(e11.clone().sub(e22)));
    let e01_new = c.clone().mul(e01.clone()).sub(s.clone().mul(e02.clone()));
    let e02_new = s.mul(e01).add(c.mul(e02));
    (e00, e01_new, e02_new, e11_new, e12_new, e22_new)
}

/// Approximate eigenvalues by cyclic Jacobi diagonalization, then
/// \(\psi^+ = \tfrac{1}{2}\sum_i \langle\lambda_i\rangle_+^2\).
#[cfg(feature = "fracture-at2")]
fn tensile_strain_energy_density_spectral_jacobi<B: Backend<FloatElem = f32>>(
    strain: Tensor<B, 4>,
) -> Tensor<B, 3> {
    let (mut e00, mut e01, mut e02, mut e11, mut e12, mut e22) = strain_sym_components_bn1(strain);
    for _ in 0..JACOBI_SWEEPS {
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_01(e00, e01, e02, e11, e12, e22);
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_02(e00, e01, e02, e11, e12, e22);
        (e00, e01, e02, e11, e12, e22) = jacobi_sweep_12(e00, e01, e02, e11, e12, e22);
    }
    let l0 = e00.clamp_min(0.0_f32);
    let l1 = e11.clamp_min(0.0_f32);
    let l2 = e22.clamp_min(0.0_f32);
    l0.powf_scalar(2.0)
        .add(l1.powf_scalar(2.0))
        .add(l2.powf_scalar(2.0))
        .mul_scalar(0.5_f32)
}

#[cfg(all(test, feature = "fracture-at2"))]
mod fracture_at2_tests {
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    use super::tensile_strain_energy_density_spectral_jacobi;

    type B = NdArray<f32>;

    #[test]
    fn psi_plus_positive_uniform_uniaxial() {
        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = 1e-3_f32;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);
        let psi = tensile_strain_energy_density_spectral_jacobi(strain);
        let sum_psi: f32 = psi.into_data().value.iter().sum();
        assert!(
            sum_psi > 1e-12_f32,
            "expected positive ψ⁺ for uniaxial tension; sum={sum_psi}"
        );
    }

    #[test]
    fn laplacian_of_zero_damage_is_finite() {
        use crate::physics::laplacian::TopologicalLaplacian;
        use burn::tensor::Int;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let e_ct = 2usize;
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
        let d = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let lap = TopologicalLaplacian::scalar_laplacian(d.clone(), edges_b1.clone(), d.clone());
        for &x in lap.into_data().value.iter() {
            assert!(x.is_finite(), "laplacian non-finite: {}", x);
        }
    }

    #[test]
    fn node_parity_masks_sum_on_three_node_chain() {
        let dev = NdArrayDevice::Cpu;
        let (mask_even, mask_odd) = super::node_parity_masks_b_n1::<B>(1, 3, &dev);
        let sum_even: f32 = mask_even.into_data().value.iter().sum();
        let sum_odd: f32 = mask_odd.into_data().value.iter().sum();
        assert!((sum_even - 2.0).abs() < 1e-6, "sum_even={sum_even}");
        assert!((sum_odd - 1.0).abs() < 1e-6, "sum_odd={sum_odd}");
    }

    #[test]
    fn update_damage_nonzero_under_fracture_at2() {
        use crate::physics::solvers::PhaseFieldFractureSolver;
        use burn::tensor::Int;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let e_ct = 2usize;

        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

        let exx = 1e-3_f32;
        let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_data[base] = exx;
            strain_data[base + 4] = 0.0;
            strain_data[base + 8] = 0.0;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let psi_sum: f32 = tensile_strain_energy_density_spectral_jacobi(strain.clone())
            .into_data()
            .value
            .iter()
            .sum();
        assert!(
            psi_sum > 1e-15_f32,
            "expected positive ψ⁺ drive on chain; psi_sum={psi_sum}"
        );

        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
        let d_new = solver.update_damage(strain, damage, fracture_energy_gc, edges_b1);
        let vals = d_new.into_data().value;
        assert!(
            vals.iter().all(|x| x.is_finite()),
            "expected finite damage; vals={vals:?}"
        );
        let sum_d: f32 = vals.iter().sum();
        let max_d = vals.iter().copied().fold(0.0_f32, f32::max);
        let mean_d = sum_d / vals.len() as f32;
        assert!(
            max_d > 1e-10_f32,
            "expected positive peak damage on chain; max_d={max_d} sum_d={sum_d}"
        );
        assert!(
            sum_d > 1e-10_f32,
            "expected stable nonzero total damage (no odd/even sum cancellation); sum_d={sum_d}"
        );
        assert!(mean_d > 1e-12_f32, "mean_d={mean_d}");
    }

    #[test]
    fn staggered_outer_one_matches_update_damage_constant_provider() {
        use crate::physics::solvers::PhaseFieldFractureSolver;
        use burn::tensor::Int;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let e_ct = 2usize;

        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

        let exx = 1e-3_f32;
        let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_data[base] = exx;
            strain_data[base + 4] = 0.0;
            strain_data[base + 8] = 0.0;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
        let strain_fixed = strain.clone();
        let d_stagg = solver.update_damage_staggered(
            move |_d: &Tensor<B, 3>| strain_fixed.clone(),
            damage.clone(),
            fracture_energy_gc.clone(),
            edges_b1.clone(),
            1,
        );
        let d_once = solver.update_damage(strain, damage, fracture_energy_gc, edges_b1);
        assert_eq!(
            d_stagg.into_data().value,
            d_once.into_data().value,
            "outer_iterations==1 + constant provider must match update_damage"
        );
    }

    #[test]
    fn staggered_second_outer_stronger_strain_increases_damage_vs_single_pass_weak_a_only() {
        use crate::physics::solvers::PhaseFieldFractureSolver;
        use burn::tensor::Int;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let e_ct = 2usize;

        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

        // Pass 1: negligible tensile drive so damage stays near zero; pass 2: same order as smoke.
        let mut strain_weak_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_weak_data[base] = 1e-12_f32;
        }
        let strain_weak: Tensor<B, 4> = Tensor::from_data(
            Data::new(strain_weak_data, Shape::new([batch, n, 3, 3])),
            &dev,
        );

        let mut strain_strong_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_strong_data[base] = 5e-3_f32;
        }
        let strain_strong: Tensor<B, 4> = Tensor::from_data(
            Data::new(strain_strong_data, Shape::new([batch, n, 3, 3])),
            &dev,
        );

        let damage0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
        let d_only_weak = solver.update_damage(
            strain_weak.clone(),
            damage0.clone(),
            fracture_energy_gc.clone(),
            edges_b1.clone(),
        );

        let mut k = 0usize;
        let d_weak_then_strong = solver.update_damage_staggered(
            |_d: &Tensor<B, 3>| {
                let s = if k == 0 {
                    strain_weak.clone()
                } else {
                    strain_strong.clone()
                };
                k += 1;
                s
            },
            damage0,
            fracture_energy_gc,
            edges_b1,
            2,
        );

        let sum_weak: f32 = d_only_weak.into_data().value.iter().sum();
        let sum_ws: f32 = d_weak_then_strong.into_data().value.iter().sum();
        assert!(
            sum_ws > sum_weak + 1e-8_f32,
            "expected second outer (strong strain) to raise total damage vs single weak pass; sum_weak={sum_weak} sum_ws={sum_ws}"
        );
    }
}

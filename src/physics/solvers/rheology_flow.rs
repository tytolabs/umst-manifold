// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Bingham / projection CFD on the graph (experimental `rheology-bingham`).
//!
//! ## Audit memo (Track E)
//! - **Steady vs transient:** [`plane_bingham_poiseuille_u`](crate::physics::rheology_analytic) is the
//!   **steady** parallel-plate reference; [`BinghamFlowSolver::step`] is an explicit Chorin split on a
//!   graph — no claim of convergence to that steady profile without inlet/outlet BCs (and typically a
//!   staggered MAC pressure) even though the pressure increment solves \(\mathcal{L}\phi=b_h\) with **Jacobi-PCG**
//!   (see deferrals in `tests/verification/rheology_poiseuille.rs`).
//! - **Grid / BC consistency:** Channel smokes pass [`BinghamFlowSolver::edge_length_scale`] as the
//!   wall-normal spacing so \(\dot\gamma \sim |\Delta u|/h\) matches SI; wall velocity is enforced
//!   **outside** the solver via a nodal mask (not embedded in `step`).
//! - **NaN pitfalls:** `BINGHAM_EPS` regularizes \(\dot\gamma\); `rho_e`, `deg_n`, and `du_mag` use
//!   clamps to avoid div-by-zero; interior **zero-velocity** with frozen-λ defaults can still make
//!   \(\tau_0/\dot\gamma\) large — watch `f32` overflow if \(\tau_0\) and `dt` are extreme.
//!
//! ## Chorin-style split
//! 1. **Predictor** \(u^\*\): explicit integration of body force, pressure gradient, and viscous
//!    acceleration. Per-component viscous diffusion could call
//!    [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`] three times on
//!    `[B,N,1]` slices; here we instead apply one **edge-assembled** divergence of
//!    \(\nu_e \,\mathrm{d}_0 u\) so **Bingham-regularized** kinematic viscosity \(\nu_e=\eta_e/\rho_e\)
//!    varies per edge (matches the gather/scatter pattern of the scalar Laplacian).
//! 2. **Pressure Poisson (graph v0 — divergence RHS)**: solve \(\mathcal{L}\phi = b_h(u^\*)\) where
//!    \(\mathcal{L}\) is [`TopologicalLaplacian::scalar_laplacian`] and \(b_h\) is the **weak primal divergence**
//!    of the scalar tangential mean flux \(q_e=(\bar u^\*\!\cdot\hat t)f_c\) (see `docs/research/rheology_pressure_poisson_roadmap.md` §2–4). This **replaces** the legacy \(\sum_c \mathcal{L}u^\*_c\) surrogate.
//!    The roadmap’s explicit \(\tilde b = b_h/\Delta t\) scaling note on this graph-only lane remains
//!    **deferred** for a MAC / staggered RHS pairing (see `docs/research/rheology_pressure_poisson_roadmap.md` §2–4).
//!    \(\mathcal{L}\) remains the graph Laplacian — not a MAC staggered operator — until that lane lands.
//!    The linear solve is **Jacobi-preconditioned CG** on \(A=-\mathcal{L}\), \(b=-b_h\) (SPD on the mean-free
//!    subspace), relative residual exit, iteration cap — then \(\phi\) is shifted to **zero mean** per batch
//!    (gauge for the pure-Neumann graph null space).
//! 3. **Projection (verification \#7 — momentum-consistent flux):** subtract \(\Delta t\) times the weak
//!    divergence of the **same** edge flux pattern as the pressure-gradient predictor,
//!    \(-(\phi_j-\phi_i)\hat t/\rho_e\) per edge (see `edge_pressure` construction), with \(\hat t\) from \(u^\*\).
//!    This replaces the legacy raw `div((φ_j-φ_i)\hat t f_c)` subtraction, which was not a discrete
//!    \(-\Delta t\,\nabla p/\rho\) Helmholtz correction.
//! 4. **Pressure update**: \(p \leftarrow p + \phi\).
//!
//! ## Bingham regularization
//! Per edge: \(\dot\gamma = \|\Delta u\| / h\) with edge scale \(h=\) [`BinghamFlowSolver::edge_length_scale`]
//! (dynamic viscosity routing can supply wall-normal spacing for channel benchmarks).
//! \(\eta = \mu + \tau_0 / (\dot\gamma + \varepsilon)\), \(\nu = \eta/\rho\) on the edge.
//!
//! ## Thixotropy (Roussel-type \(\lambda\))
//! Structure parameter \(\lambda\in[0,1]\) on nodes (`lambda_thix` `[B,N,1]`) follows an explicit
//! one-step of
//! \[
//!   \frac{\mathrm{d}\lambda}{\mathrm{d}t}
//!   = \frac{1-\lambda}{t_\mathrm{rest}} - \lambda\,\frac{|\dot\gamma|}{\gamma_\mathrm{crit}},
//! \]
//! with \(|\dot\gamma|\) **nodal** = mean of incident edge \(|\dot\gamma|\) (same edge field as
//! Bingham, wired from [`primal_scalar_edge_increment`] on `velocity`).
//! Yield on edges uses \(\tau_{0,e}\leftarrow \tau_{0,e}\,\bar\lambda_e\) with \(\bar\lambda_e\) the
//! mean of endpoint \(\lambda\) at the **start** of the step (explicit coupling).
//!
//! **Backward compatibility:** pass `lambda_thix = Tensor::ones(...)` and keep
//! [`BinghamFlowSolver::t_rest_thix`] / [`BinghamFlowSolver::gamma_crit_thix`] at the shipped
//! defaults ([`BinghamFlowSolver::T_REST_NO_THIX`], [`BinghamFlowSolver::GAMMA_CRIT_NO_THIX`]) so the
//! \(\lambda\)-ODE is effectively frozen and \(\tau_0\) matches the pre-thixotropy path.
//!
//! ## Default builds (`solver-experimental` **off**)
//! Returns `(velocity, pressure, lambda_thix)` unchanged so `cargo test` stays green.
//!
//! ## R2.2 — Honest scope (DEFERRAL — Rheology, Track J)
//! **In scope today:** explicit predictor + pressure correction (**Jacobi-PCG** on \(-\mathcal{L}\) with an
//! RHS built from the **weak primal divergence** of tangential mean edge flux — a graph-only discrete
//! source, not a full MAC \(\nabla_h\!\cdot u^\*\)); wall BCs as **external** nodal masks in tests; Bingham +
//! optional Roussel \(\lambda\) on the same 1-skeleton.
//!
//! **Out of scope / not CI-certified:** developed **2D** channel flow matching plane Poiseuille
//! (Bingham or Newtonian) on the full **64×16** cell graph; inlet/outlet pressure drop as a **consistent**
//! open boundary with this split; claims of convergence to [`plane_bingham_poiseuille_u`] or the
//! regularized quadrature profile without replacing the pressure step.
//!
//! **Known numerical boundary:** the legacy \(\sum_c \mathcal{L}u^\*_c\) surrogate plus **unscaled** tangent
//! projection produced \(\mathcal O(10^3\!-\!10^4)\) first-step \(\|u\|_\infty\) amplification on the **65×17**
//! SI channel harness. **Verification \#7** pairs a tangential mean-flux divergence \(b_h(u^\*)\) with **Jacobi-PCG**
//! on \(-\mathcal{L}\) and a **pressure-gradient flux template** projection scaled by \(\Delta t\) (plus `mean(φ)=0` gauge).
//! CI brackets the resulting step-0→1 amplification in `chorin_surrogate_poisson_amplification_regression_guard`
//! (historical name).
//! Steady vs analytic comparisons stay deferred until inlet/outlet BCs plus a MAC or cell-centred pressure solve land.
//!
//! ## MAC + Poisson — integration points (R2.2, design note)
//! Ring 2 **R2.2** calls for MAC or cell-centred pressure on the developed channel; this module ships **graph**
//! **Jacobi-PCG** on \(-\mathcal{L}\) (relative residual tolerance, capped iterations) with the \#7 projection path below.
//! The following are **hook points** for a future staggered / incompressible-correct split — **not** an implemented MAC grid:
//! - **After the predictor:** `step_experimental` forms `u_star` from explicit momentum (body, viscous,
//!   pressure-gradient acceleration). A MAC predictor would typically commit **face-normal** provisional
//!   fluxes here; today everything stays nodal on `edges_b1`.
//! - **Poisson RHS:** Shipped path uses [`primal_divergence_from_edge_flux_topo`] on scalar flux \(q_e f_c\)
//!   derived from \(u^\*\) (tangential mean; see “Chorin-style split” §2). A MAC staggered \(\nabla_h\!\cdot u^\*\)
//!   on face fluxes remains a future swap-in.
//! - **Poisson solve:** Shipped path uses **Jacobi-preconditioned CG** on \(-\mathcal{L}\) (see
//!   [`solve_pressure_phi_jacobi_cg`]); a chain **Thomas** fast lane when topology is a 1-D path remains a future
//!   swap-in — compare electrochemistry Poisson helpers.
//! - **Projection:** Edge increments [`primal_scalar_edge_increment`] / tangent projection of \(\nabla\phi\)
//!   remain the right **shape** once \(\phi\) solves the consistent discrete Poisson; **inlet/outlet** pressure
//!   or flux BCs still require explicit pinning — absent today.
//!
//! **Scope:** Wiring **2D channel MAC + consistent divergence BCs** is **not** a small patch on this scaffold
//! (well beyond a sub-hundred-line swap). Treat the bullets above as **documentation of insertion points** until
//! a dedicated pressure solve + boundary module ships — see **DEFERRAL — Rheology** in `docs/Solver-Status.md`.

#[cfg(feature = "rheology-bingham")]
use crate::physics::dec_primal::{
    primal_divergence_from_edge_flux_topo, primal_scalar_edge_increment,
};
#[cfg(feature = "rheology-bingham")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "rheology-bingham")]
use crate::physics::rheology_analytic::RHEOLOGY_FLOW_BINGHAM_EPS as BINGHAM_EPS;
#[cfg(feature = "rheology-bingham")]
use crate::physics::topology::EdgeTopology;

use burn::tensor::{backend::Backend, Int, Tensor};

#[cfg(feature = "rheology-bingham")]
/// Relative residual \(\|b - A\phi\|_2 / \|b\|_2 = \|\mathcal{L}\phi - b_h\|_2 / \|b_h\|_2\) with \(A=-\mathcal{L}\), \(b=-b_h\).
const POISSON_CG_REL_TOL: f32 = 2e-5;
#[cfg(feature = "rheology-bingham")]
/// Upper bound on PCG iterations per Chorin pressure step (graph Laplacian; Jacobi preconditioner).
const POISSON_CG_MAX_IT_CAP: usize = 4096;
#[cfg(feature = "rheology-bingham")]
/// Floor for \(t_\mathrm{rest}\), \(\gamma_\mathrm{crit}\) in denominators (SI scales, avoids div-by-zero).
const THIX_PARAM_EPS: f32 = 1e-12;

/// Fresh-state rheology + Navier-Stokes–like step on the DEC 1-skeleton.
pub struct BinghamFlowSolver {
    pub dt: f32,
    pub mu_plastic: f32,
    /// Effective mesh spacing for shear rate \(|\dot\gamma|\approx |\Delta u|/h\) on edges [m].
    ///
    /// Default `1.0` preserves legacy normalized-graph behaviour. Channel benchmarks pass the
    /// wall-normal cell height so regularized Bingham viscosity matches SI scales.
    pub edge_length_scale: f32,
    /// Roussel rest time \(t_\mathrm{rest}\) [s] in \(\mathrm{d}\lambda/\mathrm{d}t=(1-\lambda)/t_\mathrm{rest}-\cdots\).
    ///
    /// Default [`BinghamFlowSolver::T_REST_NO_THIX`] makes \((1-\lambda)/t_\mathrm{rest}\approx 0\) in `f32`
    /// so \(\lambda\) stays fixed if you do not opt into thixotropy.
    pub t_rest_thix: f32,
    /// Critical shear scale \(\gamma_\mathrm{crit}\) [1/s]; breakdown term is \(\lambda|\dot\gamma|/\gamma_\mathrm{crit}\).
    ///
    /// Default [`BinghamFlowSolver::GAMMA_CRIT_NO_THIX`] zeros the breakdown term for legacy runs.
    pub gamma_crit_thix: f32,
}

impl BinghamFlowSolver {
    /// Large \(t_\mathrm{rest}\) shipped as default so \(\lambda\) is effectively **not** rebuilt when unused.
    pub const T_REST_NO_THIX: f32 = 1e12;
    /// Large \(\gamma_\mathrm{crit}\) shipped as default so shear breakdown is off when unused.
    pub const GAMMA_CRIT_NO_THIX: f32 = 1e12;

    /// Constructor with **frozen-\(\lambda\)** defaults (matches historical two-field `BinghamFlowSolver` behavior).
    pub fn new(dt: f32, mu_plastic: f32) -> Self {
        Self {
            dt,
            mu_plastic,
            edge_length_scale: 1.0,
            t_rest_thix: Self::T_REST_NO_THIX,
            gamma_crit_thix: Self::GAMMA_CRIT_NO_THIX,
        }
    }

    /// One explicit / split step for velocity, pressure, and Roussel \(\lambda\).
    ///
    /// # Shapes (contract)
    /// - `velocity`: `[B, N, 3]`
    /// - `pressure`, `yield_stress`, `density`, `lambda_thix`: `[B, N, 1]`
    /// - `edges_b1`: `[2, E]`
    /// - `gravity`: `[3]` (acceleration; broadcast to `[B, N, 3]`).
    ///
    /// Returns `(velocity_new, pressure_new, lambda_thix_new)` with same ranks as inputs.
    ///
    /// ## Default builds (`solver-experimental` **off**)
    /// Returns inputs unchanged (documented no-op / Phase 3 stub for downstream wiring tests).
    ///
    /// ## `--features solver-experimental`
    /// Runs the Chorin MVP documented in this module and one explicit Roussel step on \(\lambda\).
    #[allow(unused_variables, clippy::too_many_arguments)]
    pub fn step<B: Backend<FloatElem = f32>>(
        &self,
        velocity: Tensor<B, 3>,
        pressure: Tensor<B, 3>,
        yield_stress: Tensor<B, 3>,
        density: Tensor<B, 3>,
        lambda_thix: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        gravity: Tensor<B, 1>,
    ) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
        #[cfg(not(feature = "rheology-bingham"))]
        {
            (velocity, pressure, lambda_thix)
        }

        #[cfg(feature = "rheology-bingham")]
        {
            step_experimental(
                self,
                velocity,
                pressure,
                yield_stress,
                density,
                lambda_thix,
                edges_b1,
                gravity,
            )
        }
    }
}

impl Default for BinghamFlowSolver {
    fn default() -> Self {
        Self::new(1e-3, 1.0)
    }
}

/// Solve \(\mathcal{L}\phi = b_h\) with **Jacobi-preconditioned CG** on \(A=-\mathcal{L}\), \(b=-b_h\) (SPD form).
/// Returns \(\phi\) with **zero mean** per batch. `rhs` should already be mean-free (gauge-compatible).
///
/// When the iterate loses finiteness or the terminal \(\|\mathcal{L}\phi-b_h\|/\|b_h\|\) diagnostic is non-finite,
/// falls back to a damped **Richardson** sweep if **`UMST_RHEOLOGY_POISSON_RICHARDSON_FALLBACK=1`** (or `true`)
/// is set, or if built with **`--features rheology_poisson_richardson_fallback`**; otherwise returns zeros
/// (safe Chorin continuation).
#[cfg(feature = "rheology-bingham")]
fn chorin_poisson_richardson_fallback_enabled() -> bool {
    if cfg!(feature = "rheology_poisson_richardson_fallback") {
        true
    } else {
        static FLAG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *FLAG.get_or_init(|| {
            std::env::var_os("UMST_RHEOLOGY_POISSON_RICHARDSON_FALLBACK")
                .map(|v| {
                    let s = v.to_string_lossy();
                    s == "1" || s.eq_ignore_ascii_case("true")
                })
                .unwrap_or(false)
        })
    }
}

/// Damped Richardson on \(\mathcal{L}\phi=b_h\) with per-iteration mean removal (Neumann null space).
#[cfg(feature = "rheology-bingham")]
fn solve_pressure_phi_richardson_fallback<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
    rhs_norm: f32,
) -> Tensor<B, 3> {
    let lambda_upper = 12.0_f32;
    let omega = (1.35_f32 / lambda_upper).clamp(0.02_f32, 0.12_f32);
    let mut phi = Tensor::<B, 3>::zeros_like(&rhs);
    let max_it = n.saturating_mul(64).clamp(1024, 16000);
    let rhs_d = rhs_norm.max(1e-30_f32);
    for _ in 0..max_it {
        let lphi =
            TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
        let r = rhs.clone().sub(lphi);
        let rn = r.clone().powf_scalar(2.0).sum().sqrt().into_scalar();
        if !rn.is_finite() {
            break;
        }
        if rn / rhs_d < POISSON_CG_REL_TOL {
            break;
        }
        phi = phi.add(r.mul_scalar(omega));
        let pm = phi
            .clone()
            .sum_dim(1)
            .div_scalar(n as f32)
            .reshape([batch, 1, 1]);
        phi = phi.sub(pm);
        let pmx = phi.clone().abs().max().into_scalar();
        if !pmx.is_finite() {
            return Tensor::zeros_like(&rhs);
        }
    }
    let phi_mean = phi
        .clone()
        .sum_dim(1)
        .div_scalar(n as f32)
        .reshape([batch, 1, 1]);
    phi.sub(phi_mean)
}

#[cfg(feature = "rheology-bingham")]
fn solve_pressure_phi_jacobi_cg<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
) -> Tensor<B, 3> {
    let rhs_abs_max = rhs.clone().abs().max().into_scalar();
    if !rhs_abs_max.is_finite() {
        return Tensor::zeros_like(&rhs);
    }

    let rhs_norm = rhs
        .clone()
        .powf_scalar(2.0)
        .sum()
        .sqrt()
        .into_scalar()
        .max(1e-30_f32);
    if !rhs_norm.is_finite() || rhs_norm < 1e-24_f32 {
        return Tensor::zeros_like(&rhs);
    }

    let diag_a =
        TopologicalLaplacian::scalar_laplacian_neg_opposite_diag(edges_b1.clone(), damage.clone());
    let diag_inv = diag_a.clamp_min(1e-14_f32).recip();

    let mut phi = Tensor::<B, 3>::zeros_like(&rhs);

    let lphi =
        TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
    let mut r = lphi.sub(rhs.clone());

    let mut z = r.clone().mul(diag_inv.clone());
    let mut p = z.clone();
    let mut rz_old = r.clone().mul(z.clone()).sum().into_scalar().max(1e-40_f32);
    if !rz_old.is_finite() {
        return if chorin_poisson_richardson_fallback_enabled() {
            solve_pressure_phi_richardson_fallback(rhs, edges_b1, damage, batch, n, rhs_norm)
        } else {
            Tensor::zeros_like(&rhs)
        };
    }

    let max_it = n.saturating_mul(10).clamp(256, POISSON_CG_MAX_IT_CAP);

    for _ in 0..max_it {
        let lp =
            TopologicalLaplacian::scalar_laplacian(p.clone(), edges_b1.clone(), damage.clone());
        let ap = lp.neg();

        let p_ap = p.clone().mul(ap.clone()).sum().into_scalar();
        if !p_ap.is_finite() || p_ap <= 1e-40_f32 {
            break;
        }
        let alpha = (rz_old / p_ap).clamp(-1e4_f32, 1e4_f32);
        if !alpha.is_finite() {
            break;
        }

        phi = phi.add(p.clone().mul_scalar(alpha));
        let phi_mx = phi.clone().abs().max().into_scalar();
        if !phi_mx.is_finite() {
            return if chorin_poisson_richardson_fallback_enabled() {
                solve_pressure_phi_richardson_fallback(
                    rhs.clone(),
                    edges_b1.clone(),
                    damage.clone(),
                    batch,
                    n,
                    rhs_norm,
                )
            } else {
                Tensor::zeros_like(&rhs)
            };
        }

        r = r.sub(ap.mul_scalar(alpha));

        let res_norm = r.clone().powf_scalar(2.0).sum().sqrt().into_scalar();
        if !res_norm.is_finite() {
            break;
        }
        if res_norm / rhs_norm < POISSON_CG_REL_TOL {
            break;
        }

        z = r.clone().mul(diag_inv.clone());
        let rz_new = r.clone().mul(z.clone()).sum().into_scalar();
        if !rz_new.is_finite() {
            break;
        }

        let beta = if rz_old > 1e-40_f32 {
            (rz_new / rz_old).clamp(0.0_f32, 1e6_f32)
        } else {
            0.0_f32
        };
        if !beta.is_finite() {
            break;
        }
        p = z.clone().add(p.mul_scalar(beta));
        rz_old = rz_new.max(1e-40_f32);
    }

    let phi_mean = phi
        .clone()
        .sum_dim(1)
        .div_scalar(n as f32)
        .reshape([batch, 1, 1]);
    let phi = phi.sub(phi_mean);

    let phi_mx = phi.clone().abs().max().into_scalar();
    let lap_phi =
        TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
    let rel_res = lap_phi
        .sub(rhs.clone())
        .powf_scalar(2.0)
        .sum()
        .sqrt()
        .into_scalar()
        / rhs_norm;

    if phi_mx.is_finite() && rel_res.is_finite() {
        return phi;
    }
    if chorin_poisson_richardson_fallback_enabled() {
        return solve_pressure_phi_richardson_fallback(rhs, edges_b1, damage, batch, n, rhs_norm);
    }
    Tensor::zeros_like(&rhs)
}

/// Shipped Chorin pressure Poisson RHS (verification \#7): mean-free weak primal divergence of
/// scalar tangential mean flux \(q_e=(\bar u^*\!\cdot\hat t)f_c\) (see module rustdoc §2).
///
/// Returns `(rhs, \hat t)` with \(\hat t\) the **same** edge unit tangents used in the Helmholtz projection.
#[cfg(feature = "rheology-bingham")]
fn chorin_pressure_rhs_mean_free_weak_divergence<B: Backend<FloatElem = f32>>(
    u_star: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
    flow_coeff: Tensor<B, 3>,
    batch: usize,
    n_edges: usize,
    n: usize,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    let ch3 = 3usize;
    let du_s = primal_scalar_edge_increment(u_star.clone(), topo);
    let du_s_mag_sq = du_s
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .reshape([batch, n_edges, 1]);
    let du_s_mag = du_s_mag_sq
        .sqrt()
        .add_scalar(BINGHAM_EPS)
        .clamp_min(BINGHAM_EPS);
    let du_s_mag3 = du_s_mag.expand([batch, n_edges, ch3]);
    let t_hat_s = du_s.div(du_s_mag3);

    let src_ix3 = topo.expand_src_gather_indices(batch, ch3);
    let tgt_ix3 = topo.expand_tgt_gather_indices(batch, ch3);
    let u_src3 = u_star.clone().gather(1, src_ix3);
    let u_tgt3 = u_star.clone().gather(1, tgt_ix3);
    let u_mean_edge = u_src3.add(u_tgt3).div_scalar(2.0_f32);
    let q_edge = u_mean_edge
        .mul(t_hat_s.clone())
        .sum_dim(2)
        .reshape([batch, n_edges, 1]);
    let fc1 = flow_coeff.narrow(2, 0, 1);
    let flux_scalar_edge = q_edge.mul(fc1);
    let u_star_x0 = u_star.narrow(2, 0, 1);
    let rhs = primal_divergence_from_edge_flux_topo(flux_scalar_edge, topo, &u_star_x0);
    let rhs_mean = rhs.clone().sum_dim(1).div_scalar(n as f32);
    let rhs = rhs.sub(rhs_mean);
    (rhs, t_hat_s)
}

#[cfg(feature = "rheology-bingham")]
#[allow(clippy::too_many_arguments)]
fn step_experimental<B: Backend<FloatElem = f32>>(
    solver: &BinghamFlowSolver,
    velocity: Tensor<B, 3>,
    pressure: Tensor<B, 3>,
    yield_stress: Tensor<B, 3>,
    density: Tensor<B, 3>,
    lambda_thix: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    gravity: Tensor<B, 1>,
) -> (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>) {
    let dt = solver.dt;
    let mu = solver.mu_plastic;
    let t_rest = solver.t_rest_thix.max(THIX_PARAM_EPS);
    let gamma_crit = solver.gamma_crit_thix.max(THIX_PARAM_EPS);

    let batch = velocity.dims()[0];
    let n = velocity.dims()[1];
    let topo = EdgeTopology::new(edges_b1.clone());
    let n_edges = topo.n_edges();
    let device = velocity.device();

    // No fracture-driven masking yet — zeros ⇒ fully connected flow coefficients (see scalar_laplacian).
    let damage = Tensor::<B, 3>::zeros_like(&density);

    let ch3 = 3usize;
    let ch1 = 1usize;
    let src_ix1 = topo.expand_src_gather_indices(batch, ch1);
    let tgt_ix1 = topo.expand_tgt_gather_indices(batch, ch1);

    let damage_src = damage.clone().gather(1, src_ix1.clone());
    let damage_tgt = damage.clone().gather(1, tgt_ix1.clone());
    let edge_damage = damage_src.add(damage_tgt).div_scalar(2.0_f32);
    let flow_coeff = Tensor::<B, 3>::ones_like(&edge_damage).sub(edge_damage); // [B,E,3]

    let du = primal_scalar_edge_increment(velocity.clone(), &topo); // [B,E,3], reused below

    let du_mag_sq = du
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .reshape([batch, n_edges, 1]);
    let du_mag = du_mag_sq.sqrt();

    let h_edge = solver.edge_length_scale.max(1e-30_f32);
    let gamma_dot = du_mag.clone().div_scalar(h_edge).add_scalar(BINGHAM_EPS);

    // Nodal |γ̇|: mean over incident edges (scatter-sum / degree).
    let ones_e = Tensor::<B, 3>::ones_like(&gamma_dot);
    let deg_n = Tensor::<B, 3>::zeros([batch, n, 1], &device)
        .scatter(1, src_ix1.clone(), ones_e.clone())
        .scatter(1, tgt_ix1.clone(), ones_e);
    let sum_gamma_n = Tensor::<B, 3>::zeros([batch, n, 1], &device)
        .scatter(1, src_ix1.clone(), gamma_dot.clone())
        .scatter(1, tgt_ix1.clone(), gamma_dot.clone());
    let gamma_dot_n = sum_gamma_n.div(deg_n.clamp_min(1.0_f32));

    // Roussel explicit Euler on λ (then clamp to [0,1]).
    let one_m_lam = Tensor::<B, 3>::ones_like(&lambda_thix).sub(lambda_thix.clone());
    let rebuild = one_m_lam.div_scalar(t_rest);
    let breakdown = lambda_thix.clone().mul(gamma_dot_n).div_scalar(gamma_crit);
    let dlam_dt = rebuild.sub(breakdown);
    let lambda_new = lambda_thix
        .clone()
        .add(dlam_dt.mul_scalar(dt))
        .clamp(0.0_f32, 1.0_f32);

    let tau_src = yield_stress.clone().gather(1, src_ix1.clone());
    let tau_tgt = yield_stress.gather(1, tgt_ix1.clone());
    let mut tau0_e = tau_src.add(tau_tgt).div_scalar(2.0_f32);
    let lam_src = lambda_thix.clone().gather(1, src_ix1.clone());
    let lam_tgt = lambda_thix.gather(1, tgt_ix1.clone());
    let lambda_e = lam_src.add(lam_tgt).div_scalar(2.0_f32);
    tau0_e = tau0_e.mul(lambda_e);

    let rho_src = density.clone().gather(1, src_ix1.clone());
    let rho_tgt = density.gather(1, tgt_ix1.clone());
    let rho_e = rho_src
        .add(rho_tgt)
        .div_scalar(2.0_f32)
        .clamp_min(1e-12_f32);

    let eta_e = tau0_e.div(gamma_dot).add_scalar(mu);
    let nu_e = eta_e.div(rho_e.clone()); // [B,E,1]

    let nu_e3 = nu_e.expand([batch, n_edges, ch3]);
    let fc3 = flow_coeff.clone();
    let edge_viscous = du.clone().mul(nu_e3).mul(fc3.clone());

    let viscous_accel = primal_divergence_from_edge_flux_topo(edge_viscous, &topo, &velocity);

    let p_src = pressure.clone().gather(1, src_ix1.clone());
    let p_tgt = pressure.clone().gather(1, tgt_ix1.clone());
    let dp = p_tgt.sub(p_src); // [B,E,1]

    let du_mag_safe = du_mag.add_scalar(BINGHAM_EPS).clamp_min(BINGHAM_EPS);
    let du_mag3 = du_mag_safe.expand([batch, n_edges, ch3]);
    let t_hat = du.div(du_mag3);

    let rho_e3 = rho_e.clone().expand([batch, n_edges, ch3]);
    let dp3 = dp.expand([batch, n_edges, ch3]);
    let edge_pressure = dp3.neg().div(rho_e3.clone()).mul(t_hat).mul(fc3.clone());

    let pressure_accel = primal_divergence_from_edge_flux_topo(edge_pressure, &topo, &velocity);

    let g = gravity.reshape([1, 1, 3]).expand([batch, n, 3]);

    let momentum = g.add(viscous_accel).add(pressure_accel);

    let u_star = velocity.add(momentum.mul_scalar(dt));

    // --- Pressure Poisson RHS: weak divergence of q_e = (ū*·t̂) f_c (reuse t̂ in projection) ---
    let (rhs, t_hat_s) = chorin_pressure_rhs_mean_free_weak_divergence(
        u_star.clone(),
        &topo,
        flow_coeff.clone(),
        batch,
        n_edges,
        n,
    );

    let phi = solve_pressure_phi_jacobi_cg(rhs, edges_b1.clone(), damage.clone(), batch, n);

    // Projection: same weak-divergence routing as `edge_pressure`, with φ increment and an extra Δt factor.
    let dphi = primal_scalar_edge_increment(phi.clone(), &topo);
    let dphi3 = dphi.expand([batch, n_edges, ch3]);
    let edge_grad_phi = dphi3.neg().div(rho_e3).mul(t_hat_s).mul(fc3);
    let proj_accel = primal_divergence_from_edge_flux_topo(edge_grad_phi, &topo, &u_star);
    let velocity_new = u_star.sub(proj_accel.mul_scalar(dt));

    let pressure_new = pressure.add(phi);

    (velocity_new, pressure_new, lambda_new)
}

#[cfg(all(test, feature = "rheology-bingham"))]
mod tests {
    use super::BinghamFlowSolver;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    #[test]
    fn roussel_lambda_decreases_under_shear_with_finite_crit() {
        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 3usize;
        let e_ct = 2usize;
        // 0—1—2 chain
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
        let velocity = Tensor::<B, 3>::from_data(
            Data::new(
                vec![
                    0.0_f32, 0.0, 0.0, //
                    0.5, 0.0, 0.0, //
                    1.0, 0.0, 0.0,
                ],
                Shape::new([batch, n, 3]),
            ),
            &dev,
        );
        let pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let density = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let lambda0 = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let gravity = Tensor::<B, 1>::zeros([3], &dev);

        let mut solver = BinghamFlowSolver::new(0.01, 1e-3);
        solver.t_rest_thix = 1e6_f32;
        solver.gamma_crit_thix = 0.5_f32;

        let (_v, _p, lam1) = solver.step(
            velocity,
            pressure,
            yield_stress,
            density,
            lambda0,
            edges_b1,
            gravity,
        );
        let lam_mid: f32 = lam1.clone().slice([0..1, 1..2, 0..1]).into_scalar();
        assert!(
            lam_mid < 1.0_f32,
            "expected breakdown term to reduce λ at interior node; got {lam_mid}"
        );
    }

    #[test]
    fn default_freeze_leaves_lambda_unchanged_within_tol() {
        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 2usize;
        let e_ct = 1usize;
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, e_ct])), &dev);
        let velocity = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
        let pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let density = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let lambda0 = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let gravity = Tensor::<B, 1>::zeros([3], &dev);

        let solver = BinghamFlowSolver::new(0.01, 1e-3);
        let (_v, _p, lam1) = solver.step(
            velocity,
            pressure,
            yield_stress,
            density,
            lambda0.clone(),
            edges_b1,
            gravity,
        );
        let d = lam1.sub(lambda0).abs().max().into_scalar();
        assert!(
            d < 1e-5_f32,
            "frozen-default λ should be unchanged; max|Δλ|={d}"
        );
    }

    #[test]
    fn jacobi_cg_pressure_residual_small_on_mean_free_synthetic_rhs() {
        use super::solve_pressure_phi_jacobi_cg;
        use crate::physics::laplacian::TopologicalLaplacian;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 5usize;
        let e_ct = 4usize;
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1, 1, 2, 2, 3, 3, 4], Shape::new([2, e_ct])),
            &dev,
        );
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);

        let phi_vals = vec![0.12_f32, -0.05, -0.03, 0.01, -0.05];
        let phi_src =
            Tensor::<B, 3>::from_data(Data::new(phi_vals, Shape::new([batch, n, 1])), &dev);
        let mean = phi_src.clone().sum_dim(1).div_scalar(n as f32);
        let phi_src = phi_src.sub(mean.reshape([batch, 1, 1]));

        let rhs = TopologicalLaplacian::scalar_laplacian(
            phi_src.clone(),
            edges_b1.clone(),
            damage.clone(),
        );
        let rhs_mean = rhs.clone().sum_dim(1).div_scalar(n as f32);
        let rhs_mf = rhs.sub(rhs_mean);

        let phi = solve_pressure_phi_jacobi_cg(
            rhs_mf.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );
        let lap_phi =
            TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
        let res = lap_phi.sub(rhs_mf.clone());
        let rn = res.clone().powf_scalar(2.0).sum().sqrt().into_scalar();
        let bn = rhs_mf
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .into_scalar()
            .max(1e-20_f32);
        let rel = rn / bn;
        assert!(
            rel < 5e-4_f32,
            "Jacobi-PCG should drive ||Lφ−b||/||b|| small; rel={rel}"
        );
        assert!(
            phi.into_data()
                .convert::<f32>()
                .value
                .iter()
                .all(|x| x.is_finite()),
            "expected finite φ"
        );
    }

    /// P1 \#7 — **`solver-experimental`:** minimal path graph — dense grounded direct Poisson vs Jacobi-PCG.
    #[cfg(feature = "solver-experimental")]
    #[allow(clippy::needless_range_loop)]
    #[test]
    fn chorin_poisson_jacobi_cg_agrees_direct_grounded_chain() {
        use super::solve_pressure_phi_jacobi_cg;
        use crate::physics::laplacian::TopologicalLaplacian;
        use burn::tensor::backend::Backend;

        fn dense_laplacian_columns_from_tensor<B: Backend<FloatElem = f32>>(
            n: usize,
            edges_b1: Tensor<B, 2, Int>,
            damage: Tensor<B, 3>,
            batch: usize,
            dev: &B::Device,
        ) -> Vec<Vec<f64>> {
            let mut l = vec![vec![0.0_f64; n]; n];
            for j in 0..n {
                let mut data = vec![0.0_f32; batch * n];
                data[j] = 1.0_f32;
                let ej = Tensor::<B, 3>::from_data(Data::new(data, Shape::new([batch, n, 1])), dev);
                let col =
                    TopologicalLaplacian::scalar_laplacian(ej, edges_b1.clone(), damage.clone());
                let vals: Vec<f32> = col.into_data().convert::<f32>().value;
                for i in 0..n {
                    l[i][j] = f64::from(vals[i]);
                }
            }
            l
        }

        fn symmetrize(l: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
            let mut s = vec![vec![0.0_f64; n]; n];
            for i in 0..n {
                for j in 0..n {
                    s[i][j] = 0.5_f64 * (l[i][j] + l[j][i]);
                }
            }
            s
        }

        fn solve_grounded_then_mean_free(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
            let n = b.len();
            let m = n - 1;
            let ridge = 1e-10_f64;
            let mut a = vec![vec![0.0_f64; m]; m];
            for i in 0..m {
                for j in 0..m {
                    a[i][j] = l[i][j];
                }
                a[i][i] += ridge;
            }
            let mut r = b[..m].to_vec();
            gauss_jordan_pivot(&mut a, &mut r);
            let mut phi = vec![0.0_f64; n];
            phi[..m].copy_from_slice(&r[..m]);
            phi[n - 1] = 0.0_f64;
            let sum: f64 = phi.iter().sum();
            for p in &mut phi {
                *p -= sum / n as f64;
            }
            phi
        }

        fn gauss_jordan_pivot(a: &mut [Vec<f64>], b: &mut [f64]) {
            let n = b.len();
            for k in 0..n {
                let mut piv = k;
                let mut best = a[k][k].abs();
                for i in k + 1..n {
                    let v = a[i][k].abs();
                    if v > best {
                        best = v;
                        piv = i;
                    }
                }
                if piv != k {
                    a.swap(k, piv);
                    b.swap(k, piv);
                }
                let akk = a[k][k];
                assert!(akk.abs() > 1e-18, "singular pivot at {k}");
                for j in k..n {
                    a[k][j] /= akk;
                }
                b[k] /= akk;
                for i in 0..n {
                    if i == k {
                        continue;
                    }
                    let f = a[i][k];
                    if f.abs() < 1e-30 {
                        continue;
                    }
                    for j in k..n {
                        a[i][j] -= f * a[k][j];
                    }
                    b[i] -= f * b[k];
                }
            }
        }

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 5usize;
        let e_ct = 4usize;
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1, 1, 2, 2, 3, 3, 4], Shape::new([2, e_ct])),
            &dev,
        );
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);

        let l_raw = dense_laplacian_columns_from_tensor(n, edges_b1.clone(), damage.clone(), batch, &dev);
        let l_sym = symmetrize(&l_raw, n);

        let phi_seed = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.11_f32, -0.07, 0.02, -0.04, -0.02],
                Shape::new([batch, n, 1]),
            ),
            &dev,
        );
        let mean = phi_seed.clone().sum_dim(1).div_scalar(n as f32);
        let phi_true = phi_seed.sub(mean.reshape([batch, 1, 1]));
        let phi_true_vals: Vec<f32> = phi_true.clone().into_data().convert::<f32>().value;
        let phi_f64: Vec<f64> = phi_true_vals.iter().map(|&x| f64::from(x)).collect();

        let mut lphi = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                lphi[i] += l_sym[i][j] * phi_f64[j];
            }
        }
        let sum_lp: f64 = lphi.iter().sum();
        let b_mf: Vec<f64> = lphi.iter().map(|x| x - sum_lp / n as f64).collect();

        let rhs_mf = Tensor::<B, 3>::from_data(
            Data::new(
                b_mf.iter().map(|&x| x as f32).collect::<Vec<_>>(),
                Shape::new([batch, n, 1]),
            ),
            &dev,
        );

        let rhs_tensor = TopologicalLaplacian::scalar_laplacian(
            phi_true.clone(),
            edges_b1.clone(),
            damage.clone(),
        );
        let rhs_mean = rhs_tensor.clone().sum_dim(1).div_scalar(n as f32);
        let rhs_mf_tensor: Vec<f32> = rhs_tensor.sub(rhs_mean).into_data().convert::<f32>().value;
        let mut max_rhs = 0.0_f32;
        for i in 0..n {
            max_rhs = max_rhs.max((rhs_mf_tensor[i] - b_mf[i] as f32).abs());
        }
        assert!(
            max_rhs < 1e-4_f32,
            "symmetrized dense L·φ should match tensor Laplacian; max|Δb|={max_rhs}"
        );

        let sol = solve_grounded_then_mean_free(&l_sym, &b_mf);
        let phi_direct: Vec<f32> = sol.iter().map(|&x| x as f32).collect();

        let phi_direct_t = Tensor::<B, 3>::from_data(
            Data::new(phi_direct.clone(), Shape::new([batch, n, 1])),
            &dev,
        );
        let lap_dir = TopologicalLaplacian::scalar_laplacian(
            phi_direct_t,
            edges_b1.clone(),
            damage.clone(),
        );
        let lap_dir_mean = lap_dir.clone().sum_dim(1).div_scalar(n as f32);
        let lap_dir_mf = lap_dir.sub(lap_dir_mean);
        let rd = lap_dir_mf.sub(rhs_mf.clone());
        let rd_n = rd.powf_scalar(2.0).sum().sqrt().into_scalar();
        let bn = rhs_mf
            .clone()
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .into_scalar()
            .max(1e-20_f32);
        assert!(
            rd_n / bn < 2e-3_f32,
            "dense grounded reference should satisfy mean-free Lφ≈b; rel={}",
            rd_n / bn
        );

        let phi_cg =
            solve_pressure_phi_jacobi_cg(rhs_mf.clone(), edges_b1.clone(), damage.clone(), batch, n);
        let lap_cg = TopologicalLaplacian::scalar_laplacian(
            phi_cg.clone(),
            edges_b1.clone(),
            damage.clone(),
        );
        let lap_cg_mean = lap_cg.clone().sum_dim(1).div_scalar(n as f32);
        let lap_cg_mf = lap_cg.sub(lap_cg_mean);
        let rc = lap_cg_mf.sub(rhs_mf.clone());
        let rc_n = rc.powf_scalar(2.0).sum().sqrt().into_scalar();
        assert!(
            rc_n / bn < 5e-4_f32,
            "Jacobi-PCG should satisfy mean-free Lφ≈b; rel={}",
            rc_n / bn
        );

        let phi_cg_vals: Vec<f32> = phi_cg.into_data().convert::<f32>().value;
        let mut max_abs = 0.0_f32;
        for i in 0..n {
            max_abs = max_abs.max((phi_cg_vals[i] - phi_direct[i]).abs());
        }
        assert!(
            max_abs < 6e-2_f32,
            "Jacobi-PCG φ should track direct solve; max|Δφ|={max_abs}"
        );
    }

    /// **P1 / verification \#7 — honest regression (`solver-experimental`):** on the same **5×5**
    /// quad channel as `chorin_single_step_finite_smoke`, compare the **legacy** surrogate RHS
    /// \(\sum_c \mathcal{L} u^\*_c\) to the **shipped** mean-free weak primal-divergence RHS, then
    /// Jacobi-PCG solves on \(-\mathcal{L}\). RHS differ; both reaches stay finite with modest residuals;
    /// \(\phi\) is not identical across RHS choices.
    #[cfg(feature = "solver-experimental")]
    #[test]
    fn chorin_poisson_rhs_surrogate_vs_weak_divergence_tiny_channel() {
        use super::{chorin_pressure_rhs_mean_free_weak_divergence, solve_pressure_phi_jacobi_cg};
        use crate::physics::laplacian::TopologicalLaplacian;
        use crate::physics::topology::EdgeTopology;

        let dev = NdArrayDevice::Cpu;
        let nx = 5usize;
        let ny = 5usize;
        let n = nx * ny;

        let mut edges_src: Vec<i64> = Vec::new();
        let mut edges_tgt: Vec<i64> = Vec::new();
        for j in 0..ny {
            for i in 0..nx - 1 {
                edges_src.push((j * nx + i) as i64);
                edges_tgt.push((j * nx + i + 1) as i64);
            }
        }
        for j in 0..ny - 1 {
            for i in 0..nx {
                edges_src.push((j * nx + i) as i64);
                edges_tgt.push(((j + 1) * nx + i) as i64);
            }
        }
        let mut edges = edges_src;
        edges.extend(edges_tgt);
        let e_ct = edges.len() / 2;
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();
        let batch = 1usize;

        let mut udat = vec![0.0_f32; batch * n * 3];
        for (idx, u) in udat.iter_mut().enumerate() {
            let k = idx / 3;
            let c = idx % 3;
            let node = k % n;
            let base = ((node * 13 + c * 7) % 97) as f32 * 1e-2;
            *u = base + (c as f32) * 0.03;
        }
        let u_star = Tensor::<B, 3>::from_data(Data::new(udat, Shape::new([batch, n, 3])), &dev);
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let flow_coeff = Tensor::<B, 3>::ones([batch, n_edges, 3], &dev);

        let (rhs_div, _t_hat) = chorin_pressure_rhs_mean_free_weak_divergence(
            u_star.clone(),
            &topo,
            flow_coeff.clone(),
            batch,
            n_edges,
            n,
        );

        let mut rhs_surr = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        for c in 0..3 {
            let uc = u_star.clone().narrow(2, c, 1);
            let lap_c =
                TopologicalLaplacian::scalar_laplacian(uc, edges_b1.clone(), damage.clone());
            rhs_surr = rhs_surr.add(lap_c);
        }
        let sm = rhs_surr.clone().sum_dim(1).div_scalar(n as f32);
        let rhs_surr = rhs_surr.sub(sm.reshape([batch, 1, 1]));

        let dnorm = rhs_div
            .clone()
            .sub(rhs_surr.clone())
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .into_scalar();
        assert!(
            dnorm > 1e-8_f32,
            "expected surrogate vs divergence RHS to differ; ||Δb||={dnorm}"
        );

        let phi_div = solve_pressure_phi_jacobi_cg(
            rhs_div.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );
        let phi_surr = solve_pressure_phi_jacobi_cg(
            rhs_surr.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );

        let rel_res = |phi: Tensor<B, 3>, rhs: Tensor<B, 3>| -> f32 {
            let lap_phi =
                TopologicalLaplacian::scalar_laplacian(phi, edges_b1.clone(), damage.clone());
            let rn = lap_phi
                .sub(rhs.clone())
                .powf_scalar(2.0)
                .sum()
                .sqrt()
                .into_scalar();
            let bn = rhs
                .powf_scalar(2.0)
                .sum()
                .sqrt()
                .into_scalar()
                .max(1e-30_f32);
            rn / bn
        };

        let rd = rel_res(phi_div.clone(), rhs_div);
        let rs = rel_res(phi_surr.clone(), rhs_surr);
        assert!(
            rd < 1e-2_f32 && rs < 1e-2_f32,
            "Jacobi-PCG residuals too large: div_rhs rel={rd}, surr_rhs rel={rs}"
        );

        let inorm = phi_div
            .clone()
            .sub(phi_surr.clone())
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .into_scalar();
        assert!(
            inorm > 1e-8_f32,
            "expected different φ for different RHS; ||Δφ||={inorm}"
        );

        assert!(phi_div.abs().max().into_scalar().is_finite());
        assert!(phi_surr.abs().max().into_scalar().is_finite());
    }
}

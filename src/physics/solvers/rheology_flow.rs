// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![allow(clippy::single_range_in_vec_init)]

//! Bingham / projection CFD on the graph (experimental `rheology-bingham`).
//!
//! ## Audit memo (Track E)
//! - **Steady vs transient:** [`plane_bingham_poiseuille_u`](crate::physics::rheology_analytic) is the
//!   **steady** parallel-plate reference; [`BinghamFlowSolver::step`] is an explicit Chorin split on a
//!   graph — no claim of convergence to that steady profile without inlet/outlet BCs (and typically a
//!   staggered MAC pressure) even though the pressure increment solves \(\mathcal{L}\phi=b_h\) with **Jacobi-PCG**
//!   (see open roadmap items in `tests/verification/rheology_poiseuille.rs`).
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
//!    **open** for a MAC / staggered RHS pairing (see `docs/research/rheology_pressure_poisson_roadmap.md` §2–4).
//!    \(\mathcal{L}\) remains the graph Laplacian — not a MAC staggered operator — until that lane lands.
//!    The linear solve is **Jacobi-preconditioned CG** on \(A=-\mathcal{L}\), \(b=-b_h\) (SPD on the mean-free
//!    subspace), early exit when \(\|r\|_2/\|b\|_2\) satisfies `Tensor::all_close` against zero at **`POISSON_CG_REL_TOL`**
//!    (no explicit host scalar reads in this module for the gate — Burn may still sync internally for `all_close`), iteration cap — then \(\phi\) is shifted to **zero mean** per batch
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
//! ## R2.2 — Honest scope (OPEN ROADMAP ITEM — Rheology, Track J)
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
//! CI brackets the resulting step-0→1 amplification in `chorin_jacobi_pcg_step_velocity_amplification_regression_guard`
//! (historical name).
//! Steady vs analytic comparisons stay open until inlet/outlet BCs plus a MAC or cell-centred pressure solve land.
//!
//! ## MAC + Poisson — integration points (R2.2, design note)
//! Ring 2 **R2.2** calls for MAC or cell-centred pressure on the developed channel; this module ships **graph**
//! **Jacobi-PCG** on \(-\mathcal{L}\) (`Tensor::all_close` residual gate, capped iterations) with the \#7 projection path below.
//! The following are **hook points** for a future staggered / incompressible-correct split — **not** an implemented MAC grid:
//! - **After the predictor:** `step_experimental` forms `u_star` from explicit momentum (body, viscous,
//!   pressure-gradient acceleration). A MAC predictor would typically commit **face-normal** provisional
//!   fluxes here; today everything stays nodal on `edges_b1`.
//! - **Poisson RHS:** Shipped path uses [`primal_divergence_from_edge_flux_topo`] on scalar flux \(q_e f_c\)
//!   derived from \(u^\*\) (tangential mean; see “Chorin-style split” §2). A MAC staggered \(\nabla_h\!\cdot u^\*\)
//!   on face fluxes remains a future swap-in. **M7 milestone (both `rheology-bingham` and `solver-experimental`):**
//!   [`chorin_pressure_rhs_mean_free_weak_divergence_mac_upstream_face_flux`] and
//!   [`chorin_open_x_chain_end_cap_flux_rhs_mean_free`] are small, opt-in building blocks toward that swap-in /
//!   open-\(x\) data — not yet wired into [`step_experimental`].
//! - **Poisson solve:** Shipped path uses **Jacobi-preconditioned CG** on \(-\mathcal{L}\) (see
//!   [`solve_pressure_phi_jacobi_cg`]); a chain **Thomas** fast lane when topology is a 1-D path remains a future
//!   swap-in — compare electrochemistry Poisson helpers.
//! - **Projection:** Edge increments [`primal_scalar_edge_increment`] / tangent projection of \(\nabla\phi\)
//!   remain the right **shape** once \(\phi\) solves the consistent discrete Poisson; **inlet/outlet** pressure
//!   or flux BCs still require explicit pinning — absent today.
//!
//! **Scope:** Wiring **2D channel MAC + consistent divergence BCs** is **not** a small patch on this scaffold
//! (well beyond a sub-hundred-line swap). Treat the bullets above as **documentation of insertion points** until
//! a dedicated pressure solve + boundary module ships — see **OPEN ROADMAP ITEM — Rheology** in `docs/Solver-Status.md`.
//!
//! # Honest boundary (W29-079)
//!
//! Graph Chorin + Jacobi-PCG + weak primal-divergence RHS + Roussel λ are the **measured** research lane under
//! `rheology-bingham`. M7 MAC-upstream / open-x end-cap helpers are **building blocks**, not wired into
//! [`BinghamFlowSolver::step`]. Unit contracts: `cargo test -p umst-manifold rheology_flow`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5.

/// W29 deepen cell — rheology flow honest fence bundle.
pub const W29_RHEOLOGY_FLOW_DEEPEN_CELL: &str = "W29-079-RHEOLOGY_FLOW";

/// Honest posture tag — graph Chorin Bingham research lane; fleet production wiring refused.
pub const RHEOLOGY_FLOW_POSTURE_TAG: &str = "honest-rheology-chorin-bingham-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const RHEOLOGY_FLOW_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by graph Chorin / Bingham research helpers alone.
pub const RHEOLOGY_FLOW_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const RHEOLOGY_FLOW_MASTER: bool = false;

/// OP-5 composition pin — not claimed by this module.
pub const RHEOLOGY_FLOW_OP5_WIRED: bool = false;

/// Graph Chorin Jacobi-PCG pressure increment + weak primal-divergence RHS landed in this module.
pub const RHEOLOGY_FLOW_CHORIN_JACOBI_PCG_LANDED: bool = true;

/// Roussel λ thixotropy ODE + τ₀ edge scaling landed (explicit; frozen defaults keep legacy path).
pub const RHEOLOGY_FLOW_ROUSSEL_THIX_LANDED: bool = true;

/// M7 MAC-upstream / open-x end-cap helpers exist as opt-in building blocks (not step-wired).
pub const RHEOLOGY_FLOW_M7_BUILDING_BLOCKS_LANDED: bool = true;

/// M7 helpers are **not** wired into [`BinghamFlowSolver::step`] / `step_experimental`.
pub const RHEOLOGY_FLOW_M7_WIRED_INTO_STEP: bool = false;

/// Full MAC staggered pressure + developed 2D Poiseuille CI certification — still open.
pub const RHEOLOGY_FLOW_MAC_STAGGERED_PRESSURE: bool = false;

/// Plane Bingham / Newtonian Poiseuille CI-certified on the developed channel — still open.
pub const RHEOLOGY_FLOW_PLANE_POISEUILLE_CI_CERTIFIED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const RHEOLOGY_FLOW_HONEST_FENCE: &str =
    "chorin_jacobi_pcg_weak_div_landed=true roussel_thix_landed=true m7_building_blocks_landed=true m7_wired_into_step=false mac_staggered_pressure=false plane_poiseuille_ci_certified=false production_wired=false master_composition_wired=false op5_wired=false physics_green=false";

const _: () = assert!(!RHEOLOGY_FLOW_PRODUCTION_WIRED);
const _: () = assert!(!RHEOLOGY_FLOW_PHYSICS_GREEN);
const _: () = assert!(!RHEOLOGY_FLOW_MASTER);
const _: () = assert!(!RHEOLOGY_FLOW_OP5_WIRED);
const _: () = assert!(!RHEOLOGY_FLOW_M7_WIRED_INTO_STEP);
const _: () = assert!(!RHEOLOGY_FLOW_MAC_STAGGERED_PRESSURE);
const _: () = assert!(!RHEOLOGY_FLOW_PLANE_POISEUILLE_CI_CERTIFIED);
const _: () = assert!(RHEOLOGY_FLOW_CHORIN_JACOBI_PCG_LANDED);
const _: () = assert!(RHEOLOGY_FLOW_ROUSSEL_THIX_LANDED);
const _: () = assert!(RHEOLOGY_FLOW_M7_BUILDING_BLOCKS_LANDED);

/// Typed probe for rheology-flow posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RheologyFlowPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5_wired: bool,
    pub chorin_jacobi_pcg_landed: bool,
    pub roussel_thix_landed: bool,
    pub m7_building_blocks_landed: bool,
    pub m7_wired_into_step: bool,
    pub mac_staggered_pressure: bool,
    pub plane_poiseuille_ci_certified: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for graph Chorin Bingham rheology flow.
#[must_use]
pub fn rheology_flow_honest_posture_bundle() -> RheologyFlowPostureProbe {
    RheologyFlowPostureProbe {
        physics_green: RHEOLOGY_FLOW_PHYSICS_GREEN,
        production_wired: RHEOLOGY_FLOW_PRODUCTION_WIRED,
        master: RHEOLOGY_FLOW_MASTER,
        op5_wired: RHEOLOGY_FLOW_OP5_WIRED,
        chorin_jacobi_pcg_landed: RHEOLOGY_FLOW_CHORIN_JACOBI_PCG_LANDED,
        roussel_thix_landed: RHEOLOGY_FLOW_ROUSSEL_THIX_LANDED,
        m7_building_blocks_landed: RHEOLOGY_FLOW_M7_BUILDING_BLOCKS_LANDED,
        m7_wired_into_step: RHEOLOGY_FLOW_M7_WIRED_INTO_STEP,
        mac_staggered_pressure: RHEOLOGY_FLOW_MAC_STAGGERED_PRESSURE,
        plane_poiseuille_ci_certified: RHEOLOGY_FLOW_PLANE_POISEUILLE_CI_CERTIFIED,
        honest_fence: RHEOLOGY_FLOW_HONEST_FENCE,
        posture_tag: RHEOLOGY_FLOW_POSTURE_TAG,
        deepen_cell: W29_RHEOLOGY_FLOW_DEEPEN_CELL,
    }
}

/// Research lane landed with production/master/GREEN/OP-5/MAC-CI composition honestly open.
#[must_use]
pub fn rheology_flow_posture_honest(probe: &RheologyFlowPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5_wired
        && probe.chorin_jacobi_pcg_landed
        && probe.roussel_thix_landed
        && probe.m7_building_blocks_landed
        && !probe.m7_wired_into_step
        && !probe.mac_staggered_pressure
        && !probe.plane_poiseuille_ci_certified
        && probe
            .honest_fence
            .contains("chorin_jacobi_pcg_weak_div_landed=true")
        && probe.honest_fence.contains("m7_wired_into_step=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 claims on the rheology-flow surface.
#[must_use]
pub fn rheology_flow_refuse_overclaim(
    probe: &RheologyFlowPostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("RHEOLOGY_FLOW_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("RHEOLOGY_FLOW_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("RHEOLOGY_FLOW_MASTER must stay false — not claimed by graph Chorin alone");
    }
    if probe.op5_wired {
        return Err("RHEOLOGY_FLOW_OP5_WIRED must stay false — not claimed by this module");
    }
    if probe.m7_wired_into_step {
        return Err("RHEOLOGY_FLOW_M7_WIRED_INTO_STEP must stay false until step wiring lands");
    }
    if probe.mac_staggered_pressure || probe.plane_poiseuille_ci_certified {
        return Err("MAC / Poiseuille CI flags must stay false until that lane ships");
    }
    if !rheology_flow_posture_honest(probe) {
        return Err("rheology_flow posture fence inconsistent");
    }
    Ok(())
}

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

#[cfg(feature = "rheology-bingham")]
use crate::core::iterate_until::iterate_until;
#[cfg(feature = "rheology-bingham")]
use core::ops::ControlFlow;

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{Field, NodalDensityField, ScalarPressureField, VelocityField};

use crate::physics::error::PhysicsError;

/// Chorin Bingham step output: velocity, pressure, Roussel λ nodal fields `[B, N, *]`.
type RheologyStepOut<B> = (Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>);

#[cfg(feature = "rheology-bingham")]
/// Relative residual tolerance \(\|r\|_2/\|b\|_2\) for early exit in Jacobi-PCG (checked with `Tensor::all_close`).
const POISSON_CG_REL_TOL: f32 = 2e-5;
#[cfg(feature = "rheology-bingham")]
/// Upper bound on PCG iterations per Chorin pressure step (graph Laplacian; Jacobi preconditioner).
const POISSON_CG_MAX_IT_CAP: usize = 4096;
#[cfg(feature = "rheology-bingham")]
/// Floor for \(t_\mathrm{rest}\), \(\gamma_\mathrm{crit}\) in denominators (SI scales, avoids div-by-zero).
const THIX_PARAM_EPS: f32 = 1e-12;

#[cfg(feature = "rheology-bingham")]
struct RichardsonPressurePhiState<B: Backend<FloatElem = f32>> {
    phi: Tensor<B, 3>,
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
    omega: f32,
}

#[cfg(feature = "rheology-bingham")]
struct JacobiPressurePhiState<B: Backend<FloatElem = f32>> {
    phi: Tensor<B, 3>,
    r: Tensor<B, 3>,
    z: Tensor<B, 3>,
    p: Tensor<B, 3>,
    rz_old: Tensor<B, 1>,
    rhs_norm: Tensor<B, 1>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    diag_inv: Tensor<B, 3>,
    batch: usize,
    n: usize,
}

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
    /// Returns inputs unchanged (documented no-op placeholder for downstream wiring tests).
    ///
    /// ## `--features solver-experimental`
    /// Runs the documented Chorin **baseline** fractional-step projection in this module and one explicit Roussel step on \(\lambda\).
    /// Canonical field-wrapped Chorin ingress (R25).
    #[allow(clippy::too_many_arguments)]
    pub fn step_from_fields<B: Backend<FloatElem = f32>>(
        &self,
        velocity: VelocityField<B>,
        pressure: ScalarPressureField<B>,
        yield_stress: ScalarPressureField<B>,
        density: NodalDensityField<B>,
        lambda_thix: ScalarPressureField<B>,
        edges_b1: Tensor<B, 2, Int>,
        gravity: Tensor<B, 1>,
    ) -> Result<RheologyStepOut<B>, PhysicsError> {
        self.step(
            velocity.into_tensor(),
            pressure.into_tensor(),
            yield_stress.into_tensor(),
            density.into_tensor(),
            lambda_thix.into_tensor(),
            edges_b1,
            gravity,
        )
    }

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
    ) -> Result<RheologyStepOut<B>, PhysicsError> {
        #[cfg(not(feature = "rheology-bingham"))]
        {
            Ok((velocity, pressure, lambda_thix))
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
/// With **`--features rheology_poisson_richardson_fallback`**, the pressure increment uses a damped
/// **Richardson** sweep instead of Jacobi-PCG (compile-time lane). The default Jacobi-PCG path runs a
/// **fixed iteration count** on a lazy tensor graph, with early exit when the \(\ell_2\) relative residual
/// `Tensor::all_close` check against **`POISSON_CG_REL_TOL`** succeeds (no explicit host scalar reads in this module).
#[cfg(feature = "rheology-bingham")]
#[cfg_attr(
    not(feature = "rheology_poisson_richardson_fallback"),
    allow(dead_code)
)]
fn solve_pressure_phi_richardson_fallback<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
) -> Tensor<B, 3> {
    let lambda_upper = 12.0_f32;
    let omega = (1.35_f32 / lambda_upper).clamp(0.02_f32, 0.12_f32);
    let max_it = n.saturating_mul(64).clamp(1024, 16000);
    let mut st = RichardsonPressurePhiState {
        phi: Tensor::<B, 3>::zeros_like(&rhs),
        rhs,
        edges_b1,
        damage,
        batch,
        n,
        omega,
    };
    let _ = iterate_until(max_it, &mut st, |s| {
        let lphi = TopologicalLaplacian::scalar_laplacian(
            s.phi.clone(),
            s.edges_b1.clone(),
            s.damage.clone(),
        );
        let r = s.rhs.clone().sub(lphi);
        s.phi = s.phi.clone().add(r.mul_scalar(s.omega));
        let pm = s
            .phi
            .clone()
            .sum_dim(1)
            .div_scalar(s.n as f32)
            .reshape([s.batch, 1, 1]);
        s.phi = s.phi.clone().sub(pm);
        ControlFlow::Continue(())
    });
    let phi = st.phi;
    let phi_mean = phi
        .clone()
        .sum_dim(1)
        .div_scalar(n as f32)
        .reshape([batch, 1, 1]);
    phi.sub(phi_mean)
}

#[cfg(feature = "rheology-bingham")]
fn jacobi_pressure_phi_step<B: Backend<FloatElem = f32>>(
    s: &mut JacobiPressurePhiState<B>,
) -> ControlFlow<(), ()> {
    let lp =
        TopologicalLaplacian::scalar_laplacian(s.p.clone(), s.edges_b1.clone(), s.damage.clone());
    let ap = lp.neg();
    let p_ap = s.p.clone().mul(ap.clone()).sum().clamp_min(1e-40_f32);
    let alpha = s.rz_old.clone().div(p_ap).clamp(-1e4_f32, 1e4_f32);
    let alpha3 = alpha.clone().reshape([1, 1, 1]).expand([s.batch, s.n, 1]);
    s.phi = s.phi.clone().add(s.p.clone().mul(alpha3.clone()));
    s.r = s.r.clone().sub(ap.mul(alpha3));

    let res_l2 = s.r.clone().powf_scalar(2.0).sum().sqrt();
    let rel = res_l2.div(s.rhs_norm.clone());
    let ztol = Tensor::<B, 1>::zeros_like(&rel);
    if rel.all_close(ztol, None, Some(f64::from(POISSON_CG_REL_TOL))) {
        return ControlFlow::Break(());
    }

    s.z = s.r.clone().mul(s.diag_inv.clone());
    let rz_new = s.r.clone().mul(s.z.clone()).sum();
    let beta = rz_new
        .clone()
        .div(s.rz_old.clone().clamp_min(1e-40_f32))
        .clamp(0.0_f32, 1e6_f32);
    let beta3 = beta.clone().reshape([1, 1, 1]).expand([s.batch, s.n, 1]);
    s.p = s.z.clone().add(s.p.clone().mul(beta3));
    s.rz_old = rz_new.clamp_min(1e-40_f32);
    ControlFlow::Continue(())
}

#[cfg(feature = "rheology-bingham")]
fn solve_pressure_phi_jacobi_cg<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    batch: usize,
    n: usize,
) -> Tensor<B, 3> {
    #[cfg(feature = "rheology_poisson_richardson_fallback")]
    {
        return solve_pressure_phi_richardson_fallback(
            rhs.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );
    }

    let rhs_norm = rhs
        .clone()
        .powf_scalar(2.0)
        .sum()
        .sqrt()
        .clamp_min(1e-30_f32);

    let diag_a =
        TopologicalLaplacian::scalar_laplacian_neg_opposite_diag(edges_b1.clone(), damage.clone());
    let diag_inv = diag_a.clamp_min(1e-14_f32).recip();

    let phi = Tensor::<B, 3>::zeros_like(&rhs);
    let lphi =
        TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
    let r = lphi.sub(rhs.clone());
    let z = r.clone().mul(diag_inv.clone());
    let p = z.clone();
    let rz_old = r.clone().mul(z.clone()).sum().clamp_min(1e-40_f32);

    let max_it = n.saturating_mul(10).clamp(256, POISSON_CG_MAX_IT_CAP);

    let mut st = JacobiPressurePhiState {
        phi,
        r,
        z,
        p,
        rz_old,
        rhs_norm,
        edges_b1,
        damage,
        diag_inv,
        batch,
        n,
    };
    let _ = iterate_until(max_it, &mut st, jacobi_pressure_phi_step);

    let phi = st.phi;
    let phi_mean = phi
        .clone()
        .sum_dim(1)
        .div_scalar(st.n as f32)
        .reshape([st.batch, 1, 1]);
    phi.sub(phi_mean)
}

/// Shipped Chorin pressure Poisson RHS (verification \#7): mean-free weak primal divergence of
/// scalar tangential mean flux \(q_e=(\bar u^\*\!\cdot\hat t)f_c\) (see module rustdoc §2).
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
    chorin_pressure_rhs_mean_free_weak_divergence_inner(
        u_star, topo, flow_coeff, batch, n_edges, n, false,
    )
}

/// **M7 (incremental MAC lane):** same weak primal divergence routing as
/// [`chorin_pressure_rhs_mean_free_weak_divergence`], but scalar edge transport uses the **upstream
/// (source-node) face value** \(q_e=(u^\*_{\mathrm{src}}\cdot\hat t)\,f_c\) instead of the tangential
/// mean \((\bar u^\*\cdot\hat t)\,f_c\). Documented proxy for staggered face-stored normal flux on
/// quasi-1-D channels; the shipped Chorin step still uses the tangential-mean contract.
#[cfg(all(feature = "rheology-bingham", feature = "solver-experimental"))]
#[allow(dead_code)]
pub(crate) fn chorin_pressure_rhs_mean_free_weak_divergence_mac_upstream_face_flux<
    B: Backend<FloatElem = f32>,
>(
    u_star: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
    flow_coeff: Tensor<B, 3>,
    batch: usize,
    n_edges: usize,
    n: usize,
) -> (Tensor<B, 3>, Tensor<B, 3>) {
    chorin_pressure_rhs_mean_free_weak_divergence_inner(
        u_star, topo, flow_coeff, batch, n_edges, n, true,
    )
}

/// **M7 (incremental open-\(x\) lane):** mean-free nodal source from a **balanced end-cap flux**
/// \(+J\) at `left_id` and \(-J\) at `right_id` (same scatter sign story as a single oriented edge
/// in [`primal_divergence_from_edge_flux_topo`]). Row-sum is already zero, so the mean-free pass is
/// a no-op for one batch; it remains the correct gauge hook when composing with other mean-centred RHS pieces.
#[cfg(all(feature = "rheology-bingham", feature = "solver-experimental"))]
#[allow(dead_code)]
pub(crate) fn chorin_open_x_chain_end_cap_flux_rhs_mean_free<B: Backend<FloatElem = f32>>(
    batch: usize,
    n_nodes: usize,
    left_id: usize,
    right_id: usize,
    cap_flux: f32,
    device: &B::Device,
) -> Tensor<B, 3> {
    debug_assert!(left_id < n_nodes && right_id < n_nodes);
    let mut vals = vec![0.0_f32; batch * n_nodes];
    for b in 0..batch {
        vals[b * n_nodes + left_id] += cap_flux;
        vals[b * n_nodes + right_id] -= cap_flux;
    }
    let t = Tensor::<B, 3>::from_data(
        burn::tensor::Data::new(vals, burn::tensor::Shape::new([batch, n_nodes, 1])),
        device,
    );
    let mean = t.clone().sum_dim(1).div_scalar(n_nodes as f32);
    t.sub(mean)
}

#[cfg(feature = "rheology-bingham")]
fn chorin_pressure_rhs_mean_free_weak_divergence_inner<B: Backend<FloatElem = f32>>(
    u_star: Tensor<B, 3>,
    topo: &EdgeTopology<B>,
    flow_coeff: Tensor<B, 3>,
    batch: usize,
    n_edges: usize,
    n: usize,
    mac_upstream_face_flux: bool,
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
    let t_hat_s = du_s.div(du_s_mag3.clone());

    let src_ix3 = topo.expand_src_gather_indices(batch, ch3);
    let tgt_ix3 = topo.expand_tgt_gather_indices(batch, ch3);
    let u_src3 = u_star.clone().gather(1, src_ix3);
    let q_edge = if mac_upstream_face_flux {
        u_src3
            .mul(t_hat_s.clone())
            .sum_dim(2)
            .reshape([batch, n_edges, 1])
    } else {
        let u_tgt3 = u_star.clone().gather(1, tgt_ix3);
        let u_mean_edge = u_src3.add(u_tgt3).div_scalar(2.0_f32);
        u_mean_edge
            .mul(t_hat_s.clone())
            .sum_dim(2)
            .reshape([batch, n_edges, 1])
    };
    let fc1 = flow_coeff.narrow(2, 0, 1);
    let flux_scalar_edge = q_edge.mul(fc1);
    let u_star_x0 = u_star.narrow(2, 0, 1);
    let rhs = primal_divergence_from_edge_flux_topo(flux_scalar_edge, topo, &u_star_x0);
    let rhs_mean = rhs.clone().sum_dim(1).div_scalar(n as f32);
    let rhs = rhs.sub(rhs_mean);
    (rhs, t_hat_s)
}

/// Legacy pre–\#7 RHS: \(\mathcal{L}u^\*_x+\mathcal{L}u^\*_y+\mathcal{L}u^\*_z\) (NOT a discrete divergence), mean-free.
#[cfg(feature = "rheology-bingham")]
#[allow(dead_code)]
fn chorin_pressure_rhs_mean_free_surrogate_sum_laplacian<B: Backend<FloatElem = f32>>(
    u_star: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    n: usize,
) -> Tensor<B, 3> {
    let lx = TopologicalLaplacian::scalar_laplacian(
        u_star.clone().narrow(2, 0, 1),
        edges_b1.clone(),
        damage.clone(),
    );
    let ly = TopologicalLaplacian::scalar_laplacian(
        u_star.clone().narrow(2, 1, 1),
        edges_b1.clone(),
        damage.clone(),
    );
    let lz = TopologicalLaplacian::scalar_laplacian(
        u_star.narrow(2, 2, 1),
        edges_b1.clone(),
        damage.clone(),
    );
    let rhs = lx.add(ly).add(lz);
    let rhs_mean = rhs.clone().sum_dim(1).div_scalar(n as f32);
    rhs.sub(rhs_mean)
}

#[cfg(feature = "rheology-bingham")]
fn bingham_step_validate_solver(solver: &BinghamFlowSolver) -> Result<(), PhysicsError> {
    if !solver.dt.is_finite() || solver.dt <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "BinghamFlowSolver: dt must be positive and finite".to_string(),
        });
    }
    if !solver.mu_plastic.is_finite() || solver.mu_plastic <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "BinghamFlowSolver: mu_plastic must be positive and finite".to_string(),
        });
    }
    if !solver.edge_length_scale.is_finite() || solver.edge_length_scale <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "BinghamFlowSolver: edge_length_scale must be positive and finite".to_string(),
        });
    }
    if !solver.t_rest_thix.is_finite() || solver.t_rest_thix <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "BinghamFlowSolver: t_rest_thix must be positive and finite".to_string(),
        });
    }
    if !solver.gamma_crit_thix.is_finite() || solver.gamma_crit_thix <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "BinghamFlowSolver: gamma_crit_thix must be positive and finite".to_string(),
        });
    }
    Ok(())
}

#[cfg(feature = "rheology-bingham")]
fn bingham_step_validate_shapes<B: Backend<FloatElem = f32>>(
    velocity: &Tensor<B, 3>,
    pressure: &Tensor<B, 3>,
    yield_stress: &Tensor<B, 3>,
    density: &Tensor<B, 3>,
    lambda_thix: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    gravity: &Tensor<B, 1>,
) -> Result<(), PhysicsError> {
    let [batch, n, c3] = velocity.dims();
    if c3 != 3 {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "velocity must be [B, N, 3]",
        });
    }
    if batch == 0 || n == 0 {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "batch and node counts must be > 0",
        });
    }
    let nodal_shape = [batch, n, 1];
    if pressure.dims() != nodal_shape {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "pressure must be [B, N, 1]",
        });
    }
    if yield_stress.dims() != nodal_shape {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "yield_stress must be [B, N, 1]",
        });
    }
    if density.dims() != nodal_shape {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "density must be [B, N, 1]",
        });
    }
    if lambda_thix.dims() != nodal_shape {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "lambda_thix must be [B, N, 1]",
        });
    }
    let [edge_rank, e_ct] = edges_b1.dims();
    if edge_rank != 2 || e_ct == 0 {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "edges_b1 must be [2, E] with E > 0",
        });
    }
    if gravity.dims() != [3] {
        return Err(PhysicsError::ShapeMismatch {
            context: "BinghamFlowSolver::step",
            detail: "gravity must be length 3",
        });
    }
    Ok(())
}

#[cfg(feature = "rheology-bingham")]
fn bingham_tensor_batch_mean_finite<B: Backend<FloatElem = f32>>(
    tensor: &Tensor<B, 3>,
    context: &'static str,
) -> Result<f32, PhysicsError> {
    let value: f32 = tensor.clone().mean().into_scalar();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(PhysicsError::NonFinite { context })
    }
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
) -> Result<RheologyStepOut<B>, PhysicsError> {
    bingham_step_validate_solver(solver)?;
    bingham_step_validate_shapes(
        &velocity,
        &pressure,
        &yield_stress,
        &density,
        &lambda_thix,
        &edges_b1,
        &gravity,
    )?;
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

    bingham_tensor_batch_mean_finite(&velocity_new, "BinghamFlowSolver: velocity after step")?;
    bingham_tensor_batch_mean_finite(&pressure_new, "BinghamFlowSolver: pressure after step")?;
    bingham_tensor_batch_mean_finite(&lambda_new, "BinghamFlowSolver: lambda after step")?;

    Ok((velocity_new, pressure_new, lambda_new))
}

#[cfg(test)]
mod honest_fence_tests {
    use super::{
        rheology_flow_honest_posture_bundle, rheology_flow_posture_honest,
        rheology_flow_refuse_overclaim, BinghamFlowSolver, RheologyFlowPostureProbe,
        RHEOLOGY_FLOW_CHORIN_JACOBI_PCG_LANDED, RHEOLOGY_FLOW_HONEST_FENCE,
        RHEOLOGY_FLOW_M7_WIRED_INTO_STEP, RHEOLOGY_FLOW_MASTER, RHEOLOGY_FLOW_OP5_WIRED,
        RHEOLOGY_FLOW_PHYSICS_GREEN, RHEOLOGY_FLOW_PLANE_POISEUILLE_CI_CERTIFIED,
        RHEOLOGY_FLOW_PRODUCTION_WIRED, W29_RHEOLOGY_FLOW_DEEPEN_CELL,
    };

    #[test]
    fn rheology_flow_honest_posture_refuses_green_production_master_op5() {
        let probe = rheology_flow_honest_posture_bundle();
        assert!(rheology_flow_posture_honest(&probe));
        assert!(rheology_flow_refuse_overclaim(&probe).is_ok());
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5_wired);
        assert!(!probe.m7_wired_into_step);
        assert!(!probe.mac_staggered_pressure);
        assert!(!probe.plane_poiseuille_ci_certified);
        assert!(probe.chorin_jacobi_pcg_landed);
        assert!(probe.roussel_thix_landed);
        assert!(probe.m7_building_blocks_landed);
        assert_eq!(probe.deepen_cell, W29_RHEOLOGY_FLOW_DEEPEN_CELL);
        assert!(RHEOLOGY_FLOW_HONEST_FENCE.contains("physics_green=false"));
        assert!(RHEOLOGY_FLOW_CHORIN_JACOBI_PCG_LANDED);
        assert!(!RHEOLOGY_FLOW_M7_WIRED_INTO_STEP);
        assert!(!RHEOLOGY_FLOW_PHYSICS_GREEN);
        assert!(!RHEOLOGY_FLOW_PRODUCTION_WIRED);
        assert!(!RHEOLOGY_FLOW_MASTER);
        assert!(!RHEOLOGY_FLOW_OP5_WIRED);
        assert!(!RHEOLOGY_FLOW_PLANE_POISEUILLE_CI_CERTIFIED);
    }

    #[test]
    fn rheology_flow_refuse_overclaim_rejects_tampered_green() {
        let mut probe = rheology_flow_honest_posture_bundle();
        probe.physics_green = true;
        assert!(rheology_flow_refuse_overclaim(&probe).is_err());
        assert!(!rheology_flow_posture_honest(&probe));
    }

    #[test]
    fn rheology_flow_default_solver_constructs_with_frozen_thix() {
        let s = BinghamFlowSolver::default();
        assert!(s.dt > 0.0 && s.dt.is_finite());
        assert!(s.mu_plastic > 0.0 && s.mu_plastic.is_finite());
        assert_eq!(s.t_rest_thix, BinghamFlowSolver::T_REST_NO_THIX);
        assert_eq!(s.gamma_crit_thix, BinghamFlowSolver::GAMMA_CRIT_NO_THIX);
        // Compile-time fence constants stay false regardless of solver construction.
        let _probe: RheologyFlowPostureProbe = rheology_flow_honest_posture_bundle();
        assert!(rheology_flow_refuse_overclaim(&_probe).is_ok());
    }
}

#[cfg(all(test, feature = "rheology-bingham"))]
mod tests {
    use super::BinghamFlowSolver;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    #[test]
    fn chorin_single_step_finite_smoke_two_node_edge() {
        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 2usize;
        let e_ct = 1usize;
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, e_ct])), &dev);
        let velocity = Tensor::<B, 3>::from_data(
            Data::new(
                vec![0.1_f32, 0.0, 0.0, 0.2, 0.0, 0.0],
                Shape::new([batch, n, 3]),
            ),
            &dev,
        );
        let pressure = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let yield_stress = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let density = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let lambda0 = Tensor::<B, 3>::ones([batch, n, 1], &dev);
        let gravity =
            Tensor::<B, 1>::from_data(Data::new(vec![0.0_f32, -9.81, 0.0], Shape::new([3])), &dev);

        let solver = BinghamFlowSolver::new(1e-3, 1e-2);
        let (v1, p1, lam1) = solver
            .step(
                velocity,
                pressure,
                yield_stress,
                density,
                lambda0,
                edges_b1,
                gravity,
            )
            .expect("finite Chorin Bingham step on 2-node edge");
        let z = Tensor::<B, 3>::zeros_like(&v1);
        assert!(
            v1.clone().all_close(v1.clone(), None, Some(0.0_f64)),
            "velocity must stay finite"
        );
        assert!(
            p1.clone().all_close(p1.clone(), None, Some(0.0_f64)),
            "pressure must stay finite"
        );
        assert!(
            lam1.clone().all_close(lam1.clone(), None, Some(0.0_f64)),
            "lambda must stay finite"
        );
        // Guard against all-zero collapse under gravity+viscous predictor on this tiny graph.
        assert!(
            !v1.abs()
                .sum()
                .all_close(z.abs().sum(), None, Some(1e-30_f64)),
            "expected non-trivial velocity after step"
        );
    }

    #[test]
    fn thix_param_domain_rejects_non_positive() {
        let mut solver = BinghamFlowSolver::new(1e-3, 1.0);
        solver.t_rest_thix = 0.0;
        let err = super::bingham_step_validate_solver(&solver);
        assert!(err.is_err(), "t_rest_thix=0 must Domain-fail");
        solver.t_rest_thix = BinghamFlowSolver::T_REST_NO_THIX;
        solver.gamma_crit_thix = f32::NAN;
        let err2 = super::bingham_step_validate_solver(&solver);
        assert!(err2.is_err(), "NaN gamma_crit_thix must Domain-fail");
    }

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

        let (_v, _p, lam1) = solver
            .step(
                velocity,
                pressure,
                yield_stress,
                density,
                lambda0,
                edges_b1,
                gravity,
            )
            .expect(
                "BinghamFlowSolver::step on 3-node shear chain for Roussel λ breakdown lib unit witness (FP §6 Track E rheology flow)",
            );
        let mid = lam1.clone().slice([0..1, 1..2, 0..1]);
        let one = Tensor::<B, 3>::ones_like(&mid);
        assert!(
            !mid.all_close(one, None, Some(1e-5_f64)),
            "expected breakdown term to reduce λ at interior node"
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
        let (_v, _p, lam1) = solver
            .step(
                velocity,
                pressure,
                yield_stress,
                density,
                lambda0.clone(),
                edges_b1,
                gravity,
            )
            .expect(
                "BinghamFlowSolver::step on 2-node quiescent lattice for default-freeze λ lib unit witness (FP §6 Track E rheology flow)",
            );
        let z = Tensor::<B, 3>::zeros_like(&lam1);
        assert!(
            lam1.sub(lambda0).abs().all_close(z, None, Some(1e-5_f64)),
            "frozen-default λ should be unchanged"
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
        let z1 = Tensor::<B, 1>::zeros([1], &dev);
        let bn = rhs_mf.powf_scalar(2.0).sum().sqrt().clamp_min(1e-20_f32);
        let rel = res.powf_scalar(2.0).sum().sqrt().div(bn);
        assert!(
            rel.all_close(z1, None, Some(5e-4_f64)),
            "Jacobi-PCG should drive ||Lφ−b||/||b|| small"
        );
        assert!(
            phi.clone().all_close(phi.clone(), None, Some(0.0_f64)),
            "expected finite φ"
        );
    }

    /// **P1 / matrix \#7 — honest surrogate vs shipped RHS (experimental):** on the **5×5** channel
    /// lattice used by `chorin_single_step_finite_smoke`, compare legacy \(\sum_c \mathcal{L}u^\*_c\) to the
    /// mean-free **weak primal divergence** RHS feeding the same graph Laplacian + Jacobi-PCG solve.
    /// Asserts both solves converge, stay finite, and the two RHS / \(\phi\) fields are **not** identical
    /// (the surrogate is **not** a discrete divergence).
    #[cfg(feature = "solver-experimental")]
    #[test]
    fn chorin_poisson_rhs_surrogate_vs_weak_divergence_tiny_channel() {
        use super::{
            chorin_pressure_rhs_mean_free_surrogate_sum_laplacian,
            chorin_pressure_rhs_mean_free_weak_divergence, solve_pressure_phi_jacobi_cg,
        };
        use crate::physics::laplacian::TopologicalLaplacian;
        use crate::physics::topology::EdgeTopology;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
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
        assert_eq!(n_edges, e_ct);

        let mut vel = vec![0.0_f32; batch * n * 3];
        for j in 0..ny {
            for i in 0..nx {
                let id = j * nx + i;
                vel[id * 3] = (i as f32) * 0.02 + (j as f32) * 0.01;
                vel[id * 3 + 1] = (i as f32) * -0.015 + (j as f32) * 0.012;
                vel[id * 3 + 2] = 0.003 * (i + j) as f32;
            }
        }
        let u_star = Tensor::<B, 3>::from_data(Data::new(vel, Shape::new([batch, n, 3])), &dev);
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let flow_coeff = Tensor::<B, 3>::ones([batch, n_edges, 3], &dev);

        let rhs_div = chorin_pressure_rhs_mean_free_weak_divergence(
            u_star.clone(),
            &topo,
            flow_coeff,
            batch,
            n_edges,
            n,
        )
        .0;
        let rhs_sur = chorin_pressure_rhs_mean_free_surrogate_sum_laplacian(
            u_star.clone(),
            edges_b1.clone(),
            damage.clone(),
            n,
        );

        let z1 = Tensor::<B, 1>::zeros([1], &dev);

        let bd_t = rhs_div
            .clone()
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .clamp_min(1e-30_f32);
        assert!(
            !bd_t.clone().all_close(z1.clone(), None, Some(1e-12_f64)),
            "expected non-trivial divergence RHS"
        );

        let diff = rhs_sur.clone().sub(rhs_div.clone());
        let dn = diff.powf_scalar(2.0).sum().sqrt();
        let rel_b = dn.div(bd_t.clone());
        let tail_lo = rel_b
            .clone()
            .sub(Tensor::<B, 1>::full([1], 5e-4_f32, &dev))
            .clamp_min(0.0_f32);
        assert!(
            !tail_lo.all_close(z1.clone(), None, Some(1e-12_f64)),
            "legacy surrogate RHS should materially differ from weak-divergence RHS"
        );
        assert!(
            rel_b.all_close(z1.clone(), None, Some(200_f64)),
            "expected RHS disagreement bounded on 5×5"
        );

        let phi_div = solve_pressure_phi_jacobi_cg(
            rhs_div.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );
        let phi_sur = solve_pressure_phi_jacobi_cg(
            rhs_sur.clone(),
            edges_b1.clone(),
            damage.clone(),
            batch,
            n,
        );

        assert!(phi_div
            .clone()
            .all_close(phi_div.clone(), None, Some(0.0_f64)));
        assert!(phi_sur
            .clone()
            .all_close(phi_sur.clone(), None, Some(0.0_f64)));

        let lap_div = TopologicalLaplacian::scalar_laplacian(
            phi_div.clone(),
            edges_b1.clone(),
            damage.clone(),
        );
        let res_div = lap_div.sub(rhs_div);
        let rn_div_t = res_div.powf_scalar(2.0).sum().sqrt().div(bd_t.clone());
        assert!(
            rn_div_t.all_close(z1.clone(), None, Some(1e-2_f64)),
            "φ_div should approximately solve Lφ=b_div"
        );

        let lap_sur = TopologicalLaplacian::scalar_laplacian(
            phi_sur.clone(),
            edges_b1.clone(),
            damage.clone(),
        );
        let bs_t = rhs_sur
            .clone()
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .clamp_min(1e-30_f32);
        let rn_sur_t = lap_sur.sub(rhs_sur).powf_scalar(2.0).sum().sqrt().div(bs_t);
        assert!(
            rn_sur_t.all_close(z1.clone(), None, Some(1e-2_f64)),
            "φ_sur should approximately solve Lφ=b_sur"
        );

        let dphi = phi_sur.sub(phi_div.clone());
        let dphi_n = dphi.powf_scalar(2.0).sum().sqrt();
        let phi_dn = phi_div.powf_scalar(2.0).sum().sqrt().clamp_min(1e-30_f32);
        let rat = dphi_n.div(phi_dn);
        let tail_phi = rat
            .sub(Tensor::<B, 1>::full([1], 1e-4_f32, &dev))
            .clamp_min(0.0_f32);
        assert!(
            !tail_phi.all_close(z1, None, Some(1e-12_f64)),
            "solutions φ should differ when RHS differs"
        );
    }

    /// **M7 — MAC upstream face flux** differs from tangential-mean transport on a short +x chain.
    #[cfg(feature = "solver-experimental")]
    #[test]
    fn m7_mac_upstream_face_flux_rhs_differs_from_tangential_mean_on_chain() {
        use super::{
            chorin_pressure_rhs_mean_free_weak_divergence,
            chorin_pressure_rhs_mean_free_weak_divergence_mac_upstream_face_flux,
        };
        use crate::physics::topology::EdgeTopology;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 5usize;
        let e_ct = 4usize;
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1, 2, 3, 1, 2, 3, 4], Shape::new([2, e_ct])),
            &dev,
        );
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();
        let flow_coeff = Tensor::<B, 3>::ones([batch, n_edges, 3], &dev);

        let mut vel = vec![0.0_f32; batch * n * 3];
        for i in 0..n {
            vel[i * 3] = (i * i) as f32 * 0.1;
            vel[i * 3 + 1] = 0.03 * i as f32;
        }
        let u_star = Tensor::<B, 3>::from_data(Data::new(vel, Shape::new([batch, n, 3])), &dev);

        let rhs_mean = chorin_pressure_rhs_mean_free_weak_divergence(
            u_star.clone(),
            &topo,
            flow_coeff.clone(),
            batch,
            n_edges,
            n,
        )
        .0;
        let rhs_mac = chorin_pressure_rhs_mean_free_weak_divergence_mac_upstream_face_flux(
            u_star, &topo, flow_coeff, batch, n_edges, n,
        )
        .0;

        let mn = rhs_mean
            .clone()
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .clamp_min(1e-30_f32);
        let diff = rhs_mac.sub(rhs_mean);
        let dn = diff.powf_scalar(2.0).sum().sqrt();
        let ratio = dn.div(mn);
        let thr = Tensor::<B, 1>::full([1], 0.02_f32, &dev);
        let excess = ratio.sub(thr).clamp_min(0.0_f32);
        assert!(
            !excess.all_close(Tensor::zeros([1], &dev), None, Some(1e-7_f64)),
            "MAC upstream q_e should materially change the Poisson RHS on this chain"
        );
    }

    /// **M7 — open-\(x\) end-cap flux** is mean-free and perturbs the graph Poisson increment vs the base RHS alone.
    #[cfg(feature = "solver-experimental")]
    #[test]
    fn m7_open_x_end_cap_flux_mean_free_and_shifts_poisson_phi() {
        use super::{
            chorin_open_x_chain_end_cap_flux_rhs_mean_free,
            chorin_pressure_rhs_mean_free_weak_divergence, solve_pressure_phi_jacobi_cg,
        };
        use crate::physics::topology::EdgeTopology;

        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n = 5usize;
        let e_ct = 4usize;
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1, 2, 3, 1, 2, 3, 4], Shape::new([2, e_ct])),
            &dev,
        );
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();
        let flow_coeff = Tensor::<B, 3>::ones([batch, n_edges, 3], &dev);
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);

        let mut vel = vec![0.0_f32; batch * n * 3];
        for i in 0..n {
            vel[i * 3] = (i * i) as f32 * 0.1;
            vel[i * 3 + 1] = 0.03 * i as f32;
        }
        let u_star = Tensor::<B, 3>::from_data(Data::new(vel, Shape::new([batch, n, 3])), &dev);

        let rhs_base = chorin_pressure_rhs_mean_free_weak_divergence(
            u_star, &topo, flow_coeff, batch, n_edges, n,
        )
        .0;
        let rhs_cap =
            chorin_open_x_chain_end_cap_flux_rhs_mean_free(batch, n, 0, n - 1, 0.07_f32, &dev);
        let row_mx = rhs_cap.clone().sum_dim(1).abs().max();
        assert!(
            row_mx
                .clone()
                .all_close(Tensor::zeros_like(&row_mx), None, Some(1e-5_f64)),
            "end-cap pattern should be mean-free"
        );

        let rhs_sum = rhs_base.clone().add(rhs_cap.clone());
        let phi0 =
            solve_pressure_phi_jacobi_cg(rhs_base, edges_b1.clone(), damage.clone(), batch, n);
        let phi1 =
            solve_pressure_phi_jacobi_cg(rhs_sum, edges_b1.clone(), damage.clone(), batch, n);
        let dphi = phi1.sub(phi0);
        let dphi_n = dphi.powf_scalar(2.0).sum().sqrt();
        let ztol = Tensor::<B, 1>::zeros([1], &dev);
        assert!(
            !dphi_n.all_close(ztol, None, Some(1e-6_f64)),
            "open-x cap increment should shift φ"
        );
    }
}

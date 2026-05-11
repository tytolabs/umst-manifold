// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Bingham / projection CFD on the graph (experimental `rheology-bingham`).
//!
//! ## Audit memo (Track E)
//! - **Steady vs transient:** [`plane_bingham_poiseuille_u`](crate::physics::rheology_analytic) is the
//!   **steady** parallel-plate reference; [`BinghamFlowSolver::step`] is an explicit Chorin split on a
//!   graph — no claim of convergence to that steady profile without inlet/outlet BCs and a proper
//!   pressure Poisson (see deferrals in `tests/verification/rheology_poiseuille.rs`).
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
//! 2. **Pressure Poisson surrogate**: solve \(\mathcal{L}\phi \approx \nabla\cdot u^\*\) where
//!    \(\mathcal{L}\) is [`TopologicalLaplacian::scalar_laplacian`]. RHS uses the **same surrogate**
//!    as three Laplacians summed on \(u^\*\) components (documented approximation on the 1-skeleton).
//!    Fixed-count Richardson iterations (no tolerance) smooth \(\phi\).
//! 3. **Projection**: subtract an edge-tangent discrete gradient of \(\phi\) (built from
//!    [`primal_scalar_edge_increment`](crate::physics::dec_primal::primal_scalar_edge_increment))
//!    so the correction sits on the local flow direction carried by \(u^\*\).
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
//! **In scope today:** explicit predictor + **surrogate** pressure correction (Richardson on the graph
//! Laplacian with an RHS built from **three scalar Laplacians** of \(u^\*\) — not the discrete
//! divergence \(\nabla_h\!\cdot u^\*\) of a staggered/incompressible discretization); wall BCs as
//! **external** nodal masks in tests; Bingham + optional Roussel \(\lambda\) on the same 1-skeleton.
//!
//! **Out of scope / not CI-certified:** developed **2D** channel flow matching plane Poiseuille
//! (Bingham or Newtonian) on the full **64×16** cell graph; inlet/outlet pressure drop as a **consistent**
//! open boundary with this split; claims of convergence to [`plane_bingham_poiseuille_u`] or the
//! regularized quadrature profile without replacing the pressure step.
//!
//! **Known numerical boundary:** on the **65×17** channel scaffold with uniform body force
//! \(a_x=\Delta p/\rho\) matching `rheology_poiseuille` SI defaults, the surrogate projection produces
//! **large** \(\mathcal O(10^3\!-\!10^4)\) growth in \(\|u\|_\infty\) **relative to the predictor**
//! on the first correction step (and quickly blows up further — see `tests/verification/rheology_poiseuille.rs`:
//! ignored steady benchmark + active two-step `chorin_surrogate_poisson_amplification_regression_guard`).
//! Steady vs analytic comparisons stay deferred until a true pressure Poisson (or MAC-type) RHS/BC story replaces this MVP.
//!
//! ## MAC + Poisson — integration points (R2.2, design note)
//! Ring 2 **R2.2** calls for real pressure Poisson (or MAC) on the developed channel; this module stays on the
//! **surrogate** Richardson step until that lands. The following are **hook points** for a future staggered /
//! incompressible-correct split — **not** an implemented MAC grid:
//! - **After the predictor:** `step_experimental` forms `u_star` from explicit momentum (body, viscous,
//!   pressure-gradient acceleration). A MAC predictor would typically commit **face-normal** provisional
//!   fluxes here; today everything stays nodal on `edges_b1`.
//! - **Poisson RHS vs surrogate:** The block `L(u*_x)+L(u*_y)+L(u*_z)` ([`TopologicalLaplacian::scalar_laplacian`])
//!   approximates a divergence source; a consistent pressure Poisson should use the **discrete divergence**
//!   of `u_star`, e.g. via [`primal_divergence_from_edge_flux_topo`] on an edge flux assembled from `u_star`
//!   (and, with staggering, from face velocities once defined). That replaces the triple-Laplacian RHS while
//!   keeping the same graph operators.
//! - **Poisson solve:** The Richardson loop on **phi** uses the same [`TopologicalLaplacian`](crate::physics::laplacian::TopologicalLaplacian) as the operator
//!   \(\mathcal{L}\); swapping in Jacobi/SOR/CG iterations (or a chain **Thomas** path when topology is a
//!   path — compare electrochemistry Poisson helpers) is the natural upgrade path without new assembly theory.
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
/// Richardson iterations for \(\mathcal{L}\phi \approx \mathrm{RHS}\).
const POISSON_ITERS: usize = 28;
#[cfg(feature = "rheology-bingham")]
const POISSON_OMEGA: f32 = 0.18;
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

    let rho_e3 = rho_e.expand([batch, n_edges, ch3]);
    let dp3 = dp.expand([batch, n_edges, ch3]);
    let edge_pressure = dp3.neg().div(rho_e3).mul(t_hat).mul(fc3.clone());

    let pressure_accel = primal_divergence_from_edge_flux_topo(edge_pressure, &topo, &velocity);

    let g = gravity.reshape([1, 1, 3]).expand([batch, n, 3]);

    let momentum = g.add(viscous_accel).add(pressure_accel);

    let u_star = velocity.add(momentum.mul_scalar(dt));

    // --- Poisson surrogate for φ: L φ ≈ sum_c L(u*_c) ---
    let uxs = u_star.clone().narrow(2, 0, 1);
    let uys = u_star.clone().narrow(2, 1, 1);
    let uzs = u_star.clone().narrow(2, 2, 1);

    let lx = TopologicalLaplacian::scalar_laplacian(uxs, edges_b1.clone(), damage.clone());
    let ly = TopologicalLaplacian::scalar_laplacian(uys, edges_b1.clone(), damage.clone());
    let lz = TopologicalLaplacian::scalar_laplacian(uzs, edges_b1.clone(), damage.clone());
    let rhs = lx.add(ly).add(lz);

    let mut phi = Tensor::<B, 3>::zeros([batch, n, 1], &device);
    for _ in 0..POISSON_ITERS {
        let lphi =
            TopologicalLaplacian::scalar_laplacian(phi.clone(), edges_b1.clone(), damage.clone());
        phi = phi.add(rhs.clone().sub(lphi).mul_scalar(POISSON_OMEGA));
    }

    // Projection: subtract divergence of ( (φ_j-φ_i) ê ), ê ≈ tangent from u*.
    let dphi = primal_scalar_edge_increment(phi.clone(), &topo);
    let du_s = primal_scalar_edge_increment(u_star.clone(), &topo);

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

    let dphi3 = dphi.expand([batch, n_edges, ch3]);
    let proj_flux = dphi3.mul(t_hat_s).mul(fc3);

    let proj = primal_divergence_from_edge_flux_topo(proj_flux, &topo, &u_star);
    let velocity_new = u_star.sub(proj);

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
}

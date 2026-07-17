// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track 13 — implicit THMC residual assembly for Newton / JFNK.
//!
//! **Solver status / matrix:** authoritative lane notes live in **[`docs/Solver-Status.md`](../../../docs/Solver-Status.md)**
//! (**Solver lanes — THMC**); numbered scope and acceptance vs next PR slice in
//! **[`docs/Solver-Status.md`](../../../docs/Solver-Status.md)** row **#8**.
//!
//! **Post-`3394b96` cap (dense Newton only):** all THMC paths that build a **dense** forward-difference Jacobian or
//! dense damped Newton on stacked implicit unknowns share [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] (**64**) — see
//! commit **`3394b96`**. There is **no** in-tree dense monolithic THMC Newton for **> 64** stacked DOFs; large-\(N\)
//! monolith work is explicitly the **sparse / matrix-free Jacobian + Krylov–JFNK** direction with **AD-safe** ways to
//! gate on **‖R‖** at scale (Solver-Status / matrix **#8** “still open” and blocker rows).
//!
//! **Milestone (v0.4):** `ThmcImplicitEulerThermalReactionExtentResidual` (feature `thmc-coupled`) implements the **backward Euler**
//! discrete residual for the **thermal + reaction extent \(\alpha\)** sub-block on the graph Laplacian used
//! by [`crate::physics::solvers::thmc::ThmcSolver`]:
//! \[
//! R_T = T - T^n - \Delta t\,\bigl(\mathcal{L}(T) + q_{\mathrm{exo}}\,\dot\alpha(\alpha,T)\bigr),\qquad
//! R_\alpha = \alpha - \alpha^n - \Delta t\,\dot\alpha(\alpha,T),
//! \]
//! with \(\dot\alpha\) from `crate::physics::solvers::thmc::reaction_extent_rate_tensor`
//! ([`crate::physics::solvers::thmc::ReactionExtentKinetics`]). Humidity, mechanics, and fracture are
//! **out of scope** for this struct.
//!
//! **Damped Newton on \((T,\alpha)\) (feature `thmc-coupled`):** `ThmcImplicitEulerThermalReactionExtentResidual::one_damped_newton_step`
//! builds a dense forward-difference Jacobian on a **small** stacked state (cap on total DOFs) and
//! applies one damped Newton update \(U \leftarrow U - \omega J^{-1} R\).
//! `ThmcImplicitEulerThermalReactionExtentResidual::damped_newton_iterations` chains that step **≥ 2**
//! times (fresh Jacobian each iteration). Track 13 stepping stone toward full JFNK; humidity and
//! mechanics remain out of scope here. See `docs/research/v0.4_track13_monolithic_newton_thmc.md`
//! (appendix **§ Implementation blueprint** for stacked-unknown layout and batched constraints).
//!
//! **Shipped (`thmc-coupled` + `solver-experimental`):** one monolithic quasi-static Newton step may use
//! **matrix-free** reduced FD + host **`f32` GMRES** via `solvers::thmc_jfnk::gmres_f32_try` and a **single**
//! fallible matvec (`ThmcImplicitEulerThermalHumidityReactionExtentResidual::quasi_static_reduced_directional_fd_matvec`);
//! `solvers::thmc_jfnk::gmres_f32` is not used on this path (see `thmc_jfnk` module docs: closure vs `dyn`). Dense
//! Gauss–Jordan remains the fallback; chained iterations use a **dense** inner solve.
//!
//! ## Follow-up (**`m8-scale-ad`**): AD-safe stacked ‖R‖
//!
//! Stacked ‖R‖₂ is built as a **pure Burn subgraph** (sum of squares → `clamp_min` → `sqrt`) on the
//! residual blocks. **Host sync** for `f32` reporting (`Vec<f32>` trails, public `residual_l2` APIs)
//! prefers [`tensor1_f32_thmc`] (`into_data` on a rank‑1 tensor). **Acceptable debt (ConvergenceRequired —
//! telemetry only):** an equivalent rank‑1 `.into_scalar()` read for the same L2 / Newton outer scopes is
//! in the same tier; **do not** blanket-remove those reductions, and **do not** push them into **hot inner
//! micro-loops** (tight per-iteration Krylov paths — see [`docs/FP_CATEGORICAL_BURN.md`](../../../docs/FP_CATEGORICAL_BURN.md)).
//! Newton early-exit predicates use [`Tensor::lower_elem`] + [`Tensor::all`] with a single
//! [`tensor1_bool_thmc`] read (electrochemistry Picard pattern). Full AD through outer loops remains a
//! **scale-out** item if stricter graph purity is required.

use burn::tensor::backend::Backend;

use crate::physics::PhysicsError;

#[cfg(feature = "thmc-coupled")]
use burn::tensor::Bool;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Data;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Int;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Shape;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Tensor;

#[cfg(feature = "thmc-coupled")]
use crate::core::field::{
    Field, HumidityField, ReactionExtentField, StepEntryDamageMask, StiffnessField,
    TemperatureField,
};
#[cfg(feature = "thmc-coupled")]
use crate::physics::dec_operators::DecEdgeOperators;
#[cfg(feature = "thmc-coupled")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "thmc-coupled")]
use crate::physics::mechanics::VectorMechanicsSolver;
#[cfg(feature = "thmc-coupled")]
use crate::physics::solvers::thmc::{
    reaction_extent_rate_field, shrink_strain_from_saturation_loss_tensor, ReactionExtentKinetics,
    ThmcState,
};

#[cfg(all(feature = "thmc-coupled", feature = "solver-experimental"))]
use super::krylov_host::gmres_f32_try;

#[cfg(not(feature = "thmc-coupled"))]
use super::thmc::ThmcState;

/// Rank‑1 `f32` tensor → host scalar via [`Tensor::into_data`] (single-element read).
///
/// **ConvergenceRequired — telemetry only:** same classification as a rank‑1 `.into_scalar()` for L2 residual /
/// Newton reporting; keep these reads at **outer** damped-Newton / JFNK scopes, not in **hot inner micro-loops**.
#[cfg(feature = "thmc-coupled")]
#[inline]
fn tensor1_f32_thmc<B: Backend<FloatElem = f32>>(t: Tensor<B, 1>) -> f32 {
    t.into_data().value[0]
}

/// Single-element rank‑1 bool tensor → host `bool` via [`Tensor::into_data`].
#[cfg(feature = "thmc-coupled")]
#[inline]
fn tensor1_bool_thmc<B: Backend<FloatElem = f32>>(t: Tensor<B, 1, Bool>) -> bool {
    t.into_data().value[0]
}

/// Return bundle for [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::one_damped_newton_step_qs_r_u_inner`]
/// (updated trial + stacked ‖R‖₂ before/after as rank‑1 tensors).
#[cfg(feature = "thmc-coupled")]
type QsRuNewtonStepTensors<B> = (ThmcState<B>, Tensor<B, 1>, Tensor<B, 1>);

/// Coupled implicit backward-Euler residual \(R(U)=U-U^n-\Delta t\,F(U)\) for THMC.
///
/// Unknown stack \(U=(T,h,\alpha,u)\) ordering is implementation-defined; block preconditioners
/// should follow the same layout as the residual vector.
///
/// **Shipped (v0.4, feature `thmc-coupled`):** `ThmcImplicitEulerThermalReactionExtentResidual` implements
/// real residual assembly / [`ResidualThmc::evaluate_residual`], tied to reaction extent kinetics (see
/// `thmc_implicit_euler_t_alpha_residual_matches_brute_force_two_nodes` in `tests/verification/thmc_drying_shrinkage.rs`).
/// Other future residual bundles should override the default [`evaluate_residual`](ResidualThmc::evaluate_residual) stub below.
pub trait ResidualThmc<B: Backend<FloatElem = f32>> {
    /// Machine-readable description of DOF ordering (for docs / tests).
    fn dof_ordering_note() -> &'static str {
        "field-major or node-major T,h,alpha,u — must match JFNK Vec layout"
    }

    /// Evaluate the residual map at `trial` (implementation-defined contract).
    fn evaluate_residual(&self, trial: &ThmcState<B>) -> Result<(), PhysicsError> {
        let _ = trial;
        Err(PhysicsError::UnsupportedLayout {
            context: "ResidualThmc::evaluate_residual",
        })
    }
}

/// Field-major flattened unknown layout for a **future** monolithic THMC Newton–Krylov stack.
///
/// Zero-sized **documentation / const-fn anchor** only — no runtime state. Matches the stacked
/// \((T,\alpha)\) ordering used in `ThmcImplicitEulerThermalReactionExtentResidual::one_damped_newton_step`
/// (thermal `vec`, then reaction extent \(\alpha\) `vec`) when extended with \(h\) and \(\mathbf u\) blocks.
///
/// See `docs/research/v0.4_track13_monolithic_newton_thmc.md` appendix **§ Implementation blueprint**.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ThmcMonolithicImplicitUnknownLayout;

/// Upper bound on stacked-unknown count for **dense** forward-difference Newton across THMC implicit
/// helpers and the monolithic / implicit-(T,α) fail-fast guards in [`crate::physics::solvers::thmc::ThmcSolver`].
///
/// **Single source of truth since `3394b96`:** every shipped dense-Newton THMC path clamps or errors at this value
/// — there is **no** dense solve for more than **64** stacked DOFs. Production monolithic THMC at large \(N\) is
/// **not** “the same dense code with a bigger cap”; it is the **sparse / JFNK / AD-safe ‖R‖** roadmap documented in
/// [`docs/Solver-Status.md`](../../../docs/Solver-Status.md) §THMC.
pub const THMC_DENSE_NEWTON_MAX_STACKED_DOFS: usize = 64;

impl ThmcMonolithicImplicitUnknownLayout {
    /// Displacement components per node (`MechanicalPlan`: `[B, N, 3]`).
    pub const MECHANICAL_DISP_PER_NODE: usize = 3;

    /// Scalar DOFs for **one** batch index: \(N F_T + N F_h + N F_\alpha + 3N\).
    pub const fn field_major_stacked_dof_count(
        n_nodes: usize,
        f_temperature: usize,
        f_humidity: usize,
        f_reaction_extent: usize,
    ) -> usize {
        n_nodes * f_temperature
            + n_nodes * f_humidity
            + n_nodes * f_reaction_extent
            + n_nodes * Self::MECHANICAL_DISP_PER_NODE
    }

    /// Flattened length for `batch` independent roots (no cross-batch coupling in \(R\)).
    pub const fn batched_flat_len(
        batch: usize,
        n_nodes: usize,
        f_temperature: usize,
        f_humidity: usize,
        f_reaction_extent: usize,
    ) -> usize {
        batch
            * Self::field_major_stacked_dof_count(
                n_nodes,
                f_temperature,
                f_humidity,
                f_reaction_extent,
            )
    }

    /// Field-major length of the **scalar transport + reaction extent** prefix \((T,h,\alpha)\) before \(\mathbf u\).
    ///
    /// \(\texttt{n\_nodes}\cdot(F_T+F_h+F_\alpha)\); consistent with the leading blocks of
    /// [`Self::field_major_stacked_dof_count`] (same ordering as `ThmcImplicitEulerThermalHumidityReactionExtentResidual`).
    pub const fn field_major_scalar_transport_hydration_dof_count(
        n_nodes: usize,
        f_temperature: usize,
        f_humidity: usize,
        f_reaction_extent: usize,
    ) -> usize {
        n_nodes * f_temperature + n_nodes * f_humidity + n_nodes * f_reaction_extent
    }
}

/// Backward-Euler residual for **thermal + reaction extent** on a fixed graph, \(F\) evaluated at the trial
/// state (consistent implicit step).
#[cfg(feature = "thmc-coupled")]
#[derive(Clone, Debug)]
pub struct ThmcImplicitEulerThermalReactionExtentResidual<B: Backend<FloatElem = f32>> {
    pub dt: f32,
    pub temperature_n: TemperatureField<B>,
    pub alpha_n: ReactionExtentField<B>,
    pub edges_b1: Tensor<B, 2, Int>,
    pub damage_m: StepEntryDamageMask<B>,
    pub kinetics: ReactionExtentKinetics,
}

#[cfg(feature = "thmc-coupled")]
impl<B: Backend<FloatElem = f32>> ThmcImplicitEulerThermalReactionExtentResidual<B> {
    #[deprecated(since = "0.2.0", note = "FP P3.2")]
    #[must_use]
    pub fn from_tensors(dt: f32, temperature_n: Tensor<B, 3>, alpha_n: Tensor<B, 3>, edges_b1: Tensor<B, 2, Int>, damage_m: Tensor<B, 3>, kinetics: ReactionExtentKinetics) -> Self {
        Self { dt, temperature_n: Field::new(temperature_n), alpha_n: Field::new(alpha_n), edges_b1, damage_m: StepEntryDamageMask::from_tensor(damage_m), kinetics }
    }
    /// Assemble \(R_T, R_\alpha\) at `trial` (same shapes as `temperature` / `reaction_extent` plans).
    pub fn assemble(&self, trial: &ThmcState<B>) -> Result<(TemperatureField<B>, ReactionExtentField<B>), PhysicsError> {
        let t = trial.thermal.temperature.as_tensor().clone();
        let alpha = trial.chemical.reaction_extent.as_tensor().clone();
        let device = t.device();
        let batch = t.dims()[0];
        let n = t.dims()[1];
        if self.temperature_n.as_tensor().dims() != t.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalReactionExtentResidual: T^n dims {:?} != trial T dims {:?}",
                self.temperature_n.as_tensor().dims(),
                t.dims()
            ),
        });
        }
        if self.alpha_n.as_tensor().dims() != alpha.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalReactionExtentResidual: α^n dims {:?} != trial α dims {:?}",
                self.alpha_n.as_tensor().dims(),
                alpha.dims()
            ),
        });
        }

        let lap_t = TopologicalLaplacian::scalar_laplacian_temperature(&Field::new(t.clone()), &self.damage_m, self.edges_b1.clone());
        let dt_lap_t = lap_t.as_tensor().clone().mul_scalar(self.dt);

        let f_alpha_ch = alpha.dims()[2];
        let t_bn1 = t.clone().slice([0..batch, 0..n, 0..1]);
        let temperature_for_alpha = Field::new(if f_alpha_ch == 1 { t_bn1 } else { t_bn1.expand::<3, _>([batch, n, f_alpha_ch]) });
        let d_alpha = reaction_extent_rate_field(&self.kinetics, &Field::new(alpha.clone()), &temperature_for_alpha, &device);

        let f_t_ch = t.dims()[2];
        let exo = d_alpha.as_tensor().clone()
            .slice([0..batch, 0..n, 0..1])
            .mul_scalar(self.kinetics.exothermic_k_per_alpha_rate * self.dt)
            .expand::<3, _>([batch, n, f_t_ch]);

        let r_t = t.sub(self.temperature_n.as_tensor().clone()).sub(dt_lap_t).sub(exo);
        let r_alpha = alpha
            .sub(self.alpha_n.as_tensor().clone())
            .sub(d_alpha.as_tensor().clone().mul_scalar(self.dt));
        Ok((Field::new(r_t), Field::new(r_alpha)))
    }

    /// Combined Euclidean norm \(\sqrt{\|R_T\|_2^2 + \|R_\alpha\|_2^2}\) at `trial`.
    pub fn residual_l2(&self, trial: &ThmcState<B>) -> Result<f32, PhysicsError> {
        let (r_t, r_a) = self.assemble(trial)?;
        Ok(combined_residual_l2(r_t.as_tensor(), r_a.as_tensor()))
    }

    /// One **damped Newton** step on the coupled \((T,\alpha)\) backward-Euler residual.
    ///
    /// - **Jacobian:** forward finite differences on the stacked unknown
    ///   \((T,\alpha)\) in flattened `[B,N,F]` order (thermal entries first, then chemical; same layout
    ///   as [`Self::assemble`]).
    /// - **Linear solve:** Gauss–Jordan elimination with partial pivoting on the dense system
    ///   \(J\,\delta = -R\) (host `f32`, intended for small chains / verification only).
    /// - **Scope:** requires `trial.thermal.temperature.as_tensor().dims()[0] == 1` and
    ///   `n (f_T + f_\alpha) \le` [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] (hard cap).
    ///
    /// Returns `(updated_trial, \|R\|_2 \text{ before}, \|R\|_2 \text{ after})`. Hydro / mechanics /
    /// damage / time on `trial` are preserved; only `temperature` and `reaction_extent` change.
    ///
    /// ## Multi-step
    /// For two or more sequential correctors with a fresh Jacobian each time, use
    /// [`Self::damped_newton_iterations`].
    pub fn one_damped_newton_step(
        &self,
        trial: &ThmcState<B>,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, f32, f32), PhysicsError> {
        if !(damping > 0.0_f32 && damping <= 1.0_f32) {
            return Err(PhysicsError::Domain {
            detail: "one_damped_newton_step: damping must lie in (0, 1]".to_string(),
        });
        }
        if fd_eps <= 0.0_f32 {
            return Err(PhysicsError::Domain {
            detail: "one_damped_newton_step: fd_eps must be positive".to_string(),
        });
        }

        let t_dims = trial.thermal.temperature.as_tensor().dims();
        let a_dims = trial.chemical.reaction_extent.as_tensor().dims();
        if t_dims[0] != 1 {
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step: batch must be 1, got {}",
                t_dims[0]
            ),
        });
        }
        if t_dims[0] != a_dims[0] || t_dims[1] != a_dims[1] {
            return Err(PhysicsError::ShapeMismatch {
            context: "one_damped_newton_step",
            detail: "T and α batch/node counts must match",
        });
        }

        let n = t_dims[1];
        let f_t = t_dims[2];
        let f_a = a_dims[2];
        let m = n * f_t + n * f_a;
        if m > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
            let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step: {m} stacked DOFs exceeds cap {cap}",
            ),
        });
        }

        let device = trial.thermal.temperature.as_tensor().device();
        let (r_t0, r_a0) = self.assemble(trial)?;
        let norm_before = combined_residual_l2(r_t0.as_tensor(), r_a0.as_tensor());
        let r0 = flatten_two_residuals(r_t0.as_tensor(), r_a0.as_tensor());

        let mut u = flatten_two_fields(trial.thermal.temperature.as_tensor(), trial.chemical.reaction_extent.as_tensor());
        if u.len() != m || r0.len() != m {
            return Err(PhysicsError::InvariantViolation {
            context: "one_damped_newton_step: internal flatten length mismatch",
        });
        }

        // Dense Jacobian: column j = ∂R/∂u_j (forward difference).
        let mut jac = vec![0.0_f32; m * m];
        for j in 0..m {
            let eps_j = fd_eps * (1.0_f32 + u[j].abs());
            u[j] += eps_j;
            let pert = trial_from_packed(
                trial,
                &device,
                &u,
                [t_dims[0], t_dims[1], t_dims[2]],
                [a_dims[0], a_dims[1], a_dims[2]],
            );
            u[j] -= eps_j;
            let (r_tp, r_ap) = self.assemble(&pert)?;
            let r_pert = flatten_two_residuals(r_tp.as_tensor(), r_ap.as_tensor());
            for i in 0..m {
                jac[i * m + j] = (r_pert[i] - r0[i]) / eps_j;
            }
        }

        let mut rhs: Vec<f32> = r0.iter().map(|x| -x).collect();
        let delta = gauss_jordan_solve(&mut jac, &mut rhs, m)?;
        for k in 0..m {
            u[k] += damping * delta[k];
        }

        let new_trial = trial_from_packed(
            trial,
            &device,
            &u,
            [t_dims[0], t_dims[1], t_dims[2]],
            [a_dims[0], a_dims[1], a_dims[2]],
        );
        let norm_after = self.residual_l2(&new_trial)?;
        Ok((new_trial, norm_before, norm_after))
    }

    /// Run **`iterations` ≥ 2** sequential damped Newton steps on the stacked \((T,\alpha)\) unknowns.
    ///
    /// Each step calls [`Self::one_damped_newton_step`] (dense forward-difference \(J\), Gauss–Jordan
    /// on \(J\,\delta=-R\), then \(U\leftarrow U+\omega\delta\)). Returns `(final trial, norms)` where
    /// `norms[k] = \|R\|_2` **after** `k` steps for `k = 1..=iterations`, and `norms[0]` is the
    /// initial residual **before** any update (length `iterations + 1`).
    pub fn damped_newton_iterations(
        &self,
        trial: &ThmcState<B>,
        iterations: usize,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, Vec<f32>), PhysicsError> {
        if iterations < 2 {
            return Err(PhysicsError::Domain {
            detail: "damped_newton_iterations: iterations must be >= 2".to_string(),
        });
        }
        let mut norms: Vec<f32> = Vec::with_capacity(iterations + 1);
        norms.push(self.residual_l2(trial)?);

        let (mut current, _, after_first) = self.one_damped_newton_step(trial, damping, fd_eps)?;
        norms.push(after_first);

        for _ in 1..iterations {
            let (next, _, after) = self.one_damped_newton_step(&current, damping, fd_eps)?;
            current = next;
            norms.push(after);
        }
        Ok((current, norms))
    }
}

/// Backward-Euler residual for **thermal + humidity + reaction extent \(\alpha\)** on a fixed graph.
///
/// Assembles the field-major stacked map (memo track 13 appendix §B) for \((R_T,R_h,R_\alpha)\).
/// Humidity is pure implicit diffusion \(R_h = h - h^n - \Delta t\,\mathcal{L}_h(h)\) — **not** the
/// explicit split’s tail drying closure. \((T,\alpha)\) blocks match
/// [`ThmcImplicitEulerThermalReactionExtentResidual`].
///
/// **Mechanics tail (verification stub):** [`Self::assemble_with_mechanics_placeholder_r_u`] appends
/// a **non-equilibrium** placeholder
/// \[
/// R_u = m\,(\mathbf u - \mathbf u^n),
/// \]
/// with diagonal lumped scale `mechanics_placeholder_mass` \(m\) (default `1`). This is **not** the
/// quasi-static bar-network \(R_u\) from the mechanics coupling plan §2.4 — it exists so field-major
/// \([\mathrm{vec}(R_T);\mathrm{vec}(R_h);\mathrm{vec}(R_\alpha);\mathrm{vec}(R_u)]\) matches
/// [`ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`] for layout-only checks.
/// For physical \(R_u\), use [`Self::evaluate_quasi_static_r_u`] /
/// [`Self::assemble_with_quasi_static_r_u`] (plan §4 Phase 1–2).
#[cfg(feature = "thmc-coupled")]
#[derive(Clone, Debug)]
pub struct ThmcImplicitEulerThermalHumidityReactionExtentResidual<B: Backend<FloatElem = f32>> {
    pub dt: f32,
    pub temperature_n: TemperatureField<B>,
    pub humidity_n: HumidityField<B>,
    pub alpha_n: ReactionExtentField<B>,
    /// Reference displacement \(\mathbf u^n\) for the placeholder block (same shape as
    /// [`MechanicalPlan::displacement`]: `[B, N, 3]`).
    pub displacement_n: Tensor<B, 3>,
    /// Scalar \(m\) in \(R_u = m(\mathbf u - \mathbf u^n)\) (layout / FD scale hook; not physical mass).
    pub mechanics_placeholder_mass: f32,
    /// Optional `w/c` for notional drying-shrink **increment** in [`Self::evaluate_quasi_static_r_u`]
    /// (coupling plan §4 Phase 4). `None` preserves shrink-free elastic \(R_u\).
    pub ru_shrinkage_binder_liquid_ratio: Option<f32>,
    pub edges_b1: Tensor<B, 2, Int>,
    pub damage_m: StepEntryDamageMask<B>,
    pub kinetics: ReactionExtentKinetics,
}

#[cfg(feature = "thmc-coupled")]
impl<B: Backend<FloatElem = f32>> ThmcImplicitEulerThermalHumidityReactionExtentResidual<B> {
    #[deprecated(since = "0.2.0", note = "FP P3.2")]
    #[must_use]
    pub fn from_tensors(dt: f32, temperature_n: Tensor<B, 3>, humidity_n: Tensor<B, 3>, alpha_n: Tensor<B, 3>, displacement_n: Tensor<B, 3>, mechanics_placeholder_mass: f32, ru_shrinkage_binder_liquid_ratio: Option<f32>, edges_b1: Tensor<B, 2, Int>, damage_m: Tensor<B, 3>, kinetics: ReactionExtentKinetics) -> Self {
        Self { dt, temperature_n: Field::new(temperature_n), humidity_n: Field::new(humidity_n), alpha_n: Field::new(alpha_n), displacement_n, mechanics_placeholder_mass, ru_shrinkage_binder_liquid_ratio, edges_b1, damage_m: StepEntryDamageMask::from_tensor(damage_m), kinetics }
    }
    /// Assemble \((R_T, R_h, R_\alpha)\) at `trial`.
    #[allow(clippy::type_complexity)]
    pub fn assemble(
        &self,
        trial: &ThmcState<B>,
    ) -> Result<(TemperatureField<B>, HumidityField<B>, ReactionExtentField<B>), PhysicsError> {
        let t = trial.thermal.temperature.as_tensor().clone();
        let h = trial.hydro.humidity.as_tensor().clone();
        let alpha = trial.chemical.reaction_extent.as_tensor().clone();
        let device = t.device();
        let batch = t.dims()[0];
        let n = t.dims()[1];
        if self.temperature_n.as_tensor().dims() != t.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalHumidityReactionExtentResidual: T^n dims {:?} != trial T dims {:?}",
                self.temperature_n.as_tensor().dims(),
                t.dims()
            ),
        });
        }
        if self.humidity_n.as_tensor().dims() != h.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalHumidityReactionExtentResidual: h^n dims {:?} != trial h dims {:?}",
                self.humidity_n.as_tensor().dims(),
                h.dims()
            ),
        });
        }
        if self.alpha_n.as_tensor().dims() != alpha.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalHumidityReactionExtentResidual: α^n dims {:?} != trial α dims {:?}",
                self.alpha_n.as_tensor().dims(),
                alpha.dims()
            ),
        });
        }

        let lap_t = TopologicalLaplacian::scalar_laplacian_temperature(&Field::new(t.clone()), &self.damage_m, self.edges_b1.clone());
        let dt_lap_t = lap_t.as_tensor().clone().mul_scalar(self.dt);

        let lap_h = TopologicalLaplacian::scalar_laplacian_humidity(&Field::new(h.clone()), &self.damage_m, self.edges_b1.clone());
        let dt_lap_h = lap_h.as_tensor().clone().mul_scalar(self.dt);

        let f_alpha_ch = alpha.dims()[2];
        let t_bn1 = t.clone().slice([0..batch, 0..n, 0..1]);
        let temperature_for_alpha = Field::new(if f_alpha_ch == 1 { t_bn1 } else { t_bn1.expand::<3, _>([batch, n, f_alpha_ch]) });
        let d_alpha = reaction_extent_rate_field(&self.kinetics, &Field::new(alpha.clone()), &temperature_for_alpha, &device);

        let f_t_ch = t.dims()[2];
        let exo = d_alpha.as_tensor().clone()
            .slice([0..batch, 0..n, 0..1])
            .mul_scalar(self.kinetics.exothermic_k_per_alpha_rate * self.dt)
            .expand::<3, _>([batch, n, f_t_ch]);

        let r_t = t.sub(self.temperature_n.as_tensor().clone()).sub(dt_lap_t).sub(exo);
        let r_h = h.sub(self.humidity_n.as_tensor().clone()).sub(dt_lap_h);
        let r_alpha = alpha
            .sub(self.alpha_n.as_tensor().clone())
            .sub(d_alpha.as_tensor().clone().mul_scalar(self.dt));
        Ok((Field::new(r_t), Field::new(r_h), Field::new(r_alpha)))
    }

    /// \((R_T,R_h,R_\alpha,R_u)\) with \(R_u = m(\mathbf u-\mathbf u^n)\) — see struct rustdoc (**not** bar equilibrium).
    #[allow(clippy::type_complexity)]
    pub fn assemble_with_mechanics_placeholder_r_u(
        &self,
        trial: &ThmcState<B>,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        let (r_t, r_h, r_alpha) = self.assemble(trial)?;
        let u = trial.mechanical.displacement.as_tensor().clone();
        if self.displacement_n.dims() != u.dims() {
            return Err(PhysicsError::Domain {
            detail: format!(
                "ThmcImplicitEulerThermalHumidityReactionExtentResidual: u^n dims {:?} != trial u dims {:?}",
                self.displacement_n.dims(),
                u.dims()
            ),
        });
        }
        let r_u = u
            .sub(self.displacement_n.clone())
            .mul_scalar(self.mechanics_placeholder_mass);
        Ok((r_t.as_tensor().clone(), r_h.as_tensor().clone(), r_alpha.as_tensor().clone(), r_u))
    }

    /// Field-major \([\mathrm{vec}(R_T);\mathrm{vec}(R_h);\mathrm{vec}(R_\alpha);\mathrm{vec}(R_u)]\)
    /// using [`Self::assemble_with_mechanics_placeholder_r_u`].
    pub fn stacked_flat_residual_field_major(
        &self,
        trial: &ThmcState<B>,
    ) -> Result<Vec<f32>, PhysicsError> {
        let (r_t, r_h, r_a, r_u) = self.assemble_with_mechanics_placeholder_r_u(trial)?;
        Ok(flatten_four_residuals(&r_t, &r_h, &r_a, &r_u))
    }

    /// \(\sqrt{\|R_T\|^2+\|R_h\|^2+\|R_\alpha\|^2+\|R_u\|^2}\) including the placeholder \(R_u\) block.
    pub fn residual_l2_including_mechanics_placeholder(
        &self,
        trial: &ThmcState<B>,
    ) -> Result<f32, PhysicsError> {
        let (r_t, r_h, r_a, r_u) = self.assemble_with_mechanics_placeholder_r_u(trial)?;
        Ok(combined_four_residual_l2(&r_t, &r_h, &r_a, &r_u))
    }

    /// **Coupling plan §4 Phase 1 — quasi-static bar \(R_u\):**
    /// \(P(\mathbf f_{\mathrm{ext}} - K(\alpha)\,\mathbf u)\) using the same \(\alpha\mapsto E\) rule as
    /// [`super::thmc::ThmcSolver::step_experimental`](super::thmc::ThmcSolver) and
    /// `VectorMechanicsSolver::projected_bar_equilibrium_residual` (crate-private helper on the mechanics solver).
    ///
    /// **Phase 4 (optional):** when [`Self::ru_shrinkage_binder_liquid_ratio`] is `Some(w/c)\), a notional
    /// shrink-strain **increment** along edges (trial vs `humidity_n` saturation deficit, edge-averaged)
    /// enters the axial bar law as an eigenstrain in `VectorMechanicsSolver::projected_bar_equilibrium_residual`.
    ///
    /// **Dirichlet tail:** on components where `boundary_mask` is zero, adds
    /// [`Self::mechanics_placeholder_mass`]\(\cdot(\mathbf u-\mathbf u^n)\) so constrained displacement
    /// unknowns participate in the dense Newton Jacobian (equilibrium rows alone would be identically zero).
    pub fn evaluate_quasi_static_r_u(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        let t_dims = trial.thermal.temperature.as_tensor().dims();
        let batch = t_dims[0];
        let n = t_dims[1];
        if coords_n3.dims() != [n, 3] {
            return Err(PhysicsError::Domain {
            detail: format!(
                "evaluate_quasi_static_r_u: coords dims {:?} != [{n}, 3]",
                coords_n3.dims()
            ),
        });
        }
        if boundary_mask_bn3.dims() != [batch, n, 3] {
            return Err(PhysicsError::Domain {
            detail: format!(
                "evaluate_quasi_static_r_u: boundary_mask dims {:?} != [{batch}, {n}, 3]",
                boundary_mask_bn3.dims()
            ),
        });
        }
        if body_force.dims() != [batch, n, 3] {
            return Err(PhysicsError::Domain {
            detail: format!(
                "evaluate_quasi_static_r_u: body_force dims {:?} != [{batch}, {n}, 3]",
                body_force.dims()
            ),
        });
        }
        let u = trial.mechanical.displacement.as_tensor().clone();
        if u.dims() != [batch, n, 3] {
            return Err(PhysicsError::Domain {
            detail: format!(
                "evaluate_quasi_static_r_u: displacement dims {:?} != [{batch}, {n}, 3]",
                u.dims()
            ),
        });
        }
        let device = u.device();
        let alpha_bn1 = trial
            .chemical
            .reaction_extent
            .as_tensor()
            .clone()
            .slice([0..batch, 0..n, 0..1])
            .clamp(1e-6_f32, 1.0_f32);

        let edge_shrink_strain_increment = if let Some(wc) = self.ru_shrinkage_binder_liquid_ratio {
            let h = trial.hydro.humidity.as_tensor().clone();
            let h_n = self.humidity_n.as_tensor().clone();
            if h.dims() != [batch, n, 1] {
                return Err(PhysicsError::Domain {
            detail: format!(
                    "evaluate_quasi_static_r_u: trial humidity dims {:?} != [{batch}, {n}, 1]",
                    h.dims()
                ),
        });
            }
            if h_n.dims() != [batch, n, 1] {
                return Err(PhysicsError::Domain {
            detail: format!(
                    "evaluate_quasi_static_r_u: humidity^n dims {:?} != [{batch}, {n}, 1]",
                    h_n.dims()
                ),
        });
            }
            let ones_h = Tensor::<B, 3>::ones(h.dims(), &device);
            let ones_hn = Tensor::<B, 3>::ones(h_n.dims(), &device);
            let loss_t = ones_h.sub(h).clamp(0.0_f32, 1.0_f32);
            let loss_n = ones_hn.sub(h_n).clamp(0.0_f32, 1.0_f32);
            let eps_t = shrink_strain_from_saturation_loss_tensor(loss_t, wc, alpha_bn1.clone());
            let eps_n = shrink_strain_from_saturation_loss_tensor(loss_n, wc, alpha_bn1.clone());
            let delta_node = eps_t.sub(eps_n);
            Some(DecEdgeOperators::arithmetic_mean_on_edges(
                delta_node,
                self.edges_b1.clone(),
            ))
        } else {
            None
        };

        let stiffness_e = alpha_bn1.mul_scalar(self.kinetics.stiffness_e_scale_pa);
        let stiffness_nu =
            Tensor::<B, 3>::zeros([batch, n, 1], &device).add_scalar(self.kinetics.stiffness_nu);
        let stiffness = StiffnessField::from_e_nu_cat(stiffness_e, stiffness_nu)
            .as_tensor()
            .clone();
        let r_eq = VectorMechanicsSolver::projected_bar_equilibrium_residual(
            u.clone(),
            coords_n3.clone(),
            stiffness,
            body_force.clone(),
            self.edges_b1.clone(),
            self.damage_m.as_tensor().clone(),
            boundary_mask_bn3.clone(),
            cross_section_area,
            edge_shrink_strain_increment,
        );
        // `projected_bar_equilibrium_residual` zeros rows where `boundary_mask` is 0, so perturbing a
        // constrained displacement would not move those residual entries → a singular FD Jacobian for
        // monolithic Newton. Add the same placeholder Dirichlet channel as
        // [`Self::assemble_with_mechanics_placeholder_r_u`]: \(m(\mathbf u-\mathbf u^n)\) on masked DOFs.
        let ones_m = Tensor::<B, 3>::ones_like(boundary_mask_bn3);
        let r_dirichlet = ones_m
            .sub(boundary_mask_bn3.clone())
            .mul(u.sub(self.displacement_n.clone()))
            .mul_scalar(self.mechanics_placeholder_mass);
        Ok(r_eq.add(r_dirichlet))
    }

    /// **Coupling plan §4 Phase 2:** \((R_T,R_h,R_\alpha,R_u)\) with \(R_u\) from
    /// [`Self::evaluate_quasi_static_r_u`].
    #[allow(clippy::type_complexity)]
    pub fn assemble_with_quasi_static_r_u(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        let (r_t, r_h, r_alpha) = self.assemble(trial)?;
        let r_u = self.evaluate_quasi_static_r_u(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        Ok((r_t.as_tensor().clone(), r_h.as_tensor().clone(), r_alpha.as_tensor().clone(), r_u))
    }

    /// Field-major flat stack using [`Self::assemble_with_quasi_static_r_u`].
    pub fn stacked_flat_residual_field_major_quasi_static(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Result<Vec<f32>, PhysicsError> {
        let (r_t, r_h, r_a, r_u) = self.assemble_with_quasi_static_r_u(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        Ok(flatten_four_residuals(&r_t, &r_h, &r_a, &r_u))
    }

    /// Combined L² of all four blocks with quasi-static \(R_u\).
    pub fn residual_l2_including_quasi_static_r_u(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Result<f32, PhysicsError> {
        let (r_t, r_h, r_a, r_u) = self.assemble_with_quasi_static_r_u(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        Ok(combined_four_residual_l2(&r_t, &r_h, &r_a, &r_u))
    }

    /// \(\sqrt{\|R_T\|_2^2 + \|R_h\|_2^2 + \|R_\alpha\|_2^2}\) (memo §B stacked norm, truncated to scalar blocks).
    pub fn residual_l2(&self, trial: &ThmcState<B>) -> Result<f32, PhysicsError> {
        let (r_t, r_h, r_a) = self.assemble(trial)?;
        Ok(combined_three_residual_l2(r_t.as_tensor(), r_h.as_tensor(), r_a.as_tensor()))
    }

    /// One damped Newton step on \((T,h,\alpha)\) in **field-major** order
    /// \([\mathrm{vec}(T), \mathrm{vec}(h), \mathrm{vec}(\alpha)]\) — same leading layout as
    /// [`ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`] before displacements.
    ///
    /// Batch must be **1**; `n (F_T+F_h+F_\alpha) \le` [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`]. Displacement / damage / time on `trial`
    /// are preserved.
    pub fn one_damped_newton_step(
        &self,
        trial: &ThmcState<B>,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, f32, f32), PhysicsError> {
        if !(damping > 0.0_f32 && damping <= 1.0_f32) {
            return Err(PhysicsError::Domain {
            detail: "one_damped_newton_step (T,h,α): damping must lie in (0, 1]".to_string(),
        });
        }
        if fd_eps <= 0.0_f32 {
            return Err(PhysicsError::Domain {
            detail: "one_damped_newton_step (T,h,α): fd_eps must be positive".to_string(),
        });
        }

        let t_dims = trial.thermal.temperature.as_tensor().dims();
        let h_dims = trial.hydro.humidity.as_tensor().dims();
        let a_dims = trial.chemical.reaction_extent.as_tensor().dims();
        if t_dims[0] != 1 {
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step (T,h,α): batch must be 1, got {}",
                t_dims[0]
            ),
        });
        }
        if t_dims != h_dims || t_dims[0] != a_dims[0] || t_dims[1] != a_dims[1] {
            return Err(PhysicsError::ShapeMismatch {
                context: "one_damped_newton_step (T,h,α)",
                detail: "T, h, α batch/node counts must match",
            });
        }

        let n = t_dims[1];
        let f_t = t_dims[2];
        let f_h = h_dims[2];
        let f_a = a_dims[2];
        let m =
            ThmcMonolithicImplicitUnknownLayout::field_major_scalar_transport_hydration_dof_count(
                n, f_t, f_h, f_a,
            );
        if m > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
            let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step (T,h,α): {m} stacked DOFs exceeds cap {cap}",
            ),
        });
        }

        let device = trial.thermal.temperature.as_tensor().device();
        let (r_t0, r_h0, r_a0) = self.assemble(trial)?;
        let norm_before = combined_three_residual_l2(r_t0.as_tensor(), r_h0.as_tensor(), r_a0.as_tensor());
        let r0 = flatten_three_residuals(r_t0.as_tensor(), r_h0.as_tensor(), r_a0.as_tensor());

        let mut u = flatten_three_fields(
            trial.thermal.temperature.as_tensor(),
            trial.hydro.humidity.as_tensor(),
            trial.chemical.reaction_extent.as_tensor(),
        );
        if u.len() != m || r0.len() != m {
            return Err(PhysicsError::InvariantViolation {
            context: "one_damped_newton_step (T,h,α): internal flatten length mismatch",
        });
        }

        let mut jac = vec![0.0_f32; m * m];
        for j in 0..m {
            let eps_j = fd_eps * (1.0_f32 + u[j].abs());
            u[j] += eps_j;
            let pert = trial_from_packed_three(
                trial,
                &device,
                &u,
                [t_dims[0], t_dims[1], t_dims[2]],
                [h_dims[0], h_dims[1], h_dims[2]],
                [a_dims[0], a_dims[1], a_dims[2]],
            );
            u[j] -= eps_j;
            let (r_tp, r_hp, r_ap) = self.assemble(&pert)?;
            let r_pert = flatten_three_residuals(r_tp.as_tensor(), r_hp.as_tensor(), r_ap.as_tensor());
            for i in 0..m {
                jac[i * m + j] = (r_pert[i] - r0[i]) / eps_j;
            }
        }

        let mut rhs: Vec<f32> = r0.iter().map(|x| -x).collect();
        let delta = gauss_jordan_solve(&mut jac, &mut rhs, m)?;
        for k in 0..m {
            u[k] += damping * delta[k];
        }

        let new_trial = trial_from_packed_three(
            trial,
            &device,
            &u,
            [t_dims[0], t_dims[1], t_dims[2]],
            [h_dims[0], h_dims[1], h_dims[2]],
            [a_dims[0], a_dims[1], a_dims[2]],
        );
        let norm_after = self.residual_l2(&new_trial)?;
        Ok((new_trial, norm_before, norm_after))
    }

    /// Same contract as [`ThmcImplicitEulerThermalReactionExtentResidual::damped_newton_iterations`], on \((T,h,\alpha)\).
    pub fn damped_newton_iterations(
        &self,
        trial: &ThmcState<B>,
        iterations: usize,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, Vec<f32>), PhysicsError> {
        if iterations < 2 {
            return Err(PhysicsError::Domain {
            detail: "damped_newton_iterations (T,h,α): iterations must be >= 2".to_string(),
        });
        }
        let mut norms: Vec<f32> = Vec::with_capacity(iterations + 1);
        norms.push(self.residual_l2(trial)?);

        let (mut current, _, after_first) = self.one_damped_newton_step(trial, damping, fd_eps)?;
        norms.push(after_first);

        for _ in 1..iterations {
            let (next, _, after) = self.one_damped_newton_step(&current, damping, fd_eps)?;
            current = next;
            norms.push(after);
        }
        Ok((current, norms))
    }

    /// Reduced directional finite-difference matvec \(v \mapsto (R(u+\sigma v)-R(u))/\sigma\) on active DOFs
    /// for [`Self::stacked_flat_residual_field_major_quasi_static`], feeding `solvers::thmc_jfnk::gmres_f32_try`.
    ///
    /// **Single** matrix-free Jacobian–vector product for the quasi-static monolithic Newton inner solve;
    /// matches the σ-scaling and residual stack used in the dense column FD branch of
    /// [`Self::one_damped_newton_step_qs_r_u_inner`].
    #[cfg(all(feature = "thmc-coupled", feature = "solver-experimental"))]
    #[allow(clippy::too_many_arguments)]
    fn quasi_static_reduced_directional_fd_matvec(
        &self,
        v_red: &[f32],
        u_base: &[f32],
        red_map: &[usize],
        r0: &[f32],
        fd_eps: f32,
        trial: &ThmcState<B>,
        device: &B::Device,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
        t_shape: [usize; 3],
        h_shape: [usize; 3],
        a_shape: [usize; 3],
        u_shape: [usize; 3],
    ) -> Result<Vec<f32>, PhysicsError> {
        let m_a = red_map.len();
        let v_norm_sq: f32 = v_red.iter().map(|x| x * x).sum();
        let v_norm = v_norm_sq.sqrt();
        if v_norm < 1e-30_f32 {
            return Ok(vec![0.0_f32; m_a]);
        }
        let u_sup = red_map
            .iter()
            .map(|&j| u_base[j].abs())
            .fold(0.0_f32, f32::max);
        let sigma = fd_eps * (1.0_f32 + u_sup) / v_norm.max(1e-30_f32);
        let mut pert = u_base.to_vec();
        for (jr, &ji) in red_map.iter().enumerate() {
            pert[ji] += sigma * v_red[jr];
        }
        let trial_p =
            trial_from_packed_four(trial, device, &pert, t_shape, h_shape, a_shape, u_shape);
        let r_s = self.stacked_flat_residual_field_major_quasi_static(
            &trial_p,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        Ok(red_map.iter().map(|&i| (r_s[i] - r0[i]) / sigma).collect())
    }

    /// **Coupling plan §4 Phase 3:** one damped Newton step on field-major \((T,h,\alpha,\mathbf u)\)
    /// with quasi-static bar \(R_u\) from [`Self::evaluate_quasi_static_r_u`].
    ///
    /// With **`solver-experimental`**, [`Self::one_damped_newton_step_with_quasi_static_r_u`] uses **JFNK**
    /// (reduced directional FD matvec) + host **`f32` GMRES** on the reduced system, with dense Gauss–Jordan fallback if
    /// GMRES does not satisfy its final residual check. [`Self::damped_newton_iterations_with_quasi_static_r_u`] keeps a **dense**
    /// inner linear solve for stability across chained Newton steps.
    ///
    /// Without **`solver-experimental`**, or when the inner solve is dense-only: column-wise FD Jacobian +
    /// Gauss–Jordan (full layout cap `M \le` [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`], batch 1).
    ///
    /// Displacement entries where `boundary_mask == 0` are **held fixed** (excluded from the reduced
    /// Newton system) so the Jacobian is not singular on Dirichlet rows of \(R_u\).
    /// `damage` / `time` on `trial` are preserved.
    #[allow(clippy::too_many_arguments)]
    fn one_damped_newton_step_qs_r_u_inner(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
        damping: f32,
        fd_eps: f32,
        matrix_free_inner: bool,
    ) -> Result<QsRuNewtonStepTensors<B>, PhysicsError> {
        if !(damping > 0.0_f32 && damping <= 1.0_f32) {
            return Err(PhysicsError::Domain {
                detail: "one_damped_newton_step_with_quasi_static_r_u: damping must lie in (0, 1]".to_string(),
            });
        }
        if fd_eps <= 0.0_f32 {
            return Err(PhysicsError::Domain {
                detail: "one_damped_newton_step_with_quasi_static_r_u: fd_eps must be positive".to_string(),
            });
        }

        #[cfg(not(feature = "solver-experimental"))]
        let _ = matrix_free_inner;

        let t_dims = trial.thermal.temperature.as_tensor().dims();
        let h_dims = trial.hydro.humidity.as_tensor().dims();
        let a_dims = trial.chemical.reaction_extent.as_tensor().dims();
        let u_dims = trial.mechanical.displacement.as_tensor().dims();
        if t_dims[0] != 1 {
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step_with_quasi_static_r_u: batch must be 1, got {}",
                t_dims[0]
            ),
        });
        }
        if t_dims != h_dims
            || t_dims[0] != a_dims[0]
            || t_dims[1] != a_dims[1]
            || u_dims != [t_dims[0], t_dims[1], 3]
        {
            return Err(PhysicsError::ShapeMismatch {
                context: "one_damped_newton_step_with_quasi_static_r_u",
                detail: "T, h, α, u batch/node/shape mismatch",
            });
        }

        let n = t_dims[1];
        let f_t = t_dims[2];
        let f_h = h_dims[2];
        let f_a = a_dims[2];
        let m =
            ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, f_t, f_h, f_a);
        if m > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
            let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step_with_quasi_static_r_u: {m} stacked DOFs exceeds cap {cap}",
            ),
        });
        }

        let device = trial.thermal.temperature.as_tensor().device();
        let (r_t0, r_h0, r_a0, r_u0) = self.assemble_with_quasi_static_r_u(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        let norm_before_t = combined_four_residual_l2_tensor(&r_t0, &r_h0, &r_a0, &r_u0);
        let r0 = flatten_four_residuals(&r_t0, &r_h0, &r_a0, &r_u0);

        let mut packed = flatten_four_fields(
            trial.thermal.temperature.as_tensor(),
            trial.hydro.humidity.as_tensor(),
            trial.chemical.reaction_extent.as_tensor(),
            trial.mechanical.displacement.as_tensor(),
        );
        if packed.len() != m || r0.len() != m {
            return Err(PhysicsError::InvariantViolation {
                context: "one_damped_newton_step_with_quasi_static_r_u: internal flatten length mismatch",
            });
        }

        let t_shape = [t_dims[0], t_dims[1], t_dims[2]];
        let h_shape = [h_dims[0], h_dims[1], h_dims[2]];
        let a_shape = [a_dims[0], a_dims[1], a_dims[2]];
        let u_shape = [u_dims[0], u_dims[1], u_dims[2]];

        let active = field_major_newton_active_mask(n, f_t, f_h, f_a, boundary_mask_bn3)?;
        let m_a: usize = active.iter().filter(|&&a| a).count();
        if m_a == 0 {
            return Err(PhysicsError::Domain {
            detail: "one_damped_newton_step_with_quasi_static_r_u: zero active DOFs".to_string(),
        });
        }
        if m_a > THMC_DENSE_NEWTON_MAX_STACKED_DOFS {
            let cap = THMC_DENSE_NEWTON_MAX_STACKED_DOFS;
            return Err(PhysicsError::Domain {
            detail: format!(
                "one_damped_newton_step_with_quasi_static_r_u: {m_a} active DOFs exceeds cap {cap}",
            ),
        });
        }
        let red_map: Vec<usize> = (0..m).filter(|&j| active[j]).collect();
        let r0_red: Vec<f32> = red_map.iter().map(|&j| r0[j]).collect();

        let delta_red: Vec<f32> = (|| -> Result<Vec<f32>, PhysicsError> {
            #[cfg(feature = "solver-experimental")]
            if matrix_free_inner {
                let u_base = packed.clone();
                let rhs: Vec<f32> = r0_red.iter().map(|x| -x).collect();
                let matvec = |v_red: &[f32]| {
                    self.quasi_static_reduced_directional_fd_matvec(
                        v_red,
                        &u_base,
                        &red_map,
                        &r0,
                        fd_eps,
                        trial,
                        &device,
                        coords_n3,
                        boundary_mask_bn3,
                        body_force,
                        cross_section_area,
                        t_shape,
                        h_shape,
                        a_shape,
                        u_shape,
                    )
                };
                if let Ok(d) = gmres_f32_try(matvec, &rhs, m_a, m_a.saturating_add(12), 2e-3_f32) {
                    return Ok(d);
                }
            }

            let mut jac = vec![0.0_f32; m_a * m_a];
            for j_r in 0..m_a {
                let j_full = red_map[j_r];
                let eps_j = fd_eps * (1.0_f32 + packed[j_full].abs());
                packed[j_full] += eps_j;
                let pert = trial_from_packed_four(
                    trial, &device, &packed, t_shape, h_shape, a_shape, u_shape,
                );
                packed[j_full] -= eps_j;
                let r_pert = self.stacked_flat_residual_field_major_quasi_static(
                    &pert,
                    coords_n3,
                    boundary_mask_bn3,
                    body_force,
                    cross_section_area,
                )?;
                for i_r in 0..m_a {
                    let i_full = red_map[i_r];
                    jac[i_r * m_a + j_r] = (r_pert[i_full] - r0[i_full]) / eps_j;
                }
            }

            let mut rhs: Vec<f32> = r0_red.iter().map(|x| -x).collect();
            gauss_jordan_solve(&mut jac, &mut rhs, m_a)
        })()?;

        for k in 0..m_a {
            packed[red_map[k]] += damping * delta_red[k];
        }

        let new_trial =
            trial_from_packed_four(trial, &device, &packed, t_shape, h_shape, a_shape, u_shape);
        let (r_t1, r_h1, r_a1, r_u1) = self.assemble_with_quasi_static_r_u(
            &new_trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        let norm_after_t = combined_four_residual_l2_tensor(&r_t1, &r_h1, &r_a1, &r_u1);
        Ok((new_trial, norm_before_t, norm_after_t))
    }

    /// Public entrypoint: one damped Newton step; **`solver-experimental`** enables matrix-free **GMRES**
    /// on this call path only (see private helper `one_damped_newton_step_qs_r_u_inner`).
    #[allow(clippy::too_many_arguments)]
    pub fn one_damped_newton_step_with_quasi_static_r_u(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, f32, f32), PhysicsError> {
        self.one_damped_newton_step_qs_r_u_inner(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
            damping,
            fd_eps,
            true,
        )
        .map(|(trial, nb, na)| (trial, tensor1_f32_thmc(nb), tensor1_f32_thmc(na)))
    }

    /// Chains [`Self::one_damped_newton_step_with_quasi_static_r_u`] (`iterations >= 2`).
    ///
    /// **Stacked residual early exit** uses \(\|R\|_2\) built on-device; tolerance checks are
    /// [`Tensor::lower_elem`] + [`Tensor::all`] with a single [`tensor1_bool_thmc`] host read per test
    /// (see [`stacked_residual_newton_tol_met_tensor`]). [`Self::residual_l2_including_quasi_static_r_u`]
    /// still reports the same norm value via [`tensor1_f32_thmc`] when a scalar is required for callers.
    ///
    /// - When **`stacked_residual_l2_tolerance > 0`**, that predicate is **active** and requires
    ///   \(\|R\|_2 <\) `stacked_residual_l2_tolerance` for an exit.
    /// - When **`stacked_residual_relative_to_initial`** is **`Some(k)`** with **`k > 0`**, that
    ///   predicate is **active** and requires \(\|R\|_2 < k\cdot \max(\|R_0\|_2,\varepsilon)\).
    ///
    /// Tolerance exit triggers only when **at least one** predicate is active and **every** active
    /// predicate holds at the head iterate or after a completed damped Newton step. When no
    /// predicate is active, always perform exactly `iterations` damped Newton steps.
    ///
    /// **Follow-up (`m8-scale-ad`):** remaining `tensor1_f32_thmc` / `tensor1_bool_thmc` reads sync a
    /// single element for `Vec<f32>` reporting and branch control; keep them at these outer scopes.
    #[allow(clippy::too_many_arguments)]
    pub fn damped_newton_iterations_with_quasi_static_r_u(
        &self,
        trial: &ThmcState<B>,
        coords_n3: &Tensor<B, 2>,
        boundary_mask_bn3: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        cross_section_area: f32,
        iterations: usize,
        damping: f32,
        fd_eps: f32,
        stacked_residual_l2_tolerance: f32,
        stacked_residual_relative_to_initial: Option<f32>,
    ) -> Result<(ThmcState<B>, Vec<f32>), PhysicsError> {
        if iterations < 2 {
            return Err(PhysicsError::Domain {
                detail: "damped_newton_iterations_with_quasi_static_r_u: iterations must be >= 2".to_string(),
            });
        }
        let mut norms: Vec<f32> = Vec::with_capacity(iterations + 1);
        let (r_t0, r_h0, r_a0, r_u0) = self.assemble_with_quasi_static_r_u(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
        )?;
        let norm0_t = combined_four_residual_l2_tensor(&r_t0, &r_h0, &r_a0, &r_u0);
        let r0_t = norm0_t.clone();
        norms.push(tensor1_f32_thmc(norm0_t.clone()));
        if stacked_residual_newton_tol_met_tensor(
            &norm0_t,
            &r0_t,
            stacked_residual_l2_tolerance,
            stacked_residual_relative_to_initial,
        ) {
            return Ok((trial.clone(), norms));
        }

        let (mut current, _, after_first_t) = self.one_damped_newton_step_qs_r_u_inner(
            trial,
            coords_n3,
            boundary_mask_bn3,
            body_force,
            cross_section_area,
            damping,
            fd_eps,
            false,
        )?;
        norms.push(tensor1_f32_thmc(after_first_t.clone()));

        if stacked_residual_newton_tol_met_tensor(
            &after_first_t,
            &r0_t,
            stacked_residual_l2_tolerance,
            stacked_residual_relative_to_initial,
        ) {
            return Ok((current, norms));
        }

        for _ in 1..iterations {
            let (next, _, after_t) = self.one_damped_newton_step_qs_r_u_inner(
                &current,
                coords_n3,
                boundary_mask_bn3,
                body_force,
                cross_section_area,
                damping,
                fd_eps,
                false,
            )?;
            current = next;
            norms.push(tensor1_f32_thmc(after_t.clone()));
            if stacked_residual_newton_tol_met_tensor(
                &after_t,
                &r0_t,
                stacked_residual_l2_tolerance,
                stacked_residual_relative_to_initial,
            ) {
                break;
            }
        }
        Ok((current, norms))
    }
}

/// Stacked \(\|R\|_2\) Newton exit: every **active** tolerance predicate must hold (see
/// [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`]).
///
/// Uses lazy comparisons `norm - bound < 0` (strict `<` on scalars) with one [`tensor1_bool_thmc`]
/// read per active predicate, then ANDs on the host.
#[cfg(feature = "thmc-coupled")]
fn stacked_residual_newton_tol_met_tensor<B: Backend<FloatElem = f32>>(
    norm: &Tensor<B, 1>,
    r0: &Tensor<B, 1>,
    stacked_residual_l2_tolerance: f32,
    stacked_residual_relative_to_initial: Option<f32>,
) -> bool {
    let device = norm.device();
    let mut any_active = false;
    let mut all_pass = true;
    if stacked_residual_l2_tolerance > 0.0_f32 {
        any_active = true;
        let tol_t = Tensor::<B, 1>::zeros([1], &device).add_scalar(stacked_residual_l2_tolerance);
        let ok_abs = tensor1_bool_thmc(norm.clone().sub(tol_t).lower_elem(0.0_f32).all());
        all_pass &= ok_abs;
    }
    if let Some(rt) = stacked_residual_relative_to_initial {
        if rt > 0.0_f32 {
            any_active = true;
            let bound = r0.clone().clamp_min(1e-30_f32).mul_scalar(rt);
            let ok_rel = tensor1_bool_thmc(norm.clone().sub(bound).lower_elem(0.0_f32).all());
            all_pass &= ok_rel;
        }
    }
    any_active && all_pass
}

#[cfg(feature = "thmc-coupled")]
fn combined_three_residual_l2_tensor<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let sum_sq = r_t.clone().mul(r_t.clone()).sum()
        + r_h.clone().mul(r_h.clone()).sum()
        + r_a.clone().mul(r_a.clone()).sum();
    sum_sq.clamp_min(0.0_f32).sqrt()
}

#[cfg(feature = "thmc-coupled")]
fn combined_three_residual_l2<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> f32 {
    tensor1_f32_thmc(combined_three_residual_l2_tensor(r_t, r_h, r_a))
}

#[cfg(feature = "thmc-coupled")]
/// On-device ‖(R_T,R_h,R_α,R_u)‖₂ as a rank‑1 tensor (Burn subgraph; no host sync).
fn combined_four_residual_l2_tensor<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
    r_u: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let sum_sq = r_t.clone().mul(r_t.clone()).sum()
        + r_h.clone().mul(r_h.clone()).sum()
        + r_a.clone().mul(r_a.clone()).sum()
        + r_u.clone().mul(r_u.clone()).sum();
    sum_sq.clamp_min(0.0_f32).sqrt()
}

#[cfg(feature = "thmc-coupled")]
/// Host `f32` ‖(R_T,R_h,R_α,R_u)‖₂ for Newton / logging (**ConvergenceRequired — telemetry only**).
///
/// Implemented via [`combined_four_residual_l2_tensor`] + [`tensor1_f32_thmc`]; callers comparing
/// stacked norms across refactors should prefer this name over ad‑hoc renames of the tensor helper.
fn combined_four_residual_l2<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
    r_u: &Tensor<B, 3>,
) -> f32 {
    tensor1_f32_thmc(combined_four_residual_l2_tensor(r_t, r_h, r_a, r_u))
}

#[cfg(feature = "thmc-coupled")]
fn flatten_four_residuals<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
    r_u: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = r_t.clone().into_data().value;
    v.extend(r_h.clone().into_data().value);
    v.extend(r_a.clone().into_data().value);
    v.extend(r_u.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn flatten_three_fields<B: Backend<FloatElem = f32>>(
    t: &Tensor<B, 3>,
    h: &Tensor<B, 3>,
    alpha: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = t.clone().into_data().value;
    v.extend(h.clone().into_data().value);
    v.extend(alpha.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn flatten_three_residuals<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_h: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = r_t.clone().into_data().value;
    v.extend(r_h.clone().into_data().value);
    v.extend(r_a.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn trial_from_packed_three<B: Backend<FloatElem = f32>>(
    base: &ThmcState<B>,
    device: &B::Device,
    u: &[f32],
    t_shape: [usize; 3],
    h_shape: [usize; 3],
    a_shape: [usize; 3],
) -> ThmcState<B> {
    let nt: usize = t_shape.iter().product();
    let nh: usize = h_shape.iter().product();
    let na: usize = a_shape.iter().product();
    let t_new = Tensor::from_data(Data::new(u[..nt].to_vec(), Shape::new(t_shape)), device);
    let h_new = Tensor::from_data(
        Data::new(u[nt..nt + nh].to_vec(), Shape::new(h_shape)),
        device,
    );
    let a_new = Tensor::from_data(
        Data::new(u[nt + nh..nt + nh + na].to_vec(), Shape::new(a_shape)),
        device,
    );
    ThmcState::from_tensors(
    t_new,
    h_new,
    base.mechanical.displacement.as_tensor().clone(),
    a_new,
    base.damage.as_tensor().clone(),
    base.time,
)
}

#[cfg(feature = "thmc-coupled")]
fn field_major_newton_active_mask<B: Backend<FloatElem = f32>>(
    n: usize,
    f_t: usize,
    f_h: usize,
    f_a: usize,
    boundary_mask_bn3: &Tensor<B, 3>,
) -> Result<Vec<bool>, PhysicsError> {
    let dm = boundary_mask_bn3.dims();
    if dm[0] != 1 || dm[1] != n || dm[2] != 3 {
        return Err(PhysicsError::Domain {
            detail: format!(
            "field_major_newton_active_mask: boundary_mask dims {dm:?}, expected [1, {n}, 3]",
        ),
        });
    }
    let m = ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, f_t, f_h, f_a);
    let mut active = vec![true; m];
    let bm = boundary_mask_bn3.clone().into_data().value;
    let u0 = n * f_t + n * f_h + n * f_a;
    for i in 0..(n * 3) {
        let node = i / 3;
        let ax = i % 3;
        let free = bm[node * 3 + ax] > 1e-6_f32;
        active[u0 + i] = free;
    }
    Ok(active)
}

#[cfg(feature = "thmc-coupled")]
fn flatten_four_fields<B: Backend<FloatElem = f32>>(
    t: &Tensor<B, 3>,
    h: &Tensor<B, 3>,
    alpha: &Tensor<B, 3>,
    disp: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = flatten_three_fields(t, h, alpha);
    v.extend(disp.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn trial_from_packed_four<B: Backend<FloatElem = f32>>(
    base: &ThmcState<B>,
    device: &B::Device,
    u: &[f32],
    t_shape: [usize; 3],
    h_shape: [usize; 3],
    a_shape: [usize; 3],
    u_shape: [usize; 3],
) -> ThmcState<B> {
    let nt: usize = t_shape.iter().product();
    let nh: usize = h_shape.iter().product();
    let na: usize = a_shape.iter().product();
    let nu: usize = u_shape.iter().product();
    let t_new = Tensor::from_data(Data::new(u[..nt].to_vec(), Shape::new(t_shape)), device);
    let h_new = Tensor::from_data(
        Data::new(u[nt..nt + nh].to_vec(), Shape::new(h_shape)),
        device,
    );
    let a_new = Tensor::from_data(
        Data::new(u[nt + nh..nt + nh + na].to_vec(), Shape::new(a_shape)),
        device,
    );
    let disp_new = Tensor::from_data(
        Data::new(
            u[nt + nh + na..nt + nh + na + nu].to_vec(),
            Shape::new(u_shape),
        ),
        device,
    );
    ThmcState::from_tensors(
    t_new,
    h_new,
    disp_new,
    a_new,
    base.damage.as_tensor().clone(),
    base.time,
)
}

#[cfg(feature = "thmc-coupled")]
impl<B: Backend<FloatElem = f32>> ResidualThmc<B>
    for ThmcImplicitEulerThermalHumidityReactionExtentResidual<B>
{
    fn evaluate_residual(&self, trial: &ThmcState<B>) -> Result<(), PhysicsError> {
        self.assemble(trial).map(|_| ())
    }
}

#[cfg(feature = "thmc-coupled")]
fn combined_residual_l2_tensor<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> Tensor<B, 1> {
    let sum_sq = r_t.clone().mul(r_t.clone()).sum() + r_a.clone().mul(r_a.clone()).sum();
    sum_sq.clamp_min(0.0_f32).sqrt()
}

#[cfg(feature = "thmc-coupled")]
fn combined_residual_l2<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> f32 {
    tensor1_f32_thmc(combined_residual_l2_tensor(r_t, r_a))
}

#[cfg(feature = "thmc-coupled")]
fn flatten_two_fields<B: Backend<FloatElem = f32>>(
    t: &Tensor<B, 3>,
    alpha: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = t.clone().into_data().value;
    v.extend(alpha.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn flatten_two_residuals<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> Vec<f32> {
    let mut v = r_t.clone().into_data().value;
    v.extend(r_a.clone().into_data().value);
    v
}

#[cfg(feature = "thmc-coupled")]
fn trial_from_packed<B: Backend<FloatElem = f32>>(
    base: &ThmcState<B>,
    device: &B::Device,
    u: &[f32],
    t_shape: [usize; 3],
    a_shape: [usize; 3],
) -> ThmcState<B> {
    let nt: usize = t_shape.iter().product();
    let na: usize = a_shape.iter().product();
    let t_new = Tensor::from_data(Data::new(u[..nt].to_vec(), Shape::new(t_shape)), device);
    let a_new = Tensor::from_data(
        Data::new(u[nt..nt + na].to_vec(), Shape::new(a_shape)),
        device,
    );
    ThmcState::from_tensors(
    t_new,
    base.hydro.humidity.as_tensor().clone(),
    base.mechanical.displacement.as_tensor().clone(),
    a_new,
    base.damage.as_tensor().clone(),
    base.time,
)
}

/// Gauss–Jordan elimination with partial pivoting; overwrites `a` (row-major `n`×`n`) and `b` (`n`).
#[cfg(feature = "thmc-coupled")]
fn gauss_jordan_solve(a: &mut [f32], b: &mut [f32], n: usize) -> Result<Vec<f32>, PhysicsError> {
    for k in 0..n {
        // Pivot
        let mut piv = k;
        let mut best = a[k * n + k].abs();
        for r in (k + 1)..n {
            let v = a[r * n + k].abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        if best < 1e-20_f32 {
            return Err(PhysicsError::IndefiniteSystem {
            context: "gauss_jordan_solve: singular or ill-conditioned Jacobian",
        });
        }
        if piv != k {
            for c in 0..n {
                a.swap(k * n + c, piv * n + c);
            }
            b.swap(k, piv);
        }

        // Normalize pivot row
        let p = a[k * n + k];
        for c in 0..n {
            a[k * n + c] /= p;
        }
        b[k] /= p;

        // Eliminate column k from all other rows
        for r in 0..n {
            if r == k {
                continue;
            }
            let f = a[r * n + k];
            if f == 0.0_f32 {
                continue;
            }
            for c in 0..n {
                a[r * n + c] -= f * a[k * n + c];
            }
            b[r] -= f * b[k];
        }
    }

    Ok(b.to_vec())
}

#[cfg(feature = "thmc-coupled")]
impl<B: Backend<FloatElem = f32>> ResidualThmc<B>
    for ThmcImplicitEulerThermalReactionExtentResidual<B>
{
    fn evaluate_residual(&self, trial: &ThmcState<B>) -> Result<(), PhysicsError> {
        self.assemble(trial).map(|_| ())
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track 13 — implicit THMC residual assembly for Newton / JFNK.
//!
//! **Milestone (v0.4):** [`ThmcImplicitEulerThermalHydrationResidual`] implements the **backward Euler**
//! discrete residual for the **thermal + hydration \(\alpha\)** sub-block on the graph Laplacian used
//! by [`crate::physics::solvers::thmc::ThmcSolver`]:
//! \[
//! R_T = T - T^n - \Delta t\,\bigl(\mathcal{L}(T) + q_{\mathrm{exo}}\,\dot\alpha(\alpha,T)\bigr),\qquad
//! R_\alpha = \alpha - \alpha^n - \Delta t\,\dot\alpha(\alpha,T),
//! \]
//! with \(\dot\alpha\) from [`crate::physics::solvers::thmc::full_hydration_alpha_rate_tensor`]
//! ([`crate::physics::solvers::thmc::ThmcHydrationKinetics`]). Humidity, mechanics, and fracture are
//! **out of scope** for this struct.
//!
//! **Damped Newton on \((T,\alpha)\) (feature `thmc-coupled`):** [`ThmcImplicitEulerThermalHydrationResidual::one_damped_newton_step`]
//! builds a dense forward-difference Jacobian on a **small** stacked state (cap on total DOFs) and
//! applies one damped Newton update \(U \leftarrow U - \omega J^{-1} R\).
//! [`ThmcImplicitEulerThermalHydrationResidual::damped_newton_iterations`] chains that step **≥ 2**
//! times (fresh Jacobian each iteration). Track 13 stepping stone toward full JFNK; humidity and
//! mechanics remain out of scope here. See `docs/research/v0.4_track13_monolithic_newton_thmc.md`.

use burn::tensor::backend::Backend;

#[cfg(feature = "thmc-coupled")]
use burn::tensor::Data;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Int;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Shape;
#[cfg(feature = "thmc-coupled")]
use burn::tensor::Tensor;

#[cfg(feature = "thmc-coupled")]
use crate::physics::laplacian::TopologicalLaplacian;
#[cfg(feature = "thmc-coupled")]
use crate::physics::solvers::thmc::{
    full_hydration_alpha_rate_tensor, ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan,
    ThmcHydrationKinetics, ThmcState,
};

#[cfg(not(feature = "thmc-coupled"))]
use super::thmc::ThmcState;

/// Coupled implicit backward-Euler residual \(R(U)=U-U^n-\Delta t\,F(U)\) for THMC.
///
/// Unknown stack \(U=(T,h,\alpha,u)\) ordering is implementation-defined; block preconditioners
/// should follow the same layout as the residual vector.
///
/// **Shipped (v0.4, feature `thmc-coupled`):** `ThmcImplicitEulerThermalHydrationResidual` implements
/// real residual assembly / [`ResidualThmc::evaluate_residual`], tied to hydration kinetics (see
/// `thmc_implicit_euler_t_alpha_residual_matches_brute_force_two_nodes` in `tests/verification/thmc_drying_shrinkage.rs`).
/// Other future residual bundles should override the default [`evaluate_residual`](ResidualThmc::evaluate_residual) stub below.
pub trait ResidualThmc<B: Backend<FloatElem = f32>> {
    /// Machine-readable description of DOF ordering (for docs / tests).
    fn dof_ordering_note() -> &'static str {
        "field-major or node-major T,h,alpha,u — must match JFNK Vec layout"
    }

    /// Evaluate the residual map at `trial` (implementation-defined contract).
    fn evaluate_residual(&self, trial: &ThmcState<B>) -> Result<(), String> {
        let _ = trial;
        Err("ResidualThmc::evaluate_residual not implemented".into())
    }
}

/// Backward-Euler residual for **thermal + hydration** on a fixed graph, \(F\) evaluated at the trial
/// state (consistent implicit step).
#[cfg(feature = "thmc-coupled")]
#[derive(Clone, Debug)]
pub struct ThmcImplicitEulerThermalHydrationResidual<B: Backend<FloatElem = f32>> {
    pub dt: f32,
    pub temperature_n: Tensor<B, 3>,
    pub alpha_n: Tensor<B, 3>,
    pub edges_b1: Tensor<B, 2, Int>,
    pub damage_m: Tensor<B, 3>,
    pub kinetics: ThmcHydrationKinetics,
}

#[cfg(feature = "thmc-coupled")]
impl<B: Backend<FloatElem = f32>> ThmcImplicitEulerThermalHydrationResidual<B> {
    /// Assemble \(R_T, R_\alpha\) at `trial` (same shapes as `temperature` / `hydration_alpha` plans).
    pub fn assemble(&self, trial: &ThmcState<B>) -> Result<(Tensor<B, 3>, Tensor<B, 3>), String> {
        let t = trial.thermal.temperature.clone();
        let alpha = trial.chemical.hydration_alpha.clone();
        let device = t.device();
        let batch = t.dims()[0];
        let n = t.dims()[1];
        if self.temperature_n.dims() != t.dims() {
            return Err(format!(
                "ThmcImplicitEulerThermalHydrationResidual: T^n dims {:?} != trial T dims {:?}",
                self.temperature_n.dims(),
                t.dims()
            ));
        }
        if self.alpha_n.dims() != alpha.dims() {
            return Err(format!(
                "ThmcImplicitEulerThermalHydrationResidual: α^n dims {:?} != trial α dims {:?}",
                self.alpha_n.dims(),
                alpha.dims()
            ));
        }

        let lap_t = TopologicalLaplacian::scalar_laplacian(
            t.clone(),
            self.edges_b1.clone(),
            self.damage_m.clone(),
        );
        let dt_lap_t = lap_t.mul_scalar(self.dt);

        let f_alpha_ch = alpha.dims()[2];
        let t_bn1 = t.clone().slice([0..batch, 0..n, 0..1]);
        let temperature_for_alpha = if f_alpha_ch == 1 {
            t_bn1
        } else {
            t_bn1.expand::<3, _>([batch, n, f_alpha_ch])
        };
        let d_alpha = full_hydration_alpha_rate_tensor(
            &self.kinetics,
            alpha.clone(),
            temperature_for_alpha,
            &device,
        );

        let f_t_ch = t.dims()[2];
        let exo = d_alpha
            .clone()
            .slice([0..batch, 0..n, 0..1])
            .mul_scalar(self.kinetics.exothermic_k_per_alpha_rate * self.dt)
            .expand::<3, _>([batch, n, f_t_ch]);

        let r_t = t.sub(self.temperature_n.clone()).sub(dt_lap_t).sub(exo);
        let r_alpha = alpha
            .sub(self.alpha_n.clone())
            .sub(d_alpha.mul_scalar(self.dt));
        Ok((r_t, r_alpha))
    }

    /// Combined Euclidean norm \(\sqrt{\|R_T\|_2^2 + \|R_\alpha\|_2^2}\) at `trial`.
    pub fn residual_l2(&self, trial: &ThmcState<B>) -> Result<f32, String> {
        let (r_t, r_a) = self.assemble(trial)?;
        Ok(combined_residual_l2(&r_t, &r_a))
    }

    /// One **damped Newton** step on the coupled \((T,\alpha)\) backward-Euler residual.
    ///
    /// - **Jacobian:** forward finite differences on the stacked unknown
    ///   \((T,\alpha)\) in flattened `[B,N,F]` order (thermal entries first, then chemical; same layout
    ///   as [`Self::assemble`]).
    /// - **Linear solve:** Gauss–Jordan elimination with partial pivoting on the dense system
    ///   \(J\,\delta = -R\) (host `f32`, intended for small chains / verification only).
    /// - **Scope:** requires `trial.thermal.temperature.dims()[0] == 1` and
    ///   `n (f_T + f_\alpha) \le 64` (hard cap).
    ///
    /// Returns `(updated_trial, \|R\|_2 \text{ before}, \|R\|_2 \text{ after})`. Hydro / mechanics /
    /// damage / time on `trial` are preserved; only `temperature` and `hydration_alpha` change.
    ///
    /// ## Multi-step
    /// For two or more sequential correctors with a fresh Jacobian each time, use
    /// [`Self::damped_newton_iterations`].
    pub fn one_damped_newton_step(
        &self,
        trial: &ThmcState<B>,
        damping: f32,
        fd_eps: f32,
    ) -> Result<(ThmcState<B>, f32, f32), String> {
        const MAX_DOFS: usize = 64;
        if !(damping > 0.0_f32 && damping <= 1.0_f32) {
            return Err("one_damped_newton_step: damping must lie in (0, 1]".into());
        }
        if fd_eps <= 0.0_f32 {
            return Err("one_damped_newton_step: fd_eps must be positive".into());
        }

        let t_dims = trial.thermal.temperature.dims();
        let a_dims = trial.chemical.hydration_alpha.dims();
        if t_dims[0] != 1 {
            return Err(format!(
                "one_damped_newton_step: batch must be 1, got {}",
                t_dims[0]
            ));
        }
        if t_dims[0] != a_dims[0] || t_dims[1] != a_dims[1] {
            return Err("one_damped_newton_step: T and α batch/node counts must match".into());
        }

        let n = t_dims[1];
        let f_t = t_dims[2];
        let f_a = a_dims[2];
        let m = n * f_t + n * f_a;
        if m > MAX_DOFS {
            return Err(format!(
                "one_damped_newton_step: {} stacked DOFs exceeds cap {}",
                m, MAX_DOFS
            ));
        }

        let device = trial.thermal.temperature.device();
        let (r_t0, r_a0) = self.assemble(trial)?;
        let norm_before = combined_residual_l2(&r_t0, &r_a0);
        let r0 = flatten_two_residuals(&r_t0, &r_a0);

        let mut u = flatten_two_fields(&trial.thermal.temperature, &trial.chemical.hydration_alpha);
        if u.len() != m || r0.len() != m {
            return Err("one_damped_newton_step: internal flatten length mismatch".into());
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
            let r_pert = flatten_two_residuals(&r_tp, &r_ap);
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
    ) -> Result<(ThmcState<B>, Vec<f32>), String> {
        if iterations < 2 {
            return Err("damped_newton_iterations: iterations must be >= 2".into());
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

#[cfg(feature = "thmc-coupled")]
fn combined_residual_l2<B: Backend<FloatElem = f32>>(
    r_t: &Tensor<B, 3>,
    r_a: &Tensor<B, 3>,
) -> f32 {
    let s = r_t.clone().mul(r_t.clone()).sum().into_scalar()
        + r_a.clone().mul(r_a.clone()).sum().into_scalar();
    s.max(0.0_f32).sqrt()
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
    ThmcState {
        thermal: ThermalPlan { temperature: t_new },
        hydro: HydrologicPlan {
            humidity: base.hydro.humidity.clone(),
        },
        mechanical: MechanicalPlan {
            displacement: base.mechanical.displacement.clone(),
        },
        chemical: ChemicalPlan {
            hydration_alpha: a_new,
        },
        damage: base.damage.clone(),
        time: base.time,
    }
}

/// Gauss–Jordan elimination with partial pivoting; overwrites `a` (row-major `n`×`n`) and `b` (`n`).
#[cfg(feature = "thmc-coupled")]
fn gauss_jordan_solve(a: &mut [f32], b: &mut [f32], n: usize) -> Result<Vec<f32>, String> {
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
            return Err("gauss_jordan_solve: singular or ill-conditioned Jacobian".into());
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
impl<B: Backend<FloatElem = f32>> ResidualThmc<B> for ThmcImplicitEulerThermalHydrationResidual<B> {
    fn evaluate_residual(&self, trial: &ThmcState<B>) -> Result<(), String> {
        self.assemble(trial).map(|_| ())
    }
}

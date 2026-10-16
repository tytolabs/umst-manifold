// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Helmholtz-type PDE density filter on the discrete primal graph ([Lazarov & Sigmund 2011]).
//!
//! Continuum **density filter**: \((I - r^2\nabla^2)\tilde\rho=\rho\) with \(-\nabla^2\) positive.
//! [`TopologicalLaplacian::scalar_laplacian`](super::laplacian::TopologicalLaplacian::scalar_laplacian)
//! equals **minus** the usual PSD graph Laplacian \(L_{\mathrm{std}}\), so the SPD resolvent is
//! **\((I - (r^2/\mathrm{d}x^2)\,L_{\mathrm{ours}})\tilde\rho
//!   = (I + (r^2/\mathrm{d}x^2)\,L_{\mathrm{std}})\tilde\rho = \rho\)**.
//!
//! formal_anchor: Literature
//! formal_citation: Lazarov & Sigmund 2011, Int. J. Numer. Meth. Engng. 86:765-781
//! formal_form: \((I + (r^2/\mathrm{d}x^2)\,L_{\mathrm{std}})\tilde\rho=\rho\); implemented as \((I - s L_{\mathrm{ours}})\tilde\rho=\rho\), \(s=r^2/\mathrm{d}x^2\)
//!
//! Enabled with **`topology-density-evolution`** / **`solver-experimental`**.
//!
//! # Solve
//!
//! [`HelmholtzFilter`] validates domain and shapes, solves the discrete Helmholtz resolvent by
//! Richardson iteration with a step from the Gershgorin bound, and offers a straight-through
//! AD reattach. The iteration stops on the relative \(\ell_2\) residual against `cg_tolerance`
//! (finite, \(>0\)) or refuses; `max_cg_iterations` is historical API naming only.

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Int, Tensor,
};

use ordered_float::NotNan;
use umst_cartridge_api::convex_phi::PassivePhi;
use umst_math::solve_combinator::{
    CombinatorRefuse, EnergyBudget, ProblemProgressWindow, ProblemTolerance, UnfoldStop,
};
use umst_math::solve_kleisli::NestedRefuse;

use super::error::PhysicsError;
use super::laplacian::TopologicalLaplacian;
use crate::solve::dissipation_step::{
    dissipative_unfold, DissipationStepRefuse, DissipativeStep, StepKind,
};
use crate::solve::PackageEnergyMeter;

fn validate_helmholtz_inputs<B: Backend>(
    rho: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    dx: f32,
    radius: f32,
) -> Result<(), PhysicsError> {
    if !dx.is_finite() || dx <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!("HelmholtzFilter::apply: dx must be finite and positive (got {dx})"),
        });
    }
    if !radius.is_finite() || radius <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: format!(
                "HelmholtzFilter::apply: radius must be finite and positive (got {radius})"
            ),
        });
    }
    let [_, n, c] = rho.dims();
    if c < 1 {
        return Err(PhysicsError::ShapeMismatch {
            context: "HelmholtzFilter::apply",
            detail: "rho channel count must be >= 1",
        });
    }
    let [eb_two, e] = edges_b1.dims();
    if eb_two != 2 {
        return Err(PhysicsError::ShapeMismatch {
            context: "HelmholtzFilter::apply",
            detail: "edges_b1 must be [2, E]",
        });
    }
    if e == 0 && n > 1 {
        return Err(PhysicsError::Domain {
            detail: "HelmholtzFilter::apply: zero edges with multiple nodes".into(),
        });
    }
    Ok(())
}

/// Precondition for Richardson stationary solve: `cg_tolerance` must be finite and positive.
fn helmholtz_richardson_residual_tol_precondition(tol: f32) -> Result<f32, PhysicsError> {
    if tol.is_finite() && tol > 0.0 {
        Ok(tol)
    } else {
        Err(PhysicsError::InvariantViolation {
            context: "HelmholtzFilter::apply: cg_tolerance",
        })
    }
}

/// Helmholtz PDE filter: solves \((I - (r^2/\mathrm{d}x^2)L_{\mathrm{ours}})\tilde\rho=\rho\) on the graph.
#[derive(Clone, Debug)]
pub struct HelmholtzFilter {
    pub radius: f32,
    pub max_cg_iterations: usize,
    pub cg_tolerance: f32,
    /// Energy budget of the metered solve; `None` runs the unmetered Richardson loop.
    pub energy_budget: Option<EnergyBudget>,
}

impl HelmholtzFilter {
    /// `radius` is the filter radius in **physical** units (same system as `dx`).
    #[must_use]
    pub fn new(radius: f32, max_cg_iterations: usize, cg_tolerance: f32) -> Self {
        Self {
            radius,
            max_cg_iterations,
            cg_tolerance,
            energy_budget: None,
        }
    }

    /// Run the filter as a metered unfold under `budget`: each Richardson step is a
    /// dissipative step on the passive route (`ψ = ½xᵀAx`, `φ = ½|v|²`) admitted by the core
    /// gate, metered by the host package energy counter.
    #[must_use]
    pub fn with_energy_budget(self, budget: EnergyBudget) -> Self {
        Self {
            energy_budget: Some(budget),
            ..self
        }
    }

    /// Apply filter: `rho` and return `[B, N, C]` filtered field (same shape).
    pub fn apply<B: Backend<FloatElem = f32>>(
        &self,
        rho: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        dx: f32,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        validate_helmholtz_inputs(&rho, &edges_b1, dx, self.radius)?;
        let tol_use = helmholtz_richardson_residual_tol_precondition(self.cg_tolerance)?;
        let dx_safe = dx.max(1e-30);
        let scale = (self.radius / dx_safe).powi(2);
        let damage = Tensor::<B, 3>::zeros_like(&rho);
        match self.energy_budget {
            Some(budget) => helmholtz_unfold(rho, edges_b1, damage, scale, tol_use, budget),
            None => helmholtz_stationary(rho, edges_b1, damage, scale, tol_use),
        }
    }

    /// Helmholtz on the **inner** backend, re-attached with straight-through gradients (B6 H2).
    pub fn apply_straight_through<B: AutodiffBackend<FloatElem = f32>>(
        &self,
        rho: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        dx: f32,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        let device = rho.device();
        let filtered_inner = self.apply(rho.clone().inner(), edges_b1.inner(), dx)?;
        let filtered = Tensor::<B, 3>::from_data(filtered_inner.into_data(), &device);
        let rho_st = rho.clone();
        Ok(rho_st + (filtered - rho).detach())
    }
}

/// Gershgorin upper bound on λmax of `A = I − s·L_ours = I + s·L_std` for edge coefficients
/// in `[0, 1]`: row `i` of `L_std` has absolute row sum `2·deg(i)`, so
/// `λmax(A) ≤ 1 + 2·s·max_i deg(i)` (Gershgorin 1931), with `deg` counting every incidence
/// in the edge list.
fn helmholtz_lambda_upper<B: Backend<FloatElem = f32>>(
    edges_b1: &Tensor<B, 2, Int>,
    n_nodes: usize,
    scale: f32,
) -> Result<f32, PhysicsError> {
    let mut degree = vec![0usize; n_nodes];
    for node in edges_b1.clone().into_data().convert::<i64>().value {
        let i = usize::try_from(node).ok().filter(|i| *i < n_nodes).ok_or(
            PhysicsError::InvariantViolation {
                context: "HelmholtzFilter::apply edge index outside the node range",
            },
        )?;
        degree[i] += 1;
    }
    let max_degree = degree.into_iter().fold(0usize, usize::max) as f32;
    let bound = 1.0 + 2.0 * scale * max_degree;
    if bound.is_finite() {
        Ok(bound)
    } else {
        Err(PhysicsError::NonFinite {
            context: "HelmholtzFilter::apply Gershgorin bound",
        })
    }
}

/// Richardson iteration `x ← x + ω(ρ − A x)` with `ω = 1/λ̄`, `λ̄` the Gershgorin bound.
///
/// `A` is symmetric with spectrum in `[1, λ̄]`, so `‖I − ωA‖₂ ≤ q = 1 − 1/λ̄` and the residual
/// contracts by `q` per step; `⌈ln(tol)/ln(q)⌉` steps reach `tol` in exact arithmetic. A run that
/// exceeds that count by a stall window, or whose residual stops decreasing over the stall
/// window, has not met
/// its tolerance and is refused with its last relative residual
/// ([`PhysicsError::Diverged`]); no unconverged field is returned.
fn helmholtz_stationary<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    scale: f32,
    tol: f32,
) -> Result<Tensor<B, 3>, PhysicsError> {
    let mut x = Tensor::<B, 3>::zeros_like(&rhs);
    let n_nodes = rhs.dims()[1];
    let lambda_upper = helmholtz_lambda_upper(&edges_b1, n_nodes, scale)?;
    let omega = 1.0 / lambda_upper;
    let rhs_norm = rhs.clone().powf_scalar(2.0).sum().into_data().value[0]
        .sqrt()
        .max(1e-20);
    if !rhs_norm.is_finite() {
        return Err(PhysicsError::NonFinite {
            context: "HelmholtzFilter::apply rhs norm",
        });
    }
    let contraction = f64::from(1.0 - omega);
    let step_bound = if contraction > 0.0 {
        ((f64::from(tol)).ln() / contraction.ln()).ceil().max(1.0) as usize
    } else {
        1
    };
    // In exact arithmetic the residual halves within `⌈ln ½ / ln q⌉` steps, so a run whose best
    // residual does not improve for that many steps sits on its binary32 rounding floor; the
    // same window is the slack the run may take beyond the exact count.
    let stall_window = if contraction > 0.0 {
        ((0.5_f64).ln() / contraction.ln()).ceil().max(1.0) as usize
    } else {
        1
    };
    let step_bound = step_bound.saturating_add(stall_window);
    let mut best_r = f32::INFINITY;
    let mut stall = 0usize;
    let mut richardson_iter = 0usize;
    loop {
        let lx =
            TopologicalLaplacian::scalar_laplacian(x.clone(), edges_b1.clone(), damage.clone());
        let ax = x.clone().sub(lx.mul_scalar(scale));
        let resid = rhs.clone().sub(ax);
        let r_norm = resid.clone().powf_scalar(2.0).sum().into_data().value[0].sqrt();
        if !r_norm.is_finite() {
            return Err(PhysicsError::NonFinite {
                context: "HelmholtzFilter::apply Richardson residual",
            });
        }
        if r_norm <= tol * rhs_norm {
            break;
        }
        if r_norm < best_r {
            best_r = r_norm;
            stall = 0;
        } else {
            stall += 1;
        }
        if richardson_iter >= step_bound || stall >= stall_window {
            return Err(PhysicsError::Diverged {
                eq_rel: r_norm / rhs_norm,
                pcg_iterations: richardson_iter,
            });
        }
        richardson_iter += 1;
        x = x.clone().add(resid.mul_scalar(omega));
    }
    if x.clone().into_data().value.iter().any(|v| !v.is_finite()) {
        return Err(PhysicsError::NonFinite {
            context: "HelmholtzFilter::apply filtered field",
        });
    }
    Ok(x)
}

/// The Helmholtz energy on the host: `ψ(x) = ½xᵀAx`, `w(x) = ρᵀx`, `φ(v) = ½|v|²`, with
/// `A = I − s·L_ours` assembled from the edge list in binary64. The state is the field
/// `[batch, node, channel]` flattened row-major.
struct HelmholtzEnergy {
    nodes: usize,
    channels: usize,
    batches: usize,
    edges: Vec<(usize, usize)>,
    /// Flow coefficient `1 − (d_s + d_t)/2` per `(batch, edge, channel)`.
    coefficient: Vec<f64>,
    scale: f64,
    rho: Vec<f64>,
    phi: PassivePhi,
    /// Steps proposed so far.
    proposals: std::cell::Cell<usize>,
}

impl HelmholtzEnergy {
    fn index(&self, b: usize, i: usize, c: usize) -> usize {
        (b * self.nodes + i) * self.channels + c
    }

    /// `A x`.
    fn apply_a(&self, x: &[f64]) -> Vec<f64> {
        let mut ax = x.to_vec();
        let e_count = self.edges.len();
        for b in 0..self.batches {
            for (e, &(src, tgt)) in self.edges.iter().enumerate() {
                for c in 0..self.channels {
                    let k = self.coefficient[(b * e_count + e) * self.channels + c];
                    let (is, it) = (self.index(b, src, c), self.index(b, tgt, c));
                    let flow = k * (x[it] - x[is]);
                    // L_ours adds +flow at the source and −flow at the target; A = I − s·L_ours.
                    ax[is] -= self.scale * flow;
                    ax[it] += self.scale * flow;
                }
            }
        }
        ax
    }

    fn relative_residual(&self, x: &[f64]) -> f64 {
        let ax = self.apply_a(x);
        let r: f64 = self
            .rho
            .iter()
            .zip(&ax)
            .map(|(p, a)| (p - a) * (p - a))
            .sum();
        r.sqrt() / norm(&self.rho)
    }
}

fn norm(x: &[f64]) -> f64 {
    x.iter().map(|v| v * v).sum::<f64>().sqrt()
}

impl DissipativeStep for HelmholtzEnergy {
    fn free_energy(&self, x: &[f64]) -> f64 {
        0.5 * x
            .iter()
            .zip(self.apply_a(x))
            .map(|(a, b)| a * b)
            .sum::<f64>()
    }
    fn dissipation(&self) -> &PassivePhi {
        &self.phi
    }
    fn work(&self, x: &[f64]) -> f64 {
        self.rho.iter().zip(x).map(|(p, v)| p * v).sum()
    }
    fn kind(&self) -> StepKind {
        StepKind::Explicit
    }
    fn propose(&self, x0: &[f64], tau: f64) -> Result<Vec<f64>, CombinatorRefuse> {
        self.proposals.set(self.proposals.get() + 1);
        let ax = self.apply_a(x0);
        Ok(x0
            .iter()
            .zip(&ax)
            .zip(&self.rho)
            .map(|((x, a), p)| x + tau * (p - a))
            .collect())
    }
}

/// `φ(v) = ½|v|²` over `dim` coordinates as a passive grammar tree of balanced depth.
fn half_square_norm(dim: usize) -> Result<PassivePhi, PhysicsError> {
    let refused = |_| PhysicsError::InvariantViolation {
        context: "HelmholtzFilter metered solve: passive dissipation potential",
    };
    let mut level: Vec<PassivePhi> = (0..dim)
        .map(|i| PassivePhi::power(dim, i, 2.0).map_err(refused))
        .collect::<Result<_, _>>()?;
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut it = level.into_iter();
        while let Some(a) = it.next() {
            next.push(match it.next() {
                Some(b) => PassivePhi::sum(a, b).map_err(refused)?,
                None => a,
            });
        }
        level = next;
    }
    let root = level.pop().ok_or(PhysicsError::InvariantViolation {
        context: "HelmholtzFilter metered solve: empty field",
    })?;
    PassivePhi::scale(0.5, root).map_err(refused)
}

fn host_values<B: Backend<FloatElem = f32>>(t: Tensor<B, 3>) -> Vec<f64> {
    t.into_data().value.into_iter().map(f64::from).collect()
}

/// The Helmholtz solve as a metered unfold of dissipative steps: the step is the Richardson
/// step `x + ω(ρ − A x)` with `ω = 1/λ̄`, a forward step on `ψ − w` that the core gate admits
/// under `φ = ½|v|²` because `ω·λmax(A) ≤ 1`. The gate tolerance is the stated rounding bound
/// `4·n·ε·λ̄²·‖ρ‖²` of the binary64 balance over `n` values (Higham 2002, *Accuracy and
/// Stability of Numerical Algorithms*, §3.1, `γ_n`).
fn helmholtz_unfold<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    scale: f32,
    tol: f32,
    budget: EnergyBudget,
) -> Result<Tensor<B, 3>, PhysicsError> {
    let device = rhs.device();
    let [batches, nodes, channels] = rhs.dims();
    let damage_channels = damage.dims()[2];
    let lambda_upper = f64::from(helmholtz_lambda_upper(&edges_b1, nodes, scale)?);
    let edge_list: Vec<i64> = edges_b1.into_data().convert::<i64>().value;
    let e_count = edge_list.len() / 2;
    let node = |v: i64| {
        usize::try_from(v)
            .ok()
            .filter(|i| *i < nodes)
            .ok_or(PhysicsError::InvariantViolation {
                context: "HelmholtzFilter::apply edge index outside the node range",
            })
    };
    let edges: Vec<(usize, usize)> = (0..e_count)
        .map(|e| Ok((node(edge_list[e])?, node(edge_list[e_count + e])?)))
        .collect::<Result<_, PhysicsError>>()?;
    let d = host_values(damage);
    let mut coefficient = Vec::with_capacity(batches * e_count * channels);
    for b in 0..batches {
        for &(src, tgt) in &edges {
            for c in 0..channels {
                let dc = if damage_channels == channels { c } else { 0 };
                let at = |i: usize| d[(b * nodes + i) * damage_channels + dc];
                coefficient.push(1.0 - (at(src) + at(tgt)) / 2.0);
            }
        }
    }
    let rho = host_values(rhs);
    let n = rho.len();
    let energy = HelmholtzEnergy {
        nodes,
        channels,
        batches,
        edges,
        coefficient,
        scale: f64::from(scale),
        phi: half_square_norm(n)?,
        rho,
        proposals: std::cell::Cell::new(0),
    };
    let omega = 1.0 / lambda_upper;
    let halving = ((0.5_f64).ln() / (1.0 - omega).ln()).ceil().max(1.0);
    let window = ProblemProgressWindow::from_krylov_restart(
        u32::try_from(halving as u64).map_err(|_| PhysicsError::InvariantViolation {
            context: "HelmholtzFilter metered solve: progress window",
        })?,
    )
    .map_err(combinator)?;
    let stop = UnfoldStop::new(
        ProblemTolerance::from_problem(1.0, f64::from(tol)).map_err(combinator)?,
        budget,
        window,
        64.0 * n as f64,
    )
    .map_err(combinator)?;
    let gate_tolerance =
        4.0 * n as f64 * f64::EPSILON * lambda_upper * lambda_upper * norm(&energy.rho).powi(2);
    let solved = dissipative_unfold(
        &[&energy],
        |x: &[f64]| {
            NotNan::new(energy.relative_residual(x))
                .map_err(|_| CombinatorRefuse::NonFiniteQuantity)
        },
        vec![0.0; n],
        omega,
        gate_tolerance,
        stop,
        &PackageEnergyMeter,
    )
    .map_err(|r| refusal(r, energy.proposals.get()))?;
    let field: Vec<f32> = solved.value().value().iter().map(|v| *v as f32).collect();
    Ok(Tensor::from_data(
        burn::tensor::Data::new(field, burn::tensor::Shape::new([batches, nodes, channels])),
        &device,
    ))
}

fn combinator(_: CombinatorRefuse) -> PhysicsError {
    PhysicsError::InvariantViolation {
        context: "HelmholtzFilter metered solve: stop policy",
    }
}

/// The metered solve's refusal as the filter's typed error; a stall or a spent budget
/// carries the last relative residual.
fn refusal(r: DissipationStepRefuse, steps: usize) -> PhysicsError {
    match r {
        DissipationStepRefuse::Outer(NestedRefuse::Stalled { evidence, .. }) => {
            PhysicsError::Diverged {
                eq_rel: evidence.residual.into_inner() as f32,
                pcg_iterations: steps,
            }
        }
        DissipationStepRefuse::Outer(NestedRefuse::BudgetSpent { progress, .. }) => {
            PhysicsError::Diverged {
                eq_rel: progress.residual.into_inner() as f32,
                pcg_iterations: steps,
            }
        }
        DissipationStepRefuse::Rejected { .. } => PhysicsError::GateEvidenceRejected {
            context: "HelmholtzFilter metered step rejected by the core gate",
        },
        DissipationStepRefuse::NonFinite => PhysicsError::NonFinite {
            context: "HelmholtzFilter metered step",
        },
        _ => PhysicsError::InvariantViolation {
            context: "HelmholtzFilter metered solve refused",
        },
    }
}

#[cfg(test)]
mod precondition_tests {
    use super::*;

    #[test]
    fn topology_filter_new_stores_constructor_fields() {
        let f = HelmholtzFilter::new(1.0, 0, 0.0);
        assert_eq!(f.max_cg_iterations, 0);
        assert_eq!(f.cg_tolerance, 0.0);
        assert_eq!(f.radius, 1.0);
    }

    /// Precondition: non-positive `cg_tolerance` is refused before the Richardson stationary loop.
    #[test]
    fn helmholtz_apply_must_refuse_non_positive_cg_tolerance() {
        assert!(helmholtz_richardson_residual_tol_precondition(0.0).is_err());
        assert!(helmholtz_richardson_residual_tol_precondition(-1e-8).is_err());
        assert!(helmholtz_richardson_residual_tol_precondition(f32::NAN).is_err());
        assert!(helmholtz_richardson_residual_tol_precondition(f32::INFINITY).is_err());
        match helmholtz_richardson_residual_tol_precondition(1e-7) {
            Ok(t) => assert_eq!(t, 1e-7),
            Err(_) => panic!("finite positive cg_tolerance must be accepted"),
        }
        match helmholtz_richardson_residual_tol_precondition(0.0) {
            Err(PhysicsError::InvariantViolation { context }) => {
                assert_eq!(context, "HelmholtzFilter::apply: cg_tolerance");
            }
            _ => panic!("non-positive cg_tolerance must InvariantViolation"),
        }
    }
}

#[cfg(all(test, feature = "topology-density-evolution"))]
mod tests {
    use super::*;
    use crate::physics::laplacian::TopologicalLaplacian;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn tensor_max(values: &[f32]) -> f32 {
        values.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    }

    #[test]
    fn burn_delta_residual_has_positive_mass() {
        let dev = Default::default();
        let n = 16usize;
        let mut rho = vec![0.0f32; n];
        rho[8] = 1.0;
        let rho_t: Tensor<B, 3> = Tensor::from_data(Data::new(rho, Shape::new([1, n, 1])), &dev);
        let mut e = Vec::with_capacity((n - 1) * 2);
        for i in 0..(n - 1) {
            e.push(i as i64);
        }
        for i in 0..(n - 1) {
            e.push((i + 1) as i64);
        }
        let edges: Tensor<B, 2, burn::tensor::Int> = Tensor::<B, 1>::from_data(
            Data::new(e.iter().map(|&x| x as f32).collect(), Shape::new([e.len()])),
            &dev,
        )
        .reshape([2, n - 1])
        .int();
        let x0 = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let zdmg = Tensor::<B, 3>::zeros_like(&rho_t);
        let lx = TopologicalLaplacian::scalar_laplacian(x0.clone(), edges.clone(), zdmg.clone());
        let ax = x0.clone().sub(lx.mul_scalar(2.25));
        let resid = rho_t.clone().sub(ax);
        let v = resid.into_data().value;
        let mx = tensor_max(&v);
        assert!(mx > 0.9, "delta residual max={mx}");
    }

    #[test]
    fn striatus_extruded_plate_inner_finite() -> Result<(), PhysicsError> {
        use crate::physics::extruded_plate::ExtrudedPlateMechanics;
        let dev = Default::default();
        let plate = ExtrudedPlateMechanics {
            nx: 40,
            ny: 40,
            nz: 4,
            dx: 4.0 / 40.0,
            dy: 4.0 / 40.0,
            dz: 0.1 / 4.0,
        };
        let n = plate.n_nodes();
        let edges = plate.edges_b1::<B>(&dev);
        let dx_f = plate.dx.min(plate.dy).min(plate.dz);
        <B as burn::tensor::backend::Backend>::seed(42);
        let rho_t: Tensor<B, 3> = Tensor::random(
            Shape::new([1, n, 1]),
            burn::tensor::Distribution::Uniform(0.05, 0.95),
            &dev,
        );
        let f = HelmholtzFilter::new((2.0 * dx_f).max(1e-6), 240, 1e-5);
        let out = f.apply(rho_t, edges, dx_f)?;
        let v = out.into_data().value;
        let mx = tensor_max(&v);
        let nan = v.iter().filter(|x| !x.is_finite()).count();
        assert!(
            nan == 0 && mx.is_finite(),
            "striatus helm: nan_count={nan} max={mx}"
        );
        Ok(())
    }

    #[test]
    fn stationary_chain_nonzero_for_delta() -> Result<(), PhysicsError> {
        let dev = Default::default();
        let n = 16usize;
        let mut rho = vec![0.0f32; n];
        rho[8] = 1.0;
        let rho_t: Tensor<B, 3> = Tensor::from_data(Data::new(rho, Shape::new([1, n, 1])), &dev);
        let mut e = Vec::with_capacity((n - 1) * 2);
        for i in 0..(n - 1) {
            e.push(i as i64);
        }
        for i in 0..(n - 1) {
            e.push((i + 1) as i64);
        }
        let edges: Tensor<B, 2, burn::tensor::Int> = Tensor::<B, 1>::from_data(
            Data::new(e.iter().map(|&x| x as f32).collect(), Shape::new([e.len()])),
            &dev,
        )
        .reshape([2, n - 1])
        .int();
        let f = HelmholtzFilter::new(1.5, 12_000, 1e-6);
        let out = f.apply(rho_t, edges, 1.0)?;
        let v = out.into_data().value;
        let mx = tensor_max(&v);
        assert!(
            mx.is_finite() && mx > 0.02 && mx <= 1.2,
            "filtered peak out of band: {mx}"
        );
        Ok(())
    }

    #[test]
    fn apply_rejects_non_positive_dx() {
        let dev = Default::default();
        let rho = Tensor::<B, 3>::full([1, 2, 1], 0.5, &dev);
        let edges: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev);
        let f = HelmholtzFilter::new(1.0, 10, 1e-6);
        assert!(matches!(
            f.apply(rho, edges, 0.0).unwrap_err(),
            PhysicsError::Domain { .. }
        ));
    }

    #[test]
    fn apply_rejects_non_positive_radius() {
        let dev = Default::default();
        let rho = Tensor::<B, 3>::full([1, 2, 1], 0.5, &dev);
        let edges: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev);
        let f = HelmholtzFilter::new(0.0, 10, 1e-6);
        assert!(matches!(
            f.apply(rho, edges, 1.0).unwrap_err(),
            PhysicsError::Domain { .. }
        ));
    }

    #[test]
    fn apply_rejects_edges_not_two_rows() {
        let dev = Default::default();
        let rho = Tensor::<B, 3>::full([1, 2, 1], 0.5, &dev);
        let edges: Tensor<B, 2, burn::tensor::Int> = Tensor::from_data(
            Data::new(vec![0_i64, 1_i64, 1_i64], Shape::new([3, 1])),
            &dev,
        );
        let f = HelmholtzFilter::new(1.0, 10, 1e-6);
        assert!(matches!(
            f.apply(rho, edges, 1.0).unwrap_err(),
            PhysicsError::ShapeMismatch { .. }
        ));
    }

    #[test]
    fn apply_rejects_zero_edges_with_multiple_nodes() {
        let dev = Default::default();
        let rho = Tensor::<B, 3>::full([1, 3, 1], 0.5, &dev);
        let edges: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(Vec::<i64>::new(), Shape::new([2, 0])), &dev);
        let f = HelmholtzFilter::new(1.0, 10, 1e-6);
        assert!(matches!(
            f.apply(rho, edges, 1.0).unwrap_err(),
            PhysicsError::Domain { .. }
        ));
    }
}

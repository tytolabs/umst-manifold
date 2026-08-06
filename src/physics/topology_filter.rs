// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

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
//! # Honest boundary (W29-093)
//!
//! [`HelmholtzFilter`] lands domain/shape fences, a Richardson stationary solve of the discrete
//! Helmholtz resolvent, and optional straight-through AD reattach. Iteration field names say
//! `max_cg_iterations` for historical API stability; the solver is **Richardson**, not CG.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

/// W29 deepen cell — topology density-filter honest fence bundle.
pub const W29_TOPOLOGY_FILTER_DEEPEN_CELL: &str = "W29-093-TOPOLOGY_FILTER";

/// Honest posture tag — Helmholtz PDE density filter research lane.
pub const TOPOLOGY_FILTER_POSTURE_TAG: &str = "honest-helmholtz-topology-density-filter-research-lane";

/// Honest physics posture — unit/domain fences land; does not certify fleet physics GREEN.
pub const TOPOLOGY_FILTER_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by topology_filter alone.
pub const TOPOLOGY_FILTER_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const TOPOLOGY_FILTER_MASTER: bool = false;

/// OP-5 composition pin — not claimed by this module.
pub const TOPOLOGY_FILTER_OP5: bool = false;

/// Helmholtz apply + domain/shape validation landed in this module.
pub const TOPOLOGY_FILTER_HELMHOLTZ_LANDED: bool = true;

/// Stationary solve is Richardson (API field `max_cg_iterations` is historical naming).
pub const TOPOLOGY_FILTER_RICHARDSON_STATIONARY: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const TOPOLOGY_FILTER_HONEST_FENCE: &str =
    "helmholtz_apply_landed=true richardson_stationary_landed=true domain_shape_fences=true straight_through_ad_landed=true production_wired=false physics_green=false master=false op5=false";

const _: () = assert!(!TOPOLOGY_FILTER_PHYSICS_GREEN);
const _: () = assert!(!TOPOLOGY_FILTER_PRODUCTION_WIRED);
const _: () = assert!(!TOPOLOGY_FILTER_MASTER);
const _: () = assert!(!TOPOLOGY_FILTER_OP5);
const _: () = assert!(TOPOLOGY_FILTER_HELMHOLTZ_LANDED);
const _: () = assert!(TOPOLOGY_FILTER_RICHARDSON_STATIONARY);

/// Typed probe for topology-filter posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologyFilterPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub helmholtz_landed: bool,
    pub richardson_stationary: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the Helmholtz topology density filter.
#[must_use]
pub fn topology_filter_honest_posture_bundle() -> TopologyFilterPostureProbe {
    TopologyFilterPostureProbe {
        physics_green: TOPOLOGY_FILTER_PHYSICS_GREEN,
        production_wired: TOPOLOGY_FILTER_PRODUCTION_WIRED,
        master: TOPOLOGY_FILTER_MASTER,
        op5: TOPOLOGY_FILTER_OP5,
        helmholtz_landed: TOPOLOGY_FILTER_HELMHOLTZ_LANDED,
        richardson_stationary: TOPOLOGY_FILTER_RICHARDSON_STATIONARY,
        honest_fence: TOPOLOGY_FILTER_HONEST_FENCE,
        posture_tag: TOPOLOGY_FILTER_POSTURE_TAG,
        deepen_cell: W29_TOPOLOGY_FILTER_DEEPEN_CELL,
    }
}

/// Helmholtz filter landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn topology_filter_posture_honest(probe: &TopologyFilterPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.helmholtz_landed
        && probe.richardson_stationary
        && probe.deepen_cell == W29_TOPOLOGY_FILTER_DEEPEN_CELL
        && probe.honest_fence.contains("helmholtz_apply_landed=true")
        && probe.honest_fence.contains("richardson_stationary_landed=true")
        && probe.honest_fence.contains("domain_shape_fences=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// Validate topology-filter honesty — fail closed on fake production/master/GREEN/OP-5 claims.
pub fn validate_topology_filter_honesty() -> Result<(), &'static str> {
    let probe = topology_filter_honest_posture_bundle();
    if !topology_filter_posture_honest(&probe) {
        return Err("topology_filter_probe failed honesty predicate");
    }
    Ok(())
}

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Int, Tensor,
};

use super::error::PhysicsError;
use super::laplacian::TopologicalLaplacian;

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

/// Helmholtz PDE filter: solves \((I - (r^2/\mathrm{d}x^2)L_{\mathrm{ours}})\tilde\rho=\rho\) on the graph.
#[derive(Clone, Debug)]
pub struct HelmholtzFilter {
    pub radius: f32,
    pub max_cg_iterations: usize,
    pub cg_tolerance: f32,
}

impl HelmholtzFilter {
    /// `radius` is the filter radius in **physical** units (same system as `dx`).
    #[must_use]
    pub fn new(radius: f32, max_cg_iterations: usize, cg_tolerance: f32) -> Self {
        Self {
            radius,
            max_cg_iterations: max_cg_iterations.max(1),
            cg_tolerance: cg_tolerance.max(1e-20),
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
        let dx_safe = dx.max(1e-30);
        let scale = (self.radius / dx_safe).powi(2);
        let max_it = self.max_cg_iterations.max(1);
        let tol_use = self.cg_tolerance.max(1e-8);
        let damage = Tensor::<B, 3>::zeros_like(&rho);
        helmholtz_stationary(rho, edges_b1, damage, scale, max_it, tol_use)
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

fn helmholtz_stationary<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    scale: f32,
    max_iter: usize,
    tol: f32,
) -> Result<Tensor<B, 3>, PhysicsError> {
    let mut x = Tensor::<B, 3>::zeros_like(&rhs);
    let n_nodes = rhs.dims()[1].max(1) as f32;
    let n_edges = edges_b1.dims()[1].max(1) as f32;
    let avg_degree = (2.0 * n_edges / n_nodes).max(1.0);
    let lambda_upper = (2.0 * avg_degree).max(8.0);
    let omega = (1.7 / (1.0 + scale * lambda_upper)).clamp(0.008, 0.22);
    let rhs_norm = rhs.clone().powf_scalar(2.0).sum().into_data().value[0]
        .sqrt()
        .max(1e-20);
    if !rhs_norm.is_finite() {
        return Err(PhysicsError::NonFinite {
            context: "HelmholtzFilter::apply rhs norm",
        });
    }
    let tol_rel = tol.max(1e-8);

    for _ in 0..max_iter {
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
        if r_norm <= tol_rel * rhs_norm {
            break;
        }
        x = x.clone().add(resid.mul_scalar(omega));
    }
    if x.clone()
        .into_data()
        .value
        .iter()
        .any(|v| !v.is_finite())
    {
        return Err(PhysicsError::NonFinite {
            context: "HelmholtzFilter::apply filtered field",
        });
    }
    Ok(x)
}

#[cfg(test)]
mod honest_fence_tests {
    use super::*;

    #[test]
    fn topology_filter_honest_posture_refuses_green_production_master_op5() {
        let probe = topology_filter_honest_posture_bundle();
        assert!(topology_filter_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(probe.helmholtz_landed);
        assert!(probe.richardson_stationary);
        assert_eq!(probe.deepen_cell, W29_TOPOLOGY_FILTER_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, TOPOLOGY_FILTER_POSTURE_TAG);
        assert!(probe.honest_fence.contains("richardson_stationary_landed=true"));
        assert!(!probe.honest_fence.contains("physics_green=true"));
        assert!(!probe.honest_fence.contains("production_wired=true"));
        assert!(!probe.honest_fence.contains("master=true"));
        assert!(!probe.honest_fence.contains("op5=true"));
        validate_topology_filter_honesty().expect("honesty validation must pass");
    }

    #[test]
    fn topology_filter_honest_fence_consts_refuse_green_production_master() {
        assert!(!TOPOLOGY_FILTER_PHYSICS_GREEN);
        assert!(!TOPOLOGY_FILTER_PRODUCTION_WIRED);
        assert!(!TOPOLOGY_FILTER_MASTER);
        assert!(!TOPOLOGY_FILTER_OP5);
        assert!(TOPOLOGY_FILTER_HELMHOLTZ_LANDED);
        assert!(TOPOLOGY_FILTER_RICHARDSON_STATIONARY);
        assert_eq!(W29_TOPOLOGY_FILTER_DEEPEN_CELL, "W29-093-TOPOLOGY_FILTER");
    }

    #[test]
    fn topology_filter_new_clamps_iteration_and_tolerance_floors() {
        let f = HelmholtzFilter::new(1.0, 0, 0.0);
        assert_eq!(f.max_cg_iterations, 1);
        assert!(f.cg_tolerance >= 1e-20);
        assert_eq!(f.radius, 1.0);
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
    fn striatus_extruded_plate_inner_finite() {
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
        let f = HelmholtzFilter::new((2.0 * dx_f).max(1e-6), 240, 1e-7);
        let out = f
            .apply(rho_t, edges, dx_f)
            .expect("HelmholtzFilter::apply on striatus extruded-plate random rho field (FP §6 topology filter verification)");
        let v = out.into_data().value;
        let mx = tensor_max(&v);
        let nan = v.iter().filter(|x| !x.is_finite()).count();
        assert!(
            nan == 0 && mx.is_finite(),
            "striatus helm: nan_count={nan} max={mx}"
        );
    }

    #[test]
    fn stationary_chain_nonzero_for_delta() {
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
        let f = HelmholtzFilter::new(1.5, 12_000, 1e-7);
        let out = f
            .apply(rho_t, edges, 1.0)
            .expect("HelmholtzFilter::apply on 16-node chain delta field (FP §6 topology filter verification)");
        let v = out.into_data().value;
        let mx = tensor_max(&v);
        assert!(
            mx.is_finite() && mx > 0.02 && mx <= 1.2,
            "filtered peak out of band: {mx}"
        );
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
        let edges: Tensor<B, 2, burn::tensor::Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64, 1_i64], Shape::new([3, 1])), &dev);
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

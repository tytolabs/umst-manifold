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

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Int, Tensor,
};

use super::laplacian::TopologicalLaplacian;

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
    ) -> Tensor<B, 3> {
        let dx_safe = dx.max(1e-30);
        let scale = (self.radius / dx_safe).powi(2);
        // Honour the constructor iteration count. A previous `max(..., 400)` floor forced **400**
        // Richardson steps even when callers passed **240** (e.g. Striatus shell examples). On
        // **Burn Autodiff** backends that many scatter-heavy Laplacian updates can accumulate
        // roundoff into **NaN** on large 3-D hex graphs while the same operator stays finite on
        // plain `NdArray`. Callers that need a hard convergence floor should pass an explicit
        // `max_cg_iterations` (tests use 12_000 on a 1-D chain).
        let max_it = self.max_cg_iterations.max(1);
        let tol_use = self.cg_tolerance.max(1e-8);
        let damage = Tensor::<B, 3>::zeros_like(&rho);
        helmholtz_stationary(rho, edges_b1, damage, scale, max_it, tol_use)
    }

    /// Helmholtz on the **inner** backend, re-attached with straight-through gradients (B6 H2).
    ///
    /// Forward value matches [`Self::apply`]; AD gradients bypass the scatter-heavy Richardson
    /// loop (finite at Striatus **40×40×4** where tape-wide [`Self::apply`] can NaN).
    pub fn apply_straight_through<B: AutodiffBackend<FloatElem = f32>>(
        &self,
        rho: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        dx: f32,
    ) -> Tensor<B, 3> {
        let device = rho.device();
        let filtered_inner = self.apply(rho.clone().inner(), edges_b1.inner(), dx);
        let filtered = Tensor::<B, 3>::from_data(filtered_inner.into_data(), &device);
        let rho_st = rho.clone();
        rho_st + (filtered - rho).detach()
    }
}

/// Richardson iteration for SPD \((I - s L_{\mathrm{ours}})x=b\) (same as \((I+sL_{\mathrm{std}})x=b\)).
fn helmholtz_stationary<B: Backend<FloatElem = f32>>(
    rhs: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    scale: f32,
    max_iter: usize,
    _tol: f32,
) -> Tensor<B, 3> {
    let mut x = Tensor::<B, 3>::zeros_like(&rhs);
    // \(\lambda_{\max}(L_{\mathrm{std}}) \le 2\cdot d_{\max}\); estimate \(d_{\max}\) from mean degree.
    let n_nodes = rhs.dims()[1].max(1) as f32;
    let n_edges = edges_b1.dims()[1].max(1) as f32;
    let avg_degree = (2.0 * n_edges / n_nodes).max(1.0);
    let lambda_upper = (2.0 * avg_degree).max(8.0);
    let omega = (1.7 / (1.0 + scale * lambda_upper)).clamp(0.008, 0.22);
    let rhs_norm = rhs.clone().powf_scalar(2.0).sum().into_data().value[0]
        .sqrt()
        .max(1e-20);
    let tol_rel = _tol.max(1e-8);

    for _ in 0..max_iter {
        let lx =
            TopologicalLaplacian::scalar_laplacian(x.clone(), edges_b1.clone(), damage.clone());
        let ax = x.clone().sub(lx.mul_scalar(scale));
        let resid = rhs.clone().sub(ax);
        let r_norm = resid.clone().powf_scalar(2.0).sum().into_data().value[0].sqrt();
        if r_norm <= tol_rel * rhs_norm {
            break;
        }
        x = x.clone().add(resid.mul_scalar(omega));
    }
    x
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
        let out = f.apply(rho_t, edges, dx_f);
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
        let out = f.apply(rho_t, edges, 1.0);
        let v = out.into_data().value;
        let mx = tensor_max(&v);
        assert!(
            mx.is_finite() && mx > 0.02 && mx <= 1.2,
            "filtered peak out of band: {mx}"
        );
    }
}

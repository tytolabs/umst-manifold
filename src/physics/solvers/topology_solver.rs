// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Density evolution on the primal 1-skeleton (“sheaf-carried” scalar field).
//!
//! [`TopologySolver`] holds nodal pseudo-density \(\rho\) (`[B, N, 1]`) and advances it with an
//! explicit diffusion step built from [`crate::physics::laplacian::TopologicalLaplacian::scalar_laplacian`]
//! — the same DEC gather/scatter pattern as mechanics and THMC transport.
//!
//! **Relation to [`crate::ai::topology::TopologyOptimizer`]**: the optimizer runs Neural-SIMP training
//! forwards (density net \(\rightarrow\) SIMP modulus \(\rightarrow\) equilibrium) and does not own a
//! persistent \(\rho\) buffer. Use `TopologySolver::set_rho_from_optimizer` (behind
//! **`topology-density-evolution`**, also enabled via **`solver-experimental`** /
//! **`solver-tests`**) to copy network output into this physics-side state, then diffuse /
//! mask-filter \(\rho\) without duplicating the training loop.
//!
//! Re-exports: [`DensityNet`], [`TopologyOptimizer`], [`TopologyOptimizerStub`] for callers that
//! previously imported them only from this module.

pub use crate::ai::topology::{DensityNet, TopologyOptimizer, TopologyOptimizerStub};

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::physics::laplacian::TopologicalLaplacian;

/// Bounds for \(\rho\) after each step (SIMP-style \((0,1)\) interval).
#[derive(Clone, Debug)]
pub struct TopologySolverConfig {
    pub rho_min: f32,
    pub rho_max: f32,
}

impl Default for TopologySolverConfig {
    fn default() -> Self {
        Self {
            rho_min: 1e-6,
            rho_max: 1.0 - 1e-6,
        }
    }
}

/// Physics-side carrier for nodal density with optional filter hooks on each step.
#[derive(Clone, Debug)]
pub struct TopologySolver<B: Backend> {
    /// Pseudo-density \(\rho\), shape `[B, N, 1]`.
    pub rho: Tensor<B, 3>,
    pub config: TopologySolverConfig,
}

impl<B: Backend<FloatElem = f32>> TopologySolver<B> {
    /// New solver with initial \(\rho\) and clamp bounds from `config`.
    pub fn new(rho: Tensor<B, 3>, config: TopologySolverConfig) -> Self {
        Self { rho, config }
    }

    /// Scalar mask `≥ 0` where **both** policy may edit density (cf. [`crate::core::tensors::UnifiedMaterialStateTensor::policy_editable_mask`])
    /// **and** mechanics BCs do not fix every translational DOF on that node (product of the three
    /// `boundary_mask` channels; shape `[B, N, 3]`, `1` = free).
    ///
    /// Result shape `[B, N, 1]`, suitable for blending with [`TopologySolver::blend_masked_update`].
    pub fn combined_edit_mask(
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
    ) -> Tensor<B, 3> {
        let [b, n, three] = boundary_mask.dims();
        debug_assert_eq!(three, 3);
        let [n_pol, one] = policy_editable_mask.dims();
        debug_assert_eq!(one, 1);
        debug_assert_eq!(
            n_pol, n,
            "combined_edit_mask: policy_editable_mask rows N must match nodal N"
        );
        let mx = boundary_mask.clone().slice([0..b, 0..n, 0..1]);
        let my = boundary_mask.clone().slice([0..b, 0..n, 1..2]);
        let mz = boundary_mask.slice([0..b, 0..n, 2..3]);
        let bc_scalar = mx.mul(my).mul(mz);
        let pol = policy_editable_mask.reshape([1, n, 1]).expand([b, n, 1]);
        bc_scalar.mul(pol)
    }

    /// Blend `proposed` toward `current` where `mask` is low: `out = proposed * mask + current * (1 - mask)`.
    #[inline]
    pub fn blend_masked_update(
        current: Tensor<B, 3>,
        proposed: Tensor<B, 3>,
        mask: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let one = Tensor::<B, 3>::ones_like(&mask);
        proposed.mul(mask.clone()).add(current.mul(one.sub(mask)))
    }

    /// One explicit diffusion step: \(\rho \leftarrow \mathrm{clamp}(\rho + \Delta t\,\Delta\rho)\) then
    /// mask projection using [`Self::combined_edit_mask`].
    pub fn step_density_diffusion(
        &mut self,
        dt: f32,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
    ) {
        self.step_density_diffusion_filtered(
            dt,
            edges_b1,
            damage,
            boundary_mask,
            policy_editable_mask,
            |t| t,
            |t| t,
        )
    }

    /// Same as [`Self::step_density_diffusion`] with optional **pre-Laplacian** and **post-clamp**
    /// hooks (e.g. sensitivity filter surrogates or projection onto a feasible set).
    ///
    /// `pre_filter` only wraps the tensor fed into [`TopologicalLaplacian::scalar_laplacian`]; the
    /// explicit Euler increment still adds \(\Delta t\,\Delta\rho\) to the previous `rho`. `post_filter`
    /// runs after clamping and before the mask blend.
    #[allow(clippy::too_many_arguments)]
    pub fn step_density_diffusion_filtered<F, G>(
        &mut self,
        dt: f32,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        policy_editable_mask: Tensor<B, 2>,
        pre_filter: F,
        post_filter: G,
    ) where
        F: Fn(Tensor<B, 3>) -> Tensor<B, 3>,
        G: Fn(Tensor<B, 3>) -> Tensor<B, 3>,
    {
        let rho_old = self.rho.clone();
        let rho_work = pre_filter(rho_old.clone());
        let lap = TopologicalLaplacian::scalar_laplacian(rho_work, edges_b1, damage);
        let proposed = rho_old.clone().add(lap.mul_scalar(dt));
        let clamped = proposed.clamp(self.config.rho_min, self.config.rho_max);
        let filtered = post_filter(clamped);
        let m = Self::combined_edit_mask(boundary_mask, policy_editable_mask);
        self.rho = Self::blend_masked_update(rho_old, filtered, m);
    }
}

#[cfg(feature = "topology-density-evolution")]
pub fn pre_filter_prime_spectral<B: Backend<FloatElem = f32>>(
    ps: &crate::physics::prime_spectral_filter::PrimeSpectralFilter,
    rho: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let n = rho.dims()[1];
    ps.apply(rho, n)
}

#[cfg(feature = "topology-density-evolution")]
impl<B: Backend<FloatElem = f32>> TopologySolver<B> {
    /// Replace \(\rho\) with [`DensityNet::forward_batched`] on `coords_bn3` (`[B, N, 3]`).
    pub fn set_rho_from_density_net(
        &mut self,
        density_net: &DensityNet<B>,
        coords_bn3: Tensor<B, 3>,
    ) {
        self.rho = density_net.forward_batched(coords_bn3);
    }

    /// Replace \(\rho\) with [`TopologyOptimizer::pseudo_density_at_coords`].
    pub fn set_rho_from_optimizer(&mut self, opt: &TopologyOptimizer<B>, coords_bn3: Tensor<B, 3>) {
        self.rho = opt.pseudo_density_at_coords(coords_bn3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Int, Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    fn two_node_edge_topology() -> Tensor<B, 2, Int> {
        let dev = Default::default();
        Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev)
    }

    #[test]
    fn uniform_rho_unchanged_on_ring() {
        let dev = Default::default();
        let n = 4_usize;
        let mut edges = Vec::with_capacity(n * 2);
        for e in 0..n {
            edges.push(e as i64);
            edges.push(((e + 1) % n) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n])), &dev);

        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);

        solver.step_density_diffusion(0.2, edges_b1, damage, boundary_mask, policy);
        let expected = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        assert!(
            solver
                .rho
                .clone()
                .all_close(expected, Some(1e-5), Some(1e-6)),
            "uniform rho should be harmonic / stationary"
        );
    }

    #[test]
    fn two_bar_mixes_toward_equilibrium() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho_data = vec![1.0_f32, 0.0_f32];
        let rho = Tensor::from_data(Data::new(rho_data, Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);

        solver.step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy);
        let expected = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        assert!(
            solver
                .rho
                .clone()
                .all_close(expected, Some(1e-4), Some(1e-5)),
            "expected ~0.5 equilibrium on two-node bar"
        );
    }

    #[test]
    fn policy_mask_freezes_masked_nodes() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho_data = vec![1.0_f32, 0.0_f32];
        let rho = Tensor::from_data(Data::new(rho_data, Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let mut pol = vec![0.0_f32; n];
        pol[1] = 1.0;
        let policy = Tensor::from_data(Data::new(pol, Shape::new([n, 1])), &dev);

        solver.step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy);
        let rho = solver.rho.clone();
        let n0 = rho.clone().slice([0..1, 0..1, 0..1]);
        let n1 = rho.slice([0..1, 1..2, 0..1]);
        let one = Tensor::<B, 3>::ones([1, 1, 1], &dev);
        assert!(
            n0.all_close(
                Tensor::<B, 3>::full([1, 1, 1], 1.0, &dev),
                Some(1e-5),
                Some(1e-6)
            ),
            "node 0 should stay fixed by policy mask"
        );
        assert!(
            n1.clone()
                .greater_elem(0.0_f32)
                .float()
                .all_close(one.clone(), Some(0.0), Some(0.0)),
            "node 1 should evolve (rho > 0)"
        );
        assert!(
            n1.lower_elem(1.0_f32)
                .float()
                .all_close(one, Some(0.0), Some(0.0)),
            "node 1 should evolve (rho < 1)"
        );
    }

    #[test]
    fn boundary_mask_freezes_fully_fixed_node() {
        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho_data = vec![1.0_f32, 0.3_f32];
        let rho = Tensor::from_data(Data::new(rho_data, Shape::new([1, n, 1])), &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let mut bm = vec![1.0_f32; n * 3];
        bm[3] = 0.0;
        bm[4] = 0.0;
        bm[5] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);

        solver.step_density_diffusion(0.5, edges_b1, damage, boundary_mask, policy);
        let rho = solver.rho.clone();
        let fixed = rho.clone().slice([0..1, 1..2, 0..1]);
        let free = rho.slice([0..1, 0..1, 0..1]);
        assert!(
            fixed.all_close(
                Tensor::<B, 3>::full([1, 1, 1], 0.3, &dev),
                Some(1e-5),
                Some(1e-6)
            ),
            "fully fixed node should keep rho"
        );
        assert!(
            free.all_close(
                Tensor::<B, 3>::full([1, 1, 1], 0.65, &dev),
                Some(1e-4),
                Some(1e-5)
            ),
            "free node should diffuse toward neighbor"
        );
    }

    #[test]
    fn step_density_diffusion_filtered_invokes_pre_post_hooks() {
        use std::cell::Cell;

        let dev = Default::default();
        let n = 2_usize;
        let edges_b1 = two_node_edge_topology();
        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        let mut solver = TopologySolver::new(rho, TopologySolverConfig::default());
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let boundary_mask = Tensor::<B, 3>::ones([1, n, 3], &dev);
        let policy = Tensor::<B, 2>::ones([n, 1], &dev);

        let pre_calls = Cell::new(0_u32);
        let post_calls = Cell::new(0_u32);
        solver.step_density_diffusion_filtered(
            0.1,
            edges_b1,
            damage,
            boundary_mask,
            policy,
            |t| {
                pre_calls.set(pre_calls.get() + 1);
                t
            },
            |t| {
                post_calls.set(post_calls.get() + 1);
                t
            },
        );
        assert_eq!(pre_calls.get(), 1);
        assert_eq!(post_calls.get(), 1);
        let expected = Tensor::<B, 3>::full([1, n, 1], 0.5, &dev);
        assert!(
            solver
                .rho
                .clone()
                .all_close(expected, Some(1e-4), Some(1e-5)),
            "uniform rho stationary under diffusion"
        );
    }
}

#[cfg(all(test, feature = "topology-density-evolution"))]
mod optimizer_sync_tests {
    use super::*;
    use burn::tensor::{Shape, Tensor};
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn set_rho_from_optimizer_matches_forward_batched() {
        let dev = Default::default();
        let opt = TopologyOptimizer::<B>::new(0.4, 3.0, 8, &dev);
        let coords = Tensor::<B, 3>::zeros(Shape::new([1, 3, 3]), &dev);
        let rho_direct = opt.pseudo_density_at_coords(coords.clone());
        let mut solver = TopologySolver::new(
            Tensor::<B, 3>::zeros([1, 3, 1], &dev),
            TopologySolverConfig::default(),
        );
        solver.set_rho_from_optimizer(&opt, coords);
        assert!(
            rho_direct.all_close(solver.rho.clone(), Some(1e-6), Some(1e-7)),
            "set_rho_from_optimizer must match pseudo_density_at_coords"
        );
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! **Composable solver protocols** — thin traits and ZST namespaces so physics stays FP-pipelined
//! (pure `fn` chains over `Tensor<B, _>`) instead of orchestrator god-objects.
//!
//! ## Architecture (May 2026 UMST)
//! - Manifold solvers are **material-agnostic**: they never import cartridge crates.
//! - Call sites may use the traits for polymorphism; hot paths can call inherent methods directly.
//! - DEC topology and gather/scatter live in [`super::topology`], [`super::dec_primal`], [`super::dec_operators`].

use burn::tensor::{backend::Backend, Int, Tensor};

use crate::core::field::{BodyForceField, BoundaryMaskField, Field, StiffnessField};

use super::error::PhysicsError;
use super::laplacian::TopologicalLaplacian;
use super::mechanics::VectorMechanicsSolver;
use super::time_orchestration::MechanicsInnerLoopConfig;

/// Namespace-style alias for discoverability (no state).
pub struct ScalarTransport;

impl ScalarTransport {
    /// Delegates to [`TopologicalLaplacian::scalar_laplacian`].
    #[inline]
    pub fn laplacian<B: Backend>(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        TopologicalLaplacian::scalar_laplacian(x, edges_b1, damage)
    }
}

/// Namespace-style alias for discoverability (no state).
pub struct MechanicsEquilibrium;

impl MechanicsEquilibrium {
    /// Delegates to [`VectorMechanicsSolver::solve_equilibrium_typed`] (FP XS-3 step 4).
    #[inline]
    #[allow(clippy::too_many_arguments)] // Mirrors typed equilibrium arity.
    pub fn solve<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        let (u, stress) = VectorMechanicsSolver::solve_equilibrium_typed(
            Field::new(displacement),
            coords,
            stiffness,
            BodyForceField::from_tensor(body_force),
            edges_b1,
            Field::new(damage),
            BoundaryMaskField::from_tensor(boundary_mask),
            cross_section_area,
            inner_cfg,
        )?;
        Ok((u.into_tensor(), stress))
    }
}

/// Differentiable scalar transport on the primal graph (diffusion / Poisson-like operators).
///
/// # Contract
/// - `x`: `[B, N, F]`; `damage`: `[B, N, 1]`; output `[B, N, F]`.
/// - With `damage ≡ 0`, the operator has **zero row-sum** per channel (discrete conservation).
pub trait ScalarTransportSolver<B: Backend> {
    fn laplacian(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3>;
}

impl<B: Backend> ScalarTransportSolver<B> for TopologicalLaplacian {
    #[inline]
    fn laplacian(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        Self::scalar_laplacian(x, edges_b1, damage)
    }
}

impl<B: Backend> ScalarTransportSolver<B> for ScalarTransport {
    #[inline]
    fn laplacian(
        x: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        // Inherent `ScalarTransport::laplacian` shares this body; call the SSOT operator
        // directly so the trait method cannot recurse into itself.
        TopologicalLaplacian::scalar_laplacian(x, edges_b1, damage)
    }
}

/// Quasi-static bar-network equilibrium (Phase 1); stress returned as `[B, N, 3, 3]`.
///
/// # Contract
/// Shapes match [`VectorMechanicsSolver::solve_equilibrium_typed`]. Dirichlet DOFs are **masked** in
/// `boundary_mask` (`0` = fixed, `1` = free). **FP XS-3 step 4:** stiffness is [`StiffnessField`].
pub trait MechanicsEquilibriumSolver<B: Backend<FloatElem = f32>> {
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError>;
}

impl<B: Backend<FloatElem = f32>> MechanicsEquilibriumSolver<B> for VectorMechanicsSolver {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        let (u, stress) = Self::solve_equilibrium_typed(
            Field::new(displacement),
            coords,
            stiffness,
            BodyForceField::from_tensor(body_force),
            edges_b1,
            Field::new(damage),
            BoundaryMaskField::from_tensor(boundary_mask),
            cross_section_area,
            inner_cfg,
        )?;
        Ok((u.into_tensor(), stress))
    }
}

impl<B: Backend<FloatElem = f32>> MechanicsEquilibriumSolver<B> for MechanicsEquilibrium {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn solve_equilibrium(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: StiffnessField<B>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> Result<(Tensor<B, 3>, Tensor<B, 4>), PhysicsError> {
        Self::solve(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
            inner_cfg,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn chain_edges(n: usize, device: &NdArrayDevice) -> Tensor<B, 2, Int> {
        let ne = n.saturating_sub(1);
        let mut e = Vec::with_capacity(ne * 2);
        for i in 0..ne {
            e.push(i as i64);
        }
        for i in 0..ne {
            e.push((i + 1) as i64);
        }
        Tensor::from_data(Data::new(e, Shape::new([2, ne])), device)
    }

    #[test]
    fn protocols_scalar_transport_namespace_matches_laplacian_and_trait() {
        let device = NdArrayDevice::Cpu;
        let n = 5usize;
        let x_data: Vec<f32> = (0..n).map(|i| (i as f32 + 1.0) * 0.25).collect();
        let x = Tensor::<B, 1>::from_data(Data::new(x_data, Shape::new([n])), &device)
            .reshape([1, n, 1]);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_edges(n, &device);

        let via_ns = ScalarTransport::laplacian(x.clone(), edges.clone(), damage.clone());
        let via_trait_lap = <TopologicalLaplacian as ScalarTransportSolver<B>>::laplacian(
            x.clone(),
            edges.clone(),
            damage.clone(),
        );
        let via_trait_ns =
            <ScalarTransport as ScalarTransportSolver<B>>::laplacian(x, edges, damage);

        let a = via_ns.into_data().value;
        let b = via_trait_lap.into_data().value;
        let c = via_trait_ns.into_data().value;
        let max_ab = a
            .iter()
            .zip(b.iter())
            .map(|(u, v)| (u - v).abs())
            .fold(0.0_f32, f32::max);
        let max_ac = a
            .iter()
            .zip(c.iter())
            .map(|(u, v)| (u - v).abs())
            .fold(0.0_f32, f32::max);
        assert!(
            max_ab < 1e-7,
            "namespace vs TopologicalLaplacian trait Δ={max_ab}"
        );
        assert!(
            max_ac < 1e-7,
            "namespace vs ScalarTransport trait Δ={max_ac}"
        );
    }

    #[test]
    fn protocols_scalar_transport_zero_row_sum_when_damage_zero() {
        // Contract: damage ≡ 0 ⇒ discrete conservation (row-sum ≈ 0 per channel).
        let device = NdArrayDevice::Cpu;
        let n = 6usize;
        let x = Tensor::<B, 3>::ones([1, n, 2], &device);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let edges = chain_edges(n, &device);

        let lx = ScalarTransport::laplacian(x, edges, damage);
        let vals = lx.into_data().value;
        let abs_sum: f32 = vals.iter().map(|v| v.abs()).sum();
        assert!(
            abs_sum < 1e-5,
            "zero-damage constant field must have ~0 Laplacian mass; abs_sum={abs_sum}"
        );
    }

    #[test]
    fn protocols_mechanics_equilibrium_namespace_matches_trait_on_zero_load() {
        let device = NdArrayDevice::Cpu;
        let n = 4usize;
        let dx = 0.25_f32;
        let e = 1.0e6_f32;
        let a = 0.01_f32;

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords: Tensor<B, 2> =
            Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &device);
        let edges = chain_edges(n, &device);

        let mut stiff = Vec::with_capacity(n * 2);
        for _ in 0..n {
            stiff.push(e);
            stiff.push(0.3);
        }
        let stiffness = StiffnessField::from_tensor(Tensor::from_data(
            Data::new(stiff, Shape::new([1, n, 2])),
            &device,
        ));
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &device);
        let displacement = Tensor::<B, 3>::zeros([1, n, 3], &device);
        let body_force = Tensor::<B, 3>::zeros([1, n, 3], &device);

        let mut bm = vec![1.0_f32; n * 3];
        for i in 0..n {
            bm[i * 3 + 1] = 0.0;
            bm[i * 3 + 2] = 0.0;
        }
        bm[0] = 0.0; // fix left x
        let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &device);

        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 80,
            cg_tolerance: 1e-6,
            pcg_tolerance: 1e-6,
            use_preconditioner: true,
            max_equilibrium_substeps: 1,
        };

        let (u_ns, s_ns) = MechanicsEquilibrium::solve(
            displacement.clone(),
            coords.clone(),
            stiffness.clone(),
            body_force.clone(),
            edges.clone(),
            damage.clone(),
            boundary_mask.clone(),
            a,
            &cfg,
        )
        .expect("MechanicsEquilibrium::solve");

        let (u_tr, s_tr) =
            <MechanicsEquilibrium as MechanicsEquilibriumSolver<B>>::solve_equilibrium(
                displacement,
                coords,
                stiffness,
                body_force,
                edges,
                damage,
                boundary_mask,
                a,
                &cfg,
            )
            .expect("MechanicsEquilibriumSolver::solve_equilibrium");

        let du = u_ns
            .clone()
            .sub(u_tr)
            .abs()
            .into_data()
            .value
            .into_iter()
            .fold(0.0_f32, f32::max);
        let ds = s_ns
            .sub(s_tr)
            .abs()
            .into_data()
            .value
            .into_iter()
            .fold(0.0_f32, f32::max);
        assert!(du < 1e-7, "namespace vs trait displacement Δ={du}");
        assert!(ds < 1e-7, "namespace vs trait stress Δ={ds}");

        // Zero-load + fixed left → displacement stays ~0 (honest fence, not physics GREEN).
        let u_max = u_ns
            .into_data()
            .value
            .into_iter()
            .fold(0.0_f32, |m, v| m.max(v.abs()));
        assert!(u_max < 1e-5, "zero-load equilibrium |u|_max={u_max}");
    }
}

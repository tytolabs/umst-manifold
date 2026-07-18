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

use super::laplacian::TopologicalLaplacian;
use super::mechanics::VectorMechanicsSolver;
use super::error::PhysicsError;
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

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Single delegation surface for topology-coupled physics stepping.
//!
//! Callers that want a **named pipeline** (transport → chemistry → mechanics → fracture → optional
//! rheology) should use [`TopologyPhysicsOrchestrator::run_plan_step`]. That method forwards to
//! [`crate::physics::solvers::ThmcSolver::step`] and does **not** duplicate its implementation.
//!
//! ## Integration contract (execution order — design intent)
//!
//! The canonical ordering below is what higher-level planners should assume when composing solvers.
//! Today, the concrete sub-steps live inside [`crate::physics::solvers::ThmcSolver`] (see its module
//! docs and `--features solver-experimental` implementation). This module documents the **contract**;
//! evolution of `ThmcSolver` is expected to stay aligned with these phases rather than scattering
//! duplicate loops across the codebase.
//!
//! 1. **Laplacian transport hints** — discrete diffusion / Laplacian-style updates on nodal fields
//!    (thermal, hydrologic proxies) using graph topology and masks (e.g. damage-degraded flux).
//! 2. **Chemistry** — hydration / reaction channels on [`crate::physics::solvers::ChemicalPlan`];
//!    **placeholder** until kinetics are wired; must not silently change conserved quantities without
//!    documenting closures via [`crate::core::traits::IScienceCartridge`].
//! 3. **Mechanics** — equilibrium or pseudo-time step for displacement / stress; requires consistent
//!    embeddings when Euclidean coordinates exist (integer-only manifold indices skip sub-solves until
//!    an embedding map is supplied — see `Thmc` solver docs).
//! 4. **Fracture** — phase-field or damage evolution coupled to strain / energy release proxies on
//!    the same node batch as transport.
//! 5. **Rheology (optional)** — Bingham / flow-like updates ([`crate::physics::solvers::BinghamFlowSolver`])
//!    are **not** folded into [`crate::physics::solvers::ThmcSolver::step`] yet. When pore flow must run in
//!    the same tick, compose **after** `run_plan_step` with explicit velocity/pressure tensors and document
//!    data dependencies in your cartridge pipeline.
//!
//! ## Errors (default builds)
//!
//! [`crate::physics::solvers::ThmcSolver::step`] returns `Err` when `solver-experimental` is disabled.
//! The orchestrator forwards that `Result` so callers can branch without panicking.

use burn::tensor::backend::Backend;

use crate::core::tensors::UnifiedMaterialStateTensor;
use crate::core::traits::IScienceCartridge;
use crate::physics::solvers::{ThmcSolver, ThmcState};

/// Names the topology physics step and holds the sole [`ThmcSolver`] used for coupled advancement.
///
/// Use [`Self::run_plan_step`] as the **one** call site that performs a full plan tick; avoid calling
/// [`ThmcSolver::step`] directly elsewhere if you want a single integration chokepoint for logging,
/// profiling, or future middleware (e.g. validation gates between phases).
pub struct TopologyPhysicsOrchestrator {
    /// Coupled THMC Newton / explicit scaffold controls and tolerances.
    pub thmc: ThmcSolver,
}

impl TopologyPhysicsOrchestrator {
    /// Wrap an existing [`ThmcSolver`] configuration.
    pub fn new(thmc: ThmcSolver) -> Self {
        Self { thmc }
    }

    /// Advance one orchestrated plan step: **delegates** to [`ThmcSolver::step`] only.
    ///
    /// This method intentionally contains no second copy of Laplacian, fracture, or rheology logic.
    /// Refer to this module’s **Integration contract** for the semantic ordering guaranteed by the
    /// solver implementation behind this call.
    ///
    /// When [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`] is `Some` and
    /// `solver-experimental` is enabled, the inner [`ThmcSolver`] consumes it for mechanics edge lengths
    /// (see [`crate::physics::solvers::ThmcSolver::step`]).
    ///
    /// # Errors
    ///
    /// Forwards [`ThmcSolver::step`] errors (including the default-feature `Err` when experimental
    /// coupling is disabled).
    pub fn run_plan_step<B, C>(
        &self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        self.thmc.step(cartridge, state, manifold)
    }

    /// Same as [`Self::run_plan_step`] — alias for planners that prefer an explicit “full integration” name.
    pub fn run_full_integration_step<B, C>(
        &self,
        cartridge: &C,
        state: ThmcState<B>,
        manifold: &UnifiedMaterialStateTensor<B>,
    ) -> Result<ThmcState<B>, String>
    where
        B: Backend<FloatElem = f32>,
        C: IScienceCartridge<B>,
    {
        self.run_plan_step(cartridge, state, manifold)
    }

    /// Borrow the inner solver for tuning `dt` / Newton counts between steps (experimental workflows).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_solver_mut(&mut self) -> &mut ThmcSolver {
        &mut self.thmc
    }

    /// Immutable access to inner solver parameters (experimental workflows).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_solver(&self) -> &ThmcSolver {
        &self.thmc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orchestrator_wraps_solver_config() {
        let o = TopologyPhysicsOrchestrator::new(ThmcSolver {
            dt: 0.01,
            max_newton: 4,
            tol: 1e-4,
        });
        assert!((o.thmc.dt - 0.01).abs() < f32::EPSILON);
        assert_eq!(o.thmc.max_newton, 4);
    }
}

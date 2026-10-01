// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Manifold Track C — shared solve combinator integration (budgeted unfold callers).
//!
//! Termination is energy + problem-derived tolerance, not a compiled `max_iters` ceiling.
//! Physics solver sites adopt these wrappers in follow-on cells; this crate stays honest open.

pub mod cap_migration;
pub mod combinator_posture;
pub mod control_flow_iterate_budget;
pub mod scalar_residual_unfold;

pub use cap_migration::{CapMigrationDisposition, CapMigrationSite, CAP_MIGRATION_SITES};
pub use combinator_posture::{
    manifold_solve_combinator_honest, manifold_solve_combinator_probe,
    ManifoldSolveCombinatorProbe, MANIFOLD_SOLVE_COMBINATOR_HONEST_FENCE,
    MANIFOLD_SOLVE_COMBINATOR_PHYSICS_GREEN,
};
pub use control_flow_iterate_budget::{
    control_flow_iterate_budget, ControlFlowIterateBudget, ControlFlowIterateCertificate,
};
pub use scalar_residual_unfold::scalar_residual_unfold;

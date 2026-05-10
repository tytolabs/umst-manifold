// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Stateful-in-tensors equilibrium and transport solvers.
//!
//! Convention: each solver is a zero-sized type with inherent `solve_*` / future `step_*`
//! methods taking explicit **config** structs (see [`crate::physics::time_orchestration`])
//! and returning updated [`burn::tensor::Tensor`]s only — no hidden buffers — so Burn
//! autodiff sees a pure computational graph.

pub mod acoustics;
pub mod electrochemistry;
pub mod fracture_field;
pub mod photonics;
pub mod rheology_flow;
pub mod statistical_mechanics;
pub mod thmc;
pub mod topology_solver;

pub use crate::physics::mechanics::VectorMechanicsSolver;
pub use acoustics::AcousticWaveSolver;
pub use electrochemistry::ElectroChemicalSolver;
pub use fracture_field::PhaseFieldFractureSolver;
pub use photonics::PhotonicsSolver;
pub use rheology_flow::BinghamFlowSolver;
pub use thmc::{ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcSolver, ThmcState};
pub use topology_solver::{
    DensityNet, TopologyOptimizer, TopologyOptimizerStub, TopologySolver, TopologySolverConfig,
};

/// Type alias: inner CG / equilibrium controls at the mechanics solver boundary.
pub type MechanicsInnerSolveConfig = crate::physics::time_orchestration::MechanicsInnerLoopConfig;

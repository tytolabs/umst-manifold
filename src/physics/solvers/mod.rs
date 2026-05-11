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
/// Johnson–Zollweg–Gubbins (1993) LJ EOS — `f64` reference (teqp-aligned); not the Burn bridge.
pub mod lj_johnson_1993_reference;
pub mod photonics;
pub mod rheology_flow;
pub mod statistical_mechanics;
pub mod thmc;
pub mod thmc_residual;
pub mod topology_solver;

pub use crate::physics::mechanics::VectorMechanicsSolver;
pub use acoustics::AcousticWaveSolver;
#[cfg(feature = "acoustics-newmark")]
pub use acoustics::{AcousticNewmarkBar1dPeriodic, AcousticNewmarkBar1dWork};
#[cfg(feature = "electrochemistry-mvp")]
pub use electrochemistry::pnp_backward_euler_residual_l2_chain_host_f64;
pub use electrochemistry::{ElectroChemicalSolver, NewtonPnpContext};
pub use fracture_field::PhaseFieldFractureSolver;
#[cfg(feature = "fracture-at2")]
pub use fracture_field::{
    spectral_tensile_psi_plus_from_strain, strain_tensor_for_fracture_after_mechanics,
    strain_tensor_from_bar_network_displacement,
};
pub use photonics::{PhotonicsHelmholtzSolver, PhotonicsSolver};
pub use rheology_flow::BinghamFlowSolver;
#[cfg(feature = "thmc-coupled")]
pub use thmc::full_hydration_alpha_rate_tensor;
#[cfg(feature = "thmc-coupled")]
pub use thmc::strain_tensor_for_fracture_from_manifold;
#[cfg(feature = "thmc-coupled")]
pub use thmc::{mc2010_style_notional_shrink_strain, shrink_strain_from_saturation_loss};
pub use thmc::{
    ChemicalPlan, HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcHydrationKinetics,
    ThmcImplicitTAlphaNewtonConfig, ThmcSolver, ThmcState,
};
pub use thmc_residual::{ResidualThmc, ThmcMonolithicImplicitUnknownLayout};
#[cfg(feature = "thmc-coupled")]
pub use thmc_residual::{
    ThmcImplicitEulerThermalHumidityHydrationResidual, ThmcImplicitEulerThermalHydrationResidual,
};
pub use topology_solver::{
    DensityNet, TopologyOptimizer, TopologyOptimizerStub, TopologySolver, TopologySolverConfig,
};

/// Type alias: inner CG / equilibrium controls at the mechanics solver boundary.
pub type MechanicsInnerSolveConfig = crate::physics::time_orchestration::MechanicsInnerLoopConfig;

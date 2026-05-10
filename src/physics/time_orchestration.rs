// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Multi-scale simulation schedules (Refinement #1).
//!
//! Outer chemistry / agent steps use large `dt_chemistry`. Inner mechanical equilibrium uses
//! its own substeps and tolerances (`MechanicsInnerLoopConfig`) so integration is not forced to a
//! single global \(\Delta t\) at the fastest physics scale.

/// Clocks for coupled THMC + fast physics. The cartridge advances `dt_chemistry`; mechanics and
/// future wave solvers sub-step internally.
#[derive(Clone, Debug)]
pub struct SimulationClocks {
    /// Thermo-chemical outer step (seconds); e.g. 1 hour = 3600.
    pub dt_chemistry: f32,
    /// Quasi-static mechanics inner step when marching toward equilibrium.
    pub dt_mechanics_substep: f32,
    /// Hard cap on mechanics substeps per outer chemistry step.
    pub max_mech_substeps_per_chem: u32,
    /// Optional step for electromagnetics / acoustics (nanoseconds).
    pub dt_fast_physics: Option<f32>,
}

impl Default for SimulationClocks {
    fn default() -> Self {
        Self {
            dt_chemistry: 3600.0,
            dt_mechanics_substep: 0.1,
            max_mech_substeps_per_chem: 10_000,
            dt_fast_physics: Some(1e-9),
        }
    }
}

/// Controls for mechanical equilibrium — **decoupled** from `dt_chemistry`.
#[derive(Clone, Debug)]
pub struct MechanicsInnerLoopConfig {
    pub max_cg_iterations: usize,
    pub cg_tolerance: f32,
    /// Reserved when multiple mechanic passes are needed per chem step.
    pub max_equilibrium_substeps: u32,
}

impl Default for MechanicsInnerLoopConfig {
    fn default() -> Self {
        Self {
            max_cg_iterations: 200,
            cg_tolerance: 1e-6,
            max_equilibrium_substeps: 1,
        }
    }
}

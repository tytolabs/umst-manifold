// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Multi-scale simulation schedules (Refinement #1).
//!
//! Outer chemistry / agent steps use large `dt_chemistry`. Inner mechanical equilibrium uses
//! its own substeps and tolerances ([`MechanicsInnerLoopConfig`]) so integration is not forced to a
//! single global \(\Delta t\) at the fastest physics scale.
//!
//! # Honest boundary (W29-091)
//!
//! [`SimulationClocks`] + [`MechanicsInnerLoopConfig`] are the **typed schedule SSOT** for
//! chemistry-vs-mechanics decoupling. Unit contracts: `cargo test -p umst-manifold time_orchestration`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5. Fast-physics (`dt_fast_physics`)
//! remains optional and does not imply EM/acoustics production wiring.

/// W29 deepen cell — multi-scale time orchestration honest fence bundle.
pub const W29_TIME_ORCHESTRATION_DEEPEN_CELL: &str = "W29-091-TIME_ORCHESTRATION";

/// Honest posture tag — schedule SSOT landed; fleet production wiring refused.
pub const TIME_ORCHESTRATION_POSTURE_TAG: &str = "honest-time-orchestration-research-lane";

/// Honest physics posture — schedule unit contracts pass; does not certify fleet physics GREEN.
pub const TIME_ORCHESTRATION_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the schedule surface alone.
pub const TIME_ORCHESTRATION_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const TIME_ORCHESTRATION_MASTER: bool = false;

/// Whether chemistry/mechanics clock decoupling is landed as typed SSOT.
pub const TIME_ORCHESTRATION_CLOCKS_LANDED: bool = true;

/// Whether optional fast-physics dt implies EM/acoustics production wiring (honestly deferred).
pub const TIME_ORCHESTRATION_FAST_PHYSICS_PRODUCTION_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const TIME_ORCHESTRATION_HONEST_FENCE: &str =
    "clocks_landed=true mechanics_inner_loop_decoupled=true fast_physics_production_wired=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!TIME_ORCHESTRATION_PRODUCTION_WIRED);
const _: () = assert!(!TIME_ORCHESTRATION_PHYSICS_GREEN);
const _: () = assert!(!TIME_ORCHESTRATION_MASTER);
const _: () = assert!(!TIME_ORCHESTRATION_FAST_PHYSICS_PRODUCTION_WIRED);
const _: () = assert!(TIME_ORCHESTRATION_CLOCKS_LANDED);

/// Typed probe for time-orchestration posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOrchestrationPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub clocks_landed: bool,
    pub fast_physics_production_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for multi-scale time orchestration.
#[must_use]
pub fn time_orchestration_honest_posture_bundle() -> TimeOrchestrationPostureProbe {
    TimeOrchestrationPostureProbe {
        physics_green: TIME_ORCHESTRATION_PHYSICS_GREEN,
        production_wired: TIME_ORCHESTRATION_PRODUCTION_WIRED,
        master: TIME_ORCHESTRATION_MASTER,
        clocks_landed: TIME_ORCHESTRATION_CLOCKS_LANDED,
        fast_physics_production_wired: TIME_ORCHESTRATION_FAST_PHYSICS_PRODUCTION_WIRED,
        honest_fence: TIME_ORCHESTRATION_HONEST_FENCE,
        posture_tag: TIME_ORCHESTRATION_POSTURE_TAG,
        deepen_cell: W29_TIME_ORCHESTRATION_DEEPEN_CELL,
    }
}

/// Schedule SSOT landed with production/master/GREEN composition honestly open.
#[must_use]
pub fn time_orchestration_posture_honest(probe: &TimeOrchestrationPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.clocks_landed
        && !probe.fast_physics_production_wired
        && probe.honest_fence.contains("clocks_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the time-orchestration surface.
#[must_use]
pub fn time_orchestration_refuse_overclaim(
    probe: &TimeOrchestrationPostureProbe,
) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("TIME_ORCHESTRATION_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err(
            "TIME_ORCHESTRATION_PRODUCTION_WIRED must stay false until embodied loop closes",
        );
    }
    if probe.master {
        return Err(
            "TIME_ORCHESTRATION_MASTER must stay false — not claimed by schedule SSOT alone",
        );
    }
    if probe.fast_physics_production_wired {
        return Err("fast_physics dt must not imply EM/acoustics production wiring");
    }
    if !time_orchestration_posture_honest(probe) {
        return Err("time_orchestration posture fence inconsistent");
    }
    Ok(())
}

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

/// Why a [`SimulationClocks`] schedule is inadmissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockValidationError {
    NonPositiveChemistryDt,
    NonPositiveMechanicsSubstep,
    ZeroMaxMechSubsteps,
    NonPositiveFastPhysicsDt,
}

impl SimulationClocks {
    /// Fail-closed positivity / finiteness fence for multi-scale schedules.
    #[must_use]
    pub fn validate(&self) -> Result<(), ClockValidationError> {
        if !(self.dt_chemistry.is_finite() && self.dt_chemistry > 0.0) {
            return Err(ClockValidationError::NonPositiveChemistryDt);
        }
        if !(self.dt_mechanics_substep.is_finite() && self.dt_mechanics_substep > 0.0) {
            return Err(ClockValidationError::NonPositiveMechanicsSubstep);
        }
        if self.max_mech_substeps_per_chem == 0 {
            return Err(ClockValidationError::ZeroMaxMechSubsteps);
        }
        if let Some(dt_fast) = self.dt_fast_physics {
            if !(dt_fast.is_finite() && dt_fast > 0.0) {
                return Err(ClockValidationError::NonPositiveFastPhysicsDt);
            }
        }
        Ok(())
    }

    /// Ideal substep count \(\lceil dt_{chem}/dt_{mech}\rceil\), uncapped.
    ///
    /// Returns `None` when clocks fail [`Self::validate`].
    #[must_use]
    pub fn ideal_mech_substeps_per_chem(&self) -> Option<u32> {
        self.validate().ok()?;
        let ratio = (self.dt_chemistry / self.dt_mechanics_substep).ceil();
        if !ratio.is_finite() || ratio <= 0.0 {
            return None;
        }
        // Cap at u32::MAX before cast; schedule caps apply separately.
        let ideal = ratio.min(u32::MAX as f32) as u32;
        Some(ideal.max(1))
    }

    /// Mechanics substeps actually taken per chemistry step (ideal ∩ hard cap).
    #[must_use]
    pub fn mech_substeps_per_chem(&self) -> Option<u32> {
        let ideal = self.ideal_mech_substeps_per_chem()?;
        Some(ideal.min(self.max_mech_substeps_per_chem).max(1))
    }

    /// Whether the hard cap truncates the ideal chemistry→mechanics ratio.
    #[must_use]
    pub fn mech_substep_cap_binds(&self) -> Option<bool> {
        let ideal = self.ideal_mech_substeps_per_chem()?;
        Some(ideal > self.max_mech_substeps_per_chem)
    }
}

/// Controls for mechanical equilibrium — **decoupled** from `dt_chemistry`.
#[derive(Clone, Debug)]
pub struct MechanicsInnerLoopConfig {
    pub max_cg_iterations: usize,
    pub cg_tolerance: f32,
    /// Same scale as [`Self::cg_tolerance`] for preconditioned residual reporting (alias for tuning PCG).
    pub pcg_tolerance: f32,
    /// Apply diagonal (Jacobi) preconditioning in the projected CG loop (`z = M⁻¹ r`).
    pub use_preconditioner: bool,
    /// Reserved when multiple mechanic passes are needed per chem step.
    pub max_equilibrium_substeps: u32,
}

impl Default for MechanicsInnerLoopConfig {
    fn default() -> Self {
        Self {
            max_cg_iterations: 200,
            cg_tolerance: 1e-6,
            pcg_tolerance: 1e-6,
            use_preconditioner: true,
            max_equilibrium_substeps: 1,
        }
    }
}

impl MechanicsInnerLoopConfig {
    /// Fail-closed positivity fence for CG / equilibrium knobs.
    #[must_use]
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_cg_iterations == 0 {
            return Err("max_cg_iterations must be ≥ 1");
        }
        if !(self.cg_tolerance.is_finite() && self.cg_tolerance > 0.0) {
            return Err("cg_tolerance must be finite and > 0");
        }
        if !(self.pcg_tolerance.is_finite() && self.pcg_tolerance > 0.0) {
            return Err("pcg_tolerance must be finite and > 0");
        }
        if self.max_equilibrium_substeps == 0 {
            return Err("max_equilibrium_substeps must be ≥ 1");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_orchestration_honest_posture_refuses_green_and_production() {
        let probe = time_orchestration_honest_posture_bundle();
        assert!(time_orchestration_posture_honest(&probe));
        assert!(time_orchestration_refuse_overclaim(&probe).is_ok());
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.fast_physics_production_wired);
        assert!(probe.clocks_landed);
        assert_eq!(probe.deepen_cell, W29_TIME_ORCHESTRATION_DEEPEN_CELL);
        assert!(probe
            .honest_fence
            .contains("mechanics_inner_loop_decoupled=true"));
        assert!(probe.honest_fence.contains("physics_green=false"));
    }

    #[test]
    fn time_orchestration_default_clocks_validate_and_cap() {
        let clocks = SimulationClocks::default();
        assert!(clocks.validate().is_ok());
        let ideal = clocks
            .ideal_mech_substeps_per_chem()
            .expect("default clocks must yield ideal mech substep count");
        // 3600 / 0.1 = 36000 ideal; default cap 10_000 binds.
        assert_eq!(ideal, 36_000);
        assert_eq!(clocks.mech_substeps_per_chem(), Some(10_000));
        assert_eq!(clocks.mech_substep_cap_binds(), Some(true));
    }

    #[test]
    fn time_orchestration_rejects_non_positive_dts() {
        let mut bad = SimulationClocks::default();
        bad.dt_chemistry = 0.0;
        assert_eq!(
            bad.validate(),
            Err(ClockValidationError::NonPositiveChemistryDt)
        );
        assert!(bad.mech_substeps_per_chem().is_none());

        let mut bad_mech = SimulationClocks::default();
        bad_mech.dt_mechanics_substep = -1.0;
        assert_eq!(
            bad_mech.validate(),
            Err(ClockValidationError::NonPositiveMechanicsSubstep)
        );

        let mut bad_fast = SimulationClocks::default();
        bad_fast.dt_fast_physics = Some(0.0);
        assert_eq!(
            bad_fast.validate(),
            Err(ClockValidationError::NonPositiveFastPhysicsDt)
        );

        let mut bad_cap = SimulationClocks::default();
        bad_cap.max_mech_substeps_per_chem = 0;
        assert_eq!(
            bad_cap.validate(),
            Err(ClockValidationError::ZeroMaxMechSubsteps)
        );
    }

    #[test]
    fn time_orchestration_uncapped_when_chem_near_mech() {
        let clocks = SimulationClocks {
            dt_chemistry: 1.0,
            dt_mechanics_substep: 0.25,
            max_mech_substeps_per_chem: 10_000,
            dt_fast_physics: None,
        };
        assert_eq!(clocks.ideal_mech_substeps_per_chem(), Some(4));
        assert_eq!(clocks.mech_substeps_per_chem(), Some(4));
        assert_eq!(clocks.mech_substep_cap_binds(), Some(false));
    }

    #[test]
    fn time_orchestration_mechanics_inner_loop_default_validates() {
        let cfg = MechanicsInnerLoopConfig::default();
        assert!(cfg.validate().is_ok());
        assert_eq!(cfg.max_cg_iterations, 200);
        assert!(cfg.use_preconditioner);
        assert_eq!(cfg.max_equilibrium_substeps, 1);

        let mut bad = MechanicsInnerLoopConfig::default();
        bad.max_cg_iterations = 0;
        assert!(bad.validate().is_err());
    }
}

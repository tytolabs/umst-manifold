// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Serde DTOs mirroring Lean [`EmittedStepRecord`] / [`EmittedTraceSchema`]
//! (`EpistemicRuntimeSchemaContract.lean`). Witness envelope only — no physics axioms.
//!
//! Wire JSON uses Lean-aligned camelCase field names (`stepMI`, `stepCost`, …).
//!
//! Per-step numeric bounds follow Lean `EmittedTraceWellFormed` (proved from
//! `EpistemicPerStepNumerics` / `epistemicMI_le_log_two`, `epistemicLandauerCost_le_landauerBitEnergy`).
//!
//! # Honest boundary (W29-096)
//!
//! Host serde + per-step / prototype-ε gate surfaces only. Does **not** certify
//! Lean `NumericTraceApproxConsistent` (needs rollout ground truth `(π, ρ₀)`),
//! fleet physics GREEN, `PRODUCTION_WIRED`, `MASTER`, or OP-5.

use std::f64::consts::LN_2;

/// W29 deepen cell — epistemic emitted-trace honest fence bundle.
pub const W29_EPISTEMIC_TRACE_DEEPEN_CELL: &str = "W29-096-EPISTEMIC_TRACE";

/// Honest posture tag — witness-envelope DTOs + well-formedness gates; no fleet claim.
pub const EPISTEMIC_TRACE_POSTURE_TAG: &str = "honest-epistemic-emitted-trace-witness-envelope";

/// Honest physics posture — catalog/well-formed checks pass; does not certify fleet physics GREEN.
pub const EPISTEMIC_TRACE_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the ROS serde witness envelope alone.
pub const EPISTEMIC_TRACE_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const EPISTEMIC_TRACE_MASTER: bool = false;

/// OP-5 claim — refused on this surface.
pub const EPISTEMIC_TRACE_OP5: bool = false;

/// Whether Lean-aligned `EmittedTraceWellFormed` host checks are landed.
pub const EPISTEMIC_TRACE_WELL_FORMED_LANDED: bool = true;

/// Whether prototypeCalibration rolled-up ε utility gates are landed.
pub const EPISTEMIC_TRACE_PROTOTYPE_EPS_LANDED: bool = true;

/// Whether Lean `NumericTraceApproxConsistent` (rollout `(π, ρ₀)` ground truth) is wired.
/// Honestly open — host sum-vs-`n·ε` is a utility stub until rollout witnesses land.
pub const EPISTEMIC_TRACE_NUMERIC_APPROX_WIRED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const EPISTEMIC_TRACE_HONEST_FENCE: &str =
    "emitted_trace_well_formed_landed=true prototype_eps_gate_landed=true numeric_trace_approx_wired=false production_wired=false master_composition_wired=false physics_green=false op5=false";

const _: () = assert!(!EPISTEMIC_TRACE_PHYSICS_GREEN);
const _: () = assert!(!EPISTEMIC_TRACE_PRODUCTION_WIRED);
const _: () = assert!(!EPISTEMIC_TRACE_MASTER);
const _: () = assert!(!EPISTEMIC_TRACE_OP5);
const _: () = assert!(!EPISTEMIC_TRACE_NUMERIC_APPROX_WIRED);
const _: () = assert!(EPISTEMIC_TRACE_WELL_FORMED_LANDED);
const _: () = assert!(EPISTEMIC_TRACE_PROTOTYPE_EPS_LANDED);

/// Typed probe for epistemic emitted-trace posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpistemicTracePostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub well_formed_landed: bool,
    pub prototype_eps_landed: bool,
    pub numeric_approx_wired: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the epistemic emitted-trace surface.
#[must_use]
pub fn epistemic_trace_honest_posture_bundle() -> EpistemicTracePostureProbe {
    EpistemicTracePostureProbe {
        physics_green: EPISTEMIC_TRACE_PHYSICS_GREEN,
        production_wired: EPISTEMIC_TRACE_PRODUCTION_WIRED,
        master: EPISTEMIC_TRACE_MASTER,
        op5: EPISTEMIC_TRACE_OP5,
        well_formed_landed: EPISTEMIC_TRACE_WELL_FORMED_LANDED,
        prototype_eps_landed: EPISTEMIC_TRACE_PROTOTYPE_EPS_LANDED,
        numeric_approx_wired: EPISTEMIC_TRACE_NUMERIC_APPROX_WIRED,
        honest_fence: EPISTEMIC_TRACE_HONEST_FENCE,
        posture_tag: EPISTEMIC_TRACE_POSTURE_TAG,
        deepen_cell: W29_EPISTEMIC_TRACE_DEEPEN_CELL,
    }
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER / OP-5 claims on the epistemic-trace surface.
#[must_use]
pub fn epistemic_trace_posture_honest(probe: &EpistemicTracePostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && !probe.numeric_approx_wired
        && probe.well_formed_landed
        && probe.prototype_eps_landed
        && probe.deepen_cell == W29_EPISTEMIC_TRACE_DEEPEN_CELL
        && probe
            .honest_fence
            .contains("emitted_trace_well_formed_landed=true")
        && probe
            .honest_fence
            .contains("numeric_trace_approx_wired=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("op5=false")
}

/// Compile-time / runtime refuse path for invented GREEN / production pins.
pub fn epistemic_trace_refuse_invented_pins() -> Result<(), &'static str> {
    if EPISTEMIC_TRACE_PHYSICS_GREEN {
        return Err(
            "EPISTEMIC_TRACE_PHYSICS_GREEN must stay false — witness envelope ≠ fleet physics",
        );
    }
    if EPISTEMIC_TRACE_PRODUCTION_WIRED {
        return Err(
            "EPISTEMIC_TRACE_PRODUCTION_WIRED must stay false until ROS/fleet production wire closes",
        );
    }
    if EPISTEMIC_TRACE_MASTER {
        return Err("EPISTEMIC_TRACE_MASTER must stay false — not an OP-5 composition pin");
    }
    if EPISTEMIC_TRACE_OP5 {
        return Err("EPISTEMIC_TRACE_OP5 must stay false — OP-5 not claimed on this surface");
    }
    if EPISTEMIC_TRACE_NUMERIC_APPROX_WIRED {
        return Err(
            "EPISTEMIC_TRACE_NUMERIC_APPROX_WIRED must stay false until NumericTraceApproxConsistent rollout witnesses land",
        );
    }
    Ok(())
}

/// Aggregate honesty gate used by unit proofs / meta probes.
pub fn validate_epistemic_trace_honesty() -> Result<(), &'static str> {
    epistemic_trace_refuse_invented_pins()?;
    let probe = epistemic_trace_honest_posture_bundle();
    if !epistemic_trace_posture_honest(&probe) {
        return Err("epistemic_trace_posture_probe failed honest fence census");
    }
    Ok(())
}

/// Landauer bit energy `k_B T ln 2` (joules), matching Lean `landauerBitEnergy`.
///
/// SSOT: [`crate::constants::landauer_bit_energy_joules`] (`umst-math` when `math-constants`).
#[must_use]
pub fn landauer_bit_energy_joules(temperature_k: f64) -> f64 {
    crate::constants::landauer_bit_energy_joules(temperature_k)
}

/// Violation of Lean `EmittedTraceWellFormed` on a single step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmittedStepWellFormedError {
    StepMiNegative,
    StepMiExceedsLog2,
    StepCostNegative,
    StepCostExceedsLandauer,
    ConfidenceOutOfRange,
    TemperatureNegative,
}

/// Violation of emitted-trace well-formedness (horizon + per-step bounds).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmittedTraceWellFormedError {
    TemperatureNegative,
    HorizonStepCountMismatch {
        horizon_n: u32,
        step_count: usize,
    },
    Step {
        index: usize,
        detail: EmittedStepWellFormedError,
    },
}

/// Violation of Lean `prototypeCalibration` rolled-up ε envelopes (`epsMIAgg` / `epsCostAgg`).
///
/// **Not** [`EmittedTraceWellFormed`]: per-step catalog caps are checked separately.
/// **Not** [`NumericTraceApproxConsistent`]: that morphism needs rollout ground truth `(π, ρ₀)`;
/// host-only sum-vs-`n·ε` is a utility-calibration stub until rollout witnesses are wired.
#[derive(Debug, Clone, PartialEq)]
pub enum PrototypeCalibrationBoundsError {
    HorizonStepCountMismatch { horizon_n: u32, step_count: usize },
    RolledMiExceeds { rolled_sum: f64, bound: f64 },
    RolledCostExceeds { rolled_sum: f64, bound: f64 },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EmittedStepRecord {
    #[cfg_attr(feature = "serde", serde(rename = "stepMI"))]
    pub step_mi: f64,
    #[cfg_attr(feature = "serde", serde(rename = "stepCost"))]
    pub step_cost: f64,
    #[cfg_attr(
        feature = "serde",
        serde(
            rename = "thermodynamicAdmissible",
            default = "default_thermodynamic_admissible"
        )
    )]
    pub thermodynamic_admissible: bool,
    #[cfg_attr(feature = "serde", serde(default = "default_confidence"))]
    pub confidence: f64,
}

impl EmittedStepRecord {
    #[must_use]
    pub fn new(step_mi: f64, step_cost: f64) -> Self {
        EmittedStepRecord {
            step_mi,
            step_cost,
            thermodynamic_admissible: true,
            confidence: 1.0,
        }
    }

    /// Runtime check for Lean `EmittedTraceWellFormed` step inequalities at temperature `T`.
    pub fn check_emitted_trace_well_formed(
        &self,
        temperature_k: f64,
    ) -> Result<(), EmittedStepWellFormedError> {
        if temperature_k < 0.0 {
            return Err(EmittedStepWellFormedError::TemperatureNegative);
        }
        if self.step_mi < 0.0 {
            return Err(EmittedStepWellFormedError::StepMiNegative);
        }
        if self.step_mi > LN_2 {
            return Err(EmittedStepWellFormedError::StepMiExceedsLog2);
        }
        if self.step_cost < 0.0 {
            return Err(EmittedStepWellFormedError::StepCostNegative);
        }
        let cap = landauer_bit_energy_joules(temperature_k);
        if self.step_cost > cap {
            return Err(EmittedStepWellFormedError::StepCostExceedsLandauer);
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(EmittedStepWellFormedError::ConfidenceOutOfRange);
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
fn default_thermodynamic_admissible() -> bool {
    true
}

#[cfg(feature = "serde")]
fn default_confidence() -> f64 {
    1.0
}

/// Finite-horizon emitted trace at temperature `T` (Lean `EmittedTraceSchema n T`).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct EmittedTraceSchema {
    #[cfg_attr(feature = "serde", serde(rename = "schemaTag", default))]
    pub schema_tag: String,
    /// Horizon `n` (Lean type parameter).
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    pub horizon_n: u32,
    /// Temperature `T` (Kelvin in formal layer; host bridge may document units).
    #[cfg_attr(feature = "serde", serde(rename = "T"))]
    pub temperature_t: f64,
    /// Per-step records in rollout order (`k < n`).
    pub steps: Vec<EmittedStepRecord>,
}

/// Per-step MI tolerance from Lean `prototypeCalibration.epsMIStep` (= `1/10000`).
pub const PROTOTYPE_EPS_MI_STEP: f64 = 1.0 / 10_000.0;

/// Per-step Landauer cost tolerance from Lean `prototypeCalibration.epsCostStep`.
pub const PROTOTYPE_EPS_COST_STEP: f64 = 1.0 / 10_000.0;

/// Rolled-up MI bound for horizon `n`: `n * PROTOTYPE_EPS_MI_STEP` (`prototypeCalibration.epsMIAgg`).
#[must_use]
pub fn prototype_eps_mi_agg(horizon_n: u32) -> f64 {
    f64::from(horizon_n) * PROTOTYPE_EPS_MI_STEP
}

/// Rolled-up cost bound for horizon `n` (`prototypeCalibration.epsCostAgg`).
#[must_use]
pub fn prototype_eps_cost_agg(horizon_n: u32) -> f64 {
    f64::from(horizon_n) * PROTOTYPE_EPS_COST_STEP
}

impl EmittedTraceSchema {
    pub const SCHEMA_TAG: &'static str = "umst.emitted_trace.v1";

    /// Whether `steps.len()` matches Lean horizon parameter `n`.
    #[must_use]
    pub fn horizon_matches_steps(&self) -> bool {
        self.steps.len() == self.horizon_n as usize
    }

    /// Number of emitted steps (host length; may disagree with `horizon_n` until well-formed).
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Sum of per-step MI (`PerStepNumericRecord.rolledMI` fold).
    #[must_use]
    pub fn summed_step_mi(&self) -> f64 {
        self.steps.iter().map(|s| s.step_mi).sum()
    }

    /// Sum of per-step Landauer cost (`PerStepNumericRecord.rolledCost` fold).
    #[must_use]
    pub fn summed_step_cost(&self) -> f64 {
        self.steps.iter().map(|s| s.step_cost).sum()
    }

    #[deprecated(note = "renamed to summed_step_mi")]
    #[must_use]
    pub fn aggregate_step_mi(&self) -> f64 {
        self.summed_step_mi()
    }

    #[deprecated(note = "renamed to summed_step_cost")]
    #[must_use]
    pub fn aggregate_step_cost(&self) -> f64 {
        self.summed_step_cost()
    }

    /// True when `steps.len() == horizon_n` and rolled-up MI/cost sit inside Lean
    /// `prototypeCalibration.epsMIAgg` / `epsCostAgg` envelopes (utility calibration, not MI cap).
    #[must_use]
    pub fn within_prototype_calibration_bounds(&self) -> bool {
        self.check_prototype_calibration_bounds().is_ok()
    }

    /// Structured check: rolled-up sums vs `prototypeCalibration.epsMIAgg` / `epsCostAgg`.
    pub fn check_prototype_calibration_bounds(
        &self,
    ) -> Result<(), PrototypeCalibrationBoundsError> {
        let step_count = self.steps.len();
        if step_count != self.horizon_n as usize {
            return Err(PrototypeCalibrationBoundsError::HorizonStepCountMismatch {
                horizon_n: self.horizon_n,
                step_count,
            });
        }
        let n = self.horizon_n;
        let mi_bound = prototype_eps_mi_agg(n);
        let mi_agg = self.summed_step_mi();
        if mi_agg > mi_bound {
            return Err(PrototypeCalibrationBoundsError::RolledMiExceeds {
                rolled_sum: mi_agg,
                bound: mi_bound,
            });
        }
        let cost_bound = prototype_eps_cost_agg(n);
        let cost_agg = self.summed_step_cost();
        if cost_agg > cost_bound {
            return Err(PrototypeCalibrationBoundsError::RolledCostExceeds {
                rolled_sum: cost_agg,
                bound: cost_bound,
            });
        }
        Ok(())
    }

    /// Fixture inside prototype rolled-up ε envelopes (Track G.2 positive case).
    #[must_use]
    pub fn sample_calibration_envelope_fixture() -> Self {
        let steps = vec![
            EmittedStepRecord::new(PROTOTYPE_EPS_MI_STEP * 0.5, PROTOTYPE_EPS_COST_STEP * 0.5),
            EmittedStepRecord::new(PROTOTYPE_EPS_MI_STEP * 0.5, PROTOTYPE_EPS_COST_STEP * 0.5),
        ];
        Self::new(2, 300.0, steps)
    }

    /// Fixture exceeding prototype rolled-up ε (Track G.2 negative case).
    #[must_use]
    pub fn sample_calibration_envelope_violation_fixture() -> Self {
        let steps = vec![EmittedStepRecord::new(
            prototype_eps_mi_agg(1) * 2.0,
            prototype_eps_cost_agg(1) * 2.0,
        )];
        Self::new(1, 300.0, steps)
    }

    #[must_use]
    pub fn new(horizon_n: u32, temperature_t: f64, steps: Vec<EmittedStepRecord>) -> Self {
        EmittedTraceSchema {
            schema_tag: Self::SCHEMA_TAG.into(),
            horizon_n,
            temperature_t,
            steps,
        }
    }

    /// Minimal fixture aligned with `EmittedTraceSchema.ofRollout` metadata defaults.
    #[must_use]
    pub fn sample_fixture() -> Self {
        let steps = vec![
            EmittedStepRecord::new(0.25, 1.0e-21),
            EmittedStepRecord::new(0.31, 1.2e-21),
        ];
        Self::new(2, 300.0, steps)
    }

    /// Well-formedness for the full trace: `0 ≤ T`, `steps.len() == n`, each step satisfies
    /// [`EmittedStepRecord::check_emitted_trace_well_formed`].
    pub fn check_emitted_trace_well_formed(&self) -> Result<(), EmittedTraceWellFormedError> {
        if self.temperature_t < 0.0 {
            return Err(EmittedTraceWellFormedError::TemperatureNegative);
        }
        let step_count = self.steps.len();
        if step_count != self.horizon_n as usize {
            return Err(EmittedTraceWellFormedError::HorizonStepCountMismatch {
                horizon_n: self.horizon_n,
                step_count,
            });
        }
        for (index, step) in self.steps.iter().enumerate() {
            step.check_emitted_trace_well_formed(self.temperature_t)
                .map_err(|detail| EmittedTraceWellFormedError::Step { index, detail })?;
        }
        Ok(())
    }
}

/// Prototype η from trace rolled-up MI vs Lean `prototypeCalibration.epsMIAgg` (Track G.3).
/// Post-CBF calibration hook only — does not bypass [`crate::ai::cbf::ThermodynamicCBF`].
#[cfg(feature = "trace-calibration")]
#[must_use]
pub fn prototype_eta_from_trace(schema: &EmittedTraceSchema) -> f32 {
    let bound = prototype_eps_mi_agg(schema.horizon_n);
    if bound <= 0.0 {
        return 0.0;
    }
    let ratio = schema.summed_step_mi() / bound;
    ratio.clamp(0.0, 1.0) as f32
}

#[cfg(feature = "serde")]
impl Default for EmittedTraceSchema {
    fn default() -> Self {
        Self {
            schema_tag: Self::SCHEMA_TAG.into(),
            horizon_n: 0,
            temperature_t: 0.0,
            steps: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epistemic_trace_honest_fence_holds() {
        validate_epistemic_trace_honesty().expect("W29-096 epistemic_trace honest fence");
        let probe = epistemic_trace_honest_posture_bundle();
        assert!(epistemic_trace_posture_honest(&probe));
        assert_eq!(probe.deepen_cell, W29_EPISTEMIC_TRACE_DEEPEN_CELL);
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(!probe.numeric_approx_wired);
        assert!(probe.well_formed_landed);
        assert!(probe.prototype_eps_landed);
        assert_eq!(probe.honest_fence, EPISTEMIC_TRACE_HONEST_FENCE);
    }

    #[test]
    fn epistemic_trace_refuse_invented_pins_ok() {
        epistemic_trace_refuse_invented_pins().expect("invented pins refused");
    }

    #[test]
    fn sample_fixture_horizon_matches_and_well_formed() {
        let trace = EmittedTraceSchema::sample_fixture();
        assert!(trace.horizon_matches_steps());
        assert_eq!(trace.step_count(), 2);
        assert_eq!(trace.horizon_n, 2);
        trace
            .check_emitted_trace_well_formed()
            .expect("sample_fixture EmittedTraceWellFormed");
    }

    #[test]
    fn horizon_mismatch_is_not_well_formed() {
        let mut trace = EmittedTraceSchema::sample_fixture();
        trace.horizon_n = 3;
        assert!(!trace.horizon_matches_steps());
        assert!(matches!(
            trace.check_emitted_trace_well_formed(),
            Err(EmittedTraceWellFormedError::HorizonStepCountMismatch {
                horizon_n: 3,
                step_count: 2
            })
        ));
    }

    #[test]
    fn landauer_bit_energy_positive_at_room_temp() {
        let e = landauer_bit_energy_joules(300.0);
        assert!(e.is_finite() && e > 0.0);
    }

    #[test]
    fn prototype_eps_agg_scales_with_horizon() {
        assert!((prototype_eps_mi_agg(0) - 0.0).abs() < f64::EPSILON);
        assert!((prototype_eps_mi_agg(5) - 5.0 * PROTOTYPE_EPS_MI_STEP).abs() < f64::EPSILON);
        assert!((prototype_eps_cost_agg(5) - 5.0 * PROTOTYPE_EPS_COST_STEP).abs() < f64::EPSILON);
    }

    #[test]
    fn calibration_envelope_fixture_within_bounds() {
        let ok = EmittedTraceSchema::sample_calibration_envelope_fixture();
        assert!(ok.horizon_matches_steps());
        assert!(ok.within_prototype_calibration_bounds());
        ok.check_prototype_calibration_bounds()
            .expect("calibration envelope fixture within epsMIAgg/epsCostAgg");
    }

    #[test]
    fn calibration_violation_fixture_exceeds_rolled_mi() {
        let bad = EmittedTraceSchema::sample_calibration_envelope_violation_fixture();
        assert!(!bad.within_prototype_calibration_bounds());
        assert!(matches!(
            bad.check_prototype_calibration_bounds(),
            Err(PrototypeCalibrationBoundsError::RolledMiExceeds { .. })
                | Err(PrototypeCalibrationBoundsError::RolledCostExceeds { .. })
        ));
    }

    #[test]
    fn step_rejects_mi_above_ln2() {
        let bad = EmittedStepRecord::new(LN_2 + 1e-9, 1.0e-21);
        assert_eq!(
            bad.check_emitted_trace_well_formed(300.0),
            Err(EmittedStepWellFormedError::StepMiExceedsLog2)
        );
    }

    #[test]
    fn summed_folds_match_manual() {
        let trace = EmittedTraceSchema::sample_fixture();
        let mi: f64 = trace.steps.iter().map(|s| s.step_mi).sum();
        let cost: f64 = trace.steps.iter().map(|s| s.step_cost).sum();
        assert!((trace.summed_step_mi() - mi).abs() < f64::EPSILON);
        assert!((trace.summed_step_cost() - cost).abs() < f64::EPSILON);
    }
}

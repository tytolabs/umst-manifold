// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Serde DTOs mirroring Lean [`EmittedStepRecord`] / [`EmittedTraceSchema`]
//! (`EpistemicRuntimeSchemaContract.lean`). Witness envelope only — no physics axioms.
//!
//! Wire JSON uses Lean-aligned camelCase field names (`stepMI`, `stepCost`, …).
//!
//! Per-step numeric bounds follow Lean `EmittedTraceWellFormed` (proved from
//! `EpistemicPerStepNumerics` / `epistemicMI_le_log_two`, `epistemicLandauerCost_le_landauerBitEnergy`).

use std::f64::consts::LN_2;

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

/// Violation of Lean `prototypeCalibration` aggregate ε envelopes (`epsMIAgg` / `epsCostAgg`).
///
/// **Not** [`EmittedTraceWellFormed`]: per-step catalog caps are checked separately.
/// **Not** [`NumericTraceApproxConsistent`]: that morphism needs rollout ground truth `(π, ρ₀)`;
/// host-only sum-vs-`n·ε` is a utility-calibration stub until rollout witnesses are wired.
#[derive(Debug, Clone, PartialEq)]
pub enum PrototypeCalibrationBoundsError {
    HorizonStepCountMismatch { horizon_n: u32, step_count: usize },
    AggregateMiExceeds { aggregate: f64, bound: f64 },
    AggregateCostExceeds { aggregate: f64, bound: f64 },
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

/// Aggregate MI bound for horizon `n`: `n * PROTOTYPE_EPS_MI_STEP` (`prototypeCalibration.epsMIAgg`).
#[must_use]
pub fn prototype_eps_mi_agg(horizon_n: u32) -> f64 {
    f64::from(horizon_n) * PROTOTYPE_EPS_MI_STEP
}

/// Aggregate cost bound for horizon `n` (`prototypeCalibration.epsCostAgg`).
#[must_use]
pub fn prototype_eps_cost_agg(horizon_n: u32) -> f64 {
    f64::from(horizon_n) * PROTOTYPE_EPS_COST_STEP
}

impl EmittedTraceSchema {
    pub const SCHEMA_TAG: &'static str = "umst.emitted_trace.v1";

    /// Sum of per-step MI (`PerStepNumericRecord.aggregateMI` fold).
    #[must_use]
    pub fn aggregate_step_mi(&self) -> f64 {
        self.steps.iter().map(|s| s.step_mi).sum()
    }

    /// Sum of per-step Landauer cost (`PerStepNumericRecord.aggregateCost` fold).
    #[must_use]
    pub fn aggregate_step_cost(&self) -> f64 {
        self.steps.iter().map(|s| s.step_cost).sum()
    }

    /// True when `steps.len() == horizon_n` and aggregate MI/cost sit inside Lean
    /// `prototypeCalibration.epsMIAgg` / `epsCostAgg` envelopes (utility calibration, not MI cap).
    #[must_use]
    pub fn within_prototype_calibration_bounds(&self) -> bool {
        self.check_prototype_calibration_bounds().is_ok()
    }

    /// Structured check: aggregate sums vs `prototypeCalibration.epsMIAgg` / `epsCostAgg`.
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
        let mi_agg = self.aggregate_step_mi();
        if mi_agg > mi_bound {
            return Err(PrototypeCalibrationBoundsError::AggregateMiExceeds {
                aggregate: mi_agg,
                bound: mi_bound,
            });
        }
        let cost_bound = prototype_eps_cost_agg(n);
        let cost_agg = self.aggregate_step_cost();
        if cost_agg > cost_bound {
            return Err(PrototypeCalibrationBoundsError::AggregateCostExceeds {
                aggregate: cost_agg,
                bound: cost_bound,
            });
        }
        Ok(())
    }

    /// Fixture inside prototype aggregate ε envelopes (Track G.2 positive case).
    #[must_use]
    pub fn sample_calibration_envelope_fixture() -> Self {
        let steps = vec![
            EmittedStepRecord::new(PROTOTYPE_EPS_MI_STEP * 0.5, PROTOTYPE_EPS_COST_STEP * 0.5),
            EmittedStepRecord::new(PROTOTYPE_EPS_MI_STEP * 0.5, PROTOTYPE_EPS_COST_STEP * 0.5),
        ];
        Self::new(2, 300.0, steps)
    }

    /// Fixture exceeding prototype aggregate ε (Track G.2 negative case).
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

/// Prototype η from trace aggregate MI vs Lean `prototypeCalibration.epsMIAgg` (Track G.3).
/// Post-CBF calibration hook only — does not bypass [`crate::ai::cbf::ThermodynamicCBF`].
#[cfg(feature = "trace-calibration")]
#[must_use]
pub fn prototype_eta_from_trace(schema: &EmittedTraceSchema) -> f32 {
    let bound = prototype_eps_mi_agg(schema.horizon_n);
    if bound <= 0.0 {
        return 0.0;
    }
    let ratio = schema.aggregate_step_mi() / bound;
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

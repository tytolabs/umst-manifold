// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Trace-driven η calibration **stub** (Track G.3).
//!
//! Reads an [`EmittedStepRecord`] stream and compares per-step `stepMI` against the
//! catalog upper bound documented in Lean `EmittedTraceWellFormed`
//! (`EpistemicRuntimeSchemaContract.lean`: `stepMI ≤ ln 2` nats).
//!
//! **Not proved here:** Lean `EpistemicTraceDrivenCalibrationWitness` /
//! `traceCalibrationWitnessAt_*` tie ε budgets to policy utility; this module only
//! suggests a conservative `eta_bound`; wired to [`crate::ai::ppo::ManifoldGateway::eta`]
//! via [`crate::ai::ppo::ManifoldGateway::calibrate_eta_from_trace`] after CBF (witness ladder R2).

use crate::ros::{EmittedStepRecord, EmittedTraceSchema};

/// Per-step MI upper bound in **nats** (Lean `Real.log 2` / path-qubit catalog).
pub const CATALOG_STEP_MI_UPPER_NAT: f64 = std::f64::consts::LN_2;

/// Report from scanning emitted steps — witness envelope, not a calibration certificate.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceCalibrationReport {
    pub steps_checked: usize,
    pub max_step_mi: f64,
    /// `max(0, step_mi − catalog_upper)` over the stream.
    pub max_excess_over_catalog: f64,
    /// Suggested cap for reward-shaping η: fractional headroom beyond catalog (`0` if all in band).
    pub eta_bound_suggested: f64,
    pub all_within_catalog: bool,
}

impl TraceCalibrationReport {
    #[must_use]
    pub fn empty() -> Self {
        TraceCalibrationReport {
            steps_checked: 0,
            max_step_mi: 0.0,
            max_excess_over_catalog: 0.0,
            eta_bound_suggested: 0.0,
            all_within_catalog: true,
        }
    }
}

/// Non-negative excess of `step_mi` above the catalog per-step MI cap (nats).
#[must_use]
pub fn step_mi_excess_over_catalog(step_mi: f64) -> f64 {
    (step_mi - CATALOG_STEP_MI_UPPER_NAT).max(0.0)
}

/// Whether `step_mi` lies in the catalog band `0 ≤ stepMI ≤ ln 2` (Lean well-formedness).
#[must_use]
pub fn step_mi_within_catalog(step_mi: f64) -> bool {
    step_mi >= 0.0 && step_mi <= CATALOG_STEP_MI_UPPER_NAT
}

/// Scan `steps` and derive a conservative suggested η bound from worst-case MI overrun.
#[must_use]
pub fn calibrate_eta_bound_from_steps<'a, I>(steps: I) -> TraceCalibrationReport
where
    I: IntoIterator<Item = &'a EmittedStepRecord>,
{
    let mut report = TraceCalibrationReport::empty();
    for step in steps {
        report.steps_checked += 1;
        report.max_step_mi = report.max_step_mi.max(step.step_mi);
        let excess = step_mi_excess_over_catalog(step.step_mi);
        report.max_excess_over_catalog = report.max_excess_over_catalog.max(excess);
        if !step_mi_within_catalog(step.step_mi) {
            report.all_within_catalog = false;
        }
    }
    if report.steps_checked == 0 {
        return report;
    }
    report.eta_bound_suggested = if report.max_excess_over_catalog <= 0.0 {
        0.0
    } else {
        report.max_excess_over_catalog / CATALOG_STEP_MI_UPPER_NAT
    };
    report
}

/// Convenience: calibrate from a full [`EmittedTraceSchema`] (rollout-ordered `steps`).
#[must_use]
pub fn calibrate_eta_bound_from_trace(trace: &EmittedTraceSchema) -> TraceCalibrationReport {
    calibrate_eta_bound_from_steps(trace.steps.iter())
}

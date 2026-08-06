// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Trace-driven η calibration scan (Track G.3) — **host envelope, not a certificate**.
//!
//! Reads an [`EmittedStepRecord`] stream and compares per-step `stepMI` against the
//! catalog upper bound documented in Lean `EmittedTraceWellFormed`
//! (`EpistemicRuntimeSchemaContract.lean`: `stepMI ≤ ln 2` nats).
//!
//! # Honest fences (do not invent)
//!
//! - **Not** Lean `EpistemicTraceDrivenCalibrationWitness` /
//!   `traceCalibrationWitnessAt_*` (ε budgets ↔ policy utility stay on the Lean side).
//! - **Not** `PRODUCTION_WIRED` / `MASTER` / OP-5 closure — this module only suggests a
//!   conservative `eta_bound` for post-CBF reward shaping.
//! - Wired to [`crate::ai::ppo::ManifoldGateway::eta`] via
//!   [`crate::ai::ppo::ManifoldGateway::calibrate_eta_from_trace`] under feature
//!   `trace-calibration` after CBF (witness ladder R2) — still a host suggestion.
//! - Non-finite `step_mi` is refused as catalog-in-band (fail-closed for the scan).

use crate::ros::{EmittedStepRecord, EmittedTraceSchema};

/// Per-step MI upper bound in **nats** (Lean `Real.log 2` / path-qubit catalog).
pub const CATALOG_STEP_MI_UPPER_NAT: f64 = std::f64::consts::LN_2;

/// Fence: Lean calibration witness certificate is **not** claimed by this host scan.
pub const TRACE_CALIBRATION_WITNESS_CERTIFIED: bool = false;

/// Fence: production η path is **not** declared fully wired / MASTER-closed here.
pub const TRACE_CALIBRATION_PRODUCTION_WIRED: bool = false;

/// Fence: MASTER / OP-5 flip is **not** authorized by this module alone.
pub const TRACE_CALIBRATION_MASTER_AUTHORIZED: bool = false;

/// Report from scanning emitted steps — witness envelope, not a calibration certificate.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceCalibrationReport {
    pub steps_checked: usize,
    /// Steps with `step_mi` outside `0 ≤ stepMI ≤ ln 2`, or non-finite.
    pub steps_outside_catalog: usize,
    /// Non-finite `step_mi` observations (NaN / ±∞); subset of outside-catalog.
    pub nonfinite_step_mi_count: usize,
    pub max_step_mi: f64,
    /// Minimum finite `step_mi` seen; `+∞` sentinel when no finite samples.
    pub min_step_mi: f64,
    /// `max(0, step_mi − catalog_upper)` over finite samples; `+∞` if any non-finite.
    pub max_excess_over_catalog: f64,
    /// Suggested cap for reward-shaping η: fractional headroom beyond catalog, clamped to
    /// `[0, 1]` (`0` if all finite samples in band; `1` if non-finite observed).
    pub eta_bound_suggested: f64,
    pub all_within_catalog: bool,
}

impl TraceCalibrationReport {
    #[must_use]
    pub fn empty() -> Self {
        TraceCalibrationReport {
            steps_checked: 0,
            steps_outside_catalog: 0,
            nonfinite_step_mi_count: 0,
            max_step_mi: 0.0,
            min_step_mi: f64::INFINITY,
            max_excess_over_catalog: 0.0,
            eta_bound_suggested: 0.0,
            all_within_catalog: true,
        }
    }

    /// Gateway-facing η in `[0, 1]` (same clamp policy as [`clamp_eta_bound_unit`]).
    #[must_use]
    pub fn eta_bound_for_gateway(&self) -> f64 {
        clamp_eta_bound_unit(self.eta_bound_suggested)
    }
}

/// Clamp a raw η suggestion into the unit interval used by reward shaping.
#[must_use]
pub fn clamp_eta_bound_unit(eta: f64) -> f64 {
    if !eta.is_finite() {
        return 1.0;
    }
    eta.clamp(0.0, 1.0)
}

/// Non-negative excess of `step_mi` above the catalog per-step MI cap (nats).
///
/// Non-finite inputs yield `+∞` (fail-closed overrun signal).
#[must_use]
pub fn step_mi_excess_over_catalog(step_mi: f64) -> f64 {
    if !step_mi.is_finite() {
        return f64::INFINITY;
    }
    (step_mi - CATALOG_STEP_MI_UPPER_NAT).max(0.0)
}

/// Whether `step_mi` lies in the catalog band `0 ≤ stepMI ≤ ln 2` (Lean well-formedness).
///
/// Non-finite values are **outside** the band (honest refuse).
#[must_use]
pub fn step_mi_within_catalog(step_mi: f64) -> bool {
    step_mi.is_finite() && step_mi >= 0.0 && step_mi <= CATALOG_STEP_MI_UPPER_NAT
}

/// Scan `steps` and derive a conservative suggested η bound from worst-case MI overrun.
#[must_use]
pub fn calibrate_eta_bound_from_steps<'a, I>(steps: I) -> TraceCalibrationReport
where
    I: IntoIterator<Item = &'a EmittedStepRecord>,
{
    debug_assert!(!TRACE_CALIBRATION_WITNESS_CERTIFIED);
    debug_assert!(!TRACE_CALIBRATION_PRODUCTION_WIRED);
    debug_assert!(!TRACE_CALIBRATION_MASTER_AUTHORIZED);

    let mut report = TraceCalibrationReport::empty();
    for step in steps {
        report.steps_checked += 1;
        let mi = step.step_mi;
        if !mi.is_finite() {
            report.nonfinite_step_mi_count += 1;
            report.steps_outside_catalog += 1;
            report.all_within_catalog = false;
            report.max_excess_over_catalog = f64::INFINITY;
            continue;
        }
        report.max_step_mi = report.max_step_mi.max(mi);
        report.min_step_mi = report.min_step_mi.min(mi);
        let excess = step_mi_excess_over_catalog(mi);
        if excess.is_finite() {
            report.max_excess_over_catalog = report.max_excess_over_catalog.max(excess);
        } else {
            report.max_excess_over_catalog = f64::INFINITY;
        }
        if !step_mi_within_catalog(mi) {
            report.steps_outside_catalog += 1;
            report.all_within_catalog = false;
        }
    }
    if report.steps_checked == 0 {
        report.min_step_mi = 0.0;
        return report;
    }
    if !report.min_step_mi.is_finite() {
        // Only non-finite samples — leave min at +∞; η saturated.
        report.eta_bound_suggested = 1.0;
        return report;
    }
    report.eta_bound_suggested = if !report.max_excess_over_catalog.is_finite() {
        1.0
    } else if report.max_excess_over_catalog <= 0.0 {
        0.0
    } else {
        clamp_eta_bound_unit(report.max_excess_over_catalog / CATALOG_STEP_MI_UPPER_NAT)
    };
    report
}

/// Convenience: calibrate from a full [`EmittedTraceSchema`] (rollout-ordered `steps`).
#[must_use]
pub fn calibrate_eta_bound_from_trace(trace: &EmittedTraceSchema) -> TraceCalibrationReport {
    calibrate_eta_bound_from_steps(trace.steps.iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ros::EmittedStepRecord;

    #[test]
    fn fences_remain_unclaimed() {
        assert!(!TRACE_CALIBRATION_WITNESS_CERTIFIED);
        assert!(!TRACE_CALIBRATION_PRODUCTION_WIRED);
        assert!(!TRACE_CALIBRATION_MASTER_AUTHORIZED);
    }

    #[test]
    fn empty_scan_is_vacuously_in_band() {
        let report = calibrate_eta_bound_from_steps(std::iter::empty());
        assert_eq!(report.steps_checked, 0);
        assert!(report.all_within_catalog);
        assert_eq!(report.eta_bound_suggested, 0.0);
        assert_eq!(report.min_step_mi, 0.0);
    }

    #[test]
    fn in_band_fixture_suggests_zero_eta() {
        let steps = [
            EmittedStepRecord::new(0.25, 1.0e-21),
            EmittedStepRecord::new(0.31, 1.2e-21),
        ];
        let report = calibrate_eta_bound_from_steps(steps.iter());
        assert!(report.all_within_catalog);
        assert_eq!(report.steps_outside_catalog, 0);
        assert_eq!(report.eta_bound_suggested, 0.0);
        assert_eq!(report.eta_bound_for_gateway(), 0.0);
        assert!((report.max_step_mi - 0.31).abs() < 1e-15);
        assert!((report.min_step_mi - 0.25).abs() < 1e-15);
    }

    #[test]
    fn catalog_overrun_scales_eta_and_clamps_unit() {
        let over = EmittedStepRecord::new(CATALOG_STEP_MI_UPPER_NAT + 0.01, 1.0e-21);
        let report = calibrate_eta_bound_from_steps(std::slice::from_ref(&over).iter());
        assert!(!report.all_within_catalog);
        assert_eq!(report.steps_outside_catalog, 1);
        let raw = 0.01 / CATALOG_STEP_MI_UPPER_NAT;
        assert!((report.eta_bound_suggested - raw).abs() < 1e-12);
        assert_eq!(report.eta_bound_for_gateway(), report.eta_bound_suggested);

        let huge = EmittedStepRecord::new(CATALOG_STEP_MI_UPPER_NAT * 10.0, 1.0e-21);
        let saturated = calibrate_eta_bound_from_steps(std::slice::from_ref(&huge).iter());
        assert_eq!(saturated.eta_bound_suggested, 1.0);
    }

    #[test]
    fn nonfinite_step_mi_fail_closed() {
        assert!(!step_mi_within_catalog(f64::NAN));
        assert!(!step_mi_within_catalog(f64::INFINITY));
        assert!(step_mi_excess_over_catalog(f64::NAN).is_infinite());

        let bad = EmittedStepRecord::new(f64::NAN, 1.0e-21);
        let report = calibrate_eta_bound_from_steps(std::slice::from_ref(&bad).iter());
        assert!(!report.all_within_catalog);
        assert_eq!(report.nonfinite_step_mi_count, 1);
        assert_eq!(report.eta_bound_suggested, 1.0);
        assert_eq!(clamp_eta_bound_unit(f64::NAN), 1.0);
    }

    #[test]
    fn negative_step_mi_outside_catalog_zero_excess_eta() {
        let neg = EmittedStepRecord::new(-0.1, 1.0e-21);
        let report = calibrate_eta_bound_from_steps(std::slice::from_ref(&neg).iter());
        assert!(!report.all_within_catalog);
        assert_eq!(report.steps_outside_catalog, 1);
        assert_eq!(report.max_excess_over_catalog, 0.0);
        assert_eq!(report.eta_bound_suggested, 0.0);
    }
}

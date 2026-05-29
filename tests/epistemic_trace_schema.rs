// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `serde_json` round-trip (G.1) and per-step numeric bounds (G.2) for Lean
//! `EmittedTraceSchema` / `EmittedStepRecord` (`EpistemicRuntimeSchemaContract`,
//! `EmittedTraceWellFormed` from `EpistemicPerStepNumerics`).

use std::f64::consts::LN_2;

use umst_manifold::ros::{
    landauer_bit_energy_joules, prototype_eps_cost_agg, prototype_eps_mi_agg,
    EmittedStepRecord, EmittedStepWellFormedError, EmittedTraceSchema,
    EmittedTraceWellFormedError, PrototypeCalibrationBoundsError, PROTOTYPE_EPS_COST_STEP,
    PROTOTYPE_EPS_MI_STEP,
};

#[test]
fn emitted_step_record_json_roundtrip() {
    let v = EmittedStepRecord::new(0.42, 2.5e-21);
    let s = serde_json::to_string(&v).expect("serialize");
    let back: EmittedStepRecord = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(back, v);
    assert!(s.contains("stepMI"));
    assert!(s.contains("stepCost"));
}

#[test]
fn emitted_step_record_omitted_defaults_deserialize() {
    let json = r#"{"stepMI":0.1,"stepCost":1.0}"#;
    let v: EmittedStepRecord = serde_json::from_str(json).expect("deserialize");
    assert!(v.thermodynamic_admissible);
    assert!((v.confidence - 1.0).abs() < f64::EPSILON);
}

#[test]
fn emitted_trace_schema_json_roundtrip() {
    let v = EmittedTraceSchema::sample_fixture();
    assert_eq!(v.steps.len(), v.horizon_n as usize);
    let s = serde_json::to_string(&v).expect("serialize");
    let back: EmittedTraceSchema = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(back, v);
    assert!(s.contains("umst.emitted_trace.v1"));
}

#[test]
fn prototype_calibration_constants_match_lean() {
    assert!((PROTOTYPE_EPS_MI_STEP - 1.0 / 10_000.0).abs() < f64::EPSILON);
    assert!((PROTOTYPE_EPS_COST_STEP - 1.0 / 10_000.0).abs() < f64::EPSILON);
    assert!((prototype_eps_mi_agg(3) - 3.0 / 10_000.0).abs() < f64::EPSILON);
    assert!((prototype_eps_cost_agg(3) - 3.0 / 10_000.0).abs() < f64::EPSILON);
}

#[test]
fn emitted_trace_well_formed_on_sample_fixture() {
    let v = EmittedTraceSchema::sample_fixture();
    v.check_emitted_trace_well_formed().expect("well-formed");
}

#[test]
fn prototype_calibration_envelope_bounds_cases() {
    let ok = EmittedTraceSchema::sample_calibration_envelope_fixture();
    assert!(ok.within_prototype_calibration_bounds());
    ok.check_prototype_calibration_bounds()
        .expect("envelope fixture inside epsMIAgg/epsCostAgg");
    let bad = EmittedTraceSchema::sample_calibration_envelope_violation_fixture();
    assert!(!bad.within_prototype_calibration_bounds());
    assert!(matches!(
        bad.check_prototype_calibration_bounds(),
        Err(PrototypeCalibrationBoundsError::AggregateMiExceeds { .. })
            | Err(PrototypeCalibrationBoundsError::AggregateCostExceeds { .. })
    ));
}

#[test]
fn well_formed_fixture_may_exceed_prototype_aggregate_envelope() {
    let trace = EmittedTraceSchema::sample_fixture();
    trace
        .check_emitted_trace_well_formed()
        .expect("per-step EmittedTraceWellFormed");
    assert!(
        !trace.within_prototype_calibration_bounds(),
        "catalog-well-formed rollout sums can exceed prototype epsMIAgg (orthogonal morphisms)"
    );
}

#[test]
fn sample_fixture_respects_emitted_trace_well_formed_bounds() {
    let trace = EmittedTraceSchema::sample_fixture();
    trace
        .check_emitted_trace_well_formed()
        .expect("sample_fixture must satisfy Lean EmittedTraceWellFormed bounds");
    for step in &trace.steps {
        step.check_emitted_trace_well_formed(trace.temperature_t)
            .expect("each step well-formed");
    }
}

#[test]
fn emitted_step_record_rejects_mi_above_log_two() {
    let bad = EmittedStepRecord::new(LN_2 + 1e-6, 1.0e-21);
    assert_eq!(
        bad.check_emitted_trace_well_formed(300.0),
        Err(EmittedStepWellFormedError::StepMiExceedsLog2)
    );
}

#[test]
fn emitted_step_record_rejects_negative_mi() {
    let bad = EmittedStepRecord::new(-0.01, 1.0e-21);
    assert_eq!(
        bad.check_emitted_trace_well_formed(300.0),
        Err(EmittedStepWellFormedError::StepMiNegative)
    );
}

#[test]
fn emitted_step_record_rejects_cost_above_landauer_bit_energy() {
    let t = 300.0;
    let cap = landauer_bit_energy_joules(t);
    let bad = EmittedStepRecord::new(0.1, cap * 1.01);
    assert_eq!(
        bad.check_emitted_trace_well_formed(t),
        Err(EmittedStepWellFormedError::StepCostExceedsLandauer)
    );
}

#[test]
fn emitted_step_record_rejects_confidence_out_of_unit_interval() {
    let mut bad = EmittedStepRecord::new(0.1, 1.0e-21);
    bad.confidence = 1.5;
    assert_eq!(
        bad.check_emitted_trace_well_formed(300.0),
        Err(EmittedStepWellFormedError::ConfidenceOutOfRange)
    );
}

#[test]
fn emitted_trace_schema_rejects_horizon_step_count_mismatch() {
    let trace = EmittedTraceSchema::new(3, 300.0, vec![EmittedStepRecord::new(0.1, 1.0e-21)]);
    assert_eq!(
        trace.check_emitted_trace_well_formed(),
        Err(EmittedTraceWellFormedError::HorizonStepCountMismatch {
            horizon_n: 3,
            step_count: 1,
        })
    );
}

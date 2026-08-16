// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Serde DTOs for cross-language bridges (ROS 2, MCP) — **no runtime ROS dependency**.
//!
//! Enable **`serde`** for `Serialize`/`Deserialize` on gated types; enable **`ros2-contract`**
//! to compile this module in dependent builds.

pub mod contract;
pub mod epistemic_trace;
#[cfg(feature = "trace-calibration")]
pub mod trace_calibration;

#[cfg(feature = "trace-calibration")]
pub use epistemic_trace::prototype_eta_from_trace;
pub use epistemic_trace::{
    landauer_bit_energy_joules, prototype_eps_cost_agg, prototype_eps_mi_agg, EmittedStepRecord,
    EmittedStepWellFormedError, EmittedTraceSchema, EmittedTraceWellFormedError,
    PrototypeCalibrationBoundsError, PROTOTYPE_EPS_COST_STEP, PROTOTYPE_EPS_MI_STEP,
};
#[cfg(feature = "trace-calibration")]
pub use trace_calibration::{
    calibrate_eta_bound_from_steps, calibrate_eta_bound_from_trace, step_mi_excess_over_catalog,
    step_mi_within_catalog, TraceCalibrationReport, CATALOG_STEP_MI_UPPER_NAT,
};

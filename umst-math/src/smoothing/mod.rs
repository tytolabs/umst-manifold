// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// SPDX-FileSource: tytolabs/umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914 (vendor discipline — §0.6 ZCD)
// Adapted: `MetricSmoother` + re-exports — §14bis.e-TUI-7
//
//! Per-metric scalar smoothers: 1D Kalman (port) + scalar 1D Joseph EKF (reduced from upstream 3D EKF).

mod ekf;
mod kalman;

/// THEOREM-BOUND: cockpit metric smoother (Λ: pure per step; no hidden global state in `update*`)
pub use ekf::{EkfSmoother, EkfState, ScalarEkf1D};
/// THEOREM-BOUND: linear 1D Kalman
pub use kalman::{KalmanFilter1D, KalmanSmoother};

/// MEASUREMENT: one recursive cockpit metric — **EKF** / **Kalman** / **none** (identity on value)
pub trait MetricSmoother: Send {
    /// MEASUREMENT: one step with the RED fixture default of **1.0** ms; ε-bisim
    fn update(&mut self, raw: f64) -> f64;
    /// MEASUREMENT: one step with explicit inter-sample time (ms)
    fn update_with_step_ms(&mut self, raw: f64, step_ms: f64) -> f64;
    /// THEOREM-BOUND: filtered value after the last `update*`
    fn current(&self) -> f64;
    /// THEOREM-BOUND: filter variance (≥ 0, clamped)
    fn variance(&self) -> f64;
    /// MEASUREMENT: return to `new` / `from_env` initial
    fn reset(&mut self);
}

/// ZCI-EXEMPT: default initial when no host measurement
fn default_initial() -> f64 {
    0.0
}

/// CONSTANT-BOUND: `UMST_COCKPIT_SMOOTHING=none` — identity
pub struct NoneSmoother {
    v: f64,
}

impl NoneSmoother {
    /// ZCI-EXEMPT: identity smoother
    pub fn new() -> Self {
        Self {
            v: default_initial(),
        }
    }
}

impl Default for NoneSmoother {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricSmoother for NoneSmoother {
    fn update(&mut self, raw: f64) -> f64 {
        self.v = raw;
        raw
    }

    fn update_with_step_ms(&mut self, raw: f64, _step_ms: f64) -> f64 {
        self.update(raw)
    }

    fn current(&self) -> f64 {
        self.v
    }

    fn variance(&self) -> f64 {
        0.0
    }

    fn reset(&mut self) {
        self.v = default_initial();
    }
}

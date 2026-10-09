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
    /// THEOREM-BOUND: filtered value after the last `update*`; `None` while the smoother holds no
    /// value (an identity smoother before its first sample)
    fn current(&self) -> Option<f64>;
    /// THEOREM-BOUND: filter variance (≥ 0, clamped)
    fn variance(&self) -> f64;
    /// MEASUREMENT: return to `new` / `from_env` initial
    fn reset(&mut self);
}

/// CONSTANT-BOUND: `UMST_COCKPIT_SMOOTHING=none` — identity; holds the last sample, none before the first
pub struct NoneSmoother {
    v: Option<f64>,
}

impl NoneSmoother {
    /// ZCI-EXEMPT: identity smoother with no sample yet
    pub fn new() -> Self {
        Self { v: None }
    }
}

impl Default for NoneSmoother {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricSmoother for NoneSmoother {
    fn update(&mut self, raw: f64) -> f64 {
        self.v = Some(raw);
        raw
    }

    fn update_with_step_ms(&mut self, raw: f64, _step_ms: f64) -> f64 {
        self.update(raw)
    }

    fn current(&self) -> Option<f64> {
        self.v
    }

    fn variance(&self) -> f64 {
        // The identity smoother passes its input through, so its filter variance is zero.
        0.0
    }

    fn reset(&mut self) {
        self.v = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{MetricSmoother, NoneSmoother};

    #[test]
    fn identity_smoother_holds_no_value_before_its_first_sample() {
        let mut s = NoneSmoother::new();
        assert_eq!(s.current(), None);
        assert_eq!(s.update(3.5), 3.5);
        assert_eq!(s.current(), Some(3.5));
        assert_eq!(s.update_with_step_ms(-2.0, 7.0), -2.0);
        assert_eq!(s.current(), Some(-2.0));
        s.reset();
        assert_eq!(s.current(), None);
    }
}

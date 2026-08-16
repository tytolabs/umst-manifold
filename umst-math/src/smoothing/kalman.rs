// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// SPDX-FileSource: tytolabs/umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914
//   prototype/src/rust/core/src/math/kalman.rs
// Adapted: scalar 1D Kalman filter; cockpit MetricSmoother wrapper — §14bis.e-TUI-7
//
//! 1D Kalman filter (ported from upstream) with variance clamp + [`super::MetricSmoother`] bridge.

use super::MetricSmoother;

/// A minimal 1D Kalman filter over power / scalar telemetry.
///
/// State: estimated value `x`; process noise `q` and measurement noise `r` per upstream defaults.
/// THEOREM-BOUND: linear-Gaussian Kalman step (scalar observation)
pub struct KalmanFilter1D {
    /// THEOREM-BOUND: estimated state
    pub x: f64,
    /// THEOREM-BOUND: state variance (clamped `≥ 0` on read)
    pub p: f64,
    /// CONSTANT-BOUND: `umst_cockpit_smoothing_default` path — process noise scale (Q)
    pub q: f64,
    /// CONSTANT-BOUND: measurement noise scale (R)
    pub r: f64,
    x0: f64,
}

impl KalmanFilter1D {
    /// THEOREM-BOUND: new filter with `x = initial` and high initial uncertainty
    pub fn new(initial: f64) -> Self {
        Self::new_with_params(initial, 10.0, 500.0)
    }

    /// THEOREM-BOUND: TUI-7b per-metric (Q, R); positive covariances (see [`KalmanSmoother::new_with_q_r`])
    pub fn new_with_params(initial: f64, q: f64, r: f64) -> Self {
        debug_assert!(q.is_finite() && r.is_finite() && q > 0.0 && r > 0.0);
        Self {
            x: initial,
            p: 1000.0,
            q,
            r,
            x0: initial,
        }
    }

    fn predict(&mut self, dt_ms: f64) {
        let dt = if dt_ms.is_finite() && dt_ms > 0.0 {
            dt_ms
        } else {
            1.0
        };
        self.p += self.q * dt;
    }

    /// THEOREM-BOUND: one scalar measurement update with time step `dt_ms` (ms).
    pub fn update(&mut self, z: f64, dt_ms: f64) -> f64 {
        self.predict(dt_ms);
        let k = self.p / (self.p + self.r);
        self.x += k * (z - self.x);
        self.p *= 1.0 - k;
        debug_assert!(self.p.is_finite() && self.p >= 0.0);
        self.p = self.p.max(0.0);
        self.x
    }

    /// THEOREM-BOUND: current filtered estimate
    pub fn estimate(&self) -> f64 {
        self.x
    }
}

/// CONSTANT-BOUND: `KalmanFilter1D` with [`MetricSmoother`] (fixed `1.0` ms per `update` for RED fixtures)
pub struct KalmanSmoother {
    inner: KalmanFilter1D,
}

impl KalmanSmoother {
    /// CONSTANT-BOUND: new smoother from initial state (defaults match upstream)
    pub fn new(initial: f64) -> Self {
        Self {
            inner: KalmanFilter1D::new(initial),
        }
    }

    /// THEOREM-BOUND: TUI-7b — explicit (Q, R) from `umst_math::constants::registry::REGISTRY` tuning rows
    pub fn new_with_q_r(initial: f64, q: f64, r: f64) -> Self {
        debug_assert!(
            q > 0.0 && r > 0.0,
            "TUI-7b ZCI: (Q, R) must be strictly positive"
        );
        Self {
            inner: KalmanFilter1D::new_with_params(initial, q, r),
        }
    }
}

impl MetricSmoother for KalmanSmoother {
    fn update(&mut self, raw: f64) -> f64 {
        self.update_with_step_ms(raw, 1.0)
    }

    /// MEASUREMENT: explicit `dt` (RED uses `1.0` ms; cockpit uses hub `sample_period_ms`)
    fn update_with_step_ms(&mut self, raw: f64, step_ms: f64) -> f64 {
        self.inner.update(raw, step_ms)
    }

    fn current(&self) -> f64 {
        self.inner.estimate()
    }

    fn variance(&self) -> f64 {
        self.inner.p.max(0.0)
    }

    fn reset(&mut self) {
        self.inner.x = self.inner.x0;
        self.inner.p = 1000.0;
    }
}

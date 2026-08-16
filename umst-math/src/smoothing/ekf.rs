// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// SPDX-FileSource: tytolabs/umst-prototype-2a@9c0434d3ebade8f697bbd402bb080ea00da76914
//   prototype/src/rust/core/src/math/ekf.rs
// Adapted: reduced scalar 1D EKF, identity h(x)=x, Joseph covariance update; cockpit §14bis.e-TUI-7
//
//! Scalar 1D extended Kalman (linear observation) with the **Joseph** covariance form from
//! the upstream 3D EKF update, specialised to `H = 1` for cockpit per-metric smoothing.

use super::MetricSmoother;

/// 1D filter state: estimate + variance.
/// THEOREM-BOUND: Kalman filter with explicit variance trace
pub struct EkfState {
    /// THEOREM-BOUND: value estimate
    pub value: f64,
    /// THEOREM-BOUND: state variance (runtime `≥ 0`)
    pub variance: f64,
}

impl EkfState {
    /// THEOREM-BOUND: new state with `variance ≥ 0` (clamped in debug)
    pub fn new(value: f64, variance: f64) -> Self {
        let v = variance.max(0.0);
        debug_assert!(v.is_finite());
        Self { value, variance: v }
    }
}

/// Scalar 1D EKF with identity transition `F=1` and `h(x)=x` (hence `H=1`).
/// Joseph `P` update: `(1-KH) P (1-KH)ᵀ + K R Kᵀ` (scalar form).
/// THEOREM-BOUND: Joseph-form covariance update; distinguishable from the classic 1D Kalman step
pub struct ScalarEkf1D {
    state: EkfState,
    /// CONSTANT-BOUND: `umst_cockpit_smoothing_default` (process noise, uniform slice)
    pub q: f64,
    /// CONSTANT-BOUND: measurement noise
    pub r: f64,
    x0: f64,
}

impl ScalarEkf1D {
    /// THEOREM-BOUND: new filter, initial `x0`, `p0=1000`, same default `q`,`r` as 1D Kalman upstream
    pub fn new(x0: f64) -> Self {
        Self::new_with_params(x0, 10.0, 500.0)
    }

    /// THEOREM-BOUND: TUI-7b — per-metric process / measurement variances (positive; REGISTRY-sourced in cockpit)
    pub fn new_with_params(x0: f64, q: f64, r: f64) -> Self {
        debug_assert!(q.is_finite() && r.is_finite() && q > 0.0 && r > 0.0);
        Self {
            state: EkfState {
                value: x0,
                variance: 1000.0,
            },
            q,
            r,
            x0,
        }
    }

    /// THEOREM-BOUND: one predict + linearised update (identity observation) in milliseconds
    pub fn update(&mut self, z: f64, dt_ms: f64) -> f64 {
        let dt = if dt_ms.is_finite() && dt_ms > 0.0 {
            dt_ms
        } else {
            1.0
        };
        // Predict (identity dynamics + process noise, same as upstream 1D Kalman shape)
        self.state.variance += self.q * dt;
        self.state.variance = self.state.variance.max(0.0);

        let s = self.state.variance + self.r;
        let k = if s > 0.0 {
            self.state.variance / s
        } else {
            0.0
        };
        // Innovation
        self.state.value += k * (z - self.state.value);

        // Joseph: P' = (1 - K)² P + K² R
        let one_m_k = 1.0 - k;
        self.state.variance = one_m_k * one_m_k * self.state.variance + k * k * self.r;
        self.state.variance = self.state.variance.max(0.0);
        debug_assert!(self.state.variance.is_finite() && self.state.variance >= 0.0);
        self.state.value
    }

    /// THEOREM-BOUND: current estimate
    pub fn estimate(&self) -> f64 {
        self.state.value
    }
}

/// CONSTANT-BOUND: [`ScalarEkf1D`] with [`MetricSmoother`]; `update` uses 1.0 ms for RED ε-bisim
pub struct EkfSmoother {
    inner: ScalarEkf1D,
}

impl EkfSmoother {
    /// CONSTANT-BOUND: new
    pub fn new(initial: f64) -> Self {
        Self {
            inner: ScalarEkf1D::new(initial),
        }
    }

    /// THEOREM-BOUND: TUI-7b — construct with explicit (Q, R) from `umst_math::constants::registry::REGISTRY` per-metric rows
    pub fn new_with_q_r(initial: f64, q: f64, r: f64) -> Self {
        debug_assert!(
            q > 0.0 && r > 0.0,
            "TUI-7b ZCI: (Q, R) must be strictly positive"
        );
        Self {
            inner: ScalarEkf1D::new_with_params(initial, q, r),
        }
    }
}

impl MetricSmoother for EkfSmoother {
    /// MEASUREMENT: 1.0 ms; RED `smoothing_ekf_e_bisim` fixtures
    fn update(&mut self, raw: f64) -> f64 {
        self.update_with_step_ms(raw, 1.0)
    }

    /// MEASUREMENT: inter-sample time (ms); cockpit `sample_period_ms`
    fn update_with_step_ms(&mut self, raw: f64, step_ms: f64) -> f64 {
        self.inner.update(raw, step_ms)
    }

    fn current(&self) -> f64 {
        self.inner.estimate()
    }

    fn variance(&self) -> f64 {
        self.inner.state.variance.max(0.0)
    }

    fn reset(&mut self) {
        self.inner.state.value = self.inner.x0;
        self.inner.state.variance = 1000.0;
    }
}

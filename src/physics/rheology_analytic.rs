// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Closed-form **plane Poiseuille** flow between parallel plates (Newtonian and Bingham).
//!
//! ## Audit memo (Track E)
//! - **Steady references:** Closed-form Buckingham / Newtonian profiles below; **regularized Bingham**
//!   steady 1D quadrature matches [`crate::physics::solvers::BinghamFlowSolver`]’s \(\eta\) law.
//!   Transient Chorin startup on the graph is **not** claimed to match either until BCs / Poisson harden.
//! - **NaN / domain:** `mu <= 0` returns `f32::NAN`; `g` is clamped non-negative in the yield-stress
//!   branch so plug half-width stays finite.
//!
//! # Coordinate system
//! - \(x\): streamwise direction; favorable pressure gradient \(g = -\partial p/\partial x\) [Pa/m] (positive when \(p\) decreases in \(+x\)).
//! - \(y\): wall-normal from the **mid-plane** between plates at \(y = \pm H/2\).
//!
//! # Bingham (Buckingham 1921)
//! Let \(a = H/2\). Define the plug half-width \(y_p = \tau_0 / g\). If \(\tau_0 \ge g\,a\), the wall shear stress never exceeds yield and the steady profile is **no motion** with no-slip walls.
//!
//! Otherwise, for \(|y| \le y_p\) (unyielded plug), \(u(y) = u(y_p)\). For \(|y| > y_p\),
//! \[
//!   u(y) = \frac{g}{2\mu}\,(a^2 - y^2) - \frac{\tau_0\,(a - |y|)}{\mu}.
//! \]
//!
//! Newtonian limit \(\tau_0 = 0\): \(u(y) = \dfrac{g}{2\mu}\,(a^2 - y^2)\); centreline \(u(0) = g H^2 / (8\mu)\).
//!
//! # Regularized Bingham (graph-consistent)
//! [`BinghamFlowSolver`](crate::physics::solvers::BinghamFlowSolver) uses
//! \(\eta = \mu + \tau_0/(|\dot\gamma|+\varepsilon)\) with \(\tau_{xy}=\eta\,\dot\gamma\),
//! \(\dot\gamma=\partial u/\partial y\). Steady developed flow gives \(\tau_{xy}(y)=-g\,y\) (mid-plane symmetry),
//! hence \(|\tau|=g\,|y|\) and, with \(x=|\dot\gamma|\),
//! \[
//!   \mu\,x + \frac{\tau_0\,x}{x+\varepsilon} = g\,|y|.
//! \]
//! Quadrature on \(u(|y|)=\int_{|y|}^{a} x(g\xi)\,\mathrm d\xi\) with \(a=H/2\) yields the **steady 1D reference**
//! ([`plane_regularized_bingham_poiseuille_u_centreline`], [`plane_regularized_bingham_poiseuille_u_sample`]).
//! Use [`RHEOLOGY_FLOW_BINGHAM_EPS`] for \(\varepsilon\) parity with `rheology_flow.rs`.

/// Regularization \(\varepsilon\) \[1/s\]; kept identical to [`crate::physics::solvers::BinghamFlowSolver`]'s ε (`rheology_flow.rs`).
pub const RHEOLOGY_FLOW_BINGHAM_EPS: f32 = 1e-5;

/// Axial velocity \[m/s\] at wall-normal position `y` \[m\] from the mid-plane.
///
/// `g` is the streamwise pressure-drop magnitude \(g=-\partial p/\partial x\) \[Pa/m\].
/// `h` is the full plate spacing \[m\]. `mu` is dynamic viscosity \[Pa·s\], `tau0` yield stress \[Pa\].
pub fn plane_bingham_poiseuille_u(y: f32, g: f32, h: f32, mu: f32, tau0: f32) -> f32 {
    if mu <= 0.0 || mu.is_nan() {
        return f32::NAN;
    }
    let half = 0.5 * h;
    let g_pos = g.max(0.0);
    if tau0 <= 0.0 {
        return (g_pos / (2.0 * mu)) * (half * half - y * y);
    }
    // Below yield at the wall: no steady flow with no-slip.
    if tau0 >= g_pos * half {
        return 0.0;
    }
    let y_p = tau0 / g_pos.max(1e-30);
    let ay = y.abs();
    let u_at_yp = (g_pos / (2.0 * mu)) * (half * half - y_p * y_p) - tau0 * (half - y_p) / mu;
    if ay <= y_p {
        return u_at_yp;
    }
    (g_pos / (2.0 * mu)) * (half * half - y * y) - tau0 * (half - ay) / mu
}

/// Half-width of the unyielded plug region from the mid-plane, \(y_p = \tau_0 / g\) \[m\].
pub fn plane_bingham_plug_half_width(tau0: f32, g: f32) -> f32 {
    tau0 / g.max(1e-30)
}

// --- Steady regularized Bingham 1D reference (f64 kernel, f32 API) --------------------------------

/// Solve \(\mu x + \tau_0 x/(x+\varepsilon) = \tau_\mathrm{target}\) for shear-rate magnitude \(x=|\dot\gamma|\ge 0\) when \(\tau_\mathrm{target}\ge 0\).
///
/// For \(\varepsilon=0\) and \(\tau_0>0\), uses the ideal limit \(x=\max(0,(\tau_\mathrm{target}-\tau_0)/\mu)\).
fn shear_rate_mag_from_stress_balance(tau_target: f64, mu: f64, tau0: f64, eps: f64) -> f64 {
    if !tau_target.is_finite() || !mu.is_finite() || !tau0.is_finite() || !eps.is_finite() {
        return f64::NAN;
    }
    if tau_target < 0.0 {
        return f64::NAN;
    }
    if mu <= 0.0 {
        return f64::NAN;
    }
    if tau_target == 0.0 {
        return 0.0;
    }
    if tau0 <= 0.0 {
        return tau_target / mu;
    }
    if eps < 0.0 {
        return f64::NAN;
    }
    if eps == 0.0 {
        return ((tau_target - tau0) / mu).max(0.0);
    }
    let b = mu * eps + tau0 - tau_target;
    let disc = b.mul_add(b, 4.0 * mu * tau_target * eps);
    debug_assert!(disc >= 0.0);
    (-b + disc.sqrt()) / (2.0 * mu)
}

fn trapezoid_integral<F>(f: F, lo: f64, hi: f64, n_seg: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    debug_assert!(n_seg >= 1);
    let n = n_seg.saturating_add(1).max(2);
    let h = (hi - lo) / (n - 1) as f64;
    let mut sum = 0.5 * (f(lo) + f(hi));
    for i in 1..n - 1 {
        sum += f(lo + (i as f64) * h);
    }
    h * sum
}

/// Mid-plane velocity \(u(0)\) \[m/s\] for steady regularized Bingham Poiseuille (trapezoid quadrature on half-channel).
///
/// `n_quad` is the number of **segments** on \([0,H/2]\) (use ≥ 8 for coarse checks). Invalid parameters yield `f32::NAN`.
pub fn plane_regularized_bingham_poiseuille_u_centreline(
    g: f32,
    h: f32,
    mu: f32,
    tau0: f32,
    eps: f32,
    n_quad: usize,
) -> f32 {
    let u = plane_regularized_bingham_poiseuille_u_sample_internal(
        0.0_f64,
        g as f64,
        h as f64,
        mu as f64,
        tau0 as f64,
        eps as f64,
        n_quad,
    );
    if u.is_nan() {
        f32::NAN
    } else {
        u as f32
    }
}

/// Sample \(u(y)\) \[m/s\] with \(y\) \[m\] from mid-plane; profile is even in \(y\).
///
/// Uses dense-grid integration \(u(|y|)=\int_{|y|}^{H/2} x(g\xi)\,\mathrm d\xi\).
pub fn plane_regularized_bingham_poiseuille_u_sample(
    y: f32,
    g: f32,
    h: f32,
    mu: f32,
    tau0: f32,
    eps: f32,
    n_quad: usize,
) -> f32 {
    let u = plane_regularized_bingham_poiseuille_u_sample_internal(
        y as f64,
        g as f64,
        h as f64,
        mu as f64,
        tau0 as f64,
        eps as f64,
        n_quad,
    );
    if u.is_nan() {
        f32::NAN
    } else {
        u as f32
    }
}

fn plane_regularized_bingham_poiseuille_u_sample_internal(
    y: f64,
    g: f64,
    h: f64,
    mu: f64,
    tau0: f64,
    eps: f64,
    n_quad: usize,
) -> f64 {
    if mu <= 0.0
        || h <= 0.0
        || eps < 0.0
        || !y.is_finite()
        || !g.is_finite()
        || !h.is_finite()
        || !eps.is_finite()
    {
        return f64::NAN;
    }
    let n_seg = n_quad.max(8);
    let half = 0.5 * h;
    let ay = y.abs();
    if ay > half + 1e-15 {
        return f64::NAN;
    }
    let g_pos = g.max(0.0);
    let integrand = |xi: f64| shear_rate_mag_from_stress_balance(g_pos * xi, mu, tau0, eps);
    trapezoid_integral(integrand, ay, half, n_seg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtonian_centreline_matches_gh2_over_8mu() {
        let g = 1000.0_f32;
        let h = 0.05_f32;
        let mu = 50.0_f32;
        let expected = g * h * h / (8.0 * mu);
        let u0 = plane_bingham_poiseuille_u(0.0, g, h, mu, 0.0);
        assert!((u0 - expected).abs() < 1e-4 * expected.abs().max(1.0));
    }

    #[test]
    fn regularized_reference_newtonian_centreline_matches_gh2_over_8mu() {
        let g = 1000.0_f32;
        let h = 0.05_f32;
        let mu = 50.0_f32;
        let expected = g * h * h / (8.0 * mu);
        let got = plane_regularized_bingham_poiseuille_u_centreline(
            g,
            h,
            mu,
            0.0,
            RHEOLOGY_FLOW_BINGHAM_EPS,
            128,
        );
        assert!((got - expected).abs() < 1e-5 * expected.abs().max(1.0));
    }

    /// \(\mu x + \tau_0 x/(x+\varepsilon) \approx g |y|\) with \(x \approx |\mathrm du/\mathrm dy|\) from FD.
    #[test]
    fn regularized_profile_balances_stress_fd() {
        let g = 800.0_f64;
        let h = 0.04_f64;
        let mu = 30.0_f64;
        let tau0 = 10.0_f64;
        let eps = 1e-4_f64;
        let n = 1024_usize;
        let y = 0.012_f64;
        let delta = 1e-7_f64;
        let u_p = plane_regularized_bingham_poiseuille_u_sample_internal(
            y + delta,
            g,
            h,
            mu,
            tau0,
            eps,
            n,
        );
        let u_m = plane_regularized_bingham_poiseuille_u_sample_internal(
            y - delta,
            g,
            h,
            mu,
            tau0,
            eps,
            n,
        );
        let dudy = (u_p - u_m) / (2.0 * delta);
        let x_mag = dudy.abs();
        let lhs = mu * x_mag + tau0 * x_mag / (x_mag + eps);
        let rhs = g * y.abs();
        assert!(
            (lhs - rhs).abs() < 5e-5 * rhs.max(1e-9),
            "stress balance mismatch: lhs={lhs} rhs={rhs}"
        );
    }
}

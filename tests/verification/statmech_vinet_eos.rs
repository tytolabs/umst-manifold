// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Track H — **Vinet equation of state** scalar checks (same closed form as clinker calibration).
//!
//! formal_citation: Vinet et al., *J. Phys. C* **19** (1986) L467.
//!
//! **Verified (stable lane):** closed-form Vinet isothermal pressure vs volume (this file).
//! **Open (research):** `solvers::statistical_mechanics::upscale_potentials` — LJ→continuum
//! bridge placeholder only; not exercised here.

/// Isothermal Vinet pressure (GPa) from \(V/V_0\) and ambient \(K_0\), \(K_0'\).
#[must_use]
fn vinet_pressure_gpa(v0: f32, k0_gpa: f32, k0_prime: f32, v_per_fu: f32) -> f32 {
    let v0 = v0.max(1e-6);
    let v = v_per_fu.max(1e-12);
    let x = (v / v0).cbrt().max(1e-9);
    let eta = 1.5 * (k0_prime - 1.0);
    3.0 * k0_gpa * ((1.0 - x) / (x * x)) * (eta * (1.0 - x)).exp()
}

#[test]
fn vinet_pressure_vanishes_at_reference_volume() {
    let k0 = 105.0_f32;
    let k0p = 4.0_f32;
    let v0 = 364.2_f32;
    let p = vinet_pressure_gpa(v0, k0, k0p, v0);
    assert!(p.abs() < 0.05, "expected P≈0 at V=V0, got {p}");
}

#[test]
fn vinet_pressure_increases_under_compression() {
    let k0 = 105.0_f32;
    let k0p = 4.0_f32;
    let v0 = 364.2_f32;
    let p0 = vinet_pressure_gpa(v0, k0, k0p, v0);
    let pc = vinet_pressure_gpa(v0, k0, k0p, v0 * 0.96);
    assert!(
        pc > p0,
        "compression should increase pressure: p0={p0} pc={pc}"
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Johnson–Zollweg–Gubbins (1993) Lennard-Jones equation of state — **f64 reference surface only**.
//!
//! This is a faithful port of the reduced-variable implementation in NIST **teqp**
//! (`teqp::mie::lennardjones::Johnson::LJ126Johnson1993`, `johnson.hpp` on `usnistgov/teqp`),
//! including the published MBWR-style coefficient vector and `γ = 3` used there.
//!
//! formal_citation: J. K. Johnson, J. A. Zollweg, K. E. Gubbins, *The Lennard-Jones equation of
//! state revisited*, **Mol. Phys.** **78**, 591–618 (1993). doi:10.1080/00268979300100411
//!
//! ## Reduced conventions (LJ 12–6, simulation units)
//!
//! - `T* = k_B T / ε`
//! - `ρ* = ρ σ³` with `ρ` the **number density** `N/V`
//! - `P* = P σ³ / ε`
//!
//! The teqp class reports `R = 1` and `alphar(T*, ρ*) = A_MBWR(ρ*, T*) / T*` where `A_MBWR` is the
//! inner sum documented in Johnson et al. (1993). Compressibility follows the teqp Helmholtz
//! residual route (`Z = 1 + ρ* ∂α^r/∂ρ*|_T*`), so **`P* = Z ρ* T*`**.
//!
//! ## Scope
//!
//! - **Single-phase homogeneous fluid** checks should pick states outside the vapor–liquid dome
//!   (e.g. supercritical `T*` well above `T_c* ≈ 1.32`) unless coexistence is explicitly intended.
//! - This module is **not** wired into [`super::statistical_mechanics::upscale_potentials`]: the
//!   bridge tensors carry only \((\varepsilon, \sigma)\), while this EOS needs \((\rho^*, T^*)\) for
//!   \(K^*\). Feature **`statistical-mechanics-johnson-reference`** re-exports a scalar entry point
//!   on `statistical_mechanics` for parity checks;
//!   [`super::statistical_mechanics::upscale_potentials`] remains a **partial** Burn placeholder until a stateful API lands.
//!   Convenience [`bulk_modulus_from_lj_state_johnson1993`] returns reduced `K*` for comparisons.
//! - **Protocol:** the JZG (1993) analytic surface — not truncated/shifted MD with a particular
//!   `r_c`; do not equate to cut-and-shifted simulation columns without an explicit mapping layer.

const GAMMA: f64 = 3.0;

/// Coefficient vector `x[1..32]` from Johnson et al. (1993) as embedded in teqp `LJ126Johnson1993`.
#[allow(clippy::excessive_precision)]
const X: [f64; 33] = [
    0.0,
    0.8623085097507421,
    2.976218765822098,
    -8.402230115796038,
    0.1054136629203555,
    -0.8564583828174598,
    1.582759470107601,
    0.7639421948305453,
    1.753173414312048,
    2.798291772190376e3,
    -4.8394220260857657e-2,
    0.9963265197721935,
    -3.698000291272493e1,
    2.084012299434647e1,
    8.305402124717285e1,
    -9.574799715203068e2,
    -1.477746229234994e2,
    6.398607852471505e1,
    1.603993673294834e1,
    6.805916615864377e1,
    -2.791293578795945e3,
    -6.245128304568454,
    -8.116836104958410e3,
    1.488735559561229e1,
    -1.059346754655084e4,
    -1.131607632802822e2,
    -8.867771540418822e3,
    -3.986982844450543e1,
    -4.689270299917261e3,
    2.593535277438717e2,
    -2.694523589434903e3,
    -7.218487631550215e2,
    1.721802063863269e2,
];

#[inline]
fn pow2(t: f64) -> f64 {
    t * t
}

#[inline]
fn pow3(t: f64) -> f64 {
    pow2(t) * t
}

#[inline]
fn pow4(t: f64) -> f64 {
    pow2(t) * pow2(t)
}

fn get_ai(i: i32, t_star: f64) -> f64 {
    match i {
        1 => X[1] * t_star + X[2] * t_star.sqrt() + X[3] + X[4] / t_star + X[5] / pow2(t_star),
        2 => X[6] * t_star + X[7] + X[8] / t_star + X[9] / pow2(t_star),
        3 => X[10] * t_star + X[11] + X[12] / t_star,
        4 => X[13],
        5 => X[14] / t_star + X[15] / pow2(t_star),
        6 => X[16] / t_star,
        7 => X[17] / t_star + X[18] / pow2(t_star),
        8 => X[19] / pow2(t_star),
        _ => f64::NAN,
    }
}

fn get_bi(i: i32, t_star: f64) -> f64 {
    match i {
        1 => X[20] / pow2(t_star) + X[21] / pow3(t_star),
        2 => X[22] / pow2(t_star) + X[23] / pow4(t_star),
        3 => X[24] / pow2(t_star) + X[25] / pow3(t_star),
        4 => X[26] / pow2(t_star) + X[27] / pow4(t_star),
        5 => X[28] / pow2(t_star) + X[29] / pow3(t_star),
        6 => X[30] / pow2(t_star) + X[31] / pow3(t_star) + X[32] / pow4(t_star),
        _ => f64::NAN,
    }
}

fn get_gi(i: i32, f: f64, rho_star: f64) -> f64 {
    if i == 1 {
        (1.0 - f) / (2.0 * GAMMA)
    } else {
        let im1 = (i - 1) as f64;
        let prev = get_gi(i - 1, f, rho_star);
        -(f * rho_star.powf(2.0 * im1) - 2.0 * im1 * prev) / (2.0 * GAMMA)
    }
}

/// Inner MBWR sum `A_inner(ρ*, T*)` from teqp `get_alphar` (before division by `T*`).
#[must_use]
pub fn johnson_lj1993_mbwr_inner(t_star: f64, rho_star: f64) -> f64 {
    let f = (-GAMMA * pow2(rho_star)).exp();
    let mut summer = 0.0_f64;
    for i in 1..=8 {
        summer += get_ai(i, t_star) * rho_star.powi(i) / f64::from(i);
    }
    for i in 1..=6 {
        summer += get_bi(i, t_star) * get_gi(i, f, rho_star);
    }
    summer
}

/// Dimensionless residual Helmholtz energy per particle `α^r` as teqp `alphar(T*, ρ*)`.
#[must_use]
pub fn johnson_lj1993_alphar(t_star: f64, rho_star: f64) -> f64 {
    johnson_lj1993_mbwr_inner(t_star, rho_star) / t_star
}

/// `∂α^r/∂ρ*` at fixed `T*` (central difference; stable for verification grids).
#[must_use]
pub fn johnson_lj1993_dalphar_drho(t_star: f64, rho_star: f64, h_rho: f64) -> f64 {
    let h = h_rho.max(1.0e-14);
    (johnson_lj1993_alphar(t_star, rho_star + h) - johnson_lj1993_alphar(t_star, rho_star - h))
        / (2.0 * h)
}

/// Compressibility factor `Z = P*/(ρ* T*) = 1 + ρ* ∂α^r/∂ρ*|_{T*}`.
#[must_use]
pub fn johnson_lj1993_compressibility_factor(t_star: f64, rho_star: f64, h_rho: f64) -> f64 {
    1.0 + rho_star * johnson_lj1993_dalphar_drho(t_star, rho_star, h_rho)
}

/// Reduced pressure `P* = Z ρ* T*` for the JZG (1993) / teqp `LJ126Johnson1993` branch.
#[must_use]
pub fn johnson_lj1993_pressure_reduced(t_star: f64, rho_star: f64, h_rho: f64) -> f64 {
    johnson_lj1993_compressibility_factor(t_star, rho_star, h_rho) * rho_star * t_star
}

/// Isothermal bulk modulus in reduced units,
/// `K* = ρ* (∂P*/∂ρ*)_{T*}` with `P*` from [`johnson_lj1993_pressure_reduced`].
///
/// `h_delta` is the step for the central difference in `ρ*` only. The derivative of `α^r` inside
/// `P*` uses a fixed internal step (`1e-7`) so outer `h_delta` refinements converge Richardson-style.
#[must_use]
pub fn johnson_lj1993_bulk_modulus_reduced_numerical(
    t_star: f64,
    rho_star: f64,
    h_delta: f64,
) -> f64 {
    const H_ALPH: f64 = 1.0e-7_f64;
    let h = h_delta.max(1.0e-14);
    let p_plus = johnson_lj1993_pressure_reduced(t_star, rho_star + h, H_ALPH);
    let p_minus = johnson_lj1993_pressure_reduced(t_star, rho_star - h, H_ALPH);
    rho_star * (p_plus - p_minus) / (2.0 * h)
}

/// Map reduced bulk modulus to `K_T` in the same energy/length units as `ε` and `σ`:
/// `K_T = (ε/σ³) K*`.
#[must_use]
pub fn bulk_modulus_from_reduced(k_star: f64, epsilon: f64, sigma: f64) -> f64 {
    let sigma_cubed = pow3(sigma);
    (epsilon / sigma_cubed) * k_star
}

/// Reduced isothermal bulk modulus `K* = ρ* (∂P*/∂ρ*)_{T*}` at `(ρ*, T*)` on the JZG (1993) surface.
///
/// Thin wrapper around [`johnson_lj1993_bulk_modulus_reduced_numerical`] with a fixed `ρ*` finite
/// difference step `1e-6` (matches `statmech_lj_johnson_eos_reference` supercritical checks). For
/// custom step sizes, call [`johnson_lj1993_bulk_modulus_reduced_numerical`] directly.
#[must_use]
pub fn bulk_modulus_from_lj_state_johnson1993(rho_star: f64, t_star: f64) -> f64 {
    const H_DELTA: f64 = 1.0e-6_f64;
    johnson_lj1993_bulk_modulus_reduced_numerical(t_star, rho_star, H_DELTA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphar_matches_python_teqp_port_at_sample_state() {
        let t = 2.0_f64;
        let rho = 0.4_f64;
        let a = johnson_lj1993_alphar(t, rho);
        assert!((a + 0.33527709335199357).abs() < 1.0e-12);
    }

    #[test]
    fn bulk_modulus_from_lj_state_matches_explicit_numerical_call() {
        let t_star = 2.0_f64;
        let rho_star = 0.35_f64;
        let h = 1.0e-6_f64;
        let k = bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
        let k_explicit = johnson_lj1993_bulk_modulus_reduced_numerical(t_star, rho_star, h);
        assert!((k - k_explicit).abs() < 1.0e-15);
    }
}

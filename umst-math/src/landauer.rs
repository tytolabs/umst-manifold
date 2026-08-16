// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Landauer cost scaling for diagonal erasure / extraction (session accounting).

use ordered_float::NotNan;

/// Boltzmann constant (J / K), CODATA 2018.
/// DOI: 10.5281/zenodo.19159660
pub const K_B: f64 = 1.380_649e-23;

/// Landauer bit energy **k_B T ln 2** [J] at temperature `T` [K].
///
/// Proof: `LandauerBound` / `idealResetErasure` family.
/// DOI: 10.5281/zenodo.19159660
pub fn landauer_bit_energy_joules(temperature_k: NotNan<f64>) -> NotNan<f64> {
    let t = temperature_k.into_inner();
    NotNan::new(K_B * t * std::f64::consts::LN_2).expect("positive energy")
}

/// Diagonal Landauer **cost proxy** in bits: **(1 − RCC) · H_bit** scale; here returns **1 − RCC**
/// as the dimensionless factor paired with [`landauer_bit_energy_joules`] in UMST Oracle v2.
///
/// Proof: `principle_of_maximal_information_collapse` linkage (PMIC + Landauer).
/// DOI: 10.5281/zenodo.19159660
pub fn landauer_cost_diagonal_bits(residual_coherence: NotNan<f64>) -> NotNan<f64> {
    let r = residual_coherence.into_inner().clamp(0.0, 1.0);
    NotNan::new(1.0 - r).expect("1-RCC")
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Shared physical constants. With feature `math-constants`, Landauer bit energy is SSOT from `umst-math`.

/// Landauer bit energy `k_B T ln 2` (joules).
///
/// With `math-constants`, delegates to [`umst_math::landauer::landauer_bit_energy_joules`].
/// Non-finite `temperature_k` yields NaN (no panic).
#[must_use]
pub fn landauer_bit_energy_joules(temperature_k: f64) -> f64 {
    #[cfg(feature = "math-constants")]
    {
        match ordered_float::NotNan::new(temperature_k) {
            Ok(t) => umst_math::landauer::landauer_bit_energy_joules(t).into_inner(),
            Err(_) => f64::NAN,
        }
    }
    #[cfg(not(feature = "math-constants"))]
    {
        const K_BOLTZMANN: f64 = 1.380649e-23;
        K_BOLTZMANN * temperature_k * std::f64::consts::LN_2
    }
}

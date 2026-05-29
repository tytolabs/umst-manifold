// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Bridges **volumetric** Clausius–Duhem surrogates into scalar joule proxies for [`crate::ai::cbf::ThermodynamicCBF`].

/// Approximate volumetric entropy production rate (Joules/step) given a nonnegative `d_int`
/// surrogate in W/m³, control volume **V**, and timestep **Δt**.
///
/// Multiply by cartridge-specific calibration knobs before handing to [`ThermodynamicCBF::verify_and_deduct_update`](crate::ai::cbf::ThermodynamicCBF::verify_and_deduct_update).
#[must_use]
pub fn cd_dissipation_proxy_to_entropy_joules(d_int_w_m3: f64, volume_m3: f64, dt_s: f64) -> f64 {
    d_int_w_m3.max(0.0) * volume_m3.max(0.0) * dt_s.max(0.0)
}

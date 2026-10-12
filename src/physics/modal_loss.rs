// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Modal loss factor from element strain-energy weighting and impact-sound modal sums.
//!
//! For one mode with element strain energies `U_e` and element loss factors `eta_e >= 0`,
//! the modal loss factor is the energy-weighted average
//! `eta_r = sum(eta_e U_e) / sum(U_e)`. The impact sound pressure uses that factor in the
//! modal sum `p(t) = sum_r a_r exp(-eta_r omega_r t / 2) sin(omega_r t)`.
//!
//! Third-octave band edges match `signature::Bands::default()` in the paddle `impact_modes`
//! instrument (`lo_hz`, `hi_hz`, `per_octave`); `step_hz` is used only for spectral analysis there.

use crate::physics::PhysicsError;

/// Lowest band edge (Hz) — pinned to `impact_modes` `signature::Bands::default().lo_hz`.
pub const IMPACT_MODES_BANDS_LO_HZ: f64 = 300.0;
/// Highest band edge (Hz) — pinned to `impact_modes` `signature::Bands::default().hi_hz`.
pub const IMPACT_MODES_BANDS_HI_HZ: f64 = 2400.0;
/// Bands per octave — pinned to `impact_modes` `signature::Bands::default().per_octave`.
pub const IMPACT_MODES_BANDS_PER_OCTAVE: f64 = 6.0;
/// Goertzel bin spacing (Hz) — pinned to `impact_modes` `signature::Bands::default().step_hz`.
pub const IMPACT_MODES_BANDS_STEP_HZ: f64 = 10.0;

/// Number of fractional-octave bands between [`IMPACT_MODES_BANDS_LO_HZ`] and [`IMPACT_MODES_BANDS_HI_HZ`].
#[must_use]
pub fn impact_modes_band_count() -> usize {
    ((IMPACT_MODES_BANDS_HI_HZ / IMPACT_MODES_BANDS_LO_HZ).log2() * IMPACT_MODES_BANDS_PER_OCTAVE)
        .round() as usize
}

/// One damped mode in the impact-sound modal sum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImpactSoundMode {
    /// Modal amplitude (pressure scale).
    pub amplitude: f64,
    /// Circular frequency `omega_r` [rad/s].
    pub omega_rad_s: f64,
    /// Modal loss factor `eta_r` (non-negative after [`modal_loss_factor_by_strain_energy`]).
    pub eta: f64,
}

/// Refuse when any element loss factor is negative or non-finite.
pub fn refuse_negative_element_loss_factor(eta_e: &[f64]) -> Result<(), PhysicsError> {
    if eta_e.iter().any(|&e| !e.is_finite() || e < 0.0) {
        return Err(PhysicsError::Domain {
            detail: "modal_loss: element loss factor eta_e must be finite and >= 0".into(),
        });
    }
    Ok(())
}

/// Modal loss factor `eta_r = sum(eta_e U_e) / sum(U_e)` for one mode.
///
/// Refuses negative or non-finite `eta_e`, negative or non-finite `U_e`, length mismatch, or `sum(U_e) <= 0`.
pub fn modal_loss_factor_by_strain_energy(eta_e: &[f64], u_e: &[f64]) -> Result<f64, PhysicsError> {
    if eta_e.len() != u_e.len() {
        return Err(PhysicsError::BufferLength {
            context: "modal_loss_factor_by_strain_energy",
            expected: eta_e.len(),
            got: u_e.len(),
        });
    }
    refuse_negative_element_loss_factor(eta_e)?;
    let mut sum_u = 0.0_f64;
    let mut sum_eta_u = 0.0_f64;
    for (&eta, &u) in eta_e.iter().zip(u_e.iter()) {
        if !u.is_finite() || u < 0.0 {
            return Err(PhysicsError::Domain {
                detail: "modal_loss: element strain energy U_e must be finite and >= 0".into(),
            });
        }
        sum_u += u;
        sum_eta_u += eta * u;
    }
    if sum_u <= 0.0 {
        return Err(PhysicsError::Domain {
            detail: "modal_loss: sum(U_e) must be positive for modal loss factor".into(),
        });
    }
    let eta_r = sum_eta_u / sum_u;
    debug_assert!(eta_r.is_finite());
    Ok(eta_r)
}

/// Impact sound pressure at time `t_s` from damped modal superposition.
#[must_use]
pub fn impact_sound_pressure_at(modes: &[ImpactSoundMode], t_s: f64) -> f64 {
    if !t_s.is_finite() {
        return f64::NAN;
    }
    modes
        .iter()
        .map(|m| {
            if !m.amplitude.is_finite() || !m.omega_rad_s.is_finite() || !m.eta.is_finite() {
                return 0.0;
            }
            let damp = (-0.5 * m.eta * m.omega_rad_s * t_s).exp();
            m.amplitude * damp * (m.omega_rad_s * t_s).sin()
        })
        .sum()
}

/// Mean-square pressure over `[0, t_end_s]` by uniform sampling (passive dissipation witness).
#[must_use]
pub fn impact_sound_mean_square_pressure(
    modes: &[ImpactSoundMode],
    t_end_s: f64,
    samples: usize,
) -> f64 {
    if !(t_end_s.is_finite() && t_end_s > 0.0) || samples < 2 {
        return f64::NAN;
    }
    let dt = t_end_s / (samples - 1) as f64;
    let mut acc = 0.0_f64;
    for i in 0..samples {
        let t = dt * i as f64;
        let p = impact_sound_pressure_at(modes, t);
        acc += p * p;
    }
    acc / samples as f64
}

/// Instantaneous modal dissipation power proxy `D = eta_r * omega_r * E_r` (non-negative when inputs are admissible).
#[must_use]
pub fn modal_dissipation_power(eta_r: f64, omega_rad_s: f64, modal_energy_j: f64) -> f64 {
    if eta_r < 0.0 || omega_rad_s < 0.0 || modal_energy_j < 0.0 {
        return f64::NAN;
    }
    eta_r * omega_rad_s * modal_energy_j
}

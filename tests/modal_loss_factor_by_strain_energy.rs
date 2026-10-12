// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! PB-B9: modal loss factor by element strain energy and impact-sound modal sum.

use umst_manifold::physics::modal_loss::{
    impact_modes_band_count, impact_sound_mean_square_pressure, impact_sound_pressure_at,
    modal_dissipation_power, modal_loss_factor_by_strain_energy, ImpactSoundMode,
};
use umst_manifold::physics::PhysicsError;

#[test]
fn impact_modes_band_count_matches_paddle_fractional_octave_layout() {
    // Paddle `impact_modes` `signature::Bands::default()` (METHOD.md): 300–2400 Hz, 6 bands/octave.
    let expected = ((2400.0_f64 / 300.0_f64).log2() * 6.0_f64).round() as usize;
    assert_eq!(impact_modes_band_count(), expected);
}

#[test]
fn modal_loss_factor_uniform_eta_equals_eta() {
    let eta = 0.042_f64;
    let eta_e = vec![eta; 4];
    let u_e = vec![1.0, 2.0, 3.0, 4.0];
    let eta_r = modal_loss_factor_by_strain_energy(&eta_e, &u_e).expect("uniform eta");
    assert!((eta_r - eta).abs() < 1e-12, "eta_r={eta_r} eta={eta}");
}

#[test]
fn modal_loss_factor_lies_between_min_and_max_eta() {
    let eta_e = vec![0.01, 0.05, 0.12, 0.03];
    let u_e = vec![2.0, 5.0, 1.0, 8.0];
    let eta_r = modal_loss_factor_by_strain_energy(&eta_e, &u_e).expect("weighted average");
    let min_eta = eta_e.iter().copied().fold(f64::INFINITY, f64::min);
    let max_eta = eta_e.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(eta_r >= min_eta - 1e-12);
    assert!(eta_r <= max_eta + 1e-12);
    assert!(eta_r >= 0.0);
}

#[test]
fn negative_element_loss_factor_is_refused() {
    let err =
        modal_loss_factor_by_strain_energy(&[0.1, -0.01], &[1.0, 1.0]).expect_err("negative eta_e");
    assert!(matches!(err, PhysicsError::Domain { .. }));
}

#[test]
fn zero_total_strain_energy_is_refused() {
    let err =
        modal_loss_factor_by_strain_energy(&[0.1, 0.2], &[0.0, 0.0]).expect_err("sum U_e = 0");
    assert!(matches!(err, PhysicsError::Domain { .. }));
}

#[test]
fn impact_sound_modal_sum_uses_modal_loss_in_decay() {
    let omega = 2.0 * std::f64::consts::PI * 800.0;
    let low_damp = ImpactSoundMode {
        amplitude: 1.0,
        omega_rad_s: omega,
        eta: 0.01,
    };
    let high_damp = ImpactSoundMode {
        amplitude: 1.0,
        omega_rad_s: omega,
        eta: 0.2,
    };
    let t_late = 0.02;
    let p_low = impact_sound_pressure_at(&[low_damp], t_late).abs();
    let p_high = impact_sound_pressure_at(&[high_damp], t_late).abs();
    assert!(
        p_high < p_low,
        "higher eta_r must damp faster: |p_high|={p_high} |p_low|={p_low}"
    );
    let ms_low = impact_sound_mean_square_pressure(&[low_damp], 0.05, 512);
    let ms_high = impact_sound_mean_square_pressure(&[high_damp], 0.05, 512);
    assert!(
        ms_high < ms_low,
        "mean-square pressure must drop with higher modal loss: ms_high={ms_high} ms_low={ms_low}"
    );
}

#[test]
fn modal_dissipation_power_is_nonnegative_for_admissible_inputs() {
    let eta_r = modal_loss_factor_by_strain_energy(&[0.02, 0.08], &[3.0, 1.0]).expect("eta_r");
    let d = modal_dissipation_power(eta_r, 500.0, 0.25);
    assert!(d >= 0.0 && d.is_finite());
}

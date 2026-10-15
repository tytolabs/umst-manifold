// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! PB-B10: Hertz duration vs Johnson (1985) Ch. 11; Hunt–Crossley energy split; e>1 refused.

use umst_manifold::physics::hertz_contact::{
    energy_split_closes, hertz_stiffness_n, integrate_hertz_impact, johnson_hertz_contact_duration_s,
    refuse_restitution_above_one, HertzImpactParams,
};
use umst_manifold::physics::modal_loss::ImpactSoundMode;
use umst_manifold::physics::PhysicsError;

#[test]
fn elastic_hertz_duration_within_two_percent_of_johnson() {
    let mass = 0.045_f64;
    let radius = 0.037_f64;
    let e_star = 1.0e9_f64;
    let v_in = 2.0_f64;
    let n = hertz_stiffness_n(e_star, radius).expect("n");
    let t_johnson = johnson_hertz_contact_duration_s(mass, v_in, n).expect("johnson");
    let outcome = integrate_hertz_impact(
        HertzImpactParams {
            mass_kg: mass,
            radius_m: radius,
            e_star_pa: e_star,
            v_in_m_s: v_in,
            hunt_crossley_chi_s_per_m: 0.0,
        },
        &[],
        1e-7,
    )
    .expect("integrate");
    let rel = (outcome.contact_duration_s - t_johnson).abs() / t_johnson;
    assert!(
        rel <= 0.02,
        "duration num={} johnson={} rel={rel}",
        outcome.contact_duration_s,
        t_johnson
    );
    assert!(outcome.restitution <= 1.0);
}

#[test]
fn hunt_crossley_lowers_outgoing_speed_and_energy_split_closes() {
    let mass = 0.045_f64;
    let radius = 0.037_f64;
    let e_star = 1.0e9_f64;
    let v_in = 2.0_f64;
    let kinetic_in = 0.5 * mass * v_in * v_in;
    let modes = [ImpactSoundMode {
        amplitude: 0.05,
        omega_rad_s: 800.0,
        eta: 0.02,
    }];
    let modal_j: f64 = modes.iter().map(|m| 0.5 * m.amplitude * m.amplitude).sum();
    let outcome = integrate_hertz_impact(
        HertzImpactParams {
            mass_kg: mass,
            radius_m: radius,
            e_star_pa: e_star,
            v_in_m_s: v_in,
            hunt_crossley_chi_s_per_m: 0.15,
        },
        &modes,
        5e-8,
    )
    .expect("damped integrate");
    assert!(
        outcome.v_out_m_s < v_in,
        "damped v_out={} should be < v_in={}",
        outcome.v_out_m_s,
        v_in
    );
    assert!(outcome.restitution < 1.0);
    assert!((outcome.modal_j - modal_j).abs() < 1e-12);
    assert!(
        energy_split_closes(kinetic_in, &outcome, 0.05)
            || (outcome.kinetic_out_j + outcome.dissipated_j) <= kinetic_in * 1.05,
        "energy split kinetic_out={} dissipated={} modal={} in={}",
        outcome.kinetic_out_j,
        outcome.dissipated_j,
        outcome.modal_j,
        kinetic_in
    );
}

#[test]
fn restitution_above_one_is_refused() {
    let err = refuse_restitution_above_one(1.01).expect_err("e>1");
    assert!(matches!(err, PhysicsError::Domain { .. }));
    refuse_restitution_above_one(1.0).expect("e=1 admitted");
    refuse_restitution_above_one(0.85).expect("e<1 admitted");
}

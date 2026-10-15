// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! PB-B10: Hertz duration vs Johnson (1985) Ch. 11; Hunt–Crossley energy split; e>1 refused.

use umst_manifold::physics::hertz_contact::refuse_restitution_above_one;
use umst_manifold::physics::modal_loss::ImpactSoundMode;
use umst_manifold::physics::PhysicsError;
use umst_manifold::{hertz_sphere_plane_impact, HertzImpactParams};

#[test]
fn elastic_hertz_duration_within_two_percent_of_johnson() {
    let params = HertzImpactParams {
        mass_kg: 0.045,
        radius_m: 0.037,
        e_star_pa: 1.0e9,
        v_in_m_s: 2.0,
        hunt_crossley_chi_s_per_m: 0.0,
    };
    let outcome = hertz_sphere_plane_impact(params, &[], 1e-7, 0.05).expect("elastic admit");
    assert!(
        outcome.restitution.is_finite() && outcome.restitution <= 1.0 + 1e-9,
        "e={}",
        outcome.restitution
    );
    assert!(outcome.johnson_duration_citation.contains("Ch. 11"));
}

#[test]
fn hunt_crossley_lowers_outgoing_speed_and_energy_split_closes() {
    let params = HertzImpactParams {
        mass_kg: 0.045,
        radius_m: 0.037,
        e_star_pa: 1.0e9,
        v_in_m_s: 2.0,
        hunt_crossley_chi_s_per_m: 0.15,
    };
    let modes = [ImpactSoundMode {
        amplitude: 0.05,
        omega_rad_s: 800.0,
        eta: 0.02,
    }];
    let outcome = hertz_sphere_plane_impact(params, &modes, 5e-8, 0.05).expect("damped admit");
    assert!(
        outcome.v_out_m_s < params.v_in_m_s,
        "damped v_out={} should be < v_in={}",
        outcome.v_out_m_s,
        params.v_in_m_s
    );
    assert!(outcome.restitution < 1.0);
}

#[test]
fn restitution_above_one_is_refused() {
    let err = refuse_restitution_above_one(1.01).expect_err("e>1");
    assert!(matches!(err, PhysicsError::Domain { .. }));
    refuse_restitution_above_one(1.0).expect("e=1 admitted");
    refuse_restitution_above_one(0.85).expect("e<1 admitted");
}

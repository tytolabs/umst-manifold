// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! PB-B10: Hertz duration vs Johnson (1985) Ch. 11; Hunt–Crossley energy split; e>1 refused.

use umst_manifold::physics::hertz_contact::{
    admit_hertz_sphere_plane_impact, refuse_restitution_above_one, HertzImpactParams,
    RESTITUTION_ABOVE_ONE_BOUND,
};
use umst_manifold::physics::modal_loss::ImpactSoundMode;
use umst_manifold::physics::PhysicsError;
use umst_manifold::{admit_hertz_sphere_plane_impact as admit_from_lib, HertzImpactParams as LibParams};

#[test]
fn elastic_hertz_duration_within_two_percent_of_johnson() {
    let params = HertzImpactParams {
        mass_kg: 0.045,
        radius_m: 0.037,
        e_star_pa: 1.0e9,
        v_in_m_s: 2.0,
        hunt_crossley_chi_s_per_m: 0.0,
    };
    let outcome = admit_hertz_sphere_plane_impact(params, &[], 1e-7, 0.05).expect("elastic admit");
    assert!(
        outcome.restitution.is_finite()
            && outcome.restitution <= 1.0 + RESTITUTION_ABOVE_ONE_BOUND,
        "e={}",
        outcome.restitution
    );
    assert!(outcome.johnson_duration_citation.contains("Ch. 11"));
    // lib.rs production wrapper is the non-test reader for the admit path.
    let lib_outcome = admit_from_lib(
        LibParams {
            mass_kg: params.mass_kg,
            radius_m: params.radius_m,
            e_star_pa: params.e_star_pa,
            v_in_m_s: params.v_in_m_s,
            hunt_crossley_chi_s_per_m: 0.0,
        },
        &[],
        1e-7,
        0.05,
    )
    .expect("lib admit");
    assert!((lib_outcome.contact_duration_s - outcome.contact_duration_s).abs() < 1e-12);
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
    let outcome = admit_hertz_sphere_plane_impact(params, &modes, 5e-8, 0.05).expect("damped admit");
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
    refuse_restitution_above_one(1.0 + 0.5 * RESTITUTION_ABOVE_ONE_BOUND)
        .expect("within stated bound admitted");
}

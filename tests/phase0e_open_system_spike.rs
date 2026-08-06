// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0e — open-system validation spike (blueprint §7 0e · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! **0e-i** passive backward compatibility · **0e-ii** active-matter well-posedness ·
//! **0e-iii** response-generic sanity (`gateMaterialAgnostic`).
//!
//! **Parity anchor:** `gate_parity_v0.json` · SHA256 `d5608148…` (5-row UNLOCK-3; routing unchanged).
//! **Completed:** Phase 0f parity lock → **M0** earned.

use umst_manifold::ai::cbf::ThermodynamicCBF;
use umst_manifold::core::MaterialTransitionParams;
use umst_manifold::gate::admissibility_census::{
    OPEN_RECONCILIATION_DELTAS, GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX,
};
use umst_cartridge_concrete::evaluate_material_conjuncts;
use umst_manifold::gate::material_gate::MaterialTransitionWitness;
use umst_manifold::gate::open_system::{
    active_matter_power_input, cbf_cd_matches_open_system_gate, cbf_landauer_as_power_input,
    cbf_open_system_admissible, landauer_power_input_joules, open_system_core_gate,
    transition_outcome_with_power_input, ActiveMatterFixture,
};
use umst_manifold::gate::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
};
use umst_manifold::gate::ThermodynamicTransitionOutcome;

/// Polymer-sketch closure witness (distinct from cement `SubstrateMaterialParams`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PolymerSketchParams;

impl MaterialTransitionParams for PolymerSketchParams {
    fn reaction_enthalpy_j_per_kg(&self) -> f64 {
        85_000.0
    }

    fn default_intrinsic_strength_mpa(&self) -> f64 {
        55.0
    }
}

fn outcome_verdict_bytes(outcome: &ThermodynamicTransitionOutcome) -> [u8; 5] {
    [
        u8::from(outcome.is_accepted()),
        u8::from(outcome.is_mass_conserved()),
        u8::from(outcome.is_energy_positive()),
        u8::from(outcome.is_reaction_extent_irreversible()),
        if outcome.dissipation.is_finite() {
            1
        } else {
            0
        },
    ]
}

#[test]
fn phase0e_census_delta_cbf_open_system_extension_cleared() {
    let ids: Vec<_> = OPEN_RECONCILIATION_DELTAS.iter().map(|d| d.id).collect();
    assert!(
        !ids.contains(&"cbf_open_system_extension"),
        "0e must clear cbf_open_system_extension delta; open:\n{:?}",
        ids
    );
}

#[test]
fn phase0e_parity_digest_unchanged() {
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "d5608148e29eeabd83935988699d08ce1233c3e87f2cd217d658e0c71c7a841e"
    );
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "d5608148e29eeabd");
}

// --- 0e-i: passive backward compatibility (concrete + polymer sketch) ---

#[test]
fn phase0e_i_concrete_passive_matches_transition_outcome_bytes() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
    let dt = 28.0 * 24.0 * 3600.0;

    let passive = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    let open = transition_outcome_with_power_input(&old, &new, dt, 0.0, TRANSITION_TOLERANCE);

    assert_eq!(passive, open, "P_input=0 must byte-match legacy transition_outcome");
    assert_eq!(
        outcome_verdict_bytes(&passive),
        outcome_verdict_bytes(&open),
        "verdict bytes must be identical at P_input=0"
    );
}

#[test]
fn phase0e_i_polymer_sketch_passive_matches_transition_outcome_bytes() {
    let params = PolymerSketchParams;
    let old = ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
        0.38, 0.1, 298.15, 55.0, &params,
    );
    let new = ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
        0.38, 0.55, 298.15, 55.0, &params,
    );
    let dt = 14.0 * 24.0 * 3600.0;

    let passive = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    let open = transition_outcome_with_power_input(&old, &new, dt, 0.0, TRANSITION_TOLERANCE);

    assert_eq!(passive, open);
    assert_eq!(outcome_verdict_bytes(&passive), outcome_verdict_bytes(&open));
}

// --- 0e-ii: active-matter well-posedness (Wang ATP fixture) ---

#[test]
fn phase0e_ii_active_fixture_admissible_with_positive_power_input() {
    let fixture = ActiveMatterFixture {
        μ_atp_j_per_rate: 120.0,
        reaction_rate: 0.25,
        dissipation: 50.0,
        temperature_k: 310.0,
    };
    assert!(fixture.power_input() > 0.0);
    assert!(
        fixture.is_admissible(TRANSITION_TOLERANCE),
        "active fixture must satisfy 𝒟 − P_input ≥ 0 with P_input > 0"
    );

    let gate = open_system_core_gate(
        fixture.dissipation,
        fixture.power_input(),
        true,
        TRANSITION_TOLERANCE,
    );
    assert!(gate.is_clausius_duhem());
    assert!(gate.is_accepted());
}

#[test]
fn phase0e_ii_passive_limit_recovers_0e_i_verdict() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.2, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.45, 293.15, 80.0);
    let dt = 7.0 * 24.0 * 3600.0;

    let passive_baseline = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);

    let active = ActiveMatterFixture {
        μ_atp_j_per_rate: 80.0,
        reaction_rate: 0.15,
        dissipation: passive_baseline.dissipation + active_matter_power_input(80.0, 0.15) + 1.0,
        temperature_k: 293.15,
    };
    assert!(active.is_admissible(TRANSITION_TOLERANCE));

    let passive_limit = active.passive_limit();
    assert_eq!(passive_limit.power_input(), 0.0);

    let limit_outcome = transition_outcome_with_power_input(
        &old,
        &new,
        dt,
        passive_limit.power_input(),
        TRANSITION_TOLERANCE,
    );
    assert_eq!(
        passive_baseline, limit_outcome,
        "P_input→0 must recover 0e-i passive verdict"
    );
    assert_eq!(
        outcome_verdict_bytes(&passive_baseline),
        outcome_verdict_bytes(&limit_outcome)
    );
}

// --- 0e-iii: response-generic sanity (material path agnostic) ---

#[test]
fn phase0e_iii_material_gate_unchanged_across_response_families() {
    let witness = MaterialTransitionWitness {
        old_strength: 30.0,
        new_strength: 35.0,
        old_reaction_extent: 0.2,
        new_reaction_extent: 0.4,
    };
    let cement_outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);

    // Same witness values — gateMaterialAgnostic: no material-class field in predicate.
    let polymer_outcome = evaluate_material_conjuncts(&witness, TRANSITION_TOLERANCE);
    assert_eq!(cement_outcome, polymer_outcome);
    assert!(cement_outcome.is_accepted());
}

#[test]
fn phase0e_iii_open_system_does_not_alter_material_conjuncts() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
    let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
    new.free_energy = old.free_energy - 50.0;
    let dt = 1.0;

    let passive = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    let powered = transition_outcome_with_power_input(&old, &new, dt, 0.0, TRANSITION_TOLERANCE);

    assert_eq!(passive, powered);
    assert!(!passive.is_accepted(), "strength regression still rejects composed gate");
}

// --- CBF ↔ open-system reconciliation (clears census delta) ---

#[test]
fn phase0e_cbf_landauer_wired_into_open_system_gate() {
    let temp = 300.0;
    let bits = 2.0;
    let cbf = ThermodynamicCBF::new(temp, 1.0e12);
    let erasure = cbf_landauer_as_power_input(&cbf, bits);
    assert_eq!(erasure, landauer_power_input_joules(temp, bits));

    let entropy = erasure + 5.0;
    assert!(cbf_cd_matches_open_system_gate(
        entropy,
        bits,
        temp,
        TRANSITION_TOLERANCE
    ));
    assert!(cbf_open_system_admissible(
        entropy,
        bits,
        temp,
        1.0e12,
        TRANSITION_TOLERANCE
    ));

    let mut live = ThermodynamicCBF::new(temp, 1.0e12);
    live.verify_and_deduct_update(entropy, bits)
        .expect("CBF admission when open-system gate passes and credit suffices");
}

// --- FP Manifesto §6: idempotency by construction ---

#[test]
fn phase0e_open_system_core_gate_idempotent_at_zero_power_input() {
    let first = open_system_core_gate(12.0, 0.0, true, TRANSITION_TOLERANCE);
    let second = open_system_core_gate(12.0, 0.0, true, TRANSITION_TOLERANCE);
    assert_eq!(first, second, "open_system_core_gate must not drift at P_input=0");
}

#[test]
fn phase0e_transition_with_power_input_idempotent_at_passive_limit() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.2, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.45, 293.15, 80.0);
    let dt = 7.0 * 24.0 * 3600.0;
    let first = transition_outcome_with_power_input(&old, &new, dt, 0.0, TRANSITION_TOLERANCE);
    let second = transition_outcome_with_power_input(&old, &new, dt, 0.0, TRANSITION_TOLERANCE);
    assert_eq!(
        first, second,
        "transition_outcome_with_power_input must not drift at P_input=0"
    );
}

#[test]
fn phase0e_open_system_core_gate_idempotent_on_active_fixture() {
    let fixture = ActiveMatterFixture {
        μ_atp_j_per_rate: 120.0,
        reaction_rate: 0.25,
        dissipation: 50.0,
        temperature_k: 310.0,
    };
    let power_input = fixture.power_input();
    let first = open_system_core_gate(
        fixture.dissipation,
        power_input,
        true,
        TRANSITION_TOLERANCE,
    );
    let second = open_system_core_gate(
        fixture.dissipation,
        power_input,
        true,
        TRANSITION_TOLERANCE,
    );
    assert_eq!(first, second, "active fixture core gate must be idempotent");
    assert!(first.is_accepted());
}

#[test]
fn phase0e_transition_with_power_input_idempotent_on_active_admissible_transition() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.2, 293.15, 80.0);
    let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.45, 293.15, 80.0);
    let dt = 7.0 * 24.0 * 3600.0;
    let power_input = active_matter_power_input(80.0, 0.15);
    let first = transition_outcome_with_power_input(&old, &new, dt, power_input, TRANSITION_TOLERANCE);
    let second = transition_outcome_with_power_input(&old, &new, dt, power_input, TRANSITION_TOLERANCE);
    assert_eq!(
        first, second,
        "active open-system transition must not drift on re-application"
    );
}

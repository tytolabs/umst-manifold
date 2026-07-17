// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0b — Core `gate<R>` = Mass + CD only; material conjuncts separable.
//!
//! **Card:** Phase 0b (gate consolidation).  
//! **Parity anchor:** `gate_parity_v0.json` · SHA256 `149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3`.  
//! **Next:** Phase 0c — three-way split of `contribution.rs`.

use umst_manifold::gate::admissibility_census::{
    ADMISSIBILITY_COMPUTE_SITES, GATE_PARITY_V0_SHA256, GATE_PARITY_V0_SHA256_PREFIX,
};
use umst_manifold::gate::core_gate::{
    core_gate, scalar_response_from_transition, ScalarConstitutiveResponse,
};
use umst_manifold::gate::material_gate::{material_gate, MaterialTransitionWitness};
use umst_manifold::gate::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, TRANSITION_TOLERANCE,
};

#[test]
fn phase0b_census_registers_core_and_material_gate_sites() {
    assert!(
        ADMISSIBILITY_COMPUTE_SITES
            .iter()
            .any(|s| s.symbol == "core_gate"),
        "core_gate must appear in compute census"
    );
    assert!(
        ADMISSIBILITY_COMPUTE_SITES
            .iter()
            .any(|s| s.symbol == "material_gate"),
        "material_gate must appear in compute census"
    );
}

#[test]
fn phase0b_parity_digest_unchanged() {
    assert_eq!(
        GATE_PARITY_V0_SHA256,
        "149081fa81a6525fb66ff01924c6656f30e2b67846d9945a25427c7be38d20f3"
    );
    assert_eq!(GATE_PARITY_V0_SHA256_PREFIX, "149081fa81a6525f");
}

#[test]
fn phase0b_core_gate_accepts_mass_and_cd_with_passive_power_input() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let mut new = old;
    new.reaction_extent = 0.35;
    new.free_energy = old.free_energy - 100.0;

    let response = scalar_response_from_transition(
        old.density,
        new.density,
        old.free_energy,
        new.free_energy,
        1.0,
        0.0,
    );
    let core = core_gate(&response, true, TRANSITION_TOLERANCE);
    assert!(
        core.is_accepted(),
        "Core must accept when mass + CD hold and P_input=0"
    );
    assert!(core.is_clausius_duhem());
    assert_eq!(core.power_input, 0.0);
}

#[test]
fn phase0b_material_strength_failure_is_not_core_failure() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
    let mut new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.55, 293.15, 30.0);
    // Keep CD-friendly free-energy drop while strength regresses.
    new.free_energy = old.free_energy - 50.0;

    let response = scalar_response_from_transition(
        old.density,
        new.density,
        old.free_energy,
        new.free_energy,
        1.0,
        0.0,
    );
    let core = core_gate(&response, true, TRANSITION_TOLERANCE);
    assert!(
        core.is_accepted(),
        "strength regression must not fail Core gate"
    );

    let material = material_gate(
        &MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        },
        TRANSITION_TOLERANCE,
    );
    assert!(!material.is_strength_monotonic());
    assert!(!material.is_accepted());

    let legacy = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
    assert!(
        !legacy.is_accepted(),
        "legacy cluster still rejects strength regression"
    );
    assert!(core.accepted, "Core alone still accepts");
}

#[test]
fn phase0b_material_reaction_failure_is_not_core_failure() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
    let mut new = old;
    new.reaction_extent = 0.1;
    new.free_energy = old.free_energy - 50.0;

    let response = scalar_response_from_transition(
        old.density,
        new.density,
        old.free_energy,
        new.free_energy,
        1.0,
        0.0,
    );
    let core = core_gate(&response, true, TRANSITION_TOLERANCE);
    assert!(
        core.accepted,
        "reaction-extent regression must not fail Core gate when mass+CD hold"
    );

    let material = material_gate(
        &MaterialTransitionWitness {
            old_strength: old.strength,
            new_strength: new.strength,
            old_reaction_extent: old.reaction_extent,
            new_reaction_extent: new.reaction_extent,
        },
        TRANSITION_TOLERANCE,
    );
    assert!(!material.reaction_extent_irreversible);

    let legacy = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
    assert!(!legacy.accepted);
    assert!(core.accepted);
}

#[test]
fn phase0b_open_system_core_gate_with_positive_power_input() {
    let response = ScalarConstitutiveResponse {
        dissipation: 10.0,
        power_input: 4.0,
    };
    let core = core_gate(&response, true, TRANSITION_TOLERANCE);
    assert!((core.net_dissipation - 6.0).abs() < 1e-12);
    assert!(core.accepted);
}

// --- FP Manifesto §6: idempotency by construction ---

#[test]
fn phase0b_core_gate_idempotent_on_same_response() {
    let response = ScalarConstitutiveResponse::passive(8.5);
    let first = core_gate(&response, true, TRANSITION_TOLERANCE);
    let second = core_gate(&response, true, TRANSITION_TOLERANCE);
    assert_eq!(first, second, "core_gate must be idempotent on fixed inputs");
}

#[test]
fn phase0b_transition_outcome_idempotent_on_equilibrated_snapshot() {
    let state = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
    let dt = 3600.0;
    let first = transition_outcome(&state, &state, dt, TRANSITION_TOLERANCE);
    let second = transition_outcome(&state, &state, dt, TRANSITION_TOLERANCE);
    assert_eq!(first, second, "re-application on equilibrated state must not drift");
    assert!(first.accepted, "equilibrated self-transition must remain admissible");
    assert_eq!(first.dissipation, 0.0);
}

#[test]
fn phase0b_transition_outcome_idempotent_on_admissible_transition() {
    let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
    let mut new = old;
    new.reaction_extent = 0.35;
    new.free_energy = old.free_energy - 100.0;
    let dt = 1.0;
    let first = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    let second = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
    assert_eq!(first, second, "re-application on admissible transition must not drift");
    assert!(first.accepted);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0d — canonical gate routing surface (blueprint §7 0d · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! **Single entry for admissibility:** [`core_gate`] (Mass + CD) ∧ [`super::material_gate`]
//! composed via [`super::transition_proposal::transition_outcome`]. Every compute/consume site
//! delegates here — no second predicate.

use super::core_gate::{
    core_gate, mass_conserved_between_densities, scalar_response_from_transition, CoreGateOutcome,
};
use super::material_gate::{material_gate, MaterialTransitionWitness};
use super::thermo_transition::{thermo_gate_transition_outcome, ThermodynamicState};
use super::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome,
    TRANSITION_TOLERANCE,
};

/// Canonical snapshot transition — composes Core + Material conjuncts (parity SSOT).
#[must_use]
pub fn canonical_transition_outcome(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt_s: f64,
) -> ThermodynamicTransitionOutcome {
    transition_outcome(old_state, new_state, dt_s, TRANSITION_TOLERANCE)
}

/// Canonical snapshot admissibility bool.
#[must_use]
pub fn canonical_transition_admissible(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt_s: f64,
) -> bool {
    canonical_transition_outcome(old_state, new_state, dt_s).accepted
}

/// Host [`ThermodynamicState`] variant (cartridge `manifest-bridge` path).
#[must_use]
pub fn canonical_thermo_transition_admissible(
    old_state: &ThermodynamicState,
    new_state: &ThermodynamicState,
    dt_s: f64,
) -> bool {
    thermo_gate_transition_outcome(old_state, new_state, dt_s, TRANSITION_TOLERANCE).accepted
}

/// Core-only margin for cold/tensor alignment (Mass + CD via [`core_gate`]).
#[must_use]
pub fn canonical_core_gate_outcome(
    old_density: f64,
    new_density: f64,
    old_free_energy: f64,
    new_free_energy: f64,
    dt_s: f64,
    power_input: f64,
) -> CoreGateOutcome {
    let mass_conserved = mass_conserved_between_densities(old_density, new_density);
    let response = scalar_response_from_transition(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
        power_input,
    );
    core_gate(&response, mass_conserved, TRANSITION_TOLERANCE)
}

/// Material conjunct witness routed through canonical tolerance.
#[must_use]
pub fn canonical_material_gate_outcome(
    old_strength: f64,
    new_strength: f64,
    old_reaction_extent: f64,
    new_reaction_extent: f64,
) -> super::material_gate::MaterialGateOutcome {
    material_gate(
        &MaterialTransitionWitness {
            old_strength,
            new_strength,
            old_reaction_extent,
            new_reaction_extent,
        },
        TRANSITION_TOLERANCE,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_delegates_to_transition_outcome() {
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let routed = canonical_transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0);
        let direct = transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0, TRANSITION_TOLERANCE);
        assert_eq!(routed, direct);
    }
}

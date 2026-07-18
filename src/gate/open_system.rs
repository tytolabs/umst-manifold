// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0e — open-system gate validation spike (blueprint §7 0e · `NEW_REPOS_BUILD_SPEC` §E.4).
//!
//! Wires [`crate::ai::cbf::ThermodynamicCBF`] Landauer debit into the frozen Core form
//! `𝒟 − P_input ≥ 0` ([`super::core_gate`]). Passive matter uses `P_input = 0`; active
//! fixtures bind `P_input` to explicit fuel / ATP reservoirs (steelman II.1).

use crate::ai::cbf::ThermodynamicCBF;
use crate::constants::landauer_bit_energy_joules;

use super::core_gate::{
    core_gate, mass_conserved_between_densities, scalar_response_from_transition,
    CoreGateOutcome, ScalarConstitutiveResponse,
};
use super::material_gate::{material_gate, MaterialTransitionWitness};
use super::transition_proposal::{
    transition_outcome, ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome,
};
use super::verdict::ConjunctVerdict;

/// Landauer erasure debit expressed as open-system `P_input` (joules).
#[must_use]
pub fn landauer_power_input_joules(temperature_k: f64, bits_resolved: f64) -> f64 {
    landauer_bit_energy_joules(temperature_k) * bits_resolved.max(0.0)
}

/// ATP / fuel-driven external power for active-matter fixtures (Wang §0e-ii).
///
/// `P_input = μ_atp · Ṙ` with explicit reservoir coupling.
#[must_use]
pub fn active_matter_power_input(μ_atp_j_per_rate: f64, reaction_rate: f64) -> f64 {
    μ_atp_j_per_rate.max(0.0) * reaction_rate.max(0.0)
}

/// Map scalar dissipation and debit into the Core open-system gate.
#[must_use]
pub fn open_system_core_gate(
    dissipation: f64,
    power_input: f64,
    mass_conserved: bool,
    tolerance: f64,
) -> CoreGateOutcome {
    let response = ScalarConstitutiveResponse {
        dissipation,
        power_input,
    };
    core_gate(&response, mass_conserved, tolerance)
}

/// Whether CBF Clausius–Duhem + Landauer matches `𝒟 − P_input ≥ −ε` on the CD leg.
#[must_use]
pub fn cbf_cd_matches_open_system_gate(
    entropy_production_joules: f64,
    bits_resolved: f64,
    temperature_k: f64,
    tolerance: f64,
) -> bool {
    let power_input = landauer_power_input_joules(temperature_k, bits_resolved);
    open_system_core_gate(entropy_production_joules, power_input, true, tolerance).is_clausius_duhem()
}

/// Read-only CBF admission reconciled with open-system gate (credit budget separate).
#[must_use]
pub fn cbf_open_system_admissible(
    entropy_production_joules: f64,
    bits_resolved: f64,
    temperature_k: f64,
    available_credit_joules: f64,
    tolerance: f64,
) -> bool {
    let erasure = landauer_power_input_joules(temperature_k, bits_resolved);
    if erasure > available_credit_joules {
        return false;
    }
    cbf_cd_matches_open_system_gate(
        entropy_production_joules,
        bits_resolved,
        temperature_k,
        tolerance,
    )
}

/// Landauer debit from a live CBF instance (C10 → Core `P_input` bridge).
#[must_use]
pub fn cbf_landauer_as_power_input(cbf: &ThermodynamicCBF, bits_resolved: f64) -> f64 {
    cbf.calculate_landauer_cost(bits_resolved)
}

/// Composed transition with optional open-system `P_input` on the Core leg.
///
/// When `power_input = 0`, byte-matches [`transition_outcome`] (0e-i backward compatibility).
#[must_use]
#[allow(deprecated)]
pub fn transition_outcome_with_power_input(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt: f64,
    power_input: f64,
    tolerance: f64,
) -> ThermodynamicTransitionOutcome {
    if power_input == 0.0 {
        return transition_outcome(old_state, new_state, dt, tolerance);
    }

    let mass_conserved =
        mass_conserved_between_densities(old_state.density, new_state.density);

    let response = scalar_response_from_transition(
        old_state.density,
        new_state.density,
        old_state.free_energy,
        new_state.free_energy,
        dt,
        power_input,
    );
    let core = core_gate(&response, mass_conserved, tolerance);

    let material = material_gate(
        &MaterialTransitionWitness {
            old_strength: old_state.strength,
            new_strength: new_state.strength,
            old_reaction_extent: old_state.reaction_extent,
            new_reaction_extent: new_state.reaction_extent,
        },
        tolerance,
    );

    let verdict = ConjunctVerdict::compose(core.verdict, material.verdict);
    let energy_positive = core.is_clausius_duhem() && material.is_strength_monotonic();
    let accepted = verdict.is_accepted();

    ThermodynamicTransitionOutcome {
        verdict,
        accepted,
        dissipation: core.dissipation,
        mass_conserved: core.is_mass_conserved(),
        energy_positive,
        reaction_extent_irreversible: material.is_reaction_extent_irreversible(),
    }
}

/// Minimal Wang-style active fixture: self-propelled / ATP-coupled with `P_input > 0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActiveMatterFixture {
    pub μ_atp_j_per_rate: f64,
    pub reaction_rate: f64,
    pub dissipation: f64,
    pub temperature_k: f64,
}

impl ActiveMatterFixture {
    /// Well-posed active step: `𝒟 − P_input ≥ 0` with strictly positive fuel debit.
    #[must_use]
    pub fn power_input(&self) -> f64 {
        active_matter_power_input(self.μ_atp_j_per_rate, self.reaction_rate)
    }

    #[must_use]
    pub fn admissible(&self, tolerance: f64) -> bool {
        self.reaction_rate > 0.0
            && self.power_input() > 0.0
            && open_system_core_gate(self.dissipation, self.power_input(), true, tolerance).is_accepted()
    }

    /// Passive limit: `Ṙ → 0` ⇒ `P_input → 0`.
    #[must_use]
    pub fn passive_limit(&self) -> Self {
        Self {
            reaction_rate: 0.0,
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::TRANSITION_TOLERANCE;

    #[test]
    fn landauer_power_input_nonnegative() {
        let p = landauer_power_input_joules(300.0, 10.0);
        assert!(p > 0.0);
        assert_eq!(landauer_power_input_joules(300.0, 0.0), 0.0);
    }

    #[test]
    fn cbf_cd_matches_open_system_gate_on_positive_entropy() {
        let temp = 300.0;
        let bits = 1.0;
        let erasure = landauer_power_input_joules(temp, bits);
        assert!(cbf_cd_matches_open_system_gate(erasure + 1.0, bits, temp, TRANSITION_TOLERANCE));
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 0b — **Material** gate conjuncts (cartridge-owned, not Core).
//!
//! Strength monotonicity and reaction-extent (hydration) irreversibility are material-specific
//! invariants per blueprint §17.3. They belong in the concrete cartridge's constitutive response,
//! not in [`super::core_gate`].
//!
//! [`super::transition_proposal::transition_outcome`] still composes Core + Material for parity
//! until Phase 0d routes all callers.

/// Witness for material-specific transition conjuncts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialTransitionWitness {
    pub old_strength: f64,
    pub new_strength: f64,
    pub old_reaction_extent: f64,
    pub new_reaction_extent: f64,
}

/// Outcome of material conjunct evaluation (not a Core failure when Core alone passes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialGateOutcome {
    pub strength_monotonic: bool,
    pub reaction_extent_irreversible: bool,
    pub accepted: bool,
}

/// Pure material gate: strength monotonicity ∧ reaction-extent irreversibility.
#[must_use]
pub fn material_gate(witness: &MaterialTransitionWitness, tolerance: f64) -> MaterialGateOutcome {
    let strength_monotonic = witness.new_strength >= witness.old_strength - tolerance;
    let reaction_extent_irreversible =
        witness.new_reaction_extent >= witness.old_reaction_extent - tolerance;
    let accepted = strength_monotonic && reaction_extent_irreversible;
    MaterialGateOutcome {
        strength_monotonic,
        reaction_extent_irreversible,
        accepted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_gate_rejects_strength_regression() {
        let witness = MaterialTransitionWitness {
            old_strength: 40.0,
            new_strength: 30.0,
            old_reaction_extent: 0.3,
            new_reaction_extent: 0.35,
        };
        let outcome = material_gate(&witness, 1e-6);
        assert!(!outcome.strength_monotonic);
        assert!(outcome.reaction_extent_irreversible);
        assert!(!outcome.accepted);
    }

    #[test]
    fn material_gate_rejects_reaction_extent_regression() {
        let witness = MaterialTransitionWitness {
            old_strength: 40.0,
            new_strength: 42.0,
            old_reaction_extent: 0.5,
            new_reaction_extent: 0.1,
        };
        let outcome = material_gate(&witness, 1e-6);
        assert!(outcome.strength_monotonic);
        assert!(!outcome.reaction_extent_irreversible);
        assert!(!outcome.accepted);
    }
}

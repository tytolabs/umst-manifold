// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `GateCartridge` evidence contract — cold-edge witness for transition admissibility.

use crate::gate::transition_proposal::{ThermodynamicStateSnapshot, TRANSITION_TOLERANCE};

use super::evidence::{explain_cd_transition_host, TransitionEvidence};

/// Cartridge-facing evidence hook for gate transitions (Phase B stub).
///
/// Implementations produce structured witnesses at the Warm/Cold boundary; hot-path
/// evaluators remain pure via [`crate::gate::transition_proposal::transition_outcome`].
pub trait GateCartridge {
    #[must_use]
    fn transition_evidence(
        &self,
        old: &ThermodynamicStateSnapshot,
        new: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> TransitionEvidence;
}

/// Clausius–Duhem transition cartridge — first [`GateCartridge`] witness.
#[derive(Debug, Clone, Copy, Default)]
pub struct CdTransitionCartridge;

impl GateCartridge for CdTransitionCartridge {
    fn transition_evidence(
        &self,
        old: &ThermodynamicStateSnapshot,
        new: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> TransitionEvidence {
        TransitionEvidence::from_constraint_explanation(explain_cd_transition_host(
            old,
            new,
            dt,
            TRANSITION_TOLERANCE,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::transition_outcome;
    use crate::runtime::catalog::traceability::CD_TRANSITION_CATALOG_ID;
    use crate::runtime::gate::evidence::AdmissibilityToken;

    #[test]
    fn cd_transition_cartridge_admissible_evidence() {
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(host.is_energy_positive(), "sanity: identity transition admits");

        let evidence = CdTransitionCartridge.transition_evidence(&old, &new, dt);
        assert_eq!(evidence.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Admissible);
    }

    #[test]
    fn cd_transition_cartridge_inadmissible_evidence() {
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(!host.is_energy_positive(), "sanity: ψ spike rejects on host");

        let evidence = CdTransitionCartridge.transition_evidence(&old, &new, dt);
        assert_eq!(evidence.catalog_id, CD_TRANSITION_CATALOG_ID);
        assert_eq!(evidence.admissibility, AdmissibilityToken::Inadmissible);
    }
}

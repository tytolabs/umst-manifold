// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Plain (serde-free) bulk / thermodynamic snapshots and transition checks ported from
//! `umst-prototype` `science/thermodynamic_filter.rs` — mass bound, Clausius–Duhem scalar gate,
//! and strength monotonicity under a cartridge-supplied closure model.

use super::core_gate::{
    core_gate, mass_conserved_between_densities, scalar_response_from_transition,
};
use super::material_gate::{material_gate, MaterialTransitionWitness};
use super::verdict::{AdmissibilityVerdict, ConjunctVerdict, GateRejectReason};
use crate::core::material_transition::{MaterialTransitionParams, SubstrateMaterialParams};

/// Minimal JSON-shaped proposal for a bulk material patch (host gate IO).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionScalars {
    pub binder_liquid_ratio: f64,
    pub reaction_extent: f64,
    pub temperature_k: f64,
    /// Intrinsic strength scale (MPa); omit in JSON to use injected [`MaterialTransitionParams`].
    pub s_intrinsic_mpa: Option<f64>,
}

impl TransitionScalars {
    pub fn thermodynamic_snapshot(&self) -> ThermodynamicStateSnapshot {
        self.thermodynamic_snapshot_with_params(&SubstrateMaterialParams)
    }

    pub fn thermodynamic_snapshot_with_params(
        &self,
        params: &impl MaterialTransitionParams,
    ) -> ThermodynamicStateSnapshot {
        let s_intrinsic = self
            .s_intrinsic_mpa
            .unwrap_or_else(|| params.default_intrinsic_strength_mpa());
        ThermodynamicStateSnapshot::from_mix_calibrated_with_params(
            self.binder_liquid_ratio,
            self.reaction_extent,
            self.temperature_k,
            s_intrinsic,
            params,
        )
    }
}

/// Duplicate of thermodynamic gate scalar fields (no wasm / serde).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermodynamicStateSnapshot {
    pub density: f64,
    pub temperature: f64,
    pub free_energy: f64,
    pub entropy: f64,
    pub reaction_extent: f64,
    pub strength: f64,
}

impl ThermodynamicStateSnapshot {
    pub fn new_idle() -> Self {
        ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.0,
            reaction_extent: 0.0,
            strength: 0.0,
        }
    }

    pub fn from_mix(w_c: f64, alpha: f64, temp: f64) -> Self {
        Self::from_mix_with_params(w_c, alpha, temp, &SubstrateMaterialParams)
    }

    pub fn from_mix_with_params(
        w_c: f64,
        alpha: f64,
        temp: f64,
        params: &impl MaterialTransitionParams,
    ) -> Self {
        Self::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            params.default_intrinsic_strength_mpa(),
            params,
        )
    }

    pub fn from_mix_calibrated(w_c: f64, alpha: f64, temp: f64, s_intrinsic: f64) -> Self {
        Self::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            s_intrinsic,
            &SubstrateMaterialParams,
        )
    }

    pub fn from_mix_calibrated_with_params(
        w_c: f64,
        alpha: f64,
        temp: f64,
        s_intrinsic: f64,
        params: &impl MaterialTransitionParams,
    ) -> Self {
        let q_reaction = params.reaction_enthalpy_j_per_kg();
        let x = 0.68 * alpha / (0.32 * alpha + w_c + 1e-6);
        let fc = s_intrinsic * x.powi(3);
        let psi = -q_reaction * alpha;

        ThermodynamicStateSnapshot {
            density: 2400.0 - 400.0 * w_c,
            temperature: temp,
            free_energy: psi,
            entropy: alpha * 0.1,
            reaction_extent: alpha,
            strength: fc,
        }
    }
}

/// Outcome parallel to prototype [`AdmissibilityResult`] fields before reason stringification.
#[allow(missing_docs)] // Legacy bool mirrors — prefer [`Self::conjunct_verdict`] / [`Self::is_accepted`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermodynamicTransitionOutcome {
    /// Primary discriminant — core ∧ material conjunct cluster.
    pub verdict: ConjunctVerdict,
    /// Legacy mirror of [`ConjunctVerdict::is_accepted`] — prefer [`Self::is_accepted`].
    #[deprecated(
        since = "0.2.0",
        note = "use ThermodynamicTransitionOutcome::is_accepted() or verdict.is_accepted()"
    )]
    pub accepted: bool,
    pub dissipation: f64,
    /// Legacy core conjunct witness — prefer [`CoreGateOutcome`] via open-system route.
    #[deprecated(
        since = "0.2.0",
        note = "use ThermodynamicTransitionOutcome::is_mass_conserved() or CoreGateOutcome::mass_conserved"
    )]
    pub mass_conserved: bool,
    /// Legacy CD ∧ strength fold — prefer [`Self::is_energy_positive`] / [`Self::rest_verdict`].
    #[deprecated(
        since = "0.2.0",
        note = "use ThermodynamicTransitionOutcome::is_energy_positive() or rest_verdict()"
    )]
    pub energy_positive: bool,
    /// Reaction-extent monotonicity (`gate_sdf` conjunct).
    #[deprecated(
        since = "0.2.0",
        note = "use ThermodynamicTransitionOutcome::is_reaction_extent_irreversible() or verdict"
    )]
    pub reaction_extent_irreversible: bool,
}

impl ThermodynamicTransitionOutcome {
    /// Borrow the primary [`ConjunctVerdict`] discriminant (FP P2.4 SSOT).
    #[inline]
    #[must_use]
    pub fn conjunct_verdict(self) -> ConjunctVerdict {
        self.verdict
    }

    /// Whether the composed transition cluster accepted (wire bytes unchanged).
    #[inline]
    #[must_use]
    pub fn is_accepted(self) -> bool {
        self.verdict.is_accepted()
    }

    /// Legacy mass-balance conjunct witness (unchanged semantics).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_mass_conserved(self) -> bool {
        self.mass_conserved
    }

    /// Legacy CD ∧ strength fold witness (unchanged semantics — not Core-only CD).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_energy_positive(self) -> bool {
        self.energy_positive
    }

    /// Legacy reaction-extent irreversibility witness (unchanged semantics).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_reaction_extent_irreversible(self) -> bool {
        self.reaction_extent_irreversible
    }

    /// REST-stable verdict via locked transition conjunct ladder (legacy `energy_positive` fold).
    #[allow(deprecated)]
    pub fn rest_verdict(&self) -> AdmissibilityVerdict {
        AdmissibilityVerdict::from_transition_conjuncts(
            self.accepted,
            self.mass_conserved,
            self.energy_positive,
        )
    }

    /// Alias for [`Self::rest_verdict`] — preserved for call-site compatibility.
    pub fn verdict(&self) -> AdmissibilityVerdict {
        self.rest_verdict()
    }
}

/// Stateful filter with tolerance and counters (prototype semantics).
#[derive(Debug, Clone)]
pub struct TransitionFilter {
    tolerance: f64,
    rejections: u64,
    acceptances: u64,
}

impl Default for TransitionFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionFilter {
    pub fn new() -> Self {
        TransitionFilter {
            tolerance: 1e-6,
            rejections: 0,
            acceptances: 0,
        }
    }

    pub fn with_tolerance(tolerance: f64) -> Self {
        TransitionFilter {
            tolerance,
            rejections: 0,
            acceptances: 0,
        }
    }

    /// Algorithm 1 from prototype: mass jump bound, `D_int = −ρ ψ̇`, strength monotonicity.
    ///
    /// Telemetry wrapper around [`transition_outcome`]; mutates accept/reject counters only.
    pub fn check_transition(
        &mut self,
        old_state: &ThermodynamicStateSnapshot,
        new_state: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> ThermodynamicTransitionOutcome {
        let outcome = transition_outcome(old_state, new_state, dt, self.tolerance);
        if outcome.is_accepted() {
            self.acceptances += 1;
        } else {
            self.rejections += 1;
        }
        outcome
    }

    pub fn acceptances(&self) -> u64 {
        self.acceptances
    }

    pub fn rejections(&self) -> u64 {
        self.rejections
    }

    pub fn reset_stats(&mut self) {
        self.acceptances = 0;
        self.rejections = 0;
    }
}

/// Default numeric tolerance for scalar transition gates (C-ABI and host evaluators).
pub const TRANSITION_TOLERANCE: f64 = 1e-6;

/// Mass jump band (kg/m³) — re-exported from Core gate (canonical owner: [`super::core_gate`]).
pub use super::core_gate::GATE_MASS_TOLERANCE_KG_M3;

#[must_use]
fn transition_snapshot_well_formed(s: &ThermodynamicStateSnapshot) -> bool {
    s.density.is_finite()
        && s.temperature.is_finite()
        && s.temperature > 0.0
        && s.free_energy.is_finite()
        && s.entropy.is_finite()
        && s.reaction_extent.is_finite()
        && s.strength.is_finite()
}

/// Pure transition evaluator: `(old, new, dt, ε) → outcome` with no filter state.
///
/// **Composition:** [`core_gate`] (Mass + CD, `P_input=0`) ∧ [`material_gate`] (strength + reaction).
/// Legacy field `energy_positive` bundles CD with strength monotonicity for parity — use
/// [`CoreGateOutcome`] / [`MaterialGateOutcome`] directly when you need the §17.3 split.
///
/// Aligns with [`thermodynamic_transition_admissible_tol`] and umst-math [`gate_sdf`].
/// Telemetry-only wrapper: [`TransitionFilter::check_transition`].
#[must_use]
#[allow(deprecated)]
pub fn transition_outcome(
    old_state: &ThermodynamicStateSnapshot,
    new_state: &ThermodynamicStateSnapshot,
    dt: f64,
    tolerance: f64,
) -> ThermodynamicTransitionOutcome {
    if !transition_snapshot_well_formed(old_state)
        || !transition_snapshot_well_formed(new_state)
        || !dt.is_finite()
        || dt <= 0.0
    {
        return ThermodynamicTransitionOutcome {
            verdict: ConjunctVerdict::Rejected(GateRejectReason::MalformedInput),
            accepted: false,
            dissipation: 0.0,
            mass_conserved: false,
            energy_positive: false,
            reaction_extent_irreversible: false,
        };
    }

    let mass_conserved =
        mass_conserved_between_densities(old_state.density, new_state.density);

    let response = scalar_response_from_transition(
        old_state.density,
        new_state.density,
        old_state.free_energy,
        new_state.free_energy,
        dt,
        0.0,
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

    // Legacy parity: `energy_positive` folds CD ∧ strength (not Core-only).
    let energy_positive = core.clausius_duhem && material.strength_monotonic;
    let accepted =
        core.mass_conserved && energy_positive && material.reaction_extent_irreversible;

    ThermodynamicTransitionOutcome {
        verdict,
        accepted,
        dissipation: core.dissipation,
        mass_conserved: core.mass_conserved,
        energy_positive,
        reaction_extent_irreversible: material.reaction_extent_irreversible,
    }
}

/// Pure transition predicate — explicit inputs → admissibility (no filter handle, no counters).
///
/// Matches the material-agnostic C-ABI semantics: mass jump bound, Clausius–Duhem
/// dissipation, reaction-extent irreversibility, strength monotonicity, and upper strength cap.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn thermodynamic_transition_admissible(
    old_density: f64,
    old_free_energy: f64,
    old_reaction_extent: f64,
    old_strength: f64,
    new_density: f64,
    new_free_energy: f64,
    new_reaction_extent: f64,
    new_strength: f64,
    new_max_strength: f64,
    dt: f64,
) -> bool {
    thermodynamic_transition_admissible_tol(
        old_density,
        old_free_energy,
        old_reaction_extent,
        old_strength,
        new_density,
        new_free_energy,
        new_reaction_extent,
        new_strength,
        new_max_strength,
        dt,
        TRANSITION_TOLERANCE,
    )
}

/// Tolerance-parameterized variant for tests and calibrated hosts.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn thermodynamic_transition_admissible_tol(
    old_density: f64,
    old_free_energy: f64,
    old_reaction_extent: f64,
    old_strength: f64,
    new_density: f64,
    new_free_energy: f64,
    new_reaction_extent: f64,
    new_strength: f64,
    new_max_strength: f64,
    dt: f64,
    tolerance: f64,
) -> bool {
    if !old_density.is_finite()
        || !old_free_energy.is_finite()
        || !old_reaction_extent.is_finite()
        || !old_strength.is_finite()
        || !new_density.is_finite()
        || !new_free_energy.is_finite()
        || !new_reaction_extent.is_finite()
        || !new_strength.is_finite()
        || !new_max_strength.is_finite()
        || !dt.is_finite()
        || dt <= 0.0
    {
        return false;
    }
    let mass_conserved = (new_density - old_density).abs() < GATE_MASS_TOLERANCE_KG_M3;
    let rho = (old_density + new_density) / 2.0;
    let psi_dot = (new_free_energy - old_free_energy) / (dt + 1e-10);
    let d_int = -rho * psi_dot;
    let strength_monotonic = new_strength >= old_strength - tolerance;
    let reaction_extent_irreversible = new_reaction_extent >= old_reaction_extent - tolerance;
    let strength_bounded = new_strength <= new_max_strength;
    mass_conserved
        && d_int >= -tolerance
        && strength_monotonic
        && reaction_extent_irreversible
        && strength_bounded
}

/// Convenience: evaluate directly from JSON-shaped proposals without building snapshots manually.
pub fn evaluate_transition(
    filter: &mut TransitionFilter,
    old: &TransitionScalars,
    new: &TransitionScalars,
    dt_seconds: f64,
) -> ThermodynamicTransitionOutcome {
    evaluate_transition_with_params(filter, old, new, dt_seconds, &SubstrateMaterialParams)
}

/// Pure proposal evaluation with an injected closure witness (no filter counters).
#[must_use]
pub fn evaluate_transition_pure_with_params(
    old: &TransitionScalars,
    new: &TransitionScalars,
    dt_seconds: f64,
    params: &impl MaterialTransitionParams,
    tolerance: f64,
) -> ThermodynamicTransitionOutcome {
    let old_s = old.thermodynamic_snapshot_with_params(params);
    let new_s = new.thermodynamic_snapshot_with_params(params);
    transition_outcome(&old_s, &new_s, dt_seconds, tolerance)
}

/// Proposal evaluation with injected witness; telemetry via optional filter counters.
pub fn evaluate_transition_with_params(
    filter: &mut TransitionFilter,
    old: &TransitionScalars,
    new: &TransitionScalars,
    dt_seconds: f64,
    params: &impl MaterialTransitionParams,
) -> ThermodynamicTransitionOutcome {
    let old_s = old.thermodynamic_snapshot_with_params(params);
    let new_s = new.thermodynamic_snapshot_with_params(params);
    filter.check_transition(&old_s, &new_s, dt_seconds)
}

#[deprecated(note = "renamed to evaluate_transition")]
pub fn evaluate_mix_transition(
    filter: &mut TransitionFilter,
    old: &TransitionScalars,
    new: &TransitionScalars,
    dt_seconds: f64,
) -> ThermodynamicTransitionOutcome {
    evaluate_transition(filter, old, new, dt_seconds)
}

#[deprecated(note = "renamed to TransitionScalars")]
pub type MixProposalScalars = TransitionScalars;

#[deprecated(note = "renamed to TransitionFilter")]
pub type ThermodynamicMixFilter = TransitionFilter;

#[cfg(test)]
#[allow(deprecated)]
mod transition_outcome_tests {
    use super::*;
    use umst_math::manifold::csg::{gate_sdf, thermo_gate_from_reaction_extent, ThermoGateState};

    fn snapshot_to_gate(s: &ThermodynamicStateSnapshot, max_strength: f64) -> ThermoGateState {
        thermo_gate_from_reaction_extent(
            s.density,
            s.free_energy,
            s.reaction_extent,
            s.strength,
            max_strength,
        )
    }

    #[test]
    fn transition_outcome_matches_thermodynamic_admissible_tol() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.35;
        new.free_energy = old.free_energy - 100.0;
        let dt = 1.0;
        let tol = TRANSITION_TOLERANCE;
        let outcome = transition_outcome(&old, &new, dt, tol);
        let adm = thermodynamic_transition_admissible_tol(
            old.density,
            old.free_energy,
            old.reaction_extent,
            old.strength,
            new.density,
            new.free_energy,
            new.reaction_extent,
            new.strength,
            80.0,
            dt,
            tol,
        );
        assert_eq!(outcome.is_accepted(), adm);
        assert!(outcome.is_reaction_extent_irreversible());
    }

    #[test]
    fn transition_outcome_rejects_reaction_extent_regression() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old;
        new.reaction_extent = 0.1;
        let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert!(!outcome.is_reaction_extent_irreversible());
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn transition_outcome_conjunct_verdict_matches_rest_ladder() {
        fn rest_from_conjunct(v: ConjunctVerdict) -> AdmissibilityVerdict {
            match v {
                ConjunctVerdict::Accepted => AdmissibilityVerdict::Accepted,
                ConjunctVerdict::Rejected(reason) => reason.to_rest_verdict(),
            }
        }

        let scenarios = [
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0),
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0),
            ),
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 40.0),
                {
                    let mut n = ThermodynamicStateSnapshot::from_mix_calibrated(
                        0.45, 0.5, 293.15, 40.0,
                    );
                    n.reaction_extent = 0.1;
                    n
                },
            ),
            (
                ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0),
                {
                    let mut n = ThermodynamicStateSnapshot::from_mix_calibrated(
                        0.45, 0.35, 293.15, 42.0,
                    );
                    n.strength = 10.0;
                    n
                },
            ),
        ];

        for (old, new) in scenarios {
            let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
            assert_eq!(
                outcome.verdict(),
                AdmissibilityVerdict::from_transition_conjuncts(
                    outcome.accepted,
                    outcome.is_mass_conserved(),
                    outcome.is_energy_positive(),
                ),
                "REST ladder must match stored bool conjuncts"
            );
            if outcome.is_accepted() {
                assert_eq!(outcome.verdict, ConjunctVerdict::Accepted);
            } else {
                assert_ne!(outcome.verdict, ConjunctVerdict::Accepted);
            }
            assert_eq!(
                rest_from_conjunct(outcome.verdict),
                outcome.verdict(),
                "composed ConjunctVerdict REST map must match legacy ladder"
            );
            if matches!(
                outcome.verdict,
                ConjunctVerdict::Rejected(GateRejectReason::StrengthRegression)
            ) {
                assert!(
                    !outcome.is_energy_positive(),
                    "strength regression must fold into energy_positive=false"
                );
                assert_eq!(
                    outcome.verdict(),
                    AdmissibilityVerdict::NegativeDissipation,
                    "strength regression → NEGATIVE_DISSIPATION via legacy fold"
                );
            }
        }
    }

    #[test]
    fn transition_outcome_rejects_malformed_input() {
        let idle = ThermodynamicStateSnapshot::new_idle();
        let outcome = transition_outcome(&idle, &idle, -1.0, TRANSITION_TOLERANCE);
        assert_eq!(
            outcome.verdict,
            ConjunctVerdict::Rejected(GateRejectReason::MalformedInput)
        );
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn transition_outcome_aligns_with_gate_sdf_sign() {
        let old = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.3, 293.15, 40.0);
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.35, 293.15, 42.0);
        let outcome = transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        let g = gate_sdf(&snapshot_to_gate(&old, 80.0), &snapshot_to_gate(&new, 80.0));
        if outcome.is_accepted() {
            assert!(g <= TRANSITION_TOLERANCE, "accepted ⇒ gate_sdf ≤ ε ({g})");
        }
    }
}

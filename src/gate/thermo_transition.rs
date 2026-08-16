// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Clausius–Duhem admissibility for bulk reaction-extent transitions (prototype thermodynamic gate).
//!
//! Ported from `umst-prototype/.../thermodynamic_filter.rs` — **wasm-free** manifold build.

use super::transition_proposal::{transition_outcome, ThermodynamicStateSnapshot};
use super::verdict::{AdmissibilityVerdict, ConjunctVerdict};
use crate::core::material_transition::{MaterialTransitionParams, SubstrateMaterialParams};

/// Result of thermodynamic admissibility check
#[allow(missing_docs)] // Legacy bool mirrors — prefer [`Self::conjunct_verdict`] / [`Self::is_accepted`]
#[derive(Clone, Debug)]
pub struct AdmissibilityResult {
    /// Primary discriminant — core ∧ material conjunct cluster.
    pub verdict: ConjunctVerdict,
    /// Legacy mirror of [`ConjunctVerdict::is_accepted`] — prefer [`Self::is_accepted`].
    #[deprecated(
        since = "0.2.0",
        note = "use AdmissibilityResult::is_accepted() or verdict.is_accepted()"
    )]
    pub accepted: bool,
    /// `D_int` value (W/m³) — Clausius–Duhem dissipation surrogate.
    pub dissipation: f64,
    /// Legacy core conjunct witness — prefer [`CoreGateOutcome`] via open-system route.
    #[deprecated(
        since = "0.2.0",
        note = "use CoreGateOutcome::mass_conserved or verdict reject reason"
    )]
    pub mass_conserved: bool,
    /// Legacy CD ∧ strength fold — prefer [`Self::rest_verdict`] / [`ConjunctVerdict`].
    #[deprecated(
        since = "0.2.0",
        note = "use rest_verdict() or ConjunctVerdict reject reason"
    )]
    pub energy_positive: bool,
}

impl AdmissibilityResult {
    /// Borrow the primary [`ConjunctVerdict`] discriminant (FP P2.4 SSOT).
    #[inline]
    #[must_use]
    pub fn conjunct_verdict(&self) -> ConjunctVerdict {
        self.verdict
    }

    /// Whether the composed transition cluster accepted (wire bytes unchanged).
    #[inline]
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.verdict.is_accepted()
    }

    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.is_accepted()
    }

    /// Legacy mass-balance conjunct witness (unchanged semantics).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_mass_conserved(&self) -> bool {
        self.mass_conserved
    }

    /// Legacy CD ∧ strength fold witness (unchanged semantics — not Core-only CD).
    #[inline]
    #[must_use]
    #[allow(deprecated)]
    pub fn is_energy_positive(&self) -> bool {
        self.energy_positive
    }

    /// REST-stable verdict via locked transition conjunct ladder (legacy `energy_positive` fold).
    pub fn rest_verdict(&self) -> AdmissibilityVerdict {
        AdmissibilityVerdict::from_transition_conjuncts(
            self.is_accepted(),
            self.is_mass_conserved(),
            self.is_energy_positive(),
        )
    }

    #[must_use]
    pub fn rejection_reason_code(&self) -> &'static str {
        self.rest_verdict().as_str()
    }
}

/// Thermodynamic state for admissibility checking (`f64`, host units).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ThermodynamicState {
    pub density: f64,         // kg/m³
    pub temperature: f64,     // K
    pub free_energy: f64,     // Helmholtz ψ (J/kg)
    pub entropy: f64,         // η (J/kg·K)
    pub reaction_extent: f64, // α (0-1)
    pub strength: f64,        // f_c (MPa)
}

impl ThermodynamicState {
    #[must_use]
    pub fn new() -> Self {
        ThermodynamicState {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.0,
            reaction_extent: 0.0,
            strength: 0.0,
        }
    }

    /// Create state from mix parameters using substrate-neutral defaults.
    #[must_use]
    pub fn from_mix(w_c: f64, alpha: f64, temp: f64) -> Self {
        Self::from_mix_with_params(w_c, alpha, temp, &SubstrateMaterialParams)
    }

    #[must_use]
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

    /// Create state from mix parameters using explicit intrinsic strength (MPa).
    #[must_use]
    pub fn from_mix_calibrated(w_c: f64, alpha: f64, temp: f64, s_intrinsic: f64) -> Self {
        Self::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            s_intrinsic,
            &SubstrateMaterialParams,
        )
    }

    #[must_use]
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

        ThermodynamicState {
            density: 2400.0 - 400.0 * w_c,
            temperature: temp,
            free_energy: psi,
            entropy: alpha * 0.1,
            reaction_extent: alpha,
            strength: fc,
        }
    }
}

impl Default for ThermodynamicState {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
fn thermodynamic_state_snapshot(state: &ThermodynamicState) -> ThermodynamicStateSnapshot {
    ThermodynamicStateSnapshot {
        density: state.density,
        temperature: state.temperature,
        free_energy: state.free_energy,
        entropy: state.entropy,
        reaction_extent: state.reaction_extent,
        strength: state.strength,
    }
}

/// Pure transition evaluator: `(old, new, dt, ε) → outcome` with no gate telemetry.
///
/// FP SSOT; [`ThermodynamicGate::check_transition`] wraps this with accept/reject counters.
#[must_use]
#[allow(deprecated)]
pub fn thermo_gate_transition_outcome(
    old_state: &ThermodynamicState,
    new_state: &ThermodynamicState,
    dt_s: f64,
    tolerance: f64,
) -> AdmissibilityResult {
    let outcome = transition_outcome(
        &thermodynamic_state_snapshot(old_state),
        &thermodynamic_state_snapshot(new_state),
        dt_s,
        tolerance,
    );
    AdmissibilityResult {
        verdict: outcome.conjunct_verdict(),
        accepted: outcome.is_accepted(),
        dissipation: outcome.dissipation,
        mass_conserved: outcome.is_mass_conserved(),
        energy_positive: outcome.is_energy_positive(),
    }
}

/// Thermodynamic gate (formerly `ThermodynamicFilter`).
#[derive(Clone, Debug)]
pub struct ThermodynamicGate {
    tolerance: f64,
    rejections: u64,
    acceptances: u64,
}

impl Default for ThermodynamicGate {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermodynamicGate {
    #[must_use]
    pub fn new() -> Self {
        ThermodynamicGate {
            tolerance: 1e-6,
            rejections: 0,
            acceptances: 0,
        }
    }

    #[must_use]
    pub fn with_tolerance(tolerance: f64) -> Self {
        ThermodynamicGate {
            tolerance,
            rejections: 0,
            acceptances: 0,
        }
    }

    /// Substantive transition check ported from Algorithm 1 in the UMST prototypes.
    #[must_use]
    pub fn transition_proposal_admissible(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> bool {
        self.check_transition(old_state, new_state, dt_s)
            .is_accepted()
    }

    /// Full transition evaluation with accounting statistics (telemetry wrapper).
    pub fn check_transition(
        &mut self,
        old_state: &ThermodynamicState,
        new_state: &ThermodynamicState,
        dt_s: f64,
    ) -> AdmissibilityResult {
        let result = thermo_gate_transition_outcome(old_state, new_state, dt_s, self.tolerance);
        if result.is_accepted() {
            self.acceptances += 1;
        } else {
            self.rejections += 1;
        }
        result
    }

    #[must_use]
    pub fn stats_summary(&self) -> String {
        let total = self.acceptances + self.rejections;
        if total == 0 {
            return "No transitions checked".to_string();
        }
        let rate = self.acceptances as f64 / total as f64 * 100.0;
        format!(
            "Accepted: {}, Rejected: {}, Rate: {:.1}%",
            self.acceptances, self.rejections, rate
        )
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::super::transition_proposal::TRANSITION_TOLERANCE;
    use super::super::verdict::AdmissibilityVerdict;
    use super::*;
    use crate::core::material_transition::SubstrateMaterialParams;

    /// Golden vectors from `docs/GOLDEN_FIXTURES.md` / `tests/gate_parity_fixture.rs`.
    fn golden_identity_admissible() -> (ThermodynamicState, ThermodynamicState, f64) {
        let s = ThermodynamicState {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        (s.clone(), s, 1.0)
    }

    /// Mass bound violation: `|Δρ| = 120` kg/m³ (registry band is `< 100`).
    fn golden_mass_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
        let old = ThermodynamicState {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.1,
            reaction_extent: 0.3,
            strength: 10.0,
        };
        let mut new = old.clone();
        new.density = 2280.0;
        (old, new, 3600.0)
    }

    /// Clausius–Duhem reject: free-energy spike breaks `D_int ≥ −tolerance`.
    fn golden_negative_dissipation_reject() -> (ThermodynamicState, ThermodynamicState, f64) {
        let old = ThermodynamicState {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let mut new = old.clone();
        new.free_energy = -1.0e4;
        (old, new, 1.0)
    }

    #[test]
    fn pure_outcome_matches_telemetry_path_without_counters() {
        let old =
            ThermodynamicState::from_mix_with_params(0.5, 0.4, 293.0, &SubstrateMaterialParams);
        let new =
            ThermodynamicState::from_mix_with_params(0.5, 0.65, 293.0, &SubstrateMaterialParams);
        let dt = 86_400.0;
        let pure = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        let mut gate = ThermodynamicGate::new();
        let telemetry = gate.check_transition(&old, &new, dt);
        assert_eq!(pure.is_accepted(), telemetry.is_accepted());
        assert_eq!(pure.is_mass_conserved(), telemetry.is_mass_conserved());
        assert_eq!(pure.is_energy_positive(), telemetry.is_energy_positive());
        assert!((pure.dissipation - telemetry.dissipation).abs() < 1e-12);
        assert!(gate.stats_summary().contains("Accepted: 1"));
    }

    #[test]
    fn golden_identity_admissible_accepted_via_pure_outcome() {
        let (old, new, dt) = golden_identity_admissible();
        let outcome = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(outcome.is_admissible());
        assert_eq!(outcome.rest_verdict(), AdmissibilityVerdict::Accepted);
        assert_eq!(
            outcome.rejection_reason_code(),
            AdmissibilityVerdict::ACCEPTED
        );
        assert!(outcome.dissipation.is_finite());
    }

    #[test]
    fn golden_mass_reject_maps_to_mass_violation() {
        let (old, new, dt) = golden_mass_reject();
        let outcome = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert!(!outcome.is_mass_conserved());
        assert_eq!(outcome.rest_verdict(), AdmissibilityVerdict::MassViolation);
        assert_eq!(
            outcome.rejection_reason_code(),
            AdmissibilityVerdict::MASS_VIOLATION
        );
    }

    #[test]
    fn golden_negative_dissipation_reject_maps_to_cd_token() {
        let (old, new, dt) = golden_negative_dissipation_reject();
        let outcome = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert_eq!(
            outcome.rest_verdict(),
            AdmissibilityVerdict::NegativeDissipation
        );
        assert_eq!(
            outcome.rejection_reason_code(),
            AdmissibilityVerdict::NEGATIVE_DISSIPATION
        );
    }

    #[test]
    fn gate_counters_track_accept_and_reject() {
        let (id_old, id_new, id_dt) = golden_identity_admissible();
        let (mass_old, mass_new, mass_dt) = golden_mass_reject();
        let mut gate = ThermodynamicGate::with_tolerance(TRANSITION_TOLERANCE);

        assert!(gate.check_transition(&id_old, &id_new, id_dt).is_accepted());
        assert!(!gate
            .check_transition(&mass_old, &mass_new, mass_dt)
            .is_accepted());

        let summary = gate.stats_summary();
        assert!(summary.contains("Accepted: 1"));
        assert!(summary.contains("Rejected: 1"));
        assert!(summary.contains("Rate: 50.0%"));
    }

    #[test]
    fn gate_stats_summary_empty_before_any_check() {
        let gate = ThermodynamicGate::new();
        assert_eq!(gate.stats_summary(), "No transitions checked");
    }

    #[test]
    fn transition_proposal_admissible_matches_check_transition() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicState::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let mut gate = ThermodynamicGate::new();
        let admissible = gate.transition_proposal_admissible(&old, &new, dt);
        let result = gate.check_transition(&old, &new, dt);
        assert_eq!(admissible, result.is_accepted());
        assert!(admissible);
    }

    #[test]
    fn hydration_progression_from_mix_calibrated_accepted() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.0, 293.15, 80.0);
        let new = ThermodynamicState::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let dt = 28.0 * 24.0 * 3600.0;
        let outcome = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert!(outcome.is_accepted());
        assert!(new.reaction_extent > old.reaction_extent);
        assert!(new.strength >= old.strength);
    }

    #[test]
    fn reaction_extent_regression_rejected() {
        let old = ThermodynamicState::from_mix_calibrated(0.45, 0.5, 293.15, 40.0);
        let mut new = old.clone();
        new.reaction_extent = 0.1;
        let outcome = thermo_gate_transition_outcome(&old, &new, 1.0, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
        assert_ne!(outcome.rest_verdict(), AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn malformed_dt_rejected() {
        let idle = ThermodynamicState::new();
        let outcome = thermo_gate_transition_outcome(&idle, &idle, -1.0, TRANSITION_TOLERANCE);
        assert!(!outcome.is_accepted());
    }

    #[test]
    fn thermodynamic_state_default_matches_new() {
        let d = ThermodynamicState::default();
        let n = ThermodynamicState::new();
        assert_eq!(d.density, n.density);
        assert_eq!(d.temperature, n.temperature);
        assert_eq!(d.free_energy, n.free_energy);
        assert_eq!(d.entropy, n.entropy);
        assert_eq!(d.reaction_extent, n.reaction_extent);
        assert_eq!(d.strength, n.strength);
    }

    #[test]
    fn from_mix_with_params_matches_calibrated_intrinsic() {
        let params = SubstrateMaterialParams;
        let w_c = 0.45;
        let alpha = 0.35;
        let temp = 293.15;
        let s_intrinsic = params.default_intrinsic_strength_mpa();
        let via_mix = ThermodynamicState::from_mix_with_params(w_c, alpha, temp, &params);
        let via_calibrated = ThermodynamicState::from_mix_calibrated_with_params(
            w_c,
            alpha,
            temp,
            s_intrinsic,
            &params,
        );
        assert_eq!(via_mix.density, via_calibrated.density);
        assert_eq!(via_mix.strength, via_calibrated.strength);
        assert_eq!(via_mix.free_energy, via_calibrated.free_energy);
        assert_eq!(via_mix.reaction_extent, alpha);
    }

    #[test]
    fn admissibility_result_conjunct_verdict_round_trips() {
        let (old, new, dt) = golden_identity_admissible();
        let outcome = thermo_gate_transition_outcome(&old, &new, dt, TRANSITION_TOLERANCE);
        assert_eq!(outcome.conjunct_verdict(), outcome.verdict);
        assert_eq!(
            outcome.conjunct_verdict().is_accepted(),
            outcome.is_accepted()
        );
    }

    #[test]
    fn w8e14_thermo_state_new_idle_is_finite() {
        let s = ThermodynamicState::new();
        assert!(s.density.is_finite());
        assert!(s.temperature.is_finite());
        assert!(s.free_energy.is_finite());
    }
}

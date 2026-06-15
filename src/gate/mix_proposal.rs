// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Plain (serde-free) mix / thermodynamic snapshots and transition checks ported from
//! `umst-prototype` `science/thermodynamic_filter.rs` — mass bound, Clausius–Duhem scalar gate,
//! and strength monotonicity under the Powers hydration model.

use super::verdict::AdmissibilityVerdict;

/// Default intrinsic gel strength for the Powers model (MPa).
pub const DEFAULT_S_INTRINSIC_MPA: f64 = 240.0;

/// Representative specific heat of hydration for OPC (J/kg).
pub const Q_HYDRATION_J_PER_KG: f64 = 450.0;

/// Minimal JSON-shaped proposal for a bulk mix patch (host gate IO).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MixProposalScalars {
    pub water_cement_ratio: f64,
    pub hydration_degree: f64,
    pub temperature_k: f64,
    /// Intrinsic gel strength (MPa); omit in JSON by sending `None` → [`DEFAULT_S_INTRINSIC_MPA`].
    pub s_intrinsic_mpa: Option<f64>,
}

impl MixProposalScalars {
    pub fn thermodynamic_snapshot(&self) -> ThermodynamicStateSnapshot {
        ThermodynamicStateSnapshot::from_mix_calibrated(
            self.water_cement_ratio,
            self.hydration_degree,
            self.temperature_k,
            self.s_intrinsic_mpa.unwrap_or(DEFAULT_S_INTRINSIC_MPA),
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
    pub hydration_degree: f64,
    pub strength: f64,
}

impl ThermodynamicStateSnapshot {
    pub fn new_idle() -> Self {
        ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.0,
            free_energy: 0.0,
            entropy: 0.0,
            hydration_degree: 0.0,
            strength: 0.0,
        }
    }

    pub fn from_mix(w_c: f64, alpha: f64, temp: f64) -> Self {
        Self::from_mix_calibrated(w_c, alpha, temp, DEFAULT_S_INTRINSIC_MPA)
    }

    pub fn from_mix_calibrated(w_c: f64, alpha: f64, temp: f64, s_intrinsic: f64) -> Self {
        let x = 0.68 * alpha / (0.32 * alpha + w_c + 1e-6);
        let fc = s_intrinsic * x.powi(3);
        let psi = -Q_HYDRATION_J_PER_KG * alpha;

        ThermodynamicStateSnapshot {
            density: 2400.0 - 400.0 * w_c,
            temperature: temp,
            free_energy: psi,
            entropy: alpha * 0.1,
            hydration_degree: alpha,
            strength: fc,
        }
    }
}

/// Outcome parallel to prototype [`AdmissibilityResult`] fields before reason stringification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermodynamicTransitionOutcome {
    pub accepted: bool,
    pub dissipation: f64,
    pub mass_conserved: bool,
    pub energy_positive: bool,
}

impl ThermodynamicTransitionOutcome {
    pub fn verdict(&self) -> AdmissibilityVerdict {
        AdmissibilityVerdict::from_thermo_flags(
            self.accepted,
            self.mass_conserved,
            self.energy_positive,
        )
    }
}

/// Stateful filter with tolerance and counters (prototype semantics).
#[derive(Debug, Clone)]
pub struct ThermodynamicMixFilter {
    tolerance: f64,
    rejections: u64,
    acceptances: u64,
}

impl Default for ThermodynamicMixFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermodynamicMixFilter {
    pub fn new() -> Self {
        ThermodynamicMixFilter {
            tolerance: 1e-6,
            rejections: 0,
            acceptances: 0,
        }
    }

    pub fn with_tolerance(tolerance: f64) -> Self {
        ThermodynamicMixFilter {
            tolerance,
            rejections: 0,
            acceptances: 0,
        }
    }

    /// Algorithm 1 from prototype: mass jump bound, `D_int = −ρ ψ̇`, strength monotonicity.
    pub fn check_transition(
        &mut self,
        old_state: &ThermodynamicStateSnapshot,
        new_state: &ThermodynamicStateSnapshot,
        dt: f64,
    ) -> ThermodynamicTransitionOutcome {
        if !transition_snapshot_well_formed(old_state)
            || !transition_snapshot_well_formed(new_state)
            || !dt.is_finite()
            || dt <= 0.0
        {
            self.rejections += 1;
            return ThermodynamicTransitionOutcome {
                accepted: false,
                dissipation: 0.0,
                mass_conserved: false,
                energy_positive: false,
            };
        }

        let mass_conserved = (new_state.density - old_state.density).abs() < 100.0;

        let rho = (old_state.density + new_state.density) / 2.0;
        let psi_dot = (new_state.free_energy - old_state.free_energy) / (dt + 1e-10);
        let d_int = -rho * psi_dot;

        let strength_valid = new_state.strength >= old_state.strength - self.tolerance;

        let energy_positive = d_int >= -self.tolerance && strength_valid;
        let accepted = mass_conserved && energy_positive;

        if accepted {
            self.acceptances += 1;
        } else {
            self.rejections += 1;
        }

        ThermodynamicTransitionOutcome {
            accepted,
            dissipation: d_int,
            mass_conserved,
            energy_positive,
        }
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

#[must_use]
fn transition_snapshot_well_formed(s: &ThermodynamicStateSnapshot) -> bool {
    s.density.is_finite()
        && s.temperature.is_finite()
        && s.temperature > 0.0
        && s.free_energy.is_finite()
        && s.entropy.is_finite()
        && s.hydration_degree.is_finite()
        && s.strength.is_finite()
}

/// Pure transition predicate — explicit inputs → admissibility (no filter handle, no counters).
///
/// Matches the material-agnostic C-ABI semantics: mass jump bound, Clausius–Duhem
/// dissipation, hydration irreversibility, strength monotonicity, and upper strength cap.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn thermodynamic_transition_admissible(
    old_density: f64,
    old_free_energy: f64,
    old_hydration: f64,
    old_strength: f64,
    new_density: f64,
    new_free_energy: f64,
    new_hydration: f64,
    new_strength: f64,
    new_max_strength: f64,
    dt: f64,
) -> bool {
    thermodynamic_transition_admissible_tol(
        old_density,
        old_free_energy,
        old_hydration,
        old_strength,
        new_density,
        new_free_energy,
        new_hydration,
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
    old_hydration: f64,
    old_strength: f64,
    new_density: f64,
    new_free_energy: f64,
    new_hydration: f64,
    new_strength: f64,
    new_max_strength: f64,
    dt: f64,
    tolerance: f64,
) -> bool {
    if !old_density.is_finite()
        || !old_free_energy.is_finite()
        || !old_hydration.is_finite()
        || !old_strength.is_finite()
        || !new_density.is_finite()
        || !new_free_energy.is_finite()
        || !new_hydration.is_finite()
        || !new_strength.is_finite()
        || !new_max_strength.is_finite()
        || !dt.is_finite()
        || dt <= 0.0
    {
        return false;
    }
    let mass_conserved = (new_density - old_density).abs() < 100.0;
    let rho = (old_density + new_density) / 2.0;
    let psi_dot = (new_free_energy - old_free_energy) / (dt + 1e-10);
    let d_int = -rho * psi_dot;
    let strength_monotonic = new_strength >= old_strength - tolerance;
    let hydration_irreversible = new_hydration >= old_hydration - tolerance;
    let strength_bounded = new_strength <= new_max_strength;
    mass_conserved
        && d_int >= -tolerance
        && strength_monotonic
        && hydration_irreversible
        && strength_bounded
}

/// Convenience: evaluate directly from JSON-shaped proposals without building snapshots manually.
pub fn evaluate_mix_transition(
    filter: &mut ThermodynamicMixFilter,
    old: &MixProposalScalars,
    new: &MixProposalScalars,
    dt_seconds: f64,
) -> ThermodynamicTransitionOutcome {
    let old_s = old.thermodynamic_snapshot();
    let new_s = new.thermodynamic_snapshot();
    filter.check_transition(&old_s, &new_s, dt_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admissible_forward_hydration() {
        let mut filter = ThermodynamicMixFilter::new();
        let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
        let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
        assert!(new.free_energy < old.free_energy);
        let r = filter.check_transition(&old, &new, 3600.0);
        assert!(r.accepted);
        assert!(r.dissipation > 0.0);
        assert_eq!(r.verdict(), AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn reject_reverse_hydration() {
        let mut filter = ThermodynamicMixFilter::new();
        let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.7, 293.0);
        let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
        let r = filter.check_transition(&old, &new, 3600.0);
        assert!(!r.accepted);
        assert!(r.dissipation < 0.0);
    }

    #[test]
    fn strength_monotonicity() {
        let mut filter = ThermodynamicMixFilter::new();
        let mut old = ThermodynamicStateSnapshot::new_idle();
        old.strength = 30.0;
        old.hydration_degree = 0.5;
        let mut new = ThermodynamicStateSnapshot::new_idle();
        new.strength = 25.0;
        new.hydration_degree = 0.5;
        let r = filter.check_transition(&old, &new, 1.0);
        assert!(!r.accepted);
    }

    #[test]
    fn pure_gate_matches_filter_forward_hydration() {
        let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
        let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.5, 293.0);
        assert!(thermodynamic_transition_admissible(
            old.density,
            old.free_energy,
            old.hydration_degree,
            old.strength,
            new.density,
            new.free_energy,
            new.hydration_degree,
            new.strength,
            240.0,
            3600.0,
        ));
    }

    #[test]
    fn pure_gate_rejects_reverse_hydration() {
        let old = ThermodynamicStateSnapshot::from_mix(0.5, 0.7, 293.0);
        let new = ThermodynamicStateSnapshot::from_mix(0.5, 0.3, 293.0);
        assert!(!thermodynamic_transition_admissible(
            old.density,
            old.free_energy,
            old.hydration_degree,
            old.strength,
            new.density,
            new.free_energy,
            new.hydration_degree,
            new.strength,
            240.0,
            3600.0,
        ));
    }

    #[test]
    fn dissipation_matches_rho_q_alpha_dot() {
        let mut filter = ThermodynamicMixFilter::new();
        let w_c = 0.45;
        let alpha_old = 0.4;
        let alpha_new = 0.6;
        let dt = 7.0 * 86400.0;
        let old = ThermodynamicStateSnapshot::from_mix(w_c, alpha_old, 293.0);
        let new = ThermodynamicStateSnapshot::from_mix(w_c, alpha_new, 293.0);
        let r = filter.check_transition(&old, &new, dt);

        let rho = (old.density + new.density) / 2.0;
        let alpha_dot = (alpha_new - alpha_old) / dt;
        let expected = rho * Q_HYDRATION_J_PER_KG * alpha_dot;
        let rel_err = ((r.dissipation - expected) / expected).abs();
        assert!(
            rel_err < 1e-10,
            "got {} expected {}",
            r.dissipation,
            expected
        );
    }
}

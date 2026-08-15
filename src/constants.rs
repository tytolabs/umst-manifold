// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Shared physical constants. With feature `math-constants`, Landauer bit energy is SSOT from `umst-math`.
//!
//! W29-019 deepen — honest SSOT fences only; does **not** claim production wiring or MASTER retick.

/// W29-019 cell id (manifold constants deepen).
pub const CONSTANTS_CELL_ID: &str = "W29-019-CONSTANTS";

/// Honest posture — runtime Landauer helper only; no GREEN invent (`MASTER_RETICK=no`).
pub const CONSTANTS_POSTURE_TAG: &str = "honest-physical-constants-ssot-only";

/// CODATA 2018 Boltzmann constant (J/K) — fallback when `math-constants` is off.
/// SSOT: `umst-math::landauer::K_B` when feature enabled.
pub const K_BOLTZMANN_FALLBACK_J_PER_K: f64 = 1.380_649e-23;

/// Operator ambient reference temperature (K) — aligns with `umst-math::landauer_registry::HOST_TEMPERATURE_REFERENCE_K`.
pub const AMBIENT_REFERENCE_TEMPERATURE_K: f64 = 300.0;

/// Landauer bit energy `k_B T ln 2` (joules).
///
/// With `math-constants`, delegates to [`umst_math::landauer::landauer_bit_energy_joules`].
/// Non-finite `temperature_k` yields NaN (no panic).
#[must_use]
pub fn landauer_bit_energy_joules(temperature_k: f64) -> f64 {
    #[cfg(feature = "math-constants")]
    {
        match ordered_float::NotNan::new(temperature_k) {
            Ok(t) => umst_math::landauer::landauer_bit_energy_joules(t).into_inner(),
            Err(_) => f64::NAN,
        }
    }
    #[cfg(not(feature = "math-constants"))]
    {
        if !temperature_k.is_finite() {
            return f64::NAN;
        }
        K_BOLTZMANN_FALLBACK_J_PER_K * temperature_k * std::f64::consts::LN_2
    }
}

/// Landauer bit energy at [`AMBIENT_REFERENCE_TEMPERATURE_K`] (joules).
#[must_use]
pub fn landauer_bit_energy_ambient_joules() -> f64 {
    landauer_bit_energy_joules(AMBIENT_REFERENCE_TEMPERATURE_K)
}

/// Honest constants deepen probe — prep wired; production flip blocked.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantsDeepenProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub k_b_fallback_j_per_k: f64,
    pub ambient_reference_k: f64,
    pub math_constants_feature: bool,
    pub production_wired: bool,
    pub master_retick_eligible: bool,
}

/// Static honest probe — no fake GREEN invent.
#[must_use]
pub fn constants_deepen_probe() -> ConstantsDeepenProbe {
    ConstantsDeepenProbe {
        cell_id: CONSTANTS_CELL_ID,
        posture_tag: CONSTANTS_POSTURE_TAG,
        k_b_fallback_j_per_k: K_BOLTZMANN_FALLBACK_J_PER_K,
        ambient_reference_k: AMBIENT_REFERENCE_TEMPERATURE_K,
        math_constants_feature: cfg!(feature = "math-constants"),
        production_wired: constants_production_wired(),
        master_retick_eligible: constants_master_retick_eligible(),
    }
}

/// Honest `production_wired` fence — never true until measured live wire proof.
#[must_use]
pub const fn constants_production_wired() -> bool {
    false
}

/// Master retick eligible — false @ constants-only deepen pass.
#[must_use]
pub const fn constants_master_retick_eligible() -> bool {
    false
}

/// Honesty gate for operator receipts.
#[must_use]
pub fn constants_deepen_honest(probe: &ConstantsDeepenProbe) -> bool {
    probe.cell_id == CONSTANTS_CELL_ID
        && probe.posture_tag == CONSTANTS_POSTURE_TAG
        && probe.k_b_fallback_j_per_k == K_BOLTZMANN_FALLBACK_J_PER_K
        && probe.ambient_reference_k == AMBIENT_REFERENCE_TEMPERATURE_K
        && !probe.production_wired
        && !probe.master_retick_eligible
        && !constants_production_wired()
        && !constants_master_retick_eligible()
}

/// Whether the constants morphism pins are stable @ HEAD.
#[must_use]
pub fn constants_morphism_pinned() -> bool {
    CONSTANTS_CELL_ID == "W29-019-CONSTANTS"
        && CONSTANTS_POSTURE_TAG == "honest-physical-constants-ssot-only"
        && K_BOLTZMANN_FALLBACK_J_PER_K == 1.380_649e-23
        && AMBIENT_REFERENCE_TEMPERATURE_K == 300.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EXPECTED_LANDAUER_300K_J: f64 = 1.380_649e-23 * 300.0 * std::f64::consts::LN_2;

    #[test]
    fn constants_cell_metadata_pinned() {
        assert!(constants_morphism_pinned());
        assert_eq!(CONSTANTS_CELL_ID, "W29-019-CONSTANTS");
        assert_eq!(CONSTANTS_POSTURE_TAG, "honest-physical-constants-ssot-only");
    }

    #[test]
    fn constants_deepen_probe_honest_fences_hold() {
        let probe = constants_deepen_probe();
        assert!(constants_deepen_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.master_retick_eligible);
        assert!(!constants_production_wired());
        assert!(!constants_master_retick_eligible());
    }

    #[test]
    fn landauer_bit_energy_300k_matches_codata_fallback() {
        let e = landauer_bit_energy_joules(AMBIENT_REFERENCE_TEMPERATURE_K);
        assert_relative_eq!(e, EXPECTED_LANDAUER_300K_J, epsilon = 1.0e-30);
        assert_relative_eq!(landauer_bit_energy_ambient_joules(), e, epsilon = 1.0e-30);
    }

    #[test]
    fn landauer_bit_energy_scales_linearly_with_temperature() {
        let t = 150.0;
        let e = landauer_bit_energy_joules(t);
        assert_relative_eq!(
            e,
            landauer_bit_energy_joules(300.0) / 2.0,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn landauer_bit_energy_zero_at_zero_kelvin() {
        assert_relative_eq!(landauer_bit_energy_joules(0.0), 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn landauer_bit_energy_nan_for_non_finite_temperature() {
        assert!(landauer_bit_energy_joules(f64::NAN).is_nan());
        assert!(landauer_bit_energy_joules(f64::INFINITY).is_nan());
        assert!(landauer_bit_energy_joules(f64::NEG_INFINITY).is_nan());
    }

    #[test]
    fn k_boltzmann_fallback_matches_codata_2018() {
        assert_relative_eq!(
            K_BOLTZMANN_FALLBACK_J_PER_K,
            1.380_649e-23,
            epsilon = 1.0e-30
        );
    }

    #[cfg(feature = "math-constants")]
    #[test]
    fn landauer_bit_energy_math_constants_path_matches_umst_math() {
        let t = 273.15;
        let runtime = landauer_bit_energy_joules(t);
        let ssot =
            umst_math::landauer::landauer_bit_energy_joules(ordered_float::NotNan::new(t).unwrap())
                .into_inner();
        assert_relative_eq!(runtime, ssot, epsilon = 1.0e-30);
        assert_relative_eq!(
            K_BOLTZMANN_FALLBACK_J_PER_K,
            umst_math::landauer::K_B,
            epsilon = 1.0e-30
        );
    }

    #[test]
    fn w29_019_constants_fleet_verify() {
        let probe = constants_deepen_probe();
        assert_eq!(probe.cell_id, "W29-019-CONSTANTS");
        assert!(constants_deepen_honest(&probe));
        assert!(constants_morphism_pinned());
    }
}

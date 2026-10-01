// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Mix-calibrated bulk density closure registry for gate thermodynamic snapshots.
//!
//! Grounded rows mirror prototype `thermodynamic_filter` idle / `from_mix_calibrated` closures and
//! golden vectors in `docs/GOLDEN_FIXTURES.md`. Fixture SSOT only — not physics GREEN.

use crate::constants_registry::GroundedConst;

/// Mix-calibrated bulk density closure for gate thermodynamic snapshots.
pub struct MixCalibratedDensityClosure;

/// Mix-calibration reference bath @ 20 °C (K).
pub const MIX_CALIBRATION_REFERENCE_TEMPERATURE_K: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_calibration_reference_temperature_k",
    value: 293.15,
    evidence: "docs/GOLDEN_FIXTURES.md; 20 °C bath for mix-calibrated gate snapshots",
};

/// Idle gate snapshot surface temperature (K).
pub const MIX_IDLE_SURFACE_TEMPERATURE_K: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_idle_surface_temperature_k",
    value: 293.0,
    evidence: "ThermodynamicStateSnapshot::new_idle default temperature",
};

/// Substrate reference bulk density (kg/m³).
pub const SUBSTRATE_REFERENCE_DENSITY_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_substrate_reference_density_kg_m3",
    value: 2400.0,
    evidence: "prototype bulk idle density; gate golden vectors",
};

/// Density slope vs water mass fraction in mix closure (kg/m³ per unit `w_c`).
pub const MIX_WATER_FRACTION_DENSITY_SLOPE_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_water_fraction_density_slope_kg_m3",
    value: 400.0,
    evidence: "from_mix_calibrated closure ρ = ρ₀ − slope · w_c",
};

/// Strength closure numerator coefficient on α.
pub const MIX_STRENGTH_NUMERATOR_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_numerator_alpha_coeff",
    value: 0.68,
    evidence: "prototype strength closure x = 0.68 α / (...)",
};

/// Strength closure denominator coefficient on α.
pub const MIX_STRENGTH_DENOM_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_denom_alpha_coeff",
    value: 0.32,
    evidence: "prototype strength closure denominator",
};

/// Entropy closure η = coeff · α.
pub const MIX_ENTROPY_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_entropy_alpha_coeff",
    value: 0.1,
    evidence: "prototype entropy η = 0.1 α",
};

/// Regularizer in strength-closure denominator.
pub const MIX_STRENGTH_DENOM_REGULARIZER: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_denom_regularizer",
    value: 1e-6,
    evidence: "numerical floor in strength closure denominator",
};

/// Compute mix-calibrated bulk density (kg/m³) from water mass fraction.
#[must_use]
pub fn mix_calibrated_density_kg_m3(w_c: f64) -> f64 {
    SUBSTRATE_REFERENCE_DENSITY_KG_M3.value
        - MIX_WATER_FRACTION_DENSITY_SLOPE_KG_M3.value * w_c
}

/// Strength closure scalar `x` before cubic power.
#[must_use]
pub fn mix_strength_closure_x(w_c: f64, alpha: f64) -> f64 {
    let denom = MIX_STRENGTH_DENOM_ALPHA_COEFF.value * alpha
        + w_c
        + MIX_STRENGTH_DENOM_REGULARIZER.value;
    MIX_STRENGTH_NUMERATOR_ALPHA_COEFF.value * alpha / denom
}

#[cfg(test)]
mod mix_substrate_fixture_density_ssot {
    use super::*;
    use crate::constants::CELSIUS_TO_KELVIN_OFFSET_K;

    #[test]
    fn mix_calibration_bath_is_twenty_celsius_in_kelvin() {
        let expected = CELSIUS_TO_KELVIN_OFFSET_K + 20.0;
        assert!(
            (MIX_CALIBRATION_REFERENCE_TEMPERATURE_K.value - expected).abs() < 1e-9,
            "mix bath should match 20 °C offset"
        );
    }

    #[test]
    fn mix_calibrated_density_matches_linear_closure() {
        let w_c = 0.45;
        let rho = mix_calibrated_density_kg_m3(w_c);
        let linear = SUBSTRATE_REFERENCE_DENSITY_KG_M3.value
            - MIX_WATER_FRACTION_DENSITY_SLOPE_KG_M3.value * w_c;
        assert!((rho - linear).abs() < 1e-9);
    }

    #[test]
    fn mix_strength_closure_x_is_positive_for_fixture_alpha() {
        let x = mix_strength_closure_x(0.45, 0.3);
        assert!(x.is_finite() && x > 0.0);
    }
}

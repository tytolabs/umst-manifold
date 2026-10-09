// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Mix-calibrated bulk density closure registry for gate thermodynamic snapshots.
//!
//! Rows of the prototype `thermodynamic_filter` idle / `from_mix_calibrated` closures, each with its
//! derivation: the bath is the ISO 554 reference, numerical guards and default states are policy, and
//! coefficients without a typed citation, proof or measurement are typed absences.

use crate::constants_registry::{Derivation, GroundedConst};
use umst_math::constants::tier1_derivation;

/// Mix-calibration reference bath @ 20 °C (K).
pub const MIX_CALIBRATION_REFERENCE_TEMPERATURE_K: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_calibration_reference_temperature_k",
    value: tier1_derivation::REFERENCE_TEMPERATURE_293_15_K,
    evidence: "umst-math reference_temperature_293_15_k: ISO 554 20 °C plus the SI offset 273.15 K",
    derivation: Some(Derivation::Definition {
        authority_url: tier1_derivation::REFERENCE_TEMPERATURE_293_15_K_AUTHORITY_URL,
        expected_sha256: tier1_derivation::REFERENCE_TEMPERATURE_293_15_K_AUTHORITY_SHA256,
    }),
};

/// Idle gate snapshot surface temperature (K).
pub const MIX_IDLE_SURFACE_TEMPERATURE_K: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_idle_surface_temperature_k",
    value: 293.0,
    evidence: "ThermodynamicStateSnapshot::new_idle default temperature",
    derivation: Some(Derivation::Policy {
        rationale: "temperature an idle snapshot starts at before a host temperature is read (prototype ThermodynamicStateSnapshot::new_idle); a default state, not a property of the material",
    }),
};

/// Substrate reference bulk density (kg/m³).
pub const SUBSTRATE_REFERENCE_DENSITY_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_substrate_reference_density_kg_m3",
    value: 2400.0,
    evidence: "prototype bulk idle density; gate golden vectors",
    derivation: Some(Derivation::Absent {
        reason: "bulk density of the idle substrate from the prototype, with no cited source or measured receipt; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// Density slope vs water mass fraction in mix closure (kg/m³ per unit `w_c`).
pub const MIX_WATER_FRACTION_DENSITY_SLOPE_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_water_fraction_density_slope_kg_m3",
    value: 400.0,
    evidence: "from_mix_calibrated closure ρ = ρ₀ − slope · w_c",
    derivation: Some(Derivation::Absent {
        reason: "slope of the prototype linear density closure in the water fraction, with no cited source, derivation or measured receipt; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// Strength closure numerator coefficient on α.
pub const MIX_STRENGTH_NUMERATOR_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_numerator_alpha_coeff",
    value: 0.68,
    evidence: "prototype strength closure x = 0.68 α / (...)",
    derivation: Some(Derivation::Absent {
        reason: "Powers (1958) gel-space coefficient 0.68, the model umst-formal states as Concrete.Powers.gelSpaceRatio; the Derivation type has no Cited form and no theorem fixes the coefficient, so the citation lands with a Cited form; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// Strength closure denominator coefficient on α.
pub const MIX_STRENGTH_DENOM_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_denom_alpha_coeff",
    value: 0.32,
    evidence: "prototype strength closure denominator",
    derivation: Some(Derivation::Absent {
        reason: "Powers (1958) gel-space coefficient 0.32, the model umst-formal states as Concrete.Powers.gelSpaceRatio; the Derivation type has no Cited form and no theorem fixes the coefficient, so the citation lands with a Cited form; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// Entropy closure η = coeff · α.
pub const MIX_ENTROPY_ALPHA_COEFF: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_entropy_alpha_coeff",
    value: 0.1,
    evidence: "prototype entropy η = 0.1 α",
    derivation: Some(Derivation::Absent {
        reason: "entropy per unit reaction extent of the prototype closure, with no cited source, derivation or measured receipt; follow-up W-119 MANIFOLD-GROUNDEDCONST-DERIVATION",
    }),
};

/// Regularizer in strength-closure denominator.
pub const MIX_STRENGTH_DENOM_REGULARIZER: GroundedConst<f64> = GroundedConst {
    name: "gate_mix_strength_denom_regularizer",
    value: 1e-6,
    evidence: "numerical floor in strength closure denominator",
    derivation: Some(Derivation::Policy {
        rationale: "floor added to the gel-space denominator so a zero water fraction at zero extent does not divide by zero; a numerical guard, not physics",
    }),
};

/// Compute mix-calibrated bulk density (kg/m³) from water mass fraction.
#[must_use]
pub fn mix_calibrated_density_kg_m3(w_c: f64) -> f64 {
    SUBSTRATE_REFERENCE_DENSITY_KG_M3.value - MIX_WATER_FRACTION_DENSITY_SLOPE_KG_M3.value * w_c
}

/// Strength closure scalar `x` before cubic power: the Powers gel-space ratio `UMST.gelSpaceRatio`
/// (umst-formal `Concrete.Powers`, `68·α / (32·α + 100·w_c)`) with the denominator floor added.
#[must_use]
pub fn mix_strength_closure_x(w_c: f64, alpha: f64) -> f64 {
    let denom =
        MIX_STRENGTH_DENOM_ALPHA_COEFF.value * alpha + w_c + MIX_STRENGTH_DENOM_REGULARIZER.value;
    MIX_STRENGTH_NUMERATOR_ALPHA_COEFF.value * alpha / denom
}

#[cfg(test)]
mod mix_substrate_fixture_density_ssot {
    use super::*;

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

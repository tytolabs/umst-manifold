// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Gate parity / census golden scalars — SSOT for repeated mix-calibrated fixture literals.
//!
//! Values align with `docs/GOLDEN_FIXTURES.md` and inline gate module census tests.
//! Fixture SSOT only — not physics GREEN.

use crate::constants_registry::GroundedConst;

/// Census fixture binder–liquid mass fraction `w_c`.
pub const CENSUS_BINDER_LIQUID_RATIO: GroundedConst<f64> = GroundedConst {
    name: "gate_census_binder_liquid_ratio",
    value: 0.45,
    evidence: "docs/GOLDEN_FIXTURES.md; repeated from_mix_calibrated census",
    derivation: None,
};

/// Low hydration reaction extent in census transitions.
pub const CENSUS_REACTION_EXTENT_LOW: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_low",
    value: 0.3,
    evidence: "gate census calibrated transition (low α)",
    derivation: None,
};

/// Mid hydration reaction extent in census transitions.
pub const CENSUS_REACTION_EXTENT_MID: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_mid",
    value: 0.35,
    evidence: "gate census strength-monotone accept path",
    derivation: None,
};

/// High hydration reaction extent in census transitions.
pub const CENSUS_REACTION_EXTENT_HIGH: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_high",
    value: 0.5,
    evidence: "gate census open-system / CD fixtures",
    derivation: None,
};

/// Near-complete hydration extent (strength regression witness).
pub const CENSUS_REACTION_EXTENT_NEAR_COMPLETE: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_near_complete",
    value: 0.9,
    evidence: "gate census strength regression reject",
    derivation: None,
};

/// Over-hydration extent in open-system census.
pub const CENSUS_REACTION_EXTENT_OVER: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_over",
    value: 0.55,
    evidence: "gate census open-system power-input fixture",
    derivation: None,
};

/// Intrinsic strength scale (MPa) for census `s_intrinsic` arguments.
pub const CENSUS_INTRINSIC_STRENGTH_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_intrinsic_strength_mpa",
    value: 40.0,
    evidence: "prototype D1 intrinsic scale in gate census",
    derivation: None,
};

/// Post-cure strength (MPa) in monotone-accept census.
pub const CENSUS_STRENGTH_MPA_MID: GroundedConst<f64> = GroundedConst {
    name: "gate_census_strength_mpa_mid",
    value: 42.0,
    evidence: "gate census material conjunct accept",
    derivation: None,
};

/// Catalog / manifest intrinsic strength default (MPa).
pub const CENSUS_STRENGTH_INTRINSIC_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_strength_intrinsic_mpa",
    value: 80.0,
    evidence: "HTTP manifest + snapshot_to_gate strength cap",
    derivation: None,
};

/// Strength regression floor (MPa) in census rejects.
pub const CENSUS_STRENGTH_REGRESSION_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_strength_regression_mpa",
    value: 10.0,
    evidence: "gate census strength regression",
    derivation: None,
};

/// Open-system census strength after dissipation (MPa).
pub const CENSUS_STRENGTH_OPEN_SYSTEM_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_strength_open_system_mpa",
    value: 30.0,
    evidence: "gate census open-system dissipation path",
    derivation: None,
};

/// Mix-calibrated bulk density at census `w_c` (kg/m³).
pub const CENSUS_MIX_CALIBRATED_DENSITY_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_census_mix_calibrated_density_kg_m3",
    value: 2220.0,
    evidence: "from_mix_calibrated(0.45, …) host density witness",
    derivation: None,
};

/// Mass-reject golden new density (kg/m³).
pub const CENSUS_MASS_REJECT_DENSITY_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_census_mass_reject_density_kg_m3",
    value: 2280.0,
    evidence: "tests/gate_parity_fixture golden_mass_reject",
    derivation: None,
};

/// Negative-dissipation golden density (kg/m³).
pub const CENSUS_NEGATIVE_DISSIPATION_DENSITY_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_census_negative_dissipation_density_kg_m3",
    value: 2200.0,
    evidence: "tests/gate_parity_fixture golden_negative_dissipation",
    derivation: None,
};

/// Golden identity admissible reaction extent.
pub const CENSUS_GOLDEN_IDENTITY_REACTION_EXTENT: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_identity_reaction_extent",
    value: 0.42,
    evidence: "golden_identity_admissible vector",
    derivation: None,
};

/// Golden identity admissible strength (MPa).
pub const CENSUS_GOLDEN_IDENTITY_STRENGTH_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_identity_strength_mpa",
    value: 12.7,
    evidence: "golden_identity_admissible vector",
    derivation: None,
};

/// Golden identity admissible entropy (J/K/kg scale in snapshot).
pub const CENSUS_GOLDEN_IDENTITY_ENTROPY: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_identity_entropy",
    value: 0.05,
    evidence: "golden_identity_admissible vector",
    derivation: None,
};

/// Golden mass-reject idle entropy witness.
pub const CENSUS_GOLDEN_MASS_REJECT_ENTROPY: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_mass_reject_entropy",
    value: 0.1,
    evidence: "golden_mass_reject vector",
    derivation: None,
};

/// Golden mass-reject idle strength (MPa).
pub const CENSUS_GOLDEN_MASS_REJECT_STRENGTH_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_mass_reject_strength_mpa",
    value: 10.0,
    evidence: "golden_mass_reject vector",
    derivation: None,
};

/// Golden negative-dissipation entropy witness.
pub const CENSUS_GOLDEN_NEGATIVE_DISSIPATION_ENTROPY: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_negative_dissipation_entropy",
    value: 0.2,
    evidence: "golden_negative_dissipation vector",
    derivation: None,
};

/// Golden negative-dissipation strength (MPa).
pub const CENSUS_GOLDEN_NEGATIVE_DISSIPATION_STRENGTH_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_negative_dissipation_strength_mpa",
    value: 20.0,
    evidence: "golden_negative_dissipation vector",
    derivation: None,
};

/// Free-energy drop in CD-accept census (J).
pub const CENSUS_FREE_ENERGY_DROP_J: GroundedConst<f64> = GroundedConst {
    name: "gate_census_free_energy_drop_j",
    value: 100.0,
    evidence: "phase0b calibrated transition dissipation witness",
    derivation: None,
};

/// Mass violation delta above census density (kg/m³).
pub const CENSUS_MASS_VIOLATION_DELTA_KG_M3: GroundedConst<f64> = GroundedConst {
    name: "gate_census_mass_violation_delta_kg_m3",
    value: 200.0,
    evidence: "canonical_core_gate_outcome mass violation",
    derivation: None,
};

/// Open-system census electrical power input (W).
pub const CENSUS_OPEN_SYSTEM_POWER_W: GroundedConst<f64> = GroundedConst {
    name: "gate_census_open_system_power_w",
    value: 3.0,
    evidence: "canonical_core_gate_outcome honors power",
    derivation: None,
};

/// Reaction-extent regression decrement in census.
pub const CENSUS_REACTION_EXTENT_REGRESSION: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_regression",
    value: 0.1,
    evidence: "reaction extent irreversibility reject",
    derivation: None,
};

/// Small reaction-extent increment in material census.
pub const CENSUS_REACTION_EXTENT_INCREMENT: GroundedConst<f64> = GroundedConst {
    name: "gate_census_reaction_extent_increment",
    value: 0.05,
    evidence: "strength regression with extent advance",
    derivation: None,
};

/// One-hour timestep in mass-reject golden (s).
pub const CENSUS_DT_ONE_HOUR_S: GroundedConst<f64> = GroundedConst {
    name: "gate_census_dt_one_hour_s",
    value: 3600.0,
    evidence: "golden_mass_reject dt",
    derivation: None,
};

/// Twenty-eight-day cure age in HTTP manifest census (days).
pub const CENSUS_HTTP_AGE_DAYS: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_age_days",
    value: 28.0,
    evidence: "HTTP manifest admitting example age_days",
    derivation: None,
};

/// HTTP manifest primary constituent mass (kg).
pub const CENSUS_HTTP_CONSTITUENT_PRIMARY_KG: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_constituent_primary_kg",
    value: 400.0,
    evidence: "HTTP manifest admitting example",
    derivation: None,
};

/// HTTP manifest water mass (kg).
pub const CENSUS_HTTP_WATER_KG: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_water_kg",
    value: 200.0,
    evidence: "HTTP manifest admitting example",
    derivation: None,
};

/// HTTP manifest predicted strength (MPa).
pub const CENSUS_HTTP_PREDICTED_STRENGTH_MPA: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_predicted_strength_mpa",
    value: 25.0,
    evidence: "HTTP manifest admitting example",
    derivation: None,
};

/// HTTP manifest air-void fraction.
pub const CENSUS_HTTP_AIR_VOID_FRACTION: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_air_void_fraction",
    value: 0.02,
    evidence: "GateManifest default / prototype PhysicsConfig",
    derivation: None,
};

/// HTTP manifest admissibility relative margin.
pub const CENSUS_HTTP_ADMISSIBILITY_REL_MARGIN: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_admissibility_rel_margin",
    value: 0.15,
    evidence: "GateManifest default",
    derivation: None,
};

/// Twenty-degree Celsius bath for HTTP manifest proposals (°C).
pub const CENSUS_HTTP_TEMPERATURE_C: GroundedConst<f64> = GroundedConst {
    name: "gate_census_http_temperature_c",
    value: 20.0,
    evidence: "HTTP manifest admitting example temperature_c",
    derivation: None,
};

/// Golden identity free energy (J).
pub const CENSUS_GOLDEN_IDENTITY_FREE_ENERGY_J: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_identity_free_energy_j",
    value: -135_000.0,
    evidence: "golden_identity_admissible free_energy",
    derivation: None,
};

/// Golden negative-dissipation initial free energy (J).
pub const CENSUS_GOLDEN_NEGATIVE_DISSIPATION_FREE_ENERGY_J: GroundedConst<f64> = GroundedConst {
    name: "gate_census_golden_negative_dissipation_free_energy_j",
    value: -200_000.0,
    evidence: "golden_negative_dissipation initial ψ",
    derivation: None,
};

/// Golden negative-dissipation perturbed free energy (J).
pub const CENSUS_GOLDEN_NEGATIVE_DISSIPATION_FREE_ENERGY_SPIKE_J: GroundedConst<f64> =
    GroundedConst {
        name: "gate_census_golden_negative_dissipation_free_energy_spike_j",
        value: -10_000.0,
        evidence: "golden_negative_dissipation spike ψ",
        derivation: None,
    };

#[cfg(test)]
mod mix_density_closure_witness {
    use super::*;
    use crate::gate::transition_proposal::constants_registry_mix_calibrated::mix_calibrated_density_kg_m3;

    #[test]
    fn gate_census_mix_density_matches_linear_closure() {
        let rho = mix_calibrated_density_kg_m3(CENSUS_BINDER_LIQUID_RATIO.value);
        assert!(
            (rho - CENSUS_MIX_CALIBRATED_DENSITY_KG_M3.value).abs() < 1.0,
            "census density should match mix-calibrated closure at binder ratio"
        );
    }
}

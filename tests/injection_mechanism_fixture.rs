// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
// Arbitrary sentinel witness for manifold injection-mechanism tests only.
// Real cement closure (450/240) + byte-equiv parity live in umst-concrete-cartridge.

#![allow(dead_code)]

use umst_manifold::core::{MaterialTransitionParams, ReactionExtentKineticsSpec};
use umst_manifold::physics::solvers::ReactionExtentKinetics;

/// Non-physical sentinel — proves param injection path, not cement SSOT.
pub const FIXTURE_REACTION_ENTHALPY_J_PER_KG: f64 = 111.0;
/// Non-physical sentinel — proves param injection path, not cement SSOT.
pub const FIXTURE_DEFAULT_S_INTRINSIC_MPA: f64 = 222.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InjectionFixtureParams;

impl MaterialTransitionParams for InjectionFixtureParams {
    fn reaction_enthalpy_j_per_kg(&self) -> f64 {
        FIXTURE_REACTION_ENTHALPY_J_PER_KG
    }

    fn default_intrinsic_strength_mpa(&self) -> f64 {
        FIXTURE_DEFAULT_S_INTRINSIC_MPA
    }

    fn reaction_extent_kinetics_spec(&self) -> ReactionExtentKineticsSpec {
        injection_fixture_kinetics_spec()
    }
}

#[must_use]
pub const fn injection_fixture_kinetics_spec() -> ReactionExtentKineticsSpec {
    ReactionExtentKineticsSpec {
        arrhenius_prefactor_s: 1.0e-6,
        activation_energy_j_per_mol: 40_000.0,
        gas_constant_j_per_mol_k: 8.314_463,
        t_min_k: 250.0,
        t_boost_ref_k: 293.15,
        t_boost_per_k: 0.02,
        exothermic_k_per_alpha_rate: 5.0,
        stiffness_e_scale_pa: 30e9,
        stiffness_nu: 0.2,
    }
}

#[must_use]
pub fn injection_fixture_kinetics() -> ReactionExtentKinetics {
    ReactionExtentKinetics::from(injection_fixture_kinetics_spec())
}

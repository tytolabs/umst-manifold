// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cartridge-supplied transition closure parameters (W9 Tier 2c).
//!
//! **Categorical view:** domain cartridges are objects in a category of closure
//! witnesses; [`MaterialTransitionParams`] is the morphism family
//! `Witness → (ℝ⁺ × ℝ⁺ × ReactionExtentKineticsSpec)` supplying reaction enthalpy,
//! intrinsic strength cap, and kinetics. Kernel gates consume witnesses via explicit
//! injection — no ambient domain literals in `src/`.

/// Reaction-extent kinetics numbers — supplied by [`MaterialTransitionParams`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReactionExtentKineticsSpec {
    pub arrhenius_prefactor_s: f32,
    pub activation_energy_j_per_mol: f32,
    pub gas_constant_j_per_mol_k: f32,
    pub t_min_k: f32,
    pub t_boost_ref_k: f32,
    pub t_boost_per_k: f32,
    pub exothermic_k_per_alpha_rate: f32,
    pub stiffness_e_scale_pa: f32,
    pub stiffness_nu: f32,
}

impl ReactionExtentKineticsSpec {
    /// Unconfigured substrate placeholder (not domain-calibrated).
    #[must_use]
    pub const fn substrate_neutral() -> Self {
        Self {
            arrhenius_prefactor_s: 1.0,
            activation_energy_j_per_mol: 1.0,
            gas_constant_j_per_mol_k: 8.314_463,
            t_min_k: 1.0,
            t_boost_ref_k: 273.15,
            t_boost_per_k: 0.0,
            exothermic_k_per_alpha_rate: 0.0,
            stiffness_e_scale_pa: 1.0,
            stiffness_nu: 0.0,
        }
    }
}

/// Domain cartridges override; kernel gates use [`SubstrateMaterialParams`] when none is injected.
pub trait MaterialTransitionParams {
    /// Specific heat of reaction progress (J/kg).
    fn reaction_enthalpy_j_per_kg(&self) -> f64 {
        0.0
    }

    /// Intrinsic strength scale (MPa) for monotonicity checks.
    fn default_intrinsic_strength_mpa(&self) -> f64 {
        0.0
    }

    /// Arrhenius / exothermic kinetics for coupled THMC reaction-extent lanes.
    fn reaction_extent_kinetics_spec(&self) -> ReactionExtentKineticsSpec {
        ReactionExtentKineticsSpec::substrate_neutral()
    }
}

/// Zero-sized substrate witness when no domain cartridge is wired.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SubstrateMaterialParams;

impl MaterialTransitionParams for SubstrateMaterialParams {}

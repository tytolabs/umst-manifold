// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cartridge-supplied transition closure parameters (W9 Tier 2c).
//!
//! **Categorical view:** domain cartridges are objects in a category of closure
//! witnesses; [`MaterialTransitionParams`] is the morphism family
//! `Witness → (ℝ⁺ × ℝ⁺ × ReactionExtentKineticsSpec)` supplying reaction enthalpy,
//! intrinsic strength cap, and kinetics. Kernel gates consume witnesses via explicit
//! injection — no ambient domain literals in `src/`.
//!
//! **Honest status (W29-028):** injection trait + kinetics spec landed — not physics
//! GREEN, not `PRODUCTION_WIRED`, not `MASTER_RETICK`. Domain cartridges supply
//! calibrated witnesses; kernel gates fall back to [`SubstrateMaterialParams`].

/// W29 deepen cell id — honest closure witness slice only.
pub const MATERIAL_TRANSITION_CELL_ID: &str = "W29-028-MATERIAL_TRANSITION";

/// Honest posture tag — tests deepen only; no GREEN invent (`MASTER_RETICK=no`).
pub const MATERIAL_TRANSITION_POSTURE_TAG: &str = "honest-closure-witness-only";

/// Honest fence string for posture probes and gate receipts.
pub const HONEST_FENCE: &str =
    "closure_witness_landed=true production_wired=false physics_green=false";

/// Honest physics posture — closure numbers are injectable; continuum lift deferred.
pub const MATERIAL_TRANSITION_PHYSICS_GREEN: bool = false;

/// Production wiring at trait / cartridge seam — deferred beyond W29 slice.
pub const MATERIAL_TRANSITION_PRODUCTION_WIRED: bool = false;

/// Honest slice posture — witness trait landed, physics GREEN refused.
#[must_use]
pub const fn material_transition_posture_is_honest() -> bool {
    !MATERIAL_TRANSITION_PHYSICS_GREEN && !MATERIAL_TRANSITION_PRODUCTION_WIRED
}

/// W29 honest posture bundle — evaluators landed, physics GREEN refused.
#[must_use]
pub const fn material_transition_w29_honest_posture_bundle() -> bool {
    material_transition_posture_is_honest()
        && !MATERIAL_TRANSITION_PHYSICS_GREEN
        && !MATERIAL_TRANSITION_PRODUCTION_WIRED
}

/// Reaction-extent kinetics numbers — supplied by [`MaterialTransitionParams`].
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Cartridge-supplied Arrhenius / stiffness scalars; THMC runtime
/// projects via [`crate::physics::solvers::ReactionExtentKinetics`].
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

    /// Whether all kinetics scalars are finite (no NaN / inf witness).
    #[must_use]
    pub fn is_finite_witness(&self) -> bool {
        self.arrhenius_prefactor_s.is_finite()
            && self.activation_energy_j_per_mol.is_finite()
            && self.gas_constant_j_per_mol_k.is_finite()
            && self.t_min_k.is_finite()
            && self.t_boost_ref_k.is_finite()
            && self.t_boost_per_k.is_finite()
            && self.exothermic_k_per_alpha_rate.is_finite()
            && self.stiffness_e_scale_pa.is_finite()
            && self.stiffness_nu.is_finite()
    }

    /// Whether this spec matches the substrate-neutral placeholder (no cartridge override).
    #[must_use]
    pub const fn is_substrate_neutral(&self) -> bool {
        let n = Self::substrate_neutral();
        self.arrhenius_prefactor_s == n.arrhenius_prefactor_s
            && self.activation_energy_j_per_mol == n.activation_energy_j_per_mol
            && self.gas_constant_j_per_mol_k == n.gas_constant_j_per_mol_k
            && self.t_min_k == n.t_min_k
            && self.t_boost_ref_k == n.t_boost_ref_k
            && self.t_boost_per_k == n.t_boost_per_k
            && self.exothermic_k_per_alpha_rate == n.exothermic_k_per_alpha_rate
            && self.stiffness_e_scale_pa == n.stiffness_e_scale_pa
            && self.stiffness_nu == n.stiffness_nu
    }
}

/// Bundled closure witness projected from [`MaterialTransitionParams`].
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Product of enthalpy, strength cap, and kinetics spec.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransitionClosureBundle {
    pub reaction_enthalpy_j_per_kg: f64,
    pub default_intrinsic_strength_mpa: f64,
    pub kinetics: ReactionExtentKineticsSpec,
}

impl TransitionClosureBundle {
    /// Whether the bundle carries only substrate-neutral defaults (no cartridge override).
    #[must_use]
    pub fn is_substrate_neutral(&self) -> bool {
        self.reaction_enthalpy_j_per_kg == 0.0
            && self.default_intrinsic_strength_mpa == 0.0
            && self.kinetics.is_substrate_neutral()
    }
}

/// Domain cartridges override; kernel gates use [`SubstrateMaterialParams`] when none is injected.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Trait morphism `Witness → TransitionClosureBundle`; default impls are
/// substrate-neutral and must not embed domain literals.
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

    /// Project the full closure witness bundle for gate injection.
    #[must_use]
    fn closure_bundle(&self) -> TransitionClosureBundle {
        TransitionClosureBundle {
            reaction_enthalpy_j_per_kg: self.reaction_enthalpy_j_per_kg(),
            default_intrinsic_strength_mpa: self.default_intrinsic_strength_mpa(),
            kinetics: self.reaction_extent_kinetics_spec(),
        }
    }
}

/// Zero-sized substrate witness when no domain cartridge is wired.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Identity object for closure witness category; all accessors neutral.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SubstrateMaterialParams;

impl MaterialTransitionParams for SubstrateMaterialParams {}

impl SubstrateMaterialParams {
    /// Substrate witness always projects the neutral closure bundle.
    #[must_use]
    pub const fn closure_bundle_neutral() -> TransitionClosureBundle {
        TransitionClosureBundle {
            reaction_enthalpy_j_per_kg: 0.0,
            default_intrinsic_strength_mpa: 0.0,
            kinetics: ReactionExtentKineticsSpec::substrate_neutral(),
        }
    }
}

/// Whether the W29 closure witness morphism is pinned @ HEAD.
#[must_use]
pub fn material_transition_morphism_pinned() -> bool {
    MATERIAL_TRANSITION_CELL_ID == "W29-028-MATERIAL_TRANSITION"
        && MATERIAL_TRANSITION_POSTURE_TAG == "honest-closure-witness-only"
        && HONEST_FENCE.contains("closure_witness_landed=true")
        && material_transition_posture_is_honest()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::solvers::ReactionExtentKinetics;

    /// Inline test witness — proves injection path without cement SSOT.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct TestCartridgeParams;

    impl MaterialTransitionParams for TestCartridgeParams {
        fn reaction_enthalpy_j_per_kg(&self) -> f64 {
            450_000.0
        }

        fn default_intrinsic_strength_mpa(&self) -> f64 {
            42.0
        }

        fn reaction_extent_kinetics_spec(&self) -> ReactionExtentKineticsSpec {
            ReactionExtentKineticsSpec {
                arrhenius_prefactor_s: 1.0e-6,
                activation_energy_j_per_mol: 40_000.0,
                stiffness_e_scale_pa: 30e9,
                stiffness_nu: 0.2,
                ..ReactionExtentKineticsSpec::substrate_neutral()
            }
        }
    }

    #[test]
    fn material_transition_posture_is_honest_witness() {
        assert!(material_transition_posture_is_honest());
        assert!(material_transition_w29_honest_posture_bundle());
        assert!(!MATERIAL_TRANSITION_PHYSICS_GREEN);
        assert!(!MATERIAL_TRANSITION_PRODUCTION_WIRED);
        assert!(HONEST_FENCE.contains("production_wired=false"));
        assert!(HONEST_FENCE.contains("physics_green=false"));
    }

    #[test]
    fn material_transition_morphism_pinned_witness() {
        assert!(material_transition_morphism_pinned());
        assert_eq!(MATERIAL_TRANSITION_CELL_ID, "W29-028-MATERIAL_TRANSITION");
    }

    #[test]
    fn substrate_neutral_kinetics_is_finite() {
        let spec = ReactionExtentKineticsSpec::substrate_neutral();
        assert!(spec.is_finite_witness());
        assert!(spec.is_substrate_neutral());
    }

    #[test]
    fn substrate_params_closure_bundle_is_neutral() {
        let bundle = SubstrateMaterialParams.closure_bundle();
        assert!(bundle.is_substrate_neutral());
        assert_eq!(bundle, SubstrateMaterialParams::closure_bundle_neutral());
    }

    #[test]
    fn substrate_params_distinct_from_test_cartridge() {
        fn accept_substrate(_: SubstrateMaterialParams) {}
        fn accept_test(_: TestCartridgeParams) {}

        accept_substrate(SubstrateMaterialParams);
        accept_test(TestCartridgeParams);
    }

    #[test]
    fn test_cartridge_closure_bundle_overrides_defaults() {
        let bundle = TestCartridgeParams.closure_bundle();
        assert!(!bundle.is_substrate_neutral());
        assert_eq!(bundle.reaction_enthalpy_j_per_kg, 450_000.0);
        assert_eq!(bundle.default_intrinsic_strength_mpa, 42.0);
        assert!(!bundle.kinetics.is_substrate_neutral());
        assert!(bundle.kinetics.is_finite_witness());
    }

    #[test]
    fn kinetics_spec_roundtrips_through_thmc_runtime() {
        let spec = ReactionExtentKineticsSpec {
            arrhenius_prefactor_s: 2.5e-5,
            activation_energy_j_per_mol: 35_000.0,
            t_boost_per_k: 0.01,
            stiffness_e_scale_pa: 25e9,
            stiffness_nu: 0.18,
            ..ReactionExtentKineticsSpec::substrate_neutral()
        };
        let runtime = ReactionExtentKinetics::from(spec);
        let projected = runtime.as_kinetics_spec();
        assert_eq!(projected, spec);
    }

    #[test]
    fn closure_bundle_trait_default_matches_substrate() {
        let trait_bundle = SubstrateMaterialParams.closure_bundle();
        let const_bundle = SubstrateMaterialParams::closure_bundle_neutral();
        assert_eq!(trait_bundle, const_bundle);
    }

    #[test]
    fn material_transition_params_injection_changes_enthalpy_only() {
        #[derive(Clone, Copy, Default)]
        struct EnthalpyOnly;

        impl MaterialTransitionParams for EnthalpyOnly {
            fn reaction_enthalpy_j_per_kg(&self) -> f64 {
                99.0
            }
        }

        let bundle = EnthalpyOnly.closure_bundle();
        assert_eq!(bundle.reaction_enthalpy_j_per_kg, 99.0);
        assert_eq!(bundle.default_intrinsic_strength_mpa, 0.0);
        assert!(bundle.kinetics.is_substrate_neutral());
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! W9 Phase A deprecated aliases — excluded from agnostic-on-fork / tier-1 lexicon scans.
//!
//! [`MixTensor`] and [`StatePoint`] are type aliases for [`MaterialCompositionTensor`];
//! prefer [`MaterialCompositionTensor`] in new code.
//!
//! ## Honest fences (W29-132)
//!
//! Phase A surfaces are **alias / stub only**. Cartridge injection for `iE_*` closures
//! remains open. This module does **not** attest physics GREEN, PRODUCTION_WIRED,
//! MASTER, or OP-5.

#![allow(non_snake_case)]

use burn::tensor::{backend::Backend, Tensor};

use crate::core::tensors::MaterialCompositionTensor;
use crate::gate::thermo_transition::{ThermodynamicGate, ThermodynamicState};
use crate::physics::solvers::thmc::ReactionExtentKinetics;

/// W29 deepen cell — W9 Phase A migration honesty (no invent GREEN).
pub const W29_W9_MIGRATION_DEEPEN_CELL: &str = "W29-132-W9_MIGRATION";

/// Honest posture tag — Phase A aliases/stubs landed; cartridge inject open.
pub const W9_MIGRATION_POSTURE_TAG: &str = "W9_PHASE_A_ALIAS_STUB";

/// Honest deepen fence for meta / fleet probes.
pub const W9_MIGRATION_HONEST_FENCE: &str = "phase_a_aliases_landed=true|cartridge_inject=false|production_wired=false|physics_green=false|master=false|op5=false";

/// Phase A type aliases + deprecated stub wrappers are present in this module.
pub const W9_PHASE_A_ALIASES_LANDED: bool = true;

/// Domain cartridge injection for `iE_*` / strength closures — **open** (stubs only).
pub const W9_CARTRIDGE_INJECT_WIRED: bool = false;

/// Honest physics posture — migration aliases ≠ physics GREEN.
pub const W9_MIGRATION_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not claimed by Phase A alias slice.
pub const W9_MIGRATION_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by Phase A alias slice.
pub const W9_MIGRATION_MASTER: bool = false;

/// OP-5 retick eligibility — honest false at Phase A deepen.
pub const W9_MIGRATION_OP5: bool = false;

/// Inventory size of Phase A deprecated surfaces in this module.
pub const W9_PHASE_A_SURFACE_COUNT: usize = 7;

/// How many surfaces are pure type aliases (rename only).
pub const W9_PHASE_A_TYPE_ALIAS_COUNT: usize = 3;

/// How many surfaces are cartridge-supplied stubs (not domain-wired).
pub const W9_PHASE_A_CARTRIDGE_STUB_COUNT: usize = 4;

const _: () = assert!(W9_PHASE_A_ALIASES_LANDED);
const _: () = assert!(!W9_CARTRIDGE_INJECT_WIRED);
const _: () = assert!(!W9_MIGRATION_PHYSICS_GREEN);
const _: () = assert!(!W9_MIGRATION_PRODUCTION_WIRED);
const _: () = assert!(!W9_MIGRATION_MASTER);
const _: () = assert!(!W9_MIGRATION_OP5);
const _: () = assert!(
    W9_PHASE_A_SURFACE_COUNT
        == W9_PHASE_A_TYPE_ALIAS_COUNT + W9_PHASE_A_CARTRIDGE_STUB_COUNT
);

/// Kind of a Phase A deprecated surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum W9PhaseASurfaceKind {
    /// Pure rename alias onto an agnostic kernel type.
    TypeAlias,
    /// Cartridge-owned closure / wiring — stub or default only in-kernel.
    CartridgeStub,
}

/// One inventory row for a Phase A deprecated surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W9PhaseASurfaceRow {
    pub legacy_name: &'static str,
    pub target_or_note: &'static str,
    pub kind: W9PhaseASurfaceKind,
    /// True only for type aliases that resolve to a landed agnostic type.
    pub alias_landed: bool,
    /// True iff a domain cartridge owns the live implementation (honest false today).
    pub cartridge_injected: bool,
}

/// Measured Phase A surface inventory (aliases + stubs).
pub const W9_PHASE_A_SURFACES: [W9PhaseASurfaceRow; W9_PHASE_A_SURFACE_COUNT] = [
    W9PhaseASurfaceRow {
        legacy_name: "MixTensor",
        target_or_note: "MaterialCompositionTensor",
        kind: W9PhaseASurfaceKind::TypeAlias,
        alias_landed: true,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "StatePoint",
        target_or_note: "MaterialCompositionTensor",
        kind: W9PhaseASurfaceKind::TypeAlias,
        alias_landed: true,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "ThmcHydrationKinetics",
        target_or_note: "ReactionExtentKinetics",
        kind: W9PhaseASurfaceKind::TypeAlias,
        alias_landed: true,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "iE_degree",
        target_or_note: "cartridge strength closure (stub returns 0.0)",
        kind: W9PhaseASurfaceKind::CartridgeStub,
        alias_landed: false,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "full_iE_alpha_rate_tensor",
        target_or_note: "R-api-w9-cartridge-inject (unimplemented)",
        kind: W9PhaseASurfaceKind::CartridgeStub,
        alias_landed: false,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "thermodynamic_gate_from_iE_defaults",
        target_or_note: "ThermodynamicGate::default (not cartridge)",
        kind: W9PhaseASurfaceKind::CartridgeStub,
        alias_landed: false,
        cartridge_injected: false,
    },
    W9PhaseASurfaceRow {
        legacy_name: "thermodynamic_state_from_iE",
        target_or_note: "ThermodynamicState::from_mix (substrate-neutral)",
        kind: W9PhaseASurfaceKind::CartridgeStub,
        alias_landed: false,
        cartridge_injected: false,
    },
];

/// Explicit non-claim: Phase A migration aliases are not a production wire attestation.
#[must_use]
pub const fn w9_migration_production_wired() -> bool {
    false
}

/// Explicit non-claim: alias/stub presence ≠ physics GREEN / MASTER / OP-5.
#[must_use]
pub const fn w9_migration_physics_green() -> bool {
    false
}

/// Explicit non-claim: MASTER retick not claimed by Phase A deepen.
#[must_use]
pub const fn w9_migration_master() -> bool {
    false
}

/// Explicit non-claim: OP-5 not claimed by Phase A deepen.
#[must_use]
pub const fn w9_migration_op5() -> bool {
    false
}

/// Explicit non-claim: domain cartridge inject remains open.
#[must_use]
pub const fn w9_cartridge_inject_wired() -> bool {
    false
}

const _: () = assert!(!w9_migration_production_wired());
const _: () = assert!(!w9_migration_physics_green());
const _: () = assert!(!w9_migration_master());
const _: () = assert!(!w9_migration_op5());
const _: () = assert!(!w9_cartridge_inject_wired());

/// Typed probe for W9 Phase A migration posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct W9MigrationPostureProbe {
    pub deepen_cell: &'static str,
    pub posture_tag: &'static str,
    pub phase_a_aliases_landed: bool,
    pub cartridge_inject_wired: bool,
    pub surface_count: usize,
    pub type_alias_count: usize,
    pub cartridge_stub_count: usize,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub op5: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for W9 migration done-when checks.
#[must_use]
pub const fn w9_migration_posture_probe() -> W9MigrationPostureProbe {
    W9MigrationPostureProbe {
        deepen_cell: W29_W9_MIGRATION_DEEPEN_CELL,
        posture_tag: W9_MIGRATION_POSTURE_TAG,
        phase_a_aliases_landed: W9_PHASE_A_ALIASES_LANDED,
        cartridge_inject_wired: W9_CARTRIDGE_INJECT_WIRED,
        surface_count: W9_PHASE_A_SURFACE_COUNT,
        type_alias_count: W9_PHASE_A_TYPE_ALIAS_COUNT,
        cartridge_stub_count: W9_PHASE_A_CARTRIDGE_STUB_COUNT,
        production_wired: W9_MIGRATION_PRODUCTION_WIRED,
        master: W9_MIGRATION_MASTER,
        physics_green: W9_MIGRATION_PHYSICS_GREEN,
        op5: W9_MIGRATION_OP5,
        honest_fence: W9_MIGRATION_HONEST_FENCE,
    }
}

/// Whether Phase A surface inventory matches pinned counts and refuses fake inject.
#[must_use]
pub fn w9_phase_a_surface_inventory_honest() -> bool {
    W9_PHASE_A_SURFACES.len() == W9_PHASE_A_SURFACE_COUNT
        && W9_PHASE_A_SURFACES
            .iter()
            .filter(|r| r.kind == W9PhaseASurfaceKind::TypeAlias)
            .count()
            == W9_PHASE_A_TYPE_ALIAS_COUNT
        && W9_PHASE_A_SURFACES
            .iter()
            .filter(|r| r.kind == W9PhaseASurfaceKind::CartridgeStub)
            .count()
            == W9_PHASE_A_CARTRIDGE_STUB_COUNT
        && W9_PHASE_A_SURFACES
            .iter()
            .all(|r| !r.cartridge_injected)
        && W9_PHASE_A_SURFACES
            .iter()
            .filter(|r| r.kind == W9PhaseASurfaceKind::TypeAlias)
            .all(|r| r.alias_landed)
        && W9_PHASE_A_SURFACES
            .iter()
            .filter(|r| r.kind == W9PhaseASurfaceKind::CartridgeStub)
            .all(|r| !r.alias_landed)
}

/// Phase A aliases landed with production / master / GREEN / OP-5 / inject honestly open.
#[must_use]
pub fn w9_migration_posture_honest(probe: &W9MigrationPostureProbe) -> bool {
    probe.deepen_cell == W29_W9_MIGRATION_DEEPEN_CELL
        && probe.posture_tag == W9_MIGRATION_POSTURE_TAG
        && probe.phase_a_aliases_landed
        && !probe.cartridge_inject_wired
        && probe.surface_count == W9_PHASE_A_SURFACE_COUNT
        && probe.type_alias_count == W9_PHASE_A_TYPE_ALIAS_COUNT
        && probe.cartridge_stub_count == W9_PHASE_A_CARTRIDGE_STUB_COUNT
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && !probe.op5
        && probe.honest_fence.contains("phase_a_aliases_landed=true")
        && probe.honest_fence.contains("cartridge_inject=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
        && w9_phase_a_surface_inventory_honest()
}

/// Validate W9 migration posture honesty — fail closed on fake GREEN / production claims.
pub fn validate_w9_migration_posture_honesty() -> Result<(), &'static str> {
    let probe = w9_migration_posture_probe();
    if probe.production_wired || w9_migration_production_wired() {
        return Err("w9_migration_production_wired must stay false until cartridge wire measured");
    }
    if probe.master || w9_migration_master() {
        return Err("w9_migration_master must stay false until fleet sign-off");
    }
    if probe.physics_green || w9_migration_physics_green() {
        return Err("W9_MIGRATION_PHYSICS_GREEN must stay false at Phase A alias slice");
    }
    if probe.op5 || w9_migration_op5() {
        return Err("W9_MIGRATION_OP5 must stay false at Phase A alias slice");
    }
    if probe.cartridge_inject_wired || w9_cartridge_inject_wired() {
        return Err("W9_CARTRIDGE_INJECT_WIRED must stay false until R-api-w9-cartridge-inject");
    }
    if !probe.phase_a_aliases_landed {
        return Err("W9_PHASE_A_ALIASES_LANDED must stay true at W29-132");
    }
    if !w9_phase_a_surface_inventory_honest() {
        return Err("w9_phase_a_surface_inventory_honest failed");
    }
    if !w9_migration_posture_honest(&probe) {
        return Err("w9_migration_posture_honest failed");
    }
    Ok(())
}

/// Renamed to [`MaterialCompositionTensor`] in v2.0.0-rc1 (W9 agnostic-on-fork).
#[deprecated(note = "renamed to MaterialCompositionTensor")]
pub type MixTensor<B> = MaterialCompositionTensor<B>;

/// Renamed to [`MaterialCompositionTensor`] in v2.0.0-rc1 (W9 agnostic-on-fork).
#[deprecated(note = "renamed to MaterialCompositionTensor")]
pub type StatePoint<B> = MaterialCompositionTensor<B>;

#[deprecated(note = "renamed to ReactionExtentKinetics")]
pub type ThmcHydrationKinetics = ReactionExtentKinetics;

#[deprecated(note = "cartridge-supplied strength closure")]
pub fn iE_degree(age_days: f64, temp_c: f64, supplementary_ratio: f64) -> f64 {
    let _ = (age_days, temp_c, supplementary_ratio);
    0.0
}

#[deprecated(note = "cartridge-supplied tensor closure")]
pub fn full_iE_alpha_rate_tensor<B: Backend<FloatElem = f32>>(
    _age_days: Tensor<B, 1>,
    _temp_c: Tensor<B, 1>,
    _supplementary_ratio: Tensor<B, 1>,
) -> Tensor<B, 1> {
    unimplemented!("R-api-w9-cartridge-inject: W9 tier-2c domain cartridge injection")
}

#[deprecated(note = "cartridge-supplied gate wiring")]
pub fn thermodynamic_gate_from_iE_defaults() -> ThermodynamicGate {
    ThermodynamicGate::default()
}

#[deprecated(note = "cartridge-supplied snapshot")]
pub fn thermodynamic_state_from_iE(w_c: f64, alpha: f64, temp_k: f64) -> ThermodynamicState {
    ThermodynamicState::from_mix(w_c, alpha, temp_k)
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use burn_ndarray::NdArray;

    #[test]
    fn w9_migration_honesty_fence_refuses_green_production_master_op5() {
        assert!(!w9_migration_production_wired());
        assert!(!w9_migration_physics_green());
        assert!(!w9_migration_master());
        assert!(!w9_migration_op5());
        assert!(!w9_cartridge_inject_wired());
        validate_w9_migration_posture_honesty().expect("posture honest");
        let probe = w9_migration_posture_probe();
        assert!(w9_migration_posture_honest(&probe));
        assert_eq!(probe.deepen_cell, "W29-132-W9_MIGRATION");
        assert!(!probe.honest_fence.contains("production_wired=true"));
        assert!(!probe.honest_fence.contains("physics_green=true"));
        assert!(!probe.honest_fence.contains("master=true"));
        assert!(!probe.honest_fence.contains("op5=true"));
    }

    #[test]
    fn w9_phase_a_surface_inventory_counts_match() {
        assert!(w9_phase_a_surface_inventory_honest());
        assert_eq!(W9_PHASE_A_SURFACES.len(), 7);
        assert_eq!(
            W9_PHASE_A_SURFACES
                .iter()
                .filter(|r| r.kind == W9PhaseASurfaceKind::TypeAlias)
                .count(),
            3
        );
        assert_eq!(
            W9_PHASE_A_SURFACES
                .iter()
                .filter(|r| r.kind == W9PhaseASurfaceKind::CartridgeStub)
                .count(),
            4
        );
        assert!(W9_PHASE_A_SURFACES.iter().all(|r| !r.cartridge_injected));
    }

    #[test]
    fn ie_degree_stub_returns_zero_not_domain_physics() {
        assert_eq!(iE_degree(28.0, 20.0, 0.3), 0.0);
        assert_eq!(iE_degree(0.0, -40.0, 1.0), 0.0);
    }

    #[test]
    fn thermodynamic_state_from_ie_matches_from_mix() {
        let via_alias = thermodynamic_state_from_iE(0.45, 0.4, 293.15);
        let via_agnostic = ThermodynamicState::from_mix(0.45, 0.4, 293.15);
        assert_eq!(via_alias.density, via_agnostic.density);
        assert_eq!(via_alias.temperature, via_agnostic.temperature);
        assert_eq!(via_alias.free_energy, via_agnostic.free_energy);
        assert_eq!(via_alias.entropy, via_agnostic.entropy);
        assert_eq!(via_alias.reaction_extent, via_agnostic.reaction_extent);
        assert_eq!(via_alias.strength, via_agnostic.strength);
    }

    #[test]
    fn thermodynamic_gate_from_ie_defaults_is_default_gate() {
        let via_alias = thermodynamic_gate_from_iE_defaults();
        let via_default = ThermodynamicGate::default();
        // Default gate starts with zero counters — measured stub, not cartridge inject.
        assert_eq!(via_alias.stats_summary(), via_default.stats_summary());
        assert_eq!(via_alias.stats_summary(), "No transitions checked");
    }

    #[test]
    fn thmc_hydration_kinetics_alias_defaults_match_reaction_extent() {
        let via_alias: ThmcHydrationKinetics = ThmcHydrationKinetics::default();
        let via_agnostic = ReactionExtentKinetics::default();
        assert_eq!(
            via_alias.arrhenius_prefactor_s,
            via_agnostic.arrhenius_prefactor_s
        );
        assert_eq!(
            via_alias.activation_energy_j_per_mol,
            via_agnostic.activation_energy_j_per_mol
        );
        assert_eq!(via_alias.stiffness_nu, via_agnostic.stiffness_nu);
    }

    #[test]
    fn full_ie_alpha_rate_tensor_panics_until_cartridge_inject() {
        type B = NdArray;
        let age = Tensor::<B, 1>::from_floats([1.0_f32], &Default::default());
        let temp = Tensor::<B, 1>::from_floats([20.0_f32], &Default::default());
        let supp = Tensor::<B, 1>::from_floats([0.2_f32], &Default::default());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = full_iE_alpha_rate_tensor::<B>(age, temp, supp);
        }));
        assert!(result.is_err(), "cartridge inject must remain unimplemented");
    }
}

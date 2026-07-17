// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §3 — macroscopic constitutive phase ADT (MP1 stubs).
//!
//! Staging vocabulary for cast lifecycle migration: invalid cross-phase ops are rejected
//! by `match` on [`MaterialPhase`], not by `is_fresh` / `is_hardened` boolean flags.
//!
//! **MP1 scope:** type introduction only — [`crate::physics::solvers::thmc::ThmcState`] remains
//! the flat product carrier until MP2; no gate or solver wiring in this slice.
//!
//! Schedule: `outputs/.tmp/fp_material_phase_adt_plan.md`.

use burn::tensor::{backend::Backend, Tensor};

use super::field::{
    DamageField, Field, HumidityField, TemperatureField,
};

/// Discrete macroscopic phase tag for `match`-dispatched routing (no bool soup).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized phase witness; mirrors `ClinkerPhase` at cast scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialPhaseKind {
    /// Printable / pumpable bulk: rheology + transport; no equilibrium solid solve.
    Fluid,
    /// Setting gel: coupled chemo-thermal + capillary transport; quasi-static mechanics optional.
    Setting,
    /// Hardened solid: mechanics + fracture + shrinkage/creep; no Bingham step.
    Solid,
}

/// Fresh-state rheology carrier — Bingham / transport lane (DEC 1-skeleton velocity).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 tensor bundle stub; P3 may wrap fields in phantom-typed [`super::Field`].
#[derive(Clone, Debug)]
pub struct RheologyState<B: Backend> {
    /// Yield stress τ₀ — shape `[B, N, F_τ]`.
    pub yield_stress: Tensor<B, 3>,
    /// Plastic viscosity η_p — shape `[B, N, F_η]`.
    pub plastic_viscosity: Tensor<B, 3>,
    /// Velocity field **u** on the DEC 1-skeleton — shape `[B, N, 3]`.
    pub velocity: Tensor<B, 3>,
}

/// Setting-gel carrier — coupled chemo-thermal + capillary transport.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 tensor bundle stub; α thresholds live in cartridge `Profile` (MP3).
#[derive(Clone, Debug)]
pub struct SettingState<B: Backend> {
    /// Chemical reaction extent α — shape `[B, N, F_α]`.
    pub reaction_extent: Tensor<B, 3>,
    /// Pore-fluid / humidity proxy — shape `[B, N, F_h]`.
    pub humidity: Tensor<B, 3>,
    /// Nodal temperature — shape `[B, N, F_T]`.
    pub temperature: Tensor<B, 3>,
}

/// Hardened-solid mechanics carrier — equilibrium solve + fracture coupling.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 tensor bundle stub; displacement channel SSOT is vector_features.
#[derive(Clone, Debug)]
pub struct MechanicsState<B: Backend> {
    /// Mechanical displacement **u** — shape `[B, N, 3]`.
    pub displacement: Tensor<B, 3>,
    /// Continuum / phase-field damage — shape `[B, N, F_d]`.
    pub damage: Tensor<B, 3>,
    /// Stiffness / modulus field — shape `[B, N, F_E]`.
    pub stiffness: Tensor<B, 3>,
}

/// Macroscopic constitutive phase — invalid cross-phase ops rejected by `match`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Algebraic sum type over rheology / setting / mechanics carriers (FP §3).
#[derive(Clone, Debug)]
pub enum MaterialPhase<B: Backend> {
    /// Printable / pumpable bulk: rheology + transport; no equilibrium solid solve.
    Fluid(RheologyState<B>),
    /// Setting gel: coupled chemo-thermal + capillary transport; quasi-static mechanics optional.
    Setting(SettingState<B>),
    /// Hardened solid: mechanics + fracture + shrinkage/creep; no Bingham step.
    Solid(MechanicsState<B>),
}

impl<B: Backend> MaterialPhase<B> {
    /// Return the discrete phase tag for routing without inspecting tensor payloads.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Exhaustive tag projection; callers should `match` on [`MaterialPhaseKind`].
    #[inline]
    #[must_use]
    pub fn kind(&self) -> MaterialPhaseKind {
        match self {
            Self::Fluid(_) => MaterialPhaseKind::Fluid,
            Self::Setting(_) => MaterialPhaseKind::Setting,
            Self::Solid(_) => MaterialPhaseKind::Solid,
        }
    }

    /// Borrow the fluid rheology payload when this phase is [`MaterialPhaseKind::Fluid`].
    #[inline]
    #[must_use]
    pub fn as_fluid(&self) -> Option<&RheologyState<B>> {
        match self {
            Self::Fluid(r) => Some(r),
            _ => None,
        }
    }

    /// Borrow the setting-gel payload when this phase is [`MaterialPhaseKind::Setting`].
    #[inline]
    #[must_use]
    pub fn as_setting(&self) -> Option<&SettingState<B>> {
        match self {
            Self::Setting(s) => Some(s),
            _ => None,
        }
    }

    /// Borrow the hardened-solid payload when this phase is [`MaterialPhaseKind::Solid`].
    #[inline]
    #[must_use]
    pub fn as_solid(&self) -> Option<&MechanicsState<B>> {
        match self {
            Self::Solid(m) => Some(m),
            _ => None,
        }
    }

    /// Mutably borrow the fluid rheology payload when this phase is [`MaterialPhaseKind::Fluid`].
    #[inline]
    #[must_use]
    pub fn as_fluid_mut(&mut self) -> Option<&mut RheologyState<B>> {
        match self {
            Self::Fluid(r) => Some(r),
            _ => None,
        }
    }

    /// Mutably borrow the setting-gel payload when this phase is [`MaterialPhaseKind::Setting`].
    #[inline]
    #[must_use]
    pub fn as_setting_mut(&mut self) -> Option<&mut SettingState<B>> {
        match self {
            Self::Setting(s) => Some(s),
            _ => None,
        }
    }

    /// Mutably borrow the hardened-solid payload when this phase is [`MaterialPhaseKind::Solid`].
    #[inline]
    #[must_use]
    pub fn as_solid_mut(&mut self) -> Option<&mut MechanicsState<B>> {
        match self {
            Self::Solid(m) => Some(m),
            _ => None,
        }
    }
}

/// Shared transport channels present in Fluid and Setting arms (MP2).
#[derive(Clone, Debug)]
pub struct TransportState<B: Backend> {
    pub humidity: HumidityField<B>,
    pub temperature: TemperatureField<B>,
}

/// THMC envelope: macroscopic phase + fracture damage + simulation clock.
///
/// Target replacement for flat [`crate::physics::solvers::thmc::ThmcState`] in MP2.
/// Bijection helpers live in [`crate::physics::solvers::thmc_envelope`].
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Product of [`MaterialPhase`], envelope-level damage, and clock.
#[derive(Clone, Debug)]
pub struct ThmcEnvelope<B: Backend> {
    pub phase: MaterialPhase<B>,
    /// Fracture coupling — lives outside variant (frozen at step entry, P3.2).
    pub damage: DamageField<B>,
    pub time: f32,
}

impl<B: Backend> ThmcEnvelope<B> {
    /// Construct an envelope from a phase variant, damage field, and clock.
    #[inline]
    #[must_use]
    pub fn new(phase: MaterialPhase<B>, damage: DamageField<B>, time: f32) -> Self {
        Self { phase, damage, time }
    }

    /// Construct an envelope from a phase variant, zero damage, and clock (test / scaffold helper).
    #[inline]
    #[must_use]
    pub fn with_zero_damage(phase: MaterialPhase<B>, time: f32, device: &B::Device) -> Self {
        let n = match &phase {
            MaterialPhase::Fluid(r) => r.velocity.dims()[1],
            MaterialPhase::Setting(s) => s.reaction_extent.dims()[1],
            MaterialPhase::Solid(m) => m.displacement.dims()[1],
        };
        let damage = Field::new(Tensor::<B, 3>::zeros([1, n, 1], device));
        Self::new(phase, damage, time)
    }

    /// Project the discrete phase tag.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> MaterialPhaseKind {
        self.phase.kind()
    }
}

#[cfg(test)]
mod tests {
    use burn::tensor::Tensor;
    use burn_ndarray::NdArray;

    use super::*;

    type B = NdArray;

    fn zeros_3() -> Tensor<B, 3> {
        Tensor::<B, 3>::zeros([1, 2, 1], &Default::default())
    }

    fn fluid_phase() -> MaterialPhase<B> {
        MaterialPhase::Fluid(RheologyState {
            yield_stress: zeros_3(),
            plastic_viscosity: zeros_3(),
            velocity: Tensor::<B, 3>::zeros([1, 2, 3], &Default::default()),
        })
    }

    fn setting_phase() -> MaterialPhase<B> {
        MaterialPhase::Setting(SettingState {
            reaction_extent: zeros_3(),
            humidity: zeros_3(),
            temperature: zeros_3(),
        })
    }

    fn solid_phase() -> MaterialPhase<B> {
        MaterialPhase::Solid(MechanicsState {
            displacement: Tensor::<B, 3>::zeros([1, 2, 3], &Default::default()),
            damage: zeros_3(),
            stiffness: zeros_3(),
        })
    }

    #[test]
    fn material_phase_kind_tags_each_variant() {
        assert_eq!(fluid_phase().kind(), MaterialPhaseKind::Fluid);
        assert_eq!(setting_phase().kind(), MaterialPhaseKind::Setting);
        assert_eq!(solid_phase().kind(), MaterialPhaseKind::Solid);
    }

    #[test]
    fn thmc_envelope_carries_phase_damage_and_time() {
        let device = Default::default();
        let env = ThmcEnvelope::with_zero_damage(setting_phase(), 42.0, &device);
        assert_eq!(env.kind(), MaterialPhaseKind::Setting);
        assert!((env.time - 42.0).abs() < f32::EPSILON);
        assert_eq!(env.damage.as_tensor().dims(), [1, 2, 1]);
    }

    #[test]
    fn match_on_material_phase_is_exhaustive() {
        fn route(phase: MaterialPhase<B>) -> &'static str {
            match phase {
                MaterialPhase::Fluid(_) => "rheology",
                MaterialPhase::Setting(_) => "setting",
                MaterialPhase::Solid(_) => "mechanics",
            }
        }

        assert_eq!(route(fluid_phase()), "rheology");
        assert_eq!(route(setting_phase()), "setting");
        assert_eq!(route(solid_phase()), "mechanics");
    }

    #[test]
    fn phase_variants_preserve_tensor_shapes() {
        match fluid_phase() {
            MaterialPhase::Fluid(r) => {
                assert_eq!(r.yield_stress.dims(), [1, 2, 1]);
                assert_eq!(r.velocity.dims(), [1, 2, 3]);
            }
            _ => panic!("expected Fluid"),
        }

        match setting_phase() {
            MaterialPhase::Setting(s) => {
                assert_eq!(s.reaction_extent.dims(), [1, 2, 1]);
                assert_eq!(s.humidity.dims(), [1, 2, 1]);
                assert_eq!(s.temperature.dims(), [1, 2, 1]);
            }
            _ => panic!("expected Setting"),
        }

        match solid_phase() {
            MaterialPhase::Solid(m) => {
                assert_eq!(m.displacement.dims(), [1, 2, 3]);
                assert_eq!(m.damage.dims(), [1, 2, 1]);
            }
            _ => panic!("expected Solid"),
        }
    }

    #[test]
    fn material_phase_variant_accessors_match_kind() {
        let fluid = fluid_phase();
        assert_eq!(fluid.kind(), MaterialPhaseKind::Fluid);
        assert!(fluid.as_fluid().is_some());
        assert!(fluid.as_setting().is_none());
        assert!(fluid.as_solid().is_none());

        let setting = setting_phase();
        assert_eq!(setting.kind(), MaterialPhaseKind::Setting);
        assert!(setting.as_setting().is_some());
        assert!(setting.as_fluid().is_none());
        assert!(setting.as_solid().is_none());

        let solid = solid_phase();
        assert_eq!(solid.kind(), MaterialPhaseKind::Solid);
        assert!(solid.as_solid().is_some());
        assert!(solid.as_fluid().is_none());
        assert!(solid.as_setting().is_none());
    }

    #[test]
    fn material_phase_mut_accessors_preserve_kind() {
        let mut fluid = fluid_phase();
        assert!(fluid.as_fluid_mut().is_some());
        assert!(fluid.as_setting_mut().is_none());
        assert_eq!(fluid.kind(), MaterialPhaseKind::Fluid);

        let mut setting = setting_phase();
        assert!(setting.as_setting_mut().is_some());
        assert!(setting.as_solid_mut().is_none());
        assert_eq!(setting.kind(), MaterialPhaseKind::Setting);

        let mut solid = solid_phase();
        assert!(solid.as_solid_mut().is_some());
        assert!(solid.as_fluid_mut().is_none());
        assert_eq!(solid.kind(), MaterialPhaseKind::Solid);
    }

    #[test]
    fn material_phase_kind_is_copy_hash_eq() {
        use std::collections::HashSet;

        let kinds = [
            MaterialPhaseKind::Fluid,
            MaterialPhaseKind::Setting,
            MaterialPhaseKind::Solid,
        ];
        let mut set = HashSet::new();
        for k in kinds {
            let copied = k;
            assert_eq!(k, copied);
            set.insert(k);
        }
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn thmc_envelope_kind_for_all_variants() {
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            let device = Default::default();
            let env = ThmcEnvelope::with_zero_damage(phase, 0.0, &device);
            assert_eq!(env.kind(), expected);
        }
    }

    #[test]
    fn thmc_envelope_with_zero_damage_uses_node_count() {
        let device = Default::default();
        let env = ThmcEnvelope::with_zero_damage(fluid_phase(), 1.0, &device);
        assert_eq!(env.damage.as_tensor().dims(), [1, 2, 1]);
    }

    #[test]
    fn clone_preserves_material_phase_kind() {
        let cloned = fluid_phase();
        assert_eq!(cloned.kind(), MaterialPhaseKind::Fluid);
        assert!(cloned.as_fluid().is_some());
    }
}

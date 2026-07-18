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

impl<B: Backend> TransportState<B> {
    /// Bundle humidity and temperature fields for transport-only routing arms.
    #[inline]
    #[must_use]
    pub fn new(humidity: HumidityField<B>, temperature: TemperatureField<B>) -> Self {
        Self { humidity, temperature }
    }
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

    /// Borrow envelope-level fracture damage (frozen at step entry, P3.2).
    #[inline]
    #[must_use]
    pub fn damage_ref(&self) -> &DamageField<B> {
        &self.damage
    }

    /// Simulation clock carried alongside the phase variant.
    #[inline]
    #[must_use]
    pub fn time(&self) -> f32 {
        self.time
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
        let fluid = fluid_phase();
        let r = fluid.as_fluid().expect(
            "fluid_phase fixture must yield MaterialPhaseKind::Fluid variant (MP1 phase ADT witness) \
             (FP §6 hydration/aging harness)",
        );
        assert_eq!(r.yield_stress.dims(), [1, 2, 1]);
        assert_eq!(r.velocity.dims(), [1, 2, 3]);

        let setting = setting_phase();
        let s = setting.as_setting().expect(
            "setting_phase fixture must yield MaterialPhaseKind::Setting variant \
             (MP1 phase ADT witness) (FP §6 hydration/aging harness)",
        );
        assert_eq!(s.reaction_extent.dims(), [1, 2, 1]);
        assert_eq!(s.humidity.dims(), [1, 2, 1]);
        assert_eq!(s.temperature.dims(), [1, 2, 1]);

        let solid = solid_phase();
        let m = solid.as_solid().expect(
            "solid_phase fixture must yield MaterialPhaseKind::Solid variant (MP1 phase ADT witness) \
             (FP §6 hydration/aging harness)",
        );
        assert_eq!(m.displacement.dims(), [1, 2, 3]);
        assert_eq!(m.damage.dims(), [1, 2, 1]);
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

    #[test]
    fn clone_preserves_setting_and_solid_kinds() {
        let setting = setting_phase();
        let setting_clone = setting.clone();
        assert_eq!(setting_clone.kind(), MaterialPhaseKind::Setting);
        assert!(setting_clone.as_setting().is_some());
        assert!(setting_clone.as_fluid().is_none());

        let solid = solid_phase();
        let solid_clone = solid.clone();
        assert_eq!(solid_clone.kind(), MaterialPhaseKind::Solid);
        assert!(solid_clone.as_solid().is_some());
        assert!(solid_clone.as_setting().is_none());
    }

    #[test]
    fn transport_state_new_wraps_field_channels() {
        let humidity = Field::new(zeros_3());
        let temperature = Field::new(zeros_3());
        let transport = TransportState::new(humidity, temperature);
        assert_eq!(transport.humidity.as_tensor().dims(), [1, 2, 1]);
        assert_eq!(transport.temperature.as_tensor().dims(), [1, 2, 1]);
    }

    #[test]
    fn thmc_envelope_damage_and_time_accessors() {
        let device = Default::default();
        let env = ThmcEnvelope::with_zero_damage(setting_phase(), 3.14, &device);
        assert_eq!(env.damage_ref().as_tensor().dims(), [1, 2, 1]);
        assert!((env.time() - 3.14).abs() < f32::EPSILON);
    }

    #[test]
    fn thmc_envelope_new_matches_with_zero_damage_shape() {
        let phase = setting_phase();
        let damage = Field::new(zeros_3());
        let env = ThmcEnvelope::new(phase, damage, 7.0);
        assert_eq!(env.kind(), MaterialPhaseKind::Setting);
        assert!((env.time() - 7.0).abs() < f32::EPSILON);
        assert_eq!(env.damage_ref().as_tensor().dims(), [1, 2, 1]);
    }

    /// FP §3 witness: Bingham rheology routing succeeds only on the `Fluid` arm.
    fn try_bingham_route(phase: &MaterialPhase<B>) -> Option<&RheologyState<B>> {
        phase.as_fluid()
    }

    /// FP §3 witness: setting chemistry routing succeeds only on the `Setting` arm.
    fn try_setting_route(phase: &MaterialPhase<B>) -> Option<&SettingState<B>> {
        phase.as_setting()
    }

    /// FP §3 witness: equilibrium mechanics routing succeeds only on the `Solid` arm.
    fn try_mechanics_route(phase: &MaterialPhase<B>) -> Option<&MechanicsState<B>> {
        phase.as_solid()
    }

    #[test]
    fn invalid_cross_phase_solver_routes_unrepresentable() {
        assert!(try_bingham_route(&fluid_phase()).is_some());
        assert!(try_bingham_route(&setting_phase()).is_none());
        assert!(try_bingham_route(&solid_phase()).is_none());

        assert!(try_setting_route(&setting_phase()).is_some());
        assert!(try_setting_route(&fluid_phase()).is_none());
        assert!(try_setting_route(&solid_phase()).is_none());

        assert!(try_mechanics_route(&solid_phase()).is_some());
        assert!(try_mechanics_route(&fluid_phase()).is_none());
        assert!(try_mechanics_route(&setting_phase()).is_none());
    }

    #[test]
    fn material_phase_fluid_arm_exhaustive_match() {
        let phase = fluid_phase();
        let _ = match phase {
            MaterialPhase::Fluid(_) => (),
            MaterialPhase::Setting(_) | MaterialPhase::Solid(_) => {
                panic!(
                    "fluid fixture must not project to Setting/Solid \
                     (FP §3 invalid dual-phase state unrepresentable)"
                );
            }
        };
    }

    #[test]
    fn material_phase_setting_arm_exhaustive_match() {
        let phase = setting_phase();
        let _ = match phase {
            MaterialPhase::Setting(_) => (),
            MaterialPhase::Fluid(_) | MaterialPhase::Solid(_) => {
                panic!(
                    "setting fixture must not project to Fluid/Solid \
                     (FP §3 invalid dual-phase state unrepresentable)"
                );
            }
        };
    }

    #[test]
    fn material_phase_solid_arm_exhaustive_match() {
        let phase = solid_phase();
        let _ = match phase {
            MaterialPhase::Solid(_) => (),
            MaterialPhase::Fluid(_) | MaterialPhase::Setting(_) => {
                panic!(
                    "solid fixture must not project to Fluid/Setting \
                     (FP §3 invalid dual-phase state unrepresentable)"
                );
            }
        };
    }

    #[test]
    fn material_phase_kind_exhaustive_three_tags() {
        for (kind, label) in [
            (MaterialPhaseKind::Fluid, "fluid"),
            (MaterialPhaseKind::Setting, "setting"),
            (MaterialPhaseKind::Solid, "solid"),
        ] {
            let routed = match kind {
                MaterialPhaseKind::Fluid => "fluid",
                MaterialPhaseKind::Setting => "setting",
                MaterialPhaseKind::Solid => "solid",
            };
            assert_eq!(routed, label);
        }
    }

    #[test]
    fn invalid_simultaneous_phase_arms_unrepresentable() {
        for phase in [fluid_phase(), setting_phase(), solid_phase()] {
            let armed = usize::from(phase.as_fluid().is_some())
                + usize::from(phase.as_setting().is_some())
                + usize::from(phase.as_solid().is_some());
            assert_eq!(
                armed, 1,
                "MaterialPhase sum type must populate exactly one arm (FP §3)"
            );
        }
    }

    #[test]
    fn thmc_envelope_nested_kind_agrees_with_projection() {
        let device = Default::default();
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            let env = ThmcEnvelope::with_zero_damage(phase, 0.0, &device);
            assert_eq!(
                env.kind(),
                env.phase.kind(),
                "ThmcEnvelope::kind must agree with nested MaterialPhase::kind (FP §3)"
            );
            assert_eq!(env.kind(), expected);
        }
    }

    /// FP §3 witness: constitutive ops are admitted only on the matching phase arm.
    enum PhaseOp {
        BinghamStep,
        SettingChemistry,
        EquilibriumSolve,
    }

    fn admit_phase_op(phase: &MaterialPhase<B>, op: PhaseOp) -> Result<(), &'static str> {
        match (phase, op) {
            (MaterialPhase::Fluid(_), PhaseOp::BinghamStep) => Ok(()),
            (MaterialPhase::Setting(_), PhaseOp::SettingChemistry) => Ok(()),
            (MaterialPhase::Solid(_), PhaseOp::EquilibriumSolve) => Ok(()),
            (MaterialPhase::Fluid(_), PhaseOp::SettingChemistry | PhaseOp::EquilibriumSolve)
            | (MaterialPhase::Setting(_), PhaseOp::BinghamStep | PhaseOp::EquilibriumSolve)
            | (MaterialPhase::Solid(_), PhaseOp::BinghamStep | PhaseOp::SettingChemistry) => {
                Err("invalid cross-phase op — dual lifecycle state unrepresentable (FP §3)")
            }
        }
    }

    #[test]
    fn phase_operation_match_rejects_cross_arm_ops() {
        assert!(admit_phase_op(&fluid_phase(), PhaseOp::BinghamStep).is_ok());
        assert!(admit_phase_op(&fluid_phase(), PhaseOp::EquilibriumSolve).is_err());
        assert!(admit_phase_op(&fluid_phase(), PhaseOp::SettingChemistry).is_err());

        assert!(admit_phase_op(&setting_phase(), PhaseOp::SettingChemistry).is_ok());
        assert!(admit_phase_op(&setting_phase(), PhaseOp::BinghamStep).is_err());
        assert!(admit_phase_op(&setting_phase(), PhaseOp::EquilibriumSolve).is_err());

        assert!(admit_phase_op(&solid_phase(), PhaseOp::EquilibriumSolve).is_ok());
        assert!(admit_phase_op(&solid_phase(), PhaseOp::BinghamStep).is_err());
        assert!(admit_phase_op(&solid_phase(), PhaseOp::SettingChemistry).is_err());
    }

    #[test]
    fn mut_borrow_preserves_phase_kind_after_touch() {
        let mut fluid = fluid_phase();
        if let Some(r) = fluid.as_fluid_mut() {
            let _ = &mut r.velocity;
        }
        assert_eq!(fluid.kind(), MaterialPhaseKind::Fluid);

        let mut setting = setting_phase();
        if let Some(s) = setting.as_setting_mut() {
            let _ = &mut s.reaction_extent;
        }
        assert_eq!(setting.kind(), MaterialPhaseKind::Setting);

        let mut solid = solid_phase();
        if let Some(m) = solid.as_solid_mut() {
            let _ = &mut m.displacement;
        }
        assert_eq!(solid.kind(), MaterialPhaseKind::Solid);
    }

    #[test]
    fn thmc_envelope_clone_preserves_single_phase_arm() {
        let device = Default::default();
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            let env = ThmcEnvelope::with_zero_damage(phase, 1.0, &device);
            let cloned = env.clone();
            assert_eq!(cloned.kind(), expected);
            let p = &cloned.phase;
            let armed = usize::from(p.as_fluid().is_some())
                + usize::from(p.as_setting().is_some())
                + usize::from(p.as_solid().is_some());
            assert_eq!(
                armed, 1,
                "cloned ThmcEnvelope must retain exactly one MaterialPhase arm (FP §3)"
            );
        }
    }

    #[test]
    fn material_phase_kind_route_table_bijective() {
        fn lane(kind: MaterialPhaseKind) -> &'static str {
            match kind {
                MaterialPhaseKind::Fluid => "rheology",
                MaterialPhaseKind::Setting => "setting",
                MaterialPhaseKind::Solid => "mechanics",
            }
        }

        let lanes: Vec<_> = [
            MaterialPhaseKind::Fluid,
            MaterialPhaseKind::Setting,
            MaterialPhaseKind::Solid,
        ]
        .into_iter()
        .map(lane)
        .collect();
        assert_eq!(lanes, ["rheology", "setting", "mechanics"]);
        assert_eq!(lanes.len(), lanes.iter().collect::<std::collections::HashSet<_>>().len());
    }

    #[test]
    fn thmc_envelope_single_phase_arm_unrepresentable_dual() {
        let device = Default::default();
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            let env = ThmcEnvelope::with_zero_damage(phase, 0.0, &device);
            assert_eq!(env.kind(), expected);
            let p = &env.phase;
            let armed = usize::from(p.as_fluid().is_some())
                + usize::from(p.as_setting().is_some())
                + usize::from(p.as_solid().is_some());
            assert_eq!(
                armed, 1,
                "ThmcEnvelope must carry exactly one MaterialPhase arm (FP §3)"
            );
        }
    }

    #[test]
    fn material_phase_kind_roundtrip_all_fixtures() {
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            assert_eq!(phase.kind(), expected);
            let reprojected = match phase.kind() {
                MaterialPhaseKind::Fluid => MaterialPhaseKind::Fluid,
                MaterialPhaseKind::Setting => MaterialPhaseKind::Setting,
                MaterialPhaseKind::Solid => MaterialPhaseKind::Solid,
            };
            assert_eq!(reprojected, expected);
        }
    }

    #[test]
    fn material_phase_clone_preserves_single_arm() {
        for phase in [fluid_phase(), setting_phase(), solid_phase()] {
            let cloned = phase.clone();
            let armed = usize::from(cloned.as_fluid().is_some())
                + usize::from(cloned.as_setting().is_some())
                + usize::from(cloned.as_solid().is_some());
            assert_eq!(
                armed, 1,
                "clone must not introduce dual MaterialPhase arms (FP §3)"
            );
            assert_eq!(cloned.kind(), phase.kind());
        }
    }

    #[test]
    fn thmc_envelope_clone_preserves_single_arm() {
        let device = Default::default();
        for phase in [fluid_phase(), setting_phase(), solid_phase()] {
            let env = ThmcEnvelope::with_zero_damage(phase, 1.0, &device);
            let cloned = env.clone();
            let armed = usize::from(cloned.phase.as_fluid().is_some())
                + usize::from(cloned.phase.as_setting().is_some())
                + usize::from(cloned.phase.as_solid().is_some());
            assert_eq!(
                armed, 1,
                "ThmcEnvelope clone must preserve single MaterialPhase arm (FP §3)"
            );
            assert_eq!(cloned.kind(), env.kind());
        }
    }

    #[test]
    fn invalid_fluid_mechanics_stiffness_unrepresentable() {
        let fluid = fluid_phase();
        assert!(fluid.as_solid().is_none());
        assert!(fluid.as_fluid().is_some());
    }

    #[test]
    fn invalid_solid_bingham_yield_unrepresentable() {
        let solid = solid_phase();
        assert!(solid.as_fluid().is_none());
        assert!(solid.as_solid().is_some());
    }

    #[test]
    fn invalid_setting_equilibrium_displacement_unrepresentable() {
        let setting = setting_phase();
        assert!(setting.as_solid().is_none());
        assert!(setting.as_setting().is_some());
    }

    #[test]
    fn transport_routing_excludes_solid_arm() {
        fn try_transport_route(phase: &MaterialPhase<B>) -> bool {
            phase.as_fluid().is_some() || phase.as_setting().is_some()
        }

        assert!(try_transport_route(&fluid_phase()));
        assert!(try_transport_route(&setting_phase()));
        assert!(!try_transport_route(&solid_phase()));
    }

    #[test]
    fn material_phase_kind_matches_exhaustive_match_projection() {
        for (phase, expected) in [
            (fluid_phase(), MaterialPhaseKind::Fluid),
            (setting_phase(), MaterialPhaseKind::Setting),
            (solid_phase(), MaterialPhaseKind::Solid),
        ] {
            let projected = match &phase {
                MaterialPhase::Fluid(_) => MaterialPhaseKind::Fluid,
                MaterialPhase::Setting(_) => MaterialPhaseKind::Setting,
                MaterialPhase::Solid(_) => MaterialPhaseKind::Solid,
            };
            assert_eq!(projected, expected);
            assert_eq!(phase.kind(), projected);
        }
    }
}

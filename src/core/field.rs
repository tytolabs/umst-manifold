// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §1 — phantom-typed Burn tensor carriers ([`Field`]).
//!
//! Staging vocabulary for THMC / fracture migration (P3): solvers still accept naked
//! [`burn::tensor::Tensor`] at call sites. This module introduces compile-time space
//! witnesses without breaking Burn APIs; P3.1–P3.7 schedule in
//! `outputs/.tmp/fp_p3_thmc_field_migration_plan.md`.
//!
//! Prior art: [`super::dec_typestate::B1Incidence`] (topology) and
//! [`super::dec_typestate::ScalarChannelIdx`] (scalar layout).
//!
//! # Migration
//!
//! P3.1 wrapped `ThmcState` plan fields; solvers unwrap via [`Field::as_tensor`] / [`Field::into_tensor`]
//! at kernel boundaries. P3.2–P3.7 schedule in `outputs/.tmp/fp_p3_thmc_field_migration_plan.md`.

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Tensor};

/// Phantom space marker: nodal temperature field \(T\) — shape `[B, N, F_T]`, kelvin.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_TEMPERATURE`.
#[derive(Clone, Copy, Debug)]
pub struct Temperature;

/// Phantom space marker: pore-fluid / humidity proxy \(h\) — shape `[B, N, F_h]`, typically `[0, 1]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_HUMIDITY`.
#[derive(Clone, Copy, Debug)]
pub struct Humidity;

/// Phantom space marker: mechanical displacement \(\mathbf u\) — shape `[B, N, 3]`, SI metres.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; vector channel SSOT is `vector_features[*, 0, *]`.
#[derive(Clone, Copy, Debug)]
pub struct Displacement;

/// Phantom space marker: phase-field / continuum damage \(d\) — shape `[B, N, 1]` (or `[B, N, F_d]`).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; layout SSOT is `umst_schema::SCALAR_DAMAGE`.
#[derive(Clone, Copy, Debug)]
pub struct Damage;

/// Phantom space marker: chemical reaction extent \(\alpha\) — shape `[B, N, F_\alpha]`, clipped to `[0, 1]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; THMC chemical channel (P3.1 migration target).
#[derive(Clone, Copy, Debug)]
pub struct ReactionExtent;

/// Phantom space marker: symmetric small strain ε — shape `[B, N, 3, 3]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; fracture AT2 strain rank/shape SSOT is `[B, N, 3, 3]` symmetric tensor layout.
#[derive(Clone, Copy, Debug)]
pub struct SmallStrain;

/// Phantom space marker: fracture energy release rate \(G_c\) — shape `[B, N, 1]`, J/m².
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; AT2 fracture toughness nodal field SSOT is `[B, N, 1]` (distinct from [`Damage`] despite shared rank).
#[derive(Clone, Copy, Debug)]
pub struct FractureEnergy;

/// Phantom space marker: nodal stiffness / modulus pair \([E_\mathrm{young}, \nu]\) — shape `[B, N, 2]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Zero-sized space witness; bar-network mechanics SSOT is `[B, N, 2]` with columns `[E, ν]` (distinct from [`Damage`] / [`ReactionExtent`] despite shared rank-3).
#[derive(Clone, Copy, Debug)]
pub struct Stiffness;

/// Phantom-typed tensor carrier: physical meaning encoded at compile time via `Space`.
///
/// Uses `PhantomData<fn() -> Space>` so the space witness is invariant (not covariant),
/// preventing accidental subtyping between distinct material quantities that share rank/shape.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Pure newtype over Burn `Tensor`; no new physics claim beyond caller layout contracts.
#[derive(Clone, Debug)]
pub struct Field<B: Backend, Space, const D: usize> {
    tensor: Tensor<B, D>,
    _space: PhantomData<fn() -> Space>,
}

impl<B: Backend, Space, const D: usize> Field<B, Space, D> {
    /// Wrap an existing Burn tensor (layout contracts remain caller-owned).
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Opaque constructor; does not validate shape or units.
    #[inline]
    #[must_use]
    pub fn new(tensor: Tensor<B, D>) -> Self {
        Self {
            tensor,
            _space: PhantomData,
        }
    }

    /// Borrow the underlying Burn tensor for kernel / solver ops.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Explicit escape hatch to Burn APIs; preserves staging boundary.
    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, D> {
        &self.tensor
    }

    /// Consume and return the underlying Burn tensor.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Explicit escape hatch to Burn APIs; preserves staging boundary.
    #[inline]
    pub fn into_tensor(self) -> Tensor<B, D> {
        self.tensor
    }

    /// Map the inner tensor while preserving the space witness.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Functorial map over carrier; space marker unchanged by construction.
    #[inline]
    #[must_use]
    pub fn map(self, f: impl FnOnce(Tensor<B, D>) -> Tensor<B, D>) -> Self {
        Self::new(f(self.tensor))
    }
}

/// Temperature plan field — `[B, N, F_T]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Temperature`] witness.
pub type TemperatureField<B> = Field<B, Temperature, 3>;
/// Humidity plan field — `[B, N, F_h]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Humidity`] witness.
pub type HumidityField<B> = Field<B, Humidity, 3>;
/// Displacement plan field — `[B, N, 3]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Displacement`] witness.
pub type DisplacementField<B> = Field<B, Displacement, 3>;
/// Damage plan field — `[B, N, 1]` (typical).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Damage`] witness.
pub type DamageField<B> = Field<B, Damage, 3>;
/// Reaction-extent plan field — `[B, N, F_\alpha]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`ReactionExtent`] witness.
pub type ReactionExtentField<B> = Field<B, ReactionExtent, 3>;
/// Small-strain tensor field — `[B, N, 3, 3]` symmetric ε.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-4 alias for [`Field`] with [`SmallStrain`] witness.
pub type SmallStrainField<B> = Field<B, SmallStrain, 4>;
/// Fracture-energy plan field — `[B, N, 1]` \(G_c\).
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`FractureEnergy`] witness.
pub type FractureEnergyField<B> = Field<B, FractureEnergy, 3>;
/// Stiffness / modulus plan field — `[B, N, 2]` with columns `[E_\mathrm{young}, \nu]`.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Rank-3 alias for [`Field`] with [`Stiffness`] witness.
pub type StiffnessField<B> = Field<B, Stiffness, 3>;

/// Frozen damage mask at THMC step entry — distinct from live `state.damage` after fracture.
#[derive(Clone, Debug)]
pub struct StepEntryDamageMask<B: Backend>(DamageField<B>);

impl<B: Backend> StepEntryDamageMask<B> {
    #[inline]
    #[must_use]
    pub fn from_damage_field(damage: DamageField<B>) -> Self {
        Self(damage)
    }

    #[must_use]
    pub fn from_step_entry_damage(
        state_damage: &DamageField<B>,
        batch: usize,
        n: usize,
    ) -> Self {
        let damage_tensor = state_damage.as_tensor();
        let tensor = match damage_tensor.dims()[2] {
            1 => damage_tensor.clone(),
            _ => damage_tensor.clone().slice([0..batch, 0..n, 0..1]),
        };
        Self(Field::new(tensor))
    }

    #[deprecated(since = "0.2.0", note = "use from_step_entry_damage — FP P3.2")]
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Self(Field::new(tensor))
    }

    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, 3> {
        self.0.as_tensor()
    }

    #[inline]
    pub fn as_damage_field(&self) -> &DamageField<B> {
        &self.0
    }

    #[inline]
    pub fn into_damage_field(self) -> DamageField<B> {
        self.0
    }
}

impl<B: Backend> FractureEnergyField<B> {
    /// Zero-filled fracture-energy field.
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    /// Wrap an existing fracture-energy tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }
}

impl<B: Backend> StiffnessField<B> {
    /// Zero-filled stiffness field.
    #[must_use]
    pub fn zeros(dims: [usize; 3], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 3>::zeros(dims, device))
    }

    /// Wrap an existing `[B, N, 2]` stiffness tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 3>) -> Self {
        Field::new(tensor)
    }

    /// Canonical bar-network assembly: `cat([e_young, nu], dim=2)` → `[B, N, 2]`.
    ///
    /// `e_young` and `nu` are typically `[B, N, 1]` nodal channels.
    #[inline]
    #[must_use]
    pub fn from_e_nu_cat(e_young: Tensor<B, 3>, nu: Tensor<B, 3>) -> Self {
        Field::new(Tensor::cat(vec![e_young, nu], 2))
    }
}

impl<B: Backend> SmallStrainField<B> {
    /// Zero-filled small-strain field.
    #[must_use]
    pub fn zeros(dims: [usize; 4], device: &B::Device) -> Self {
        Field::new(Tensor::<B, 4>::zeros(dims, device))
    }

    /// Wrap an existing symmetric strain tensor.
    #[inline]
    #[must_use]
    pub fn from_tensor(tensor: Tensor<B, 4>) -> Self {
        Field::new(tensor)
    }
}

#[cfg(test)]
mod tests {
    use burn_ndarray::NdArray;
    use burn::tensor::Tensor;

    use super::*;

    type B = NdArray;

    #[test]
    fn field_newtype_round_trips_tensor() {
        let device = Default::default();
        let raw = Tensor::<B, 3>::ones([1, 4, 1], &device);
        let field: TemperatureField<B> = Field::new(raw.clone());
        assert_eq!(field.as_tensor().dims(), [1, 4, 1]);
        assert_eq!(field.clone().into_tensor().dims(), raw.dims());
    }

    #[test]
    fn field_map_preserves_space_marker() {
        let device = Default::default();
        let raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        let scaled = Field::<B, Humidity, 3>::new(raw).map(|t| t.add_scalar(0.5_f32));
        assert!((scaled.as_tensor().clone().into_data().value[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn distinct_space_markers_are_separate_types() {
        fn accept_temperature(_: TemperatureField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_temperature(Field::new(raw.clone()));
        accept_damage(Field::new(raw));
    }

    #[test]
    fn small_strain_field_distinct_from_damage() {
        fn accept_strain(_: SmallStrainField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let strain_raw = Tensor::<B, 4>::zeros([1, 2, 3, 3], &device);
        let damage_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_strain(Field::new(strain_raw));
        accept_damage(Field::new(damage_raw));
    }

    #[test]
    fn fracture_energy_field_distinct_from_damage() {
        fn accept_gc(_: FractureEnergyField<B>) {}
        fn accept_damage(_: DamageField<B>) {}

        let device = Default::default();
        let gc_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        let damage_raw = Tensor::<B, 3>::zeros([1, 2, 1], &device);
        accept_gc(FractureEnergyField::from_tensor(gc_raw));
        accept_damage(Field::new(damage_raw));
    }

    #[test]
    fn stiffness_field_distinct_from_damage_and_reaction_extent() {
        fn accept_stiffness(_: StiffnessField<B>) {}
        fn accept_damage(_: DamageField<B>) {}
        fn accept_alpha(_: ReactionExtentField<B>) {}

        let device = Default::default();
        let stiff = StiffnessField::from_e_nu_cat(
            Tensor::<B, 3>::zeros([1, 2, 1], &device),
            Tensor::<B, 3>::zeros([1, 2, 1], &device),
        );
        accept_stiffness(stiff);
        accept_damage(Field::new(Tensor::<B, 3>::zeros([1, 2, 1], &device)));
        accept_alpha(Field::new(Tensor::<B, 3>::zeros([1, 2, 1], &device)));
    }
}

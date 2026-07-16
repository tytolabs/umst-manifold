// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP Manifesto §1 — phantom-typed Burn tensor carriers ([`Field`]).
//!
//! Staging layer only: solvers still accept naked [`burn::tensor::Tensor`] at call sites.
//! This module introduces the newtype vocabulary without breaking Burn APIs; full carve deferred.

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Tensor};

/// Nodal temperature field \(T\) — shape `[B, N, F_T]`, kelvin.
#[derive(Clone, Copy, Debug)]
pub struct Temperature;

/// Pore-fluid / humidity proxy \(h\) — shape `[B, N, F_h]`, typically in `[0, 1]`.
#[derive(Clone, Copy, Debug)]
pub struct Humidity;

/// Mechanical displacement \(\mathbf u\) — shape `[B, N, 3]`, SI metres.
#[derive(Clone, Copy, Debug)]
pub struct Displacement;

/// Phase-field / continuum damage \(d\) — shape `[B, N, 1]` (or `[B, N, F_d]`).
#[derive(Clone, Copy, Debug)]
pub struct Damage;

/// Chemical reaction extent \(\alpha\) — shape `[B, N, F_\alpha]`, clipped to `[0, 1]`.
#[derive(Clone, Copy, Debug)]
pub struct ReactionExtent;

/// Phantom-typed tensor carrier: physical meaning encoded at compile time via `Space`.
#[derive(Clone, Debug)]
pub struct Field<B: Backend, Space, const D: usize> {
    tensor: Tensor<B, D>,
    _space: PhantomData<fn() -> Space>,
}

impl<B: Backend, Space, const D: usize> Field<B, Space, D> {
    /// Wrap an existing Burn tensor (layout contracts remain caller-owned).
    #[inline]
    #[must_use]
    pub fn new(tensor: Tensor<B, D>) -> Self {
        Self {
            tensor,
            _space: PhantomData,
        }
    }

    /// Borrow the underlying Burn tensor for kernel / solver ops.
    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, D> {
        &self.tensor
    }

    /// Consume and return the underlying Burn tensor.
    #[inline]
    pub fn into_tensor(self) -> Tensor<B, D> {
        self.tensor
    }

    /// Map the inner tensor while preserving the space witness.
    #[inline]
    #[must_use]
    pub fn map(self, f: impl FnOnce(Tensor<B, D>) -> Tensor<B, D>) -> Self {
        Self::new(f(self.tensor))
    }
}

/// Temperature plan field — `[B, N, F_T]`.
pub type TemperatureField<B> = Field<B, Temperature, 3>;
/// Humidity plan field — `[B, N, F_h]`.
pub type HumidityField<B> = Field<B, Humidity, 3>;
/// Displacement plan field — `[B, N, 3]`.
pub type DisplacementField<B> = Field<B, Displacement, 3>;
/// Damage plan field — `[B, N, 1]` (typical).
pub type DamageField<B> = Field<B, Damage, 3>;
/// Reaction-extent plan field — `[B, N, F_\alpha]`.
pub type ReactionExtentField<B> = Field<B, ReactionExtent, 3>;

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
}

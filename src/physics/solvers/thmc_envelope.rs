// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! MP2 bridge — bijection between flat [`ThmcState`] and algebraic [`ThmcEnvelope`].
//!
//! Parity window: legacy golden tests continue to use flat product; envelope routing lands in
//! `ThmcSolver::step_envelope` (MP2b). See `outputs/.tmp/fp_matph_mp2_spec.md`.

use burn::tensor::{backend::Backend, Tensor};

use crate::core::field::{
    DisplacementField, HumidityField, ReactionExtentField, TemperatureField,
};
use crate::core::material_phase::{
    MaterialPhase, MaterialPhaseKind, MechanicsState, RheologyState, SettingState, ThmcEnvelope,
};

use super::thmc::ThmcState;

impl<B: Backend> ThmcEnvelope<B> {
    /// Classify flat state into envelope using caller-supplied phase witness (MP2).
    ///
    /// Thresholds are **not** hardcoded in manifold — caller supplies [`MaterialPhaseKind`]
    /// until MP3 cartridge `Profile` supplies them.
    #[must_use]
    pub fn from_flat_state(state: &ThmcState<B>, kind: MaterialPhaseKind) -> Self {
        let damage = state.damage.clone();
        let time = state.time;
        let device = state.thermal.temperature.as_tensor().device();
        let phase = match kind {
            MaterialPhaseKind::Fluid => {
                let shape = state.chemical.reaction_extent.as_tensor().dims();
                let zeros = Tensor::<B, 3>::zeros(shape, &device);
                MaterialPhase::Fluid(RheologyState {
                    yield_stress: zeros.clone(),
                    plastic_viscosity: zeros,
                    velocity: state.mechanical.displacement.as_tensor().clone(),
                })
            }
            MaterialPhaseKind::Setting => MaterialPhase::Setting(SettingState {
                reaction_extent: state.chemical.reaction_extent.as_tensor().clone(),
                humidity: state.hydro.humidity.as_tensor().clone(),
                temperature: state.thermal.temperature.as_tensor().clone(),
            }),
            MaterialPhaseKind::Solid => {
                let stiff_shape = state.chemical.reaction_extent.as_tensor().dims();
                let stiffness = Tensor::<B, 3>::zeros(stiff_shape, &device);
                MaterialPhase::Solid(MechanicsState {
                    displacement: state.mechanical.displacement.as_tensor().clone(),
                    damage: damage.as_tensor().clone(),
                    stiffness,
                })
            }
        };
        Self { phase, damage, time }
    }

    /// Parity window: reconstruct flat product for legacy callers / golden hashes.
    #[deprecated(note = "MP2 parity shim — remove after MP4")]
    #[must_use]
    pub fn to_flat_state(&self) -> ThmcState<B> {
        let device = self.damage.as_tensor().device();
        let [batch, n, _] = self.damage.as_tensor().dims();
        let zeros_n1 = Tensor::<B, 3>::zeros([batch, n, 1], &device);
        let zeros_n3 = Tensor::<B, 3>::zeros([batch, n, 3], &device);

        let (temperature, humidity, displacement, reaction_extent) = match &self.phase {
            MaterialPhase::Fluid(r) => (
                TemperatureField::new(zeros_n1.clone()),
                HumidityField::new(zeros_n1.clone()),
                DisplacementField::new(r.velocity.clone()),
                ReactionExtentField::new(zeros_n1.clone()),
            ),
            MaterialPhase::Setting(s) => (
                TemperatureField::new(s.temperature.clone()),
                HumidityField::new(s.humidity.clone()),
                DisplacementField::new(zeros_n3),
                ReactionExtentField::new(s.reaction_extent.clone()),
            ),
            MaterialPhase::Solid(m) => {
                let alpha = ReactionExtentField::new(zeros_n1.clone());
                (
                    TemperatureField::new(zeros_n1.clone()),
                    HumidityField::new(zeros_n1),
                    DisplacementField::new(m.displacement.clone()),
                    alpha,
                )
            }
        };

        ThmcState::from_tensors(
            temperature.into_tensor(),
            humidity.into_tensor(),
            displacement.into_tensor(),
            reaction_extent.into_tensor(),
            self.damage.clone().into_tensor(),
            self.time,
        )
    }
}

#[cfg(test)]
mod tests {
    use burn_ndarray::NdArray;

    use super::*;

    type B = NdArray;

    fn sample_flat(device: &<B as burn::tensor::backend::Backend>::Device) -> ThmcState<B> {
        let t = Tensor::<B, 3>::zeros([1, 4, 1], device);
        let h = Tensor::<B, 3>::zeros([1, 4, 1], device);
        let u = Tensor::<B, 3>::zeros([1, 4, 3], device);
        let alpha = Tensor::<B, 3>::zeros([1, 4, 1], device);
        let d = Tensor::<B, 3>::zeros([1, 4, 1], device);
        ThmcState::from_tensors(t, h, u, alpha, d, 1.5)
    }

    /// Parity shim entry — single deprecated-allow site for MP2 golden roundtrips.
    #[allow(deprecated)]
    fn to_flat(env: &ThmcEnvelope<B>) -> ThmcState<B> {
        env.to_flat_state()
    }

    #[test]
    fn from_flat_setting_preserves_transport_and_time() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        assert_eq!(env.kind(), MaterialPhaseKind::Setting);
        assert!((env.time - 1.5).abs() < f32::EPSILON);
        let s = env.phase.as_setting().expect(
            "ThmcEnvelope::from_flat_state(Setting) must yield MaterialPhase::Setting arm \
             (MP2 bijection witness)",
        );
        assert_eq!(s.temperature.dims(), [1, 4, 1]);
        assert_eq!(s.humidity.dims(), [1, 4, 1]);
    }

    #[test]
    fn flat_roundtrip_setting_preserves_damage_and_time() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        let back = to_flat(&env);
        assert!((back.time - flat.time).abs() < f32::EPSILON);
        assert_eq!(back.damage.as_tensor().dims(), flat.damage.as_tensor().dims());
    }

    #[test]
    fn from_flat_fluid_routes_velocity_and_kind() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        assert_eq!(env.kind(), MaterialPhaseKind::Fluid);
        let r = env.phase.as_fluid().expect(
            "ThmcEnvelope::from_flat_state(Fluid) must yield MaterialPhase::Fluid arm \
             (MP2 bijection witness)",
        );
        assert_eq!(r.velocity.dims(), [1, 4, 3]);
        assert_eq!(r.yield_stress.dims(), [1, 4, 1]);
        assert_eq!(r.plastic_viscosity.dims(), [1, 4, 1]);
    }

    #[test]
    fn from_flat_solid_routes_displacement_damage_and_kind() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Solid);
        assert_eq!(env.kind(), MaterialPhaseKind::Solid);
        let m = env.phase.as_solid().expect(
            "ThmcEnvelope::from_flat_state(Solid) must yield MaterialPhase::Solid arm \
             (MP2 bijection witness)",
        );
        assert_eq!(m.displacement.dims(), [1, 4, 3]);
        assert_eq!(m.damage.dims(), [1, 4, 1]);
        assert_eq!(m.stiffness.dims(), [1, 4, 1]);
    }

    #[test]
    fn envelope_damage_preserved_across_all_kinds() {
        let device = Default::default();
        let flat = sample_flat(&device);
        for kind in [
            MaterialPhaseKind::Fluid,
            MaterialPhaseKind::Setting,
            MaterialPhaseKind::Solid,
        ] {
            let env = ThmcEnvelope::from_flat_state(&flat, kind);
            assert_eq!(env.damage.as_tensor().dims(), flat.damage.as_tensor().dims());
            assert!((env.time - flat.time).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn flat_roundtrip_fluid_preserves_velocity_dims() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        let back = to_flat(&env);
        assert_eq!(
            back.mechanical.displacement.as_tensor().dims(),
            [1, 4, 3]
        );
    }

    #[test]
    fn flat_roundtrip_solid_preserves_displacement_dims() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Solid);
        let back = to_flat(&env);
        assert_eq!(
            back.mechanical.displacement.as_tensor().dims(),
            [1, 4, 3]
        );
    }

    #[test]
    fn idempotency_to_flat_from_flat_setting() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        let once = to_flat(&env);
        let env2 = ThmcEnvelope::from_flat_state(&once, MaterialPhaseKind::Setting);
        assert_eq!(env2.kind(), MaterialPhaseKind::Setting);
        assert!((env2.time - env.time).abs() < f32::EPSILON);
        assert_eq!(
            env2.damage.as_tensor().dims(),
            env.damage.as_tensor().dims()
        );
    }

    #[test]
    fn idempotency_to_flat_from_flat_fluid() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        let once = to_flat(&env);
        let env2 = ThmcEnvelope::from_flat_state(&once, MaterialPhaseKind::Fluid);
        assert_eq!(env2.kind(), MaterialPhaseKind::Fluid);
        assert!((env2.time - env.time).abs() < f32::EPSILON);
        let r = env2.phase.as_fluid().expect(
            "idempotent from_flat(Fluid) roundtrip must preserve MaterialPhase::Fluid arm \
             (MP2 bijection witness)",
        );
        assert_eq!(r.velocity.dims(), [1, 4, 3]);
    }

    #[test]
    fn idempotency_to_flat_from_flat_solid() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Solid);
        let once = to_flat(&env);
        let env2 = ThmcEnvelope::from_flat_state(&once, MaterialPhaseKind::Solid);
        assert_eq!(env2.kind(), MaterialPhaseKind::Solid);
        assert!((env2.time - env.time).abs() < f32::EPSILON);
        let m = env2.phase.as_solid().expect(
            "idempotent from_flat(Solid) roundtrip must preserve MaterialPhase::Solid arm \
             (MP2 bijection witness)",
        );
        assert_eq!(m.displacement.dims(), [1, 4, 3]);
    }
}

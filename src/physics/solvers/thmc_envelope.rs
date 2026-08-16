// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! MP2 bridge — bijection between flat [`ThmcState`] and algebraic [`ThmcEnvelope`].
//!
//! Parity window: legacy golden tests continue to use flat product; envelope routing lands in
//! `ThmcSolver::step_envelope` (MP2b). See `old/residuals/residuals/misc-outputs-tmp/fp_matph_mp2_spec.md`.
//!
//! # Honest fences (W29-082)
//!
//! - **Not claimed:** GREEN / PRODUCTION_WIRED / MASTER / OP-5. This module is a typed bijection
//!   + writeback shim only — not a physics-admissibility or production-wire verdict.
//! - **Not claimed:** cartridge `Profile` α/T thresholds (MP3). [`from_flat_state`] requires an
//!   explicit [`MaterialPhaseKind`] witness; it does not classify from flat scalars.
//! - **Not claimed:** Fluid rheology params or Solid stiffness from flat product — those channels
//!   are zero-filled on ingress (flat `ThmcState` does not carry them); sync does not invent them.
//! - **Parity shim:** [`to_flat_state`] is deprecated (remove after MP4); zeros fill inactive arms.
//! - **Writeback:** [`sync_from_flat_state`] preserves the active arm; it never reclassifies kind.

use burn::tensor::{backend::Backend, Tensor};

use crate::core::field::{DisplacementField, HumidityField, ReactionExtentField, TemperatureField};
use crate::core::material_phase::{
    MaterialPhase, MaterialPhaseKind, MechanicsState, RheologyState, SettingState, ThmcEnvelope,
};

use super::thmc::ThmcState;

impl<B: Backend> ThmcEnvelope<B> {
    /// Classify flat state into envelope using caller-supplied phase witness (MP2).
    ///
    /// Thresholds are **not** hardcoded in manifold — caller supplies [`MaterialPhaseKind`]
    /// until MP3 cartridge `Profile` supplies them. Does **not** invent GREEN / PRODUCTION_WIRED.
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
        Self {
            phase,
            damage,
            time,
        }
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

    /// MP2b: project flat [`ThmcState`] after routed step back into the algebraic envelope.
    ///
    /// Preserves the active [`MaterialPhase`] arm; does not reclassify phase kind.
    /// Fluid yield/viscosity and Solid stiffness are left unchanged (not present on flat product).
    /// Not a GREEN / PRODUCTION_WIRED claim — writeback only.
    pub fn sync_from_flat_state(&mut self, flat: &ThmcState<B>) {
        self.time = flat.time;
        self.damage = flat.damage.clone();
        match &mut self.phase {
            MaterialPhase::Fluid(r) => {
                r.velocity = flat.mechanical.displacement.as_tensor().clone();
            }
            MaterialPhase::Setting(s) => {
                s.reaction_extent = flat.chemical.reaction_extent.as_tensor().clone();
                s.humidity = flat.hydro.humidity.as_tensor().clone();
                s.temperature = flat.thermal.temperature.as_tensor().clone();
            }
            MaterialPhase::Solid(m) => {
                m.displacement = flat.mechanical.displacement.as_tensor().clone();
                m.damage = flat.damage.as_tensor().clone();
            }
        }
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

    /// Non-zero flat product for value-level bijection / writeback witnesses.
    fn sample_flat_nonzero(device: &<B as burn::tensor::backend::Backend>::Device) -> ThmcState<B> {
        let t = Tensor::<B, 3>::from_floats([[[1.0], [2.0], [3.0], [4.0]]], device);
        let h = Tensor::<B, 3>::from_floats([[[0.1], [0.2], [0.3], [0.4]]], device);
        let u = Tensor::<B, 3>::from_floats(
            [[
                [1.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
                [0.0, 0.0, 3.0],
                [0.5, 0.5, 0.5],
            ]],
            device,
        );
        let alpha = Tensor::<B, 3>::from_floats([[[0.25], [0.50], [0.75], [1.0]]], device);
        let d = Tensor::<B, 3>::from_floats([[[0.01], [0.02], [0.03], [0.04]]], device);
        ThmcState::from_tensors(t, h, u, alpha, d, 2.25)
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
             (MP2 bijection witness) (FP §6 Track MP2 transport envelope harness)",
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
        assert_eq!(
            back.damage.as_tensor().dims(),
            flat.damage.as_tensor().dims()
        );
    }

    #[test]
    fn from_flat_fluid_routes_velocity_and_kind() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        assert_eq!(env.kind(), MaterialPhaseKind::Fluid);
        let r = env.phase.as_fluid().expect(
            "ThmcEnvelope::from_flat_state(Fluid) must yield MaterialPhase::Fluid arm \
             (MP2 bijection witness) (FP §6 Track MP2 transport envelope harness)",
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
             (MP2 bijection witness) (FP §6 Track MP2 transport envelope harness)",
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
            assert_eq!(
                env.damage.as_tensor().dims(),
                flat.damage.as_tensor().dims()
            );
            assert!((env.time - flat.time).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn flat_roundtrip_fluid_preserves_velocity_dims() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        let back = to_flat(&env);
        assert_eq!(back.mechanical.displacement.as_tensor().dims(), [1, 4, 3]);
    }

    #[test]
    fn flat_roundtrip_solid_preserves_displacement_dims() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Solid);
        let back = to_flat(&env);
        assert_eq!(back.mechanical.displacement.as_tensor().dims(), [1, 4, 3]);
    }

    #[test]
    fn sync_from_flat_setting_preserves_transport_and_clock() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let mut env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        let mut stepped = flat.clone();
        stepped.time += 0.5;
        env.sync_from_flat_state(&stepped);
        assert!((env.time - stepped.time).abs() < f32::EPSILON);
        let s = env.phase.as_setting().expect(
            "sync_from_flat_state(Setting) must retain MaterialPhase::Setting arm \
             (MP2b U3 envelope writeback witness)",
        );
        assert_eq!(s.temperature.dims(), [1, 4, 1]);
        assert_eq!(s.humidity.dims(), [1, 4, 1]);
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
             (MP2 bijection witness) (FP §6 Track MP2 transport envelope harness)",
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
             (MP2 bijection witness) (FP §6 Track MP2 transport envelope harness)",
        );
        assert_eq!(m.displacement.dims(), [1, 4, 3]);
    }

    #[test]
    fn sync_from_flat_fluid_writes_velocity_and_clock() {
        let device = Default::default();
        let flat = sample_flat_nonzero(&device);
        let mut env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Fluid);
        let mut stepped = flat.clone();
        stepped.time += 0.25;
        env.sync_from_flat_state(&stepped);
        assert_eq!(env.kind(), MaterialPhaseKind::Fluid);
        assert!((env.time - stepped.time).abs() < f32::EPSILON);
        let r = env.phase.as_fluid().expect(
            "sync_from_flat_state(Fluid) must retain MaterialPhase::Fluid arm \
             (MP2b U3 envelope writeback witness)",
        );
        assert_eq!(
            r.velocity.clone().into_data().value,
            stepped
                .mechanical
                .displacement
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
        // Honest fence: rheology zeros on ingress remain untouched by sync.
        let ys_sum: f32 = r.yield_stress.clone().into_data().value.iter().sum();
        assert!((ys_sum).abs() < f32::EPSILON);
    }

    #[test]
    fn sync_from_flat_solid_writes_displacement_damage_and_clock() {
        let device = Default::default();
        let flat = sample_flat_nonzero(&device);
        let mut env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Solid);
        let mut stepped = flat.clone();
        stepped.time += 1.0;
        env.sync_from_flat_state(&stepped);
        assert_eq!(env.kind(), MaterialPhaseKind::Solid);
        assert!((env.time - stepped.time).abs() < f32::EPSILON);
        let m = env.phase.as_solid().expect(
            "sync_from_flat_state(Solid) must retain MaterialPhase::Solid arm \
             (MP2b U3 envelope writeback witness)",
        );
        assert_eq!(
            m.displacement.clone().into_data().value,
            stepped
                .mechanical
                .displacement
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
        assert_eq!(
            m.damage.clone().into_data().value,
            stepped.damage.as_tensor().clone().into_data().value
        );
        assert_eq!(
            env.damage.as_tensor().clone().into_data().value,
            stepped.damage.as_tensor().clone().into_data().value
        );
        // Honest fence: stiffness zero-fill on ingress is not invented by sync.
        let stiff_sum: f32 = m.stiffness.clone().into_data().value.iter().sum();
        assert!((stiff_sum).abs() < f32::EPSILON);
    }

    #[test]
    fn sync_preserves_kind_for_all_arms() {
        let device = Default::default();
        let flat = sample_flat_nonzero(&device);
        for kind in [
            MaterialPhaseKind::Fluid,
            MaterialPhaseKind::Setting,
            MaterialPhaseKind::Solid,
        ] {
            let mut env = ThmcEnvelope::from_flat_state(&flat, kind);
            let mut stepped = flat.clone();
            stepped.time += 0.1;
            env.sync_from_flat_state(&stepped);
            assert_eq!(
                env.kind(),
                kind,
                "sync_from_flat_state must not reclassify MaterialPhaseKind (honest fence)"
            );
        }
    }

    #[test]
    fn from_flat_setting_nonzero_preserves_transport_values() {
        let device = Default::default();
        let flat = sample_flat_nonzero(&device);
        let env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        let s = env
            .phase
            .as_setting()
            .expect("from_flat_state(Setting) nonzero must yield Setting arm (MP2 value witness)");
        assert_eq!(
            s.temperature.clone().into_data().value,
            flat.thermal
                .temperature
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
        assert_eq!(
            s.humidity.clone().into_data().value,
            flat.hydro.humidity.as_tensor().clone().into_data().value
        );
        assert_eq!(
            s.reaction_extent.clone().into_data().value,
            flat.chemical
                .reaction_extent
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
        assert_eq!(
            env.damage.as_tensor().clone().into_data().value,
            flat.damage.as_tensor().clone().into_data().value
        );
        assert!((env.time - 2.25).abs() < f32::EPSILON);
    }

    #[test]
    fn sync_from_flat_setting_nonzero_writeback_values() {
        let device = Default::default();
        let flat = sample_flat(&device);
        let mut env = ThmcEnvelope::from_flat_state(&flat, MaterialPhaseKind::Setting);
        let stepped = sample_flat_nonzero(&device);
        env.sync_from_flat_state(&stepped);
        assert_eq!(env.kind(), MaterialPhaseKind::Setting);
        assert!((env.time - stepped.time).abs() < f32::EPSILON);
        let s = env
            .phase
            .as_setting()
            .expect("sync Setting nonzero writeback must retain Setting arm (MP2b value witness)");
        assert_eq!(
            s.temperature.clone().into_data().value,
            stepped
                .thermal
                .temperature
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
        assert_eq!(
            s.humidity.clone().into_data().value,
            stepped.hydro.humidity.as_tensor().clone().into_data().value
        );
        assert_eq!(
            s.reaction_extent.clone().into_data().value,
            stepped
                .chemical
                .reaction_extent
                .as_tensor()
                .clone()
                .into_data()
                .value
        );
    }
}

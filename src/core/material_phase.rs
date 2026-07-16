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
}

/// THMC envelope: macroscopic phase + simulation clock.
///
/// Target replacement for flat [`crate::physics::solvers::thmc::ThmcState`] in MP2.
/// MP1 introduces the type only; legacy flat accessors remain until parity window closes.
///
/// formal_anchor: NONE
/// formal_status: Structural
/// formal_anchor_rationale: Product of [`MaterialPhase`] and scalar clock; no solver coupling yet.
#[derive(Clone, Debug)]
pub struct ThmcEnvelope<B: Backend> {
    pub phase: MaterialPhase<B>,
    pub time: f32,
}

impl<B: Backend> ThmcEnvelope<B> {
    /// Construct an envelope from a phase variant and clock.
    ///
    /// formal_anchor: NONE
    /// formal_status: Structural
    /// formal_anchor_rationale: Pure constructor; does not validate tensor layouts.
    #[inline]
    #[must_use]
    pub fn new(phase: MaterialPhase<B>, time: f32) -> Self {
        Self { phase, time }
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
    fn thmc_envelope_carries_phase_and_time() {
        let env = ThmcEnvelope::new(setting_phase(), 42.0);
        assert_eq!(env.kind(), MaterialPhaseKind::Setting);
        assert!((env.time - 42.0).abs() < f32::EPSILON);
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

        match solid_phase() {
            MaterialPhase::Solid(m) => {
                assert_eq!(m.displacement.dims(), [1, 2, 3]);
                assert_eq!(m.damage.dims(), [1, 2, 1]);
            }
            _ => panic!("expected Solid"),
        }
    }
}

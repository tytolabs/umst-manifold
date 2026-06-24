// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! W9 Phase A deprecated aliases — excluded from agnostic-on-fork / tier-1 lexicon scans.
//!
//! [`MixTensor`] and [`StatePoint`] are type aliases for [`MaterialCompositionTensor`];
//! prefer [`MaterialCompositionTensor`] in new code.

#![allow(non_snake_case)]

use burn::tensor::{backend::Backend, Tensor};

use crate::core::tensors::MaterialCompositionTensor;
use crate::gate::thermo_transition::{ThermodynamicGate, ThermodynamicState};
use crate::physics::solvers::thmc::ReactionExtentKinetics;

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
    unimplemented!("W9 Tier 2c: inject via domain cartridge")
}

#[deprecated(note = "cartridge-supplied gate wiring")]
pub fn thermodynamic_gate_from_iE_defaults() -> ThermodynamicGate {
    ThermodynamicGate::default()
}

#[deprecated(note = "cartridge-supplied snapshot")]
pub fn thermodynamic_state_from_iE(w_c: f64, alpha: f64, temp_k: f64) -> ThermodynamicState {
    ThermodynamicState::from_mix(w_c, alpha, temp_k)
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! W9 Phase A deprecated aliases — excluded from agnostic-on-fork / tier-1 lexicon scans.

use burn::tensor::{backend::Backend, Tensor};

use crate::core::tensors::StatePoint;
use crate::gate::http_manifest::HttpTransitionEvaluator;
use crate::gate::thermo_transition::{ThermodynamicGate, ThermodynamicState};
use crate::gate::transition_proposal::Q_REACTION_ENTHALPY_J_PER_KG;
use crate::physics::solvers::thmc::ReactionExtentKinetics;

/// Renamed to [`StatePoint`] in v2.0.0-rc1 (W9 agnostic-on-fork).
#[deprecated(note = "renamed to StatePoint")]
pub type MixTensor<B> = StatePoint<B>;

#[deprecated(note = "renamed to Q_REACTION_ENTHALPY_J_PER_KG")]
pub const Q_HYDRATION_J_PER_KG: f64 = Q_REACTION_ENTHALPY_J_PER_KG;

#[deprecated(note = "renamed to ReactionExtentKinetics")]
pub type ThmcHydrationKinetics = ReactionExtentKinetics;

#[deprecated(note = "renamed to REACTION_EXTENT_ACTIVATION_ENERGY_J_PER_MOL")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_ACTIVATION_ENERGY_J_PER_MOL as HYDRATION_ACTIVATION_ENERGY_J_PER_MOL;
#[deprecated(note = "renamed to REACTION_EXTENT_ARRHENIUS_PREFACTOR_S")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_ARRHENIUS_PREFACTOR_S as HYDRATION_ARRHENIUS_PREFACTOR_S;
#[deprecated(note = "renamed to REACTION_EXTENT_T_MIN_K")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_T_MIN_K as HYDRATION_T_MIN_K;
#[deprecated(note = "renamed to REACTION_EXTENT_T_BOOST_REF_K")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_T_BOOST_REF_K as HYDRATION_T_BOOST_REF_K;
#[deprecated(note = "renamed to REACTION_EXTENT_T_BOOST_PER_K")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_T_BOOST_PER_K as HYDRATION_T_BOOST_PER_K;
#[deprecated(note = "renamed to REACTION_EXTENT_EXOTHERMIC_K_PER_ALPHA_RATE")]
pub use crate::physics::solvers::thmc::REACTION_EXTENT_EXOTHERMIC_K_PER_ALPHA_RATE as HYDRATION_EXOTHERMIC_K_PER_ALPHA_RATE;

#[deprecated(note = "use HttpTransitionEvaluator::from_umst_manifest — injection-only")]
pub fn from_concrete_cartridge_defaults() -> HttpTransitionEvaluator {
    HttpTransitionEvaluator::from_umst_manifest(&crate::manifest::UmstManifest::default())
}

#[deprecated(note = "renamed to reaction_extent_from_age")]
pub fn hydration_degree(age_days: f64, temp_c: f64, supplementary_ratio: f64) -> f64 {
    crate::gate::http_manifest::reaction_extent_from_age(age_days, temp_c, supplementary_ratio)
}

#[deprecated(note = "renamed to transition_proposal")]
pub mod mix_proposal {
    pub use crate::gate::transition_proposal::*;
}

#[deprecated(note = "renamed to transition_proposal_admissible")]
pub fn mix_proposal_admissible(
    gate: &mut ThermodynamicGate,
    old_state: &ThermodynamicState,
    new_state: &ThermodynamicState,
    dt_s: f64,
) -> bool {
    gate.transition_proposal_admissible(old_state, new_state, dt_s)
}

#[deprecated(note = "renamed to reaction_extent_rate_tensor")]
#[cfg(feature = "thmc-coupled")]
pub fn full_hydration_alpha_rate_tensor<B: Backend<FloatElem = f32>>(
    k: &ReactionExtentKinetics,
    alpha: Tensor<B, 3>,
    temperature_for_alpha: Tensor<B, 3>,
    device: &B::Device,
) -> Tensor<B, 3> {
    crate::physics::solvers::thmc::reaction_extent_rate_tensor(
        k,
        alpha,
        temperature_for_alpha,
        device,
    )
}

#[deprecated(note = "renamed to ThmcImplicitEulerThermalReactionExtentResidual")]
#[cfg(feature = "thmc-coupled")]
pub use crate::physics::solvers::thmc_residual::ThmcImplicitEulerThermalReactionExtentResidual as ThmcImplicitEulerThermalHydrationResidual;

#[deprecated(note = "renamed to ThmcImplicitEulerThermalHumidityReactionExtentResidual")]
#[cfg(feature = "thmc-coupled")]
pub use crate::physics::solvers::thmc_residual::ThmcImplicitEulerThermalHumidityReactionExtentResidual as ThmcImplicitEulerThermalHumidityHydrationResidual;

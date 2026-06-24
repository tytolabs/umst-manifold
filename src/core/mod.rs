// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod apply_physics;
pub mod dec_typestate;
pub mod emergence;
pub mod iterate_until;
pub mod material_transition;
pub mod tensors;
pub mod traits;
pub mod umst_schema;

pub use apply_physics::apply_physics_to_umst;
pub use dec_typestate::{
    B1Incidence, DecTypestateError, ScalarChannel, ScalarChannelIdx,
};
pub use material_transition::{
    MaterialTransitionParams, ReactionExtentKineticsSpec, SubstrateMaterialParams,
};
pub use tensors::*;
pub use traits::*;
pub use umst_schema::*;

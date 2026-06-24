// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! RL / topology orchestration. Policy boundary: [`ppo::ManifoldGateway`] documents the
//! **IO barrier** (tensor reductions vs [`cbf::ThermodynamicCBF`] scalar sync); solver cores should
//! stay lazy and avoid host reads in inner loops where possible.

pub mod adjoint;
pub mod cbf;
pub mod constraint_loss;
#[cfg(feature = "epistemic-ppo")]
pub mod epistemic_mi;
pub mod formal;
pub mod info_gain;
pub mod liquid_ppo;
pub mod ppo;
pub mod topology;

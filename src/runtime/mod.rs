// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side runtime scaffolding (catalog lock, fingerprints).

pub mod catalog;
pub mod gate;
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
pub mod ppo_host;
#[cfg(feature = "photonics")]
pub mod photonics_host;

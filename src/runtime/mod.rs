// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Host-side runtime scaffolding (catalog lock, fingerprints).

pub mod atoms_f1_deepen;
pub mod atoms_scalar_bridge;
pub mod atoms_tensor_lift;
pub mod atoms_tensor_lift_adapter;
pub mod atoms_tensor_lift_ledger;
pub mod atoms_tensor_lift_ops;
pub mod atoms_tensor_lift_residual;
pub mod catalog;
pub mod gate;
#[cfg(feature = "nalgebra-tensor")]
pub mod nalgebra_algebra;
#[cfg(feature = "photonics")]
pub mod photonics_host;
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
pub mod ppo_host;

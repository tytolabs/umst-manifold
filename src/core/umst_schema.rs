// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Column indices for [`crate::core::tensors::UnifiedMaterialStateTensor::scalar_features`]
//! (`[N_active_nodes, F_scalars]`).
//!
//! ## Spatial units
//!
//! - [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`], when present, uses **SI
//!   metres** per axis (`[N, 3]`). This is independent of
//!   [`crate::core::tensors::UnifiedMaterialStateTensor::resolution_mm`], which remains a **millimetre**
//!   voxel/grid spacing hint for cartridges and visualization.
//!
//! These values are the **layout contract** for topology and THMC-style passes that read nodal
//! scalars from the manifold. Downstream cartridges (for example `umst-concrete-cartridge`
//! `compute_topology` / `thmc_state_from_umst`) must use these constants instead of duplicating
//! numeric literals.
//!
//! Channel `0` is reserved for material-specific bulk scalars (not yet fixed in the shared
//! contract); concrete workflows in-tree bind standard physics from [`SCALAR_HUMIDITY`] through
//! [`SCALAR_DAMAGE`], with optional [`SCALAR_FRACTURE_ENERGY_GC`] when `F_scalars > 5`.

/// Scalar column `0`: reserved / material-specific (define meaning per cartridge).
pub const SCALAR_CHANNEL0: usize = 0;

/// Relative humidity (or equivalent moisture scalar), column `1`.
pub const SCALAR_HUMIDITY: usize = 1;

/// Hydration degree / chemical progress \(\alpha\), column `2`.
pub const SCALAR_HYDRATION_ALPHA: usize = 2;

/// Nodal temperature (physical units are cartridge-defined; often °C on the concrete path), column `3`.
pub const SCALAR_TEMPERATURE: usize = 3;

/// Continuum / phase-field damage \(d \in \[0,1\]\), column `4`.
pub const SCALAR_DAMAGE: usize = 4;

/// Optional per-node fracture energy \(G_c\) [J/m²] for phase-field / cohesive models, column `5`.
///
/// Present only when `F_scalars > 5` (extends the baseline humidity→damage layout). Cartridges that
/// omit this column keep a **uniform** \(G_c\) from calibration / material closure.
pub const SCALAR_FRACTURE_ENERGY_GC: usize = 5;

/// Epistemic uncertainty σ (normalized 0–1), column `6` when `F_scalars > 6`.
///
/// Written by [`crate::ai::adjoint::AdjointNeuralODE::forward`] under **`epistemic-ppo`** from
/// policy-driven scalar deltas on [`policy_editable_mask`](crate::core::tensors::UnifiedMaterialStateTensor::policy_editable_mask).
pub const SCALAR_EPISTEMIC_UNCERTAINTY: usize = 6;

/// Nodal mechanical displacement **u** (SI metres), vector slot `0` in [`crate::core::tensors::UnifiedMaterialStateTensor::vector_features`]
/// (`[N, F_vectors, 3]`). When `F_vectors == 0`, THMC / mechanics adapters use zero displacement.
pub const VECTOR_MECHANICAL_DISPLACEMENT: usize = 0;

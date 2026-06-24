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
//! scalars from the manifold. Downstream domain cartridges bind these column indices in their
//! own crates; the kernel keeps only the shared layout contract.
//!
//! Channel `0` is reserved for material-specific bulk scalars (not yet fixed in the shared
//! contract); standard physics channels bind from [`SCALAR_HUMIDITY`] through
//! [`SCALAR_DAMAGE`], with optional [`SCALAR_FRACTURE_ENERGY_GC`] when `F_scalars > 5`.
//!
//! The pinned channel map is `artifacts/scalar_layout.lock.json` (Phase 1 §1B sidecar).
//! [`UMST_SCALAR_CHANNEL_COUNT`] and [`SCALAR_*`] indices are generated at build time via
//! `build.rs` → `umst-layout-codegen`; compile-time drift guard below panics on lock mismatch.

include!(concat!(env!("OUT_DIR"), "/scalar_layout_indices.rs"));

include!(concat!(env!("OUT_DIR"), "/scalar_layout_guard.rs"));

const _: [(); UMST_SCALAR_CHANNEL_COUNT] = [(); UMST_SCALAR_CHANNEL_COUNT_LOCK];

/// Nodal mechanical displacement **u** (SI metres), vector slot `0` in [`crate::core::tensors::UnifiedMaterialStateTensor::vector_features`]
/// (`[N, F_vectors, 3]`). When `F_vectors == 0`, THMC / mechanics adapters use zero displacement.
pub const VECTOR_MECHANICAL_DISPLACEMENT: usize = 0;

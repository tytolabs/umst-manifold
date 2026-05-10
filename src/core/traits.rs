// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use crate::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
use burn::tensor::{backend::Backend, Tensor};

/// The unified thermodynamic return type expected by the Orchestrator and the CBF.
/// Kept in Sparse Space [Batch, N_active_voxels] so the agent can compute topology gradients directly.
///
/// Consumed by [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`](crate::ai::ppo::ManifoldGateway::evaluate_topology_step)
/// (reward + CBF wiring): spatial terms use `free_energy`, `dissipation`, and `cost`; the per-batch
/// scalar reward optionally adds **ζ · mean(safety_margin)** when [`crate::ai::ppo::ManifoldGateway::zeta`]
/// is non-zero. With the **`information_density`** crate feature, the same scalar reward optionally adds
/// **η · mean(information_density)** when [`crate::ai::ppo::ManifoldGateway::eta`] is non-zero (defaults
/// preserve legacy behavior). Merged into UMST state via [`crate::core::apply_physics::apply_physics_to_umst`]
/// for damage and optional temperature.
pub struct PhysicalResult<B: Backend> {
    pub free_energy: Tensor<B, 2>,
    pub dissipation: Tensor<B, 2>,
    pub safety_margin: Tensor<B, 2>,
    pub cost: Tensor<B, 2>,
    pub damage: Tensor<B, 2>,
    pub temperature_delta: Option<Tensor<B, 2>>,
    /// Per-voxel information-density signal at shape `[Batch, N_active_voxels]`.
    ///
    /// Only present with the **`information_density`** feature. When present, it participates in the
    /// scalar reward only if [`crate::ai::ppo::ManifoldGateway::eta`] is non-zero (see struct-level docs).
    #[cfg(feature = "information_density")]
    pub information_density: Tensor<B, 2>,
}

/// The core interface that any Material Engine (Concrete, Supercap, Steel) must implement.
pub trait IScienceCartridge<B: Backend> {
    /// Standard homogeneous forward pass (0D/1D). Evaluates the bulk material.
    fn compute_all(&self, mix: &MixTensor<B>) -> PhysicalResult<B>;

    /// Multi-agent heterogeneous topology pass.
    /// The cartridge computes physics using the Cellular Sheaf topology (Discrete Exterior Calculus).
    /// Shape of returned tensors: [Batch, N_active_voxels]
    fn compute_topology(&self, manifold: &UnifiedMaterialStateTensor<B>) -> PhysicalResult<B>;
}

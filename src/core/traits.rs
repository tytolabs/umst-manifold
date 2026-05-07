// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

use crate::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
use burn::tensor::{backend::Backend, Tensor};

/// The unified thermodynamic return type expected by the Orchestrator and the CBF.
/// Kept in Sparse Space [Batch, N_active_voxels] so the agent can compute topology gradients directly.
pub struct PhysicalResult<B: Backend> {
    pub free_energy: Tensor<B, 2>,
    pub dissipation: Tensor<B, 2>,
    pub safety_margin: Tensor<B, 2>,
    pub cost: Tensor<B, 2>,
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

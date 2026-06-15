// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cartridge traits and thermodynamic summaries (`fp-categorical-v04` / `fp-v04-traits-category`).
//!
//! # Categorical vocabulary (design sketch)
//!
//! - **Objects:** [`crate::core::tensors::StatePoint`] (homogeneous bulk) and
//!   [`crate::core::tensors::UnifiedMaterialStateTensor`] (topology-carrying UMST) are the primary
//!   *state carriers* solvers and cartridges reason about.
//! - **Morphisms:** [`IScienceCartridge`] is the stable **material-law port**—two evaluation heads
//!   (`compute_all`, `compute_topology`) from those objects into [`PhysicalResult`]. Orchestrated
//!   graph stepping lives in [`crate::physics::orchestration`] and [`crate::physics::solvers`], not
//!   in this trait (cartridge stays a functor *into* thermodynamic summaries).
//! - **Second law at the interface:** [`PhysicalResult`] exposes `free_energy`, `dissipation`, and
//!   related sparse fields so merge, CBF, and RL paths can audit **dissipative consistency** as a
//!   policy invariant; concrete constitutive closures must populate those tensors consistently with
//!   their numerical schemes.
//!
//! Longer note (objects / solvers / composition table): `docs/Category-of-Material-Updates.md`.

use crate::core::tensors::{StatePoint, UnifiedMaterialStateTensor};
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

/// Material-law port: bulk and topology evaluation into [`PhysicalResult`] (no THMC stepping here).
pub trait IScienceCartridge<B: Backend> {
    /// Standard homogeneous forward pass (0D/1D). Evaluates the bulk material.
    fn compute_all(&self, mix: &StatePoint<B>) -> PhysicalResult<B>;

    /// Multi-agent heterogeneous topology pass.
    /// The cartridge computes physics using the Cellular Sheaf topology (Discrete Exterior Calculus).
    /// Shape of returned tensors: [Batch, N_active_voxels]
    fn compute_topology(&self, manifold: &UnifiedMaterialStateTensor<B>) -> PhysicalResult<B>;
}

/// Universal gate port (Phase B) — independent of spatial physics.
pub trait GateCartridge {
    fn provides_spatial_physics(&self) -> bool {
        true
    }
}

/// Spatial physics port (Phase B subtyping marker).
pub trait SpatialCartridge<B: Backend>: IScienceCartridge<B> {}

/// Cartridge-supplied transition closure parameters (W9 Tier 2c bridge).
///
/// Default implementations preserve legacy OPC hydration literals; concrete cartridges override
/// in a follow-up pin. Kernel transition math consumes these via injection, not hard-coded cement.
pub trait MaterialTransitionParams {
    /// Specific heat of reaction progress (J/kg), default OPC hydration enthalpy scale.
    fn hydration_heat_j_per_kg(&self) -> f64 {
        450.0
    }

    /// Intrinsic gel strength scale (MPa) for Powers-style monotonicity checks.
    fn default_intrinsic_strength_mpa(&self) -> f64 {
        240.0
    }
}

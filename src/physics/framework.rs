// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cross-cutting physics architecture traits (Burn-safe: no trait objects over kernels).

use burn::tensor::backend::Backend;

/// Zero-sized solver façade types (`VectorMechanicsSolver`, future `ThmcCoupledStep`, …).
pub trait PhysicsSolverZst: Send + Sync + 'static {}

/// Marker for backends used in f32 equilibrium / transport stacks.
pub trait PhysicsBackend: Backend<FloatElem = f32> {}

impl<B: Backend<FloatElem = f32>> PhysicsBackend for B {}

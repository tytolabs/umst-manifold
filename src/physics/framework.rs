// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cross-cutting physics architecture traits (Burn-safe: no trait objects over kernels).
//!
//! # Solver types as morphisms (sketch)
//!
//! [`PhysicsSolverZst`] marks **which solver family** is in play (ZST façade). Actual stepping and
//! composition happen in concrete solver modules and [`crate::physics::orchestration`]; the marker
//! trait keeps dispatch monomorphized while documenting the categorical “typed morphism” boundary.
//!
//! See `docs/Category-of-Material-Updates.md` (`maos-fp-categorical-v04`).

use burn::tensor::backend::Backend;

/// Zero-sized façade for a solver **family** (`VectorMechanicsSolver`, …): identity-like marker, not a `dyn` kernel.
pub trait PhysicsSolverZst: Send + Sync + 'static {}

/// Marker for backends used in f32 equilibrium / transport stacks.
pub trait PhysicsBackend: Backend<FloatElem = f32> {}

impl<B: Backend<FloatElem = f32>> PhysicsBackend for B {}

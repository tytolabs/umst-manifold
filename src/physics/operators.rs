// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure graph / DEC operators: no clocks, no material cartridges, no Krylov loops.
//!
//! Re-exports live modules for the `physics::operators::*` path; legacy `physics::laplacian`
//! and `physics::dec_operators` remain for stable imports.

pub use super::dec_operators::*;
pub use super::dec_primal::*;
pub use super::laplacian::*;

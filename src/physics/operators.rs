// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Pure graph / DEC operators: no clocks, no material cartridges, no Krylov loops.
//!
//! ## Functorial / natural reading (primal vs dual discrete spaces)
//!
//! Fix an oriented graph \(G=(V,E)\). Nodal tensors live on **primal 0-cells** \(C^0\); edge tensors
//! on **primal 1-cells** \(C^1\). The incidence gather/scatter used by [`super::topology::EdgeTopology`]
//! is the concrete Burn realisation of the chain maps \(B_1^\top: C^1\to C^0\) (weak divergence) and
//! \(d_0: C^0\to C^1\) (coboundary / edge increment). Changing only the **cochain values** while
//! holding `edges_b1` fixed is a linear natural transformation between finite-dimensional spaces of
//! sections; metric/Hodge weights (when present in callers) post-compose on \(C^k\) before duality
//! pairings. [`super::dec_primal::primal_d1_edge_flux_to_faces`] and its transpose extend the same
//! pattern to 2-cells via `faces_b2` column ranges.
//!
//! Re-exports live modules for the `physics::operators::*` path; legacy `physics::laplacian`
//! and `physics::dec_operators` remain for stable imports.

pub use super::dec_operators::*;
pub use super::dec_primal::*;
pub use super::laplacian::*;

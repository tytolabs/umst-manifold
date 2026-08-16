// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Provenance (I6 CDD) — Vendored / Native / Theorem-bound markers for SDF carriers.

/// §0.5 NED: provenance of an analytic SDF or voxel witness (MEMORY-ARC-PLAN Q4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provenance {
    /// Vendored mathematical kernel (SPDX on file, see `primitives` / CDD comments).
    Vendored,
    /// Native M-Arc reference implementation in this crate.
    Native,
    /// Grounds to `umst-formal` (SDFGate.hs, Gate.lean, Coq, Agda).
    TheoremBound,
}

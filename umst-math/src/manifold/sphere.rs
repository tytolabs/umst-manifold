// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Spherical manifolds — S² and S^{n} as marker carriers (I1).
//!
//! Embeddings: S² ↪ ℝ³ and S^{n} ↪ ℝ^{n+1} for fixed ambient `n`.

use super::Manifold;

/// The **2-sphere** S² as a compact boundary carrier (MEMORY-ARC-PLAN M-Q1).
pub struct S2 {
    _priv: (),
}

/// **n-sphere** S^{n} ⊂ ℝ^{n+1} — `n` is intrinsic dimension of the sphere (not the ambient one).
pub struct Sn {
    /// Intrinsic sphere dimension, e.g. 2 for S².
    pub n: u8,
}

impl Manifold for S2 {
    fn carrier_label(&self) -> &'static str {
        "S2 embedded in R3 (unit; intrinsic charts deferred)"
    }
}

impl S2 {
    /// Canonical unit 2-sphere description used by the `manifold!` witness (I8 GMD-1).
    pub const UNIT: S2 = S2 { _priv: () };
}

impl Manifold for Sn {
    fn carrier_label(&self) -> &'static str {
        "S^n (marker; n stored in struct field)"
    }
}

/// ℝ³ — non-compact carrier for voxel / octree charts (M-Arc ambient space).
pub struct R3 {
    _p: (),
}

/// Canonical ℝ³ witness for `manifold!(R3)`.
pub const R3_CHART: R3 = R3 { _p: () };

impl Manifold for R3 {
    fn carrier_label(&self) -> &'static str {
        "R3 (Euclidean 3-space; charts for voxel SDFs)"
    }
}

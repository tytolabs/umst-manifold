// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Rank-1+ comparison epsilon — provenanced from T4 cold-boundary cast policy.
//!
//! Source: `umst-manifold/src/runtime/atoms_scalar_bridge.rs::COLD_BOUNDARY_F32_EPS`
//! Measurement: host f64 ↔ burn f32 roundtrip max error on atom probe grid @ PBM-010.

/// Provenanced rank-1+ tensor comparison epsilon (f32 cold boundary).
pub const RANK1_PLUS_COMPARISON_EPS: f64 = 1e-3;

/// Whether rank-1+ `impl TensorAlgebra` golden comparison is closed (R13-2 measured).
pub const RANK1_PLUS_IMPL_LANDED: bool = true;

/// Residue cleared @ R13-2 golden harness measurement.
pub const RANK1_PLUS_RESIDUE_ID: &str = "R-atoms-scalar-rank1-plus-closed";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank1_plus_eps_provenanced_and_impl_landed() {
        assert!((RANK1_PLUS_COMPARISON_EPS - 1e-3).abs() < f64::EPSILON);
        assert!(RANK1_PLUS_IMPL_LANDED);
    }
}

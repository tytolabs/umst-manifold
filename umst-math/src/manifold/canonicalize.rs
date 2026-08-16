// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Voxelized canonical SDF + deterministic FNV hash (GMD-2, I3).

use super::error::ManifoldError;
use super::sdf::Sdf;

/// 8-byte FNV-1a-64 of `data` (deterministic across runs).
pub fn fnv1a_64(data: &[u8]) -> [u8; 8] {
    const FNV_OFF: u64 = 0xcbf29ce484222325;
    const FNV_PR: u64 = 0x100000001b3;
    let mut h = FNV_OFF;
    for b in data {
        h ^= u64::from(*b);
        h = h.wrapping_mul(FNV_PR);
    }
    h.to_le_bytes()
}

// --- Fixed-world AABB (§ M-0): unit cube in logical space, affine invariants in tests. ---

/// I3: pure; samples `Sdf` on a uniform `2^bits` per-axis grid in `[-1,1]³` cell centers.
/// Returns the **8**-byte hash of the raw **f64** voxel little-endian block (GMD-2: determinism).
pub fn canonicalize_voxelize(
    sdf: &impl Sdf,
    bits: u8,
) -> Result<([u8; 8], Vec<u8>), ManifoldError> {
    if !(1..=10).contains(&bits) {
        return Err(ManifoldError::InvalidResolution(bits));
    }
    let n = 1u32 << bits;
    let mut v = Vec::new();
    let scale = 2.0f64 / f64::from(n);
    for iz in 0u32..n {
        for iy in 0u32..n {
            for ix in 0u32..n {
                let cx = (f64::from(ix) + 0.5) * scale - 1.0;
                let cy = (f64::from(iy) + 0.5) * scale - 1.0;
                let cz = (f64::from(iz) + 0.5) * scale - 1.0;
                v.extend_from_slice(&sdf.dist([cx, cy, cz]).to_le_bytes());
            }
        }
    }
    let h = fnv1a_64(&v);
    Ok((h, v))
}

/// Refinement as **byte-prefix**: H(r+1) = h_r (8) || t (8) — t carries fine-scale witness (B-Arc can formalise).
#[inline]
pub fn stack_refinement_h8(h_r: [u8; 8], tail8: [u8; 8]) -> [u8; 16] {
    let mut o = [0u8; 16];
    o[..8].copy_from_slice(&h_r);
    o[8..].copy_from_slice(&tail8);
    o
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Voxelized canonical SDF + deterministic FNV hash (GMD-2, I3).
//!
//! [`canonicalize_voxelize`] is the dense oracle. [`canonicalize_emax_sample`] is an
//! Emax far-band truncation on the same `[-1,1]³` lattice beside dense voxelize: keep all
//! near-surface samples, cap far samples at [`CANON_SAMPLE_EMAX`].

use super::error::ManifoldError;
use super::sdf::Sdf;

/// Emax-style truncation cap for [`canonicalize_emax_sample`] far-band samples.
pub const CANON_SAMPLE_EMAX: usize = 32;

/// Lattice sample-codec posture field.
pub const CANON_SAMPLE_PHYSICS_GREEN: bool = false;

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

/// Emax-style canonical sample on the same `2^bits` cell-center lattice as
/// [`canonicalize_voxelize`].
///
/// Near-surface band: keep **all** samples with `|d| ≤ 2·cell_diag`. Far band: keep at
/// most [`CANON_SAMPLE_EMAX`] samples with smallest `|d|` (farthest-truncated — drop deep
/// interior / far exterior beyond the cap). Payload is kept f64 LE distances in grid
/// walk order; hash is FNV-1a-64. Emax far-band truncation on the `[-1,1]³` lattice beside
/// dense [`canonicalize_voxelize`].
pub fn canonicalize_emax_sample(
    sdf: &impl Sdf,
    bits: u8,
) -> Result<([u8; 8], Vec<u8>), ManifoldError> {
    if !(1..=10).contains(&bits) {
        return Err(ManifoldError::InvalidResolution(bits));
    }
    let n = 1u32 << bits;
    let scale = 2.0f64 / f64::from(n);
    let cell_diag = scale * 3.0_f64.sqrt();
    let near_thresh = 2.0 * cell_diag;

    #[derive(Clone, Copy)]
    struct Sample {
        ix: u32,
        iy: u32,
        iz: u32,
        dist: f64,
    }

    let mut near = Vec::new();
    let mut far = Vec::new();
    for iz in 0u32..n {
        for iy in 0u32..n {
            for ix in 0u32..n {
                let cx = (f64::from(ix) + 0.5) * scale - 1.0;
                let cy = (f64::from(iy) + 0.5) * scale - 1.0;
                let cz = (f64::from(iz) + 0.5) * scale - 1.0;
                let dist = sdf.dist([cx, cy, cz]);
                let s = Sample { ix, iy, iz, dist };
                if dist.abs() <= near_thresh {
                    near.push(s);
                } else {
                    far.push(s);
                }
            }
        }
    }

    // Farthest-truncated: keep the `CANON_SAMPLE_EMAX` far samples closest to the surface.
    far.sort_by(|a, b| {
        a.dist
            .abs()
            .partial_cmp(&b.dist.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.iz.cmp(&b.iz))
            .then_with(|| a.iy.cmp(&b.iy))
            .then_with(|| a.ix.cmp(&b.ix))
    });
    far.truncate(CANON_SAMPLE_EMAX);

    let mut kept = near;
    kept.extend(far);
    kept.sort_by(|a, b| {
        a.iz
            .cmp(&b.iz)
            .then_with(|| a.iy.cmp(&b.iy))
            .then_with(|| a.ix.cmp(&b.ix))
    });

    let mut v = Vec::with_capacity(kept.len().saturating_mul(8));
    for s in kept {
        v.extend_from_slice(&s.dist.to_le_bytes());
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
#[cfg(test)]
mod tests {
    use super::{
        canonicalize_emax_sample, canonicalize_voxelize, fnv1a_64, CANON_SAMPLE_EMAX,
        CANON_SAMPLE_PHYSICS_GREEN,
    };
    use crate::manifold::sdf::SphereSdf;

    const _: () = assert!(!CANON_SAMPLE_PHYSICS_GREEN);

    #[test]
    fn emax_sample_hash_deterministic() {
        let s = SphereSdf {
            c: [0.0, 0.0, 0.0],
            r: 0.5,
        };
        let (h1, p1) = canonicalize_emax_sample(&s, 4).expect("emax");
        let (h2, p2) = canonicalize_emax_sample(&s, 4).expect("emax");
        assert_eq!(h1, h2);
        assert_eq!(p1, p2);
        assert_ne!(h1, [0u8; 8]);
    }

    #[test]
    fn emax_payload_shorter_than_voxel_at_bits6() {
        let s = SphereSdf {
            c: [0.0, 0.0, 0.0],
            r: 0.5,
        };
        let (_, vox) = canonicalize_voxelize(&s, 6).expect("vox");
        let (_, emax) = canonicalize_emax_sample(&s, 6).expect("emax");
        assert!(
            emax.len() < vox.len(),
            "emax {} bytes should be < voxel {} bytes",
            emax.len(),
            vox.len()
        );
        assert!(emax.len() <= vox.len());
        assert_eq!(CANON_SAMPLE_EMAX, 32);
    }

    #[test]
    fn voxelize_still_works_beside_emax() {
        let s = SphereSdf {
            c: [0.0, 0.0, 0.0],
            r: 0.5,
        };
        let (h1, v1) = canonicalize_voxelize(&s, 3).expect("vox");
        let (h2, v2) = canonicalize_voxelize(&s, 3).expect("vox");
        assert_eq!(v1.len(), 8 * (1usize << 9));
        assert_eq!(h1, fnv1a_64(&v1));
        assert_eq!(h1, h2);
        assert_eq!(v1, v2);
        canonicalize_emax_sample(&s, 3).expect("emax coexists");
    }

    #[test]
    fn emax_canon_not_physics_green() {
        assert!(!CANON_SAMPLE_PHYSICS_GREEN);
    }
}


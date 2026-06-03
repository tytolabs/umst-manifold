//! Typed errors for the manifold module (I5 — no `unwrap` in non-test code).

use thiserror::Error;

/// Manifold / SDF / grid failure modes.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum ManifoldError {
    /// Hilbert / octree index out of the declared range.
    #[error("index out of range: {0} (max {1})")]
    OutOfRange(u32, u32),
    /// Resolution bits outside policy bounds (see `REGISTRY`: floor..=ceiling).
    #[error("invalid resolution: bits {0}")]
    InvalidResolution(u8),
    /// Octree or DAG structural violation.
    #[error("octree: invalid parent/child link")]
    OctreeLayout,
    /// Voxel or affine pipeline failed a conservative bound.
    #[error("canonicalize: {0}")]
    Canonicalize(&'static str),
}

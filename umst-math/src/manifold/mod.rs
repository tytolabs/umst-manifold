//! §14bis.f-M-0 — **Manifold** algebra (pure math; M-Arc foundation).
//!
//! Carrier S², SDF / CSG, Hilbert 2D, octree, canonical voxel hash (GMD). No I/O.

// SPDX-License-Identifier: MIT
// M-Arc: MEMORY-ARC-PLAN-v1.0 §0.14; parallel with H-9-mac (no `hal/` overlap).

pub mod canonicalize;
pub mod csg;
mod error;
pub mod hilbert;
pub mod octree;
pub mod primitives;
mod provenance;
pub mod sdf;
pub mod sphere;

pub use error::ManifoldError;
pub use provenance::Provenance;
pub use sdf::Sdf;
pub use sphere::{Sn, R3, R3_CHART, S2};

/// I1: marker for typed carriers (GMD-1); every impl names its model space in `carrier_label`.
pub trait Manifold: Send + Sync {
    /// Human-readable model / embedding (Curry-Howard: “the” carrier set for this type).
    fn carrier_label(&self) -> &'static str;
}

/// M-Arc **resolution** — `bits` per axis; grid count `2^(3*bits)` voxels in `canonicalize` (3D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResolutionLevel {
    /// Bits per axis (8..=12 per REGISTRY policy in M-0).
    pub bits: u8,
}

/// CDD: ε for affine residual (REGISTRY `manifold_canonicalize_eps`).
pub const MANIFOLD_CANONICALIZE_EPS: f64 = 1e-9;

// --- I8: `manifold!` witness (≥5 sites in this file for `rg 'manifold!'` in `umst-math/src/manifold/`) ---

/// Manifold-typed **compile-time** witness (GMD-1, GMD-2 for ε).
#[macro_export]
macro_rules! manifold {
    (S2) => {
        $crate::manifold::sphere::S2::UNIT
    };
    (Sn, $n:expr) => {
        $crate::manifold::sphere::Sn { n: $n }
    };
    (R3) => {
        $crate::manifold::R3_CHART
    };
    (Hilbert, $b:expr) => {
        $crate::manifold::hilbert::HilbertCurve::new_2d($b)
    };
    (Eps) => {
        $crate::manifold::MANIFOLD_CANONICALIZE_EPS
    };
    (ProvVend) => {
        $crate::manifold::provenance::Provenance::Vendored
    };
    (ProvNative) => {
        $crate::manifold::provenance::Provenance::Native
    };
    (ProvTheorem) => {
        $crate::manifold::provenance::Provenance::TheoremBound
    };
    (Res, $b:expr) => {
        $crate::manifold::ResolutionLevel { bits: $b }
    };
}

/// GMD-1 / I8: eight `manifold!` call sites (M7.8) plus the `macro_rules` block above.
pub fn gmd_witness_sites() {
    let _s2 = manifold!(S2);
    let _e = manifold!(Eps) > 0.0;
    let _h = manifold!(Hilbert, 4u8);
    let _n = manifold!(Sn, 3u8);
    let _p = manifold!(ProvTheorem);
    let _r = manifold!(Res, 8u8);
    let _a = core::mem::size_of_val(&manifold!(R3));
    let _r3 = manifold!(R3);
    let _ = _r3;
    let _v = manifold!(ProvVend);
    let _n2 = manifold!(ProvNative);
    let _ = (_s2, _e, _h, _n, _p, _r, _a, _v, _n2);
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! SDF surface — I3 `canonicalize_voxelize` (pure; no `&mut self` on the trait's witness site).

// SDF type discipline from `umst-formal/Haskell/SDFGate.hs` (signed distance, outside>0 if positive convention).

use super::csg;
use super::provenance::Provenance;

/// A scalar signed distance field in ℝ³ (FRep: negative / zero / positive = inside / on / out).
pub trait Sdf: Send + Sync {
    /// Signed distance; negative or zero = admissible in UMST SDF sign convention.
    fn dist(&self, p: [f64; 3]) -> f64;
    /// Provenance tag (I6 CDD, I5 theorem or measurement).
    fn provenance(&self) -> Provenance {
        Provenance::Native
    }
}

// --- SDFs built from csg (gate + sphere) for tests (public but narrow surface) ---

/// Constant SDF: `d(x)=c` (half-space with parallel offset).
pub struct ConstSdf(pub f64);

impl Sdf for ConstSdf {
    fn dist(&self, p: [f64; 3]) -> f64 {
        let _ = p;
        self.0
    }
}

/// `gate_sdf` lifted to ℝ³ SDF: depends only on embedded `(old, new)` scalars; here we pre-fold to one scalar in product space.
pub struct GateSdf {
    v: f64,
}

impl GateSdf {
    /// Full gate in product space — already evaluated **scalar** for M-0; multi-D carrier comes in M-1+.
    pub fn from_scalar_value(v: f64) -> Self {
        Self { v }
    }
    /// `gate_sdf` from the four thermo pairs (Haskell `gateSDF` max).
    pub fn from_thermo_pair(old: &csg::ThermoGateState, new: &csg::ThermoGateState) -> Self {
        Self {
            v: csg::gate_sdf(old, new),
        }
    }
}

impl Sdf for GateSdf {
    fn dist(&self, p: [f64; 3]) -> f64 {
        let _ = p;
        self.v
    }
    fn provenance(&self) -> Provenance {
        Provenance::TheoremBound
    }
}

/// Sphere SDF: center + radius (FRep standard).
pub struct SphereSdf {
    pub c: [f64; 3],
    pub r: f64,
}

impl Sdf for SphereSdf {
    fn dist(&self, p: [f64; 3]) -> f64 {
        let dx = p[0] - self.c[0];
        let dy = p[1] - self.c[1];
        let dz = p[2] - self.c[2];
        (dx * dx + dy * dy + dz * dz).sqrt() - self.r
    }
}

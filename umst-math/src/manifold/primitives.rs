//! Native **FRep SDF** primitives and small **metric** helpers for M-Arc.
//!
//! **SDF kernel:** `umst-prototype-2a/.../tensors/geometry.rs` is a **SAV/curvature consumer** of
//! pre-digitised SDF *bytes* (no SDF distance functions) — it is **not** a vendor source for M-0.
//! Primitives here follow the algebra in `umst-formal/Haskell/SDFGate.hs`: **CSG intersection =
//! `max`**, **union = `min`** in the inside-negative convention, **zero level set = boundary**,
//! **half-spaces** as 1D slab / offset SDFs (see `csg` module for the gate’s four half-spaces on
//! state-pair space).
// SPDX-License-Identifier: MIT
//
// Algebra ref (pinned for CDD / traceability; file path in `umst-formal` monorepo):
// `Haskell/SDFGate.hs` @ a9096507df5c1d8053641f14b36b712c1ecefd7b

use std::f64::consts::PI;

// Provenance: native; **M-3 alignment** (not SDFGate): `GeometryData` in prototype uses these
// as spherical **stand-ins** for volume/SAV; same numeric identities will feed Pareto retention.
/// Volume and surface of a full sphere of radius `r` (SAV and packing inputs; MEMORY-ARC M-3).
pub fn volume_and_surface_of_sphere(r: f64) -> (f64, f64) {
    let r = r.max(0.0);
    let vol = (4.0 / 3.0) * PI * r * r * r;
    let sa = 4.0 * PI * r * r;
    (vol, sa)
}

// Provenance: native; M-3 AABB in-radius proxy (not an SDF kernel).
/// Half-extent of the largest in-sphere in an AABB of side lengths `(w,h,d)`.
pub fn bounding_inradius_from_aabb(width: f64, height: f64, depth: f64) -> f64 {
    let w = width.max(0.0);
    let h = height.max(0.0);
    let d = depth.max(0.0);
    w.min(h).min(d) / 2.0
}

// Provenance: native impl following SDFGate.hs L14–L19 (FRep sign: interior ≤0, zero set = surface);
// algebra ref umst-formal/Haskell/SDFGate.hs@a9096507df5c1d8053641f14b36b712c1ecefd7b
// Same convention as a symmetric slab around the origin, c.f. massConservation as |Δ|−δ (L82–L85).
/// Unit-sphere SDF: signed distance, **inside negative / outside positive** on the radial axis.
pub fn sphere_sdf(p: [f64; 3], center: [f64; 3], r: f64) -> f64 {
    let dx = p[0] - center[0];
    let dy = p[1] - center[1];
    let dz = p[2] - center[2];
    (dx * dx + dy * dy + dz * dz).sqrt() - r
}

// Provenance: native; CSG as intersection of 6 **half-spaces** (L18–L19, L128–L138: intersection =
// `max` of member SDFs on the product; here axis-aligned `box` in ℝ³ = standard FRep min of face slabs).
// algebra ref umst-formal/Haskell/SDFGate.hs@a9096507df5c1d8053641f14b36b712c1ecefd7b
// Implementation pattern: Inigo Quilez axis-aligned box (exact SDF; crease-smooth is optional via `csg::smooth_min`).
/// Axis-aligned **box** SDF (half-extents) centered at `c`.
pub fn box_sdf(p: [f64; 3], c: [f64; 3], half: [f64; 3]) -> f64 {
    let o = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
    let q = [
        o[0].abs() - half[0],
        o[1].abs() - half[1],
        o[2].abs() - half[2],
    ];
    let ax = q[0].max(0.0);
    let ay = q[1].max(0.0);
    let az = q[2].max(0.0);
    let term = (ax * ax + ay * ay + az * az).sqrt();
    term + q[0].max(q[1].max(q[2])).min(0.0)
}

// Provenance: native impl following SDFGate.hs L14–L16, L62–L68; algebra ref
// umst-formal/Haskell/SDFGate.hs@a9096507df5c1d8053641f14b36b712c1ecefd7b
// Degenerate 0D body; distance to point is the **positive** unsigned radius (outside convention on exterior).
/// Point carrier SDF: Euclidean distance to `c`.
pub fn point_sdf(p: [f64; 3], c: [f64; 3]) -> f64 {
    let d = (p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2) + (p[2] - c[2]).powi(2);
    d.sqrt()
}

// Provenance: native; CSG Minkowski sum of **segment** ∩ **ball** (1D + radius); composes with `csg` max/min
// (L176–L178) like `intersectSDF` / `gateSDF` composition discipline.
// algebra ref umst-formal/Haskell/SDFGate.hs@a9096507df5c1d8053641f14b36b712c1ecefd7b
/// Capsule between `a` and `b` with radius `r`.
pub fn capsule_sdf(p: [f64; 3], a: [f64; 3], b: [f64; 3], r: f64) -> f64 {
    let pa = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
    let ba = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len2 = ba[0] * ba[0] + ba[1] * ba[1] + ba[2] * ba[2];
    if len2 < 1e-30 {
        return (pa[0] * pa[0] + pa[1] * pa[1] + pa[2] * pa[2]).sqrt() - r;
    }
    let t = (pa[0] * ba[0] + pa[1] * ba[1] + pa[2] * ba[2]) / len2;
    let t = t.clamp(0.0, 1.0);
    let h = [pa[0] - t * ba[0], pa[1] - t * ba[1], pa[2] - t * ba[2]];
    (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt() - r
}

// Provenance: native; supporting **line** carrier (infinite) as 1D Eikonal-like spine (CSG with radius via offset is M-1+);
// sign convention L14–L16.
// algebra ref umst-formal/Haskell/SDFGate.hs@a9096507df5c1d8053641f14b36b712c1ecefd7b
/// Distance to the infinite line through `a` with direction `d` (un-normalised; scaled internally).
pub fn line_sdf(p: [f64; 3], a: [f64; 3], d: [f64; 3]) -> f64 {
    let dlen = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if dlen < 1e-20 {
        return point_sdf(p, a);
    }
    let u = [d[0] / dlen, d[1] / dlen, d[2] / dlen];
    let ap = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
    let t = ap[0] * u[0] + ap[1] * u[1] + ap[2] * u[2];
    let q = [
        a[0] + t * u[0] - p[0],
        a[1] + t * u[1] - p[1],
        a[2] + t * u[2] - p[2],
    ];
    (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt()
}

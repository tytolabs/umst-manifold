// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Geometry as an action hash: analytic action trees, their SDF, and their quotient keys (§14bis.f-M-4).
//!
//! An [`Action`] is an analytic solid (sphere, box, capped cylinder, constant or gate field) or a hard CSG
//! composition of actions. Two keys identify it:
//! - [`action_sdf_canonicalize`] voxelises its signed-distance field at `bits` per axis and hashes the
//!   voxels ([`super::canonicalize::sdf_shape_canonicalize`]): two actions with the same voxel shape share it;
//! - [`action_geometry_quotient_key`] hashes the analytic tree itself and does not depend on `bits`.
//!
//! The two domain separators below are wire bytes: recorded ledgers key on them, so they stay fixed.

use super::canonicalize::sdf_shape_canonicalize;
use super::sdf::{ConstSdf, GateSdf, SphereSdf};
use super::{ManifoldError, Sdf};

/// Domain separator of [`action_geometry_quotient_key`] (wire bytes, fixed since v1).
pub const GEOMETRY_QUOTIENT_DOMAIN: &[u8] = b"egoff.v1.action_geometry_quotient_key";

/// Domain separator of [`action_quotient_id`] (wire bytes, fixed since v1).
pub const SHAPE_QUOTIENT_DOMAIN: &[u8] = b"egoff.v1.action_quotient_id_from_shape";

/// Blake3-sized identifier for a voxelised canonical SDF at a given resolution.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ActionShapeId(pub [u8; 32]);

/// Hard CSG combinator for a [`Action::Compose`] node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ComposeOp {
    /// Minimum of the children's distances.
    Union = 0,
    /// Maximum of the children's distances.
    Intersection = 1,
    /// The first child with every later child removed: `max(d0, -d1, -d2, …)`.
    Difference = 2,
}

impl ComposeOp {
    #[inline]
    fn wire_tag(self) -> u8 {
        self as u8
    }
}

/// Analytic action geometry (negative inside, positive outside).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    /// Ball of `radius` about `center`.
    Sphere {
        /// Centre in R³.
        center: [f64; 3],
        /// Radius.
        radius: f64,
    },
    /// Constant distance field.
    ConstDist(f64),
    /// Gate scalar lifted to a field ([`GateSdf::from_scalar_value`]).
    GateScalar(f64),
    /// Axis-aligned box.
    Box {
        /// Centre in R³.
        center: [f64; 3],
        /// Half extent along each axis.
        half_extents: [f64; 3],
    },
    /// Capped cylinder from `base` along `axis` (normalised; a zero axis reads as +y).
    Cylinder {
        /// Centre of the base cap.
        base: [f64; 3],
        /// Direction of the cylinder axis.
        axis: [f64; 3],
        /// Radius.
        radius: f64,
        /// Length along the axis.
        height: f64,
    },
    /// Hard CSG over the children; an empty list is the empty solid (+∞ everywhere).
    Compose {
        /// Combinator.
        op: ComposeOp,
        /// Operands, in order (order matters for `Difference`).
        children: Vec<Action>,
    },
}

impl Action {
    #[inline]
    fn dist(&self, p: [f64; 3]) -> f64 {
        match self {
            Action::Sphere { center, radius } => SphereSdf {
                c: *center,
                r: *radius,
            }
            .dist(p),
            Action::ConstDist(c) => ConstSdf(*c).dist(p),
            Action::GateScalar(v) => GateSdf::from_scalar_value(*v).dist(p),
            Action::Box {
                center,
                half_extents,
            } => box_sdf(*center, *half_extents, p),
            Action::Cylinder {
                base,
                axis,
                radius,
                height,
            } => cylinder_sdf(*base, *axis, *radius, *height, p),
            Action::Compose { op, children } => compose_sdf(*op, children, p),
        }
    }
}

/// Signed distance to an axis-aligned box.
#[inline]
fn box_sdf(center: [f64; 3], half_extents: [f64; 3], p: [f64; 3]) -> f64 {
    let q = [
        (p[0] - center[0]).abs() - half_extents[0],
        (p[1] - center[1]).abs() - half_extents[1],
        (p[2] - center[2]).abs() - half_extents[2],
    ];
    let outside = (q[0].max(0.0).powi(2) + q[1].max(0.0).powi(2) + q[2].max(0.0).powi(2)).sqrt();
    let inside = q[0].max(q[1]).max(q[2]).min(0.0);
    outside + inside
}

/// Signed distance to a capped cylinder along `axis` from `base`.
#[inline]
fn cylinder_sdf(base: [f64; 3], axis: [f64; 3], radius: f64, height: f64, p: [f64; 3]) -> f64 {
    let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let a = if len > 1e-15 {
        [axis[0] / len, axis[1] / len, axis[2] / len]
    } else {
        [0.0, 1.0, 0.0]
    };
    let pa = [p[0] - base[0], p[1] - base[1], p[2] - base[2]];
    let h = pa[0] * a[0] + pa[1] * a[1] + pa[2] * a[2];
    let radial = [pa[0] - a[0] * h, pa[1] - a[1] * h, pa[2] - a[2] * h];
    let d_radial = (radial[0] * radial[0] + radial[1] * radial[1] + radial[2] * radial[2]).sqrt();
    let d_cap = (-h).max(h - height);
    (d_radial - radius).max(d_cap)
}

/// Hard CSG on signed-distance fields.
#[inline]
fn compose_sdf(op: ComposeOp, children: &[Action], p: [f64; 3]) -> f64 {
    match children {
        [] => f64::INFINITY,
        [only] => only.dist(p),
        [first, rest @ ..] => match op {
            ComposeOp::Union => children
                .iter()
                .map(|c| c.dist(p))
                .fold(f64::INFINITY, f64::min),
            ComposeOp::Intersection => children
                .iter()
                .map(|c| c.dist(p))
                .fold(f64::NEG_INFINITY, f64::max),
            ComposeOp::Difference => rest
                .iter()
                .fold(first.dist(p), |acc, c| acc.max(-c.dist(p))),
        },
    }
}

/// [`Sdf`] view of an [`Action`].
pub struct ActionSdf<'a>(pub &'a Action);

impl Sdf for ActionSdf<'_> {
    fn dist(&self, p: [f64; 3]) -> f64 {
        self.0.dist(p)
    }
}

/// Voxel shape key of `action` at `bits` per axis.
///
/// # Errors
/// The [`ManifoldError`] of [`sdf_shape_canonicalize`] (resolution outside policy, failed bound).
pub fn action_sdf_canonicalize(action: &Action, bits: u8) -> Result<ActionShapeId, ManifoldError> {
    Ok(ActionShapeId(sdf_shape_canonicalize(
        &ActionSdf(action),
        bits,
    )?))
}

fn hash_f64_le(h: &mut blake3::Hasher, x: f64) {
    h.update(&x.to_le_bytes());
}

fn hash_f64_slice_le(h: &mut blake3::Hasher, xs: &[f64; 3]) {
    for x in xs {
        hash_f64_le(h, *x);
    }
}

fn hash_action_geometry_quotient(h: &mut blake3::Hasher, action: &Action) {
    match action {
        Action::Sphere { center, radius } => {
            h.update(&[0u8]);
            hash_f64_slice_le(h, center);
            hash_f64_le(h, *radius);
        }
        Action::ConstDist(c) => {
            h.update(&[1u8]);
            hash_f64_le(h, *c);
        }
        Action::GateScalar(v) => {
            h.update(&[2u8]);
            hash_f64_le(h, *v);
        }
        Action::Box {
            center,
            half_extents,
        } => {
            h.update(&[3u8]);
            hash_f64_slice_le(h, center);
            hash_f64_slice_le(h, half_extents);
        }
        Action::Cylinder {
            base,
            axis,
            radius,
            height,
        } => {
            h.update(&[4u8]);
            hash_f64_slice_le(h, base);
            hash_f64_slice_le(h, axis);
            hash_f64_le(h, *radius);
            hash_f64_le(h, *height);
        }
        Action::Compose { op, children } => {
            h.update(&[5u8]);
            h.update(&[op.wire_tag()]);
            let n = u32::try_from(children.len()).unwrap_or(u32::MAX);
            h.update(&n.to_le_bytes());
            for child in children {
                hash_action_geometry_quotient(h, child);
            }
        }
    }
}

/// Intrinsic geometry key of the analytic tree; independent of any voxel resolution.
#[must_use]
pub fn action_geometry_quotient_key(action: &Action) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(GEOMETRY_QUOTIENT_DOMAIN);
    hash_action_geometry_quotient(&mut h, action);
    *h.finalize().as_bytes()
}

/// Ledger key derived from a canonical [`ActionShapeId`].
#[must_use]
pub fn action_quotient_id(shape: &ActionShapeId) -> ActionShapeId {
    let mut h = blake3::Hasher::new();
    h.update(SHAPE_QUOTIENT_DOMAIN);
    h.update(&shape.0);
    ActionShapeId(*h.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{
        action_geometry_quotient_key, action_quotient_id, action_sdf_canonicalize, Action,
        ActionSdf, ComposeOp,
    };
    use crate::manifold::Sdf;

    fn unit_box() -> Action {
        Action::Box {
            center: [0.0, 0.0, 0.0],
            half_extents: [0.5, 0.5, 0.5],
        }
    }

    fn ball() -> Action {
        Action::Sphere {
            center: [0.25, 0.0, 0.0],
            radius: 0.3,
        }
    }

    #[test]
    fn csg_distances_follow_min_max_and_difference() {
        let p = [0.6, 0.0, 0.0];
        let (db, ds) = (ActionSdf(&unit_box()).dist(p), ActionSdf(&ball()).dist(p));
        let of = |op, children| {
            ActionSdf(&Action::Compose { op, children })
                .dist(p)
                .to_bits()
        };
        let pair = || vec![unit_box(), ball()];
        assert_eq!(of(ComposeOp::Union, pair()), db.min(ds).to_bits());
        assert_eq!(of(ComposeOp::Intersection, pair()), db.max(ds).to_bits());
        assert_eq!(of(ComposeOp::Difference, pair()), db.max(-ds).to_bits());
        assert_eq!(of(ComposeOp::Union, vec![]), f64::INFINITY.to_bits());
        assert_eq!(of(ComposeOp::Difference, vec![ball()]), ds.to_bits());
    }

    #[test]
    fn box_and_cylinder_are_negative_inside_and_positive_outside() {
        let ub = unit_box();
        let b = ActionSdf(&ub);
        assert!(b.dist([0.0, 0.0, 0.0]) < 0.0);
        assert!(b.dist([1.0, 0.0, 0.0]) > 0.0);
        let c = Action::Cylinder {
            base: [0.0, 0.0, 0.0],
            axis: [0.0, 2.0, 0.0],
            radius: 0.1,
            height: 1.0,
        };
        let c = ActionSdf(&c);
        assert!(c.dist([0.0, 0.5, 0.0]) < 0.0);
        assert!(c.dist([0.0, 1.5, 0.0]) > 0.0);
        assert!(c.dist([0.5, 0.5, 0.0]) > 0.0);
    }

    #[test]
    fn geometry_key_separates_trees_and_ignores_resolution() {
        let u = Action::Compose {
            op: ComposeOp::Union,
            children: vec![unit_box(), ball()],
        };
        let i = Action::Compose {
            op: ComposeOp::Intersection,
            children: vec![unit_box(), ball()],
        };
        assert_eq!(
            action_geometry_quotient_key(&u),
            action_geometry_quotient_key(&u.clone())
        );
        assert_ne!(
            action_geometry_quotient_key(&u),
            action_geometry_quotient_key(&i)
        );
        let s4 = action_sdf_canonicalize(&u, 4).expect("bits 4 in policy");
        let s6 = action_sdf_canonicalize(&u, 6).expect("bits 6 in policy");
        assert_ne!(s4, s6);
        assert_eq!(action_quotient_id(&s6), action_quotient_id(&s6));
        assert_ne!(action_quotient_id(&s6), s6);
    }

    #[test]
    fn out_of_policy_resolution_is_refused() {
        assert!(action_sdf_canonicalize(&unit_box(), 0).is_err());
        assert!(action_sdf_canonicalize(&unit_box(), u8::MAX).is_err());
    }
}

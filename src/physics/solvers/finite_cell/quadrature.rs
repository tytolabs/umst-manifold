// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Quadrature over the solid part of a grid cell, with positive weights only.
//!
//! A positive weight keeps `K = Σ w Bᵀ D B` positive semidefinite and `M = Σ w ρ Nᵀ N` positive definite on the
//! solid, which moment-fitted rules with negative weights can lose (Hansbo, Larson & Larsson 2017, fig. 4).

use super::{gauss_legendre_2, gauss_legendre_3, real, OccupancyField};

/// One quadrature point: local coordinates in the parent cell (`[−1, 1]³`), a physical volume weight (m³), and
/// the material there.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QPoint {
    /// Local coordinates in the parent cell.
    pub xi: [f64; 3],
    /// Physical volume carried by the point.
    pub weight: f64,
    /// Material index.
    pub material: u16,
}

/// How faithfully a cell's rule integrates its solid part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Exactness {
    /// A whole cell of one material: exact for the element integrands.
    Whole,
    /// A cell cut in the plane: exact on the region bounded by the piecewise-linear zero contour.
    Clipped,
    /// A cell whose solid varies through its thickness, or whose material varies inside a cut leaf: octree point
    /// inclusion or a centroid material, convergent but not exact.
    Approximate,
}

/// The rule of one cell.
#[derive(Clone, Debug, PartialEq)]
pub struct CellQuadrature {
    /// Points with positive weights.
    pub points: Vec<QPoint>,
    /// Solid volume, the sum of the weights.
    pub volume: f64,
    /// Faithfulness of the rule.
    pub exactness: Exactness,
}

/// Subdivision depths and the geometric tolerance, derived from the cell size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadratureSpec {
    /// Quadtree depth for cells cut in the plane.
    pub plane_depth: u32,
    /// Octree depth for cells cut through the thickness.
    pub solid_depth: u32,
    /// Length below which a zero crossing is located and two signed distances are equal (m).
    pub tolerance: f64,
}

impl QuadratureSpec {
    /// Depths from the largest cell edge and the geometric tolerance: subdivide until a leaf edge is no longer than
    /// `√(tolerance · cell)`, the size at which the chord of a boundary of curvature radius about `cell` departs from
    /// the arc by `tolerance`. The octree takes the same leaf size: its point-included boundary leaves then carry a
    /// relative volume error of order `√(tolerance / cell)` per cut cell, and a finer octree would multiply its
    /// boundary leaves by four per level for that one-dimensional gain.
    #[must_use]
    pub fn from_tolerance(cell: f64, tolerance: f64) -> Self {
        let levels = |ratio: f64| -> u32 {
            if ratio.is_finite() && ratio > 1.0 {
                // log2 of a finite ratio above one is positive and small; the ceiling fits a u32.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let d = ratio.log2().ceil() as u32;
                d
            } else {
                0
            }
        };
        let leaf = (tolerance * cell).sqrt();
        let depth = levels(cell / leaf);
        Self {
            plane_depth: depth,
            solid_depth: depth,
            tolerance,
        }
    }
}

/// Bisection steps that shrink a segment of length `len` to `tol`: `⌈log₂(len / tol)⌉`.
fn bisections(len: f64, tol: f64) -> u32 {
    let ratio = len / tol;
    if ratio.is_finite() && ratio > 1.0 {
        // log2 of a finite ratio above one is positive and small; the ceiling fits a u32.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let steps = ratio.log2().ceil() as u32;
        steps
    } else {
        0
    }
}

/// The zero of a distance `dist` between an inside point `a` and an outside point `b`, by bisection.
fn crossing<D: Fn([f64; 3]) -> f64>(dist: &D, a: [f64; 3], b: [f64; 3], tol: f64) -> [f64; 3] {
    let len = (0..3).map(|i| (b[i] - a[i]).powi(2)).sum::<f64>().sqrt();
    let steps = bisections(len, tol);
    let mid = |p: [f64; 3], q: [f64; 3]| std::array::from_fn(|i| (p[i] + q[i]) / real(2));
    let (inside, outside) = (0..steps).fold((a, b), |(inside, outside), _| {
        let m = mid(inside, outside);
        if dist(m) <= 0.0 {
            (m, outside)
        } else {
            (inside, m)
        }
    });
    mid(inside, outside)
}

/// Points of the collapsed-square (Duffy) rule on a triangle in the plane, 3×3 Gauss: exact for total degree 4.
fn triangle_rule(t: [[f64; 2]; 3]) -> Vec<([f64; 2], f64)> {
    let g = gauss_legendre_3();
    let [a, b, c] = t;
    let twice_area = ((b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])).abs();
    g.iter()
        .flat_map(|&(tu, wu)| {
            g.iter().map(move |&(tv, wv)| {
                let u = (1.0 + tu) / real(2);
                let v = (1.0 + tv) / real(2);
                let p = std::array::from_fn(|i| a[i] + u * (b[i] - a[i]) + u * v * (c[i] - b[i]));
                (p, wu * wv / real(4) * u * twice_area)
            })
        })
        .collect()
}

/// The part of a triangle where `dist` is not positive, as a convex polygon (the zero contour taken as straight
/// across the triangle).
fn clip_triangle<D: Fn([f64; 3]) -> f64>(
    dist: &D,
    t: [[f64; 2]; 3],
    z: f64,
    tol: f64,
) -> Vec<[f64; 2]> {
    let at = |p: [f64; 2]| [p[0], p[1], z];
    let inside: Vec<bool> = t.iter().map(|&p| dist(at(p)) <= 0.0).collect();
    (0..3).fold(Vec::new(), |mut poly, i| {
        let j = (i + 1) % 3;
        if inside[i] {
            poly.push(t[i]);
        }
        if inside[i] != inside[j] {
            let (pin, pout) = if inside[i] {
                (t[i], t[j])
            } else {
                (t[j], t[i])
            };
            let x = crossing(dist, at(pin), at(pout), tol);
            poly.push([x[0], x[1]]);
        }
        poly
    })
}

/// The one material all samples share, if every sample is solid and they agree.
fn uniform(samples: &[Option<u16>]) -> Option<u16> {
    let first = samples.first().copied().flatten()?;
    samples.iter().all(|&m| m == Some(first)).then_some(first)
}

/// Build the rule of the cell `[lo, hi]`.
///
/// Classification uses the Lipschitz contract of the field's distances: a box is empty when its centre reads at
/// least its half-diagonal, wholly solid when it reads at most minus its half-diagonal (and its material samples
/// agree), and is subdivided otherwise. A prismatic cell is classified in the plane by
/// [`OccupancyField::section_distance`] at mid-height and integrated as its cross-section times its height.
#[must_use]
pub fn cell_quadrature<F: OccupancyField + ?Sized>(
    field: &F,
    lo: [f64; 3],
    hi: [f64; 3],
    spec: &QuadratureSpec,
) -> CellQuadrature {
    let h: [f64; 3] = std::array::from_fn(|i| hi[i] - lo[i]);
    let centre: [f64; 3] = std::array::from_fn(|i| (lo[i] + hi[i]) / real(2));
    let r3 = (h.iter().map(|x| x * x).sum::<f64>()).sqrt() / real(2);
    if field.signed_distance(centre) >= r3 {
        return finish(Vec::new(), Exactness::Whole);
    }
    let at = |t: [usize; 3]| -> [f64; 3] {
        std::array::from_fn(|i| lo[i] + h[i] * real(t[i]) / real(2))
    };
    let materials: Vec<Option<u16>> = (0..27)
        .map(|n| field.material_at(at([n % 3, (n / 3) % 3, n / 9])))
        .collect();
    let prismatic = field.prismatic(lo, hi);
    let r2 = (h[0] * h[0] + h[1] * h[1]).sqrt() / real(2);
    let solid_whole = if prismatic {
        field.section_distance(centre) <= -r2
    } else {
        field.signed_distance(centre) <= -r3
    };
    if let (true, Some(material)) = (solid_whole, uniform(&materials)) {
        return finish(box_points(lo, hi, lo, hi, material), Exactness::Whole);
    }
    if prismatic {
        let z = centre[2];
        let (pts2, mixed) = plane_leaf(
            field,
            [lo[0], lo[1]],
            [hi[0], hi[1]],
            z,
            spec,
            spec.plane_depth,
        );
        let gz = gauss_legendre_2();
        let points = pts2
            .into_iter()
            .flat_map(|(p, w, material)| {
                gz.iter().map(move |&(zeta, wz)| QPoint {
                    xi: [
                        (p[0] - lo[0]) / h[0] * real(2) - 1.0,
                        (p[1] - lo[1]) / h[1] * real(2) - 1.0,
                        zeta,
                    ],
                    weight: w * wz * h[2] / real(2),
                    material,
                })
            })
            .collect();
        return finish(
            points,
            if mixed {
                Exactness::Approximate
            } else {
                Exactness::Clipped
            },
        );
    }
    finish(
        solid_leaf(field, lo, hi, lo, hi, spec.solid_depth),
        Exactness::Approximate,
    )
}

fn finish(points: Vec<QPoint>, exactness: Exactness) -> CellQuadrature {
    let volume = points.iter().map(|q| q.weight).sum();
    CellQuadrature {
        points,
        volume,
        exactness,
    }
}

/// 2×2×2 Gauss points of the box `[blo, bhi]` inside the parent cell `[lo, hi]`.
fn box_points(
    lo: [f64; 3],
    hi: [f64; 3],
    blo: [f64; 3],
    bhi: [f64; 3],
    material: u16,
) -> Vec<QPoint> {
    let g = gauss_legendre_2();
    let vol: f64 = (0..3).map(|i| bhi[i] - blo[i]).product();
    (0..8)
        .map(|n| {
            let local: [f64; 3] = std::array::from_fn(|i| g[(n >> i) & 1].0);
            let p: [f64; 3] =
                std::array::from_fn(|i| blo[i] + (local[i] + 1.0) / real(2) * (bhi[i] - blo[i]));
            QPoint {
                xi: std::array::from_fn(|i| (p[i] - lo[i]) / (hi[i] - lo[i]) * real(2) - 1.0),
                weight: vol / real(8),
                material,
            }
        })
        .collect()
}

/// In-plane rule of the rectangle `[a, b]` at height `z`: `(point, area weight, material)`, and whether a leaf at the
/// last level held more than one material among its samples.
fn plane_leaf<F: OccupancyField + ?Sized>(
    field: &F,
    a: [f64; 2],
    b: [f64; 2],
    z: f64,
    spec: &QuadratureSpec,
    depth: u32,
) -> (Vec<([f64; 2], f64, u16)>, bool) {
    let corners = [[a[0], a[1]], [b[0], a[1]], [b[0], b[1]], [a[0], b[1]]];
    let centre = [(a[0] + b[0]) / real(2), (a[1] + b[1]) / real(2)];
    let r = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt() / real(2);
    let dist = |p: [f64; 3]| field.section_distance(p);
    let d = dist([centre[0], centre[1], z]);
    if d >= r {
        return (Vec::new(), false);
    }
    let materials: Vec<Option<u16>> = corners
        .iter()
        .chain(std::iter::once(&centre))
        .map(|p| field.material_at([p[0], p[1], z]))
        .collect();
    if let (true, Some(m)) = (d <= -r, uniform(&materials)) {
        let g = gauss_legendre_2();
        let area = (b[0] - a[0]) * (b[1] - a[1]);
        let pts = (0..4)
            .map(|n| {
                let p = std::array::from_fn(|i| {
                    a[i] + (g[(n >> i) & 1].0 + 1.0) / real(2) * (b[i] - a[i])
                });
                (p, area / real(4), m)
            })
            .collect();
        return (pts, false);
    }
    if depth > 0 {
        let quads = [
            ([a[0], a[1]], centre),
            ([centre[0], a[1]], [b[0], centre[1]]),
            ([a[0], centre[1]], [centre[0], b[1]]),
            (centre, [b[0], b[1]]),
        ];
        return quads
            .iter()
            .fold((Vec::new(), false), |(mut pts, mixed), &(qa, qb)| {
                let (p, m) = plane_leaf(field, qa, qb, z, spec, depth - 1);
                pts.extend(p);
                (pts, mixed || m)
            });
    }
    let solid: Vec<u16> = materials.iter().flatten().copied().collect();
    let mixed = solid.windows(2).any(|w| w[0] != w[1]);
    let pts = (0..4)
        .flat_map(|i| {
            let tri = [corners[i], corners[(i + 1) % 4], centre];
            let poly = clip_triangle(&dist, tri, z, spec.tolerance);
            let fan: Vec<[[f64; 2]; 3]> = (1..poly.len().saturating_sub(1))
                .map(|k| [poly[0], poly[k], poly[k + 1]])
                .collect();
            let material = poly_material(field, &poly, z);
            fan.into_iter().flat_map(move |t| {
                triangle_rule(t)
                    .into_iter()
                    .filter_map(move |(p, w)| material.map(|m| (p, w, m)))
            })
        })
        .collect();
    (pts, mixed)
}

fn poly_material<F: OccupancyField + ?Sized>(field: &F, poly: &[[f64; 2]], z: f64) -> Option<u16> {
    if poly.is_empty() {
        return None;
    }
    let n = real(poly.len());
    let c = poly
        .iter()
        .fold([0.0, 0.0], |c, p| [c[0] + p[0] / n, c[1] + p[1] / n]);
    field
        .material_at([c[0], c[1], z])
        .or_else(|| poly.iter().find_map(|p| field.material_at([p[0], p[1], z])))
}

/// Octree rule of the box `[blo, bhi]` in the parent `[lo, hi]`, classified by the Lipschitz bound: wholly solid
/// boxes of one material take Gauss points, empty boxes none, and boxes at the last level keep the Gauss points
/// that fall in the solid.
fn solid_leaf<F: OccupancyField + ?Sized>(
    field: &F,
    lo: [f64; 3],
    hi: [f64; 3],
    blo: [f64; 3],
    bhi: [f64; 3],
    depth: u32,
) -> Vec<QPoint> {
    let mid: [f64; 3] = std::array::from_fn(|i| (blo[i] + bhi[i]) / real(2));
    let r = (0..3)
        .map(|i| (bhi[i] - blo[i]).powi(2))
        .sum::<f64>()
        .sqrt()
        / real(2);
    let d = field.signed_distance(mid);
    if d >= r {
        return Vec::new();
    }
    let materials: Vec<Option<u16>> = (0..8)
        .map(|n| std::array::from_fn(|i| if (n >> i) & 1 == 0 { blo[i] } else { bhi[i] }))
        .chain(std::iter::once(mid))
        .map(|p| field.material_at(p))
        .collect();
    if let (true, Some(m)) = (d <= -r, uniform(&materials)) {
        return box_points(lo, hi, blo, bhi, m);
    }
    if depth == 0 {
        return box_points(lo, hi, blo, bhi, 0)
            .into_iter()
            .filter_map(|q| {
                let p: [f64; 3] =
                    std::array::from_fn(|i| lo[i] + (q.xi[i] + 1.0) / real(2) * (hi[i] - lo[i]));
                let inside = field.signed_distance(p) <= 0.0;
                field
                    .material_at(p)
                    .filter(|_| inside)
                    .map(|material| QPoint { material, ..q })
            })
            .collect();
    }
    (0..8)
        .flat_map(|n| {
            let clo: [f64; 3] =
                std::array::from_fn(|i| if (n >> i) & 1 == 0 { blo[i] } else { mid[i] });
            let chi: [f64; 3] =
                std::array::from_fn(|i| if (n >> i) & 1 == 0 { mid[i] } else { bhi[i] });
            solid_leaf(field, lo, hi, clo, chi, depth - 1)
        })
        .collect()
}

/// Area rule of the rectangle `[a, b]` of a plane at height `z` over the solid of `field`: `(point, area weight,
/// material)` with positive weights, classified by the field's section distance as in a prismatic cell.
#[must_use]
pub fn face_rule<F: OccupancyField + ?Sized>(
    field: &F,
    a: [f64; 2],
    b: [f64; 2],
    z: f64,
    spec: &QuadratureSpec,
) -> Vec<([f64; 2], f64, u16)> {
    plane_leaf(field, a, b, z, spec, spec.plane_depth).0
}

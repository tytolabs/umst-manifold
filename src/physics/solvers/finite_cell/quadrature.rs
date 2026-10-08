// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Quadrature over the solid part of a grid cell, with positive weights only.
//!
//! A positive weight keeps `K = Σ w Bᵀ D B` positive semidefinite and `M = Σ w ρ Nᵀ N` positive definite on the
//! solid, which moment-fitted rules with negative weights can lose (Hansbo, Larson & Larsson 2017, fig. 4).

use super::{gauss_legendre, real, OccupancyField};

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

/// The zero of the signed distance between an inside point `a` and an outside point `b`, by bisection.
fn crossing<F: OccupancyField + ?Sized>(field: &F, a: [f64; 3], b: [f64; 3], tol: f64) -> [f64; 3] {
    let len = (0..3).map(|i| (b[i] - a[i]).powi(2)).sum::<f64>().sqrt();
    let steps = bisections(len, tol);
    let mid = |p: [f64; 3], q: [f64; 3]| std::array::from_fn(|i| (p[i] + q[i]) / real(2));
    let (inside, outside) = (0..steps).fold((a, b), |(inside, outside), _| {
        let m = mid(inside, outside);
        if field.signed_distance(m) <= 0.0 {
            (m, outside)
        } else {
            (inside, m)
        }
    });
    mid(inside, outside)
}

/// Points of the collapsed-square (Duffy) rule on a triangle in the plane, 3×3 Gauss: exact for total degree 4.
fn triangle_rule(t: [[f64; 2]; 3]) -> Vec<([f64; 2], f64)> {
    let g = gauss_legendre(3);
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

/// The part of a triangle where the signed distance is not positive, as a convex polygon.
fn clip_triangle<F: OccupancyField + ?Sized>(
    field: &F,
    t: [[f64; 2]; 3],
    z: f64,
    tol: f64,
) -> Vec<[f64; 2]> {
    let at = |p: [f64; 2]| [p[0], p[1], z];
    let inside: Vec<bool> = t
        .iter()
        .map(|&p| field.signed_distance(at(p)) <= 0.0)
        .collect();
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
            let x = crossing(field, at(pin), at(pout), tol);
            poly.push([x[0], x[1]]);
        }
        poly
    })
}

/// Build the rule of the cell `[lo, hi]`.
#[must_use]
pub fn cell_quadrature<F: OccupancyField + ?Sized>(
    field: &F,
    lo: [f64; 3],
    hi: [f64; 3],
    spec: &QuadratureSpec,
) -> CellQuadrature {
    let h: [f64; 3] = std::array::from_fn(|i| hi[i] - lo[i]);
    let at = |t: [usize; 3]| -> [f64; 3] {
        std::array::from_fn(|i| lo[i] + h[i] * real(t[i]) / real(2))
    };
    let lattice: Vec<[usize; 3]> = (0..27).map(|n| [n % 3, (n / 3) % 3, n / 9]).collect();
    let samples: Vec<(f64, Option<u16>)> = lattice
        .iter()
        .map(|&t| (field.signed_distance(at(t)), field.material_at(at(t))))
        .collect();
    let first = samples[0].1;
    if first.is_some() && samples.iter().all(|&(d, m)| d <= 0.0 && m == first) {
        let material = first.unwrap_or_default();
        return finish(box_points(lo, hi, lo, hi, material), Exactness::Whole);
    }
    if samples.iter().all(|&(d, m)| d > 0.0 && m.is_none()) {
        return finish(Vec::new(), Exactness::Whole);
    }
    // Prismatic: inside/outside and the material agree through the thickness at every in-plane sample. The value
    // of the distance may still vary (a body whose faces coincide with the cell's faces reads zero there), but
    // the solid part is then the clipped in-plane region times the cell height.
    let prismatic = (0..9).all(|n| {
        let (d0, m0) = samples[n];
        (1..3).all(|layer| {
            let (d, m) = samples[n + 9 * layer];
            (d <= 0.0) == (d0 <= 0.0) && m == m0
        })
    });
    if prismatic {
        let zmid = (lo[2] + hi[2]) / real(2);
        let (pts2, exact) = plane_leaf(
            field,
            [lo[0], lo[1]],
            [hi[0], hi[1]],
            zmid,
            spec,
            spec.plane_depth,
        );
        let gz = gauss_legendre(2);
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
            if exact {
                Exactness::Clipped
            } else {
                Exactness::Approximate
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
    let g = gauss_legendre(2);
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

/// In-plane rule of the rectangle `[a, b]` at height `z`: `(point, area weight, material)` and whether it is exact.
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
    let probe = |p: [f64; 2]| {
        (
            field.signed_distance([p[0], p[1], z]),
            field.material_at([p[0], p[1], z]),
        )
    };
    let s: Vec<(f64, Option<u16>)> = corners
        .iter()
        .chain(std::iter::once(&centre))
        .map(|&p| probe(p))
        .collect();
    let m0 = s[0].1.or(s[4].1);
    let uniform = s.iter().all(|&(_, m)| m.is_none() || m == m0);
    if let Some(m) = m0 {
        if s.iter().all(|&(d, mm)| d <= 0.0 && mm == Some(m)) {
            let g = gauss_legendre(2);
            let area = (b[0] - a[0]) * (b[1] - a[1]);
            let pts = (0..4)
                .map(|n| {
                    let p = std::array::from_fn(|i| {
                        a[i] + (g[(n >> i) & 1].0 + 1.0) / real(2) * (b[i] - a[i])
                    });
                    (p, area / real(4), m)
                })
                .collect();
            return (pts, true);
        }
    }
    if s.iter().all(|&(d, m)| d > 0.0 && m.is_none()) {
        return (Vec::new(), true);
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
            .fold((Vec::new(), true), |(mut pts, exact), &(qa, qb)| {
                let (p, e) = plane_leaf(field, qa, qb, z, spec, depth - 1);
                pts.extend(p);
                (pts, exact && e)
            });
    }
    let pts = (0..4)
        .flat_map(|i| {
            let tri = [corners[i], corners[(i + 1) % 4], centre];
            let poly = clip_triangle(field, tri, z, spec.tolerance);
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
    (pts, uniform)
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

/// Octree rule of the box `[blo, bhi]` in the parent `[lo, hi]`: whole boxes take Gauss points; boxes at the
/// last level keep the Gauss points that fall in the solid.
fn solid_leaf<F: OccupancyField + ?Sized>(
    field: &F,
    lo: [f64; 3],
    hi: [f64; 3],
    blo: [f64; 3],
    bhi: [f64; 3],
    depth: u32,
) -> Vec<QPoint> {
    let mid: [f64; 3] = std::array::from_fn(|i| (blo[i] + bhi[i]) / real(2));
    let probes: Vec<[f64; 3]> = (0..8)
        .map(|n| std::array::from_fn(|i| if (n >> i) & 1 == 0 { blo[i] } else { bhi[i] }))
        .chain(std::iter::once(mid))
        .collect();
    let s: Vec<(f64, Option<u16>)> = probes
        .iter()
        .map(|&p| (field.signed_distance(p), field.material_at(p)))
        .collect();
    if let Some(m) = s[8].1 {
        if s.iter().all(|&(d, mm)| d <= 0.0 && mm == Some(m)) {
            return box_points(lo, hi, blo, bhi, m);
        }
    }
    if s.iter().all(|&(d, m)| d > 0.0 && m.is_none()) {
        return Vec::new();
    }
    if depth == 0 {
        let parent: Vec<QPoint> = box_points(lo, hi, blo, bhi, 0);
        return parent
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

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Tensor-product grids whose node planes include every layer interface.

use super::{real, FiniteCellRefuse};

/// A tensor-product grid: strictly increasing node coordinates along each axis.
///
/// Nodes and cells are numbered with the axis of fewest nodes fastest, then the next, so a plate's profile is
/// bounded by its smallest cross-section.
#[derive(Clone, Debug, PartialEq)]
pub struct TensorGrid {
    axes: [Vec<f64>; 3],
    order: [usize; 3],
}

impl TensorGrid {
    /// A grid on `[lo, hi]` whose node planes include every interface plane inside the bounds. Each interval
    /// between consecutive breakpoints is split into the fewest equal cells no wider than `max_spacing` on that
    /// axis. Breakpoints closer than `tolerance` merge.
    ///
    /// # Errors
    /// [`FiniteCellRefuse::InvalidGeometry`] for non-finite or non-positive spacings and tolerance, or `lo ≥ hi`.
    pub fn conforming(
        lo: [f64; 3],
        hi: [f64; 3],
        max_spacing: [f64; 3],
        planes: &[Vec<f64>; 3],
        tolerance: f64,
    ) -> Result<Self, FiniteCellRefuse> {
        if !(tolerance.is_finite() && tolerance > 0.0) {
            return Err(FiniteCellRefuse::InvalidGeometry);
        }
        let axis = |a: usize| -> Result<Vec<f64>, FiniteCellRefuse> {
            let (l, h, s) = (lo[a], hi[a], max_spacing[a]);
            if !(l.is_finite() && h.is_finite() && s.is_finite() && s > 0.0 && l < h) {
                return Err(FiniteCellRefuse::InvalidGeometry);
            }
            let mut breaks: Vec<f64> = std::iter::once(l)
                .chain(
                    planes[a]
                        .iter()
                        .copied()
                        .filter(|p| p.is_finite() && *p > l + tolerance && *p < h - tolerance),
                )
                .chain(std::iter::once(h))
                .collect();
            breaks.sort_by(f64::total_cmp);
            breaks.dedup_by(|b, a| (*b - *a).abs() <= tolerance);
            let nodes = breaks.windows(2).fold(vec![breaks[0]], |mut nodes, w| {
                let span = w[1] - w[0];
                let parts = cells_for(span, s);
                (1..=parts).for_each(|k| nodes.push(w[0] + span * real(k) / real(parts)));
                nodes
            });
            Ok(nodes)
        };
        let axes = [axis(0)?, axis(1)?, axis(2)?];
        let mut order = [0, 1, 2];
        order.sort_by_key(|&a| axes[a].len());
        Ok(Self { axes, order })
    }

    /// Node coordinates along `axis`.
    #[must_use]
    pub fn nodes(&self, axis: usize) -> &[f64] {
        &self.axes[axis]
    }

    /// Cells along each axis.
    #[must_use]
    pub fn cell_counts(&self) -> [usize; 3] {
        [
            self.axes[0].len() - 1,
            self.axes[1].len() - 1,
            self.axes[2].len() - 1,
        ]
    }

    /// Nodes along each axis.
    #[must_use]
    pub fn node_counts(&self) -> [usize; 3] {
        [self.axes[0].len(), self.axes[1].len(), self.axes[2].len()]
    }

    /// Total nodes.
    #[must_use]
    pub fn node_total(&self) -> usize {
        self.axes.iter().map(Vec::len).product()
    }

    /// Linear index of node `(i, j, k)`, fewest-node axis fastest.
    #[must_use]
    pub fn node_index(&self, ijk: [usize; 3]) -> usize {
        let n = self.node_counts();
        let [a, b, c] = self.order;
        ijk[a] + n[a] * (ijk[b] + n[b] * ijk[c])
    }

    /// Node `(i, j, k)` of a linear index.
    #[must_use]
    pub fn node_ijk(&self, index: usize) -> [usize; 3] {
        let n = self.node_counts();
        let [a, b, c] = self.order;
        let mut ijk = [0; 3];
        ijk[a] = index % n[a];
        ijk[b] = (index / n[a]) % n[b];
        ijk[c] = index / (n[a] * n[b]);
        ijk
    }

    /// Coordinates of node `(i, j, k)`.
    #[must_use]
    pub fn node_point(&self, ijk: [usize; 3]) -> [f64; 3] {
        [
            self.axes[0][ijk[0]],
            self.axes[1][ijk[1]],
            self.axes[2][ijk[2]],
        ]
    }

    /// Lower and upper corners of cell `(i, j, k)`.
    #[must_use]
    pub fn cell_box(&self, ijk: [usize; 3]) -> ([f64; 3], [f64; 3]) {
        (
            [
                self.axes[0][ijk[0]],
                self.axes[1][ijk[1]],
                self.axes[2][ijk[2]],
            ],
            [
                self.axes[0][ijk[0] + 1],
                self.axes[1][ijk[1] + 1],
                self.axes[2][ijk[2] + 1],
            ],
        )
    }

    /// The eight node indices of cell `(i, j, k)`, local corner `a = ix + 2 iy + 4 iz`.
    #[must_use]
    pub fn cell_nodes(&self, ijk: [usize; 3]) -> [usize; 8] {
        std::array::from_fn(|a| {
            self.node_index([
                ijk[0] + (a & 1),
                ijk[1] + ((a >> 1) & 1),
                ijk[2] + ((a >> 2) & 1),
            ])
        })
    }

    /// All cells in the grid's numbering order.
    pub fn cells(&self) -> impl Iterator<Item = [usize; 3]> + '_ {
        let n = self.cell_counts();
        let [a, b, c] = self.order;
        (0..n[c]).flat_map(move |kc| {
            (0..n[b]).flat_map(move |kb| {
                (0..n[a]).map(move |ka| {
                    let mut ijk = [0; 3];
                    ijk[a] = ka;
                    ijk[b] = kb;
                    ijk[c] = kc;
                    ijk
                })
            })
        })
    }

    /// The cell containing `p`, if inside the grid (a point on a shared face goes to the lower cell's
    /// neighbour above, except on the upper bound).
    #[must_use]
    pub fn locate(&self, p: [f64; 3]) -> Option<[usize; 3]> {
        let find = |a: usize| -> Option<usize> {
            let x = &self.axes[a];
            let (lo, hi) = (x[0], x[x.len() - 1]);
            if !(p[a] >= lo && p[a] <= hi) {
                return None;
            }
            let i = x.partition_point(|&v| v <= p[a]);
            Some(i.saturating_sub(1).min(x.len() - 2))
        };
        Some([find(0)?, find(1)?, find(2)?])
    }
}

/// The fewest equal cells no wider than `max_spacing` across `span`.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn cells_for(span: f64, max_spacing: f64) -> usize {
    // span / spacing is finite and positive here; the ceiling of a value below 2⁵³ fits a usize. A ratio that is
    // an integer up to rounding must not gain a cell from the last ulp.
    let ratio = span / max_spacing;
    ((ratio - ratio * f64::EPSILON.sqrt()).ceil() as usize).max(1)
}

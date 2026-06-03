//! Hilbert **2D** curve: space-filling index (L-M0 empirical; `C` locality bound in tests).
//!
//! Algorithm: the classic recursive quadrant transform (public-domain Wikipedia shape).

// SPDX-License-Identifier: MIT
// 2D Hilbert d2xy/xy2d — `xy2d` is implemented by **linear scan** on `0..2^(2*bits)`; ok for
// `bits ≤ 8` (B-Arc can replace with O(bits) if needed for hot paths).

use super::error::ManifoldError;

/// `{dim, bits}` — `bits` is the per-axis order `n` (grid side `1 << n`); M-0 tests `dim=2, bits=4` → 256 cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HilbertCurve {
    /// Ambient 2 (primary) or 3 (reserved; not wired in M-0).
    pub dim: u8,
    /// Hilbert order: side length `1 << bits`.
    pub bits: u8,
}

/// Rotate / reflect quadrant to walk the curve (2D, Wikipedia `rot` helper).
fn rot2(s: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 {
            *x = s - 1u32 - *x;
            *y = s - 1u32 - *y;
        }
        std::mem::swap(x, y);
    }
}

/// `d` in `0..4^n` to `(x,y)` in `0..2^n` (2D only).
pub fn d2xy(n: u8, d: u32) -> (u32, u32) {
    let nbits = n as u32;
    let limit = 1u32 << nbits;
    if d >= limit * limit {
        // outside valid range: caller bug
        return (0, 0);
    }
    let mut t = d;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut s: u32 = 1;
    while s < limit {
        let rx = (t / 2) & 1u32;
        let ry = (t ^ rx) & 1u32;
        rot2(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        t /= 4;
        s <<= 1;
    }
    (x, y)
}

/// Point `(x,y)` to linear distance along the Hilbert curve. **M-0** uses linear scan; **O(4^bits)** (bits≤8 is fine for registry policy).
pub fn xy2d(n: u8, x: u32, y: u32) -> Result<u32, ManifoldError> {
    let limit = 1u32 << (2 * n as u32);
    for d in 0u32..limit {
        if d2xy(n, d) == (x, y) {
            return Ok(d);
        }
    }
    Err(ManifoldError::OutOfRange(0, limit - 1))
}

impl HilbertCurve {
    /// `dim` must be 2; `bits` the order `n` (1..=8 in policy).
    pub fn new_2d(bits: u8) -> Option<Self> {
        if bits == 0 || bits > 8 {
            return None;
        }
        Some(HilbertCurve { dim: 2, bits })
    }

    /// `d2xy` with error if `d` out of range.
    pub fn d2xy(&self, d: u32) -> Result<(u32, u32), ManifoldError> {
        if self.dim != 2 {
            return Err(ManifoldError::OutOfRange(0, 0));
        }
        let side = 1u32 << self.bits;
        if d >= side * side {
            return Err(ManifoldError::OutOfRange(
                d,
                side.saturating_mul(side).saturating_sub(1),
            ));
        }
        Ok(d2xy(self.bits, d))
    }
}

/// Manhattan distance on a 2D `bits`-grid.
pub fn manhattan(a: (u32, u32), b: (u32, u32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

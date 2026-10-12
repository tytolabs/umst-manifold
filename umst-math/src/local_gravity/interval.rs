// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Exact rational intervals with outward rounding to a fixed scale: every operation returns an interval that
//! contains the exact result, so a decision taken on the endpoints is a decision about the real value.

use core::cmp::Ordering;

use super::{ExactRat, RatRefusal};

/// Endpoints are rounded outward to multiples of `1 / SCALE`.
pub const SCALE: i128 = 1_000_000_000_000;

/// Why an interval operation gives no interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntervalRefusal {
    /// The lower endpoint exceeds the upper endpoint.
    Reversed,
    /// An exact intermediate left `i128`, or an input was not a finite number in range.
    Overflow,
    /// A divisor interval does not lie strictly above zero.
    DivisorNotPositive,
}

impl From<RatRefusal> for IntervalRefusal {
    fn from(_: RatRefusal) -> Self {
        Self::Overflow
    }
}

/// A closed interval `[lo, hi]` of exact rationals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RatInterval {
    lo: ExactRat,
    hi: ExactRat,
}

/// The hull of four candidates, each rounded outward before any comparison (a comparison of unreduced products
/// would multiply two large denominators).
fn hull(values: [ExactRat; 4]) -> Result<RatInterval, IntervalRefusal> {
    let mut lo = values[0].floor_to_scale(SCALE)?;
    let mut hi = values[0].ceil_to_scale(SCALE)?;
    for v in &values[1..] {
        let (f, c) = (v.floor_to_scale(SCALE)?, v.ceil_to_scale(SCALE)?);
        if f.try_cmp(lo)? == Ordering::Less {
            lo = f;
        }
        if c.try_cmp(hi)? == Ordering::Greater {
            hi = c;
        }
    }
    RatInterval::new(lo, hi)
}

impl RatInterval {
    /// The interval `[lo, hi]`.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Reversed`] for `lo > hi`.
    pub fn new(lo: ExactRat, hi: ExactRat) -> Result<Self, IntervalRefusal> {
        if lo.try_cmp(hi)? == Ordering::Greater {
            return Err(IntervalRefusal::Reversed);
        }
        Ok(Self { lo, hi })
    }

    /// The degenerate interval `[x, x]`.
    #[must_use]
    pub const fn point(x: ExactRat) -> Self {
        Self { lo: x, hi: x }
    }

    /// The lower endpoint.
    #[must_use]
    pub const fn lo(self) -> ExactRat {
        self.lo
    }

    /// The upper endpoint.
    #[must_use]
    pub const fn hi(self) -> ExactRat {
        self.hi
    }

    /// An interval around the float `x` of half-width `radius`, rounded outward to `1e-9` and widened by one unit
    /// there, so it contains `[x − radius, x + radius]` despite the float arithmetic of this conversion.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] for a non-finite or out-of-range input or a negative radius.
    pub fn around_f64(x: f64, radius: f64) -> Result<Self, IntervalRefusal> {
        const UNIT: f64 = 1.0e9;
        if !x.is_finite() || !radius.is_finite() || radius < 0.0 || x.abs() > 1.0e15 || radius > 1.0e15 {
            return Err(IntervalRefusal::Overflow);
        }
        let lo = ((x - radius) * UNIT).floor() as i128 - 1;
        let hi = ((x + radius) * UNIT).ceil() as i128 + 1;
        Self::new(ExactRat::new(lo, 1_000_000_000)?, ExactRat::new(hi, 1_000_000_000)?)
    }

    fn outward(lo: ExactRat, hi: ExactRat) -> Result<Self, IntervalRefusal> {
        Self::new(lo.floor_to_scale(SCALE)?, hi.ceil_to_scale(SCALE)?)
    }

    /// `self + other`, rounded outward.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn add(self, other: Self) -> Result<Self, IntervalRefusal> {
        Self::outward(self.lo.checked_add(other.lo)?, self.hi.checked_add(other.hi)?)
    }

    /// `self - other`, rounded outward.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn sub(self, other: Self) -> Result<Self, IntervalRefusal> {
        Self::outward(self.lo.checked_sub(other.hi)?, self.hi.checked_sub(other.lo)?)
    }

    /// `self · other`, rounded outward.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn mul(self, other: Self) -> Result<Self, IntervalRefusal> {
        hull([
            self.lo.checked_mul(other.lo)?,
            self.lo.checked_mul(other.hi)?,
            self.hi.checked_mul(other.lo)?,
            self.hi.checked_mul(other.hi)?,
        ])
    }

    /// `k · self` for an exact scalar, rounded outward.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn mul_scalar(self, k: ExactRat) -> Result<Self, IntervalRefusal> {
        self.mul(Self::point(k))
    }

    /// `{x² : x ∈ self}`, rounded outward (zero is the lower endpoint when the interval straddles zero).
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn square(self) -> Result<Self, IntervalRefusal> {
        let (l2, h2) = (self.lo.checked_mul(self.lo)?, self.hi.checked_mul(self.hi)?);
        if self.lo.sign() != Ordering::Greater && self.hi.sign() != Ordering::Less {
            let large = if l2.floor_to_scale(SCALE)?.try_cmp(h2.floor_to_scale(SCALE)?)? == Ordering::Greater {
                l2
            } else {
                h2
            };
            Self::outward(ExactRat::ZERO, large)
        } else {
            hull([l2, h2, l2, h2])
        }
    }

    /// `self / divisor` for a divisor interval strictly above zero, rounded outward.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::DivisorNotPositive`] unless `divisor.lo > 0`; [`IntervalRefusal::Overflow`] otherwise.
    pub fn div(self, divisor: Self) -> Result<Self, IntervalRefusal> {
        if divisor.lo.sign() != Ordering::Greater {
            return Err(IntervalRefusal::DivisorNotPositive);
        }
        hull([
            self.lo.checked_div(divisor.lo)?,
            self.lo.checked_div(divisor.hi)?,
            self.hi.checked_div(divisor.lo)?,
            self.hi.checked_div(divisor.hi)?,
        ])
    }

    /// Whether `self ⊆ other`.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when a comparison leaves `i128`.
    pub fn is_subset_of(self, other: Self) -> Result<bool, IntervalRefusal> {
        Ok(self.lo.try_cmp(other.lo)? != Ordering::Less && self.hi.try_cmp(other.hi)? != Ordering::Greater)
    }

    /// Whether `self ∩ other = ∅`.
    ///
    /// # Errors
    ///
    /// [`IntervalRefusal::Overflow`] when a comparison leaves `i128`.
    pub fn is_disjoint_from(self, other: Self) -> Result<bool, IntervalRefusal> {
        Ok(self.hi.try_cmp(other.lo)? == Ordering::Less || other.hi.try_cmp(self.lo)? == Ordering::Less)
    }
}

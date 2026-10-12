// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! The free-fall gravity gate: a local `g` is admissible when it is at least `v² / (2·h)`.
//!
//! THEOREM-BOUND: `UMST.FreeFall.freeFall_gravity_lower_bound` (umst-formal `Lean/FreeFall.lean`, with its Coq,
//! Agda and Haskell twins; `formal_parity.json` id `free_fall.gravity_lower_bound`). A body of mass `m > 0` that
//! falls from rest through `h > 0` as a `.transition` of `UMST.ProcessFamily.SecondLaw` has dissipation
//! `D = m·g·h − ½·m·v² ≥ 0`, hence `v² ≤ 2·g·h` and `g ≥ v² / (2·h)`. Gravity is the gradient of a scalar
//! potential on the configuration manifold and `ψ(z) = m·g·z` is linear, hence convex, in `z`, so the passive route
//! of `ConvexPhiDissipation` applies. The gate adds no law: it checks the lemma's consequence on exact rationals.
//!
//! Every function is pure and total: arithmetic that leaves `i128` is a typed refusal, never a panic or a wrap.
//! The value of `g` at a place is instance data; this module carries none.
//! Typed absence: the `k_B·T·ln 2` cost per bit extracted from the recording frames is the `measureFeedback` case,
//! follow-up cell FORMAL-FREEFALL-MEASURE-FEEDBACK.

use core::cmp::Ordering;

pub mod geodetic;
pub mod interval;
pub mod normal_gravity;

use geodetic::GeodeticPosition;
use interval::RatInterval;

/// Why a pair of integers or an operation does not give a rational.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RatRefusal {
    /// The denominator is zero.
    ZeroDenominator,
    /// The exact result does not fit in `i128`.
    Overflow,
}

/// An exact rational: normalised, with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactRat {
    num: i128,
    den: i128,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl ExactRat {
    /// Zero.
    pub const ZERO: Self = Self { num: 0, den: 1 };
    /// One.
    pub const ONE: Self = Self { num: 1, den: 1 };

    /// The rational `num / den` in lowest terms.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::ZeroDenominator`] for `den = 0`; [`RatRefusal::Overflow`] when a sign move leaves `i128`.
    pub fn new(num: i128, den: i128) -> Result<Self, RatRefusal> {
        if den == 0 {
            return Err(RatRefusal::ZeroDenominator);
        }
        let (num, den) = if den < 0 {
            (
                num.checked_neg().ok_or(RatRefusal::Overflow)?,
                den.checked_neg().ok_or(RatRefusal::Overflow)?,
            )
        } else {
            (num, den)
        };
        let d = gcd(num.unsigned_abs(), den.unsigned_abs()).max(1);
        // d divides both magnitudes, and den > 0, so the quotients fit in i128.
        Ok(Self {
            num: num / d as i128,
            den: den / d as i128,
        })
    }

    /// The integer `n`.
    #[must_use]
    pub const fn int(n: i128) -> Self {
        Self { num: n, den: 1 }
    }

    /// Normalised numerator.
    #[must_use]
    pub const fn num(self) -> i128 {
        self.num
    }

    /// Normalised positive denominator.
    #[must_use]
    pub const fn den(self) -> i128 {
        self.den
    }

    /// The sign: `Less` below zero, `Equal` at zero, `Greater` above.
    #[must_use]
    pub const fn sign(self) -> Ordering {
        if self.num < 0 {
            Ordering::Less
        } else if self.num == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    /// `self + other`.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn checked_add(self, other: Self) -> Result<Self, RatRefusal> {
        let a = self.num.checked_mul(other.den).ok_or(RatRefusal::Overflow)?;
        let b = other.num.checked_mul(self.den).ok_or(RatRefusal::Overflow)?;
        Self::new(
            a.checked_add(b).ok_or(RatRefusal::Overflow)?,
            self.den.checked_mul(other.den).ok_or(RatRefusal::Overflow)?,
        )
    }

    /// `-self`.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::Overflow`] for `i128::MIN` in the numerator.
    pub fn checked_neg(self) -> Result<Self, RatRefusal> {
        Ok(Self {
            num: self.num.checked_neg().ok_or(RatRefusal::Overflow)?,
            den: self.den,
        })
    }

    /// `self - other`.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn checked_sub(self, other: Self) -> Result<Self, RatRefusal> {
        self.checked_add(other.checked_neg()?)
    }

    /// `self · other`.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn checked_mul(self, other: Self) -> Result<Self, RatRefusal> {
        Self::new(
            self.num.checked_mul(other.num).ok_or(RatRefusal::Overflow)?,
            self.den.checked_mul(other.den).ok_or(RatRefusal::Overflow)?,
        )
    }

    /// `self / other`.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::ZeroDenominator`] for a zero divisor; [`RatRefusal::Overflow`] when an intermediate leaves `i128`.
    pub fn checked_div(self, other: Self) -> Result<Self, RatRefusal> {
        if other.num == 0 {
            return Err(RatRefusal::ZeroDenominator);
        }
        Self::new(
            self.num.checked_mul(other.den).ok_or(RatRefusal::Overflow)?,
            self.den.checked_mul(other.num).ok_or(RatRefusal::Overflow)?,
        )
    }

    /// The exact order of two rationals, by cross-multiplication.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::Overflow`] when a cross product leaves `i128`.
    pub fn try_cmp(self, other: Self) -> Result<Ordering, RatRefusal> {
        let a = self.num.checked_mul(other.den).ok_or(RatRefusal::Overflow)?;
        let b = other.num.checked_mul(self.den).ok_or(RatRefusal::Overflow)?;
        Ok(a.cmp(&b))
    }

    /// The largest `k / scale` not above `self` (`scale > 0`): an enclosure endpoint rounded down.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::ZeroDenominator`] for `scale ≤ 0`; [`RatRefusal::Overflow`] when a product leaves `i128`.
    pub fn floor_to_scale(self, scale: i128) -> Result<Self, RatRefusal> {
        if scale <= 0 {
            return Err(RatRefusal::ZeroDenominator);
        }
        let n = self.num.checked_mul(scale).ok_or(RatRefusal::Overflow)?;
        Self::new(n.div_euclid(self.den), scale)
    }

    /// The smallest `k / scale` not below `self` (`scale > 0`): an enclosure endpoint rounded up.
    ///
    /// # Errors
    ///
    /// [`RatRefusal::ZeroDenominator`] for `scale ≤ 0`; [`RatRefusal::Overflow`] when a product leaves `i128`.
    pub fn ceil_to_scale(self, scale: i128) -> Result<Self, RatRefusal> {
        if scale <= 0 {
            return Err(RatRefusal::ZeroDenominator);
        }
        let n = self.num.checked_mul(scale).ok_or(RatRefusal::Overflow)?;
        let q = n.div_euclid(self.den);
        let up = if n.rem_euclid(self.den) == 0 {
            q
        } else {
            q.checked_add(1).ok_or(RatRefusal::Overflow)?
        };
        Self::new(up, scale)
    }
}

/// Why the gate refuses a local gravity value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GravityRefusal {
    /// The fall height is not strictly positive (the lemma needs `h > 0`).
    NonPositiveHeight,
    /// `g` is below the lemma's lower bound `v² / (2·h)`: the fall would create free energy.
    BelowLemmaBound {
        /// The bound `v² / (2·h)` the value fell short of.
        bound: ExactRat,
    },
    /// An exact intermediate left `i128`; the gate decides nothing it cannot compute exactly.
    Overflow,
    /// The stated uncertainty `u` is not strictly positive.
    UncertaintyNotPositive,
    /// The stated lower bound is not the lemma's bound for the fall given.
    StatedBoundDiffers {
        /// The bound the record states.
        stated: ExactRat,
        /// The bound `v² / (2·h)` the lemma gives.
        lemma: ExactRat,
    },
    /// The normal gravity at the position lies wholly outside `[value − u, value + u]`.
    NormalGravityOutsideU {
        /// The normal-gravity interval at the position.
        gamma: RatInterval,
    },
    /// The position's uncertainty leaves the normal-gravity interval partly inside and partly outside
    /// `[value − u, value + u]`: the location is too coarse to decide.
    LocationTooCoarse {
        /// The normal-gravity interval at the position.
        gamma: RatInterval,
    },
    /// The position lies outside the domain of the normal-gravity series.
    PositionOutOfDomain,
}

impl From<RatRefusal> for GravityRefusal {
    fn from(_: RatRefusal) -> Self {
        // A zero denominator cannot arise from a positive `h`; both refusals mean "not computable exactly".
        Self::Overflow
    }
}

/// The lemma's lower bound on `g` for a body that falls from rest through `h` and reaches speed `v`: `v² / (2·h)`.
///
/// # Errors
///
/// [`GravityRefusal::NonPositiveHeight`] for `h ≤ 0`; [`GravityRefusal::Overflow`] when the bound is not computable.
pub fn local_gravity_lower_bound(h: ExactRat, v: ExactRat) -> Result<ExactRat, GravityRefusal> {
    if h.sign() != Ordering::Greater {
        return Err(GravityRefusal::NonPositiveHeight);
    }
    let v2 = v.checked_mul(v)?;
    let two_h = h.checked_mul(ExactRat::int(2))?;
    Ok(v2.checked_div(two_h)?)
}

/// The gate: `g` is admissible for a fall through `h` at speed `v` exactly when `h > 0` and `g ≥ v² / (2·h)`.
///
/// # Errors
///
/// [`GravityRefusal::NonPositiveHeight`] for `h ≤ 0`; [`GravityRefusal::BelowLemmaBound`] for `g < v² / (2·h)`;
/// [`GravityRefusal::Overflow`] when the comparison is not computable exactly.
pub fn local_gravity_admissible(h: ExactRat, v: ExactRat, g: ExactRat) -> Result<(), GravityRefusal> {
    let bound = local_gravity_lower_bound(h, v)?;
    match g.try_cmp(bound)? {
        Ordering::Less => Err(GravityRefusal::BelowLemmaBound { bound }),
        Ordering::Equal | Ordering::Greater => Ok(()),
    }
}

/// A location-bound value of `g`: the value, its standard uncertainty, the lemma's lower bound it was checked
/// against, the position it belongs to, the `ucrs` sequence number of its stamp and the receipt that records it.
/// The record carries no value of its own: instance data is filled by the instance that measured it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalGravity {
    /// The value of `g` [m s⁻²].
    pub value: ExactRat,
    /// Its standard uncertainty `u` [m s⁻²], strictly positive.
    pub u: ExactRat,
    /// The lemma's lower bound `v² / (2·h)` for the fall that supports it.
    pub lower_bound_from_lemma: ExactRat,
    /// Where the value holds.
    pub at: GeodeticPosition,
    /// The sequence number of the `ucrs` stamp of the observation.
    pub ucrs_seq: u64,
    /// The path of the receipt that records the measurement.
    pub receipt: String,
}

/// The combined gate: the scalar gate on `g = value` for a fall through `h` at speed `v`, the stated lemma bound
/// equal to `v² / (2·h)`, and the normal gravity at the position inside `[value − u, value + u]`.
///
/// # Errors
///
/// The refusals of [`local_gravity_admissible`]; [`GravityRefusal::UncertaintyNotPositive`],
/// [`GravityRefusal::StatedBoundDiffers`], [`GravityRefusal::PositionOutOfDomain`];
/// [`GravityRefusal::NormalGravityOutsideU`] when the normal-gravity interval and `[value − u, value + u]` are
/// disjoint; [`GravityRefusal::LocationTooCoarse`] when they overlap without containment.
pub fn local_gravity_admissible_at(lg: &LocalGravity, h: ExactRat, v: ExactRat) -> Result<(), GravityRefusal> {
    local_gravity_admissible(h, v, lg.value)?;
    if lg.u.sign() != Ordering::Greater {
        return Err(GravityRefusal::UncertaintyNotPositive);
    }
    let lemma = local_gravity_lower_bound(h, v)?;
    if lg.lower_bound_from_lemma != lemma {
        return Err(GravityRefusal::StatedBoundDiffers {
            stated: lg.lower_bound_from_lemma,
            lemma,
        });
    }
    let gamma = match normal_gravity::normal_gravity_interval(&lg.at) {
        Ok(g) => g,
        Err(normal_gravity::NormalGravityRefusal::HeightOutOfRange) => return Err(GravityRefusal::PositionOutOfDomain),
        Err(normal_gravity::NormalGravityRefusal::Interval(_)) => return Err(GravityRefusal::Overflow),
    };
    let band = RatInterval::new(lg.value.checked_sub(lg.u)?, lg.value.checked_add(lg.u)?)
        .map_err(|_| GravityRefusal::Overflow)?;
    if gamma.is_subset_of(band).map_err(|_| GravityRefusal::Overflow)? {
        Ok(())
    } else if gamma.is_disjoint_from(band).map_err(|_| GravityRefusal::Overflow)? {
        Err(GravityRefusal::NormalGravityOutsideU { gamma })
    } else {
        Err(GravityRefusal::LocationTooCoarse { gamma })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i128, d: i128) -> ExactRat {
        ExactRat::new(n, d).expect("test rational")
    }

    #[test]
    fn admits_g_at_or_above_the_bound() {
        // v² / (2h) = 25 / 2 = 12.5 for h = 1, v = 5.
        assert_eq!(local_gravity_lower_bound(r(1, 1), r(5, 1)), Ok(r(25, 2)));
        assert_eq!(local_gravity_admissible(r(1, 1), r(5, 1), r(25, 2)), Ok(()));
        assert_eq!(local_gravity_admissible(r(1, 1), r(5, 1), r(13, 1)), Ok(()));
    }

    #[test]
    fn refuses_g_below_the_bound() {
        assert_eq!(
            local_gravity_admissible(r(1, 1), r(5, 1), r(12, 1)),
            Err(GravityRefusal::BelowLemmaBound { bound: r(25, 2) })
        );
        // one part in 10^12 below the bound is still refused: the comparison is exact
        let just_below = r(25_000_000_000_000 - 1, 2_000_000_000_000);
        assert!(matches!(
            local_gravity_admissible(r(1, 1), r(5, 1), just_below),
            Err(GravityRefusal::BelowLemmaBound { .. })
        ));
    }

    #[test]
    fn refuses_a_height_that_is_not_positive() {
        assert_eq!(
            local_gravity_admissible(ExactRat::ZERO, r(5, 1), r(100, 1)),
            Err(GravityRefusal::NonPositiveHeight)
        );
        assert_eq!(
            local_gravity_admissible(r(-1, 2), r(5, 1), r(100, 1)),
            Err(GravityRefusal::NonPositiveHeight)
        );
    }

    #[test]
    fn refuses_what_it_cannot_compute_exactly() {
        let big = ExactRat::int(i128::MAX / 2);
        assert_eq!(
            local_gravity_admissible(r(1, 1), big, r(1, 1)),
            Err(GravityRefusal::Overflow)
        );
    }

    #[test]
    fn rationals_are_normalised_and_total() {
        assert_eq!(r(2, -4), r(-1, 2));
        assert_eq!(ExactRat::new(1, 0), Err(RatRefusal::ZeroDenominator));
        assert_eq!(r(7, 2).floor_to_scale(1), Ok(r(3, 1)));
        assert_eq!(r(7, 2).ceil_to_scale(1), Ok(r(4, 1)));
        assert_eq!(r(-7, 2).floor_to_scale(1), Ok(r(-4, 1)));
    }

    fn position(lat_lo: i128, lat_hi: i128, den: i128, height: i128) -> GeodeticPosition {
        let lat = RatInterval::new(r(lat_lo, den), r(lat_hi, den)).unwrap();
        let lon = RatInterval::point(ExactRat::ZERO);
        GeodeticPosition::new(lat, lon, RatInterval::point(ExactRat::int(height))).unwrap()
    }

    fn record(value: ExactRat, u: ExactRat, at: GeodeticPosition) -> LocalGravity {
        LocalGravity {
            value,
            u,
            lower_bound_from_lemma: local_gravity_lower_bound(r(1, 1), r(5, 1)).unwrap(),
            at,
            ucrs_seq: 0,
            receipt: String::from("synthetic"),
        }
    }

    #[test]
    fn normal_gravity_at_the_equator_and_the_pole_matches_the_tabulated_values() {
        // Synthetic locations only: gamma_e = 9.7803253359 and gamma_p = 9.8321849378 (TR8350.2).
        let tol = r(1, 1_000_000_000);
        let eq = normal_gravity::normal_gravity_interval(&position(0, 0, 1, 0)).unwrap();
        let ge = r(97_803_253_359, 10_000_000_000);
        assert!(eq.lo().try_cmp(ge.checked_add(tol).unwrap()).unwrap() == Ordering::Less);
        assert!(eq.hi().try_cmp(ge.checked_sub(tol).unwrap()).unwrap() == Ordering::Greater);
        let pole = normal_gravity::normal_gravity_interval(&position(90, 90, 1, 0)).unwrap();
        let gp = r(98_321_849_378, 10_000_000_000);
        assert!(pole.lo().try_cmp(gp.checked_add(tol).unwrap()).unwrap() == Ordering::Less);
        assert!(pole.hi().try_cmp(gp.checked_sub(tol).unwrap()).unwrap() == Ordering::Greater);
        // enclosure is tight: well under 1e-9 wide
        assert!(eq.hi().checked_sub(eq.lo()).unwrap().try_cmp(tol).unwrap() == Ordering::Less);
    }

    #[test]
    fn height_lowers_normal_gravity_by_the_free_air_gradient() {
        let ground = normal_gravity::normal_gravity_interval(&position(0, 0, 1, 0)).unwrap();
        let high = normal_gravity::normal_gravity_interval(&position(0, 0, 1, 1000)).unwrap();
        // about 3.0877e-3 m/s^2 per 1000 m at the equator
        let drop = ground.lo().checked_sub(high.hi()).unwrap();
        assert!(drop.try_cmp(r(3_000, 1_000_000)).unwrap() == Ordering::Greater);
        assert!(drop.try_cmp(r(3_200, 1_000_000)).unwrap() == Ordering::Less);
        assert_eq!(
            normal_gravity::normal_gravity_interval(&position(0, 0, 1, 200_000)),
            Err(normal_gravity::NormalGravityRefusal::HeightOutOfRange)
        );
    }

    #[test]
    fn the_combined_gate_admits_a_value_inside_u_at_a_sharp_position() {
        let at = position(0, 0, 1, 0);
        let lg = record(r(97_803_253_359, 10_000_000_000), r(1, 1000), at);
        // v = 5, h = 1 gives a lemma bound of 12.5 > g at the equator, so use a fall that the value supports.
        let lg = LocalGravity {
            lower_bound_from_lemma: local_gravity_lower_bound(r(1, 1), r(4, 1)).unwrap(),
            ..lg
        };
        assert_eq!(local_gravity_admissible_at(&lg, r(1, 1), r(4, 1)), Ok(()));
    }

    #[test]
    fn the_combined_gate_refuses_below_the_lemma_outside_u_and_too_coarse() {
        let at = position(0, 0, 1, 0);
        let g_e = r(97_803_253_359, 10_000_000_000);
        let ok_bound = local_gravity_lower_bound(r(1, 1), r(4, 1)).unwrap();
        // below the lemma bound: v = 5 needs g >= 12.5
        let below = LocalGravity {
            lower_bound_from_lemma: local_gravity_lower_bound(r(1, 1), r(5, 1)).unwrap(),
            ..record(g_e, r(1, 1000), at)
        };
        assert!(matches!(
            local_gravity_admissible_at(&below, r(1, 1), r(5, 1)),
            Err(GravityRefusal::BelowLemmaBound { .. })
        ));
        // gamma outside u: the stated value is 0.5 above normal gravity with u = 1e-3
        let far = LocalGravity {
            lower_bound_from_lemma: ok_bound,
            ..record(r(103_000_000_000, 10_000_000_000), r(1, 1000), at)
        };
        assert!(matches!(
            local_gravity_admissible_at(&far, r(1, 1), r(4, 1)),
            Err(GravityRefusal::NormalGravityOutsideU { .. })
        ));
        // too coarse: latitude known only to within the whole equator-to-pole span, u small
        let wide = position(0, 90, 1, 0);
        let coarse = LocalGravity {
            lower_bound_from_lemma: ok_bound,
            ..record(g_e, r(1, 1000), wide)
        };
        assert!(matches!(
            local_gravity_admissible_at(&coarse, r(1, 1), r(4, 1)),
            Err(GravityRefusal::LocationTooCoarse { .. })
        ));
        // a stated bound that is not the lemma's, and a non-positive uncertainty
        let off = record(g_e, r(1, 1000), at);
        assert!(matches!(
            local_gravity_admissible_at(&off, r(1, 1), r(4, 1)),
            Err(GravityRefusal::StatedBoundDiffers { .. })
        ));
        let zero_u = LocalGravity {
            lower_bound_from_lemma: ok_bound,
            ..record(g_e, ExactRat::ZERO, at)
        };
        assert_eq!(
            local_gravity_admissible_at(&zero_u, r(1, 1), r(4, 1)),
            Err(GravityRefusal::UncertaintyNotPositive)
        );
    }

    #[test]
    fn a_position_outside_the_coordinate_range_or_reversed_is_refused() {
        let beyond = RatInterval::new(r(0, 1), r(91, 1)).unwrap();
        let lon = RatInterval::point(ExactRat::ZERO);
        assert_eq!(
            GeodeticPosition::new(beyond, lon, lon),
            Err(geodetic::GeodeticRefusal::LatitudeOutOfRange)
        );
        assert_eq!(
            RatInterval::new(r(2, 1), r(1, 1)),
            Err(interval::IntervalRefusal::Reversed)
        );
    }

    /// The gate cites the lemma by the name `formal_parity.json` lists; renaming the lemma breaks this test.
    #[test]
    fn the_pin_names_a_statement_of_the_parity_contract() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../umst-formal/formal_parity.json");
        let Ok(text) = std::fs::read_to_string(path) else {
            return;
        };
        assert!(text.contains("\"free_fall.gravity_lower_bound\""));
        assert!(text.contains("\"UMST.FreeFall.freeFall_gravity_lower_bound\""));
    }
}

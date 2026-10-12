// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! WGS84 normal gravity γ(φ, h) as a rational interval: the Somigliana closed form on the ellipsoid and the free-air
//! height series, with sin²φ and the square root enclosed by Taylor and Newton bounds and the position's
//! uncertainty propagated into the result.
//!
//! Cited coefficients (exact rationals, the decimals the source tabulates): NIMA TR8350.2, *Department of Defense
//! World Geodetic System 1984*, 3rd edition (2000), chapter 3 (the defining and derived WGS84 parameters a, f, γ_e,
//! k, e², m) and chapter 4 (normal gravity: γ = γ_e (1 + k sin²φ) / √(1 − e² sin²φ), and its height series
//! γ_h = γ [1 − (2/a)(1 + f + m − 2f sin²φ) h + (3/a²) h²]). Their use is a cited form; registry rows in umst-formal
//! `constants/constants.json` are the follow-up cell FORMAL-NORMAL-GRAVITY-ROWS. The result is normal gravity: the
//! local anomaly of the real field is not in it and belongs in the stated uncertainty of a measured value.

use core::cmp::Ordering;

use super::geodetic::GeodeticPosition;
use super::interval::{IntervalRefusal, RatInterval, SCALE};
use super::{ExactRat, RatRefusal};

/// Heights beyond this lie outside the series' stated domain.
const MAX_ABS_HEIGHT_M: i128 = 100_000;

/// Why the normal gravity of a position is not computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalGravityRefusal {
    /// The ellipsoidal height interval leaves `[−100 km, 100 km]`.
    HeightOutOfRange,
    /// An interval operation was refused.
    Interval(IntervalRefusal),
}

impl From<IntervalRefusal> for NormalGravityRefusal {
    fn from(e: IntervalRefusal) -> Self {
        Self::Interval(e)
    }
}

impl From<RatRefusal> for NormalGravityRefusal {
    fn from(e: RatRefusal) -> Self {
        Self::Interval(e.into())
    }
}

fn r(n: i128, d: i128) -> Result<ExactRat, RatRefusal> {
    ExactRat::new(n, d)
}

/// An enclosure of π: 3.141592653589793 ≤ π ≤ 3.141592653589794.
fn pi() -> Result<RatInterval, NormalGravityRefusal> {
    Ok(RatInterval::new(
        r(3_141_592_653_589_793, 1_000_000_000_000_000)?,
        r(3_141_592_653_589_794, 1_000_000_000_000_000)?,
    )?)
}

/// `sin x` for a rational `x` with `|x| ≤ 1.6`: the Taylor sum to `x^25` plus the remainder bound
/// `|x|^27 / 27! < 2·10⁻²³`, enclosed by `±10⁻²⁰`.
fn sin_point(x: ExactRat) -> Result<RatInterval, NormalGravityRefusal> {
    let xi = RatInterval::point(x);
    let x2 = xi.square()?;
    let mut term = xi;
    let mut sum = xi;
    for k in 1..=12_i128 {
        term = term.mul(x2)?.mul_scalar(r(-1, (2 * k) * (2 * k + 1))?)?;
        sum = sum.add(term)?;
    }
    let rem = RatInterval::new(r(-1, 100_000_000_000_000_000_000)?, r(1, 100_000_000_000_000_000_000)?)?;
    Ok(sum.add(rem)?)
}

/// `sin` over an interval inside `[−π/2 − 10⁻¹², π/2 + 10⁻¹²]`, where it is increasing.
fn sin_interval(x: RatInterval) -> Result<RatInterval, NormalGravityRefusal> {
    Ok(RatInterval::new(sin_point(x.lo())?.lo(), sin_point(x.hi())?.hi())?)
}

/// An upper bound of `√y` for rational `y > 0` by Newton's iteration, which never falls below the root.
fn sqrt_upper(y: ExactRat) -> Result<ExactRat, NormalGravityRefusal> {
    let mut x = if y.try_cmp(ExactRat::ONE)? == Ordering::Greater {
        y
    } else {
        ExactRat::ONE
    };
    for _ in 0..12 {
        x = x
            .checked_add(y.checked_div(x)?)?
            .checked_div(ExactRat::int(2))?
            .ceil_to_scale(SCALE)?;
    }
    Ok(x)
}

/// An enclosure of `√y` for an interval strictly above zero: `[y_lo / U(y_lo), U(y_hi)]`.
fn sqrt_interval(y: RatInterval) -> Result<RatInterval, NormalGravityRefusal> {
    if y.lo().sign() != Ordering::Greater {
        return Err(IntervalRefusal::DivisorNotPositive.into());
    }
    let lower = y.lo().checked_div(sqrt_upper(y.lo())?)?.floor_to_scale(SCALE)?;
    Ok(RatInterval::new(lower, sqrt_upper(y.hi())?)?)
}

/// The normal gravity [m s⁻²] at a position, as an interval that contains the value of the WGS84 formula for every
/// latitude and height in the position's intervals.
///
/// # Errors
///
/// [`NormalGravityRefusal::HeightOutOfRange`] for a height beyond ±100 km; [`NormalGravityRefusal::Interval`] when
/// an exact intermediate is not representable.
pub fn normal_gravity_interval(p: &GeodeticPosition) -> Result<RatInterval, NormalGravityRefusal> {
    let h = p.height_m();
    let cap = ExactRat::int(MAX_ABS_HEIGHT_M);
    if h.lo().try_cmp(cap.checked_neg()?)? == Ordering::Less || h.hi().try_cmp(cap)? == Ordering::Greater {
        return Err(NormalGravityRefusal::HeightOutOfRange);
    }
    // TR8350.2 parameters, exact: a [m]; f, m, k, e² dimensionless; γ_e [m s⁻²].
    let a = ExactRat::int(6_378_137);
    let f = r(335_281_066_474, 100_000_000_000_000)?;
    let m = r(344_978_650_684, 100_000_000_000_000)?;
    let k = r(193_185_265_241, 100_000_000_000_000)?;
    let e2 = r(669_437_999_014, 100_000_000_000_000)?;
    let gamma_e = r(97_803_253_359, 10_000_000_000)?;

    let phi = p.lat_deg().mul(pi()?)?.mul_scalar(r(1, 180)?)?;
    let s = sin_interval(phi)?.square()?;
    let one = RatInterval::point(ExactRat::ONE);
    let y = one.sub(s.mul_scalar(e2)?)?;
    let gamma0 = one.add(s.mul_scalar(k)?)?.mul_scalar(gamma_e)?.div(sqrt_interval(y)?)?;
    let coef = RatInterval::point(ExactRat::ONE.checked_add(f)?.checked_add(m)?)
        .sub(s.mul_scalar(f.checked_mul(ExactRat::int(2))?)?)?;
    let linear = coef.mul(h)?.mul_scalar(ExactRat::int(2).checked_div(a)?)?;
    let quad = h
        .square()?
        .mul_scalar(ExactRat::int(3).checked_div(a.checked_mul(a)?)?)?;
    let factor = one.sub(linear)?.add(quad)?;
    Ok(gamma0.mul(factor)?)
}

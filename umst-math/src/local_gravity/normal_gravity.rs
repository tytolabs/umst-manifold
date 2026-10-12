// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Normal gravity γ(φ, h) of a rotating ellipsoid as a rational interval, computed from its four defining
//! parameters (a, 1/f, GM, ω) and nothing else: the Somigliana closed form γ = γ_e (1 + k sin²φ) / √(1 − e² sin²φ)
//! and the free-air height series γ_h = γ [1 − (2/a)(1 + f + m − 2f sin²φ) h + (3/a²) h²].
//!
//! Derived exactly from the defining parameters: f = 1/(1/f), e² = f(2 − f), b = a(1 − f), e'² = e²/(1 − e²) and,
//! as outward-rounded intervals, m = ω²a²b/GM. The transcendental part needs no `arctan` and no held decimal: with
//! t = e'² the Somigliana functions are q₀ = e'³·A(t) and q₀' = e'²·B(t), where
//! A(t) = 2 Σ_{j≥1} (−1)^{j+1} j t^{j−1} / ((2j+1)(2j+3)) and B(t) = 6 Σ_{i≥1} (−1)^{i+1} t^{i−1} / ((2i+1)(2i+3)),
//! obtained from the series of arctan e' with the terms that cancel removed; then e'·q₀'/q₀ = B/A,
//! γ_e = GM/(ab) · (1 − m − m·B/(6A)), γ_p = GM/a² · (1 + m·B/(3A)) and k = bγ_p/(aγ_e) − 1 (Somigliana's relations,
//! Hofmann-Wellenhof and Moritz, *Physical Geodesy*, 2nd ed., 2006, ch. 2). Each series is cut after `TERMS` terms
//! with the remainder ≤ 2·t^TERMS (every coefficient is at most 1 and t ≤ 1/2), enclosed in the result. sin²φ and
//! the square root are enclosed by Taylor and Newton bounds, and the position's uncertainty propagates.
//!
//! The four defining parameters of WGS84 are the cited values of Table 3.1 of the standard ([`wgs84_defining`]).
//! The formulas are those of the standard's equations (4-1) and (4-3), quoted at [`wgs84_defining`]. The result is normal gravity: the local anomaly of the
//! real field is not in it and belongs in the stated uncertainty of a measured value.

use core::cmp::Ordering;

use super::geodetic::GeodeticPosition;
use super::interval::{IntervalRefusal, RatInterval, SCALE};
use super::{ExactRat, RatRefusal};

/// Heights beyond this lie outside the series' stated domain.
const MAX_ABS_HEIGHT_M: i128 = 100_000;

/// Terms kept in the series A(t) and B(t); the remainder is ≤ 2·t^TERMS for t ≤ 1/2.
const TERMS: i128 = 12;

/// The four defining parameters of a rotating ellipsoid of reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wgs84Defining {
    /// Semi-major axis a [m].
    pub a_m: ExactRat,
    /// Inverse flattening 1/f.
    pub inv_f: ExactRat,
    /// Geocentric gravitational constant GM [m³ s⁻²].
    pub gm_m3_s2: ExactRat,
    /// Angular velocity ω [rad s⁻¹].
    pub omega_rad_s: ExactRat,
}

/// The WGS84 defining parameters, `Cited`: NGA.STND.0036_1.0.0_WGS84 (2014), Table 3.1, "WGS 84 Defining
/// Parameters": semi-major axis a = 6378137.0 m; flattening factor of the Earth 1/f = 298.257223563; geocentric
/// gravitational constant GM = 3.986004418 × 10+14 m³/s²; nominal mean angular velocity of the Earth
/// ω = 7.292115 × 10−05 rad/s. Each is the exact rational of the printed decimal.
///
/// The formulas below are the standard's own. Equation (4-1), Somigliana: γ = γ_e (1 + k sin²φ) / √(1 − e² sin²φ)
/// with k = bγ_p/(aγ_e) − 1. Equation (4-3): γ_h = γ [1 − (2/a)(1 + f + m − 2f sin²φ) h + (3/a²) h²] with
/// m = ω²a²b/GM, h positive upward along the geodetic normal.
pub fn wgs84_defining() -> Option<Wgs84Defining> {
    // The proof table is the one source: each defining parameter is a definition row of umst-formal's
    // constants.json (NGA2014), read here as its exact rational and checked against the row's named f64 pin
    // (relative agreement within 1e-15, the f64 rounding of the exact value); a missing or disagreeing row
    // gives None, and the combined gate reports the normal-gravity check as absent.
    let exact = |id: &str, pin: f64| -> Option<ExactRat> {
        let row = umst_constants::ROWS.iter().find(|r| r.id == id)?;
        let q = ExactRat::new(row.exact_num.parse().ok()?, row.exact_den.parse().ok()?).ok()?;
        let approx = q.num() as f64 / q.den() as f64;
        ((approx - pin).abs() <= pin.abs() * 1e-15).then_some(q)
    };
    Some(Wgs84Defining {
        a_m: exact("wgs84SemiMajorAxis", umst_constants::WGS84SEMI_MAJOR_AXIS)?,
        inv_f: exact(
            "wgs84InverseFlattening",
            umst_constants::WGS84INVERSE_FLATTENING,
        )?,
        gm_m3_s2: exact("wgs84GM", umst_constants::WGS84GM)?,
        omega_rad_s: exact(
            "wgs84AngularVelocity",
            umst_constants::WGS84ANGULAR_VELOCITY,
        )?,
    })
}

/// Why the normal gravity of a position is not computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalGravityRefusal {
    /// The ellipsoidal height interval leaves `[−100 km, 100 km]`.
    HeightOutOfRange,
    /// An interval operation was refused.
    Interval(IntervalRefusal),
    /// The defining parameters are not an ellipsoid: need a > 0, 1/f ≥ 2, GM > 0, ω ≥ 0.
    InvalidDefining,
    /// A derivation step is not available for these parameters (the series needs e'² ≤ 1/2).
    DerivationAbsent {
        /// The cell that will supply it.
        follow_up: &'static str,
    },
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

fn gt(x: ExactRat, y: ExactRat) -> Result<bool, RatRefusal> {
    Ok(x.try_cmp(y)? == Ordering::Greater)
}

fn check_defining(d: &Wgs84Defining) -> Result<(), NormalGravityRefusal> {
    let ok = gt(d.a_m, ExactRat::ZERO)?
        && !gt(ExactRat::int(2), d.inv_f)?
        && gt(d.gm_m3_s2, ExactRat::ZERO)?
        && !gt(ExactRat::ZERO, d.omega_rad_s)?;
    if ok {
        Ok(())
    } else {
        Err(NormalGravityRefusal::InvalidDefining)
    }
}

/// The flattening f = 1 / (1/f), exact.
///
/// # Errors
///
/// [`NormalGravityRefusal::InvalidDefining`] unless the parameters describe an ellipsoid.
pub fn flattening(d: &Wgs84Defining) -> Result<ExactRat, NormalGravityRefusal> {
    check_defining(d)?;
    Ok(ExactRat::ONE.checked_div(d.inv_f)?)
}

/// The first eccentricity squared e² = f(2 − f), exact.
///
/// # Errors
///
/// As [`flattening`].
pub fn first_eccentricity_squared(d: &Wgs84Defining) -> Result<ExactRat, NormalGravityRefusal> {
    let f = flattening(d)?;
    Ok(f.checked_mul(ExactRat::int(2).checked_sub(f)?)?)
}

/// The semi-minor axis b = a(1 − f), exact.
///
/// # Errors
///
/// As [`flattening`].
pub fn semi_minor_axis(d: &Wgs84Defining) -> Result<ExactRat, NormalGravityRefusal> {
    let f = flattening(d)?;
    Ok(d.a_m.checked_mul(ExactRat::ONE.checked_sub(f)?)?)
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
    let rem = RatInterval::new(
        r(-1, 100_000_000_000_000_000_000)?,
        r(1, 100_000_000_000_000_000_000)?,
    )?;
    Ok(sum.add(rem)?)
}

/// `sin` over an interval inside `[−π/2 − 10⁻¹², π/2 + 10⁻¹²]`, where it is increasing.
fn sin_interval(x: RatInterval) -> Result<RatInterval, NormalGravityRefusal> {
    Ok(RatInterval::new(
        sin_point(x.lo())?.lo(),
        sin_point(x.hi())?.hi(),
    )?)
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
    let lower = y
        .lo()
        .checked_div(sqrt_upper(y.lo())?)?
        .floor_to_scale(SCALE)?;
    Ok(RatInterval::new(lower, sqrt_upper(y.hi())?)?)
}

/// The factor `10^MICRO` that keeps the small derived constants (e², k, m of order 10⁻³) clear of the `1e-12` grid
/// of [`RatInterval`]: they are carried as `MICRO_SCALE · x` and divided out only by exact scalars.
const MICRO: u32 = 6;
const MICRO_SCALE: i128 = 1_000_000;

/// An enclosure of `10^k · x` on the `1e-12` grid for a rational `x ≥ 0`, by long division (the exact product
/// `num · 10^(12+k)` would leave `i128` for the standard's magnitudes).
fn scaled_enclosure(x: ExactRat, k: u32) -> Result<RatInterval, NormalGravityRefusal> {
    let overflow = || NormalGravityRefusal::Interval(IntervalRefusal::Overflow);
    if x.sign() == Ordering::Less {
        return Err(overflow());
    }
    let (num, den) = (x.num().unsigned_abs(), x.den().unsigned_abs());
    if den > u128::MAX / 10 {
        return Err(overflow());
    }
    let mut whole = num / den;
    let mut rem = num % den;
    for _ in 0..(12 + k) {
        rem *= 10;
        whole = whole
            .checked_mul(10)
            .and_then(|w| w.checked_add(rem / den))
            .ok_or_else(overflow)?;
        rem %= den;
    }
    let lo = i128::try_from(whole).map_err(|_| overflow())?;
    let hi = lo.checked_add(i128::from(rem != 0)).ok_or_else(overflow)?;
    Ok(RatInterval::new(r(lo, SCALE)?, r(hi, SCALE)?)?)
}

/// Enclosures of `MICRO_SCALE · A(t)` and `MICRO_SCALE · B(t)` for `0 ≤ t ≤ 1/2`: Horner in `−t` over `TERMS`
/// terms, plus `±2·MICRO_SCALE·t^TERMS`. `t` enters as `MICRO_SCALE · t` on the interval grid, so its rounding is
/// 10⁻⁶ of the grid's.
fn scaled_series(t: ExactRat) -> Result<(RatInterval, RatInterval), NormalGravityRefusal> {
    let absent = NormalGravityRefusal::DerivationAbsent {
        follow_up: "FORMAL-NORMAL-GRAVITY-DERIVED",
    };
    if t.sign() == Ordering::Less || gt(t, r(1, 2)?)? {
        return Err(absent);
    }
    let tau = scaled_enclosure(t, MICRO)?;
    let neg_inv = r(-1, MICRO_SCALE)?;
    let horner = |coef: &dyn Fn(i128) -> Result<ExactRat, RatRefusal>| -> Result<RatInterval, NormalGravityRefusal> {
        let scaled = |j: i128| -> Result<ExactRat, RatRefusal> { coef(j)?.checked_mul(ExactRat::int(MICRO_SCALE)) };
        let mut acc = RatInterval::point(scaled(TERMS)?);
        for j in (1..TERMS).rev() {
            acc = RatInterval::point(scaled(j)?).add(acc.mul(tau)?.mul_scalar(neg_inv)?)?;
        }
        Ok(acc)
    };
    let a_coef = |j: i128| r(2 * j, (2 * j + 1) * (2 * j + 3));
    let b_coef = |i: i128| r(6, (2 * i + 1) * (2 * i + 3));
    // t ≤ t_up = ceil(1000 t) / 1000, so t^TERMS ≤ t_up^TERMS
    let t_up = t
        .checked_mul(ExactRat::int(1000))?
        .ceil_to_scale(1)?
        .checked_div(ExactRat::int(1000))?;
    let mut pow = ExactRat::ONE;
    for _ in 0..TERMS {
        pow = pow.checked_mul(t_up)?;
    }
    let bound = scaled_enclosure(pow, MICRO)?
        .hi()
        .checked_mul(ExactRat::int(2))?;
    let rem = RatInterval::new(bound.checked_neg()?, bound)?;
    Ok((horner(&a_coef)?.add(rem)?, horner(&b_coef)?.add(rem)?))
}

/// Enclosures of A(t) and B(t) for `0 ≤ t ≤ 1/2`.
///
/// # Errors
///
/// [`NormalGravityRefusal::DerivationAbsent`] for `t` outside `[0, 1/2]`.
pub fn somigliana_series(t: ExactRat) -> Result<(RatInterval, RatInterval), NormalGravityRefusal> {
    let (a, b) = scaled_series(t)?;
    let inv = r(1, MICRO_SCALE)?;
    Ok((a.mul_scalar(inv)?, b.mul_scalar(inv)?))
}

/// The constants derived from the four defining parameters, as enclosures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DerivedConstants {
    /// Normal gravity at the equator γ_e [m s⁻²].
    pub gamma_e: RatInterval,
    /// Normal gravity at the pole γ_p [m s⁻²].
    pub gamma_p: RatInterval,
    /// Somigliana's constant k = bγ_p/(aγ_e) − 1.
    pub k: RatInterval,
    /// m = ω²a²b/GM.
    pub m: RatInterval,
    /// `10⁶ · e²`, enclosed finer than the `1e-12` grid allows for e² itself.
    pub e2_micro: RatInterval,
    /// `10⁶ · k`.
    pub k_micro: RatInterval,
    /// `10⁶ · m`.
    pub m_micro: RatInterval,
}

/// The derived constants of the ellipsoid of `d` as enclosures: m = ω²a²b/GM and Somigliana's relations through
/// the series A(t), B(t) (see the module text). With R = B/A and S = 10⁶, the exact algebra
/// bγ_p/(aγ_e) = (b/a)²(1 + mR/3)/(1 − m − mR/6) gives k = [−e² + (1 − e²)·mR/3 + m + mR/6]/(1 − m − mR/6), whose
/// numerator carries only terms of order 10⁻³. Every small term is held as S·x and divided out by an exact
/// scalar, so the 10⁻¹² grid of the intervals costs 10⁻¹⁸ in the result.
///
/// # Errors
///
/// [`NormalGravityRefusal::InvalidDefining`] unless the parameters describe an ellipsoid;
/// [`NormalGravityRefusal::DerivationAbsent`] when e'² > 1/2; [`NormalGravityRefusal::Interval`] when an exact
/// intermediate is not representable.
pub fn derived_constants(d: &Wgs84Defining) -> Result<DerivedConstants, NormalGravityRefusal> {
    let f = flattening(d)?;
    let e2 = first_eccentricity_squared(d)?;
    let b = semi_minor_axis(d)?;
    let a = d.a_m;
    let inv_s = r(1, MICRO_SCALE)?;
    // e'² = e²/(1 − e²) = n/(den − n) for e² = n/den
    let t = ExactRat::new(
        e2.num(),
        e2.den().checked_sub(e2.num()).ok_or(RatRefusal::Overflow)?,
    )?;
    let (ser_a, ser_b) = scaled_series(t)?;
    let ratio_micro = ser_b.mul_scalar(ExactRat::int(MICRO_SCALE))?.div(ser_a)?; // S · e'·q₀'/q₀
                                                                                 // m = (ω²a³/GM)(1 − f)
    let c = d
        .omega_rad_s
        .checked_mul(d.omega_rad_s)?
        .checked_mul(a)?
        .checked_mul(a)?
        .checked_mul(a)?
        .checked_div(d.gm_m3_s2)?;
    let m_micro = scaled_enclosure(c, MICRO)?.mul_scalar(ExactRat::ONE.checked_sub(f)?)?;
    let mr_micro = m_micro.mul(ratio_micro)?.mul_scalar(inv_s)?;
    let e2_micro = scaled_enclosure(e2, MICRO)?;
    let third = r(1, 3)?;
    let sixth = r(1, 6)?;
    let delta_micro = m_micro.add(mr_micro.mul_scalar(sixth)?)?; // S(m + mR/6)
    let numerator_micro = mr_micro
        .mul_scalar(third)?
        .sub(
            e2_micro
                .mul(mr_micro)?
                .mul_scalar(inv_s)?
                .mul_scalar(third)?,
        )?
        .add(delta_micro)?
        .sub(e2_micro)?;
    // 1/(1 − δ) = 1 + δ/(1 − δ), so k·S = N + N·(Sδ)/(S − Sδ) without dividing by a number rounded near 1
    let k_micro = numerator_micro.add(
        numerator_micro
            .mul(delta_micro)?
            .div(RatInterval::point(ExactRat::int(MICRO_SCALE)).sub(delta_micro)?)?,
    )?;
    let g = scaled_enclosure(d.gm_m3_s2.checked_div(a.checked_mul(b)?)?, 0)?;
    let p = scaled_enclosure(d.gm_m3_s2.checked_div(a.checked_mul(a)?)?, 0)?;
    let gamma_e = g.sub(g.mul(delta_micro)?.mul_scalar(inv_s)?)?;
    let gamma_p = p.add(p.mul(mr_micro)?.mul_scalar(inv_s.checked_mul(third)?)?)?;
    Ok(DerivedConstants {
        gamma_e,
        gamma_p,
        k: k_micro.mul_scalar(inv_s)?,
        m: m_micro.mul_scalar(inv_s)?,
        e2_micro,
        k_micro,
        m_micro,
    })
}

/// The normal gravity [m s⁻²] at a position on the ellipsoid of `d`, as an interval that contains the value of the
/// closed form for every latitude and height in the position's intervals.
///
/// # Errors
///
/// [`NormalGravityRefusal::HeightOutOfRange`] for a height beyond ±100 km; [`NormalGravityRefusal::InvalidDefining`];
/// [`NormalGravityRefusal::DerivationAbsent`] when e'² > 1/2; [`NormalGravityRefusal::Interval`] when an exact
/// intermediate is not representable.
pub fn normal_gravity_interval(
    p: &GeodeticPosition,
    d: &Wgs84Defining,
) -> Result<RatInterval, NormalGravityRefusal> {
    let h = p.height_m();
    let cap = ExactRat::int(MAX_ABS_HEIGHT_M);
    if h.lo().try_cmp(cap.checked_neg()?)? == Ordering::Less
        || h.hi().try_cmp(cap)? == Ordering::Greater
    {
        return Err(NormalGravityRefusal::HeightOutOfRange);
    }
    let f = flattening(d)?;
    let e2 = first_eccentricity_squared(d)?;
    let a = d.a_m;
    let one = RatInterval::point(ExactRat::ONE);
    let DerivedConstants { gamma_e, k, m, .. } = derived_constants(d)?;
    let phi = p.lat_deg().mul(pi()?)?.mul_scalar(r(1, 180)?)?;
    let s = sin_interval(phi)?.square()?;
    let y = one.sub(s.mul_scalar(e2)?)?;
    let gamma0 = gamma_e.mul(one.add(s.mul(k)?)?)?.div(sqrt_interval(y)?)?;
    let coef = one
        .add(RatInterval::point(f))?
        .add(m)?
        .sub(s.mul_scalar(f.checked_mul(ExactRat::int(2))?)?)?;
    let linear = coef.mul(h)?.mul_scalar(ExactRat::int(2).checked_div(a)?)?;
    let quad = h
        .square()?
        .mul_scalar(ExactRat::int(3).checked_div(a.checked_mul(a)?)?)?;
    let factor = one.sub(linear)?.add(quad)?;
    Ok(gamma0.mul(factor)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rr(n: i128, d: i128) -> ExactRat {
        ExactRat::new(n, d).unwrap()
    }

    /// A synthetic ellipsoid (round numbers, not a standard's values).
    fn synthetic(omega: ExactRat) -> Wgs84Defining {
        Wgs84Defining {
            a_m: ExactRat::int(6_000_000),
            inv_f: ExactRat::int(300),
            gm_m3_s2: ExactRat::int(400_000_000_000_000),
            omega_rad_s: omega,
        }
    }

    fn at(lat: i128, height: i128) -> GeodeticPosition {
        let lat = RatInterval::point(ExactRat::int(lat));
        GeodeticPosition::new(
            lat,
            RatInterval::point(ExactRat::ZERO),
            RatInterval::point(ExactRat::int(height)),
        )
        .unwrap()
    }

    fn dec(n: i128, exp10: u32) -> ExactRat {
        ExactRat::new(n, 10_i128.pow(exp10)).unwrap()
    }

    #[test]
    fn the_defining_parameters_are_the_printed_decimals_of_table_3_1() {
        let d = wgs84_defining().unwrap();
        assert_eq!(d.a_m, dec(63_781_370, 1));
        assert_eq!(d.inv_f, dec(298_257_223_563, 9));
        assert_eq!(
            d.gm_m3_s2,
            dec(3_986_004_418, 0)
                .checked_mul(ExactRat::int(100_000))
                .unwrap()
        );
        assert_eq!(d.omega_rad_s, dec(7_292_115, 11));
    }

    /// A table value is printed rounded to its last digit, so the derived interval must lie within half a unit
    /// of that digit around the printed value (it rounds to the printed value) and be narrower than half a unit.
    fn assert_oracle(name: &str, iv: RatInterval, printed: ExactRat, last_digit: ExactRat) {
        let half = last_digit.checked_div(ExactRat::int(2)).unwrap();
        assert_ne!(
            iv.lo().try_cmp(printed.checked_sub(half).unwrap()).unwrap(),
            Ordering::Less,
            "{name} lo"
        );
        assert_ne!(
            iv.hi().try_cmp(printed.checked_add(half).unwrap()).unwrap(),
            Ordering::Greater,
            "{name} hi"
        );
        let width = iv.hi().checked_sub(iv.lo()).unwrap();
        assert_eq!(width.try_cmp(half).unwrap(), Ordering::Less, "{name} width");
    }

    #[test]
    fn the_derived_constants_round_to_the_printed_table_values() {
        let d = wgs84_defining().unwrap();
        let c = derived_constants(&d).unwrap();
        let micro = ExactRat::int(MICRO_SCALE);
        let scaled = |x: ExactRat| x.checked_mul(micro).unwrap();
        // Table 3.5: e² = 6.694379990141 × 10^-3; Table 3.6: γe = 9.7803253359, γp = 9.8321849379,
        // k = 1.931852652458 × 10^-3, m = 3.449786506841 × 10^-3
        assert_oracle(
            "e2",
            c.e2_micro,
            scaled(dec(6_694_379_990_141, 15)),
            scaled(dec(1, 15)),
        );
        assert_oracle("gamma_e", c.gamma_e, dec(97_803_253_359, 10), dec(1, 10));
        assert_oracle("gamma_p", c.gamma_p, dec(98_321_849_379, 10), dec(1, 10));
        assert_oracle(
            "k",
            c.k_micro,
            scaled(dec(1_931_852_652_458, 15)),
            scaled(dec(1, 15)),
        );
        assert_oracle(
            "m",
            c.m_micro,
            scaled(dec(3_449_786_506_841, 15)),
            scaled(dec(1, 15)),
        );
    }

    #[test]
    fn the_equator_and_pole_of_the_closed_form_are_the_printed_normal_gravities() {
        let d = wgs84_defining().unwrap();
        let eq = normal_gravity_interval(&at(0, 0), &d).unwrap();
        let pole = normal_gravity_interval(&at(90, 0), &d).unwrap();
        // the closed form adds the sin² and √ enclosures: its width is held to a unit of 10⁻⁸
        assert_oracle("eq", eq, dec(97_803_253_359, 10), dec(1, 8));
        assert_oracle("pole", pole, dec(98_321_849_379, 10), dec(1, 8));
    }

    #[test]
    fn algebraic_derivations_match_hand_computed_rationals() {
        // a = 1, 1/f = 4: f = 1/4, e² = (1/4)(7/4) = 7/16, b = 3/4.
        let d = Wgs84Defining {
            a_m: ExactRat::ONE,
            inv_f: ExactRat::int(4),
            gm_m3_s2: ExactRat::ONE,
            omega_rad_s: ExactRat::ZERO,
        };
        assert_eq!(flattening(&d), Ok(rr(1, 4)));
        assert_eq!(first_eccentricity_squared(&d), Ok(rr(7, 16)));
        assert_eq!(semi_minor_axis(&d), Ok(rr(3, 4)));
        // e'² = e²/(1 − e²) = 7/9 > 1/2: the series is not available there, a typed absence
        assert_eq!(
            normal_gravity_interval(&at(0, 0), &d),
            Err(NormalGravityRefusal::DerivationAbsent {
                follow_up: "FORMAL-NORMAL-GRAVITY-DERIVED"
            })
        );
        // a ≤ 0, 1/f < 2 and ω < 0 are not ellipsoids
        assert_eq!(
            flattening(&Wgs84Defining {
                inv_f: ExactRat::ONE,
                ..d
            }),
            Err(NormalGravityRefusal::InvalidDefining)
        );
        assert_eq!(
            semi_minor_axis(&Wgs84Defining {
                a_m: ExactRat::ZERO,
                ..d
            }),
            Err(NormalGravityRefusal::InvalidDefining)
        );
    }

    #[test]
    fn the_series_at_zero_is_the_sphere_limit() {
        // A(0) = 2/15 and B(0) = 6/15: the leading coefficients, so B/A = 3.
        let (a, b) = somigliana_series(ExactRat::ZERO).unwrap();
        assert!(a
            .is_subset_of(
                RatInterval::new(
                    rr(2, 15).checked_sub(rr(1, 1_000_000_000)).unwrap(),
                    rr(2, 15).checked_add(rr(1, 1_000_000_000)).unwrap()
                )
                .unwrap()
            )
            .unwrap());
        assert!(b
            .is_subset_of(
                RatInterval::new(
                    rr(2, 5).checked_sub(rr(1, 1_000_000_000)).unwrap(),
                    rr(2, 5).checked_add(rr(1, 1_000_000_000)).unwrap()
                )
                .unwrap()
            )
            .unwrap());
        assert!(
            a.lo().try_cmp(rr(2, 15)).unwrap() != Ordering::Greater
                && a.hi().try_cmp(rr(2, 15)).unwrap() != Ordering::Less
        );
        assert_eq!(
            somigliana_series(rr(2, 3)),
            Err(NormalGravityRefusal::DerivationAbsent {
                follow_up: "FORMAL-NORMAL-GRAVITY-DERIVED"
            })
        );
    }

    #[test]
    fn without_rotation_equator_and_pole_are_gm_over_ab_and_gm_over_a_squared() {
        // ω = 0 gives m = 0, γ_e = GM/(ab) and γ_p = GM/a² exactly.
        let d = synthetic(ExactRat::ZERO);
        let (a, b, gm) = (d.a_m, semi_minor_axis(&d).unwrap(), d.gm_m3_s2);
        let ge = gm.checked_div(a.checked_mul(b).unwrap()).unwrap();
        let gp = gm.checked_div(a.checked_mul(a).unwrap()).unwrap();
        let eq = normal_gravity_interval(&at(0, 0), &d).unwrap();
        let pole = normal_gravity_interval(&at(90, 0), &d).unwrap();
        let tol = rr(1, 100_000_000);
        for (iv, v) in [(eq, ge), (pole, gp)] {
            assert_ne!(iv.lo().try_cmp(v).unwrap(), Ordering::Greater);
            assert_ne!(iv.hi().try_cmp(v).unwrap(), Ordering::Less);
            assert_eq!(
                iv.hi().checked_sub(iv.lo()).unwrap().try_cmp(tol).unwrap(),
                Ordering::Less
            );
        }
    }

    #[test]
    fn normal_gravity_rises_with_latitude_and_falls_with_height() {
        let d = synthetic(rr(7, 100_000));
        let mut prev = normal_gravity_interval(&at(0, 0), &d).unwrap();
        for lat in [30, 60, 90] {
            let next = normal_gravity_interval(&at(lat, 0), &d).unwrap();
            assert_eq!(
                next.lo().try_cmp(prev.hi()).unwrap(),
                Ordering::Greater,
                "lat {lat}"
            );
            prev = next;
        }
        let ground = normal_gravity_interval(&at(45, 0), &d).unwrap();
        let high = normal_gravity_interval(&at(45, 1000), &d).unwrap();
        assert_eq!(ground.lo().try_cmp(high.hi()).unwrap(), Ordering::Greater);
        assert_eq!(
            normal_gravity_interval(&at(0, 200_000), &d),
            Err(NormalGravityRefusal::HeightOutOfRange)
        );
    }
}

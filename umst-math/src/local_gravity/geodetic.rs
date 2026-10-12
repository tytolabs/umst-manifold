// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! A WGS84 geodetic position as exact rational intervals: the position and its uncertainty in one value.

use core::cmp::Ordering;

use super::interval::{IntervalRefusal, RatInterval};
use super::ExactRat;

/// Why a triple of intervals is not a geodetic position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeodeticRefusal {
    /// The latitude interval leaves `[−90°, 90°]`.
    LatitudeOutOfRange,
    /// The longitude interval leaves `[−180°, 180°]`.
    LongitudeOutOfRange,
    /// An interval could not be formed or compared (reversed or not representable).
    Interval(IntervalRefusal),
}

impl From<IntervalRefusal> for GeodeticRefusal {
    fn from(e: IntervalRefusal) -> Self {
        Self::Interval(e)
    }
}

impl From<super::RatRefusal> for GeodeticRefusal {
    fn from(e: super::RatRefusal) -> Self {
        Self::Interval(e.into())
    }
}

/// Latitude and longitude in degrees and ellipsoidal height in metres, each an interval that contains the true value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeodeticPosition {
    lat_deg: RatInterval,
    lon_deg: RatInterval,
    height_m: RatInterval,
}

impl GeodeticPosition {
    /// The position with the given intervals.
    ///
    /// # Errors
    ///
    /// [`GeodeticRefusal::LatitudeOutOfRange`] or [`GeodeticRefusal::LongitudeOutOfRange`] for an interval outside
    /// the ellipsoid's coordinate range; [`GeodeticRefusal::Interval`] when a comparison is not representable.
    pub fn new(
        lat_deg: RatInterval,
        lon_deg: RatInterval,
        height_m: RatInterval,
    ) -> Result<Self, GeodeticRefusal> {
        let (lat_lim, lon_lim) = (ExactRat::int(90), ExactRat::int(180));
        if lat_deg.lo().try_cmp(lat_lim.checked_neg()?)? == Ordering::Less
            || lat_deg.hi().try_cmp(lat_lim)? == Ordering::Greater
        {
            return Err(GeodeticRefusal::LatitudeOutOfRange);
        }
        if lon_deg.lo().try_cmp(lon_lim.checked_neg()?)? == Ordering::Less
            || lon_deg.hi().try_cmp(lon_lim)? == Ordering::Greater
        {
            return Err(GeodeticRefusal::LongitudeOutOfRange);
        }
        Ok(Self {
            lat_deg,
            lon_deg,
            height_m,
        })
    }

    /// Latitude [deg].
    #[must_use]
    pub const fn lat_deg(&self) -> RatInterval {
        self.lat_deg
    }

    /// Longitude [deg].
    #[must_use]
    pub const fn lon_deg(&self) -> RatInterval {
        self.lon_deg
    }

    /// Ellipsoidal height [m].
    #[must_use]
    pub const fn height_m(&self) -> RatInterval {
        self.height_m
    }
}

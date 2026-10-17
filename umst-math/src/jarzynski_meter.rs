// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Integral-fluctuation meter: the alarm a finite work ensemble raises against a known free-energy change,
//! and the tally of its false alarms.
//!
//! The integral fluctuation theorem `⟨e^{−w}⟩ = e^{−ΔF}` (`umst-formal` `Lean/FluctuationTheorem.lean`,
//! `Protocol.integral_fluctuation`) fixes the mean of `e^{−w}` for every protocol started in equilibrium. A
//! work ensemble whose Hoeffding interval ([`crate::jarzynski::hoeffding_interval`]) excludes a known ΔF
//! contradicts the theorem at level δ, or the ensemble does not come from such a protocol. By Hoeffding's
//! inequality (W. Hoeffding, J. Am. Stat. Assoc. 58, 13, 1963) an ensemble that does satisfy the theorem raises
//! the alarm with probability at most δ; [`FalseAlarmTally`] counts the alarms over replicates of such an
//! ensemble, so the stated level is itself measured.
//!
//! The level δ is an exact ratio of integers; the comparison of a tally against it is integer arithmetic.
//! `f64` enters at two edges only: the works and ΔF, whose conversion error the caller states in
//! `delta_f_error_bound`, and the binomial slack of [`FalseAlarmTally::within_level`].

use crate::jarzynski::{estimate, hoeffding_interval, FreeEnergyInterval, JarzynskiRefusal};

/// The confidence level δ of an alarm as the exact ratio `num / den`, with `0 < num < den`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level {
    num: u64,
    den: u64,
}

impl Level {
    /// δ = `num / den`; refused unless `0 < num < den`.
    pub fn new(num: u64, den: u64) -> Result<Self, JarzynskiRefusal> {
        if num == 0 || num >= den {
            return Err(JarzynskiRefusal::Confidence {
                delta: if den == 0 {
                    f64::NAN
                } else {
                    num as f64 / den as f64
                },
            });
        }
        Ok(Self { num, den })
    }

    /// Numerator of δ.
    pub fn num(self) -> u64 {
        self.num
    }

    /// Denominator of δ.
    pub fn den(self) -> u64 {
        self.den
    }

    /// δ at the `f64` edge.
    pub fn as_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

/// What a work ensemble says about a known free-energy change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegralFluctuationVerdict {
    /// ΔF lies in the interval widened by its stated error bound.
    Consistent(FreeEnergyInterval),
    /// ΔF lies outside the widened interval: the ensemble contradicts the theorem at level δ.
    Alarm(FreeEnergyInterval),
}

impl IntegralFluctuationVerdict {
    /// Whether the ensemble raised the alarm.
    pub fn is_alarm(self) -> bool {
        matches!(self, Self::Alarm(_))
    }

    /// The Hoeffding interval the verdict compares ΔF against.
    pub fn interval(self) -> FreeEnergyInterval {
        match self {
            Self::Consistent(ci) | Self::Alarm(ci) => ci,
        }
    }
}

/// Alarm check of the works (reduced units, every work in `support`) against `delta_f`, whose `f64` value is
/// within `delta_f_error_bound` of the exact free-energy change.
pub fn integral_fluctuation_check(
    works: &[f64],
    delta_f: f64,
    delta_f_error_bound: f64,
    support: (f64, f64),
    level: Level,
) -> Result<IntegralFluctuationVerdict, JarzynskiRefusal> {
    let ci = hoeffding_interval(works, support, level.as_f64())?;
    let inside =
        ci.lower <= delta_f + delta_f_error_bound && delta_f - delta_f_error_bound <= ci.upper;
    Ok(if inside {
        IntegralFluctuationVerdict::Consistent(ci)
    } else {
        IntegralFluctuationVerdict::Alarm(ci)
    })
}

/// Tally of alarms over replicates of an ensemble that satisfies the theorem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FalseAlarmTally {
    /// Replicates requested.
    pub replicates_attempted: usize,
    /// Replicates whose draw delivered works inside the support.
    pub replicates_valid: usize,
    /// Valid replicates that raised the alarm.
    pub alarms: usize,
    /// The level δ each replicate was checked at.
    pub level: Level,
}

impl FalseAlarmTally {
    /// Whether the exact rate `alarms / replicates_valid` exceeds δ, by integer cross-multiplication.
    pub fn rate_exceeds_level(&self) -> bool {
        (self.alarms as u128) * (self.level.den as u128)
            > (self.level.num as u128) * (self.replicates_valid as u128)
    }

    /// The rate `alarms / replicates_valid` at the `f64` edge; `0` with no valid replicate.
    pub fn rate(&self) -> f64 {
        if self.replicates_valid == 0 {
            0.0
        } else {
            self.alarms as f64 / self.replicates_valid as f64
        }
    }

    /// Whether the rate is at most δ plus `sigmas` binomial standard deviations `√(δ(1 − δ) / valid)`.
    pub fn within_level(&self, sigmas: f64) -> bool {
        if self.replicates_valid == 0 {
            return true;
        }
        let delta = self.level.as_f64();
        let slack = sigmas * (delta * (1.0 - delta) / self.replicates_valid as f64).sqrt();
        self.rate() <= delta + slack
    }
}

/// Alarm tally over `replicates` draws; a draw that returns `None` (works outside the support) is not counted
/// as valid.
pub fn record_false_alarms(
    draw: &mut impl FnMut() -> Option<Vec<f64>>,
    delta_f: f64,
    delta_f_error_bound: f64,
    support: (f64, f64),
    level: Level,
    replicates: usize,
) -> Result<FalseAlarmTally, JarzynskiRefusal> {
    let mut alarms = 0usize;
    let mut valid = 0usize;
    for _ in 0..replicates {
        let Some(works) = draw() else {
            continue;
        };
        valid += 1;
        if integral_fluctuation_check(&works, delta_f, delta_f_error_bound, support, level)?
            .is_alarm()
        {
            alarms += 1;
        }
    }
    Ok(FalseAlarmTally {
        replicates_attempted: replicates,
        replicates_valid: valid,
        alarms,
        level,
    })
}

/// Leading finite-N bias `Var(e^{−w}) / (2 N ⟨e^{−w}⟩²)` of the estimator for Gaussian reduced work
/// `w ~ N(μ, σ²)` (G. Gore, F. Ritort and C. Bustamante, Proc. Natl. Acad. Sci. U.S.A. 100, 12564, 2003), from
/// `⟨e^{−tw}⟩ = e^{−tμ + t²σ²/2}`.
pub fn gaussian_leading_bias(mu: f64, sigma_sq: f64, n: usize) -> f64 {
    let mean_exp = |t: f64| (-t * mu + 0.5 * t * t * sigma_sq).exp();
    let m1 = mean_exp(1.0);
    let var = mean_exp(2.0) - m1 * m1;
    var / (2.0 * n as f64 * m1 * m1)
}

/// What the meter reports for one ensemble.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeterReport {
    /// Number of works.
    pub samples: usize,
    /// Estimate `ΔF̂ = −ln mean e^{−w}`.
    pub delta_f_estimate: f64,
    /// Mean work, the upper bound of the estimate.
    pub mean_work: f64,
    /// Delta-method bias estimate of the estimator.
    pub delta_method_bias: f64,
    /// The Hoeffding interval and the alarm against a known ΔF, when support, level and ΔF are given.
    pub verdict: Option<IntegralFluctuationVerdict>,
}

/// Refusal of a meter report: the text carries no ensemble, or the estimator refuses it.
#[derive(Debug, Clone, PartialEq)]
pub enum MeterRefusal {
    /// A line (counted from 1) that is neither blank, a `#` comment nor a number.
    Unparsed {
        /// The line number.
        line: usize,
    },
    /// The estimator or the interval refuses the works.
    Estimator(JarzynskiRefusal),
}

impl From<JarzynskiRefusal> for MeterRefusal {
    fn from(r: JarzynskiRefusal) -> Self {
        Self::Estimator(r)
    }
}

/// Parse one reduced work per line (blank lines and `#` comments skipped).
pub fn parse_works(text: &str) -> Result<Vec<f64>, MeterRefusal> {
    let mut works = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.parse::<f64>() {
            Ok(w) => works.push(w),
            Err(_) => return Err(MeterRefusal::Unparsed { line: index + 1 }),
        }
    }
    Ok(works)
}

/// A known free-energy change to test the ensemble against, with the support its works lie in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KnownDeltaF {
    /// The free-energy change in units of k_B·T.
    pub delta_f: f64,
    /// Stated error bound of `delta_f` at the `f64` edge.
    pub error_bound: f64,
    /// Interval every work lies in.
    pub support: (f64, f64),
    /// Level of the alarm.
    pub level: Level,
}

/// Estimator, bias and, with a known ΔF, the alarm for the works in `text`.
pub fn report(text: &str, known: Option<KnownDeltaF>) -> Result<MeterReport, MeterRefusal> {
    let works = parse_works(text)?;
    let est = estimate(&works)?;
    let verdict = match known {
        Some(k) => Some(integral_fluctuation_check(
            &works,
            k.delta_f,
            k.error_bound,
            k.support,
            k.level,
        )?),
        None => None,
    };
    Ok(MeterReport {
        samples: est.samples,
        delta_f_estimate: est.delta_f,
        mean_work: est.mean_work,
        delta_method_bias: est.delta_method_bias(),
        verdict,
    })
}

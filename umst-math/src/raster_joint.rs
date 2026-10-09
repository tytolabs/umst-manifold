// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Two-part tone joint: the 2 ⊗ 2 diagonal density of a luminance sample set split into two parts.
//!
//! The joint counts samples by `(part, tone)`: part 0 or 1 as the caller partitions them (left and right
//! columns of an image, first and second half of a video's frames), tone `bright` when the sample is at or
//! above the mean of all samples. The threshold is the data's own mean, computed in exact integers, so the
//! projection carries no tuning constant. Each cell is a count over the total, so the diagonal is
//! non-negative with trace one: a valid measurement-channel output.
//!
//! Proof: `UMST.DoubleSlit.MeasurementChannel` (a classical channel to a diagonal density), `tensorDensity`
//! (`DensityState.lean`). DOI: 10.5281/zenodo.19159660

use crate::density::DensityDiag;

/// Why a sample set has no tone joint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToneJointRefusal {
    /// One part holds no samples: the part marginal is undefined.
    EmptyPart {
        /// The empty part (0 or 1).
        part: u8,
    },
    /// The normalised counts failed the density check (rounding beyond its tolerance).
    NotADensity(&'static str),
}

/// Index of cell `(part, bright)` in the returned diagonal: `2 · part + bright`.
#[must_use]
pub const fn tone_cell(part: usize, bright: bool) -> usize {
    2 * part + if bright { 1 } else { 0 }
}

/// The `(part, tone)` joint of `first` and `second`.
///
/// # Errors
///
/// [`ToneJointRefusal::EmptyPart`] when either part is empty; [`ToneJointRefusal::NotADensity`] if the
/// normalised counts miss trace one beyond the density tolerance.
// Counts and total below 2^53 convert exactly; above it the ratio keeps f64 relative precision.
#[allow(clippy::cast_precision_loss)]
pub fn two_part_tone_joint(first: &[u8], second: &[u8]) -> Result<DensityDiag<4>, ToneJointRefusal> {
    if first.is_empty() {
        return Err(ToneJointRefusal::EmptyPart { part: 0 });
    }
    if second.is_empty() {
        return Err(ToneJointRefusal::EmptyPart { part: 1 });
    }
    let total = (first.len() + second.len()) as u128;
    let sum: u128 = first.iter().chain(second).map(|&x| u128::from(x)).sum();
    // bright ⇔ x ≥ mean ⇔ x · total ≥ sum (exact).
    let bright = |x: u8| u128::from(x) * total >= sum;
    let mut counts = [0_u128; 4];
    for (part, samples) in [first, second].into_iter().enumerate() {
        for &x in samples {
            counts[tone_cell(part, bright(x))] += 1;
        }
    }
    let raw = counts.map(|c| c as f64 / total as f64);
    DensityDiag::try_from_diag(raw).map_err(ToneJointRefusal::NotADensity)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(d: &DensityDiag<4>) -> [f64; 4] {
        d.p.map(ordered_float::NotNan::into_inner)
    }

    #[test]
    fn dark_left_bright_right_puts_all_mass_on_the_anti_diagonal_cells() {
        let d = two_part_tone_joint(&[0, 10, 20], &[200, 250, 240]).expect("joint");
        let c = cells(&d);
        assert!((c[tone_cell(0, false)] - 0.5).abs() < 1e-15);
        assert!((c[tone_cell(1, true)] - 0.5).abs() < 1e-15);
        assert!(c[tone_cell(0, true)].abs() < f64::MIN_POSITIVE);
        assert!(c[tone_cell(1, false)].abs() < f64::MIN_POSITIVE);
    }

    #[test]
    fn uniform_samples_are_all_bright_and_the_parts_keep_their_sizes() {
        let d = two_part_tone_joint(&[7; 3], &[7; 1]).expect("joint");
        let c = cells(&d);
        assert!((c[tone_cell(0, true)] - 0.75).abs() < 1e-15);
        assert!((c[tone_cell(1, true)] - 0.25).abs() < 1e-15);
        assert!((c.iter().sum::<f64>() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn threshold_is_the_exact_mean_not_a_constant() {
        // Mean of {1, 2, 3, 4} is 2.5: 3 and 4 are bright, 1 and 2 dark.
        let d = two_part_tone_joint(&[1, 4], &[2, 3]).expect("joint");
        let c = cells(&d);
        assert!((c[tone_cell(0, true)] - 0.25).abs() < 1e-15);
        assert!((c[tone_cell(0, false)] - 0.25).abs() < 1e-15);
        assert!((c[tone_cell(1, true)] - 0.25).abs() < 1e-15);
        assert!((c[tone_cell(1, false)] - 0.25).abs() < 1e-15);
    }

    #[test]
    fn an_empty_part_is_refused() {
        assert_eq!(two_part_tone_joint(&[], &[1]), Err(ToneJointRefusal::EmptyPart { part: 0 }));
        assert_eq!(two_part_tone_joint(&[1], &[]), Err(ToneJointRefusal::EmptyPart { part: 1 }));
    }
}

//! Gaussian bivariate MI from Pearson correlation.
//!
//! Proof: `UMST.Formal.RhoEstimator::rho_based_mi_formula`
//!
//! Implementation lives in [`crate::kernels`] (scalar + optional SIMD dispatch).

/// `MI(ρ) = −½ · log₂(1 − ρ²)` in bits for `|ρ| < 1`.
///
/// Proof: UMST.Formal.RhoEstimator::rho_based_mi_formula
#[must_use]
pub fn rho_mi_bits(rho: f64) -> f64 {
    crate::kernels::rho_mi_bits(rho)
}

/// Pearson correlation then [`rho_mi_bits`]; `None` if fewer than two finite samples or zero variance.
///
/// Proof: UMST.Formal.RhoEstimator::rho_based_mi_formula (plug-in on empirical ρ̂)
#[must_use]
pub fn rho_mi_from_samples(xs: &[f64], ys: &[f64]) -> Option<f64> {
    crate::kernels::rho_mi_from_samples(xs, ys)
}

#[cfg(test)]
mod tests {
    use super::{rho_mi_bits, rho_mi_from_samples};

    #[test]
    fn rho_mi_zero_at_zero() {
        assert!((rho_mi_bits(0.0)).abs() < 1e-15);
    }

    #[test]
    fn rho_mi_monotone_abs_rho() {
        let a = rho_mi_bits(0.2);
        let b = rho_mi_bits(0.5);
        assert!(a < b, "larger |ρ| should yield larger MI on [0,1)");
    }

    #[test]
    fn rho_mi_clamps_extreme_rho() {
        let v = rho_mi_bits(1.0);
        assert!(v.is_finite() && v >= 0.0);
        let w = rho_mi_bits(-1.0);
        assert!(w.is_finite() && w >= 0.0);
    }

    #[test]
    fn rho_mi_from_samples_perfect_line() {
        let xs = [0.0_f64, 1.0, 2.0, 3.0];
        let ys = [0.0_f64, 1.0, 2.0, 3.0];
        let m = rho_mi_from_samples(&xs, &ys).expect("finite correlation");
        assert!(m > 0.0);
    }

    #[test]
    fn rho_mi_from_samples_rejects_short() {
        assert!(rho_mi_from_samples(&[1.0], &[1.0]).is_none());
    }

    #[test]
    fn rho_mi_from_samples_rejects_zero_variance() {
        let xs = [1.0_f64, 1.0, 1.0];
        let ys = [2.0_f64, 3.0, 4.0];
        assert!(rho_mi_from_samples(&xs, &ys).is_none());
    }
}

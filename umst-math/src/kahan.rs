//! Kahan compensated summation for long-running f64 totals (1000+ turns).
//!
//! Proof anchor: numerical stability for Shannon / von Neumann diagonal sums (`DensityDiag::trace`, `shannon_diag_bits`).
//! DOI: 10.5281/zenodo.19159660

/// Neumaier-style compensated sum (robust when magnitudes differ).
#[derive(Clone, Debug, Default)]
pub struct KahanSum {
    sum: f64,
    c: f64,
}

impl KahanSum {
    /// Proof: numerical analysis standard; used for Shannon / von Neumann sums in Egoff hot path.
    /// DOI: 10.5281/zenodo.19159660
    pub fn new() -> Self {
        Self { sum: 0.0, c: 0.0 }
    }

    /// Add a value with compensation.
    /// DOI: 10.5281/zenodo.19159660
    pub fn add(&mut self, x: f64) {
        let t = self.sum + x;
        self.c = if self.sum.abs() >= x.abs() {
            (self.sum - t) + x
        } else {
            (x - t) + self.sum
        };
        self.sum = t + self.c;
    }

    /// Current total.
    /// DOI: 10.5281/zenodo.19159660
    pub fn total(&self) -> f64 {
        self.sum + self.c
    }
}

impl std::ops::AddAssign<f64> for KahanSum {
    fn add_assign(&mut self, x: f64) {
        self.add(x);
    }
}

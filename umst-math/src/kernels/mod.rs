//! Host-dispatchable numerical kernels (scalar always built; optional `simd` feature for `std::simd`).
//!
//! **Scope (Phase M-simd):** compile-time dispatch via `cfg(feature = "simd")` only.
//! Runtime feature detection (CPUID / HWCAP) is explicitly **out of scope** for this slice.

mod scalar;

#[cfg(feature = "simd")]
mod simd;

#[cfg(all(test, feature = "simd"))]
mod equiv_tests;

#[cfg(feature = "simd")]
mod tolerance_receipt;

#[cfg(feature = "simd")]
pub use tolerance_receipt::kernel_tolerance_rows;

pub use scalar::{
    classify_band_scalar, rho_mi_bits, rho_mi_from_samples_scalar, sample_percentile_presorted,
    sample_percentile_scalar, BandLabel,
};

/// Labels for cockpit telemetry (`KernelDispatchReport` in cockpit). Values are `"scalar"` or `"simd"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KernelDispatchLabels {
    /// `rho_mi_from_samples` implementation.
    pub rho_mi_from_samples: &'static str,
    /// `sample_percentile` implementation.
    pub sample_percentile: &'static str,
    /// `classify_band` implementation.
    pub classify_band: &'static str,
}

/// Which implementation the current build selected at compile time.
#[must_use]
pub fn kernel_dispatch_labels() -> KernelDispatchLabels {
    #[cfg(feature = "simd")]
    {
        KernelDispatchLabels {
            rho_mi_from_samples: "simd",
            sample_percentile: "simd",
            classify_band: "simd",
        }
    }
    #[cfg(not(feature = "simd"))]
    {
        KernelDispatchLabels {
            rho_mi_from_samples: "scalar",
            sample_percentile: "scalar",
            classify_band: "scalar",
        }
    }
}

/// Dispatched ρ-MI from samples (scalar or SIMD per feature).
#[must_use]
pub fn rho_mi_from_samples(xs: &[f64], ys: &[f64]) -> Option<f64> {
    #[cfg(feature = "simd")]
    {
        simd::rho_mi_from_samples_simd(xs, ys)
    }
    #[cfg(not(feature = "simd"))]
    {
        scalar::rho_mi_from_samples_scalar(xs, ys)
    }
}

/// Dispatched sample percentile.
#[must_use]
pub fn sample_percentile(samples: &[f64], q: f64) -> Option<f64> {
    #[cfg(feature = "simd")]
    {
        simd::sample_percentile_simd(samples, q)
    }
    #[cfg(not(feature = "simd"))]
    {
        scalar::sample_percentile_scalar(samples, q)
    }
}

/// Dispatched band classifier.
#[must_use]
pub fn classify_band(samples: &[f64], eta: f64) -> Option<BandLabel> {
    #[cfg(feature = "simd")]
    {
        simd::classify_band_simd(samples, eta)
    }
    #[cfg(not(feature = "simd"))]
    {
        scalar::classify_band_scalar(samples, eta)
    }
}

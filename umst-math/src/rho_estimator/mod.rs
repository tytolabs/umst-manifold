//! ρ-based Gaussian mutual information (bits) — engineering mirror of `UMST.Formal.RhoEstimator`.

pub mod core;

pub use core::{rho_mi_bits, rho_mi_from_samples};

//! Thermodynamic–epistemic dignity scalar (Lean `UMST.Formal.Dignity`).
//!
//! Pure `f64` engineering mirror; see crate-level ISA note for Egoff integration.

pub mod core;

pub use core::{
    dignity_monotone_under_mi_gain_check, dignity_step, honest_spend, try_dignity, D_MAX,
};

//! UMST mathematical kernel — pure Rust mirror of identities proved in
//! `tytolabs/umst-formal` and `tytolabs/umst-formal-double-slit`.
//!
//! Each public function cites the Lean module and Zenodo DOI in its doc comment.
//! See `theorem_registry::THEOREM_REGISTRY` for a compile-time index.
//!
//! # Crate invariants
//! - **`#![forbid(unsafe_code)]`** — no `unsafe` in this crate.
//! - Prefer **`NotNan<f64>`** on hot numerical boundaries (oracle / closed-loop).

#![cfg_attr(feature = "simd", feature(portable_simd))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod kahan;

/// Phase 1 catalog → scalar-layout functor witness (pure; no I/O).
pub mod catalog_functor;
pub mod constants;
pub mod credit;
/// §14bis.f-S-0 — PQC primitives (ML-KEM / ML-DSA / SLH-DSA / SHA3-256).
#[allow(missing_docs)]
pub mod crypto;
pub mod density;
pub mod dignity;
pub mod dpi;
pub mod economic;
pub mod englert;
pub mod epistemic;
pub mod erasure;
pub mod eta_cog;
pub mod fixtures;
/// §14bis.f-H-8: hardware abstraction *trait* surface (category **𝓗**; FORWARD-PLAN v1.2) — no backends
#[allow(missing_docs)]
pub mod hal;
pub mod hypergraph;
pub mod info_entropy;
pub mod io;
pub mod kernels;
pub mod klein;
pub mod kraus;
pub mod landauer;
/// P3 CODATA / Landauer compile-time registry (`math-constants` feature).
#[cfg(feature = "math-constants")]
pub mod landauer_registry;
pub mod lindblad;
/// §14bis.f-M-0: M-Arc **manifold** pure math (S², SDF/CSG, Hilbert, octree; GMD) — no I/O
#[allow(missing_docs)]
pub mod manifold;
pub mod median_convergence;
pub mod mi;
pub mod order_statistics_band;
pub mod pmic;
pub mod rho_estimator;
pub mod schrodinger;
/// THEOREM-BOUND: scalar Kalman + Joseph EKF smoothers (§14bis.e-TUI-7; vendor umst-prototype-2a)
pub mod smoothing;
pub mod sparse;
pub mod tensor;
pub mod theorem_registry;
pub mod vne;

/// THEOREM-BOUND: `combine_density_between` (re-export: density diagonal / CGD struct)
pub use density::DensityDiag;
/// THEOREM-BOUND: `clausiusDuhemFwd` (re-export: Englert duality / thermo bridge)
pub use englert::{englert_bound_holds, englert_lhs};
/// CONSTANT-BOUND: `landauer_floor_j_per_bit` (re-export: Landauer bit cost)
pub use landauer::landauer_cost_diagonal_bits;
/// THEOREM-BOUND: Clausius–Duhem admissibility (`Gate.lean` conjunct; SSOT for ucrs/formal drift)
pub use manifold::csg::clausius_duhem_admissible;
/// THEOREM-BOUND: `credit_greedy_optimal` (re-export: PMIC / residual coherence capacity)
pub use pmic::residual_coherence_capacity;

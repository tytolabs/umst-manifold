// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
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
pub mod content_address;
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
/// L1a: matrix-free linear operators with certified structure witnesses (design §2.1).
pub mod linear_operator;
/// §14bis.f-M-0: M-Arc **manifold** pure math (S², SDF/CSG, Hilbert, octree; GMD) — no I/O
#[allow(missing_docs)]
pub mod manifold;
pub mod median_convergence;
pub mod mi;
pub mod order_statistics_band;
pub mod pmic;
/// L1a: matrix-free preconditioners paired with [`linear_operator`].
pub mod preconditioner;
pub mod rho_estimator;
pub mod schrodinger;
/// THEOREM-BOUND: scalar Kalman + Joseph EKF smoothers (§14bis.e-TUI-7; vendor umst-prototype-2a)
pub mod smoothing;
/// L1a: typed structural refusals before iterative solve (SOLVER_COMPOSITION_DESIGN §2.2).
pub mod solver_refusal;
/// Track C coalgebraic solve combinator — Thermo unfold, certified outcomes.
pub mod solve_combinator;
/// n_unknowns is escalation evidence for unfold, not a stop.
pub mod problem_size_escalation;
/// CG stall window: median gap between strict residual improvements.
pub mod cg_stall_window;
/// CG stall window from the Lanczos spectrum of the coefficient trace.
pub mod cg_spectral_window;
/// Outer loop stops on a KKT certificate or the budget.
pub mod kkt_outer_stop;
/// Solver / regression numeric tolerance SSOT (derive from [`numeric_tolerance::ProblemScale`]).
pub mod numeric_tolerance;
/// Symmetric profile (skyline) matrices, their `L D Lᵀ` factor, definiteness and inertia certificates.
pub mod profile_ldlt;
/// Certified lowest eigenpairs of a symmetric pencil: shift-invert Lanczos as a budgeted unfold.
pub mod generalized_eigen;
pub mod sparse;
pub mod tensor;
pub mod theorem_blurbs;
pub mod theorem_registry;
pub mod vne;

/// L1a: linear operator witnesses and matvec trait.
pub use linear_operator::{
    semidefinite_from_nullspace, semidefinite_nullspace, semidefinite_operator,
    spd_from_diagonal_positive, spd_operator, symmetric_from_bilinear_test, symmetric_from_spd,
    symmetric_operator, DiagonalOperator, IdentityOperator, LinearOperator, NullspaceWitness,
    OpError, Semidefinite, Spd, Symmetric,
};
/// L1a: preconditioner trait and diagonal / identity implementations.
pub use preconditioner::{DiagonalPreconditioner, IdentityPreconditioner, Preconditioner};
/// L1a: pre-iteration structural refusal witnesses (§2.2).
pub use solver_refusal::{
    Inconsistent, Mechanism, PrecisionInsufficient, SolverRefusal, SolverRefusalCertifyRefuse,
    Underconstrained,
};
/// CONSTANT-BOUND: CGD registry row types (re-export for wire gate on registry edits).
pub use constants::registry::{ConstantEntry, ConstantTier};
/// CONSTANT-BOUND: compiler pin snapshot parsed from `TOOLCHAIN_PIN.txt`.
pub use constants::toolchain_pin::ToolchainSnapshot;
/// THEOREM-BOUND: `combine_density_between` (re-export: density diagonal / CGD struct)
pub use density::DensityDiag;
/// THEOREM-BOUND: `clausiusDuhemFwd` (re-export: Englert duality / thermo bridge)
pub use englert::{englert_bound_holds, englert_lhs};
/// CONSTANT-BOUND: `landauer_floor_j_per_bit` (re-export: Landauer bit cost)
pub use landauer::landauer_cost_diagonal_bits;
/// Track C: certified solve outcomes + UCRS Landauer budget types.
pub use solve_combinator::{
    landauer_step_joules, unfold, BoundedPackageMeter, CombinatorRefuse, EnergyBudget,
    EnergySpendProvenance, EnergySpent, FixedJouleMeter, MeasuredPackageMeter, PackagePowerReader,
    ProblemProgressWindow, ProblemTolerance, ProgressCertificate, ResidualCertificate,
    SolveOutcome, StallEvidence, StepEnergyMeter, StrategyRung, Thermo, UnmeasuredMeter,
};
/// THEOREM-BOUND: Clausius–Duhem admissibility (`Gate.lean` conjunct; SSOT for ucrs/formal drift)
pub use manifold::csg::{clausius_duhem_admissible, ThermoGateState};
/// THEOREM-BOUND: `credit_greedy_optimal` (re-export: PMIC / residual coherence capacity)
pub use pmic::residual_coherence_capacity;

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Typed physics solver failures (FP manifesto §2 — total functions, no panic on domain errors).

use core::fmt;

/// Physics solver failures surfaced as `Result::Err` instead of panic (Wave 3a+).
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicsError {
    /// Tensor / vector shape or rank mismatch on the hot path.
    ShapeMismatch {
        context: &'static str,
        detail: &'static str,
    },
    /// Host buffer length does not match assembled DOF count.
    BufferLength {
        context: &'static str,
        expected: usize,
        got: usize,
    },
    /// Equilibrium / Newton / PCG did not meet relative tolerance.
    Diverged {
        eq_rel: f32,
        pcg_iterations: usize,
    },
    /// Compliance scalar non-finite after forward solve.
    NonFiniteCompliance,
    /// Non-finite field or residual detected.
    NonFinite {
        context: &'static str,
    },
    /// Host GMRES / Krylov breakdown or residual blow-up.
    KrylovDiverged {
        context: &'static str,
    },
    /// Linear system indefinite or singular on the masked subspace.
    IndefiniteSystem {
        context: &'static str,
    },
    /// Reusable scratch / workspace exhausted.
    BufferExhausted {
        context: &'static str,
    },
    /// Internal invariant violated (precondition not met).
    InvariantViolation {
        context: &'static str,
    },
    /// Discretization layout not supported by this solver entry point.
    UnsupportedLayout {
        context: &'static str,
    },
    /// Gate evidence wiring rejected the post-step state.
    GateEvidenceRejected {
        context: &'static str,
    },
    /// Solver / sync domain error with human-readable detail (THMC `step` migration path).
    Domain {
        detail: String,
    },
}

impl fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShapeMismatch { context, detail } => {
                write!(f, "{context}: shape mismatch ({detail})")
            }
            Self::BufferLength {
                context,
                expected,
                got,
            } => write!(
                f,
                "{context}: buffer length mismatch (expected {expected}, got {got})"
            ),
            Self::Diverged {
                eq_rel,
                pcg_iterations,
            } => write!(
                f,
                "solver diverged (eq_rel={eq_rel}, pcg_iterations={pcg_iterations})"
            ),
            Self::NonFiniteCompliance => f.write_str("non-finite compliance scalar"),
            Self::NonFinite { context } => write!(f, "{context}: non-finite value"),
            Self::KrylovDiverged { context } => write!(f, "{context}: Krylov diverged"),
            Self::IndefiniteSystem { context } => write!(f, "{context}: indefinite system"),
            Self::BufferExhausted { context } => write!(f, "{context}: buffer exhausted"),
            Self::InvariantViolation { context } => write!(f, "{context}: invariant violation"),
            Self::UnsupportedLayout { context } => write!(f, "{context}: unsupported layout"),
            Self::GateEvidenceRejected { context } => {
                write!(f, "{context}: gate evidence rejected")
            }
            Self::Domain { detail } => f.write_str(detail),
        }
    }
}

impl From<String> for PhysicsError {
    fn from(detail: String) -> Self {
        Self::Domain { detail }
    }
}

impl From<&str> for PhysicsError {
    fn from(detail: &str) -> Self {
        Self::Domain {
            detail: detail.to_string(),
        }
    }
}

impl From<PhysicsError> for String {
    fn from(err: PhysicsError) -> Self {
        err.to_string()
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Typed physics solver failures (FP manifesto §2 — total functions, no panic on domain errors).
//!
//! # Honest boundary (W29-052)
//!
//! [`PhysicsError`] is the **typed Result::Err surface** for physics hot paths (shape, buffers,
//! Krylov, gate evidence, THMC domain strings). Display + `From` shims + `std::error::Error` are
//! landed for diagnostics. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER` / OP-5.
//! Unit contracts: `cargo test -p umst-manifold error`.

use core::fmt;

/// W29 deepen cell — physics error taxonomy honest fence bundle.
pub const W29_ERROR_DEEPEN_CELL: &str = "W29-052-ERROR";

/// Honest posture tag — typed solver Err taxonomy landed; fleet production wiring refused.
pub const ERROR_POSTURE_TAG: &str = "honest-physics-error-taxonomy-research-lane";

/// Honest physics posture — Display/From/Error contracts pass; does not certify fleet physics GREEN.
pub const ERROR_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by the error taxonomy alone.
pub const ERROR_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const ERROR_MASTER: bool = false;

/// Whether typed [`PhysicsError`] variants are landed (non-string taxonomy present).
pub const ERROR_TYPED_VARIANTS_LANDED: bool = true;

/// Whether `std::error::Error` is implemented for [`PhysicsError`].
pub const ERROR_STD_ERROR_IMPL_LANDED: bool = true;

/// Whether legacy `From<String>` / `From<&str>` Domain shims remain required at call sites.
pub const ERROR_LEGACY_STRING_SHIMS_REQUIRED: bool = true;

/// Count of [`PhysicsError`] variants (including Domain).
pub const PHYSICS_ERROR_VARIANT_COUNT: usize = 12;

/// Honest deepen fence for meta / fleet probes.
pub const ERROR_HONEST_FENCE: &str =
    "typed_variants_landed=true std_error_impl_landed=true legacy_string_shims_required=true production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!ERROR_PRODUCTION_WIRED);
const _: () = assert!(!ERROR_PHYSICS_GREEN);
const _: () = assert!(!ERROR_MASTER);
const _: () = assert!(ERROR_TYPED_VARIANTS_LANDED);
const _: () = assert!(ERROR_STD_ERROR_IMPL_LANDED);
const _: () = assert!(PHYSICS_ERROR_VARIANT_COUNT == 12);

/// Typed probe for physics error taxonomy posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub typed_variants_landed: bool,
    pub std_error_impl_landed: bool,
    pub legacy_string_shims_required: bool,
    pub variant_count: usize,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for physics error taxonomy.
#[must_use]
pub fn error_honest_posture_bundle() -> ErrorPostureProbe {
    ErrorPostureProbe {
        physics_green: ERROR_PHYSICS_GREEN,
        production_wired: ERROR_PRODUCTION_WIRED,
        master: ERROR_MASTER,
        typed_variants_landed: ERROR_TYPED_VARIANTS_LANDED,
        std_error_impl_landed: ERROR_STD_ERROR_IMPL_LANDED,
        legacy_string_shims_required: ERROR_LEGACY_STRING_SHIMS_REQUIRED,
        variant_count: PHYSICS_ERROR_VARIANT_COUNT,
        honest_fence: ERROR_HONEST_FENCE,
        posture_tag: ERROR_POSTURE_TAG,
        deepen_cell: W29_ERROR_DEEPEN_CELL,
    }
}

/// Typed taxonomy landed with production/master/GREEN composition honestly open.
#[must_use]
pub fn error_posture_honest(probe: &ErrorPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.typed_variants_landed
        && probe.std_error_impl_landed
        && probe.legacy_string_shims_required
        && probe.variant_count == PHYSICS_ERROR_VARIANT_COUNT
        && probe.honest_fence.contains("typed_variants_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the physics error surface.
#[must_use]
pub fn error_refuse_overclaim(probe: &ErrorPostureProbe) -> Result<(), &'static str> {
    if probe.physics_green {
        return Err("ERROR_PHYSICS_GREEN must stay false until fleet physics closes");
    }
    if probe.production_wired {
        return Err("ERROR_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("ERROR_MASTER must stay false — not claimed by error taxonomy alone");
    }
    if !error_posture_honest(probe) {
        return Err("physics error posture fence inconsistent");
    }
    Ok(())
}

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
    Diverged { eq_rel: f32, pcg_iterations: usize },
    /// Compliance scalar non-finite after forward solve.
    NonFiniteCompliance,
    /// Non-finite field or residual detected.
    NonFinite { context: &'static str },
    /// Host GMRES / Krylov breakdown or residual blow-up.
    KrylovDiverged { context: &'static str },
    /// Linear system indefinite or singular on the masked subspace.
    IndefiniteSystem { context: &'static str },
    /// Reusable scratch / workspace exhausted.
    BufferExhausted { context: &'static str },
    /// Internal invariant violated (precondition not met).
    InvariantViolation { context: &'static str },
    /// Discretization layout not supported by this solver entry point.
    UnsupportedLayout { context: &'static str },
    /// Gate evidence wiring rejected the post-step state.
    GateEvidenceRejected { context: &'static str },
    /// Solver / sync domain error with human-readable detail (THMC `step` migration path).
    Domain { detail: String },
}

impl PhysicsError {
    /// Optional static context label when the variant carries one.
    #[must_use]
    pub fn context(&self) -> Option<&'static str> {
        match self {
            Self::ShapeMismatch { context, .. }
            | Self::BufferLength { context, .. }
            | Self::NonFinite { context }
            | Self::KrylovDiverged { context }
            | Self::IndefiniteSystem { context }
            | Self::BufferExhausted { context }
            | Self::InvariantViolation { context }
            | Self::UnsupportedLayout { context }
            | Self::GateEvidenceRejected { context } => Some(*context),
            Self::Diverged { .. } | Self::NonFiniteCompliance | Self::Domain { .. } => None,
        }
    }

    /// Whether this failure is a non-finite numeric pathology.
    #[must_use]
    pub fn is_non_finite(&self) -> bool {
        matches!(self, Self::NonFiniteCompliance | Self::NonFinite { .. })
    }

    /// Whether this failure is an iterative / Krylov divergence.
    #[must_use]
    pub fn is_divergence(&self) -> bool {
        matches!(self, Self::Diverged { .. } | Self::KrylovDiverged { .. })
    }
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
                "iterative solve failed to converge within tolerance \
                 (relative residual {eq_rel} after {pcg_iterations} iterations)"
            ),
            Self::NonFiniteCompliance => f.write_str(
                "compliance functional is non-finite (NaN or Inf) after equilibrium forward solve",
            ),
            Self::NonFinite { context } => {
                write!(f, "{context}: field or residual contains NaN or Inf")
            }
            Self::KrylovDiverged { context } => write!(
                f,
                "{context}: Krylov/GMRES sub-solve stalled or residual blew up"
            ),
            Self::IndefiniteSystem { context } => write!(
                f,
                "{context}: stiffness operator indefinite or singular on masked DOF subspace"
            ),
            Self::BufferExhausted { context } => write!(
                f,
                "{context}: preallocated scratch/workspace capacity exhausted"
            ),
            Self::InvariantViolation { context } => write!(
                f,
                "{context}: internal precondition or invariant violated before solver step"
            ),
            Self::UnsupportedLayout { context } => write!(
                f,
                "{context}: mesh or discretization layout not supported by this entry point"
            ),
            Self::GateEvidenceRejected { context } => write!(
                f,
                "{context}: post-step gate or CBF evidence check rejected state"
            ),
            Self::Domain { detail } => f.write_str(detail),
        }
    }
}

impl std::error::Error for PhysicsError {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_all_physics_error_variants() {
        let cases: Vec<(PhysicsError, &str)> = vec![
            (
                PhysicsError::ShapeMismatch {
                    context: "q1_hex",
                    detail: "rank",
                },
                "shape mismatch",
            ),
            (
                PhysicsError::BufferLength {
                    context: "assemble",
                    expected: 8,
                    got: 4,
                },
                "buffer length",
            ),
            (
                PhysicsError::Diverged {
                    eq_rel: 1.5,
                    pcg_iterations: 42,
                },
                "relative residual",
            ),
            (PhysicsError::NonFiniteCompliance, "compliance functional"),
            (
                PhysicsError::NonFinite {
                    context: "residual",
                },
                "NaN or Inf",
            ),
            (
                PhysicsError::KrylovDiverged { context: "gmres" },
                "Krylov/GMRES",
            ),
            (
                PhysicsError::IndefiniteSystem { context: "pcg" },
                "indefinite or singular",
            ),
            (
                PhysicsError::BufferExhausted { context: "scratch" },
                "capacity exhausted",
            ),
            (
                PhysicsError::InvariantViolation { context: "pre" },
                "invariant violated",
            ),
            (
                PhysicsError::UnsupportedLayout { context: "poisson" },
                "layout not supported",
            ),
            (
                PhysicsError::GateEvidenceRejected { context: "cbf" },
                "evidence check rejected",
            ),
            (
                PhysicsError::Domain {
                    detail: "thmc sync failed".into(),
                },
                "thmc sync failed",
            ),
        ];
        assert_eq!(cases.len(), PHYSICS_ERROR_VARIANT_COUNT);
        for (err, needle) in cases {
            let msg = err.to_string();
            assert!(
                msg.contains(needle),
                "display missing '{needle}' in '{msg}'"
            );
        }
    }

    #[test]
    fn from_string_and_str_become_domain() {
        let from_owned: PhysicsError = String::from("owned detail").into();
        let from_str: PhysicsError = "borrowed detail".into();
        assert!(matches!(from_owned, PhysicsError::Domain { .. }));
        assert!(matches!(from_str, PhysicsError::Domain { .. }));
        assert_eq!(from_owned.to_string(), "owned detail");
        assert_eq!(from_str.to_string(), "borrowed detail");
    }

    #[test]
    fn physics_error_into_string_uses_display() {
        let err = PhysicsError::NonFinite { context: "field" };
        let s: String = err.into();
        assert!(s.contains("field"));
        assert!(s.contains("NaN or Inf"));
    }

    #[test]
    fn context_and_classifier_helpers() {
        assert_eq!(
            PhysicsError::NonFinite { context: "r" }.context(),
            Some("r")
        );
        assert_eq!(
            PhysicsError::Diverged {
                eq_rel: 0.1,
                pcg_iterations: 1
            }
            .context(),
            None
        );
        assert!(PhysicsError::NonFiniteCompliance.is_non_finite());
        assert!(PhysicsError::NonFinite { context: "x" }.is_non_finite());
        assert!(!PhysicsError::Domain { detail: "d".into() }.is_non_finite());
        assert!(PhysicsError::KrylovDiverged { context: "k" }.is_divergence());
        assert!(PhysicsError::Diverged {
            eq_rel: 1.0,
            pcg_iterations: 2
        }
        .is_divergence());
        assert!(!PhysicsError::ShapeMismatch {
            context: "c",
            detail: "d"
        }
        .is_divergence());
    }

    #[test]
    fn std_error_trait_object_ok() {
        let err: Box<dyn std::error::Error> =
            Box::new(PhysicsError::BufferExhausted { context: "ws" });
        assert!(err.to_string().contains("capacity exhausted"));
    }

    #[test]
    fn error_posture_bundle_refuses_overclaim() {
        let probe = error_honest_posture_bundle();
        assert_eq!(probe.deepen_cell, "W29-052-ERROR");
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.typed_variants_landed);
        assert!(probe.std_error_impl_landed);
        assert_eq!(probe.variant_count, 12);
        assert!(error_posture_honest(&probe));
        assert!(error_refuse_overclaim(&probe).is_ok());
        assert!(ERROR_HONEST_FENCE.contains("physics_green=false"));
        assert!(ERROR_HONEST_FENCE.contains("production_wired=false"));
        assert!(!ERROR_PHYSICS_GREEN);
        assert!(!ERROR_PRODUCTION_WIRED);
        assert!(!ERROR_MASTER);
    }

    #[test]
    fn error_clone_eq_roundtrip() {
        let a = PhysicsError::BufferLength {
            context: "dof",
            expected: 3,
            got: 1,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}

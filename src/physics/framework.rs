// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Cross-cutting physics architecture traits (Burn-safe: no trait objects over kernels).
//!
//! # Solver types as morphisms (sketch)
//!
//! [`PhysicsSolverZst`] marks **which solver family** is in play (ZST façade). Actual stepping and
//! composition happen in named solver modules and [`crate::physics::orchestration`]; the marker
//! trait keeps dispatch monomorphized while documenting the categorical “typed morphism” boundary.
//!
//! See `docs/Category-of-Material-Updates.md` (`fp-categorical-v04`).
//!
//! # Honest boundary (W29-054)
//!
//! Marker traits only — [`PhysicsSolverZst`] + [`PhysicsBackend`] — with monomorphized Burn
//! dispatch. No `dyn` kernels, no fleet production wiring, no physics GREEN claim, no MASTER /
//! OP-5 composition pin. Production solvers keep their own deepen fences.

use burn::tensor::backend::Backend;
use core::mem::size_of;

/// W29 deepen cell — physics framework marker-trait honest fence bundle.
pub const W29_FRAMEWORK_DEEPEN_CELL: &str = "W29-054-FRAMEWORK";

/// Honest posture tag — ZST/backend markers landed; fleet production wiring refused.
pub const FRAMEWORK_POSTURE_TAG: &str = "honest-physics-framework-marker-traits-research-lane";

/// Honest physics posture — marker contracts compile; does not certify fleet physics GREEN.
pub const FRAMEWORK_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by marker façades alone.
pub const FRAMEWORK_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const FRAMEWORK_MASTER: bool = false;

/// OP-5 composition pin — not claimed by marker traits.
pub const FRAMEWORK_OP5: bool = false;

/// Whether solver-family ZST + f32 PhysicsBackend marker traits are landed here.
pub const FRAMEWORK_MARKER_TRAITS_LANDED: bool = true;

/// Whether `dyn` kernel objects are refused on this surface (Burn monomorphization only).
pub const FRAMEWORK_DYN_KERNELS_REFUSED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const FRAMEWORK_HONEST_FENCE: &str =
    "marker_traits_landed=true solver_zst_wired=true physics_backend_f32_wired=true dyn_kernels_refused=true production_wired=false master_composition_wired=false physics_green=false op5=false";

const _: () = assert!(!FRAMEWORK_PHYSICS_GREEN);
const _: () = assert!(!FRAMEWORK_PRODUCTION_WIRED);
const _: () = assert!(!FRAMEWORK_MASTER);
const _: () = assert!(!FRAMEWORK_OP5);
const _: () = assert!(FRAMEWORK_MARKER_TRAITS_LANDED);
const _: () = assert!(FRAMEWORK_DYN_KERNELS_REFUSED);

/// Typed probe for physics-framework posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameworkPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub marker_traits_landed: bool,
    pub dyn_kernels_refused: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for physics framework markers.
#[must_use]
pub fn framework_honest_posture_bundle() -> FrameworkPostureProbe {
    FrameworkPostureProbe {
        physics_green: FRAMEWORK_PHYSICS_GREEN,
        production_wired: FRAMEWORK_PRODUCTION_WIRED,
        master: FRAMEWORK_MASTER,
        op5: FRAMEWORK_OP5,
        marker_traits_landed: FRAMEWORK_MARKER_TRAITS_LANDED,
        dyn_kernels_refused: FRAMEWORK_DYN_KERNELS_REFUSED,
        honest_fence: FRAMEWORK_HONEST_FENCE,
        posture_tag: FRAMEWORK_POSTURE_TAG,
        deepen_cell: W29_FRAMEWORK_DEEPEN_CELL,
    }
}

/// Marker-trait SSOT landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn framework_posture_honest(probe: &FrameworkPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.marker_traits_landed
        && probe.dyn_kernels_refused
        && probe.deepen_cell == W29_FRAMEWORK_DEEPEN_CELL
        && probe.honest_fence.contains("marker_traits_landed=true")
        && probe.honest_fence.contains("dyn_kernels_refused=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("op5=false")
}

/// Refuse invented GREEN / PRODUCTION_WIRED / MASTER / OP-5 claims on the framework surface.
pub fn framework_refuse_invented_pins() -> Result<(), &'static str> {
    if FRAMEWORK_PHYSICS_GREEN {
        return Err("FRAMEWORK_PHYSICS_GREEN must stay false — markers ≠ fleet physics");
    }
    if FRAMEWORK_PRODUCTION_WIRED {
        return Err("FRAMEWORK_PRODUCTION_WIRED must stay false until embodied solver loop closes");
    }
    if FRAMEWORK_MASTER {
        return Err("FRAMEWORK_MASTER must stay false — not an OP-5 composition pin");
    }
    if FRAMEWORK_OP5 {
        return Err("FRAMEWORK_OP5 must stay false — framework markers are not OP-5");
    }
    Ok(())
}

/// Zero-sized façade for a solver **family** (`VectorMechanicsSolver`, …): identity-like marker, not a `dyn` kernel.
pub trait PhysicsSolverZst: Send + Sync + 'static {}

/// Marker for backends used in f32 equilibrium / transport stacks.
pub trait PhysicsBackend: Backend<FloatElem = f32> {}

impl<B: Backend<FloatElem = f32>> PhysicsBackend for B {}

/// Compile-time witness that a [`PhysicsSolverZst`] implementor is a true ZST façade.
#[must_use]
pub const fn physics_solver_zst_byte_size<S: PhysicsSolverZst>() -> usize {
    size_of::<S>()
}

/// Monomorphized backend witness — proves `B: PhysicsBackend` without `dyn` dispatch.
#[inline]
#[must_use]
pub fn assert_physics_backend<B: PhysicsBackend>() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn_ndarray::NdArray;

    /// Test-only ZST solver family — proves the marker trait stays zero-sized.
    #[derive(Debug, Default, Clone, Copy)]
    struct FrameworkMarkerSolver;

    impl PhysicsSolverZst for FrameworkMarkerSolver {}

    #[test]
    fn framework_honest_posture_refuses_green_production_master_op5() {
        let probe = framework_honest_posture_bundle();
        assert!(framework_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert!(probe.marker_traits_landed);
        assert!(probe.dyn_kernels_refused);
        assert_eq!(probe.deepen_cell, W29_FRAMEWORK_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, FRAMEWORK_POSTURE_TAG);
        assert!(framework_refuse_invented_pins().is_ok());
    }

    #[test]
    fn framework_solver_zst_is_zero_sized() {
        assert_eq!(physics_solver_zst_byte_size::<FrameworkMarkerSolver>(), 0);
        assert_eq!(size_of::<FrameworkMarkerSolver>(), 0);
    }

    #[test]
    fn framework_physics_backend_monomorphizes_ndarray_f32() {
        assert!(assert_physics_backend::<NdArray<f32>>());
    }

    #[test]
    fn framework_honest_fence_string_is_measured() {
        assert!(FRAMEWORK_HONEST_FENCE.contains("marker_traits_landed=true"));
        assert!(FRAMEWORK_HONEST_FENCE.contains("dyn_kernels_refused=true"));
        assert!(FRAMEWORK_HONEST_FENCE.contains("production_wired=false"));
        assert!(FRAMEWORK_HONEST_FENCE.contains("physics_green=false"));
        assert!(FRAMEWORK_HONEST_FENCE.contains("op5=false"));
        assert!(!FRAMEWORK_HONEST_FENCE.contains("production_wired=true"));
        assert!(!FRAMEWORK_HONEST_FENCE.contains("physics_green=true"));
    }
}

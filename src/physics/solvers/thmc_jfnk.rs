// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side `f32` Krylov helpers for THMC JFNK slices (`solver-experimental`).
//!
//! Implementation lives in [`super::krylov_host`] so other solver lanes (acoustics) can share
//! GMRES without pulling THMC-only symbols. This module is the **THMC-facing re-export surface**
//! plus an honest deepen fence: host GMRES for matrix-free Newton inners is landed under the
//! experimental feature; production-scale monolith / adaptive `dt` / fleet GREEN are **not**
//! claimed here.
//!
//! Preferred production call shape (from `thmc_residual`): fallible matvec →
//! [`gmres_f32_try`]. Infallible [`gmres_f32`] is a thin Ok-adapter for smoke / identity mats.
//!
//! # Honest boundary (W29-084)
//!
//! Host `f32` GMRES re-export for THMC JFNK under `solver-experimental`. Smoke contracts are
//! exercised by `cargo test -p umst-manifold thmc_jfnk --features solver-experimental`.
//! Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`, not OP-5.

pub use super::krylov_host::{gmres_f32, gmres_f32_try};

/// W29 deepen cell — THMC JFNK host Krylov honest fence bundle.
pub const W29_THMC_JFNK_DEEPEN_CELL: &str = "W29-084-THMC_JFNK";

/// Honest posture tag — experimental host GMRES / JFNK research lane.
pub const THMC_JFNK_POSTURE_TAG: &str = "honest-thmc-jfnk-host-gmres-research-lane";

/// Honest physics posture — unit/smoke contracts pass; does not certify fleet physics GREEN.
pub const THMC_JFNK_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by this re-export shim alone.
pub const THMC_JFNK_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const THMC_JFNK_MASTER: bool = false;

/// OP-5 fleet claim — refused at this deepen fence.
pub const THMC_JFNK_OP5_CLAIMED: bool = false;

/// Whether the host GMRES re-export surface is landed for THMC JFNK callers.
pub const THMC_JFNK_HOST_GMRES_LANDED: bool = true;

/// Whether production-scale / adaptive-`dt` monolith JFNK is claimed (honestly open).
pub const THMC_JFNK_PRODUCTION_SCALE_CLAIMED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const THMC_JFNK_HONEST_FENCE: &str =
    "thmc_jfnk_host_gmres_landed=true solver_experimental_only=true production_scale_jfnk=false production_wired=false physics_green=false master=false op5=false";

const _: () = assert!(!THMC_JFNK_PHYSICS_GREEN);
const _: () = assert!(!THMC_JFNK_PRODUCTION_WIRED);
const _: () = assert!(!THMC_JFNK_MASTER);
const _: () = assert!(!THMC_JFNK_OP5_CLAIMED);
const _: () = assert!(!THMC_JFNK_PRODUCTION_SCALE_CLAIMED);
const _: () = assert!(THMC_JFNK_HOST_GMRES_LANDED);

/// Typed probe for THMC JFNK posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThmcJfnkPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5_claimed: bool,
    pub host_gmres_landed: bool,
    pub production_scale_claimed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the THMC JFNK host Krylov surface.
#[must_use]
pub fn thmc_jfnk_honest_posture_bundle() -> ThmcJfnkPostureProbe {
    ThmcJfnkPostureProbe {
        physics_green: THMC_JFNK_PHYSICS_GREEN,
        production_wired: THMC_JFNK_PRODUCTION_WIRED,
        master: THMC_JFNK_MASTER,
        op5_claimed: THMC_JFNK_OP5_CLAIMED,
        host_gmres_landed: THMC_JFNK_HOST_GMRES_LANDED,
        production_scale_claimed: THMC_JFNK_PRODUCTION_SCALE_CLAIMED,
        honest_fence: THMC_JFNK_HONEST_FENCE,
        posture_tag: THMC_JFNK_POSTURE_TAG,
        deepen_cell: W29_THMC_JFNK_DEEPEN_CELL,
    }
}

/// THMC JFNK surface landed with production/master/GREEN/OP-5 honestly open.
#[must_use]
pub fn thmc_jfnk_posture_honest(probe: &ThmcJfnkPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5_claimed
        && probe.host_gmres_landed
        && !probe.production_scale_claimed
        && probe.deepen_cell == W29_THMC_JFNK_DEEPEN_CELL
        && probe.honest_fence.contains("thmc_jfnk_host_gmres_landed=true")
        && probe.honest_fence.contains("production_scale_jfnk=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// Compile-time / runtime refuse path for invented GREEN / production pins.
pub fn thmc_jfnk_refuse_invented_pins() -> Result<(), &'static str> {
    if THMC_JFNK_PHYSICS_GREEN {
        return Err("THMC_JFNK_PHYSICS_GREEN must stay false — host GMRES smoke ≠ fleet physics");
    }
    if THMC_JFNK_PRODUCTION_WIRED {
        return Err(
            "THMC_JFNK_PRODUCTION_WIRED must stay false until embodied production loop closes",
        );
    }
    if THMC_JFNK_MASTER {
        return Err("THMC_JFNK_MASTER must stay false — not an OP-5 composition pin");
    }
    if THMC_JFNK_OP5_CLAIMED {
        return Err("THMC_JFNK_OP5_CLAIMED must stay false — OP-5 not claimed by JFNK shim");
    }
    if THMC_JFNK_PRODUCTION_SCALE_CLAIMED {
        return Err(
            "THMC_JFNK_PRODUCTION_SCALE_CLAIMED must stay false — large-N monolith JFNK open",
        );
    }
    Ok(())
}

#[cfg(test)]
mod w29_084_thmc_jfnk_deepen_tests {
    use super::*;
    use crate::physics::PhysicsError;

    #[test]
    fn thmc_jfnk_honest_posture_refuses_green_production_master_op5() {
        let probe = thmc_jfnk_honest_posture_bundle();
        assert!(thmc_jfnk_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5_claimed);
        assert!(probe.host_gmres_landed);
        assert!(!probe.production_scale_claimed);
        assert_eq!(probe.deepen_cell, W29_THMC_JFNK_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, THMC_JFNK_POSTURE_TAG);
        assert!(probe.honest_fence.contains("solver_experimental_only=true"));
        thmc_jfnk_refuse_invented_pins().expect("refuse invented pins");
    }

    #[test]
    fn thmc_jfnk_gmres_identity_via_reexport() {
        let n = 4usize;
        let b = vec![1.0_f32, 2.0_f32, -0.5_f32, 0.25_f32];
        let matvec = |v: &[f32]| v.to_vec();
        let x = gmres_f32(matvec, &b, n, n, 1e-5_f32).expect(
            "thmc_jfnk::gmres_f32 identity matvec n=4 must converge (W29-084 host GMRES smoke)",
        );
        for i in 0..n {
            assert!(
                (x[i] - b[i]).abs() < 1e-4_f32,
                "i={i} x={} b={}",
                x[i],
                b[i]
            );
        }
    }

    #[test]
    fn thmc_jfnk_gmres_try_propagates_matvec_error() {
        let n = 2usize;
        let b = vec![1.0_f32, 0.0_f32];
        let mut calls = 0usize;
        let matvec = |_v: &[f32]| -> Result<Vec<f32>, PhysicsError> {
            calls += 1;
            Err(PhysicsError::Domain {
                detail: "thmc_jfnk_injected".into(),
            })
        };
        let err = gmres_f32_try(matvec, &b, n, n, 1e-5_f32).expect_err(
            "thmc_jfnk::gmres_f32_try must propagate matvec PhysicsError (W29-084)",
        );
        assert!(err.to_string().contains("thmc_jfnk_injected"), "{err}");
        assert_eq!(calls, 1, "should not retry after matvec Err");
    }

    #[test]
    fn thmc_jfnk_gmres_small_spd_tridiagonal() {
        // Tiny SPD tridiagonal — same class of matvec the residual JFNK path feeds GMRES.
        let a: [f32; 9] = [
            4.0, 1.0, 0.0, //
            1.0, 4.0, 1.0, //
            0.0, 1.0, 4.0,
        ];
        let n = 3usize;
        let b = vec![1.0_f32, 0.0_f32, 0.0_f32];
        let matvec = |v: &[f32]| -> Vec<f32> {
            let mut out = vec![0.0_f32; n];
            for i in 0..n {
                let mut s = 0.0_f32;
                for j in 0..n {
                    s += a[i * n + j] * v[j];
                }
                out[i] = s;
            }
            out
        };
        let x = gmres_f32(matvec, &b, n, n + 4, 1e-4_f32).expect(
            "thmc_jfnk::gmres_f32 3×3 SPD tridiagonal must converge (W29-084)",
        );
        let ax = matvec(&x);
        let res: f32 = b
            .iter()
            .zip(ax.iter())
            .map(|(bi, axi)| {
                let d = bi - axi;
                d * d
            })
            .sum::<f32>()
            .sqrt();
        assert!(res < 1e-3_f32, "residual {res}");
    }
}

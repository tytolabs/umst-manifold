// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Reusable PCG workspace + operator cache for Q1-hex outer loops (H0 hardware perf).
//!
//! # Honest boundary (W29-071)
//!
//! [`SolverRegion`] / [`PcgWorkspace`] reuse displacement, Jacobi diag, `K·u` scratch, and an
//! optional uniform-brick [`HexStructuredOperatorCache`] across outer TO iterations.
//! Unit contracts cover capacity reuse, warm-seed wiring, and ke-cache invalidation on mesh /
//! spacing / `ν` change. Does **not** certify Striatus wall-clock wins, fleet TO production
//! wiring, or embodied master composition. Not physics GREEN, not `PRODUCTION_WIRED`, not
//! `MASTER` / OP-5.

use super::adjoint::AdjointForwardPhaseTiming;
use super::adjoint_q1_hex::Q1HexSolveOptions;
use super::q1_hex_elasticity::HexStructuredOperatorCache;

/// W29 deepen cell — SolverRegion honest fence bundle.
pub const W29_SOLVER_REGION_DEEPEN_CELL: &str = "W29-071-SOLVER_REGION";

/// Honest posture tag — H0 PCG workspace / op-cache reuse research lane.
pub const SOLVER_REGION_POSTURE_TAG: &str = "honest-solver-region-pcg-reuse-h0-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const SOLVER_REGION_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by PCG workspace reuse alone.
pub const SOLVER_REGION_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const SOLVER_REGION_MASTER: bool = false;

/// Whether PCG workspace capacity / warm-u contracts are landed.
pub const SOLVER_REGION_PCG_WORKSPACE_LANDED: bool = true;

/// Whether uniform-brick ke-unit operator cache reuse is landed.
pub const SOLVER_REGION_KE_CACHE_LANDED: bool = true;

/// Whether Striatus-scale wall-clock win is certified (honestly open — measured no win).
pub const SOLVER_REGION_STRIATUS_WALLCLOCK_CERTIFIED: bool = false;

/// Honest deepen fence for meta / fleet probes.
pub const SOLVER_REGION_HONEST_FENCE: &str =
    "pcg_workspace_landed=true ke_cache_reuse_landed=true warm_u_seed_wired=true geometry_nu_invalidate_wired=true striatus_wallclock_certified=false production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!SOLVER_REGION_PHYSICS_GREEN);
const _: () = assert!(!SOLVER_REGION_PRODUCTION_WIRED);
const _: () = assert!(!SOLVER_REGION_MASTER);
const _: () = assert!(!SOLVER_REGION_STRIATUS_WALLCLOCK_CERTIFIED);
const _: () = assert!(SOLVER_REGION_PCG_WORKSPACE_LANDED);
const _: () = assert!(SOLVER_REGION_KE_CACHE_LANDED);

/// Typed probe for SolverRegion posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverRegionPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub pcg_workspace_landed: bool,
    pub ke_cache_landed: bool,
    pub striatus_wallclock_certified: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for SolverRegion / PcgWorkspace.
#[must_use]
pub fn solver_region_honest_posture_bundle() -> SolverRegionPostureProbe {
    SolverRegionPostureProbe {
        physics_green: SOLVER_REGION_PHYSICS_GREEN,
        production_wired: SOLVER_REGION_PRODUCTION_WIRED,
        master: SOLVER_REGION_MASTER,
        pcg_workspace_landed: SOLVER_REGION_PCG_WORKSPACE_LANDED,
        ke_cache_landed: SOLVER_REGION_KE_CACHE_LANDED,
        striatus_wallclock_certified: SOLVER_REGION_STRIATUS_WALLCLOCK_CERTIFIED,
        honest_fence: SOLVER_REGION_HONEST_FENCE,
        posture_tag: SOLVER_REGION_POSTURE_TAG,
        deepen_cell: W29_SOLVER_REGION_DEEPEN_CELL,
    }
}

/// Refuse GREEN / PRODUCTION_WIRED / MASTER claims on the SolverRegion surface.
#[must_use]
pub fn solver_region_posture_honest(probe: &SolverRegionPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.striatus_wallclock_certified
        && probe.pcg_workspace_landed
        && probe.ke_cache_landed
        && probe.deepen_cell == W29_SOLVER_REGION_DEEPEN_CELL
        && probe.honest_fence.contains("pcg_workspace_landed=true")
        && probe.honest_fence.contains("geometry_nu_invalidate_wired=true")
        && probe.honest_fence.contains("striatus_wallclock_certified=false")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
}

/// Compile-time / runtime refuse path for invented GREEN / production pins.
pub fn solver_region_refuse_invented_pins() -> Result<(), &'static str> {
    if SOLVER_REGION_PHYSICS_GREEN {
        return Err("SOLVER_REGION_PHYSICS_GREEN must stay false — H0 reuse ≠ fleet physics");
    }
    if SOLVER_REGION_PRODUCTION_WIRED {
        return Err(
            "SOLVER_REGION_PRODUCTION_WIRED must stay false until fleet TO production pin closes",
        );
    }
    if SOLVER_REGION_MASTER {
        return Err("SOLVER_REGION_MASTER must stay false — not an OP-5 composition pin");
    }
    if SOLVER_REGION_STRIATUS_WALLCLOCK_CERTIFIED {
        return Err(
            "SOLVER_REGION_STRIATUS_WALLCLOCK_CERTIFIED must stay false — measured no Striatus win",
        );
    }
    Ok(())
}

/// Scratch buffers for one matrix-free PCG solve (`u`, Jacobi diag, `K·u` scratch).
#[derive(Clone, Debug, Default)]
pub struct PcgWorkspace {
    pub u: Vec<f32>,
    pub diag: Vec<f32>,
    pub scratch_ku: Vec<f32>,
}

impl PcgWorkspace {
    /// Grow buffers once; does not shrink on smaller grids.
    #[must_use]
    pub fn ensure_capacity(&mut self, n_dof: usize) -> &mut Self {
        if self.u.len() < n_dof {
            self.u.resize(n_dof, 0.0);
        }
        if self.diag.len() < n_dof {
            self.diag.resize(n_dof, 0.0);
        }
        if self.scratch_ku.len() < n_dof {
            self.scratch_ku.resize(n_dof, 0.0);
        }
        self
    }

    /// Zero displacement buffer (cold start).
    pub fn zero_u(&mut self, n_dof: usize) {
        let _ = self.ensure_capacity(n_dof);
        self.u[..n_dof].fill(0.0);
    }

    /// Copy seed into `u` when length matches.
    pub fn seed_u(&mut self, seed: &[f32], n_dof: usize) -> bool {
        if seed.len() != n_dof {
            return false;
        }
        let _ = self.ensure_capacity(n_dof);
        self.u[..n_dof].copy_from_slice(seed);
        true
    }
}

/// Per-outer-loop solver state: warm displacement, optional `ke_unit` cache, timing.
#[derive(Clone, Debug, Default)]
pub struct SolverRegion {
    pub ke_cache: Option<HexStructuredOperatorCache>,
    pub workspace: PcgWorkspace,
    pub warm_u: Option<Vec<f32>>,
    pub last_timing: AdjointForwardPhaseTiming,
}

impl SolverRegion {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// True when stored warm displacement matches `n_dof`.
    #[must_use]
    pub fn has_warm_u_for(&self, n_dof: usize) -> bool {
        self.warm_u.as_ref().is_some_and(|u| u.len() == n_dof)
    }

    /// Drop warm displacement (e.g. after a material mesh change that invalidates the seed).
    pub fn clear_warm_u(&mut self) {
        self.warm_u = None;
    }

    /// Ensure uniform-brick operator cache exists when `use_operator_cache` is set.
    ///
    /// Rebuilds when cell counts **or** brick spacing / Poisson `ν` change (`ke_unit`
    /// depends on `dx,dy,dz,nu`).
    #[allow(clippy::too_many_arguments)]
    pub fn ensure_ke_cache(
        &mut self,
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        nu: f32,
    ) {
        let needs_new = match self.ke_cache.as_ref() {
            None => true,
            Some(c) => {
                c.nx != nx
                    || c.ny != ny
                    || c.nz != nz
                    || c.dx != dx
                    || c.dy != dy
                    || c.dz != dz
                    || c.nu != nu
            }
        };
        if needs_new {
            self.ke_cache = Some(HexStructuredOperatorCache::new(nx, ny, nz, dx, dy, dz, nu));
        }
    }

    /// Drop operator cache (e.g. when SIMP `p` or mesh changes materially).
    pub fn invalidate_ke_cache(&mut self) {
        self.ke_cache = None;
    }

    /// If warm-start is enabled, seed options from stored displacement.
    pub fn seed_options_if_warm(&self, opts: &mut Q1HexSolveOptions, n_dof: usize) {
        if !opts.pcg_warm_start {
            return;
        }
        if opts.pcg_seed_displacement.is_some() {
            return;
        }
        if let Some(ref u) = self.warm_u {
            if u.len() == n_dof {
                opts.pcg_seed_displacement = Some(u.clone());
            }
        }
    }

    /// Store equilibrium displacement for the next outer PCG warm-start.
    pub fn store_warm_u(&mut self, u: &[f32]) {
        if let Some(w) = self.warm_u.as_mut() {
            if w.len() == u.len() {
                w.copy_from_slice(u);
                return;
            }
        }
        self.warm_u = Some(u.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_region_honest_posture_refuses_green_production_master() {
        let probe = solver_region_honest_posture_bundle();
        assert!(solver_region_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.striatus_wallclock_certified);
        assert!(probe.pcg_workspace_landed);
        assert!(probe.ke_cache_landed);
        assert_eq!(probe.deepen_cell, W29_SOLVER_REGION_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, SOLVER_REGION_POSTURE_TAG);
        assert!(SOLVER_REGION_HONEST_FENCE.contains("production_wired=false"));
        assert!(SOLVER_REGION_HONEST_FENCE.contains("physics_green=false"));
        assert!(SOLVER_REGION_HONEST_FENCE.contains("master_composition_wired=false"));
        assert!(SOLVER_REGION_HONEST_FENCE.contains("geometry_nu_invalidate_wired=true"));
        solver_region_refuse_invented_pins().expect(
            "solver_region_refuse_invented_pins must hold on W29-071 honest fence (FP §6 Track G H0)",
        );
    }

    #[test]
    fn solver_region_pcg_workspace_ensure_zero_seed() {
        let mut ws = PcgWorkspace::default();
        let _ = ws.ensure_capacity(6);
        assert_eq!(ws.u.len(), 6);
        assert_eq!(ws.diag.len(), 6);
        assert_eq!(ws.scratch_ku.len(), 6);
        ws.u.fill(3.0);
        ws.zero_u(4);
        assert!(ws.u[..4].iter().all(|&v| v == 0.0));
        // Grow-only: capacity stays ≥ prior ensure.
        assert_eq!(ws.u.len(), 6);
        let seed = [1.0_f32, 2.0, 3.0, 4.0];
        assert!(ws.seed_u(&seed, 4));
        assert_eq!(&ws.u[..4], &seed);
        assert!(!ws.seed_u(&[1.0, 2.0], 4));
    }

    #[test]
    fn solver_region_ke_cache_invalidates_on_geometry_or_nu() {
        let mut region = SolverRegion::new();
        region.ensure_ke_cache(2, 2, 1, 0.1, 0.1, 0.05, 0.3);
        let ptr_a = region.ke_cache.as_ref().map(|c| c.nx).expect("ke cache");
        assert_eq!(ptr_a, 2);
        // Same geometry+ν → reuse (no rebuild required for identity of stored params).
        region.ensure_ke_cache(2, 2, 1, 0.1, 0.1, 0.05, 0.3);
        let c = region.ke_cache.as_ref().expect("ke cache reuse");
        assert_eq!(c.dx, 0.1);
        assert_eq!(c.nu, 0.3);
        // Spacing change must rebuild.
        region.ensure_ke_cache(2, 2, 1, 0.2, 0.1, 0.05, 0.3);
        let c = region.ke_cache.as_ref().expect("ke cache after dx");
        assert_eq!(c.dx, 0.2);
        // ν change must rebuild.
        region.ensure_ke_cache(2, 2, 1, 0.2, 0.1, 0.05, 0.25);
        let c = region.ke_cache.as_ref().expect("ke cache after nu");
        assert_eq!(c.nu, 0.25);
        region.invalidate_ke_cache();
        assert!(region.ke_cache.is_none());
    }

    #[test]
    fn solver_region_warm_u_store_seed_clear() {
        let mut region = SolverRegion::new();
        let u = vec![0.1_f32, 0.2, 0.3, 0.4, 0.5, 0.6];
        region.store_warm_u(&u);
        assert!(region.has_warm_u_for(6));
        assert!(!region.has_warm_u_for(3));

        let mut opts = Q1HexSolveOptions {
            pcg_warm_start: true,
            ..Default::default()
        };
        region.seed_options_if_warm(&mut opts, 6);
        assert_eq!(opts.pcg_seed_displacement.as_deref(), Some(u.as_slice()));

        // Explicit seed wins — do not overwrite.
        let mut opts2 = Q1HexSolveOptions {
            pcg_warm_start: true,
            pcg_seed_displacement: Some(vec![9.0; 6]),
            ..Default::default()
        };
        region.seed_options_if_warm(&mut opts2, 6);
        assert_eq!(opts2.pcg_seed_displacement.as_deref(), Some(&[9.0; 6][..]));

        // Warm-start off → no seed injection.
        let mut opts3 = Q1HexSolveOptions::default();
        region.seed_options_if_warm(&mut opts3, 6);
        assert!(opts3.pcg_seed_displacement.is_none());

        // In-place refresh when length matches.
        let u2 = vec![1.0_f32; 6];
        region.store_warm_u(&u2);
        assert_eq!(region.warm_u.as_deref(), Some(u2.as_slice()));

        region.clear_warm_u();
        assert!(!region.has_warm_u_for(6));
        assert!(region.warm_u.is_none());
    }
}

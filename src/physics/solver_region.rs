// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Reusable PCG workspace + operator cache for Q1-hex outer loops (H0 hardware perf).

use super::adjoint::AdjointForwardPhaseTiming;
use super::adjoint_q1_hex::Q1HexSolveOptions;
use super::q1_hex_elasticity::HexStructuredOperatorCache;

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

    /// Ensure uniform-brick operator cache exists when `use_operator_cache` is set.
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
        let needs_new = self.ke_cache.as_ref().is_none_or(|c| {
            c.nx != nx || c.ny != ny || c.nz != nz
        });
        if needs_new {
            self.ke_cache = Some(HexStructuredOperatorCache::new(
                nx, ny, nz, dx, dy, dz, nu,
            ));
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
        if self.warm_u.as_ref().is_some_and(|w| w.len() == u.len()) {
            self.warm_u.as_mut().unwrap().copy_from_slice(u);
        } else {
            self.warm_u = Some(u.to_vec());
        }
    }
}

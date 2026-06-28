// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Host-side staging slab for Q1Hex forward (H1) — reuse buffers across `into_data` syncs.

use burn::tensor::{backend::Backend, Tensor};

/// Pinned host buffers for one Q1-hex solve; avoids per-call `Vec` alloc for ρ, f, mask.
#[derive(Clone, Debug, Default)]
pub struct DeviceSheet {
    pub rho_flat: Vec<f32>,
    pub f_flat: Vec<f32>,
    pub m_flat: Vec<f32>,
    n_nodes: usize,
}

impl DeviceSheet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Grow once to fit `n_nodes` scalar ρ and `n_nodes*3` vector DOFs.
    pub fn ensure_capacity(&mut self, n_nodes: usize) {
        if self.rho_flat.len() < n_nodes {
            self.rho_flat.resize(n_nodes, 0.0);
        }
        let ndof = n_nodes * 3;
        if self.f_flat.len() < ndof {
            self.f_flat.resize(ndof, 0.0);
        }
        if self.m_flat.len() < ndof {
            self.m_flat.resize(ndof, 0.0);
        }
        self.n_nodes = n_nodes;
    }

    #[must_use]
    pub fn n_nodes(&self) -> usize {
        self.n_nodes
    }

    /// HostBridge sync: copy tensor payloads into the reusable slab (still 3 device reads).
    pub fn sync_from_tensors<B: Backend<FloatElem = f32>>(
        &mut self,
        rho: &Tensor<B, 3>,
        body_force: &Tensor<B, 3>,
        boundary_mask: &Tensor<B, 3>,
        n_nodes: usize,
    ) {
        self.ensure_capacity(n_nodes);
        let rho_v = rho.clone().into_data().value;
        let f_v = body_force.clone().into_data().value;
        let m_v = boundary_mask.clone().into_data().value;
        debug_assert_eq!(rho_v.len(), n_nodes);
        debug_assert_eq!(f_v.len(), n_nodes * 3);
        debug_assert_eq!(m_v.len(), n_nodes * 3);
        self.rho_flat[..n_nodes].copy_from_slice(&rho_v[..n_nodes]);
        self.f_flat[..n_nodes * 3].copy_from_slice(&f_v[..n_nodes * 3]);
        self.m_flat[..n_nodes * 3].copy_from_slice(&m_v[..n_nodes * 3]);
    }

    #[must_use]
    pub fn rho_slice(&self) -> &[f32] {
        &self.rho_flat[..self.n_nodes]
    }

    #[must_use]
    pub fn f_slice(&self) -> &[f32] {
        &self.f_flat[..self.n_nodes * 3]
    }

    #[must_use]
    pub fn m_slice(&self) -> &[f32] {
        &self.m_flat[..self.n_nodes * 3]
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Host-side staging slab for Q1Hex forward (H1) — reuse buffers across `into_data` syncs.
//!
//! # Honest boundary (W29-051)
//!
//! [`DeviceSheet`] is a **host IO staging** slab for ρ / body-force / boundary-mask payloads.
//! Capacity reuse and `sync_from_tensors` are measured by unit tests under
//! `cargo test -p umst-manifold device_sheet`. Host reuse does **not** certify fleet physics
//! GREEN, `PRODUCTION_WIRED`, or `MASTER` composition.

use burn::tensor::{backend::Backend, Tensor};

/// W29 deepen cell — DeviceSheet honest fence bundle.
pub const W29_DEVICE_SHEET_DEEPEN_CELL: &str = "W29-051-DEVICE_SHEET";

/// Honest posture tag — host slab reuse research lane (H1).
pub const DEVICE_SHEET_POSTURE_TAG: &str = "honest-device-sheet-host-slab-h1-research-lane";

/// Honest physics posture — slab contracts pass unit tests; does not certify fleet physics GREEN.
pub const DEVICE_SHEET_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by host staging alone.
pub const DEVICE_SHEET_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const DEVICE_SHEET_MASTER: bool = false;

/// Whether host ρ/f/mask slab + `sync_from_tensors` contracts are landed.
pub const DEVICE_SHEET_HOST_SLAB_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const DEVICE_SHEET_HONEST_FENCE: &str =
    "host_slab_landed=true sync_from_tensors_wired=true capacity_reuse_wired=true production_wired=false master_composition_wired=false physics_green=false";

const _: () = assert!(!DEVICE_SHEET_PHYSICS_GREEN);
const _: () = assert!(!DEVICE_SHEET_PRODUCTION_WIRED);
const _: () = assert!(!DEVICE_SHEET_MASTER);
const _: () = assert!(DEVICE_SHEET_HOST_SLAB_LANDED);

/// Typed probe for DeviceSheet posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSheetPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub host_slab_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for DeviceSheet.
#[must_use]
pub fn device_sheet_honest_posture_bundle() -> DeviceSheetPostureProbe {
    DeviceSheetPostureProbe {
        physics_green: DEVICE_SHEET_PHYSICS_GREEN,
        production_wired: DEVICE_SHEET_PRODUCTION_WIRED,
        master: DEVICE_SHEET_MASTER,
        host_slab_landed: DEVICE_SHEET_HOST_SLAB_LANDED,
        honest_fence: DEVICE_SHEET_HONEST_FENCE,
        posture_tag: DEVICE_SHEET_POSTURE_TAG,
        deepen_cell: W29_DEVICE_SHEET_DEEPEN_CELL,
    }
}

/// Host-slab SSOT landed with production/master/physics-green honestly open.
#[must_use]
pub fn device_sheet_posture_honest(probe: &DeviceSheetPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.host_slab_landed
        && probe.honest_fence.contains("host_slab_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.deepen_cell == W29_DEVICE_SHEET_DEEPEN_CELL
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::NdArray;

    type B = NdArray<f32>;

    #[test]
    fn device_sheet_honest_posture_refuses_green_production_master() {
        let probe = device_sheet_honest_posture_bundle();
        assert!(device_sheet_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(probe.host_slab_landed);
        assert_eq!(probe.deepen_cell, W29_DEVICE_SHEET_DEEPEN_CELL);
        assert!(DEVICE_SHEET_HONEST_FENCE.contains("production_wired=false"));
        assert!(DEVICE_SHEET_HONEST_FENCE.contains("physics_green=false"));
        assert!(DEVICE_SHEET_HONEST_FENCE.contains("master_composition_wired=false"));
    }

    #[test]
    fn device_sheet_ensure_capacity_grows_and_reuses() {
        let mut sheet = DeviceSheet::new();
        sheet.ensure_capacity(4);
        assert_eq!(sheet.n_nodes(), 4);
        assert_eq!(sheet.rho_flat.len(), 4);
        assert_eq!(sheet.f_flat.len(), 12);
        assert_eq!(sheet.m_flat.len(), 12);
        let rho_cap = sheet.rho_flat.capacity();
        let f_cap = sheet.f_flat.capacity();
        sheet.ensure_capacity(2);
        assert_eq!(sheet.n_nodes(), 2);
        assert!(sheet.rho_flat.len() >= 4, "ensure_capacity must not shrink");
        assert!(sheet.rho_flat.capacity() >= rho_cap);
        assert!(sheet.f_flat.capacity() >= f_cap);
        assert_eq!(sheet.rho_slice().len(), 2);
        assert_eq!(sheet.f_slice().len(), 6);
        assert_eq!(sheet.m_slice().len(), 6);
    }

    #[test]
    fn device_sheet_sync_from_tensors_copies_payloads() {
        let device = Default::default();
        let n = 2usize;
        let rho =
            Tensor::<B, 3>::from_data(Data::new(vec![0.25_f32, 0.75], [1, n, 1].into()), &device);
        let body = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0, 0.0, 0.0, 0.0, 2.0, 0.0], [1, n, 3].into()),
            &device,
        );
        let mask = Tensor::<B, 3>::from_data(
            Data::new(vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0], [1, n, 3].into()),
            &device,
        );
        let mut sheet = DeviceSheet::new();
        sheet.sync_from_tensors(&rho, &body, &mask, n);
        assert_eq!(sheet.rho_slice(), &[0.25, 0.75]);
        assert_eq!(sheet.f_slice(), &[1.0, 0.0, 0.0, 0.0, 2.0, 0.0]);
        assert_eq!(sheet.m_slice(), &[1.0, 1.0, 1.0, 0.0, 0.0, 0.0]);

        // Second sync reuses capacity; overwrites logical window only.
        let rho2 =
            Tensor::<B, 3>::from_data(Data::new(vec![0.5_f32, 0.5], [1, n, 1].into()), &device);
        sheet.sync_from_tensors(&rho2, &body, &mask, n);
        assert_eq!(sheet.rho_slice(), &[0.5, 0.5]);
        assert!(sheet.rho_flat.capacity() >= 2);
    }
}

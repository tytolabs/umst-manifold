// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Matrix-free linear operators as `f32` vectors for host Krylov drivers
//! ([`super::solvers::krylov_host::gmres_f32_try`]).
//!
//! [`BarMatvecOperator`] wraps `VectorMechanicsSolver::bar_matvec` without re-implementing bar physics.
//!
//! # Honest boundary (W29-061)
//!
//! Host-side [`F32VecLinearOperator`] + [`BarMatvecOperator`] adapt packed nodal vectors to the
//! Burn bar stiffness matvec for GMRES. Shape gates and `into_gmres_matvec` are exercised by
//! `cargo test -p umst-manifold operator`. Not physics GREEN, not `PRODUCTION_WIRED`, not `MASTER`,
//! not OP-5.

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};

use super::error::PhysicsError;
use super::mechanics::VectorMechanicsSolver;

/// W29 deepen cell — host bar matvec operator honest fence bundle.
pub const W29_OPERATOR_DEEPEN_CELL: &str = "W29-061-OPERATOR";

/// Honest posture tag — host Krylov bar adapter landed; fleet production wiring refused.
pub const OPERATOR_POSTURE_TAG: &str = "honest-host-bar-matvec-operator-research-lane";

/// Honest physics posture — unit contracts pass; does not certify fleet physics GREEN.
pub const OPERATOR_PHYSICS_GREEN: bool = false;

/// Production wiring — not claimed by host bar matvec adapter alone.
pub const OPERATOR_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by this module.
pub const OPERATOR_MASTER: bool = false;

/// OP-5 ceremony pin — not claimed by this module.
pub const OPERATOR_OP5: bool = false;

/// Whether [`F32VecLinearOperator`] + [`BarMatvecOperator`] contracts are landed in this module.
pub const OPERATOR_HOST_ADAPTER_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const OPERATOR_HONEST_FENCE: &str =
    "host_bar_matvec_adapter_landed=true|f32_vec_linear_operator_wired=true|into_gmres_matvec_wired=true|production_wired=false|physics_green=false|master=false|op5=false";

/// Typed probe for host bar-matvec operator posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostBarMatvecOperatorPostureProbe {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5: bool,
    pub host_adapter_landed: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
    pub deepen_cell: &'static str,
}

/// Measured honest-posture snapshot for the host bar matvec operator.
#[must_use]
pub fn host_bar_matvec_operator_honest_posture_bundle() -> HostBarMatvecOperatorPostureProbe {
    HostBarMatvecOperatorPostureProbe {
        physics_green: OPERATOR_PHYSICS_GREEN,
        production_wired: OPERATOR_PRODUCTION_WIRED,
        master: OPERATOR_MASTER,
        op5: OPERATOR_OP5,
        host_adapter_landed: OPERATOR_HOST_ADAPTER_LANDED,
        honest_fence: OPERATOR_HONEST_FENCE,
        posture_tag: OPERATOR_POSTURE_TAG,
        deepen_cell: W29_OPERATOR_DEEPEN_CELL,
    }
}

/// Host adapter landed with production / master / OP-5 composition honestly open.
#[must_use]
pub fn host_bar_matvec_operator_posture_honest(probe: &HostBarMatvecOperatorPostureProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && !probe.op5
        && probe.host_adapter_landed
        && probe.deepen_cell == W29_OPERATOR_DEEPEN_CELL
        && probe.posture_tag == OPERATOR_POSTURE_TAG
        && probe
            .honest_fence
            .contains("host_bar_matvec_adapter_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5=false")
}

/// Host-side linear map \( \mathbb{R}^{3N} \to \mathbb{R}^{3N} \) consumable by GMRES-style routines.
pub trait F32VecLinearOperator {
    fn vec_dim(&self) -> usize;

    /// Apply the operator to a packed nodal vector `[u_{0,x}, u_{0,y}, u_{0,z}, …]`.
    fn apply_vec(&mut self, v: &[f32]) -> Result<Vec<f32>, PhysicsError>;
}

/// Bundles bar-network stiffness tensors so [`Self::apply_vec`] delegates to `VectorMechanicsSolver::bar_matvec`.
///
/// Vector length is **`n_v * 3`** for one active batch row embedded into `template` at the `batch_row` index.
pub struct BarMatvecOperator<B: Backend<FloatElem = f32>> {
    template: Tensor<B, 3>,
    batch_row: usize,
    n_v: usize,
    pub(crate) k_axial: Tensor<B, 3>,
    pub(crate) edge_unit: Tensor<B, 3>,
    pub(crate) src_indices: Tensor<B, 3, Int>,
    pub(crate) tgt_indices: Tensor<B, 3, Int>,
    pub(crate) edge_len: Tensor<B, 3>,
    edge_shrink_strain_increment: Option<Tensor<B, 3>>,
}

impl<B: Backend<FloatElem = f32>> BarMatvecOperator<B> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        template: Tensor<B, 3>,
        batch_row: usize,
        n_v: usize,
        k_axial: Tensor<B, 3>,
        edge_unit: Tensor<B, 3>,
        src_indices: Tensor<B, 3, Int>,
        tgt_indices: Tensor<B, 3, Int>,
        edge_len: Tensor<B, 3>,
        edge_shrink_strain_increment: Option<Tensor<B, 3>>,
    ) -> Self {
        Self {
            template,
            batch_row,
            n_v,
            k_axial,
            edge_unit,
            src_indices,
            tgt_indices,
            edge_len,
            edge_shrink_strain_increment,
        }
    }

    /// Packed nodal DOF count for this operator (`n_v * 3`).
    #[must_use]
    pub fn packed_dof_count(&self) -> usize {
        self.n_v.saturating_mul(3)
    }

    /// Consume `self` and return a fallible matvec for [`super::solvers::krylov_host::gmres_f32_try`].
    pub fn into_gmres_matvec(mut self) -> impl FnMut(&[f32]) -> Result<Vec<f32>, PhysicsError> {
        move |v| self.apply_vec(v)
    }
}

impl<B: Backend<FloatElem = f32>> F32VecLinearOperator for BarMatvecOperator<B> {
    fn vec_dim(&self) -> usize {
        self.packed_dof_count()
    }

    fn apply_vec(&mut self, v: &[f32]) -> Result<Vec<f32>, PhysicsError> {
        if v.len() != self.vec_dim() {
            return Err(PhysicsError::ShapeMismatch {
                context: "BarMatvecOperator::apply_vec",
                detail: "packed nodal vector length",
            });
        }
        let device = self.template.device();
        let row: Tensor<B, 3> = Tensor::from_data(
            Data::new(Vec::from(v), Shape::new([1, self.n_v, 3])),
            &device,
        );
        let u_full =
            VectorMechanicsSolver::embed_batch_row(&self.template, self.batch_row, self.n_v, row);
        let ku = VectorMechanicsSolver::bar_matvec(
            u_full,
            &self.k_axial,
            &self.edge_unit,
            &self.src_indices,
            &self.tgt_indices,
            self.n_v,
            self.edge_shrink_strain_increment.as_ref(),
            &self.edge_len,
        );
        // One device→host materialisation for host GMRES (`NdArray` slice is already narrowed).
        let row: Tensor<B, 1> = ku
            .slice([self.batch_row..self.batch_row + 1, 0..self.n_v, 0..3])
            .reshape([self.n_v * 3]);
        Ok(row.into_data().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Data;
    use burn_ndarray::{NdArray, NdArrayDevice};

    use crate::physics::topology::EdgeTopology;

    type B = NdArray<f32>;

    /// Two-node single edge along \(+\hat e_x\): `k=2`, unit tangent, length 1.
    fn single_edge_bar_operator() -> BarMatvecOperator<B> {
        let device = NdArrayDevice::Cpu;
        let n_v = 2usize;
        let template = Tensor::<B, 3>::zeros([1, n_v, 3], &device);
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 1], Shape::new([2, 1])), &device);
        let topo = EdgeTopology::new(edges_b1);
        let src_indices = topo.expand_src_gather_indices(1, 3);
        let tgt_indices = topo.expand_tgt_gather_indices(1, 3);
        let k_axial = Tensor::from_data(Data::new(vec![2.0_f32], Shape::new([1, 1, 1])), &device);
        let edge_unit = Tensor::from_data(
            Data::new(vec![1.0_f32, 0.0, 0.0], Shape::new([1, 1, 3])),
            &device,
        );
        let edge_len = Tensor::from_data(Data::new(vec![1.0_f32], Shape::new([1, 1, 1])), &device);
        BarMatvecOperator::new(
            template,
            0,
            n_v,
            k_axial,
            edge_unit,
            src_indices,
            tgt_indices,
            edge_len,
            None,
        )
    }

    #[test]
    fn operator_honest_posture_refuses_green_production_master_op5() {
        let probe = host_bar_matvec_operator_honest_posture_bundle();
        assert!(host_bar_matvec_operator_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.op5);
        assert_eq!(probe.deepen_cell, "W29-061-OPERATOR");
        assert!(OPERATOR_HOST_ADAPTER_LANDED);
    }

    #[test]
    fn bar_matvec_operator_vec_dim_is_packed_nodal() {
        let op = single_edge_bar_operator();
        assert_eq!(op.vec_dim(), 6);
        assert_eq!(op.packed_dof_count(), 6);
    }

    #[test]
    fn bar_matvec_operator_rejects_shape_mismatch() {
        let mut op = single_edge_bar_operator();
        let err = op
            .apply_vec(&[0.0_f32; 5])
            .expect_err("wrong packed length must ShapeMismatch");
        match err {
            PhysicsError::ShapeMismatch { context, detail } => {
                assert_eq!(context, "BarMatvecOperator::apply_vec");
                assert_eq!(detail, "packed nodal vector length");
            }
            other => panic!("expected ShapeMismatch, got {other:?}"),
        }
    }

    #[test]
    fn bar_matvec_operator_single_edge_axial_extension() {
        let mut op = single_edge_bar_operator();
        // u_src=(1,0,0), u_tgt=(0,0,0) → elong=1 → f=k*elong=2 along +x
        let ku = op
            .apply_vec(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .expect("apply_vec on matching length");
        assert_eq!(ku.len(), 6);
        assert!((ku[0] - 2.0).abs() < 1e-5, "src Fx got {}", ku[0]);
        assert!(ku[1].abs() < 1e-5 && ku[2].abs() < 1e-5);
        assert!((ku[3] + 2.0).abs() < 1e-5, "tgt Fx got {}", ku[3]);
        assert!(ku[4].abs() < 1e-5 && ku[5].abs() < 1e-5);
    }

    #[test]
    fn into_gmres_matvec_preserves_axial_extension() {
        let op = single_edge_bar_operator();
        let mut matvec = op.into_gmres_matvec();
        let ku = matvec(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("gmres matvec");
        assert!((ku[0] - 2.0).abs() < 1e-5);
        assert!((ku[3] + 2.0).abs() < 1e-5);
    }
}

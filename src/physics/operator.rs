// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Matrix-free linear operators as `f32` vectors for host Krylov drivers ([`super::krylov_host::gmres_f32_try`]).
//!
//! [`BarMatvecOperator`] wraps `VectorMechanicsSolver::bar_matvec` without re-implementing bar physics.

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};

use super::error::PhysicsError;
use super::mechanics::VectorMechanicsSolver;

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

    /// Consume `self` and return a fallible matvec for [`super::krylov_host::gmres_f32_try`].
    pub fn into_gmres_matvec(mut self) -> impl FnMut(&[f32]) -> Result<Vec<f32>, PhysicsError> {
        move |v| self.apply_vec(v)
    }
}

impl<B: Backend<FloatElem = f32>> F32VecLinearOperator for BarMatvecOperator<B> {
    fn vec_dim(&self) -> usize {
        self.n_v * 3
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

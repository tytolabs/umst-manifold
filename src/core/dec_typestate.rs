// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Phase 1 §1C staging: DEC incidence and scalar-channel typestates.
//!
//! Makes invalid B₁ layout and out-of-range scalar channel indices unrepresentable or
//! rejectable via [`Result`] — no panics on the public API. Existing [`super::umst_schema`]
//! `SCALAR_*` literals remain the layout SSOT until a later migration pass.

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Int, Tensor};

use super::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

/// Errors surfaced by DEC typestate constructors (total public API).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecTypestateError {
    /// `edges_b1` must have shape `[2, E]` (primal node→edge incidence).
    B1WrongRowCount { rows: usize },
    /// Scalar channel index is outside the pinned layout contract.
    ScalarChannelOutOfRange { index: usize, channel_count: usize },
}

/// Oriented primal **B₁** incidence: nodes → edges, shape `[2, E]`.
#[derive(Clone, Debug)]
pub struct B1Incidence<B: Backend> {
    tensor: Tensor<B, 2, Int>,
}

impl<B: Backend> B1Incidence<B> {
    /// Validate `edges_b1` layout `[2, E]` and wrap as a typed incidence carrier.
    pub fn try_new(edges_b1: Tensor<B, 2, Int>) -> Result<Self, DecTypestateError> {
        let rows = edges_b1.dims()[0];
        if rows != 2 {
            return Err(DecTypestateError::B1WrongRowCount { rows });
        }
        Ok(Self { tensor: edges_b1 })
    }

    /// Borrow the underlying Burn incidence tensor.
    #[inline]
    pub fn as_tensor(&self) -> &Tensor<B, 2, Int> {
        &self.tensor
    }

    /// Consume and return the underlying Burn incidence tensor.
    #[inline]
    pub fn into_tensor(self) -> Tensor<B, 2, Int> {
        self.tensor
    }

    /// Number of oriented edges `E` in the incidence matrix.
    #[inline]
    pub fn n_edges(&self) -> usize {
        self.tensor.dims()[1]
    }

    /// View as [`crate::physics::topology::EdgeTopology`] for existing DEC call sites.
    #[inline]
    pub fn to_edge_topology(&self) -> crate::physics::topology::EdgeTopology<B> {
        crate::physics::topology::EdgeTopology::new(self.tensor.clone())
    }
}

/// Compile-time scalar channel index validated against [`UMST_SCALAR_CHANNEL_COUNT`].
///
/// When `N >= UMST_SCALAR_CHANNEL_COUNT`, [`ScalarChannel::try_index`] returns `None` — the
/// invalid index cannot be obtained without falling back to raw `usize`.
#[derive(Clone, Copy, Debug)]
pub struct ScalarChannel<const N: usize>(PhantomData<()>);

impl<const N: usize> ScalarChannel<N> {
    /// `Some(N)` only when `N` is inside the pinned layout contract.
    pub const fn try_index() -> Option<usize> {
        if N < UMST_SCALAR_CHANNEL_COUNT {
            Some(N)
        } else {
            None
        }
    }
}

/// Runtime-validated scalar channel index (staging alternative to raw `usize`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalarChannelIdx(usize);

impl ScalarChannelIdx {
    /// Reject indices outside `0 .. UMST_SCALAR_CHANNEL_COUNT`.
    pub fn try_new(index: usize) -> Result<Self, DecTypestateError> {
        if index >= UMST_SCALAR_CHANNEL_COUNT {
            return Err(DecTypestateError::ScalarChannelOutOfRange {
                index,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            });
        }
        Ok(Self(index))
    }

    /// Channel column index for tensor slicing / projection.
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn_ndarray::NdArray;
    use burn::tensor::Tensor;

    type B = NdArray;

    #[test]
    fn dec_scalar_channel_runtime_rejects_out_of_range_index() {
        let err = ScalarChannelIdx::try_new(UMST_SCALAR_CHANNEL_COUNT).unwrap_err();
        assert_eq!(
            err,
            DecTypestateError::ScalarChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
        assert!(ScalarChannelIdx::try_new(UMST_SCALAR_CHANNEL_COUNT + 1).is_err());
        assert_eq!(ScalarChannelIdx::try_new(0).unwrap().index(), 0);
    }

    #[test]
    fn dec_scalar_channel_const_generic_makes_overflow_unrepresentable() {
        assert_eq!(ScalarChannel::<0>::try_index(), Some(0));
        assert_eq!(
            ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT - 1 }>::try_index(),
            Some(UMST_SCALAR_CHANNEL_COUNT - 1)
        );
        assert!(ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT }>::try_index().is_none());
        assert!(ScalarChannel::<{ UMST_SCALAR_CHANNEL_COUNT + 1 }>::try_index().is_none());
    }

    #[test]
    fn dec_b1_incidence_rejects_non_two_row_layout() {
        let device = Default::default();
        let bad: Tensor<B, 2, Int> =
            Tensor::zeros([3, 4], &device);
        let err = B1Incidence::try_new(bad).unwrap_err();
        assert_eq!(err, DecTypestateError::B1WrongRowCount { rows: 3 });

        let ok: Tensor<B, 2, Int> = Tensor::zeros([2, 5], &device);
        assert_eq!(B1Incidence::try_new(ok).unwrap().n_edges(), 5);
    }

    /// Toy 2-node / 1-edge mesh: witness \(\langle B_1^\top \omega, u\rangle = \langle \omega, B_1 u\rangle\)
    /// for unweighted Frobenius pairings (graph DEC adjoint; no solver).
    mod dec_graph_adjoint_identity {
        use super::*;
        use burn::tensor::{Data, Shape};
        use crate::physics::dec_primal::{
            primal_divergence_from_edge_flux_topo, primal_scalar_edge_increment,
        };

        fn tensor_inner(a: Tensor<B, 3>, b: Tensor<B, 3>) -> f32 {
            a.mul(b).sum().into_scalar()
        }

        /// Single oriented edge `0 → 1` on two vertices, wrapped as [`B1Incidence`].
        fn toy_two_node_one_edge_b1() -> B1Incidence<B> {
            let device = Default::default();
            let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
                Data::new(vec![0i64, 1], Shape::new([2, 1])),
                &device,
            );
            B1Incidence::try_new(edges_b1).expect("toy mesh B1 layout [2, 1]")
        }

        #[test]
        fn b1_transpose_composition_matches_graph_adjoint_pairing() {
            let b1 = toy_two_node_one_edge_b1();
            assert_eq!(b1.n_edges(), 1);
            let topo = b1.to_edge_topology();
            let device = Default::default();

            let omega = Tensor::from_data(
                Data::new(vec![1.2_f32, -0.45], Shape::new([1, 2, 1])),
                &device,
            );
            let edge_flux = Tensor::from_data(Data::new(vec![0.7_f32], Shape::new([1, 1, 1])), &device);
            let nodal_template = Tensor::zeros([1, 2, 1], &device);

            let b1t_omega = primal_scalar_edge_increment(omega.clone(), &topo).neg();
            let b1_u =
                primal_divergence_from_edge_flux_topo(edge_flux.clone(), &topo, &nodal_template);

            let lhs = tensor_inner(b1t_omega, edge_flux);
            let rhs = tensor_inner(omega, b1_u);
            assert!(
                (lhs - rhs).abs() < 1.0e-5,
                "⟨B₁ᵀω,u⟩ = {lhs} must equal ⟨ω,B₁u⟩ = {rhs} on 2-node 1-edge toy mesh"
            );
        }

        #[test]
        fn b1_transpose_b1_is_two_node_graph_laplacian_on_toy_mesh() {
            let b1 = toy_two_node_one_edge_b1();
            let topo = b1.to_edge_topology();
            let device = Default::default();

            let omega = Tensor::from_data(
                Data::new(vec![2.0_f32, 5.0], Shape::new([1, 2, 1])),
                &device,
            );
            let nodal_template = Tensor::zeros([1, 2, 1], &device);

            let grad = primal_scalar_edge_increment(omega.clone(), &topo);
            let lap_omega = primal_divergence_from_edge_flux_topo(grad, &topo, &nodal_template);
            let v: Vec<f32> = lap_omega.into_data().value;
            assert_eq!(v.len(), 2);
            let increment = 5.0_f32 - 2.0_f32;
            assert!((v[0] - increment).abs() < 1.0e-5);
            assert!((v[1] + increment).abs() < 1.0e-5);
            assert!((v[0] + v[1]).abs() < 1.0e-5, "row-sum zero (closed incidence)");
        }
    }
}

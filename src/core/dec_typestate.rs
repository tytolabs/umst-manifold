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
    /// Nodal scalar tensor width does not match the compile-time layout witness.
    ScalarWidthMismatch { expected: usize, found: usize },
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

<<<<<<< HEAD
/// Channel selector for [`super::tensors::UnifiedMaterialStateTensor::project_scalar_channel`] /
/// [`super::tensors::UnifiedMaterialStateTensor::write_scalar_channel`].
///
/// [`usize`] preserves legacy call sites; [`ScalarChannelIdx`] carries a layout-validated index.
pub trait ScalarChannelSelector {
    /// Scalar column index into `scalar_features`.
    fn scalar_channel_index(&self) -> usize;
}

impl ScalarChannelSelector for usize {
    #[inline]
    fn scalar_channel_index(&self) -> usize {
        *self
    }
}

impl ScalarChannelSelector for ScalarChannelIdx {
    #[inline]
    fn scalar_channel_index(&self) -> usize {
        self.index()
    }
}

/// Staging DEC + scalar-layout bundle (`math-verified-umst-typestate`).
///
/// Composes a validated [`B1Incidence`] with a runtime [`ScalarChannelIdx`], pinned to
/// [`UMST_SCALAR_CHANNEL_COUNT`] at compile time via [`ScalarChannel`]. Distinct from the
/// proof-carrying gateway type [`super::tensors::VerifiedUMST`].
#[derive(Clone, Debug)]
pub struct VerifiedUMST<B: Backend> {
    b1: B1Incidence<B>,
    channel: ScalarChannelIdx,
    _layout: PhantomData<ScalarChannel<UMST_SCALAR_CHANNEL_COUNT>>,
}

impl<B: Backend> VerifiedUMST<B> {
    /// Pinned nodal scalar width from [`UMST_SCALAR_CHANNEL_COUNT`].
    pub const CHANNEL_COUNT: usize = UMST_SCALAR_CHANNEL_COUNT;

    /// Validate incidence layout, scalar tensor width, and channel index; assemble staging bundle.
    pub fn try_assemble(
        edges_b1: Tensor<B, 2, Int>,
        scalar_cols: usize,
        channel: usize,
    ) -> Result<Self, DecTypestateError> {
        if scalar_cols != Self::CHANNEL_COUNT {
            return Err(DecTypestateError::ScalarWidthMismatch {
                expected: Self::CHANNEL_COUNT,
                found: scalar_cols,
            });
        }
        let b1 = B1Incidence::try_new(edges_b1)?;
        let channel = ScalarChannelIdx::try_new(channel)?;
        Ok(Self {
            b1,
            channel,
            _layout: PhantomData,
        })
    }

    /// Borrow the validated **B₁** incidence carrier.
    #[inline]
    pub fn b1(&self) -> &B1Incidence<B> {
        &self.b1
    }

    /// Active scalar channel index inside the pinned layout.
    #[inline]
    pub fn channel(&self) -> ScalarChannelIdx {
        self.channel
    }

    /// Consume and return the underlying incidence tensor.
    #[inline]
    pub fn into_b1(self) -> B1Incidence<B> {
        self.b1
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
}

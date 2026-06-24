// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Staging DEC + scalar-layout bundle (`math-verified-umst-typestate`).
//!
//! Composes [`super::dec_typestate::B1Incidence`] with [`super::dec_typestate::ScalarChannelIdx`],
//! pinned to [`super::umst_schema::UMST_SCALAR_CHANNEL_COUNT`] at compile time via
//! [`super::dec_typestate::ScalarChannel`]. Distinct from the proof-carrying gateway type
//! [`super::tensors::VerifiedUMST`].

use std::marker::PhantomData;

use burn::tensor::{backend::Backend, Int, Tensor};

use super::dec_typestate::{
    B1Incidence, DecTypestateError, ScalarChannel, ScalarChannelIdx,
};
use super::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

/// Staging product of validated **B₁** incidence and scalar-channel layout witnesses.
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

    /// Consume and return the underlying incidence carrier.
    #[inline]
    pub fn into_b1(self) -> B1Incidence<B> {
        self.b1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::Tensor;
    use burn_ndarray::NdArray;

    type B = NdArray;

    #[test]
    fn verified_umst_staging_assembles_b1_and_scalar_channel() {
        let device = Default::default();
        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);
        let staging =
            VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, 0).expect("valid staging");
        assert_eq!(staging.b1().n_edges(), 2);
        assert_eq!(staging.channel().index(), 0);
        assert_eq!(VerifiedUMST::<B>::CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT);
    }

    #[test]
    fn verified_umst_staging_rejects_scalar_width_and_channel_mismatch() {
        let device = Default::default();
        let edges: Tensor<B, 2, Int> = Tensor::zeros([2, 2], &device);

        let width_err = VerifiedUMST::try_assemble(edges.clone(), UMST_SCALAR_CHANNEL_COUNT - 1, 0)
            .unwrap_err();
        assert_eq!(
            width_err,
            DecTypestateError::ScalarWidthMismatch {
                expected: UMST_SCALAR_CHANNEL_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT - 1,
            }
        );

        let channel_err =
            VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT)
                .unwrap_err();
        assert_eq!(
            channel_err,
            DecTypestateError::ScalarChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
    }
}

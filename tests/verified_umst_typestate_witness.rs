//! Witness spike: staging [`umst_manifold::core::dec_typestate::VerifiedUMST`] bundles
//! validated B₁ incidence with pinned scalar-channel layout.

use burn::tensor::{backend::Backend, Int, Tensor};
use burn_ndarray::NdArray;
use umst_manifold::core::dec_typestate::{DecTypestateError, VerifiedUMST};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

type B = NdArray;

fn toy_b1_two_edges(device: &<B as Backend>::Device) -> Tensor<B, 2, Int> {
    Tensor::zeros([2, 2], device)
}

#[test]
fn verified_umst_staging_assembles_b1_and_scalar_channel() {
    let device = Default::default();
    let edges = toy_b1_two_edges(&device);
    let staging =
        VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, 0).expect("valid staging");
    assert_eq!(staging.b1().n_edges(), 2);
    assert_eq!(staging.channel().index(), 0);
    assert_eq!(VerifiedUMST::<B>::CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT);
}

#[test]
fn verified_umst_staging_rejects_scalar_width_mismatch() {
    let device = Default::default();
    let edges = toy_b1_two_edges(&device);

    let too_narrow =
        VerifiedUMST::try_assemble(edges.clone(), UMST_SCALAR_CHANNEL_COUNT - 1, 0).unwrap_err();
    assert_eq!(
        too_narrow,
        DecTypestateError::ScalarWidthMismatch {
            expected: UMST_SCALAR_CHANNEL_COUNT,
            found: UMST_SCALAR_CHANNEL_COUNT - 1,
        }
    );

    let too_wide = VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT + 1, 0).unwrap_err();
    assert_eq!(
        too_wide,
        DecTypestateError::ScalarWidthMismatch {
            expected: UMST_SCALAR_CHANNEL_COUNT,
            found: UMST_SCALAR_CHANNEL_COUNT + 1,
        }
    );
}

#[test]
fn verified_umst_staging_rejects_scalar_channel_out_of_range() {
    let device = Default::default();
    let edges = toy_b1_two_edges(&device);

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

#[test]
fn verified_umst_staging_assemble_round_trip_preserves_b1_and_channel() {
    let device = Default::default();
    let edges = toy_b1_two_edges(&device);
    let channel_idx = UMST_SCALAR_CHANNEL_COUNT.saturating_sub(1);

    let staging = VerifiedUMST::try_assemble(edges, UMST_SCALAR_CHANNEL_COUNT, channel_idx)
        .expect("assemble");
    assert_eq!(staging.b1().n_edges(), 2);
    assert_eq!(staging.channel().index(), channel_idx);

    let b1 = staging.into_b1();
    let round_trip =
        VerifiedUMST::try_assemble(b1.into_tensor(), UMST_SCALAR_CHANNEL_COUNT, channel_idx)
            .expect("round-trip assemble");

    assert_eq!(round_trip.b1().n_edges(), 2);
    assert_eq!(round_trip.channel().index(), channel_idx);
    assert_eq!(VerifiedUMST::<B>::CHANNEL_COUNT, UMST_SCALAR_CHANNEL_COUNT);
}

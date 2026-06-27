// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! R4: `VoxelDensity` golden parity — zero latent offset matches legacy `forward_batched`.

#![cfg(feature = "topology-density-evolution")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use rand::{rngs::StdRng, Rng, SeedableRng};

use umst_manifold::ai::topology::{DensityNet, VoxelDensity};
use umst_manifold::core::traits::{DesignLatent, DesignRepresentation};

type B = NdArray<f32>;

#[test]
fn voxel_density_decode_matches_forward_batched_seed42() {
    let dev = NdArrayDevice::default();
    let repr = VoxelDensity::new(DensityNet::new(16, &dev));

    let mut rng = StdRng::seed_from_u64(42);
    let n = 27_usize;
    let coords: Vec<f32> = (0..n)
        .flat_map(|_| {
            [
                rng.gen_range(-0.5_f32..0.5),
                rng.gen_range(-0.5_f32..0.5),
                rng.gen_range(0.0_f32..1.0),
            ]
        })
        .collect();
    let coords_t = Tensor::<B, 3>::from_data(Data::new(coords, Shape::new([1, n, 3])), &dev);

    let baseline = repr.density_net.forward_batched(coords_t.clone());
    let latent = DesignLatent {
        tensor: Tensor::<B, 2>::zeros([1, 1], &dev),
    };
    let geom = repr.decode(&latent, coords_t).expect("decode");
    let decoded = geom.density;

    let b: Vec<f32> = baseline.into_data().value;
    let d: Vec<f32> = decoded.into_data().value;
    assert_eq!(b.len(), d.len());
    for (i, (x, y)) in b.iter().zip(d.iter()).enumerate() {
        assert!(
            (x - y).abs() < 1e-6,
            "parity fail @ {i}: baseline {x} decoded {y}"
        );
    }
}

#[test]
fn voxel_density_repr_id_stable() {
    let dev = NdArrayDevice::default();
    let repr: VoxelDensity<B> = VoxelDensity::new(DensityNet::new(8, &dev));
    assert_eq!(repr.repr_id(), "umst.design.voxel_density");
}

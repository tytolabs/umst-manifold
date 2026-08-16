// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! R4: `ImplicitField` decode scaffold compiles and returns finite shapes.

#![cfg(feature = "design-implicit-field")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::ai::implicit_field::{ImplicitField, ImplicitFieldNet};
use umst_manifold::core::traits::{DesignLatent, DesignRepresentation};

type B = NdArray<f32>;

#[test]
fn implicit_field_decode_finite_bar_fixture() {
    let dev = NdArrayDevice::default();
    let net = ImplicitFieldNet::<B>::new(4, 8, &dev);
    let field = ImplicitField::new(net, 1.0);
    let n = 5_usize;
    let coords = Tensor::<B, 3>::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, 0.25, 0.0, 0.0, 0.5, 0.0, 0.0, 0.75, 0.0, 0.0, 1.0, 0.0, 0.0,
            ],
            Shape::new([1, n, 3]),
        ),
        &dev,
    );
    let latent = DesignLatent {
        tensor: Tensor::<B, 2>::zeros([1, 4], &dev),
    };
    let geom = field.decode(&latent, coords).expect("decode");
    assert_eq!(geom.density.dims(), [1, n, 1]);
    assert!(geom.signed_distance.is_some());
    let vals: Vec<f32> = geom.density.into_data().value;
    assert!(vals.iter().all(|x| x.is_finite()));
}

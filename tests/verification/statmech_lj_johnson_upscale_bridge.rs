// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! **P4C — Johnson / virial bridge:** [`upscale_potentials`] on **`[B,4]`** (Burn **virial surrogate** **`K_T`**)
//! vs host [`upscale_potentials_b4_johnson_reference_bulk_modulus_host`] (**Johnson 1993** **`f64`**).

use approx::assert_abs_diff_eq;
use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::solvers::statistical_mechanics::{
    upscale_potentials, upscale_potentials_b4_johnson_reference_bulk_modulus_host,
};

type B = NdArray<f32>;

#[test]
fn upscale_bridge_b4_virial_batch_tracks_johnson_order_one_dilute_grid() {
    let dev = NdArrayDevice::Cpu;
    let t_star = 2.0_f64;
    let eps = 1.0_f32;
    let sig = 0.85_f32;
    let mut rows = Vec::new();
    for k in 0..4 {
        let rho = (0.02_f64 + 0.03_f64 * f64::from(k)) as f32;
        rows.push(eps);
        rows.push(sig);
        rows.push(rho);
        rows.push(t_star as f32);
    }
    let batch = 4usize;
    let lj: Tensor<B, 2> = Tensor::from_data(Data::new(rows, Shape::new([batch, 4])), &dev);
    let (k_virial, _) = upscale_potentials(lj.clone())
        .expect("upscale_potentials b4 virial batch for johnson bridge");
    let k_johnson = upscale_potentials_b4_johnson_reference_bulk_modulus_host(lj)
        .expect("johnson reference bulk modulus host [B,4]");
    let kv = k_virial.into_data().value;
    assert_eq!(kv.len(), batch);
    assert_eq!(k_johnson.len(), batch);
    for i in 0..batch {
        let ratio = f64::from(kv[i]) / k_johnson[i];
        assert!(
            ratio > 0.2 && ratio < 5.0,
            "row {i}: virial {} vs johnson {} ratio {}",
            kv[i],
            k_johnson[i],
            ratio
        );
    }
}

#[test]
fn upscale_bridge_johnson_host_matches_physical_bulk_modulus_scalar() {
    let dev = NdArrayDevice::Cpu;
    use umst_manifold::physics::solvers::statistical_mechanics::physical_bulk_modulus_johnson1993;
    let rho = 0.18_f64;
    let t = 2.2_f64;
    let e = 1.05_f64;
    let s = 0.9_f64;
    let lj: Tensor<B, 2> = Tensor::from_data(
        Data::new(
            vec![e as f32, s as f32, rho as f32, t as f32],
            Shape::new([1, 4]),
        ),
        &dev,
    );
    let v = upscale_potentials_b4_johnson_reference_bulk_modulus_host(lj)
        .expect("johnson reference bulk modulus host single row");
    assert_eq!(v.len(), 1);
    assert_abs_diff_eq!(
        v[0],
        physical_bulk_modulus_johnson1993(rho, t, e, s),
        epsilon = 5e-5_f64
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP P3.6 — `UnifiedMaterialStateTensor::typed_views()` parity.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{DamageField, HumidityField, TemperatureField};
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
use umst_manifold::core::umst_schema::{
    SCALAR_DAMAGE, SCALAR_HUMIDITY, SCALAR_TEMPERATURE, UMST_SCALAR_CHANNEL_COUNT,
};
#[cfg(feature = "thmc-coupled")]
use umst_manifold::physics::solvers::ThmcState;
#[cfg(feature = "thmc-coupled")]
use umst_manifold::physics::thmc_umst_sync::sync_thmc_to_umst;

type B = NdArray<f32>;

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

fn toy_umst(n: usize, t: f32, h: f32, d: f32) -> UnifiedMaterialStateTensor<B> {
    let dev = device();
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let mut flat = vec![0.0_f32; n * f];
    flat[SCALAR_TEMPERATURE] = t;
    flat[SCALAR_HUMIDITY] = h;
    flat[SCALAR_DAMAGE] = d;
    if n > 1 {
        flat[f + SCALAR_TEMPERATURE] = t + 1.0;
        flat[f + SCALAR_HUMIDITY] = h + 0.1;
        flat[f + SCALAR_DAMAGE] = d + 0.05;
    }
    let scalars = Tensor::from_data(Data::new(flat, Shape::new([n, f])), &dev);
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features: scalars,
        vector_features: Tensor::<B, 3>::zeros([n, 1, 3], &dev),
        matrix_features: Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev),
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
        policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

fn assert_plan_field_matches_column(
    plan: &burn::tensor::Tensor<B, 3>,
    umst: &UnifiedMaterialStateTensor<B>,
    channel: usize,
    node: usize,
) {
    assert_eq!(plan.dims(), [1, umst.scalar_features.dims()[0], 1]);
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let expected = umst.scalar_features.clone().into_data().value[node * f + channel];
    let actual = plan.clone().into_data().value[node];
    assert!((actual - expected).abs() < 1e-6);
}

#[test]
fn typed_views_lifts_scalar_columns_to_plan_fields() {
    let n = 3usize;
    let umst = toy_umst(n, 305.0, 0.55, 0.2);
    let views = umst
        .typed_views()
        .expect("UnifiedMaterialStateTensor::typed_views lifts scalar columns to plan fields (FP §6 Track G inverse read morphism)");
    assert_plan_field_matches_column(views.temperature.as_tensor(), &umst, SCALAR_TEMPERATURE, 0);
    assert_plan_field_matches_column(views.humidity.as_tensor(), &umst, SCALAR_HUMIDITY, 0);
    assert_plan_field_matches_column(views.damage.as_tensor(), &umst, SCALAR_DAMAGE, 0);
    assert_plan_field_matches_column(views.temperature.as_tensor(), &umst, SCALAR_TEMPERATURE, 2);
}

#[test]
fn scalar_channel_shims_match_typed_views() {
    let umst = toy_umst(2, 300.0, 0.5, 0.1);
    let views = umst
        .typed_views()
        .expect("UnifiedMaterialStateTensor::typed_views for scalar channel shim parity (FP §6 Track G inverse read morphism)");
    let t = umst
        .temperature_scalar_channel()
        .expect("temperature_scalar_channel shim matches typed_views temperature plan field (FP §6 Track G inverse read morphism)");
    let h = umst
        .humidity_scalar_channel()
        .expect("humidity_scalar_channel shim matches typed_views humidity plan field (FP §6 Track G inverse read morphism)");
    let d = umst
        .damage_scalar_channel()
        .expect("damage_scalar_channel shim matches typed_views damage plan field (FP §6 Track G inverse read morphism)");
    assert_eq!(
        t.as_tensor().clone().into_data().value,
        views.temperature.as_tensor().clone().into_data().value
    );
    assert_eq!(
        h.as_tensor().clone().into_data().value,
        views.humidity.as_tensor().clone().into_data().value
    );
    assert_eq!(
        d.as_tensor().clone().into_data().value,
        views.damage.as_tensor().clone().into_data().value
    );
}

#[cfg(feature = "thmc-coupled")]
#[test]
fn typed_views_roundtrip_after_sync_thmc() {
    let dev = device();
    let n = 2usize;
    let mut umst = toy_umst(n, 100.0, 0.4, 0.2);
    let state = ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 310.0, &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.6, &dev),
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        Tensor::<B, 3>::zeros([1, n, 1], &dev),
        Tensor::<B, 3>::full([1, n, 1], 0.25, &dev),
        0.0,
    );
    sync_thmc_to_umst(&state, &mut umst).expect(
        "sync_thmc_to_umst on toy UMST scalar channels before typed_views roundtrip (FP §6 Track G inverse read morphism)",
    );
    let views = umst.typed_views().expect(
        "UnifiedMaterialStateTensor::typed_views after sync_thmc_to_umst roundtrip on scalar channels (FP §6 Track G inverse read morphism)",
    );
    let eps = 1e-5_f32;
    for node in 0..n {
        let t = views.temperature.as_tensor().clone().into_data().value[node];
        let h = views.humidity.as_tensor().clone().into_data().value[node];
        let d = views.damage.as_tensor().clone().into_data().value[node];
        assert!(
            (t - state
                .thermal
                .temperature
                .as_tensor()
                .clone()
                .into_data()
                .value[node])
                .abs()
                < eps
        );
        assert!((h - state.hydro.humidity.as_tensor().clone().into_data().value[node]).abs() < eps);
        assert!((d - state.damage.as_tensor().clone().into_data().value[node]).abs() < eps);
    }
}

// Compile-time witness: shims return distinct field types.
#[allow(dead_code)]
fn _typed_view_types_compile(
    umst: &UnifiedMaterialStateTensor<B>,
) -> (TemperatureField<B>, HumidityField<B>, DamageField<B>) {
    (
        umst
            .temperature_scalar_channel()
            .expect("temperature scalar channel in typed-view compile witness (FP §6 Track G inverse read morphism)"),
        umst
            .humidity_scalar_channel()
            .expect("humidity scalar channel in typed-view compile witness (FP §6 Track G inverse read morphism)"),
        umst
            .damage_scalar_channel()
            .expect("damage scalar channel in typed-view compile witness (FP §6 Track G inverse read morphism)"),
    )
}

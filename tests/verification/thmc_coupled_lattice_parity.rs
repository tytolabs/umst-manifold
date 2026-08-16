// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![cfg(feature = "thmc-coupled")]

//! R14-6 — THMC coupled fixture with non-zero cross-field terms.

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::field::{Field, TemperatureField};
use umst_manifold::physics::solvers::{
    reaction_extent_rate_tensor, ReactionExtentKinetics, ThmcState,
};

type B = NdArray<f32>;

#[test]
fn r14_thmc_coupled_cross_field_terms_nonzero() {
    let device = NdArrayDevice::default();
    let kinetics = ReactionExtentKinetics::default();
    let t_hot: Tensor<B, 3> = Tensor::from_data(
        Data::new(vec![350.0_f32, 300.0], Shape::new([1, 2, 1])),
        &device,
    );
    let t_cold = Tensor::from_data(
        Data::new(vec![300.0_f32, 300.0], Shape::new([1, 2, 1])),
        &device,
    );
    let alpha = Tensor::<B, 3>::zeros([1, 2, 1], &device);
    let rate_hot = reaction_extent_rate_tensor(&kinetics, alpha.clone(), t_hot.clone(), &device);
    let rate_cold = reaction_extent_rate_tensor(&kinetics, alpha.clone(), t_cold, &device);
    let delta = (rate_hot - rate_cold).abs().sum().into_scalar();
    assert!(delta > 0.0, "cross-field T→α coupling must be visible, delta={delta}");

    let state = ThmcState::from_fields(
        Field::new(t_hot),
        Field::new(Tensor::from_data(
            Data::new(vec![0.2_f32, 0.8], Shape::new([1, 2, 1])),
            &device,
        )),
        Field::new(Tensor::<B, 3>::zeros([1, 2, 3], &device)),
        Field::new(alpha),
        Field::new(Tensor::<B, 3>::zeros([1, 2, 1], &device)),
        0.0,
    );
    let _t_field: &TemperatureField<B> = &state.thermal.temperature;
    let rate_from_state = reaction_extent_rate_tensor(
        &kinetics,
        state.chemical.reaction_extent.as_tensor().clone(),
        state.thermal.temperature.as_tensor().clone(),
        &device,
    );
    let host_rate = rate_from_state.into_data().value[0];
    assert!(host_rate.is_finite() && host_rate > 0.0);
    let h = state.hydro.humidity.as_tensor().clone().into_data().value;
    assert_ne!(h[0], h[1], "humidity gradient required for coupled fixture");
}

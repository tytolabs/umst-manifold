// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Milestone **2.4** — `statistical_mechanics` virial bridge → **mechanics** / **fracture** hooks and
//! autodiff parity on **physical pressure** w.r.t. \(\rho^*\) (Burn reverse mode vs finite differences),
//! matching \(K_T=\rho(\partial P/\partial\rho)_T\) with \(\partial P/\partial\rho\) from the same \(P(\rho^*)\) graph.

use approx::assert_abs_diff_eq;
use burn::backend::Autodiff;
use burn::tensor::{
    backend::{AutodiffBackend, Backend as BackendTrait},
    Data, Shape, Tensor,
};
use burn_ndarray::NdArray;

use umst_manifold::physics::mechanics::scale_stiffness_young_first_channel_with_statmech_ratio;
use umst_manifold::physics::solvers::statistical_mechanics::{
    lj_mayer_b2_star_tensor, lj_virial_b3_star_tensor, reduced_pressure_lj_virial_third_order,
    VIADU_K_REF_F32,
};

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

/// \(\partial P_{\mathrm{phys}}/\partial \rho^*\) with \(P_{\mathrm{phys}}=(\varepsilon/\sigma^3)P^*\) at fixed \((\varepsilon,\sigma,T^*)\).
fn analytic_dp_phys_drho(eps: f32, sig: f32, rho: f32, t: f32) -> f32 {
    let dev = <NdArray<f32> as BackendTrait>::Device::default();
    let t_t = Tensor::<NdArray<f32>, 2>::from_data(Data::new(vec![t], Shape::new([1, 1])), &dev);
    let b2v = lj_mayer_b2_star_tensor(t_t.clone()).into_scalar();
    let b3v = lj_virial_b3_star_tensor(t_t).into_scalar();
    let dp_star = t + 2.0 * t * b2v * rho + 3.0 * t * b3v * rho * rho;
    dp_star * eps / sig.powi(3)
}

fn p_phys_inner(eps: f32, sig: f32, rho: f32, t: f32, dev: &<NdArray<f32> as BackendTrait>::Device) -> f32 {
    let rho_i = Tensor::<Inner, 2>::from_data(Data::new(vec![rho], Shape::new([1, 1])), dev);
    let t_i = Tensor::<Inner, 2>::full([1, 1], t, dev);
    let sig_cu = Tensor::<Inner, 2>::full([1, 1], sig, dev)
        .mul(Tensor::<Inner, 2>::full([1, 1], sig, dev))
        .mul(Tensor::<Inner, 2>::full([1, 1], sig, dev));
    reduced_pressure_lj_virial_third_order(rho_i, t_i)
        .mul(Tensor::<Inner, 2>::full([1, 1], eps, dev).div(sig_cu))
        .into_scalar()
}

#[test]
fn statmech_virial_pressure_autodiff_matches_fd_wrt_rho_star() {
    let dev = Default::default();
    let eps = 1.0_f32;
    let sig = 0.9_f32;
    let t = 2.5_f32;
    let rho0 = 0.15_f32;
    let h = 2.0e-4_f32;

    let rho_ad = Tensor::<AD, 2>::from_data(Data::new(vec![rho0], Shape::new([1, 1])), &dev).require_grad();
    let eps_t = Tensor::<AD, 2>::full([1, 1], eps, &dev);
    let sig_t = Tensor::<AD, 2>::full([1, 1], sig, &dev);
    let t_t = Tensor::<AD, 2>::full([1, 1], t, &dev);
    let sig_cu = sig_t.clone().mul(sig_t.clone()).mul(sig_t.clone());
    let p_phys = reduced_pressure_lj_virial_third_order(rho_ad.clone(), t_t).mul(eps_t.div(sig_cu));
    let loss = p_phys.sum();
    let grads = loss.backward();
    let g = rho_ad
        .grad(&grads)
        .expect("grad w.r.t. rho*")
        .into_data()
        .value[0];

    let p_plus = p_phys_inner(eps, sig, rho0 + h, t, &dev);
    let p_minus = p_phys_inner(eps, sig, rho0 - h, t, &dev);
    let fd = (p_plus - p_minus) / (2.0 * h);
    let want = analytic_dp_phys_drho(eps, sig, rho0, t);
    assert_abs_diff_eq!(want, fd, epsilon = 5.0e-4_f32);
    let denom = fd.abs().max(1e-9_f32);
    assert!((g - fd).abs() / denom < 0.02, "autograd={g} fd={fd} analytic={want}");
}

#[test]
fn statmech_mechanics_stiffness_scale_non_panic_and_ratio() {
    let dev = Default::default();
    let batch = 1usize;
    let n = 3usize;
    let lj = Tensor::<NdArray<f32>, 2>::from_data(
        Data::new(vec![1.0_f32, 1.0_f32, 0.2_f32, 2.0_f32], Shape::new([1, 4])),
        &dev,
    );
    let stiff = Tensor::<NdArray<f32>, 3>::full([batch, n, 2], 200.0_f32, &dev);
    let out = scale_stiffness_young_first_channel_with_statmech_ratio(stiff, lj).unwrap();
    assert_eq!(out.dims(), [batch, n, 2]);
    let e0 = out.clone().slice([0..batch, 0..n, 0..1]).into_data().value[0];
    assert!(e0.is_finite());
    assert_abs_diff_eq!(e0, 200.0_f32, epsilon = 1.0e-3_f32);
    assert_abs_diff_eq!(VIADU_K_REF_F32, 0.1249024_f32, epsilon = 1.0e-4_f32);
}

#[cfg(feature = "fracture-at2")]
#[test]
fn statmech_fracture_gc_scale_non_panic() {
    use umst_manifold::physics::solvers::fracture_field::gc_bn1_scaled_by_statmech_gamma_ratio;
    let dev = Default::default();
    let lj = Tensor::<NdArray<f32>, 2>::from_data(
        Data::new(vec![1.0_f32, 1.0_f32, 0.2_f32, 2.0_f32], Shape::new([1, 4])),
        &dev,
    );
    let gc0 = Tensor::<NdArray<f32>, 2>::full([1, 1], 42.0_f32, &dev);
    let gc1 = gc_bn1_scaled_by_statmech_gamma_ratio(gc0, lj).unwrap();
    assert!(gc1.into_scalar().is_finite());
}

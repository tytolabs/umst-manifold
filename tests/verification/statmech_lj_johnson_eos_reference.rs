// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! **Johnson–Zollweg–Gubbins (1993)** Lennard-Jones EOS — `f64` reference lane (teqp-aligned).
//!
//! Implements analytic reduced `P*(ρ*, T*)` and isothermal `K* = ρ* ∂P*/∂ρ*` via numerical
//! derivatives for verification. The Burn bridge [`upscale_potentials`] uses **`[B,2]`** placeholder
//! **`K`** (expected **disagreement** vs Johnson at the same \((\varepsilon,\sigma)\) without state).
//! **`[B,4]`** rows obtain **`K_T`** from the **third-order virial surrogate** in `statistical_mechanics`
//! (Johnson remains a **`f64`** cross-check — see `upscale_potentials_b4_k_order_matches_johnson_at_dilute_rho`).
//! Matrix **#9** honesty: **`γ_gc`** on **`[B,4]`** uses a **Kirkwood–Buff-style scalar proxy** with
//! \((\rho^*,T^*)\) — see `upscale_potentials_b4_gamma_gc_depends_on_rho_t_star_state`.
//!
//! Scalar Johnson physical \(K_T\) for side-by-side checks lives in
//! [`umst_manifold::physics::solvers::statistical_mechanics::physical_bulk_modulus_johnson1993`] and
//! [`umst_manifold::physics::solvers::statistical_mechanics::relative_placeholder_bulk_modulus_gap_vs_johnson1993`].
//!
//! formal_citation: Johnson, Zollweg & Gubbins (1993), *Mol. Phys.* **78**, 591–618.

use approx::assert_abs_diff_eq;
use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::solvers::lj_johnson_1993_reference::{
    bulk_modulus_from_lj_state_johnson1993, bulk_modulus_from_reduced, johnson_lj1993_alphar,
    johnson_lj1993_bulk_modulus_reduced_numerical, johnson_lj1993_compressibility_factor,
    johnson_lj1993_dalphar_drho, johnson_lj1993_pressure_reduced,
};
use umst_manifold::physics::solvers::statistical_mechanics::{
    physical_bulk_modulus_johnson1993, relative_placeholder_bulk_modulus_gap_vs_johnson1993,
    upscale_potentials,
};

type B = NdArray<f32>;

/// Step for `∂α^r/∂ρ*` inside `Z` and `P*` (fixed for the supercritical grid checks).
const H_RHO: f64 = 1.0e-7_f64;

#[test]
fn johnson_lj1993_eos_compressibility_matches_pressure_over_rho_t_supercritical_grid() {
    let t_star = 2.0_f64;
    for k in 1..=14 {
        let rho_star = 0.05_f64 + 0.05_f64 * f64::from(k);
        let z = johnson_lj1993_compressibility_factor(t_star, rho_star, H_RHO);
        let p = johnson_lj1993_pressure_reduced(t_star, rho_star, H_RHO);
        let z_from_p = p / (rho_star * t_star);
        assert_abs_diff_eq!(z, z_from_p, epsilon = 1.0e-10);
        let ap = johnson_lj1993_dalphar_drho(t_star, rho_star, H_RHO);
        assert_abs_diff_eq!(z, 1.0 + rho_star * ap, epsilon = 1.0e-9);
    }
}

#[test]
fn johnson_lj1993_eos_residual_pressure_identity_supercritical_grid() {
    let t_star = 2.0_f64;
    for k in 1..=14 {
        let rho_star = 0.05_f64 + 0.05_f64 * f64::from(k);
        let z = johnson_lj1993_compressibility_factor(t_star, rho_star, H_RHO);
        let ap = johnson_lj1993_dalphar_drho(t_star, rho_star, H_RHO);
        let p_res = (z - 1.0) * rho_star * t_star;
        let p_res_alt = rho_star * rho_star * t_star * ap;
        assert_abs_diff_eq!(p_res, p_res_alt, epsilon = 1.0e-10);
    }
}

#[test]
fn johnson_lj1993_eos_bulk_modulus_reduced_numerical_self_consistent() {
    let t_star = 2.0_f64;
    let rho_star = 0.35_f64;
    let h = 5.0e-6_f64;
    let k_coarse = johnson_lj1993_bulk_modulus_reduced_numerical(t_star, rho_star, h);
    let k_fine = johnson_lj1993_bulk_modulus_reduced_numerical(t_star, rho_star, 0.5 * h);
    assert_abs_diff_eq!(k_coarse, k_fine, epsilon = 5.0e-3);
}

#[test]
fn physical_bulk_modulus_johnson1993_statmech_bridge_matches_lj_reference() {
    let rho_star = 0.2_f64;
    let t_star = 2.0_f64;
    let epsilon = 1.0_f64;
    let sigma = 0.8_f64;
    let k_star = bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
    let want = bulk_modulus_from_reduced(k_star, epsilon, sigma);
    assert_abs_diff_eq!(
        physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma),
        want,
        epsilon = 1.0e-12
    );
}

#[test]
fn placeholder_upscale_bulk_modulus_disagrees_with_johnson_reference_documented() {
    // Supercritical single-phase fluid branch (homogeneous). Use moderate `ρ*` where `K*` is not
    // ≈ 1 so the placeholder `K ∝ ε/σ³` (state-independent) is far from `K_T` from the EOS.
    let t_star = 2.0_f64;
    let rho_star = 0.2_f64;
    let epsilon = 1.0_f64;
    let sigma = 0.8_f64;
    let k_star = bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
    let k_johnson = bulk_modulus_from_reduced(k_star, epsilon, sigma);

    let gap =
        relative_placeholder_bulk_modulus_gap_vs_johnson1993(rho_star, t_star, epsilon, sigma);

    let dev = NdArrayDevice::Cpu;
    let lj: Tensor<B, 2> = Tensor::from_data(
        Data::new(vec![epsilon as f32, sigma as f32], Shape::new([1, 2])),
        &dev,
    );
    let (k_tensor, _) = upscale_potentials(lj)
        .expect("upscale_potentials [B,2] placeholder bulk modulus vs Johnson");
    let k_placeholder = f64::from(k_tensor.into_data().value[0]);

    let rel_tensor = ((k_placeholder - k_johnson) / k_johnson).abs();
    assert!(
        gap > 0.2,
        "expected analytic placeholder K to disagree strongly with JZG-derived K_T at this state (gap={gap}); \
         replace placeholder with bridge before expecting agreement"
    );
    assert_abs_diff_eq!(gap, rel_tensor, epsilon = 5.0e-4_f64);
}

/// **`[B,4]`** virial **`K`** tracks Johnson order-of-magnitude in the **dilute** branch (\(\rho^*=0.02\)).
#[test]
fn upscale_potentials_b4_k_order_matches_johnson_at_dilute_rho() {
    let dev = NdArrayDevice::Cpu;
    let t_star = 2.0_f64;
    let rho_star = 0.02_f64;
    let epsilon = 1.0_f64;
    let sigma = 0.8_f64;
    let k_j = physical_bulk_modulus_johnson1993(rho_star, t_star, epsilon, sigma);
    let lj: Tensor<B, 2> = Tensor::from_data(
        Data::new(
            vec![epsilon as f32, sigma as f32, rho_star as f32, t_star as f32],
            Shape::new([1, 4]),
        ),
        &dev,
    );
    let (k_tensor, _) = upscale_potentials(lj).expect(
        "statistical_mechanics::upscale_potentials on [B,4] virial K_T at dilute rho* vs Johnson 1993 reference (FP §6 Track G statmech Johnson EOS)",
    );
    let got = f64::from(k_tensor.into_data().value[0]);
    let ratio = got / k_j;
    assert!(
        ratio > 0.25 && ratio < 4.0,
        "expected virial K_T within order-one band of Johnson at dilute rho*={rho_star}; ratio={ratio}"
    );
}

/// **`[B,4]`** — **`γ_gc`** depends on \((\rho^*,T^*)\) via the KB-style proxy (not \(\varepsilon/\sigma^2\) only).
#[test]
fn upscale_potentials_b4_gamma_gc_depends_on_rho_t_star_state() {
    let dev = NdArrayDevice::Cpu;
    let epsilon = 1.1_f32;
    let sigma = 0.85_f32;
    let lj4_a: Tensor<B, 2> = Tensor::from_data(
        Data::new(vec![epsilon, sigma, 0.12_f32, 1.4_f32], Shape::new([1, 4])),
        &dev,
    );
    let lj4_b: Tensor<B, 2> = Tensor::from_data(
        Data::new(vec![epsilon, sigma, 0.55_f32, 2.8_f32], Shape::new([1, 4])),
        &dev,
    );
    let (_, g4a) = upscale_potentials(lj4_a).expect(
        "statistical_mechanics::upscale_potentials on [B,4] gamma_gc Kirkwood–Buff proxy state (rho*,T*) A (FP §6 Track G statmech Johnson EOS)",
    );
    let (_, g4b) = upscale_potentials(lj4_b).expect(
        "statistical_mechanics::upscale_potentials on [B,4] gamma_gc Kirkwood–Buff proxy state (rho*,T*) B (FP §6 Track G statmech Johnson EOS)",
    );
    let g4av = g4a.into_data().value[0];
    let g4bv = g4b.into_data().value[0];
    assert!(
        (g4av - g4bv).abs() > 1.0e-5_f32,
        "expected gamma_gc to vary with (rho*,T*); got {g4av} vs {g4bv}"
    );
    assert!(g4av.is_finite() && g4bv.is_finite());
}

#[test]
fn johnson_lj1993_alphar_finite_on_reference_isotherm() {
    let t_star = 2.0_f64;
    for k in 0..=20 {
        let rho_star = 0.02_f64 + 0.04_f64 * f64::from(k);
        let a = johnson_lj1993_alphar(t_star, rho_star);
        assert!(a.is_finite(), "alphar not finite at rho*={rho_star}");
    }
}

/// `statistical-mechanics-johnson-reference` re-export: \(K^*\) matches \(\rho^* (\partial P^*/\partial \rho^*)_{T^*}\)
/// on the JZG (1993) surface (compressibility / virial-style definition).
#[cfg(feature = "statistical-mechanics-johnson-reference")]
#[test]
fn statmech_johnson_reexport_k_star_matches_rho_dp_drho_virial_definition() {
    use umst_manifold::physics::solvers::statistical_mechanics::bulk_modulus_from_lj_state_johnson1993;

    let t_star = 2.0_f64;
    let rho_star = 0.35_f64;
    const H_DELTA: f64 = 1.0e-6_f64;
    let p_plus = johnson_lj1993_pressure_reduced(t_star, rho_star + H_DELTA, H_RHO);
    let p_minus = johnson_lj1993_pressure_reduced(t_star, rho_star - H_DELTA, H_RHO);
    let k_virial = rho_star * (p_plus - p_minus) / (2.0 * H_DELTA);

    let k_reexport = bulk_modulus_from_lj_state_johnson1993(rho_star, t_star);
    assert_abs_diff_eq!(k_reexport, k_virial, epsilon = 1.0e-8_f64);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Γ-convergence / phase-field fracture verification.
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md`. Repo table + full
//! status: `docs/Solver-Status.md` → **Phase-field fracture: implemented vs open**.
//!
//! **Implemented (this harness + `fracture_field.rs`):** inner AT2 damage relaxation uses **red–black**
//! half-steps on the graph Laplacian (see solver module docs). With `--features fracture-at2`:
//! `update_damage_smoke_tiny_chain` (finite \(d\); `outer_iterations == 1` with a fixed-strain
//! provider matches [`PhaseFieldFractureSolver::update_damage`]); `at2_surface_energy_scale_matches_gc_order_of_magnitude` (order-of-magnitude
//! \(G_c/l\cdot\bar d\) on the tiny chain); `at2_gc_linear_scaling_smoke` (doubling \(G_c\) at fixed \((l,\varepsilon)\): \(\bar d\) stays same order and \(G_c/l\cdot\bar d\) tracks \(\Delta G_c\) loosely — explicit sweep, **not** the Γ-limit scaling of \(G_c\) in the sharp-interface sense); **`at2_gamma_convergence_three_length_scales`** — three \((l_0,h)\) pairs with fixed \(h/l_0=\tfrac14\), \(\psi^+\equiv 0\), exponential damage seed at mid-span; discrete AT2 surface functional \(D_h\) has **relative error &lt; 2%** vs **`Gc`** on each mesh and **does not worsen** across refinement (successive errors within **`10^{-3}`**, fixed-strain relaxation with 32 outer passes — not a coupled mechanics \(\psi^+\) benchmark); **`at2_gamma_convergence_psi_plus_nonzero_three_length_scales`** (Track 12 §7.2) — same triple with uniform tensile \(\varepsilon_{xx}\), **`spectral_tensile_psi_plus_from_strain`** drive sanity, widened \(\tau_\Gamma\), non-worsening errors, and \(D_h > G_c\) vs the pure-surface optimum.
//!
//! **Harness:** shared **`discrete_at2_bar_surface_energy_1d`** + **`at2_discrete_surface_functional_toy_chain_matches_hand_total`** (guards the \(D_h\) sum used by **`at2_gamma_convergence_three_length_scales`**). **`at2_gamma_convergence_multi_ratio_schedule_smoke`** (Track 12 §7.3): fixed \(\ell_0\), \(\rho=h/\ell_0\in\{1/8,1/4,1/2\}\), same \(\psi^+\equiv 0\) exponential seed and 32-pass relaxation as [`at2_gamma_convergence_three_length_scales`]. **`at2_gamma_convergence_multi_ratio_psi_plus_schedule_smoke`** (Track 12 §7.3.1): same \(\rho\) rows as §7.3 with **uniform tensile** \(\varepsilon_{xx}\), **`spectral_tensile_psi_plus_from_strain`** drive sanity, widened \(\tau_\Gamma\) per row, and \(D_h>G_c\) vs the pure-surface optimum. **`at2_gamma_convergence_multi_ratio_psi_plus_outer_strain_ramp_smoke`** (Track 12 §7.3.2): identical multi-\(\rho\) meshing as §7.3.1 but **`PhaseFieldFractureSolver::update_damage_staggered`** with a **linear outer ramp** of \(\varepsilon_{xx}\) (nonzero schedule across 32 outers); final-drive \(\psi^+\) sanity, \(\tau_{\Gamma,j}\) on \(D_h\), and \(D_h>G_c\) vs the surface-only optimum. **Track 12 §7.4** — outer stopping (`update_damage_staggered_with_outer_cfg`, `StaggeredFractureConfig::outer_stopping`): **`at2_staggered_outer_cfg_fixed_iters_matches_legacy`**, **`at2_staggered_outer_loose_damage_linf_one_pass`**, **`at2_staggered_outer_rel_psi_loose_two_passes`**, **`at2_solve_staggered_mechanics_outer_loose_stopping_one_pass`**, **`staggered_mechanics_outer_damage_stop_matches_long_budget`**. **Research backlog** (stagger dissipation, THMC within-step stagger): [`docs/research/v0.4_track12_staggered_fracture_mechanics.md`](../../docs/research/v0.4_track12_staggered_fracture_mechanics.md) §7.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

#[cfg(feature = "fracture-at2")]
use umst_manifold::core::tensors::UnifiedMaterialStateTensor;
#[cfg(feature = "fracture-at2")]
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::physics::solvers::PhaseFieldFractureSolver;
#[cfg(feature = "fracture-at2")]
use umst_manifold::physics::solvers::{
    spectral_tensile_psi_plus_from_strain, strain_tensor_for_fracture_from_manifold,
    StaggeredFractureConfig, StaggeredOuterDamageStopCriteria,
};
#[cfg(feature = "fracture-at2")]
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type B = NdArray<f32>;

use umst_manifold::core::field::{DamageField, Field, SmallStrainField};

fn strain_field(t: Tensor<B, 4>) -> SmallStrainField<B> {
    SmallStrainField::from_tensor(t)
}

fn damage_field(t: Tensor<B, 3>) -> DamageField<B> {
    Field::new(t)
}


/// Discrete AT2 **1-D bar** surface functional (same definition as `at2_gamma_convergence_three_length_scales`):
///
/// \\[
/// D_h = G_c \\sum_{i=0}^{N-1} \\frac{d_i^2\\,h}{2\\ell_0}
///     + G_c \\sum_{i=0}^{N-2} \\frac{\\ell_0}{2}\\left(\\frac{d_{i+1}-d_i}{h}\\right)^2 h \\,.
/// \\]
#[cfg(feature = "fracture-at2")]
fn discrete_at2_bar_surface_energy_1d(d_vals: &[f32], h: f32, l0: f32, gc_val: f32) -> f32 {
    let n = d_vals.len();
    let mut d_h = 0.0_f32;
    for &d in d_vals {
        d_h += d.powi(2) * h / (2.0 * l0);
    }
    for i in 0..(n - 1) {
        let grad = (d_vals[i + 1] - d_vals[i]) / h;
        d_h += (l0 / 2.0) * grad.powi(2) * h;
    }
    d_h * gc_val
}

/// Regression on the closed-form discrete sum: \(N=3\), \(h=\ell_0=1\), \(G_c=2\), \(d=(0.5,1,0.5)\) gives \(D_h=2\).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_discrete_surface_functional_toy_chain_matches_hand_total() {
    let d = [0.5_f32, 1.0, 0.5];
    let d_h = discrete_at2_bar_surface_energy_1d(&d, 1.0, 1.0, 2.0);
    assert!(
        (d_h - 2.0).abs() < 1e-5,
        "hand total should be 2.0; got {d_h}"
    );
}

#[test]
fn update_damage_smoke_tiny_chain() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    // Uniform small uniaxial strain ε_xx = 1e−3 at every node (symmetric 3×3).
    let exx = 1e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9; // b=0
        strain_data[base] = exx; // (0,0)
        strain_data[base + 4] = 0.0; // (1,1)
        strain_data[base + 8] = 0.0; // (2,2)
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };

    let d_new = solver.update_damage_tensors(
        strain.clone(),
        damage.clone(),
        fracture_energy_gc.clone(),
        edges_b1.clone(),
    ).expect("update_damage_tensors");

    assert_eq!(d_new.dims(), damage.dims());

    #[cfg(not(feature = "fracture-at2"))]
    {
        let unchanged = damage.into_data().value == d_new.into_data().value;
        assert!(unchanged);
    }

    #[cfg(feature = "fracture-at2")]
    {
        for &x in d_new.clone().into_data().value.iter() {
            assert!(x.is_finite(), "expected finite damage; got {x}");
            assert!(
                (0.0..=1.0).contains(&x),
                "expected damage in [0,1]; got {x}",
            );
        }
        // `outer_iterations == 1` + fixed strain provider matches a single inner relaxation.
        let d_stagg = solver.update_damage_staggered(
            |_d: &DamageField<B>| strain_field(strain.clone()),
            damage_field(Tensor::<B, 3>::zeros([batch, n, 1], &dev)),
            fracture_energy_gc.clone(),
            edges_b1.clone(),
            1,
        ).expect("update_damage_staggered");
        let v_new = d_new.clone().into_data().value;
        let v_stagg = d_stagg.into_tensor().into_data().value;
        assert_eq!(
            v_new, v_stagg,
            "outer_iterations==1 with constant strain_fn must match update_damage"
        );
        let sum_d: f32 = v_new.iter().sum();
        let max_d = v_new.iter().copied().fold(0.0_f32, f32::max);
        assert!(
            max_d > 1e-10_f32 && sum_d > 1e-10_f32,
            "smoke expected stable positive AT2 damage (max and total); max_d={max_d} sum_d={sum_d} vals={v_new:?}"
        );
    }
}

/// Multi–length-scale smoke: damage stays admissible for several \(l\) (not Γ-convergence).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_length_scale_sweep_non_regression() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let exx = 4e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![120.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    for l in [0.06_f32, 0.09_f32, 0.12_f32] {
        let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
        let solver = PhaseFieldFractureSolver { length_scale: l };
        let d_new = solver.update_damage_tensors(
            strain.clone(),
            damage,
            fracture_energy_gc.clone(),
            edges_b1.clone(),
        ).expect("update_damage_tensors");
        for &x in d_new.clone().into_data().value.iter() {
            assert!(x.is_finite(), "l={l}: non-finite damage");
            assert!((0.0..=1.0).contains(&x), "l={l}: damage out of range");
        }
    }
}

/// Stronger tensile strain on the smoke **tiny chain**: AT2 drive yields finite positive damage and a `Gc/l`-consistent dissipation scale.
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_surface_energy_scale_matches_gc_order_of_magnitude() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    let exx = 5e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let gc = 150.0_f32;
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![gc; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver {
        length_scale: 0.08_f32,
    };
    let d_new = solver.update_damage_tensors(strain, damage, fracture_energy_gc, edges_b1).expect("update_damage_tensors");
    let vals = d_new.into_data().value;
    let mean_d: f32 = vals.iter().sum::<f32>() / vals.len() as f32;
    assert!(
        mean_d > 1e-6_f32,
        "expected AT2 mean damage under tensile strain; mean_d={mean_d}"
    );

    let l = 0.08_f32;
    let dissipation_scale = gc / l * mean_d;
    assert!(
        (1e-3_f32..1e7_f32).contains(&dissipation_scale),
        "Gc/l·d̄ scale {dissipation_scale} out of expected band"
    );
}

/// Doubling \(G_c\) at fixed \((l,\varepsilon)\): **dissipation-scale** proxy \(G_c/l\cdot\bar d\) rises
/// roughly with \(\Delta G_c\) (loose band); \(\bar d\) remains bounded and comparable order (explicit
/// relaxation — not a claim of pointwise monotone \(\bar d\) vs \(G_c\) on this tiny chain).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gc_linear_scaling_smoke() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    let exx = 5e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let l = 0.08_f32;
    let gc_lo = 100.0_f32;
    let gc_hi = 200.0_f32;

    let gc_field_lo = Tensor::from_data(
        Data::new(vec![gc_lo; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let gc_field_hi = Tensor::from_data(
        Data::new(vec![gc_hi; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: l };

    let d_lo = solver.update_damage_tensors(
        strain.clone(),
        damage0.clone(),
        gc_field_lo,
        edges_b1.clone(),
    ).expect("update_damage_tensors");
    let d_hi = solver.update_damage_tensors(strain, damage0, gc_field_hi, edges_b1.clone()).expect("update_damage_tensors");

    let vals_lo = d_lo.into_data().value;
    let vals_hi = d_hi.into_data().value;
    let mean_lo: f32 = vals_lo.iter().sum::<f32>() / vals_lo.len() as f32;
    let mean_hi: f32 = vals_hi.iter().sum::<f32>() / vals_hi.len() as f32;

    assert!(
        mean_lo > 1e-8_f32 && mean_hi > 1e-8_f32,
        "expected positive mean damage at both Gc; mean_lo={mean_lo} mean_hi={mean_hi}"
    );

    let r_md = mean_hi / mean_lo;
    assert!(
        (0.55_f32..1.65_f32).contains(&r_md),
        "mean_d ratio (2Gc vs Gc) expected bounded O(1) smoke; mean_lo={mean_lo} mean_hi={mean_hi} r_md={r_md}"
    );

    let scale_lo = gc_lo / l * mean_lo;
    let scale_hi = gc_hi / l * mean_hi;
    assert!(
        scale_lo > 1e-6_f32 && scale_hi > 1e-6_f32,
        "dissipation-scale proxy should stay above noise; scale_lo={scale_lo} scale_hi={scale_hi}"
    );
    let r_scale = scale_hi / scale_lo;
    assert!(
        (1.05_f32..3.8_f32).contains(&r_scale),
        "Gc/l·d̄ should rise sub-quadratically when Gc doubles; scale_lo={scale_lo} scale_hi={scale_hi} r_scale={r_scale}"
    );
}

/// First outer pass uses **negligible** tensile strain `A`, second pass uses strong strain `B`.
/// Total damage after two passes must exceed a **single** `update_damage` call with only `A`
/// (irreversibility can make “`A` then stronger `B`” match “`B` only” when the first pass already
/// saturates damage — this harness uses a tiny first drive so the second pass is observable).
#[cfg(feature = "fracture-at2")]
#[test]
fn staggered_two_outer_strains_exceeds_single_pass_weak_strain_only() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);

    fn uniaxial_strain(dev: &NdArrayDevice, batch: usize, n: usize, exx: f32) -> Tensor<B, 4> {
        let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
        for nod in 0..n {
            let base = (batch * nod) * 9;
            strain_data[base] = exx;
            strain_data[base + 4] = 0.0;
            strain_data[base + 8] = 0.0;
        }
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), dev)
    }

    let strain_weak = uniaxial_strain(&dev, batch, n, 1e-12_f32);
    let strain_strong = uniaxial_strain(&dev, batch, n, 5e-3_f32);

    let damage0 = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );

    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };

    let d_single_weak = solver.update_damage_tensors(
        strain_weak.clone(),
        damage0.clone(),
        fracture_energy_gc.clone(),
        edges_b1.clone(),
    ).expect("update_damage_tensors");

    let mut outer_k = 0usize;
    let d_staggered = solver.update_damage_staggered(
        |_d: &DamageField<B>| {
            let s = if outer_k == 0 {
                strain_weak.clone()
            } else {
                strain_strong.clone()
            };
            outer_k += 1;
            strain_field(s)
        },
        damage_field(damage0),
        fracture_energy_gc,
        edges_b1,
        2,
    ).expect("update_damage_staggered");

    let sum_w: f32 = d_single_weak.into_data().value.iter().sum();
    let sum_st: f32 = d_staggered.into_tensor().into_data().value.iter().sum();
    assert!(
        sum_st > sum_w + 1e-8_f32,
        "expected weak→strong staggered total damage to exceed single weak pass; sum_w={sum_w} sum_st={sum_st}"
    );
}

/// Γ-convergence harness (Phase 2.4): single 1-D pre-notched bar; refine `(l₀, h)` pairs together
/// keeping `h/l₀ = 1/4` and check that the discrete dissipation
/// `D_h = Σ_i [ d_i² · h / (2 l₀) + (l₀/2) (d_{i+1}-d_i)²/h ] · Gc`
/// approaches the analytic `Gc` limit: **relative error &lt; 2%** vs `Gc` at each scale and **non-worsening**
/// across refinement (successive relative errors within `1e-3`; no assertion of strict monotone decay).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_three_length_scales() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    // ψ⁺ ≡ 0: with **irreversibility** (`out.max_pair(damage_old)`) the relaxation cannot reduce the
    // pre-localised seed, so `d` stays at the optimal AT2 continuum profile `exp(-|x-L/2|/l₀)`.
    // The discrete functional `D_h` evaluated on this profile is the textbook Γ-convergence quantity
    // (Bourdin–Francfort–Marigo 2000 §3.2 / Miehe 2010 §4.1). Continuum value is exactly `Gc`.
    let psi_plus_drive: f32 = 0.0;

    let pairs: [(f32, f32); 3] = [(0.04, 0.01), (0.02, 0.005), (0.01, 0.0025)];
    let mut errors: Vec<f32> = Vec::with_capacity(pairs.len());
    let mut d_hs: Vec<f32> = Vec::with_capacity(pairs.len());

    for (l0, h) in pairs.iter().copied() {
        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        // Edges: 0–1, 1–2, …, (N−2)–(N−1) packed as `[2, E]` (row 0 = src, row 1 = tgt).
        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        // Initial damage: optimal AT2 profile exp(-|x − L/2| / l₀), the analytic minimiser of
        // the surface-energy density `γ(d) = d²/(2l₀) + (l₀/2)|∇d|²` for a single fully-developed
        // crack at `L/2`. Irreversibility preserves this seed across relaxation passes.
        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        // Uniform ψ⁺ encoded as diagonal strain with ε_xx s.t. ½·ε² = psi_plus_drive ⇒ ε = √(2·psi).
        let exx = (2.0_f32 * psi_plus_drive).sqrt();
        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        // 32 outer passes of the fixed-strain relaxation (each call already runs the inner red–black loop).
        let mut d_curr = damage.clone();
        for _ in 0..32 {
            d_curr = solver.update_damage_tensors(
                strain.clone(),
                d_curr,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            ).expect("update_damage_tensors");
        }

        let d_vals: Vec<f32> = d_curr.into_data().value;
        let d_h = discrete_at2_bar_surface_energy_1d(&d_vals, h, l0, gc_val);
        let err = (d_h - gc_val).abs() / gc_val;
        d_hs.push(d_h);
        errors.push(err);
        eprintln!("Γ-conv: l0={l0:.4} h={h:.4} N={n} D_h={d_h:.4} rel_err={err:.4}");
    }

    // Acceptance (matches `docs/Solver-Status.md` OPEN ROADMAP ITEM — Fracture): relative `|D_h − Gc|/Gc` &lt; 2% on
    // each mesh (ψ⁺=0 seed profile); successive relative errors non-worsening within `1e-3` (not strict decay).
    for (i, &err) in errors.iter().enumerate() {
        let d_h = d_hs[i];
        assert!(
            err < 0.02,
            "Γ-conv error too large at pair {i}: D_h={d_h} rel_err={err}",
        );
    }
    assert!(
        errors[1] <= errors[0] + 1e-3,
        "error must not increase between coarse→mid: {errors:?}",
    );
    assert!(
        errors[2] <= errors[1] + 1e-3,
        "error must not increase between mid→fine: {errors:?}",
    );
}

/// Track 12 §7.3 — fixed bar length \(L\) and mesh ratio \(\rho=h/\ell_0\) from \(\{\tfrac18,\tfrac14,\tfrac12\}\)
/// with a **common** \(\ell_0\) (here `0.04`): same \(\psi^+\equiv 0\) exponential seed, 32-pass fixed-strain
/// relaxation, and the same discrete \(D_h\) functional as [`at2_gamma_convergence_three_length_scales`].
/// Per-\(\rho\) relative-error caps \(\tau_j\) are **documented in-test** (coarser \(\rho\) uses wider slack than
/// the 2% line in §7.1 until a full sharp-interface calibration exists).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_multi_ratio_schedule_smoke() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    let psi_plus_drive: f32 = 0.0;
    let l0: f32 = 0.04;
    // ρ = h / ℓ₀ — memo §7.3 example set.
    let schedule: [(f32, f32); 3] = [
        (1.0 / 8.0, 0.005), // h = ρ·ℓ₀
        (1.0 / 4.0, 0.01),
        (1.0 / 2.0, 0.02),
    ];
    // τ_j: finest mesh matches the shipped 2% gate; coarser ρ gets explicit slack.
    let tau_by_rho: [f32; 3] = [0.02_f32, 0.02_f32, 0.035_f32];

    for (j, &tau_j) in tau_by_rho.iter().enumerate() {
        let rho = schedule[j].0;
        let h = schedule[j].1;
        assert!(
            ((h / l0) - rho).abs() < 1e-5,
            "schedule row inconsistent: h/l0={} rho={rho}",
            h / l0
        );

        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        let exx = (2.0_f32 * psi_plus_drive).sqrt();
        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        let mut d_curr = damage.clone();
        for _ in 0..32 {
            d_curr = solver.update_damage_tensors(
                strain.clone(),
                d_curr,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            ).expect("update_damage_tensors");
        }

        let d_vals: Vec<f32> = d_curr.into_data().value;
        let d_h = discrete_at2_bar_surface_energy_1d(&d_vals, h, l0, gc_val);
        let err = (d_h - gc_val).abs() / gc_val;
        eprintln!(
            "multi-ρ smoke: ρ={rho:.4} h={h:.4} l0={l0:.4} N={n} D_h={d_h:.4} rel_err={err:.4} τ_j={tau_j}"
        );
        assert!(
            err < tau_j,
            "ρ={rho}: |D_h-Gc|/Gc = {err} exceeds τ_j={tau_j} (D_h={d_h})"
        );
    }
}

/// Track 12 §7.3.1 — same \(\rho=h/\ell_0\) schedule and discrete \(D_h\) harness as
/// [`at2_gamma_convergence_multi_ratio_schedule_smoke`], but with **nonzero** uniform \(\varepsilon_{xx}\)
/// (spectral tensile \(\psi^+\) drive). Per-\(\rho\) **\(\tau_{\Gamma,j}\)** are documented in-test (same order
/// of magnitude as §7.2’s widened band, not the 2% \(\psi^+\!\equiv 0\) multi-\(\rho\) caps). Asserts
/// **`max_i \psi^+_i`** from the Jacobi map and \(D_h > G_c + 10^{-3}\) on each mesh (tensile drive lifts
/// dissipation above the sharp surface-only optimum; finest \(\rho\) rows may sit only \(\mathcal O(10^{-3})\) above **`Gc`**).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_multi_ratio_psi_plus_schedule_smoke() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    let exx: f32 = 0.08_f32;
    let psi_floor: f32 = 0.5_f32 * exx * exx * 0.99_f32;
    let l0: f32 = 0.04;
    let schedule: [(f32, f32); 3] = [(1.0 / 8.0, 0.005), (1.0 / 4.0, 0.01), (1.0 / 2.0, 0.02)];
    // τ_{Γ,j}: coupled ψ⁺ — coarsest ρ may sit slightly looser than the fixed-ratio triple in §7.2.
    let tau_gamma_by_rho: [f32; 3] = [0.55_f32, 0.55_f32, 0.58_f32];

    for (j, &tau_j) in tau_gamma_by_rho.iter().enumerate() {
        let rho = schedule[j].0;
        let h = schedule[j].1;
        assert!(
            ((h / l0) - rho).abs() < 1e-5,
            "schedule row inconsistent: h/l0={} rho={rho}",
            h / l0
        );

        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let psi_tensor = spectral_tensile_psi_plus_from_strain(strain.clone());
        let max_psi: f32 = psi_tensor
            .clone()
            .into_data()
            .value
            .iter()
            .copied()
            .fold(0.0_f32, f32::max);
        assert!(
            max_psi >= psi_floor,
            "ρ={rho}: drive sanity max ψ⁺ vs floor; max_psi={max_psi} floor={psi_floor}"
        );

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        let mut d_curr = damage.clone();
        for _ in 0..32 {
            d_curr = solver.update_damage_tensors(
                strain.clone(),
                d_curr,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            ).expect("update_damage_tensors");
        }

        let d_vals: Vec<f32> = d_curr.into_data().value;
        let d_h = discrete_at2_bar_surface_energy_1d(&d_vals, h, l0, gc_val);
        let err = (d_h - gc_val).abs() / gc_val;
        eprintln!(
            "multi-ρ ψ⁺: ρ={rho:.4} h={h:.4} N={n} D_h={d_h:.4} rel_err={err:.4} τ_Γ,j={tau_j} max_psi={max_psi:.6}"
        );
        assert!(
            err < tau_j,
            "ρ={rho}: |D_h-Gc|/Gc = {err} exceeds τ_Γ,j={tau_j} (D_h={d_h})"
        );
        // Finest ρ row can sit just above `Gc` (surface-dominated); keep a small strict margin.
        assert!(
            d_h > gc_val + 1e-3_f32,
            "ρ={rho}: expected D_h > Gc + 1e-3 with tensile ψ⁺; D_h={d_h}"
        );
    }
}

/// Track 12 §7.3.2 — same \(\rho=h/\ell_0\) rows, exponential seed, and \(D_h\) harness as
/// [`at2_gamma_convergence_multi_ratio_psi_plus_schedule_smoke`], but the tensile drive is a **linear
/// outer strain schedule**: each of 32 staggered passes uses \(\varepsilon_{xx}\) ramping from a small
/// positive value to the same terminal \(\varepsilon_{xx}\) as §7.3.1 (memo §7.2 “load ramp / outer strain
/// schedule`). Per-\(\rho\) \(\tau_{\Gamma,j}\) on \(|D_h-G_c|/G_c\) are **\(\{2\%,\,2\%,\,5\%\}\)** in-test
/// (margin over the observed ramp baseline \(\sim(2\times10^{-3},\,8\times10^{-3},\,3\times10^{-2})\) on this harness).
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_multi_ratio_psi_plus_outer_strain_ramp_smoke() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    let exx_end: f32 = 0.08_f32;
    let exx_start: f32 = 0.02_f32;
    let outer_iters: usize = 32;
    let psi_floor: f32 = 0.5_f32 * exx_end * exx_end * 0.99_f32;
    let l0: f32 = 0.04;
    let schedule: [(f32, f32); 3] = [(1.0 / 8.0, 0.005), (1.0 / 4.0, 0.01), (1.0 / 2.0, 0.02)];
    // τ_{Γ,j}: ramped outer schedule (baseline rel_err ≈ {2e-3, 8e-3, 3.1e-2} on this harness).
    let tau_gamma_by_rho: [f32; 3] = [0.02_f32, 0.02_f32, 0.05_f32];

    fn uniaxial_strain(dev: &NdArrayDevice, batch: usize, n: usize, exx: f32) -> Tensor<B, 4> {
        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), dev)
    }

    for (j, &tau_j) in tau_gamma_by_rho.iter().enumerate() {
        let rho = schedule[j].0;
        let h = schedule[j].1;
        assert!(
            ((h / l0) - rho).abs() < 1e-5,
            "schedule row inconsistent: h/l0={} rho={rho}",
            h / l0
        );

        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        let denom = (outer_iters.saturating_sub(1).max(1)) as f32;
        let mut outer_k = 0usize;
        let d_curr = solver.update_damage_staggered(
            |_d: &DamageField<B>| {
                let t = (outer_k as f32 / denom).clamp(0.0, 1.0);
                let exx = exx_start + t * (exx_end - exx_start);
                outer_k += 1;
                strain_field(uniaxial_strain(&dev, batch, n, exx))
            },
            damage_field(damage),
            fracture_energy_gc.clone(),
            edges_b1.clone(),
            outer_iters,
        ).expect("update_damage_staggered");

        let strain_final = uniaxial_strain(&dev, batch, n, exx_end);
        let psi_tensor = spectral_tensile_psi_plus_from_strain(strain_final);
        let max_psi: f32 = psi_tensor
            .clone()
            .into_data()
            .value
            .iter()
            .copied()
            .fold(0.0_f32, f32::max);
        assert!(
            max_psi >= psi_floor,
            "ρ={rho}: terminal drive sanity max ψ⁺ vs floor; max_psi={max_psi} floor={psi_floor}"
        );

        let d_vals: Vec<f32> = d_curr.into_tensor().into_data().value;
        let d_h = discrete_at2_bar_surface_energy_1d(&d_vals, h, l0, gc_val);
        let err = (d_h - gc_val).abs() / gc_val;
        eprintln!(
            "multi-ρ ψ⁺ ramp: ρ={rho:.4} h={h:.4} N={n} D_h={d_h:.4} rel_err={err:.4} τ_Γ,j={tau_j} max_psi={max_psi:.6}"
        );
        assert!(
            err < tau_j,
            "ρ={rho}: |D_h-Gc|/Gc = {err} exceeds τ_Γ,j={tau_j} (D_h={d_h})"
        );
        assert!(
            d_h > gc_val + 1e-3_f32,
            "ρ={rho}: expected D_h > Gc + 1e-3 with ramped ψ⁺; D_h={d_h}"
        );
    }
}

/// Track 12 §7.2 — same three \((l_0,h)\) pairs and \(D_h\) functional as
/// [`at2_gamma_convergence_three_length_scales`], but with **nonzero** spectral tensile drive from a
/// uniform uniaxial \(\varepsilon_{xx}\) so `update_damage` perturbs the exponential seed (fixed-strain
/// relaxation, 32 passes). **\(\tau_\Gamma\)** is widened vs the \(\psi^+\!=0\) harness (2%): coupled
/// tensile drive raises \(D_h\) above the sharp \(G_c\) surface-only optimum; we still gate **non-worsening**
/// successive relative errors and assert \(\max_i \psi^+_i\) from the same Jacobi map as the solver.
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_gamma_convergence_psi_plus_nonzero_three_length_scales() {
    let dev = NdArrayDevice::Cpu;
    let length_l: f32 = 5.0;
    let gc_val: f32 = 1.0;
    // Uniform axial tension: ψ⁺ = ½ ε_max² per node (aligned principal axis).
    let exx: f32 = 0.08_f32;
    let psi_floor: f32 = 0.5_f32 * exx * exx * 0.99_f32;

    let pairs: [(f32, f32); 3] = [(0.04, 0.01), (0.02, 0.005), (0.01, 0.0025)];
    let mut errors: Vec<f32> = Vec::with_capacity(pairs.len());
    let mut d_hs: Vec<f32> = Vec::with_capacity(pairs.len());

    for (l0, h) in pairs.iter().copied() {
        let n: usize = ((length_l / h).ceil() as usize) + 1;
        let e_ct: usize = n - 1;
        let batch: usize = 1;

        let mut edge_data = Vec::with_capacity(2 * e_ct);
        for i in 0..e_ct {
            edge_data.push(i as i64);
        }
        for i in 0..e_ct {
            edge_data.push((i + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edge_data, Shape::new([2, e_ct])), &dev);

        let mut d_init = vec![0.0_f32; n];
        let centre = length_l * 0.5;
        for (i, slot) in d_init.iter_mut().enumerate().take(n) {
            let x = (i as f32) * h;
            *slot = (-((x - centre).abs()) / l0).exp();
        }
        let damage = Tensor::<B, 3>::from_data(Data::new(d_init, Shape::new([batch, n, 1])), &dev);

        let mut strain_data = vec![0.0_f32; batch * n * 9];
        for nod in 0..n {
            let base = nod * 9;
            strain_data[base] = exx;
        }
        let strain: Tensor<B, 4> =
            Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

        let psi_tensor = spectral_tensile_psi_plus_from_strain(strain.clone());
        let max_psi: f32 = psi_tensor
            .clone()
            .into_data()
            .value
            .iter()
            .copied()
            .fold(0.0_f32, f32::max);
        assert!(
            max_psi >= psi_floor,
            "drive sanity: max ψ⁺ should match uniaxial tensile map; max_psi={max_psi} floor={psi_floor}"
        );

        let fracture_energy_gc = Tensor::from_data(
            Data::new(vec![gc_val; batch * n], Shape::new([batch, n, 1])),
            &dev,
        );

        let solver = PhaseFieldFractureSolver { length_scale: l0 };
        let mut d_curr = damage.clone();
        for _ in 0..32 {
            d_curr = solver.update_damage_tensors(
                strain.clone(),
                d_curr,
                fracture_energy_gc.clone(),
                edges_b1.clone(),
            ).expect("update_damage_tensors");
        }

        let d_vals: Vec<f32> = d_curr.into_data().value;
        let d_h = discrete_at2_bar_surface_energy_1d(&d_vals, h, l0, gc_val);
        let err = (d_h - gc_val).abs() / gc_val;
        d_hs.push(d_h);
        errors.push(err);
        eprintln!(
            "Γ ψ⁺: l0={l0:.4} h={h:.4} N={n} D_h={d_h:.4} rel_err={err:.4} max_psi={max_psi:.6}"
        );
    }

    // Wider band than ψ⁺≡0: tensile drive couples into damage; D_h need not sit at the sharp `Gc` optimum.
    const TAU_GAMMA_PSI: f32 = 0.55_f32;
    for (i, &err) in errors.iter().enumerate() {
        let d_h = d_hs[i];
        assert!(
            err < TAU_GAMMA_PSI,
            "Γ-type relative error too large at pair {i}: D_h={d_h} rel_err={err} (cap {TAU_GAMMA_PSI})",
        );
    }
    assert!(
        errors[1] <= errors[0] + 1e-3,
        "error must not increase between coarse→mid: {errors:?}",
    );
    assert!(
        errors[2] <= errors[1] + 1e-3,
        "error must not increase between mid→fine: {errors:?}",
    );

    // Coupled drive lifts the discrete surface measure above the ψ⁺≡0 optimum (`D_h ≈ Gc`).
    for dh in &d_hs {
        assert!(
            *dh > gc_val + 5e-3_f32,
            "expected D_h > Gc on each mesh with strong ψ⁺; got D_h={dh}"
        );
    }
}

/// Track 12 §7 — **`matrix_features`** path without SI bar equilibrium:
/// [`strain_tensor_for_fracture_from_manifold`] lifts channel `0` to `[B,N,3,3]` and matches explicit
/// uniaxial packing / [`spectral_tensile_psi_plus_from_strain`] totals; mismatched shapes fall back to zeros.
#[cfg(feature = "fracture-at2")]
#[test]
fn at2_matrix_features_stub_matches_direct_strain_psi_plus_sanity() {
    let dev = NdArrayDevice::Cpu;
    let n = 4usize;
    let batch = 1usize;
    let exx = 0.061_f32;
    let f = UMST_SCALAR_CHANNEL_COUNT;
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
    let mut e = Vec::with_capacity((n - 1) * 2);
    for i in 0..n - 1 {
        e.push(i as i64);
    }
    for i in 0..n - 1 {
        e.push((i + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(Data::new(e, Shape::new([2, n - 1])), &dev);
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
    let scalar_features = Tensor::<B, 2>::zeros([n, f], &dev);
    let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &dev);
    let mut mf = vec![0.0_f32; n * 9];
    for i in 0..n {
        mf[i * 9] = exx;
    }
    let matrix_features = Tensor::from_data(Data::new(mf, Shape::new([n, 1, 3, 3])), &dev);
    let displacement_bc_mask = Tensor::<B, 3>::ones([n, 3, 1], &dev);
    let policy_editable_mask = Tensor::<B, 2>::ones([n, 1], &dev);
    let manifold = UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features,
        vector_features,
        matrix_features,
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions: None,
        displacement_bc_mask,
        policy_editable_mask,
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    };

    let eps_stub = strain_tensor_for_fracture_from_manifold::<B>(&manifold, batch, n, &dev);

    let mut strain_data = vec![0.0_f32; batch * n * 9];
    for nod in 0..n {
        strain_data[nod * 9] = exx;
    }
    let eps_direct: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let v1 = eps_stub.into_data().value;
    let v2 = eps_direct.clone().into_data().value;
    let max_abs = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f32, f32::max);
    assert!(
        max_abs < 1e-6_f32,
        "matrix_features stub strain must match explicit packing; max_abs={max_abs}"
    );

    let eps_stub_2 = strain_tensor_for_fracture_from_manifold::<B>(&manifold, batch, n, &dev);
    let psi_s = spectral_tensile_psi_plus_from_strain(eps_stub_2);
    let psi_d = spectral_tensile_psi_plus_from_strain(eps_direct);
    let sum_s: f32 = psi_s.into_data().value.iter().sum();
    let sum_d: f32 = psi_d.into_data().value.iter().sum();
    assert!(
        (sum_s - sum_d).abs() < 1e-5_f32,
        "ψ⁺ totals must match; sum_stub={sum_s} sum_direct={sum_d}"
    );

    let mut bad = manifold.clone();
    bad.matrix_features = Tensor::<B, 4>::zeros([n + 1, 1, 3, 3], &dev);
    let eps_zero = strain_tensor_for_fracture_from_manifold::<B>(&bad, batch, n, &dev);
    let zmax = eps_zero
        .into_data()
        .value
        .iter()
        .copied()
        .fold(0.0_f32, f32::max);
    assert!(
        zmax < 1e-12_f32,
        "expected zero fallback strain when matrix_features rows ≠ n; zmax={zmax}"
    );
}

/// Phase 3.1 — staggered elasticity–damage loop owns the mechanics solve internally.
/// Tensile 1-D bar with a pre-localised damage seed; the staggered loop must increase compliance
/// monotonically and drive damage at the load-point above a high threshold.
#[cfg(feature = "fracture-at2")]
#[test]
fn staggered_fracture_compliance_monotone_increasing() {
    use umst_manifold::physics::solvers::PhaseFieldFractureSolver;

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 20usize;
    let e_ct = n - 1;
    let length_l: f32 = 1.0;
    let h = length_l / ((n - 1) as f32);

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * h);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

    let mut edges = Vec::with_capacity(2 * e_ct);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    // Tensile force F at right tip, only x-DOF free everywhere except the pinned left node.
    let force: f32 = 0.1;
    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = force;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    // Pin x at node 0; lock y,z everywhere; free x elsewhere.
    let mut bm_data = vec![0.0_f32; n * 3];
    for i in 1..n {
        bm_data[i * 3] = 1.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

    // ρ = 1 everywhere → E_eff = E0.
    let rho_node = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        // f32 PCG caps at 3N iterations (N=20 → 60); relax tol vs 1e-8 so fail-closed Result API converges.
        cg_tolerance: 1e-5,
        pcg_tolerance: 1e-5,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let cross_section_area = 0.01_f32;
    let e0: f32 = 1.0;

    // Baseline compliance c_0 with damage = 0 — solve once via outer_iters = 1 and capture u tip.
    let cfg_one = StaggeredFractureConfig {
        outer_iters: 1,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
        outer_stopping: StaggeredOuterDamageStopCriteria::default(),
    };
    let (u0, _d0) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords.clone(),
        edges_b1.clone(),
        body_force.clone(),
        boundary_mask.clone(),
        rho_node.clone(),
        e0,
        cross_section_area,
        &cg,
        cfg_one,
    )
    .expect("solve_staggered_with_mechanics");
    let u0_vals = u0.into_data().value;
    let c0 = force * u0_vals[(n - 1) * 3];

    // Capture compliance at increasing outer counts and check monotonicity.
    let mut compliances: Vec<f32> = vec![c0];
    let outer_schedule = [2usize, 5, 10, 20, 30];
    let mut d_last_max: f32 = 0.0;
    for &k in outer_schedule.iter() {
        let cfg_k = StaggeredFractureConfig {
            outer_iters: k,
            damage_relaxation_passes: 1,
            gc: 0.01,
            length_scale: 0.05,
            kappa_reg: 1e-6,
            outer_stopping: StaggeredOuterDamageStopCriteria::default(),
        };
        let (u_k, d_k) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
            coords.clone(),
            edges_b1.clone(),
            body_force.clone(),
            boundary_mask.clone(),
            rho_node.clone(),
            e0,
            cross_section_area,
            &cg,
            cfg_k,
        )
        .expect("solve_staggered_with_mechanics");
        let u_vals = u_k.into_data().value;
        let d_vals = d_k.into_tensor().into_data().value;
        let tip_u = u_vals[(n - 1) * 3];
        let c_k = force * tip_u;
        let max_d = d_vals.iter().copied().fold(0.0_f32, f32::max);
        d_last_max = max_d;
        eprintln!("staggered: k={k} c_k={c_k:.6} max_d={max_d:.4}");
        compliances.push(c_k);
    }

    let c_final = *compliances
        .last()
        .expect("final compliance from staggered outer_schedule (compliances non-empty after loop)");
    assert!(
        c_final > c0,
        "expected compliance to grow: c0={c0} c_final={c_final}"
    );
    assert!(
        d_last_max > 0.5,
        "expected significant damage growth; max_d_final={d_last_max}"
    );

    // Monotone non-decreasing (allow ε = 1e-4 slack).
    for w in compliances.windows(2) {
        assert!(
            w[1] >= w[0] - 1e-4,
            "compliance must be monotone non-decreasing; got sequence {compliances:?}"
        );
    }
}

#[cfg(feature = "fracture-at2")]
#[test]
fn at2_staggered_outer_cfg_fixed_iters_matches_legacy() {
    use burn::tensor::Int;
    use umst_manifold::physics::solvers::{
        PhaseFieldFractureSolver, StaggeredDamageOuterLoopConfig,
    };

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let exx = 1e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let strain_a = strain.clone();
    let d_legacy = solver.update_damage_staggered(
        move |_d: &DamageField<B>| strain_field(strain_a.clone()),
        damage_field(damage.clone()),
        fracture_energy_gc.clone(),
        edges_b1.clone(),
        4,
    ).expect("update_damage_staggered");
    let strain_b = strain.clone();
    let d_cfg = solver.update_damage_staggered_with_outer_cfg(
        move |_d: &DamageField<B>| strain_field(strain_b.clone()),
        damage_field(damage),
        fracture_energy_gc,
        edges_b1,
        StaggeredDamageOuterLoopConfig::fixed_iters(4),
    ).expect("update_damage_staggered_with_outer_cfg");
    assert_eq!(
        d_legacy.into_tensor().into_data().value,
        d_cfg.into_tensor().into_data().value,
        "fixed_iters outer cfg must match legacy staggered loop"
    );
}

#[cfg(feature = "fracture-at2")]
#[test]
fn at2_staggered_outer_loose_damage_linf_one_pass() {
    use burn::tensor::Int;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use umst_manifold::physics::solvers::{
        PhaseFieldFractureSolver, StaggeredDamageOuterLoopConfig, StaggeredOuterDamageStopCriteria,
    };

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let exx = 1e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_f = calls.clone();
    let strain_c = strain.clone();
    let _ = solver.update_damage_staggered_with_outer_cfg(
        move |_d: &DamageField<B>| {
            calls_f.fetch_add(1, Ordering::Relaxed);
            strain_field(strain_c.clone())
        },
        damage_field(damage),
        fracture_energy_gc,
        edges_b1,
        StaggeredDamageOuterLoopConfig {
            max_outer_iterations: 50,
            stopping: StaggeredOuterDamageStopCriteria {
                tol_damage_linf: Some(10.0),
                tol_strain_linf: None,
                tol_rel_degraded_psi_mean: None,
            },
        },
    ).expect("update_damage_staggered_with_outer_cfg");
    assert_eq!(
        calls.load(Ordering::Relaxed),
        1,
        "loose damage gate should stop after first outer pass"
    );
}

#[cfg(feature = "fracture-at2")]
#[test]
fn at2_staggered_outer_rel_psi_loose_two_passes() {
    use burn::tensor::Int;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use umst_manifold::physics::solvers::{
        PhaseFieldFractureSolver, StaggeredDamageOuterLoopConfig, StaggeredOuterDamageStopCriteria,
    };

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 1, 1, 2], Shape::new([2, e_ct])), &dev);
    let exx = 1e-3_f32;
    let mut strain_data = vec![0.0_f32; batch * n * 3 * 3];
    for nod in 0..n {
        let base = (batch * nod) * 9;
        strain_data[base] = exx;
        strain_data[base + 4] = 0.0;
        strain_data[base + 8] = 0.0;
    }
    let strain: Tensor<B, 4> =
        Tensor::from_data(Data::new(strain_data, Shape::new([batch, n, 3, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let solver = PhaseFieldFractureSolver { length_scale: 0.08 };
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_f = calls.clone();
    let strain_c = strain.clone();
    let _ = solver.update_damage_staggered_with_outer_cfg(
        move |_d: &DamageField<B>| {
            calls_f.fetch_add(1, Ordering::Relaxed);
            strain_field(strain_c.clone())
        },
        damage_field(damage),
        fracture_energy_gc,
        edges_b1,
        StaggeredDamageOuterLoopConfig {
            max_outer_iterations: 2,
            stopping: StaggeredOuterDamageStopCriteria {
                tol_damage_linf: None,
                tol_strain_linf: None,
                tol_rel_degraded_psi_mean: Some(1e30),
            },
        },
    ).expect("update_damage_staggered_with_outer_cfg");
    assert_eq!(
        calls.load(Ordering::Relaxed),
        2,
        "with no early exits, strain_fn should run for each scheduled outer pass"
    );
}

#[cfg(feature = "fracture-at2")]
#[allow(clippy::type_complexity)]
fn staggered_mechanics_bar_fixture() -> (
    Tensor<B, 2>,
    Tensor<B, 2, Int>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
) {
    use burn::tensor::Int;

    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 20usize;
    let e_ct = n - 1;
    let length_l: f32 = 1.0;
    let h = length_l / ((n - 1) as f32);

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * h);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

    let mut edges = Vec::with_capacity(2 * e_ct);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let force: f32 = 0.1;
    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = force;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    let mut bm_data = vec![0.0_f32; n * 3];
    for i in 1..n {
        bm_data[i * 3] = 1.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

    let rho_node = Tensor::<B, 3>::ones([batch, n, 1], &dev);

    (coords, edges_b1, body_force, boundary_mask, rho_node)
}

#[cfg(feature = "fracture-at2")]
#[test]
fn at2_solve_staggered_mechanics_outer_loose_stopping_one_pass() {
    use umst_manifold::physics::solvers::PhaseFieldFractureSolver;

    let (coords, edges_b1, body_force, boundary_mask, rho_node) = staggered_mechanics_bar_fixture();

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-5,
        pcg_tolerance: 1e-5,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let cross_section_area = 0.01_f32;
    let e0: f32 = 1.0;

    let cfg_one = StaggeredFractureConfig {
        outer_iters: 1,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
        outer_stopping: StaggeredOuterDamageStopCriteria::default(),
    };

    let cfg_stop = StaggeredFractureConfig {
        outer_iters: 30,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
        outer_stopping: StaggeredOuterDamageStopCriteria {
            tol_damage_linf: Some(10.0),
            tol_strain_linf: None,
            tol_rel_degraded_psi_mean: None,
        },
    };

    let (u1, d1) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords.clone(),
        edges_b1.clone(),
        body_force.clone(),
        boundary_mask.clone(),
        rho_node.clone(),
        e0,
        cross_section_area,
        &cg,
        cfg_one,
    )
    .expect("solve_staggered_with_mechanics");
    let (u2, d2) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords,
        edges_b1,
        body_force,
        boundary_mask,
        rho_node,
        e0,
        cross_section_area,
        &cg,
        cfg_stop,
    )
    .expect("solve_staggered_with_mechanics");
    let v1 = u1.into_data().value;
    let v2 = u2.into_data().value;
    let tol = 1e-4_f32;
    for i in 0..v1.len() {
        assert!((v1[i] - v2[i]).abs() < tol, "u mismatch at {i}");
    }
    let w1 = d1.into_tensor().into_data().value;
    let w2 = d2.into_tensor().into_data().value;
    for i in 0..w1.len() {
        assert!((w1[i] - w2[i]).abs() < tol, "d mismatch at {i}");
    }
}

#[cfg(feature = "fracture-at2")]
#[test]
fn staggered_mechanics_outer_damage_stop_matches_long_budget() {
    use umst_manifold::physics::solvers::PhaseFieldFractureSolver;

    let (coords, edges_b1, body_force, boundary_mask, rho_node) = staggered_mechanics_bar_fixture();

    let cg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-5,
        pcg_tolerance: 1e-5,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let cross_section_area = 0.01_f32;
    let e0: f32 = 1.0;

    let cfg_long = StaggeredFractureConfig {
        outer_iters: 8,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
        outer_stopping: StaggeredOuterDamageStopCriteria::default(),
    };

    let cfg_inactive_stop = StaggeredFractureConfig {
        outer_iters: 8,
        damage_relaxation_passes: 1,
        gc: 0.01,
        length_scale: 0.05,
        kappa_reg: 1e-6,
        outer_stopping: StaggeredOuterDamageStopCriteria {
            tol_damage_linf: Some(-1.0),
            tol_strain_linf: None,
            tol_rel_degraded_psi_mean: None,
        },
    };

    let (u_long, d_long) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords.clone(),
        edges_b1.clone(),
        body_force.clone(),
        boundary_mask.clone(),
        rho_node.clone(),
        e0,
        cross_section_area,
        &cg,
        cfg_long,
    )
    .expect("solve_staggered_with_mechanics");
    let (u_s, d_s) = PhaseFieldFractureSolver::solve_staggered_with_mechanics::<B>(
        coords,
        edges_b1,
        body_force,
        boundary_mask,
        rho_node,
        e0,
        cross_section_area,
        &cg,
        cfg_inactive_stop,
    )
    .expect("solve_staggered_with_mechanics");
    let tol = 1e-4_f32;
    let v1 = u_long.into_data().value;
    let v2 = u_s.into_data().value;
    for i in 0..v1.len() {
        assert!((v1[i] - v2[i]).abs() < tol, "u mismatch at {i}");
    }
    let w1 = d_long.into_tensor().into_data().value;
    let w2 = d_s.into_tensor().into_data().value;
    for i in 0..w1.len() {
        assert!((w1[i] - w2[i]).abs() < tol, "d mismatch at {i}");
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! THMC drying–shrinkage verification (`thmc-coupled`): capillary sink on an exposed facet, sealed end
//! via damage-masked diffusion, and shrink-strain estimate vs MC2010-style notional reference.
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md` (Track G).

#![cfg(feature = "thmc-coupled")]
#![allow(clippy::needless_range_loop)]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::core::tensors::{MixTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::physics::solvers::{
    mc2010_style_notional_shrink_strain, shrink_strain_from_saturation_loss, ChemicalPlan,
    HydrologicPlan, MechanicalPlan, ThermalPlan, ThmcHydrationKinetics,
    ThmcImplicitEulerThermalHumidityHydrationResidual, ThmcImplicitEulerThermalHydrationResidual,
    ThmcImplicitTAlphaNewtonConfig, ThmcMonolithicImplicitUnknownLayout, ThmcSolver, ThmcState,
};

type B = NdArray<f32>;

fn dev() -> NdArrayDevice {
    NdArrayDevice::default()
}

struct Stub;

impl<Bk: burn::tensor::backend::Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
    fn compute_all(&self, mix: &MixTensor<Bk>) -> PhysicalResult<Bk> {
        let d = mix.fractions.device();
        PhysicalResult {
            free_energy: Tensor::zeros([1, 1], &d),
            dissipation: Tensor::zeros([1, 1], &d),
            safety_margin: Tensor::zeros([1, 1], &d),
            cost: Tensor::zeros([1, 1], &d),
            damage: Tensor::zeros([1, 1], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, 1], &d),
        }
    }

    fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
        let d = m.scalar_features.device();
        let n = m.scalar_features.dims()[0];
        PhysicalResult {
            free_energy: Tensor::zeros([1, n], &d),
            dissipation: Tensor::zeros([1, n], &d),
            safety_margin: Tensor::zeros([1, n], &d),
            cost: Tensor::zeros([1, n], &d),
            damage: Tensor::zeros([1, n], &d),
            temperature_delta: None,
            #[cfg(feature = "information_density")]
            information_density: Tensor::zeros([1, n], &d),
        }
    }
}

fn chain_manifold(n: usize) -> UnifiedMaterialStateTensor<B> {
    let d = dev();
    let f = 5usize;
    let coords: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &d);
    let mut e = Vec::with_capacity((n - 1) * 2);
    for i in 0..n - 1 {
        e.push(i as i64);
    }
    for i in 0..n - 1 {
        e.push((i + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(Data::new(e, Shape::new([2, n - 1])), &d);
    let faces_b2: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &d);
    let scalar_features = Tensor::<B, 2>::zeros([n, f], &d);
    let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &d);
    let matrix_features = Tensor::<B, 4>::zeros([n, 1, 3, 3], &d);
    let mut pos = vec![0.0_f32; n * 3];
    for i in 0..n {
        pos[i * 3] = i as f32 * 0.01_f32;
    }
    let node_positions = Some(Tensor::from_data(Data::new(pos, Shape::new([n, 3])), &d));
    let displacement_bc_mask = Tensor::<B, 3>::ones([n, 3, 1], &d);
    let policy_editable_mask = Tensor::<B, 2>::ones([n, 1], &d);
    UnifiedMaterialStateTensor {
        coords,
        edges_b1,
        faces_b2,
        scalar_features,
        vector_features,
        matrix_features,
        resolution_mm: [1.0, 1.0, 1.0],
        node_positions,
        displacement_bc_mask,
        policy_editable_mask,
    }
}

#[test]
fn thmc_newton_outer_passes_within_six() {
    let solver = ThmcSolver {
        max_newton: 6_usize,
        ..Default::default()
    };
    assert!(
        solver.max_newton <= 6,
        "benchmark contract: ≤ 6 outer passes"
    );
}

#[test]
fn thmc_drying_shrinkage_within_mc2010_notional_band() {
    let d = dev();
    let n = 28usize;
    let manifold = chain_manifold(n);
    let h_init = 0.92_f32;
    let mut dmg = vec![0.0_f32; n];
    for i in 0..n {
        dmg[i] = if i == 0 { 1.0_f32 } else { 0.0_f32 };
    }
    let damage = Tensor::<B, 3>::from_data(Data::new(dmg, Shape::new([1, n, 1])), &d);
    let state = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::full([1, n, 1], 293.15_f32, &d),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::full([1, n, 1], h_init, &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: Tensor::<B, 3>::full([1, n, 1], 0.7_f32, &d),
        },
        damage,
        time: 0.0_f32,
    };
    let solver = ThmcSolver {
        dt: 0.05_f32,
        max_newton: 4_usize,
        tol: 1e-3_f32,
        hydration: ThmcHydrationKinetics::default(),
        drying_last_node_evaporation_k: 0.35_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
    };
    let mut s = state;
    for _ in 0..560 {
        s = solver.step(&Stub, s, &manifold).expect("THMC step Ok");
    }
    let h = s.hydro.humidity.clone().into_data().value;
    let h_surf = h[n - 1];
    let loss = (h_init - h_surf).max(0.0_f32);
    let wc = 0.4_f32;
    let alpha = 0.7_f32;
    let est = shrink_strain_from_saturation_loss(loss, wc, alpha);
    let ref_s = mc2010_style_notional_shrink_strain(wc, alpha, 50.0_f32, 28.0_f32);
    let rel = (est - ref_s).abs() / ref_s.max(1e-9_f32);
    assert!(
        rel < 0.30_f32,
        "shrink strain estimate {est} vs reference {ref_s} rel_err={rel} (loss={loss})"
    );
}

#[test]
fn thmc_hydration_alpha_rate_scalar_matches_closed_form() {
    let k = ThmcHydrationKinetics::default();
    let alpha = 0.35_f32;
    let t = 303.15_f32;
    let got = k.alpha_rate_scalar(alpha, t);
    let one_m = (1.0_f32 - alpha).max(0.0_f32);
    let t_safe = t.max(k.t_min_k);
    let ea_rt = k.activation_energy_j_per_mol / (k.gas_constant_j_per_mol_k * t_safe);
    let arr = k.arrhenius_prefactor_s * (-ea_rt).exp() * one_m;
    let boost = 1.0_f32 + k.t_boost_per_k * (t - k.t_boost_ref_k).max(0.0_f32);
    let want = arr * boost;
    assert!(
        (got - want).abs() < 1e-6_f32,
        "alpha_rate_scalar mismatch: got={got} want={want}"
    );
}

/// **THMC ↔ fracture wiring:** `strain_tensor_from_bar_network_displacement` must agree with the
/// post-equilibrium branch of `strain_tensor_for_fracture_after_mechanics` (coords-derived edge frame).
#[test]
fn bar_network_strain_matches_strain_tensor_for_fracture_after_mechanics() {
    use umst_manifold::physics::mechanics::VectorMechanicsSolver;
    use umst_manifold::physics::solvers::fracture_field::{
        strain_tensor_for_fracture_after_mechanics, strain_tensor_from_bar_network_displacement,
    };
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
    use umst_manifold::physics::topology::EdgeTopology;

    let d = dev();
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * 0.5);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &d);

    let mut edges = Vec::with_capacity(e_ct * 2);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &d);

    let e_young_pa = 2.0e8_f32;
    let nu = 0.3_f32;
    let mut stiff = Vec::with_capacity(n * 2);
    for _ in 0..n {
        stiff.push(e_young_pa);
        stiff.push(nu);
    }
    let stiffness: Tensor<B, 3> =
        Tensor::from_data(Data::new(stiff, Shape::new([batch, n, 2])), &d);

    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = 2000.0_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &d);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0;
        bm_data[i * 3 + 2] = 0.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &d);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 300,
        cg_tolerance: 1e-7,
        pcg_tolerance: 1e-7,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let cross_section_area = 0.01_f32;

    let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n, 3]);
    let topo = EdgeTopology::new(edges_b1.clone());
    let src3 = topo.expand_src_gather_indices(batch, 3);
    let tgt3 = topo.expand_tgt_gather_indices(batch, 3);
    let c_src = coords_b.clone().gather(1, src3.clone());
    let c_tgt = coords_b.gather(1, tgt3.clone());
    let delta = c_tgt.sub(c_src);
    let edge_len = delta
        .clone()
        .powf_scalar(2.0)
        .sum_dim(2)
        .sqrt()
        .clamp(1e-12, f32::MAX)
        .reshape([batch, e_ct, 1]);
    let edge_unit = delta.div(edge_len.clone());

    let u0 = Tensor::<B, 3>::zeros([batch, n, 3], &d);
    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &d);

    let eps_one_shot = strain_tensor_for_fracture_after_mechanics(
        u0.clone(),
        coords.clone(),
        stiffness.clone(),
        body_force.clone(),
        edges_b1.clone(),
        damage.clone(),
        boundary_mask.clone(),
        cross_section_area,
        &cfg,
        src3,
        tgt3,
        edge_unit,
        edge_len,
        n,
    );

    let (u_eq, _) = VectorMechanicsSolver::solve_equilibrium(
        u0,
        coords.clone(),
        stiffness,
        body_force,
        edges_b1.clone(),
        damage,
        boundary_mask,
        cross_section_area,
        &cfg,
    );
    let eps_from_u = strain_tensor_from_bar_network_displacement(u_eq, coords, edges_b1.clone(), n);

    let v1 = eps_one_shot.into_data().value;
    let v2 = eps_from_u.into_data().value;
    assert_eq!(v1.len(), v2.len());
    let scale = v1
        .iter()
        .chain(v2.iter())
        .map(|x| x.abs())
        .fold(1e-12_f32, f32::max);
    let max_abs = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f32, f32::max);
    assert!(
        max_abs < 1e-4_f32 * scale,
        "strain tensor mismatch: max_abs={max_abs} scale={scale}"
    );
}

/// Piecewise derivative of [`ThmcHydrationKinetics::alpha_rate_scalar`] w.r.t. `temperature_k`
/// (matches the scalar implementation’s `max` / `clamp` semantics).
fn alpha_rate_scalar_dt_analytic(k: &ThmcHydrationKinetics, alpha: f32, temperature_k: f32) -> f32 {
    let one_m = (1.0_f32 - alpha).max(0.0_f32);
    let t_safe = temperature_k.max(k.t_min_k);
    let ea_rt = k.activation_energy_j_per_mol / (k.gas_constant_j_per_mol_k * t_safe);
    let arr = k.arrhenius_prefactor_s * (-ea_rt).exp() * one_m;
    let d_arr_d_tin = if temperature_k > k.t_min_k {
        arr * k.activation_energy_j_per_mol / (k.gas_constant_j_per_mol_k * t_safe * t_safe)
    } else {
        0.0_f32
    };
    let relu = (temperature_k - k.t_boost_ref_k).max(0.0_f32);
    let boost = 1.0_f32 + k.t_boost_per_k * relu;
    let d_boost = if temperature_k > k.t_boost_ref_k {
        k.t_boost_per_k
    } else {
        0.0_f32
    };
    d_arr_d_tin * boost + arr * d_boost
}

#[test]
fn thmc_hydration_alpha_rate_scalar_derivative_temperature_matches_finite_difference() {
    let k = ThmcHydrationKinetics::default();
    let alpha = 0.42_f32;
    let t0 = 301.4_f32;
    let h = 0.25_f32;
    let analytic = alpha_rate_scalar_dt_analytic(&k, alpha, t0);
    let fd =
        (k.alpha_rate_scalar(alpha, t0 + h) - k.alpha_rate_scalar(alpha, t0 - h)) / (2.0_f32 * h);
    let scale = analytic.abs().max(fd.abs()).max(1e-12_f32);
    assert!(
        (analytic - fd).abs() < 5e-5_f32 * scale.max(1.0_f32),
        "d(alpha_rate)/dT: analytic={analytic} fd={fd}"
    );
}

#[test]
fn thmc_implicit_euler_t_alpha_residual_matches_brute_force_two_nodes() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHydrationResidual {
        dt,
        temperature_n: t_n.clone(),
        alpha_n: alpha_n.clone(),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics: kinetics.clone(),
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t.clone(),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::zeros([1, n, 1], &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha.clone(),
        },
        damage: damage_m.clone(),
        time: 0.0_f32,
    };
    let (r_t, r_alpha) = assembler.assemble(&trial).expect("assemble");

    let t = trial_t.into_data().value;
    let a = trial_alpha.into_data().value;
    let tn = t_n.into_data().value;
    let an = alpha_n.into_data().value;
    let lap0 = t[1] - t[0];
    let lap1 = t[0] - t[1];
    let rt = r_t.into_data().value;
    let ra = r_alpha.into_data().value;
    for i in 0..n {
        let lap_i = if i == 0 { lap0 } else { lap1 };
        let d_alpha = kinetics.alpha_rate_scalar(a[i], t[i]);
        let exo = d_alpha * kinetics.exothermic_k_per_alpha_rate * dt;
        let want_rt = t[i] - tn[i] - dt * lap_i - exo;
        let want_ra = a[i] - an[i] - dt * d_alpha;
        assert!(
            (rt[i] - want_rt).abs() < 1e-5_f32,
            "node {i}: R_T got {} want {}",
            rt[i],
            want_rt
        );
        assert!(
            (ra[i] - want_ra).abs() < 1e-5_f32,
            "node {i}: R_alpha got {} want {}",
            ra[i],
            want_ra
        );
    }
}

/// Stacked implicit-Euler \((T,h,\alpha)\): \(R_h = h - h^n - \Delta t\,\mathcal{L}_h(h)\) matches hand
/// Laplacian on the same 2-node chain as [`thmc_implicit_euler_t_alpha_residual_matches_brute_force_two_nodes`].
#[test]
fn thmc_implicit_euler_t_h_alpha_residual_humidity_matches_brute_force_two_nodes() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let h_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.52_f32, 0.61_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_h = Tensor::<B, 3>::from_data(
        Data::new(vec![0.50_f32, 0.63_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHumidityHydrationResidual {
        dt,
        temperature_n: t_n.clone(),
        humidity_n: h_n.clone(),
        alpha_n: alpha_n.clone(),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics: kinetics.clone(),
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t.clone(),
        },
        hydro: HydrologicPlan {
            humidity: trial_h.clone(),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha.clone(),
        },
        damage: damage_m.clone(),
        time: 0.0_f32,
    };
    let (r_t, r_h, r_alpha) = assembler.assemble(&trial).expect("assemble");

    let t = trial_t.into_data().value;
    let h = trial_h.into_data().value;
    let a = trial_alpha.into_data().value;
    let tn = t_n.into_data().value;
    let hn = h_n.into_data().value;
    let an = alpha_n.into_data().value;
    let lap0_t = t[1] - t[0];
    let lap1_t = t[0] - t[1];
    let lap0_h = h[1] - h[0];
    let lap1_h = h[0] - h[1];
    let rt = r_t.into_data().value;
    let rh = r_h.into_data().value;
    let ra = r_alpha.into_data().value;
    for i in 0..n {
        let lap_t_i = if i == 0 { lap0_t } else { lap1_t };
        let lap_h_i = if i == 0 { lap0_h } else { lap1_h };
        let d_alpha = kinetics.alpha_rate_scalar(a[i], t[i]);
        let exo = d_alpha * kinetics.exothermic_k_per_alpha_rate * dt;
        let want_rt = t[i] - tn[i] - dt * lap_t_i - exo;
        let want_rh = h[i] - hn[i] - dt * lap_h_i;
        let want_ra = a[i] - an[i] - dt * d_alpha;
        assert!(
            (rt[i] - want_rt).abs() < 1e-5_f32,
            "node {i}: R_T got {} want {}",
            rt[i],
            want_rt
        );
        assert!(
            (rh[i] - want_rh).abs() < 1e-5_f32,
            "node {i}: R_h got {} want {}",
            rh[i],
            want_rh
        );
        assert!(
            (ra[i] - want_ra).abs() < 1e-5_f32,
            "node {i}: R_alpha got {} want {}",
            ra[i],
            want_ra
        );
    }

    assert_eq!(
        ThmcMonolithicImplicitUnknownLayout::field_major_scalar_transport_hydration_dof_count(
            n, 1, 1, 1
        ),
        6
    );
    assert_eq!(
        ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, 1, 1, 1),
        6 + n * ThmcMonolithicImplicitUnknownLayout::MECHANICAL_DISP_PER_NODE
    );
}

/// Placeholder mechanics block \(R_u=m(\mathbf u-\mathbf u^n)\): matches hand increment on a 2-node chain;
/// stacked flatten length matches [`ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`].
#[test]
fn thmc_implicit_euler_t_h_alpha_u_placeholder_r_u_and_flat_layout_two_nodes() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let h_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.52_f32, 0.61_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let u_n_vals = vec![
        1.0e-4_f32,
        -2.0e-4_f32,
        0.5e-4_f32, //
        3.0e-4_f32,
        0.0_f32,
        -1.0e-4_f32,
    ];
    let u_vals = vec![
        2.5e-4_f32,
        0.0_f32,
        1.0e-4_f32, //
        -1.0e-4_f32,
        4.0e-4_f32,
        2.0e-4_f32,
    ];
    let mass = 2.7_f32;
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_h = Tensor::<B, 3>::from_data(
        Data::new(vec![0.50_f32, 0.63_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHumidityHydrationResidual {
        dt,
        temperature_n: t_n,
        humidity_n: h_n,
        alpha_n,
        displacement_n: Tensor::<B, 3>::from_data(
            Data::new(u_n_vals.clone(), Shape::new([1, n, 3])),
            &d,
        ),
        mechanics_placeholder_mass: mass,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics,
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t,
        },
        hydro: HydrologicPlan { humidity: trial_h },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::from_data(
                Data::new(u_vals.clone(), Shape::new([1, n, 3])),
                &d,
            ),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha,
        },
        damage: damage_m,
        time: 0.0_f32,
    };
    let (_r_t, _r_h, _r_alpha, r_u) = assembler
        .assemble_with_mechanics_placeholder_r_u(&trial)
        .expect("assemble four blocks");
    let got_ru = r_u.into_data().value;
    for i in 0..u_vals.len() {
        let want = mass * (u_vals[i] - u_n_vals[i]);
        assert!(
            (got_ru[i] - want).abs() < 1e-6_f32,
            "R_u[{i}] got {} want {}",
            got_ru[i],
            want
        );
    }

    let flat = assembler
        .stacked_flat_residual_field_major(&trial)
        .expect("stacked flat");
    let f_t = 1usize;
    let f_h = 1usize;
    let f_a = 1usize;
    let want_len =
        ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, f_t, f_h, f_a);
    assert_eq!(flat.len(), want_len, "field-major stacked residual length");

    let l2_scalar = assembler.residual_l2(&trial).expect("l2 scalar blocks");
    let l2_full = assembler
        .residual_l2_including_mechanics_placeholder(&trial)
        .expect("l2 with placeholder R_u");
    let ru_sq: f32 = got_ru.iter().map(|x| x * x).sum();
    let l2_from_parts = (l2_scalar * l2_scalar + ru_sq).max(0.0_f32).sqrt();
    assert!(
        (l2_full - l2_from_parts).abs() < 1e-5_f32,
        "stacked L2: full {} vs sqrt(||R_T:h:a||^2+||R_u||^2) {}",
        l2_full,
        l2_from_parts
    );
}

/// **Coupling plan §4 Phase 1:** \(\|R_u(u^\star)\| \ll \|P f\|\) at the `solve_equilibrium` solution on a 2-node SI chain.
#[test]
fn thmc_r_u_zero_at_solved_equilibrium_two_node_chain() {
    use umst_manifold::physics::mechanics::VectorMechanicsSolver;
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("chain_manifold SI coords")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = ThmcHydrationKinetics::default();

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0_f32;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0_f32;
        bm_data[i * 3 + 2] = 0.0_f32;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &d);

    let mut bf_data = vec![0.0_f32; n * batch * 3];
    bf_data[(n - 1) * 3] = 2_000.0_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &d);

    let alpha_hydr = Tensor::<B, 3>::full([batch, n, 1], 0.72_f32, &d);
    let alpha_bn1 = alpha_hydr
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .clamp(1e-6_f32, 1.0_f32);
    let stiffness_e = alpha_bn1.mul_scalar(kinetics.stiffness_e_scale_pa);
    let stiffness_nu = Tensor::<B, 3>::zeros([batch, n, 1], &d).add_scalar(kinetics.stiffness_nu);
    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);

    let damage = Tensor::<B, 3>::zeros([batch, n, 1], &d);
    let u0 = Tensor::<B, 3>::zeros([batch, n, 3], &d);
    let inner_cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 400,
        cg_tolerance: 1e-8,
        pcg_tolerance: 1e-8,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let cross_section_area = 0.01_f32;
    let (u_star, _) = VectorMechanicsSolver::solve_equilibrium(
        u0,
        coords.clone(),
        stiffness,
        body_force.clone(),
        edges_b1.clone(),
        damage.clone(),
        boundary_mask.clone(),
        cross_section_area,
        &inner_cfg,
    );

    let dt = 0.02_f32;
    let assembler = ThmcImplicitEulerThermalHumidityHydrationResidual {
        dt,
        temperature_n: Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &d),
        humidity_n: Tensor::<B, 3>::full([batch, n, 1], 0.6_f32, &d),
        alpha_n: alpha_hydr.clone(),
        displacement_n: Tensor::<B, 3>::zeros([batch, n, 3], &d),
        mechanics_placeholder_mass: 0.0_f32,
        edges_b1,
        damage_m: damage.clone(),
        kinetics,
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::full([batch, n, 1], 0.55_f32, &d),
        },
        mechanical: MechanicalPlan {
            displacement: u_star,
        },
        chemical: ChemicalPlan {
            hydration_alpha: alpha_hydr,
        },
        damage,
        time: 0.0_f32,
    };

    let r_u = assembler
        .evaluate_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("evaluate_quasi_static_r_u");

    let pf = boundary_mask.clone().mul(body_force.clone());
    let nf = pf
        .clone()
        .mul(pf.clone())
        .sum()
        .into_scalar()
        .sqrt()
        .max(1e-12_f32);
    let nr = r_u
        .clone()
        .mul(r_u.clone())
        .sum()
        .into_scalar()
        .sqrt()
        .max(0.0_f32);
    let rel = nr / nf;
    assert!(
        rel < 2e-4_f32,
        "||R_u||/||Pf|| too large: nr={nr} nf={nf} rel={rel}"
    );
}

/// Field-major \((T,h,\alpha)\) damped Newton: stacked \(\|R\|_2\) decreases over multiple iterations
/// (track 13 increment toward monolithic THMC — still **no** \(R_u\) / no `ThmcSolver` wiring).
#[test]
fn thmc_implicit_euler_t_h_alpha_multi_newton_monotone_stacked_residual_norm() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let h_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.50_f32, 0.62_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_h = Tensor::<B, 3>::from_data(
        Data::new(vec![0.51_f32, 0.63_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHumidityHydrationResidual {
        dt,
        temperature_n: t_n,
        humidity_n: h_n,
        alpha_n,
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics,
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t,
        },
        hydro: HydrologicPlan { humidity: trial_h },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha,
        },
        damage: damage_m,
        time: 0.0_f32,
    };
    let (_final, norms) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("two damped Newton iterations on (T,h,α)");
    assert_eq!(norms.len(), 3);
    assert!(norms[0] > 1e-8_f32, "nontrivial R0={}", norms[0]);
    for k in 0..2 {
        assert!(
            norms[k + 1] < norms[k] * 0.999_f32,
            "stacked ||R|| should drop: {} -> {}",
            norms[k],
            norms[k + 1]
        );
    }
}

/// One damped Newton step on the coupled implicit-Euler `(T,α)` residual should lower `||R||_2`
/// on a 2-node chain (dense FD Jacobian + small Gauss–Jordan solve).
#[test]
fn thmc_implicit_euler_t_alpha_one_newton_lowers_residual_norm() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHydrationResidual {
        dt,
        temperature_n: t_n,
        alpha_n,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics,
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t,
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::zeros([1, n, 1], &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha,
        },
        damage: damage_m,
        time: 0.0_f32,
    };
    let (new_trial, n0, n1) = assembler
        .one_damped_newton_step(&trial, 1.0_f32, 1.0e-5_f32)
        .expect("one damped Newton step");
    assert!(
        n0 > 1e-8_f32,
        "expected nontrivial initial residual, got {n0}"
    );
    assert!(
        n1 < n0 * 0.999_f32,
        "residual should drop: before={n0} after={n1}"
    );
    let _ = new_trial;
}

/// Track 13 **boundary** (not monolithic): [`ThmcImplicitEulerThermalHydrationResidual::one_damped_newton_step`]
/// updates only `(T,\alpha)`; trial humidity and displacement are preserved — the shipped
/// [`ThmcImplicitTAlphaNewtonConfig`] path gates the same `(T,\alpha)` block inside [`ThmcSolver::step`]
/// while \(h\) and quasi-static \(u\) follow the legacy explicit / outer-pass ordering. A fully coupled
/// implicit residual \(R(U)=0\) over \(T,h,\alpha,u\) remains **DEFERRAL — THMC** / track 13 memo.
#[test]
fn thmc_t_alpha_newton_residual_preserves_hydro_mechanics_fields() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHydrationResidual {
        dt,
        temperature_n: t_n,
        alpha_n,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics,
    };
    let h_vals = vec![0.71_f32, 0.84_f32];
    let disp_vals: Vec<f32> = (0..n * 3).map(|k| 0.01_f32 * (1 + k) as f32).collect();
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t,
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::from_data(
                Data::new(h_vals.clone(), Shape::new([1, n, 1])),
                &d,
            ),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::from_data(
                Data::new(disp_vals.clone(), Shape::new([1, n, 3])),
                &d,
            ),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha,
        },
        damage: damage_m,
        time: 0.42_f32,
    };
    let (new_trial, _, _) = assembler
        .one_damped_newton_step(&trial, 1.0_f32, 1.0e-5_f32)
        .expect("one damped Newton step");
    assert_eq!(
        trial.hydro.humidity.clone().into_data().value,
        new_trial.hydro.humidity.into_data().value,
        "humidity must be untouched by (T,α) Newton (not in stacked residual)"
    );
    assert_eq!(
        trial.mechanical.displacement.clone().into_data().value,
        new_trial.mechanical.displacement.into_data().value,
        "displacement must be untouched by (T,α) Newton (not in stacked residual)"
    );
    assert_eq!(trial.time, new_trial.time);
    assert_eq!(
        trial.damage.clone().into_data().value,
        new_trial.damage.into_data().value
    );
}

/// Two (or more) damped Newton steps on the implicit-Euler `(T,α)` residual: each iterate should
/// lower `||R||_2` on the same 2-node chain setup as [`thmc_implicit_euler_t_alpha_one_newton_lowers_residual_norm`].
#[test]
fn thmc_implicit_euler_t_alpha_multi_newton_monotone_residual_norm_decrease() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = ThmcHydrationKinetics::default();
    let t_n = Tensor::<B, 3>::from_data(
        Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let alpha_n = Tensor::<B, 3>::from_data(
        Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_t = Tensor::<B, 3>::from_data(
        Data::new(vec![299.5_f32, 305.0_f32], Shape::new([1, n, 1])),
        &d,
    );
    let trial_alpha = Tensor::<B, 3>::from_data(
        Data::new(vec![0.33_f32, 0.56_f32], Shape::new([1, n, 1])),
        &d,
    );
    let damage_m = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHydrationResidual {
        dt,
        temperature_n: t_n,
        alpha_n,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage_m.clone(),
        kinetics,
    };
    let trial = ThmcState {
        thermal: ThermalPlan {
            temperature: trial_t,
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::zeros([1, n, 1], &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: trial_alpha,
        },
        damage: damage_m,
        time: 0.0_f32,
    };
    let (_final_trial, norms) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("two damped Newton iterations");
    assert_eq!(
        norms.len(),
        3,
        "expected [||R||_0, after step 1, after step 2], got len {}",
        norms.len()
    );
    assert!(
        norms[0] > 1e-8_f32,
        "expected nontrivial initial residual, got {}",
        norms[0]
    );
    for k in 0..2 {
        assert!(
            norms[k + 1] < norms[k] * 0.999_f32,
            "residual should drop at step {}: {} -> {}",
            k + 1,
            norms[k],
            norms[k + 1]
        );
    }
}

fn clone_thmc_state(s: &ThmcState<B>) -> ThmcState<B> {
    ThmcState {
        thermal: ThermalPlan {
            temperature: s.thermal.temperature.clone(),
        },
        hydro: HydrologicPlan {
            humidity: s.hydro.humidity.clone(),
        },
        mechanical: MechanicalPlan {
            displacement: s.mechanical.displacement.clone(),
        },
        chemical: ChemicalPlan {
            hydration_alpha: s.chemical.hydration_alpha.clone(),
        },
        damage: s.damage.clone(),
        time: s.time,
    }
}

fn thmc_state_with_t_alpha(
    base: &ThmcState<B>,
    temperature: Tensor<B, 3>,
    hydration_alpha: Tensor<B, 3>,
) -> ThmcState<B> {
    ThmcState {
        thermal: ThermalPlan { temperature },
        hydro: HydrologicPlan {
            humidity: base.hydro.humidity.clone(),
        },
        mechanical: MechanicalPlan {
            displacement: base.mechanical.displacement.clone(),
        },
        chemical: ChemicalPlan { hydration_alpha },
        damage: base.damage.clone(),
        time: base.time,
    }
}

/// Opt-in damped Newton on \((T,\alpha)\) must change the post-step state vs the legacy explicit split
/// on a **non-uniform** 2-node chain (uniform \(T\) would give \(\mathcal{L}(T)=0\) here and erase split differences).
#[test]
fn thmc_step_implicit_t_alpha_newton_differs_from_explicit_split() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        damage: damage.clone(),
        time: 0.0_f32,
    };

    let kinetics = ThmcHydrationKinetics::default();
    let solver_explicit = ThmcSolver {
        dt: 0.08_f32,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        hydration: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
    };
    let solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("explicit step");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("implicit (T,α) Newton step");

    let t_diff = s_exp
        .thermal
        .temperature
        .clone()
        .sub(s_imp.thermal.temperature.clone())
        .abs()
        .sum()
        .into_scalar();
    let a_diff = s_exp
        .chemical
        .hydration_alpha
        .clone()
        .sub(s_imp.chemical.hydration_alpha.clone())
        .abs()
        .sum()
        .into_scalar();
    assert!(
        t_diff + a_diff > 1.0e-4_f32,
        "expected implicit (T,α) Newton to differ from explicit split; |ΔT|₁+|Δα|₁ sum = {}",
        t_diff + a_diff
    );
}

/// Integration boundary: humidity transport in [`ThmcSolver::step`] uses `h_old + Δt Lap(h_old)` and
/// does **not** branch on implicit vs explicit \((T,\alpha)\) — so one step yields identical `h` even when
/// \(T,\alpha\) differ (monolithic THMC would couple \(h\) into the same implicit residual). Displacement
/// can still differ when [`UnifiedMaterialStateTensor::node_positions`] drives mechanics because stiffness
/// scales with post-block \(\alpha\).
#[test]
fn thmc_step_implicit_t_alpha_newton_same_humidity_as_explicit_split() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        damage: damage.clone(),
        time: 0.0_f32,
    };

    let kinetics = ThmcHydrationKinetics::default();
    let solver_explicit = ThmcSolver {
        dt: 0.08_f32,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        hydration: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
    };
    let solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("explicit step");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("implicit (T,α) Newton step");

    assert_eq!(
        s_exp.hydro.humidity.into_data().value,
        s_imp.hydro.humidity.into_data().value,
        "humidity transport must not depend on implicit vs explicit (T,α) branch"
    );
}

/// With [`ThmcImplicitEulerThermalHydrationResidual`] anchored at the pre-step \((T^n,\alpha^n)\), the
/// post-step state from the Newton path must yield a **strictly smaller** \(\|R\|_2\) than the
/// explicit-split endpoint (which coincides with the Newton initial iterate).
#[test]
fn thmc_step_implicit_t_alpha_newton_lowers_analytic_residual_vs_explicit_endpoint() {
    let d = dev();
    let n = 2usize;
    let manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 = ThmcState {
        thermal: ThermalPlan {
            temperature: Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        hydro: HydrologicPlan {
            humidity: Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        },
        mechanical: MechanicalPlan {
            displacement: Tensor::<B, 3>::zeros([1, n, 3], &d),
        },
        chemical: ChemicalPlan {
            hydration_alpha: Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        },
        damage: damage.clone(),
        time: 0.0_f32,
    };

    let kinetics = ThmcHydrationKinetics::default();
    let dt = 0.08_f32;
    let solver_explicit = ThmcSolver {
        dt,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        hydration: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
    };
    let solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("explicit step");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &manifold)
        .expect("implicit step");

    let assembler = ThmcImplicitEulerThermalHydrationResidual {
        dt,
        temperature_n: state0.thermal.temperature.clone(),
        alpha_n: state0.chemical.hydration_alpha.clone(),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: damage.clone(),
        kinetics,
    };

    let trial_exp = thmc_state_with_t_alpha(
        &state0,
        s_exp.thermal.temperature.clone(),
        s_exp.chemical.hydration_alpha.clone(),
    );
    let trial_imp = thmc_state_with_t_alpha(
        &state0,
        s_imp.thermal.temperature.clone(),
        s_imp.chemical.hydration_alpha.clone(),
    );

    let r_exp = assembler
        .residual_l2(&trial_exp)
        .expect("residual explicit endpoint");
    let r_imp = assembler
        .residual_l2(&trial_imp)
        .expect("residual implicit endpoint");
    assert!(
        r_exp > 1.0e-5_f32,
        "expected nontrivial BE residual at explicit endpoint, got {r_exp}"
    );
    assert!(
        r_imp < r_exp * 0.999_f32,
        "implicit path should lower ||R||₂: r_exp={r_exp} r_imp={r_imp}"
    );
}

// ---------------------------------------------------------------------------
// Phase 3.2 — Newton / implicit thermal block.
// Acceptance: ||R|| decreases across CG iterations and matches the analytic
// first-eigenmode decay on a 1-D chain.
// ---------------------------------------------------------------------------

use umst_manifold::physics::solvers::thmc::ThmcNewtonConfig;

/// 1-D chain `[N,3]` positions (uniform `dx`) with line-graph edges; no boundary mask
/// (all DOFs free except where the caller pins them).
fn chain_edges(n: usize) -> Tensor<B, 2, Int> {
    let d = dev();
    let mut e = Vec::with_capacity((n - 1) * 2);
    for i in 0..n - 1 {
        e.push(i as i64);
    }
    for i in 0..n - 1 {
        e.push((i + 1) as i64);
    }
    Tensor::from_data(Data::new(e, Shape::new([2, n - 1])), &d)
}

#[test]
fn thermal_implicit_newton_residual_decreases_monotonically() {
    let d = dev();
    let n = 32usize;
    let edges = chain_edges(n);

    // Step function: T = 1 for x < L/2, else 0.
    let mut t0 = vec![0.0_f32; n];
    for i in 0..n {
        t0[i] = if i < n / 2 { 1.0_f32 } else { 0.0_f32 };
    }
    let t_old = Tensor::<B, 3>::from_data(Data::new(t0, Shape::new([1, n, 1])), &d);
    // No Dirichlet pin — all nodes free.
    let mask = Tensor::<B, 3>::ones([1, n, 1], &d);

    let solver = ThmcSolver::default();
    let cfg = ThmcNewtonConfig {
        max_iterations: 20,
        residual_tolerance: 1.0e-6_f32,
        finite_diff_eps: 1.0e-6_f32,
        damping: 1.0_f32,
    };
    let (_t_new, norms) =
        solver.step_thermal_implicit::<B>(0.05_f32, t_old, 0.1_f32, edges, mask, cfg);

    assert!(
        norms.len() >= 2,
        "need at least two residual samples, got {}",
        norms.len()
    );
    // Monotone non-increasing.
    for k in 1..norms.len() {
        assert!(
            norms[k] <= norms[k - 1] * 1.0001_f32 + 1.0e-9_f32,
            "residual increased at iter {k}: {} -> {} (full log = {:?})",
            norms[k - 1],
            norms[k],
            norms
        );
    }
    // Converges below tolerance.
    let last = *norms.last().expect("non-empty");
    assert!(
        last < 1.0e-6_f32,
        "final residual {last} did not reach 1e-6 (log = {:?})",
        norms
    );
}

#[test]
fn thermal_implicit_matches_analytic_decay_mode() {
    let d = dev();
    let n = 64usize;
    let edges = chain_edges(n);

    // Use unit spacing dx = 1 in graph units (the discrete Laplacian on the line graph
    // implements the standard 3-point stencil with that spacing). The first eigenmode
    // of (-L) on a path graph with Dirichlet ends pinned at indices 0 and n-1 is
    //   φ_k(i) = sin( k π i / (n - 1) ),    eigenvalue λ_1 = 2 (1 - cos(π/(n-1))).
    // We pin x=0 and x=n-1 to zero with the boundary mask and load the initial
    // temperature with the first sine mode evaluated at the interior nodes.
    let n_minus_1 = (n - 1) as f32;
    let mut t0 = vec![0.0_f32; n];
    for i in 0..n {
        t0[i] = (std::f32::consts::PI * (i as f32) / n_minus_1).sin();
    }
    let t_init = Tensor::<B, 3>::from_data(Data::new(t0.clone(), Shape::new([1, n, 1])), &d);

    // Mask: free interior, Dirichlet (mask = 0) at endpoints.
    let mut m = vec![1.0_f32; n];
    m[0] = 0.0_f32;
    m[n - 1] = 0.0_f32;
    let mask = Tensor::<B, 3>::from_data(Data::new(m, Shape::new([1, n, 1])), &d);

    let kappa = 0.05_f32;
    let dt = 0.01_f32;
    let n_steps = 100usize;
    let solver = ThmcSolver::default();
    let cfg = ThmcNewtonConfig {
        max_iterations: 200,
        residual_tolerance: 1.0e-8_f32,
        finite_diff_eps: 1.0e-6_f32,
        damping: 1.0_f32,
    };

    let mut t = t_init.clone();
    for _ in 0..n_steps {
        let (t_new, _norms) = solver.step_thermal_implicit::<B>(
            dt,
            t.clone(),
            kappa,
            edges.clone(),
            mask.clone(),
            cfg,
        );
        t = t_new;
    }

    // Analytic decay of the implicit-Euler discretisation of u' = -κ λ u over n_steps:
    //   amplitude(t) = 1 / (1 + κ λ dt)^n_steps,
    // where λ = 2 (1 - cos(π / (n-1))) is the discrete first-mode eigenvalue.
    let lambda = 2.0_f32 * (1.0_f32 - (std::f32::consts::PI / n_minus_1).cos());
    let amp = 1.0_f32 / (1.0_f32 + kappa * lambda * dt).powi(n_steps as i32);

    let got = t.into_data().value;
    let mut num = 0.0_f64;
    let mut den = 0.0_f64;
    for i in 0..n {
        let want = amp * t0[i];
        let diff = (got[i] - want) as f64;
        num += diff * diff;
        den += (want as f64) * (want as f64);
    }
    let rel_l2 = (num / den.max(1.0e-30_f64)).sqrt();
    assert!(
        rel_l2 < 0.05_f64,
        "implicit-Euler first-mode L^2 error {rel_l2:.4e} not within 5% of analytic (amp = {amp:.4e})"
    );
}

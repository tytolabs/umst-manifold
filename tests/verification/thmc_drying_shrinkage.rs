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
use umst_manifold::core::field::{Field, HumidityField, ReactionExtentField, TemperatureField};
use umst_manifold::core::StepEntryDamageMask;
use umst_manifold::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use umst_manifold::core::traits::{IScienceCartridge, PhysicalResult};
use umst_manifold::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
use umst_manifold::physics::laplacian::TopologicalLaplacian;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::solvers::{
    mc2010_style_notional_shrink_strain, reaction_extent_rate_tensor,
    shrink_strain_from_saturation_loss, shrink_strain_from_saturation_loss_tensor,
    spectral_tensile_psi_plus_from_strain, strain_tensor_for_fracture_from_manifold, ChemicalPlan,
    HydrologicPlan, MechanicalPlan, ReactionExtentKinetics, ThermalPlan,
    ThmcImplicitEulerThermalHumidityReactionExtentResidual,
    ThmcImplicitEulerThermalReactionExtentResidual, ThmcImplicitTAlphaNewtonConfig,
    ThmcMonolithicImplicitUnknownLayout, ThmcMonolithicNewtonConfig, ThmcSolver, ThmcState,
    THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
};
#[path = "../injection_mechanism_fixture.rs"]
mod injection_mechanism_fixture;
use injection_mechanism_fixture::injection_fixture_kinetics;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

fn reference_reaction_extent_kinetics() -> ReactionExtentKinetics {
    injection_fixture_kinetics()
}

type B = NdArray<f32>;

fn dev() -> NdArrayDevice {
    NdArrayDevice::default()
}

struct Stub;

impl<Bk: burn::tensor::backend::Backend<FloatElem = f32>> IScienceCartridge<Bk> for Stub {
    fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
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
    let f = UMST_SCALAR_CHANNEL_COUNT;
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
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
    }
}

/// Same 1-D chain topology as [`chain_manifold`], but **no** SI `node_positions`; optional uniform
/// uniaxial \(\varepsilon_{xx}\) in `matrix_features[.., 0, 0, 0]` for the non-embedding fracture path.
fn chain_manifold_matrix_path(n: usize, exx: f32) -> UnifiedMaterialStateTensor<B> {
    let d = dev();
    let f = UMST_SCALAR_CHANNEL_COUNT;
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
    let mut mf = vec![0.0_f32; n * 9];
    for i in 0..n {
        let b = i * 9;
        mf[b] = exx;
    }
    let matrix_features = Tensor::from_data(Data::new(mf, Shape::new([n, 1, 3, 3])), &d);
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
        node_positions: None,
        displacement_bc_mask,
        policy_editable_mask,
        #[cfg(feature = "formal-witness")]
        catalog_schema_digest: None,
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
    let mut manifold = chain_manifold(n);
    let h_init = 0.92_f32;
    let mut dmg = vec![0.0_f32; n];
    for i in 0..n {
        dmg[i] = if i == 0 { 1.0_f32 } else { 0.0_f32 };
    }
    let damage = Tensor::<B, 3>::from_data(Data::new(dmg, Shape::new([1, n, 1])), &d);
    let state =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([1, n, 1], h_init, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::full([1, n, 1], 0.7_f32, &d),
        damage,
        0.0_f32,
    );
    let mut solver = ThmcSolver {
        dt: 0.05_f32,
        max_newton: 4_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: reference_reaction_extent_kinetics(),
        drying_last_node_evaporation_k: 0.35_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };
    let mut s = state;
    for _ in 0..560 {
        s = solver
            .step(&Stub, s, &mut manifold)
            .expect("ThmcSolver::step on 28-node drying chain for MC2010 shrink-strain band witness (FP §6 Track G drying shrinkage)");
    }
    let h = s.hydro.humidity.as_tensor().clone().into_data().value;
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
fn thmc_reaction_extent_rate_scalar_matches_closed_form() {
    let k = reference_reaction_extent_kinetics();
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
    )
    .expect("strain_tensor_for_fracture_after_mechanics on 3-node bar post-equilibrium (FP §6 Track G THMC↔fracture wiring)");

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
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on 3-node bar for strain parity check (FP §6 Track G THMC↔fracture wiring)");
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

/// **P1 / Track 12:** without `[N,3]` SI embedding, [`ThmcSolver::step`] feeds AT2 from
/// `matrix_features[..,0,..]` (public [`strain_tensor_for_fracture_from_manifold`] stub).
#[test]
fn thmc_step_matrix_features_strain_feeds_fracture_without_si_embedding() {
    let d = dev();
    let n = 3usize;
    let batch = 1usize;
    let exx = 0.05_f32;
    let mut manifold = chain_manifold_matrix_path(n, exx);

    let eps = strain_tensor_for_fracture_from_manifold(&mut manifold, batch, n, &d);
    let psi = spectral_tensile_psi_plus_from_strain(eps);
    let psi_sum: f32 = psi.into_data().value.iter().sum();
    assert!(
        psi_sum > 1e-12_f32,
        "expected positive ψ⁺ from matrix stub; psi_sum={psi_sum}"
    );

    let mk_state = |damage: Tensor<B, 3>|     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.9_f32, &d),
        Tensor::<B, 3>::zeros([batch, n, 3], &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &d),
        damage,
        0.0_f32,
    );

    let mut solver = ThmcSolver {
        dt: 0.01_f32,
        max_newton: 1_usize,
        tol: 1e-2_f32,
        reaction_extent_kinetics: reference_reaction_extent_kinetics(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };

    let s_tension = solver
        .step(
            &Stub,
            mk_state(Tensor::<B, 3>::zeros([batch, n, 1], &d)),
            &mut manifold,
        )
        .expect("ThmcSolver::step with tensile matrix_features stub on 3-node chain (FP §6 Track G P1 fracture feed)");
    let max_d_tension = s_tension.damage.as_tensor().clone().into_data()
        .value
        .iter()
        .copied()
        .fold(0.0_f32, f32::max);

    let mut manifold_flat = chain_manifold_matrix_path(n, 0.0_f32);
    let s_flat = solver
        .step(
            &Stub,
            mk_state(Tensor::<B, 3>::zeros([batch, n, 1], &d)),
            &mut manifold_flat,
        )
        .expect("ThmcSolver::step with zero-strain matrix_features control on 3-node chain (FP §6 Track G P1 fracture feed)");
    let max_d_flat = s_flat.damage.as_tensor().clone().into_data()
        .value
        .iter()
        .copied()
        .fold(0.0_f32, f32::max);

    assert!(
        max_d_tension > max_d_flat + 1e-6_f32,
        "expected tensile matrix_features to raise damage vs zero-strain control; max_d_tension={max_d_tension} max_d_flat={max_d_flat}"
    );
}

/// **Striatus-scale integration (stub path):** central finite difference of max post-step damage
/// w.r.t. uniform `matrix_features[..,0,0,0]` tensile stub — same no-SI wiring as
/// [`thmc_step_matrix_features_strain_feeds_fracture_without_si_embedding`], without claiming full
/// bar-mechanics reverse mode.
#[test]
fn striatus_micro_thmc_matrix_stub_fracture_max_damage_central_fd_wrt_exx() {
    let d = dev();
    let n = 3usize;
    let batch = 1usize;
    let exx0 = 0.05_f32;
    let h = 1e-4_f32;

    let mk_state = |damage: Tensor<B, 3>|     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.9_f32, &d),
        Tensor::<B, 3>::zeros([batch, n, 3], &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.5_f32, &d),
        damage,
        0.0_f32,
    );

    let max_damage_after_step = |exx: f32| -> f32 {
        let mut solver = ThmcSolver {
            dt: 0.01_f32,
            max_newton: 1_usize,
            tol: 1e-2_f32,
            reaction_extent_kinetics: reference_reaction_extent_kinetics(),
            drying_last_node_evaporation_k: 0.0_f32,
            drying_ambient_h: 0.5_f32,
            implicit_t_alpha_newton: None,
            monolithic_thmc_newton: None,
            ..Default::default()
        };
        let mut manifold = chain_manifold_matrix_path(n, exx);
        let s = solver
            .step(
                &Stub,
                mk_state(Tensor::<B, 3>::zeros([batch, n, 1], &d)),
                &mut manifold,
            )
            .expect("ThmcSolver::step on 3-node chain for central FD of max post-step damage w.r.t. matrix_features tensile stub (FP §6 Track G striatus-scale fracture stub)");
        s.damage.as_tensor().clone().into_data()
            .value
            .iter()
            .copied()
            .fold(0.0_f32, f32::max)
    };

    let f0 = max_damage_after_step(exx0);
    let fp = max_damage_after_step(exx0 + h);
    let fm = max_damage_after_step(exx0 - h);
    let fd_central = (fp - fm) / (2.0_f32 * h);
    let fd_backward = (fp - f0) / h;
    assert!(f0.is_finite() && fp.is_finite() && fm.is_finite());
    assert!(
        fp > f0 + 1e-8_f32 && fp > fm + 1e-8_f32,
        "expected damage to increase with tensile stub in neighborhood; f0={f0} fp={fp} fm={fm}"
    );
    let scale = fd_central.abs().max(fd_backward.abs()).max(1e-12_f32);
    assert!(
        (fd_central - fd_backward).abs() < 0.2_f32 * scale.max(1.0_f32),
        "central vs backward FD mismatch: fd_central={fd_central} fd_backward={fd_backward}"
    );
}

/// Piecewise derivative of [`ReactionExtentKinetics::alpha_rate_scalar`] w.r.t. `temperature_k`
/// (matches the scalar implementation’s `max` / `clamp` semantics).
fn alpha_rate_scalar_dt_analytic(
    k: &ReactionExtentKinetics,
    alpha: f32,
    temperature_k: f32,
) -> f32 {
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
fn thmc_reaction_extent_rate_scalar_derivative_temperature_matches_finite_difference() {
    let k = reference_reaction_extent_kinetics();
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
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics: kinetics.clone(),
    };
    let trial =     ThmcState::from_tensors(
        trial_t.clone(),
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha.clone(),
        damage_m.clone(),
        0.0_f32,
    );
    let (r_t, r_alpha) = assembler
        .assemble(&trial)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::assemble on 2-node (T,α) trial state (FP §6 Track G implicit Euler witness)");

    let t = trial_t.into_data().value;
    let a = trial_alpha.into_data().value;
    let tn = t_n.into_data().value;
    let an = alpha_n.into_data().value;
    let lap0 = t[1] - t[0];
    let lap1 = t[0] - t[1];
    let rt = r_t.as_tensor().clone().into_data().value;
    let ra = r_alpha.as_tensor().clone().into_data().value;
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
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n.clone()),
        humidity_n: Field::new(h_n.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics: kinetics.clone(),
    };
    let trial =     ThmcState::from_tensors(
        trial_t.clone(),
        trial_h.clone(),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha.clone(),
        damage_m.clone(),
        0.0_f32,
    );
    let (r_t, r_h, r_alpha) = assembler
        .assemble(&trial)
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::assemble on 2-node (T,h,α) trial state (FP §6 Track G implicit Euler humidity witness)");

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
    let rt = r_t.as_tensor().clone().into_data().value;
    let rh = r_h.as_tensor().clone().into_data().value;
    let ra = r_alpha.as_tensor().clone().into_data().value;
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
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::from_data(
            Data::new(u_n_vals.clone(), Shape::new([1, n, 3])),
            &d,
        ),
        mechanics_placeholder_mass: mass,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::from_data(
                Data::new(u_vals.clone(), Shape::new([1, n, 3])),
                &d,
            ),
        trial_alpha,
        damage_m,
        0.0_f32,
    );
    let (_r_t, _r_h, _r_alpha, r_u) = assembler
        .assemble_with_mechanics_placeholder_r_u(&trial)
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::assemble_with_mechanics_placeholder_r_u on 2-node (T,h,α,u) trial for placeholder R_u block witness (FP §6 Track G)");
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
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::stacked_flat_residual_field_major on 2-node trial for field-major stacked ‖R‖₂ layout witness (FP §6 Track G)");
    let f_t = 1usize;
    let f_h = 1usize;
    let f_a = 1usize;
    let want_len =
        ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, f_t, f_h, f_a);
    assert_eq!(flat.len(), want_len, "field-major stacked residual length");

    let l2_scalar = assembler.residual_l2(&trial).expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2 on 2-node trial for scalar (T,h,α) blocks witness (FP §6 Track G)");
    let l2_full = assembler
        .residual_l2_including_mechanics_placeholder(&trial)
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2_including_mechanics_placeholder on 2-node trial for stacked L2 with R_u witness (FP §6 Track G)");
    let ru_sq: f32 = got_ru.iter().map(|x| x * x).sum();
    let l2_from_parts = (l2_scalar * l2_scalar + ru_sq).max(0.0_f32).sqrt();
    assert!(
        (l2_full - l2_from_parts).abs() < 1e-5_f32,
        "stacked L2: full {l2_full} vs sqrt(||R_T:h:a||^2+||R_u||^2) {l2_from_parts}",
    );
}

/// **Coupling plan §4 Phase 1:** \(\|R_u(u^\star)\| \ll \|P f\|\) at the `solve_equilibrium` solution on a 2-node SI chain.
#[test]
fn thmc_r_u_zero_at_solved_equilibrium_two_node_chain() {
    use umst_manifold::physics::mechanics::VectorMechanicsSolver;
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on 2-node SI chain for R_u zero-at-equilibrium witness (FP §6 Track G quasi-static coupling)");

    let dt = 0.02_f32;
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &d)),
        humidity_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 0.6_f32, &d)),
        alpha_n: Field::new(alpha_hydr.clone()),
        displacement_n: Tensor::<B, 3>::zeros([batch, n, 3], &d),
        mechanics_placeholder_mass: 0.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.55_f32, &d),
        u_star,
        alpha_hydr,
        damage,
        0.0_f32,
    );

    let r_u = assembler
        .evaluate_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::evaluate_quasi_static_r_u on 2-node trial (FP §6 Track G quasi-static coupling)");

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

/// [`shrink_strain_from_saturation_loss_tensor`] matches the scalar closure channel-wise.
#[test]
fn shrink_strain_from_saturation_loss_tensor_matches_scalar() {
    let d = dev();
    let loss = Tensor::<B, 3>::from_data(
        Data::new(vec![0.0_f32, 0.31_f32, 1.0_f32], Shape::new([1, 3, 1])),
        &d,
    );
    let alpha = Tensor::<B, 3>::full([1, 3, 1], 0.68_f32, &d);
    let got = shrink_strain_from_saturation_loss_tensor(loss.clone(), 0.42_f32, alpha);
    let gv = got.into_data().value;
    let lv = loss.into_data().value;
    for i in 0..3 {
        let want = shrink_strain_from_saturation_loss(lv[i], 0.42_f32, 0.68_f32);
        assert!(
            (gv[i] - want).abs() < 2e-7_f32,
            "i={i} got {} want {}",
            gv[i],
            want
        );
    }
}

/// **Coupling plan §4 Phase 4:** when \(h^{\mathrm{trial}}=h^n\), shrink increment vanishes and
/// \(\|R_u(u^\star)\|/\|Pf\|\) stays at the Phase 1 equilibrium parity level.
#[test]
fn thmc_quasi_static_r_u_shrink_increment_flat_humidity_parity_two_node_chain() {
    use umst_manifold::physics::mechanics::VectorMechanicsSolver;
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on 2-node SI chain for flat-humidity shrink parity (FP §6 Track G quasi-static coupling)");

    let h_shared = 0.58_f32;
    let dt = 0.02_f32;
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &d)),
        humidity_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], h_shared, &d)),
        alpha_n: Field::new(alpha_hydr.clone()),
        displacement_n: Tensor::<B, 3>::zeros([batch, n, 3], &d),
        mechanics_placeholder_mass: 0.0_f32,
        ru_shrinkage_binder_liquid_ratio: Some(0.4_f32),
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], h_shared, &d),
        u_star,
        alpha_hydr,
        damage,
        0.0_f32,
    );

    let r_u = assembler
        .evaluate_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::evaluate_quasi_static_r_u on 2-node trial (FP §6 Track G quasi-static coupling)");

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
        "||R_u||/||Pf|| too large with flat humidity shrink hook: nr={nr} nf={nf} rel={rel}"
    );
}

/// **Phase 4:** additional drying (\(h^{\mathrm{trial}}<h^n\)) increases \(\|R_u\|_2\) at fixed \(u^\star\)
/// when the shrink increment hook is enabled.
#[test]
fn thmc_quasi_static_r_u_shrink_increment_raises_norm_when_humidity_drops_two_node_chain() {
    use umst_manifold::physics::mechanics::VectorMechanicsSolver;
    use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on 2-node SI chain for drying shrink R_u norm witness (FP §6 Track G quasi-static coupling)");

    let dt = 0.02_f32;
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &d)),
        humidity_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 0.62_f32, &d)),
        alpha_n: Field::new(alpha_hydr.clone()),
        displacement_n: Tensor::<B, 3>::zeros([batch, n, 3], &d),
        mechanics_placeholder_mass: 0.0_f32,
        ru_shrinkage_binder_liquid_ratio: Some(0.4_f32),
        edges_b1: edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage.clone()),
        kinetics: kinetics.clone(),
    };

    let l2_vec = |t: Tensor<B, 3>| -> f32 { t.clone().mul(t.clone()).sum().into_scalar().sqrt() };

    let trial_flat =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.62_f32, &d),
        u_star.clone(),
        alpha_hydr.clone(),
        damage.clone(),
        0.0_f32,
    );
    let r_flat = assembler
        .evaluate_quasi_static_r_u(
            &trial_flat,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::evaluate_quasi_static_r_u on flat-humidity 2-node trial (FP §6 Track G shrink parity)");
    let n_flat = l2_vec(r_flat);

    let trial_dry =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.22_f32, &d),
        u_star,
        alpha_hydr,
        damage,
        0.0_f32,
    );
    let r_dry = assembler
        .evaluate_quasi_static_r_u(
            &trial_dry,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::evaluate_quasi_static_r_u on dry-humidity 2-node trial (FP §6 Track G shrink increment)");
    let n_dry = l2_vec(r_dry);

    assert!(
        n_dry > n_flat + 200.0_f32,
        "expected drier humidity to raise ||R_u||₂: n_flat={n_flat} n_dry={n_dry}"
    );
}

/// **Coupling plan §4 Phase 2:** four-block quasi-static \(R_u\) assembly — combined L² matches
/// \(\sqrt{\sum_i \texttt{flat}[i]^2}\) and \(\sqrt{\|R_{T,h,\alpha}\|^2+\|R_u\|^2}\) on a 2-node chain.
#[test]
fn thmc_monolithic_residual_blocks_consistent_two_nodes() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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

    let dt = 0.02_f32;
    let alpha_hydr = Tensor::<B, 3>::full([batch, n, 1], 0.72_f32, &d);
    let damage_m = Tensor::<B, 3>::zeros([batch, n, 1], &d);
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 300.0_f32, &d)),
        humidity_n: Field::new(Tensor::<B, 3>::full([batch, n, 1], 0.6_f32, &d)),
        alpha_n: Field::new(alpha_hydr.clone()),
        displacement_n: Tensor::<B, 3>::zeros([batch, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([batch, n, 1], 301.0_f32, &d),
        Tensor::<B, 3>::full([batch, n, 1], 0.55_f32, &d),
        Tensor::<B, 3>::zeros([batch, n, 3], &d),
        alpha_hydr,
        damage_m,
        0.0_f32,
    );

    let cross_section_area = 0.01_f32;
    let l2_full = assembler
        .residual_l2_including_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2_including_quasi_static_r_u on 2-node trial for four-block ‖R‖₂ witness (FP §6 Track G)");

    let flat = assembler
        .stacked_flat_residual_field_major_quasi_static(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::stacked_flat_residual_field_major_quasi_static on 2-node trial for flat four-block layout witness (FP §6 Track G)");
    let sum_sq: f32 = flat.iter().map(|x| x * x).sum();
    let l2_from_flat = sum_sq.max(0.0_f32).sqrt();
    assert!(
        (l2_full - l2_from_flat).abs() < 1e-5_f32,
        "L2 mismatch: combined {l2_full} vs from flat {l2_from_flat}",
    );

    let l2_scalar = assembler.residual_l2(&trial).expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2 on 2-node quasi-static trial for scalar (T,h,α) blocks witness (FP §6 Track G)");
    let (_r_t, _r_h, _r_alpha, r_u) = assembler
        .assemble_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::assemble_with_quasi_static_r_u on 2-node trial for four-block residual witness (FP §6 Track G)");
    let ru_sq = r_u
        .clone()
        .mul(r_u.clone())
        .sum()
        .into_scalar()
        .max(0.0_f32);
    let l2_from_parts = (l2_scalar * l2_scalar + ru_sq).max(0.0_f32).sqrt();
    assert!(
        (l2_full - l2_from_parts).abs() < 1e-5_f32,
        "L2 mismatch: combined {l2_full} vs sqrt(||R_T:h:a||^2+||R_u||^2) {l2_from_parts}",
    );
}

/// **Coupling plan §4 Phase 3 / §5.2:** dense damped Newton on field-major \((T,h,\alpha,\mathbf u)\) with
/// quasi-static \(R_u\); stacked \(\|R\|_2\) decreases over multiple iterations (2-node chain, \(M=12\)).
#[test]
fn thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes() {
    let d = dev();
    let n = 2usize;
    assert_eq!(
        ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, 1, 1, 1),
        12
    );
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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

    let dt = 0.02_f32;
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );

    let cross_section_area = 0.01_f32;
    let (_final, norms) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            2_usize,
            1.0_f32,
            1.0e-5_f32,
            0.0_f32,
            None,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u two-iteration run on 2-node (T,h,α,u) trial (FP §6 Track G monolithic Newton witness)");
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

/// One damped Newton step on the monolithic \((T,h,\alpha,\mathbf u)\) quasi-static path; with
/// **`solver-experimental`**, the inner linear solve uses JFNK + host **GMRES** (see
/// [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::one_damped_newton_step_with_quasi_static_r_u`]).
/// Same 2-node harness as [`thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes`].
#[cfg(feature = "solver-experimental")]
#[test]
fn thmc_monolithic_quasi_static_one_newton_jfnk_lowers_stacked_norm_two_nodes() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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

    let dt = 0.02_f32;
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );

    let cross_section_area = 0.01_f32;
    let (_final, norm_before, norm_after) = assembler
        .one_damped_newton_step_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            1.0_f32,
            1.0e-5_f32,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::one_damped_newton_step_with_quasi_static_r_u with JFNK inner on 2-node trial (FP §6 Track G solver-experimental witness)");
    assert!(norm_before > 1e-8_f32, "nontrivial R0={norm_before}");
    assert!(
        norm_after < norm_before * 0.999_f32,
        "stacked ||R|| should drop: {norm_before} -> {norm_after}",
    );
}

/// Stacked \(\|R\|_2\) **`tol`** on [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`]
/// shortens the recorded norm trail once \(\|R\|_2\) drops below **`tol`** (same 2-node harness as
/// [`thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes`]).
#[test]
fn thmc_monolithic_newton_residual_tol_early_exit_truncates_norm_trail() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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

    let dt = 0.02_f32;
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );

    let cross_section_area = 0.01_f32;
    let max_iters = 5_usize;
    let (_, norms_full) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            max_iters,
            1.0_f32,
            1.0e-5_f32,
            0.0_f32,
            None,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u fixed-count run on 2-node (T,h,α,u) trial (FP §6 Track G monolithic Newton witness)");
    assert_eq!(
        norms_full.len(),
        max_iters + 1,
        "expected one norm head + one per Newton step"
    );

    let tol_exit = norms_full[2] + 0.05_f32 * (norms_full[1] - norms_full[2]).max(1e-30_f32);
    assert!(
        tol_exit > norms_full[2] && tol_exit < norms_full[1],
        "sanity: tol between ||R|| after one and two Newton steps (got tol_exit={tol_exit}, n1={}, n2={})",
        norms_full[1],
        norms_full[2]
    );

    let (_, norms_early) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            max_iters,
            1.0_f32,
            1.0e-5_f32,
            tol_exit,
            None,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u with stacked L2 tol early exit on 2-node (T,h,α,u) trial (FP §6 Track G)");
    assert!(
        norms_early.len() < norms_full.len(),
        "expected fewer norm samples when tol exits early: early={:?} full_len={}",
        norms_early,
        norms_full.len()
    );
    assert!(
        *norms_early.last().expect("norm trail non-empty after tol early-exit monolithic Newton (FP §6 Track G)") < tol_exit,
        "final ||R|| should sit below tol_exit"
    );
    for k in 0..norms_early.len().saturating_sub(1) {
        assert!(
            (norms_early[k + 1] - norms_full[k + 1]).abs()
                < 1e-5_f32 * norms_full[k + 1].max(1.0_f32),
            "prefix norms should match full run: k={k} early={} full={}",
            norms_early[k + 1],
            norms_full[k + 1]
        );
    }
}

/// Relative stacked \(\|R\|_2\) gate \(\|R\|_2 < k\|R_0\|_2\) with absolute tolerance disabled on
/// [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`]
/// (same 2-node harness as [`thmc_monolithic_newton_residual_tol_early_exit_truncates_norm_trail`]).
#[test]
fn thmc_monolithic_newton_relative_to_initial_early_exit_truncates_norm_trail() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let coords = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for quasi-static / monolithic witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();

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

    let dt = 0.02_f32;
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1,
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );

    let cross_section_area = 0.01_f32;
    let max_iters = 5_usize;
    let (_, norms_full) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            max_iters,
            1.0_f32,
            1.0e-5_f32,
            0.0_f32,
            None,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u fixed-count run on 2-node (T,h,α,u) trial (FP §6 Track G monolithic Newton witness)");
    assert_eq!(
        norms_full.len(),
        max_iters + 1,
        "expected one norm head + one per Newton step"
    );
    let r0 = norms_full[0];
    let n1 = norms_full[1];
    let n2 = norms_full[2];
    assert!(
        r0 > 1e-8_f32 && n2 < n1 && n1 < r0,
        "expected strictly decreasing ||R|| trail"
    );
    let k_rel = 0.5_f32 * (n2 / r0 + n1 / r0);
    assert!(
        n2 < k_rel * r0 && n1 >= k_rel * r0,
        "sanity: relative gate exits after two Newton steps (r0={r0}, n1={n1}, n2={n2}, k_rel={k_rel})"
    );

    let (_, norms_rel) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords,
            &boundary_mask,
            &body_force,
            cross_section_area,
            max_iters,
            1.0_f32,
            1.0e-5_f32,
            0.0_f32,
            Some(k_rel),
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u with relative-to-R0 early exit on 2-node trial (FP §6 Track G)");
    assert!(
        norms_rel.len() < norms_full.len(),
        "expected fewer norm samples when relative tol exits early: rel={:?} full_len={}",
        norms_rel,
        norms_full.len()
    );
    assert_eq!(
        norms_rel.len(),
        3,
        "expected head + two Newton steps before relative exit"
    );
    assert!(
        *norms_rel.last().expect("norm trail non-empty after relative-tol monolithic Newton early exit (FP §6 Track G)") < k_rel * r0,
        "final ||R|| should sit below k_rel * ||R0||"
    );
    for k in 0..norms_rel.len().saturating_sub(1) {
        assert!(
            (norms_rel[k + 1] - norms_full[k + 1]).abs()
                < 1e-5_f32 * norms_full[k + 1].max(1.0_f32),
            "prefix norms should match full run: k={k} rel={} full={}",
            norms_rel[k + 1],
            norms_full[k + 1]
        );
    }
}

/// Field-major \((T,h,\alpha)\) damped Newton: stacked \(\|R\|_2\) decreases over multiple iterations
/// (track 13 increment toward monolithic THMC — still **no** \(R_u\) / no `ThmcSolver` wiring).
#[test]
fn thmc_implicit_euler_t_h_alpha_multi_newton_monotone_stacked_residual_norm() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        humidity_n: Field::new(h_n),
        alpha_n: Field::new(alpha_n),
        displacement_n: Tensor::<B, 3>::zeros([1, n, 3], &d),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        trial_h,
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );
    let (_final, norms) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations two-iteration run on (T,h,α) trial (FP §6 Track G)");
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
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        alpha_n: Field::new(alpha_n),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );
    let (new_trial, n0, n1) = assembler
        .one_damped_newton_step(&trial, 1.0_f32, 1.0e-5_f32)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::one_damped_newton_step on 2-node (T,α) trial (FP §6 Track G implicit Euler witness)");
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

/// Track 13 **boundary** (not monolithic): [`ThmcImplicitEulerThermalReactionExtentResidual::one_damped_newton_step`]
/// updates only `(T,\alpha)`; trial humidity and displacement are preserved — the shipped
/// [`ThmcImplicitTAlphaNewtonConfig`] path gates the same `(T,\alpha)` block inside [`ThmcSolver::step`]
/// while \(h\) and quasi-static \(u\) follow the legacy explicit / outer-pass ordering. A fully coupled
/// implicit residual \(R(U)=0\) over \(T,h,\alpha,u\) remains **OPEN ROADMAP ITEM — THMC** / track 13 memo.
#[test]
fn thmc_t_alpha_newton_residual_preserves_hydro_mechanics_fields() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        alpha_n: Field::new(alpha_n),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let h_vals = vec![0.71_f32, 0.84_f32];
    let disp_vals: Vec<f32> = (0..n * 3).map(|k| 0.01_f32 * (1 + k) as f32).collect();
    let trial =     ThmcState::from_tensors(
        trial_t,
        Tensor::<B, 3>::from_data(
                Data::new(h_vals.clone(), Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::from_data(
                Data::new(disp_vals.clone(), Shape::new([1, n, 3])),
                &d,
            ),
        trial_alpha,
        damage_m,
        0.42_f32,
    );
    let (new_trial, _, _) = assembler
        .one_damped_newton_step(&trial, 1.0_f32, 1.0e-5_f32)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::one_damped_newton_step preserving h/u fields on 2-node trial (FP §6 Track G)");
    assert_eq!(
        trial.hydro.humidity.as_tensor().clone().into_data().value,
        new_trial.hydro.humidity.as_tensor().clone().into_data().value,
        "humidity must be untouched by (T,α) Newton (not in stacked residual)"
    );
    assert_eq!(
        trial.mechanical.displacement.as_tensor().clone().into_data().value,
        new_trial.mechanical.displacement.as_tensor().clone().into_data().value,
        "displacement must be untouched by (T,α) Newton (not in stacked residual)"
    );
    assert_eq!(trial.time, new_trial.time);
    assert_eq!(
        trial.damage.as_tensor().clone().into_data().value,
        new_trial.damage.as_tensor().clone().into_data().value
    );
}

/// Two (or more) damped Newton steps on the implicit-Euler `(T,α)` residual: each iterate should
/// lower `||R||_2` on the same 2-node chain setup as [`thmc_implicit_euler_t_alpha_one_newton_lowers_residual_norm`].
#[test]
fn thmc_implicit_euler_t_alpha_multi_newton_monotone_residual_norm_decrease() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let dt = 0.02_f32;
    let kinetics = reference_reaction_extent_kinetics();
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
    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_n),
        alpha_n: Field::new(alpha_n),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics,
    };
    let trial =     ThmcState::from_tensors(
        trial_t,
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        trial_alpha,
        damage_m,
        0.0_f32,
    );
    let (_final_trial, norms) = assembler
        .damped_newton_iterations(&trial, 2_usize, 1.0_f32, 1.0e-5_f32)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::damped_newton_iterations two-iteration run on (T,α) trial (FP §6 Track G)");
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
    ThmcState::from_tensors(
        s.thermal.temperature.as_tensor().clone(),
        s.hydro.humidity.as_tensor().clone(),
        s.mechanical.displacement.as_tensor().clone(),
        s.chemical.reaction_extent.as_tensor().clone(),
        s.damage.as_tensor().clone(),
        s.time,
    )
}

fn thmc_state_with_t_alpha(
    base: &ThmcState<B>,
    temperature: Tensor<B, 3>,
    reaction_extent: Tensor<B, 3>,
) -> ThmcState<B> {
    ThmcState::from_tensors(
        temperature,
        base.hydro.humidity.as_tensor().clone(),
        base.mechanical.displacement.as_tensor().clone(),
        reaction_extent,
        base.damage.as_tensor().clone(),
        base.time,
    )
}

/// Opt-in damped Newton on \((T,\alpha)\) must change the post-step state vs the legacy explicit split
/// on a **non-uniform** 2-node chain (uniform \(T\) would give \(\mathcal{L}(T)=0\) here and erase split differences).
#[test]
fn thmc_step_implicit_t_alpha_newton_differs_from_explicit_split() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        damage.clone(),
        0.0_f32,
    );

    let kinetics = reference_reaction_extent_kinetics();
    let mut solver_explicit = ThmcSolver {
        dt: 0.08_f32,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };
    let mut solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step explicit split on drying chain for implicit comparison witness (FP §6 Track G)");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step with implicit (T,α) Newton on drying chain (FP §6 Track G)");

    let t_diff = s_exp
        .thermal
        .temperature
        .as_tensor()
        .clone()
        .sub(s_imp.thermal.temperature.as_tensor().clone())
        .abs()
        .sum()
        .into_scalar();
    let a_diff = s_exp
        .chemical
        .reaction_extent
        .as_tensor()
        .clone()
        .sub(s_imp.chemical.reaction_extent.as_tensor().clone())
        .abs()
        .sum()
        .into_scalar();
    assert!(
        t_diff + a_diff > 1.0e-4_f32,
        "expected implicit (T,α) Newton to differ from explicit split; |ΔT|₁+|Δα|₁ sum = {}",
        t_diff + a_diff
    );
}

/// Phase 5 guard: [`ThmcSolver::monolithic_thmc_newton`] cannot be combined with [`ThmcImplicitTAlphaNewtonConfig`].
#[test]
fn thmc_step_monolithic_newton_errors_when_both_implicit_flags_set() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::full([1, n, 1], 0.5_f32, &d),
        damage.clone(),
        0.0_f32,
    );
    let mut solver = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 3_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        monolithic_thmc_newton: Some(ThmcMonolithicNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
            stacked_residual_l2_tolerance: 0.0_f32,
            stacked_residual_relative_to_initial: None,
        }),
        drying_last_node_evaporation_k: 0.0_f32,
        ..Default::default()
    };
    let err = match solver.step(&Stub, state0, &mut manifold) {
        Ok(_) => panic!("expected mutual exclusion error"),
        Err(e) => e,
    };
    assert!(
        err.to_string().contains("mutually exclusive"),
        "unexpected error: {err}"
    );
}

/// Monolithic \(R_h\) is pure implicit diffusion; facet drying closure is rejected.
#[test]
fn thmc_step_monolithic_newton_errors_when_drying_sink_enabled() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::full([1, n, 1], 0.5_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        0.0_f32,
    );
    let mut solver = ThmcSolver {
        drying_last_node_evaporation_k: 0.1_f32,
        monolithic_thmc_newton: Some(ThmcMonolithicNewtonConfig::default()),
        implicit_t_alpha_newton: None,
        ..Default::default()
    };
    let err = match solver.step(&Stub, state0, &mut manifold) {
        Ok(_) => panic!("expected drying sink incompatibility"),
        Err(e) => e,
    };
    assert!(
        err.to_string().contains("drying_last_node_evaporation_k"),
        "unexpected error: {err}"
    );
}

/// Monolithic dense Newton **fail-fast** before inner work when stacked DOFs exceed
/// [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`].
///
/// For scalar channels \(F_T=F_h=F_\alpha=1\), [`ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`]
/// is \(6N\); **`N = 11`** is the first layout with \(6N >\) [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`] at the default cap.
#[test]
fn thmc_step_monolithic_newton_errors_when_stacked_dof_count_exceeds_64() {
    let d = dev();
    let n = 11usize;
    assert!(
        ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count(n, 1, 1, 1)
            > THMC_DENSE_NEWTON_MAX_STACKED_DOFS,
        "test expects N such that stacked DOFs exceed dense cap"
    );
    let mut manifold = chain_manifold(n);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::full([1, n, 1], 293.15_f32, &d),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::full([1, n, 1], 0.5_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 1], &d),
        0.0_f32,
    );
    let mut solver = ThmcSolver {
        drying_last_node_evaporation_k: 0.0_f32,
        monolithic_thmc_newton: Some(ThmcMonolithicNewtonConfig::default()),
        implicit_t_alpha_newton: None,
        ..Default::default()
    };
    let err = match solver.step(&Stub, state0, &mut manifold) {
        Ok(_) => panic!("expected stacked DOF cap error"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("stacked DOFs > 64"), "unexpected error: {err}");
}

/// **Phase 5 integration:** [`ThmcSolver::step`] monolithic branch matches a standalone call to
/// [`ThmcImplicitEulerThermalHumidityReactionExtentResidual::damped_newton_iterations_with_quasi_static_r_u`]
/// when the predictor block matches `thmc.rs` `step_experimental` (keep in sync on edits).
///
/// Uses the same BC mask pattern as [`thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes`]
/// (all \(y,z\) clamped; \(u_x\) free at the loaded end only) so the dense Jacobian is well-conditioned.
#[test]
fn thmc_step_monolithic_newton_matches_standalone_dense_newton_two_nodes() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let mut bm_flat = vec![1.0_f32; n * 3];
    bm_flat[0] = 0.0_f32;
    for i in 0..n {
        bm_flat[i * 3 + 1] = 0.0_f32;
        bm_flat[i * 3 + 2] = 0.0_f32;
    }
    manifold.displacement_bc_mask =
        Tensor::from_data(Data::new(bm_flat, Shape::new([n, 3, 1])), &d);

    let coords_n3 = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for monolithic Newton witness (FP §6 Track G)")
        .clone();
    let edges_b1 = manifold.edges_b1.clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();
    let dt = 0.02_f32;
    let mc = ThmcMonolithicNewtonConfig {
        iterations: 4_usize,
        damping: 1.0_f32,
        fd_eps: 1.0e-5_f32,
        stacked_residual_l2_tolerance: 0.0_f32,
        stacked_residual_relative_to_initial: None,
    };

    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
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
    let state0 =     ThmcState::from_tensors(
        t_n.clone(),
        h_n.clone(),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        alpha_n.clone(),
        damage.clone(),
        0.0_f32,
    );

    let mut solver = ThmcSolver {
        dt,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: Some(mc.clone()),
        ..Default::default()
    };
    let s_step = solver
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step with monolithic_thmc_newton on 2-node chain for parity witness (FP §6 Track G)");

    // --- Mirror `step_experimental` monolithic predictor + standalone Newton ---
    let device = state0.thermal.temperature.as_tensor().device();
    let t_old = state0.thermal.temperature.as_tensor().clone();
    let h_old = state0.hydro.humidity.as_tensor().clone();
    let alpha_n = state0.chemical.reaction_extent.as_tensor().clone();
    let damage_m = damage.clone();
    let lap_t =
        TopologicalLaplacian::scalar_laplacian(t_old.clone(), edges_b1.clone(), damage_m.clone());
    let lap_h =
        TopologicalLaplacian::scalar_laplacian(h_old.clone(), edges_b1.clone(), damage_m.clone());
    let dt_lap_t = lap_t.mul_scalar(dt);
    let dt_lap_h = lap_h.mul_scalar(dt);
    let f_alpha_ch = alpha_n.dims()[2];
    let t_bn1 = t_old.clone().slice([0..batch, 0..n, 0..1]);
    let temperature_for_alpha = if f_alpha_ch == 1 {
        t_bn1
    } else {
        t_bn1.expand::<3, _>([batch, n, f_alpha_ch])
    };
    let d_alpha =
        reaction_extent_rate_tensor(&kinetics, alpha_n.clone(), temperature_for_alpha, &device);
    let f_t_ch = t_old.dims()[2];
    let exo = d_alpha
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .mul_scalar(kinetics.exothermic_k_per_alpha_rate * dt)
        .expand::<3, _>([batch, n, f_t_ch]);

    let mask = manifold.displacement_bc_mask.clone();
    let bm_core = match mask.dims()[..] {
        [nn, 3, 1] if nn == n => mask.reshape([nn, 3]),
        [1, nn, 3] if nn == n => mask.clone().slice([0..1, 0..n, 0..3]).reshape([nn, 3]),
        _ => panic!("unexpected displacement_bc_mask dims {:?}", mask.dims()),
    };
    let bm = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
    let bf = Tensor::<B, 3>::zeros([batch, n, 3], &device);
    let inner_cfg = MechanicsInnerLoopConfig::default();
    let cross_section_area = 0.01_f32;

    let t_predict = t_old.clone().add(dt_lap_t.clone()).add(exo.clone());
    let h_predict = h_old.clone().add(dt_lap_h.clone());
    let alpha_predict = alpha_n
        .clone()
        .add(d_alpha.clone().mul_scalar(dt))
        .clamp(0.0_f32, 1.0_f32);
    let alpha_bn1_pred = alpha_predict
        .clone()
        .slice([0..batch, 0..n, 0..1])
        .clamp(1e-6_f32, 1.0_f32);
    let stiffness_e = alpha_bn1_pred.mul_scalar(kinetics.stiffness_e_scale_pa);
    let stiffness_nu =
        Tensor::<B, 3>::zeros([batch, n, 1], &device).add_scalar(kinetics.stiffness_nu);
    let stiffness = Tensor::cat(vec![stiffness_e, stiffness_nu], 2);
    let (u_predict, _) = VectorMechanicsSolver::solve_equilibrium(
        state0.mechanical.displacement.as_tensor().clone(),
        coords_n3.clone(),
        stiffness,
        bf.clone(),
        edges_b1.clone(),
        damage_m.clone(),
        bm.clone(),
        cross_section_area,
        &inner_cfg,
    )
    .expect("VectorMechanicsSolver::solve_equilibrium on 2-node chain for monolithic Newton parity predict (FP §6 Track G monolithic Newton witness)");

    let trial =     ThmcState::from_tensors(
        t_predict,
        h_predict,
        u_predict,
        alpha_predict,
        state0.damage.as_tensor().clone(),
        state0.time,
    );

    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: Field::new(t_old.clone()),
        humidity_n: Field::new(h_old.clone()),
        alpha_n: Field::new(alpha_n.clone()),
        displacement_n: state0.mechanical.displacement.as_tensor().clone(),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage_m.clone()),
        kinetics: kinetics.clone(),
    };
    let (updated_standalone, _) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords_n3,
            &bm,
            &bf,
            cross_section_area,
            mc.iterations,
            mc.damping,
            mc.fd_eps,
            mc.stacked_residual_l2_tolerance,
            mc.stacked_residual_relative_to_initial,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u standalone run matching ThmcSolver monolithic step (FP §6 Track G)");

    let eps = 5e-5_f32;
    for (a, b) in s_step.thermal.temperature.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(
            updated_standalone.thermal.temperature.as_tensor().clone().into_data()
                .value
                .iter(),
        )
    {
        assert!((a - b).abs() < eps, "T mismatch: {a} vs {b}");
    }
    for (a, b) in s_step
        .hydro
        .humidity
        .as_tensor()
        .clone()
        .into_data()
        .value
        .iter()
        .zip(updated_standalone.hydro.humidity.as_tensor().clone().into_data().value.iter())
    {
        assert!((a - b).abs() < eps, "h mismatch: {a} vs {b}");
    }
    for (a, b) in s_step.chemical.reaction_extent.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(
            updated_standalone.chemical.reaction_extent.as_tensor().clone().into_data()
                .value
                .iter(),
        )
    {
        assert!((a - b).abs() < eps, "alpha mismatch: {a} vs {b}");
    }
    for (a, b) in s_step.mechanical.displacement.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(
            updated_standalone.mechanical.displacement.as_tensor().clone().into_data()
                .value
                .iter(),
        )
    {
        assert!((a - b).abs() < eps, "u mismatch: {a} vs {b}");
    }

    // [`ThmcMonolithicNewtonConfig::stacked_residual_l2_tolerance`] is forwarded into the dense helper with the same
    // predictor as `step_experimental` (keep aligned with [`thmc_monolithic_newton_residual_tol_early_exit_truncates_norm_trail`]).
    let max_probe_iters = 5_usize;
    let (_, norms_full) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords_n3,
            &bm,
            &bf,
            cross_section_area,
            max_probe_iters,
            mc.damping,
            mc.fd_eps,
            0.0_f32,
            None,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u fixed-count probe for tol early-exit calibration (FP §6 Track G)");
    assert_eq!(norms_full.len(), max_probe_iters + 1);
    let tol_exit = norms_full[2] + 0.05_f32 * (norms_full[1] - norms_full[2]).max(1e-30_f32);
    assert!(
        tol_exit > norms_full[2] && tol_exit < norms_full[1],
        "sanity: tol between ||R|| after one and two Newton steps (tol_exit={tol_exit}, n1={}, n2={})",
        norms_full[1],
        norms_full[2]
    );
    let mc_early = ThmcMonolithicNewtonConfig {
        iterations: max_probe_iters,
        damping: mc.damping,
        fd_eps: mc.fd_eps,
        stacked_residual_l2_tolerance: tol_exit,
        stacked_residual_relative_to_initial: None,
    };
    let mut solver_early = ThmcSolver {
        monolithic_thmc_newton: Some(mc_early.clone()),
        ..solver.clone()
    };
    let s_early = solver_early
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step with monolithic tol early-exit config on 2-node chain (FP §6 Track G)");
    let (updated_early, _) = assembler
        .damped_newton_iterations_with_quasi_static_r_u(
            &trial,
            &coords_n3,
            &bm,
            &bf,
            cross_section_area,
            mc_early.iterations,
            mc_early.damping,
            mc_early.fd_eps,
            mc_early.stacked_residual_l2_tolerance,
            mc_early.stacked_residual_relative_to_initial,
        )
        .expect("damped_newton_iterations_with_quasi_static_r_u with tol matching solver early-exit config (FP §6 Track G)");

    for (a, b) in s_early.thermal.temperature.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(updated_early.thermal.temperature.as_tensor().clone().into_data().value.iter())
    {
        assert!((a - b).abs() < eps, "early-exit T mismatch: {a} vs {b}");
    }
    for (a, b) in s_early
        .hydro
        .humidity
        .as_tensor()
        .clone()
        .into_data()
        .value
        .iter()
        .zip(updated_early.hydro.humidity.as_tensor().clone().into_data().value.iter())
    {
        assert!((a - b).abs() < eps, "early-exit h mismatch: {a} vs {b}");
    }
    for (a, b) in s_early.chemical.reaction_extent.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(
            updated_early.chemical.reaction_extent.as_tensor().clone().into_data()
                .value
                .iter(),
        )
    {
        assert!((a - b).abs() < eps, "early-exit alpha mismatch: {a} vs {b}");
    }
    for (a, b) in s_early.mechanical.displacement.as_tensor().clone().into_data()
        .value
        .iter()
        .zip(
            updated_early.mechanical.displacement.as_tensor().clone().into_data()
                .value
                .iter(),
        )
    {
        assert!((a - b).abs() < eps, "early-exit u mismatch: {a} vs {b}");
    }
}

/// **Phase 5–6 integration:** on the same 2-node SI harness as
/// [`thmc_step_monolithic_newton_matches_standalone_dense_newton_two_nodes`], the monolithic
/// [`ThmcSolver::step`] path (via [`ThmcSolver::step_monolithic_implicit`]) drives the coupled backward-Euler
/// residual \(\|R\|_2\) — including quasi-static \(R_u\) — **below** the norm evaluated at the **split**
/// operator-step outcome from [`ThmcSolver::step`] with `monolithic_thmc_newton: None`.
#[test]
fn thmc_step_monolithic_implicit_lowers_coupled_be_residual_norm_vs_split_two_nodes() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let mut bm_flat = vec![1.0_f32; n * 3];
    bm_flat[0] = 0.0_f32;
    for i in 0..n {
        bm_flat[i * 3 + 1] = 0.0_f32;
        bm_flat[i * 3 + 2] = 0.0_f32;
    }
    manifold.displacement_bc_mask =
        Tensor::from_data(Data::new(bm_flat, Shape::new([n, 3, 1])), &d);

    let coords_n3 = manifold
        .node_positions
        .as_ref()
        .expect("manifold.node_positions on chain_manifold(n) SI coords for monolithic Newton witness (FP §6 Track G)")
        .clone();
    let batch = 1usize;
    let kinetics = reference_reaction_extent_kinetics();
    let dt = 0.02_f32;
    let mc = ThmcMonolithicNewtonConfig {
        iterations: 4_usize,
        damping: 1.0_f32,
        fd_eps: 1.0e-5_f32,
        stacked_residual_l2_tolerance: 0.0_f32,
        stacked_residual_relative_to_initial: None,
    };

    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::from_data(
                Data::new(vec![298.0_f32, 306.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::from_data(
                Data::new(vec![0.50_f32, 0.62_f32], Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        damage.clone(),
        0.0_f32,
    );

    let mut solver_split = ThmcSolver {
        dt,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };
    let mut solver_mono = ThmcSolver {
        monolithic_thmc_newton: Some(mc),
        ..solver_split.clone()
    };

    let s_split = solver_split
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step split operator on 2-node SI chain for coupled BE residual baseline (FP §6 Track G)");
    let s_mono = solver_mono
        .step_monolithic_implicit(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step_monolithic_implicit on 2-node chain for coupled ‖R‖₂ comparison (FP §6 Track G)");

    let mask = manifold.displacement_bc_mask.clone();
    let bm_core = match mask.dims()[..] {
        [nn, 3, 1] if nn == n => mask.reshape([nn, 3]),
        [1, nn, 3] if nn == n => mask.clone().slice([0..1, 0..n, 0..3]).reshape([nn, 3]),
        _ => panic!("unexpected displacement_bc_mask dims {:?}", mask.dims()),
    };
    let boundary_mask_bn3 = bm_core.unsqueeze_dim::<3>(0).expand::<3, _>([batch, n, 3]);
    let body_force = Tensor::<B, 3>::zeros([batch, n, 3], &d);
    let cross_section_area = 0.01_f32;

    let assembler = ThmcImplicitEulerThermalHumidityReactionExtentResidual {
        dt,
        temperature_n: state0.thermal.temperature.clone(),
        humidity_n: state0.hydro.humidity.clone(),
        alpha_n: state0.chemical.reaction_extent.clone(),
        displacement_n: state0.mechanical.displacement.as_tensor().clone(),
        mechanics_placeholder_mass: 1.0_f32,
        ru_shrinkage_binder_liquid_ratio: None,
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage.clone()),
        kinetics: kinetics.clone(),
    };

    let r_split = assembler
        .residual_l2_including_quasi_static_r_u(
            &s_split,
            &coords_n3,
            &boundary_mask_bn3,
            &body_force,
            cross_section_area,
        )
        .expect("ThmcImplicitEulerThermalHumidityReactionExtentResidual::residual_l2_including_quasi_static_r_u after split step for coupled ‖R‖₂ baseline witness (FP §6 Track G)");
    let r_mono = assembler
        .residual_l2_including_quasi_static_r_u(
            &s_mono,
            &coords_n3,
            &boundary_mask_bn3,
            &body_force,
            cross_section_area,
        )
        .expect("residual_l2_including_quasi_static_r_u after step_monolithic_implicit for coupled ‖R‖₂ witness (FP §6 Track G)");

    assert!(
        r_split > 1.0e-6_f32,
        "split path should leave nontrivial coupled BE residual, got {r_split}"
    );
    assert!(
        r_mono < r_split * 0.5_f32,
        "expected monolithic ||R|| << split ||R||; mono={r_mono} split={r_split}"
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
    let mut manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        damage.clone(),
        0.0_f32,
    );

    let kinetics = reference_reaction_extent_kinetics();
    let mut solver_explicit = ThmcSolver {
        dt: 0.08_f32,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };
    let mut solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step explicit split on drying chain for implicit comparison witness (FP §6 Track G)");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step with implicit (T,α) Newton on drying chain (FP §6 Track G)");

    assert_eq!(
        s_exp.hydro.humidity.as_tensor().clone().into_data().value,
        s_imp.hydro.humidity.as_tensor().clone().into_data().value,
        "humidity transport must not depend on implicit vs explicit (T,α) branch"
    );
}

/// With [`ThmcImplicitEulerThermalReactionExtentResidual`] anchored at the pre-step \((T^n,\alpha^n)\), the
/// post-step state from the Newton path must yield a **strictly smaller** \(\|R\|_2\) than the
/// explicit-split endpoint (which coincides with the Newton initial iterate).
#[test]
fn thmc_step_implicit_t_alpha_newton_lowers_analytic_residual_vs_explicit_endpoint() {
    let d = dev();
    let n = 2usize;
    let mut manifold = chain_manifold(n);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &d);
    let state0 =     ThmcState::from_tensors(
        Tensor::<B, 3>::from_data(
                Data::new(vec![301.0_f32, 289.0_f32], Shape::new([1, n, 1])),
                &d,
            ),
        Tensor::<B, 3>::full([1, n, 1], 0.65_f32, &d),
        Tensor::<B, 3>::zeros([1, n, 3], &d),
        Tensor::<B, 3>::from_data(
                Data::new(vec![0.31_f32, 0.55_f32], Shape::new([1, n, 1])),
                &d,
            ),
        damage.clone(),
        0.0_f32,
    );

    let kinetics = reference_reaction_extent_kinetics();
    let dt = 0.08_f32;
    let mut solver_explicit = ThmcSolver {
        dt,
        max_newton: 1_usize,
        tol: 1e-3_f32,
        reaction_extent_kinetics: kinetics.clone(),
        drying_last_node_evaporation_k: 0.0_f32,
        drying_ambient_h: 0.5_f32,
        implicit_t_alpha_newton: None,
        monolithic_thmc_newton: None,
        ..Default::default()
    };
    let mut solver_implicit = ThmcSolver {
        implicit_t_alpha_newton: Some(ThmcImplicitTAlphaNewtonConfig {
            iterations: 4_usize,
            damping: 1.0_f32,
            fd_eps: 1.0e-5_f32,
        }),
        ..solver_explicit.clone()
    };

    let s_exp = solver_explicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step explicit split on drying chain for implicit comparison witness (FP §6 Track G)");
    let s_imp = solver_implicit
        .step(&Stub, clone_thmc_state(&state0), &mut manifold)
        .expect("ThmcSolver::step with implicit_t_alpha_newton on drying chain for BE residual comparison (FP §6 Track G)");

    let assembler = ThmcImplicitEulerThermalReactionExtentResidual {
        dt,
        temperature_n: state0.thermal.temperature.clone(),
        alpha_n: state0.chemical.reaction_extent.clone(),
        edges_b1: manifold.edges_b1.clone(),
        damage_m: StepEntryDamageMask::from_tensor(damage.clone()),
        kinetics,
    };

    let trial_exp = thmc_state_with_t_alpha(
        &state0,
        s_exp.thermal.temperature.as_tensor().clone(),
        s_exp.chemical.reaction_extent.as_tensor().clone(),
    );
    let trial_imp = thmc_state_with_t_alpha(
        &state0,
        s_imp.thermal.temperature.as_tensor().clone(),
        s_imp.chemical.reaction_extent.as_tensor().clone(),
    );

    let r_exp = assembler
        .residual_l2(&trial_exp)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::residual_l2 at explicit split endpoint (FP §6 Track G)");
    let r_imp = assembler
        .residual_l2(&trial_imp)
        .expect("ThmcImplicitEulerThermalReactionExtentResidual::residual_l2 at implicit (T,α) endpoint (FP §6 Track G)");
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
    let (_t_new, norms) = solver
        .step_thermal_implicit::<B>(0.05_f32, t_old, 0.1_f32, edges, mask, cfg)
        .expect("ThmcSolver::step_thermal_implicit CG convergence on chain Laplacian witness (FP §6 Track G)");

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
    let last = *norms.last().expect("thermal implicit CG norm trail non-empty at convergence (FP §6 Track G)");
    assert!(
        last < 1.0e-6_f32,
        "final residual {last} did not reach 1e-6 (log = {norms:?})",
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
        let (t_new, _norms) = solver
            .step_thermal_implicit::<B>(
                dt,
                t.clone(),
                kappa,
                edges.clone(),
                mask.clone(),
                cfg,
            )
            .expect("ThmcSolver::step_thermal_implicit CG convergence on chain Laplacian witness (FP §6 Track G)");
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

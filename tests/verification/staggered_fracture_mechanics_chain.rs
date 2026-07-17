// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track 12 smoke: `update_damage_staggered` with strain from [`VectorMechanicsSolver::solve_equilibrium`].
//! See `docs/research/v0.4_track12_staggered_fracture_mechanics.md`.

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::solvers::PhaseFieldFractureSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

use umst_manifold::core::field::{DamageField, Field, SmallStrainField};

fn strain_field(t: Tensor<B, 4>) -> SmallStrainField<B> {
    SmallStrainField::from_tensor(t)
}

fn damage_field(t: Tensor<B, 3>) -> DamageField<B> {
    Field::new(t)
}


/// Voigt `[εxx,εyy,εzz,εxy,εyz,εxz]` (tensor shear) → symmetric `[B,N,3,3]`.
fn voigt6_to_sym_tensor3<Bk: Backend<FloatElem = f32>>(v: Tensor<Bk, 3>) -> Tensor<Bk, 4> {
    let b = v.dims()[0];
    let n = v.dims()[1];
    let exx = v.clone().slice([0..b, 0..n, 0..1]);
    let eyy = v.clone().slice([0..b, 0..n, 1..2]);
    let ezz = v.clone().slice([0..b, 0..n, 2..3]);
    let exy = v.clone().slice([0..b, 0..n, 3..4]);
    let eyz = v.clone().slice([0..b, 0..n, 4..5]);
    let exz = v.clone().slice([0..b, 0..n, 5..6]);
    let row0 = Tensor::cat(vec![exx.clone(), exy.clone(), exz.clone()], 2).unsqueeze_dim::<4>(2);
    let row1 = Tensor::cat(vec![exy.clone(), eyy.clone(), eyz.clone()], 2).unsqueeze_dim::<4>(2);
    let row2 = Tensor::cat(vec![exz, eyz, ezz], 2).unsqueeze_dim::<4>(2);
    Tensor::cat(vec![row0, row1, row2], 2)
}

#[test]
fn staggered_one_outer_mechanics_strain_drives_at2_damage() {
    let dev = NdArrayDevice::Cpu;
    let batch = 1usize;
    let n = 3usize;
    let e_ct = 2usize;

    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * 0.5);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

    let mut edges = Vec::with_capacity(e_ct * 2);
    for eid in 0..e_ct {
        edges.push(eid as i64);
    }
    for eid in 0..e_ct {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, e_ct])), &dev);

    let e_young_pa = 2.0e8_f32;
    let nu = 0.3_f32;
    let mut stiff = Vec::with_capacity(n * 2);
    for _ in 0..n {
        stiff.push(e_young_pa);
        stiff.push(nu);
    }
    let stiffness: Tensor<B, 3> =
        Tensor::from_data(Data::new(stiff, Shape::new([batch, n, 2])), &dev);

    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = 2000.0_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0;
        bm_data[i * 3 + 2] = 0.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

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

    let u0 = Tensor::<B, 3>::zeros([batch, n, 3], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![150.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let fracture = PhaseFieldFractureSolver { length_scale: 0.08 };

    let edges_for_damage = edges_b1.clone();
    let d_out = fracture.update_damage_staggered(
        |damage: &DamageField<B>| {
            let (u, _) = VectorMechanicsSolver::solve_equilibrium(
                u0.clone(),
                coords.clone(),
                stiffness.clone(),
                body_force.clone(),
                edges_b1.clone(),
                damage.as_tensor().clone(),
                boundary_mask.clone(),
                cross_section_area,
                &cfg,
            ).expect("VectorMechanicsSolver::solve_equilibrium on 3-node bar with damage field (FP §6 Track 12 stagger mechanics inner loop witness)");
            let u_src = u.clone().gather(1, src3.clone());
            let u_tgt = u.gather(1, tgt3.clone());
            let edge_disp = u_tgt.sub(u_src);
            let eps_v = VectorMechanicsSolver::voigt_strain_from_edge_displacement(
                edge_disp,
                edge_unit.clone(),
                edge_len.clone(),
                edges_b1.clone(),
                n,
            );
            strain_field(voigt6_to_sym_tensor3(eps_v))
        },
        damage_field(Tensor::<B, 3>::zeros([batch, n, 1], &dev)),
        fracture_energy_gc,
        edges_for_damage,
        1,
    ).expect("PhaseFieldFractureSolver::update_damage_staggered with mechanics-sourced strain on 3-node bar (FP §6 Track 12 smoke)");

    let vals = d_out.into_tensor().into_data().value;
    assert!(vals.iter().all(|x| x.is_finite()));
    let max_d = vals.iter().copied().fold(0.0_f32, f32::max);
    assert!(
        max_d > 1e-10_f32,
        "expected mechanics-sourced strain to drive damage; max_d={max_d}"
    );
}

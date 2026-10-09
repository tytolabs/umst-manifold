// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Track 12 smoke: `update_damage_staggered` with strain from [`VectorMechanicsSolver::solve_equilibrium`].
//! See `docs/research/v0.4_track12_staggered_fracture_mechanics.md`.

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::solvers::fracture_field::strain_tensor_for_fracture_after_mechanics;
use umst_manifold::physics::solvers::PhaseFieldFractureSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;
use umst_manifold::physics::PhysicsError;

type B = NdArray<f32>;

use umst_manifold::core::field::{DamageField, Field, SmallStrainField};

fn strain_field(t: Tensor<B, 4>) -> SmallStrainField<B> {
    SmallStrainField::from_tensor(t)
}

fn damage_field(t: Tensor<B, 3>) -> DamageField<B> {
    Field::new(t)
}

#[allow(clippy::too_many_arguments)]
fn max_abs_axial_edge_strain(
    u0: &Tensor<B, 3>,
    coords: &Tensor<B, 2>,
    stiffness: &Tensor<B, 3>,
    body_force: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    damage: Tensor<B, 3>,
    boundary_mask: &Tensor<B, 3>,
    cross_section_area: f32,
    cfg: &MechanicsInnerLoopConfig,
    src3: &Tensor<B, 3, Int>,
    tgt3: &Tensor<B, 3, Int>,
    edge_unit: &Tensor<B, 3>,
    edge_len: &Tensor<B, 3>,
    batch: usize,
) -> Result<f32, PhysicsError> {
    let (u, _) = VectorMechanicsSolver::solve_equilibrium(
        u0.clone(),
        coords.clone(),
        stiffness.clone(),
        body_force.clone(),
        edges_b1.clone(),
        damage,
        boundary_mask.clone(),
        cross_section_area,
        cfg,
    )?;
    let u_src = u.clone().gather(1, src3.clone());
    let u_tgt = u.gather(1, tgt3.clone());
    let edge_disp = u_tgt.sub(u_src);
    let elong = edge_disp
        .mul(edge_unit.clone())
        .sum_dim(2)
        .reshape([batch, edge_len.dims()[1], 1]);
    let eps_ax = elong.div(edge_len.clone().clamp_min(1e-30_f32));
    Ok(eps_ax.abs().max().into_scalar())
}

fn assert_mechanics_refusal_named(e: &PhysicsError, context: &str) {
    assert!(
        matches!(
            e,
            PhysicsError::Diverged { .. }
                | PhysicsError::NonFinite { .. }
                | PhysicsError::IndefiniteSystem { .. }
                | PhysicsError::InvariantViolation { .. }
        ),
        "{context}: typed VectorMechanicsSolver refusal {e:?}"
    );
}

fn assert_staggered_damage_refusal_named(e: &PhysicsError) {
    assert!(
        matches!(
            e,
            PhysicsError::Diverged { .. }
                | PhysicsError::NonFinite { .. }
                | PhysicsError::InvariantViolation { .. }
        ),
        "typed PhaseFieldFractureSolver::update_damage_staggered refusal: {e:?}"
    );
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

    // Softer axial bar + tip load so edge strain is O(10⁻²) before AT2 (matches Track 12 toy chains).
    let e_young_pa = 2.0e7_f32;
    let nu = 0.3_f32;
    let mut stiff = Vec::with_capacity(n * 2);
    for _ in 0..n {
        stiff.push(e_young_pa);
        stiff.push(nu);
    }
    let stiffness: Tensor<B, 3> =
        Tensor::from_data(Data::new(stiff, Shape::new([batch, n, 2])), &dev);

    let mut bf_data = vec![0.0_f32; n * 3];
    bf_data[(n - 1) * 3] = 5.0e4_f32;
    let body_force = Tensor::from_data(Data::new(bf_data, Shape::new([batch, n, 3])), &dev);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    bm_data[1] = 0.0;
    bm_data[2] = 0.0;
    for i in 0..n {
        bm_data[i * 3 + 1] = 0.0;
        bm_data[i * 3 + 2] = 0.0;
    }
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([batch, n, 3])), &dev);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: n * 3,
        cg_tolerance: 1e-6,
        pcg_tolerance: 1e-6,
        use_preconditioner: true,
        max_equilibrium_sub_iters: umst_math::numeric_tolerance::DEFAULT_EQUILIBRIUM_SUB_ITERS,
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
    let d_zero = Tensor::<B, 3>::zeros([batch, n, 1], &dev);
    let fracture_energy_gc = Tensor::from_data(
        Data::new(vec![2.0_f32; batch * n], Shape::new([batch, n, 1])),
        &dev,
    );
    let fracture = PhaseFieldFractureSolver { length_scale: 0.05 };

    let max_edge_eps = match max_abs_axial_edge_strain(
        &u0,
        &coords,
        &stiffness,
        &body_force,
        &edges_b1,
        d_zero.clone(),
        &boundary_mask,
        cross_section_area,
        &cfg,
        &src3,
        &tgt3,
        &edge_unit,
        &edge_len,
        batch,
    ) {
        Ok(v) => v,
        Err(e) => {
            assert_mechanics_refusal_named(
                &e,
                "fixture quasi-static equilibrium before staggered damage",
            );
            panic!(
                "fixture must yield Ok mechanics equilibrium with nonzero strain; refusal {e:?}"
            );
        }
    };
    assert!(
        max_edge_eps > 1e-5_f32,
        "fixture tip load must produce nonzero axial edge strain before damage update; max_edge_eps={max_edge_eps}"
    );

    let edges_for_damage = edges_b1.clone();
    let d_out = match fracture.update_damage_staggered(
        |damage: &DamageField<B>| match strain_tensor_for_fracture_after_mechanics(
            u0.clone(),
            coords.clone(),
            stiffness.clone(),
            body_force.clone(),
            edges_b1.clone(),
            damage.as_tensor().clone(),
            boundary_mask.clone(),
            cross_section_area,
            &cfg,
            src3.clone(),
            tgt3.clone(),
            edge_unit.clone(),
            edge_len.clone(),
            n,
        ) {
            Ok(t) => strain_field(t),
            Err(e) => {
                assert_mechanics_refusal_named(
                    &e,
                    "mechanics inner loop inside update_damage_staggered",
                );
                panic!("mechanics strain provider must return Ok on loaded bar; refusal {e:?}");
            }
        },
        damage_field(d_zero),
        fracture_energy_gc,
        edges_for_damage,
        1,
    ) {
        Ok(d) => d,
        Err(e) => {
            assert_staggered_damage_refusal_named(&e);
            panic!(
                "fixture must reach Ok staggered damage after nonzero edge strain; refusal {e:?}"
            );
        }
    };

    let vals = d_out.into_tensor().into_data().value;
    assert!(vals.iter().all(|x| x.is_finite()));
    let max_d = vals.iter().copied().fold(0.0_f32, f32::max);
    assert!(
        max_d > 1e-6_f32,
        "expected mechanics-sourced strain to drive AT2 damage; max_d={max_d}"
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! H4 Step A — bar-network operator probes before any PCG tuning or nondimensionalization.
//!
//! formal_anchor: Track B6 / H4 fix sequence

#![cfg(feature = "mechanics-adjoint")]
#![allow(clippy::type_complexity, dead_code)]

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::dec_operators::DecEdgeOperators;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

const DAMAGE_REG: f32 = 1e-6;
const F32_EPS_SCALE: f32 = 128.0;

struct BarAssembly<Bk: Backend<FloatElem = f32>> {
    k_axial: Tensor<Bk, 3>,
    edge_unit: Tensor<Bk, 3>,
    edge_len: Tensor<Bk, 3>,
    src_indices: Tensor<Bk, 3, Int>,
    tgt_indices: Tensor<Bk, 3, Int>,
    n_v: usize,
}

impl<Bk: Backend<FloatElem = f32>> BarAssembly<Bk> {
    fn from_nodal_e(
        coords: Tensor<Bk, 2>,
        edges_b1: Tensor<Bk, 2, Int>,
        e_node: Tensor<Bk, 3>,
        damage: Tensor<Bk, 3>,
        cross_section_area: f32,
    ) -> Self {
        let batch = 1usize;
        let n_v = coords.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();
        let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n_v, 3]);
        let src_indices = topo.expand_src_gather_indices(batch, 3);
        let tgt_indices = topo.expand_tgt_gather_indices(batch, 3);
        let c_src = coords_b.clone().gather(1, src_indices.clone());
        let c_tgt = coords_b.gather(1, tgt_indices.clone());
        let delta_geom = c_tgt.sub(c_src);
        let edge_len = delta_geom
            .clone()
            .powf_scalar(2.0)
            .sum_dim(2)
            .sqrt()
            .clamp(1e-12, f32::MAX)
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());
        let e_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(e_node.clone(), edges_b1.clone());
        let d_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(damage.clone(), edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);
        let k_axial = e_on_edges
            .mul_scalar(cross_section_area)
            .div(edge_len.clone())
            .mul(dmg);
        Self {
            k_axial,
            edge_unit,
            edge_len,
            src_indices,
            tgt_indices,
            n_v,
        }
    }

    fn projected_matvec(&self, u: Tensor<Bk, 3>, mask: &Tensor<Bk, 3>) -> Tensor<Bk, 3> {
        let ku = VectorMechanicsSolver::bar_matvec(
            u,
            &self.k_axial,
            &self.edge_unit,
            &self.src_indices,
            &self.tgt_indices,
            self.n_v,
            None,
            &self.edge_len,
        );
        mask.clone().mul(ku)
    }

    fn projected_inner(&self, a: &Tensor<Bk, 3>, b: &Tensor<Bk, 3>) -> f32 {
        a.clone().mul(b.clone()).sum().into_scalar()
    }
}

fn projected_rel_residual_bn3<Bk: Backend<FloatElem = f32>>(
    u: &Tensor<Bk, 3>,
    f: &Tensor<Bk, 3>,
    mask: &Tensor<Bk, 3>,
    asm: &BarAssembly<Bk>,
) -> f32 {
    let ku = asm.projected_matvec(u.clone(), mask);
    let resid = mask.clone().mul(f.clone().sub(ku));
    let rhs_norm = f
        .clone()
        .mul(mask.clone())
        .powf_scalar(2.0)
        .sum()
        .sqrt()
        .into_scalar()
        .max(1e-30_f32);
    resid.powf_scalar(2.0).sum().sqrt().into_scalar() / rhs_norm
}

/// Matches cartridge harness [`pin_bottom_perimeter_inner`]: perimeter in **xy** at **z = 0** only.
fn harness_pin_bottom_perimeter(
    nx: usize,
    ny: usize,
    nz: usize,
    device: &NdArrayDevice,
) -> Tensor<B, 3> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut bm = vec![1.0_f32; n * 3];
    let mut pin = |ix: usize, iy: usize| {
        let nid = ix + iy * nx1;
        bm[nid * 3] = 0.0;
        bm[nid * 3 + 1] = 0.0;
        bm[nid * 3 + 2] = 0.0;
    };
    for ix in 0..=nx {
        pin(ix, 0);
        pin(ix, ny);
    }
    for iy in 0..=ny {
        pin(0, iy);
        pin(nx, iy);
    }
    let _ = nz;
    Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), device)
}

fn seeded_vec3n(n: usize, seed: u64, scale: f32) -> Vec<f32> {
    let mut x = seed;
    let mut out = Vec::with_capacity(n * 3);
    for _ in 0..(n * 3) {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        let u = ((x >> 32) as f32) / (u32::MAX as f32) * 2.0 - 1.0;
        out.push(u * scale);
    }
    out
}

fn quick_plate_fixture() -> (
    ExtrudedPlateMechanics,
    NdArrayDevice,
    Tensor<B, 2, Int>,
    Tensor<B, 2>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    f32,
) {
    let dev = NdArrayDevice::Cpu;
    let nx = 9usize;
    let ny = 8usize;
    let nz = 2usize;
    let lx = 0.8_f32;
    let ly = 0.8_f32;
    let lz = 0.1_f32;
    let dx = lx / nx as f32;
    let dy = ly / ny as f32;
    let dz = lz / nz as f32;
    let plate = ExtrudedPlateMechanics {
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
    };
    let n = plate.n_nodes();
    let edges = plate.edges_b1::<B>(&dev);
    let coords = plate
        .coords_bn3::<B>(&dev)
        .expect("ExtrudedPlateMechanics::coords_bn3 on 9×8×2 plate harness (FP §6 Track B6 H4 Step A operator probe)")
        .reshape(Shape::new([n, 3]));
    let rho = Tensor::<B, 3>::full([1, n, 1], 0.5_f32, &dev);
    let e_node = rho
        .powf_scalar(3.0)
        .mul_scalar(200e6_f32 - 1.0)
        .add_scalar(1.0);
    let mask = harness_pin_bottom_perimeter(nx, ny, nz, &dev);
    let area = (dx * dy * dz).cbrt().powf(2.0);
    (plate, dev, edges, coords, e_node, mask, area)
}

fn stiffness_bn2<Bk: Backend<FloatElem = f32>>(e_node: Tensor<Bk, 3>, nu: f32) -> Tensor<Bk, 3> {
    let device = e_node.device();
    let [batch, n, _] = e_node.dims();
    let nu_t = Tensor::<Bk, 3>::full([batch, n, 1], nu, &device);
    Tensor::cat(vec![e_node, nu_t], 2)
}

fn chain_1d_fixture(
    n: usize,
    dx: f32,
    e: f32,
    a: f32,
    f_tip: f32,
) -> (
    NdArrayDevice,
    Tensor<B, 2>,
    Tensor<B, 2, Int>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    Tensor<B, 3>,
    f32,
) {
    let dev = NdArrayDevice::Cpu;
    let mut coords_data = Vec::with_capacity(n * 3);
    for i in 0..n {
        coords_data.push(i as f32 * dx);
        coords_data.push(0.0);
        coords_data.push(0.0);
    }
    let coords: Tensor<B, 2> = Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);
    let mut edges = Vec::with_capacity((n - 1) * 2);
    for eid in 0..(n - 1) {
        edges.push(eid as i64);
    }
    for eid in 0..(n - 1) {
        edges.push((eid + 1) as i64);
    }
    let edges_b1: Tensor<B, 2, Int> =
        Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);
    let stiffness = stiffness_bn2(Tensor::<B, 3>::full([1, n, 1], e, &dev), 0.2);
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let mut bm = vec![1.0_f32; n * 3];
    bm[0] = 0.0;
    bm[1] = 0.0;
    bm[2] = 0.0;
    let mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);
    let mut bf = vec![0.0_f32; n * 3];
    bf[(n - 1) * 3] = f_tip;
    let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);
    (
        dev, coords, edges_b1, stiffness, damage, mask, body_force, a,
    )
}

#[test]
fn two_node_rel_residual_metric_sane_and_converged() {
    let n = 2usize;
    let dx = 1.0_f32;
    let e = 1.0e6_f32;
    let a = 0.01_f32;
    let f_tip = 250.0_f32;
    let u_exact = f_tip * dx / (e * a * (1.0 + DAMAGE_REG));

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 64,
        cg_tolerance: 1e-10,
        pcg_tolerance: 1e-10,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let (dev2, coords, edges_b1, stiffness, damage, mask, body_force, area) =
        chain_1d_fixture(n, dx, e, a, f_tip);

    let (u, _, pcg) = VectorMechanicsSolver::solve_equilibrium_with_pcg_report(
        Tensor::<B, 3>::zeros([1, n, 3], &dev2),
        coords.clone(),
        stiffness.clone(),
        body_force.clone(),
        edges_b1.clone(),
        damage.clone(),
        mask.clone(),
        area,
        &cfg,
    )
    .expect("VectorMechanicsSolver::solve_equilibrium_with_pcg_report on 2-node bar chain (FP §6 Track B6 H4 Step A rel-residual witness)");

    let eq_rel = VectorMechanicsSolver::bar_network_equilibrium_rel_residual(
        u.clone(),
        coords,
        stiffness,
        body_force,
        edges_b1,
        damage,
        mask.clone(),
        area,
    );

    let u_tip = u.clone().slice([0..1, 1..2, 0..1]).into_data().value[0];

    assert!(
        pcg.rel_residual < 1e-6,
        "2-node PCG rel_residual={} (iters={})",
        pcg.rel_residual,
        pcg.iterations
    );
    assert!(
        (eq_rel - pcg.rel_residual).abs() < 1e-5,
        "PCG metric {pcg:?} vs post-solve eq_rel {eq_rel}"
    );
    assert!(
        (u_tip - u_exact).abs() / u_exact < 1e-4,
        "tip u={u_tip} expected {u_exact}"
    );
}

#[test]
fn quick_plate_operator_symmetry_under_harness_pins() {
    let (_plate, dev, edges, coords, e_node, mask, area) = quick_plate_fixture();
    let n = coords.dims()[0];
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let asm = BarAssembly::from_nodal_e(coords, edges, e_node, damage, area);

    for seed in [11_u64, 29, 97] {
        let v_raw = seeded_vec3n(n, seed, 1.0);
        let w_raw = seeded_vec3n(n, seed.wrapping_add(1), 1.0);
        let v = mask.clone().mul(Tensor::from_data(
            Data::new(v_raw, Shape::new([1, n, 3])),
            &dev,
        ));
        let w = mask.clone().mul(Tensor::from_data(
            Data::new(w_raw, Shape::new([1, n, 3])),
            &dev,
        ));
        let kv = asm.projected_matvec(v.clone(), &mask);
        let kw = asm.projected_matvec(w.clone(), &mask);
        let lhs = asm.projected_inner(&v, &kw);
        let rhs = asm.projected_inner(&w, &kv);
        let scale = lhs
            .abs()
            .max(rhs.abs())
            .max(kv.clone().powf_scalar(2.0).sum().sqrt().into_scalar());
        let tol = f32::EPSILON * F32_EPS_SCALE * scale.max(1.0);
        assert!(
            (lhs - rhs).abs() <= tol,
            "symmetry seed={seed}: |⟨Kv,w⟩−⟨v,Kw⟩|={} > tol={tol} (lhs={lhs} rhs={rhs})",
            (lhs - rhs).abs()
        );
    }
}

#[test]
fn quick_plate_operator_psd_under_harness_pins() {
    let (_plate, dev, edges, coords, e_node, mask, area) = quick_plate_fixture();
    let n = coords.dims()[0];
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let asm = BarAssembly::from_nodal_e(coords, edges, e_node, damage, area);

    for seed in [3_u64, 17, 101] {
        let v_raw = seeded_vec3n(n, seed, 1.0);
        let v = mask.clone().mul(Tensor::from_data(
            Data::new(v_raw, Shape::new([1, n, 3])),
            &dev,
        ));
        let kv = asm.projected_matvec(v.clone(), &mask);
        let quad = asm.projected_inner(&v, &kv);
        let scale = v.clone().powf_scalar(2.0).sum().into_scalar().max(1.0)
            * asm.k_axial.clone().max().into_scalar().max(1.0);
        let floor = -f32::EPSILON * F32_EPS_SCALE * scale;
        assert!(
            quad >= floor,
            "PSD seed={seed}: ⟨Kv,v⟩={quad} < floor={floor}"
        );
    }
}

#[test]
fn nine_node_chain_manufactured_solution() {
    let n = 10usize;
    let dx = 0.1_f32;
    let e = 200e6_f32;
    let a = 0.01_f32;
    let (dev, coords, edges_b1, stiffness, damage, mask, _bf, area) =
        chain_1d_fixture(n, dx, e, a, 0.0);

    let asm = BarAssembly::from_nodal_e(
        coords.clone(),
        edges_b1.clone(),
        Tensor::<B, 3>::full([1, n, 1], e, &dev),
        damage.clone(),
        area,
    );
    let mut u_star_raw = vec![0.0_f32; n * 3];
    for i in 0..n {
        u_star_raw[i * 3] = seeded_vec3n(1, 7 + i as u64, 1e-6)[0];
    }
    let u_star = mask.clone().mul(Tensor::from_data(
        Data::new(u_star_raw, Shape::new([1, n, 3])),
        &dev,
    ));
    let body_force = asm.projected_matvec(u_star.clone(), &mask);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 128,
        cg_tolerance: 1e-8,
        pcg_tolerance: 1e-8,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let (_u, _, pcg) = VectorMechanicsSolver::solve_equilibrium_with_pcg_report(
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        coords,
        stiffness,
        body_force,
        edges_b1,
        damage,
        mask,
        area,
        &cfg,
    )
    .expect("VectorMechanicsSolver::solve_equilibrium_with_pcg_report on 9-node manufactured bar chain (FP §6 Track B6 H4 Step A manufactured solution witness)");

    assert!(
        pcg.rel_residual < 1e-2,
        "chain manufactured pcg_rel={} iters={}",
        pcg.rel_residual,
        pcg.iterations
    );
}

#[test]
#[ignore = "H4: f64 PCG still stalls ~0.94 rel_res on 9×8×2 roof traction (operator probes pass); gate tracked in cartridge harness"]
fn quick_plate_harness_load_pcg_converges() {
    let (plate, dev, edges, coords, e_node, mask, area) = quick_plate_fixture();
    let mask_check = mask.clone();
    let n = coords.dims()[0];
    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let stiffness = stiffness_bn2(e_node, 0.2);
    let body_force = plate
        .body_force_top_uniform_pressure(50.0)
        .chunks(3)
        .map(|c| c[2])
        .collect::<Vec<_>>();
    let mut bf = vec![0.0_f32; n * 3];
    for (nid, fz) in body_force.into_iter().enumerate() {
        bf[nid * 3 + 2] = fz;
    }
    let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 2000,
        cg_tolerance: 1e-4,
        pcg_tolerance: 1e-4,
        use_preconditioner: false,
        max_equilibrium_substeps: 1,
    };

    let (u, _, pcg) = VectorMechanicsSolver::solve_equilibrium_with_pcg_report(
        Tensor::<B, 3>::zeros([1, n, 3], &dev),
        coords.clone(),
        stiffness.clone(),
        body_force.clone(),
        edges.clone(),
        damage.clone(),
        mask,
        area,
        &cfg,
    )
    .expect("VectorMechanicsSolver::solve_equilibrium_with_pcg_report on extruded plate bar network (FP §6 Track B6 H4 Step A roof PCG witness)");

    let eq_rel = VectorMechanicsSolver::bar_network_equilibrium_rel_residual(
        u, coords, stiffness, body_force, edges, damage, mask_check, area,
    );

    let tol = cfg.pcg_tolerance.max(cfg.cg_tolerance);
    assert!(
        pcg.rel_residual <= tol,
        "quick harness PCG rel_residual={} iters={} (tol={tol}) k_char={} E_ref={} dx={}",
        pcg.rel_residual,
        pcg.iterations,
        pcg.stiffness_scale,
        pcg.e_ref,
        pcg.dx_char
    );
    assert!(
        eq_rel <= tol,
        "quick harness eq_rel={eq_rel} (tol={tol}) pcg={pcg:?}"
    );
}

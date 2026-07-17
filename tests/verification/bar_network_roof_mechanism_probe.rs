// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! H4 roof-traction stall — mechanism probes (9×8×2 harness fixture).
//!
//! formal_anchor: Track B6 / b6-roof-mechanism-research

#![cfg(feature = "mechanics-adjoint")]
#![allow(
    dead_code,
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::type_complexity
)]

use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::dec_operators::DecEdgeOperators;
use umst_manifold::physics::error::PhysicsError;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

const DAMAGE_REG: f32 = 1e-6;

struct HarnessFixture {
    plate: ExtrudedPlateMechanics,
    dev: NdArrayDevice,
    n: usize,
    nx: usize,
    ny: usize,
    nz: usize,
    edges: Tensor<B, 2, Int>,
    coords: Tensor<B, 2>,
    e_node: Tensor<B, 3>,
    mask: Tensor<B, 3>,
    mask_flat: Vec<f64>,
    f_roof: Tensor<B, 3>,
    f_roof_flat: Vec<f64>,
    area: f32,
    stiffness: Tensor<B, 3>,
    damage: Tensor<B, 3>,
    k_axial: Tensor<B, 3>,
    edge_unit: Tensor<B, 3>,
    edge_len: Tensor<B, 3>,
    src: Vec<usize>,
    tgt: Vec<usize>,
    src_indices: Tensor<B, 3, Int>,
    tgt_indices: Tensor<B, 3, Int>,
    ndof: usize,
    k64: Vec<f64>,
    eu64: Vec<f64>,
}

impl HarnessFixture {
    fn quick() -> Self {
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
        let ndof = n * 3;
        let edges = plate.edges_b1::<B>(&dev);
        let coords = plate
            .coords_bn3::<B>(&dev)
            .expect("ExtrudedPlateMechanics::coords_bn3 on 9×8×2 roof harness fixture (FP §6 Track B6 H4 mechanism probe)")
            .reshape(Shape::new([n, 3]));
        let rho = Tensor::<B, 3>::full([1, n, 1], 0.5_f32, &dev);
        let e_node = rho
            .powf_scalar(3.0)
            .mul_scalar(200e6_f32 - 1.0)
            .add_scalar(1.0);
        let mask = harness_pin_bottom_perimeter(nx, ny, nz, &dev);
        let mask_flat: Vec<f64> = mask
            .clone()
            .into_data()
            .value
            .iter()
            .map(|&x| x as f64)
            .collect();
        let bf = plate.body_force_top_uniform_pressure(50.0);
        let f_roof = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);
        let f_roof_flat: Vec<f64> = f_roof
            .clone()
            .into_data()
            .value
            .iter()
            .map(|&x| x as f64)
            .collect();
        let area = (dx * dy * dz).cbrt().powf(2.0);
        let stiffness = stiffness_bn2(e_node.clone(), 0.2);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);

        let batch = 1usize;
        let topo = EdgeTopology::new(edges.clone());
        let n_e = topo.n_edges();
        let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n, 3]);
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
            .reshape([batch, n_e, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());
        let e_on_edges = DecEdgeOperators::arithmetic_mean_on_edges(e_node.clone(), edges.clone());
        let d_on_edges = DecEdgeOperators::arithmetic_mean_on_edges(damage.clone(), edges.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);
        let k_axial = e_on_edges.mul_scalar(area).div(edge_len.clone()).mul(dmg);

        let edge_data = edges.clone().into_data().value;
        let src: Vec<usize> = (0..n_e).map(|e| edge_data[e] as usize).collect();
        let tgt: Vec<usize> = (0..n_e).map(|e| edge_data[n_e + e] as usize).collect();
        let k_flat = k_axial.clone().into_data().value;
        let eu_flat = edge_unit.clone().into_data().value;
        let k64: Vec<f64> = k_flat.iter().map(|&x| x as f64).collect();
        let eu64: Vec<f64> = eu_flat.iter().map(|&x| x as f64).collect();
        let src_indices = topo.expand_src_gather_indices(batch, 3);
        let tgt_indices = topo.expand_tgt_gather_indices(batch, 3);

        Self {
            plate,
            dev,
            n,
            nx,
            ny,
            nz,
            edges,
            coords,
            e_node,
            mask,
            mask_flat,
            f_roof,
            f_roof_flat,
            area,
            stiffness,
            damage,
            k_axial,
            edge_unit,
            edge_len,
            src,
            tgt,
            src_indices,
            tgt_indices,
            ndof,
            k64,
            eu64,
        }
    }

    fn node_id(&self, ix: usize, iy: usize, iz: usize) -> usize {
        let nx1 = self.nx + 1;
        let ny1 = self.ny + 1;
        ix + iy * nx1 + iz * nx1 * ny1
    }

    fn cg_cfg(&self) -> MechanicsInnerLoopConfig {
        MechanicsInnerLoopConfig {
            max_cg_iterations: 2000,
            cg_tolerance: 1e-4,
            pcg_tolerance: 1e-4,
            // Matches cartridge harness + Step A ignored roof test (precond hides the floor).
            use_preconditioner: false,
            max_equilibrium_substeps: 1,
        }
    }

    fn mechanism_mode_candidates(&self) -> Vec<(&'static str, Vec<f64>)> {
        let mut out = Vec::new();

        let mut uniform_x = vec![0.0_f64; self.ndof];
        let mut uniform_y = vec![0.0_f64; self.ndof];
        let mut uniform_z = vec![0.0_f64; self.ndof];
        for i in 0..self.n {
            if self.mask_flat[i * 3] > 0.5 {
                uniform_x[i * 3] = 1.0;
            }
            if self.mask_flat[i * 3 + 1] > 0.5 {
                uniform_y[i * 3 + 1] = 1.0;
            }
            if self.mask_flat[i * 3 + 2] > 0.5 {
                uniform_z[i * 3 + 2] = 1.0;
            }
        }
        out.push(("uniform_x_free", uniform_x));
        out.push(("uniform_y_free", uniform_y));
        out.push(("uniform_z_free", uniform_z));

        for ix in 1..self.nx {
            for iy in 1..self.ny {
                let mut v_z = vec![0.0_f64; self.ndof];
                for iz in 0..=self.nz {
                    let nid = self.node_id(ix, iy, iz);
                    if self.mask_flat[nid * 3 + 2] > 0.5 {
                        v_z[nid * 3 + 2] = 1.0;
                    }
                }
                out.push(("column_z_slide", v_z));
            }
        }
        out
    }
}

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

fn stiffness_bn2<Bk: Backend<FloatElem = f32>>(e_node: Tensor<Bk, 3>, nu: f32) -> Tensor<Bk, 3> {
    let device = e_node.device();
    let [batch, n, _] = e_node.dims();
    let nu_t = Tensor::<Bk, 3>::full([batch, n, 1], nu, &device);
    Tensor::cat(vec![e_node, nu_t], 2)
}

fn projected_matvec_f64(
    u: &[f64],
    ku: &mut [f64],
    mask: &[f64],
    k_axial: &[f64],
    edge_unit: &[f64],
    src: &[usize],
    tgt: &[usize],
) {
    ku.fill(0.0);
    for e in 0..k_axial.len() {
        let s = src[e];
        let t = tgt[e];
        let ke = k_axial[e];
        let tu = e * 3;
        let ex = edge_unit[tu];
        let ey = edge_unit[tu + 1];
        let ez = edge_unit[tu + 2];
        let dx = u[s * 3] - u[t * 3];
        let dy = u[s * 3 + 1] - u[t * 3 + 1];
        let dz = u[s * 3 + 2] - u[t * 3 + 2];
        let elong = dx * ex + dy * ey + dz * ez;
        let f = ke * elong;
        ku[s * 3] += f * ex;
        ku[s * 3 + 1] += f * ey;
        ku[s * 3 + 2] += f * ez;
        ku[t * 3] -= f * ex;
        ku[t * 3 + 1] -= f * ey;
        ku[t * 3 + 2] -= f * ez;
    }
    for (k, &m) in ku.iter_mut().zip(mask) {
        *k *= m;
    }
}

fn matvec_f32(fx: &HarnessFixture, u: &[f32], ku: &mut [f32]) {
    let u_t = Tensor::from_data(Data::new(u.to_vec(), Shape::new([1, fx.n, 3])), &fx.dev);
    let ku_t = VectorMechanicsSolver::bar_matvec(
        u_t,
        &fx.k_axial,
        &fx.edge_unit,
        &fx.src_indices,
        &fx.tgt_indices,
        fx.n,
        None,
        &fx.edge_len,
    );
    let vals = ku_t.into_data().value;
    ku.copy_from_slice(&vals);
    for i in 0..fx.ndof {
        ku[i] *= fx.mask_flat[i] as f32;
    }
}

fn norm_masked(v: &[f64], mask: &[f64]) -> f64 {
    v.iter()
        .zip(mask)
        .map(|(&a, &m)| (a * m).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn inner_masked(a: &[f64], b: &[f64], mask: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .zip(mask)
        .map(|((&x, &y), &m)| x * y * m)
        .sum()
}

fn f_proj_flat(f: &[f64], mask: &[f64]) -> Vec<f64> {
    f.iter().zip(mask).map(|(&fi, &m)| fi * m).collect()
}

fn matvec_kff(kff: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut y = vec![0.0_f64; n];
    for i in 0..n {
        for j in 0..n {
            y[i] += kff[i][j] * x[j];
        }
    }
    y
}

/// Minimum \(\|K_{ff}u-f_f\|_2/\|f_f\|_2\) via CGLS (incompatible-RHS floor for semidefinite \(K\)).
fn min_residual_ratio_kff(kff: &[Vec<f64>], ff: &[f64], max_iter: usize) -> f64 {
    let n = ff.len();
    if n == 0 {
        return 0.0;
    }
    let ff_norm = ff.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-30);

    let mut u = vec![0.0_f64; n];
    let mut r = ff.to_vec();
    let mut s = r.clone();
    let mut norm_r = r.iter().map(|x| x * x).sum::<f64>().sqrt();

    for _ in 0..max_iter {
        let q = matvec_kff(kff, &s);
        let alpha_denom = s.iter().zip(&q).map(|(a, b)| a * b).sum::<f64>().max(1e-30);
        let alpha = norm_r.powi(2) / alpha_denom;

        for i in 0..n {
            u[i] += alpha * s[i];
        }
        for i in 0..n {
            r[i] -= alpha * q[i];
        }

        let norm_r_new = r.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm_r_new / ff_norm < 1e-14 {
            break;
        }

        let w = matvec_kff(&transpose_kff(kff), &r);
        let beta = norm_r_new.powi(2) / norm_r.powi(2).max(1e-30);
        norm_r = norm_r_new;
        for i in 0..n {
            s[i] = w[i] + beta * s[i];
        }
    }

    let ku = matvec_kff(kff, &u);
    let res = ku
        .iter()
        .zip(ff)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt();
    res / ff_norm
}

fn transpose_kff(kff: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = kff.len();
    let mut t = vec![vec![0.0_f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[i][j] = kff[j][i];
        }
    }
    t
}

fn best_mechanism_rho_pred(fx: &HarnessFixture) -> (f64, &'static str, f64) {
    let mut best_rho = 0.0_f64;
    let mut best_name = "none";
    let mut best_kappa = f64::INFINITY;
    for (name, v_raw) in fx.mechanism_mode_candidates() {
        let (kappa, rho, _) = mode_metrics(fx, &v_raw);
        if kappa < 1e-3 * rayleigh_max_eig_block(fx) {
            if rho > best_rho {
                best_rho = rho;
                best_name = name;
                best_kappa = kappa;
            }
        }
    }
    (best_rho, best_name, best_kappa)
}

fn rayleigh_max_eig_block(fx: &HarnessFixture) -> f64 {
    let free = free_dof_indices(&fx.mask_flat);
    let kff = assemble_kff(fx, &free);
    rayleigh_max_eig(&kff, 30).max(1.0)
}

fn mode_metrics(fx: &HarnessFixture, v_raw: &[f64]) -> (f64, f64, f64) {
    let mut v = vec![0.0_f64; fx.ndof];
    for i in 0..fx.ndof {
        v[i] = v_raw[i] * fx.mask_flat[i];
    }
    let v_norm = norm_masked(&v, &fx.mask_flat).max(1e-30);
    let mut kv = vec![0.0_f64; fx.ndof];
    projected_matvec_f64(
        &v,
        &mut kv,
        &fx.mask_flat,
        &fx.k64,
        &fx.eu64,
        &fx.src,
        &fx.tgt,
    );
    let kappa = norm_masked(&kv, &fx.mask_flat) / v_norm;
    let fp = f_proj_flat(&fx.f_roof_flat, &fx.mask_flat);
    let f_norm = norm_masked(&fp, &fx.mask_flat).max(1e-30);
    let rho_pred = inner_masked(&fp, &v, &fx.mask_flat).abs() / (f_norm * v_norm);
    (kappa, rho_pred, v_norm)
}

/// PCG relative residual + iteration count from the bar-network equilibrium witness.
///
/// FP §2 fail-closed: roof traction on the 9×8×2 singular harness stalls at `eq_rel ≈ 0.94`
/// (incompatible RHS floor). `solve_equilibrium_with_pcg_report` returns `PhysicsError::Diverged`
/// instead of `Ok` — probes 1/3 observe stall telemetry via the error arm, not a tolerance bump.
fn bar_pcg_rel_res(
    fx: &HarnessFixture,
    f_flat: &[f32],
    cfg: &MechanicsInnerLoopConfig,
) -> (f32, usize) {
    let body_force = Tensor::from_data(
        Data::new(f_flat.to_vec(), Shape::new([1, fx.n, 3])),
        &fx.dev,
    );
    match VectorMechanicsSolver::solve_equilibrium_with_pcg_report(
        Tensor::<B, 3>::zeros([1, fx.n, 3], &fx.dev),
        fx.coords.clone(),
        fx.stiffness.clone(),
        body_force,
        fx.edges.clone(),
        fx.damage.clone(),
        fx.mask.clone(),
        fx.area,
        cfg,
    ) {
        Ok((_, _, pcg)) => (pcg.rel_residual, pcg.iterations),
        Err(PhysicsError::Diverged {
            eq_rel,
            pcg_iterations,
        }) => (eq_rel, pcg_iterations),
        Err(e) => panic!(
            "VectorMechanicsSolver::solve_equilibrium_with_pcg_report on extruded plate bar network (FP §6 Track B6 H4 roof PCG witness): {e}"
        ),
    }
}

fn free_dof_indices(mask: &[f64]) -> Vec<usize> {
    mask.iter()
        .enumerate()
        .filter(|(_, &m)| m > 0.5)
        .map(|(i, _)| i)
        .collect()
}

fn assemble_kff(fx: &HarnessFixture, free: &[usize]) -> Vec<Vec<f64>> {
    let nf = free.len();
    let mut a = vec![vec![0.0_f64; nf]; nf];
    let mut u = vec![0.0_f64; fx.ndof];
    let mut ku = vec![0.0_f64; fx.ndof];
    for (col, &gd) in free.iter().enumerate() {
        u.fill(0.0);
        u[gd] = 1.0;
        projected_matvec_f64(
            &u,
            &mut ku,
            &fx.mask_flat,
            &fx.k64,
            &fx.eu64,
            &fx.src,
            &fx.tgt,
        );
        for (row, &gr) in free.iter().enumerate() {
            a[row][col] = ku[gr];
        }
    }
    a
}

/// Cholesky attempt; returns `None` if a non-positive pivot appears (singular / indefinite).
fn cholesky_ok(a: &[Vec<f64>], pivot_tol: f64) -> (bool, f64) {
    let n = a.len();
    if n == 0 {
        return (true, f64::INFINITY);
    }
    let mut l = vec![vec![0.0_f64; n]; n];
    let mut min_pivot = f64::INFINITY;
    for i in 0..n {
        let mut s = 0.0_f64;
        for k in 0..i {
            s += l[i][k] * l[i][k];
        }
        let piv = a[i][i] - s;
        min_pivot = min_pivot.min(piv);
        if piv <= pivot_tol {
            return (false, piv);
        }
        l[i][i] = piv.sqrt();
        for j in (i + 1)..n {
            let mut sum = 0.0_f64;
            for k in 0..i {
                sum += l[j][k] * l[i][k];
            }
            l[j][i] = (a[j][i] - sum) / l[i][i];
        }
    }
    (true, min_pivot)
}

fn rayleigh_max_eig(a: &[Vec<f64>], iters: usize) -> f64 {
    let n = a.len();
    if n == 0 {
        return 0.0;
    }
    let mut v: Vec<f64> = (0..n)
        .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
        .collect();
    let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    for x in &mut v {
        *x /= norm;
    }
    let mut lambda = 0.0_f64;
    for _ in 0..iters {
        let mut w = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                w[i] += a[i][j] * v[j];
            }
        }
        lambda = v.iter().zip(&w).map(|(a, b)| a * b).sum();
        let wn = w.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-30);
        for i in 0..n {
            v[i] = w[i] / wn;
        }
    }
    lambda
}

#[test]
fn probe1_mechanism_modes_and_roof_floor() {
    let fx = HarnessFixture::quick();
    let cfg = fx.cg_cfg();

    let (_, mode_name, kappa_best) = best_mechanism_rho_pred(&fx);
    let free = free_dof_indices(&fx.mask_flat);
    let kff = assemble_kff(&fx, &free);
    let ff: Vec<f64> = free.iter().map(|&gd| fx.f_roof_flat[gd]).collect();
    let rho_pred = min_residual_ratio_kff(&kff, &ff, 4000);

    let f32_roof: Vec<f32> = fx.f_roof.clone().into_data().value;
    let (pcg_obs, pcg_iters) = bar_pcg_rel_res(&fx, &f32_roof, &cfg);

    eprintln!(
        "PROBE1: best_mode={mode_name} kappa={kappa_best:.3e} rho_cgls={rho_pred:.4} pcg_obs={pcg_obs:.4} iters={pcg_iters}"
    );

    assert!(
        kappa_best < 1e-3 * rayleigh_max_eig_block(&fx),
        "best candidate should be a mechanism (κ={kappa_best})"
    );
    assert!(
        (rho_pred - pcg_obs as f64).abs() < 0.15,
        "CGLS min-residual floor {rho_pred} should reproduce observed pcg_rel_res {pcg_obs}"
    );
    assert!(
        pcg_obs > 0.5,
        "roof traction should stall (observed {pcg_obs})"
    );
}

#[test]
fn probe1b_perimeter_column_point_load_converges() {
    let fx = HarnessFixture::quick();
    let cfg = fx.cg_cfg();
    let mut bf = vec![0.0_f32; fx.ndof];
    let iz = fx.nz;
    let nid = fx.node_id(0, 0, iz);
    bf[nid * 3 + 2] = -50.0 * (fx.plate.dx * fx.plate.dy);

    let (pcg_rel, iters) = bar_pcg_rel_res(&fx, &bf, &cfg);
    eprintln!("PROBE1b: column point load pcg_rel={pcg_rel:.3e} iters={iters}");
    assert!(
        pcg_rel <= cfg.pcg_tolerance.max(cfg.cg_tolerance),
        "pinned-column axial path should converge: pcg_rel={pcg_rel} iters={iters}"
    );
    assert!(iters < 100, "expected O(10) iters, got {iters}");
}

#[test]
fn probe2_f32_f64_matvec_agreement() {
    let fx = HarnessFixture::quick();
    let mut max_err = 0.0_f32;
    let mut seed = 99_u64;
    for _ in 0..24 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut u = vec![0.0_f32; fx.ndof];
        for i in 0..fx.ndof {
            let r = ((seed >> 32) as f32) / (u32::MAX as f32) * 2.0 - 1.0;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            u[i] = r * fx.mask_flat[i] as f32;
        }
        let mut kf = vec![0.0_f32; fx.ndof];
        let mut kd = vec![0.0_f64; fx.ndof];
        matvec_f32(&fx, &u, &mut kf);
        let u64: Vec<f64> = u.iter().map(|&x| x as f64).collect();
        projected_matvec_f64(
            &u64,
            &mut kd,
            &fx.mask_flat,
            &fx.k64,
            &fx.eu64,
            &fx.src,
            &fx.tgt,
        );
        let num: f32 = kf
            .iter()
            .zip(&kd)
            .map(|(a, b)| (a - *b as f32).powi(2))
            .sum::<f32>()
            .sqrt();
        let den: f32 = kf.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-30);
        max_err = max_err.max(num / den);
    }
    eprintln!("PROBE2: max rel matvec err f32 vs f64 = {max_err:.3e}");
    assert!(max_err < 1e-4, "matvec cross-validation failed: {max_err}");
}

#[test]
fn probe3_kff_singular_and_incompatible_rhs() {
    let fx = HarnessFixture::quick();
    let free = free_dof_indices(&fx.mask_flat);
    let kff = assemble_kff(&fx, &free);
    let nf = free.len();
    eprintln!("PROBE3: n_free_dof={nf}");

    let lam_max = rayleigh_max_eig(&kff, 40);
    let pivot_tol = 1e-6 * lam_max.max(1.0);
    let (chol_ok, min_pivot) = cholesky_ok(&kff, pivot_tol);

    eprintln!("PROBE3: chol_ok={chol_ok} min_chol_pivot={min_pivot:.3e} λ_max_est={lam_max:.3e}");

    assert!(
        !chol_ok,
        "K_ff Cholesky should fail (singular semidefinite); chol_ok={chol_ok} min_pivot={min_pivot:.3e}"
    );

    let f32_roof: Vec<f32> = fx.f_roof.clone().into_data().value;
    let (pcg_obs, _) = bar_pcg_rel_res(&fx, &f32_roof, &fx.cg_cfg());
    let ff: Vec<f64> = free.iter().map(|&gd| fx.f_roof_flat[gd]).collect();
    let rho_cgls = min_residual_ratio_kff(&kff, &ff, 4000);
    eprintln!("PROBE3: rho_cgls={rho_cgls:.4} pcg_obs={pcg_obs:.4}");
    assert!(
        (rho_cgls - pcg_obs as f64).abs() < 0.15,
        "incompatible RHS floor mismatch: rho_cgls={rho_cgls} pcg={pcg_obs}"
    );
}

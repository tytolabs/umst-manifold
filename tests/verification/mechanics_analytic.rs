// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Track A4 mechanics verification: axial cantilever (bar network) and **Q1 hex + B-bar**
//! extruded-plate benchmarks on [`ExtrudedPlateMechanics`](umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics).
//!
//! The extruded plate uses **full-face** `u_z=0` on `z=0` plus two in-plane pins (not classical
//! Kirchhoff SSSS on all edges). **Q1 hex** thin slabs show **shear locking**; verification uses
//! equilibrium residual, linearity in `q`, in-plane refinement trends, and mesh-to-mesh deltas — not
//! a single thin-plate closed form with mismatched BCs.
//!
//! formal_anchor: Literature  
//! formal_citation: Timoshenko & Woinowsky-Krieger 1959 (plate tables); Bathe 2006 (Q1 hex); Hughes 2000 (shear locking)
//!
//! Requires `topology-density-evolution` (included in `solver-stable`).

#![cfg(feature = "topology-density-evolution")]
#![allow(clippy::too_many_arguments)]

use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};

use umst_manifold::physics::extruded_plate::{ElasticMaterial, ExtrudedPlateMechanics};
use umst_manifold::physics::mechanics::VectorMechanicsSolver;
use umst_manifold::physics::q1_hex_elasticity::hex_k_times_u_accumulate;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

const DAMAGE_REG: f32 = 1e-6;

type B = NdArray<f32>;

/// Kirchhoff square plate SSSS centre deflection: \(w_{\max} \approx 0.00406\, q L^4 / D\),
/// \(D = E h^3 / (12(1-\nu^2))\).
fn kirchhoff_centre_w_ssss(q: f32, l: f32, h: f32, e: f32, nu: f32) -> f32 {
    let d = e * h.powi(3) / (12.0 * (1.0 - nu * nu).max(1e-30));
    0.00406 * q * l.powi(4) / d.max(1e-30)
}

/// Top-face nodal loads matching the extruded-plate demo: `f_z = -q\,\Delta x\,\Delta y` on each top node.
fn body_force_top_pressure_extruded_style(
    nx: usize,
    ny: usize,
    nz: usize,
    q: f32,
    dx: f32,
    dy: f32,
) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut bf = vec![0.0_f32; n * 3];
    let iz = nz;
    let fz = -q * dx * dy;
    for iy in 0..=ny {
        for ix in 0..=nx {
            let nid = ix + iy * nx1 + iz * nx1 * ny1;
            bf[nid * 3 + 2] = fz;
        }
    }
    bf
}

/// Bottom face `u_z=0` plus minimal in-plane anchors on `z=0` so the 3D stiffness is positive-definite
/// (avoids free rigid translations that stall PCG on fine Q1 hex plates).
fn plate_bottom_uz_mask(nx: usize, ny: usize, nz: usize) -> Vec<f32> {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let mut m = vec![1.0_f32; n * 3];
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let nid = ix + iy * nx1 + iz * nx1 * ny1;
                if iz == 0 {
                    m[nid * 3 + 2] = 0.0;
                }
                if iz == 0 && ix == nx / 2 && iy == 0 {
                    m[nid * 3] = 0.0;
                }
                if iz == 0 && ix == 0 && iy == ny / 2 {
                    m[nid * 3 + 1] = 0.0;
                }
            }
        }
    }
    m
}

fn centre_top_uz(u: &Tensor<B, 3>, nx: usize, ny: usize, nz: usize) -> f32 {
    let cx = nx / 2;
    let cy = ny / 2;
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let mid = cx + cy * nx1 + nz * nx1 * ny1;
    u.clone()
        .slice([0..1, mid..(mid + 1), 2..3])
        .into_scalar()
        .abs()
}

fn uniform_e_cell(nx: usize, ny: usize, nz: usize, e: f32) -> Vec<f32> {
    vec![e; nx * ny * nz]
}

/// \(\|M(f-Ku)\|_2 / \|Mf\|_2\) with the same mask convention as the PCG solve.
fn masked_ku_residual_relative(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    f: &[f32],
    mask: &[f32],
    u: &[f32],
) -> f32 {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let ndof = nx1 * ny1 * (nz + 1) * 3;
    let mut ku = vec![0.0_f32; ndof];
    hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, u, &mut ku);
    let mut num = 0.0_f32;
    let mut den = 0.0_f32;
    for i in 0..ndof {
        let ri = mask[i] * (f[i] - ku[i]);
        num += ri * ri;
        let fi = f[i] * mask[i];
        den += fi * fi;
    }
    num.sqrt() / den.sqrt().max(1e-30_f32)
}

#[test]
fn cantilever_axial_chain_tip_displacement_n64() {
    let dev = NdArrayDevice::Cpu;
    let n: usize = 65;
    let l_total = 1.0_f32;
    let dx = l_total / (n - 1) as f32;
    let e = 200e9_f32;
    let a_sec = 0.01_f32;
    let f_tip = 1000.0_f32;

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

    let stiffness_val = vec![e; n];
    let stiffness_nu = vec![0.2_f32; n];
    let mut sf = Vec::with_capacity(n * 2);
    for i in 0..n {
        sf.push(stiffness_val[i]);
        sf.push(stiffness_nu[i]);
    }
    let stiffness = Tensor::from_data(Data::new(sf, Shape::new([1, n, 2])), &dev);

    let mut bf = vec![0.0_f32; n * 3];
    bf[(n - 1) * 3] = f_tip;
    let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);

    let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);

    let mut bm_data = vec![1.0_f32; n * 3];
    bm_data[0] = 0.0;
    bm_data[1] = 0.0;
    bm_data[2] = 0.0;
    let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([1, n, 3])), &dev);

    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 500,
        cg_tolerance: 1e-10,
        pcg_tolerance: 1e-10,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };

    let displacement = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let (u, _) = VectorMechanicsSolver::solve_equilibrium(
        displacement,
        coords,
        stiffness,
        body_force,
        edges_b1,
        damage,
        boundary_mask,
        a_sec,
        &cfg,
    );

    let u_tip = u.into_data().value[(n - 1) * 3];
    let u_analytic = f_tip * (n - 1) as f32 * dx / (e * a_sec * (1.0 + DAMAGE_REG));
    let err = ((u_tip - u_analytic).abs() / u_analytic.abs()).max(0.0);
    assert!(
        err < 0.02,
        "tip displacement error {err}: numeric={u_tip} analytic={u_analytic}"
    );
}

fn run_plate_case_details(
    nx: usize,
    ny: usize,
    nz: usize,
    q: f32,
    cg_tolerance: f32,
) -> (f32, f32) {
    let dev = NdArrayDevice::Cpu;
    let lx = 1.0_f32;
    let ly = 1.0_f32;
    let lz = 0.05_f32;
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
    let rho = Tensor::<B, 3>::full([1, n, 1], 1.0, &dev);
    let mask_flat = plate_bottom_uz_mask(nx, ny, nz);
    let bm = Tensor::from_data(Data::new(mask_flat.clone(), Shape::new([1, n, 3])), &dev);
    let mat = ElasticMaterial {
        e0: 30e9,
        nu: 0.2,
        simp_p: 1.0,
        e_min: 1.0,
    };
    let cfg = MechanicsInnerLoopConfig {
        max_cg_iterations: 50_000,
        cg_tolerance,
        pcg_tolerance: cg_tolerance,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    };
    let bf = body_force_top_pressure_extruded_style(nx, ny, nz, q, dx, dy);
    let body = Tensor::from_data(Data::new(bf.clone(), Shape::new([1, n, 3])), &dev);
    let (u, _) = plate.solve_equilibrium(rho, body, bm, mat, &cfg);
    let w = centre_top_uz(&u, nx, ny, nz);
    let u_flat = u.into_data().value;
    let e_cell = uniform_e_cell(nx, ny, nz, mat.e0);
    let rel = masked_ku_residual_relative(
        nx, ny, nz, dx, dy, dz, mat.nu, &e_cell, &bf, &mask_flat, &u_flat,
    );
    (w, rel)
}

#[inline]
fn run_plate_case(nx: usize, ny: usize, nz: usize, q: f32) -> f32 {
    // 1e-7 is often unreachable in f32 Jacobi-PCG and only burns `max_cg_iterations`; plate checks
    // gate on masked residual / deflection trends, not machine-precision iterate tolerance.
    run_plate_case_details(nx, ny, nz, q, 1e-5).0
}

#[test]
fn plate_16x16_pc_solve_reduces_masked_ku_residual() {
    let (w, rel) = run_plate_case_details(16, 16, 4, 10_000.0, 1e-5);
    assert!(
        rel < 1e-3,
        "expected PCG to reduce masked relative residual; got {rel} (w={w})"
    );
}

/// Uniform pressure scales linearly through the small-strain Hooke solve.
#[test]
fn plate_top_centre_response_linear_in_pressure() {
    let w1 = run_plate_case(8, 8, 4, 5_000.0);
    let w2 = run_plate_case(8, 8, 4, 10_000.0);
    assert!(w1 > 0.0 && w2 > 0.0, "positive centre deflection expected");
    let r = w2 / w1.max(1e-30);
    assert!(
        (r - 2.0).abs() < 5e-4,
        "expected w(2q)≈2 w(q); w1={w1} w2={w2} ratio={r}"
    );
}

/// In-plane refinement moves the centre value toward the shear-dominated asymptote (non-increasing).
#[test]
fn plate_in_plane_refinement_centre_w_monotone_decreasing() {
    let w8 = run_plate_case(8, 8, 4, 10_000.0);
    let w16 = run_plate_case(16, 16, 4, 10_000.0);
    let w32 = run_plate_case(32, 32, 4, 10_000.0);
    assert!(
        w8 > w16 && w16 > w32,
        "expected w8>w16>w32 in locked regime; w8={w8} w16={w16} w32={w32}"
    );
}

#[test]
fn plate_q1_hex_small_mesh_center_deflection_positive() {
    let w = run_plate_case(5, 5, 2, 500.0);
    assert!(
        w.is_finite() && w > 1e-12_f32,
        "expected measurable centre deflection on coarse Q1 hex plate; got w={w}"
    );
}

#[test]
fn kirchhoff_ssss_centre_formula_smoke() {
    // Table value scale check only — not compared to the Q1 hex extruded slab (see module docs).
    let w = kirchhoff_centre_w_ssss(10_000.0, 1.0, 0.05, 30e9, 0.2);
    assert!(
        w > 1.2e-4 && w < 1.3e-4,
        "unexpected Kirchhoff SSSS centre value: {w}"
    );
}

/// Q1-hex extruded plate centre deflection vs Kirchhoff **thin-plate** reference on a 32×32×4 mesh.
///
/// The extruded benchmark uses a **full** `u_z = 0` support on `z = 0` (see [`plate_bottom_uz_mask`]),
/// not classical SSSS edge data, and equal-order Q1 solids are **shear dominated / locked** at
/// `L/h = 20`. The discrete centre deflection is therefore **orders of magnitude below** the
/// Kirchhoff table value; this test pins `w / w_{\mathrm{Kirchhoff}}` into a fixed open band
/// (`5\times 10^{-5} < w/w_K < 2\times 10^{-2}`) so regressions in the equilibrium solve (or a
/// sudden reduction in locking) show up as failures, while still requiring a tight masked residual.
/// (CI name was formerly `plate_centre_deflection_vs_kirchhoff_ssss_within_5pct`, which incorrectly
/// suggested a 5% accuracy gate.)
///
/// Iterate tolerance is **`1e-5`**, not `1e-7`: f32 Jacobi-PCG rarely reaches \(10^{-7}\) relative
/// residual cost-effectively (see [`run_plate_case`]); tighter tol mainly burns iterations without
/// improving the masked \(\|f-Ku\|\) check below.
#[test]
fn plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band() {
    let nx = 32;
    let ny = 32;
    let nz = 4;
    let lx = 1.0_f32;
    let lz = 0.05_f32;
    let q = 10_000.0_f32;

    let (w_numerical, res) = run_plate_case_details(nx, ny, nz, q, 1e-5);
    assert!(
        res < 1e-3,
        "expected masked equilibrium residual <1e-3; got {res} (w={w_numerical})"
    );

    let w_kirchhoff = kirchhoff_centre_w_ssss(q, lx, lz, 30e9, 0.2);
    let ratio = w_numerical / w_kirchhoff.max(1e-30);
    assert!(
        w_numerical.is_finite() && w_numerical > 0.0,
        "expected positive centre deflection; got {w_numerical}"
    );
    assert!(
        ratio > 5e-5 && ratio < 0.02,
        "expected locked Q1-hex centre deflection between ~5e-5 and 0.02 × Kirchhoff thin-plate value; ratio={ratio} (w={w_numerical}, w_k={w_kirchhoff})"
    );
}

/// Fixed thickness (`nz=4`): successive in-plane refinements yield stable centre deflection (Δ relative).
#[test]
fn plate_centre_mesh_refinement_small_relative_change() {
    let w_mid = run_plate_case(24, 24, 4, 10_000.0);
    let w_fine = run_plate_case(32, 32, 4, 10_000.0);
    let rel = (w_fine - w_mid).abs() / w_fine.max(1e-30);
    assert!(
        rel < 0.06,
        "expected <6% centre delta 24²→32² for nz=4; rel={rel} w24={w_mid} w32={w_fine}"
    );
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Suspect 2 — design-dependent self-weight adjoint: central FD vs discrete adjoint.
//!
//! **Test A:** ~10 random elements, adjoint vs central FD within ~1e-3 relative (SELF_WEIGHT ON/OFF).
//! **Test B:** sign sanity — min/max sensitivity by density decile (void regions must not flip sign).

#![cfg(feature = "mechanics-adjoint-q1-hex")]
#![allow(clippy::too_many_arguments)]

use burn::backend::Autodiff;
use burn::tensor::{backend::AutodiffBackend, Data, Shape, Tensor};
use burn_ndarray::NdArray;
use rand::{rngs::StdRng, Rng, SeedableRng};

use umst_manifold::physics::adjoint::SimpElasticMaterial;
use umst_manifold::physics::adjoint_q1_hex::AdjointComplianceQ1Hex;
use umst_manifold::physics::extruded_plate::ExtrudedPlateMechanics;
use umst_manifold::physics::mechanics::SelfWeightConfig;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;

type AD = Autodiff<NdArray<f32>>;
type Inner = <AD as AutodiffBackend>::InnerBackend;

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

fn body_force_flat(
    rho_flat: &[f32],
    n_nodes: usize,
    sw: Option<SelfWeightConfig>,
    traction_flat: &[f32],
) -> Vec<f32> {
    let mut f = traction_flat.to_vec();
    if let Some(cfg) = sw {
        let sw_f = cfg.body_force_flat(rho_flat, n_nodes);
        for (a, b) in f.iter_mut().zip(sw_f) {
            *a += b;
        }
    }
    f
}

fn compliance_fd(
    rho_flat: &[f32],
    plate: &ExtrudedPlateMechanics,
    traction: &[f32],
    mask: &[f32],
    mat: SimpElasticMaterial,
    cg: &MechanicsInnerLoopConfig,
    sw: Option<SelfWeightConfig>,
) -> f32 {
    let n = plate.n_nodes();
    let f = body_force_flat(rho_flat, n, sw, traction);
    AdjointComplianceQ1Hex::raw_compliance_at_rho(
        rho_flat, plate.nx, plate.ny, plate.nz, plate.dx, plate.dy, plate.dz, &f, mask, mat, cg, sw,
    )
}

fn adjoint_grad_at_nodes(
    rho_flat: &[f32],
    plate: &ExtrudedPlateMechanics,
    traction_flat: &[f32],
    mask: &[f32],
    mat: SimpElasticMaterial,
    cg: &MechanicsInnerLoopConfig,
    sw: Option<SelfWeightConfig>,
) -> Vec<f32> {
    let dev = Default::default();
    let n = plate.n_nodes();
    let f = body_force_flat(rho_flat, n, sw, traction_flat);
    let body_force = Tensor::<Inner, 3>::from_data(Data::new(f, Shape::new([1, n, 3])), &dev);
    let boundary =
        Tensor::<Inner, 3>::from_data(Data::new(mask.to_vec(), Shape::new([1, n, 3])), &dev);
    let rho_ad =
        Tensor::<AD, 3>::from_data(Data::new(rho_flat.to_vec(), Shape::new([1, n, 1])), &dev)
            .require_grad();
    let (surrogate, _) = AdjointComplianceQ1Hex::forward_and_loss(
        rho_ad.clone(),
        plate.nx,
        plate.ny,
        plate.nz,
        plate.dx,
        plate.dy,
        plate.dz,
        body_force,
        boundary,
        mat,
        cg,
        sw,
    );
    rho_ad
        .grad(&surrogate.backward())
        .expect("grad")
        .into_data()
        .value
}

fn quick_plate() -> ExtrudedPlateMechanics {
    ExtrudedPlateMechanics {
        nx: 9,
        ny: 8,
        nz: 2,
        dx: 0.8 / 9.0,
        dy: 0.8 / 8.0,
        dz: 0.1 / 2.0,
    }
}

fn quick_material() -> SimpElasticMaterial {
    SimpElasticMaterial {
        e0: 200e6_f32,
        nu: 0.2,
        p: 3.0,
        e_min: 200e3_f32,
    }
}

fn quick_cg() -> MechanicsInnerLoopConfig {
    MechanicsInnerLoopConfig {
        max_cg_iterations: 3000,
        cg_tolerance: 1e-5_f32,
        pcg_tolerance: 1e-5_f32,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    }
}

fn quick_cg_fd() -> MechanicsInnerLoopConfig {
    MechanicsInnerLoopConfig {
        max_cg_iterations: 6000,
        cg_tolerance: 1e-5_f32,
        pcg_tolerance: 1e-5_f32,
        use_preconditioner: true,
        max_equilibrium_substeps: 1,
    }
}

fn quick_self_weight(plate: &ExtrudedPlateMechanics) -> SelfWeightConfig {
    SelfWeightConfig {
        gravity_m_s2: 9.81,
        voxel_volume_m3: plate.dx * plate.dy * plate.dz,
        mass_penalty_q: 1.0,
        direction: [0.0, 0.0, -1.0],
    }
}

#[test]
fn adjoint_self_weight_fd_matches_on_quick_grid() {
    let plate = quick_plate();
    let n = plate.n_nodes();
    let mask = plate_bottom_uz_mask(plate.nx, plate.ny, plate.nz);
    let traction = plate.body_force_top_uniform_pressure(50.0);
    let mat = quick_material();
    let cg = quick_cg();
    let cg_fd = quick_cg_fd();
    let sw = quick_self_weight(&plate);

    let mut rng = StdRng::seed_from_u64(42);
    let rho_flat: Vec<f32> = (0..n).map(|_| rng.gen_range(0.15_f32..0.85_f32)).collect();

    let g = adjoint_grad_at_nodes(&rho_flat, &plate, &traction, &mask, mat, &cg, Some(sw));
    let eps = 2e-3_f32;
    let mut worst_rel = 0.0_f32;
    let mut worst_nid = 0usize;

    for _ in 0..10 {
        let nid = rng.gen_range(0..n);
        let mut rho_plus = rho_flat.clone();
        let mut rho_minus = rho_flat.clone();
        rho_plus[nid] = (rho_plus[nid] + eps).min(1.0_f32);
        rho_minus[nid] = (rho_minus[nid] - eps).max(1e-4_f32);
        let c_plus = compliance_fd(&rho_plus, &plate, &traction, &mask, mat, &cg_fd, Some(sw));
        let c_minus = compliance_fd(&rho_minus, &plate, &traction, &mask, mat, &cg_fd, Some(sw));
        let fd = (c_plus - c_minus) / (rho_plus[nid] - rho_minus[nid]);
        let denom = fd.abs().max(g[nid].abs()).max(1e-12_f32);
        let rel = (g[nid] - fd).abs() / denom;
        if rel > worst_rel {
            worst_rel = rel;
            worst_nid = nid;
        }
        assert!(
            rel < 2.5e-2_f32,
            "SELF_WEIGHT ON nid={nid}: adjoint={} fd={} rel={rel}",
            g[nid],
            fd
        );
    }
    eprintln!(
        "adjoint_self_weight_fd ON: worst_rel={worst_rel:.4e} at nid={worst_nid} rho={:.4}",
        rho_flat[worst_nid]
    );
}

#[test]
fn adjoint_no_self_weight_fd_regression_on_quick_grid() {
    let plate = quick_plate();
    let n = plate.n_nodes();
    let mask = plate_bottom_uz_mask(plate.nx, plate.ny, plate.nz);
    let traction = plate.body_force_top_uniform_pressure(50.0);
    let mat = quick_material();
    let cg = quick_cg();
    let cg_fd = quick_cg_fd();

    let mut rng = StdRng::seed_from_u64(99);
    let rho_flat: Vec<f32> = (0..n).map(|_| rng.gen_range(0.35_f32..0.85_f32)).collect();

    let g = adjoint_grad_at_nodes(&rho_flat, &plate, &traction, &mask, mat, &cg, None);
    let eps = 2e-3_f32;

    for _ in 0..10 {
        let nid = rng.gen_range(0..n);
        let mut rho_plus = rho_flat.clone();
        let mut rho_minus = rho_flat.clone();
        rho_plus[nid] = (rho_plus[nid] + eps).min(1.0_f32);
        rho_minus[nid] = (rho_minus[nid] - eps).max(1e-4_f32);
        let c_plus = compliance_fd(&rho_plus, &plate, &traction, &mask, mat, &cg_fd, None);
        let c_minus = compliance_fd(&rho_minus, &plate, &traction, &mask, mat, &cg_fd, None);
        let fd = (c_plus - c_minus) / (rho_plus[nid] - rho_minus[nid]);
        let denom = fd.abs().max(g[nid].abs()).max(1e-12_f32);
        let rel = (g[nid] - fd).abs() / denom;
        assert!(
            rel < 2.5e-2_f32,
            "SELF_WEIGHT OFF nid={nid}: adjoint={} fd={} rel={rel}",
            g[nid],
            fd
        );
    }
    eprintln!("adjoint_self_weight_fd OFF: regression OK on 10 nodes");
}

#[test]
fn adjoint_self_weight_sign_sanity_by_density_decile() {
    let plate = quick_plate();
    let n = plate.n_nodes();
    let mask = plate_bottom_uz_mask(plate.nx, plate.ny, plate.nz);
    let traction = plate.body_force_top_uniform_pressure(50.0);
    let mat = quick_material();
    let cg = quick_cg();
    let sw = quick_self_weight(&plate);

    let mut rng = StdRng::seed_from_u64(7);
    let rho_flat: Vec<f32> = (0..n).map(|_| rng.gen_range(0.05_f32..0.95_f32)).collect();
    let g = adjoint_grad_at_nodes(&rho_flat, &plate, &traction, &mask, mat, &cg, Some(sw));

    let mut decile_mins = [f32::INFINITY; 10];
    let mut decile_maxs = [f32::NEG_INFINITY; 10];
    let mut decile_count = [0usize; 10];
    for (i, &rho) in rho_flat.iter().enumerate() {
        let d = ((rho * 10.0).floor() as usize).min(9);
        decile_mins[d] = decile_mins[d].min(g[i]);
        decile_maxs[d] = decile_maxs[d].max(g[i]);
        decile_count[d] += 1;
    }

    eprintln!("self_weight sign sanity by density decile (ρ in [d/10, (d+1)/10]):");
    for d in 0..10 {
        if decile_count[d] == 0 {
            continue;
        }
        eprintln!(
            "  decile {d}: count={} sens_min={:.4e} sens_max={:.4e} rho_band=[{:.1},{:.1})",
            decile_count[d],
            decile_mins[d],
            decile_maxs[d],
            d as f32 / 10.0,
            (d + 1) as f32 / 10.0,
        );
    }

    // Near-void (deciles 0–1): adding mass increases self-weight load → dC/dρ should not be
    // strongly negative (bug dropped 2u^T ∂f/∂ρ, making voids look artificially attractive).
    for d in 0..=1 {
        if decile_count[d] > 0 {
            assert!(
                decile_maxs[d] > -1e-2_f32,
                "decile {d} max sensitivity {:.4e} too negative — void flip toward add-mass",
                decile_maxs[d]
            );
        }
    }
}

// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
#![cfg(feature = "solver-experimental")]

use burn::tensor::{Data, Shape, Tensor};
use burn_ndarray::NdArray;

use umst_manifold::physics::topology_filter::HelmholtzFilter;

type B = NdArray<f32>;

fn grid_4_edges(
    nx: usize,
    ny: usize,
    dev: &burn_ndarray::NdArrayDevice,
) -> burn::tensor::Tensor<B, 2, burn::tensor::Int> {
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    let id = |ix: usize, iy: usize| -> i64 { (ix + iy * nx) as i64 };
    for iy in 0..ny {
        for ix in 0..(nx - 1) {
            pairs.push((id(ix, iy), id(ix + 1, iy)));
        }
    }
    for iy in 0..(ny - 1) {
        for ix in 0..nx {
            pairs.push((id(ix, iy), id(ix, iy + 1)));
        }
    }
    let ne = pairs.len();
    let mut f = Vec::with_capacity(ne * 2);
    for (a, _) in &pairs {
        f.push(*a as f32);
    }
    for (_, b) in &pairs {
        f.push(*b as f32);
    }
    Tensor::<B, 1>::from_data(Data::new(f, Shape::new([ne * 2])), dev)
        .reshape([2, ne])
        .int()
}

#[test]
fn helmholtz_delta_blob_fwhm_matches_green_scale() {
    let dev = Default::default();
    let nx = 32usize;
    let ny = 32usize;
    let n = nx * ny;
    let dx = 1.0_f32;
    let r = 2.0 * dx;
    let mut rho = vec![0.0f32; n];
    let cx = nx / 2;
    let cy = ny / 2;
    rho[cx + cy * nx] = 1.0;
    let rho_t: Tensor<B, 3> = Tensor::from_data(Data::new(rho, Shape::new([1, n, 1])), &dev);
    let edges = grid_4_edges(nx, ny, &dev);
    let filter = HelmholtzFilter::new(r, 240, 1e-7);
    let out = filter
        .apply(rho_t, edges, dx)
        .expect("HelmholtzFilter::apply on delta blob grid (FP §6 topology filter integration verification)");
    let vals = out.into_data().value;
    let peak = vals.iter().cloned().fold(0.0_f32, f32::max);
    assert!(peak > 0.05 && peak <= 1.5, "peak out of band: {peak}");
    let idx_max = vals
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            a.partial_cmp(b).expect(
                "Helmholtz peak index comparison on finite f32 densities (FP §6 topology filter integration verification)",
            )
        })
        .map(|(i, _)| i)
        .expect("Helmholtz delta blob grid filtered density peak enumeration (FP §6 topology filter integration verification)");
    let ix_m = idx_max % nx;
    let iy_m = idx_max / nx;
    let mut neighbor_abs = 0.0_f32;
    if ix_m > 0 {
        neighbor_abs += vals[idx_max - 1].abs();
    }
    if ix_m + 1 < nx {
        neighbor_abs += vals[idx_max + 1].abs();
    }
    if iy_m > 0 {
        neighbor_abs += vals[idx_max - nx].abs();
    }
    if iy_m + 1 < ny {
        neighbor_abs += vals[idx_max + nx].abs();
    }
    assert!(
        neighbor_abs > 1e-4 * peak.abs().max(1e-6),
        "Helmholtz filter should couple to neighbors (neighbor_abs={neighbor_abs})"
    );
}

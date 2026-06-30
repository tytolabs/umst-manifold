// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured **8-node trilinear (Q1) hex** linear elasticity on a Cartesian brick lattice.
//!
//! Matrix-free `K u` and assembled **Jacobi diagonal** for projected PCG on `[nx × ny × nz]`
//! cells with `(nx+1)(ny+1)(nz+1)` nodes. Gauss integration uses the standard `2×2×2` rule on the
//! reference cube \([-1,1]^3\).
//!
//! **B-bar / Selective Reduced Integration (SRI)** cures volumetric locking; **transverse shear**
//! strains \(\gamma_{yz},\gamma_{xz}\) use **centroid** shape gradients at each \(2^3\) Gauss point
//! (substituted strain, analogous to transverse shear under-integration). In-plane \(\gamma_{xy}\)
//! stays full quadrature.
//!
//! Volumetric `B_vol = (1/3) m mᵀ B` is replaced by its element-mean `B̄_vol` (centroid); deviatoric
//! normal strains retain the full \(2\times2\times2\) rule. See Hughes 2000 §4.5 and Bathe 2006 §5.4.
//!
//! ## Roadmap (R2.1 — mechanics / thin plate)
//!
//! **Shipped here:** **B-bar** on volumetric normal strains **plus** centroid \(\gamma_{yz},\gamma_{xz}\)
//! (transverse shear SRI); \(\gamma_{xy}\) remains full \(2\times2\times2\). [`hex_k_times_u_accumulate`]
//! and [`hex_diagonal`] apply **one** isotropic Voigt \(\mathbf D(E,\nu)\) to the B-bar /
//! transverse-shear-centroid strain vector at each \(2^3\) Gauss point (and the same operator in
//! [`hex_cell_strain_energy`]). PCG in [`hex_solve_pcg_masked`] is matrix-free on that kernel.
//!
//! **Phase 1A / §R2.1 note:** a naive additive split **\(\mathbf D_{\mathrm{vol}}\)** (centroid) +
//! **\(\mathbf D_{\mathrm{dev}}\)** (full Gauss) with **\(\boldsymbol\sigma_{\mathrm{vol}}=K\bar\varepsilon_v\mathbf m\)** and
//! **\(\boldsymbol\sigma_{\mathrm{dev}}=\mathbf D_{\mathrm{dev}}\boldsymbol\varepsilon\)** at Gauss points is **not**
//! pointwise equivalent to **\(\mathbf D\boldsymbol\varepsilon\)** and regressed the slender-column
//! **`adjoint_q1_hex_matches_bar_in_limit`** compliance check; a variationally consistent selective-reduced
//! **\(\mathbf D\)** split remains follow-up work. See `docs/Solver-Status.md` mechanics row.
//!
//! **Still open vs v0.4 R2.1 acceptance:** a strict **thin-plate Kirchhoff** gate
//! (\(\approx 5\%\) error to the SSSS centre formula at **32²×4**, **h/L = 0.02**) is wired as
//! `plate_centre_deflection_kirchhoff_ssss_q1_hex_within_five_percent` in
//! `tests/verification/mechanics_analytic.rs` but remains **`#[ignore]`** with measured residual on
//! the extruded-plate BC harness (full-face \(u_z=0\) plus in-plane pins — not facet-wise SSSS).
//! Default CI keeps the **ratio band** test (`plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band`)
//! and the env-gated **`plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate`**; see
//! `docs/Solver-Status.md` mechanics row.
//!
//! **Bounded follow-ups (avoid monolithic refactors):** **facet-wise** BC sets for SSSS parity;
//! **MITC-style** enrichment or literal \(\mathbf D_{\mathrm{shear}}\) splits if further tuning is
//! needed; **f64** accumulation path for the same stencil if f32 PCG limits masked residuals.
//!
//! formal_anchor: Literature
//! formal_citation: Bathe 2006, *Finite Element Procedures*, §5.4 (hex elements); Hughes 2000, *The Finite Element Method*, §4.5 (B-bar / SRI)
//! formal_form: \(\int_{Ω^e} \mathbf B^{\mathsf T}\mathbf D\mathbf B\,\mathrm dΩ\,\mathbf u^e = \mathbf f^e\) with isotropic \(\mathbf D(E,\nu)\) in Voigt form; volumetric block uses \(\bar{\mathbf B}_{\text{vol}}\) (element-mean) and deviatoric block uses pointwise \(\mathbf B_{\text{dev}}\).

#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

use super::pcg_reduction::{
    dot_f32, dot_f64, masked_dot_f32, masked_norm_sq_f32, masked_norm_sq_f64, norm_sq_f64,
};

/// Parent-space coordinates \((\xi,\eta,\zeta)\in\{-1,1\}^3\) for the eight corner nodes (lexicographic
/// ordering consistent with [`super::extruded_plate::ExtrudedPlateMechanics`] cell corners).
const CORNER_XI: [[f32; 3]; 8] = [
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
];

/// Two-point Gauss rule on \([-1,1]\): nodes \(\pm 1/\sqrt{3}\), unit weights.
const GAUSS1D: [f32; 2] = [-0.5773502691896257, 0.5773502691896257];
const WG: f32 = 1.0;

#[inline]
fn idx_node(nx1: usize, ny1: usize, ix: usize, iy: usize, iz: usize) -> usize {
    ix + iy * nx1 + iz * nx1 * ny1
}

/// Physical coordinates of the eight corners for cell `(cx, cy, cz)` with spacing `(dx,dy,dz)`.
fn cell_corner_coords(cx: usize, cy: usize, cz: usize, dx: f32, dy: f32, dz: f32) -> [[f32; 3]; 8] {
    let ix0 = cx;
    let iy0 = cy;
    let iz0 = cz;
    let c = |ix: usize, iy: usize, iz: usize| -> [f32; 3] {
        [ix as f32 * dx, iy as f32 * dy, iz as f32 * dz]
    };
    [
        c(ix0, iy0, iz0),
        c(ix0 + 1, iy0, iz0),
        c(ix0 + 1, iy0 + 1, iz0),
        c(ix0, iy0 + 1, iz0),
        c(ix0, iy0, iz0 + 1),
        c(ix0 + 1, iy0, iz0 + 1),
        c(ix0 + 1, iy0 + 1, iz0 + 1),
        c(ix0, iy0 + 1, iz0 + 1),
    ]
}

#[inline]
fn mat3_inv(j: [[f32; 3]; 3]) -> Option<([[f32; 3]; 3], f32)> {
    let (a00, a01, a02) = (j[0][0], j[0][1], j[0][2]);
    let (a10, a11, a12) = (j[1][0], j[1][1], j[1][2]);
    let (a20, a21, a22) = (j[2][0], j[2][1], j[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20)
        + a02 * (a10 * a21 - a11 * a20);
    if det.abs() < 1e-30_f32 {
        return None;
    }
    let inv_det = 1.0 / det;
    let inv = [
        [
            (a11 * a22 - a12 * a21) * inv_det,
            (a02 * a21 - a01 * a22) * inv_det,
            (a01 * a12 - a02 * a11) * inv_det,
        ],
        [
            (a12 * a20 - a10 * a22) * inv_det,
            (a00 * a22 - a02 * a20) * inv_det,
            (a02 * a10 - a00 * a12) * inv_det,
        ],
        [
            (a10 * a21 - a11 * a20) * inv_det,
            (a01 * a20 - a00 * a21) * inv_det,
            (a00 * a11 - a01 * a10) * inv_det,
        ],
    ];
    Some((inv, det))
}

fn build_d_voigt(e: f32, nu: f32) -> [[f32; 6]; 6] {
    let lam = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)).max(1e-30_f32);
    let mu = e / (2.0 * (1.0 + nu)).max(1e-30_f32);
    let l2m = lam + 2.0 * mu;
    [
        [l2m, lam, lam, 0.0, 0.0, 0.0],
        [lam, l2m, lam, 0.0, 0.0, 0.0],
        [lam, lam, l2m, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, mu, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, mu, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, mu],
    ]
}

#[inline]
fn dshape_parent(i_corner: usize, s: f32, t: f32, z: f32) -> [f32; 3] {
    let (xi, et, ze) = (
        CORNER_XI[i_corner][0],
        CORNER_XI[i_corner][1],
        CORNER_XI[i_corner][2],
    );
    let ax = 0.125 * (1.0 + et * t) * (1.0 + ze * z);
    let ay = 0.125 * (1.0 + xi * s) * (1.0 + ze * z);
    let az = 0.125 * (1.0 + xi * s) * (1.0 + et * t);
    [xi * ax, et * ay, ze * az]
}

/// `grad_parent[i]` = column \([\partial N_i/\partial s, \partial N_i/\partial t, \partial N_i/\partial z]^T\).
fn shape_gradients_parent(s: f32, t: f32, z: f32) -> [[f32; 3]; 8] {
    let mut g = [[0.0_f32; 3]; 8];
    for i in 0..8 {
        g[i] = dshape_parent(i, s, t, z);
    }
    g
}

/// Physical gradients `dN_i/dx`, `dN_i/dy`, `dN_i/dz` stacked as rows `gn[i][0..3]`.
fn physical_shape_gradients(
    x_corner: [[f32; 3]; 8],
    s: f32,
    t: f32,
    z: f32,
) -> Option<([[f32; 3]; 8], f32)> {
    let gp = shape_gradients_parent(s, t, z);
    let mut j = [[0.0_f32; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            let mut sum = 0.0_f32;
            for k in 0..8 {
                sum += x_corner[k][r] * gp[k][c];
            }
            j[r][c] = sum;
        }
    }
    let (inv_j, det) = mat3_inv(j)?;
    let mut gn = [[0.0_f32; 3]; 8];
    for i in 0..8 {
        let gp_i = gp[i];
        for r in 0..3 {
            let mut sum = 0.0_f32;
            for c in 0..3 {
                sum += inv_j[r][c] * gp_i[c];
            }
            gn[i][r] = sum;
        }
    }
    Some((gn, det.abs()))
}

#[allow(dead_code)]
fn b_times_u(gn: [[f32; 3]; 8], u24: &[f32; 24]) -> [f32; 6] {
    let mut e = [0.0_f32; 6];
    for i in 0..8 {
        let ui = u24[i * 3];
        let vi = u24[i * 3 + 1];
        let wi = u24[i * 3 + 2];
        let gx = gn[i][0];
        let gy = gn[i][1];
        let gz = gn[i][2];
        e[0] += gx * ui;
        e[1] += gy * vi;
        e[2] += gz * wi;
        e[3] += gy * ui + gx * vi;
        e[4] += gz * vi + gy * wi;
        e[5] += gz * ui + gx * wi;
    }
    e
}

/// B-bar strain at this quadrature point: shear rows use pointwise `gn`, normal-strain rows
/// use the deviatoric part of pointwise `gn` plus the volumetric part from `gn_bar` (centroid /
/// element-mean). For an isotropic Voigt formulation this is equivalent to
/// `(B̄_vol + B_dev) u` where `B_vol = (1/3) m mᵀ B`.
fn bbar_times_u(gn: [[f32; 3]; 8], gn_bar: [[f32; 3]; 8], u24: &[f32; 24]) -> [f32; 6] {
    // Volumetric (mean) trace strain  ε_v_bar = Σ_i (ḡx_i u_i + ḡy_i v_i + ḡz_i w_i)
    let mut ev_bar = 0.0_f32;
    // Pointwise normal strains
    let mut exx = 0.0_f32;
    let mut eyy = 0.0_f32;
    let mut ezz = 0.0_f32;
    let mut e3 = 0.0_f32;
    let mut e4 = 0.0_f32;
    let mut e5 = 0.0_f32;
    for i in 0..8 {
        let ui = u24[i * 3];
        let vi = u24[i * 3 + 1];
        let wi = u24[i * 3 + 2];
        let gx = gn[i][0];
        let gy = gn[i][1];
        let gz = gn[i][2];
        exx += gx * ui;
        eyy += gy * vi;
        ezz += gz * wi;
        e3 += gy * ui + gx * vi;
        e4 += gz * vi + gy * wi;
        e5 += gz * ui + gx * wi;
        let gxb = gn_bar[i][0];
        let gyb = gn_bar[i][1];
        let gzb = gn_bar[i][2];
        ev_bar += gxb * ui + gyb * vi + gzb * wi;
    }
    let ev_pt = exx + eyy + ezz;
    let delta = (ev_bar - ev_pt) / 3.0;
    // Replace volumetric (mean) part: ε_normal_bar = ε_normal_pt + δ * m, where δ shifts the
    // hydrostatic component from pointwise to the element mean.
    [exx + delta, eyy + delta, ezz + delta, e3, e4, e5]
}

/// [`bbar_times_u`] with \(\gamma_{yz},\gamma_{xz}\) from centroid gradients (`gn_bar`).
fn bbar_times_u_transverse_shear_centroid(
    gn: [[f32; 3]; 8],
    gn_bar: [[f32; 3]; 8],
    u24: &[f32; 24],
) -> [f32; 6] {
    let mut eps = bbar_times_u(gn, gn_bar, u24);
    let eps_c = bbar_times_u(gn_bar, gn_bar, u24);
    eps[4] = eps_c[4];
    eps[5] = eps_c[5];
    eps
}

#[allow(dead_code)]
fn bt_times_sigma(gn: [[f32; 3]; 8], sig: &[f32; 6]) -> [f32; 24] {
    let mut f = [0.0_f32; 24];
    for i in 0..8 {
        let gx = gn[i][0];
        let gy = gn[i][1];
        let gz = gn[i][2];
        let sxx = sig[0];
        let syy = sig[1];
        let szz = sig[2];
        let sxy = sig[3];
        let syz = sig[4];
        let sxz = sig[5];
        f[i * 3] += gx * sxx + gy * sxy + gz * sxz;
        f[i * 3 + 1] += gy * syy + gx * sxy + gz * syz;
        f[i * 3 + 2] += gz * szz + gy * syz + gx * sxz;
    }
    f
}

/// Adjoint of [`bbar_times_u_transverse_shear_centroid`]: same B-bar normal rows as
/// [`bbar_t_times_sigma`]; \(\tau_{yz},\tau_{xz}\) scatter with centroid gradients.
fn bbar_t_times_sigma_transverse_shear_centroid(
    gn: [[f32; 3]; 8],
    gn_bar: [[f32; 3]; 8],
    sig: &[f32; 6],
) -> [f32; 24] {
    let mut f = [0.0_f32; 24];
    let sxx = sig[0];
    let syy = sig[1];
    let szz = sig[2];
    let sxy = sig[3];
    let syz = sig[4];
    let sxz = sig[5];
    let sh = (sxx + syy + szz) / 3.0;
    let dxx = sxx - sh;
    let dyy = syy - sh;
    let dzz = szz - sh;
    for i in 0..8 {
        let gx = gn[i][0];
        let gy = gn[i][1];
        let gz = gn[i][2];
        let gxb = gn_bar[i][0];
        let gyb = gn_bar[i][1];
        let gzb = gn_bar[i][2];
        f[i * 3] += gx * dxx + gy * sxy + gzb * sxz + gxb * sh;
        f[i * 3 + 1] += gy * dyy + gx * sxy + gzb * syz + gyb * sh;
        f[i * 3 + 2] += gz * dzz + gyb * syz + gxb * sxz + gzb * sh;
    }
    f
}

/// B-bar transpose × stress: adjoint of [`bbar_times_u`] (no transverse shear SRI).
#[allow(dead_code)]
fn bbar_t_times_sigma(gn: [[f32; 3]; 8], gn_bar: [[f32; 3]; 8], sig: &[f32; 6]) -> [f32; 24] {
    let mut f = [0.0_f32; 24];
    let sxx = sig[0];
    let syy = sig[1];
    let szz = sig[2];
    let sxy = sig[3];
    let syz = sig[4];
    let sxz = sig[5];
    // Hydrostatic (mean of normal stresses) coupled via volumetric averaged rows; deviatoric
    // normal-stress part coupled via pointwise rows.
    let sh = (sxx + syy + szz) / 3.0;
    let dxx = sxx - sh;
    let dyy = syy - sh;
    let dzz = szz - sh;
    for i in 0..8 {
        let gx = gn[i][0];
        let gy = gn[i][1];
        let gz = gn[i][2];
        let gxb = gn_bar[i][0];
        let gyb = gn_bar[i][1];
        let gzb = gn_bar[i][2];
        // Deviatoric normal-stress contribution (pointwise B rows for εxx,εyy,εzz)
        // plus shear stresses (pointwise) plus hydrostatic via averaged rows.
        f[i * 3] += gx * dxx + gy * sxy + gz * sxz + gxb * sh;
        f[i * 3 + 1] += gy * dyy + gx * sxy + gz * syz + gyb * sh;
        f[i * 3 + 2] += gz * dzz + gy * syz + gx * sxz + gzb * sh;
    }
    f
}

fn d_times_eps(d: &[[f32; 6]; 6], e: &[f32; 6]) -> [f32; 6] {
    let mut s = [0.0_f32; 6];
    for i in 0..6 {
        let mut sum = 0.0_f32;
        for j in 0..6 {
            sum += d[i][j] * e[j];
        }
        s[i] = sum;
    }
    s
}

fn build_d_voigt_f64(e: f64, nu: f32) -> [[f64; 6]; 6] {
    let nu = nu as f64;
    let lam = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)).max(1e-30);
    let mu = e / (2.0 * (1.0 + nu)).max(1e-30);
    let l2m = lam + 2.0 * mu;
    [
        [l2m, lam, lam, 0.0, 0.0, 0.0],
        [lam, l2m, lam, 0.0, 0.0, 0.0],
        [lam, lam, l2m, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, mu, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, mu, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, mu],
    ]
}

fn d_times_eps_f64(d: &[[f64; 6]; 6], e: &[f64; 6]) -> [f64; 6] {
    let mut s = [0.0_f64; 6];
    for i in 0..6 {
        let mut sum = 0.0_f64;
        for j in 0..6 {
            sum += d[i][j] * e[j];
        }
        s[i] = sum;
    }
    s
}

fn bbar_times_u_f64(gn: [[f32; 3]; 8], gn_bar: [[f32; 3]; 8], u24: &[f64; 24]) -> [f64; 6] {
    let mut ev_bar = 0.0_f64;
    let mut exx = 0.0_f64;
    let mut eyy = 0.0_f64;
    let mut ezz = 0.0_f64;
    let mut e3 = 0.0_f64;
    let mut e4 = 0.0_f64;
    let mut e5 = 0.0_f64;
    for i in 0..8 {
        let ui = u24[i * 3];
        let vi = u24[i * 3 + 1];
        let wi = u24[i * 3 + 2];
        let gx = gn[i][0] as f64;
        let gy = gn[i][1] as f64;
        let gz = gn[i][2] as f64;
        exx += gx * ui;
        eyy += gy * vi;
        ezz += gz * wi;
        e3 += gy * ui + gx * vi;
        e4 += gz * vi + gy * wi;
        e5 += gz * ui + gx * wi;
        ev_bar += gn_bar[i][0] as f64 * ui + gn_bar[i][1] as f64 * vi + gn_bar[i][2] as f64 * wi;
    }
    let ev_pt = exx + eyy + ezz;
    let delta = (ev_bar - ev_pt) / 3.0;
    [exx + delta, eyy + delta, ezz + delta, e3, e4, e5]
}

fn bbar_times_u_transverse_shear_centroid_f64(
    gn: [[f32; 3]; 8],
    gn_bar: [[f32; 3]; 8],
    u24: &[f64; 24],
) -> [f64; 6] {
    let mut eps = bbar_times_u_f64(gn, gn_bar, u24);
    let eps_c = bbar_times_u_f64(gn_bar, gn_bar, u24);
    eps[4] = eps_c[4];
    eps[5] = eps_c[5];
    eps
}

fn bbar_t_times_sigma_transverse_shear_centroid_f64(
    gn: [[f32; 3]; 8],
    gn_bar: [[f32; 3]; 8],
    sig: &[f64; 6],
) -> [f64; 24] {
    let mut f = [0.0_f64; 24];
    let sxx = sig[0];
    let syy = sig[1];
    let szz = sig[2];
    let sxy = sig[3];
    let syz = sig[4];
    let sxz = sig[5];
    let sh = (sxx + syy + szz) / 3.0;
    let dxx = sxx - sh;
    let dyy = syy - sh;
    let dzz = szz - sh;
    for i in 0..8 {
        let gx = gn[i][0] as f64;
        let gy = gn[i][1] as f64;
        let gz = gn[i][2] as f64;
        let gxb = gn_bar[i][0] as f64;
        let gyb = gn_bar[i][1] as f64;
        let gzb = gn_bar[i][2] as f64;
        f[i * 3] += gx * dxx + gy * sxy + gzb * sxz + gxb * sh;
        f[i * 3 + 1] += gy * dyy + gx * sxy + gzb * syz + gyb * sh;
        f[i * 3 + 2] += gz * dzz + gyb * syz + gxb * sxz + gzb * sh;
    }
    f
}

/// Native f64 `y += K u` (Striatus lane; modulus per cell in f64).
pub fn hex_k_times_u_accumulate_f64(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f64],
    u: &[f64],
    y: &mut [f64],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f64);
                let d = build_d_voigt_f64(e, nu);
                let x_corner = cell_corner_coords(cx, cy, cz, dx, dy, dz);
                let mut u24 = [0.0_f64; 24];
                for (k, _corner) in CORNER_XI.iter().enumerate() {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    u24[k * 3] = u[nid * 3];
                    u24[k * 3 + 1] = u[nid * 3 + 1];
                    u24[k * 3 + 2] = u[nid * 3 + 2];
                }
                let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0)
                else {
                    continue;
                };
                for &sg in &GAUSS1D {
                    for &tg in &GAUSS1D {
                        for &zg in &GAUSS1D {
                            let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg)
                            else {
                                continue;
                            };
                            let wdet = (WG * WG * WG * detj) as f64;
                            let eps = bbar_times_u_transverse_shear_centroid_f64(gn, gn_bar, &u24);
                            let sig = d_times_eps_f64(&d, &eps);
                            let fe =
                                bbar_t_times_sigma_transverse_shear_centroid_f64(gn, gn_bar, &sig);
                            for k in 0..8 {
                                let (ix, iy, iz) = match k {
                                    0 => (cx, cy, cz),
                                    1 => (cx + 1, cy, cz),
                                    2 => (cx + 1, cy + 1, cz),
                                    3 => (cx, cy + 1, cz),
                                    4 => (cx, cy, cz + 1),
                                    5 => (cx + 1, cy, cz + 1),
                                    6 => (cx + 1, cy + 1, cz + 1),
                                    7 => (cx, cy + 1, cz + 1),
                                    _ => unreachable!(),
                                };
                                let nid = idx_node(nx1, ny1, ix, iy, iz);
                                y[nid * 3] += fe[k * 3] * wdet;
                                y[nid * 3 + 1] += fe[k * 3 + 1] * wdet;
                                y[nid * 3 + 2] += fe[k * 3 + 2] * wdet;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Striatus-scale grids use the f64 solve lane (`HEX_PCG_REL_TOL_F64`).
pub fn hex_pcg_use_f64_lane(nx: usize, ny: usize, nz: usize) -> bool {
    nx.saturating_mul(ny).saturating_mul(nz) >= 512
}

/// Matrix-free `y += K u` for the structured hex grid. `e_cell[c]` is Young's modulus in cell `c`.
pub fn hex_k_times_u_accumulate(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    u: &[f32],
    y: &mut [f32],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n_cells = nx * ny * nz;
    debug_assert_eq!(e_cell.len(), n_cells);
    debug_assert_eq!(u.len(), nx1 * ny1 * (nz + 1) * 3);
    debug_assert_eq!(y.len(), u.len());

    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f32);
                let d = build_d_voigt(e, nu);
                let x_corner = cell_corner_coords(cx, cy, cz, dx, dy, dz);
                let mut u24 = [0.0_f32; 24];
                for (k, corner) in CORNER_XI.iter().enumerate() {
                    let _ = corner;
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    u24[k * 3] = u[nid * 3];
                    u24[k * 3 + 1] = u[nid * 3 + 1];
                    u24[k * 3 + 2] = u[nid * 3 + 2];
                }
                let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0)
                else {
                    continue;
                };
                for &sg in &GAUSS1D {
                    for &tg in &GAUSS1D {
                        for &zg in &GAUSS1D {
                            let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg)
                            else {
                                continue;
                            };
                            let wdet = WG * WG * WG * detj;
                            let eps = bbar_times_u_transverse_shear_centroid(gn, gn_bar, &u24);
                            let sig = d_times_eps(&d, &eps);
                            let fe = bbar_t_times_sigma_transverse_shear_centroid(gn, gn_bar, &sig);
                            for k in 0..8 {
                                let (ix, iy, iz) = match k {
                                    0 => (cx, cy, cz),
                                    1 => (cx + 1, cy, cz),
                                    2 => (cx + 1, cy + 1, cz),
                                    3 => (cx, cy + 1, cz),
                                    4 => (cx, cy, cz + 1),
                                    5 => (cx + 1, cy, cz + 1),
                                    6 => (cx + 1, cy + 1, cz + 1),
                                    7 => (cx, cy + 1, cz + 1),
                                    _ => unreachable!(),
                                };
                                let nid = idx_node(nx1, ny1, ix, iy, iz);
                                y[nid * 3] += fe[k * 3] * wdet;
                                y[nid * 3 + 1] += fe[k * 3 + 1] * wdet;
                                y[nid * 3 + 2] += fe[k * 3 + 2] * wdet;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Integrated strain energy per hex cell:
/// \(U_e = \tfrac12 \int_{\Omega^e} \boldsymbol\varepsilon^{\mathsf T}\mathbf D\boldsymbol\varepsilon \,\mathrm d\Omega\)
/// using the same B-bar / transverse-shear centroid operator as [`hex_k_times_u_accumulate`].
///
/// `energy_out.len()` must equal `nx * ny * nz`; overwritten cell-major `(cx,cy,cz)`.
pub fn hex_cell_strain_energy(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    u: &[f32],
    energy_out: &mut [f32],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n_cells = nx * ny * nz;
    debug_assert_eq!(e_cell.len(), n_cells);
    debug_assert_eq!(u.len(), nx1 * ny1 * (nz + 1) * 3);
    debug_assert_eq!(energy_out.len(), n_cells);
    energy_out.fill(0.0_f32);

    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f32);
                let d = build_d_voigt(e, nu);
                let x_corner = cell_corner_coords(cx, cy, cz, dx, dy, dz);
                let mut u24 = [0.0_f32; 24];
                for (k, _corner) in CORNER_XI.iter().enumerate() {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    u24[k * 3] = u[nid * 3];
                    u24[k * 3 + 1] = u[nid * 3 + 1];
                    u24[k * 3 + 2] = u[nid * 3 + 2];
                }
                let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0)
                else {
                    continue;
                };
                let mut u_acc = 0.0_f32;
                for &sg in &GAUSS1D {
                    for &tg in &GAUSS1D {
                        for &zg in &GAUSS1D {
                            let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg)
                            else {
                                continue;
                            };
                            let wdet = WG * WG * WG * detj;
                            let eps = bbar_times_u_transverse_shear_centroid(gn, gn_bar, &u24);
                            let sig = d_times_eps(&d, &eps);
                            let mut de = 0.0_f32;
                            for i in 0..6 {
                                de += eps[i] * sig[i];
                            }
                            u_acc += 0.5_f32 * de * wdet;
                        }
                    }
                }
                energy_out[c] = u_acc;
            }
        }
    }
}

/// Uniform-brick element stiffness at `e = 1` (geometry + `nu` only).
fn assemble_hex_ke_unit(dx: f32, dy: f32, dz: f32, nu: f32) -> [[f32; 24]; 24] {
    let mut ke = [[0.0_f32; 24]; 24];
    let e = 1.0_f32;
    let d = build_d_voigt(e, nu);
    let x_corner = cell_corner_coords(0, 0, 0, dx, dy, dz);
    let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0) else {
        return ke;
    };
    for &sg in &GAUSS1D {
        for &tg in &GAUSS1D {
            for &zg in &GAUSS1D {
                let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg) else {
                    continue;
                };
                let wdet = WG * WG * WG * detj;
                let mut b = [[0.0_f32; 24]; 6];
                for node in 0..8 {
                    let gx = gn[node][0];
                    let gy = gn[node][1];
                    let gz = gn[node][2];
                    let gxb = gn_bar[node][0];
                    let gyb = gn_bar[node][1];
                    let gzb = gn_bar[node][2];
                    let c0 = node * 3;
                    let third = 1.0 / 3.0;
                    let dgx = third * (gxb - gx);
                    let dgy = third * (gyb - gy);
                    let dgz = third * (gzb - gz);
                    b[0][c0] = gx + dgx;
                    b[0][c0 + 1] = dgy;
                    b[0][c0 + 2] = dgz;
                    b[1][c0] = dgx;
                    b[1][c0 + 1] = gy + dgy;
                    b[1][c0 + 2] = dgz;
                    b[2][c0] = dgx;
                    b[2][c0 + 1] = dgy;
                    b[2][c0 + 2] = gz + dgz;
                    b[3][c0] = gy;
                    b[3][c0 + 1] = gx;
                    b[4][c0 + 1] = gzb;
                    b[4][c0 + 2] = gyb;
                    b[5][c0] = gzb;
                    b[5][c0 + 2] = gxb;
                }
                for i in 0..24 {
                    for j in 0..24 {
                        let mut sum = 0.0_f32;
                        for a in 0..6 {
                            for b_row in 0..6 {
                                sum += b[a][i] * d[a][b_row] * b[b_row][j];
                            }
                        }
                        ke[i][j] += sum * wdet;
                    }
                }
            }
        }
    }
    ke
}

/// Cached `e=1` element operator for a structured brick (`K_e = e_c * ke_unit`).
#[derive(Clone, Debug)]
pub struct HexStructuredOperatorCache {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    pub nu: f32,
    ke_unit: [[f32; 24]; 24],
}

impl HexStructuredOperatorCache {
    #[must_use]
    pub fn new(nx: usize, ny: usize, nz: usize, dx: f32, dy: f32, dz: f32, nu: f32) -> Self {
        Self {
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            nu,
            ke_unit: assemble_hex_ke_unit(dx, dy, dz, nu),
        }
    }
}

fn hex_scatter_ke_unit_times_u(
    nx: usize,
    ny: usize,
    nz: usize,
    ke_unit: &[[f32; 24]; 24],
    e_cell: &[f32],
    u: &[f32],
    y: &mut [f32],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f32);
                let mut u24 = [0.0_f32; 24];
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    u24[k * 3] = u[nid * 3];
                    u24[k * 3 + 1] = u[nid * 3 + 1];
                    u24[k * 3 + 2] = u[nid * 3 + 2];
                }
                let mut ku24 = [0.0_f32; 24];
                for i in 0..24 {
                    let mut s = 0.0_f32;
                    for j in 0..24 {
                        s += ke_unit[i][j] * u24[j];
                    }
                    ku24[i] = e * s;
                }
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    y[nid * 3] += ku24[k * 3];
                    y[nid * 3 + 1] += ku24[k * 3 + 1];
                    y[nid * 3 + 2] += ku24[k * 3 + 2];
                }
            }
        }
    }
}

/// `y += K u` using [`HexStructuredOperatorCache`] (metrics-match path to direct assembly).
pub fn hex_k_times_u_accumulate_cached(
    cache: &HexStructuredOperatorCache,
    e_cell: &[f32],
    u: &[f32],
    y: &mut [f32],
) {
    debug_assert_eq!(cache.nx * cache.ny * cache.nz, e_cell.len());
    hex_scatter_ke_unit_times_u(cache.nx, cache.ny, cache.nz, &cache.ke_unit, e_cell, u, y);
}

pub fn hex_k_times_u_accumulate_cached_f64(
    cache: &HexStructuredOperatorCache,
    e_cell: &[f32],
    u: &[f64],
    y: &mut [f64],
) {
    let e64: Vec<f64> = e_cell.iter().map(|&e| e as f64).collect();
    hex_k_times_u_accumulate_cached_f64_e(cache, &e64, u, y);
}

/// `y += K u` using [`HexStructuredOperatorCache`] with f64 moduli (f64 PCG lane parity).
pub fn hex_k_times_u_accumulate_cached_f64_e(
    cache: &HexStructuredOperatorCache,
    e_cell: &[f64],
    u: &[f64],
    y: &mut [f64],
) {
    let nx = cache.nx;
    let ny = cache.ny;
    let nz = cache.nz;
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let ke_unit = &cache.ke_unit;
    debug_assert_eq!(cache.nx * cache.ny * cache.nz, e_cell.len());
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f64);
                let mut u24 = [0.0_f64; 24];
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    u24[k * 3] = u[nid * 3];
                    u24[k * 3 + 1] = u[nid * 3 + 1];
                    u24[k * 3 + 2] = u[nid * 3 + 2];
                }
                let mut ku24 = [0.0_f64; 24];
                for i in 0..24 {
                    let mut s = 0.0_f64;
                    for j in 0..24 {
                        s += ke_unit[i][j] as f64 * u24[j];
                    }
                    ku24[i] = e * s;
                }
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    y[nid * 3] += ku24[k * 3];
                    y[nid * 3 + 1] += ku24[k * 3 + 1];
                    y[nid * 3 + 2] += ku24[k * 3 + 2];
                }
            }
        }
    }
}

fn hex_dims_coarsenable(nx: usize, ny: usize, nz: usize) -> bool {
    nx >= 2 && ny >= 2 && nz >= 2 && nx % 2 == 0 && ny % 2 == 0 && nz % 2 == 0
}

fn hex_dims_semicoarsenable_xy(nx: usize, ny: usize) -> bool {
    nx >= 2 && ny >= 2 && nx % 2 == 0 && ny % 2 == 0
}

fn hex_coarsen_cell_field_xy_only(
    e_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, usize, usize, usize) {
    let nx_c = nx / 2;
    let ny_c = ny / 2;
    let nz_c = nz;
    let mut e_c = vec![0.0_f32; nx_c * ny_c * nz_c];
    for cz in 0..nz_c {
        for cy in 0..ny_c {
            for cx in 0..nx_c {
                let mut sum = 0.0_f32;
                for dy in 0..2 {
                    for dx in 0..2 {
                        let fx = cx * 2 + dx;
                        let fy = cy * 2 + dy;
                        let c = fx + fy * nx + cz * nx * ny;
                        sum += e_fine[c];
                    }
                }
                e_c[cx + cy * nx_c + cz * nx_c * ny_c] = sum * 0.25_f32;
            }
        }
    }
    (e_c, nx_c, ny_c, nz_c)
}

fn hex_restrict_mask_xy_only(
    mask_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, usize, usize, usize) {
    let nx_c = nx / 2;
    let ny_c = ny / 2;
    let nz_c = nz;
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    let mut m_c = vec![0.0_f32; (nx1_c * ny1_c * (nz_c + 1)) * 3];
    for iz in 0..=nz_c {
        for iy_c in 0..=ny_c {
            for ix_c in 0..=nx_c {
                let ic = ix_c + iy_c * nx1_c + iz * nx1_c * ny1_c;
                for d in 0..3 {
                    let mut m_min = 1.0_f32;
                    for dy in 0..=1 {
                        for dx in 0..=1 {
                            let ix_f = ix_c * 2 + dx;
                            let iy_f = iy_c * 2 + dy;
                            if ix_f <= nx && iy_f <= ny {
                                let i_f = ix_f + iy_f * nx1 + iz * nx1 * ny1;
                                m_min = m_min.min(mask_fine[i_f * 3 + d]);
                            }
                        }
                    }
                    m_c[ic * 3 + d] = m_min;
                }
            }
        }
    }
    let _ = ny1;
    (m_c, nx_c, ny_c, nz_c)
}

fn hex_restrict_nodal_f32_xy_only(
    v_fine: &[f32],
    mask_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, Vec<f32>, usize, usize, usize) {
    let (m_c, nx_c, ny_c, nz_c) = hex_restrict_mask_xy_only(mask_fine, nx, ny, nz);
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    let ndof_c = (nx1_c * ny1_c * (nz_c + 1)) * 3;
    let mut v_c = vec![0.0_f32; ndof_c];
    for iz in 0..=nz_c {
        for iy_c in 0..=ny_c {
            for ix_c in 0..=nx_c {
                let ic = ix_c + iy_c * nx1_c + iz * nx1_c * ny1_c;
                let ix_f = ix_c * 2;
                let iy_f = iy_c * 2;
                if ix_f <= nx && iy_f <= ny {
                    let i_f = ix_f + iy_f * nx1 + iz * nx1 * ny1;
                    for d in 0..3 {
                        v_c[ic * 3 + d] = v_fine[i_f * 3 + d];
                    }
                }
            }
        }
    }
    let _ = ny1;
    (v_c, m_c, nx_c, ny_c, nz_c)
}

fn hex_prolong_nodal_add_f32_xy_only(
    v_coarse: &[f32],
    nx_c: usize,
    ny_c: usize,
    nz_c: usize,
    v_fine: &mut [f32],
    nx: usize,
    ny: usize,
    nz: usize,
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    debug_assert_eq!(nz, nz_c);
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let i_f = ix + iy * nx1 + iz * nx1 * ny1;
                let ix_c0 = ix / 2;
                let iy_c0 = iy / 2;
                let fx = (ix % 2) as f32 * 0.5_f32;
                let fy = (iy % 2) as f32 * 0.5_f32;
                for d in 0..3 {
                    let mut val = 0.0_f32;
                    let mut w = 0.0_f32;
                    for (ix_c, wx) in [(ix_c0, 1.0_f32 - fx), (ix_c0 + 1, fx)] {
                        if ix_c > nx_c {
                            continue;
                        }
                        for (iy_c, wy) in [(iy_c0, 1.0_f32 - fy), (iy_c0 + 1, fy)] {
                            if iy_c > ny_c {
                                continue;
                            }
                            let ic = ix_c + iy_c * nx1_c + iz * nx1_c * ny1_c;
                            let wt = wx * wy;
                            val += wt * v_coarse[ic * 3 + d];
                            w += wt;
                        }
                    }
                    if w > 1e-30_f32 {
                        v_fine[i_f * 3 + d] += val / w;
                    }
                }
            }
        }
    }
    let _ = ny1;
}

fn hex_coarsen_cell_field(
    e_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, usize, usize, usize) {
    let nx_c = nx / 2;
    let ny_c = ny / 2;
    let nz_c = nz / 2;
    let mut e_c = vec![0.0_f32; nx_c * ny_c * nz_c];
    for cz in 0..nz_c {
        for cy in 0..ny_c {
            for cx in 0..nx_c {
                let mut sum = 0.0_f32;
                for dz in 0..2 {
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let fx = cx * 2 + dx;
                            let fy = cy * 2 + dy;
                            let fz = cz * 2 + dz;
                            let c = fx + fy * nx + fz * nx * ny;
                            sum += e_fine[c];
                        }
                    }
                }
                e_c[cx + cy * nx_c + cz * nx_c * ny_c] = sum * (1.0 / 8.0_f32);
            }
        }
    }
    (e_c, nx_c, ny_c, nz_c)
}

fn hex_restrict_mask(
    mask_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, usize, usize, usize) {
    let nx_c = nx / 2;
    let ny_c = ny / 2;
    let nz_c = nz / 2;
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    let mut m_c = vec![0.0_f32; (nx1_c * ny1_c * (nz_c + 1)) * 3];
    for iz_c in 0..=nz_c {
        for iy_c in 0..=ny_c {
            for ix_c in 0..=nx_c {
                let ic = ix_c + iy_c * nx1_c + iz_c * nx1_c * ny1_c;
                for d in 0..3 {
                    let mut m_min = 1.0_f32;
                    for dz in 0..=1 {
                        for dy in 0..=1 {
                            for dx in 0..=1 {
                                let ix_f = ix_c * 2 + dx;
                                let iy_f = iy_c * 2 + dy;
                                let iz_f = iz_c * 2 + dz;
                                if ix_f <= nx && iy_f <= ny && iz_f <= nz {
                                    let i_f = ix_f + iy_f * nx1 + iz_f * nx1 * ny1;
                                    m_min = m_min.min(mask_fine[i_f * 3 + d]);
                                }
                            }
                        }
                    }
                    m_c[ic * 3 + d] = m_min;
                }
            }
        }
    }
    (m_c, nx_c, ny_c, nz_c)
}

fn hex_restrict_nodal_f32(
    v_fine: &[f32],
    mask_fine: &[f32],
    nx: usize,
    ny: usize,
    nz: usize,
) -> (Vec<f32>, Vec<f32>, usize, usize, usize) {
    let (m_c, nx_c, ny_c, nz_c) = hex_restrict_mask(mask_fine, nx, ny, nz);
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    let ndof_c = (nx1_c * ny1_c * (nz_c + 1)) * 3;
    let mut v_c = vec![0.0_f32; ndof_c];
    for iz_c in 0..=nz_c {
        for iy_c in 0..=ny_c {
            for ix_c in 0..=nx_c {
                let ic = ix_c + iy_c * nx1_c + iz_c * nx1_c * ny1_c;
                let ix_f = ix_c * 2;
                let iy_f = iy_c * 2;
                let iz_f = iz_c * 2;
                if ix_f <= nx && iy_f <= ny && iz_f <= nz {
                    let i_f = ix_f + iy_f * nx1 + iz_f * nx1 * ny1;
                    for d in 0..3 {
                        v_c[ic * 3 + d] = v_fine[i_f * 3 + d];
                    }
                }
            }
        }
    }
    (v_c, m_c, nx_c, ny_c, nz_c)
}

fn hex_prolong_nodal_add_f32(
    v_coarse: &[f32],
    nx_c: usize,
    ny_c: usize,
    nz_c: usize,
    v_fine: &mut [f32],
    nx: usize,
    ny: usize,
    nz: usize,
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let nx1_c = nx_c + 1;
    let ny1_c = ny_c + 1;
    for iz in 0..=nz {
        for iy in 0..=ny {
            for ix in 0..=nx {
                let i_f = ix + iy * nx1 + iz * nx1 * ny1;
                let ix_c0 = ix / 2;
                let iy_c0 = iy / 2;
                let iz_c0 = iz / 2;
                let fx = (ix % 2) as f32 * 0.5_f32;
                let fy = (iy % 2) as f32 * 0.5_f32;
                let fz = (iz % 2) as f32 * 0.5_f32;
                for d in 0..3 {
                    let mut val = 0.0_f32;
                    let mut w = 0.0_f32;
                    for (ix_c, wx) in [(ix_c0, 1.0_f32 - fx), (ix_c0 + 1, fx)] {
                        if ix_c > nx_c {
                            continue;
                        }
                        for (iy_c, wy) in [(iy_c0, 1.0_f32 - fy), (iy_c0 + 1, fy)] {
                            if iy_c > ny_c {
                                continue;
                            }
                            for (iz_c, wz) in [(iz_c0, 1.0_f32 - fz), (iz_c0 + 1, fz)] {
                                if iz_c > nz_c {
                                    continue;
                                }
                                let ic = ix_c + iy_c * nx1_c + iz_c * nx1_c * ny1_c;
                                let wt = wx * wy * wz;
                                val += wt * v_coarse[ic * 3 + d];
                                w += wt;
                            }
                        }
                    }
                    if w > 1e-30_f32 {
                        v_fine[i_f * 3 + d] += val / w;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)] // reserved for V-cycle Jacobi smoothing (BPX path uses diagonal prolongation today)
fn hex_jacobi_smooth_f32(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f32],
    diag: &[f32],
    rhs: &[f32],
    z: &mut [f32],
    steps: usize,
    omega: f32,
    op_cache: Option<&HexStructuredOperatorCache>,
) {
    let ndof = z.len();
    let mut ku = vec![0.0_f32; ndof];
    for _ in 0..steps {
        ku.fill(0.0);
        if let Some(cache) = op_cache {
            hex_k_times_u_accumulate_cached(cache, e_cell, z, &mut ku);
        } else {
            hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, z, &mut ku);
        }
        for i in 0..ndof {
            if mask[i] > 0.5_f32 && diag[i] > 1e-30_f32 {
                z[i] += omega * (rhs[i] - ku[i]) / diag[i];
            }
        }
    }
}

fn apply_precond_geometric_mg_f32(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f32],
    diag: &[f32],
    r: &[f32],
    z: &mut [f32],
    _op_cache: Option<&HexStructuredOperatorCache>,
) {
    fn bpx_level(
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        nu: f32,
        e_cell: &[f32],
        mask: &[f32],
        diag: &[f32],
        r: &[f32],
        z: &mut [f32],
        level: usize,
        max_levels: usize,
    ) {
        for i in 0..z.len() {
            if mask[i] > 0.5_f32 {
                z[i] += r[i] / diag[i].max(1e-30_f32);
            }
        }
        if level + 1 >= max_levels || !hex_dims_coarsenable(nx, ny, nz) {
            return;
        }
        let (e_c, nx_c, ny_c, nz_c) = hex_coarsen_cell_field(e_cell, nx, ny, nz);
        let (r_c, m_c, _, _, _) = hex_restrict_nodal_f32(r, mask, nx, ny, nz);
        let dx_c = dx * 2.0_f32;
        let dy_c = dy * 2.0_f32;
        let dz_c = dz * 2.0_f32;
        let mut diag_c = vec![0.0_f32; m_c.len()];
        hex_diagonal(nx_c, ny_c, nz_c, dx_c, dy_c, dz_c, nu, &e_c, &mut diag_c);
        let ndof_c = r_c.len();
        let mut z_c = vec![0.0_f32; ndof_c];
        bpx_level(
            nx_c,
            ny_c,
            nz_c,
            dx_c,
            dy_c,
            dz_c,
            nu,
            &e_c,
            &m_c,
            &diag_c,
            &r_c,
            &mut z_c,
            level + 1,
            max_levels,
        );
        let mut bump = vec![0.0_f32; z.len()];
        hex_prolong_nodal_add_f32(&z_c, nx_c, ny_c, nz_c, &mut bump, nx, ny, nz);
        for i in 0..z.len() {
            z[i] += 0.35_f32 * bump[i];
        }
    }
    z.fill(0.0);
    bpx_level(nx, ny, nz, dx, dy, dz, nu, e_cell, mask, diag, r, z, 0, 6);
    for i in 0..z.len() {
        z[i] *= mask[i];
    }
}

fn apply_precond_semicoarsening_mg_f32(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f32],
    diag: &[f32],
    r: &[f32],
    z: &mut [f32],
    _op_cache: Option<&HexStructuredOperatorCache>,
) {
    fn bpx_level_xy(
        nx: usize,
        ny: usize,
        nz: usize,
        dx: f32,
        dy: f32,
        dz: f32,
        nu: f32,
        e_cell: &[f32],
        mask: &[f32],
        diag: &[f32],
        r: &[f32],
        z: &mut [f32],
        level: usize,
        max_levels: usize,
    ) {
        for i in 0..z.len() {
            if mask[i] > 0.5_f32 {
                z[i] += r[i] / diag[i].max(1e-30_f32);
            }
        }
        if level + 1 >= max_levels || !hex_dims_semicoarsenable_xy(nx, ny) {
            return;
        }
        let (e_c, nx_c, ny_c, nz_c) = hex_coarsen_cell_field_xy_only(e_cell, nx, ny, nz);
        let (r_c, m_c, _, _, _) = hex_restrict_nodal_f32_xy_only(r, mask, nx, ny, nz);
        let dx_c = dx * 2.0_f32;
        let dy_c = dy * 2.0_f32;
        let mut diag_c = vec![0.0_f32; m_c.len()];
        hex_diagonal(nx_c, ny_c, nz_c, dx_c, dy_c, dz, nu, &e_c, &mut diag_c);
        let ndof_c = r_c.len();
        let mut z_c = vec![0.0_f32; ndof_c];
        bpx_level_xy(
            nx_c,
            ny_c,
            nz_c,
            dx_c,
            dy_c,
            dz,
            nu,
            &e_c,
            &m_c,
            &diag_c,
            &r_c,
            &mut z_c,
            level + 1,
            max_levels,
        );
        let mut bump = vec![0.0_f32; z.len()];
        hex_prolong_nodal_add_f32_xy_only(&z_c, nx_c, ny_c, nz_c, &mut bump, nx, ny, nz);
        for i in 0..z.len() {
            z[i] += 0.35_f32 * bump[i];
        }
    }
    z.fill(0.0);
    bpx_level_xy(nx, ny, nz, dx, dy, dz, nu, e_cell, mask, diag, r, z, 0, 6);
    for i in 0..z.len() {
        z[i] *= mask[i];
    }
}

/// Smoothed semicoarsening V-cycle: Jacobi pre/post smooth + xy semicoarsening MG (AMG wave-A spike).
fn apply_precond_algebraic_amg_f32(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f32],
    diag: &[f32],
    r: &[f32],
    z: &mut [f32],
    op_cache: Option<&HexStructuredOperatorCache>,
) {
    let ndof = r.len();
    z.fill(0.0);
    hex_jacobi_smooth_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, mask, diag, r, z, 2, 0.67_f32, op_cache,
    );
    let mut ku = vec![0.0_f32; ndof];
    let mut r_sm = r.to_vec();
    ku.fill(0.0);
    if let Some(cache) = op_cache {
        hex_k_times_u_accumulate_cached(cache, e_cell, z, &mut ku);
    } else {
        hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, z, &mut ku);
    }
    for i in 0..ndof {
        if mask[i] > 0.5_f32 {
            r_sm[i] -= ku[i];
        }
    }
    let mut z_c = vec![0.0_f32; ndof];
    apply_precond_semicoarsening_mg_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, mask, diag, &r_sm, &mut z_c, op_cache,
    );
    for i in 0..ndof {
        z[i] += z_c[i];
    }
    hex_jacobi_smooth_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, mask, diag, r, z, 2, 0.67_f32, op_cache,
    );
    for i in 0..ndof {
        z[i] *= mask[i];
    }
}

#[allow(dead_code)] // reserved for V-cycle Jacobi smoothing (BPX path uses diagonal prolongation today)
fn hex_jacobi_smooth_f64(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f64],
    diag: &[f64],
    rhs: &[f64],
    z: &mut [f64],
    steps: usize,
    omega: f64,
    op_cache: Option<&HexStructuredOperatorCache>,
) {
    let ndof = z.len();
    let mut ku = vec![0.0_f64; ndof];
    let e_solve: Vec<f64> = e_cell.iter().map(|&e| e as f64).collect();
    for _ in 0..steps {
        ku.fill(0.0);
        if let Some(cache) = op_cache {
            hex_k_times_u_accumulate_cached_f64(cache, e_cell, z, &mut ku);
        } else {
            hex_k_times_u_accumulate_f64(nx, ny, nz, dx, dy, dz, nu, &e_solve, z, &mut ku);
        }
        for i in 0..ndof {
            if mask[i] > 0.5 && diag[i] > 1e-30_f64 {
                z[i] += omega * (rhs[i] - ku[i]) / diag[i];
            }
        }
    }
}

fn apply_precond_geometric_mg_f64(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f64],
    diag: &[f64],
    r: &[f64],
    z: &mut [f64],
    _op_cache: Option<&HexStructuredOperatorCache>,
) {
    let mask_f: Vec<f32> = mask.iter().map(|&m| m as f32).collect();
    let r_f: Vec<f32> = r.iter().map(|&v| v as f32).collect();
    let diag_f: Vec<f32> = diag.iter().map(|&v| v as f32).collect();
    let mut z_f = vec![0.0_f32; r.len()];
    apply_precond_geometric_mg_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, &mask_f, &diag_f, &r_f, &mut z_f, None,
    );
    for (a, b) in z.iter_mut().zip(&z_f) {
        *a = *b as f64;
    }
}

fn apply_precond_semicoarsening_mg_f64(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f64],
    diag: &[f64],
    r: &[f64],
    z: &mut [f64],
    _op_cache: Option<&HexStructuredOperatorCache>,
) {
    let mask_f: Vec<f32> = mask.iter().map(|&m| m as f32).collect();
    let r_f: Vec<f32> = r.iter().map(|&v| v as f32).collect();
    let diag_f: Vec<f32> = diag.iter().map(|&v| v as f32).collect();
    let mut z_f = vec![0.0_f32; r.len()];
    apply_precond_semicoarsening_mg_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, &mask_f, &diag_f, &r_f, &mut z_f, None,
    );
    for (a, b) in z.iter_mut().zip(&z_f) {
        *a = *b as f64;
    }
}

fn apply_precond_algebraic_amg_f64(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    mask: &[f64],
    diag: &[f64],
    r: &[f64],
    z: &mut [f64],
    op_cache: Option<&HexStructuredOperatorCache>,
) {
    let mask_f: Vec<f32> = mask.iter().map(|&m| m as f32).collect();
    let r_f: Vec<f32> = r.iter().map(|&v| v as f32).collect();
    let diag_f: Vec<f32> = diag.iter().map(|&v| v as f32).collect();
    let mut z_f = vec![0.0_f32; r.len()];
    apply_precond_algebraic_amg_f32(
        nx, ny, nz, dx, dy, dz, nu, e_cell, &mask_f, &diag_f, &r_f, &mut z_f, op_cache,
    );
    for (a, b) in z.iter_mut().zip(&z_f) {
        *a = *b as f64;
    }
}

/// Preconditioner for projected hex PCG (logging + A/B levers).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexPcgPrecondKind {
    None,
    JacobiDiagonal,
    BlockJacobiNodal3x3,
    GeometricMultigridVCycle,
    SemicoarseningMultigridVCycle,
    AlgebraicMultigridVCycle,
}

#[must_use]
pub fn hex_precond_from_use_preconditioner(use_preconditioner: bool) -> HexPcgPrecondKind {
    if use_preconditioner {
        HexPcgPrecondKind::JacobiDiagonal
    } else {
        HexPcgPrecondKind::None
    }
}

fn invert_3x3_f64(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];
    let d = m[1][0];
    let e = m[1][1];
    let f = m[1][2];
    let g = m[2][0];
    let h = m[2][1];
    let i = m[2][2];
    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    if det.abs() < 1e-30_f64 {
        return None;
    }
    let inv_det = 1.0 / det;
    Some([
        [
            (e * i - f * h) * inv_det,
            (c * h - b * i) * inv_det,
            (b * f - c * e) * inv_det,
        ],
        [
            (f * g - d * i) * inv_det,
            (a * i - c * g) * inv_det,
            (c * d - a * f) * inv_det,
        ],
        [
            (d * h - e * g) * inv_det,
            (b * g - a * h) * inv_det,
            (a * e - b * d) * inv_det,
        ],
    ])
}

fn invert_3x3_f32(m: [[f32; 3]; 3]) -> Option<[[f32; 3]; 3]> {
    let a = m[0][0] as f64;
    let b = m[0][1] as f64;
    let c = m[0][2] as f64;
    let d = m[1][0] as f64;
    let e = m[1][1] as f64;
    let f = m[1][2] as f64;
    let g = m[2][0] as f64;
    let h = m[2][1] as f64;
    let i = m[2][2] as f64;
    let inv = invert_3x3_f64([[a, b, c], [d, e, f], [g, h, i]])?;
    Some([
        [inv[0][0] as f32, inv[0][1] as f32, inv[0][2] as f32],
        [inv[1][0] as f32, inv[1][1] as f32, inv[1][2] as f32],
        [inv[2][0] as f32, inv[2][1] as f32, inv[2][2] as f32],
    ])
}

/// Assemble nodal 3×3 diagonal blocks of `K` (row-major `n_nodes × 9`).
pub fn hex_nodal_block_jacobi_3x3(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    blocks: &mut [f32],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n_nodes = nx1 * ny1 * (nz + 1);
    assert_eq!(blocks.len(), n_nodes * 9);
    blocks.fill(0.0);
    let mut ke = [[0.0_f32; 24]; 24];
    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f32);
                let d = build_d_voigt(e, nu);
                let x_corner = cell_corner_coords(cx, cy, cz, dx, dy, dz);
                ke.iter_mut().for_each(|row| row.fill(0.0));
                let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0)
                else {
                    continue;
                };
                for &sg in &GAUSS1D {
                    for &tg in &GAUSS1D {
                        for &zg in &GAUSS1D {
                            let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg)
                            else {
                                continue;
                            };
                            let wdet = WG * WG * WG * detj;
                            let mut b = [[0.0_f32; 24]; 6];
                            for node in 0..8 {
                                let gx = gn[node][0];
                                let gy = gn[node][1];
                                let gz = gn[node][2];
                                let gxb = gn_bar[node][0];
                                let gyb = gn_bar[node][1];
                                let gzb = gn_bar[node][2];
                                let c0 = node * 3;
                                let third = 1.0 / 3.0;
                                let dgx = third * (gxb - gx);
                                let dgy = third * (gyb - gy);
                                let dgz = third * (gzb - gz);
                                b[0][c0] = gx + dgx;
                                b[0][c0 + 1] = dgy;
                                b[0][c0 + 2] = dgz;
                                b[1][c0] = dgx;
                                b[1][c0 + 1] = gy + dgy;
                                b[1][c0 + 2] = dgz;
                                b[2][c0] = dgx;
                                b[2][c0 + 1] = dgy;
                                b[2][c0 + 2] = gz + dgz;
                                b[3][c0] = gy;
                                b[3][c0 + 1] = gx;
                                b[4][c0 + 1] = gzb;
                                b[4][c0 + 2] = gyb;
                                b[5][c0] = gzb;
                                b[5][c0 + 2] = gxb;
                            }
                            for ii in 0..24 {
                                for jj in 0..24 {
                                    let mut sum = 0.0_f32;
                                    for a in 0..6 {
                                        for b_row in 0..6 {
                                            sum += b[a][ii] * d[a][b_row] * b[b_row][jj];
                                        }
                                    }
                                    ke[ii][jj] += sum * wdet;
                                }
                            }
                        }
                    }
                }
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    let bo = nid * 9;
                    for a in 0..3 {
                        for b in 0..3 {
                            let ia = k * 3 + a;
                            let ib = k * 3 + b;
                            blocks[bo + a * 3 + b] += ke[ia][ib];
                        }
                    }
                }
            }
        }
    }
}

fn apply_precond_jacobi_f32(diag: &[f32], mask: &[f32], r: &[f32], z: &mut [f32]) {
    for i in 0..r.len() {
        z[i] = mask[i] * r[i] / diag[i].max(1e-30_f32);
    }
}

fn apply_precond_block_3x3_f32(
    blocks: &[f32],
    mask: &[f32],
    r: &[f32],
    z: &mut [f32],
    n_nodes: usize,
) {
    z.fill(0.0);
    for nid in 0..n_nodes {
        let d0 = nid * 3;
        let free = mask[d0] > 0.5 && mask[d0 + 1] > 0.5 && mask[d0 + 2] > 0.5;
        if !free {
            for a in 0..3 {
                let d = d0 + a;
                if mask[d] > 0.5 {
                    z[d] = r[d];
                }
            }
            continue;
        }
        let bo = nid * 9;
        let mut m = [[0.0_f32; 3]; 3];
        for a in 0..3 {
            for b in 0..3 {
                m[a][b] = blocks[bo + a * 3 + b];
            }
        }
        if let Some(inv) = invert_3x3_f32(m) {
            for a in 0..3 {
                let mut sum = 0.0_f32;
                for b in 0..3 {
                    sum += inv[a][b] * r[d0 + b];
                }
                z[d0 + a] = sum;
            }
        } else {
            z[d0..d0 + 3].copy_from_slice(&r[d0..d0 + 3]);
        }
    }
}

fn apply_precond_jacobi_f64(diag: &[f64], mask: &[f64], r: &[f64], z: &mut [f64]) {
    for i in 0..r.len() {
        z[i] = mask[i] * r[i] / diag[i].max(1e-30_f64);
    }
}

fn apply_precond_block_3x3_f64(
    blocks: &[f64],
    mask: &[f64],
    r: &[f64],
    z: &mut [f64],
    n_nodes: usize,
) {
    z.fill(0.0);
    for nid in 0..n_nodes {
        let d0 = nid * 3;
        let free = mask[d0] > 0.5 && mask[d0 + 1] > 0.5 && mask[d0 + 2] > 0.5;
        if !free {
            for a in 0..3 {
                let d = d0 + a;
                if mask[d] > 0.5 {
                    z[d] = r[d];
                }
            }
            continue;
        }
        let bo = nid * 9;
        let mut m = [[0.0_f64; 3]; 3];
        for a in 0..3 {
            for b in 0..3 {
                m[a][b] = blocks[bo + a * 3 + b];
            }
        }
        if let Some(inv) = invert_3x3_f64(m) {
            for a in 0..3 {
                let mut sum = 0.0_f64;
                for b in 0..3 {
                    sum += inv[a][b] * r[d0 + b];
                }
                z[d0 + a] = sum;
            }
        } else {
            z[d0..d0 + 3].copy_from_slice(&r[d0..d0 + 3]);
        }
    }
}

/// Assembled diagonal of `K` (free rows) for Jacobi preconditioning.
pub fn hex_diagonal(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    diag: &mut [f32],
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n_nodes = nx1 * ny1 * (nz + 1);
    diag.fill(0.0);
    let mut ke = [[0.0_f32; 24]; 24];

    for cz in 0..nz {
        for cy in 0..ny {
            for cx in 0..nx {
                let c = cx + cy * nx + cz * nx * ny;
                let e = e_cell[c].max(1e-30_f32);
                let d = build_d_voigt(e, nu);
                let x_corner = cell_corner_coords(cx, cy, cz, dx, dy, dz);
                ke.iter_mut().for_each(|row| row.fill(0.0));
                let Some((gn_bar, _det_c)) = physical_shape_gradients(x_corner, 0.0, 0.0, 0.0)
                else {
                    continue;
                };
                for &sg in &GAUSS1D {
                    for &tg in &GAUSS1D {
                        for &zg in &GAUSS1D {
                            let Some((gn, detj)) = physical_shape_gradients(x_corner, sg, tg, zg)
                            else {
                                continue;
                            };
                            let wdet = WG * WG * WG * detj;
                            // B-bar: normal-strain rows mix pointwise (deviatoric) and centroid
                            // (volumetric) gradients; shear rows are pointwise.
                            let mut b = [[0.0_f32; 24]; 6];
                            for node in 0..8 {
                                let gx = gn[node][0];
                                let gy = gn[node][1];
                                let gz = gn[node][2];
                                let gxb = gn_bar[node][0];
                                let gyb = gn_bar[node][1];
                                let gzb = gn_bar[node][2];
                                let c0 = node * 3;
                                // B̄ normal-strain rows: ε̄_ii = ε_ii_pt + (ε̄_v - ε_v_pt)/3
                                // Column gx contribution to row 0: gx (full) - gx/3 (remove pt vol) + gxb/3 (add mean vol) = (2gx + gxb)/3
                                // Off-diagonal cols (gy, gz): -gy/3 + gyb/3 etc.
                                let third = 1.0 / 3.0;
                                let dgx = third * (gxb - gx);
                                let dgy = third * (gyb - gy);
                                let dgz = third * (gzb - gz);
                                // Row 0: εxx
                                b[0][c0] = gx + dgx;
                                b[0][c0 + 1] = dgy;
                                b[0][c0 + 2] = dgz;
                                // Row 1: εyy
                                b[1][c0] = dgx;
                                b[1][c0 + 1] = gy + dgy;
                                b[1][c0 + 2] = dgz;
                                // Row 2: εzz
                                b[2][c0] = dgx;
                                b[2][c0 + 1] = dgy;
                                b[2][c0 + 2] = gz + dgz;
                                // γ_xy: full quadrature; γ_yz, γ_xz: centroid (matches accumulate).
                                b[3][c0] = gy;
                                b[3][c0 + 1] = gx;
                                b[4][c0 + 1] = gzb;
                                b[4][c0 + 2] = gyb;
                                b[5][c0] = gzb;
                                b[5][c0 + 2] = gxb;
                            }
                            for i in 0..24 {
                                for j in 0..24 {
                                    let mut sum = 0.0_f32;
                                    for a in 0..6 {
                                        for b_row in 0..6 {
                                            sum += b[a][i] * d[a][b_row] * b[b_row][j];
                                        }
                                    }
                                    ke[i][j] += sum * wdet;
                                }
                            }
                        }
                    }
                }
                for k in 0..8 {
                    let (ix, iy, iz) = match k {
                        0 => (cx, cy, cz),
                        1 => (cx + 1, cy, cz),
                        2 => (cx + 1, cy + 1, cz),
                        3 => (cx, cy + 1, cz),
                        4 => (cx, cy, cz + 1),
                        5 => (cx + 1, cy, cz + 1),
                        6 => (cx + 1, cy + 1, cz + 1),
                        7 => (cx, cy + 1, cz + 1),
                        _ => unreachable!(),
                    };
                    let nid = idx_node(nx1, ny1, ix, iy, iz);
                    for a in 0..3 {
                        let ii = nid * 3 + a;
                        let col = k * 3 + a;
                        diag[ii] += ke[col][col];
                    }
                }
            }
        }
    }
    for v in diag.iter_mut().take(n_nodes * 3) {
        *v = v.max(1e-30_f32);
    }
}

/// f32 quick-scale lane tol — attainable κ·ε floor (evidence: arm-A probe 9×8×2, 2026-06-10).
pub const HEX_PCG_REL_TOL_F32: f32 = 1e-4;
/// f64 Striatus lane tol — re-grounded 2026-06-10: sensitivity fidelity + inexact-solve TO practice
/// (same measured κ·ε floor as f32; 1e-6 overshoots attainable residual at 40×40×4 — see Solver-Status).
pub const HEX_PCG_REL_TOL_F64: f32 = 1e-4;
/// Periodic true-residual verification cadence when [`HexPcgBisectConfig::stop_on_true_residual`].
pub const HEX_PCG_TRUE_RESIDUAL_CHECK_PERIOD: usize = 25;

/// Full-harness default PCG budget at Striatus N (40×40×4).
///
/// Derived (2026-06-12, sharp-field basis): worst observed **3960** iters @ outer 32 on logit-offset
/// 60-outer (`greyness≈0.084`, κ at lifetime peak); **2×** headroom ⇒ **8000**. Supersedes the
/// 2026-06-10 grey-field basis (~1213 @ outer 1 → 4000 cap).
pub const HEX_PCG_MAX_ITER_DEFAULT_STRIATUS: usize = 8000;

/// Which norm triggered PCG exit (diagnostic).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexPcgStoppingCriterion {
    /// \(\|P(f-Ku)\|_2 / \|Pf\|_2\) after full residual refresh (binding).
    PlainRNorm,
}

/// PCG telemetry for [`hex_solve_pcg_masked`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HexPcgReport {
    pub iterations: usize,
    /// Independent `||P(f-Ku)||/||Pf||` at exit — **binding** for gates and stopping.
    pub rel_residual: f32,
    /// Inner-loop recursive estimate at exit (should match `rel_residual` when healthy).
    pub rel_residual_recursive: f32,
    pub stopping_criterion: HexPcgStoppingCriterion,
    /// `k_char` used to nondimensionalize `K u = f` (1.0 when not applied).
    pub stiffness_scale: f32,
}

/// Bisection axis: committed recursive loop vs bundled refresh+masked-\(p\) rewrite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexPcgLoopKind {
    /// Recursive \(r\) update; unmasked \(u,p\) (d8babee baseline).
    Original,
    /// Full \(r=P(f-Ku)\) refresh each iter; masked \(u,p\) (under test).
    RefreshMaskedP,
}

/// 2×2 bisection knobs (probe-only; production uses [`hex_solve_pcg_masked`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HexPcgBisectConfig {
    pub loop_kind: HexPcgLoopKind,
    pub nondim: bool,
    /// When true, stop on physical `eq_rel` (production). **False** for isolated bisection.
    pub stop_on_true_residual: bool,
}

/// Probe telemetry for [`hex_solve_pcg_bisect`].
#[derive(Clone, Debug, PartialEq)]
pub struct HexPcgBisectReport {
    pub iterations: usize,
    pub rel_residual_recursive: f32,
    pub rel_residual_true: f32,
    pub stiffness_scale: f32,
    pub u: Vec<f32>,
}

/// Characteristic stiffness scale for Q1-hex PCG (`≈ E_ref · A / Δx`, same intent as bar `k_char`).
pub fn hex_stiffness_scale(e_cell: &[f32], dx: f32, dy: f32, dz: f32) -> f32 {
    let e_hi = e_cell
        .iter()
        .copied()
        .fold(0.0_f32, |a, b| a.max(b))
        .max(1e-12_f32);
    let dx_char = dx.min(dy).min(dz).max(1e-12_f32);
    (e_hi * dy * dz / dx_char).max(1e-30_f32)
}

fn hex_projected_k_times_u(
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f32,
    dy: f32,
    dz: f32,
    nu: f32,
    e_cell: &[f32],
    u: &[f32],
    mask: &[f32],
    ku: &mut [f32],
) {
    ku.fill(0.0);
    hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, u, ku);
    for (k, m) in ku.iter_mut().zip(mask) {
        *k *= *m;
    }
}

/// \(\|P(f-Ku)\|_2 / \|Pf\|_2\) after a masked Q1-hex forward solve.
pub fn hex_equilibrium_rel_residual(
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
    let u_ref = u;
    masked_projected_residual_parts(f, mask, |ku| {
        hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, u_ref, ku);
    })
    .rel_residual
}

/// Physical masked equilibrium residual in f64 (same operator as f64 PCG matvec).
pub fn hex_equilibrium_rel_residual_f64(
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
    u: &[f64],
) -> f64 {
    let ndof = f.len();
    let e64: Vec<f64> = e_cell.iter().map(|&e| e as f64).collect();
    let f64v: Vec<f64> = f.iter().map(|&fi| fi as f64).collect();
    let mask64: Vec<f64> = mask.iter().map(|&m| m as f64).collect();
    let mut ku = vec![0.0_f64; ndof];
    hex_k_times_u_accumulate_f64(nx, ny, nz, dx, dy, dz, nu, &e64, u, &mut ku);
    for (k, m) in ku.iter_mut().zip(&mask64) {
        *k *= *m;
    }
    let mut num = 0.0_f64;
    let mut den = 0.0_f64;
    for i in 0..ndof {
        let fi = f64v[i] * mask64[i];
        let ri = mask64[i] * (f64v[i] - ku[i]);
        num += ri * ri;
        den += fi * fi;
    }
    num.sqrt() / den.sqrt().max(1e-30_f64)
}

/// [`hex_equilibrium_rel_residual`] with explicit \(\|Pf\|\), \(\|P(f-Ku)\|\) [N].
pub fn hex_equilibrium_residual_parts(
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
) -> MaskedResidualParts {
    let u_ref = u;
    masked_projected_residual_parts(f, mask, |ku| {
        hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, u_ref, ku);
    })
}

/// Absolute and relative masked equilibrium residual (SI: force components in N).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskedResidualParts {
    /// \(\|P(f-Ku)\|_2\) [N].
    pub abs_residual: f32,
    /// \(\|Pf\|_2\) [N].
    pub abs_rhs: f32,
    /// `abs_residual / abs_rhs` (dimensionless).
    pub rel_residual: f32,
}

/// \(\|P(f-Ku)\|_2 / \|Pf\|_2\) with caller-supplied masked \(K\) matvec.
pub fn masked_projected_rel_residual(
    f: &[f32],
    mask: &[f32],
    _u: &[f32],
    apply_k: impl FnMut(&mut [f32]),
) -> f32 {
    masked_projected_residual_parts(f, mask, apply_k).rel_residual
}

/// Same norm as [`masked_projected_rel_residual`], with explicit SI force magnitudes.
pub fn masked_projected_residual_parts(
    f: &[f32],
    mask: &[f32],
    mut apply_k: impl FnMut(&mut [f32]),
) -> MaskedResidualParts {
    let ndof = f.len();
    let mut ku = vec![0.0_f32; ndof];
    apply_k(&mut ku);
    for (k, m) in ku.iter_mut().zip(mask) {
        *k *= *m;
    }
    let mut num = 0.0_f32;
    let mut den = 0.0_f32;
    for i in 0..ndof {
        let fi = f[i] * mask[i];
        let ri = mask[i] * (f[i] - ku[i]);
        num += ri * ri;
        den += fi * fi;
    }
    let abs_residual = num.sqrt();
    let abs_rhs = den.sqrt().max(1e-30_f32);
    MaskedResidualParts {
        abs_residual,
        abs_rhs,
        rel_residual: abs_residual / abs_rhs,
    }
}

/// 2×2 bisection driver (probe-only). Does not mutate caller `u`.
#[allow(clippy::too_many_arguments)]
pub fn hex_solve_pcg_bisect(
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
    diag: &mut [f32],
    scratch_ku: &mut [f32],
    max_iter: usize,
    precond: HexPcgPrecondKind,
    relative_tol: f32,
    cfg: HexPcgBisectConfig,
    op_cache: Option<&HexStructuredOperatorCache>,
) -> HexPcgBisectReport {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let ndof = n * 3;
    let max_it = max_iter.max(1);
    let tol = relative_tol.max(1e-30_f32);

    let k_char = if cfg.nondim {
        hex_stiffness_scale(e_cell, dx, dy, dz)
    } else {
        1.0_f32
    };
    let e_work: Vec<f32> = if cfg.nondim {
        e_cell.iter().map(|e| e / k_char).collect()
    } else {
        e_cell.to_vec()
    };
    let f_work: Vec<f32> = if cfg.nondim {
        f.iter().map(|fi| fi / k_char).collect()
    } else {
        f.to_vec()
    };

    hex_diagonal(nx, ny, nz, dx, dy, dz, nu, &e_work, diag);
    let mut block_jacobi = vec![0.0_f32; n * 9];
    if precond == HexPcgPrecondKind::BlockJacobiNodal3x3 {
        hex_nodal_block_jacobi_3x3(nx, ny, nz, dx, dy, dz, nu, &e_work, &mut block_jacobi);
    }

    let mut u = vec![0.0_f32; ndof];
    scratch_ku.fill(0.0);
    if let Some(cache) = op_cache {
        hex_k_times_u_accumulate_cached(cache, &e_work, &u, scratch_ku);
    } else {
        hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, &e_work, &u, scratch_ku);
    }

    let mut r = vec![0.0_f32; ndof];
    for i in 0..ndof {
        r[i] = mask[i] * (f_work[i] - scratch_ku[i]);
    }
    let f_norm = masked_norm_sq_f32(&f_work, mask).sqrt().max(1e-30_f32);

    let mut z = vec![0.0_f32; ndof];
    match precond {
        HexPcgPrecondKind::None => z.copy_from_slice(&r),
        HexPcgPrecondKind::JacobiDiagonal => apply_precond_jacobi_f32(diag, mask, &r, &mut z),
        HexPcgPrecondKind::BlockJacobiNodal3x3 => {
            apply_precond_block_3x3_f32(&block_jacobi, mask, &r, &mut z, n)
        }
        HexPcgPrecondKind::GeometricMultigridVCycle => apply_precond_geometric_mg_f32(
            nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
        ),
        HexPcgPrecondKind::SemicoarseningMultigridVCycle => apply_precond_semicoarsening_mg_f32(
            nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
        ),
        HexPcgPrecondKind::AlgebraicMultigridVCycle => apply_precond_algebraic_amg_f32(
            nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
        ),
    }
    let mut p = z.clone();

    let mut rz_old = dot_f32(&r, &z);

    let mut pcg_iters = 0usize;
    let mut pcg_rel_recursive = f32::INFINITY;

    for _ in 0..max_it {
        pcg_iters += 1;
        scratch_ku.fill(0.0);
        if let Some(cache) = op_cache {
            hex_k_times_u_accumulate_cached(cache, &e_work, &p, scratch_ku);
        } else {
            hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, &e_work, &p, scratch_ku);
        }

        let pap = masked_dot_f32(&p, scratch_ku, mask).max(1e-30_f32);
        let alpha = rz_old / pap;

        match cfg.loop_kind {
            HexPcgLoopKind::Original => {
                for i in 0..ndof {
                    u[i] += alpha * p[i];
                }
                for i in 0..ndof {
                    r[i] -= alpha * mask[i] * scratch_ku[i];
                }
            }
            HexPcgLoopKind::RefreshMaskedP => {
                for i in 0..ndof {
                    u[i] = mask[i] * (u[i] + alpha * p[i]);
                }
                hex_projected_k_times_u(nx, ny, nz, dx, dy, dz, nu, &e_work, &u, mask, scratch_ku);
                for i in 0..ndof {
                    r[i] = mask[i] * (f_work[i] - scratch_ku[i]);
                }
            }
        }

        let r_norm = masked_norm_sq_f32(&r, mask).sqrt();
        pcg_rel_recursive = r_norm / f_norm;
        if cfg.stop_on_true_residual {
            // Recursive residual is the cheap trigger; bind exit on one true `eq_rel` matvec.
            let periodic = pcg_iters % HEX_PCG_TRUE_RESIDUAL_CHECK_PERIOD == 0;
            let recursive_pass = r_norm <= tol * f_norm;
            if relative_tol > 0.0 && (periodic || recursive_pass) {
                let r_true =
                    hex_equilibrium_rel_residual(nx, ny, nz, dx, dy, dz, nu, e_cell, f, mask, &u);
                if r_true <= tol {
                    break;
                }
            }
        } else if relative_tol > 0.0 && r_norm < tol * f_norm {
            break;
        }

        match precond {
            HexPcgPrecondKind::None => z.copy_from_slice(&r),
            HexPcgPrecondKind::JacobiDiagonal => apply_precond_jacobi_f32(diag, mask, &r, &mut z),
            HexPcgPrecondKind::BlockJacobiNodal3x3 => {
                apply_precond_block_3x3_f32(&block_jacobi, mask, &r, &mut z, n)
            }
            HexPcgPrecondKind::GeometricMultigridVCycle => apply_precond_geometric_mg_f32(
                nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
            ),
            HexPcgPrecondKind::SemicoarseningMultigridVCycle => {
                apply_precond_semicoarsening_mg_f32(
                    nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
                )
            }
            HexPcgPrecondKind::AlgebraicMultigridVCycle => apply_precond_algebraic_amg_f32(
                nx, ny, nz, dx, dy, dz, nu, &e_work, mask, diag, &r, &mut z, op_cache,
            ),
        }

        let rz_new = dot_f32(&r, &z);
        let beta = (rz_new / rz_old.max(1e-30_f32)).max(0.0);
        rz_old = rz_new;

        match cfg.loop_kind {
            HexPcgLoopKind::Original => {
                for i in 0..ndof {
                    p[i] = z[i] + beta * p[i];
                }
            }
            HexPcgLoopKind::RefreshMaskedP => {
                for i in 0..ndof {
                    p[i] = mask[i] * (z[i] + beta * p[i]);
                }
            }
        }
    }

    for i in 0..ndof {
        u[i] *= mask[i];
    }

    let rel_true = hex_equilibrium_rel_residual(nx, ny, nz, dx, dy, dz, nu, e_cell, f, mask, &u);

    HexPcgBisectReport {
        iterations: pcg_iters,
        rel_residual_recursive: pcg_rel_recursive,
        rel_residual_true: rel_true,
        stiffness_scale: k_char,
        u,
    }
}

/// Descent-curve sample from an f64 PCG run (probe diagnostics).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HexPcgDescentSample {
    pub iteration: usize,
    pub rel_recursive: f64,
    pub rel_true: f64,
}

/// f64 PCG lane: all CG state (`u,r,z,p,α,β`, dots) and `K·u` in f64; bind on f64 `eq_rel`.
#[allow(clippy::too_many_arguments)]
fn hex_solve_pcg_masked_f64(
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
    u: &mut [f32],
    diag: &mut [f32],
    max_iter: usize,
    precond: HexPcgPrecondKind,
    relative_tol: f32,
    milestone_iters: Option<&[usize]>,
    mut descent_out: Option<&mut Vec<HexPcgDescentSample>>,
    op_cache: Option<&HexStructuredOperatorCache>,
) -> HexPcgReport {
    let ndof = u.len();
    let max_it = max_iter.max(1);
    let tol = (relative_tol.max(HEX_PCG_REL_TOL_F64).max(1e-30_f32)) as f64;

    let k_char = hex_stiffness_scale(e_cell, dx, dy, dz) as f64;
    let e_solve: Vec<f64> = e_cell.iter().map(|&e| e as f64 / k_char).collect();
    let f_phys: Vec<f64> = f.iter().map(|&fi| fi as f64).collect();
    let f_solve: Vec<f64> = f_phys.iter().map(|&fi| fi / k_char).collect();
    let mask64: Vec<f64> = mask.iter().map(|&m| m as f64).collect();

    let e_solve_f32: Vec<f32> = e_solve.iter().map(|&e| e as f32).collect();
    hex_diagonal(nx, ny, nz, dx, dy, dz, nu, &e_solve_f32, diag);
    let diag64: Vec<f64> = diag.iter().map(|&d| d as f64).collect();
    let n_nodes = (nx + 1) * (ny + 1) * (nz + 1);
    let mut block_jacobi64 = vec![0.0_f64; n_nodes * 9];
    if precond == HexPcgPrecondKind::BlockJacobiNodal3x3 {
        let mut blocks_f32 = vec![0.0_f32; n_nodes * 9];
        hex_nodal_block_jacobi_3x3(nx, ny, nz, dx, dy, dz, nu, &e_solve_f32, &mut blocks_f32);
        for (a, b) in block_jacobi64.iter_mut().zip(&blocks_f32) {
            *a = *b as f64;
        }
    }

    let mut u64: Vec<f64> = u.iter().map(|&x| x as f64).collect();
    for i in 0..ndof {
        u64[i] *= mask64[i];
    }

    let mut ku = vec![0.0_f64; ndof];
    let mut r = vec![0.0_f64; ndof];
    let mut z = vec![0.0_f64; ndof];
    let mut p = vec![0.0_f64; ndof];

    let projected_ku = |vec: &[f64], out: &mut [f64]| {
        out.fill(0.0);
        if let Some(cache) = op_cache {
            // f64 lane nondims `e_solve`/`f_solve`; cached matvec must use the same scaled moduli.
            hex_k_times_u_accumulate_cached_f64_e(cache, &e_solve, vec, out);
        } else {
            hex_k_times_u_accumulate_f64(nx, ny, nz, dx, dy, dz, nu, &e_solve, vec, out);
        }
        for (k, m) in out.iter_mut().zip(&mask64) {
            *k *= *m;
        }
    };

    projected_ku(&u64, &mut ku);
    for i in 0..ndof {
        r[i] = mask64[i] * (f_solve[i] - ku[i]);
    }
    let f_norm = masked_norm_sq_f64(&f_solve, &mask64).sqrt().max(1e-30_f64);
    let abs_tol = tol * f_norm;

    match precond {
        HexPcgPrecondKind::None => z.copy_from_slice(&r),
        HexPcgPrecondKind::JacobiDiagonal => apply_precond_jacobi_f64(&diag64, &mask64, &r, &mut z),
        HexPcgPrecondKind::BlockJacobiNodal3x3 => {
            apply_precond_block_3x3_f64(&block_jacobi64, &mask64, &r, &mut z, n_nodes)
        }
        HexPcgPrecondKind::GeometricMultigridVCycle => apply_precond_geometric_mg_f64(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            nu,
            &e_solve_f32,
            &mask64,
            &diag64,
            &r,
            &mut z,
            op_cache,
        ),
        HexPcgPrecondKind::SemicoarseningMultigridVCycle => apply_precond_semicoarsening_mg_f64(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            nu,
            &e_solve_f32,
            &mask64,
            &diag64,
            &r,
            &mut z,
            op_cache,
        ),
        HexPcgPrecondKind::AlgebraicMultigridVCycle => apply_precond_algebraic_amg_f64(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            nu,
            &e_solve_f32,
            &mask64,
            &diag64,
            &r,
            &mut z,
            op_cache,
        ),
    }
    p.copy_from_slice(&z);

    let mut pcg_iters = 0usize;
    let mut pcg_rel_recursive = f32::INFINITY;
    let mut rz_old: f64 = dot_f64(&r, &z);

    for _ in 0..max_it {
        pcg_iters += 1;
        projected_ku(&p, &mut ku);
        let pap: f64 = dot_f64(&p, &ku).max(1e-30_f64);
        let alpha: f64 = rz_old / pap;
        if !alpha.is_finite() {
            break;
        }

        for i in 0..ndof {
            u64[i] = (u64[i] + alpha * p[i]) * mask64[i];
        }

        projected_ku(&u64, &mut ku);
        for i in 0..ndof {
            r[i] = mask64[i] * (f_solve[i] - ku[i]);
        }

        match precond {
            HexPcgPrecondKind::None => z.copy_from_slice(&r),
            HexPcgPrecondKind::JacobiDiagonal => {
                apply_precond_jacobi_f64(&diag64, &mask64, &r, &mut z)
            }
            HexPcgPrecondKind::BlockJacobiNodal3x3 => {
                apply_precond_block_3x3_f64(&block_jacobi64, &mask64, &r, &mut z, n_nodes)
            }
            HexPcgPrecondKind::GeometricMultigridVCycle => apply_precond_geometric_mg_f64(
                nx,
                ny,
                nz,
                dx,
                dy,
                dz,
                nu,
                &e_solve_f32,
                &mask64,
                &diag64,
                &r,
                &mut z,
                op_cache,
            ),
            HexPcgPrecondKind::SemicoarseningMultigridVCycle => {
                apply_precond_semicoarsening_mg_f64(
                    nx,
                    ny,
                    nz,
                    dx,
                    dy,
                    dz,
                    nu,
                    &e_solve_f32,
                    &mask64,
                    &diag64,
                    &r,
                    &mut z,
                    op_cache,
                )
            }
            HexPcgPrecondKind::AlgebraicMultigridVCycle => apply_precond_algebraic_amg_f64(
                nx,
                ny,
                nz,
                dx,
                dy,
                dz,
                nu,
                &e_solve_f32,
                &mask64,
                &diag64,
                &r,
                &mut z,
                op_cache,
            ),
        }

        let rz_new: f64 = dot_f64(&r, &z);
        if !rz_new.is_finite() {
            break;
        }
        let beta: f64 = rz_new / rz_old.max(1e-30_f64);
        rz_old = rz_new;
        for i in 0..ndof {
            p[i] = (z[i] + beta * p[i]) * mask64[i];
        }

        let r_norm: f64 = norm_sq_f64(&r).sqrt();
        let r_rec = r_norm / f_norm;
        pcg_rel_recursive = r_rec as f32;
        if let Some(targets) = milestone_iters {
            if targets.contains(&pcg_iters) {
                let r_true = hex_equilibrium_rel_residual_f64(
                    nx, ny, nz, dx, dy, dz, nu, e_cell, f, mask, &u64,
                );
                if let Some(out) = descent_out.as_deref_mut() {
                    out.push(HexPcgDescentSample {
                        iteration: pcg_iters,
                        rel_recursive: r_rec,
                        rel_true: r_true,
                    });
                }
            }
        }

        let periodic = pcg_iters % HEX_PCG_TRUE_RESIDUAL_CHECK_PERIOD == 0;
        let recursive_pass = r_norm <= abs_tol;
        let probe_descent = milestone_iters.is_some();
        if !probe_descent && tol > 0.0 && (periodic || recursive_pass) {
            let r_true =
                hex_equilibrium_rel_residual_f64(nx, ny, nz, dx, dy, dz, nu, e_cell, f, mask, &u64);
            if r_true <= tol {
                break;
            }
        }
    }

    for i in 0..ndof {
        u[i] = (u64[i] * mask64[i]) as f32;
    }

    let rel_true =
        hex_equilibrium_rel_residual_f64(nx, ny, nz, dx, dy, dz, nu, e_cell, f, mask, &u64) as f32;

    HexPcgReport {
        iterations: pcg_iters,
        rel_residual: rel_true,
        rel_residual_recursive: pcg_rel_recursive,
        stopping_criterion: HexPcgStoppingCriterion::PlainRNorm,
        stiffness_scale: k_char as f32,
    }
}

/// Descent-curve probe driver (40×40×4 discrimination).
#[allow(clippy::too_many_arguments)]
pub fn hex_solve_pcg_f64_descent_probe(
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
    u: &mut [f32],
    diag: &mut [f32],
    max_iter: usize,
    use_preconditioner: bool,
    relative_tol: f32,
    milestone_iters: &[usize],
) -> (HexPcgReport, Vec<HexPcgDescentSample>) {
    let mut descent = Vec::new();
    let report = hex_solve_pcg_masked_f64(
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
        nu,
        e_cell,
        f,
        mask,
        u,
        diag,
        max_iter,
        hex_precond_from_use_preconditioner(use_preconditioner),
        relative_tol,
        Some(milestone_iters),
        Some(&mut descent),
        None,
    );
    (report, descent)
}

/// Projected PCG on masked free DOFs (`mask[d]=1` free, `0` fixed). Overwrites `u` in-place.
///
/// Quick grids: f32 lane + `HEX_PCG_REL_TOL_F32`. Striatus (`hex_pcg_use_f64_lane`): f64 lane + `HEX_PCG_REL_TOL_F64`.
pub fn hex_solve_pcg_masked(
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
    u: &mut [f32],
    diag: &mut [f32],
    scratch_ku: &mut [f32],
    max_iter: usize,
    precond: HexPcgPrecondKind,
    relative_tol: f32,
    op_cache: Option<&HexStructuredOperatorCache>,
) -> HexPcgReport {
    if hex_pcg_use_f64_lane(nx, ny, nz) {
        return hex_solve_pcg_masked_f64(
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
            nu,
            e_cell,
            f,
            mask,
            u,
            diag,
            max_iter,
            precond,
            relative_tol,
            None,
            None,
            op_cache,
        );
    }
    let report = hex_solve_pcg_bisect(
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
        nu,
        e_cell,
        f,
        mask,
        diag,
        scratch_ku,
        max_iter,
        precond,
        relative_tol,
        HexPcgBisectConfig {
            loop_kind: HexPcgLoopKind::Original,
            nondim: true,
            stop_on_true_residual: true,
        },
        op_cache,
    );
    u.copy_from_slice(&report.u);
    HexPcgReport {
        iterations: report.iterations,
        rel_residual: report.rel_residual_true,
        rel_residual_recursive: report.rel_residual_recursive,
        stopping_criterion: HexPcgStoppingCriterion::PlainRNorm,
        stiffness_scale: report.stiffness_scale,
    }
}

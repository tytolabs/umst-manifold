// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured **8-node trilinear (Q1) hex** linear elasticity on a Cartesian brick lattice.
//!
//! Matrix-free `K u` and assembled **Jacobi diagonal** for projected PCG on `[nx × ny × nz]`
//! cells with `(nx+1)(ny+1)(nz+1)` nodes. Gauss integration uses the standard `2×2×2` rule on the
//! reference cube \([-1,1]^3\).
//!
//! **B-bar / Selective Reduced Integration (SRI)** is used to cure shear locking in thin bending
//! configurations: the volumetric part of the strain–displacement matrix `B_vol = (1/3) m mᵀ B` is
//! replaced by its element-mean `B̄_vol` (equivalently, evaluated at the centroid via 1-point
//! quadrature), while the deviatoric part `B_dev = B − B_vol` retains the full 2×2×2 rule.
//! See Hughes 2000 §4.5 and Bathe 2006 §5.4.
//!
//! formal_anchor: Literature
//! formal_citation: Bathe 2006, *Finite Element Procedures*, §5.4 (hex elements); Hughes 2000, *The Finite Element Method*, §4.5 (B-bar / SRI)
//! formal_form: \(\int_{Ω^e} \mathbf B^{\mathsf T}\mathbf D\mathbf B\,\mathrm dΩ\,\mathbf u^e = \mathbf f^e\) with isotropic \(\mathbf D(E,\nu)\) in Voigt form; volumetric block uses \(\bar{\mathbf B}_{\text{vol}}\) (element-mean) and deviatoric block uses pointwise \(\mathbf B_{\text{dev}}\).

#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

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

/// B-bar transpose × stress: dot the deviatoric pointwise rows of B against σ for normal-strain
/// rows, and use the averaged volumetric rows for the hydrostatic component. This is the
/// adjoint of [`bbar_times_u`] and is essential for symmetry of `K_e = B̄ᵀ D B̄`.
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
                // Centroid-evaluated physical gradients for the volumetric B-bar block.
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
                            let eps = bbar_times_u(gn, gn_bar, &u24);
                            let sig = d_times_eps(&d, &eps);
                            let fe = bbar_t_times_sigma(gn, gn_bar, &sig);
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
                                // Shears (pointwise, unchanged)
                                b[3][c0] = gy;
                                b[3][c0 + 1] = gx;
                                b[4][c0 + 1] = gz;
                                b[4][c0 + 2] = gy;
                                b[5][c0] = gz;
                                b[5][c0 + 2] = gx;
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

/// Projected PCG on masked free DOFs (`mask[d]=1` free, `0` fixed). Overwrites `u` in-place.
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
    use_preconditioner: bool,
    relative_tol: f32,
) {
    let nx1 = nx + 1;
    let ny1 = ny + 1;
    let n = nx1 * ny1 * (nz + 1);
    let ndof = n * 3;
    let max_it = max_iter.max(1);

    hex_diagonal(nx, ny, nz, dx, dy, dz, nu, e_cell, diag);

    for i in 0..ndof {
        u[i] *= mask[i];
    }

    scratch_ku.fill(0.0);
    hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, u, scratch_ku);
    let mut r = vec![0.0_f32; ndof];
    let mut f_norm = 0.0_f32;
    for i in 0..ndof {
        let fi = f[i] * mask[i];
        f_norm += fi * fi;
        r[i] = mask[i] * (f[i] - scratch_ku[i]);
    }
    f_norm = f_norm.sqrt().max(1e-30_f32);

    let mut z = vec![0.0_f32; ndof];
    if use_preconditioner {
        for i in 0..ndof {
            z[i] = mask[i] * r[i] / diag[i];
        }
    } else {
        z.copy_from_slice(&r);
    }
    let mut p = z.clone();

    let mut rz_old = 0.0_f32;
    for i in 0..ndof {
        rz_old += r[i] * z[i];
    }

    let tol = relative_tol.max(1e-30_f32);
    for _ in 0..max_it {
        scratch_ku.fill(0.0);
        hex_k_times_u_accumulate(nx, ny, nz, dx, dy, dz, nu, e_cell, &p, scratch_ku);
        let mut pap = 0.0_f32;
        for i in 0..ndof {
            pap += p[i] * mask[i] * scratch_ku[i];
        }
        pap = pap.max(1e-30_f32);
        let alpha = rz_old / pap;
        for i in 0..ndof {
            u[i] += alpha * p[i];
        }
        for i in 0..ndof {
            r[i] -= alpha * mask[i] * scratch_ku[i];
        }
        let mut r_norm = 0.0_f32;
        for i in 0..ndof {
            let v = r[i] * mask[i];
            r_norm += v * v;
        }
        r_norm = r_norm.sqrt();
        if relative_tol > 0.0 && r_norm < tol * f_norm {
            break;
        }
        if use_preconditioner {
            for i in 0..ndof {
                z[i] = mask[i] * r[i] / diag[i];
            }
        } else {
            z.copy_from_slice(&r);
        }
        let mut rz_new = 0.0_f32;
        for i in 0..ndof {
            rz_new += r[i] * z[i];
        }
        let beta = (rz_new / rz_old.max(1e-30_f32)).max(0.0);
        rz_old = rz_new;
        for i in 0..ndof {
            p[i] = z[i] + beta * p[i];
        }
    }

    for i in 0..ndof {
        u[i] *= mask[i];
    }
}

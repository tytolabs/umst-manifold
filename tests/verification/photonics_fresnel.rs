// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FDFD Helmholtz verification (`photonics`): MMS on a Dirichlet line, two-media continuum Fresnel
//! MMS without PML, plus interface/stack smokes with PML on. Curl–curl vs Helmholtz checks are
//! **1-D uniform-chain** regressions only (including a piecewise \(\varepsilon_r\) profile and
//! **`curl_curl_y_mode_matches_scalar_helmholtz_xy_embedded_chain`**: same path graph with
//! **non-collinear** \((x,y,z)\) SI coordinates — still **not** a simplicial \(d_1\) patch solve).
//! **Verification §6 increment:** [`umst_manifold::physics::solvers::photonics::dec_maxwell_assembly`]
//! re-exports [`primal_d1_edge_flux_to_faces`](umst_manifold::physics::dec_primal::primal_d1_edge_flux_to_faces)
//! and [`primal_d1_transpose_face_flux_to_edges`](umst_manifold::physics::dec_primal::primal_d1_transpose_face_flux_to_edges)
//! on a **quad-split** `faces_b2` patch (same topology as `tests/dec_identities.rs`); plus
//! **`solve_maxwell_curl_curl`** pass-through when the graph is **not** a uniform x-chain; and
//! [`apply_dec_te_curl_curl_chain_operator`](umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator)
//! vs hand stencil with **piecewise** \(\varepsilon_r\).
//! **Verification #6 (metric + TE field split):** [`curl_curl_y_mode_matches_scalar_helmholtz_affine_x_metric_preserves_ex_ez`]
//! — affine SI \(x\) (\(x_j=x_0+jh\)) stresses difference-based **`h`** in [`solve_maxwell_curl_curl`]; **`E_x`,`E_z`** unchanged.
//! **Verification #6:** [`apply_dec_te_curl_curl_chain_operator_none_on_quad_split_expanded_patch`] —
//! quad-split patch (\(E=5\), \(N=4\)) rejects the chain extractor (memo
//! [`docs/research/v0.4_track15_dec_curl_curl_photonics.md`](../../docs/research/v0.4_track15_dec_curl_curl_photonics.md) §1).
//! **Verification #6 (assembled two-quad):** [`assembled_two_quads_dec_primal_photonics_maxwell_deferred`]
//! — six-node / nine-edge **`faces_b2`** mesh (same incidence as `tests/dec_identities.rs`):
//! \(d_1(d_0\omega)\) via [`dec_maxwell_assembly`](umst_manifold::physics::solvers::photonics::dec_maxwell_assembly),
//! [`primal_divergence_from_edge_flux_topo`] on the edge increment, then photonics **non-chain** `None` /
//! [`PhotonicsSolver::solve_maxwell_curl_curl`] pass-through.
//! **Verification #6 (tensor ε stub):** [`curl_curl_y_mode_matches_scalar_helmholtz_piecewise_eps_tensor_yy`] —
//! `[1,N,9]` row-major nodal **3×3** with **\(\varepsilon_{yy}\)** (isotropic diagonal in that test) matching scalar **`[1,N,1]`**
//! on [`PhotonicsHelmholtzSolver::solve_helmholtz`] / [`PhotonicsSolver::solve_maxwell_curl_curl`].
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md` (Track H).

#![cfg(feature = "photonics")]
#![allow(clippy::needless_range_loop)]

use approx::assert_relative_eq;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use num_complex::Complex32;
use umst_manifold::physics::dec_primal::{
    primal_divergence_from_edge_flux_topo, primal_scalar_edge_increment,
};
use umst_manifold::physics::solvers::PhotonicsHelmholtzSolver;
use umst_manifold::physics::time_orchestration::MechanicsInnerLoopConfig;
use umst_manifold::physics::topology::EdgeTopology;

type B = NdArray<f32>;

#[derive(Clone, Copy)]
struct C {
    re: f32,
    im: f32,
}

impl C {
    fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    fn add(a: Self, b: Self) -> Self {
        Self {
            re: a.re + b.re,
            im: a.im + b.im,
        }
    }
    fn sub(a: Self, b: Self) -> Self {
        Self {
            re: a.re - b.re,
            im: a.im - b.im,
        }
    }
    fn scale(s: f32, a: Self) -> Self {
        Self {
            re: s * a.re,
            im: s * a.im,
        }
    }
    fn mul(a: Self, b: Self) -> Self {
        Self {
            re: a.re * b.re - a.im * b.im,
            im: a.re * b.im + a.im * b.re,
        }
    }
    fn div(a: Self, b: Self) -> Self {
        let den = b.re * b.re + b.im * b.im;
        if den < 1e-30 {
            return Self::zero();
        }
        Self {
            re: (a.re * b.re + a.im * b.im) / den,
            im: (a.im * b.re - a.re * b.im) / den,
        }
    }
}

fn device() -> NdArrayDevice {
    NdArrayDevice::default()
}

/// Frobenius inner \(\langle a,b\rangle = \sum a\,b\) on matching `[B,N,C]` tensors (Burn).
fn tensor_inner_b3(a: Tensor<B, 3>, b: Tensor<B, 3>) -> f32 {
    a.mul(b).sum().into_scalar()
}

fn chain_edges(n: usize) -> Tensor<B, 2, Int> {
    let e = n - 1;
    let mut v = Vec::with_capacity(2 * e);
    for i in 0..e {
        v.push(i as i64);
    }
    for i in 0..e {
        v.push((i + 1) as i64);
    }
    Tensor::from_data(Data::new(v, Shape::new([2, e])), &device())
}

fn coords_line_x(n: usize, h: f32) -> Tensor<B, 2> {
    let mut v = Vec::with_capacity(n * 3);
    for i in 0..n {
        v.push(i as f32 * h);
        v.push(0.0);
        v.push(0.0);
    }
    Tensor::from_data(Data::new(v, Shape::new([n, 3])), &device())
}

/// Affine uniform spacing on \(x\): \(x_j = x_0 + j h\) (same **`h`** inference as [`coords_line_x`] via successive differences).
fn coords_affine_line_x(n: usize, x0: f32, h: f32) -> Tensor<B, 2> {
    let mut v = Vec::with_capacity(n * 3);
    for i in 0..n {
        v.push(x0 + i as f32 * h);
        v.push(0.0);
        v.push(0.0);
    }
    Tensor::from_data(Data::new(v, Shape::new([n, 3])), &device())
}

/// Same index-wise \(x = i h\) as [`coords_line_x`], with small smooth **y, z** so the chain is not
/// collinear in \(\mathbb{R}^3\) while remaining **x-monotone** for the uniform-chain gate inside
/// [`PhotonicsSolver::solve_maxwell_curl_curl`](umst_manifold::physics::solvers::PhotonicsSolver::solve_maxwell_curl_curl).
fn coords_xy_embedded_chain(n: usize, h: f32) -> Tensor<B, 2> {
    let dev = device();
    let mut v = Vec::with_capacity(n * 3);
    let denom = (n.saturating_sub(1).max(1)) as f32;
    let twopi = core::f32::consts::TAU;
    for i in 0..n {
        v.push(i as f32 * h);
        let t = twopi * (i as f32 / denom);
        v.push(1e-2_f32 * t.sin());
        v.push(1e-2_f32 * t.cos());
    }
    Tensor::from_data(Data::new(v, Shape::new([n, 3])), &dev)
}

/// Quad `0–1–2–3` with diagonal **`0→2`** (five edges, two CCW triangles) — same incidence as
/// `dec_curl_d1_annihilates_gradient_quad_split_two_faces_burn` in `tests/dec_identities.rs`.
fn quad_split_patch_tensors() -> (Tensor<B, 2, Int>, Tensor<B, 2, Int>, EdgeTopology<B>) {
    let dev = device();
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, 3, 0, //
                1, 2, 3, 0, 2,
            ],
            Shape::new([2, 5]),
        ),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 4, 4, 2, 3, //
                1, 1, -1, 1, 1, 1,
            ],
            Shape::new([2, 6]),
        ),
        &dev,
    );
    let topo = EdgeTopology::new(edges_b1.clone());
    (edges_b1, faces_b2, topo)
}

/// Two CCW quads side-by-side sharing oriented edge **`1→4`** — same **`edges_b1` / `faces_b2`**
/// incidence as `two_quads_shared_edge_faces_b2_and_topo` in `tests/dec_identities.rs`
/// (assembled patch, **not** a hand-built uniform path).
fn two_quads_shared_edge_patch_tensors() -> (Tensor<B, 2, Int>, Tensor<B, 2, Int>, EdgeTopology<B>)
{
    let dev = device();
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, 5, 4, 3, 0, 1, 1, //
                1, 2, 5, 4, 3, 0, 4, 5, 4,
            ],
            Shape::new([2, 9]),
        ),
        &dev,
    );
    let faces_b2: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 8, 6, 6, 4, 5, 1, 2, 7, 7, 3, 8, //
                1, 1, -1, 1, 1, 1, 1, 1, -1, 1, 1, -1,
            ],
            Shape::new([2, 12]),
        ),
        &dev,
    );
    let topo = EdgeTopology::new(edges_b1.clone());
    (edges_b1, faces_b2, topo)
}

fn cis32(theta: f32) -> C {
    C {
        re: theta.cos(),
        im: theta.sin(),
    }
}

/// Continuum TE Fresnel field (normal incidence) sampled at `x_j = j h`, with interface at
/// `x_I = n_left * h` (first `ε_right` node index `n_left`). Uses `k_a = k₀ √ε_a` in each half-space.
fn continuum_fresnel_te_sampled(
    n: usize,
    h: f32,
    k0: f32,
    n_left: usize,
    eps_l: f32,
    eps_r: f32,
) -> Vec<C> {
    let n1 = eps_l.sqrt();
    let n2 = eps_r.sqrt();
    let r = (n1 - n2) / (n1 + n2);
    let t = 2.0_f32 * n1 / (n1 + n2);
    let k1 = k0 * n1;
    let k2 = k0 * n2;
    let x_int = n_left as f32 * h;
    let mut v = Vec::with_capacity(n);
    for j in 0..n {
        let x = j as f32 * h;
        let e = if j < n_left {
            let dx = x - x_int;
            C::add(cis32(k1 * dx), C::scale(r, cis32(-k1 * dx)))
        } else {
            let dx = x - x_int;
            C::scale(t, cis32(k2 * dx))
        };
        v.push(e);
    }
    v
}

/// Subtract the linear Dirichlet bridge between `e[0]` and `e[n-1]` so endpoints are exactly zero.
fn dirichlet_zero_linear_bridge(e: &[C], n: usize) -> Vec<C> {
    let inv = 1.0_f32 / (n - 1).max(1) as f32;
    let mut out = Vec::with_capacity(n);
    for j in 0..n {
        let s = j as f32 * inv;
        let b = C::add(C::scale(1.0 - s, e[0]), C::scale(s, e[n - 1]));
        out.push(C::sub(e[j], b));
    }
    out
}

/// Undo [`dirichlet_zero_linear_bridge`]: recover the pre-bridge samples from bridged values.
#[inline]
fn invert_dirichlet_linear_bridge(bridged: &[C], e_end0: C, e_end1: C, n: usize) -> Vec<C> {
    let inv = 1.0_f32 / (n - 1).max(1) as f32;
    let mut out = Vec::with_capacity(n);
    for j in 0..n {
        let s = j as f32 * inv;
        let b = C::add(C::scale(1.0 - s, e_end0), C::scale(s, e_end1));
        out.push(C::add(bridged[j], b));
    }
    out
}

/// Discrete-only Fresnel reflection estimate on the **left** bulk using
/// \(E \approx a\,\mathrm e^{i k_1 (x-x_I)} + b\,\mathrm e^{-i k_1 (x-x_I)}\) at two interior nodes.
/// Returns `b/a` when the local \(2\times2\) complex system is well-conditioned.
fn fresnel_r_disc_two_point(
    e_left_unbridged: &[C],
    j_a: usize,
    j_b: usize,
    h: f32,
    k1: f32,
    x_interface: f32,
) -> Option<C> {
    if j_a >= e_left_unbridged.len() || j_b >= e_left_unbridged.len() {
        return None;
    }
    let x_a = j_a as f32 * h;
    let x_b = j_b as f32 * h;
    let z_a = cis32(k1 * (x_a - x_interface));
    let z_b = cis32(k1 * (x_b - x_interface));
    let inv_za = C::div(C { re: 1.0, im: 0.0 }, z_a);
    let inv_zb = C::div(C { re: 1.0, im: 0.0 }, z_b);
    // [z_a, inv_za; z_b, inv_zb] [a; b] = [u_a; u_b]
    let u_a = e_left_unbridged[j_a];
    let u_b = e_left_unbridged[j_b];
    let det = C::sub(C::mul(z_a, inv_zb), C::mul(z_b, inv_za));
    let den = det.re * det.re + det.im * det.im;
    if den < 1e-20_f32 {
        return None;
    }
    let det_a = C::sub(C::mul(u_a, inv_zb), C::mul(u_b, inv_za));
    let det_b = C::sub(C::mul(z_a, u_b), C::mul(z_b, u_a));
    let a = C::div(det_a, det);
    let b = C::div(det_b, det);
    let a_mag = a.re * a.re + a.im * a.im;
    if a_mag < 1e-16_f32 {
        return None;
    }
    Some(C::div(b, a))
}

/// Multi-point **least squares** estimate of `b/a` for \(E \approx a\,\mathrm e^{i k_1 (x-x_I)} + b\,\mathrm e^{-i k_1 (x-x_I)}\)
/// on the **left** bulk (`j h` coordinates, same as [`coords_line_x`]). More stable than [`fresnel_r_disc_two_point`] near the interface.
fn fresnel_r_disc_ls_left_bulk(
    e_left_unbridged: &[C],
    probe_js: &[usize],
    h: f32,
    k1: f32,
    x_interface: f32,
) -> Option<C> {
    if probe_js.len() < 2 {
        return None;
    }
    let mut m = 0usize;
    let mut g01 = Complex32::new(0.0, 0.0);
    let mut r0 = Complex32::new(0.0, 0.0);
    let mut r1 = Complex32::new(0.0, 0.0);
    for &j in probe_js {
        if j >= e_left_unbridged.len() {
            return None;
        }
        let x = j as f32 * h;
        let phase = k1 * (x - x_interface);
        let z = Complex32::new(phase.cos(), phase.sin());
        let u = Complex32::new(e_left_unbridged[j].re, e_left_unbridged[j].im);
        m += 1;
        g01 += z.conj() * z.conj();
        r0 += z.conj() * u;
        r1 += z * u;
    }
    let g00 = m as f32;
    let det = g00 * g00 - g01.norm_sqr();
    if det.abs() < 1e-20_f32 {
        return None;
    }
    let numer_a = r0 * g00 - g01 * r1;
    let numer_b = Complex32::new(g00, 0.0) * r1 - g01.conj() * r0;
    let a = numer_a / det;
    let b = numer_b / det;
    if a.norm() < 1e-18_f32 {
        return None;
    }
    let rb = b / a;
    Some(C {
        re: rb.re,
        im: rb.im,
    })
}

/// Same TE stencil as production (`inv_eps = 2/(ε_i+ε_{i+1})`, uniform `h`).
fn apply_te_helmholtz_chain(n: usize, h: f32, k0: f32, eps: &[C], e: &[C]) -> Vec<C> {
    let inv_h2 = 1.0 / (h * h);
    let k0c = C {
        re: k0 * k0,
        im: 0.0,
    };
    let mut out = vec![C::zero(); n];
    for i in 0..n {
        if i == 0 || i + 1 == n {
            out[i] = e[i];
            continue;
        }
        let inv_eps_m = C::div(C { re: 2.0, im: 0.0 }, C::add(eps[i - 1], eps[i]));
        let inv_eps_p = C::div(C { re: 2.0, im: 0.0 }, C::add(eps[i], eps[i + 1]));
        let lap = C::add(
            C::add(
                C::mul(C::scale(inv_h2, inv_eps_m), e[i - 1]),
                C::mul(C::scale(-inv_h2, C::add(inv_eps_m, inv_eps_p)), e[i]),
            ),
            C::mul(C::scale(inv_h2, inv_eps_p), e[i + 1]),
        );
        out[i] = C::add(lap, C::mul(k0c, e[i]));
    }
    out
}

#[test]
fn helmholtz_mms_sin_mode_recover() {
    let dev = device();
    let n = 129usize;
    // Grid spacing ~ O(1) so Laplacian and k₀² terms stay in a healthy f32 range.
    let h = 0.01_f32;
    let l = (n - 1) as f32 * h;
    // Spatial pattern sin(π x / L) with Helmholtz k₀ **detuned** from the Dirichlet eigenvalue
    // so the discrete system is well-conditioned (pure resonance would make A nearly singular).
    let k_spatial = core::f32::consts::PI / l;
    let k0 = 0.85 * k_spatial;
    let c_light = 2.998e8_f32;
    let f_hz = k0 * c_light / (2.0 * core::f32::consts::PI);

    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);

    let eps = vec![C { re: 1.0, im: 0.0 }; n];
    let mut e_ex = vec![C::zero(); n];
    for i in 0..n {
        let x = i as f32 * h;
        e_ex[i] = C {
            re: (k_spatial * x).sin(),
            im: 0.0,
        };
    }

    let b = apply_te_helmholtz_chain(n, h, k0, &eps, &e_ex);
    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
    let scale = omega * mu0;

    let mut jim = vec![0.0_f32; n];
    let jre = {
        let mut v = vec![0.0_f32; n];
        for i in 0..n {
            v[i] = -b[i].im / scale;
        }
        v
    };
    for i in 0..n {
        jim[i] = b[i].re / scale;
    }
    let jre_copy = jre.clone();
    let jim_copy = jim.clone();

    let eps_t = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let jr = Tensor::<B, 3>::from_data(Data::new(jre, Shape::new([1, n, 1])), &dev);
    let ji = Tensor::<B, 3>::from_data(Data::new(jim, Shape::new([1, n, 1])), &dev);

    let solver = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let cg = MechanicsInnerLoopConfig::default();
    let (er, ei) = solver.solve_helmholtz(eps_t, eps_i, jr, ji, edges, coords, &cg);

    let got_r = er.into_data().value;
    let got_i = ei.into_data().value;
    let mut sol = vec![C::zero(); n];
    for i in 0..n {
        sol[i] = C {
            re: got_r[i],
            im: got_i[i],
        };
    }
    let mut rhs_j = vec![C::zero(); n];
    for i in 0..n {
        let jm = jim_copy[i];
        let jr_ = jre_copy[i];
        rhs_j[i] = C {
            re: scale * jm,
            im: -scale * jr_,
        };
    }
    let res = apply_te_helmholtz_chain(n, h, k0, &eps, &sol);
    let mut rnorm = 0.0_f32;
    for i in 0..n {
        let d = C::sub(res[i], rhs_j[i]);
        rnorm = rnorm.max((d.re * d.re + d.im * d.im).sqrt());
    }
    assert_relative_eq!(rnorm, 0.0_f32, epsilon = 1e-3_f32, max_relative = 1.0);

    let mut max_err: f32 = 0.0;
    for i in 0..n {
        let dr = sol[i].re - e_ex[i].re;
        let di = sol[i].im - e_ex[i].im;
        max_err = max_err.max((dr * dr + di * di).sqrt());
    }
    assert_relative_eq!(max_err, 0.0_f32, epsilon = 1e-3_f32);
}

/// Two dielectric half-spaces on a **Dirichlet-closed** x-chain, **PML off** (`pml_thickness = 0`).
///
/// ## Geometry → chain indices (same contract as `photonics::solve_helmholtz`)
/// - **Graph / tensor node id** `i` is the row index in `eps_r_*` and in `coords_n3[i, :]`.
/// - **Edges** `edges_b1` are the monotone chain `i — (i+1)` for `i = 0..n-2` (see [`chain_edges`]).
/// - **Coordinates** `x_i = i * h` (see [`coords_line_x`]); the solver walks increasing `x`.
/// - **Half-spaces (piecewise constant εᵣ on nodes):**
///   - Left bulk: nodes `i ∈ [0, n_left - 1]` have `εᵣ = ε_left`.
///   - Right bulk: nodes `i ∈ [n_left, n - 1]` have `εᵣ = ε_right`.
///   - The **discrete material jump** is the half-edge between nodes `n_left - 1` and `n_left`
///     (harmonic average `2/(ε_left + ε_right)` on that link in the stencil).
///
/// ## Frequency / grid (aligned with [`helmholtz_mms_sin_mode_recover`])
/// Same `n`, `h`, and `k₀` as that MMS test (`k₀ = 0.85 π / L` with `L = (n-1) h`).
///
/// ## Continuum Fresnel + MMS (no PML)
/// **Analytic** TE Fresnel at normal incidence with `nₐ = √εₐ`: `r = (n₁ - n₂)/(n₁ + n₂)`,
/// `t = 2 n₁/(n₁ + n₂)`. With `ε_left = 1`, `ε_right = 4` ⇒ `r = -1/3`, `t = 2/3`.
///
/// We sample the **continuum** piecewise phasor (`k = k₀ √ε` in each half-space) on the chain via
/// [`continuum_fresnel_te_sampled`], then apply [`dirichlet_zero_linear_bridge`] so the target vanishes
/// at the Dirichlet caps. The impressed current follows the same residual-to-`J` map as
/// [`helmholtz_mms_sin_mode_recover`]. Assertions: discrete Helmholtz residual of the recovered field
/// matches the implied RHS, and the solution tracks the bridged target within an `f32` tolerance; the
/// analytic `r` value is pinned with a tight check.
#[test]
fn two_half_spaces_fresnel_te_no_pml_matches_analytic() {
    let dev = device();
    let n = 129usize;
    let h = 0.01_f32;
    let l = (n - 1) as f32 * h;
    let k_spatial = core::f32::consts::PI / l;
    let k0 = 0.85_f32 * k_spatial;
    let c_light = 2.998e8_f32;
    let f_hz = k0 * c_light / (2.0 * core::f32::consts::PI);

    let n_left = n / 2;
    let eps_left = 1.0_f32;
    let eps_right = 4.0_f32;
    let n1 = eps_left.sqrt();
    let n2 = eps_right.sqrt();
    let r_analytic = (n1 - n2) / (n1 + n2);

    let e_pw = continuum_fresnel_te_sampled(n, h, k0, n_left, eps_left, eps_right);
    let e_ex = dirichlet_zero_linear_bridge(&e_pw, n);

    let eps: Vec<C> = (0..n)
        .map(|i| C {
            re: if i < n_left { eps_left } else { eps_right },
            im: 0.0,
        })
        .collect();

    let b = apply_te_helmholtz_chain(n, h, k0, &eps, &e_ex);
    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
    let scale = omega * mu0;

    let mut jim = vec![0.0_f32; n];
    let jre = {
        let mut v = vec![0.0_f32; n];
        for i in 0..n {
            v[i] = -b[i].im / scale;
        }
        v
    };
    for i in 0..n {
        jim[i] = b[i].re / scale;
    }
    let jre_copy = jre.clone();
    let jim_copy = jim.clone();

    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let eps_t = Tensor::<B, 3>::from_data(
        Data::new(
            eps.iter().map(|e| e.re).collect::<Vec<_>>(),
            Shape::new([1, n, 1]),
        ),
        &dev,
    );
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let jr = Tensor::<B, 3>::from_data(Data::new(jre, Shape::new([1, n, 1])), &dev);
    let ji = Tensor::<B, 3>::from_data(Data::new(jim, Shape::new([1, n, 1])), &dev);

    let solver = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let cg = MechanicsInnerLoopConfig::default();
    let (er, ei) = solver.solve_helmholtz(eps_t, eps_i, jr, ji, edges, coords, &cg);

    let got_r = er.into_data().value;
    let got_i = ei.into_data().value;
    let mut sol = vec![C::zero(); n];
    for i in 0..n {
        sol[i] = C {
            re: got_r[i],
            im: got_i[i],
        };
    }
    let mut rhs_j = vec![C::zero(); n];
    for i in 0..n {
        rhs_j[i] = C {
            re: scale * jim_copy[i],
            im: -scale * jre_copy[i],
        };
    }
    let res = apply_te_helmholtz_chain(n, h, k0, &eps, &sol);
    let mut rnorm = 0.0_f32;
    for i in 0..n {
        let d = C::sub(res[i], rhs_j[i]);
        rnorm = rnorm.max((d.re * d.re + d.im * d.im).sqrt());
    }
    assert_relative_eq!(rnorm, 0.0_f32, epsilon = 8e-3_f32, max_relative = 1.0);

    let mut max_err: f32 = 0.0;
    for i in 0..n {
        let dr = sol[i].re - e_ex[i].re;
        let di = sol[i].im - e_ex[i].im;
        max_err = max_err.max((dr * dr + di * di).sqrt());
    }
    assert_relative_eq!(max_err, 0.0_f32, epsilon = 7.5e-2_f32);

    assert_relative_eq!(r_analytic, -1.0_f32 / 3.0_f32, epsilon = 1e-6_f32);

    // Discrete-only reflection: invert the Dirichlet linear bridge on the solved field, then fit
    // (+k₁) / (−k₁) amplitudes via multi-point LS (fallback: [`fresnel_r_disc_two_point`]).
    let e_unb = invert_dirichlet_linear_bridge(&sol, e_pw[0], e_pw[n - 1], n);
    let x_int = n_left as f32 * h;
    let k1 = k0 * n1;
    let lo = (n_left / 8).max(2);
    let hi = n_left.saturating_sub(n_left / 8 + 1);
    let mut probes: Vec<usize> = Vec::new();
    if hi > lo {
        for t in 0..5 {
            let j = lo + (t * (hi - lo)) / 4;
            if j < n_left {
                probes.push(j);
            }
        }
    }
    let r_disc = fresnel_r_disc_ls_left_bulk(&e_unb, &probes, h, k1, x_int)
        .or_else(|| {
            let j_a = (n_left / 4).max(2) + 2;
            let j_b = n_left.saturating_sub(4).saturating_sub(n_left / 8);
            if j_b > j_a && j_b < n_left {
                fresnel_r_disc_two_point(&e_unb, j_a, j_b, h, k1, x_int)
            } else {
                None
            }
        })
        .expect("r_disc LS / two-point system should be well-conditioned");
    assert_relative_eq!(
        r_disc.re,
        r_analytic,
        epsilon = 3.5e-2_f32,
        max_relative = 0.2
    );
    assert_relative_eq!(r_disc.im, 0.0_f32, epsilon = 4.5e-2_f32, max_relative = 1.0);
}

/// `PhotonicsSolver::solve_maxwell_curl_curl` (minimal primal-chain DEC + Thomas) matches
/// [`PhotonicsHelmholtzSolver::solve_helmholtz`] on the same uniform x-chain for TE \(E_y\).
#[test]
fn curl_curl_y_mode_matches_scalar_helmholtz() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let n = 41usize;
    let h = 1e-3_f32;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let center = n / 2;
    let mut jdat = vec![0.0_f32; n * 3];
    jdat[center * 3 + 1] = 1.0_f32;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let e0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let f_hz = 1e9_f32;
    let cg = MechanicsInnerLoopConfig::default();

    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let e_cc = ps.solve_maxwell_curl_curl(
        e0.clone(),
        eps_r.clone(),
        eps_i.clone(),
        j.clone(),
        edges.clone(),
        coords.clone(),
        &cg,
    );

    let helm = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let jy = j.narrow(2, 1, 1);
    let jy_im = Tensor::<B, 3>::zeros_like(&jy);
    let (ey_h, _) = helm.solve_helmholtz(eps_r, eps_i, jy, jy_im, edges, coords, &cg);

    let ey_cc = e_cc.narrow(2, 1, 1);
    let v_cc = ey_cc.into_data().value;
    let v_h = ey_h.into_data().value;
    assert_eq!(v_cc.len(), v_h.len());
    let mut mx = 0.0_f32;
    for i in 0..v_cc.len() {
        mx = mx.max((v_cc[i] - v_h[i]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-4_f32);
}

/// Affine SI \(x\) coordinates (\(x_j = x_0 + j h\)) so mesh spacing [**`h`**](../../src/physics/solvers/photonics.rs)
/// is inferred from **differences only**; TE **`E_y`** still matches [`PhotonicsHelmholtzSolver::solve_helmholtz`], and
/// **`E_x`,`E_z`** from `e_field` are **unchanged** (orthogonal components pass through the TE reduction in
/// [`PhotonicsSolver::solve_maxwell_curl_curl`](../../src/physics/solvers/photonics.rs)).
#[test]
fn curl_curl_y_mode_matches_scalar_helmholtz_affine_x_metric_preserves_ex_ez() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let n = 41usize;
    let h = 1e-3_f32;
    let x0 = 1.28e2_f32;
    let edges = chain_edges(n);
    let coords = coords_affine_line_x(n, x0, h);

    let mut e0 = vec![0.0_f32; n * 3];
    for i in 0..n {
        let t = i as f32 * 0.21_f32;
        e0[i * 3] = 0.31_f32 * t.sin();
        e0[i * 3 + 2] = -0.19_f32 * t.cos();
    }
    let center = n / 2;
    let mut jdat = vec![0.0_f32; n * 3];
    jdat[center * 3 + 1] = 1.0_f32;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let e_field = Tensor::<B, 3>::from_data(Data::new(e0, Shape::new([1, n, 3])), &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let f_hz = 1e9_f32;
    let cg = MechanicsInnerLoopConfig::default();

    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let e_cc = ps.solve_maxwell_curl_curl(
        e_field.clone(),
        eps_r.clone(),
        eps_i.clone(),
        j.clone(),
        edges.clone(),
        coords.clone(),
        &cg,
    );

    let helm = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let jy = j.narrow(2, 1, 1);
    let jy_im = Tensor::<B, 3>::zeros_like(&jy);
    let (ey_h, _) = helm.solve_helmholtz(eps_r, eps_i, jy, jy_im, edges, coords, &cg);

    let ex_cc = e_cc.clone().narrow(2, 0, 1);
    let ey_cc = e_cc.clone().narrow(2, 1, 1);
    let ez_cc = e_cc.clone().narrow(2, 2, 1);
    let ex_in = e_field.clone().narrow(2, 0, 1);
    let ez_in = e_field.narrow(2, 2, 1);

    let v_ex_cc = ex_cc.into_data().value;
    let v_ex_in = ex_in.into_data().value;
    let v_ez_cc = ez_cc.into_data().value;
    let v_ez_in = ez_in.into_data().value;
    let mut mx_pass = 0.0_f32;
    for i in 0..v_ex_cc.len() {
        mx_pass = mx_pass.max((v_ex_cc[i] - v_ex_in[i]).abs());
        mx_pass = mx_pass.max((v_ez_cc[i] - v_ez_in[i]).abs());
    }
    assert_relative_eq!(mx_pass, 0.0_f32, epsilon = 1e-6_f32, max_relative = 1.0);

    let v_cc = ey_cc.into_data().value;
    let v_h = ey_h.into_data().value;
    assert_eq!(v_cc.len(), v_h.len());
    let mut mx = 0.0_f32;
    for i in 0..v_cc.len() {
        mx = mx.max((v_cc[i] - v_h[i]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-4_f32);
}

/// Same parity as [`curl_curl_y_mode_matches_scalar_helmholtz`], with **non-collinear** SI
/// \((x,y,z)\) on the same path graph ([`coords_xy_embedded_chain`]) — still a 1-D chain gate, not a
/// simplicial \(d_1\) patch solve.
#[test]
fn curl_curl_y_mode_matches_scalar_helmholtz_xy_embedded_chain() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let n = 41usize;
    let h = 1e-3_f32;
    let edges = chain_edges(n);
    let coords = coords_xy_embedded_chain(n, h);
    let center = n / 2;
    let mut jdat = vec![0.0_f32; n * 3];
    jdat[center * 3 + 1] = 1.0_f32;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let e0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let f_hz = 1e9_f32;
    let cg = MechanicsInnerLoopConfig::default();

    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let e_cc = ps.solve_maxwell_curl_curl(
        e0.clone(),
        eps_r.clone(),
        eps_i.clone(),
        j.clone(),
        edges.clone(),
        coords.clone(),
        &cg,
    );

    let helm = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let jy = j.narrow(2, 1, 1);
    let jy_im = Tensor::<B, 3>::zeros_like(&jy);
    let (ey_h, _) = helm.solve_helmholtz(eps_r, eps_i, jy, jy_im, edges, coords, &cg);

    let ey_cc = e_cc.narrow(2, 1, 1);
    let v_cc = ey_cc.into_data().value;
    let v_h = ey_h.into_data().value;
    assert_eq!(v_cc.len(), v_h.len());
    let mut mx = 0.0_f32;
    for i in 0..v_cc.len() {
        mx = mx.max((v_cc[i] - v_h[i]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-4_f32);
}

/// Same operator identity as [`curl_curl_y_mode_matches_scalar_helmholtz`], but with a **non-uniform**
/// relative permittivity on nodes (three bulk values along the chain). This stresses harmonic means
/// \(2/(\varepsilon_i+\varepsilon_{i+1})\) on interior links; it does **not** extend the proof to 2D/3D DEC.
#[test]
fn curl_curl_y_mode_matches_scalar_helmholtz_piecewise_eps() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let n = 53usize;
    let h = 8e-4_f32;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let i0 = n / 7;
    let i1 = 5 * n / 7;
    let mut eps_flat = vec![0.0_f32; n];
    for i in 0..n {
        eps_flat[i] = if i < i0 {
            1.15_f32
        } else if i < i1 {
            4.6_f32
        } else {
            2.05_f32
        };
    }

    let mut jdat = vec![0.0_f32; n * 3];
    jdat[(n / 3) * 3 + 1] = 1.0_f32;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let e0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let eps_r = Tensor::<B, 3>::from_data(Data::new(eps_flat, Shape::new([1, n, 1])), &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let f_hz = 2.2e9_f32;
    let cg = MechanicsInnerLoopConfig::default();

    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let e_cc = ps.solve_maxwell_curl_curl(
        e0.clone(),
        eps_r.clone(),
        eps_i.clone(),
        j.clone(),
        edges.clone(),
        coords.clone(),
        &cg,
    );

    let helm = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let jy = j.narrow(2, 1, 1);
    let jy_im = Tensor::<B, 3>::zeros_like(&jy);
    let (ey_h, _) = helm.solve_helmholtz(eps_r, eps_i, jy, jy_im, edges, coords, &cg);

    let ey_cc = e_cc.narrow(2, 1, 1);
    let v_cc = ey_cc.into_data().value;
    let v_h = ey_h.into_data().value;
    assert_eq!(v_cc.len(), v_h.len());
    let mut mx = 0.0_f32;
    for i in 0..v_cc.len() {
        mx = mx.max((v_cc[i] - v_h[i]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-4_f32);
}

/// Same as [`curl_curl_y_mode_matches_scalar_helmholtz_piecewise_eps`], but `relative_permittivity`
/// is **`[1,N,9]`** with **isotropic diagonal** \(\varepsilon_{xx}=\varepsilon_{yy}=\varepsilon_{zz}\)
/// on each node (off-diagonals zero). TE reduction uses the tensor's \(\varepsilon_{yy}\) entry
/// (row-major 3×3, index **4** per [`photonics`](umst_manifold::physics::solvers::photonics)); scalar Helmholtz
/// uses **`[1,N,1]`** with the same nodal values — parity locks the tensor layout on
/// [`PhotonicsSolver::solve_maxwell_curl_curl`].
#[test]
fn curl_curl_y_mode_matches_scalar_helmholtz_piecewise_eps_tensor_yy() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let n = 53usize;
    let h = 8e-4_f32;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let i0 = n / 7;
    let i1 = 5 * n / 7;
    let mut eps_flat = vec![0.0_f32; n];
    for i in 0..n {
        eps_flat[i] = if i < i0 {
            1.15_f32
        } else if i < i1 {
            4.6_f32
        } else {
            2.05_f32
        };
    }

    let mut eps9 = vec![0.0_f32; n * 9];
    for i in 0..n {
        let e = eps_flat[i];
        let b = i * 9;
        eps9[b] = e;
        eps9[b + 4] = e;
        eps9[b + 8] = e;
    }

    let mut jdat = vec![0.0_f32; n * 3];
    jdat[(n / 3) * 3 + 1] = 1.0_f32;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let e0 = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let eps_r_tensor = Tensor::<B, 3>::from_data(Data::new(eps9, Shape::new([1, n, 9])), &dev);
    let eps_r_scalar = Tensor::<B, 3>::from_data(Data::new(eps_flat, Shape::new([1, n, 1])), &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let f_hz = 2.2e9_f32;
    let cg = MechanicsInnerLoopConfig::default();

    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let e_cc = ps.solve_maxwell_curl_curl(
        e0.clone(),
        eps_r_tensor,
        eps_i.clone(),
        j.clone(),
        edges.clone(),
        coords.clone(),
        &cg,
    );

    let helm = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 0,
        pml_max_sigma: 0.0,
    };
    let jy = j.narrow(2, 1, 1);
    let jy_im = Tensor::<B, 3>::zeros_like(&jy);
    let (ey_h, _) = helm.solve_helmholtz(eps_r_scalar, eps_i, jy, jy_im, edges, coords, &cg);

    let ey_cc = e_cc.narrow(2, 1, 1);
    let v_cc = ey_cc.into_data().value;
    let v_h = ey_h.into_data().value;
    assert_eq!(v_cc.len(), v_h.len());
    let mut mx = 0.0_f32;
    for i in 0..v_cc.len() {
        mx = mx.max((v_cc[i] - v_h[i]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-4_f32);
}

/// Primal DEC tensor matvec [`umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator`]
/// agrees with the hand-rolled TE chain stencil (real fields, vacuum \(\varepsilon_r=1\)).
#[test]
fn dec_te_primal_tensor_matches_chain_stencil() {
    use umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator;

    let dev = device();
    let n = 37usize;
    let h = 2e-3_f32;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let f_hz = 2.4e9_f32;
    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let k0 = omega / 2.998e8_f32;

    let mut eyv = vec![0.0_f32; n];
    for i in 0..n {
        eyv[i] = ((i * 7) as f32 * 0.031_f32).sin();
    }
    let ey = Tensor::<B, 3>::from_data(Data::new(eyv.clone(), Shape::new([1, n, 1])), &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);

    let got = apply_dec_te_curl_curl_chain_operator(
        ey.clone(),
        eps_r,
        edges.clone(),
        coords.clone(),
        f_hz,
    )
    .expect("uniform chain");

    let eps: Vec<C> = vec![C { re: 1.0, im: 0.0 }; n];
    let e_c: Vec<C> = eyv.iter().map(|&re| C { re, im: 0.0 }).collect();
    let stencil = apply_te_helmholtz_chain(n, h, k0, &eps, &e_c);

    let got_v = got.into_data().value;
    let mut mx = 0.0_f32;
    for i in 0..n {
        mx = mx.max((got_v[i] - stencil[i].re).abs());
        mx = mx.max(stencil[i].im.abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 5e-5_f32, max_relative = 1.0);
}

/// [`primal_d1_edge_flux_to_faces`] imported through [`umst_manifold::physics::solvers::photonics::dec_maxwell_assembly`]
/// annihilates \(d_0\omega\) on the quad-split patch (shared-edge `faces_b2` COO).
#[test]
fn dec_maxwell_assembly_quad_split_d1_annihilates_gradient_burn() {
    use umst_manifold::physics::solvers::photonics::dec_maxwell_assembly::primal_d1_edge_flux_to_faces;

    let dev = device();
    let (_edges_b1, faces_b2, topo) = quad_split_patch_tensors();
    let omega = [0.9_f32, -1.4, 2.2, 0.15];
    let nodal = Tensor::from_data(
        Data::new(
            vec![omega[0], omega[1], omega[2], omega[3]],
            Shape::new([1, 4, 1]),
        ),
        &dev,
    );
    let grad_on_edges = primal_scalar_edge_increment(nodal, &topo);
    let d1_grad = primal_d1_edge_flux_to_faces(grad_on_edges, faces_b2, &[(0, 3), (3, 6)]);
    let v: Vec<f32> = d1_grad.into_data().value;
    assert_eq!(v.len(), 2);
    assert_relative_eq!(v[0], 0.0_f32, epsilon = 1e-4_f32, max_relative = 1.0);
    assert_relative_eq!(v[1], 0.0_f32, epsilon = 1e-4_f32, max_relative = 1.0);
}

/// [`primal_d1_transpose_face_flux_to_edges`] via [`umst_manifold::physics::solvers::photonics::dec_maxwell_assembly`]:
/// discrete adjoint \(\langle d_1 u, w\rangle = \langle u, d_1^\top w\rangle\) on the quad-split patch
/// (same tensors / ranges as [`dec_primal_d1_adjoint_identity_quad_split_two_faces_burn`] in `dec_identities.rs`).
#[test]
fn dec_maxwell_assembly_quad_split_d1_adjoint_identity_burn() {
    use umst_manifold::physics::solvers::photonics::dec_maxwell_assembly::{
        primal_d1_edge_flux_to_faces, primal_d1_transpose_face_flux_to_edges,
    };

    let dev = device();
    let (_edges_b1, faces_b2, _) = quad_split_patch_tensors();
    let ranges = [(0usize, 3usize), (3usize, 6usize)];
    let u = Tensor::from_data(
        Data::new(vec![0.55_f32, -0.2, 1.05, -0.9, 0.3], Shape::new([1, 5, 1])),
        &dev,
    );
    let w = Tensor::from_data(Data::new(vec![0.4_f32, -0.65], Shape::new([1, 2, 1])), &dev);
    let d1u = primal_d1_edge_flux_to_faces(u.clone(), faces_b2.clone(), &ranges);
    let lhs = tensor_inner_b3(d1u, w.clone());
    let d1t_w = primal_d1_transpose_face_flux_to_edges(w, faces_b2, &ranges, &u);
    let rhs = tensor_inner_b3(u.clone(), d1t_w);
    assert_relative_eq!(lhs, rhs, epsilon = 1e-4_f32, max_relative = 1.0);
}

/// Track 15 — [`docs/research/v0.4_track15_dec_curl_curl_photonics.md`](../../docs/research/v0.4_track15_dec_curl_curl_photonics.md) §1:
/// the quad-split **expanded patch** is not a spanning path (\(E \neq N-1\) — five edges on four nodes —
/// and branch vertices), so [`apply_dec_te_curl_curl_chain_operator`] returns `None` and cannot silently
/// apply the uniform-chain TE matvec where production \(d_1\) / `faces_b2` DEC is required (Verification **#6**).
#[test]
fn apply_dec_te_curl_curl_chain_operator_none_on_quad_split_expanded_patch() {
    use umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator;

    let dev = device();
    let (edges_b1, _, _) = quad_split_patch_tensors();
    let n = 4usize;
    let coords = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, //
                1.0, 0.0, 0.0, //
                1.0, 1.0, 0.0, //
                0.0, 1.0, 0.0,
            ],
            Shape::new([n, 3]),
        ),
        &dev,
    );
    let ey = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let f_hz = 1e9_f32;

    let got = apply_dec_te_curl_curl_chain_operator(ey, eps_r, edges_b1, coords, f_hz);
    assert!(
        got.is_none(),
        "quad-split expanded patch must not activate the uniform-chain TE curl–curl operator"
    );
}

/// Non-chain **2D patch** topology: [`PhotonicsSolver::solve_maxwell_curl_curl`] returns the
/// incoming `e_field` unchanged (documented pass-through until vector curl–curl ships).
#[test]
fn solve_maxwell_curl_curl_pass_through_quad_split_not_chain() {
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let (edges_b1, _, _) = quad_split_patch_tensors();
    let coords = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, //
                1.0, 0.0, 0.0, //
                1.0, 1.0, 0.0, //
                0.0, 1.0, 0.0,
            ],
            Shape::new([4, 3]),
        ),
        &dev,
    );
    let n = 4usize;
    let mut e0 = vec![0.0_f32; n * 3];
    for i in 0..n {
        e0[i * 3] = 0.1 * i as f32;
        e0[i * 3 + 1] = -0.25 + 0.07 * i as f32;
        e0[i * 3 + 2] = 0.33 - 0.04 * i as f32;
    }
    let e_field = Tensor::<B, 3>::from_data(Data::new(e0, Shape::new([1, n, 3])), &dev);
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let mut jdat = vec![0.0_f32; n * 3];
    jdat[2] = 0.5;
    jdat[2 * 3 + 2] = -0.5;
    jdat[4] = 1.0;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let cg = MechanicsInnerLoopConfig::default();
    let ps = PhotonicsSolver {
        frequency_hz: 1e9_f32,
    };
    let out = ps.solve_maxwell_curl_curl(e_field.clone(), eps_r, eps_i, j, edges_b1, coords, &cg);
    let vi = out.into_data().value;
    let ei = e_field.into_data().value;
    assert_eq!(vi.len(), ei.len());
    let mut mx = 0.0_f32;
    for k in 0..vi.len() {
        mx = mx.max((vi[k] - ei[k]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-6_f32, max_relative = 1.0);
}

/// **Verification #6 — assembled two-quad strip:** six-node / nine-edge **`faces_b2`** incidence
/// (same as `dec_curl_d1_annihilates_gradient_two_quads_shared_edge_burn` in `tests/dec_identities.rs`)
/// exercises [`primal_scalar_edge_increment`](umst_manifold::physics::dec_primal::primal_scalar_edge_increment),
/// photonics [`dec_maxwell_assembly::primal_d1_edge_flux_to_faces`](umst_manifold::physics::solvers::photonics::dec_maxwell_assembly),
/// [`primal_divergence_from_edge_flux_topo`](umst_manifold::physics::dec_primal::primal_divergence_from_edge_flux_topo) on \(d_0\omega\),
/// then [`apply_dec_te_curl_curl_chain_operator`](umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator) **`None`**
/// and [`PhotonicsSolver::solve_maxwell_curl_curl`](umst_manifold::physics::solvers::PhotonicsSolver::solve_maxwell_curl_curl) pass-through
/// (documented deferral for non-uniform-chain topologies).
#[test]
fn assembled_two_quads_dec_primal_photonics_maxwell_deferred() {
    use umst_manifold::physics::solvers::photonics::{
        apply_dec_te_curl_curl_chain_operator, dec_maxwell_assembly::primal_d1_edge_flux_to_faces,
    };
    use umst_manifold::physics::solvers::PhotonicsSolver;

    let dev = device();
    let (edges_b1, faces_b2, topo) = two_quads_shared_edge_patch_tensors();
    let omega = [0.5_f32, -0.9, 1.7, 0.2, -1.1, 0.35];
    let nodal = Tensor::from_data(
        Data::new(
            vec![omega[0], omega[1], omega[2], omega[3], omega[4], omega[5]],
            Shape::new([1, 6, 1]),
        ),
        &dev,
    );
    let grad_on_edges = primal_scalar_edge_increment(nodal.clone(), &topo);
    let ranges = [(0usize, 3usize), (3, 6), (6, 9), (9, 12)];
    let d1_grad = primal_d1_edge_flux_to_faces(grad_on_edges.clone(), faces_b2.clone(), &ranges);
    let v: Vec<f32> = d1_grad.into_data().value;
    assert_eq!(v.len(), 4);
    for x in v {
        assert_relative_eq!(x, 0.0_f32, epsilon = 1e-4_f32, max_relative = 1.0);
    }

    let div = primal_divergence_from_edge_flux_topo(grad_on_edges, &topo, &nodal);
    assert_eq!(div.dims(), [1, 6, 1]);
    let div_v = div.into_data().value;
    assert!(div_v.iter().copied().all(f32::is_finite));

    let coords = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 1.0, 0.0, //
                1.0, 1.0, 0.0, //
                2.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, //
                1.0, 0.0, 0.0, //
                2.0, 0.0, 0.0,
            ],
            Shape::new([6, 3]),
        ),
        &dev,
    );
    let n = 6usize;
    let ey = Tensor::<B, 3>::from_data(
        Data::new(
            vec![0.2_f32, -0.11, 0.37, 0.05, -0.29, 0.18],
            Shape::new([1, n, 1]),
        ),
        &dev,
    );
    let eps_r = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let f_hz = 1.5e9_f32;

    let chain_mv =
        apply_dec_te_curl_curl_chain_operator(ey, eps_r, edges_b1.clone(), coords.clone(), f_hz);
    assert!(
        chain_mv.is_none(),
        "two-quad assembled strip must not use the uniform-chain TE curl–curl matvec"
    );

    let mut e0 = vec![0.0_f32; n * 3];
    for i in 0..n {
        e0[i * 3] = 0.11 * i as f32;
        e0[i * 3 + 1] = -0.19 + 0.06 * i as f32;
        e0[i * 3 + 2] = 0.27 - 0.03 * i as f32;
    }
    let e_field = Tensor::<B, 3>::from_data(Data::new(e0, Shape::new([1, n, 3])), &dev);
    let eps_r3 = Tensor::<B, 3>::ones([1, n, 1], &dev);
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let j = Tensor::<B, 3>::zeros([1, n, 3], &dev);
    let cg = MechanicsInnerLoopConfig::default();
    let ps = PhotonicsSolver { frequency_hz: f_hz };
    let out = ps.solve_maxwell_curl_curl(e_field.clone(), eps_r3, eps_i, j, edges_b1, coords, &cg);
    let vi = out.into_data().value;
    let ei = e_field.into_data().value;
    assert_eq!(vi.len(), ei.len());
    let mut mx = 0.0_f32;
    for k in 0..vi.len() {
        mx = mx.max((vi[k] - ei[k]).abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 1e-6_f32, max_relative = 1.0);
}

/// Same check as [`dec_te_primal_tensor_matches_chain_stencil`], with **piecewise** \(\varepsilon_r\)
/// on nodes (harmonic means on links via [`primal_scalar_edge_increment`] / divergence path).
#[test]
fn dec_te_primal_piecewise_eps_matches_chain_stencil() {
    use umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator;

    let dev = device();
    let n = 43usize;
    let h = 1.8e-3_f32;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);
    let f_hz = 1.7e9_f32;
    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let k0 = omega / 2.998e8_f32;

    let mut eyv = vec![0.0_f32; n];
    for i in 0..n {
        eyv[i] = ((i * 11) as f32 * 0.017_f32).cos();
    }
    let i0 = n / 5;
    let i1 = 4 * n / 5;
    let mut eps_flat = vec![0.0_f32; n];
    let mut eps_c = vec![C::zero(); n];
    for i in 0..n {
        let er = if i < i0 {
            1.2_f32
        } else if i < i1 {
            3.8_f32
        } else {
            2.3_f32
        };
        eps_flat[i] = er;
        eps_c[i] = C { re: er, im: 0.0 };
    }

    let ey = Tensor::<B, 3>::from_data(Data::new(eyv.clone(), Shape::new([1, n, 1])), &dev);
    let eps_r = Tensor::<B, 3>::from_data(Data::new(eps_flat, Shape::new([1, n, 1])), &dev);

    let got = apply_dec_te_curl_curl_chain_operator(
        ey.clone(),
        eps_r,
        edges.clone(),
        coords.clone(),
        f_hz,
    )
    .expect("uniform chain");

    let e_c: Vec<C> = eyv.iter().map(|&re| C { re, im: 0.0 }).collect();
    let stencil = apply_te_helmholtz_chain(n, h, k0, &eps_c, &e_c);

    let got_v = got.into_data().value;
    let mut mx = 0.0_f32;
    for i in 0..n {
        mx = mx.max((got_v[i] - stencil[i].re).abs());
        mx = mx.max(stencil[i].im.abs());
    }
    assert_relative_eq!(mx, 0.0_f32, epsilon = 5e-5_f32, max_relative = 1.0);
}

/// Standing-wave proxy in vacuum left of a dielectric half-space; compare inferred |r|² to 1/9.
#[test]
fn fresnel_interface_standing_wave_proxy() {
    let dev = device();
    let n = 801usize;
    let f_hz = 500e12_f32;
    let c_light = 2.998e8_f32;
    let lambda = c_light / f_hz;
    let domain = 12.0 * lambda;
    let h = domain / (n - 1) as f32;

    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);

    let mid = ((n as f32) * 0.5).round() as usize;
    let eps_t = {
        let mut v = vec![1.0_f32; n];
        for i in mid..n {
            v[i] = 4.0;
        }
        Tensor::<B, 3>::from_data(Data::new(v, Shape::new([1, n, 1])), &dev)
    };
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);

    let src = mid / 5;
    let jre = vec![0.0_f32; n];
    let mut jim = vec![0.0_f32; n];
    jim[src] = 1.0;
    let jr = Tensor::<B, 3>::from_data(Data::new(jre, Shape::new([1, n, 1])), &dev);
    let ji = Tensor::<B, 3>::from_data(Data::new(jim, Shape::new([1, n, 1])), &dev);

    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let solver = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 96,
        pml_max_sigma: 3.5 * omega,
    };
    let cg = MechanicsInnerLoopConfig::default();
    let (er, ei) = solver.solve_helmholtz(eps_t, eps_i, jr, ji, edges, coords, &cg);

    let gr = er.into_data().value;
    let gi = ei.into_data().value;
    let lo = src + 32;
    let hi = mid.saturating_sub(40).max(lo + 8);
    let mut emax = 0.0_f32;
    let mut emin = 1e30_f32;
    for i in lo..=hi {
        let m = gr[i] * gr[i] + gi[i] * gi[i];
        emax = emax.max(m);
        emin = emin.min(m);
    }
    let num = emax - emin;
    let den = emax + emin;
    let swr_proxy = if den > 1e-20 { num / den } else { 0.0 };

    let r_analytic = (1.0_f32 - 2.0) / (1.0 + 2.0);
    let r2_target = r_analytic * r_analytic;
    // Loose smoke: nontrivial standing-wave contrast (full analytic Fresnel match is PML-sensitive).
    assert!(
        swr_proxy > 0.08 && swr_proxy < 0.95,
        "expected oscillating |E|² in vacuum window (swr_proxy={swr_proxy}, |r|²_analytic={r2_target})"
    );
}

#[test]
fn quarter_wave_stack_high_reflectivity() {
    let dev = device();
    let f_hz = 10e9_f32;
    let c_light = 2.998e8_f32;
    let lambda = c_light / f_hz;
    let n1 = 2.0_f32;
    let n2 = 1.0_f32;
    let d1 = lambda / (4.0 * n1);
    let d2 = lambda / (4.0 * n2);
    let bilayers = 10usize;
    let cells_per = 16usize;
    let cells_one = cells_per * bilayers * 2 + 2;
    let domain = bilayers as f32 * (d1 + d2) + 2.0 * lambda;
    let h = domain / (cells_one - 1) as f32;

    let n = cells_one;
    let edges = chain_edges(n);
    let coords = coords_line_x(n, h);

    let mut eps = vec![C { re: 1.0, im: 0.0 }; n];
    for i in 0..n {
        let x = i as f32 * h;
        let mut xi = 0.0_f32;
        let mut layer = 0usize;
        loop {
            let thick = if layer % 2 == 0 { d1 } else { d2 };
            if x < xi + thick || layer >= bilayers * 2 {
                eps[i].re = if layer % 2 == 0 { n1 * n1 } else { n2 * n2 };
                break;
            }
            xi += thick;
            layer += 1;
        }
    }

    let eps_t = Tensor::<B, 3>::from_data(
        Data::new(
            (0..n).map(|i| eps[i].re).collect::<Vec<_>>(),
            Shape::new([1, n, 1]),
        ),
        &dev,
    );
    let eps_i = Tensor::<B, 3>::zeros([1, n, 1], &dev);
    let jre = vec![0.0_f32; n];
    let mut jim = vec![0.0_f32; n];
    jim[n / 4] = 1.0;
    let jr = Tensor::<B, 3>::from_data(Data::new(jre, Shape::new([1, n, 1])), &dev);
    let ji = Tensor::<B, 3>::from_data(Data::new(jim, Shape::new([1, n, 1])), &dev);

    let omega = 2.0 * core::f32::consts::PI * f_hz;
    let solver = PhotonicsHelmholtzSolver {
        frequency_hz: f_hz,
        pml_thickness: 32,
        pml_max_sigma: 2.5 * omega,
    };
    let cg = MechanicsInnerLoopConfig::default();
    let (er, ei) = solver.solve_helmholtz(eps_t, eps_i, jr, ji, edges, coords, &cg);

    let gr = er.into_data().value;
    let gi = ei.into_data().value;
    let mut peak = 0.0_f32;
    for i in (n / 10)..(9 * n / 10) {
        let m = (gr[i] * gr[i] + gi[i] * gi[i]).sqrt();
        peak = peak.max(m);
    }
    assert!(
        peak > 1e-4,
        "expected non-trivial field in stack (peak {peak})"
    );
}

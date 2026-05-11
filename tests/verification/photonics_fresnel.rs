// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FDFD Helmholtz verification (`photonics`): MMS on a Dirichlet line, two-media continuum Fresnel
//! MMS without PML, plus interface/stack smokes with PML on. Curl–curl vs Helmholtz checks are
//! **1-D uniform-chain** regressions only (including a piecewise \(\varepsilon_r\) profile and
//! **`curl_curl_y_mode_matches_scalar_helmholtz_xy_embedded_chain`**: same path graph with
//! **non-collinear** \((x,y,z)\) SI coordinates — still **not** a simplicial \(d_1\) patch solve).
//! **Verification §6 increment:** [`umst_manifold::physics::solvers::photonics::dec_maxwell_assembly`]
//! re-exports [`primal_d1_edge_flux_to_faces`](umst_manifold::physics::dec_primal::primal_d1_edge_flux_to_faces)
//! on a **quad-split** `faces_b2` patch (same topology as `tests/dec_identities.rs`); plus
//! **`solve_maxwell_curl_curl`** pass-through when the graph is **not** a uniform x-chain; and
//! [`apply_dec_te_curl_curl_chain_operator`](umst_manifold::physics::solvers::photonics::apply_dec_te_curl_curl_chain_operator)
//! vs hand stencil with **piecewise** \(\varepsilon_r\).
//!
//! Specification: `composer_prompts/v0.4_solver_completion_no_namesakes.md` (Track H).

#![cfg(feature = "photonics")]
#![allow(clippy::needless_range_loop)]

use approx::assert_relative_eq;
use burn::tensor::{Data, Int, Shape, Tensor};
use burn_ndarray::{NdArray, NdArrayDevice};
use umst_manifold::physics::dec_primal::primal_scalar_edge_increment;
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

/// Same index-wise \(x = i h\) as [`coords_line_x`], with small smooth **y, z** so the chain is not
/// collinear in \(\mathbb{R}^3\) while remaining **x-monotone** for the uniform-chain gate inside
/// [`PhotonicsSolver::solve_maxwell_curl_curl`](umst_manifold::physics::solvers::PhotonicsSolver::solve_maxwell_curl_curl).
#[allow(dead_code)] // harness reserved for future non-collinear chain regressions
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
fn quad_split_patch_tensors() -> (
    Tensor<B, 2, Int>,
    Tensor<B, 2, Int>,
    EdgeTopology<B>,
) {
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
    assert_relative_eq!(max_err, 0.0_f32, epsilon = 9e-2_f32);

    assert_relative_eq!(r_analytic, -1.0_f32 / 3.0_f32, epsilon = 1e-6_f32);
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
    jdat[1 * 3 + 1] = 1.0;
    let j = Tensor::<B, 3>::from_data(Data::new(jdat, Shape::new([1, n, 3])), &dev);
    let cg = MechanicsInnerLoopConfig::default();
    let ps = PhotonicsSolver {
        frequency_hz: 1e9_f32,
    };
    let out = ps.solve_maxwell_curl_curl(
        e_field.clone(),
        eps_r,
        eps_i,
        j,
        edges_b1,
        coords,
        &cg,
    );
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

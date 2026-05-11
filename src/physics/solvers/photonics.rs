// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Frequency-domain **scalar Helmholtz** (FDFD-style) on a **1-D path graph** plus a **minimal primal DEC**
//! curl–curl path on the same graph class.
//!
//! ## Scope (curl–curl / Maxwell)
//! **Implemented:** frequency-domain TE with **\(E_y\)** as the only solved component on a **single
//! uniform x-monotone chain** (`extract_uniform_x_chain`). The discrete operator is the **1-D
//! primal reduction** of \(\nabla\times(\varepsilon_r^{-1}\nabla\times\cdot)\) on this skeleton,
//! which equals the **scalar Helmholtz** stencil already used by [`PhotonicsHelmholtzSolver::solve_helmholtz`]
//! (harmonic \(2/(\varepsilon_i+\varepsilon_{i+1})\) on half-links, \(1/h^2\) scaling, Dirichlet caps).
//! **Not implemented:** general **2D/3D** DEC with \(d_1\) (edge→face curl), dual mesh Hodge stars, or
//! full vector edge unknowns — those require `faces_b2` and metric machinery beyond this module.
//! Public deferral / next acceptance criteria: **`docs/Solver-Status.md`** section **DEFERRAL — Photonics**.
//!
//! **Regression coverage (1-D only):** [`tests/verification/photonics_fresnel.rs`](../../../tests/verification/photonics_fresnel.rs)
//! asserts that `solve_maxwell_curl_curl` and `solve_helmholtz` return the same \(E_y\) on a uniform
//! x-chain for both uniform and **piecewise-varying** \(\varepsilon_r\) profiles. That locks the shared
//! tridiagonal / half-link stencil — not a claim of equivalence on general simplicial patches.
//! **Default builds** (`photonics` feature **off**): [`tests/verification/photonics_curl_curl_stub_default_build.rs`](../../../tests/verification/photonics_curl_curl_stub_default_build.rs)
//! pins that [`PhotonicsSolver::solve_maxwell_curl_curl`] is an identity on representative chain tensors.
//!
//! ## Scalar TE weak form (phasor notation, \(\partial_t \rightarrow i\omega\))
//! \[
//!   \nabla \cdot (\varepsilon_r^{-1} \nabla E) + k_0^2 E = -i\omega\mu_0 J,\quad k_0=\omega/c.
//! \]
//!
//! On a 1-skeleton line graph with uniform spacing \(h\), central fluxes use harmonic averages
//! \(2/(\varepsilon_{i}+\varepsilon_{i+1})\) on half-edges. Complex \(\varepsilon\) encodes ohmic loss
//! (`eps_r_imag`) and **PML** conductivity via \(\varepsilon \leftarrow \varepsilon_r(1 - i\sigma/\omega)\)
//! [Berenger 1994; Rumpf 2022 §3.4].
//!
//! ## `edges_b1` layout (Burn / ndarray row-major)
//! `edges_b1` is `[2, E]`: row `0` = all source node ids, row `1` = all targets (same contract as
//! [`crate::physics::topology::EdgeTopology`]). Flattened data is **not** interleaved
//! `[s0,t0,s1,t1,…]`.
//!
//! ## Audit memo (Track H1)
//! - **Two-region / analytic vs PML:** `tests/verification/photonics_fresnel.rs` runs MMS on a
//!   Dirichlet-closed vacuum chain with PML off, `two_half_spaces_fresnel_te_no_pml_matches_analytic`
//!   (continuum Fresnel field + Dirichlet bridge, MMS source, no PML), then interface / stack smokes
//!   with PML on (loose tolerances).
//! - **Burn / complex:** Helmholtz uses a hand-rolled `f32` complex type and host-side Thomas; no
//!   native `burn` complex dtype — gradients through the solve are not the goal of this path.
//! - **Curl–curl:** [`PhotonicsSolver::solve_maxwell_curl_curl`] solves the same **TE \(E_y\)** tridiagonal
//!   system as Helmholtz via an explicit **primal-chain DEC** assembly (then Thomas), documented below;
//!   general vector DEC on simplicial \(d_1\) remains future work.
//!
//! ## `PhotonicsSolver::solve_maxwell_curl_curl`
//! On a **uniform x-monotone path chain** with `[N,3]` SI coordinates, the **discrete TE curl–curl**
//! for the transverse phasor \(E_y\) coincides with the scalar Helmholtz operator
//! \(\nabla\cdot(\varepsilon_r^{-1}\nabla E_y)+k_0^2 E_y\) on that chain. Implementation-wise:
//! - **Primal DEC (1-graph):** oriented \(d_0\) is the edge increment
//!   [`crate::physics::dec_primal::primal_scalar_edge_increment`]; flux on each edge uses the same
//!   FDFD half-link weight \(\eta_e = (1/h^2)\,2/(\varepsilon_{\mathrm{src}}+\varepsilon_{\mathrm{tgt}})\);
//!   the **codivergence** \(d_0^\top\) is [`crate::physics::dec_primal::primal_divergence_from_edge_flux_topo`].
//!   Interior rows are therefore \( (d_0^\top \,\mathrm{diag}(\eta_h)\, d_0 E)_i + k_0^2 E_i\) with
//!   \(\eta_h\) the per-edge scalar above (matches the existing Thomas coefficients).
//! - **Source:** \(-i\omega\mu_0 J_y\) encoded like [`PhotonicsHelmholtzSolver::solve_helmholtz`]
//!   from `impressed_current[:,:,1]` (real channel today; imag \(J_y\) channel is zero if absent).
//! - **Pass-through:** \(E_x,E_z\) are copied from the incoming `e_field`.
//!   **Non-chain** topologies: warns and returns `e_field` unchanged.
//!
//! ## `--features photonics`
//! [`PhotonicsHelmholtzSolver::solve_helmholtz`] runs the discrete Helmholtz solve (tridiagonal
//! complex Thomas) when the edge graph is a **single x-aligned chain** with uniform spacing;
//! otherwise it logs a warning and returns zeros.

#[cfg(feature = "photonics")]
use burn::tensor::Shape;
use burn::tensor::{backend::Backend, Data, Int, Tensor};

#[cfg(feature = "photonics")]
use crate::physics::dec_primal::{
    primal_divergence_from_edge_flux_topo, primal_scalar_edge_increment,
};
use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
#[cfg(feature = "photonics")]
use crate::physics::topology::EdgeTopology;

/// formal_anchor: Literature
/// formal_citation: Rumpf 2022, Computational Electromagnetics in MATLAB, §3.4 (FDFD); Berenger 1994 (PML); Taflove & Hagness 2005 (FDFD context)
/// formal_form: ∇·(ε_r⁻¹ ∇E) + k₀² E = −iω μ₀ J on a 1-D chain, with PML via complex ε stretching
pub struct PhotonicsHelmholtzSolver {
    pub frequency_hz: f32,
    /// Number of nodes at each end using a quadratic PML conductivity ramp (0 = disabled).
    pub pml_thickness: usize,
    /// Peak PML \(\sigma\) scale (rad/s); effective stretching uses \(\sigma/\omega\).
    pub pml_max_sigma: f32,
}

#[derive(Clone, Copy, Debug)]
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

impl PhotonicsHelmholtzSolver {
    /// formal_anchor: Literature
    /// formal_citation: Rumpf 2022, Computational Electromagnetics in MATLAB, §3.4 (FDFD); Berenger 1994 (PML)
    /// formal_form: ∇·(ε_r⁻¹ ∇E) + k₀² E = −iω μ₀ J  on the 1-skeleton, with PML at boundaries
    #[allow(clippy::too_many_arguments)]
    pub fn solve_helmholtz<B: Backend<FloatElem = f32>>(
        &self,
        eps_r_real: Tensor<B, 3>,
        eps_r_imag: Tensor<B, 3>,
        source_re: Tensor<B, 3>,
        source_im: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        coords_n3: Tensor<B, 2>,
        _cg: &MechanicsInnerLoopConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 3>) {
        let device = eps_r_real.device();
        let shape = eps_r_real.shape();
        let n = shape.dims[1];
        let zeros = Tensor::<B, 3>::zeros(shape.clone(), &device);

        let chain = match extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3) {
            Some(c) => c,
            None => {
                tracing::warn!(
                    target: "umst_manifold::photonics",
                    "solve_helmholtz: need a single x-monotone chain with uniform spacing; returning zeros"
                );
                return (zeros.clone(), zeros);
            }
        };

        let eps_rr = eps_r_real.clone().into_data().value;
        let eps_ri = eps_r_imag.into_data().value;
        let sr = source_re.into_data().value;
        let si = source_im.into_data().value;

        let (alpha, beta, gamma, rhs_t) = uniform_chain_te_tridiagonal_and_rhs(
            &chain,
            &eps_rr,
            &eps_ri,
            &sr,
            &si,
            self.frequency_hz,
            self.pml_thickness,
            self.pml_max_sigma,
        );

        let sol = thomas_complex(&alpha, &beta, &gamma, &rhs_t);

        let mut out_re = vec![0.0_f32; n];
        let mut out_im = vec![0.0_f32; n];
        for (k, &orig) in chain.order.iter().enumerate() {
            let ix = orig as usize;
            out_re[ix] = sol[k].re;
            out_im[ix] = sol[k].im;
        }

        let er = Tensor::<B, 3>::from_data(Data::new(out_re, shape.clone()), &device);
        let ei = Tensor::<B, 3>::from_data(Data::new(out_im, shape), &device);
        (er, ei)
    }
}

struct UniformChain {
    order: Vec<i64>,
    len: usize,
    h: f32,
}

fn pml_sigma_at(i: usize, n: usize, thick: usize, sigma_max: f32) -> f32 {
    if thick == 0 || sigma_max <= 0.0 {
        return 0.0;
    }
    let t = thick as f32;
    if i < thick {
        let u = (thick - i) as f32 / t;
        return sigma_max * u * u;
    }
    if i + thick >= n {
        let u = (i - (n - thick - 1)) as f32 / t;
        return sigma_max * u * u;
    }
    0.0
}

/// Tridiagonal complex Thomas: `alpha[i]*x[i-1] + beta[i]*x[i] + gamma[i]*x[i+1] = rhs[i]`.
fn thomas_complex(alpha: &[C], beta: &[C], gamma: &[C], rhs: &[C]) -> Vec<C> {
    let n = rhs.len();
    let mut cp = vec![C::zero(); n];
    let mut dp = vec![C::zero(); n];

    cp[0] = C::div(gamma[0], beta[0]);
    dp[0] = C::div(rhs[0], beta[0]);
    for i in 1..n {
        let denom = C::sub(beta[i], C::mul(alpha[i], cp[i - 1]));
        if i + 1 < n {
            cp[i] = C::div(gamma[i], denom);
        } else {
            cp[i] = C::zero();
        }
        dp[i] = C::div(C::sub(rhs[i], C::mul(alpha[i], dp[i - 1])), denom);
    }

    let mut x = vec![C::zero(); n];
    x[n - 1] = dp[n - 1];
    for i in (0..n - 1).rev() {
        x[i] = C::sub(dp[i], C::mul(cp[i], x[i + 1]));
    }
    x
}

fn extract_uniform_x_chain<B: Backend<FloatElem = f32>>(
    n: usize,
    edges: &Tensor<B, 2, Int>,
    coords: &Tensor<B, 2>,
) -> Option<UniformChain> {
    let e = edges.clone().float().into_data().value;
    let c = coords.clone().into_data().value;
    let n_edges = n.saturating_sub(1);
    if e.len() != n_edges * 2 {
        return None;
    }
    // Build adjacency (row-major [2, E]: sources e[0..E], targets e[E..2E]).
    let mut neigh: Vec<Vec<usize>> = vec![vec![]; n];
    for k in 0..n_edges {
        let a = (e[k] as i64) as usize;
        let b = (e[n_edges + k] as i64) as usize;
        if a >= n || b >= n {
            return None;
        }
        neigh[a].push(b);
        neigh[b].push(a);
    }
    for v in &neigh {
        if v.is_empty() || v.len() > 2 {
            return None;
        }
    }
    // Find an endpoint (degree 1)
    let mut start = None;
    for (i, v) in neigh.iter().enumerate() {
        if v.len() == 1 {
            start = Some(i);
            break;
        }
    }
    let start = start.or(if n == 1 { Some(0) } else { None })?;

    let mut order = Vec::with_capacity(n);
    let mut prev = usize::MAX;
    let mut cur = start;
    for _ in 0..n {
        order.push(cur as i64);
        let nx = neigh[cur]
            .iter()
            .copied()
            .find(|&j| j != prev)
            .unwrap_or(cur);
        if nx == cur {
            break;
        }
        prev = cur;
        cur = nx;
    }
    if order.len() != n {
        return None;
    }

    let mut xs = Vec::with_capacity(n);
    for &id in &order {
        let i = id as usize;
        xs.push(c[i * 3]);
    }
    for k in 1..n {
        if xs[k] <= xs[k - 1] + 1e-9 {
            return None;
        }
    }
    let h0 = xs[1] - xs[0];
    for k in 2..n {
        let hk = xs[k] - xs[k - 1];
        if (hk - h0).abs() > 1e-4 * h0.max(1e-12) {
            return None;
        }
    }

    Some(UniformChain {
        order,
        len: n,
        h: h0,
    })
}

/// TE operator on a uniform x-chain in **chain index order** \(k=0..L-1\): interior rows are
/// \((d_0^\top \,\mathrm{diag}(\eta)\, d_0 u)_k + k_0^2 u_k\) with \(\eta_e=(1/h^2)\,2/(\varepsilon_{k}+\varepsilon_{k+1})\)
/// on the edge between chain nodes \(k\to k+1\) (same half-link weights as FDFD / [`solve_helmholtz`]).
/// Endpoints are **Dirichlet** rows \(u_0\) and \(u_{L-1}\) (identity).
#[allow(dead_code, clippy::too_many_arguments)]
fn uniform_chain_te_tridiagonal_and_rhs(
    chain: &UniformChain,
    eps_rr: &[f32],
    eps_ri: &[f32],
    sr: &[f32],
    si: &[f32],
    frequency_hz: f32,
    pml_thickness: usize,
    pml_max_sigma: f32,
) -> (Vec<C>, Vec<C>, Vec<C>, Vec<C>) {
    let omega = 2.0 * core::f32::consts::PI * frequency_hz;
    let k0 = omega / 2.998e8_f32;
    let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
    let scale_j = omega * mu0;

    let mut rhs = vec![C::zero(); chain.len];
    for (k, &orig) in chain.order.iter().enumerate() {
        let ix = orig as usize;
        let jre = sr[ix];
        let jim = si[ix];
        rhs[k] = C {
            re: scale_j * jim,
            im: -scale_j * jre,
        };
    }

    let h = chain.h;
    let inv_h2 = 1.0 / (h * h);

    let mut eps_node = vec![C::zero(); chain.len];
    for (k, &orig) in chain.order.iter().enumerate() {
        let ix = orig as usize;
        let er = eps_rr[ix];
        let mut ei = eps_ri[ix];
        if pml_thickness > 0 && pml_max_sigma > 0.0 {
            let sigma = pml_sigma_at(k, chain.len, pml_thickness, pml_max_sigma);
            ei -= er * sigma / omega;
        }
        eps_node[k] = C { re: er, im: ei };
    }

    let mut alpha = vec![C::zero(); chain.len];
    let mut beta = vec![C::zero(); chain.len];
    let mut gamma = vec![C::zero(); chain.len];
    let mut rhs_t = rhs;

    for i in 0..chain.len {
        let k0c = C {
            re: k0 * k0,
            im: 0.0,
        };
        if i == 0 {
            beta[i] = C { re: 1.0, im: 0.0 };
            gamma[i] = C::zero();
            rhs_t[i] = C::zero();
            continue;
        }
        if i + 1 == chain.len {
            alpha[i] = C::zero();
            beta[i] = C { re: 1.0, im: 0.0 };
            rhs_t[i] = C::zero();
            continue;
        }

        let inv_eps_m = C::div(C { re: 2.0, im: 0.0 }, C::add(eps_node[i - 1], eps_node[i]));
        let inv_eps_p = C::div(C { re: 2.0, im: 0.0 }, C::add(eps_node[i], eps_node[i + 1]));
        alpha[i] = C::scale(inv_h2, inv_eps_m);
        gamma[i] = C::scale(inv_h2, inv_eps_p);
        beta[i] = C::add(
            C::scale(inv_h2, C::scale(-1.0, C::add(inv_eps_m, inv_eps_p))),
            k0c,
        );
    }

    (alpha, beta, gamma, rhs_t)
}

/// Phase 7 photonics driver: holds the **driving frequency** \(f\) (Hz) for phasor solves.
pub struct PhotonicsSolver {
    pub frequency_hz: f32,
}

impl PhotonicsSolver {
    /// Solve (or relax) the discrete curl–curl system for the electric field phasor.
    ///
    /// # Shapes (contract)
    /// - `e_field`: `[B, N, 3]` — electric field phasor components per node.
    /// - `relative_permittivity`: `[B, N, 1]` — relative permittivity \(\varepsilon_r\) (real part; extend later for tensors / loss).
    /// - `impressed_current`: `[B, N, 3]` — impressed current density \(\mathbf{J}\) (source term).
    /// - `edges_b1`: `[2, E]` — undirected edge pairs for the primal 1-skeleton (curl / gradient assembly).
    /// - Returns updated `e_field` `[B, N, 3]`.
    ///
    /// ## Default builds (`photonics` **off**)
    /// Returns `e_field` unchanged (documented no-op / Phase 7 stub).
    ///
    /// ## `--features photonics`
    /// **Uniform x-chain (batch \(B=1\)):** solves the **primal-chain DEC TE system** (same tridiagonal
    /// as [`PhotonicsHelmholtzSolver::solve_helmholtz`]) for \(E_y\) with `J_y=\mathrm{impressed\_current}[:,:,1]`;
    /// \(E_x,E_z\) pass through from `e_field`. Other topologies or \(B>1\): warns and returns `e_field` unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_maxwell_curl_curl<B: Backend<FloatElem = f32>>(
        &self,
        e_field: Tensor<B, 3>,
        relative_permittivity: Tensor<B, 3>,
        eps_r_imag: Tensor<B, 3>,
        impressed_current: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        coords_n3: Tensor<B, 2>,
        cg: &MechanicsInnerLoopConfig,
    ) -> Tensor<B, 3> {
        #[cfg(not(feature = "photonics"))]
        {
            let _ = (
                relative_permittivity,
                eps_r_imag,
                impressed_current,
                edges_b1,
                coords_n3,
                cg,
            );
            e_field
        }

        #[cfg(feature = "photonics")]
        {
            let _ = cg;
            let d = e_field.dims();
            if d.len() != 3 || d[2] != 3 {
                tracing::warn!(
                    target: "umst_manifold::photonics",
                    "solve_maxwell_curl_curl: expected e_field [B,N,3]; returning unchanged"
                );
                return e_field;
            }
            let n = d[1];
            if relative_permittivity.dims()[1] != n
                || eps_r_imag.dims()[1] != n
                || impressed_current.dims() != d
                || coords_n3.dims() != [n, 3]
            {
                tracing::warn!(
                    target: "umst_manifold::photonics",
                    "solve_maxwell_curl_curl: shape mismatch (permittivity / coords); returning e_field unchanged"
                );
                return e_field;
            }
            let chain = match extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3) {
                Some(c) => c,
                None => {
                    tracing::warn!(
                        target: "umst_manifold::photonics",
                        "solve_maxwell_curl_curl: need uniform x-chain; returning e_field unchanged"
                    );
                    return e_field;
                }
            };

            if d[0] != 1 {
                tracing::warn!(
                    target: "umst_manifold::photonics",
                    "solve_maxwell_curl_curl: only batch size 1 is supported; returning e_field unchanged"
                );
                return e_field;
            }

            let eps_rr = relative_permittivity.clone().into_data().value;
            let eps_ri = eps_r_imag.clone().into_data().value;
            let j_flat = impressed_current.clone().into_data().value;
            let mut sr = vec![0.0_f32; n];
            let si = vec![0.0_f32; n];
            for ix in 0..n {
                sr[ix] = j_flat[ix * 3 + 1];
            }

            let (alpha, beta, gamma, rhs_t) = uniform_chain_te_tridiagonal_and_rhs(
                &chain,
                &eps_rr,
                &eps_ri,
                &sr,
                &si,
                self.frequency_hz,
                0,
                0.0,
            );
            let sol = thomas_complex(&alpha, &beta, &gamma, &rhs_t);

            let mut out_re = vec![0.0_f32; n];
            for (k, &orig) in chain.order.iter().enumerate() {
                let ix = orig as usize;
                out_re[ix] = sol[k].re;
            }
            let device = e_field.device();
            let shape_ey = Shape::new([d[0], n, 1]);
            let ey_re = Tensor::<B, 3>::from_data(Data::new(out_re, shape_ey), &device);
            let ex = e_field.clone().narrow(2, 0, 1);
            let ez = e_field.clone().narrow(2, 2, 1);
            Tensor::cat(vec![ex, ey_re, ez], 2)
        }
    }
}

/// Primal DEC matvec for the **TE \(E_y\)** reduced operator on a **uniform x-chain** (real \(\varepsilon_r\) only).
///
/// Interior nodes use \((1/h^2)\, d_0^\top \,\mathrm{diag}\bigl(2/(\varepsilon_{\mathrm{src}}+\varepsilon_{\mathrm{tgt}})\bigr)\, d_0 + k_0^2 I\) with [`primal_scalar_edge_increment`] as \(d_0\) and [`primal_divergence_from_edge_flux_topo`] as \(d_0^\top\). Endpoints use the same **Dirichlet identity** rows as [`PhotonicsHelmholtzSolver::solve_helmholtz`].
///
/// **Scope:** \(B=1\), one \(E_y\) channel (`ey` shape `[1,N,1]`). Not a general 2D/3D \(d_1\) curl. Returns `None` if the graph is not a uniform x-monotone chain.
#[cfg(feature = "photonics")]
pub fn apply_dec_te_curl_curl_chain_operator<B: Backend<FloatElem = f32>>(
    ey: Tensor<B, 3>,
    relative_permittivity: Tensor<B, 3>,
    edges_b1: Tensor<B, 2, Int>,
    coords_n3: Tensor<B, 2>,
    frequency_hz: f32,
) -> Option<Tensor<B, 3>> {
    let d = ey.dims();
    if d.len() != 3 || d[2] != 1 || d[0] != 1 {
        return None;
    }
    let n = d[1];
    if relative_permittivity.dims() != [1, n, 1] || coords_n3.dims() != [n, 3] {
        return None;
    }
    let chain = extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3)?;
    let h = chain.h;
    let inv_h2 = 1.0 / (h * h);
    let omega = 2.0 * core::f32::consts::PI * frequency_hz;
    let k0 = omega / 2.998e8_f32;

    let topo = EdgeTopology::new(edges_b1);
    let d0 = primal_scalar_edge_increment(ey.clone(), &topo);
    let (src_eps, tgt_eps) = topo.gather_endpoints(relative_permittivity);
    let denom = src_eps.add(tgt_eps).clamp_min(1e-12_f32);
    let inv_w = Tensor::<B, 3>::ones_like(&denom)
        .mul_scalar(2.0_f32)
        .div(denom);
    let flux = d0.mul(inv_w);
    let template = Tensor::zeros_like(&ey);
    let div = primal_divergence_from_edge_flux_topo(flux, &topo, &template);
    let lap = div.mul_scalar(inv_h2);
    let out = lap.add(ey.clone().mul_scalar(k0 * k0));

    let device = ey.device();
    let shape = ey.shape();
    let left = *chain.order.first()? as usize;
    let right = chain.order[chain.len - 1] as usize;

    let mut o = out.into_data().value;
    let e = ey.into_data().value;
    for &ix in &[left, right] {
        if ix < n {
            o[ix] = e[ix];
        }
    }
    Some(Tensor::from_data(Data::new(o, shape), &device))
}

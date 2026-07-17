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
//! **Tensor \(\varepsilon\) (stub):** `relative_permittivity` may be `[B,N,1]` (scalar \(\varepsilon_r\)) or
//! **`[B,N,9]`** with row-major **3×3** symmetric storage per node
//! \([\varepsilon_{xx},\varepsilon_{xy},\varepsilon_{xz},\varepsilon_{yx},\varepsilon_{yy},\varepsilon_{yz},\varepsilon_{zx},\varepsilon_{zy},\varepsilon_{zz}]\).
//! The shipped **TE \(E_y\)** chain path uses **\(\varepsilon_{yy}\)** only (indices `4` in the 9-channel slice);
//! off-diagonal coupling in Maxwell is **not** modelled — this is a documented reduction toward full tensor DEC.
//!
//! **Not production-complete:** circumcentric/barycentric **dual** Hodge refinement, **sparse assembled**
//! Maxwell at scale (the shipped patch path uses **CSR matvec CG** as the default lossless inner solve when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`] with **`UMST_PHOTONICS_DEC_PATCH_CSR_INNER=auto`**, **O(dim²)** COO assembly — **dense Gauss–Jordan** only as fallback under [`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`] when CSR fails or CSR is **`off`** — else **matrix-free CG** up to [`PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV`]),
//! **complex** \(\varepsilon\) / PML on **`faces_b2`** patches, and general volumetric **BC**s beyond the
//! gauge **pin** remain open (see **OPEN ROADMAP ITEM — Photonics**).
//! **Verification matrix row #6** ([`Solver-Status.md`](../../../docs/Solver-Status.md))
//! therefore stays **partial** (**50%** bin in [`Solver-Status.md`](../../../docs/Solver-Status.md)) until
//! full dual-mesh metrics, **sparse factorizations / preconditioners** at production \(N\),
//! and complex \(\varepsilon\) / PML on that path land with CI-backed acceptance — see the predicate
//! [`photonics_dec_patch_uses_metric_dual_edge_hodge`] ( **`photonics`** feature; diagonal primal-length \(\star_1\) **wired**)
//! and the **DEC honesty /
//! 3D sequencing** memo [`PHOTONICS_DEC_3D_ROADMAP.md`](../../../docs/PHOTONICS_DEC_3D_ROADMAP.md) (uniform chain vs
//! `faces_b2` patch vs volumetric roadmap; same flip rule as the predicate).
//!
//! ### Track 15 / photonics lane (explicit scope boundary)
//!
//! **In this module (`photonics` on):**
//! - **Uniform x-monotone chain:** TE \(E_y\) via Thomas (same tridiagonal as [`PhotonicsHelmholtzSolver::solve_helmholtz`]).
//! - **Small `faces_b2` patch (optional [`PhotonicsDecFacesPatch`]):** real **vector** \(\mathbf{E}\in\mathbb{R}^{3N}\)
//!   **3D curl–curl** assembly on an **embedded 2-chain** in \(\mathbb{R}^3\): primal **grad–div** per diagonal
//!   \(\varepsilon\) channel on edges (still **diagonal** entries \(\varepsilon_{xx},\varepsilon_{yy},\varepsilon_{zz}\) for the
//!   FDFD-style flux weights), **\(d_1^\top d_1\)** on a **Whitney-style** edge trace: for **`[B,N,9]`** tensors,
//!   [`dec_patch_maxwell_natural_matvec_flat`] applies the **symmetric edge average** of the full **3×3** \(\varepsilon\) to the
//!   midpoint nodal field **before** the tangential projection \(t\cdot(\cdot)\) into the curl pipeline, with the matching
//!   adjoint scatter (documented surrogate toward \(\varepsilon\)-weighted 1-forms — **not** \(\varepsilon^{-1}\) on the curl leg
//!   yet), plus diagonal **\(\star_1=\ell_e\)** (symmetric \(\sqrt{\star_1}\) sandwich; see
//!   [`dec_patch_diagonal_star1_primal_edge_length_lumped_si`]) from SI **`coords_n3`**, nodal **\(k_0^2\,\varepsilon\)** (**3×3**
//!   tensor or scalar), and **gauge pin** at node `0`. Scalar **`[B,N,1]`** patches keep the prior curl leg (no \(\varepsilon\)
//!   in the Whitney trace).
//!   Topology is checked with the same **`faces_b2`** COO as [`tests/dec_identities.rs`](../../../tests/dec_identities.rs),
//!   including a cheap **\(d_1\!\circ\!d_0\approx 0\)** witness via [`dec_primal_max_abs_d1_of_scalar_gradient`].
//!   **Solve:** **CSR matvec CG** first when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`] and **`UMST_PHOTONICS_DEC_PATCH_CSR_INNER`** is not `off` (`auto` default); Gauss–Jordan on \(3N\) unknowns as fallback when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`] (overridable to **0**
//!   via `UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV=1` for tests), then **CSR** retry when **`auto`** and dense failed, else **capped
//!   matrix-free CG** up to [`PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV`] with [`PHOTONICS_DEC_PATCH_KRYLOV_MAX_ITERS`]
//!   — not a sparse-factorized production volumetric path.
//!
//! **Shipped elsewhere (same repo, not called from this solver path):**
//! - **DEC \(d_1\!\circ\!d_0=0\) and unweighted \(d_1^\top\) adjoint** on one CCW triangle **and** on the
//!   **closed boundary** of a canonical **3-simplex** (four faces, six edges) via
//!   [`crate::physics::dec_primal::canonical_tetrahedron_boundary_dec_coo`] — Burn tensors shaped like
//!   [`crate::core::tensors::UnifiedMaterialStateTensor::faces_b2`]:
//!   [`tests/dec_identities.rs`](../../../tests/dec_identities.rs)
//!   (`dec_curl_d1_annihilates_gradient_on_triangle_faces_b2_burn`,
//!   `dec_primal_d1_adjoint_identity_single_triangle_burn`,
//!   `dec_curl_d1_annihilates_gradient_tetrahedron_boundary_burn`,
//!   `dec_primal_d1_adjoint_identity_tetrahedron_boundary_burn`) — scope stops at [`crate::physics::dec_primal`];
//!   does **not** wire `faces_b2` into [`PhotonicsSolver`] **except** via optional [`PhotonicsDecFacesPatch`]
//!   on the **small dense** patch path in [`PhotonicsSolver::solve_maxwell_curl_curl`].
//! - **Fresnel / Helmholtz MMS + curl–curl vs scalar checks:** [`tests/verification/photonics_fresnel.rs`](../../../tests/verification/photonics_fresnel.rs).
//!   `two_half_spaces_fresnel_te_no_pml_matches_analytic` compares the discrete solve to a **Dirichlet-linear-bridged
//!   continuum** Fresnel field (nodal LS) and asserts a **discrete-only `r_disc`** (multi-point LS fit of \( \pm k_1 \) waves after
//!   bridge inversion) near the analytic Fresnel \(r\); tighter semi-infinite calibration remains optional follow-up.
//!
//! **Regression coverage (1-D only):** [`tests/verification/photonics_fresnel.rs`](../../../tests/verification/photonics_fresnel.rs)
//! asserts that `solve_maxwell_curl_curl` and `solve_helmholtz` return the same \(E_y\) on a uniform
//! x-chain for both uniform and **piecewise-varying** \(\varepsilon_r\) profiles; **`curl_curl_y_mode_matches_scalar_helmholtz_xy_embedded_chain`**
//! repeats the parity check with **non-collinear** \((x,y,z)\) SI coordinates on the same path graph
//! (still **not** a \(d_1\) patch solve). That locks the shared
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
//! - **Curl–curl:** [`PhotonicsSolver::solve_maxwell_curl_curl`] solves the **TE \(E_y\)** tridiagonal
//!   system as Helmholtz via an explicit **primal-chain DEC** assembly (then Thomas), and optionally a
//!   **small-patch** vector DEC assembly when [`PhotonicsDecFacesPatch`] is supplied; general sparse
//!   **3D** production Maxwell remains future work.
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
//! - **Pass-through:** \(E_x,E_z\) are copied from the incoming `e_field` on the **chain** path only.
//!   **Non-chain** without a valid [`PhotonicsDecFacesPatch`] (or with **`faces_b2`** that fails structural /
//!   \(d_1\!\circ\!d_0\) checks, or with \(N\) above the Krylov cap): warns and returns `e_field` unchanged.
//!
//! ## `--features photonics`
//! [`PhotonicsHelmholtzSolver::solve_helmholtz`] runs the discrete Helmholtz solve (tridiagonal
//! complex Thomas) when the edge graph is a **single x-aligned chain** with uniform spacing;
//! otherwise it logs a warning and returns zeros.

#![cfg_attr(feature = "photonics", allow(dead_code))]

#[cfg(feature = "photonics")]
use burn::tensor::Shape;
use burn::tensor::{backend::Backend, Data, Int, Tensor};

#[cfg(feature = "photonics")]
use crate::physics::dec_primal::{
    dec_primal_max_abs_d1_of_scalar_gradient, primal_divergence_from_edge_flux_topo,
    primal_scalar_edge_increment,
};
use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
use crate::physics::PhysicsError;
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
    ) -> Result<(Tensor<B, 3>, Tensor<B, 3>), PhysicsError> {
        let device = eps_r_real.device();
        let shape = eps_r_real.shape();
        let n = shape.dims[1];
        let chain = match extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3) {
            Some(c) => c,
            None => {
                return Err(PhysicsError::UnsupportedLayout {
                    context: "solve_helmholtz: uniform x-monotone chain required",
                });
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
        Ok((er, ei))
    }
}

struct UniformChain {
    order: Vec<i64>,
    len: usize,
    h: f32,
}

/// Row-major nodal **3×3** relative permittivity: channel **`4`** is \(\varepsilon_{yy}\) (TE stub).
#[cfg(feature = "photonics")]
const RELATIVE_PERMITTIVITY_CHANNELS_SCALAR: usize = 1;
#[cfg(feature = "photonics")]
const RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3: usize = 9;
#[cfg(feature = "photonics")]
const EPS_TENSOR_YY: usize = 4;

/// Extract per-node **real** \(\varepsilon_r\) for the TE \(E_y\) chain reduction: scalar channel **or**
/// tensor channel layout `[B,N,9]` using **\(\varepsilon_{yy}\)** only ([`EPS_TENSOR_YY`]).
#[cfg(feature = "photonics")]
fn nodal_eps_r_real_for_te_chain<B: Backend<FloatElem = f32>>(
    relative_permittivity: &Tensor<B, 3>,
    batch: usize,
    n: usize,
) -> Option<Vec<f32>> {
    let d = relative_permittivity.dims();
    if d.len() != 3 || d[0] != batch || d[1] != n {
        return None;
    }
    let flat = relative_permittivity.clone().into_data().value;
    match d[2] {
        RELATIVE_PERMITTIVITY_CHANNELS_SCALAR => {
            if flat.len() != batch * n {
                return None;
            }
            if batch != 1 {
                return None;
            }
            Some(flat)
        }
        RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3 => {
            if flat.len() != batch * n * RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3 {
                return None;
            }
            if batch != 1 {
                return None;
            }
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                let base = i * RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3 + EPS_TENSOR_YY;
                v.push(flat[base]);
            }
            Some(v)
        }
        _ => None,
    }
}

#[cfg(feature = "photonics")]
fn scalar_eps_channel_for_dec<B: Backend<FloatElem = f32>>(
    relative_permittivity: Tensor<B, 3>,
) -> Result<Tensor<B, 3>, PhysicsError> {
    let d = relative_permittivity.dims();
    if d.len() != 3 || d[0] != 1 {
        return Err(PhysicsError::UnsupportedLayout {
            context: "scalar_eps_channel_for_dec: expected batch=1 [1,N,C]",
        });
    }
    match d[2] {
        RELATIVE_PERMITTIVITY_CHANNELS_SCALAR => Ok(relative_permittivity),
        RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3 => {
            Ok(relative_permittivity.narrow(2, EPS_TENSOR_YY, 1))
        }
        _ => Err(PhysicsError::UnsupportedLayout {
            context: "scalar_eps_channel_for_dec: unsupported channel count",
        }),
    }
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
    // f32-safe: infer h from total x-span (translation-invariant; robust for affine x0 + j·h).
    let h = (xs[n - 1] - xs[0]) / (n - 1) as f32;
    if h <= 0.0 {
        return None;
    }
    let rtol = 1e-2_f32;
    for k in 1..n {
        let hk = xs[k] - xs[k - 1];
        if (hk - h).abs() > rtol * h.abs().max(1e-12) {
            return None;
        }
    }

    Some(UniformChain {
        order,
        len: n,
        h,
    })
}

/// TE operator on a uniform x-chain in **chain index order** \(k=0..L-1\): interior rows are
/// \((d_0^\top \,\mathrm{diag}(\eta)\, d_0 u)_k + k_0^2 u_k\) with \(\eta_e=(1/h^2)\,2/(\varepsilon_{k}+\varepsilon_{k+1})\)
/// on the edge between chain nodes \(k\to k+1\) (same half-link weights as FDFD / [`PhotonicsHelmholtzSolver::solve_helmholtz`]).
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

/// **2D/3D-embedded simplicial 2-cell** bundle for DEC \(d_1\) / \(d_1^\top\) on `faces_b2` (same COO
/// contract as [`crate::core::tensors::UnifiedMaterialStateTensor::faces_b2`]).
///
/// When passed to [`PhotonicsSolver::solve_maxwell_curl_curl`], the solver may take a **small-patch
/// direct** path (host dense Gaussian, \(3N\) unknowns) instead of the uniform **x-chain** TE reduction.
/// `face_column_ranges` partitions columns of `faces_b2` into faces (see [`crate::physics::dec_primal::primal_d1_edge_flux_to_faces`]).
#[derive(Clone, Copy, Debug)]
pub struct PhotonicsDecFacesPatch<'a, B: Backend> {
    pub faces_b2: &'a Tensor<B, 2, Int>,
    pub face_column_ranges: &'a [(usize, usize)],
}

/// Inner-solve policy for the **lossless** DEC patch path (CSR vs dense Gauss–Jordan vs matrix-free CG).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DecPatchCsrInnerMode {
    #[default]
    Auto,
    On,
    Off,
}

/// Injected knobs for the **small dense** [`PhotonicsDecFacesPatch`] solve branch.
///
/// Pure config — **no `std::env` reads** in the physics core. Host / CLI layers may parse
/// `UMST_PHOTONICS_DEC_PATCH_*` and construct this struct at the orchestrator boundary.
#[derive(Clone, Copy, Debug)]
pub struct PhotonicsDecPatchConfig {
    /// When `true`, the effective dense node cap is **0**, forcing CSR / matrix-free Krylov fallbacks.
    pub force_krylov: bool,
    /// CSR matvec CG inner solve policy on the lossless gauge-pinned patch operator.
    pub csr_inner: DecPatchCsrInnerMode,
}

impl Default for PhotonicsDecPatchConfig {
    fn default() -> Self {
        Self {
            force_krylov: false,
            csr_inner: DecPatchCsrInnerMode::Auto,
        }
    }
}

impl PhotonicsDecPatchConfig {
    /// Lossless patch: CSR matvec CG first when \(N\le\) CSR assembly cap (`auto` default).
    #[must_use]
    pub const fn lossless_auto() -> Self {
        Self {
            force_krylov: false,
            csr_inner: DecPatchCsrInnerMode::Auto,
        }
    }

    /// Lossless patch: dense Gauss–Jordan only (skip CSR inner).
    #[must_use]
    pub const fn dense_only() -> Self {
        Self {
            force_krylov: false,
            csr_inner: DecPatchCsrInnerMode::Off,
        }
    }

    /// Lossless patch: skip dense fallback (effective dense cap **0**); CSR / matrix-free Krylov only.
    #[must_use]
    pub const fn force_krylov() -> Self {
        Self {
            force_krylov: true,
            csr_inner: DecPatchCsrInnerMode::Auto,
        }
    }
}

/// Column ranges for the **two-quad strip** uniform brick (`6` nodes / `9` edges / `4` triangles) —
/// same incidence as [`photonics_uniform_brick_two_quad_strip_tensors`].
#[cfg(feature = "photonics")]
pub const UNIFORM_BRICK_TWO_QUAD_STRIP_FACE_RANGES: [(usize, usize); 4] =
    [(0, 3), (3, 6), (6, 9), (9, 12)];

/// Column ranges for [`photonics_uniform_brick_tetrahedron_boundary_tensors`] / [`crate::physics::dec_primal::canonical_tetrahedron_boundary_dec_coo`].
#[cfg(feature = "photonics")]
pub const UNIFORM_BRICK_TETRAHEDRON_BOUNDARY_FACE_RANGES: [(usize, usize); 4] =
    [(0, 3), (3, 6), (6, 9), (9, 12)];

/// **P4B — volumetric 2D brick:** two unit squares side-by-side (six nodes), each split into two CCW
/// triangles — same `edges_b1` / `faces_b2` COO as `two_quads_shared_edge` integration tests.
/// SI vertex coordinates are scaled by `cell_h` (metres); the patch lies in the \(z=0\) plane with
/// \(x\in[0,2h]\), \(y\in[0,h]\).
#[cfg(feature = "photonics")]
pub fn photonics_uniform_brick_two_quad_strip_tensors<
    B: Backend<FloatElem = f32, IntElem = i64>,
>(
    cell_h: f32,
    device: &B::Device,
) -> (Tensor<B, 2, Int>, Tensor<B, 2, Int>, Tensor<B, 2>) {
    let h = cell_h.max(1e-9_f32);
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 1, 2, 5, 4, 3, 0, 1, 1, //
                1, 2, 5, 4, 3, 0, 4, 5, 4,
            ],
            Shape::new([2, 9]),
        ),
        device,
    );
    let faces_b2: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(
            vec![
                0i64, 8, 6, 6, 4, 5, 1, 2, 7, 7, 3, 8, //
                1, 1, -1, 1, 1, 1, 1, 1, -1, 1, 1, -1,
            ],
            Shape::new([2, 12]),
        ),
        device,
    );
    let coords = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32 * h,
                1.0 * h,
                0.0, //
                1.0 * h,
                1.0 * h,
                0.0, //
                2.0 * h,
                1.0 * h,
                0.0, //
                0.0 * h,
                0.0 * h,
                0.0, //
                1.0 * h,
                0.0 * h,
                0.0, //
                2.0 * h,
                0.0 * h,
                0.0,
            ],
            Shape::new([6, 3]),
        ),
        device,
    );
    (edges_b1, faces_b2, coords)
}

/// **P4B — volumetric 3D brick (tetrahedron):** canonical **3-simplex** boundary — four triangular faces,
/// six edges, **four** nodes at \((0,0,0)\), \((h,0,0)\), \((0,h,0)\), \((0,0,h)\) so \(\det>0\) matches
/// [`crate::physics::dec_primal::canonical_tetrahedron_boundary_dec_coo`]. Passes `dec_patch_topology_valid_for_solve`
/// when wired into [`PhotonicsSolver::solve_maxwell_curl_curl`] with [`PhotonicsDecFacesPatch`].
#[cfg(feature = "photonics")]
pub fn photonics_uniform_brick_tetrahedron_boundary_tensors<
    B: Backend<FloatElem = f32, IntElem = i64>,
>(
    cell_h: f32,
    device: &B::Device,
) -> (Tensor<B, 2, Int>, Tensor<B, 2, Int>, Tensor<B, 2>) {
    let h = cell_h.max(1e-9_f32);
    let coo = crate::physics::dec_primal::canonical_tetrahedron_boundary_dec_coo();
    let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(coo.edges_b1_flat.to_vec(), Shape::new([2, 6])),
        device,
    );
    let faces_b2: Tensor<B, 2, Int> = Tensor::from_data(
        Data::new(coo.faces_b2_flat.to_vec(), Shape::new([2, 12])),
        device,
    );
    let coords = Tensor::from_data(
        Data::new(
            vec![
                0.0_f32, 0.0, 0.0, //
                h, 0.0, 0.0, //
                0.0, h, 0.0, //
                0.0, 0.0, h,
            ],
            Shape::new([4, 3]),
        ),
        device,
    );
    (edges_b1, faces_b2, coords)
}

/// **Hard cap** on nodal count for the shipped **dense** DEC-patch Maxwell solve (gauge-pinned \(3\times 3\) system).
#[cfg(feature = "photonics")]
pub const PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT: usize = 64;

/// **Hard cap** on nodal count for the **matrix-free CG** fallback on the same `faces_b2` patch operator
/// (still **batch 1**, real \(\varepsilon\); no sparse matrix assembly).
#[cfg(feature = "photonics")]
pub const PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV: usize = 512;

/// **Hard cap** on nodal count for **O(dim²)** COO column-probe assembly feeding **CSR matvec CG** on the
/// lossless patch path (see [`dec_patch_try_csr_inner_lossless`] / `UMST_PHOTONICS_DEC_PATCH_CSR_INNER`).
#[cfg(feature = "photonics")]
pub const PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY: usize = 128;

/// Maximum **inner** CG iterations on the patch system (early exit on relative residual).
#[cfg(feature = "photonics")]
pub const PHOTONICS_DEC_PATCH_KRYLOV_MAX_ITERS: usize = 512;

/// Structural checks plus a cheap **\(d_1\!\circ\!d_0\approx 0\)** witness on scalar nodal data (excludes
/// **N** solve caps, `eps_r_imag`, and matrix singularity).
#[cfg(feature = "photonics")]
fn dec_patch_topology_valid_for_solve<B: Backend<FloatElem = f32>>(
    n: usize,
    edges_b1: &Tensor<B, 2, Int>,
    patch: &PhotonicsDecFacesPatch<'_, B>,
) -> bool {
    let fd = patch.faces_b2.dims();
    if fd.len() != 2 || fd[0] != 2 {
        tracing::warn!(
            target: "umst_manifold::photonics",
            "dec_patch_topology_valid_for_solve: faces_b2 must be [2, K]"
        );
        return false;
    }
    let kcols = fd[1];
    if patch.face_column_ranges.is_empty() {
        tracing::warn!(
            target: "umst_manifold::photonics",
            "dec_patch_topology_valid_for_solve: face_column_ranges is empty"
        );
        return false;
    }
    for &(s, e) in patch.face_column_ranges {
        if s > e || e > kcols {
            tracing::warn!(
                target: "umst_manifold::photonics",
                "dec_patch_topology_valid_for_solve: invalid face column range [{s}, {e}) for K={kcols}"
            );
            return false;
        }
    }
    let n_edges = edges_b1.dims()[1];
    let edges_f = edges_b1.clone().float().into_data().value;
    if edges_f.len() != n_edges * 2 {
        tracing::warn!(
            target: "umst_manifold::photonics",
            "dec_patch_topology_valid_for_solve: edges_b1 data length mismatch"
        );
        return false;
    }
    for k in 0..n_edges {
        let a = edges_f[k] as usize;
        let b = edges_f[n_edges + k] as usize;
        if a >= n || b >= n {
            tracing::warn!(
                target: "umst_manifold::photonics",
                "dec_patch_topology_valid_for_solve: edge endpoint index out of range"
            );
            return false;
        }
    }
    let faces_f = patch.faces_b2.clone().float().into_data().value;
    if faces_f.len() != kcols * 2 {
        tracing::warn!(
            target: "umst_manifold::photonics",
            "dec_patch_topology_valid_for_solve: faces_b2 data length mismatch"
        );
        return false;
    }
    let faces_edge: Vec<i64> = faces_f[..kcols].iter().map(|&x| x as i64).collect();
    for &eid in &faces_edge {
        if eid < 0 || (eid as usize) >= n_edges {
            tracing::warn!(
                target: "umst_manifold::photonics",
                "dec_patch_topology_valid_for_solve: faces_b2 edge id out of range"
            );
            return false;
        }
    }

    let device = edges_b1.device();
    let omega_v: Vec<f32> = (0..n)
        .map(|i| (i as f32).mul_add(0.37, -1.1).sin())
        .collect();
    let nodal = Tensor::<B, 3>::from_data(Data::new(omega_v, Shape::new([1, n, 1])), &device);
    let topo = EdgeTopology::new(edges_b1.clone());
    let mx = dec_primal_max_abs_d1_of_scalar_gradient(
        nodal,
        &topo,
        patch.faces_b2.clone(),
        patch.face_column_ranges,
    );
    if !mx.is_finite() || mx > 5e-3_f32 {
        tracing::warn!(
            target: "umst_manifold::photonics",
            "dec_patch_topology_valid_for_solve: primal d1∘d0 witness failed (|d1(d0 ω)|∞ = {mx:.3e}); faces_b2 may not match edges_b1 / orientations"
        );
        return false;
    }
    true
}

/// Whether the **small dense** DEC patch in [`PhotonicsSolver::solve_maxwell_curl_curl`] uses a
/// **metric-weighted dual-edge Hodge** \(\star_1\) in the curl–curl stack (\(d_1^\top \star_1 d_1\) on
/// 1-forms / edge fluxes).
///
/// **Shipped value:** **`true`**. [`dec_patch_maxwell_natural_matvec_flat`] applies a **diagonal
/// primal-edge-length lump** \(\bigl(\star_1\bigr)_{ee}=\ell_e\) from
/// [`dec_patch_primal_edge_lengths_si`], in a **symmetric sandwich**
/// \(\sqrt{\star_1}\, d_1^\top d_1\, \sqrt{\star_1}\) on the edge-trace flux before \(d_1\) and after
/// \(d_1^\top\) (see [`dec_patch_diagonal_star1_primal_edge_length_lumped_si`]). This is **not**
/// circumcentric/barycentric dual lengths, **not** a sparse production solve, and **not** matrix
/// **#6** closure — verification row **#6** stays **partial** per
/// [`Solver-Status.md`](../../../docs/Solver-Status.md).
#[cfg(feature = "photonics")]
#[must_use]
pub const fn photonics_dec_patch_uses_metric_dual_edge_hodge() -> bool {
    true
}

/// Phase 7 photonics driver: holds the **driving frequency** \(f\) (Hz) for phasor solves.
pub struct PhotonicsSolver {
    pub frequency_hz: f32,
    /// DEC patch inner-solve policy (CSR vs dense vs Krylov). Ignored when `photonics` feature is off.
    pub dec_patch_config: PhotonicsDecPatchConfig,
}

impl Default for PhotonicsSolver {
    fn default() -> Self {
        Self {
            frequency_hz: 0.0,
            dec_patch_config: PhotonicsDecPatchConfig::default(),
        }
    }
}

impl PhotonicsSolver {
    /// Solve (or relax) the discrete curl–curl system for the electric field phasor.
    ///
    /// # Shapes (contract)
    /// - `e_field`: `[B, N, 3]` — electric field phasor components per node.
    /// - `relative_permittivity`: `[B, N, 1]` **or** `[B, N, 9]` — scalar \(\varepsilon_r\) **or** row-major **3×3** per node; TE uses **\(\varepsilon_{yy}\)** only (see module docs). **`eps_r_imag`** remains `[B, N, 1]`.
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
    /// \(E_x,E_z\) pass through from `e_field`.
    ///
    /// **Non-chain + `dec_patch`:** when `dec_patch` is `Some` and the topology validates, runs a
    /// **DEC-assembled** (primal \(d_0\) grad–div per **diagonal** \(\varepsilon_{xx},\varepsilon_{yy},\varepsilon_{zz}\) channel +
    /// metric curl leg from [`photonics_dec_patch_uses_metric_dual_edge_hodge`]: for **`[B,N,9]`** tensors, the
    /// Whitney midpoint → tangential map uses the **symmetrized edge average** of the full **3×3** \(\varepsilon\); scalar
    /// **`[B,N,1]`** leaves the curl leg \(\varepsilon\)-free as before; nodal \(k_0^2\,\varepsilon\) mass uses **3×3**
    /// tensor support) host solve on \(3N\) DoFs, **gauge-pinned** at node `0` to incoming `e_field`.
    /// **Lossless** (`max|eps_r_imag| \le 10^{-6}` in code): **CSR matvec CG** first when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`]
    /// and `UMST_PHOTONICS_DEC_PATCH_CSR_INNER` is not `off` (default **`auto`**); on CSR failure or when CSR is skipped (`off`), Gauss–Jordan when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`]
    /// (effective cap **0** with `UMST_PHOTONICS_DEC_PATCH_FORCE_KRYLOV=1` for tests); further fallbacks: CSR again when **`auto`** and dense failed, then **matrix-free CG** up to [`PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV`].
    /// **Lossy** scalar `eps_r_imag`: stacked **real** \(2\cdot 3N\) dense Gauss–Jordan when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`];
    /// the tensor API returns **\(\Re\mathbf{E}\)** only. `impressed_current` drives the RHS via the same `ω μ₀` scaling as the chain TE phasor map on **all three** vector components.
    /// **Not** a production sparse volumetric Maxwell solve.
    ///
    /// ## Routing (`photonics`, \(B=1\), tensor shapes valid)
    /// | Priority | Condition | Result |
    /// | --- | --- | --- |
    /// | 1 | Uniform **x-monotone** path chain from `edges_b1` + `coords_n3` | TE \(E_y\) Thomas; \(E_x,E_z\) pass-through |
    /// | 2 | Else, `dec_patch` **Some** and [`dec_patch_topology_valid_for_solve`] | `faces_b2` vector DEC (metric \(\sqrt{\star_1}\) curl leg per [`photonics_dec_patch_uses_metric_dual_edge_hodge`]): CSR-first lossless inner when \(N\le\) CSR assembly cap and env not `off`, else dense / matrix-free fallbacks |
    /// | 2a | Same topology OK, dense path returns [`None`] (cap / lossy `eps_r_imag` / singular) | Warn + pass-through |
    /// | 2b | `dec_patch` **Some**, topology invalid | Structural warn + pass-through |
    /// | 3 | Otherwise | Warn + pass-through |
    ///
    /// **Other:** warns and returns `e_field` unchanged.
    ///
    /// ## Verification row **#6** (honest partial, not 100%)
    /// This entry point implements what [`Solver-Status.md`](../../../docs/Solver-Status.md) calls the
    /// **partial** photonics lane ([`Solver-Status.md`](../../../docs/Solver-Status.md) row **#6**):
    /// uniform-chain TE + optional **small dense** `PhotonicsDecFacesPatch` branch (see also
    /// `photonics_dec_patch_uses_metric_dual_edge_hodge` (feature **`photonics`**) — diagonal primal-length \(\star_1\) on the patch curl leg;
    /// **`[B,N,9]`** tensors additionally feed a **symmetrized edge-averaged 3×3** map in the Whitney trace — **not** \(\varepsilon^{-1}\) constitutive on the curl leg).
    /// **Completion bin remains ~50%** until production volumetrics / dual Hodge / complex patch \(\varepsilon\) / BCs land — see [`Solver-Status.md`](../../../docs/Solver-Status.md). **Still open:** circumcentric/barycentric dual metrics, sparse inner solves at production \(N\), complex \(\varepsilon\) / PML on the patch path, broader BCs, and \(\varepsilon^{-1}\) on the curl constitutive (not modelled here).
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
        dec_patch: Option<&PhotonicsDecFacesPatch<'_, B>>,
    ) -> Result<Tensor<B, 3>, PhysicsError> {
        #[cfg(not(feature = "photonics"))]
        {
            let _ = (
                relative_permittivity,
                eps_r_imag,
                impressed_current,
                edges_b1,
                coords_n3,
                cg,
                dec_patch,
            );
            Ok(e_field)
        }

        #[cfg(feature = "photonics")]
        {
            let _ = cg;
            let d = e_field.dims();
            if d.len() != 3 || d[2] != 3 {
                return Err(PhysicsError::ShapeMismatch {
                    context: "solve_maxwell_curl_curl",
                    detail: "expected e_field [B,N,3]",
                });
            }
            let n = d[1];
            let pe = relative_permittivity.dims();
            let pi = eps_r_imag.dims();
            let perm_ok = pe.len() == 3
                && pe[0] == d[0]
                && pe[1] == n
                && (pe[2] == RELATIVE_PERMITTIVITY_CHANNELS_SCALAR
                    || pe[2] == RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3);
            let imag_ok = pi.len() == 3 && pi == [d[0], n, RELATIVE_PERMITTIVITY_CHANNELS_SCALAR];
            if !perm_ok || !imag_ok || impressed_current.dims() != d || coords_n3.dims() != [n, 3] {
                return Err(PhysicsError::ShapeMismatch {
                    context: "solve_maxwell_curl_curl",
                    detail: "permittivity [B,N,1|9], imag [B,N,1], coords shape mismatch",
                });
            }
            if d[0] != 1 {
                return Err(PhysicsError::UnsupportedLayout {
                    context: "solve_maxwell_curl_curl: only batch size 1 is supported",
                });
            }

            if let Some(chain) = extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3) {
                let eps_rr = match nodal_eps_r_real_for_te_chain(&relative_permittivity, d[0], n) {
                    Some(v) => v,
                    None => {
                        return Err(PhysicsError::UnsupportedLayout {
                            context: "solve_maxwell_curl_curl: unsupported relative_permittivity layout",
                        });
                    }
                };
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
                return Ok(Tensor::cat(vec![ex, ey_re, ez], 2));
            }

            if let Some(patch) = dec_patch {
                if dec_patch_topology_valid_for_solve::<B>(n, &edges_b1, patch) {
                    return solve_maxwell_dec_patch_direct::<B>(
                        &e_field,
                        &relative_permittivity,
                        &eps_r_imag,
                        &impressed_current,
                        &edges_b1,
                        &coords_n3,
                        self.frequency_hz,
                        patch,
                        self.dec_patch_config,
                    );
                }
                return Err(PhysicsError::UnsupportedLayout {
                    context: "solve_maxwell_curl_curl: dec_patch faces_b2 / column ranges failed structural validation",
                });
            }

            Err(PhysicsError::UnsupportedLayout {
                context: "solve_maxwell_curl_curl: no uniform x-chain and no dec_patch supplied",
            })
        }
    }
}

#[cfg(feature = "photonics")]
fn gauss_jordan_solve_f32(a: &mut [f32], b: &mut [f32], n: usize) -> Result<(), ()> {
    for k in 0..n {
        let mut piv = k;
        let mut best = a[k * n + k].abs();
        for r in (k + 1)..n {
            let v = a[r * n + k].abs();
            if v > best {
                best = v;
                piv = r;
            }
        }
        if best < 1e-20_f32 {
            return Err(());
        }
        if piv != k {
            for c in 0..n {
                a.swap(k * n + c, piv * n + c);
            }
            b.swap(k, piv);
        }
        let p = a[k * n + k];
        for c in 0..n {
            a[k * n + c] /= p;
        }
        b[k] /= p;
        for r in 0..n {
            if r == k {
                continue;
            }
            let f = a[r * n + k];
            if f == 0.0_f32 {
                continue;
            }
            for c in 0..n {
                a[r * n + c] -= f * a[k * n + c];
            }
            b[r] -= f * b[k];
        }
    }
    Ok(())
}

#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
pub fn dec_patch_operator_apply_gauged(
    x: &[f32],
    y: &mut [f32],
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
) {
    dec_patch_maxwell_natural_matvec_flat(
        x,
        y,
        n,
        n_edges,
        src,
        tgt,
        coords,
        k0,
        eps_scalar,
        eps_tensor9,
        faces_edge,
        faces_sign,
        face_ranges,
    );
    y[0] = x[0];
    y[1] = x[1];
    y[2] = x[2];
}

#[cfg(feature = "photonics")]
fn vec_dot_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

#[cfg(feature = "photonics")]
fn vec_l2_f32(a: &[f32]) -> f32 {
    vec_dot_f32(a, a).sqrt()
}

/// **Capped** conjugate-gradient solve for the gauge-pinned patch operator (matrix-free matvec only).
#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
fn solve_maxwell_dec_patch_conjugate_gradient(
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
    b: &[f32],
    dim: usize,
) -> Option<Vec<f32>> {
    const REL_TOL: f32 = 1e-7_f32;
    let max_iter = PHOTONICS_DEC_PATCH_KRYLOV_MAX_ITERS.min(dim.saturating_mul(8).max(64));

    let mut x = vec![0.0_f32; dim];
    let mut r = vec![0.0_f32; dim];
    let mut p = vec![0.0_f32; dim];
    let mut ap = vec![0.0_f32; dim];
    let mut ybuf = vec![0.0_f32; dim];

    dec_patch_operator_apply_gauged(
        &x,
        &mut ybuf,
        n,
        n_edges,
        src,
        tgt,
        coords,
        k0,
        eps_scalar,
        eps_tensor9,
        faces_edge,
        faces_sign,
        face_ranges,
    );
    for i in 0..dim {
        r[i] = b[i] - ybuf[i];
    }
    let bn = vec_l2_f32(b).max(1e-30_f32);
    let mut rn = vec_l2_f32(&r);
    if rn / bn < REL_TOL {
        return Some(x);
    }
    p.copy_from_slice(&r);
    let mut r_dot = vec_dot_f32(&r, &r);

    for _ in 0..max_iter {
        dec_patch_operator_apply_gauged(
            &p,
            &mut ap,
            n,
            n_edges,
            src,
            tgt,
            coords,
            k0,
            eps_scalar,
            eps_tensor9,
            faces_edge,
            faces_sign,
            face_ranges,
        );
        let p_ap = vec_dot_f32(&p, &ap);
        let pn = vec_l2_f32(&p);
        if !p_ap.is_finite() || p_ap <= 1e-28_f32 * pn * pn.max(1.0_f32) {
            tracing::warn!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_conjugate_gradient: breakdown (p·Ap={p_ap:.3e})"
            );
            return None;
        }
        let alpha = r_dot / p_ap;
        for i in 0..dim {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        rn = vec_l2_f32(&r);
        if rn / bn < REL_TOL {
            tracing::debug!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_conjugate_gradient: converged (rel residual {:.3e})",
                rn / bn
            );
            return Some(x);
        }
        let r_dot_new = vec_dot_f32(&r, &r);
        if !r_dot_new.is_finite() || r_dot_new <= 0.0_f32 {
            return None;
        }
        let beta = r_dot_new / r_dot;
        for i in 0..dim {
            p[i] = r[i] + beta * p[i];
        }
        r_dot = r_dot_new;
    }
    tracing::warn!(
        target: "umst_manifold::photonics",
        "solve_maxwell_dec_patch_conjugate_gradient: exceeded max_iter={max_iter} (rel residual {:.3e})",
        rn / bn
    );
    None
}

/// **COO** triplets \((\texttt{row},\texttt{col},\texttt{val})\) for the **gauge-pinned** patch Maxwell
/// operator on \(\mathbb{R}^{3N}\) (same semantics as [`dec_patch_operator_apply_gauged`]): columns are
/// recovered by probing with unit vectors; entries with \(|a_{rc}|\le \texttt{drop\_tol}\) are dropped.
///
/// **Cost:** **O(dim²)** matvecs — capped by [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`] on the shipped
/// lossless Krylov path (see [`dec_patch_try_csr_inner_lossless`]).
#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
pub fn dec_patch_maxwell_gauged_operator_csr_coo(
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
    drop_tol: f32,
) -> Vec<(usize, usize, f32)> {
    let dim = 3 * n;
    let mut xv = vec![0.0_f32; dim];
    let mut yv = vec![0.0_f32; dim];
    let mut coo = Vec::new();
    for col in 0..dim {
        xv.fill(0.0_f32);
        xv[col] = 1.0_f32;
        dec_patch_operator_apply_gauged(
            &xv,
            &mut yv,
            n,
            n_edges,
            src,
            tgt,
            coords,
            k0,
            eps_scalar,
            eps_tensor9,
            faces_edge,
            faces_sign,
            face_ranges,
        );
        for (row, &v) in yv.iter().enumerate().take(dim) {
            if v.abs() > drop_tol {
                coo.push((row, col, v));
            }
        }
    }
    coo
}

/// Multiply a **COO** matrix (see [`dec_patch_maxwell_gauged_operator_csr_coo`]) by a dense vector.
#[cfg(feature = "photonics")]
pub fn dec_patch_csr_coo_matvec_f32(coo: &[(usize, usize, f32)], x: &[f32], y: &mut [f32]) {
    y.fill(0.0_f32);
    for &(row, col, v) in coo {
        y[row] += v * x[col];
    }
}

/// Sort **COO** triplets by `(row, col)` and **merge** duplicate indices by summing values.
#[cfg(feature = "photonics")]
pub fn dec_patch_coo_sort_merge_f32(coo: &[(usize, usize, f32)]) -> Vec<(usize, usize, f32)> {
    let mut v: Vec<(usize, usize, f32)> = coo.to_vec();
    v.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let mut out: Vec<(usize, usize, f32)> = Vec::with_capacity(v.len());
    for (r, c, val) in v {
        if let Some(last) = out.last_mut() {
            if last.0 == r && last.1 == c {
                last.2 += val;
                continue;
            }
        }
        out.push((r, c, val));
    }
    out
}

/// **CSR** `(row_ptr, col_ind, vals)` from sorted-merged COO with `dim` rows (square `dim×dim`).
#[cfg(feature = "photonics")]
pub fn dec_patch_csr_from_sorted_coo_f32(
    dim: usize,
    sorted_merged: &[(usize, usize, f32)],
) -> Option<(Vec<usize>, Vec<usize>, Vec<f32>)> {
    if sorted_merged.is_empty() {
        return None;
    }
    for &(r, c, v) in sorted_merged {
        if r >= dim || c >= dim || !v.is_finite() {
            return None;
        }
    }
    let nnz = sorted_merged.len();
    let mut row_ptr = vec![0usize; dim + 1];
    for &(r, _, _) in sorted_merged {
        row_ptr[r + 1] += 1;
    }
    for i in 0..dim {
        row_ptr[i + 1] += row_ptr[i];
    }
    let mut col_ind = vec![0usize; nnz];
    let mut vals = vec![0.0_f32; nnz];
    let mut next = row_ptr[..dim].to_vec();
    for &(r, c, v) in sorted_merged {
        let k = next[r];
        if k >= nnz {
            return None;
        }
        col_ind[k] = c;
        vals[k] = v;
        next[r] += 1;
    }
    Some((row_ptr, col_ind, vals))
}

/// **CSR** sparse matrix–vector multiply: `y = A x` with `A` in compressed sparse row form.
#[cfg(feature = "photonics")]
pub fn dec_patch_csr_matvec_f32(
    row_ptr: &[usize],
    col_ind: &[usize],
    vals: &[f32],
    x: &[f32],
    y: &mut [f32],
) {
    let dim = row_ptr.len().saturating_sub(1);
    y.fill(0.0_f32);
    debug_assert_eq!(row_ptr.len(), dim + 1);
    for r in 0..dim {
        let mut acc = 0.0_f32;
        for k in row_ptr[r]..row_ptr[r + 1] {
            acc += vals[k] * x[col_ind[k]];
        }
        y[r] = acc;
    }
}

/// **Capped** conjugate-gradient solve using an explicit **CSR** matvec for the same gauge-pinned
/// patch operator as [`dec_patch_operator_apply_gauged`] / [`solve_maxwell_dec_patch_conjugate_gradient`].
#[cfg(feature = "photonics")]
fn solve_maxwell_dec_patch_conjugate_gradient_csr(
    row_ptr: &[usize],
    col_ind: &[usize],
    vals: &[f32],
    b: &[f32],
    dim: usize,
) -> Option<Vec<f32>> {
    const REL_TOL: f32 = 1e-7_f32;
    let max_iter = PHOTONICS_DEC_PATCH_KRYLOV_MAX_ITERS.min(dim.saturating_mul(8).max(64));

    let mut x = vec![0.0_f32; dim];
    let mut r = vec![0.0_f32; dim];
    let mut p = vec![0.0_f32; dim];
    let mut ap = vec![0.0_f32; dim];

    dec_patch_csr_matvec_f32(row_ptr, col_ind, vals, &x, &mut ap);
    for i in 0..dim {
        r[i] = b[i] - ap[i];
    }
    let bn = vec_l2_f32(b).max(1e-30_f32);
    let mut rn = vec_l2_f32(&r);
    if rn / bn < REL_TOL {
        return Some(x);
    }
    p.copy_from_slice(&r);
    let mut r_dot = vec_dot_f32(&r, &r);

    for _ in 0..max_iter {
        dec_patch_csr_matvec_f32(row_ptr, col_ind, vals, &p, &mut ap);
        let p_ap = vec_dot_f32(&p, &ap);
        let pn = vec_l2_f32(&p);
        if !p_ap.is_finite() || p_ap <= 1e-28_f32 * pn * pn.max(1.0_f32) {
            tracing::warn!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_conjugate_gradient_csr: breakdown (p·Ap={p_ap:.3e})"
            );
            return None;
        }
        let alpha = r_dot / p_ap;
        for i in 0..dim {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        rn = vec_l2_f32(&r);
        if rn / bn < REL_TOL {
            tracing::debug!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_conjugate_gradient_csr: converged (rel residual {:.3e})",
                rn / bn
            );
            return Some(x);
        }
        let r_dot_new = vec_dot_f32(&r, &r);
        if !r_dot_new.is_finite() || r_dot_new <= 0.0_f32 {
            return None;
        }
        let beta = r_dot_new / r_dot;
        for i in 0..dim {
            p[i] = r[i] + beta * p[i];
        }
        r_dot = r_dot_new;
    }
    tracing::warn!(
        target: "umst_manifold::photonics",
        "solve_maxwell_dec_patch_conjugate_gradient_csr: exceeded max_iter={max_iter} (rel residual {:.3e})",
        rn / bn
    );
    None
}

/// Stacked real operator for \(\mathbf{E}=\mathbf{E}'+i\mathbf{E}''\) with nodal scalar \(\varepsilon''\)
/// in \(k_0^2(\varepsilon_r+i\varepsilon'')\mathbf{E}\); curl / grad–div use **`relative_permittivity`** real part only.
///
/// Unknown layout: `[Er_flat; Ei_flat]` each length `3N`. Gauge: pins `Er[0..3]` and `Ei[0..3]` to the
/// incoming stacked values.
#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
pub fn dec_patch_operator_apply_gauged_stacked_lossy(
    x_stack: &[f32],
    y_stack: &mut [f32],
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    eps_imag: &[f32],
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
) {
    let dim = 3 * n;
    debug_assert_eq!(x_stack.len(), 2 * dim);
    debug_assert_eq!(y_stack.len(), 2 * dim);
    let (xr, xi) = x_stack.split_at(dim);
    let (yr, yi) = y_stack.split_at_mut(dim);

    dec_patch_maxwell_natural_matvec_flat(
        xr,
        yr,
        n,
        n_edges,
        src,
        tgt,
        coords,
        k0,
        eps_scalar,
        eps_tensor9,
        faces_edge,
        faces_sign,
        face_ranges,
    );
    dec_patch_maxwell_natural_matvec_flat(
        xi,
        yi,
        n,
        n_edges,
        src,
        tgt,
        coords,
        k0,
        eps_scalar,
        eps_tensor9,
        faces_edge,
        faces_sign,
        face_ranges,
    );

    let k02 = k0 * k0;
    for (i, sim) in eps_imag.iter().copied().enumerate().take(n) {
        for c in 0..3usize {
            let ix = 3 * i + c;
            yr[ix] -= k02 * sim * xi[ix];
            yi[ix] += k02 * sim * xr[ix];
        }
    }

    yr[0] = xr[0];
    yr[1] = xr[1];
    yr[2] = xr[2];
    yi[0] = xi[0];
    yi[1] = xi[1];
    yi[2] = xi[2];
}

/// **Test / harness hook:** dense stacked-real lossy patch solve returning \((\Re\mathbf{E},\Im\mathbf{E})\)
/// per-node flat `[3N]` vectors (same operator as [`solve_maxwell_dec_patch_direct`] for `max|eps_r_imag|>1e-6`).
#[cfg(feature = "photonics")]
#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
pub fn photonics_dec_patch_dense_stacked_lossy_solution_vectors(
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    eps_imag: &[f32],
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
    b_re: &[f32],
) -> Option<(Vec<f32>, Vec<f32>)> {
    let dim = 3 * n;
    if b_re.len() != dim || eps_imag.len() != n {
        return None;
    }
    let dim2 = 2 * dim;
    let mut b_stack = vec![0.0_f32; dim2];
    b_stack[..dim].copy_from_slice(b_re);
    b_stack[dim..dim + 3].fill(0.0_f32);

    let mut a = vec![0.0_f32; dim2 * dim2];
    let mut xv = vec![0.0_f32; dim2];
    let mut yv = vec![0.0_f32; dim2];
    for col in 0..dim2 {
        xv.fill(0.0_f32);
        xv[col] = 1.0_f32;
        dec_patch_operator_apply_gauged_stacked_lossy(
            &xv,
            &mut yv,
            n,
            n_edges,
            src,
            tgt,
            coords,
            k0,
            eps_scalar,
            eps_tensor9,
            eps_imag,
            faces_edge,
            faces_sign,
            face_ranges,
        );
        for r in 0..dim2 {
            a[r * dim2 + col] = yv[r];
        }
    }
    gauss_jordan_solve_f32(&mut a, &mut b_stack, dim2).ok()?;
    let er = b_stack[..dim].to_vec();
    let ei = b_stack[dim..].to_vec();
    Some((er, ei))
}

#[cfg(feature = "photonics")]
fn dec_patch_effective_dense_node_cap(force_krylov: bool) -> usize {
    if force_krylov {
        0
    } else {
        PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT
    }
}

/// Optional **CSR matvec CG** inner solve on the **lossless** gauge-pinned patch operator.
///
/// **Policy:** [`DecPatchCsrInnerMode::Auto`]: caller may pass **`prefer_csr_inner`** so CSR runs
/// before dense Gauss–Jordan when \(N\le\) [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`]; otherwise (second pass) CSR runs when
/// \(N\) exceeds the effective dense cap ([`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`], or **0** with [`PhotonicsDecPatchConfig::force_krylov`])
/// or when a lossless dense attempt failed. [`DecPatchCsrInnerMode::On`]: try CSR whenever \(N\le\) the CSR assembly cap.
/// [`DecPatchCsrInnerMode::Off`]: skip CSR. **Cap:** [`PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY`] bounds **O(dim²)** COO assembly.
#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
fn dec_patch_try_csr_inner_lossless(
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
    b: &[f32],
    dim: usize,
    dense_node_cap_eff: usize,
    lossless_dense_tried_failed: bool,
    prefer_csr_inner: bool,
    csr_inner: DecPatchCsrInnerMode,
) -> Option<Vec<f32>> {
    if n > PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY {
        tracing::debug!(
            target: "umst_manifold::photonics",
            "dec_patch_try_csr_inner_lossless: N={n} exceeds PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY={}",
            PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY
        );
        return None;
    }

    let want_csr = match csr_inner {
        DecPatchCsrInnerMode::Off => false,
        DecPatchCsrInnerMode::On => true,
        DecPatchCsrInnerMode::Auto => {
            prefer_csr_inner || n > dense_node_cap_eff || lossless_dense_tried_failed
        }
    };
    if !want_csr {
        return None;
    }

    const DROP_TOL: f32 = 1e-20_f32;
    let coo = dec_patch_maxwell_gauged_operator_csr_coo(
        n,
        n_edges,
        src,
        tgt,
        coords,
        k0,
        eps_scalar,
        eps_tensor9,
        faces_edge,
        faces_sign,
        face_ranges,
        DROP_TOL,
    );
    let merged = dec_patch_coo_sort_merge_f32(&coo);
    let (rp, ci, va) = dec_patch_csr_from_sorted_coo_f32(dim, &merged)?;
    tracing::debug!(
        target: "umst_manifold::photonics",
        "dec_patch_try_csr_inner_lossless: CSR matvec CG (N={n}, nnz={})",
        va.len()
    );
    solve_maxwell_dec_patch_conjugate_gradient_csr(&rp, &ci, &va, b, dim)
}

/// Primal **SI edge lengths** \(\ell_e=\lVert \mathbf{x}_j-\mathbf{x}_i\rVert\) for each oriented edge in `edges_b1`
/// row layout (`src` = row 0, `tgt` = row 1).
///
/// This is the **primal SI edge length** \(\ell_e\) feeding [`dec_patch_diagonal_star1_primal_edge_length_lumped_si`]
/// and the symmetric \(\sqrt{\star_1}\) sandwich in [`dec_patch_maxwell_natural_matvec_flat`].
#[cfg(feature = "photonics")]
#[must_use]
pub fn dec_patch_primal_edge_lengths_si(
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
) -> Vec<f32> {
    let mut edge_len = vec![0.0_f32; n_edges];
    for e in 0..n_edges {
        let i = src[e] as usize;
        let j = tgt[e] as usize;
        if i >= n || j >= n {
            continue;
        }
        let dx = coords[j * 3] - coords[i * 3];
        let dy = coords[j * 3 + 1] - coords[i * 3 + 1];
        let dz = coords[j * 3 + 2] - coords[i * 3 + 2];
        edge_len[e] = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-12_f32);
    }
    edge_len
}

/// Diagonal **\(\star_1\)** entries (SI) for the DEC patch **curl** leg: per edge \(e\),
/// \(\bigl(\star_1\bigr)_{ee}=\ell_e\) clamped positive, using the same primal lengths as
/// [`dec_patch_primal_edge_lengths_si`].
///
/// **Semantics:** a **lumped primal-edge surrogate** for dual-edge mass (no circumcentric dual lengths).
/// [`dec_patch_maxwell_natural_matvec_flat`] applies \(\sqrt{\bigl(\star_1\bigr)_{ee}}\) before \(d_1\)
/// and after \(d_1^\top\) so the edge-space curl block matches \(\sqrt{\star_1}\, d_1^\top d_1\, \sqrt{\star_1}\)
/// on the tangential edge trace.
#[cfg(feature = "photonics")]
#[must_use]
pub fn dec_patch_diagonal_star1_primal_edge_length_lumped_si(edge_len_si: &[f32]) -> Vec<f32> {
    edge_len_si
        .iter()
        .copied()
        .map(|l| l.max(1e-12_f32))
        .collect()
}

#[cfg(feature = "photonics")]
#[inline]
fn dec_patch_eps_tensor9_at_node(t9: &[f32], node: usize) -> [f32; 9] {
    let b = node * 9;
    let mut a = [0.0_f32; 9];
    a.copy_from_slice(&t9[b..b + 9]);
    a
}

/// Symmetrized **edge average** of nodal row-major **3×3** tensors at endpoints `i`, `j`.
#[cfg(feature = "photonics")]
#[must_use]
fn dec_patch_sym_avg_eps_edge_tensor9(t9: &[f32], i: usize, j: usize) -> [f32; 9] {
    let ai = dec_patch_eps_tensor9_at_node(t9, i);
    let aj = dec_patch_eps_tensor9_at_node(t9, j);
    let mut o = [0.0_f32; 9];
    for k in 0..9 {
        o[k] = 0.5_f32 * (ai[k] + aj[k]);
    }
    o[1] = 0.5_f32 * (o[1] + o[3]);
    o[3] = o[1];
    o[2] = 0.5_f32 * (o[2] + o[6]);
    o[6] = o[2];
    o[5] = 0.5_f32 * (o[5] + o[7]);
    o[7] = o[5];
    o
}

#[cfg(feature = "photonics")]
#[inline]
fn dec_patch_matvec_sym3(a9: &[f32; 9], v: [f32; 3]) -> [f32; 3] {
    [
        a9[0] * v[0] + a9[1] * v[1] + a9[2] * v[2],
        a9[3] * v[0] + a9[4] * v[1] + a9[5] * v[2],
        a9[6] * v[0] + a9[7] * v[1] + a9[8] * v[2],
    ]
}

/// Host reference matvec for the **DEC patch** Maxwell operator (grad–div per diagonal \(\varepsilon\)
/// channel + \(d_1^\top d_1\) on tangential edge projections with diagonal \(\star_1\) from primal edge
/// lengths (see [`photonics_dec_patch_uses_metric_dual_edge_hodge`]) + \(k_0^2\) nodal \(\varepsilon\) **3×3**).
///
/// **Tensor \([N,9]\) curl leg:** before \(t\cdot\) into the edge circulation, the midpoint field is multiplied by the
/// **symmetrized edge average** of the nodal **3×3** \(\varepsilon\) (row-major); the return scatter uses the same average
/// applied to \(t\) (self-adjoint on each edge under symmetric \(\varepsilon\)). Scalar **`eps_scalar`** path leaves the curl
/// leg independent of \(\varepsilon\) as before.
///
/// **Gauge:** caller applies pinning when assembling the full linear system.
#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
pub fn dec_patch_maxwell_natural_matvec_flat(
    x: &[f32],
    y: &mut [f32],
    n: usize,
    n_edges: usize,
    src: &[i64],
    tgt: &[i64],
    coords: &[f32],
    k0: f32,
    eps_scalar: Option<&[f32]>,
    eps_tensor9: Option<&[f32]>,
    faces_edge: &[i64],
    faces_sign: &[f32],
    face_ranges: &[(usize, usize)],
) {
    debug_assert_eq!(x.len(), 3 * n);
    debug_assert_eq!(y.len(), 3 * n);
    y.fill(0.0_f32);

    let edge_len = dec_patch_primal_edge_lengths_si(n, n_edges, src, tgt, coords);
    let mut tx = vec![0.0_f32; n_edges];
    let mut ty = vec![0.0_f32; n_edges];
    let mut tz = vec![0.0_f32; n_edges];
    for e in 0..n_edges {
        let i = src[e] as usize;
        let j = tgt[e] as usize;
        if i >= n || j >= n {
            continue;
        }
        let dx = coords[j * 3] - coords[i * 3];
        let dy = coords[j * 3 + 1] - coords[i * 3 + 1];
        let dz = coords[j * 3 + 2] - coords[i * 3 + 2];
        let l = edge_len[e].max(1e-12_f32);
        tx[e] = dx / l;
        ty[e] = dy / l;
        tz[e] = dz / l;
    }

    let eps_diag = |node: usize, ch: usize| -> f32 {
        if let Some(s) = eps_scalar {
            s[node]
        } else if let Some(t9) = eps_tensor9 {
            let ix = [0usize, 4, 8][ch];
            t9[node * 9 + ix]
        } else {
            1.0
        }
    };

    for c in 0..3usize {
        for e in 0..n_edges {
            let i = src[e] as usize;
            let j = tgt[e] as usize;
            if i >= n || j >= n {
                continue;
            }
            let inv_l2 = 1.0 / (edge_len[e] * edge_len[e]);
            let ea = eps_diag(i, c);
            let eb = eps_diag(j, c);
            let eta = inv_l2 * 2.0_f32 / (ea + eb).max(1e-12_f32);
            let xi = x[3 * i + c];
            let xj = x[3 * j + c];
            let f = eta * (xi - xj);
            y[3 * i + c] += f;
            y[3 * j + c] -= f;
        }
    }

    let mut u_e = vec![0.0_f32; n_edges];
    for e in 0..n_edges {
        let i = src[e] as usize;
        let j = tgt[e] as usize;
        if i >= n || j >= n {
            continue;
        }
        let mx = 0.5_f32 * (x[3 * i] + x[3 * j]);
        let my = 0.5_f32 * (x[3 * i + 1] + x[3 * j + 1]);
        let mz = 0.5_f32 * (x[3 * i + 2] + x[3 * j + 2]);
        let m = [mx, my, mz];
        u_e[e] = if let Some(t9) = eps_tensor9 {
            let a = dec_patch_sym_avg_eps_edge_tensor9(t9, i, j);
            let d = dec_patch_matvec_sym3(&a, m);
            d[0] * tx[e] + d[1] * ty[e] + d[2] * tz[e]
        } else {
            mx * tx[e] + my * ty[e] + mz * tz[e]
        };
    }

    if photonics_dec_patch_uses_metric_dual_edge_hodge() {
        for e in 0..n_edges {
            u_e[e] *= edge_len[e].max(1e-12_f32).sqrt();
        }
    }

    let n_face = face_ranges.len();
    let mut d1u = vec![0.0_f32; n_face];
    for (f, &(start, end)) in face_ranges.iter().enumerate() {
        let mut s = 0.0_f32;
        for k in start..end {
            let eid = faces_edge[k] as usize;
            if eid < n_edges {
                s += faces_sign[k] * u_e[eid];
            }
        }
        d1u[f] = s;
    }

    let mut w_e = vec![0.0_f32; n_edges];
    for (f, &(start, end)) in face_ranges.iter().enumerate() {
        let phi = d1u[f];
        for k in start..end {
            let eid = faces_edge[k] as usize;
            if eid < n_edges {
                w_e[eid] += faces_sign[k] * phi;
            }
        }
    }

    if photonics_dec_patch_uses_metric_dual_edge_hodge() {
        for e in 0..n_edges {
            w_e[e] *= edge_len[e].max(1e-12_f32).sqrt();
        }
    }

    for e in 0..n_edges {
        let i = src[e] as usize;
        let j = tgt[e] as usize;
        if i >= n || j >= n {
            continue;
        }
        let v = 0.5_f32 * w_e[e];
        let (wx, wy, wz) = if let Some(t9) = eps_tensor9 {
            let a = dec_patch_sym_avg_eps_edge_tensor9(t9, i, j);
            let d = dec_patch_matvec_sym3(&a, [tx[e], ty[e], tz[e]]);
            (v * d[0], v * d[1], v * d[2])
        } else {
            (v * tx[e], v * ty[e], v * tz[e])
        };
        y[3 * i] += wx;
        y[3 * i + 1] += wy;
        y[3 * i + 2] += wz;
        y[3 * j] += wx;
        y[3 * j + 1] += wy;
        y[3 * j + 2] += wz;
    }

    let k02 = k0 * k0;
    if let Some(s) = eps_scalar {
        for i in 0..n {
            let m = k02 * s[i];
            y[3 * i] += m * x[3 * i];
            y[3 * i + 1] += m * x[3 * i + 1];
            y[3 * i + 2] += m * x[3 * i + 2];
        }
    } else if let Some(t9) = eps_tensor9 {
        for i in 0..n {
            let b = i * 9;
            let v0 = t9[b] * x[3 * i] + t9[b + 1] * x[3 * i + 1] + t9[b + 2] * x[3 * i + 2];
            let v1 = t9[b + 3] * x[3 * i] + t9[b + 4] * x[3 * i + 1] + t9[b + 5] * x[3 * i + 2];
            let v2 = t9[b + 6] * x[3 * i] + t9[b + 7] * x[3 * i + 1] + t9[b + 8] * x[3 * i + 2];
            y[3 * i] += k02 * v0;
            y[3 * i + 1] += k02 * v1;
            y[3 * i + 2] += k02 * v2;
        }
    }
}

#[cfg(feature = "photonics")]
#[allow(clippy::too_many_arguments)]
fn solve_maxwell_dec_patch_direct<B: Backend<FloatElem = f32>>(
    e_field: &Tensor<B, 3>,
    relative_permittivity: &Tensor<B, 3>,
    eps_r_imag: &Tensor<B, 3>,
    impressed_current: &Tensor<B, 3>,
    edges_b1: &Tensor<B, 2, Int>,
    coords_n3: &Tensor<B, 2>,
    frequency_hz: f32,
    patch: &PhotonicsDecFacesPatch<'_, B>,
    dec_patch_config: PhotonicsDecPatchConfig,
) -> Result<Tensor<B, 3>, PhysicsError> {
    let n = e_field.dims()[1];
    if n > PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV {
        return Err(PhysicsError::UnsupportedLayout {
            context: "solve_maxwell_dec_patch_direct: N exceeds PHOTONICS_DEC_PATCH_MAX_NODES_KRYLOV",
        });
    }

    let imag_max = eps_r_imag.clone().abs().max().into_scalar();
    let lossy = imag_max > 1e-6_f32;

    let fd = patch.faces_b2.dims();
    if fd.len() != 2 || fd[0] != 2 {
        return Err(PhysicsError::InvariantViolation {
            context: "solve_maxwell_dec_patch_direct: faces_b2 must be [2, K]",
        });
    }
    let kcols = fd[1];
    for &(s, e) in patch.face_column_ranges {
        if s > e || e > kcols {
            return Err(PhysicsError::InvariantViolation {
                context: "solve_maxwell_dec_patch_direct: invalid face column range",
            });
        }
    }

    let n_edges = edges_b1.dims()[1];
    let edges_f = edges_b1.clone().float().into_data().value;
    if edges_f.len() != n_edges * 2 {
        return Err(PhysicsError::ShapeMismatch {
            context: "solve_maxwell_dec_patch_direct",
            detail: "edges_b1 data length mismatch",
        });
    }
    let src: Vec<i64> = edges_f[..n_edges].iter().map(|&x| x as i64).collect();
    let tgt: Vec<i64> = edges_f[n_edges..].iter().map(|&x| x as i64).collect();

    let faces_f = patch.faces_b2.clone().float().into_data().value;
    if faces_f.len() != kcols * 2 {
        return Err(PhysicsError::ShapeMismatch {
            context: "solve_maxwell_dec_patch_direct",
            detail: "faces_b2 data length mismatch",
        });
    }
    let faces_edge: Vec<i64> = faces_f[..kcols].iter().map(|&x| x as i64).collect();
    let faces_sign: Vec<f32> = faces_f[kcols..].to_vec();

    for &eid in &faces_edge {
        if eid < 0 || (eid as usize) >= n_edges {
            return Err(PhysicsError::InvariantViolation {
                context: "solve_maxwell_dec_patch_direct: faces_b2 edge index out of range",
            });
        }
    }

    let coords = coords_n3.clone().into_data().value;
    let pe = relative_permittivity.dims();
    let eps_scalar = if pe[2] == RELATIVE_PERMITTIVITY_CHANNELS_SCALAR {
        Some(relative_permittivity.clone().into_data().value)
    } else {
        None
    };
    let eps_tensor9 = if pe[2] == RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3 {
        Some(relative_permittivity.clone().into_data().value)
    } else {
        None
    };

    let eps_imag = eps_r_imag.clone().into_data().value;
    if eps_imag.len() != n {
        return Err(PhysicsError::ShapeMismatch {
            context: "solve_maxwell_dec_patch_direct",
            detail: "eps_r_imag nodal length mismatch",
        });
    }

    if lossy && n > PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT {
        return Err(PhysicsError::UnsupportedLayout {
            context: "solve_maxwell_dec_patch_direct: lossy eps_r_imag with N above dense cap",
        });
    }

    let omega = 2.0 * core::f32::consts::PI * frequency_hz;
    let k0 = omega / 2.998e8_f32;
    let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
    let scale_j = omega * mu0;
    let j_flat = impressed_current.clone().into_data().value;

    let dim = 3 * n;
    let mut b = vec![0.0_f32; dim];
    for i in 0..n {
        for c in 0..3usize {
            b[3 * i + c] = scale_j * j_flat[3 * i + c];
        }
    }
    let e0 = e_field.clone().into_data().value;
    b[..3].copy_from_slice(&e0[..3]);

    let dense_node_cap_eff = dec_patch_effective_dense_node_cap(dec_patch_config.force_krylov);
    let csr_inner = dec_patch_config.csr_inner;
    let mut lossless_dense_tried_failed = false;
    let mut sol: Option<Vec<f32>> = None;

    if lossy {
        let dim2 = 2 * dim;
        let mut b_stack = vec![0.0_f32; dim2];
        b_stack[..dim].copy_from_slice(&b);
        b_stack[dim..dim + 3].fill(0.0_f32);

        let mut a = vec![0.0_f32; dim2 * dim2];
        let mut xv = vec![0.0_f32; dim2];
        let mut yv = vec![0.0_f32; dim2];
        for col in 0..dim2 {
            xv.fill(0.0_f32);
            xv[col] = 1.0_f32;
            dec_patch_operator_apply_gauged_stacked_lossy(
                &xv,
                &mut yv,
                n,
                n_edges,
                &src,
                &tgt,
                &coords,
                k0,
                eps_scalar.as_deref(),
                eps_tensor9.as_deref(),
                &eps_imag,
                &faces_edge,
                &faces_sign,
                patch.face_column_ranges,
            );
            for r in 0..dim2 {
                a[r * dim2 + col] = yv[r];
            }
        }
        if gauss_jordan_solve_f32(&mut a, &mut b_stack, dim2).is_ok() {
            tracing::debug!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_direct: stacked dense Gauss–Jordan OK (N={n}, lossy ε'')"
            );
            sol = Some(b_stack[..dim].to_vec());
        } else {
            return Err(PhysicsError::IndefiniteSystem {
                context: "solve_maxwell_dec_patch_direct: stacked dense solve singular",
            });
        }
    } else {
        let under_csr_cap = n <= PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY;
        if csr_inner != DecPatchCsrInnerMode::Off && under_csr_cap {
            sol = dec_patch_try_csr_inner_lossless(
                n,
                n_edges,
                &src,
                &tgt,
                &coords,
                k0,
                eps_scalar.as_deref(),
                eps_tensor9.as_deref(),
                &faces_edge,
                &faces_sign,
                patch.face_column_ranges,
                &b,
                dim,
                dense_node_cap_eff,
                false,
                true,
                csr_inner,
            );
        }
        if sol.is_none() && n <= dense_node_cap_eff {
            let mut a = vec![0.0_f32; dim * dim];
            let mut xv = vec![0.0_f32; dim];
            let mut yv = vec![0.0_f32; dim];
            for col in 0..dim {
                xv.fill(0.0_f32);
                xv[col] = 1.0_f32;
                dec_patch_maxwell_natural_matvec_flat(
                    &xv,
                    &mut yv,
                    n,
                    n_edges,
                    &src,
                    &tgt,
                    &coords,
                    k0,
                    eps_scalar.as_deref(),
                    eps_tensor9.as_deref(),
                    &faces_edge,
                    &faces_sign,
                    patch.face_column_ranges,
                );
                for r in 0..dim {
                    if r < 3 {
                        a[r * dim + col] = if r == col { 1.0_f32 } else { 0.0_f32 };
                    } else {
                        a[r * dim + col] = yv[r];
                    }
                }
            }
            let mut bd = b.clone();
            if gauss_jordan_solve_f32(&mut a, &mut bd, dim).is_ok() {
                tracing::debug!(
                    target: "umst_manifold::photonics",
                    "solve_maxwell_dec_patch_direct: dense Gauss–Jordan OK (N={n})"
                );
                sol = Some(bd);
            } else {
                tracing::warn!(
                    target: "umst_manifold::photonics",
                    "solve_maxwell_dec_patch_direct: dense solve failed (singular?); attempting Krylov fallbacks"
                );
                lossless_dense_tried_failed = true;
            }
        } else if sol.is_none() && n > dense_node_cap_eff {
            tracing::debug!(
                target: "umst_manifold::photonics",
                "solve_maxwell_dec_patch_direct: N={n} exceeds effective dense cap {dense_node_cap_eff}; CSR (if enabled) then matrix-free CG",
            );
        }
    }

    let sol = sol
        .or_else(|| {
            if lossy {
                return None;
            }
            dec_patch_try_csr_inner_lossless(
                n,
                n_edges,
                &src,
                &tgt,
                &coords,
                k0,
                eps_scalar.as_deref(),
                eps_tensor9.as_deref(),
                &faces_edge,
                &faces_sign,
                patch.face_column_ranges,
                &b,
                dim,
                dense_node_cap_eff,
                lossless_dense_tried_failed,
                false,
                csr_inner,
            )
        })
        .or_else(|| {
            if lossy {
                return None;
            }
            solve_maxwell_dec_patch_conjugate_gradient(
                n,
                n_edges,
                &src,
                &tgt,
                &coords,
                k0,
                eps_scalar.as_deref(),
                eps_tensor9.as_deref(),
                &faces_edge,
                &faces_sign,
                patch.face_column_ranges,
                &b,
                dim,
            )
        })
        .ok_or(PhysicsError::KrylovDiverged {
            context: "solve_maxwell_dec_patch_direct: patch inner solve did not converge",
        })?;

    let device = e_field.device();
    let shape = Shape::new([1, n, 3]);
    Ok(Tensor::<B, 3>::from_data(Data::new(sol, shape), &device))
}

/// Primal DEC matvec for the **TE \(E_y\)** reduced operator on a **uniform x-chain** (real \(\varepsilon_r\) only).
///
/// Interior nodes use \((1/h^2)\, d_0^\top \,\mathrm{diag}\bigl(2/(\varepsilon_{\mathrm{src}}+\varepsilon_{\mathrm{tgt}})\bigr)\, d_0 + k_0^2 I\) with [`primal_scalar_edge_increment`] as \(d_0\) and [`primal_divergence_from_edge_flux_topo`] as \(d_0^\top\). Endpoints use the same **Dirichlet identity** rows as [`PhotonicsHelmholtzSolver::solve_helmholtz`].
///
/// **Scope:** \(B=1\), one \(E_y\) channel (`ey` shape `[1,N,1]`). `relative_permittivity` is **`[1,N,1]`** or **`[1,N,9]`** (TE uses **\(\varepsilon_{yy}\)** from the 9-channel slice). Not a general 2D/3D \(d_1\) curl. Returns `None` if the graph is not a uniform x-monotone chain.
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
    let pe = relative_permittivity.dims();
    if coords_n3.dims() != [n, 3] {
        return None;
    }
    if pe != [1, n, RELATIVE_PERMITTIVITY_CHANNELS_SCALAR]
        && pe != [1, n, RELATIVE_PERMITTIVITY_CHANNELS_TENSOR3]
    {
        return None;
    }
    let eps_s = scalar_eps_channel_for_dec(relative_permittivity).ok()?;
    let chain = extract_uniform_x_chain::<B>(n, &edges_b1, &coords_n3)?;
    let h = chain.h;
    let inv_h2 = 1.0 / (h * h);
    let omega = 2.0 * core::f32::consts::PI * frequency_hz;
    let k0 = omega / 2.998e8_f32;

    let topo = EdgeTopology::new(edges_b1);
    let d0 = primal_scalar_edge_increment(ey.clone(), &topo);
    let (src_eps, tgt_eps) = topo.gather_endpoints(eps_s);
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

/// Documented assembly surface for **future** simplicial \(d_1\) / `faces_b2` curl–curl (Track 15 §R3.2).
///
/// The shipped [`PhotonicsSolver::solve_maxwell_curl_curl`] path uses only
/// [`crate::physics::dec_primal::primal_scalar_edge_increment`] and
/// [`crate::physics::dec_primal::primal_divergence_from_edge_flux_topo`]. Patch operators
/// live in [`crate::physics::dec_primal`]; this module re-exports them so photonics integration tests
/// and downstream crates can import one namespace while the chain-only solve remains unchanged.
#[cfg(feature = "photonics")]
pub mod dec_maxwell_assembly {
    pub use crate::physics::dec_primal::{
        canonical_tetrahedron_boundary_dec_coo, primal_d1_edge_flux_to_faces,
        primal_d1_transpose_face_flux_to_edges, CanonicalTetrahedronBoundaryDecCoo,
    };
}

#[cfg(all(test, feature = "photonics"))]
mod photonics_matrix_six_honesty_tests {
    use super::{
        photonics_dec_patch_uses_metric_dual_edge_hodge,
        PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY, PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT,
    };

    #[test]
    fn dec_patch_dense_node_cap_is_stable_contract() {
        assert_eq!(PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT, 64);
    }

    #[test]
    fn dec_patch_csr_assembly_node_cap_is_stable_contract() {
        assert_eq!(PHOTONICS_DEC_PATCH_MAX_NODES_CSR_ASSEMBLY, 128);
    }

    #[test]
    fn dec_patch_dual_edge_hodge_diagonal_primal_length_wired() {
        assert!(
            photonics_dec_patch_uses_metric_dual_edge_hodge(),
            "patch curl leg uses diagonal star_1 from primal edge lengths (lumped; row #6 still partial)"
        );
    }
}

/// **Sparse inner solve harness:** CSR matvec matches COO / operator; CSR CG matches matrix-free CG
/// on the quad-split **N=4** patch (same topology as `photonics_fresnel` integration tests).
#[cfg(all(test, feature = "photonics"))]
mod photonics_sparse_csr_cg_parity_tests {
    use super::{
        dec_patch_coo_sort_merge_f32, dec_patch_csr_coo_matvec_f32,
        dec_patch_csr_from_sorted_coo_f32, dec_patch_csr_matvec_f32,
        dec_patch_maxwell_gauged_operator_csr_coo, dec_patch_operator_apply_gauged,
        solve_maxwell_dec_patch_conjugate_gradient, solve_maxwell_dec_patch_conjugate_gradient_csr,
    };

    #[allow(clippy::type_complexity)]
    fn quad_split_host_layout() -> (
        usize,
        usize,
        Vec<i64>,
        Vec<i64>,
        Vec<f32>,
        Vec<i64>,
        Vec<f32>,
        [(usize, usize); 2],
        f32,
        Vec<f32>,
    ) {
        let n = 4usize;
        let n_e = 5usize;
        let src = vec![0_i64, 1, 2, 3, 0];
        let tgt = vec![1_i64, 2, 3, 0, 2];
        let coords: Vec<f32> = vec![
            0.0, 0.0, 0.0, //
            1.0, 0.0, 0.0, //
            1.0, 1.0, 0.0, //
            0.0, 1.0, 0.0,
        ];
        let faces_edge = vec![0_i64, 1, 4, 4, 2, 3];
        let faces_sign = vec![1.0_f32, 1.0, -1.0, 1.0, 1.0, 1.0];
        let ranges: [(usize, usize); 2] = [(0, 3), (3, 6)];
        let f_hz = 2.4e9_f32;
        let omega = core::f32::consts::TAU * f_hz;
        let k0 = omega / 2.998e8_f32;
        let mu0 = 4.0e-7_f32 * core::f32::consts::PI;
        let scale_j = omega * mu0;
        let mut j_flat = vec![0.0_f32; n * 3];
        j_flat[5] = 0.02;
        j_flat[11] = -0.015;
        let dim = 3 * n;
        let mut b = vec![0.0_f32; dim];
        for i in 0..n {
            for c in 0..3usize {
                b[3 * i + c] = scale_j * j_flat[3 * i + c];
            }
        }
        (
            n, n_e, src, tgt, coords, faces_edge, faces_sign, ranges, k0, b,
        )
    }

    #[test]
    fn dec_patch_csr_matvec_matches_coo_and_operator_quad_split() {
        let (n, n_e, src, tgt, coords, faces_edge, faces_sign, ranges, k0, _) =
            quad_split_host_layout();
        let ones_eps = vec![1.0_f32; n];
        let dim = 3 * n;
        let coo = dec_patch_maxwell_gauged_operator_csr_coo(
            n,
            n_e,
            &src,
            &tgt,
            &coords,
            k0,
            Some(&ones_eps),
            None,
            &faces_edge,
            &faces_sign,
            &ranges,
            1e-20_f32,
        );
        let merged = dec_patch_coo_sort_merge_f32(&coo);
        let (rp, ci, va) = dec_patch_csr_from_sorted_coo_f32(dim, &merged).expect("csr");

        let xv: Vec<f32> = (0..dim)
            .map(|i| ((i * 17 + 3) as f32 * 0.013).sin())
            .collect();
        let mut y_coo = vec![0.0_f32; dim];
        let mut y_csr = vec![0.0_f32; dim];
        let mut y_op = vec![0.0_f32; dim];
        dec_patch_csr_coo_matvec_f32(&coo, &xv, &mut y_coo);
        dec_patch_csr_matvec_f32(&rp, &ci, &va, &xv, &mut y_csr);
        dec_patch_operator_apply_gauged(
            &xv,
            &mut y_op,
            n,
            n_e,
            &src,
            &tgt,
            &coords,
            k0,
            Some(&ones_eps),
            None,
            &faces_edge,
            &faces_sign,
            &ranges,
        );
        for i in 0..dim {
            assert!(
                (y_csr[i] - y_coo[i]).abs() <= 1e-5_f32,
                "CSR assembly must match COO matvec i={i} csr={} coo={}",
                y_csr[i],
                y_coo[i]
            );
            let tol = 1e-4_f32 + 1e-3_f32 * y_op[i].abs().max(1.0_f32);
            assert!(
                (y_coo[i] - y_op[i]).abs() <= tol,
                "COO vs operator i={i} coo={} op={}",
                y_coo[i],
                y_op[i]
            );
        }
    }

    #[test]
    fn dec_patch_csr_cg_matches_matrix_free_cg_quad_split() {
        let (n, n_e, src, tgt, coords, faces_edge, faces_sign, ranges, k0, b) =
            quad_split_host_layout();
        let ones_eps = vec![1.0_f32; n];
        let dim = 3 * n;
        let coo = dec_patch_maxwell_gauged_operator_csr_coo(
            n,
            n_e,
            &src,
            &tgt,
            &coords,
            k0,
            Some(&ones_eps),
            None,
            &faces_edge,
            &faces_sign,
            &ranges,
            1e-20_f32,
        );
        let merged = dec_patch_coo_sort_merge_f32(&coo);
        let (rp, ci, va) = dec_patch_csr_from_sorted_coo_f32(dim, &merged).expect("csr");

        let x_mf = solve_maxwell_dec_patch_conjugate_gradient(
            n,
            n_e,
            &src,
            &tgt,
            &coords,
            k0,
            Some(&ones_eps),
            None,
            &faces_edge,
            &faces_sign,
            &ranges,
            &b,
            dim,
        )
        .expect("matrix-free cg");
        let x_csr =
            solve_maxwell_dec_patch_conjugate_gradient_csr(&rp, &ci, &va, &b, dim).expect("csr cg");

        let mut mx = 0.0_f32;
        for i in 0..dim {
            mx = mx.max((x_mf[i] - x_csr[i]).abs());
        }
        assert!(
            mx < 5e-4_f32,
            "CSR CG vs matrix-free CG max abs diff {mx:.3e}"
        );
    }
}

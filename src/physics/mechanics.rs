// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![allow(clippy::single_range_in_vec_init)]

//! Differentiable bar-network solid mechanics on the DEC 1-skeleton (Phase 1).
//!
//! ## Capability gaps (vs. full Phase-1 plan in `composer-plans/umst_bleeding_edge_solvers.md`)
//!
//! The shipped [`VectorMechanicsSolver::solve_equilibrium`] path is an **axial bar network** on the graph:
//! isotropic \(E,\nu\) is reduced to edges, and equilibrium is **projected conjugate gradient** on the
//! homogeneous Dirichlet subspace (`u = P u`, `P` = `boundary_mask`), implemented with pure Burn ops so
//! autodiff flows through stiffness and geometry. It does **not** yet
//! implement full 3D Voigt \(6\times6\) anisotropic shells, face-based curl operators, or per-edge
//! cross-section tensors — those belong to later DEC refinements and require additional UMST feature banks.
//! **Thin-plate Q1 hex** on the extruded-brick path (matrix **#2** / §R2.1) is verified in
//! `tests/verification/mechanics_analytic.rs` (ratio-band regressions + ignored within-5% Kirchhoff gate),
//! not in this bar-network module. **`extruded_plate`** / **`q1_hex_elasticity`** compile when either
//! **`topology-density-evolution`** or **`mechanics-voigt-cauchy`** is enabled (`src/physics/mod.rs`).
//!
//! Enable **`solver-experimental`** for [`VectorMechanicsSolver::solve_equilibrium_with_voigt_cauchy`] (same bar equilibrium; Cauchy stress via graph Voigt strain and isotropic Hooke).
//!
//! Isotropic Young’s modulus is **reduced to edges** via
//! [`crate::physics::dec_operators::DecEdgeOperators::arithmetic_mean_on_edges`]. Axial bar
//! stiffness is \(k = (EA/L)\,(1-d)^2\) per edge (damage matches the spirit of
//! [`crate::physics::laplacian::TopologicalLaplacian`]).

use burn::tensor::{backend::Backend, Int, Tensor};
#[cfg(feature = "solver-experimental")]
use burn::tensor::ElementConversion;

use super::dec_operators::DecEdgeOperators;
use super::framework::PhysicsSolverZst;
use super::time_orchestration::MechanicsInnerLoopConfig;
use super::topology::EdgeTopology;

/// PCG loop telemetry from [`VectorMechanicsSolver::packed_bar_network_equilibrium`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarNetworkPcgReport {
    pub iterations: usize,
    pub rel_residual: f32,
    /// Stiffness scale `k_ref ≈ E_ref·A/Δx` applied before PCG (1.0 when not used).
    pub stiffness_scale: f32,
    /// `E_ref = max nodal Young` in the assembled system.
    pub e_ref: f32,
    /// Characteristic edge length `Δx` (mean hex edge).
    pub dx_char: f32,
}

impl Default for BarNetworkPcgReport {
    fn default() -> Self {
        Self {
            iterations: 0,
            rel_residual: f32::INFINITY,
            stiffness_scale: 1.0,
            e_ref: 1.0,
            dx_char: 1.0,
        }
    }
}

pub struct VectorMechanicsSolver;

impl PhysicsSolverZst for VectorMechanicsSolver {}

const DAMAGE_REG: f32 = 1e-6;

impl VectorMechanicsSolver {
    /// Nodal Voigt **strain** `[B, N, 6]` from edge displacement on the primal graph.
    ///
    /// # Voigt layout (symmetric \(3\times3\))
    /// `[εxx, εyy, εzz, εxy, εyz, εxz]` — off-diagonals are **tensor** shear \(\varepsilon_{ij}\), not
    /// engineering \(\gamma_{ij}=2\varepsilon_{ij}\).
    ///
    /// # Shapes
    /// * `edge_displacement` — `[B, E, 3]`: **`u_tgt − u_src`** so axial extension is positive along
    ///   `edge_unit`.
    /// * `edge_unit` — `[B, E, 3]`: unit tangent **src → tgt**.
    /// * `edge_len` — `[B, E, 1]`: reference length \(L\) per edge.
    /// * `edges_b1` — `[2, E]`: endpoint rows (same layout as [`EdgeTopology`]).
    ///
    /// Each edge carries \(\varepsilon_{\mathrm{ax}} = ((\mathbf u_{\mathrm{tgt}}-\mathbf u_{\mathrm{src}})\cdot\hat{\mathbf t})/L\)
    /// and rank-one strain \(\varepsilon_{\mathrm{ax}}\,\hat{\mathbf t}\otimes\hat{\mathbf t}\).
    /// Values are **summed** onto endpoints and divided by nodal **degree** (# incident edge ends).
    pub fn voigt_strain_from_edge_displacement<B: Backend<FloatElem = f32>>(
        edge_displacement: Tensor<B, 3>,
        edge_unit: Tensor<B, 3>,
        edge_len: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        n_v: usize,
    ) -> Tensor<B, 3> {
        let batch = edge_displacement.dims()[0];
        let n_e = edge_displacement.dims()[1];
        debug_assert_eq!(edge_unit.dims(), [batch, n_e, 3]);
        debug_assert_eq!(edge_len.dims(), [batch, n_e, 1]);
        let device = edge_displacement.device();

        let elong = edge_displacement
            .clone()
            .mul(edge_unit.clone())
            .sum_dim(2)
            .reshape([batch, n_e, 1]);
        let eps_ax = elong.div(edge_len.clamp_min(1e-30));

        let tx = edge_unit.clone().slice([0..batch, 0..n_e, 0..1]);
        let ty = edge_unit.clone().slice([0..batch, 0..n_e, 1..2]);
        let tz = edge_unit.slice([0..batch, 0..n_e, 2..3]);

        let v0 = eps_ax.clone().mul(tx.clone().mul(tx.clone()));
        let v1 = eps_ax.clone().mul(ty.clone().mul(ty.clone()));
        let v2 = eps_ax.clone().mul(tz.clone().mul(tz.clone()));
        let v3 = eps_ax.clone().mul(tx.clone().mul(ty.clone()));
        let v4 = eps_ax.clone().mul(ty.clone().mul(tz.clone()));
        let v5 = eps_ax.clone().mul(tx.clone().mul(tz.clone()));

        let voigt_edge = Tensor::cat(vec![v0, v1, v2, v3, v4, v5], 2);

        let topo = EdgeTopology::new(edges_b1);
        let src6 = topo.expand_src_gather_indices(batch, 6);
        let tgt6 = topo.expand_tgt_gather_indices(batch, 6);

        let flat6 = voigt_edge.reshape([batch, n_e, 6]);
        let acc = Tensor::<B, 3>::zeros([batch, n_v, 6], &device)
            .scatter(1, src6, flat6.clone())
            .scatter(1, tgt6, flat6);

        let deg = Self::nodal_degree_bn1(&topo.edges_b1, batch, n_v, &device);
        acc.div(deg.clamp_min(1.0))
    }

    /// Cauchy stress \(\boldsymbol\sigma\) as `[B, N, 3, 3]` (symmetric) from Voigt strain and isotropic Hooke law.
    ///
    /// # Shapes
    /// * `epsilon_voigt` — `[B, N, 6]` in the **same** Voigt layout as
    ///   [`Self::voigt_strain_from_edge_displacement`] (tensor \(\varepsilon_{ij}\), \(i\le j\) block).
    /// * `e_young`, `nu` — `[B, N, 1]` Lamé pair inputs.
    /// * `rotation` — `[B, N, 3, 3]`: **`R`** maps **local** frame (where `epsilon_voigt` lives) to **global**
    ///   lab axes via \(\boldsymbol\sigma_{\mathrm{global}} = \mathbf R\,\boldsymbol\sigma_{\mathrm{local}}\,\mathbf R^{\mathsf T}\).
    ///
    /// For identity `rotation`, output stress matches \(\boldsymbol\sigma=\lambda\,\mathrm{tr}(\boldsymbol\varepsilon)\,\mathbf I + 2\mu\boldsymbol\varepsilon\).
    pub fn isotropic_hooke_sigma<B: Backend<FloatElem = f32>>(
        epsilon_voigt: Tensor<B, 3>,
        e_young: Tensor<B, 3>,
        nu: Tensor<B, 3>,
        rotation: Tensor<B, 4>,
    ) -> Tensor<B, 4> {
        let batch = epsilon_voigt.dims()[0];
        let n = epsilon_voigt.dims()[1];
        debug_assert_eq!(e_young.dims(), [batch, n, 1]);
        debug_assert_eq!(nu.dims(), [batch, n, 1]);
        debug_assert_eq!(rotation.dims(), [batch, n, 3, 3]);

        let one = Tensor::<B, 3>::ones([batch, n, 1], &epsilon_voigt.device());
        let one_plus_nu = nu.clone().add_scalar(1.0);
        let one_minus_2nu = one.sub(nu.clone().mul_scalar(2.0));
        let lam = e_young
            .clone()
            .mul(nu)
            .div(one_plus_nu.clone().mul(one_minus_2nu));
        let mu = e_young.div(one_plus_nu.mul_scalar(2.0));

        let exx = epsilon_voigt.clone().slice([0..batch, 0..n, 0..1]);
        let eyy = epsilon_voigt.clone().slice([0..batch, 0..n, 1..2]);
        let ezz = epsilon_voigt.clone().slice([0..batch, 0..n, 2..3]);
        let exy = epsilon_voigt.clone().slice([0..batch, 0..n, 3..4]);
        let eyz = epsilon_voigt.clone().slice([0..batch, 0..n, 4..5]);
        let exz = epsilon_voigt.clone().slice([0..batch, 0..n, 5..6]);

        let tr = exx.clone().add(eyy.clone()).add(ezz.clone());
        let two_mu = mu.mul_scalar(2.0);

        let sig_xx = tr
            .clone()
            .mul(lam.clone())
            .add(exx.clone().mul(two_mu.clone()));
        let sig_yy = tr
            .clone()
            .mul(lam.clone())
            .add(eyy.clone().mul(two_mu.clone()));
        let sig_zz = tr
            .clone()
            .mul(lam.clone())
            .add(ezz.clone().mul(two_mu.clone()));
        let sig_xy = exy.clone().mul(two_mu.clone());
        let sig_yz = eyz.clone().mul(two_mu.clone());
        let sig_xz = exz.clone().mul(two_mu);

        let sigma_local =
            Self::symmetric_voigt6_to_tensor::<B>(sig_xx, sig_yy, sig_zz, sig_xy, sig_yz, sig_xz);
        let rt = rotation.clone().transpose();
        rotation.matmul(sigma_local).matmul(rt)
    }

    fn symmetric_voigt6_to_tensor<B: Backend<FloatElem = f32>>(
        sxx: Tensor<B, 3>,
        syy: Tensor<B, 3>,
        szz: Tensor<B, 3>,
        sxy: Tensor<B, 3>,
        syz: Tensor<B, 3>,
        sxz: Tensor<B, 3>,
    ) -> Tensor<B, 4> {
        let row0 =
            Tensor::cat(vec![sxx.clone(), sxy.clone(), sxz.clone()], 2).unsqueeze_dim::<4>(2);
        let row1 =
            Tensor::cat(vec![sxy.clone(), syy.clone(), syz.clone()], 2).unsqueeze_dim::<4>(2);
        let row2 = Tensor::cat(vec![sxz, syz, szz], 2).unsqueeze_dim::<4>(2);
        Tensor::cat(vec![row0, row1, row2], 2)
    }

    fn nodal_degree_bn1<B: Backend<FloatElem = f32>>(
        edges_b1: &Tensor<B, 2, Int>,
        batch: usize,
        n_v: usize,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_e = topo.n_edges();
        let src1 = topo.expand_src_gather_indices(batch, 1);
        let tgt1 = topo.expand_tgt_gather_indices(batch, 1);
        let ones = Tensor::<B, 3>::ones([batch, n_e, 1], device);
        Tensor::<B, 3>::zeros([batch, n_v, 1], device)
            .scatter(1, src1, ones.clone())
            .scatter(1, tgt1, ones)
    }

    /// Embed a single batch row `[1, n_v, 3]` into `[batch, n_v, 3]` with zeros elsewhere (for `bar_matvec`).
    pub(crate) fn embed_batch_row<B: Backend<FloatElem = f32>>(
        template: &Tensor<B, 3>,
        batch_idx: usize,
        n_v: usize,
        row: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let batch = template.dims()[0];
        Tensor::<B, 3>::zeros([batch, n_v, 3], &template.device())
            .slice_assign([batch_idx..batch_idx + 1, 0..n_v, 0..3], row)
    }

    /// \(R_u = P(\mathbf f_{\mathrm{ext}} - K(P\mathbf u))\) for the packed axial bar network (`P` =
    /// `boundary_mask`). Used by THMC at a trial displacement. **Not** the scalar **`acoustics-newmark`**
    /// periodic-bar wave path (verification **#10**).
    ///
    /// Optional `edge_shrink_strain_increment` \([B,E,1]\): dimensionless **increment** of free shrink
    /// strain along each edge (THMC notional hook). Elastic axial elongation uses
    /// \(\delta L_{\mathrm{el}} = \delta L_{\mathrm{geom}} - \varepsilon_{\Delta}\,L\) before the \(k_e\)
    /// spring law.
    #[cfg(feature = "thmc-coupled")]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn projected_bar_equilibrium_residual<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        edge_shrink_strain_increment: Option<Tensor<B, 3>>,
    ) -> Tensor<B, 3> {
        let batch = stiffness.dims()[0];
        let n_v = coords.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();

        let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n_v, 3]);

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
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());

        let e_young = stiffness.clone().slice([0..batch, 0..n_v, 0..1]);
        let e_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(e_young.clone(), edges_b1.clone());

        let d_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(damage.clone(), edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);

        let k_axial = e_on_edges
            .mul_scalar(cross_section_area)
            .div(edge_len.clone())
            .mul(dmg);

        let u_proj = displacement.mul(boundary_mask.clone());
        let ku = Self::bar_matvec(
            u_proj,
            &k_axial,
            &edge_unit,
            &src_indices,
            &tgt_indices,
            n_v,
            edge_shrink_strain_increment.as_ref(),
            &edge_len,
        );
        boundary_mask.mul(body_force.sub(ku))
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub(crate) fn packed_bar_network_equilibrium<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> (
        Tensor<B, 3>,
        Tensor<B, 3>,
        Tensor<B, 3>,
        Tensor<B, 3>,
        Tensor<B, 3, Int>,
        Tensor<B, 3, Int>,
        usize,
        BarNetworkPcgReport,
    ) {
        let batch = stiffness.dims()[0];
        let n_v = coords.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();

        let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n_v, 3]);

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
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());

        let e_young = stiffness.clone().slice([0..batch, 0..n_v, 0..1]);
        let e_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(e_young.clone(), edges_b1.clone());

        let d_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(damage.clone(), edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);

        let k_axial = e_on_edges
            .mul_scalar(cross_section_area)
            .div(edge_len.clone())
            .mul(dmg.clone());

        // Nondimensionalize `K u = f` as `(K/k_char) u = f/k_char`. Use `E_max−E_min` when SIMP spans
        // a range; on uniform modulus fall back to `E_max·A/Δx` so a flat field does not zero out `K`.
        let e_lo = e_young.clone().min().into_scalar();
        let e_hi = e_young.clone().max().into_scalar().max(1e-12_f32);
        let e_range = e_hi - e_lo;
        let dx_char = edge_len.clone().mean().into_scalar().max(1e-12_f32);
        let k_char = if e_range < e_hi * 1e-8_f32 {
            (e_hi * cross_section_area / dx_char).max(1e-30_f32)
        } else {
            (e_range * cross_section_area / dx_char).max(1e-30_f32)
        };
        #[cfg(not(feature = "mechanics-adjoint"))]
        let k_solve = k_axial.clone().div_scalar(k_char);
        #[cfg(not(feature = "mechanics-adjoint"))]
        let body_force_solve = body_force.clone().div_scalar(k_char);

        let mut u = displacement.mul(boundary_mask.clone());
        #[cfg(not(feature = "mechanics-adjoint"))]
        let template = u.clone();
        // Projected CG on \(\{u = P u\}\) with \(P=\) `boundary_mask`. The adjoint lane uses an f64
        // matvec/PCG loop (tensors remain f32); non-adjoint builds keep the legacy f32 Burn loop.
        #[cfg(feature = "mechanics-adjoint")]
        let pcg_report = {
            Self::packed_bar_network_equilibrium_pcg_f64(
                &mut u,
                &body_force,
                &boundary_mask,
                &k_axial,
                &edge_unit,
                &edges_b1,
                &src_indices,
                &tgt_indices,
                n_v,
                batch,
                inner_cfg,
                k_char,
                e_hi,
                dx_char,
            )
        };
        #[cfg(not(feature = "mechanics-adjoint"))]
        let pcg_report = {
            let mut report = BarNetworkPcgReport::default();
            let max_it = inner_cfg
                .max_cg_iterations
                .max(1)
                .min(n_v.saturating_mul(3).max(1));
            let rel_tol = inner_cfg.pcg_tolerance.max(inner_cfg.cg_tolerance).max(0.0);
            for b in 0..batch {
                let p_mask = boundary_mask.clone().slice([b..b + 1, 0..n_v, 0..3]);
                let f_b = body_force_solve.clone().slice([b..b + 1, 0..n_v, 0..3]);
                let mut u_c = u.clone().slice([b..b + 1, 0..n_v, 0..3]);

                let u_emb = Self::embed_batch_row(&template, b, n_v, u_c.clone());
                let ku_b = Self::bar_matvec(
                    u_emb,
                    &k_solve,
                    &edge_unit,
                    &src_indices,
                    &tgt_indices,
                    n_v,
                    None,
                    &edge_len,
                )
                .slice([b..b + 1, 0..n_v, 0..3]);
                let mut r = p_mask.clone().mul(f_b.clone().sub(ku_b));

                let rhs_norm = f_b
                    .clone()
                    .mul(p_mask.clone())
                    .powf_scalar(2.0)
                    .sum()
                    .sqrt()
                    .into_scalar()
                    .max(1e-30_f32);
                let abs_tol = rel_tol * rhs_norm;
                let use_tol_exit = rel_tol > 0.0;

                let k_b = k_solve.clone().slice([b..b + 1, 0..n_edges, 0..1]);
                let eu_b = edge_unit.clone().slice([b..b + 1, 0..n_edges, 0..3]);
                let diag_bn3 =
                    Self::assemble_bar_network_diagonal_bn3(k_b, eu_b, edges_b1.clone(), n_v);

                let mut z = if inner_cfg.use_preconditioner {
                    p_mask
                        .clone()
                        .mul(r.clone().div(diag_bn3.clone().clamp_min(1e-18_f32)))
                } else {
                    r.clone()
                };
                let mut p = z.clone();
                let mut pcg_iters = 0usize;
                let mut pcg_rel_res = f32::INFINITY;

                for _ in 0..max_it {
                    pcg_iters += 1;
                    let p_emb = Self::embed_batch_row(&template, b, n_v, p.clone());
                    let ap_raw = Self::bar_matvec(
                        p_emb,
                        &k_solve,
                        &edge_unit,
                        &src_indices,
                        &tgt_indices,
                        n_v,
                        None,
                        &edge_len,
                    )
                    .slice([b..b + 1, 0..n_v, 0..3]);
                    let ap_b = p_mask.clone().mul(ap_raw);

                    let rz = (r.clone().mul(z.clone())).sum();
                    if !rz.clone().into_scalar().is_finite() {
                        break;
                    }
                    let pap = (p.clone().mul(ap_b.clone())).sum().clamp_min(1e-30_f32);
                    let alpha = rz.clone().div(pap).reshape([1, 1, 1]);
                    u_c = u_c.add(p.clone().mul(alpha.clone()));
                    let u_emb2 = Self::embed_batch_row(&template, b, n_v, u_c.clone());
                    let ku_next = Self::bar_matvec(
                        u_emb2,
                        &k_solve,
                        &edge_unit,
                        &src_indices,
                        &tgt_indices,
                        n_v,
                        None,
                        &edge_len,
                    )
                    .slice([b..b + 1, 0..n_v, 0..3]);
                    let r_next = p_mask.clone().mul(f_b.clone().sub(ku_next));

                    let z_next = if inner_cfg.use_preconditioner {
                        p_mask
                            .clone()
                            .mul(r_next.clone().div(diag_bn3.clone().clamp_min(1e-18_f32)))
                    } else {
                        r_next.clone()
                    };

                    let rz_next = (r_next.clone().mul(z_next.clone())).sum();
                    let beta = rz_next
                        .div(rz.clone().clamp_min(1e-30_f32))
                        .reshape([1, 1, 1]);
                    if !beta.clone().into_scalar().is_finite() {
                        break;
                    }
                    p = z_next.clone().add(p.mul(beta));
                    r = r_next;
                    z = z_next;

                    let r_norm = r.clone().powf_scalar(2.0).sum().sqrt().into_scalar();
                    pcg_rel_res = r_norm / rhs_norm;
                    if use_tol_exit && r_norm <= abs_tol {
                        break;
                    }
                }

                u = u.slice_assign([b..b + 1, 0..n_v, 0..3], u_c);
                report = BarNetworkPcgReport {
                    iterations: pcg_iters,
                    rel_residual: pcg_rel_res,
                    stiffness_scale: k_char,
                    e_ref: e_hi,
                    dx_char,
                };
            }
            report
        };

        (
            u,
            k_axial,
            edge_unit,
            edge_len,
            src_indices,
            tgt_indices,
            n_v,
            pcg_report,
        )
    }

    /// \(\|P(f-Ku)\|_2 / \|Pf\|_2\) after a bar-network solve (B6 H4 static residual; no adjoint PCG).
    #[allow(clippy::too_many_arguments)]
    pub fn bar_network_equilibrium_rel_residual<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
    ) -> f32 {
        let f = body_force.clone();
        let mask = boundary_mask.clone();
        let resid = Self::projected_bar_equilibrium_residual_inner(
            displacement,
            coords,
            stiffness,
            body_force,
            edges_b1,
            damage,
            boundary_mask,
            cross_section_area,
        );
        let rhs_norm = f
            .mul(mask)
            .powf_scalar(2.0)
            .sum()
            .sqrt()
            .into_scalar()
            .max(1e-30_f32);
        resid.powf_scalar(2.0).sum().sqrt().into_scalar() / rhs_norm
    }

    #[allow(clippy::too_many_arguments)]
    fn projected_bar_equilibrium_residual_inner<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
    ) -> Tensor<B, 3> {
        let batch = stiffness.dims()[0];
        let n_v = coords.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let n_edges = topo.n_edges();

        let coords_b = coords.clone().unsqueeze_dim::<3>(0).expand([batch, n_v, 3]);

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
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());

        let e_young = stiffness.clone().slice([0..batch, 0..n_v, 0..1]);
        let e_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(e_young.clone(), edges_b1.clone());

        let d_on_edges =
            DecEdgeOperators::arithmetic_mean_on_edges(damage.clone(), edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);

        let k_axial = e_on_edges
            .mul_scalar(cross_section_area)
            .div(edge_len.clone())
            .mul(dmg);

        let u_proj = displacement.mul(boundary_mask.clone());
        let ku = Self::bar_matvec(
            u_proj,
            &k_axial,
            &edge_unit,
            &src_indices,
            &tgt_indices,
            n_v,
            None,
            &edge_len,
        );
        boundary_mask.mul(body_force.sub(ku))
    }

    #[cfg(feature = "mechanics-voigt-cauchy")]
    fn identity_rotation_bn33<B: Backend<FloatElem = f32>>(
        batch: usize,
        n_v: usize,
        device: &B::Device,
    ) -> Tensor<B, 4> {
        Tensor::<B, 2>::eye(3, device)
            .reshape([1, 1, 3, 3])
            .expand([batch, n_v, 3, 3])
    }

    #[cfg(feature = "mechanics-voigt-cauchy")]
    fn nodal_cauchy_stress_voigt_isotropic<B: Backend<FloatElem = f32>>(
        u: Tensor<B, 3>,
        stiffness: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        edge_unit: Tensor<B, 3>,
        edge_len: Tensor<B, 3>,
        n_v: usize,
    ) -> Tensor<B, 4> {
        let batch = u.dims()[0];
        let topo = EdgeTopology::new(edges_b1.clone());
        let src_indices = topo.expand_src_gather_indices(batch, 3);
        let tgt_indices = topo.expand_tgt_gather_indices(batch, 3);
        let u_src = u.clone().gather(1, src_indices.clone());
        let u_tgt = u.clone().gather(1, tgt_indices.clone());
        // Same convention as [`Self::voigt_strain_from_edge_displacement`]: `u_tgt − u_src`.
        let edge_displacement = u_tgt.sub(u_src);
        let eps_v = Self::voigt_strain_from_edge_displacement(
            edge_displacement,
            edge_unit,
            edge_len,
            edges_b1,
            n_v,
        );
        let e_young = stiffness.clone().slice([0..batch, 0..n_v, 0..1]);
        let nu = stiffness.slice([0..batch, 0..n_v, 1..2]);
        let device = u.device();
        let rotation = Self::identity_rotation_bn33::<B>(batch, n_v, &device);
        Self::isotropic_hooke_sigma(eps_v, e_young, nu, rotation)
    }

    /// Equilibrium solve \(K\mathbf u = \mathbf f\) for a bar network.
    ///
    /// # Shapes
    /// * `displacement` — `[B, N, 3]` initial guess.
    /// * `coords` — `[N, 3]` vertex positions (shared across batch).
    /// * `stiffness` — `[B, N, 2]`, columns `[E_young, \nu]` (`\nu` reserved for continuum shell solids).
    /// * `body_force` — `[B, N, 3]`.
    /// * `edges_b1` — `[2, E]` (source row, target row).
    /// * `damage` — `[B, N, 1]` continuous damage in \([0, 1]\).
    /// * `boundary_mask` — `[B, N, 3]`, `1` = free DOF, `0` = fixed (Dirichlet).
    ///
    /// Returns `(displacement, cauchy_stress)` with **bar-network** rank-one Cauchy stress `[B, N, 3, 3]`. For continuum Voigt+Hooke recovery on the same displacement field, see [`Self::solve_equilibrium_with_voigt_cauchy`] (`solver-experimental`).
    #[allow(clippy::too_many_arguments)] // Tensor-shaped inputs stay explicit at the burn boundary.
    pub fn solve_equilibrium<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 4>) {
        let (u, k_axial, edge_unit, _edge_len, src_indices, tgt_indices, n_v, _pcg) =
            Self::packed_bar_network_equilibrium(
                displacement,
                coords,
                stiffness.clone(),
                body_force,
                edges_b1.clone(),
                damage,
                boundary_mask,
                cross_section_area,
                inner_cfg,
            );
        let stress = Self::nodal_stress_from_bars(
            u.clone(),
            &k_axial,
            &edge_unit,
            &src_indices,
            &tgt_indices,
            n_v,
            cross_section_area,
        );
        (u, stress)
    }

    /// Quasi-static bar-network equilibrium with PCG iteration / relative-residual report (B6 H4 gates).
    #[cfg(feature = "mechanics-adjoint")]
    #[allow(clippy::too_many_arguments)]
    pub fn solve_equilibrium_with_pcg_report<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 4>, BarNetworkPcgReport) {
        let (u, k_axial, edge_unit, _edge_len, src_indices, tgt_indices, n_v, pcg) =
            Self::packed_bar_network_equilibrium(
                displacement,
                coords,
                stiffness.clone(),
                body_force,
                edges_b1.clone(),
                damage,
                boundary_mask,
                cross_section_area,
                inner_cfg,
            );
        let stress = Self::nodal_stress_from_bars(
            u.clone(),
            &k_axial,
            &edge_unit,
            &src_indices,
            &tgt_indices,
            n_v,
            cross_section_area,
        );
        (u, stress, pcg)
    }

    #[cfg(feature = "mechanics-voigt-cauchy")]
    /// Same bar-network equilibrium as [`Self::solve_equilibrium`], but recover Cauchy stress with
    /// [`Self::voigt_strain_from_edge_displacement`] and [`Self::isotropic_hooke_sigma`] (\(\mathbf R = \mathbf I\)).
    ///
    /// The stiffness operator is unchanged from the DEC axial bar network; only **stress post-processing**
    /// follows the graph Voigt strain to the 3D isotropic Hooke map (Phase-1 elasticity slice).
    #[allow(clippy::too_many_arguments)]
    pub fn solve_equilibrium_with_voigt_cauchy<B: Backend<FloatElem = f32>>(
        displacement: Tensor<B, 3>,
        coords: Tensor<B, 2>,
        stiffness: Tensor<B, 3>,
        body_force: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        damage: Tensor<B, 3>,
        boundary_mask: Tensor<B, 3>,
        cross_section_area: f32,
        inner_cfg: &MechanicsInnerLoopConfig,
    ) -> (Tensor<B, 3>, Tensor<B, 4>) {
        let (u, _k_axial, edge_unit, edge_len, _src_indices, _tgt_indices, n_v, _pcg) =
            Self::packed_bar_network_equilibrium(
                displacement,
                coords,
                stiffness.clone(),
                body_force,
                edges_b1.clone(),
                damage,
                boundary_mask,
                cross_section_area,
                inner_cfg,
            );
        let stress = Self::nodal_cauchy_stress_voigt_isotropic(
            u.clone(),
            stiffness,
            edges_b1,
            edge_unit,
            edge_len,
            n_v,
        );
        (u, stress)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bar_matvec<B: Backend<FloatElem = f32>>(
        u: Tensor<B, 3>,
        k_axial: &Tensor<B, 3>,
        edge_unit: &Tensor<B, 3>,
        src_indices: &Tensor<B, 3, Int>,
        tgt_indices: &Tensor<B, 3, Int>,
        n_v: usize,
        edge_shrink_strain_increment: Option<&Tensor<B, 3>>,
        edge_len: &Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let batch = u.dims()[0];
        let device = u.device();
        let u_src = u.clone().gather(1, src_indices.clone());
        let u_tgt = u.clone().gather(1, tgt_indices.clone());
        let du = u_src.sub(u_tgt);
        let mut elong = du
            .mul(edge_unit.clone())
            .sum_dim(2)
            .reshape([batch, k_axial.dims()[1], 1]);
        if let Some(ess) = edge_shrink_strain_increment {
            elong = elong.sub(ess.clone().mul(edge_len.clone()));
        }
        let axial = elong.mul(k_axial.clone());
        let f_vec = edge_unit.clone().mul(axial);

        // Same scatter-sum pattern as [`super::laplacian::TopologicalLaplacian::scalar_laplacian`]:
        // endpoint src receives `+f`, tgt receives `−f` (bar tension force convention).
        Tensor::<B, 3>::zeros([batch, n_v, 3], &device)
            .scatter(1, src_indices.clone(), f_vec.clone())
            .scatter(1, tgt_indices.clone(), f_vec.neg())
    }

    /// Diagonal entries of the bar stiffness operator on \(\mathbb{R}^{3N}\): per node / axis
    /// \(\sum_{e\ni i} k_e \hat{t}_{e,d}^2\) (used for Jacobi preconditioning).
    fn assemble_bar_network_diagonal_bn3<B: Backend<FloatElem = f32>>(
        k_axial: Tensor<B, 3>,
        edge_unit: Tensor<B, 3>,
        edges_b1: Tensor<B, 2, Int>,
        n_v: usize,
    ) -> Tensor<B, 3> {
        let batch = k_axial.dims()[0];
        let n_e = k_axial.dims()[1];
        debug_assert_eq!(edge_unit.dims(), [batch, n_e, 3]);
        let device = k_axial.device();
        let t2 = edge_unit.powf_scalar(2.0);
        let contrib = k_axial.mul(t2);
        let topo = EdgeTopology::new(edges_b1);
        let src_ix = topo.expand_src_gather_indices(batch, 3);
        let tgt_ix = topo.expand_tgt_gather_indices(batch, 3);
        Tensor::<B, 3>::zeros([batch, n_v, 3], &device)
            .scatter(1, src_ix, contrib.clone())
            .scatter(1, tgt_ix, contrib)
    }

    #[cfg(feature = "mechanics-adjoint")]
    fn bar_network_projected_matvec_f64(
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

    #[cfg(feature = "mechanics-adjoint")]
    #[allow(clippy::too_many_arguments)]
    fn packed_bar_network_equilibrium_pcg_f64<B: Backend<FloatElem = f32>>(
        u: &mut Tensor<B, 3>,
        body_force_solve: &Tensor<B, 3>,
        boundary_mask: &Tensor<B, 3>,
        k_solve: &Tensor<B, 3>,
        edge_unit: &Tensor<B, 3>,
        edges_b1: &Tensor<B, 2, Int>,
        src_indices: &Tensor<B, 3, Int>,
        tgt_indices: &Tensor<B, 3, Int>,
        n_v: usize,
        batch: usize,
        inner_cfg: &MechanicsInnerLoopConfig,
        k_char: f32,
        e_hi: f32,
        dx_char: f32,
    ) -> BarNetworkPcgReport {
        let n_e = k_solve.dims()[1];
        let edges_flat = edges_b1.clone().into_data().value;
        let src: Vec<usize> = (0..n_e)
            .map(|e| edges_flat[e].elem::<i32>() as usize)
            .collect();
        let tgt: Vec<usize> = (0..n_e)
            .map(|e| edges_flat[n_e + e].elem::<i32>() as usize)
            .collect();
        let _ = (src_indices, tgt_indices);
        // f64 lane: honour caller `max_cg_iterations` without the f32 `3N` early cap (ill-conditioned
        // Striatus-scale bar nets can need more passes than subspace dimension in f32).
        let max_it = inner_cfg.max_cg_iterations.max(1);
        let rel_tol = inner_cfg.pcg_tolerance.max(inner_cfg.cg_tolerance).max(0.0) as f64;

        let mut report = BarNetworkPcgReport::default();
        let ndof = n_v * 3;
        let mut ku = vec![0.0_f64; ndof];
        let mut ap = vec![0.0_f64; ndof];
        let mut r = vec![0.0_f64; ndof];
        let mut z = vec![0.0_f64; ndof];
        let mut p = vec![0.0_f64; ndof];

        for b in 0..batch {
            let f_flat = body_force_solve
                .clone()
                .slice([b..b + 1, 0..n_v, 0..3])
                .into_data()
                .value;
            let mask_flat = boundary_mask
                .clone()
                .slice([b..b + 1, 0..n_v, 0..3])
                .into_data()
                .value;
            let mut u_flat = u.clone().slice([b..b + 1, 0..n_v, 0..3]).into_data().value;
            let k_flat = k_solve
                .clone()
                .slice([b..b + 1, 0..n_e, 0..1])
                .into_data()
                .value;
            let e_scale = e_hi.max(1e-12) as f64;
            let eu_flat = edge_unit
                .clone()
                .slice([b..b + 1, 0..n_e, 0..3])
                .into_data()
                .value;

            let mask64: Vec<f64> = mask_flat.iter().map(|&x| x as f64).collect();
            let f_rhs: Vec<f64> = f_flat.iter().map(|&x| (x as f64) / e_scale).collect();
            let mut u64: Vec<f64> = u_flat.iter().map(|&x| x as f64).collect();
            let k64: Vec<f64> = k_flat.iter().map(|&x| (x as f64) / e_scale).collect();
            let eu64: Vec<f64> = eu_flat.iter().map(|&x| x as f64).collect();

            for i in 0..ndof {
                u64[i] *= mask64[i];
            }

            Self::bar_network_projected_matvec_f64(&u64, &mut ku, &mask64, &k64, &eu64, &src, &tgt);
            for i in 0..ndof {
                r[i] = mask64[i] * (f_rhs[i] - ku[i]);
            }

            let rhs_norm = f_rhs
                .iter()
                .zip(&mask64)
                .map(|(&fi, &m)| (fi * m).powi(2))
                .sum::<f64>()
                .sqrt()
                .max(1e-30);
            let abs_tol = rel_tol * rhs_norm;

            let diag_bn3 = Self::assemble_bar_network_diagonal_bn3(
                k_solve.clone().slice([b..b + 1, 0..n_e, 0..1]),
                edge_unit.clone().slice([b..b + 1, 0..n_e, 0..3]),
                edges_b1.clone(),
                n_v,
            );
            let diag_flat = diag_bn3.into_data().value;
            let diag64: Vec<f64> = diag_flat.iter().map(|&x| x as f64).collect();

            if inner_cfg.use_preconditioner {
                for i in 0..ndof {
                    z[i] = mask64[i] * r[i] / diag64[i].max(1e-18);
                }
            } else {
                z.copy_from_slice(&r);
            }
            p.copy_from_slice(&z);

            let mut pcg_iters = 0usize;
            let mut pcg_rel_res = f64::INFINITY;

            for _ in 0..max_it {
                pcg_iters += 1;
                Self::bar_network_projected_matvec_f64(
                    &p, &mut ap, &mask64, &k64, &eu64, &src, &tgt,
                );

                let rz: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
                if !rz.is_finite() {
                    break;
                }
                let pap: f64 = p
                    .iter()
                    .zip(&ap)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    .max(1e-30);
                let alpha = rz / pap;
                for i in 0..ndof {
                    u64[i] += alpha * p[i];
                    u64[i] *= mask64[i];
                }

                Self::bar_network_projected_matvec_f64(
                    &u64, &mut ku, &mask64, &k64, &eu64, &src, &tgt,
                );
                for i in 0..ndof {
                    r[i] = mask64[i] * (f_rhs[i] - ku[i]);
                }

                if inner_cfg.use_preconditioner {
                    for i in 0..ndof {
                        z[i] = mask64[i] * r[i] / diag64[i].max(1e-18);
                    }
                } else {
                    z.copy_from_slice(&r);
                }

                let rz_next: f64 = r.iter().zip(&z).map(|(a, b)| a * b).sum();
                let beta = rz_next / rz.max(1e-30);
                if !beta.is_finite() {
                    break;
                }
                for i in 0..ndof {
                    p[i] = (z[i] + beta * p[i]) * mask64[i];
                }

                let r_norm: f64 = r.iter().map(|x| x * x).sum::<f64>().sqrt();
                pcg_rel_res = r_norm / rhs_norm;
                if rel_tol > 0.0 && r_norm <= abs_tol {
                    break;
                }
            }

            u_flat = u64.iter().map(|&x| x as f32).collect();
            let u_slice = Tensor::from_data(
                burn::tensor::Data::new(u_flat, burn::tensor::Shape::new([1, n_v, 3])),
                &u.device(),
            );
            *u = u.clone().slice_assign([b..b + 1, 0..n_v, 0..3], u_slice);
            report = BarNetworkPcgReport {
                iterations: pcg_iters,
                rel_residual: pcg_rel_res as f32,
                stiffness_scale: k_char,
                e_ref: e_hi,
                dx_char,
            };
        }
        report
    }

    fn nodal_stress_from_bars<B: Backend<FloatElem = f32>>(
        u: Tensor<B, 3>,
        k_axial: &Tensor<B, 3>,
        edge_unit: &Tensor<B, 3>,
        src_indices: &Tensor<B, 3, Int>,
        tgt_indices: &Tensor<B, 3, Int>,
        n_v: usize,
        area: f32,
    ) -> Tensor<B, 4> {
        let batch = u.dims()[0];
        let n_edges = k_axial.dims()[1];
        let device = u.device();
        let u_src = u.clone().gather(1, src_indices.clone());
        let u_tgt = u.clone().gather(1, tgt_indices.clone());
        let du = u_src.sub(u_tgt);
        let elong = du
            .mul(edge_unit.clone())
            .sum_dim(2)
            .reshape([batch, n_edges, 1]);
        let faxial = elong.mul(k_axial.clone());
        let sigma_scale = faxial.div_scalar(area);

        let ex = edge_unit.clone().unsqueeze_dim::<4>(3);
        let ey = edge_unit.clone().unsqueeze_dim::<4>(2);
        let sig = sigma_scale.reshape([batch, n_edges, 1, 1]);
        let stress_edge = ex.mul(ey).mul(sig);

        let flat = stress_edge.reshape([batch, n_edges, 9]);
        let src9 = src_indices
            .clone()
            .slice([0..batch, 0..n_edges, 0..1])
            .expand([batch, n_edges, 9]);
        let tgt9 = tgt_indices
            .clone()
            .slice([0..batch, 0..n_edges, 0..1])
            .expand([batch, n_edges, 9]);

        let acc = Tensor::<B, 3>::zeros([batch, n_v, 9], &device)
            .scatter(1, src9, flat.clone())
            .scatter(1, tgt9, flat);

        acc.reshape([batch, n_v, 3, 3])
    }
}

/// Self-weight nodal body force with Bruyneel–Duysinx mass penalty \(m(\rho)=\rho^q\) decoupled from SIMP stiffness \(p\).
///
/// formal_anchor: Literature  
/// formal_citation: Bruyneel & Duysinx 2005, Struct. Multidisc. Optim. 29:245-256  
/// formal_form: \(\mathbf f = \rho^q\,V_{\mathrm{voxel}}\,g\,\hat{\mathbf d}\) with unit direction \(\hat{\mathbf d}\)
#[cfg(feature = "topology-density-evolution")]
#[derive(Clone, Copy, Debug)]
pub struct SelfWeightConfig {
    /// Downward acceleration magnitude (positive e.g. `9.81`); combined with [`Self::direction`].
    pub gravity_m_s2: f32,
    pub voxel_volume_m3: f32,
    pub mass_penalty_q: f32,
    pub direction: [f32; 3],
}

#[cfg(feature = "topology-density-evolution")]
impl SelfWeightConfig {
    /// Per-node body force flat `[N*3]` for FD / inner-only paths (batch **1**).
    #[must_use]
    pub fn body_force_flat(&self, rho_flat: &[f32], n_nodes: usize) -> Vec<f32> {
        let [dx, dy, dz] = self.direction;
        let mag = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-30);
        let ux = dx / mag;
        let uy = dy / mag;
        let uz = dz / mag;
        let gvol = self.voxel_volume_m3 * self.gravity_m_s2;
        let q = self.mass_penalty_q;
        let mut f = vec![0.0_f32; n_nodes * 3];
        for i in 0..n_nodes.min(rho_flat.len()) {
            let m = rho_flat[i].clamp(0.0, 1.0).powf(q) * gvol;
            f[i * 3] = m * ux;
            f[i * 3 + 1] = m * uy;
            f[i * 3 + 2] = m * uz;
        }
        f
    }

    /// Per-node body force `[B,N,3]` matching `rho_projected` `[B,N,1]`.
    pub fn body_force<B: Backend<FloatElem = f32>>(
        &self,
        rho_projected: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        let [dx, dy, dz] = self.direction;
        let mag = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-30);
        let ux = dx / mag;
        let uy = dy / mag;
        let uz = dz / mag;
        let m = rho_projected
            .powf_scalar(self.mass_penalty_q)
            .mul_scalar(self.voxel_volume_m3 * self.gravity_m_s2);
        let fx = m.clone().mul_scalar(ux);
        let fy = m.clone().mul_scalar(uy);
        let fz = m.mul_scalar(uz);
        Tensor::cat(vec![fx, fy, fz], 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::dec_operators::DecEdgeOperators;
    use crate::physics::time_orchestration::MechanicsInnerLoopConfig;
    use crate::physics::topology::EdgeTopology;
    use burn::tensor::{Data, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    /// Tip displacement of a uniform axial chain with fixed left end — same tridiagonal stencil as a
    /// line of identical DEC bars (`k` = axial stiffness per edge).
    fn dense_chain_tip_displacement(n: usize, k_edge: f32, f_tip: f32) -> f32 {
        let v = dense_chain_reduced_solve(n, k_edge, f_tip);
        *v.last().expect("m >= 1")
    }

    /// Reduced unknowns `v[j] = u_{j+1}` for `j = 0 … n-2` with `u_0 = 0`.
    ///
    /// Uses the **`k_edge`-scaled** tridiagonal (coefficients `{-1, 2, -1}` on the interior, tip row
    /// `{-1, 1}`) and Thomas elimination — stable for the large axial stiffness magnitudes here.
    fn dense_chain_reduced_solve(n: usize, k_edge: f32, f_tip: f32) -> Vec<f32> {
        let m = n - 1;
        assert!(m > 0);
        if m == 1 {
            return vec![f_tip / k_edge];
        }
        let lower = vec![-k_edge; m - 1];
        let mut diag = vec![2.0 * k_edge; m];
        let upper = vec![-k_edge; m - 1];
        diag[m - 1] = k_edge;
        let mut rhs = vec![0.0_f32; m];
        rhs[m - 1] = f_tip;
        thomas_symmetric_chain(lower, diag, upper, rhs)
    }

    fn thomas_symmetric_chain(
        lower: Vec<f32>,
        diag: Vec<f32>,
        upper: Vec<f32>,
        rhs: Vec<f32>,
    ) -> Vec<f32> {
        let m = diag.len();
        let mut cp = diag;
        let mut rp = rhs;
        for i in 1..m {
            let w = lower[i - 1] / cp[i - 1];
            cp[i] -= w * upper[i - 1];
            rp[i] -= w * rp[i - 1];
        }
        let mut x = vec![0.0_f32; m];
        x[m - 1] = rp[m - 1] / cp[m - 1];
        for i in (0..m - 1).rev() {
            x[i] = (rp[i] - upper[i] * x[i + 1]) / cp[i];
        }
        x
    }

    #[test]
    fn one_d_bar_tip_displacement() {
        let dev = NdArrayDevice::Cpu;
        // Uniform axial chain, `n` nodes, fixed left (`u_x=0`), tip load `f` on free x-DOFs only.
        // Undamaged edges use axial stiffness `(EA/L)·((1−d)² + DAMAGE_REG)`; at `d = 0` this is
        // `k_edge = (EA/dx)·(1+DAMAGE_REG)` on every segment. The reduced displacement along x obeys the
        // textbook chain tridiagonal, so `u_tip = FL/(EA·(1+DAMAGE_REG))` with `L=(n−1)dx`.
        //
        // `edges_b1` must follow Burn row-major `[2, E]` layout: first `E` entries are **all** source
        // nodes, then `E` targets — not interleaved `(src,tgt)` pairs.
        //
        // Equilibrium uses **projected tensor CG** on the homogeneous Dirichlet subspace (`u = P u`),
        // with `bar_matvec` and inner products implemented as pure Burn ops so autodiff reaches
        // `k_axial` / geometry. That matches Thomas on the reduced chain for this uniform axial
        // benchmark (tight f32 agreement). The historical **packed-index** CG on `K_ff` was CPU-only
        // and severed the tape; it is superseded by this tensor path.
        let n: usize = 10;
        let l_total = 1.0_f32;
        let dx = l_total / (n - 1) as f32;
        let e = 200e9_f32;
        let a = 0.01_f32;
        let f = 1000.0_f32;
        let k_edge = e * a / dx * (1.0 + DAMAGE_REG);

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords: Tensor<B, 2> =
            Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);

        let mut stiff = Vec::with_capacity(n * 2);
        for _ in 0..n {
            stiff.push(e);
            stiff.push(0.3);
        }
        let stiffness: Tensor<B, 3> =
            Tensor::from_data(Data::new(stiff, Shape::new([1, n, 2])), &dev);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let displacement = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let mut bf_data = vec![0.0_f32; n * 3];
        bf_data[(n - 1) * 3] = f;
        let bf = Tensor::from_data(Data::new(bf_data, Shape::new([1, n, 3])), &dev);

        let mut bm_data = vec![1.0_f32; n * 3];
        for i in 0..n {
            bm_data[i * 3 + 1] = 0.0;
            bm_data[i * 3 + 2] = 0.0;
        }
        bm_data[0] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm_data, Shape::new([1, n, 3])), &dev);

        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 500,
            // Tight enough that packed CG’s `‖r‖₂ / ‖f‖₂` clears the residual assertion below in f32.
            cg_tolerance: 1e-8,
            pcg_tolerance: 1e-8,
            use_preconditioner: true,
            max_equilibrium_substeps: 1,
        };

        let (u, _) = VectorMechanicsSolver::solve_equilibrium(
            displacement,
            coords.clone(),
            stiffness,
            bf.clone(),
            edges_b1.clone(),
            damage,
            boundary_mask.clone(),
            a,
            &cfg,
        );

        // Consistency: static residual `(f − Ku)` on free DOFs after equilibrium solve.
        let batch = 1usize;
        let n_edges = n - 1;
        let coords_b = coords
            .clone()
            .unsqueeze_dim::<3>(0)
            .expand::<3, _>([batch, n, 3]);
        let topo = EdgeTopology::new(edges_b1.clone());
        assert_eq!(topo.n_edges(), n_edges);
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
            .reshape([batch, n_edges, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());
        let e_young = Tensor::<B, 3>::from_data(Data::new(vec![e; n], Shape::new([1, n, 1])), &dev);
        let e_on_edges = crate::physics::dec_operators::DecEdgeOperators::arithmetic_mean_on_edges(
            e_young,
            edges_b1.clone(),
        );
        let d_zero = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let d_on_edges = crate::physics::dec_operators::DecEdgeOperators::arithmetic_mean_on_edges(
            d_zero,
            edges_b1.clone(),
        );
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);
        let k_axial = e_on_edges.mul_scalar(a).div(edge_len.clone()).mul(dmg);

        let ku = VectorMechanicsSolver::bar_matvec(
            u.clone(),
            &k_axial,
            &edge_unit,
            &src_indices,
            &tgt_indices,
            n,
            None,
            &edge_len,
        );
        let res = bf.sub(ku).mul(boundary_mask.clone());
        let res_max = res.clone().abs().max().into_scalar();
        let res_flat = res.clone().into_data().value;
        let res_l2: f32 = res_flat.iter().map(|x| x * x).sum::<f32>().sqrt();
        let res_rel_l2 = res_l2 / f.max(1e-30_f32);
        // Packed CG + Thomas reference should agree; residual bounds guard assembly vs `bar_matvec`.
        assert!(
            res_max < f * 2e-6_f32 + 1e-5_f32,
            "free-DOF equilibrium residual max abs {res_max} (applied force scale {f})"
        );
        // Explicit `f − Ku` uses the same f32 `bar_matvec` path as CG; recurrent CG can report
        // `‖r‖/‖f‖ < cg_tolerance` while rounding in the final matvec leaves ‖(f−Ku)_free‖₂/‖f‖₂ ≈ 1.5×10⁻⁶.
        // A backward-error style margin `10⁻⁶ + m·ε` (`m` = count of free axial DOFs = `n−1`) matches
        // IEEE-754 f32 for this scale without hiding a broken assembly.
        let m_free_axial = (n - 1) as f32;
        let res_rel_tol = 1e-6_f32 + m_free_axial * f32::EPSILON;
        assert!(
            res_rel_l2 < res_rel_tol,
            "free-DOF equilibrium relative L2 residual {res_rel_l2} (tol {res_rel_tol}, rhs scale {f})"
        );

        let u_flat = u.clone().into_data().value;
        let tip = u.clone().slice([0..1, (n - 1)..n, 0..1]);
        let tip_val = tip.into_data().value[0];
        assert!(tip_val.is_finite() && tip_val > 0.0);

        let continuum_fl_over_ea = f * l_total / (a * e);
        let discrete_tip = continuum_fl_over_ea / (1.0 + DAMAGE_REG);
        let tip_dense = dense_chain_tip_displacement(n, k_edge, f);
        let v_dense = dense_chain_reduced_solve(n, k_edge, f);
        assert!(
            (tip_dense - discrete_tip).abs() / discrete_tip < 1e-4_f32,
            "dense_chain_tip_displacement {tip_dense} vs discrete analytic {discrete_tip}"
        );
        assert!(
            (tip_val - tip_dense).abs() / tip_dense < 1e-4_f32,
            "equilibrium CG tip {tip_val} vs dense_chain_tip_displacement {tip_dense}"
        );
        // Same `K_ff` as Thomas: every free axial displacement must match the dense reduced solve.
        let tol_u = tip_dense * 1e-5_f32 + 1e-12_f32;
        for j in 0..(n - 1) {
            let ux = u_flat[(j + 1) * 3];
            assert!(
                (ux - v_dense[j]).abs() < tol_u,
                "node {} u_x {ux} vs Thomas reduced {} (tol {tol_u})",
                j + 1,
                v_dense[j]
            );
        }
    }

    /// Probe reduced axial stiffness `K_ff` from [`VectorMechanicsSolver::bar_matvec`] columns vs the
    /// uniform-chain tridiagonal (`{-k, 2k, -k}` interior, tip row `{-k, k}`).
    #[test]
    fn chain_bar_reduced_k_matches_tridiagonal() {
        let dev = NdArrayDevice::Cpu;
        let n: usize = 10;
        let l_total = 1.0_f32;
        let dx = l_total / (n - 1) as f32;
        let e_y = 200e9_f32;
        let a_sec = 0.01_f32;
        let k_edge = e_y * a_sec / dx * (1.0 + DAMAGE_REG);

        let mut coords_data = Vec::with_capacity(n * 3);
        for i in 0..n {
            coords_data.push(i as f32 * dx);
            coords_data.push(0.0);
            coords_data.push(0.0);
        }
        let coords: Tensor<B, 2> =
            Tensor::from_data(Data::new(coords_data, Shape::new([n, 3])), &dev);

        let mut edges = Vec::with_capacity((n - 1) * 2);
        for eid in 0..(n - 1) {
            edges.push(eid as i64);
        }
        for eid in 0..(n - 1) {
            edges.push((eid + 1) as i64);
        }
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(edges, Shape::new([2, n - 1])), &dev);

        let batch = 1usize;
        let coords_b = coords
            .clone()
            .unsqueeze_dim::<3>(0)
            .expand::<3, _>([batch, n, 3]);
        let topo = EdgeTopology::new(edges_b1.clone());
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
            .reshape([batch, n - 1, 1]);
        let edge_unit = delta_geom.div(edge_len.clone());
        let e_on_edges = DecEdgeOperators::arithmetic_mean_on_edges(
            Tensor::<B, 3>::from_data(Data::new(vec![e_y; n], Shape::new([1, n, 1])), &dev),
            edges_b1.clone(),
        );
        let d_zero = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let d_on_edges = DecEdgeOperators::arithmetic_mean_on_edges(d_zero, edges_b1.clone());
        let dmg = Tensor::ones_like(&d_on_edges)
            .sub(d_on_edges)
            .powf_scalar(2.0)
            .add_scalar(DAMAGE_REG);
        let k_axial = e_on_edges.mul_scalar(a_sec).div(edge_len.clone()).mul(dmg);

        let pack_idx: Vec<usize> = (1..n).map(|node| node * 3).collect();
        let m = pack_idx.len();
        let mut k_red = vec![vec![0f32; m]; m];
        for j in 0..m {
            let mut row_u = vec![0f32; n * 3];
            row_u[pack_idx[j]] = 1.0;
            let u_t = Tensor::from_data(Data::new(row_u, Shape::new([1, n, 3])), &dev);
            let ku = VectorMechanicsSolver::bar_matvec(
                u_t,
                &k_axial,
                &edge_unit,
                &src_indices,
                &tgt_indices,
                n,
                None,
                &edge_len,
            );
            let ku_flat = ku.into_data().value;
            for i in 0..m {
                k_red[i][j] = ku_flat[pack_idx[i]];
            }
        }

        for (i, k_row) in k_red.iter().enumerate() {
            for (j, &val) in k_row.iter().enumerate() {
                let expected = if i == j {
                    if i == m - 1 {
                        k_edge
                    } else {
                        2.0 * k_edge
                    }
                } else if i.abs_diff(j) == 1 {
                    -k_edge
                } else {
                    0.0
                };
                assert!(
                    (val - expected).abs() < k_edge * 1e-4_f32,
                    "K[{i},{j}] got {val} expected {expected}",
                );
            }
        }
    }

    /// Single edge along \(+\hat{\mathbf e}_x\): uniform axial extension, Voigt strain matches rank-one
    /// \(\varepsilon_{xx}=\varepsilon_0\) (shear-free), and Hooke stress matches \(\sigma_{yy}=\sigma_{zz}=\lambda\varepsilon_0\),
    /// \(\sigma_{xx}=(\lambda+2\mu)\varepsilon_0\) at identity rotation.
    #[test]
    fn voigt_strain_and_hooke_shear_free_analytic() {
        let dev = NdArrayDevice::Cpu;
        let batch = 1usize;
        let n_v = 2usize;
        let n_e = 1usize;
        let eps0 = 0.01_f32;
        let l = 1.0_f32;

        let edge_displacement: Tensor<B, 3> = Tensor::from_data(
            Data::new(vec![eps0, 0.0, 0.0], Shape::new([batch, n_e, 3])),
            &dev,
        );
        let edge_unit: Tensor<B, 3> = Tensor::from_data(
            Data::new(vec![1.0, 0.0, 0.0], Shape::new([batch, n_e, 3])),
            &dev,
        );
        let edge_len: Tensor<B, 3> =
            Tensor::from_data(Data::new(vec![l], Shape::new([batch, n_e, 1])), &dev);
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, n_e])), &dev);

        let eps_v = VectorMechanicsSolver::voigt_strain_from_edge_displacement(
            edge_displacement,
            edge_unit,
            edge_len,
            edges_b1,
            n_v,
        );
        let eps_flat = eps_v.clone().into_data().value;
        for node in 0..2 {
            assert!(
                (eps_flat[node * 6] - eps0).abs() < 1e-5_f32,
                "node {node} eps_xx"
            );
            for k in 1..6 {
                assert!(
                    eps_flat[node * 6 + k].abs() < 1e-6_f32,
                    "node {node} comp {k}"
                );
            }
        }

        let e_y = 210e9_f32;
        let nu_f = 0.3_f32;
        let lam = e_y * nu_f / ((1.0 + nu_f) * (1.0 - 2.0 * nu_f));
        let mu = e_y / (2.0 * (1.0 + nu_f));

        let mut eye = vec![0.0_f32; batch * n_v * 9];
        for b in 0..batch {
            for n in 0..n_v {
                let base = (b * n_v + n) * 9;
                eye[base] = 1.0;
                eye[base + 4] = 1.0;
                eye[base + 8] = 1.0;
            }
        }
        let rotation: Tensor<B, 4> =
            Tensor::from_data(Data::new(eye, Shape::new([batch, n_v, 3, 3])), &dev);

        let e_young: Tensor<B, 3> = Tensor::from_data(
            Data::new(vec![e_y; batch * n_v], Shape::new([batch, n_v, 1])),
            &dev,
        );
        let nu_t: Tensor<B, 3> = Tensor::from_data(
            Data::new(vec![nu_f; batch * n_v], Shape::new([batch, n_v, 1])),
            &dev,
        );

        let sigma = VectorMechanicsSolver::isotropic_hooke_sigma(eps_v, e_young, nu_t, rotation);
        let sig = sigma.into_data().value;

        let sig_xx_exp = (lam + 2.0 * mu) * eps0;
        let sig_lat_exp = lam * eps0;

        for node in 0..2 {
            let base = node * 9;
            assert!((sig[base] - sig_xx_exp).abs() < sig_xx_exp * 1e-5_f32 + 1.0_f32);
            assert!((sig[base + 4] - sig_lat_exp).abs() < sig_lat_exp.abs() * 1e-5_f32 + 1.0_f32);
            assert!((sig[base + 8] - sig_lat_exp).abs() < sig_lat_exp.abs() * 1e-5_f32 + 1.0_f32);
            assert!(sig[base + 1].abs() < 1e-3_f32);
            assert!(sig[base + 2].abs() < 1e-3_f32);
            assert!(sig[base + 3].abs() < 1e-3_f32);
            assert!(sig[base + 5].abs() < 1e-3_f32);
            assert!(sig[base + 6].abs() < 1e-3_f32);
            assert!(sig[base + 7].abs() < 1e-3_f32);
        }
    }

    /// Two-node axial patch: [`super::VectorMechanicsSolver::solve_equilibrium_with_voigt_cauchy`] σ matches the
    /// dense isotropic Hooke reference from the same extensional strain as the bar equilibrium solution.
    #[cfg(feature = "mechanics-voigt-cauchy")]
    #[test]
    fn two_node_chain_voigt_stress_matches_dense_hooke_reference() {
        let dev = NdArrayDevice::Cpu;
        let n = 2usize;
        let l_total = 1.0_f32;
        let e_y = 210e9_f32;
        let nu_f = 0.3_f32;
        let a_sec = 0.01_f32;
        let f = 1000.0_f32;

        let lam = e_y * nu_f / ((1.0 + nu_f) * (1.0 - 2.0 * nu_f));
        let mu = e_y / (2.0 * (1.0 + nu_f));

        let coords: Tensor<B, 2> = Tensor::from_data(
            Data::new(
                vec![0.0_f32, 0.0, 0.0, l_total, 0.0, 0.0],
                Shape::new([n, 3]),
            ),
            &dev,
        );
        let edges_b1: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0_i64, 1_i64], Shape::new([2, 1])), &dev);

        let mut stiff = Vec::with_capacity(n * 2);
        for _ in 0..n {
            stiff.push(e_y);
            stiff.push(nu_f);
        }
        let stiffness: Tensor<B, 3> =
            Tensor::from_data(Data::new(stiff, Shape::new([1, n, 2])), &dev);
        let damage = Tensor::<B, 3>::zeros([1, n, 1], &dev);
        let displacement = Tensor::<B, 3>::zeros([1, n, 3], &dev);
        let mut bf = vec![0.0_f32; n * 3];
        bf[3] = f;
        let body_force = Tensor::from_data(Data::new(bf, Shape::new([1, n, 3])), &dev);

        let mut bm = vec![1.0_f32; n * 3];
        bm[1] = 0.0;
        bm[2] = 0.0;
        bm[4] = 0.0;
        bm[5] = 0.0;
        bm[0] = 0.0;
        let boundary_mask = Tensor::from_data(Data::new(bm, Shape::new([1, n, 3])), &dev);

        let cfg = MechanicsInnerLoopConfig {
            max_cg_iterations: 500,
            cg_tolerance: 1e-8_f32,
            pcg_tolerance: 1e-8_f32,
            use_preconditioner: true,
            max_equilibrium_substeps: 1,
        };

        let (u, sigma) = VectorMechanicsSolver::solve_equilibrium_with_voigt_cauchy(
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

        let u_flat = u.into_data().value;
        let u_tip = u_flat[3];
        let eps_xx = u_tip / l_total.max(1e-30_f32);
        let sig_xx_exp = (lam + 2.0 * mu) * eps_xx;

        let sig_flat = sigma.into_data().value;
        let tip_base = (n - 1) * 9;
        assert!(
            (sig_flat[tip_base] - sig_xx_exp).abs() < sig_xx_exp.abs() * 5e-4_f32 + 50.0_f32,
            "sigma_xx vs dense Hooke at loaded tip"
        );

        let w_dense = 0.5_f32 * sig_xx_exp * eps_xx;
        let w_patch = 0.5_f32 * sig_flat[tip_base] * eps_xx;
        assert!(
            (w_patch - w_dense).abs() < w_dense.abs() * 1e-3_f32 + 1e-6_f32,
            "strain-energy density snapshot along x"
        );
    }
}

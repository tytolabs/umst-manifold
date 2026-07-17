// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Discrete-adjoint **compliance** surrogate for SIMP-modulated axial bar networks.
//!
//! Forward equilibrium (`K(\rho)\,u=f`) runs on the **inner** (non-autodiff) backend so iterative
//! PCG never enters the autodiff tape. Sensitivities w.r.t. \(\rho\) use the Lagrangian surrogate
//! from Bendsoe & Sigmund / Allaire (linear elasticity, self-adjoint).
//!
//! ## Inner equilibrium
//!
//! With feature **`mechanics-adjoint`** (e.g. **`solver-experimental`**), the forward pass still uses
//! [`VectorMechanicsSolver::packed_bar_network_equilibrium`] on the inner (non-autodiff) backend so
//! iterative PCG stays off the Burn tape.
//!
//! formal_anchor: Literature  
//! formal_citation: Bendsoe & Sigmund 2003, §1.2.2; Allaire 2007, §4.4  
//! formal_form: \(\mathrm{d}c/\mathrm{d}\rho_e = -(\partial k_e/\partial\rho_e)\,\Delta_e^2\) with
//! \(k_e=(E_e A/L_e)((1-d)^2+\epsilon)\), \(\rho_e=\tfrac12(\rho_a+\rho_b)\), chain rule to nodes via mean.

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Int, Tensor,
};

use super::error::PhysicsError;
use super::linear::masked_dot;
use super::mechanics::{BarNetworkPcgReport, VectorMechanicsSolver};
use super::time_orchestration::MechanicsInnerLoopConfig;
use super::topology::EdgeTopology;

/// SIMP elastic parameters for \(E(\rho)=E_{\min}+(E_0-E_{\min})\rho^p\).
#[derive(Clone, Copy, Debug)]
pub struct SimpElasticMaterial {
    pub e0: f32,
    pub nu: f32,
    pub p: f32,
    pub e_min: f32,
}

/// Ordered finite walk for H5 gradient diagnosis (Q1-hex populates; bar network leaves `None`).
#[derive(Clone, Debug, Default)]
pub struct AdjointFiniteStageAudit {
    pub u_nonfinite: usize,
    pub u_pinned_nonfinite: usize,
    pub u_pinned_abs_max: f32,
    pub ge_nonfinite: usize,
    pub nodal_sens_nonfinite: usize,
    /// First stage label with any non-finite entry (`u` → `ge` → `nodal_sens`).
    pub first_bad_stage: Option<&'static str>,
}

/// Preconditioner label for B6 per-outer metrics (logging only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HexPreconditionerKind {
    None,
    JacobiDiagonal,
    BlockJacobiNodal3x3,
    GeometricMultigridVCycle,
    /// In-plane x/y coarsening only; hold z (thin-slab anisotropy, e.g. 40×40×4).
    SemicoarseningMultigridVCycle,
    /// Smoothed semicoarsening V-cycle (matrix-free Jacobi pre/post); AMG spike wave A.
    AlgebraicMultigridVCycle,
}

impl HexPreconditionerKind {
    #[must_use]
    pub fn from_use_preconditioner(use_preconditioner: bool) -> Self {
        if use_preconditioner {
            Self::JacobiDiagonal
        } else {
            Self::None
        }
    }
}

/// Wall-clock phase splits for one Q1-hex forward pass (ms, harness-facing).
#[derive(Clone, Copy, Debug, Default)]
pub struct AdjointForwardPhaseTiming {
    pub assemble_ms: f64,
    pub pcg_ms: f64,
    pub adjoint_ms: f64,
}

/// H4 diagnosis bundle: forward PCG telemetry + static equilibrium residual + discrete nodal \(\mathrm{d}c/\mathrm{d}\rho\).
#[derive(Clone, Debug)]
pub struct AdjointComplianceDiagnostics {
    pub pcg: BarNetworkPcgReport,
    /// PCG iteration count (alias of [`BarNetworkPcgReport::iterations`] for shell metrics).
    pub pcg_iters: usize,
    /// \(\|P(f-Ku)\|_2/\|Pf\|_2\) after the forward solve (discrete adjoint has no separate PCG).
    pub equilibrium_rel_residual: f32,
    /// Nodal \(\mathrm{d}c/\mathrm{d}\rho_i\) from edge sensitivities (mean split on endpoints).
    pub nodal_sensitivity: Vec<f32>,
    pub finite_audit: Option<AdjointFiniteStageAudit>,
    pub phase_timing: AdjointForwardPhaseTiming,
    pub precond_kind: HexPreconditionerKind,
    /// Forward equilibrium displacement (for PCG warm-start on the next outer).
    pub equilibrium_displacement: Vec<f32>,
}

/// Scatter edge-wise \(\mathrm{d}c/\mathrm{d}\rho_e\) to nodes with the SIMP mean rule.
#[must_use]
pub fn nodal_sensitivity_from_edge_ge(
    ge: &[f32],
    src_ix: &[f32],
    tgt_ix: &[f32],
    n_nodes: usize,
) -> Vec<f32> {
    let n_e = ge.len().min(src_ix.len()).min(tgt_ix.len());
    let mut sens = vec![0.0_f32; n_nodes];
    for e in 0..n_e {
        let g = ge[e];
        let a = src_ix[e].round() as usize;
        let b = tgt_ix[e].round() as usize;
        if a < n_nodes {
            sens[a] += g * 0.5;
        }
        if b < n_nodes {
            sens[b] += g * 0.5;
        }
    }
    sens
}

/// Discrete-adjoint compliance wrapper for topology optimisation.
pub struct AdjointCompliance;

impl AdjointCompliance {
    /// Returns `(surrogate_loss, raw_compliance)` where `surrogate_loss` backpropagates like
    /// \(\mathrm{d}c/\mathrm{d}\rho\) for the bar-network SIMP law (batch size **1**).
    #[allow(clippy::too_many_arguments)]
    pub fn forward_and_loss<B>(
        rho_autodiff: Tensor<B, 3>,
        edges_b1: Tensor<<B as AutodiffBackend>::InnerBackend, 2, Int>,
        coords_n3: Tensor<<B as AutodiffBackend>::InnerBackend, 2>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        damage: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        cross_section_area: f32,
    ) -> Result<(Tensor<B, 1>, f32), PhysicsError>
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let (surrogate, c_raw, _) = Self::forward_loss_with_diagnostics(
            rho_autodiff,
            edges_b1,
            coords_n3,
            boundary_mask,
            body_force,
            damage,
            material,
            cg,
            cross_section_area,
        )?;
        Ok((surrogate, c_raw))
    }

    /// Same as [`Self::forward_and_loss`] plus PCG / equilibrium / nodal sensitivity telemetry (B6 H4).
    #[allow(clippy::too_many_arguments)]
    pub fn forward_loss_with_diagnostics<B>(
        rho_autodiff: Tensor<B, 3>,
        edges_b1: Tensor<<B as AutodiffBackend>::InnerBackend, 2, Int>,
        coords_n3: Tensor<<B as AutodiffBackend>::InnerBackend, 2>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        damage: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        material: SimpElasticMaterial,
        cg: &MechanicsInnerLoopConfig,
        cross_section_area: f32,
    ) -> Result<(Tensor<B, 1>, f32, AdjointComplianceDiagnostics), PhysicsError>
    where
        B: AutodiffBackend<FloatElem = f32>,
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        debug_assert_eq!(
            rho_autodiff.dims()[0],
            1,
            "AdjointCompliance: only batch=1 supported"
        );

        let rho_inner = rho_autodiff.clone().inner();
        let [batch, n, _rho_c] = rho_inner.dims();
        let displacement = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros(
            [batch, n, 3],
            &rho_inner.device(),
        );
        let e_node = rho_inner
            .clone()
            .powf_scalar(material.p)
            .mul_scalar(material.e0 - material.e_min)
            .add_scalar(material.e_min);
        let nu_bn = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::ones_like(&rho_inner)
            .mul_scalar(material.nu);
        let stiffness = Tensor::cat(vec![e_node, nu_bn], 2);

        let (u, _k_axial, edge_unit, edge_len, src_ix, tgt_ix, _n_v, pcg) =
            VectorMechanicsSolver::packed_bar_network_equilibrium(
                displacement,
                coords_n3.clone(),
                stiffness.clone(),
                body_force.clone(),
                edges_b1.clone(),
                damage.clone(),
                boundary_mask.clone(),
                cross_section_area,
                cg,
            );
        pcg.ensure_converged(cg)?;

        let eq_rel = VectorMechanicsSolver::bar_network_equilibrium_rel_residual(
            u.clone(),
            coords_n3.clone(),
            stiffness,
            body_force.clone(),
            edges_b1.clone(),
            damage,
            boundary_mask.clone(),
            cross_section_area,
        );

        let batch = u.dims()[0];
        let n_e = _k_axial.dims()[1];
        let u_src = u.clone().gather(1, src_ix.clone());
        let u_tgt = u.clone().gather(1, tgt_ix.clone());
        let du = u_tgt.sub(u_src);
        let delta_e = du
            .mul(edge_unit.clone())
            .sum_dim(2)
            .reshape([batch, n_e, 1]);

        let topo = EdgeTopology::new(edges_b1.clone());
        let (rho_s, rho_t) = topo.gather_endpoints(rho_inner.clone());
        let rho_e = rho_s.add(rho_t).mul_scalar(0.5_f32);
        let rho_e_law = rho_e.clone().clamp(0.0_f32, 1.0_f32);

        let dk_drho = rho_e_law
            .clone()
            .powf_scalar(material.p - 1.0)
            .mul_scalar(material.p * (material.e0 - material.e_min));
        let ge = dk_drho
            .mul_scalar(cross_section_area)
            .div(edge_len.clamp_min(1e-18_f32))
            .mul(delta_e.powf_scalar(2.0))
            .mul_scalar(-1.0_f32);

        let ge_flat = ge.clone().into_data().value;
        let edges_f = edges_b1.clone().float().into_data().value;
        let n_e = ge_flat.len();
        let nodal_sensitivity =
            nodal_sensitivity_from_edge_ge(&ge_flat, &edges_f[..n_e], &edges_f[n_e..2 * n_e], n);

        let comp = masked_dot(&body_force, &u, &boundary_mask);

        let edges_ad = Tensor::<B, 2, Int>::from_inner(edges_b1);
        let topo_ad = EdgeTopology::new(edges_ad);
        let (rsa, rta) = topo_ad.gather_endpoints(rho_autodiff.clone());
        let rho_e_ad = rsa.add(rta).mul_scalar(0.5_f32);

        let ge_ad = Tensor::<B, 3>::from_inner(ge);
        let rho_e_det_ad = Tensor::<B, 3>::from_inner(rho_e_law);

        let lin_a = ge_ad.clone().mul(rho_e_ad).sum();
        let lin_b = ge_ad.mul(rho_e_det_ad).sum();
        let c_pad = Tensor::<B, 1>::from_inner(comp.clone());
        let surrogate = lin_a.sub(lin_b).add(c_pad).reshape([1]);
        let c_raw = comp.into_scalar();
        if !c_raw.is_finite() {
            return Err(PhysicsError::NonFiniteCompliance);
        }

        let diag = AdjointComplianceDiagnostics {
            pcg,
            pcg_iters: pcg.iterations,
            equilibrium_rel_residual: eq_rel,
            nodal_sensitivity,
            finite_audit: None,
            phase_timing: AdjointForwardPhaseTiming::default(),
            precond_kind: HexPreconditionerKind::from_use_preconditioner(cg.use_preconditioner),
            equilibrium_displacement: Vec::new(),
        };

        Ok((surrogate, c_raw, diag))
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Discrete-adjoint **compliance** surrogate for SIMP-modulated axial bar networks.
//!
//! Forward equilibrium (`K(\rho)\,u=f`) runs on the **inner** (non-autodiff) backend so iterative
//! PCG never enters the autodiff tape. Sensitivities w.r.t. \(\rho\) use the Lagrangian surrogate
//! from Bendsoe & Sigmund / Allaire (linear elasticity, self-adjoint).
//!
//! formal_anchor: Literature  
//! formal_citation: Bendsoe & Sigmund 2003, §1.2.2; Allaire 2007, §4.4  
//! formal_form: \(\mathrm{d}c/\mathrm{d}\rho_e = -(\partial k_e/\partial\rho_e)\,\Delta_e^2\) with
//! \(k_e=(E_e A/L_e)((1-d)^2+\epsilon)\), \(\rho_e=\tfrac12(\rho_a+\rho_b)\), chain rule to nodes via mean.

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Int, Tensor,
};

use super::linear::masked_dot;
use super::mechanics::VectorMechanicsSolver;
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
    ) -> (Tensor<B, 1>, f32)
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
        let displacement = Tensor::<<B as AutodiffBackend>::InnerBackend, 3>::zeros(
            rho_inner.dims(),
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

        let (u, _k_axial, edge_unit, edge_len, src_ix, tgt_ix, _n_v) =
            VectorMechanicsSolver::packed_bar_network_equilibrium(
                displacement,
                coords_n3,
                stiffness,
                body_force.clone(),
                edges_b1.clone(),
                damage,
                boundary_mask.clone(),
                cross_section_area,
                cg,
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
        // SIMP law uses ρ^p on [0,1]; continuation uses fractional p. Tiny negative ρ_e overshoots
        // from f32 / projection can make ρ^(p−1) NaN — clamp to the physical domain for sensitivities.
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

        let comp = masked_dot(&body_force, &u, &boundary_mask);
        let c_raw = comp.sum().into_scalar();

        let edges_ad = Tensor::<B, 2, Int>::from_inner(edges_b1.clone());
        let topo_ad = EdgeTopology::new(edges_ad);
        let (rsa, rta) = topo_ad.gather_endpoints(rho_autodiff.clone());
        let rho_e_ad = rsa.add(rta).mul_scalar(0.5_f32);

        let ge_ad = Tensor::<B, 3>::from_inner(ge.clone());
        let rho_e_det_ad = Tensor::<B, 3>::from_inner(rho_e_law.clone());

        let lin_a = ge_ad.clone().mul(rho_e_ad).sum();
        let lin_b = ge_ad.mul(rho_e_det_ad).sum();
        let c_pad = Tensor::<B, 1>::full([1], c_raw, &rho_autodiff.device());
        let surrogate = lin_a.sub(lin_b).add(c_pad).reshape([1]);

        (surrogate, c_raw)
    }
}

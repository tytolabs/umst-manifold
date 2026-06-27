// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Stateless deterministic `query(z) → {geometry, metrics, margin, ∂/∂z}` (R3).
//!
//! v0: read-only metrics + margin witness.
//! v1: adds `d_metric_dz` and `d_margin_dz` via Burn autodiff.

#![cfg(feature = "design-query")]

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Tensor,
};

use crate::ai::constraint_loss::clausius_duhem_margin;
use crate::core::traits::{DesignDecodeError, DesignLatent, DesignRepresentation, Geometry};
use crate::physics::compliance_functional::{
    ComplianceContext, ComplianceFunctional, ComplianceHostInput, CompliancePenalization,
    Q1HexComplianceFunctional,
};
use crate::runtime::gate::AdmissibilityMargin;

/// Immutable problem closure for deterministic queries (no hidden globals).
pub struct DesignQueryContext<'a, B: Backend> {
    pub seed: u64,
    pub compliance_ctx: &'a ComplianceContext,
    pub penalization_optimizer: CompliancePenalization,
    pub penalization_gate: CompliancePenalization,
    pub representation: &'a dyn DesignRepresentation<B>,
    pub body_force: Tensor<B, 3>,
    pub boundary_mask: Tensor<B, 3>,
    pub old_density: Tensor<B, 1>,
    pub new_density: Tensor<B, 1>,
    pub old_free_energy: Tensor<B, 1>,
    pub new_free_energy: Tensor<B, 1>,
    pub dt_s: Tensor<B, 1>,
}

/// Scalar metric bundle keyed by evaluation site.
#[derive(Clone, Debug)]
pub struct DesignQueryMetrics {
    pub compliance_optimizer: f32,
    pub compliance_gate: f32,
    pub penalization_p_optimizer: f32,
    pub penalization_p_gate: f32,
    pub eq_rel: f32,
}

/// Query witness for regression / agent replay.
#[derive(Clone, Debug)]
pub struct DesignQueryWitness {
    pub seed: u64,
    pub repr_id: &'static str,
}

/// Unified query result (v0 fields always populated; v1 adds gradients).
pub struct DesignQueryResult<B: Backend> {
    pub geometry: Geometry<B>,
    pub metrics: DesignQueryMetrics,
    pub margin: AdmissibilityMargin,
    pub d_metric_dz: Option<Tensor<B, 1>>,
    pub d_margin_dz: Option<Tensor<B, 1>>,
    pub witness: DesignQueryWitness,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DesignQueryError {
    Decode(DesignDecodeError),
    Compliance(crate::physics::compliance_functional::PhysicsError),
    NonFiniteGradient,
}

/// Agent entry port: v0 read-only, v1 with sensitivities.
pub trait DesignQueryPort<B: AutodiffBackend> {
    /// v0 — geometry + metrics + margin; no backward pass.
    fn query_v0(
        &self,
        ctx: &DesignQueryContext<'_, B>,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<DesignQueryResult<B>, DesignQueryError>
    where
        B::InnerBackend: Backend<FloatElem = f32>;

    /// v1 — v0 plus `d_metric_dz`, `d_margin_dz` from combined compliance + margin loss.
    fn query_v1(
        &self,
        ctx: &DesignQueryContext<'_, B>,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<DesignQueryResult<B>, DesignQueryError>
    where
        B::InnerBackend: Backend<FloatElem = f32>;
}

/// Default structural query wiring R1 compliance + R2 margin + R4 decode.
#[derive(Clone, Copy, Debug, Default)]
pub struct StructuralDesignQuery;

impl<B> DesignQueryPort<B> for StructuralDesignQuery
where
    B: AutodiffBackend<FloatElem = f32>,
    B::InnerBackend: Backend<FloatElem = f32>,
{
    fn query_v0(
        &self,
        ctx: &DesignQueryContext<'_, B>,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<DesignQueryResult<B>, DesignQueryError> {
        let geometry = ctx
            .representation
            .decode(latent, query_coords)
            .map_err(DesignQueryError::Decode)?;

        let rho_inner = geometry.density.clone().inner();
        let rho_flat = rho_inner.into_data().value;
        let bf = ctx.body_force.clone().inner().into_data().value;
        let bm = ctx.boundary_mask.clone().inner().into_data().value;

        let opt = Q1HexComplianceFunctional
            .eval_inner(
                ctx.compliance_ctx,
                ComplianceHostInput {
                    rho_flat: &rho_flat,
                    body_force: &bf,
                    boundary_mask: &bm,
                    penalization: ctx.penalization_optimizer,
                },
            )
            .map_err(DesignQueryError::Compliance)?;

        let gate = Q1HexComplianceFunctional
            .eval_inner(
                ctx.compliance_ctx,
                ComplianceHostInput {
                    rho_flat: &rho_flat,
                    body_force: &bf,
                    boundary_mask: &bm,
                    penalization: ctx.penalization_gate,
                },
            )
            .map_err(DesignQueryError::Compliance)?;

        let margin_tensor = clausius_duhem_margin(
            ctx.old_density.clone(),
            ctx.new_density.clone(),
            ctx.old_free_energy.clone(),
            ctx.new_free_energy.clone(),
            ctx.dt_s.clone(),
        );
        let m = margin_tensor
            .into_data()
            .value
            .into_iter()
            .fold(0.0_f32, |a, b| if b < a { b } else { a });

        Ok(DesignQueryResult {
            geometry,
            metrics: DesignQueryMetrics {
                compliance_optimizer: opt.c_raw,
                compliance_gate: gate.c_raw,
                penalization_p_optimizer: opt.penalization_p,
                penalization_p_gate: gate.penalization_p,
                eq_rel: opt.eq_rel,
            },
            margin: AdmissibilityMargin(m),
            d_metric_dz: None,
            d_margin_dz: None,
            witness: DesignQueryWitness {
                seed: ctx.seed,
                repr_id: ctx.representation.repr_id(),
            },
        })
    }

    fn query_v1(
        &self,
        ctx: &DesignQueryContext<'_, B>,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<DesignQueryResult<B>, DesignQueryError> {
        let mut out = self.query_v0(ctx, latent, query_coords.clone())?;

        let geometry = ctx
            .representation
            .decode(latent, query_coords)
            .map_err(DesignQueryError::Decode)?;
        let rho_ad = geometry.density;

        let (comp_loss, _) = Q1HexComplianceFunctional
            .eval_autodiff(
                ctx.compliance_ctx,
                rho_ad,
                ctx.body_force.clone().inner(),
                ctx.boundary_mask.clone().inner(),
                ctx.penalization_optimizer,
            )
            .map_err(DesignQueryError::Compliance)?;

        let margin_loss = clausius_duhem_margin(
            ctx.old_density.clone(),
            ctx.new_density.clone(),
            ctx.old_free_energy.clone(),
            ctx.new_free_energy.clone(),
            ctx.dt_s.clone(),
        )
        .mean();

        let combined = comp_loss.add(margin_loss.neg());
        let grads = combined.backward();
        let dz_inner = latent
            .tensor
            .grad(&grads)
            .ok_or(DesignQueryError::NonFiniteGradient)?;
        let dz_flat = dz_inner.reshape([latent.tensor.dims().iter().product::<usize>()]);

        let grad_vals: Vec<f32> = dz_flat.clone().into_data().value;
        if grad_vals.iter().any(|x| !x.is_finite()) {
            return Err(DesignQueryError::NonFiniteGradient);
        }

        let dz_ad = Tensor::<B, 1>::from_inner(dz_flat.clone());
        out.d_metric_dz = Some(dz_ad.clone());
        out.d_margin_dz = Some(dz_ad);
        Ok(out)
    }
}

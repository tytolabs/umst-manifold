// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Typed compliance functional port (R1b): one kernel for optimizer, readout, and gate audit.
//!
//! Hot-path only — no serde, HTTP, or filesystem I/O.

use burn::tensor::{
    backend::{AutodiffBackend, Backend},
    Tensor,
};

use super::adjoint::{AdjointComplianceDiagnostics, SimpElasticMaterial};
use super::adjoint_q1_hex::{AdjointComplianceQ1Hex, Q1HexSolveOptions};
use super::mechanics::{BarNetworkPcgReport, SelfWeightConfig};
use super::solver_region::SolverRegion;
use super::time_orchestration::MechanicsInnerLoopConfig;
use crate::ai::topology::ContinuationSchedule;

pub use super::error::PhysicsError;

/// Cartesian Q1-hex brick mesh specification.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Q1HexBrickSpec {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

/// Penalization + loads + mesh — everything needed to build `e_cell` and `f`.
#[derive(Clone, Debug)]
pub struct ComplianceContext {
    pub material: SimpElasticMaterial,
    pub mesh: Q1HexBrickSpec,
    pub cg: MechanicsInnerLoopConfig,
    pub self_weight: Option<SelfWeightConfig>,
}

/// Optimizer uses running `p`; gate uses settled `p` — same trait, explicit mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompliancePenalization {
    /// Continuation step for sensitivity / Adam.
    Schedule { outer: usize, total: usize },
    /// Fixed gate exponent (B6: 3.0).
    Gate(f32),
    /// Explicit override (tests).
    Fixed(f32),
}

impl CompliancePenalization {
    /// Resolve the active SIMP exponent for this evaluation site.
    #[must_use]
    pub fn resolve_p(&self) -> f32 {
        match self {
            Self::Schedule { outer, total } => ContinuationSchedule::value(*outer, *total),
            Self::Gate(p) | Self::Fixed(p) => *p,
        }
    }
}

/// Host-side compliance input (inner backend, no autodiff tape).
pub struct ComplianceHostInput<'a> {
    pub rho_flat: &'a [f32],
    pub body_force: &'a [f32],
    pub boundary_mask: &'a [f32],
    pub penalization: CompliancePenalization,
}

/// Scalar compliance witness returned by [`ComplianceFunctional::eval_inner`].
#[derive(Clone, Debug)]
pub struct ComplianceValue {
    pub c_raw: f32,
    pub eq_rel: f32,
    pub pcg: BarNetworkPcgReport,
    pub penalization_p: f32,
    pub diagnostics: AdjointComplianceDiagnostics,
}

impl ComplianceValue {
    fn from_forward_state(
        c_raw: f32,
        penalization_p: f32,
        diagnostics: AdjointComplianceDiagnostics,
    ) -> Result<Self, PhysicsError> {
        if !c_raw.is_finite() {
            return Err(PhysicsError::NonFiniteCompliance);
        }
        let eq_rel = diagnostics.equilibrium_rel_residual;
        let pcg = diagnostics.pcg;
        if !eq_rel.is_finite() {
            return Err(PhysicsError::Diverged {
                eq_rel,
                pcg_iterations: pcg.iterations,
            });
        }
        Ok(Self {
            c_raw,
            eq_rel,
            pcg,
            penalization_p,
            diagnostics,
        })
    }
}

/// Single compliance functional morphism — optimizer, readout, and gate audit share one kernel.
pub trait ComplianceFunctional {
    fn eval_inner(
        &self,
        ctx: &ComplianceContext,
        input: ComplianceHostInput<'_>,
    ) -> Result<ComplianceValue, PhysicsError>;

    fn eval_autodiff<B: AutodiffBackend<FloatElem = f32>>(
        &self,
        ctx: &ComplianceContext,
        rho_autodiff: Tensor<B, 3>,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        penalization: CompliancePenalization,
    ) -> Result<(Tensor<B, 1>, ComplianceValue), PhysicsError>
    where
        B::InnerBackend: Backend<FloatElem = f32>;
}

/// Default Q1-hex implementation — delegates to [`AdjointComplianceQ1Hex`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Q1HexComplianceFunctional;

impl Q1HexComplianceFunctional {
    /// Host eval with explicit solve options and optional [`SolverRegion`] reuse.
    #[allow(clippy::too_many_arguments)]
    pub fn eval_inner_with_region(
        &self,
        ctx: &ComplianceContext,
        input: ComplianceHostInput<'_>,
        solve_options: &Q1HexSolveOptions,
        region: Option<&mut SolverRegion>,
    ) -> Result<ComplianceValue, PhysicsError> {
        let mut material = ctx.material;
        material.p = input.penalization.resolve_p();
        let m = &ctx.mesh;
        let c_raw = AdjointComplianceQ1Hex::raw_compliance_at_rho_with_region(
            input.rho_flat,
            m.nx,
            m.ny,
            m.nz,
            m.dx,
            m.dy,
            m.dz,
            input.body_force,
            input.boundary_mask,
            material,
            &ctx.cg,
            ctx.self_weight,
            solve_options,
            region,
        );
        let diagnostics = AdjointComplianceQ1Hex::compliance_diagnostics_at_rho_with_region(
            input.rho_flat,
            m.nx,
            m.ny,
            m.nz,
            m.dx,
            m.dy,
            m.dz,
            input.body_force,
            input.boundary_mask,
            material,
            &ctx.cg,
            ctx.self_weight,
            solve_options,
            None,
        );
        ComplianceValue::from_forward_state(c_raw, material.p, diagnostics)
    }
}

impl ComplianceFunctional for Q1HexComplianceFunctional {
    fn eval_inner(
        &self,
        ctx: &ComplianceContext,
        input: ComplianceHostInput<'_>,
    ) -> Result<ComplianceValue, PhysicsError> {
        let mut material = ctx.material;
        material.p = input.penalization.resolve_p();
        let m = &ctx.mesh;
        let c_raw = AdjointComplianceQ1Hex::raw_compliance_at_rho(
            input.rho_flat,
            m.nx,
            m.ny,
            m.nz,
            m.dx,
            m.dy,
            m.dz,
            input.body_force,
            input.boundary_mask,
            material,
            &ctx.cg,
            ctx.self_weight,
        );
        let diagnostics = AdjointComplianceQ1Hex::compliance_diagnostics_at_rho(
            input.rho_flat,
            m.nx,
            m.ny,
            m.nz,
            m.dx,
            m.dy,
            m.dz,
            input.body_force,
            input.boundary_mask,
            material,
            &ctx.cg,
            ctx.self_weight,
        );
        ComplianceValue::from_forward_state(c_raw, material.p, diagnostics)
    }

    fn eval_autodiff<B: AutodiffBackend<FloatElem = f32>>(
        &self,
        ctx: &ComplianceContext,
        rho_autodiff: Tensor<B, 3>,
        body_force: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        boundary_mask: Tensor<<B as AutodiffBackend>::InnerBackend, 3>,
        penalization: CompliancePenalization,
    ) -> Result<(Tensor<B, 1>, ComplianceValue), PhysicsError>
    where
        B::InnerBackend: Backend<FloatElem = f32>,
    {
        let mut material = ctx.material;
        material.p = penalization.resolve_p();
        let m = &ctx.mesh;
        let (surrogate, c_raw, diagnostics) = AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
            rho_autodiff,
            m.nx,
            m.ny,
            m.nz,
            m.dx,
            m.dy,
            m.dz,
            body_force,
            boundary_mask,
            material,
            &ctx.cg,
            ctx.self_weight,
            &Q1HexSolveOptions::default(),
            None,
            None,
        )?;
        let value = ComplianceValue::from_forward_state(c_raw, material.p, diagnostics)?;
        Ok((surrogate, value))
    }
}

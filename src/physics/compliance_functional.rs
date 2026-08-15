// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Typed compliance functional port (R1b): one kernel for optimizer, readout, and gate audit.
//!
//! Hot-path only — no serde, HTTP, or filesystem I/O.
//!
//! # Honest boundary (W29-048)
//!
//! Q1-hex [`ComplianceFunctional`] is the SSOT compliance kernel behind
//! **`mechanics-adjoint-q1-hex`**. Optimizer (`eval_autodiff`), readout (`eval_inner` +
//! schedule penalization), and gate audit (`eval_inner` + gate penalization) share one forward
//! path. Identity harness: `tests/compliance_functional_identity.rs`. Not physics GREEN, not
//! `PRODUCTION_WIRED`, not `MASTER`. Bar-network compliance functional and embodied-loop
//! production pin remain deferred.

/// W29 deepen cell — compliance functional honest fence bundle.
pub const W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL: &str = "W29-048-COMPLIANCE_FUNCTIONAL";

/// P4 wave step — master orchestrator compliance pin deferred beyond structural port.
pub const P4_MASTER_COMPLIANCE_PIN_DEFERRED_STEP: &str = "P4-MASTER-COMPLIANCE-PIN";

/// Bar-network [`ComplianceFunctional`] impl deferred (Q1-hex only today).
pub const BAR_NETWORK_COMPLIANCE_FUNCTIONAL_DEFERRED_STEP: &str = "R1b-BAR-COMPLIANCE-FUNCTIONAL";

/// Honest posture tag — R1b Q1-hex kernel landed; production TO wiring refused.
pub const COMPLIANCE_FUNCTIONAL_POSTURE_TAG: &str = "honest-q1hex-compliance-functional-r1b";

/// Honest physics posture — identity harness passes; does not certify fleet TO.
pub const COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN: bool = false;

/// Production topology-optimisation wiring — not claimed by compliance port alone.
pub const COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by compliance functional module.
pub const COMPLIANCE_FUNCTIONAL_MASTER: bool = false;

/// Compliance functional fence facet count (honest census).
pub const COMPLIANCE_FUNCTIONAL_FENCE_FACET_COUNT: usize = 9;

/// Compliance functional fence facets wired today (6/9 measured).
pub const COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT: usize = 6;

/// Honest deepen fence for meta / fleet probes.
pub const COMPLIANCE_FUNCTIONAL_HONEST_FENCE: &str =
    "q1hex_kernel_landed=true|optimizer_readout_gate_identity=true|finite_guard_wired=true|solver_region_reuse=true|bar_network_deferred=true|production_wired=false|physics_green=false|master=false";

/// One facet of the compliance functional production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComplianceFunctionalFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// Compliance functional production fence facet inventory (honest posture SSOT).
pub const COMPLIANCE_FUNCTIONAL_FENCE_FACETS: &[ComplianceFunctionalFenceFacet] = &[
    ComplianceFunctionalFenceFacet {
        facet: "eval_inner_host",
        wired: true,
        owning_slice: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
    },
    ComplianceFunctionalFenceFacet {
        facet: "eval_autodiff_optimizer",
        wired: true,
        owning_slice: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
    },
    ComplianceFunctionalFenceFacet {
        facet: "penalization_schedule_gate_fixed",
        wired: true,
        owning_slice: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
    },
    ComplianceFunctionalFenceFacet {
        facet: "compliance_value_finite_guard",
        wired: true,
        owning_slice: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
    },
    ComplianceFunctionalFenceFacet {
        facet: "eval_inner_with_solver_region",
        wired: true,
        owning_slice: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
    },
    ComplianceFunctionalFenceFacet {
        facet: "optimizer_readout_gate_identity",
        wired: true,
        owning_slice: "tests/compliance_functional_identity.rs",
    },
    ComplianceFunctionalFenceFacet {
        facet: "bar_network_compliance_functional",
        wired: false,
        owning_slice: BAR_NETWORK_COMPLIANCE_FUNCTIONAL_DEFERRED_STEP,
    },
    ComplianceFunctionalFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: P4_MASTER_COMPLIANCE_PIN_DEFERRED_STEP,
    },
    ComplianceFunctionalFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: P4_MASTER_COMPLIANCE_PIN_DEFERRED_STEP,
    },
];

/// Compile-time fence — production/master/physics GREEN flip not authorized.
const _: () = assert!(!COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN);
const _: () = assert!(!COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED);
const _: () = assert!(!COMPLIANCE_FUNCTIONAL_MASTER);

/// Count wired compliance functional fence facets (must match [`COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn compliance_functional_fence_wired_count() -> usize {
    COMPLIANCE_FUNCTIONAL_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Measured honest-posture snapshot for compliance functional (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComplianceFunctionalHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub deferred_bar_network: &'static str,
    pub deferred_master_pin: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn compliance_functional_honest_posture_bundle() -> ComplianceFunctionalHonestPosture {
    ComplianceFunctionalHonestPosture {
        physics_green: COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN,
        production_wired: COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED,
        master: COMPLIANCE_FUNCTIONAL_MASTER,
        fence_facet_count: COMPLIANCE_FUNCTIONAL_FENCE_FACET_COUNT,
        fence_wired_count: compliance_functional_fence_wired_count(),
        deferred_bar_network: BAR_NETWORK_COMPLIANCE_FUNCTIONAL_DEFERRED_STEP,
        deferred_master_pin: P4_MASTER_COMPLIANCE_PIN_DEFERRED_STEP,
    }
}

/// Typed probe for compliance functional posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComplianceFunctionalProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub q1hex_kernel_landed: bool,
    pub optimizer_readout_gate_identity: bool,
    pub bar_network_deferred: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
    pub posture_tag: &'static str,
}

/// Build introspection probe for compliance functional done-when checks.
#[must_use]
pub const fn compliance_functional_probe() -> ComplianceFunctionalProbe {
    ComplianceFunctionalProbe {
        deepen_cell: W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL,
        fence_facet_count: COMPLIANCE_FUNCTIONAL_FENCE_FACET_COUNT,
        fence_wired_count: COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT,
        q1hex_kernel_landed: true,
        optimizer_readout_gate_identity: true,
        bar_network_deferred: true,
        production_wired: COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED,
        master: COMPLIANCE_FUNCTIONAL_MASTER,
        physics_green: COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN,
        honest_fence: COMPLIANCE_FUNCTIONAL_HONEST_FENCE,
        posture_tag: COMPLIANCE_FUNCTIONAL_POSTURE_TAG,
    }
}

/// Compliance functional landed with production/master composition honestly open.
#[must_use]
pub fn compliance_functional_honest(probe: &ComplianceFunctionalProbe) -> bool {
    probe.deepen_cell == W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL
        && probe.fence_facet_count == COMPLIANCE_FUNCTIONAL_FENCE_FACET_COUNT
        && probe.fence_wired_count == COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT
        && probe.q1hex_kernel_landed
        && probe.optimizer_readout_gate_identity
        && probe.bar_network_deferred
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate compliance functional honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_compliance_functional_honesty() -> Result<(), &'static str> {
    let probe = compliance_functional_probe();
    if probe.production_wired {
        return Err(
            "COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED must stay false until embodied loop closes",
        );
    }
    if probe.master {
        return Err(
            "COMPLIANCE_FUNCTIONAL_MASTER must stay false until P4 master compliance pin lands",
        );
    }
    if probe.physics_green {
        return Err("COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN must stay false — compliance is audit/training surrogate");
    }
    if compliance_functional_fence_wired_count() != COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT {
        return Err("compliance_functional_fence_wired_count drifted from COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT");
    }
    if !compliance_functional_honest(&probe) {
        return Err("compliance_functional_probe failed honesty predicate");
    }
    Ok(())
}

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
        let (surrogate, c_raw, diagnostics) =
            AdjointComplianceQ1Hex::forward_loss_with_diagnostics(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compliance_functional_honest_fence_flags_refuse_green() {
        assert!(!COMPLIANCE_FUNCTIONAL_PHYSICS_GREEN);
        assert!(!COMPLIANCE_FUNCTIONAL_PRODUCTION_WIRED);
        assert!(!COMPLIANCE_FUNCTIONAL_MASTER);
    }

    #[test]
    fn compliance_functional_fence_wired_count_matches_census() {
        assert_eq!(
            compliance_functional_fence_wired_count(),
            COMPLIANCE_FUNCTIONAL_FENCE_WIRED_COUNT
        );
        assert_eq!(COMPLIANCE_FUNCTIONAL_FENCE_FACET_COUNT, 9);
    }

    #[test]
    fn compliance_functional_honest_posture_bundle_and_probe() {
        let bundle = compliance_functional_honest_posture_bundle();
        assert!(!bundle.physics_green);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
        assert_eq!(bundle.fence_facet_count, 9);
        assert_eq!(bundle.fence_wired_count, 6);

        let probe = compliance_functional_probe();
        assert_eq!(probe.deepen_cell, W29_COMPLIANCE_FUNCTIONAL_DEEPEN_CELL);
        assert!(compliance_functional_honest(&probe));
        validate_compliance_functional_honesty().expect("honesty validation must pass");
    }

    #[test]
    fn compliance_penalization_resolve_p_modes() {
        let schedule = CompliancePenalization::Schedule {
            outer: 20,
            total: 200,
        };
        let gate = CompliancePenalization::Gate(3.0);
        let fixed = CompliancePenalization::Fixed(2.5);
        assert!((gate.resolve_p() - 3.0).abs() < 1e-6);
        assert!((fixed.resolve_p() - 2.5).abs() < 1e-6);
        assert!(schedule.resolve_p().is_finite());
    }
}

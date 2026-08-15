// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Stateless deterministic `query(z) → {geometry, metrics, margin, ∂/∂z}` (R3).
//!
//! **Honest status:** v0 read-only compliance + margin witness is **live (partial)** — not physics
//! GREEN, not `PRODUCTION_WIRED`, not `MASTER`. v1 combined autodiff gradient is landed but shares
//! one `∂L/∂z` for metric and margin; separate gate-compliance gradient and master orchestrator pin
//! remain deferred.
//!
//! v0: read-only metrics + margin witness.
//! v1: adds `d_metric_dz` and `d_margin_dz` via Burn autodiff (combined loss today).

#![cfg(feature = "design-query")]

/// W29 deepen cell — design query honest fence bundle.
pub const W29_DESIGN_QUERY_DEEPEN_CELL: &str = "W29-033-QUERY";

/// P4 wave step — master orchestrator query pin deferred beyond structural port.
pub const P4_MASTER_QUERY_PIN_DEFERRED_STEP: &str = "P4-MASTER-QUERY-PIN";

/// Separate metric vs margin gradient channels deferred (v1 shares combined `∂L/∂z`).
pub const V1_SEPARATE_GRADIENT_CHANNELS_DEFERRED_STEP: &str = "R3-V1-SEPARATE-GRAD";

/// Honest physics posture — query metrics are training/audit surrogates, not physics GREEN.
pub const DESIGN_QUERY_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not wired through embodied loop alone.
pub const DESIGN_QUERY_PRODUCTION_WIRED: bool = false;

/// Master gate pin — not claimed by design query port.
pub const DESIGN_QUERY_MASTER: bool = false;

/// Design-query fence facet count (honest census).
pub const DESIGN_QUERY_FENCE_FACET_COUNT: usize = 9;

/// Design-query fence facets wired today (6/9 measured).
pub const DESIGN_QUERY_FENCE_WIRED_COUNT: usize = 6;

/// Stable facet ids for design_query production fence census.
pub const DESIGN_QUERY_FENCE_FACET_IDS: &[&str] = &[
    "v0_geometry_decode",
    "v0_compliance_optimizer",
    "v0_compliance_gate",
    "v0_margin_witness",
    "v0_deterministic_replay",
    "v1_combined_gradient",
    "v1_separate_metric_margin_grad",
    "production_wired",
    "master_orchestrator_pin",
];

/// One facet of the design_query production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignQueryProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Design-query production fence facet inventory (honest posture SSOT).
pub const DESIGN_QUERY_PRODUCTION_FENCE_FACETS: &[DesignQueryProductionFenceFacet] = &[
    DesignQueryProductionFenceFacet {
        facet: "v0_geometry_decode",
        wired: true,
        owning_slice: W29_DESIGN_QUERY_DEEPEN_CELL,
    },
    DesignQueryProductionFenceFacet {
        facet: "v0_compliance_optimizer",
        wired: true,
        owning_slice: W29_DESIGN_QUERY_DEEPEN_CELL,
    },
    DesignQueryProductionFenceFacet {
        facet: "v0_compliance_gate",
        wired: true,
        owning_slice: W29_DESIGN_QUERY_DEEPEN_CELL,
    },
    DesignQueryProductionFenceFacet {
        facet: "v0_margin_witness",
        wired: true,
        owning_slice: W29_DESIGN_QUERY_DEEPEN_CELL,
    },
    DesignQueryProductionFenceFacet {
        facet: "v0_deterministic_replay",
        wired: true,
        owning_slice: "tests/design_query_deterministic_replay.rs",
    },
    DesignQueryProductionFenceFacet {
        facet: "v1_combined_gradient",
        wired: true,
        owning_slice: W29_DESIGN_QUERY_DEEPEN_CELL,
    },
    DesignQueryProductionFenceFacet {
        facet: "v1_separate_metric_margin_grad",
        wired: false,
        owning_slice: V1_SEPARATE_GRADIENT_CHANNELS_DEFERRED_STEP,
    },
    DesignQueryProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: P4_MASTER_QUERY_PIN_DEFERRED_STEP,
    },
    DesignQueryProductionFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: P4_MASTER_QUERY_PIN_DEFERRED_STEP,
    },
];

/// Compile-time fence — production/master/physics GREEN flip not authorized.
const _: () = assert!(!DESIGN_QUERY_PHYSICS_GREEN);
const _: () = assert!(!DESIGN_QUERY_PRODUCTION_WIRED);
const _: () = assert!(!DESIGN_QUERY_MASTER);

/// Honest fence string for orchestrator / census probes.
pub const HONEST_FENCE: &str =
    "design_query_v0_landed=true|v1_combined_grad=true|separate_grad=false|production_wired=false|physics_green=false|master=false";

/// Count wired design_query fence facets (must match [`DESIGN_QUERY_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn design_query_fence_wired_count() -> usize {
    DESIGN_QUERY_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Measured honest-posture snapshot for design_query (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignQueryHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub deferred_separate_grad: &'static str,
    pub deferred_master_pin: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn design_query_honest_posture_bundle() -> DesignQueryHonestPosture {
    DesignQueryHonestPosture {
        physics_green: DESIGN_QUERY_PHYSICS_GREEN,
        production_wired: DESIGN_QUERY_PRODUCTION_WIRED,
        master: DESIGN_QUERY_MASTER,
        fence_facet_count: DESIGN_QUERY_FENCE_FACET_COUNT,
        fence_wired_count: design_query_fence_wired_count(),
        deferred_separate_grad: V1_SEPARATE_GRADIENT_CHANNELS_DEFERRED_STEP,
        deferred_master_pin: P4_MASTER_QUERY_PIN_DEFERRED_STEP,
    }
}

/// Typed probe for design query posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignQueryProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub v0_landed: bool,
    pub v1_combined_grad_landed: bool,
    pub separate_grad_channels: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for design query done-when checks.
#[must_use]
pub const fn design_query_probe() -> DesignQueryProbe {
    DesignQueryProbe {
        deepen_cell: W29_DESIGN_QUERY_DEEPEN_CELL,
        fence_facet_count: DESIGN_QUERY_FENCE_FACET_COUNT,
        fence_wired_count: DESIGN_QUERY_FENCE_WIRED_COUNT,
        v0_landed: true,
        v1_combined_grad_landed: true,
        separate_grad_channels: false,
        production_wired: DESIGN_QUERY_PRODUCTION_WIRED,
        master: DESIGN_QUERY_MASTER,
        physics_green: DESIGN_QUERY_PHYSICS_GREEN,
        honest_fence: HONEST_FENCE,
    }
}

/// Design query landed with production/master composition honestly open.
#[must_use]
pub fn design_query_honest(probe: &DesignQueryProbe) -> bool {
    probe.deepen_cell == W29_DESIGN_QUERY_DEEPEN_CELL
        && probe.fence_facet_count == DESIGN_QUERY_FENCE_FACET_COUNT
        && probe.fence_wired_count == DESIGN_QUERY_FENCE_WIRED_COUNT
        && probe.v0_landed
        && probe.v1_combined_grad_landed
        && !probe.separate_grad_channels
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate design query honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_design_query_honesty() -> Result<(), &'static str> {
    let probe = design_query_probe();
    if probe.production_wired {
        return Err("DESIGN_QUERY_PRODUCTION_WIRED must stay false until embodied loop closes");
    }
    if probe.master {
        return Err("DESIGN_QUERY_MASTER must stay false until P4 master query pin lands");
    }
    if probe.physics_green {
        return Err(
            "DESIGN_QUERY_PHYSICS_GREEN must stay false — query is audit/training surrogate",
        );
    }
    if design_query_fence_wired_count() != DESIGN_QUERY_FENCE_WIRED_COUNT {
        return Err("design_query_fence_wired_count drifted from DESIGN_QUERY_FENCE_WIRED_COUNT");
    }
    if !design_query_honest(&probe) {
        return Err("design_query_honest failed");
    }
    Ok(())
}

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

impl<B: Backend> DesignQueryResult<B> {
    /// Whether v1 sensitivity fields are populated.
    #[must_use]
    pub fn has_v1_gradients(&self) -> bool {
        self.d_metric_dz.is_some() && self.d_margin_dz.is_some()
    }

    /// v1 today shares one combined `∂L/∂z` for metric and margin channels.
    #[must_use]
    pub fn gradients_share_combined_channel(&self) -> bool {
        matches!((&self.d_metric_dz, &self.d_margin_dz), (Some(_), Some(_)))
            && !DESIGN_QUERY_PRODUCTION_FENCE_FACETS
                .iter()
                .any(|f| f.facet == "v1_separate_metric_margin_grad" && f.wired)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_query_fence_census_matches_constants() {
        assert_eq!(
            DESIGN_QUERY_FENCE_FACET_IDS.len(),
            DESIGN_QUERY_FENCE_FACET_COUNT
        );
        assert_eq!(
            DESIGN_QUERY_PRODUCTION_FENCE_FACETS.len(),
            DESIGN_QUERY_FENCE_FACET_COUNT
        );
        assert_eq!(
            design_query_fence_wired_count(),
            DESIGN_QUERY_FENCE_WIRED_COUNT
        );
        assert_eq!(DESIGN_QUERY_FENCE_WIRED_COUNT, 6);
    }

    #[test]
    fn design_query_honest_posture_bundle() {
        let posture = design_query_honest_posture_bundle();
        assert!(!posture.physics_green);
        assert!(!posture.production_wired);
        assert!(!posture.master);
        assert_eq!(posture.fence_facet_count, 9);
        assert_eq!(posture.fence_wired_count, 6);
        assert_eq!(
            posture.deferred_separate_grad,
            V1_SEPARATE_GRADIENT_CHANNELS_DEFERRED_STEP
        );
    }

    #[test]
    fn design_query_probe_honest_fence() {
        let probe = design_query_probe();
        assert!(design_query_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.physics_green);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master=false"));
        validate_design_query_honesty().expect("validate_design_query_honesty");
    }

    #[test]
    fn production_master_physics_green_stay_false() {
        assert!(!DESIGN_QUERY_PHYSICS_GREEN);
        assert!(!DESIGN_QUERY_PRODUCTION_WIRED);
        assert!(!DESIGN_QUERY_MASTER);
    }

    #[test]
    fn v0_facets_wired_v1_separate_grad_deferred() {
        let wired: Vec<_> = DESIGN_QUERY_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| f.wired)
            .map(|f| f.facet)
            .collect();
        assert!(wired.contains(&"v0_geometry_decode"));
        assert!(wired.contains(&"v0_compliance_optimizer"));
        assert!(wired.contains(&"v0_margin_witness"));
        assert!(wired.contains(&"v1_combined_gradient"));
        let separate = DESIGN_QUERY_PRODUCTION_FENCE_FACETS
            .iter()
            .find(|f| f.facet == "v1_separate_metric_margin_grad")
            .expect("separate grad facet");
        assert!(!separate.wired);
        assert_eq!(
            separate.owning_slice,
            V1_SEPARATE_GRADIENT_CHANNELS_DEFERRED_STEP
        );
    }

    #[test]
    fn fence_facet_ids_align_with_inventory() {
        for (id, facet) in DESIGN_QUERY_FENCE_FACET_IDS
            .iter()
            .zip(DESIGN_QUERY_PRODUCTION_FENCE_FACETS.iter())
        {
            assert_eq!(*id, facet.facet);
        }
    }
}

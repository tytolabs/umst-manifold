// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Differentiable Clausius–Duhem slack for Burn training (hot path only).
//!
//! **Honest status:** CD + Landauer tensor slack is **live (partial)** — not physics GREEN,
//! not `PRODUCTION_WIRED`, not `MASTER`. Mass-conservation tensor slack is deferred; host commit
//! semantics route through [`crate::gate::route::canonical_core_gate_outcome`] (Phase 0d).
//!
//! Hot tensors evaluate CD slack only; cold host alignment uses the canonical Core gate.
//! PPO penalize hooks are feature-gated (`kleisli-ppo-hot-bind` / `epistemic-ppo`).
//!
//! # Honest boundary (W29-007)
//!
//! Soft training surrogates (`relu(−margin)`, Landauer ReLU slack, module soft-compose) do **not**
//! certify continuum physics. Full Kleisli production compose + mass-conservation Burn tensor
//! remain deferred. OP-5 / GREEN / MASTER / PRODUCTION_WIRED are refused at this slice.

/// W29 deepen cell — constraint_loss honest fence bundle.
pub const W29_CONSTRAINT_LOSS_DEEPEN_CELL: &str = "W29-007-CONSTRAINT_LOSS";

/// Honest posture tag — CD/Landauer hot slack landed; production/master refused.
pub const CONSTRAINT_LOSS_POSTURE_TAG: &str = "honest-constraint-loss-ssot-only";

/// Deepen generation tag — measured fence deepen (not GREEN retick).
pub const CONSTRAINT_LOSS_DEEPEN_GEN: &str = "w29-007-constraint-loss-deepen-v2";

/// P4 wave step — full Kleisli compose in production rollout deferred.
pub const P4_KLEISLI_COMPOSE_DEFERRED_STEP: &str = "P4-KLEISLI-COMPOSE";

/// Mass-conservation tensor slack deferred beyond CD-only hot path.
pub const MASS_CONSERVATION_TENSOR_DEFERRED_STEP: &str = "P4-MASS-TENSOR";

/// Honest physics posture — soft penalty is a training surrogate, not physics GREEN.
pub const CONSTRAINT_LOSS_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not wired through this module alone.
pub const CONSTRAINT_LOSS_PRODUCTION_WIRED: bool = false;

/// Master gate pin — not claimed by constraint_loss.
pub const CONSTRAINT_LOSS_MASTER: bool = false;

/// OP-5 claim pin — constraint_loss does **not** claim operator OP-5.
pub const CONSTRAINT_LOSS_OP5_CLAIMED: bool = false;

/// Whether CD hot margin/violation tensors are landed.
pub const CONSTRAINT_LOSS_CD_HOT_LANDED: bool = true;

/// Whether Landauer hot slack tensors are landed.
pub const CONSTRAINT_LOSS_LANDAUER_HOT_LANDED: bool = true;

/// Whether scaled λ penalty hooks (CD + Landauer short-circuit) are landed.
pub const CONSTRAINT_LOSS_SCALED_HOOKS_LANDED: bool = true;

/// Whether module-owned CD+Landauer soft-compose surrogate is landed.
pub const CONSTRAINT_LOSS_SOFT_COMPOSE_LANDED: bool = true;

/// Whether host-mirror explanation telemetry is landed.
pub const CONSTRAINT_LOSS_EXPLANATION_LANDED: bool = true;

/// Whether canonical Core host dissipation mirror is landed.
pub const CONSTRAINT_LOSS_CANONICAL_HOST_MIRROR_LANDED: bool = true;

/// Whether mass-conservation Burn tensor slack is landed (honest: deferred).
pub const CONSTRAINT_LOSS_MASS_TENSOR_LANDED: bool = false;

/// Whether cold host mass density residual helper is landed.
pub const CONSTRAINT_LOSS_HOST_MASS_RESIDUAL_LANDED: bool = true;

/// Honest fence string for orchestrator / census probes.
pub const CONSTRAINT_LOSS_HONEST_FENCE: &str =
    "cd_hot_landed=true|landauer_hot_landed=true|scaled_hooks_landed=true|soft_compose_landed=true|explanation_landed=true|canonical_host_mirror=true|host_mass_residual=true|mass_tensor_deferred=true|production_wired=false|physics_green=false|master=false|op5_claimed=false";

/// Constraint-loss fence facet count (honest census).
pub const CONSTRAINT_LOSS_FENCE_FACET_COUNT: usize = 10;

/// Constraint-loss fence facets wired today (8/10 measured; mass tensor + production open).
pub const CONSTRAINT_LOSS_FENCE_WIRED_COUNT: usize = 8;

/// Constraint-loss wire-hop count (honest census).
pub const CONSTRAINT_LOSS_WIRE_HOP_COUNT: usize = 9;

/// Constraint-loss wire hops closed today (8/9 measured; mass tensor deferred).
pub const CONSTRAINT_LOSS_WIRE_HOPS_CLOSED: usize = 8;

/// Stable facet ids for constraint_loss production fence census.
pub const CONSTRAINT_LOSS_FENCE_FACET_IDS: &[&str] = &[
    "cd_margin_hot",
    "cd_violation_relu",
    "cd_explanation_hot",
    "landauer_slack_hot",
    "scaled_penalty_hooks",
    "soft_compose_surrogate",
    "canonical_host_mirror",
    "host_mass_residual",
    "mass_conservation_tensor",
    "production_wired",
];

/// One facet of the constraint_loss production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstraintLossProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Constraint-loss production fence facet inventory (honest posture SSOT).
pub const CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS: &[ConstraintLossProductionFenceFacet] = &[
    ConstraintLossProductionFenceFacet {
        facet: "cd_margin_hot",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "cd_violation_relu",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "cd_explanation_hot",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "landauer_slack_hot",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "scaled_penalty_hooks",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "soft_compose_surrogate",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "canonical_host_mirror",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "host_mass_residual",
        wired: true,
        owning_slice: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
    },
    ConstraintLossProductionFenceFacet {
        facet: "mass_conservation_tensor",
        wired: false,
        owning_slice: MASS_CONSERVATION_TENSOR_DEFERRED_STEP,
    },
    ConstraintLossProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: P4_KLEISLI_COMPOSE_DEFERRED_STEP,
    },
];

/// One hop in the constraint_loss hot→gateway wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstraintLossWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Constraint-loss hot→gateway wire map (training surrogate only).
pub const CONSTRAINT_LOSS_WIRE_HOPS: &[ConstraintLossWireHop] = &[
    ConstraintLossWireHop {
        ordinal: 1,
        surface: "umst-manifold::ai::constraint_loss::clausius_duhem_margin",
        role: "Hot CD margin D_int = −ρ ψ̇",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 2,
        surface: "umst-manifold::ai::constraint_loss::clausius_duhem_violation",
        role: "ReLU slack relu(−margin) for Burn backprop",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 3,
        surface: "umst-manifold::ai::constraint_loss::landauer_slack_violation",
        role: "Landauer erasure ReLU slack at tensor granularity",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 4,
        surface: "umst-manifold::ai::constraint_loss::soft_compose_cd_landauer_penalty",
        role: "Module CD+Landauer soft compose (training surrogate)",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 5,
        surface: "umst-manifold::ai::ppo::ManifoldGateway::constraint_loss_penalty",
        role: "Gateway λ_cd scaled CD soft penalty (feature-gated)",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 6,
        surface: "umst-manifold::ai::constraint_loss::canonical_core_net_dissipation_host",
        role: "Cold host mirror via canonical_core_gate_outcome",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 7,
        surface: "umst-manifold::ai::constraint_loss::host_mass_density_residual",
        role: "Cold host |ρ_new−ρ_old| residual (not Burn tensor)",
        wired: true,
    },
    ConstraintLossWireHop {
        ordinal: 8,
        surface: "umst-manifold::ai::constraint_loss::mass_conservation_tensor_slack",
        role: "Mass-conservation tensor slack (deferred refuse)",
        wired: false,
    },
    ConstraintLossWireHop {
        ordinal: 9,
        surface: "umst-manifold::ai::ppo::ManifoldGateway::total_constraint_loss_penalty",
        role: "Feature-gated CD+Landauer compose (not production_wired)",
        wired: true,
    },
];

/// Compile-time fence — production/master/physics GREEN / OP-5 flip not authorized.
const _: () = assert!(!CONSTRAINT_LOSS_PHYSICS_GREEN);
const _: () = assert!(!CONSTRAINT_LOSS_PRODUCTION_WIRED);
const _: () = assert!(!CONSTRAINT_LOSS_MASTER);
const _: () = assert!(!CONSTRAINT_LOSS_OP5_CLAIMED);
const _: () = assert!(!CONSTRAINT_LOSS_MASS_TENSOR_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_CD_HOT_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_LANDAUER_HOT_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_SCALED_HOOKS_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_SOFT_COMPOSE_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_EXPLANATION_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_CANONICAL_HOST_MIRROR_LANDED);
const _: () = assert!(CONSTRAINT_LOSS_HOST_MASS_RESIDUAL_LANDED);

/// Count wired constraint_loss fence facets (must match [`CONSTRAINT_LOSS_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn constraint_loss_fence_wired_count() -> usize {
    CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count wired constraint_loss wire hops (must match [`CONSTRAINT_LOSS_WIRE_HOPS_CLOSED`]).
#[must_use]
pub fn constraint_loss_wire_hops_closed() -> usize {
    CONSTRAINT_LOSS_WIRE_HOPS.iter().filter(|h| h.wired).count()
}

/// Honest production wiring — **false** until P4 Kleisli compose measured.
#[must_use]
pub const fn constraint_loss_production_wired() -> bool {
    false
}

/// Master composition wiring — **false** until fleet orch claims MASTER.
#[must_use]
pub const fn constraint_loss_master_composition_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!constraint_loss_production_wired());

/// Measured honest-posture snapshot for constraint_loss (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstraintLossHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub op5_claimed: bool,
    pub cd_hot_landed: bool,
    pub landauer_hot_landed: bool,
    pub scaled_hooks_landed: bool,
    pub soft_compose_landed: bool,
    pub explanation_landed: bool,
    pub canonical_host_mirror_landed: bool,
    pub host_mass_residual_landed: bool,
    pub mass_tensor_landed: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub wire_hops_closed: usize,
    pub honest_fence: &'static str,
    pub deepen_gen: &'static str,
    pub deferred_mass_tensor: &'static str,
    pub deferred_kleisli_compose: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn constraint_loss_honest_posture_bundle() -> ConstraintLossHonestPosture {
    ConstraintLossHonestPosture {
        physics_green: CONSTRAINT_LOSS_PHYSICS_GREEN,
        production_wired: CONSTRAINT_LOSS_PRODUCTION_WIRED,
        master: CONSTRAINT_LOSS_MASTER,
        op5_claimed: CONSTRAINT_LOSS_OP5_CLAIMED,
        cd_hot_landed: CONSTRAINT_LOSS_CD_HOT_LANDED,
        landauer_hot_landed: CONSTRAINT_LOSS_LANDAUER_HOT_LANDED,
        scaled_hooks_landed: CONSTRAINT_LOSS_SCALED_HOOKS_LANDED,
        soft_compose_landed: CONSTRAINT_LOSS_SOFT_COMPOSE_LANDED,
        explanation_landed: CONSTRAINT_LOSS_EXPLANATION_LANDED,
        canonical_host_mirror_landed: CONSTRAINT_LOSS_CANONICAL_HOST_MIRROR_LANDED,
        host_mass_residual_landed: CONSTRAINT_LOSS_HOST_MASS_RESIDUAL_LANDED,
        mass_tensor_landed: CONSTRAINT_LOSS_MASS_TENSOR_LANDED,
        fence_facet_count: CONSTRAINT_LOSS_FENCE_FACET_COUNT,
        fence_wired_count: constraint_loss_fence_wired_count(),
        wire_hops_closed: constraint_loss_wire_hops_closed(),
        honest_fence: CONSTRAINT_LOSS_HONEST_FENCE,
        deepen_gen: CONSTRAINT_LOSS_DEEPEN_GEN,
        deferred_mass_tensor: MASS_CONSERVATION_TENSOR_DEFERRED_STEP,
        deferred_kleisli_compose: P4_KLEISLI_COMPOSE_DEFERRED_STEP,
    }
}

/// Typed probe for constraint_loss posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstraintLossPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub deepen_gen: &'static str,
    pub cd_hot_landed: bool,
    pub landauer_hot_landed: bool,
    pub scaled_hooks_landed: bool,
    pub soft_compose_landed: bool,
    pub explanation_landed: bool,
    pub canonical_host_mirror_landed: bool,
    pub host_mass_residual_landed: bool,
    pub mass_tensor_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub op5_claimed: bool,
    pub physics_green: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub wire_hops_closed: usize,
    pub honest_fence: &'static str,
}

/// Build introspection probe for constraint_loss done-when / fleet checks.
#[must_use]
pub const fn constraint_loss_posture_probe() -> ConstraintLossPostureProbe {
    ConstraintLossPostureProbe {
        cell_id: W29_CONSTRAINT_LOSS_DEEPEN_CELL,
        posture_tag: CONSTRAINT_LOSS_POSTURE_TAG,
        deepen_gen: CONSTRAINT_LOSS_DEEPEN_GEN,
        cd_hot_landed: CONSTRAINT_LOSS_CD_HOT_LANDED,
        landauer_hot_landed: CONSTRAINT_LOSS_LANDAUER_HOT_LANDED,
        scaled_hooks_landed: CONSTRAINT_LOSS_SCALED_HOOKS_LANDED,
        soft_compose_landed: CONSTRAINT_LOSS_SOFT_COMPOSE_LANDED,
        explanation_landed: CONSTRAINT_LOSS_EXPLANATION_LANDED,
        canonical_host_mirror_landed: CONSTRAINT_LOSS_CANONICAL_HOST_MIRROR_LANDED,
        host_mass_residual_landed: CONSTRAINT_LOSS_HOST_MASS_RESIDUAL_LANDED,
        mass_tensor_landed: CONSTRAINT_LOSS_MASS_TENSOR_LANDED,
        production_wired: constraint_loss_production_wired(),
        master_composition_wired: constraint_loss_master_composition_wired(),
        op5_claimed: CONSTRAINT_LOSS_OP5_CLAIMED,
        physics_green: CONSTRAINT_LOSS_PHYSICS_GREEN,
        fence_facet_count: CONSTRAINT_LOSS_FENCE_FACET_COUNT,
        fence_wired_count: CONSTRAINT_LOSS_FENCE_WIRED_COUNT,
        wire_hops_closed: CONSTRAINT_LOSS_WIRE_HOPS_CLOSED,
        honest_fence: CONSTRAINT_LOSS_HONEST_FENCE,
    }
}

/// Constraint-loss SSOT landed with production/master/OP-5 composition honestly open.
#[must_use]
pub fn constraint_loss_posture_honest(probe: &ConstraintLossPostureProbe) -> bool {
    probe.cell_id == W29_CONSTRAINT_LOSS_DEEPEN_CELL
        && probe.posture_tag == CONSTRAINT_LOSS_POSTURE_TAG
        && probe.deepen_gen == CONSTRAINT_LOSS_DEEPEN_GEN
        && probe.cd_hot_landed
        && probe.landauer_hot_landed
        && probe.scaled_hooks_landed
        && probe.soft_compose_landed
        && probe.explanation_landed
        && probe.canonical_host_mirror_landed
        && probe.host_mass_residual_landed
        && !probe.mass_tensor_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_composition_wired
        && !probe.op5_claimed
        && probe.fence_facet_count == CONSTRAINT_LOSS_FENCE_FACET_COUNT
        && probe.fence_wired_count == CONSTRAINT_LOSS_FENCE_WIRED_COUNT
        && probe.wire_hops_closed == constraint_loss_wire_hops_closed()
        && probe.honest_fence.contains("cd_hot_landed=true")
        && probe.honest_fence.contains("soft_compose_landed=true")
        && probe.honest_fence.contains("mass_tensor_deferred=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("op5_claimed=false")
}

/// Validate constraint_loss posture honesty — fail closed on fake production/master/GREEN/OP-5.
pub fn validate_constraint_loss_posture_honesty() -> Result<(), &'static str> {
    let probe = constraint_loss_posture_probe();
    if probe.physics_green {
        return Err(
            "CONSTRAINT_LOSS_PHYSICS_GREEN must stay false — soft penalty is surrogate only",
        );
    }
    if probe.production_wired {
        return Err("constraint_loss_production_wired must stay false until P4 Kleisli compose");
    }
    if probe.master_composition_wired {
        return Err("constraint_loss_master_composition_wired must stay false until fleet MASTER");
    }
    if probe.op5_claimed {
        return Err("CONSTRAINT_LOSS_OP5_CLAIMED must stay false — OP-5 not owned here");
    }
    if probe.mass_tensor_landed || CONSTRAINT_LOSS_MASS_TENSOR_LANDED {
        return Err("CONSTRAINT_LOSS_MASS_TENSOR_LANDED must stay false until P4-MASS-TENSOR");
    }
    if constraint_loss_fence_wired_count() != CONSTRAINT_LOSS_FENCE_WIRED_COUNT {
        return Err("fence wired count drift vs CONSTRAINT_LOSS_FENCE_WIRED_COUNT");
    }
    if constraint_loss_wire_hops_closed() != CONSTRAINT_LOSS_WIRE_HOPS_CLOSED {
        return Err("wire hops closed drift vs CONSTRAINT_LOSS_WIRE_HOPS_CLOSED");
    }
    if CONSTRAINT_LOSS_WIRE_HOPS.len() != CONSTRAINT_LOSS_WIRE_HOP_COUNT {
        return Err("wire hop inventory length drift");
    }
    if CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS.len() != CONSTRAINT_LOSS_FENCE_FACET_COUNT {
        return Err("fence facet inventory length drift");
    }
    if CONSTRAINT_LOSS_FENCE_FACET_IDS.len() != CONSTRAINT_LOSS_FENCE_FACET_COUNT {
        return Err("fence facet id inventory length drift");
    }
    for (i, hop) in CONSTRAINT_LOSS_WIRE_HOPS.iter().enumerate() {
        if hop.ordinal as usize != i + 1 {
            return Err("wire hop ordinal not sequential");
        }
    }
    if !constraint_loss_posture_honest(&probe) {
        return Err("constraint_loss_posture_probe failed honest fence census");
    }
    Ok(())
}

/// Deferred facet ids still open in the production fence matrix.
#[must_use]
pub fn constraint_loss_deferred_facet_ids() -> Vec<&'static str> {
    CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .map(|f| f.facet)
        .collect()
}

/// Open (unwired) wire-hop surfaces still deferred.
#[must_use]
pub fn constraint_loss_open_hop_surfaces() -> Vec<&'static str> {
    CONSTRAINT_LOSS_WIRE_HOPS
        .iter()
        .filter(|h| !h.wired)
        .map(|h| h.surface)
        .collect()
}

/// Deferred / refuse errors for surfaces not authorized at this slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintLossDeferredError {
    /// Mass-conservation Burn tensor slack not landed (`P4-MASS-TENSOR`).
    MassConservationTensor,
    /// Full Kleisli production compose not authorized.
    ProductionKleisliCompose,
}

impl ConstraintLossDeferredError {
    /// Stable owning-slice label for census.
    #[must_use]
    pub const fn owning_slice(self) -> &'static str {
        match self {
            Self::MassConservationTensor => MASS_CONSERVATION_TENSOR_DEFERRED_STEP,
            Self::ProductionKleisliCompose => P4_KLEISLI_COMPOSE_DEFERRED_STEP,
        }
    }
}

/// Batch length mismatch on the CD hot path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintLossBatchError {
    LengthMismatch {
        old_density_len: usize,
        new_density_len: usize,
        old_free_energy_len: usize,
        new_free_energy_len: usize,
        dt_s_len: usize,
    },
}

use burn::tensor::activation::relu;
use burn::tensor::{backend::Backend, Tensor};

use crate::runtime::catalog::traceability::{CD_TRANSITION_CATALOG_ID, LANDAUER_CBF_CATALOG_ID};
pub use crate::runtime::gate::evidence::{
    admissibility_from_violation, AdmissibilityToken, ConstraintExplanation,
};
pub use crate::runtime::gate::{AdmissibilityMargin, ADMISSIBILITY_MARGIN_EPS};

/// Validate CD batch contract — all `[B]` tensors share length.
pub fn clausius_duhem_batch_contract<B: Backend<FloatElem = f32>>(
    old_density: &Tensor<B, 1>,
    new_density: &Tensor<B, 1>,
    old_free_energy: &Tensor<B, 1>,
    new_free_energy: &Tensor<B, 1>,
    dt_s: &Tensor<B, 1>,
) -> Result<usize, ConstraintLossBatchError> {
    let n = old_density.dims()[0];
    let new_d = new_density.dims()[0];
    let old_fe = old_free_energy.dims()[0];
    let new_fe = new_free_energy.dims()[0];
    let dt = dt_s.dims()[0];
    if new_d != n || old_fe != n || new_fe != n || dt != n {
        return Err(ConstraintLossBatchError::LengthMismatch {
            old_density_len: n,
            new_density_len: new_d,
            old_free_energy_len: old_fe,
            new_free_energy_len: new_fe,
            dt_s_len: dt,
        });
    }
    Ok(n)
}

/// Host-side Core gate net dissipation — routes through canonical surface (Phase 0d).
#[must_use]
pub fn canonical_core_net_dissipation_host(
    old_density: f64,
    new_density: f64,
    old_free_energy: f64,
    new_free_energy: f64,
    dt_s: f64,
    power_input: f64,
) -> f64 {
    crate::gate::route::canonical_core_gate_outcome(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
        power_input,
    )
    .net_dissipation
}

/// Cold host density residual `|ρ_new − ρ_old|` — **not** a Burn tensor mass slack.
///
/// Honest: this is a scalar residual helper for census / host mirrors. Mass-conservation
/// tensor backprop remains deferred via [`mass_conservation_tensor_slack`].
#[must_use]
pub fn host_mass_density_residual(old_density: f64, new_density: f64) -> f64 {
    (new_density - old_density).abs()
}

/// Deferred Burn mass-conservation tensor slack — **refuses** until `P4-MASS-TENSOR`.
///
/// Call sites may type-check against this surface; the Result is always `Err` today.
pub fn mass_conservation_tensor_slack<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    _tolerance: f32,
) -> Result<Tensor<B, 1>, ConstraintLossDeferredError> {
    // Consume inputs so typed call sites compile; refuse until mass tensor lands.
    drop(old_density);
    drop(new_density);
    Err(ConstraintLossDeferredError::MassConservationTensor)
}

/// Numerical floor on `dt` matching [`crate::gate::transition_proposal::transition_outcome`].
const DT_EPS: f32 = 1e-10;

/// Boltzmann constant (J/K) — matches [`crate::constants::landauer_bit_energy_joules`] fallback path.
const K_BOLTZMANN_F32: f32 = 1.380_649e-23;

/// ln(2) bit factor for Landauer erasure floor.
const LN2_F32: f32 = std::f32::consts::LN_2;

/// Per-batch signed Clausius–Duhem margin `D_int = −ρ ψ̇`.
///
/// Hot-path CD slack only. For Mass + CD host alignment see
/// [`crate::gate::route::canonical_core_gate_outcome`].
pub fn clausius_duhem_margin<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let rho = old_density.add(new_density).div_scalar(2.0);
    let psi_dot = new_free_energy
        .sub(old_free_energy)
        .div(dt_s.add_scalar(DT_EPS));
    psi_dot.mul(rho).neg()
}

/// Per-batch ReLU slack for Clausius–Duhem dissipation violation (`relu(−margin)`).
///
/// Mirrors the host gate surrogate `D_int = −ρ ψ̇` with
/// `ρ = (ρ_old + ρ_new) / 2`, `ψ̇ = (ψ_new − ψ_old) / (Δt + ε)`, and returns
/// `relu(−D_int)` so admissible transitions (non-negative dissipation) yield zero loss.
///
/// # Tensor contract
///
/// All inputs are shaped `[B]` with identical batch length.
pub fn clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    relu(
        clausius_duhem_margin(
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
        .neg(),
    )
}

/// Weighted violation slack from host gate evidence (`λ_cd · violation` per witness).
pub fn scaled_constraint_violation_penalty<B: Backend<FloatElem = f32>>(
    lambda_cd: f32,
    violations: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let batch = violations.dims()[0];
    let device = violations.device();
    if lambda_cd == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    violations.mul_scalar(lambda_cd)
}

/// Weighted Clausius–Duhem slack for gateway / PPO penalty hooks.
///
/// Returns zeros when `lambda_cd == 0` without building the violation graph.
pub fn scaled_clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    lambda_cd: f32,
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> Tensor<B, 1> {
    let batch = old_density.dims()[0];
    let device = old_density.device();
    if lambda_cd == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    clausius_duhem_violation(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    )
    .mul_scalar(lambda_cd)
}

/// Per-batch ReLU slack when resolved bits exceed the available Landauer credit (joules).
///
/// Mirrors [`crate::ai::cbf::ThermodynamicCBF::calculate_landauer_cost`] at tensor granularity:
/// `erasure_cost = k_B · T · ln(2) · bits`, returns `relu(erasure_cost − credit_j)`.
pub fn landauer_slack_violation<B: Backend<FloatElem = f32>>(
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> Tensor<B, 1> {
    let bit_energy = temperature_k * LN2_F32 * K_BOLTZMANN_F32;
    let erasure_cost = info_gain_bits.mul_scalar(bit_energy);
    relu(erasure_cost.sub_scalar(available_credit_joules))
}

/// Weighted Landauer slack for gateway / PPO penalty hooks.
pub fn scaled_landauer_slack_violation<B: Backend<FloatElem = f32>>(
    lambda_landauer: f32,
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> Tensor<B, 1> {
    let batch = info_gain_bits.dims()[0];
    let device = info_gain_bits.device();
    if lambda_landauer == 0.0_f32 {
        return Tensor::zeros([batch], &device);
    }
    landauer_slack_violation(info_gain_bits, temperature_k, available_credit_joules)
        .mul_scalar(lambda_landauer)
}

/// Module-owned CD + Landauer soft-compose surrogate (training path only).
///
/// Honest: this is **not** `PRODUCTION_WIRED` / Kleisli production compose. Gateway
/// `total_constraint_loss_penalty` remains feature-gated; this surface is the module SSOT
/// for the same arithmetic without inventing GREEN.
pub fn soft_compose_cd_landauer_penalty<B: Backend<FloatElem = f32>>(
    lambda_cd: f32,
    lambda_landauer: f32,
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> Tensor<B, 1> {
    scaled_clausius_duhem_violation(
        lambda_cd,
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    )
    .add(scaled_landauer_slack_violation(
        lambda_landauer,
        info_gain_bits,
        temperature_k,
        available_credit_joules,
    ))
}

/// Structured explanation for Clausius–Duhem slack at the same batch contract as
/// [`clausius_duhem_violation`].
///
/// Aggregates batch elements by maximum violation (worst offender) for telemetry.
pub fn explain_clausius_duhem_violation<B: Backend<FloatElem = f32>>(
    old_density: Tensor<B, 1>,
    new_density: Tensor<B, 1>,
    old_free_energy: Tensor<B, 1>,
    new_free_energy: Tensor<B, 1>,
    dt_s: Tensor<B, 1>,
) -> ConstraintExplanation {
    let margin_tensor = clausius_duhem_margin(
        old_density.clone(),
        new_density.clone(),
        old_free_energy.clone(),
        new_free_energy.clone(),
        dt_s.clone(),
    );
    let violation = clausius_duhem_violation(
        old_density,
        new_density,
        old_free_energy,
        new_free_energy,
        dt_s,
    );
    let m = margin_tensor
        .clone()
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, |a, b| if b < a { b } else { a });
    let v = violation
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, f32::max);
    ConstraintExplanation {
        margin: AdmissibilityMargin(m),
        violation: v,
        channel_id: CD_TRANSITION_CATALOG_ID,
        admissibility: admissibility_from_violation(v),
    }
}

/// Structured explanation for Landauer slack at the same batch contract as
/// [`landauer_slack_violation`].
///
/// Aggregates batch elements by maximum violation (worst offender) for telemetry.
pub fn explain_landauer_slack_violation<B: Backend<FloatElem = f32>>(
    info_gain_bits: Tensor<B, 1>,
    temperature_k: f32,
    available_credit_joules: f32,
) -> ConstraintExplanation {
    let bit_energy = temperature_k * LN2_F32 * K_BOLTZMANN_F32;
    let erasure_cost = info_gain_bits.clone().mul_scalar(bit_energy);
    let margin_tensor = erasure_cost.sub_scalar(available_credit_joules);
    let violation =
        landauer_slack_violation(info_gain_bits, temperature_k, available_credit_joules);
    let m = margin_tensor
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, |a, b| if b < a { b } else { a });
    let v = violation
        .into_data()
        .value
        .into_iter()
        .fold(0.0_f32, f32::max);
    ConstraintExplanation {
        margin: AdmissibilityMargin(m),
        violation: v,
        channel_id: LANDAUER_CBF_CATALOG_ID,
        admissibility: admissibility_from_violation(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::transition_proposal::transition_outcome;
    use crate::gate::ThermodynamicStateSnapshot;
    use burn::tensor::{Data, Shape};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    fn scalar_tensor(dev: &NdArrayDevice, values: &[f32]) -> Tensor<B, 1> {
        let b = values.len();
        Tensor::<B, 1>::from_data(Data::new(values.to_vec(), Shape::new([b])), dev)
    }

    #[test]
    fn clausius_duhem_violation_zero_when_host_admits() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, 1e-6);
        assert!(
            host.is_energy_positive(),
            "sanity: identity transition admits"
        );

        let violation = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = violation.into_data().value;
        assert!(
            v[0].abs() < 1e-4,
            "admissible host path → zero slack, got {}",
            v[0]
        );
    }

    #[test]
    fn clausius_duhem_violation_matches_host_negative_dissipation() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, 1e-6);
        assert!(
            !host.is_energy_positive(),
            "sanity: ψ spike rejects on host"
        );

        let violation = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = violation.into_data().value;
        let expected = (-host.dissipation).max(0.0) as f32;
        assert!(
            v[0] > 0.0,
            "inadmissible transition must incur positive slack, got {}",
            v[0]
        );
        assert!(
            (v[0] - expected).abs() < 1.0,
            "slack {v0} should track host relu(-D_int) ≈ {expected}",
            v0 = v[0]
        );
    }

    #[test]
    fn explain_clausius_duhem_violation_admissible_token() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2400.0,
            temperature: 293.15,
            free_energy: -1.35e5,
            entropy: 0.05,
            reaction_extent: 0.42,
            strength: 12.7,
        };
        let new = old;
        let dt = 1.0_f64;

        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        assert!(
            explanation.violation.abs() < 1e-4,
            "admissible → zero violation, got {}",
            explanation.violation
        );
        assert_eq!(explanation.admissibility, AdmissibilityToken::Admissible);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
    }

    #[test]
    fn explain_clausius_duhem_violation_inadmissible_token() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;

        let explanation = explain_clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        assert!(
            explanation.violation > 0.0,
            "inadmissible → positive violation, got {}",
            explanation.violation
        );
        assert_eq!(explanation.admissibility, AdmissibilityToken::Inadmissible);
        assert_eq!(explanation.channel_id, CD_TRANSITION_CATALOG_ID);
    }

    #[test]
    fn scaled_clausius_duhem_violation_zero_when_lambda_disabled() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;

        let penalty = scaled_clausius_duhem_violation(
            0.0_f32,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let v: Vec<f32> = penalty.into_data().value;
        assert_eq!(v[0], 0.0_f32, "λ_cd = 0 must short-circuit to zero penalty");
    }

    #[test]
    fn scaled_clausius_duhem_violation_scales_slack() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let lambda = 2.5_f32;

        let slack = clausius_duhem_violation(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let penalty = scaled_clausius_duhem_violation(
            lambda,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let s: Vec<f32> = slack.into_data().value;
        let p: Vec<f32> = penalty.into_data().value;
        assert!(
            s[0] > 0.0,
            "inadmissible transition must incur positive slack"
        );
        assert!(
            (p[0] - lambda * s[0]).abs() < 1e-3,
            "penalty {p0} should equal λ·slack ≈ {expected}",
            p0 = p[0],
            expected = lambda * s[0]
        );
    }

    #[test]
    fn landauer_slack_violation_zero_when_credit_sufficient() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[0.01_f32]);
        let slack = landauer_slack_violation(bits, 300.0_f32, 1.0e6_f32);
        let v: Vec<f32> = slack.into_data().value;
        assert!(
            v[0].abs() < 1e-12,
            "ample credit → zero Landauer slack, got {}",
            v[0]
        );
    }

    #[test]
    fn landauer_slack_violation_positive_when_credit_exhausted() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[1.0_f32]);
        let slack = landauer_slack_violation(bits, 300.0_f32, 0.0_f32);
        let v: Vec<f32> = slack.into_data().value;
        assert!(v[0] > 0.0, "zero credit → positive Landauer slack");
        let expected = 300.0_f32 * LN2_F32 * K_BOLTZMANN_F32;
        assert!(
            (v[0] - expected).abs() < 1e-20,
            "slack {v0} should track k_B T ln2 bits ≈ {expected}",
            v0 = v[0]
        );
    }

    #[test]
    fn scaled_landauer_slack_violation_zero_when_lambda_disabled() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[1.0_f32]);
        let penalty = scaled_landauer_slack_violation(0.0_f32, bits, 300.0_f32, 0.0_f32);
        let v: Vec<f32> = penalty.into_data().value;
        assert_eq!(v[0], 0.0_f32);
    }

    #[test]
    fn scaled_landauer_slack_violation_scales_slack() {
        let dev = NdArrayDevice::default();
        let bits = scalar_tensor(&dev, &[1.0_f32]);
        let lambda = 4.0_f32;
        let slack = landauer_slack_violation(scalar_tensor(&dev, &[1.0_f32]), 300.0_f32, 0.0_f32);
        let penalty = scaled_landauer_slack_violation(lambda, bits, 300.0_f32, 0.0_f32);
        let s: Vec<f32> = slack.into_data().value;
        let p: Vec<f32> = penalty.into_data().value;
        assert!(s[0] > 0.0);
        assert!(
            (p[0] - lambda * s[0]).abs() < 1e-20,
            "penalty {p0} should equal λ·slack ≈ {expected}",
            p0 = p[0],
            expected = lambda * s[0]
        );
    }

    #[test]
    fn constraint_loss_honest_posture_no_invented_green() {
        let posture = constraint_loss_honest_posture_bundle();
        assert!(!posture.physics_green);
        assert!(!posture.production_wired);
        assert!(!posture.master);
        assert!(!posture.op5_claimed);
        assert!(posture.cd_hot_landed);
        assert!(posture.landauer_hot_landed);
        assert!(posture.scaled_hooks_landed);
        assert!(posture.soft_compose_landed);
        assert!(posture.explanation_landed);
        assert!(posture.canonical_host_mirror_landed);
        assert!(posture.host_mass_residual_landed);
        assert!(!posture.mass_tensor_landed);
        assert_eq!(posture.fence_facet_count, CONSTRAINT_LOSS_FENCE_FACET_COUNT);
        assert_eq!(posture.fence_wired_count, CONSTRAINT_LOSS_FENCE_WIRED_COUNT);
        assert_eq!(
            posture.fence_wired_count,
            constraint_loss_fence_wired_count()
        );
        assert_eq!(posture.wire_hops_closed, constraint_loss_wire_hops_closed());
        assert_eq!(posture.honest_fence, CONSTRAINT_LOSS_HONEST_FENCE);
        assert_eq!(posture.deepen_gen, CONSTRAINT_LOSS_DEEPEN_GEN);
        assert_eq!(
            posture.deferred_mass_tensor,
            MASS_CONSERVATION_TENSOR_DEFERRED_STEP
        );
        assert_eq!(
            posture.deferred_kleisli_compose,
            P4_KLEISLI_COMPOSE_DEFERRED_STEP
        );
        assert!(!constraint_loss_production_wired());
        assert!(!constraint_loss_master_composition_wired());
        assert!(!CONSTRAINT_LOSS_OP5_CLAIMED);
        assert!(!CONSTRAINT_LOSS_MASS_TENSOR_LANDED);
    }

    #[test]
    fn constraint_loss_fence_inventory_consistent() {
        assert_eq!(
            CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS.len(),
            CONSTRAINT_LOSS_FENCE_FACET_COUNT
        );
        assert_eq!(
            CONSTRAINT_LOSS_FENCE_FACET_IDS.len(),
            CONSTRAINT_LOSS_FENCE_FACET_COUNT
        );
        assert_eq!(
            constraint_loss_fence_wired_count(),
            CONSTRAINT_LOSS_FENCE_WIRED_COUNT
        );
        assert_eq!(
            CONSTRAINT_LOSS_WIRE_HOPS.len(),
            CONSTRAINT_LOSS_WIRE_HOP_COUNT
        );
        assert_eq!(
            constraint_loss_wire_hops_closed(),
            CONSTRAINT_LOSS_WIRE_HOPS_CLOSED
        );
        for facet in CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS {
            assert!(
                CONSTRAINT_LOSS_FENCE_FACET_IDS.contains(&facet.facet),
                "facet {} must appear in FENCE_FACET_IDS",
                facet.facet
            );
        }
        let production_facet = CONSTRAINT_LOSS_PRODUCTION_FENCE_FACETS
            .iter()
            .find(|f| f.facet == "production_wired")
            .expect("production_wired facet");
        assert!(!production_facet.wired);
        assert!(!CONSTRAINT_LOSS_PRODUCTION_WIRED);
        let deferred = constraint_loss_deferred_facet_ids();
        assert!(deferred.contains(&"mass_conservation_tensor"));
        assert!(deferred.contains(&"production_wired"));
        assert_eq!(deferred.len(), 2);
        let mass_hop = CONSTRAINT_LOSS_WIRE_HOPS
            .iter()
            .find(|h| h.surface.contains("mass_conservation_tensor_slack"))
            .expect("mass hop");
        assert!(!mass_hop.wired);
        assert_eq!(mass_hop.ordinal, 8);
        let open = constraint_loss_open_hop_surfaces();
        assert_eq!(open.len(), 1);
        assert!(open[0].contains("mass_conservation_tensor_slack"));
    }

    #[test]
    fn constraint_loss_posture_probe_validates_honest() {
        let probe = constraint_loss_posture_probe();
        assert!(constraint_loss_posture_honest(&probe));
        assert!(validate_constraint_loss_posture_honesty().is_ok());
        assert_eq!(probe.cell_id, W29_CONSTRAINT_LOSS_DEEPEN_CELL);
        assert_eq!(probe.posture_tag, CONSTRAINT_LOSS_POSTURE_TAG);
        assert_eq!(probe.deepen_gen, CONSTRAINT_LOSS_DEEPEN_GEN);
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(!probe.op5_claimed);
        assert!(!probe.mass_tensor_landed);
        assert!(probe.soft_compose_landed);
        assert!(probe.host_mass_residual_landed);
    }

    #[test]
    fn canonical_core_net_dissipation_host_matches_transition_outcome() {
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let power = 1e-6_f64;
        let host = transition_outcome(&old, &new, dt, power);
        let net = canonical_core_net_dissipation_host(
            old.density,
            new.density,
            old.free_energy,
            new.free_energy,
            dt,
            power,
        );
        assert!(
            (net - host.dissipation).abs() < 1e-5 * host.dissipation.abs().max(1.0),
            "canonical host mirror {net} != transition_outcome.dissipation {}",
            host.dissipation
        );
    }

    #[test]
    fn clausius_duhem_margin_matches_host_dissipation() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let host = transition_outcome(&old, &new, dt, 1e-6);
        let margin = clausius_duhem_margin(
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let m: Vec<f32> = margin.into_data().value;
        assert!(
            (m[0] - host.dissipation as f32).abs() < 1.0,
            "hot margin {m0} should track host dissipation {d}",
            m0 = m[0],
            d = host.dissipation
        );
    }

    #[test]
    fn explain_landauer_slack_violation_admissible_token() {
        let dev = NdArrayDevice::default();
        let explanation = explain_landauer_slack_violation(
            scalar_tensor(&dev, &[0.01_f32]),
            300.0_f32,
            1.0e6_f32,
        );
        assert!(explanation.violation.abs() < 1e-12);
        assert_eq!(explanation.admissibility, AdmissibilityToken::Admissible);
        assert_eq!(explanation.channel_id, LANDAUER_CBF_CATALOG_ID);
    }

    #[test]
    fn explain_landauer_slack_violation_inadmissible_token() {
        let dev = NdArrayDevice::default();
        // Macroscopic bit count so k_B T ln2 · bits ≫ ADMISSIBILITY_MARGIN_EPS.
        // Single-bit Landauer (~1e-21 J) is below the host eps floor and stays Admissible.
        let bits = 1.0e20_f32;
        let explanation =
            explain_landauer_slack_violation(scalar_tensor(&dev, &[bits]), 300.0_f32, 0.0_f32);
        assert!(explanation.violation > ADMISSIBILITY_MARGIN_EPS);
        assert_eq!(explanation.admissibility, AdmissibilityToken::Inadmissible);
        assert_eq!(explanation.channel_id, LANDAUER_CBF_CATALOG_ID);
        let expected = bits * 300.0_f32 * LN2_F32 * K_BOLTZMANN_F32;
        assert!(
            (explanation.violation - expected).abs() < 1e-6 * expected,
            "violation {} should track erasure cost {}",
            explanation.violation,
            expected
        );
    }

    #[test]
    fn scaled_constraint_violation_penalty_scales_host_evidence() {
        let dev = NdArrayDevice::default();
        let violations = scalar_tensor(&dev, &[0.5_f32, 2.0_f32]);
        let lambda = 3.0_f32;
        let penalty = scaled_constraint_violation_penalty(lambda, violations);
        let p: Vec<f32> = penalty.into_data().value;
        assert!((p[0] - 1.5_f32).abs() < 1e-6);
        assert!((p[1] - 6.0_f32).abs() < 1e-6);
    }

    #[test]
    fn mass_conservation_tensor_slack_refuses_deferred() {
        let dev = NdArrayDevice::default();
        let err = mass_conservation_tensor_slack(
            scalar_tensor(&dev, &[2400.0_f32]),
            scalar_tensor(&dev, &[2400.0_f32]),
            1e-6_f32,
        )
        .expect_err("mass tensor must refuse until P4-MASS-TENSOR");
        assert_eq!(err, ConstraintLossDeferredError::MassConservationTensor);
        assert_eq!(err.owning_slice(), MASS_CONSERVATION_TENSOR_DEFERRED_STEP);
        assert!(!CONSTRAINT_LOSS_MASS_TENSOR_LANDED);
    }

    #[test]
    fn host_mass_density_residual_tracks_abs_delta() {
        assert!((host_mass_density_residual(2400.0, 2400.0) - 0.0).abs() < 1e-12);
        assert!((host_mass_density_residual(2400.0, 2410.0) - 10.0).abs() < 1e-12);
        assert!((host_mass_density_residual(2410.0, 2400.0) - 10.0).abs() < 1e-12);
        assert!(CONSTRAINT_LOSS_HOST_MASS_RESIDUAL_LANDED);
    }

    #[test]
    fn soft_compose_cd_landauer_penalty_sums_scaled_slacks() {
        let dev = NdArrayDevice::default();
        let old = ThermodynamicStateSnapshot {
            density: 2200.0,
            temperature: 300.0,
            free_energy: -2.0e5,
            entropy: 0.2,
            reaction_extent: 0.5,
            strength: 20.0,
        };
        let new = ThermodynamicStateSnapshot {
            free_energy: -1.0e4,
            ..old
        };
        let dt = 1.0_f64;
        let lambda_cd = 2.0_f32;
        let lambda_l = 3.0_f32;
        let cd = scaled_clausius_duhem_violation(
            lambda_cd,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
        );
        let land = scaled_landauer_slack_violation(
            lambda_l,
            scalar_tensor(&dev, &[1.0_f32]),
            300.0_f32,
            0.0_f32,
        );
        let composed = soft_compose_cd_landauer_penalty(
            lambda_cd,
            lambda_l,
            scalar_tensor(&dev, &[old.density as f32]),
            scalar_tensor(&dev, &[new.density as f32]),
            scalar_tensor(&dev, &[old.free_energy as f32]),
            scalar_tensor(&dev, &[new.free_energy as f32]),
            scalar_tensor(&dev, &[dt as f32]),
            scalar_tensor(&dev, &[1.0_f32]),
            300.0_f32,
            0.0_f32,
        );
        let c: Vec<f32> = cd.into_data().value;
        let l: Vec<f32> = land.into_data().value;
        let p: Vec<f32> = composed.into_data().value;
        assert!(c[0] > 0.0);
        assert!(l[0] > 0.0);
        assert!(
            (p[0] - (c[0] + l[0])).abs() < 1e-3,
            "soft compose {p0} should equal CD+Landauer {sum}",
            p0 = p[0],
            sum = c[0] + l[0]
        );
        assert!(CONSTRAINT_LOSS_SOFT_COMPOSE_LANDED);
        assert!(!CONSTRAINT_LOSS_PRODUCTION_WIRED);
    }

    #[test]
    fn clausius_duhem_batch_contract_accepts_matched_lens() {
        let dev = NdArrayDevice::default();
        let a = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let b = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let c = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let d = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let e = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        assert_eq!(clausius_duhem_batch_contract(&a, &b, &c, &d, &e), Ok(2));
    }

    #[test]
    fn clausius_duhem_batch_contract_rejects_mismatch() {
        let dev = NdArrayDevice::default();
        let a = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let b = scalar_tensor(&dev, &[1.0_f32]);
        let c = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let d = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        let e = scalar_tensor(&dev, &[1.0_f32, 2.0_f32]);
        match clausius_duhem_batch_contract(&a, &b, &c, &d, &e) {
            Err(ConstraintLossBatchError::LengthMismatch {
                old_density_len,
                new_density_len,
                ..
            }) => {
                assert_eq!(old_density_len, 2);
                assert_eq!(new_density_len, 1);
            }
            Ok(_) => panic!("expected length mismatch"),
        }
    }

    #[test]
    fn constraint_loss_deepen_gen_and_op5_fence_locked() {
        assert_eq!(
            CONSTRAINT_LOSS_DEEPEN_GEN,
            "w29-007-constraint-loss-deepen-v2"
        );
        assert!(!CONSTRAINT_LOSS_OP5_CLAIMED);
        assert!(CONSTRAINT_LOSS_HONEST_FENCE.contains("op5_claimed=false"));
        assert!(CONSTRAINT_LOSS_HONEST_FENCE.contains("soft_compose_landed=true"));
        assert!(CONSTRAINT_LOSS_HONEST_FENCE.contains("host_mass_residual=true"));
        assert_eq!(
            ConstraintLossDeferredError::ProductionKleisliCompose.owning_slice(),
            P4_KLEISLI_COMPOSE_DEFERRED_STEP
        );
    }
}

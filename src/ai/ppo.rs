// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! PPO gateway and reward wiring.
//!
//! ## IO barrier (lazy solver cores, **fp-categorical-v04**)
//!
//! Treat [`ManifoldGateway`] as the **policy-facing boundary** between differentiable
//! physics (cartridge / solver stacks) and scalar **host** decisions:
//!
//! - **On-device reductions**: `dissipation.sum_dim(1)`, `free_energy` / reward reductions use
//!   `sum_dim` / `mean_dim` and return [`Tensor`]s — no `.into_scalar()` on the reward path here.
//! - **Deliberate scalar sync**: [`ThermodynamicCBF::verify_tensor_update`](crate::ai::cbf::ThermodynamicCBF::verify_tensor_update)
//!   sums `info_gain` and batch-summed `d_int`, then performs the **two** `.into_scalar()` reductions
//!   per topology step so Landauer erasure, Clausius–Duhem dissipation credit, and energy bookkeeping
//!   run in ordinary `f64` control flow (see that method’s docs). That is the canonical **bits +
//!   dissipation → host** read for this stack; keep additional `.into_scalar()` out of inner solver
//!   iterations unless required for numerics or convergence tests.
//! - **File / JSON**: this crate does not load UMST from disk; any serialization or filesystem
//!   I/O belongs in cartridges or upstream runners — keep solver kernels free of `std::fs` so
//!   they stay composable and lazy-friendly.
//!
//! Nodal diagnostics: [`crate::core::emergence::nodal_defect_tensor`],
//! [`crate::core::emergence::combine_nodal_for_reward`]; grid hotspots:
//! [`crate::core::emergence::EmergenceMonitor`].
//!
//! Optional structural-margin shaping: [`ManifoldGateway::zeta`] scales a per-batch
//! **mean** of [`PhysicalResult::safety_margin`](crate::core::traits::PhysicalResult::safety_margin)
//! added to the scalar reward. Default **ζ = 0** keeps the legacy reward and leaves the
//! thermodynamic CBF gate unchanged.
//!
//! With the **`information_density`** crate feature, [`ManifoldGateway::eta`] adds
//! **η · mean(information_density)** from the optional `information_density` field on [`PhysicalResult`]
//! the same way. Default **η = 0** preserves the reward without that term.
//!
//! With **`formal-witness`**, [`Self::evaluate_topology_step`] (and [`Self::evaluate_topology_step_formal`])
//! may reject transitions when gateway and UMST disagree on [`crate::core::tensors::UnifiedMaterialStateTensor::catalog_schema_digest`]
//! (both sides must opt in via `Some(digest)`; with **`formal-witness`**, [`Self::new`] pins the
//! gateway expectation to compiled `catalog.lock.json` — UMST still needs
//! [`UnifiedMaterialStateTensor::with_lock_catalog_schema_digest`] or an explicit digest).
//!
//! # Honest boundary (W29-013)
//!
//! [`ManifoldGateway`] is the **policy-facing IO barrier** — CBF admit/reject, scalar reward
//! shaping (α/β/γ/ζ/η), and formal reject bridge. It does **not** attest physics GREEN,
//! end-to-end production PPO training closure, or MASTER retick. Learner spine BIND lives in
//! [`crate::ai::liquid_ppo`] / ADK harness — this module never flips those bits.

use crate::ai::cbf::ThermodynamicCBF;
use crate::ai::constraint_loss::scaled_clausius_duhem_violation;

/// W29 wave cell id — manifold gateway SSOT deepen.
pub const PPO_CELL_ID: &str = "W29-013-PPO";

/// Primary topology-step morphism @ SSOT (not gate alias).
pub const PPO_MORPHISM_ID: &str = "evaluate_topology_step";

/// Formal reject morphism companion (structured [`FormalReject`] surface).
pub const PPO_FORMAL_MORPHISM_ID: &str = "evaluate_topology_step_formal";

/// Honest posture — gateway SSOT partial; tests deepen only (`MASTER_RETICK=no`).
pub const PPO_POSTURE_TAG: &str = "honest-manifold-gateway-ssot-partial";

/// Compile-time honest fence — no production / GREEN / MASTER flip at posture tier.
pub const PPO_HONEST_FENCE: &str =
    "gateway_ssot_landed=true cbf_gate_landed=true reward_weights_pinned=true production_wired=false physics_green=false master_retick=false";

/// Honest non-claim @ source — CBF + reward path measured in crate tests; full training loop is not.
pub const PPO_SOURCE_NON_CLAIM: &str =
    "ManifoldGateway CBF admit + scalar reward shaping measured in crate tests; production PPO training loop / physics GREEN / MASTER retick not claimed at W29 deepen tier.";

/// Whether production PPO training loop is closed end-to-end — **false** until orchestrator BIND + learner receipts.
pub const PPO_PRODUCTION_WIRED: bool = false;

/// Whether physics GREEN is claimed for gateway reward path — **false** (CBF gate is real; full physics closure is not).
pub const PPO_PHYSICS_GREEN: bool = false;

/// Whether MASTER retick is authorized for this cell — **false** @ W29 deepen tier.
pub const PPO_MASTER_RETICK: bool = false;

/// Whether gateway SSOT + CBF + default reward weights are landed @ this slice.
pub const PPO_GATEWAY_SSOT_LANDED: bool = true;

/// Whether CBF gate path through [`ManifoldGateway::evaluate_topology_step_formal`] is landed.
pub const PPO_CBF_GATE_LANDED: bool = true;

/// Whether default α/β/γ/ζ/η reward weights are pinned in [`ManifoldGateway::new`].
pub const PPO_REWARD_WEIGHTS_PINNED: bool = true;

/// Production fence facet count (honest census).
pub const PPO_FENCE_FACET_COUNT: usize = 6;

/// Fence facets wired today (3/6 measured; production / GREEN / MASTER remain open).
pub const PPO_FENCE_WIRED_COUNT: usize = 3;

/// Default reward weights pinned in [`ManifoldGateway::new`].
pub const PPO_DEFAULT_ALPHA: f32 = 1.0;
pub const PPO_DEFAULT_BETA: f32 = 0.5;
pub const PPO_DEFAULT_GAMMA: f32 = 2.0;
pub const PPO_DEFAULT_ZETA: f32 = 0.0;
pub const PPO_DEFAULT_ETA: f32 = 0.0;

/// Allowlisted proof filter for this cell (fleet / status crosswalk; not a GREEN claim).
pub const PPO_PROOF_CMD: &str =
    "bash outputs/.tmp/fleet_away_rustc188.sh --check && cargo test -p umst-manifold ppo";

/// Compile-time fence — production / GREEN / MASTER flip not authorized at posture tier.
const _: () = assert!(!PPO_PRODUCTION_WIRED);
const _: () = assert!(!PPO_PHYSICS_GREEN);
const _: () = assert!(!PPO_MASTER_RETICK);
const _: () = assert!(PPO_GATEWAY_SSOT_LANDED);
const _: () = assert!(PPO_CBF_GATE_LANDED);
const _: () = assert!(PPO_REWARD_WEIGHTS_PINNED);
const _: () = assert!(PPO_FENCE_WIRED_COUNT <= PPO_FENCE_FACET_COUNT);
const _: () = assert!(PPO_FENCE_FACET_COUNT - PPO_FENCE_WIRED_COUNT == 3);

/// Pinned scalar reward weights (α/β/γ/ζ/η) — host shaping only; not physics GREEN.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpoRewardWeights {
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub zeta: f32,
    pub eta: f32,
}

impl PpoRewardWeights {
    /// Defaults matching [`ManifoldGateway::new`] / [`PPO_DEFAULT_*`] pins.
    #[must_use]
    pub const fn defaults() -> Self {
        Self {
            alpha: PPO_DEFAULT_ALPHA,
            beta: PPO_DEFAULT_BETA,
            gamma: PPO_DEFAULT_GAMMA,
            zeta: PPO_DEFAULT_ZETA,
            eta: PPO_DEFAULT_ETA,
        }
    }
}

/// Default reward-weight snapshot @ SSOT.
#[must_use]
pub const fn ppo_default_reward_weights() -> PpoRewardWeights {
    PpoRewardWeights::defaults()
}

/// One facet of the PPO production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PpoProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue / deferred.
    pub owning_slice: &'static str,
}

/// PPO production fence facet inventory (honest posture SSOT).
pub const PPO_PRODUCTION_FENCE_FACETS: &[PpoProductionFenceFacet] = &[
    PpoProductionFenceFacet {
        facet: "gateway_ssot",
        wired: true,
        owning_slice: PPO_CELL_ID,
    },
    PpoProductionFenceFacet {
        facet: "cbf_gate",
        wired: true,
        owning_slice: PPO_CELL_ID,
    },
    PpoProductionFenceFacet {
        facet: "reward_weights",
        wired: true,
        owning_slice: PPO_CELL_ID,
    },
    PpoProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "orch-bind-learner-receipts",
    },
    PpoProductionFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "continuum-physics-closure",
    },
    PpoProductionFenceFacet {
        facet: "master_retick",
        wired: false,
        owning_slice: "operator-master-retick",
    },
];

/// Count wired facets in [`PPO_PRODUCTION_FENCE_FACETS`] (runtime check vs [`PPO_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn ppo_fence_wired_count() -> usize {
    PPO_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count open (unwired) production-fence facets — residue census, not a GREEN claim.
#[must_use]
pub fn ppo_fence_open_count() -> usize {
    PPO_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .count()
}

/// Open fence facets deferred to other slices (honest residue inventory).
#[must_use]
pub fn ppo_fence_residue_facets() -> &'static [PpoProductionFenceFacet] {
    // Static partition: first three wired, last three open — return open slice by filter collect is not const;
    // expose via iterator-backed helper for tests / meta.
    const OPEN: &[PpoProductionFenceFacet] = &[
        PpoProductionFenceFacet {
            facet: "production_wired",
            wired: false,
            owning_slice: "orch-bind-learner-receipts",
        },
        PpoProductionFenceFacet {
            facet: "physics_green",
            wired: false,
            owning_slice: "continuum-physics-closure",
        },
        PpoProductionFenceFacet {
            facet: "master_retick",
            wired: false,
            owning_slice: "operator-master-retick",
        },
    ];
    OPEN
}

/// Whether an open-fence owning_slice is deferred off this cell (refuse self-claim of GREEN/MASTER/prod).
#[must_use]
pub fn ppo_residue_owning_slice_deferred(owning_slice: &str) -> bool {
    !owning_slice.is_empty()
        && owning_slice != PPO_CELL_ID
        && !owning_slice.eq_ignore_ascii_case("w29-013-ppo")
}

/// Typed probe for W29 PPO SSOT posture honesty (meta / fleet probes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PpoPostureProbe {
    pub cell_id: &'static str,
    pub morphism_id: &'static str,
    pub formal_morphism_id: &'static str,
    pub posture_tag: &'static str,
    pub honest_fence: &'static str,
    pub source_non_claim: &'static str,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master_retick: bool,
    pub gateway_ssot_landed: bool,
    pub cbf_gate_landed: bool,
    pub reward_weights_pinned: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
}

/// Build live posture probe from compile-time SSOT constants.
#[must_use]
pub fn ppo_posture_probe() -> PpoPostureProbe {
    PpoPostureProbe {
        cell_id: PPO_CELL_ID,
        morphism_id: PPO_MORPHISM_ID,
        formal_morphism_id: PPO_FORMAL_MORPHISM_ID,
        posture_tag: PPO_POSTURE_TAG,
        honest_fence: PPO_HONEST_FENCE,
        source_non_claim: PPO_SOURCE_NON_CLAIM,
        production_wired: PPO_PRODUCTION_WIRED,
        physics_green: PPO_PHYSICS_GREEN,
        master_retick: PPO_MASTER_RETICK,
        gateway_ssot_landed: PPO_GATEWAY_SSOT_LANDED,
        cbf_gate_landed: PPO_CBF_GATE_LANDED,
        reward_weights_pinned: PPO_REWARD_WEIGHTS_PINNED,
        fence_facet_count: PPO_FENCE_FACET_COUNT,
        fence_wired_count: PPO_FENCE_WIRED_COUNT,
    }
}

/// Whether PPO SSOT morphism metadata is pinned @ HEAD (visibility only; no GREEN invent).
#[must_use]
pub fn ppo_morphism_pinned() -> bool {
    PPO_CELL_ID == "W29-013-PPO"
        && PPO_MORPHISM_ID == "evaluate_topology_step"
        && PPO_FORMAL_MORPHISM_ID == "evaluate_topology_step_formal"
        && PPO_POSTURE_TAG == "honest-manifold-gateway-ssot-partial"
        && PPO_GATEWAY_SSOT_LANDED
        && PPO_CBF_GATE_LANDED
        && PPO_REWARD_WEIGHTS_PINNED
        && !PPO_PRODUCTION_WIRED
        && !PPO_PHYSICS_GREEN
        && !PPO_MASTER_RETICK
}

/// Validate PPO posture honesty — fail closed on fake production / GREEN / MASTER claims.
pub fn validate_ppo_posture_honesty() -> Result<(), &'static str> {
    let probe = ppo_posture_probe();
    if !ppo_morphism_pinned() {
        return Err("ppo_morphism_pinned failed");
    }
    if probe.posture_tag.to_ascii_lowercase().contains("green") {
        return Err("posture_tag must not claim green");
    }
    let lower_claim = probe.source_non_claim.to_ascii_lowercase();
    if lower_claim.contains("physics green") && probe.physics_green {
        return Err("source_non_claim must not pair with physics_green=true");
    }
    if !probe.honest_fence.contains("gateway_ssot_landed=true") {
        return Err("honest_fence missing gateway_ssot_landed=true");
    }
    if !probe.honest_fence.contains("cbf_gate_landed=true") {
        return Err("honest_fence missing cbf_gate_landed=true");
    }
    if !probe.honest_fence.contains("reward_weights_pinned=true") {
        return Err("honest_fence missing reward_weights_pinned=true");
    }
    if !probe.honest_fence.contains("production_wired=false") {
        return Err("honest_fence missing production_wired=false");
    }
    if !probe.honest_fence.contains("physics_green=false") {
        return Err("honest_fence missing physics_green=false");
    }
    if !probe.honest_fence.contains("master_retick=false") {
        return Err("honest_fence missing master_retick=false");
    }
    if probe.production_wired || probe.physics_green || probe.master_retick {
        return Err("honest booleans must stay false at W29 deepen tier");
    }
    if !probe.gateway_ssot_landed || !probe.cbf_gate_landed || !probe.reward_weights_pinned {
        return Err("landed facets must stay true at W29 deepen tier");
    }
    if PPO_PRODUCTION_FENCE_FACETS.len() != PPO_FENCE_FACET_COUNT {
        return Err("PPO_PRODUCTION_FENCE_FACETS length != PPO_FENCE_FACET_COUNT");
    }
    if ppo_fence_wired_count() != PPO_FENCE_WIRED_COUNT {
        return Err("ppo_fence_wired_count != PPO_FENCE_WIRED_COUNT");
    }
    if ppo_fence_open_count() != PPO_FENCE_FACET_COUNT - PPO_FENCE_WIRED_COUNT {
        return Err("ppo_fence_open_count mismatch vs facet census");
    }
    if ppo_fence_residue_facets().len() != ppo_fence_open_count() {
        return Err("residue facet inventory length != open count");
    }
    for facet in PPO_PRODUCTION_FENCE_FACETS.iter().filter(|f| !f.wired) {
        if !ppo_residue_owning_slice_deferred(facet.owning_slice) {
            return Err("open fence facet must defer owning_slice off W29-013-PPO");
        }
    }
    for residue in ppo_fence_residue_facets() {
        if residue.wired {
            return Err("residue inventory must only list unwired facets");
        }
        if !ppo_residue_owning_slice_deferred(residue.owning_slice) {
            return Err("residue owning_slice must stay deferred");
        }
        let matched = PPO_PRODUCTION_FENCE_FACETS.iter().any(|f| {
            f.facet == residue.facet && !f.wired && f.owning_slice == residue.owning_slice
        });
        if !matched {
            return Err("residue inventory drifted from PPO_PRODUCTION_FENCE_FACETS");
        }
    }
    if probe.fence_wired_count > probe.fence_facet_count {
        return Err("fence_wired_count exceeds fence_facet_count");
    }
    if !PPO_PROOF_CMD.contains("umst-manifold") || !PPO_PROOF_CMD.contains("ppo") {
        return Err("PPO_PROOF_CMD must stay scoped to umst-manifold ppo");
    }
    let defaults = ppo_default_reward_weights();
    if defaults.alpha != PPO_DEFAULT_ALPHA
        || defaults.beta != PPO_DEFAULT_BETA
        || defaults.gamma != PPO_DEFAULT_GAMMA
        || defaults.zeta != PPO_DEFAULT_ZETA
        || defaults.eta != PPO_DEFAULT_ETA
    {
        return Err("ppo_default_reward_weights drift vs PPO_DEFAULT_* pins");
    }
    Ok(())
}
#[cfg(feature = "thmc-coupled")]
use crate::ai::constraint_loss::scaled_constraint_violation_penalty;
#[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
use crate::ai::constraint_loss::scaled_landauer_slack_violation;
use crate::ai::formal::FormalReject;
use crate::core::error_boundary::ApplyPhysicsError;
use crate::core::traits::{IScienceCartridge, PhysicalResult};
use burn::tensor::{backend::Backend, Tensor};

/// The Gateway interface for Thermodynamic Topology Optimization.
///
/// It wraps physical cartridges and enforces the Thermodynamic CBF. As an **IO barrier**:
/// [`Self::evaluate_topology_step`] keeps spatial economics on the tensor graph and performs the
/// only required **host scalar** extractions for mutual-information bits and batch-summed `d_int`
/// inside [`ThermodynamicCBF::verify_tensor_update`](crate::ai::cbf::ThermodynamicCBF::verify_tensor_update)
/// (not in the cartridge’s Newton / CG inner loops).
pub struct ManifoldGateway<B: Backend, C: IScienceCartridge<B>> {
    pub cartridge: C,
    pub cbf: ThermodynamicCBF,
    /// Safety-margin reward weight **ζ** (dimensionless). When non-zero, the scalar reward adds
    /// `ζ * mean_voxels(safety_margin)` per batch row (positive ζ rewards higher structural margin).
    /// **Default 0** in [`Self::new`] — no effect on CBF admissibility checks or legacy reward.
    pub zeta: f32,
    /// Information-density reward weight **η** (dimensionless). With the **`information_density`**
    /// feature enabled, when non-zero the scalar reward adds `η * mean_voxels(information_density)` per
    /// batch row (same reduction pattern as [`Self::zeta`] on `safety_margin`).
    /// **Default 0** in [`Self::new`]; ignored when the feature is off (field is not compiled into
    /// [`PhysicalResult`]).
    pub eta: f32,
    /// Performance reward weight **α** (free-energy term). Default **1.0**.
    pub alpha: f32,
    /// Dissipation penalty weight **β**. Default **0.5**.
    pub beta: f32,
    /// Carbon / cost penalty weight **γ**. Default **2.0**.
    pub gamma: f32,
    /// Clausius–Duhem constraint slack weight **λ_cd** (epistemic / Kleisli hot-bind paths).
    /// When non-zero with **`epistemic-ppo`** or **`kleisli-ppo-hot-bind`**, [`Self::constraint_loss_penalty`]
    /// scales [`crate::ai::constraint_loss::clausius_duhem_violation`] for soft training penalties.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub lambda_cd: f32,
    /// Landauer erasure slack weight **λ_landauer** (same feature gate as [`Self::lambda_cd`]).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub lambda_landauer: f32,
    /// Optional catalog/schema digest asserted against the incoming UMST when **`formal-witness`** is on.
    /// [`Self::new`] defaults to compiled lock bytes; set `None` explicitly to skip the witness.
    #[cfg(feature = "formal-witness")]
    pub expected_catalog_schema_digest: Option<[u8; 32]>,
    /// Phase 4 exit witnesses — warm orchestrator only ([`crate::ai::rejection_telemetry::RejectionTelemetry`]).
    pub rejection_telemetry: crate::ai::rejection_telemetry::RejectionTelemetry,
    /// Kleisli penalize drain from [`crate::physics::solvers::ThmcSolver::drain_gate_evidence`].
    #[cfg(feature = "thmc-coupled")]
    pub thmc_gate_evidence: Vec<crate::physics::solvers::thmc_step::ThmcStepGateEvidence>,
    /// Mechanics solve witnesses from [`crate::physics::solvers::ThmcSolver::drain_mechanics_solve_reports`].
    #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
    pub mechanics_solve_reports: Vec<crate::solve_report::SolveReport>,
    _backend: std::marker::PhantomData<B>,
}

/// Map [`ApplyPhysicsError`] into [`FormalReject`] at the PPO gateway writeback boundary.
///
/// All variants surface as [`FormalReject::DecTypestateStaging`] with
/// [`ApplyPhysicsError`]'s [`Display`] text so legacy `evaluate_topology_step` string parity
/// is preserved (`catalog_id`: `umst.gate.dec_typestate`).
#[must_use]
fn formal_reject_from_apply_physics(err: ApplyPhysicsError) -> FormalReject {
    FormalReject::DecTypestateStaging {
        detail: err.to_string(),
    }
}

impl<B: Backend<FloatElem = f32>, C: IScienceCartridge<B>> ManifoldGateway<B, C> {
    pub fn new(cartridge: C, temperature_k: f64, initial_credit: f64) -> Self {
        Self {
            cartridge,
            cbf: ThermodynamicCBF::new(temperature_k, initial_credit),
            zeta: PPO_DEFAULT_ZETA,
            eta: PPO_DEFAULT_ETA,
            alpha: PPO_DEFAULT_ALPHA,
            beta: PPO_DEFAULT_BETA,
            gamma: PPO_DEFAULT_GAMMA,
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            lambda_cd: 0.0_f32,
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            lambda_landauer: 0.0_f32,
            #[cfg(feature = "formal-witness")]
            expected_catalog_schema_digest: Some(
                crate::runtime::catalog::lock_upstream_catalog_digest_bytes(),
            ),
            rejection_telemetry: crate::ai::rejection_telemetry::RejectionTelemetry::default(),
            #[cfg(feature = "thmc-coupled")]
            thmc_gate_evidence: Vec::new(),
            #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
            mechanics_solve_reports: Vec::new(),
            _backend: std::marker::PhantomData,
        }
    }

    /// Snapshot pinned α/β/γ/ζ/η reward weights (host shaping; not a GREEN attest).
    #[must_use]
    pub fn reward_weights(&self) -> PpoRewardWeights {
        PpoRewardWeights {
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
            zeta: self.zeta,
            eta: self.eta,
        }
    }

    /// Builder: replace α/β/γ/ζ/η from a typed snapshot (CBF gate unchanged).
    #[must_use]
    pub fn with_reward_weights(mut self, weights: PpoRewardWeights) -> Self {
        self.alpha = weights.alpha;
        self.beta = weights.beta;
        self.gamma = weights.gamma;
        self.zeta = weights.zeta;
        self.eta = weights.eta;
        self
    }

    /// Inject host-parsed constraint slack weights (see [`crate::runtime::ppo_host`]).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    #[must_use]
    pub fn with_constraint_weights(
        mut self,
        weights: crate::runtime::ppo_host::PpoConstraintWeights,
    ) -> Self {
        self.lambda_cd = weights.lambda_cd;
        self.lambda_landauer = weights.lambda_landauer;
        self
    }

    /// Apply [`crate::runtime::ppo_host::ppo_constraint_weights_from_env`] at the IO boundary.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    #[must_use]
    pub fn with_constraint_weights_from_env(self) -> Self {
        self.with_constraint_weights(crate::runtime::ppo_host::ppo_constraint_weights_from_env())
    }

    /// Phase 4 exit witness accessor (cold edge).
    #[must_use]
    pub fn telemetry(&self) -> &crate::ai::rejection_telemetry::RejectionTelemetry {
        &self.rejection_telemetry
    }

    /// Absorb THMC post-step gate evidence from solver drain into Kleisli penalize path.
    #[cfg(feature = "thmc-coupled")]
    pub fn absorb_thmc_gate_evidence(
        &mut self,
        batch: impl IntoIterator<Item = crate::physics::solvers::thmc_step::ThmcStepGateEvidence>,
    ) {
        use crate::runtime::gate::evidence::AdmissibilityToken;
        for evidence in batch {
            if evidence.transition.admissibility == AdmissibilityToken::Inadmissible {
                self.rejection_telemetry
                    .record_soft_penalty(f64::from(evidence.constraint.margin.violation()));
            } else {
                self.rejection_telemetry
                    .record_commit(f64::from(evidence.constraint.margin.violation()));
            }
            self.thmc_gate_evidence.push(evidence);
        }
    }

    /// Differentiable penalize morphism from drained THMC gate evidence → [`Self::constraint_loss_penalty`].
    #[cfg(feature = "thmc-coupled")]
    pub fn constraint_loss_penalty_from_gate_evidence(&self, device: &B::Device) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        let lambda_cd = {
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            {
                self.lambda_cd
            }
            #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
            {
                0.0_f32
            }
        };
        let batch = self.thmc_gate_evidence.len().max(1);
        let violations: Vec<f32> = self
            .thmc_gate_evidence
            .iter()
            .map(|e| e.constraint.margin.violation())
            .collect();
        let data = if violations.is_empty() {
            vec![0.0_f32]
        } else {
            violations
        };
        let violation_tensor = Tensor::<B, 1>::from_floats(data.as_slice(), device);
        let sized = if violation_tensor.dims()[0] == batch {
            violation_tensor
        } else {
            violation_tensor.reshape([batch])
        };
        scaled_constraint_violation_penalty(lambda_cd, sized)
    }

    /// Absorb mechanics [`crate::solve_report::SolveReport`] into warm telemetry (non-converged → soft penalty).
    #[cfg(all(feature = "thmc-coupled", feature = "mechanics-adjoint"))]
    pub fn absorb_mechanics_solve_reports(
        &mut self,
        reports: impl IntoIterator<Item = crate::solve_report::SolveReport>,
    ) {
        for report in reports {
            if report.converged() {
                self.rejection_telemetry
                    .record_commit(f64::from(report.rel_residual));
            } else {
                self.rejection_telemetry
                    .record_soft_penalty(f64::from(report.rel_residual));
            }
            self.mechanics_solve_reports.push(report);
        }
    }

    /// Drain accumulated THMC gate evidence after an episode rollout.
    #[cfg(feature = "thmc-coupled")]
    pub fn drain_thmc_gate_evidence(
        &mut self,
    ) -> Vec<crate::physics::solvers::thmc_step::ThmcStepGateEvidence> {
        std::mem::take(&mut self.thmc_gate_evidence)
    }

    /// Host-side CD slack from drained THMC evidence (inadmissible → unit slack per step).
    #[cfg(feature = "thmc-coupled")]
    pub fn thmc_evidence_penalty_hint(&self) -> f32 {
        use crate::runtime::gate::evidence::AdmissibilityToken;
        let inadmissible = self
            .thmc_gate_evidence
            .iter()
            .filter(|e| e.transition.admissibility == AdmissibilityToken::Inadmissible)
            .count();
        inadmissible as f32
    }

    /// Set η from catalog per-step MI scan ([`crate::ros::calibrate_eta_bound_from_trace`]; Track G.3).
    /// Uses `TraceCalibrationReport::eta_bound_suggested` (post-CBF reward weight only).
    #[cfg(feature = "trace-calibration")]
    pub fn calibrate_eta_from_trace(&mut self, schema: &crate::ros::EmittedTraceSchema) {
        let report = crate::ros::calibrate_eta_bound_from_trace(schema);
        self.eta = report.eta_bound_suggested.clamp(0.0, 1.0) as f32;
    }

    /// Set η from prototype rolled-up ε envelope ([`crate::ros::prototype_eta_from_trace`]; Track G.3).
    #[cfg(feature = "trace-calibration")]
    pub fn calibrate_eta_from_prototype_envelope(
        &mut self,
        schema: &crate::ros::EmittedTraceSchema,
    ) {
        self.eta = crate::ros::prototype_eta_from_trace(schema);
    }

    /// Optional Clausius–Duhem soft penalty (`λ_cd · relu(−margin)` per batch row).
    ///
    /// With penalize features disabled or [`Self::lambda_cd`] = 0, returns a zero `[B]` tensor.
    pub fn constraint_loss_penalty(
        &self,
        old_density: Tensor<B, 1>,
        new_density: Tensor<B, 1>,
        old_free_energy: Tensor<B, 1>,
        new_free_energy: Tensor<B, 1>,
        dt_s: Tensor<B, 1>,
    ) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        let lambda_cd = {
            #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
            {
                self.lambda_cd
            }
            #[cfg(not(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind")))]
            {
                0.0_f32
            }
        };
        scaled_clausius_duhem_violation(
            lambda_cd,
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
    }

    /// Optional Landauer erasure soft penalty (`λ_landauer · relu(k_B T ln2 · bits − credit)`).
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub fn landauer_constraint_loss_penalty(&self, info_gain_bits: Tensor<B, 1>) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        if self.lambda_landauer == 0.0_f32 {
            let batch = info_gain_bits.dims()[0];
            return Tensor::zeros([batch], &info_gain_bits.device());
        }
        scaled_landauer_slack_violation(
            self.lambda_landauer,
            info_gain_bits,
            self.cbf.temperature_k as f32,
            self.cbf.available_credit_joules as f32,
        )
    }

    /// Combined CD + Landauer soft penalties for Kleisli penalize hooks.
    #[cfg(any(feature = "epistemic-ppo", feature = "kleisli-ppo-hot-bind"))]
    pub fn total_constraint_loss_penalty(
        &self,
        old_density: Tensor<B, 1>,
        new_density: Tensor<B, 1>,
        old_free_energy: Tensor<B, 1>,
        new_free_energy: Tensor<B, 1>,
        dt_s: Tensor<B, 1>,
        info_gain_bits: Tensor<B, 1>,
    ) -> Tensor<B, 1>
    where
        B: Backend<FloatElem = f32>,
    {
        self.constraint_loss_penalty(
            old_density,
            new_density,
            old_free_energy,
            new_free_energy,
            dt_s,
        )
        .add(self.landauer_constraint_loss_penalty(info_gain_bits))
    }

    /// Evaluates a proposed topology state; errors are structured as [`FormalReject`].
    ///
    /// Prefer this when rejecting transitions must be classified (CBF vs catalog witness). The
    /// legacy [`Self::evaluate_topology_step`] API maps these cases to [`String`] via [`FormalReject`]'s [`Display`] impl.
    pub fn evaluate_topology_step_formal(
        &mut self,
        raw_state: crate::core::tensors::UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
    ) -> Result<
        (
            crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
            Tensor<B, 1>,
        ),
        FormalReject,
    > {
        #[cfg(feature = "formal-witness")]
        if let (Some(expected), Some(observed)) = (
            self.expected_catalog_schema_digest,
            raw_state.catalog_schema_digest,
        ) {
            if expected != observed {
                return Err(FormalReject::CatalogSchemaDigestMismatch { expected, observed });
            }
        }

        let staging = raw_state.try_as_verified_dec_bundle(0).map_err(|e| {
            FormalReject::DecTypestateStaging {
                detail: format!("{e:?}"),
            }
        })?;

        // 1. Execute the physics simulation across the topological Cellular Sheaf
        let physical_result: PhysicalResult<B> = self.cartridge.compute_topology(&raw_state);

        let mut secured_state = raw_state;
        crate::core::apply_physics::apply_physics_to_umst(&physical_result, &mut secured_state)
            .map_err(formal_reject_from_apply_physics)?;

        // Keep metrics in Sparse Space [Batch, N_active_voxels]
        let free_energy = physical_result.free_energy.clone();
        let dissipation = physical_result.dissipation.clone();
        let cost = physical_result.cost.clone();

        // 2. Validate against the Thermodynamic Control Barrier Function
        // Sum dissipation across all voxels for the CBF macro check
        let d_int = dissipation.clone().sum_dim(1).squeeze(1);

        match self.cbf.verify_tensor_update::<B>(d_int, info_gain.clone()) {
            Ok(erasure_cost) => {
                self.rejection_telemetry.record_commit(0.0);
                // Spatial Reward = (Alpha * Performance) - (Beta * Dissipation) - (Gamma * CO2) - Erasure Cost
                let performance = free_energy.mul_scalar(self.alpha);
                let penalty = dissipation
                    .mul_scalar(self.beta)
                    .add(cost.mul_scalar(self.gamma));

                // The erasure cost is paid uniformly across the topology
                let final_spatial_reward = performance.sub(penalty).sub_scalar(erasure_cost as f32);

                // Construct the mathematically secured tensor (staging → proof morphism).
                let verified_state = crate::core::tensors::VerifiedUMST::<
                    B,
                    crate::core::tensors::ClausiusDuhemProof,
                >::lift_after_dec_staging_witness(
                    staging, secured_state
                );

                // Flatten the spatial reward to a single scalar [Batch] for the policy gradient (Adjoint Method target)
                let mut total_reward = final_spatial_reward.sum_dim(1).squeeze(1);
                if self.zeta != 0.0_f32 {
                    let margin_mean = physical_result.safety_margin.clone().mean_dim(1).squeeze(1);
                    total_reward = total_reward.add(margin_mean.mul_scalar(self.zeta));
                }
                #[cfg(feature = "information_density")]
                if self.eta != 0.0_f32 {
                    let info_mean = physical_result
                        .information_density
                        .clone()
                        .mean_dim(1)
                        .squeeze(1);
                    total_reward = total_reward.add(info_mean.mul_scalar(self.eta));
                }

                Ok((verified_state, total_reward))
            }
            Err(reject) => {
                self.rejection_telemetry.record_reject();
                Err(FormalReject::ThermodynamicControlBarrier {
                    catalog_id: crate::ai::formal::LANDAUER_CBF_CATALOG_ID,
                    detail: reject.to_string(),
                })
            }
        }
    }

    /// Evaluates a proposed topology state.
    /// This runs the full Cartridge functor pass and gates the result through the CBF.
    ///
    /// # Arguments
    /// * `raw_state` - The proposed UMST Cellular Sheaf
    /// * `info_gain` - The calculated mutual information resolved by this step.
    ///
    /// # Returns
    /// * Ok(VerifiedUMST, Reward) - The mathematically secured state and the per-batch scalar reward
    ///   (spatial thermodynamic terms plus **ζ · mean(safety_margin)** when [`ManifoldGateway::zeta`] ≠ 0,
    ///   and with **`information_density`**, **η · mean(information_density)** when [`ManifoldGateway::eta`] ≠ 0).
    /// * Err(String) - If the state violates the Clausius-Duhem Thermodynamic gate (or, with **`formal-witness`**, the catalog digest witness).
    pub fn evaluate_topology_step(
        &mut self,
        raw_state: crate::core::tensors::UnifiedMaterialStateTensor<B>,
        info_gain: Tensor<B, 1>,
    ) -> Result<
        (
            crate::core::tensors::VerifiedUMST<B, crate::core::tensors::ClausiusDuhemProof>,
            Tensor<B, 1>,
        ),
        String,
    > {
        self.evaluate_topology_step_formal(raw_state, info_gain)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod apply_physics_formal_reject_tests {
    use super::formal_reject_from_apply_physics;
    use crate::ai::formal::FormalReject;
    use crate::core::dec_typestate::DecTypestateError;
    use crate::core::error_boundary::ApplyPhysicsError;

    #[test]
    fn dec_typestate_maps_to_dec_typestate_staging_with_display_detail() {
        let err = ApplyPhysicsError::DecTypestate {
            context: "invalid SCALAR_DAMAGE channel",
            source: DecTypestateError::ScalarChannelOutOfRange {
                index: 99,
                channel_count: 8,
            },
        };
        let rej = formal_reject_from_apply_physics(err.clone());
        assert_eq!(
            rej,
            FormalReject::DecTypestateStaging {
                detail: err.to_string(),
            }
        );
        assert_eq!(rej.catalog_id(), "umst.gate.dec_typestate");
    }

    #[test]
    fn damage_width_mismatch_preserves_display_parity() {
        let err = ApplyPhysicsError::DamageWidthMismatch {
            damage_width: 3,
            umst_nodes: 5,
        };
        let rej = formal_reject_from_apply_physics(err.clone());
        assert_eq!(
            rej,
            FormalReject::DecTypestateStaging {
                detail: err.to_string(),
            }
        );
        assert!(rej.to_string().contains("damage width 3 != UMST nodes 5"));
    }

    #[test]
    fn temperature_width_mismatch_preserves_display_parity() {
        let err = ApplyPhysicsError::TemperatureWidthMismatch {
            delta_width: 2,
            umst_nodes: 4,
        };
        let rej = formal_reject_from_apply_physics(err.clone());
        assert_eq!(rej.catalog_id(), "umst.gate.dec_typestate");
        assert!(rej.to_string().contains("temperature_delta width 2 != UMST nodes 4"));
    }
}

#[cfg(test)]
mod w29_ppo_deepen_tests {
    use super::*;
    use crate::ai::formal::FormalReject;
    use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
    use crate::core::traits::{IScienceCartridge, PhysicalResult};
    use crate::core::umst_schema::UMST_SCALAR_CHANNEL_COUNT;
    use approx::assert_relative_eq;
    use burn::tensor::{backend::Backend, Data, Int, Shape, Tensor};
    use burn_ndarray::{NdArray, NdArrayDevice};

    type B = NdArray<f32>;

    const GATEWAY_TEMP_K: f64 = 300.0;
    const GATEWAY_CREDIT_J: f64 = 1.0e-12;
    const SCALAR_DELTA: f32 = 0.1_f32;

    fn device() -> NdArrayDevice {
        NdArrayDevice::default()
    }

    fn tiny_umst() -> UnifiedMaterialStateTensor<B> {
        let dev = device();
        let n = 2usize;
        let f = UMST_SCALAR_CHANNEL_COUNT;
        let coords: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64; n * 5], Shape::new([n, 5])), &dev);
        let edges_b1: Tensor<B, 2, Int> = Tensor::from_data(
            Data::new(vec![0i64, 1i64, 1i64, 0i64], Shape::new([2, 2])),
            &dev,
        );
        let faces_b2: Tensor<B, 2, Int> =
            Tensor::from_data(Data::new(vec![0i64, 0i64], Shape::new([2, 1])), &dev);
        let scalar_features = Tensor::<B, 2>::zeros([n, f], &dev);
        let vector_features = Tensor::<B, 3>::zeros([n, 1, 3], &dev);
        let matrix_features = Tensor::<B, 4>::zeros([n, 1, 3, 3], &dev);
        UnifiedMaterialStateTensor {
            coords,
            edges_b1,
            faces_b2,
            scalar_features,
            vector_features,
            matrix_features,
            resolution_mm: [1.0, 1.0, 1.0],
            node_positions: None,
            displacement_bc_mask: Tensor::<B, 3>::ones([1, n, 3], &dev),
            policy_editable_mask: Tensor::<B, 2>::ones([n, 1], &dev),
            #[cfg(feature = "formal-witness")]
            catalog_schema_digest: None,
        }
    }

    struct PpoStubCartridge;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoStubCartridge {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::zeros([1, 1], &d),
                dissipation: Tensor::zeros([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::zeros([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, 1], &d),
            }
        }

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::zeros([1, n], &d),
                dissipation: Tensor::zeros([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::zeros([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    fn gateway() -> ManifoldGateway<B, PpoStubCartridge> {
        ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J)
    }

    fn info_gain_tensor() -> Tensor<B, 1> {
        Tensor::<B, 1>::full([1], SCALAR_DELTA, &device())
    }

    #[test]
    fn w29_ppo_posture_probe_honest_not_green() {
        let probe = ppo_posture_probe();
        assert_eq!(probe.cell_id, "W29-013-PPO");
        assert_eq!(probe.formal_morphism_id, PPO_FORMAL_MORPHISM_ID);
        assert!(probe.posture_tag.contains("honest"));
        assert!(!probe.posture_tag.to_ascii_lowercase().contains("green"));
        assert!(probe.gateway_ssot_landed);
        assert!(probe.cbf_gate_landed);
        assert!(probe.reward_weights_pinned);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master_retick);
        assert!(probe.source_non_claim.contains("not claimed"));
        assert_eq!(probe.fence_facet_count, PPO_FENCE_FACET_COUNT);
        assert_eq!(probe.fence_wired_count, PPO_FENCE_WIRED_COUNT);
    }

    #[test]
    fn w29_ppo_morphism_pinned() {
        assert!(ppo_morphism_pinned());
        assert_eq!(PPO_MORPHISM_ID, "evaluate_topology_step");
        assert_eq!(PPO_FORMAL_MORPHISM_ID, "evaluate_topology_step_formal");
        assert_eq!(
            PPO_HONEST_FENCE,
            "gateway_ssot_landed=true cbf_gate_landed=true reward_weights_pinned=true production_wired=false physics_green=false master_retick=false"
        );
    }

    #[test]
    fn w29_ppo_validate_posture_honesty_ok() {
        assert!(validate_ppo_posture_honesty().is_ok());
    }

    #[test]
    fn w29_ppo_fence_census_partial_not_production() {
        assert_eq!(PPO_PRODUCTION_FENCE_FACETS.len(), PPO_FENCE_FACET_COUNT);
        assert_eq!(ppo_fence_wired_count(), PPO_FENCE_WIRED_COUNT);
        assert_eq!(ppo_fence_open_count(), PPO_FENCE_FACET_COUNT - PPO_FENCE_WIRED_COUNT);
        assert!(PPO_FENCE_WIRED_COUNT < PPO_FENCE_FACET_COUNT);
        for facet in PPO_PRODUCTION_FENCE_FACETS {
            match facet.facet {
                "gateway_ssot" | "cbf_gate" | "reward_weights" => assert!(facet.wired),
                "production_wired" | "physics_green" | "master_retick" => assert!(!facet.wired),
                other => panic!("unexpected fence facet {other}"),
            }
        }
    }

    #[test]
    fn w29_ppo_default_reward_weights_pinned() {
        let gw = gateway();
        assert_relative_eq!(f64::from(gw.alpha), f64::from(PPO_DEFAULT_ALPHA), epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gw.beta), f64::from(PPO_DEFAULT_BETA), epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gw.gamma), f64::from(PPO_DEFAULT_GAMMA), epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gw.zeta), f64::from(PPO_DEFAULT_ZETA), epsilon = 1.0e-6);
        assert_relative_eq!(f64::from(gw.eta), f64::from(PPO_DEFAULT_ETA), epsilon = 1.0e-6);
    }

    #[test]
    fn w29_ppo_new_pins_temperature_and_credit_via_cbf() {
        let gw = gateway();
        assert_relative_eq!(gw.cbf.temperature_k, GATEWAY_TEMP_K, epsilon = 1.0e-9);
        assert_relative_eq!(
            gw.cbf.available_credit_joules,
            GATEWAY_CREDIT_J,
            epsilon = 1.0e-18
        );
    }

    #[test]
    fn w29_ppo_evaluate_topology_step_smoke_accepts_tiny_umst() {
        let mut gw = gateway();
        let state = tiny_umst();
        let info = info_gain_tensor();
        let out = gw.evaluate_topology_step(state, info);
        assert!(out.is_ok(), "expected Ok, got {:?}", out.err());
        let (verified, reward) = out.expect("topology step");
        assert_eq!(verified.state.scalar_features.dims()[0], 2);
        let reward_v: Vec<f32> = reward.into_data().value;
        assert_eq!(reward_v.len(), 1);
        assert!(reward_v[0].is_finite());
    }

    #[test]
    fn w29_ppo_topology_step_records_commit_telemetry() {
        let mut gw = gateway();
        let state = tiny_umst();
        let info = info_gain_tensor();
        gw.evaluate_topology_step(state, info)
            .expect("topology step");
        assert_eq!(gw.telemetry().rejection_rate(), 0.0);
        assert_relative_eq!(gw.telemetry().acceptance_rate(), 1.0, epsilon = 1.0e-9);
    }

    #[test]
    fn w29_ppo_topology_step_deducts_credit_monotonically() {
        let mut gw = gateway();
        let credit0 = gw.cbf.available_credit_joules;
        let state = tiny_umst();
        let info = info_gain_tensor();
        gw.evaluate_topology_step(state, info)
            .expect("first topology step");
        let credit1 = gw.cbf.available_credit_joules;
        assert!(
            credit1 <= credit0,
            "CBF credit must not increase after admissible step: {credit0} -> {credit1}"
        );
    }

    #[test]
    fn w29_ppo_constraint_loss_penalty_zero_when_disabled() {
        let gw = gateway();
        let dev = device();
        let batch = 1usize;
        let zeros = Tensor::<B, 1>::zeros([batch], &dev);
        let dt = Tensor::<B, 1>::full([batch], 1.0_f32, &dev);
        let penalty = gw.constraint_loss_penalty(
            zeros.clone(),
            zeros.clone(),
            zeros.clone(),
            zeros,
            dt,
        );
        let values: Vec<f32> = penalty.into_data().value;
        assert_eq!(values.len(), batch);
        assert_relative_eq!(f64::from(values[0]), 0.0, epsilon = 1.0e-30);
    }

    #[test]
    fn w29_ppo_formal_and_string_evaluate_parity() {
        let mut gw = gateway();
        let state_formal = tiny_umst();
        let state_string = tiny_umst();
        let info = info_gain_tensor();
        let formal = gw.evaluate_topology_step_formal(state_formal, info.clone());
        let string = gw.evaluate_topology_step(state_string, info);
        assert_eq!(formal.is_ok(), string.is_ok());
        if let (Ok((_, r_formal)), Ok((_, r_string))) = (formal, string) {
            let v_formal: Vec<f32> = r_formal.into_data().value;
            let v_string: Vec<f32> = r_string.into_data().value;
            assert_eq!(v_formal.len(), v_string.len());
            for (a, b) in v_formal.iter().zip(v_string.iter()) {
                assert_relative_eq!(f64::from(*a), f64::from(*b), epsilon = 1.0e-5);
            }
        }
    }

    #[test]
    fn w29_ppo_zeta_zero_preserves_zero_margin_contribution() {
        let mut gw = gateway();
        assert_relative_eq!(f64::from(gw.zeta), 0.0, epsilon = 1.0e-6);
        let state = tiny_umst();
        let info = info_gain_tensor();
        let reward_zeta0 = gw
            .evaluate_topology_step(state, info.clone())
            .expect("zeta=0 step")
            .1;
        gw.zeta = 0.5_f32;
        let state2 = tiny_umst();
        let reward_zeta_half = gw
            .evaluate_topology_step(state2, info)
            .expect("zeta=0.5 step")
            .1;
        let v0: Vec<f32> = reward_zeta0.into_data().value;
        let v1: Vec<f32> = reward_zeta_half.into_data().value;
        assert_eq!(v0.len(), 1);
        assert_eq!(v1.len(), 1);
        assert_relative_eq!(f64::from(v0[0]), f64::from(v1[0]), epsilon = 1.0e-5);
    }

    /// Stub with unit free-energy so α scaling is measurable on the reward path.
    struct PpoValuedCartridge;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoValuedCartridge {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::ones([1, 1], &d),
                dissipation: Tensor::zeros([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::zeros([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, 1], &d),
            }
        }

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::ones([1, n], &d),
                dissipation: Tensor::zeros([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::zeros([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    #[test]
    fn w29_ppo_alpha_scales_free_energy_reward() {
        let mut gw = ManifoldGateway::new(PpoValuedCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        let info = info_gain_tensor();
        let r_alpha1 = gw
            .evaluate_topology_step(tiny_umst(), info.clone())
            .expect("alpha=1 step")
            .1;
        let mut gw2 = ManifoldGateway::new(PpoValuedCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        gw2.alpha = 2.0_f32;
        let r_alpha2 = gw2
            .evaluate_topology_step(tiny_umst(), info)
            .expect("alpha=2 step")
            .1;
        let v1: Vec<f32> = r_alpha1.into_data().value;
        let v2: Vec<f32> = r_alpha2.into_data().value;
        assert_eq!(v1.len(), 1);
        assert_eq!(v2.len(), 1);
        // Free-energy ones → spatial sum scales with α; erasure is identical scalar debit.
        // Δreward = (α2 − α1) * n_voxels = 1.0 * 2 = 2.0
        assert_relative_eq!(
            f64::from(v2[0] - v1[0]),
            2.0,
            epsilon = 1.0e-4
        );
    }

    #[test]
    fn w29_ppo_cbf_reject_records_telemetry_and_formal_barrier() {
        // Near-zero credit → Landauer floor rejects; gateway must not invent admit.
        let mut gw = ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, 1.0e-30_f64);
        let info = Tensor::<B, 1>::full([1], 1.0_f32, &device());
        let err = match gw.evaluate_topology_step_formal(tiny_umst(), info) {
            Err(e) => e,
            Ok(_) => panic!("insufficient credit must reject"),
        };
        match err {
            FormalReject::ThermodynamicControlBarrier { catalog_id, detail } => {
                assert_eq!(catalog_id, crate::ai::formal::LANDAUER_CBF_CATALOG_ID);
                assert!(!detail.is_empty());
            }
            other => panic!("expected ThermodynamicControlBarrier, got {other:?}"),
        }
        assert!(gw.telemetry().rejection_rate() > 0.0);
        assert_relative_eq!(gw.telemetry().acceptance_rate(), 0.0, epsilon = 1.0e-9);
    }

    #[test]
    fn w29_ppo_source_non_claim_refuses_green_language_as_claim() {
        assert!(PPO_SOURCE_NON_CLAIM.contains("not claimed"));
        assert!(!PPO_PHYSICS_GREEN);
        assert!(!PPO_PRODUCTION_WIRED);
        assert!(!PPO_MASTER_RETICK);
    }

    #[test]
    fn w29_ppo_fence_residue_deferred_off_cell() {
        assert_eq!(ppo_fence_open_count(), 3);
        assert_eq!(ppo_fence_residue_facets().len(), 3);
        for facet in ppo_fence_residue_facets() {
            assert!(!facet.wired);
            assert!(ppo_residue_owning_slice_deferred(facet.owning_slice));
        }
        assert!(!ppo_residue_owning_slice_deferred(PPO_CELL_ID));
        assert!(!ppo_residue_owning_slice_deferred(""));
    }

    #[test]
    fn w29_ppo_reward_weights_builder_roundtrip() {
        let defaults = ppo_default_reward_weights();
        assert_eq!(defaults, PpoRewardWeights::defaults());
        let gw: ManifoldGateway<B, PpoStubCartridge> =
            ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        assert_eq!(gw.reward_weights(), defaults);
        let shaped = PpoRewardWeights {
            alpha: 1.5,
            beta: 0.25,
            gamma: 3.0,
            zeta: 0.1,
            eta: 0.0,
        };
        let gw2: ManifoldGateway<B, PpoStubCartridge> =
            ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J)
                .with_reward_weights(shaped);
        assert_eq!(gw2.reward_weights(), shaped);
        // Builder does not invent production / GREEN / MASTER.
        assert!(!PPO_PRODUCTION_WIRED);
        assert!(!PPO_PHYSICS_GREEN);
        assert!(!PPO_MASTER_RETICK);
    }

    #[test]
    fn w29_ppo_proof_cmd_allowlisted_scope() {
        assert_eq!(
            PPO_PROOF_CMD,
            "bash outputs/.tmp/fleet_away_rustc188.sh --check && cargo test -p umst-manifold ppo"
        );
        assert!(PPO_PROOF_CMD.contains("fleet_away_rustc188"));
        assert!(PPO_PROOF_CMD.contains("-p umst-manifold"));
    }

    /// Stub with unit dissipation so β scaling is measurable on the reward path.
    struct PpoDissipationCartridge;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoDissipationCartridge {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::zeros([1, 1], &d),
                dissipation: Tensor::ones([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::zeros([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, 1], &d),
            }
        }

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::zeros([1, n], &d),
                dissipation: Tensor::ones([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::zeros([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    #[test]
    fn w29_ppo_beta_scales_dissipation_penalty() {
        let mut gw = ManifoldGateway::new(PpoDissipationCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        gw.beta = 1.0_f32;
        let info = info_gain_tensor();
        let r_b1 = gw
            .evaluate_topology_step(tiny_umst(), info.clone())
            .expect("beta=1 step")
            .1;
        let mut gw2 = ManifoldGateway::new(PpoDissipationCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        gw2.beta = 2.0_f32;
        let r_b2 = gw2
            .evaluate_topology_step(tiny_umst(), info)
            .expect("beta=2 step")
            .1;
        let v1: Vec<f32> = r_b1.into_data().value;
        let v2: Vec<f32> = r_b2.into_data().value;
        // Higher β → more penalty → lower reward; Δ = −(β2−β1)*n_voxels = −2.
        assert_relative_eq!(f64::from(v2[0] - v1[0]), -2.0, epsilon = 1.0e-4);
    }

    /// Stub with unit cost so γ scaling is measurable on the reward path.
    struct PpoCostCartridge;

    impl<Bk: Backend<FloatElem = f32>> IScienceCartridge<Bk> for PpoCostCartridge {
        fn compute_all(&self, mix: &MaterialCompositionTensor<Bk>) -> PhysicalResult<Bk> {
            let d = mix.fractions.device();
            PhysicalResult {
                free_energy: Tensor::zeros([1, 1], &d),
                dissipation: Tensor::zeros([1, 1], &d),
                safety_margin: Tensor::zeros([1, 1], &d),
                cost: Tensor::ones([1, 1], &d),
                damage: Tensor::zeros([1, 1], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, 1], &d),
            }
        }

        fn compute_topology(&self, m: &UnifiedMaterialStateTensor<Bk>) -> PhysicalResult<Bk> {
            let d = m.scalar_features.device();
            let n = m.scalar_features.dims()[0];
            PhysicalResult {
                free_energy: Tensor::zeros([1, n], &d),
                dissipation: Tensor::zeros([1, n], &d),
                safety_margin: Tensor::zeros([1, n], &d),
                cost: Tensor::ones([1, n], &d),
                damage: Tensor::zeros([1, n], &d),
                temperature_delta: None,
                #[cfg(feature = "information_density")]
                information_density: Tensor::zeros([1, n], &d),
            }
        }
    }

    #[test]
    fn w29_ppo_gamma_scales_cost_penalty() {
        let mut gw = ManifoldGateway::new(PpoCostCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        gw.gamma = 1.0_f32;
        let info = info_gain_tensor();
        let r_g1 = gw
            .evaluate_topology_step(tiny_umst(), info.clone())
            .expect("gamma=1 step")
            .1;
        let mut gw2 = ManifoldGateway::new(PpoCostCartridge, GATEWAY_TEMP_K, GATEWAY_CREDIT_J);
        gw2.gamma = 3.0_f32;
        let r_g3 = gw2
            .evaluate_topology_step(tiny_umst(), info)
            .expect("gamma=3 step")
            .1;
        let v1: Vec<f32> = r_g1.into_data().value;
        let v3: Vec<f32> = r_g3.into_data().value;
        // Δ = −(γ3−γ1)*n_voxels = −4.
        assert_relative_eq!(f64::from(v3[0] - v1[0]), -4.0, epsilon = 1.0e-4);
    }

    #[test]
    fn w29_ppo_cbf_reject_string_api_display_parity() {
        let mut gw = ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, 1.0e-30_f64);
        let info = Tensor::<B, 1>::full([1], 1.0_f32, &device());
        let formal_err = match gw.evaluate_topology_step_formal(tiny_umst(), info.clone()) {
            Err(e) => e,
            Ok(_) => panic!("formal reject"),
        };
        let mut gw2 = ManifoldGateway::new(PpoStubCartridge, GATEWAY_TEMP_K, 1.0e-30_f64);
        let string_err = match gw2.evaluate_topology_step(tiny_umst(), info) {
            Err(e) => e,
            Ok(_) => panic!("string reject"),
        };
        assert_eq!(string_err, formal_err.to_string());
        assert!(matches!(
            formal_err,
            FormalReject::ThermodynamicControlBarrier { .. }
        ));
    }
}

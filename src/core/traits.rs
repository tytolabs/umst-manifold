// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Cartridge traits and thermodynamic summaries (`fp-categorical-v04` / `fp-v04-traits-category`).
//!
//! # Categorical vocabulary (design sketch)
//!
//! - **Objects:** [`crate::core::tensors::MaterialCompositionTensor`] (homogeneous bulk) and
//!   [`crate::core::tensors::UnifiedMaterialStateTensor`] (topology-carrying UMST) are the primary
//!   *state carriers* solvers and cartridges reason about.
//! - **Morphisms:** [`IScienceCartridge`] is the stable **material-law port**—two evaluation heads
//!   (`compute_all`, `compute_topology`) from those objects into [`PhysicalResult`]. Orchestrated
//!   graph stepping lives in [`crate::physics::orchestration`] and [`crate::physics::solvers`], not
//!   in this trait (cartridge stays a functor *into* thermodynamic summaries).
//! - **Second law at the interface:** [`PhysicalResult`] exposes `free_energy`, `dissipation`, and
//!   related sparse fields so merge, CBF, and RL paths can audit **dissipative consistency** as a
//!   policy invariant; constitutive closures must populate those tensors consistently with
//!   their numerical schemes.
//!
//! Longer note (objects / solvers / composition table): `docs/Category-of-Material-Updates.md`.
//!
//! # Honest fences (W29-031)
//!
//! Trait contracts here are **T1 lattice ports** — they define admissible boundaries without
//! claiming fleet-wide physics GREEN, production wiring, or MASTER certification. Use
//! [`trait_port_status`], [`missing_trait_ports`], and [`traits_deepen_probe`] for gap inventory;
//! do not invent readiness.
//!
//! **Name collision:** [`GateCartridge`] (spatial-marker, Phase B) is **not**
//! [`crate::runtime::gate::GateCartridge`] (Clausius–Duhem transition witness).

use crate::core::tensors::{MaterialCompositionTensor, UnifiedMaterialStateTensor};
use burn::tensor::{backend::Backend, Tensor};

/// W29 deepen wave step — trait lattice honesty (no invent GREEN).
pub const TRAITS_W29_WAVE_STEP: &str = "W29-031-TRAITS";

/// Depth tier — T1 trait lattice (port definitions only).
pub const DEPTH_TIER: &str = "T1";

/// Honest posture — contracts live; constitutive coverage varies by cartridge crate.
pub const POSTURE_TAG: &str = "PARTIAL";

/// Operator-visible honesty string — does **not** authorize GREEN / production / MASTER flip.
pub const HONEST_FENCE: &str =
    "trait_lattice=T1 science_port=wired spatial_marker=unwired production_wired=false physics_green=false master=false";

/// Honest refusal — trait surface is not end-to-end production-wired.
pub const PRODUCTION_WIRED: bool = false;

/// Honest refusal — no blanket physics GREEN at the trait boundary.
pub const PHYSICS_GREEN_CLAIMED: bool = false;

/// Honest refusal — no MASTER certification fence at trait lattice.
pub const MASTER_CERT_CLAIMED: bool = false;

/// Whether [`IScienceCartridge`] has measured in-tree consumers (PPO / orchestrator / harvest).
pub const SCIENCE_CARTRIDGE_CONSUMERS_MEASURED: bool = true;

/// Whether [`DesignRepresentation`] has a measured decode consumer path.
pub const DESIGN_REPR_CONSUMERS_MEASURED: bool = true;

/// Whether Phase-B [`SpatialCartridge`] has a measured subtype consumer (honest: no).
pub const SPATIAL_CARTRIDGE_CONSUMERS_MEASURED: bool = false;

/// Whether Phase-B [`GateCartridge`] spatial marker is fully wired beyond the default stub.
pub const GATE_SPATIAL_FULLY_WIRED: bool = false;

/// Runtime namesake for disambiguation audits (transition-evidence cartridge).
pub const GATE_CARTRIDGE_RUNTIME_NAMESAKE: &str = "runtime::gate::GateCartridge";

/// Expected rank-2 channel layout for sparse nodal [`PhysicalResult`] tensors.
pub const PHYSICAL_RESULT_RANK: usize = 2;

/// Count of always-present [`PhysicalResult`] tensor fields (excludes optional / feature-gated).
pub const PHYSICAL_RESULT_CORE_FIELD_COUNT: usize = 5;

/// Optional [`PhysicalResult`] fields that may be absent (`temperature_delta`, feature-gated density).
pub const PHYSICAL_RESULT_OPTIONAL_FIELD_SLOTS: usize = 2;

/// Measured method count on [`IScienceCartridge`] (`compute_all`, `compute_topology`).
pub const ISCIENCE_CARTRIDGE_METHOD_COUNT: usize = 2;

/// Measured method count on [`DesignRepresentation`] (`repr_id`, `decode`).
pub const DESIGN_REPR_METHOD_COUNT: usize = 2;

/// Compile-time count of [`TraitPort::ALL`] inventory slots.
pub const TRAIT_PORT_INVENTORY_LEN: usize = 5;

/// Expected wired ports at T1 (science + design + physical_result) — not a GREEN claim.
pub const TRAIT_PORT_WIRED_EXPECTED: usize = 3;

/// Expected partial ports at T1 (gate spatial marker stub only).
pub const TRAIT_PORT_PARTIAL_EXPECTED: usize = 1;

/// Expected unwired ports at T1 (Phase-B spatial cartridge marker).
pub const TRAIT_PORT_UNWIRED_EXPECTED: usize = 1;

/// Core trait ports on the manifold lattice (honest wire inventory).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitPort {
    ScienceCartridge,
    GateCartridgeSpatial,
    SpatialCartridgeMarker,
    DesignRepresentation,
    PhysicalResult,
}

impl TraitPort {
    /// All inventory ports in stable audit order.
    pub const ALL: [Self; 5] = [
        Self::ScienceCartridge,
        Self::GateCartridgeSpatial,
        Self::SpatialCartridgeMarker,
        Self::DesignRepresentation,
        Self::PhysicalResult,
    ];

    /// Stable audit label for witness transcripts.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScienceCartridge => "science_cartridge",
            Self::GateCartridgeSpatial => "gate_cartridge_spatial",
            Self::SpatialCartridgeMarker => "spatial_cartridge_marker",
            Self::DesignRepresentation => "design_representation",
            Self::PhysicalResult => "physical_result",
        }
    }

    /// Compile-time wire status for this port.
    #[must_use]
    pub const fn wire_status(self) -> TraitWireStatus {
        trait_port_status(self)
    }

    /// Whether this port still counts as a deepen gap (partial or unwired).
    #[must_use]
    pub const fn is_gap(self) -> bool {
        !matches!(self.wire_status(), TraitWireStatus::Wired)
    }
}

/// Measured wiring posture for a [`TraitPort`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitWireStatus {
    /// In-tree impl(s) and measured consumer path(s).
    Wired,
    /// Port defined; stub or partial wiring only.
    Partial,
    /// Port defined; no measured consumer yet.
    Unwired,
}

impl TraitWireStatus {
    /// Stable audit label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wired => "wired",
            Self::Partial => "partial",
            Self::Unwired => "unwired",
        }
    }
}

/// Honest per-port wiring snapshot (compile-time inventory; not a runtime probe).
#[must_use]
pub const fn trait_port_status(port: TraitPort) -> TraitWireStatus {
    match port {
        TraitPort::ScienceCartridge => TraitWireStatus::Wired,
        TraitPort::GateCartridgeSpatial => TraitWireStatus::Partial,
        TraitPort::SpatialCartridgeMarker => TraitWireStatus::Unwired,
        TraitPort::DesignRepresentation => TraitWireStatus::Wired,
        TraitPort::PhysicalResult => TraitWireStatus::Wired,
    }
}

/// Count of ports still partial or unwired (const-friendly deepen census).
#[must_use]
pub const fn missing_trait_port_count() -> usize {
    let mut n = 0usize;
    let mut i = 0usize;
    while i < TraitPort::ALL.len() {
        if TraitPort::ALL[i].is_gap() {
            n += 1;
        }
        i += 1;
    }
    n
}

/// Partitioned wire census over [`TraitPort::ALL`] (compile-time; not a runtime probe).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraitWireCensus {
    pub wired: usize,
    pub partial: usize,
    pub unwired: usize,
}

impl TraitWireCensus {
    /// Total inventory slots (must equal [`TRAIT_PORT_INVENTORY_LEN`]).
    #[must_use]
    pub const fn total(self) -> usize {
        self.wired + self.partial + self.unwired
    }

    /// Gaps = partial + unwired (deepen residual).
    #[must_use]
    pub const fn gaps(self) -> usize {
        self.partial + self.unwired
    }
}

/// Honest wire-status partition — refuses inventing full wiring.
#[must_use]
pub const fn trait_wire_census() -> TraitWireCensus {
    let mut wired = 0usize;
    let mut partial = 0usize;
    let mut unwired = 0usize;
    let mut i = 0usize;
    while i < TraitPort::ALL.len() {
        match TraitPort::ALL[i].wire_status() {
            TraitWireStatus::Wired => wired += 1,
            TraitWireStatus::Partial => partial += 1,
            TraitWireStatus::Unwired => unwired += 1,
        }
        i += 1;
    }
    TraitWireCensus {
        wired,
        partial,
        unwired,
    }
}

/// Compile-time fence — production / GREEN / MASTER flips not authorized at T1.
const _: () = assert!(!PRODUCTION_WIRED);
const _: () = assert!(!PHYSICS_GREEN_CLAIMED);
const _: () = assert!(!MASTER_CERT_CLAIMED);
const _: () = assert!(missing_trait_port_count() == 2);
const _: () = assert!(PHYSICAL_RESULT_RANK == 2);
const _: () = assert!(PHYSICAL_RESULT_CORE_FIELD_COUNT == 5);
const _: () = assert!(TraitPort::ALL.len() == TRAIT_PORT_INVENTORY_LEN);
const _: () = assert!(trait_wire_census().total() == TRAIT_PORT_INVENTORY_LEN);
const _: () = assert!(trait_wire_census().wired == TRAIT_PORT_WIRED_EXPECTED);
const _: () = assert!(trait_wire_census().partial == TRAIT_PORT_PARTIAL_EXPECTED);
const _: () = assert!(trait_wire_census().unwired == TRAIT_PORT_UNWIRED_EXPECTED);
const _: () = assert!(trait_wire_census().gaps() == missing_trait_port_count());
const _: () = assert!(ISCIENCE_CARTRIDGE_METHOD_COUNT == 2);
const _: () = assert!(DESIGN_REPR_METHOD_COUNT == 2);
const _: () = assert!(PHYSICAL_RESULT_OPTIONAL_FIELD_SLOTS == 2);

/// Honest posture probe — partial contracts only; refuses GREEN / production / MASTER.
#[must_use]
pub const fn traits_posture_is_honest_partial() -> bool {
    !PHYSICS_GREEN_CLAIMED && !PRODUCTION_WIRED && !MASTER_CERT_CLAIMED
}

/// Returns ports still partial or unwired — honest gap inventory for orchestration.
#[must_use]
pub fn missing_trait_ports() -> Vec<TraitPort> {
    TraitPort::ALL.into_iter().filter(|p| p.is_gap()).collect()
}

/// W29-031 deepen census — lattice ports land; production / GREEN / MASTER blocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitsDeepenProbe {
    pub wave_step: &'static str,
    pub depth_tier: &'static str,
    pub posture_tag: &'static str,
    pub honest_fence: &'static str,
    pub science_cartridge_consumers_measured: bool,
    pub design_repr_consumers_measured: bool,
    pub spatial_cartridge_consumers_measured: bool,
    pub gate_spatial_fully_wired: bool,
    pub missing_port_count: usize,
    pub wired_port_count: usize,
    pub partial_port_count: usize,
    pub unwired_port_count: usize,
    pub inventory_len: usize,
    pub iscience_method_count: usize,
    pub design_repr_method_count: usize,
    pub physical_result_rank: usize,
    pub physical_result_core_field_count: usize,
    pub physical_result_optional_field_slots: usize,
    pub production_wired: bool,
    pub physics_green_claimed: bool,
    pub master_cert_claimed: bool,
    pub gate_runtime_namesake: &'static str,
}

/// Honest deepen probe — surfaces wired prep without inventing GREEN.
#[must_use]
pub const fn traits_deepen_probe() -> TraitsDeepenProbe {
    let census = trait_wire_census();
    TraitsDeepenProbe {
        wave_step: TRAITS_W29_WAVE_STEP,
        depth_tier: DEPTH_TIER,
        posture_tag: POSTURE_TAG,
        honest_fence: HONEST_FENCE,
        science_cartridge_consumers_measured: SCIENCE_CARTRIDGE_CONSUMERS_MEASURED,
        design_repr_consumers_measured: DESIGN_REPR_CONSUMERS_MEASURED,
        spatial_cartridge_consumers_measured: SPATIAL_CARTRIDGE_CONSUMERS_MEASURED,
        gate_spatial_fully_wired: GATE_SPATIAL_FULLY_WIRED,
        missing_port_count: missing_trait_port_count(),
        wired_port_count: census.wired,
        partial_port_count: census.partial,
        unwired_port_count: census.unwired,
        inventory_len: TRAIT_PORT_INVENTORY_LEN,
        iscience_method_count: ISCIENCE_CARTRIDGE_METHOD_COUNT,
        design_repr_method_count: DESIGN_REPR_METHOD_COUNT,
        physical_result_rank: PHYSICAL_RESULT_RANK,
        physical_result_core_field_count: PHYSICAL_RESULT_CORE_FIELD_COUNT,
        physical_result_optional_field_slots: PHYSICAL_RESULT_OPTIONAL_FIELD_SLOTS,
        production_wired: PRODUCTION_WIRED,
        physics_green_claimed: PHYSICS_GREEN_CLAIMED,
        master_cert_claimed: MASTER_CERT_CLAIMED,
        gate_runtime_namesake: GATE_CARTRIDGE_RUNTIME_NAMESAKE,
    }
}

/// Honesty gate for operator receipts — T1 lattice only, no production / GREEN / MASTER flip.
#[must_use]
pub fn traits_deepen_honest(probe: &TraitsDeepenProbe) -> bool {
    let census = trait_wire_census();
    probe.wave_step == TRAITS_W29_WAVE_STEP
        && probe.depth_tier == DEPTH_TIER
        && probe.posture_tag == POSTURE_TAG
        && probe.honest_fence == HONEST_FENCE
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("spatial_marker=unwired")
        && probe.science_cartridge_consumers_measured
        && probe.design_repr_consumers_measured
        && !probe.spatial_cartridge_consumers_measured
        && !probe.gate_spatial_fully_wired
        && probe.missing_port_count == missing_trait_port_count()
        && probe.missing_port_count == 2
        && probe.wired_port_count == census.wired
        && probe.partial_port_count == census.partial
        && probe.unwired_port_count == census.unwired
        && probe.wired_port_count == TRAIT_PORT_WIRED_EXPECTED
        && probe.partial_port_count == TRAIT_PORT_PARTIAL_EXPECTED
        && probe.unwired_port_count == TRAIT_PORT_UNWIRED_EXPECTED
        && probe.inventory_len == TRAIT_PORT_INVENTORY_LEN
        && probe.inventory_len == census.total()
        && probe.missing_port_count == census.gaps()
        && probe.iscience_method_count == ISCIENCE_CARTRIDGE_METHOD_COUNT
        && probe.design_repr_method_count == DESIGN_REPR_METHOD_COUNT
        && probe.physical_result_rank == PHYSICAL_RESULT_RANK
        && probe.physical_result_core_field_count == PHYSICAL_RESULT_CORE_FIELD_COUNT
        && probe.physical_result_optional_field_slots == PHYSICAL_RESULT_OPTIONAL_FIELD_SLOTS
        && !probe.production_wired
        && !probe.physics_green_claimed
        && !probe.master_cert_claimed
        && probe.gate_runtime_namesake.contains("runtime::gate")
}

/// Validate trait-lattice posture honesty — fail closed on fake GREEN / production claims.
pub fn validate_traits_posture_honesty() -> Result<(), &'static str> {
    let probe = traits_deepen_probe();
    if probe.production_wired {
        return Err("PRODUCTION_WIRED must stay false at T1 trait lattice");
    }
    if probe.physics_green_claimed {
        return Err("PHYSICS_GREEN_CLAIMED must stay false — no invent GREEN");
    }
    if probe.master_cert_claimed {
        return Err("MASTER_CERT_CLAIMED must stay false until fleet sign-off");
    }
    if probe.spatial_cartridge_consumers_measured {
        return Err("SPATIAL_CARTRIDGE_CONSUMERS_MEASURED must stay false until Phase B measures");
    }
    if probe.gate_spatial_fully_wired {
        return Err("GATE_SPATIAL_FULLY_WIRED must stay false while marker is Partial");
    }
    if probe.wired_port_count + probe.partial_port_count + probe.unwired_port_count
        != probe.inventory_len
    {
        return Err("trait wire census must partition inventory");
    }
    if probe.wired_port_count == probe.inventory_len {
        return Err("full wire census would invent production readiness — refuse");
    }
    if !traits_posture_is_honest_partial() {
        return Err("traits_posture_is_honest_partial failed");
    }
    if !traits_deepen_honest(&probe) {
        return Err("traits_deepen_honest failed");
    }
    Ok(())
}

/// The unified thermodynamic return type expected by the Orchestrator and the CBF.
/// Kept in Sparse Space [Batch, N_active_voxels] so the agent can compute topology gradients directly.
///
/// Consumed by [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`](crate::ai::ppo::ManifoldGateway::evaluate_topology_step)
/// (reward + CBF wiring): spatial terms use `free_energy`, `dissipation`, and `cost`; the per-batch
/// scalar reward optionally adds **ζ · mean(safety_margin)** when [`crate::ai::ppo::ManifoldGateway::zeta`]
/// is non-zero. With the **`information_density`** crate feature, the same scalar reward optionally adds
/// **η · mean(information_density)** when [`crate::ai::ppo::ManifoldGateway::eta`] is non-zero (defaults
/// preserve legacy behavior). Merged into UMST state via [`crate::core::apply_physics::apply_physics_to_umst`]
/// for damage and optional temperature.
pub struct PhysicalResult<B: Backend> {
    pub free_energy: Tensor<B, 2>,
    pub dissipation: Tensor<B, 2>,
    pub safety_margin: Tensor<B, 2>,
    pub cost: Tensor<B, 2>,
    pub damage: Tensor<B, 2>,
    pub temperature_delta: Option<Tensor<B, 2>>,
    /// Per-voxel information-density signal at shape `[Batch, N_active_voxels]`.
    ///
    /// Only present with the **`information_density`** feature. When present, it participates in the
    /// scalar reward only if [`crate::ai::ppo::ManifoldGateway::eta`] is non-zero (see struct-level docs).
    #[cfg(feature = "information_density")]
    pub information_density: Tensor<B, 2>,
}

/// Material-law port: bulk and topology evaluation into [`PhysicalResult`] (no THMC stepping here).
pub trait IScienceCartridge<B: Backend> {
    /// Standard homogeneous forward pass (0D/1D). Evaluates the bulk material.
    fn compute_all(&self, mix: &MaterialCompositionTensor<B>) -> PhysicalResult<B>;

    /// Multi-agent heterogeneous topology pass.
    /// The cartridge computes physics using the Cellular Sheaf topology (Discrete Exterior Calculus).
    /// Shape of returned tensors: [Batch, N_active_voxels]
    fn compute_topology(&self, manifold: &UnifiedMaterialStateTensor<B>) -> PhysicalResult<B>;
}

/// Universal gate port (Phase B) — spatial-physics capability marker.
///
/// **Not** [`crate::runtime::gate::GateCartridge`] — that trait witnesses transition admissibility.
/// See [`GATE_CARTRIDGE_RUNTIME_NAMESAKE`].
pub trait GateCartridge {
    fn provides_spatial_physics(&self) -> bool {
        true
    }
}

/// Spatial physics port (Phase B subtyping marker).
pub trait SpatialCartridge<B: Backend>: IScienceCartridge<B> {}

// --- R4 DesignRepresentation port (agent-facing refactor) ---

/// Learnable or checkpointed design parameters (latent code or flattened logits).
#[derive(Clone, Debug)]
pub struct DesignLatent<B: Backend> {
    pub tensor: Tensor<B, 2>,
}

/// Decoded geometry on the active DEC graph — input to SIMP / UMST projection.
#[derive(Clone, Debug)]
pub struct Geometry<B: Backend> {
    /// Nodal density ρ ∈ (0,1), shape `[B, N, 1]`.
    pub density: Tensor<B, 3>,
    /// Optional signed distance φ at nodes (implicit path).
    pub signed_distance: Option<Tensor<B, 3>>,
    /// Query coordinates used for decode, `[B, N, 3]`.
    pub coords: Tensor<B, 3>,
}

/// Decode failures on the hot design path (total — no panic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignDecodeError {
    ShapeMismatch,
    NonFinite,
}

impl DesignDecodeError {
    /// Stable reject label for witness transcripts.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShapeMismatch => "design_decode_shape_mismatch",
            Self::NonFinite => "design_decode_non_finite",
        }
    }
}

/// Design geometry decode port — orthogonal to [`IScienceCartridge`] material law.
pub trait DesignRepresentation<B: Backend> {
    fn repr_id(&self) -> &'static str;

    /// Decode latent → geometry. Must be pure (no IO).
    fn decode(
        &self,
        latent: &DesignLatent<B>,
        query_coords: Tensor<B, 3>,
    ) -> Result<Geometry<B>, DesignDecodeError>;
}

#[cfg(test)]
mod traits_tests {
    use super::*;

    #[test]
    fn traits_honest_posture_refuses_green_production_master() {
        assert!(traits_posture_is_honest_partial());
        assert!(!PHYSICS_GREEN_CLAIMED);
        assert!(!PRODUCTION_WIRED);
        assert!(!MASTER_CERT_CLAIMED);
        assert_eq!(POSTURE_TAG, "PARTIAL");
        assert_eq!(DEPTH_TIER, "T1");
        assert_eq!(TRAITS_W29_WAVE_STEP, "W29-031-TRAITS");
    }

    #[test]
    fn traits_port_inventory_honest_gaps() {
        assert_eq!(
            trait_port_status(TraitPort::ScienceCartridge),
            TraitWireStatus::Wired
        );
        assert_eq!(
            trait_port_status(TraitPort::GateCartridgeSpatial),
            TraitWireStatus::Partial
        );
        assert_eq!(
            trait_port_status(TraitPort::SpatialCartridgeMarker),
            TraitWireStatus::Unwired
        );
        assert_eq!(missing_trait_port_count(), 2);
        let gaps = missing_trait_ports();
        assert_eq!(gaps.len(), 2);
        assert!(gaps.contains(&TraitPort::GateCartridgeSpatial));
        assert!(gaps.contains(&TraitPort::SpatialCartridgeMarker));
        assert!(TraitPort::GateCartridgeSpatial.is_gap());
        assert!(TraitPort::SpatialCartridgeMarker.is_gap());
        assert!(!TraitPort::ScienceCartridge.is_gap());
    }

    #[test]
    fn traits_port_labels_stable() {
        assert_eq!(TraitPort::ScienceCartridge.as_str(), "science_cartridge");
        assert_eq!(
            TraitPort::GateCartridgeSpatial.as_str(),
            "gate_cartridge_spatial"
        );
        assert_eq!(
            TraitPort::SpatialCartridgeMarker.as_str(),
            "spatial_cartridge_marker"
        );
        assert_eq!(
            TraitPort::DesignRepresentation.as_str(),
            "design_representation"
        );
        assert_eq!(TraitPort::PhysicalResult.as_str(), "physical_result");
        assert_eq!(TraitWireStatus::Wired.as_str(), "wired");
        assert_eq!(TraitWireStatus::Partial.as_str(), "partial");
        assert_eq!(TraitWireStatus::Unwired.as_str(), "unwired");
        assert_eq!(TraitPort::ALL.len(), 5);
    }

    #[test]
    fn traits_deepen_probe_honest_fence() {
        let probe = traits_deepen_probe();
        assert!(traits_deepen_honest(&probe));
        assert_eq!(probe.wave_step, TRAITS_W29_WAVE_STEP);
        assert_eq!(probe.honest_fence, HONEST_FENCE);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master=false"));
        assert!(probe.science_cartridge_consumers_measured);
        assert!(!probe.spatial_cartridge_consumers_measured);
        assert!(!probe.gate_spatial_fully_wired);
        assert_eq!(probe.missing_port_count, 2);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green_claimed);
        assert!(!probe.master_cert_claimed);
        assert!(validate_traits_posture_honesty().is_ok());
    }

    #[test]
    fn traits_gate_cartridge_default_claims_spatial() {
        struct DefaultGate;
        impl GateCartridge for DefaultGate {}
        assert!(DefaultGate.provides_spatial_physics());
        // Default stub ≠ fully wired Phase B spatial consumer.
        assert!(!GATE_SPATIAL_FULLY_WIRED);
    }

    #[test]
    fn traits_design_decode_error_labels_stable() {
        assert_eq!(
            DesignDecodeError::ShapeMismatch.as_str(),
            "design_decode_shape_mismatch"
        );
        assert_eq!(
            DesignDecodeError::NonFinite.as_str(),
            "design_decode_non_finite"
        );
    }

    #[test]
    fn traits_physical_result_rank_contract() {
        assert_eq!(PHYSICAL_RESULT_RANK, 2);
        assert_eq!(PHYSICAL_RESULT_CORE_FIELD_COUNT, 5);
        assert_eq!(PHYSICAL_RESULT_OPTIONAL_FIELD_SLOTS, 2);
    }

    #[test]
    fn traits_wire_census_partitions_inventory() {
        let census = trait_wire_census();
        assert_eq!(census.total(), TRAIT_PORT_INVENTORY_LEN);
        assert_eq!(census.wired, TRAIT_PORT_WIRED_EXPECTED);
        assert_eq!(census.partial, TRAIT_PORT_PARTIAL_EXPECTED);
        assert_eq!(census.unwired, TRAIT_PORT_UNWIRED_EXPECTED);
        assert_eq!(census.gaps(), missing_trait_port_count());
        assert_eq!(TraitPort::ALL.len(), TRAIT_PORT_INVENTORY_LEN);
        // Full wiring would invent readiness — census must keep gaps.
        assert!(census.gaps() > 0);
        assert_ne!(census.wired, census.total());
    }

    #[test]
    fn traits_method_counts_measured() {
        assert_eq!(ISCIENCE_CARTRIDGE_METHOD_COUNT, 2);
        assert_eq!(DESIGN_REPR_METHOD_COUNT, 2);
        let probe = traits_deepen_probe();
        assert_eq!(probe.iscience_method_count, 2);
        assert_eq!(probe.design_repr_method_count, 2);
        assert_eq!(probe.physical_result_optional_field_slots, 2);
        assert_eq!(probe.wired_port_count, 3);
        assert_eq!(probe.partial_port_count, 1);
        assert_eq!(probe.unwired_port_count, 1);
        assert_eq!(probe.inventory_len, 5);
    }

    #[test]
    fn traits_runtime_gate_namesake_documented() {
        assert!(GATE_CARTRIDGE_RUNTIME_NAMESAKE.contains("runtime::gate"));
        assert_eq!(
            traits_deepen_probe().gate_runtime_namesake,
            GATE_CARTRIDGE_RUNTIME_NAMESAKE
        );
    }

    #[test]
    fn traits_honest_fence_blocks_green_invent() {
        assert!(!HONEST_FENCE.to_ascii_lowercase().contains("green=true"));
        assert!(!POSTURE_TAG.to_ascii_lowercase().contains("green"));
        assert!(!POSTURE_TAG.to_ascii_lowercase().contains("master"));
        assert!(!PRODUCTION_WIRED);
        assert!(!PHYSICS_GREEN_CLAIMED);
        assert!(!MASTER_CERT_CLAIMED);
    }

    #[test]
    fn traits_port_wire_status_roundtrip() {
        for port in TraitPort::ALL {
            assert_eq!(port.wire_status(), trait_port_status(port));
            assert_eq!(
                port.is_gap(),
                !matches!(port.wire_status(), TraitWireStatus::Wired)
            );
        }
    }
}

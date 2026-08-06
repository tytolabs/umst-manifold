// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! UMST deployment manifest: catalog fingerprint, thermodynamic defaults, and grounding contract.
//!
//! ## Dev vs release grounding (R5 v1)
//!
//! | Profile | [`GroundingContract`] | `formal-witness` | Runtime digest witness |
//! |---------|----------------------|------------------|------------------------|
//! | **Default** ([`UmstManifestBuilder::default`]) | [`StrictCatalogMatch`] when [`UmstManifestBuilder::release_manifest_profile_enabled`] (CI default `UMST_RELEASE_MANIFEST_PROFILE=1`); else staging in debug | **off** | Skipped without feature |
//! | **Staging** ([`UmstManifestBuilder::for_staging`]) | [`GroundingContract::CatalogPinnedRos2`] | optional | Advisory pin; set `UMST_RELEASE_MANIFEST_PROFILE=0` for debug staging default |
//! | **Release / god-grade** | [`StrictCatalogMatch`] + `--features formal-witness` | **on** | `ManifoldGateway::new` and [`UmstManifest::apply_witness_to_gateway`] pin lock digest; mismatch → [`FormalReject::CatalogSchemaDigestMismatch`](crate::ai::formal::FormalReject::CatalogSchemaDigestMismatch) |
//!
//! Default manifest policy is fail-closed at the contract level. Enable the **release triple**
//! documented in [`VERIFY.md`](../../docs/VERIFY.md) §3.3 for runtime digest reject:
//! `formal-witness` + [`GroundingContract::StrictCatalogMatch`] + downstream `manifest-bridge`
//! (cartridge repo).
//!
//! # Honest boundary (W29-043)
//!
//! [`UmstManifest`] / [`UmstManifestBuilder`] are **deployment SSOT scaffolds** — catalog pin,
//! thermodynamic defaults, and grounding vocabulary. Not physics GREEN, not `PRODUCTION_WIRED`,
//! not `MASTER`. Cartridge `manifest-bridge` and fleet orchestration remain deferred slices.

/// W29 deepen cell — manifest honest fence bundle.
pub const W29_MANIFEST_DEEPEN_CELL: &str = "W29-043-UMST_MANIFEST";

/// Honest posture tag — builder + catalog pin landed; production/master refused.
pub const MANIFEST_POSTURE_TAG: &str = "honest-manifest-ssot-only";

/// Cartridge manifest-bridge integration deferred beyond in-repo builder.
pub const MANIFEST_BRIDGE_DEFERRED_STEP: &str = "manifest-bridge-cartridge";

/// Fleet production orchestration pin deferred beyond manifest SSOT.
pub const MANIFEST_PRODUCTION_ORCH_DEFERRED_STEP: &str = "W4-JG-6-loop-close";

/// Honest physics posture — manifest pins catalog identity; does not certify continuum physics.
pub const MANIFEST_PHYSICS_GREEN: bool = false;

/// Production deployment wiring — not claimed by manifest module alone.
pub const MANIFEST_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by manifest module.
pub const MANIFEST_MASTER: bool = false;

/// Whether builder + catalog lock pin API is landed.
pub const MANIFEST_BUILDER_LANDED: bool = true;

/// Whether catalog lock bundle SHA-256 pin is landed.
pub const MANIFEST_CATALOG_PIN_LANDED: bool = true;

/// Whether gate-registry declaration hook is landed (telemetry only — no gate execution).
pub const MANIFEST_GATE_REGISTRY_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const MANIFEST_HONEST_FENCE: &str =
    "manifest_builder_landed=true catalog_pin_landed=true production_wired=false master_composition_wired=false";

/// Manifest fence facet count (honest census).
pub const MANIFEST_FENCE_FACET_COUNT: usize = 8;

/// Manifest fence facets wired today (5/8 measured; formal-witness feature-gated).
pub const MANIFEST_FENCE_WIRED_COUNT: usize = 5;

/// Stable facet ids for manifest production fence census.
pub const MANIFEST_FENCE_FACET_IDS: &[&str] = &[
    "catalog_lock_pin",
    "builder_thermo_defaults",
    "grounding_contract_vocab",
    "gate_registry_declarations",
    "strict_profile_env_gate",
    "formal_witness_digest",
    "manifest_bridge_cartridge",
    "production_wired",
];

/// One facet of the manifest production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Manifest production fence facet inventory (honest posture SSOT).
pub const MANIFEST_PRODUCTION_FENCE_FACETS: &[ManifestProductionFenceFacet] = &[
    ManifestProductionFenceFacet {
        facet: "catalog_lock_pin",
        wired: true,
        owning_slice: W29_MANIFEST_DEEPEN_CELL,
    },
    ManifestProductionFenceFacet {
        facet: "builder_thermo_defaults",
        wired: true,
        owning_slice: W29_MANIFEST_DEEPEN_CELL,
    },
    ManifestProductionFenceFacet {
        facet: "grounding_contract_vocab",
        wired: true,
        owning_slice: W29_MANIFEST_DEEPEN_CELL,
    },
    ManifestProductionFenceFacet {
        facet: "gate_registry_declarations",
        wired: true,
        owning_slice: W29_MANIFEST_DEEPEN_CELL,
    },
    ManifestProductionFenceFacet {
        facet: "strict_profile_env_gate",
        wired: true,
        owning_slice: W29_MANIFEST_DEEPEN_CELL,
    },
    ManifestProductionFenceFacet {
        facet: "formal_witness_digest",
        wired: false,
        owning_slice: "formal-witness-feature",
    },
    ManifestProductionFenceFacet {
        facet: "manifest_bridge_cartridge",
        wired: false,
        owning_slice: MANIFEST_BRIDGE_DEFERRED_STEP,
    },
    ManifestProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: MANIFEST_PRODUCTION_ORCH_DEFERRED_STEP,
    },
];

/// Count wired manifest fence facets (must match [`MANIFEST_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn manifest_fence_wired_count() -> usize {
    MANIFEST_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Honest production wiring — **false** until cartridge bridge + fleet orch measured.
#[must_use]
pub const fn manifest_production_wired() -> bool {
    false
}

/// Master composition wiring — **false** until W4-JG-6 loop-close.
#[must_use]
pub const fn manifest_master_composition_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!manifest_production_wired());

/// Measured honest-posture snapshot for manifest (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub builder_landed: bool,
    pub catalog_pin_landed: bool,
    pub gate_registry_landed: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
    pub deferred_manifest_bridge: &'static str,
    pub deferred_production_orch: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn manifest_honest_posture_bundle() -> ManifestHonestPosture {
    ManifestHonestPosture {
        physics_green: MANIFEST_PHYSICS_GREEN,
        production_wired: MANIFEST_PRODUCTION_WIRED,
        master: MANIFEST_MASTER,
        builder_landed: MANIFEST_BUILDER_LANDED,
        catalog_pin_landed: MANIFEST_CATALOG_PIN_LANDED,
        gate_registry_landed: MANIFEST_GATE_REGISTRY_LANDED,
        fence_facet_count: MANIFEST_FENCE_FACET_COUNT,
        fence_wired_count: manifest_fence_wired_count(),
        honest_fence: MANIFEST_HONEST_FENCE,
        deferred_manifest_bridge: MANIFEST_BRIDGE_DEFERRED_STEP,
        deferred_production_orch: MANIFEST_PRODUCTION_ORCH_DEFERRED_STEP,
    }
}

/// Typed probe for manifest posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub builder_landed: bool,
    pub catalog_pin_landed: bool,
    pub gate_registry_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub honest_fence: &'static str,
}

/// Build introspection probe for manifest done-when / fleet checks.
#[must_use]
pub const fn manifest_posture_probe() -> ManifestPostureProbe {
    ManifestPostureProbe {
        cell_id: W29_MANIFEST_DEEPEN_CELL,
        posture_tag: MANIFEST_POSTURE_TAG,
        builder_landed: MANIFEST_BUILDER_LANDED,
        catalog_pin_landed: MANIFEST_CATALOG_PIN_LANDED,
        gate_registry_landed: MANIFEST_GATE_REGISTRY_LANDED,
        production_wired: manifest_production_wired(),
        master_composition_wired: manifest_master_composition_wired(),
        physics_green: MANIFEST_PHYSICS_GREEN,
        fence_facet_count: MANIFEST_FENCE_FACET_COUNT,
        fence_wired_count: MANIFEST_FENCE_WIRED_COUNT,
        honest_fence: MANIFEST_HONEST_FENCE,
    }
}

/// Manifest SSOT landed with production/master composition honestly open.
#[must_use]
pub fn manifest_posture_honest(probe: &ManifestPostureProbe) -> bool {
    probe.cell_id == W29_MANIFEST_DEEPEN_CELL
        && probe.posture_tag == MANIFEST_POSTURE_TAG
        && probe.builder_landed
        && probe.catalog_pin_landed
        && probe.gate_registry_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_composition_wired
        && probe.fence_facet_count == MANIFEST_FENCE_FACET_COUNT
        && probe.fence_wired_count == MANIFEST_FENCE_WIRED_COUNT
        && probe.honest_fence.contains("manifest_builder_landed=true")
        && probe.honest_fence.contains("catalog_pin_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("master_composition_wired=false")
}

/// Validate manifest posture honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_manifest_posture_honesty() -> Result<(), &'static str> {
    let probe = manifest_posture_probe();
    if probe.physics_green {
        return Err("MANIFEST_PHYSICS_GREEN must stay false — manifest is SSOT pin only");
    }
    if probe.production_wired {
        return Err("manifest_production_wired must stay false until manifest-bridge lands");
    }
    if probe.master_composition_wired {
        return Err("manifest_master_composition_wired must stay false until W4-JG-6 closes");
    }
    if manifest_fence_wired_count() != MANIFEST_FENCE_WIRED_COUNT {
        return Err("manifest_fence_wired_count drift from MANIFEST_FENCE_WIRED_COUNT");
    }
    if !manifest_posture_honest(&probe) {
        return Err("manifest_posture_honest failed");
    }
    Ok(())
}

use crate::ai::cbf::ThermodynamicCBF;
use crate::gate::{KleisliUnitEvaluator, ThermodynamicTransitionEvaluator};
use crate::runtime::catalog::{
    catalog_lock_bundle_sha256_bytes, catalog_lock_bundle_sha256_hex,
    lock_upstream_catalog_digest_bytes,
    traceability::{
        CD_TRANSITION_CATALOG_ID, GATE_REGISTRY_CATALOG_IDS, GATE_UNIFICATION_SPEC_CATALOG_IDS,
        HTTP_SHIM_CATALOG_ID, LANDAUER_CBF_CATALOG_ID, THERMODYNAMIC_MIX_CATALOG_ID,
    },
    WitnessPriorityQueue,
};

/// Documentation-only enum describing **how** manifold state and ROS topics are pinned to a catalog
/// revision and admissibility boundary.
///
/// Variants are not enforced at runtime here; they exist so deploy manifests and ROS bridges can
/// share a stable vocabulary for reviews and CI matrices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroundingContract {
    /// Pin all tensor layouts and solver lanes to the catalog identified by [`UmstManifest::catalog_hash`];
    /// ROS payloads must echo the same hash (see [`crate::ros::contract`] when `ros2-contract` is enabled).
    CatalogPinnedRos2,
    /// **Release default (documented):** hard-fail when a proposal's embedded catalog fingerprint
    /// differs from [`UmstManifest::catalog_hash`]. Pair with crate feature **`formal-witness`** and
    /// `ManifoldGateway::expected_catalog_schema_digest = Some(lock bytes)` for the v1 digest witness
    /// ([`FormalReject::CatalogSchemaDigestMismatch`](crate::ai::formal::FormalReject::CatalogSchemaDigestMismatch)).
    /// **Release default** (`not(debug_assertions)`); dev/debug builds keep [`CatalogPinnedRos2`].
    /// Explicit opt-in anytime via [`UmstManifestBuilder::for_release_witness`].
    StrictCatalogMatch,
    /// Research / staging: catalog hash advisory only (logged, not hard-failing). Do not use in
    /// release manifests when god-grade R5 is required.
    AdvisoryCatalogOnly,
}

/// Placeholder registry for named admissibility gates. This does **not** execute gate logic; it is a
/// manifest-owned hook for telemetry / auditing (`GateUnificationSpec.md` registry-first flow).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRegistry {
    /// Logical gate lanes declared for telemetry / auditing (empty by default).
    pub declared_lanes: Vec<String>,
}

impl Default for GateRegistry {
    fn default() -> Self {
        Self::default_for_unified_catalog()
    }
}

impl GateRegistry {
    /// Declared lanes for the **119-module** unified catalog pin (classical + double-slit).
    ///
    /// Includes registry evaluators and spec slugs used by host/tensor paths. See
    /// [`docs/DUAL_PIN_ARCHITECTURE.md`](../../docs/DUAL_PIN_ARCHITECTURE.md) for per-fiber digests.
    #[must_use]
    pub fn default_for_unified_catalog() -> Self {
        let mut lanes: Vec<String> = GATE_REGISTRY_CATALOG_IDS
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        for id in GATE_UNIFICATION_SPEC_CATALOG_IDS {
            let s = (*id).to_string();
            if !lanes.contains(&s) {
                lanes.push(s);
            }
        }
        lanes.sort();
        lanes.dedup();
        Self {
            declared_lanes: lanes,
        }
    }

    /// Classical formal anchors (strength closure, constitutional Kleisli, DEC) for manifest reviews.
    #[must_use]
    pub fn classical_formal_lane_ids() -> [&'static str; 5] {
        [
            CD_TRANSITION_CATALOG_ID,
            KleisliUnitEvaluator::CATALOG_ID,
            THERMODYNAMIC_MIX_CATALOG_ID,
            HTTP_SHIM_CATALOG_ID,
            LANDAUER_CBF_CATALOG_ID,
        ]
    }
}

/// Frozen UMST deployment bundle: catalog identity, thermodynamic defaults, gate-registry hook, and default transition evaluator.
#[derive(Debug, Clone)]
pub struct UmstManifest {
    /// Blake3 / SHA-256 / toolchain-defined **32-byte** catalog fingerprint (caller-defined semantics).
    pub catalog_hash: [u8; 32],
    pub thermodynamic_cbf: ThermodynamicCBF,
    pub gate_registry: Box<GateRegistry>,
    pub grounding_contract: GroundingContract,
    /// When `true`, policy loops run **both** the host transition gate and CBF checks ([`crate::gate`] +
    /// [`crate::ai::cbf::ThermodynamicCBF`]); see **`docs/GateUnificationSpec.md`** dual-run mode.
    pub dual_run: bool,
    pub default_transition_gate: ThermodynamicTransitionEvaluator,
    /// Optional adaptive module scheduler (tests / coverage scaffolding — not on inference hot path).
    pub witness_priority_queue: Option<WitnessPriorityQueue>,
}

impl UmstManifest {
    /// Hex fingerprint of the verbatim **`artifacts/catalog.lock.json`** bundle (`UMST_CATALOG_LOCK_SHA256_HEX`).
    #[must_use]
    pub fn compiled_catalog_lock_bundle_sha256_hex() -> &'static str {
        catalog_lock_bundle_sha256_hex()
    }

    /// Whether this manifest opts into fail-closed catalog digest witness (R5 v1 release profile).
    #[must_use]
    pub fn is_strict_catalog_grounding(&self) -> bool {
        self.grounding_contract == GroundingContract::StrictCatalogMatch
    }

    /// Digest asserted on gateway and UMST when **`formal-witness`** + [`GroundingContract::StrictCatalogMatch`].
    ///
    /// Returns `None` for dev defaults ([`GroundingContract::CatalogPinnedRos2`]) so witness stays skipped.
    #[cfg(feature = "formal-witness")]
    #[must_use]
    pub fn witness_catalog_digest(&self) -> Option<[u8; 32]> {
        if self.is_strict_catalog_grounding() {
            Some(self.catalog_hash)
        } else {
            None
        }
    }

    /// Wire [`crate::ai::ppo::ManifoldGateway::expected_catalog_schema_digest`] from this manifest.
    #[cfg(feature = "formal-witness")]
    pub fn apply_witness_to_gateway<
        B: burn::tensor::backend::Backend,
        C: crate::core::traits::IScienceCartridge<B>,
    >(
        &self,
        gateway: &mut crate::ai::ppo::ManifoldGateway<B, C>,
    ) {
        gateway.expected_catalog_schema_digest = self.witness_catalog_digest();
    }
}

impl Default for UmstManifest {
    fn default() -> Self {
        UmstManifestBuilder::default().build()
    }
}

/// Builder for [`UmstManifest`] with conservative thermodynamic defaults aligned with gateway smoke tests
/// (`300` K, `1e-12` J initial Landauer budget — see crate integration tests).
#[derive(Debug, Clone)]
pub struct UmstManifestBuilder {
    catalog_hash: [u8; 32],
    thermodynamic_cbf: ThermodynamicCBF,
    gate_registry: Box<GateRegistry>,
    grounding_contract: GroundingContract,
    dual_run: bool,
    default_transition_gate: ThermodynamicTransitionEvaluator,
    witness_priority_queue: Option<WitnessPriorityQueue>,
}

/// Dev/staging grounding unless [`UmstManifestBuilder::release_manifest_profile_enabled`].
fn default_grounding_contract() -> GroundingContract {
    if cfg!(debug_assertions) && !UmstManifestBuilder::release_manifest_profile_enabled() {
        GroundingContract::CatalogPinnedRos2
    } else {
        GroundingContract::StrictCatalogMatch
    }
}

impl Default for UmstManifestBuilder {
    fn default() -> Self {
        Self {
            catalog_hash: catalog_lock_bundle_sha256_bytes(),
            thermodynamic_cbf: ThermodynamicCBF::new(300.0_f64, 1.0e-12_f64),
            gate_registry: Box::new(GateRegistry::default()),
            grounding_contract: default_grounding_contract(),
            dual_run: false,
            default_transition_gate: ThermodynamicTransitionEvaluator::new(),
            witness_priority_queue: None,
        }
    }
}

impl UmstManifestBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn catalog_hash(mut self, hash: [u8; 32]) -> Self {
        self.catalog_hash = hash;
        self
    }

    #[must_use]
    pub fn thermodynamic_cbf(mut self, cbf: ThermodynamicCBF) -> Self {
        self.thermodynamic_cbf = cbf;
        self
    }

    #[must_use]
    pub fn gate_registry(mut self, registry: Box<GateRegistry>) -> Self {
        self.gate_registry = registry;
        self
    }

    #[must_use]
    pub fn grounding_contract(mut self, contract: GroundingContract) -> Self {
        self.grounding_contract = contract;
        self
    }

    #[must_use]
    pub fn dual_run(mut self, enabled: bool) -> Self {
        self.dual_run = enabled;
        self
    }

    #[must_use]
    pub fn default_transition_gate(mut self, gate: ThermodynamicTransitionEvaluator) -> Self {
        self.default_transition_gate = gate;
        self
    }

    /// Staging / dev iteration: [`GroundingContract::CatalogPinnedRos2`] (advisory ROS pin).
    #[must_use]
    pub fn for_staging(mut self) -> Self {
        self.grounding_contract = GroundingContract::CatalogPinnedRos2;
        self
    }

    /// Release / god-grade profile: [`GroundingContract::StrictCatalogMatch`] + lock-pinned digest.
    ///
    /// Pair with `--features formal-witness` and [`UmstManifest::apply_witness_to_gateway`] (or
    /// [`EmbodiedOrchestrator::from_manifest`]) in CI. Debug [`Default`] keeps [`CatalogPinnedRos2`];
    /// release binaries default strict via [`default_grounding_contract`].
    #[must_use]
    pub fn for_release_witness(mut self) -> Self {
        self.catalog_hash = lock_upstream_catalog_digest_bytes();
        self.grounding_contract = GroundingContract::StrictCatalogMatch;
        self
    }

    /// Whether CI / ops treat the release manifest lane as strict (matches `verify_umst_stack.sh`).
    #[must_use]
    pub fn release_manifest_profile_enabled() -> bool {
        std::env::var("UMST_RELEASE_MANIFEST_PROFILE")
            .map(|v| v != "0")
            .unwrap_or(true)
    }

    /// CI-aligned release entry: strict witness when [`Self::release_manifest_profile_enabled`].
    #[must_use]
    pub fn for_release_profile(self) -> Self {
        if Self::release_manifest_profile_enabled() {
            self.for_release_witness()
        } else {
            self
        }
    }

    /// R0 composed digest from `artifacts/catalog.lock.json` (`upstream_catalog_digest_hex`).
    ///
    /// Auto-filled on strict builds and [`ManifoldGateway`] / UMST witness paths (Track **G-05**).
    #[cfg(feature = "formal-witness")]
    #[must_use]
    pub fn lock_catalog_schema_digest_bytes() -> [u8; 32] {
        lock_upstream_catalog_digest_bytes()
    }

    /// Attach an adaptive witness priority queue (CI / adaptive-coverage tests only).
    #[must_use]
    pub fn witness_priority_queue(mut self, queue: WitnessPriorityQueue) -> Self {
        self.witness_priority_queue = Some(queue);
        self
    }

    #[must_use]
    pub fn build(self) -> UmstManifest {
        let catalog_hash = if self.grounding_contract == GroundingContract::StrictCatalogMatch {
            lock_upstream_catalog_digest_bytes()
        } else {
            self.catalog_hash
        };
        UmstManifest {
            catalog_hash,
            thermodynamic_cbf: self.thermodynamic_cbf,
            gate_registry: self.gate_registry,
            grounding_contract: self.grounding_contract,
            dual_run: self.dual_run,
            default_transition_gate: self.default_transition_gate,
            witness_priority_queue: self.witness_priority_queue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::catalog::{
        catalog_lock_bundle_sha256_bytes, lock_upstream_catalog_digest_bytes,
    };

    #[test]
    fn default_builder_catalog_hash_pins_lock_fields() {
        let manifest = UmstManifestBuilder::default().build();
        if manifest.grounding_contract == GroundingContract::StrictCatalogMatch {
            assert_eq!(
                manifest.catalog_hash,
                lock_upstream_catalog_digest_bytes(),
                "strict default must auto-fill upstream_catalog_digest_hex from lock"
            );
        } else {
            assert_eq!(
                manifest.catalog_hash,
                catalog_lock_bundle_sha256_bytes(),
                "staging default pins verbatim lock-bundle SHA-256"
            );
        }
    }

    #[test]
    fn default_manifest_catalog_hash_pins_lock_fields() {
        let manifest = UmstManifest::default();
        if manifest.is_strict_catalog_grounding() {
            assert_eq!(manifest.catalog_hash, lock_upstream_catalog_digest_bytes());
        } else {
            assert_eq!(manifest.catalog_hash, catalog_lock_bundle_sha256_bytes());
        }
    }

    #[test]
    fn default_gate_registry_declares_unified_catalog_lanes() {
        let manifest = UmstManifestBuilder::default().build();
        let lanes = &manifest.gate_registry.declared_lanes;
        assert!(
            lanes.contains(&"umst.gate.cd_transition".to_string()),
            "manifest must declare CD lane for classical DEC/Gate alignment"
        );
        assert!(
            lanes.contains(&"umst.gate.kleisli_unit".to_string()),
            "manifest must declare Kleisli lane (Constitutional / DIBKleisli)"
        );
        assert!(
            lanes.contains(&"umst.formal.catalog_lock".to_string()),
            "manifest must declare catalog_lock for formal-witness path"
        );
        assert_eq!(
            lanes.len(),
            GateRegistry::default_for_unified_catalog()
                .declared_lanes
                .len()
        );
    }

    #[test]
    fn default_builder_grounding_follows_release_manifest_profile() {
        let manifest = UmstManifestBuilder::default().build();
        if UmstManifestBuilder::release_manifest_profile_enabled() {
            assert_eq!(
                manifest.grounding_contract,
                GroundingContract::StrictCatalogMatch,
                "UMST_RELEASE_MANIFEST_PROFILE=1 (default) must use StrictCatalogMatch even in debug"
            );
        } else if cfg!(debug_assertions) {
            assert_eq!(
                manifest.grounding_contract,
                GroundingContract::CatalogPinnedRos2,
                "explicit UMST_RELEASE_MANIFEST_PROFILE=0 keeps staging default in debug"
            );
        } else {
            assert_eq!(
                manifest.grounding_contract,
                GroundingContract::StrictCatalogMatch
            );
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn release_default_builder_is_strict_catalog_match() {
        let manifest = UmstManifestBuilder::default().build();
        assert_eq!(
            manifest.grounding_contract,
            GroundingContract::StrictCatalogMatch
        );
        assert_eq!(manifest.catalog_hash, lock_upstream_catalog_digest_bytes());
    }

    #[test]
    fn strict_build_auto_fills_catalog_hash_from_lock_even_if_builder_hash_wrong() {
        use crate::runtime::catalog::lock_upstream_catalog_digest_bytes;
        let mut wrong = lock_upstream_catalog_digest_bytes();
        wrong[0] ^= 0xff;
        let manifest = UmstManifestBuilder::default()
            .catalog_hash(wrong)
            .grounding_contract(GroundingContract::StrictCatalogMatch)
            .build();
        assert_eq!(manifest.catalog_hash, lock_upstream_catalog_digest_bytes());
    }

    #[cfg(feature = "formal-witness")]
    #[test]
    fn lock_catalog_schema_digest_bytes_matches_upstream_lock_field() {
        use crate::runtime::catalog::lock_upstream_catalog_digest_bytes;
        assert_eq!(
            UmstManifestBuilder::lock_catalog_schema_digest_bytes(),
            lock_upstream_catalog_digest_bytes()
        );
        assert_ne!(
            UmstManifestBuilder::lock_catalog_schema_digest_bytes(),
            catalog_lock_bundle_sha256_bytes(),
            "R0 upstream digest differs from verbatim lock-bundle SHA-256"
        );
    }

    #[test]
    fn for_release_witness_builder_matches_strict_profile() {
        let manifest = UmstManifestBuilder::default().for_release_witness().build();
        assert_eq!(
            manifest.grounding_contract,
            GroundingContract::StrictCatalogMatch
        );
        assert_eq!(
            manifest.catalog_hash,
            crate::runtime::catalog::lock_upstream_catalog_digest_bytes()
        );
    }

    #[cfg(feature = "formal-witness")]
    #[test]
    fn for_release_witness_exposes_digest_witness() {
        use crate::runtime::catalog::lock_upstream_catalog_digest_bytes;
        let manifest = UmstManifestBuilder::default().for_release_witness().build();
        assert_eq!(
            manifest.witness_catalog_digest(),
            Some(lock_upstream_catalog_digest_bytes())
        );
        let staging = UmstManifestBuilder::default().for_staging().build();
        assert_eq!(staging.witness_catalog_digest(), None);
    }

    #[test]
    fn release_profile_strict_contract_pins_lock_digest() {
        let manifest = UmstManifestBuilder::default()
            .grounding_contract(GroundingContract::StrictCatalogMatch)
            .build();
        assert_eq!(
            manifest.grounding_contract,
            GroundingContract::StrictCatalogMatch
        );
        assert_eq!(
            manifest.catalog_hash,
            crate::runtime::catalog::lock_upstream_catalog_digest_bytes()
        );
    }

    #[test]
    fn optional_witness_priority_queue_on_manifest() {
        let mut q = WitnessPriorityQueue::for_adaptive_coverage();
        q.record_reject(LANDAUER_CBF_CATALOG_ID);
        let manifest = UmstManifestBuilder::default()
            .witness_priority_queue(q)
            .build();
        let q = manifest.witness_priority_queue.expect("attached queue");
        assert!(q.is_enabled());
        assert_eq!(q.priority_of_module("LandauerLaw"), 15);
    }

    #[test]
    fn staging_builder_keeps_caller_catalog_hash() {
        let mut wrong = catalog_lock_bundle_sha256_bytes();
        wrong[0] ^= 0xff;
        let manifest = UmstManifestBuilder::default()
            .for_staging()
            .catalog_hash(wrong)
            .build();
        assert_eq!(manifest.catalog_hash, wrong);
        assert_eq!(
            manifest.grounding_contract,
            GroundingContract::CatalogPinnedRos2
        );
    }

    #[test]
    fn umst_manifest_honest_fence_posture_probe() {
        let probe = manifest_posture_probe();
        assert!(manifest_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("master_composition_wired=false"));
        validate_manifest_posture_honesty().expect("validate_manifest_posture_honesty");
    }

    #[test]
    fn umst_manifest_production_and_master_stay_false() {
        assert!(!MANIFEST_PHYSICS_GREEN);
        assert!(!MANIFEST_PRODUCTION_WIRED);
        assert!(!MANIFEST_MASTER);
        assert!(!manifest_production_wired());
        assert!(!manifest_master_composition_wired());
    }

    #[test]
    fn umst_manifest_fence_facet_census_matches_constants() {
        assert_eq!(MANIFEST_FENCE_FACET_IDS.len(), MANIFEST_FENCE_FACET_COUNT);
        assert_eq!(
            MANIFEST_PRODUCTION_FENCE_FACETS.len(),
            MANIFEST_FENCE_FACET_COUNT
        );
        assert_eq!(manifest_fence_wired_count(), MANIFEST_FENCE_WIRED_COUNT);
        let bundle = manifest_honest_posture_bundle();
        assert_eq!(bundle.fence_facet_count, MANIFEST_FENCE_FACET_COUNT);
        assert_eq!(bundle.fence_wired_count, MANIFEST_FENCE_WIRED_COUNT);
        assert!(!bundle.physics_green);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
    }

    #[test]
    fn umst_manifest_deferred_facets_stay_unwired() {
        let deferred: Vec<_> = MANIFEST_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| !f.wired)
            .map(|f| f.facet)
            .collect();
        assert!(deferred.contains(&"formal_witness_digest"));
        assert!(deferred.contains(&"manifest_bridge_cartridge"));
        assert!(deferred.contains(&"production_wired"));
        assert_eq!(deferred.len(), 3);
    }
}

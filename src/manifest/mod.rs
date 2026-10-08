// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Composes default manifold gate bindings for downstream cartridges (thin SSOT façade).
//!
//! **Default:** [`UmstManifestBuilder::default`] pins `catalog.lock.json`; release binaries default
//! strict via `not(debug_assertions)`. Debug builds use [`UmstManifestBuilder::for_staging`] for
//! [`GroundingContract::CatalogPinnedRos2`].
//!
//! **Witness:** [`UmstManifestBuilder::for_release_profile`] + `--features formal-witness`
//! + cartridge `manifest-bridge` — see [`umst_manifest`](umst_manifest) and [`VERIFY.md`](../../docs/VERIFY.md) §3.3.
//!
//!   CI: `scripts/verify_umst_stack.sh` runs `--test manifest_strict_witness` in the release lane (skip with `UMST_RELEASE_MANIFEST_PROFILE=0`).

mod orchestrator;
mod umst_manifest;

pub use crate::runtime::catalog::WitnessPriorityQueue;
pub use orchestrator::{EmbodiedOrchestrator, EmbodiedReject, HostTransitionStep};
pub use orchestrator::{
    orchestrator_posture_is_honest, ORCHESTRATOR_CELL_ID, ORCHESTRATOR_HONEST_FENCE,
    ORCHESTRATOR_MORPHISM_ID, ORCHESTRATOR_PHYSICS_GREEN, ORCHESTRATOR_POSTURE_TAG,
    ORCHESTRATOR_PRODUCTION_WIRED,
};
pub use umst_manifest::{GateRegistry, GroundingContract, UmstManifest, UmstManifestBuilder};
pub use umst_manifest::{
    manifest_fence_wired_count, manifest_honest_posture_bundle, manifest_master_composition_wired,
    manifest_posture_honest, manifest_posture_probe, manifest_production_wired,
    validate_manifest_posture_honesty, ManifestHonestPosture, ManifestPostureProbe,
    ManifestProductionFenceFacet, MANIFEST_BRIDGE_DEFERRED_STEP, MANIFEST_BUILDER_LANDED,
    MANIFEST_CATALOG_PIN_LANDED, MANIFEST_FENCE_FACET_COUNT, MANIFEST_FENCE_FACET_IDS,
    MANIFEST_FENCE_WIRED_COUNT, MANIFEST_GATE_REGISTRY_LANDED, MANIFEST_HONEST_FENCE,
    MANIFEST_MASTER, MANIFEST_PHYSICS_GREEN, MANIFEST_POSTURE_TAG, MANIFEST_PRODUCTION_FENCE_FACETS,
    MANIFEST_PRODUCTION_ORCH_DEFERRED_STEP, MANIFEST_PRODUCTION_WIRED, W29_MANIFEST_DEEPEN_CELL,
};

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Composes default manifold gate bindings for downstream cartridges (thin SSOT façade).
//!
//! **Default:** [`UmstManifestBuilder::default`] pins `catalog.lock.json`; release binaries default
//! strict via `not(debug_assertions)`. Debug builds use [`UmstManifestBuilder::for_staging`] for
//! [`GroundingContract::CatalogPinnedRos2`].
//!
//! **Witness:** [`UmstManifestBuilder::for_release_profile`] + `--features formal-witness`
//! + cartridge `manifest-bridge` — see [`umst_manifest`](umst_manifest) and [`VERIFY.md`](../../docs/VERIFY.md) §3.3.
//! CI: `scripts/verify_umst_stack.sh` runs `--test manifest_strict_witness` in the release lane (skip with `UMST_RELEASE_MANIFEST_PROFILE=0`).

mod orchestrator;
mod umst_manifest;

pub use orchestrator::{EmbodiedOrchestrator, EmbodiedReject, HostTransitionStep};
pub use umst_manifest::{GateRegistry, GroundingContract, UmstManifest, UmstManifestBuilder};
pub use crate::runtime::catalog::WitnessPriorityQueue;

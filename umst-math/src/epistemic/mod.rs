// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Epistemic probes, policy hooks, and **proxy ranking** (Phase K1).
//!
//! Host-adaptive *policy* (cockpit / meta-loop) may tune [`selector::SelectorParams`] in Phase N3+;
//! this module stays **ISA-agnostic** pure math (§0.3 `docs/HSAD_PLAN.md`).

pub mod selector;

pub use selector::{
    rank_epistemic_proxies_by_mi, EpistemicProxyCandidate, RankedProxy, SelectorParams,
};

/// Probe label for provider-indexed MI accounting.
///
/// Proof: `EpistemicSensing` / `QuantumProbe`.
/// DOI: 10.5281/zenodo.19159660
#[derive(Clone, Debug)]
pub struct ProbeId(pub &'static str);

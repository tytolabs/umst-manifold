// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thermodynamic gates, Kleisli admissibility, and CBF bridging.
//!
//! Source lineage: **`umst-prototype` / `umst-prototype-2a`** `science/thermodynamic_filter.rs`,
//! `tensors/kleisli.rs` — ported without `wasm-bindgen` (see **`docs/GateUnificationSpec.md`**).

pub mod cbf;
pub mod cbf_bridge;
pub mod evaluator;
pub mod http_manifest;
pub mod kleisli;
pub mod thermo_transition;
pub mod transition_eval_registry;
pub mod transition_proposal;
pub mod verdict;

pub use cbf::GateThermodynamicCBF;
pub use cbf_bridge::cd_dissipation_proxy_to_entropy_joules;
pub use evaluator::{
    GateEvaluator, ThermodynamicTransitionEvaluator, TransitionGateEvaluator, TransitionVerdict,
};
pub use http_manifest::{
    default_gate_manifest, evaluate as evaluate_http_mix_manifest, gate_json_parse_response,
    reaction_extent_from_age, physics_compressive_strength_mpa, pinned_catalog_bundle_sha256_hex,
    GateHttpRuntime, GateManifest as HttpGateManifest, GateResponse as HttpGateResponse,
    HttpTransitionEvaluator, MixProposal as HttpMixProposal,
};
pub use kleisli::{
    gate_arrow_generic as gate_arrow, kleisli_compose_pair,
    AdmissibilityResult as KleisliAdmissibilityResult, Admissible, KleisliArrow, KleisliPipeline,
    KleisliUnitEvaluator,
};
pub use transition_eval_registry::{
    GateEvaluatorRegistry, ThermodynamicTransitionContext, TransitionEvaluator,
};
pub use transition_proposal::{
    evaluate_transition, thermodynamic_transition_admissible, thermodynamic_transition_admissible_tol,
    ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome, TransitionFilter,
    TRANSITION_TOLERANCE,
};
pub use thermo_transition::{
    AdmissibilityResult as ThermodynamicAdmissibilityResult, ThermodynamicGate, ThermodynamicState,
};
pub use verdict::AdmissibilityVerdict;

#[deprecated(note = "renamed to transition_eval_registry")]
pub mod mix_eval_registry {
    pub use super::transition_eval_registry::*;
}

#[deprecated(note = "renamed to HttpTransitionEvaluator")]
pub use http_manifest::HttpTransitionEvaluator as HttpMixGateEvaluator;

#[deprecated(note = "renamed to TransitionEvaluator")]
pub use transition_eval_registry::TransitionEvaluator as ThermodynamicMixEvaluator;

#[deprecated(note = "renamed to TransitionFilter")]
pub use transition_proposal::TransitionFilter as ThermodynamicMixFilter;

#[deprecated(note = "renamed to TransitionScalars")]
pub use transition_proposal::TransitionScalars as MixProposalScalars;

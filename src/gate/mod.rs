// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thermodynamic gates, Kleisli admissibility, and CBF bridging.
//!
//! Source lineage: **`umst-prototype` / `umst-prototype-2a`** `science/thermodynamic_filter.rs`,
//! `tensors/kleisli.rs` — ported without `wasm-bindgen` (see **`docs/GateUnificationSpec.md`**).

pub mod admissibility_census;
pub mod cbf;
pub mod cbf_bridge;
pub mod core_gate;
pub mod evaluator;
pub mod http_manifest;
pub mod kleisli;
pub mod material_gate;
pub mod open_system;
pub mod route;
pub mod thermo_transition;
pub mod transition_eval_registry;
pub mod transition_proposal;
pub mod verdict;

pub use admissibility_census::{
    format_open_deltas, gate_parity_fixture_path_from, ADMISSIBILITY_COMPUTE_SITES,
    ADMISSIBILITY_CONSUME_SITES, ConjunctFamily, GATE_PARITY_V0_FIXTURE_REL, GATE_PARITY_V0_SHA256,
    GATE_PARITY_V0_SHA256_PREFIX, OPEN_RECONCILIATION_DELTAS, ReconciliationDelta, SiteRole,
};
pub use cbf::GateThermodynamicCBF;
pub use cbf_bridge::cd_dissipation_proxy_to_entropy_joules;
pub use core_gate::{
    core_gate, gate as core_gate_predicate, mass_conserved_between_densities,
    scalar_response_from_transition, AdmissibilityResponse, CoreGateOutcome,
    ScalarConstitutiveResponse,
    GATE_MASS_TOLERANCE_KG_M3 as CORE_GATE_MASS_TOLERANCE_KG_M3,
};
pub use material_gate::{material_gate, MaterialGateOutcome, MaterialTransitionWitness};
pub use open_system::{
    active_matter_power_input, cbf_cd_matches_open_system_gate, cbf_landauer_as_power_input,
    cbf_open_system_admissible, landauer_power_input_joules, open_system_core_gate,
    transition_outcome_with_power_input, ActiveMatterFixture,
};
pub use route::{
    canonical_core_gate_outcome, canonical_material_gate_outcome,
    canonical_thermo_transition_admissible, canonical_transition_admissible,
    canonical_transition_outcome,
};
pub use evaluator::{
    GateEvaluator, ThermodynamicTransitionEvaluator, TransitionGateEvaluator, TransitionVerdict,
};
#[allow(deprecated)]
pub use http_manifest::{
    evaluate as evaluate_http_mix_manifest, gate_json_parse_response,
    physics_compressive_strength_mpa, pinned_catalog_bundle_sha256_hex, reaction_extent_from_age,
    GateHttpRuntime, GateManifest as HttpGateManifest, GateResponse as HttpGateResponse,
    HttpTransitionEvaluator, MixProposal as HttpMixProposal,
};
pub use kleisli::{
    gate_arrow_generic as gate_arrow, kleisli_compose_pair,
    AdmissibilityResult as KleisliAdmissibilityResult, Admissible, KleisliArrow, KleisliPipeline,
    KleisliUnitEvaluator,
};
pub use thermo_transition::{
    thermo_gate_transition_outcome, AdmissibilityResult as ThermodynamicAdmissibilityResult,
    ThermodynamicGate, ThermodynamicState,
};
pub use transition_eval_registry::{
    GateEvaluatorRegistry, ThermodynamicTransitionContext, TransitionEvaluator,
};
pub use transition_proposal::{
    evaluate_transition, evaluate_transition_pure_with_params, evaluate_transition_with_params,
    thermodynamic_transition_admissible, thermodynamic_transition_admissible_tol,
    transition_outcome, ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome,
    TransitionFilter, TransitionScalars, TRANSITION_TOLERANCE,
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

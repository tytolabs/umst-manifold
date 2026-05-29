// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thermodynamic gates, Kleisli admissibility, and CBF bridging.
//!
//! Source lineage: **`umst-prototype` / `umst-prototype-2a`** `science/thermodynamic_filter.rs`,
//! `tensors/kleisli.rs` — ported without `wasm-bindgen` (see **`docs/GateUnificationSpec.md`**).

pub mod cbf;
pub mod cbf_bridge;
pub mod concrete_cartridge;
pub mod evaluator;
pub mod http_manifest;
pub mod kleisli;
pub mod mix_eval_registry;
pub mod mix_proposal;
pub mod thermo_transition;
pub mod verdict;

pub use cbf::GateThermodynamicCBF;
pub use cbf_bridge::cd_dissipation_proxy_to_entropy_joules;
pub use concrete_cartridge::ConcreteCartridge;
pub use evaluator::{
    GateEvaluator, ThermodynamicTransitionEvaluator, TransitionGateEvaluator, TransitionVerdict,
};
pub use http_manifest::{
    default_gate_manifest, evaluate as evaluate_http_mix_manifest, gate_json_parse_response,
    hydration_degree, physics_compressive_strength_mpa, pinned_catalog_bundle_sha256_hex,
    GateHttpRuntime, GateManifest as HttpGateManifest, GateResponse as HttpGateResponse,
    HttpMixGateEvaluator, MixProposal as HttpMixProposal,
};
pub use kleisli::{
    gate_arrow_generic as gate_arrow, kleisli_compose_pair,
    AdmissibilityResult as KleisliAdmissibilityResult, Admissible, KleisliArrow, KleisliPipeline,
    KleisliUnitEvaluator,
};
pub use mix_eval_registry::{
    GateEvaluatorRegistry, ThermodynamicMixEvaluator, ThermodynamicTransitionContext,
};
pub use mix_proposal::{
    ThermodynamicMixFilter, ThermodynamicStateSnapshot, ThermodynamicTransitionOutcome,
};
pub use thermo_transition::{
    AdmissibilityResult as ThermodynamicAdmissibilityResult, ThermodynamicGate, ThermodynamicState,
};
pub use verdict::AdmissibilityVerdict;

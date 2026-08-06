// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Thermodynamic gates, Kleisli admissibility, and CBF bridging.
//!
//! Source lineage: **`umst-prototype` / `umst-prototype-2a`** `science/thermodynamic_filter.rs`,
//! `tensors/kleisli.rs` — ported without `wasm-bindgen` (see **`docs/GateUnificationSpec.md`**).
//!
//! Barrel census: [`gate_module_census_probe`] / [`gate_module_census_honest`] — honest module-band
//! inventory for reorg consumers (`PORT-MF-GATE-W2`, absorbs `PORT-MF-GATE-RETRY-W2` +
//! `PORT-MF-S2-EXPORT-W2`). Does **not** claim `production_wired=true`, registry GREEN, or MASTER
//! retick eligibility.

use crate::runtime::gate::{S2_EXTRACT_FENCE_FACET_COUNT, S2_EXTRACT_FENCE_FACET_IDS};

pub mod admissibility_census;
pub mod cbf;
pub mod info_gain;
pub mod liquid_ppo;
pub mod optim;
pub mod ppo_gateway;
pub mod cbf_bridge;
pub mod core_gate;
pub mod evaluator;
pub mod http_manifest;
pub mod kleisli;
pub mod material_gate;
pub mod open_system;
pub mod route;
pub mod semantic_cbf;
pub mod thermo_transition;
pub mod transition_eval_registry;
pub mod transition_proposal;
pub mod verdict;
pub mod web_route;

pub use admissibility_census::{
    format_open_deltas, gate_parity_fixture_path_from, ADMISSIBILITY_COMPUTE_SITES,
    ADMISSIBILITY_CONSUME_SITES, ConjunctFamily, GATE_PARITY_V0_FIXTURE_REL, GATE_PARITY_V0_SHA256,
    GATE_PARITY_V0_SHA256_PREFIX, OPEN_RECONCILIATION_DELTAS, ReconciliationDelta, SiteRole,
};
pub use cbf::GateThermodynamicCBF;
pub use info_gain::{
    suggested_info_gain_from_batched_nodal_scalars,
    suggested_info_gain_from_state_delta,
};
#[cfg(feature = "epistemic-ppo")]
pub use info_gain::nodal_scalar_means;
pub use liquid_ppo::GateBurnLiquidPPOAgent;
pub use optim::LPP_008_PROPOSED_HOME;
pub use ppo_gateway::GateManifoldGateway;
pub use cbf_bridge::cd_dissipation_proxy_to_entropy_joules;
pub use semantic_cbf::{
    gate_semantic_hot, gate_semantic_hot_bundled, hot_gate_lookup_cold_witness,
    verify_cold_witness_digest, SemanticCBF, SemanticCbfReject,
};
pub use core_gate::{
    core_gate, gate as core_gate_predicate, mass_conserved_between_densities,
    scalar_response_from_transition, AdmissibilityResponse, CoreGateOutcome,
    ScalarConstitutiveResponse,
    GATE_MASS_TOLERANCE_KG_M3 as CORE_GATE_MASS_TOLERANCE_KG_M3,
};
#[allow(deprecated)]
pub use material_gate::{MaterialGateOutcome, MaterialTransitionWitness};
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
pub use web_route::{
    canonical_web_gate_from_quantities, canonical_web_gate_outcome,
    canonical_web_semantic_gate_outcome, canonical_web_transition_admissible,
    canonical_web_transition_from_tensors, canonical_web_transition_from_tensors_with_semantic,
    canonical_web_transition_outcome,
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

// —— PORT-MF-GATE-W2 barrel census deepen (reorg consumer witness) ————————————

/// Honest adoption tier — barrel inventory only; no GREEN invent.
pub const GATE_MODULE_POSTURE_TAG: &str = "honest-barrel-census-only";

/// PORT-MF-GATE-W2 cell id — base census + consumer reorg witness @ HEAD.
pub const GATE_MODULE_CENSUS_CELL_ID: &str = "PORT-MF-GATE-W2";

/// Retry cell absorbed after S2 export unblock (`PORT-MF-GATE-RETRY-W2`).
pub const GATE_MODULE_CENSUS_RETRY_CELL_ID: &str = "PORT-MF-GATE-RETRY-W2";

/// S2 export unblock cell that fixed `sec_s2_extract_fence.rs` compile.
pub const GATE_MODULE_CENSUS_S2_EXPORT_CELL_ID: &str = "PORT-MF-S2-EXPORT-W2";

/// Live `pub mod` sibling count under `src/gate/` (excludes deprecated `mix_eval_registry`).
pub const GATE_MODULE_COUNT: usize = 20;

/// Rerouted consumer edges @ `src/gate` locus per PORT_GRAIN_BAND §3.4.
pub const GATE_REROUTE_EDGE_COUNT: usize = 13;

/// Deprecated barrel aliases retained for tombstone parity (`mix_eval_registry` + 4 type aliases).
pub const GATE_DEPRECATED_ALIAS_COUNT: usize = 5;

/// G0 gate parity fixture family pin (all gate bands bound).
pub const GATE_GOLDEN_FIXTURE_FAMILY: &str = "fixtures/gate_parity.json";

/// Morphism role taxonomy @ PORT_GRAIN_BAND_MANIFOLD_GATE_CORE §3.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateModuleRole {
    /// `transition_proposal`, `route`, `evaluator`.
    HotSpine,
    /// `kleisli`, `thermo_transition`.
    KleisliAlgebra,
    /// `open_system`, `cbf`, `cbf_bridge`.
    OpenSystem,
    /// `http_manifest`, `web_route`.
    HttpManifest,
    /// `semantic_cbf`.
    SemanticWitness,
    /// `transition_eval_registry`.
    Registry,
    /// `admissibility_census`, `core_gate`, `material_gate`, `verdict` re-export shims.
    ReExport,
    /// `info_gain`, `liquid_ppo`, `optim`, `ppo_gateway` — learner optional per LEARNER_OPTIONAL map.
    LearnerOptional,
}

impl GateModuleRole {
    /// Stable tag for receipts / meta probes.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::HotSpine => "hot_spine",
            Self::KleisliAlgebra => "kleisli_algebra",
            Self::OpenSystem => "open_system",
            Self::HttpManifest => "http_manifest",
            Self::SemanticWitness => "semantic_witness",
            Self::Registry => "registry",
            Self::ReExport => "re_export",
            Self::LearnerOptional => "learner_optional",
        }
    }
}

/// One `pub mod` band in the gate barrel lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateModuleBand {
    /// Stable band id (`gate:<module>`).
    pub band_id: &'static str,
    /// Sibling module stem (`src/gate/<module>.rs`).
    pub module: &'static str,
    /// Morphism role @ port-grain taxonomy.
    pub role: GateModuleRole,
    /// Rerouted consumer edges attributed to this band (0 = preserved-only).
    pub reroute_edges: u8,
}

/// Frozen gate module inventory @ HEAD — matches live `pub mod` declarations above.
pub const GATE_MODULE_BANDS: &[GateModuleBand] = &[
    GateModuleBand {
        band_id: "gate:admissibility_census",
        module: "admissibility_census",
        role: GateModuleRole::ReExport,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:cbf",
        module: "cbf",
        role: GateModuleRole::OpenSystem,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:info_gain",
        module: "info_gain",
        role: GateModuleRole::LearnerOptional,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:liquid_ppo",
        module: "liquid_ppo",
        role: GateModuleRole::LearnerOptional,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:optim",
        module: "optim",
        role: GateModuleRole::LearnerOptional,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:ppo_gateway",
        module: "ppo_gateway",
        role: GateModuleRole::LearnerOptional,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:cbf_bridge",
        module: "cbf_bridge",
        role: GateModuleRole::OpenSystem,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:core_gate",
        module: "core_gate",
        role: GateModuleRole::ReExport,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:evaluator",
        module: "evaluator",
        role: GateModuleRole::HotSpine,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:http_manifest",
        module: "http_manifest",
        role: GateModuleRole::HttpManifest,
        reroute_edges: 1,
    },
    GateModuleBand {
        band_id: "gate:kleisli",
        module: "kleisli",
        role: GateModuleRole::KleisliAlgebra,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:material_gate",
        module: "material_gate",
        role: GateModuleRole::ReExport,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:open_system",
        module: "open_system",
        role: GateModuleRole::OpenSystem,
        reroute_edges: 3,
    },
    GateModuleBand {
        band_id: "gate:route",
        module: "route",
        role: GateModuleRole::HotSpine,
        reroute_edges: 2,
    },
    GateModuleBand {
        band_id: "gate:semantic_cbf",
        module: "semantic_cbf",
        role: GateModuleRole::SemanticWitness,
        reroute_edges: 2,
    },
    GateModuleBand {
        band_id: "gate:thermo_transition",
        module: "thermo_transition",
        role: GateModuleRole::KleisliAlgebra,
        reroute_edges: 2,
    },
    GateModuleBand {
        band_id: "gate:transition_eval_registry",
        module: "transition_eval_registry",
        role: GateModuleRole::Registry,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:transition_proposal",
        module: "transition_proposal",
        role: GateModuleRole::HotSpine,
        reroute_edges: 3,
    },
    GateModuleBand {
        band_id: "gate:verdict",
        module: "verdict",
        role: GateModuleRole::ReExport,
        reroute_edges: 0,
    },
    GateModuleBand {
        band_id: "gate:web_route",
        module: "web_route",
        role: GateModuleRole::HttpManifest,
        reroute_edges: 0,
    },
];

/// Consumer reroute witness row — frozen @ PORT_GRAIN_BAND §3.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateConsumerRerouteRow {
    /// Source band id.
    pub band_id: &'static str,
    /// G_new adapter target (summary label).
    pub target: &'static str,
    /// Rerouted edge count for this band.
    pub edge_count: u8,
}

/// Frozen consumer reroute inventory — sums to [`GATE_REROUTE_EDGE_COUNT`].
pub const GATE_CONSUMER_REROUTE_ROWS: &[GateConsumerRerouteRow] = &[
    GateConsumerRerouteRow {
        band_id: "gate:transition_proposal",
        target: "umst-manifold:: (material_transition flatten)",
        edge_count: 3,
    },
    GateConsumerRerouteRow {
        band_id: "gate:open_system",
        target: "orchestration::ThermodynamicCBF",
        edge_count: 3,
    },
    GateConsumerRerouteRow {
        band_id: "gate:thermo_transition",
        target: "umst-manifold::SubstrateMaterialParams flatten",
        edge_count: 2,
    },
    GateConsumerRerouteRow {
        band_id: "gate:semantic_cbf",
        target: "core::CbfReject + orchestration::ThermodynamicCBF",
        edge_count: 2,
    },
    GateConsumerRerouteRow {
        band_id: "gate:route",
        target: "umst-cartridge-concrete::evaluate_material_conjuncts",
        edge_count: 2,
    },
    GateConsumerRerouteRow {
        band_id: "gate:http_manifest",
        target: "umst-manifold::UmstManifest flatten",
        edge_count: 1,
    },
];

/// Barrel census probe for reorg consumers and meta crosswalk.
#[derive(Debug, Clone, PartialEq)]
pub struct GateModuleCensusProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub module_count: usize,
    pub reroute_edge_count: usize,
    pub deprecated_alias_count: usize,
    pub compute_site_count: usize,
    pub consume_site_count: usize,
    pub parity_digest_prefix: &'static str,
    pub golden_fixture_family: &'static str,
    pub kleisli_catalog_id: &'static str,
    pub barrel_reexports_wired: bool,
    pub reroute_inventory_closed: bool,
    pub production_wired: bool,
    pub green_claim_blocked: bool,
    /// Base census measurements (module bands + reroute inventory) still hold.
    pub prior_census_honest: bool,
    /// `S2_EXTRACT_FENCE_FACET_IDS` re-export unblocked by PORT-MF-S2-EXPORT-W2.
    pub s2_extract_fence_export_unblocked: bool,
    /// Consumer reroute rows align with band inventory edge counts.
    pub consumer_reroute_witness_honest: bool,
    /// Retry chain (`PORT-MF-GATE-RETRY-W2`) measurements absorbed.
    pub retry_chain_absorbed: bool,
    /// Deepen honesty — base + S2 export + consumer witness; no GREEN invent.
    pub census_deepen_honest: bool,
}

/// Whether live barrel re-exports resolve to non-empty census surfaces.
#[must_use]
pub fn gate_barrel_reexports_wired() -> bool {
    !ADMISSIBILITY_COMPUTE_SITES.is_empty()
        && !ADMISSIBILITY_CONSUME_SITES.is_empty()
        && GATE_PARITY_V0_SHA256.len() == 64
        && TRANSITION_TOLERANCE.is_finite()
        && KleisliUnitEvaluator::CATALOG_ID == "umst.gate.kleisli_unit"
        && LPP_008_PROPOSED_HOME.contains("umst")
}

/// Whether reroute witness rows sum to the pinned edge total.
#[must_use]
pub fn gate_reroute_inventory_closed() -> bool {
    let sum: u32 = GATE_CONSUMER_REROUTE_ROWS
        .iter()
        .map(|row| u32::from(row.edge_count))
        .sum();
    sum == GATE_REROUTE_EDGE_COUNT as u32
        && GATE_MODULE_BANDS
            .iter()
            .map(|band| u32::from(band.reroute_edges))
            .sum::<u32>()
            == GATE_REROUTE_EDGE_COUNT as u32
}

/// Whether `S2_EXTRACT_FENCE_FACET_IDS` re-export resolves after PORT-MF-S2-EXPORT-W2.
#[must_use]
pub fn gate_s2_extract_fence_export_unblocked() -> bool {
    S2_EXTRACT_FENCE_FACET_IDS.len() == S2_EXTRACT_FENCE_FACET_COUNT
        && S2_EXTRACT_FENCE_FACET_COUNT == 7
        && S2_EXTRACT_FENCE_FACET_IDS
            .iter()
            .any(|facet| *facet == "trust_crate_reexport")
}

/// Whether base census measurements still hold @ HEAD.
#[must_use]
pub fn gate_module_prior_census_honest() -> bool {
    GATE_MODULE_BANDS.len() == GATE_MODULE_COUNT
        && gate_reroute_inventory_closed()
        && gate_barrel_reexports_wired()
}

/// Whether consumer reroute rows match band inventory edge attribution.
#[must_use]
pub fn gate_consumer_reroute_witness_honest() -> bool {
    GATE_CONSUMER_REROUTE_ROWS.len() == 6
        && GATE_CONSUMER_REROUTE_ROWS.iter().all(|row| {
            !row.band_id.is_empty()
                && !row.target.is_empty()
                && row.edge_count > 0
                && GATE_MODULE_BANDS.iter().any(|band| {
                    band.band_id == row.band_id && band.reroute_edges == row.edge_count
                })
        })
}

/// Whether retry chain cell measurements are absorbed into W2 census.
#[must_use]
pub fn gate_module_retry_chain_absorbed() -> bool {
    GATE_MODULE_CENSUS_RETRY_CELL_ID == "PORT-MF-GATE-RETRY-W2"
        && gate_module_prior_census_honest()
        && gate_s2_extract_fence_export_unblocked()
        && gate_consumer_reroute_witness_honest()
}

/// Build gate barrel census probe from live module measurements.
#[must_use]
pub fn gate_module_census_probe() -> GateModuleCensusProbe {
    let prior_census_honest = gate_module_prior_census_honest();
    let s2_extract_fence_export_unblocked = gate_s2_extract_fence_export_unblocked();
    let consumer_reroute_witness_honest = gate_consumer_reroute_witness_honest();
    let retry_chain_absorbed = gate_module_retry_chain_absorbed();
    let barrel_reexports_wired = gate_barrel_reexports_wired();
    let reroute_inventory_closed = gate_reroute_inventory_closed();
    let census_deepen_honest = prior_census_honest
        && s2_extract_fence_export_unblocked
        && consumer_reroute_witness_honest
        && retry_chain_absorbed
        && barrel_reexports_wired
        && reroute_inventory_closed;
    GateModuleCensusProbe {
        cell_id: GATE_MODULE_CENSUS_CELL_ID,
        posture_tag: GATE_MODULE_POSTURE_TAG,
        module_count: GATE_MODULE_BANDS.len(),
        reroute_edge_count: GATE_REROUTE_EDGE_COUNT,
        deprecated_alias_count: GATE_DEPRECATED_ALIAS_COUNT,
        compute_site_count: ADMISSIBILITY_COMPUTE_SITES.len(),
        consume_site_count: ADMISSIBILITY_CONSUME_SITES.len(),
        parity_digest_prefix: GATE_PARITY_V0_SHA256_PREFIX,
        golden_fixture_family: GATE_GOLDEN_FIXTURE_FAMILY,
        kleisli_catalog_id: KleisliUnitEvaluator::CATALOG_ID,
        barrel_reexports_wired,
        reroute_inventory_closed,
        production_wired: false,
        green_claim_blocked: true,
        prior_census_honest,
        s2_extract_fence_export_unblocked,
        consumer_reroute_witness_honest,
        retry_chain_absorbed,
        census_deepen_honest,
    }
}

/// Honesty gate — census wired; production/GREEN blocked; deepen chain closed.
#[must_use]
pub fn gate_module_census_honest(probe: &GateModuleCensusProbe) -> bool {
    probe.cell_id == GATE_MODULE_CENSUS_CELL_ID
        && probe.posture_tag == GATE_MODULE_POSTURE_TAG
        && probe.module_count == GATE_MODULE_COUNT
        && probe.reroute_edge_count == GATE_REROUTE_EDGE_COUNT
        && probe.deprecated_alias_count == GATE_DEPRECATED_ALIAS_COUNT
        && probe.compute_site_count == ADMISSIBILITY_COMPUTE_SITES.len()
        && probe.consume_site_count == ADMISSIBILITY_CONSUME_SITES.len()
        && probe.parity_digest_prefix == GATE_PARITY_V0_SHA256_PREFIX
        && GATE_PARITY_V0_SHA256.starts_with(probe.parity_digest_prefix)
        && probe.golden_fixture_family == GATE_GOLDEN_FIXTURE_FAMILY
        && probe.kleisli_catalog_id == "umst.gate.kleisli_unit"
        && probe.barrel_reexports_wired
        && probe.reroute_inventory_closed
        && probe.prior_census_honest
        && probe.s2_extract_fence_export_unblocked
        && probe.consumer_reroute_witness_honest
        && probe.retry_chain_absorbed
        && probe.census_deepen_honest
        && !probe.production_wired
        && probe.green_claim_blocked
}

/// Deepen honesty — absorbs PORT-MF-GATE-RETRY-W2 + PORT-MF-S2-EXPORT-W2 chain.
#[must_use]
pub fn gate_module_census_deepen_honest(probe: &GateModuleCensusProbe) -> bool {
    gate_module_census_honest(probe)
        && probe.prior_census_honest
        && probe.s2_extract_fence_export_unblocked
        && probe.consumer_reroute_witness_honest
        && probe.retry_chain_absorbed
        && probe.census_deepen_honest
        && GATE_MODULE_CENSUS_CELL_ID == "PORT-MF-GATE-W2"
        && GATE_MODULE_CENSUS_RETRY_CELL_ID == "PORT-MF-GATE-RETRY-W2"
        && GATE_MODULE_CENSUS_S2_EXPORT_CELL_ID == "PORT-MF-S2-EXPORT-W2"
}

/// Retry-cell mission closure — PORT-MF-GATE-RETRY-W2 after S2 export unblock.
#[must_use]
pub fn gate_module_census_retry_w2_honest() -> bool {
    GATE_MODULE_CENSUS_RETRY_CELL_ID == "PORT-MF-GATE-RETRY-W2"
        && GATE_MODULE_CENSUS_S2_EXPORT_CELL_ID == "PORT-MF-S2-EXPORT-W2"
        && gate_module_retry_chain_absorbed()
        && verify_gate_module_census().is_ok()
}

/// Validate gate barrel census — fail closed on drift or invented posture.
pub fn verify_gate_module_census() -> Result<GateModuleCensusProbe, String> {
    let probe = gate_module_census_probe();
    if probe.module_count != GATE_MODULE_COUNT {
        return Err(format!(
            "gate module count drift: expected {GATE_MODULE_COUNT}, got {}",
            probe.module_count
        ));
    }
    if !gate_reroute_inventory_closed() {
        return Err("gate reroute inventory not closed at 13 edges".into());
    }
    if !gate_barrel_reexports_wired() {
        return Err("gate barrel re-exports not wired".into());
    }
    if !gate_module_census_honest(&probe) {
        return Err("gate module census honesty predicate failed".into());
    }
    if !gate_consumer_reroute_witness_honest() {
        return Err("gate consumer reroute witness not honest".into());
    }
    if !gate_module_retry_chain_absorbed() {
        return Err("gate retry chain not absorbed".into());
    }
    if !gate_module_census_deepen_honest(&probe) {
        return Err("gate module census deepen honesty predicate failed".into());
    }
    Ok(probe)
}

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

#[cfg(test)]
mod gate_module_census_tests {
    use super::*;

    #[test]
    fn gate_module_band_inventory_matches_live_mod_count() {
        assert_eq!(GATE_MODULE_BANDS.len(), GATE_MODULE_COUNT);
        assert_eq!(GATE_MODULE_BANDS.len(), 20);
        let reroute_sum: u32 = GATE_MODULE_BANDS
            .iter()
            .map(|band| u32::from(band.reroute_edges))
            .sum();
        assert_eq!(reroute_sum, GATE_REROUTE_EDGE_COUNT as u32);
    }

    #[test]
    fn gate_consumer_reroute_rows_sum_to_thirteen() {
        assert!(gate_reroute_inventory_closed());
        let row_sum: u32 = GATE_CONSUMER_REROUTE_ROWS
            .iter()
            .map(|row| u32::from(row.edge_count))
            .sum();
        assert_eq!(row_sum, 13);
    }

    #[test]
    fn gate_consumer_reroute_rows_match_band_inventory() {
        assert!(gate_consumer_reroute_witness_honest());
        for row in GATE_CONSUMER_REROUTE_ROWS {
            let band = GATE_MODULE_BANDS
                .iter()
                .find(|band| band.band_id == row.band_id)
                .expect("band for reroute row");
            assert_eq!(band.reroute_edges, row.edge_count);
            assert!(!row.target.is_empty());
        }
    }

    #[test]
    fn gate_barrel_reexports_resolve_at_crate_root() {
        assert!(gate_barrel_reexports_wired());
        assert!(!ADMISSIBILITY_COMPUTE_SITES.is_empty());
        assert!(!ADMISSIBILITY_CONSUME_SITES.is_empty());
        assert_eq!(GATE_PARITY_V0_SHA256.len(), 64);
        assert!(GATE_PARITY_V0_SHA256.starts_with(GATE_PARITY_V0_SHA256_PREFIX));
        assert!(TRANSITION_TOLERANCE.is_finite());
        assert_eq!(KleisliUnitEvaluator::CATALOG_ID, "umst.gate.kleisli_unit");
    }

    #[test]
    fn gate_module_census_probe_honest_not_green() {
        let probe = gate_module_census_probe();
        assert!(gate_module_census_honest(&probe));
        assert!(gate_module_census_deepen_honest(&probe));
        assert!(!probe.production_wired);
        assert!(probe.green_claim_blocked);
        assert_eq!(probe.cell_id, "PORT-MF-GATE-W2");
        assert!(probe.prior_census_honest);
        assert!(probe.s2_extract_fence_export_unblocked);
        assert!(probe.consumer_reroute_witness_honest);
        assert!(probe.retry_chain_absorbed);
        assert!(probe.census_deepen_honest);
    }

    #[test]
    fn gate_s2_extract_fence_export_unblocked_after_s2_export_cell() {
        assert!(gate_s2_extract_fence_export_unblocked());
        assert_eq!(S2_EXTRACT_FENCE_FACET_IDS.len(), S2_EXTRACT_FENCE_FACET_COUNT);
        assert_eq!(GATE_MODULE_CENSUS_S2_EXPORT_CELL_ID, "PORT-MF-S2-EXPORT-W2");
        assert_eq!(GATE_MODULE_CENSUS_CELL_ID, "PORT-MF-GATE-W2");
        assert_eq!(GATE_MODULE_CENSUS_RETRY_CELL_ID, "PORT-MF-GATE-RETRY-W2");
        assert!(gate_module_retry_chain_absorbed());
    }

    #[test]
    fn gate_module_census_verify_passes() {
        let probe = verify_gate_module_census().expect("HEAD census");
        assert!(gate_module_census_honest(&probe));
    }

    #[test]
    fn gate_retry_w2_cell_deepen_honest_after_s2_export() {
        assert!(gate_module_census_retry_w2_honest());
        assert!(gate_s2_extract_fence_export_unblocked());
        assert!(gate_consumer_reroute_witness_honest());
        assert!(gate_module_retry_chain_absorbed());
        let probe = gate_module_census_probe();
        assert!(probe.census_deepen_honest);
        assert!(!probe.production_wired);
        assert!(probe.green_claim_blocked);
    }

    #[test]
    fn gate_hot_spine_symbols_reexported() {
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(0.45, 0.5, 293.15, 80.0);
        let routed = canonical_transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0);
        let direct = transition_outcome(&old, &new, 28.0 * 24.0 * 3600.0, TRANSITION_TOLERANCE);
        assert_eq!(routed, direct);
        assert!(CORE_GATE_MASS_TOLERANCE_KG_M3 > 0.0);
        assert_eq!(AdmissibilityVerdict::Accepted, AdmissibilityVerdict::Accepted);
    }

    #[test]
    fn gate_role_taxonomy_covers_all_bands() {
        let mut hot = 0usize;
        let mut kleisli = 0usize;
        let mut open = 0usize;
        let mut http = 0usize;
        let mut semantic = 0usize;
        let mut registry = 0usize;
        let mut reexport = 0usize;
        let mut learner = 0usize;
        for band in GATE_MODULE_BANDS {
            match band.role {
                GateModuleRole::HotSpine => hot += 1,
                GateModuleRole::KleisliAlgebra => kleisli += 1,
                GateModuleRole::OpenSystem => open += 1,
                GateModuleRole::HttpManifest => http += 1,
                GateModuleRole::SemanticWitness => semantic += 1,
                GateModuleRole::Registry => registry += 1,
                GateModuleRole::ReExport => reexport += 1,
                GateModuleRole::LearnerOptional => learner += 1,
            }
        }
        assert_eq!(hot, 3);
        assert_eq!(kleisli, 2);
        assert_eq!(open, 3);
        assert_eq!(http, 2);
        assert_eq!(semantic, 1);
        assert_eq!(registry, 1);
        assert_eq!(reexport, 4);
        assert_eq!(learner, 4);
    }
}

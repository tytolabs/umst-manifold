// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! HTTP `POST /gate` bulk evaluation (prototype-style strength closure + Parrott kinetics).
//!
//! [`GateResponse::catalog_hash_hex`] mirrors [`crate::runtime::catalog::catalog_lock_bundle_sha256_hex`]
//! (pinned by `build.rs` over `artifacts/catalog.lock.json`).

use serde::{Deserialize, Serialize};

use super::evaluator::GateEvaluator;
use super::transition_eval_registry::{ThermodynamicTransitionContext, TransitionEvaluator};
use super::transition_proposal::{ThermodynamicStateSnapshot, TransitionFilter};
use crate::manifest::UmstManifest;
use crate::runtime::catalog::catalog_lock_bundle_sha256_hex;
use crate::runtime::catalog::traceability::{
    HTTP_SHIM_CATALOG_ID, MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY,
};

/// Embedded rule defaults (orthogonal to catalog bytes; bump `catalog_version` when literals change).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GateManifest {
    pub catalog_version: u32,
    /// Intrinsic strength scale in the closure (prototype D1 **`80`** MPa default).
    pub strength_intrinsic_mpa: f64,
    pub air_void_fraction: f64,
    pub admissibility_rel_margin: f64,
}

/// Closure defaults (prototype `PhysicsConfig::default` / UCI D1 **`s_intrinsic`**).
///
/// Prefer [`GateManifest::from`] — see **`docs/RUNTIME_TOPOLOGY.md`** § HTTP egress retirement.
#[must_use]
pub fn default_gate_manifest() -> GateManifest {
    GateManifest {
        catalog_version: 1,
        strength_intrinsic_mpa: 80.0,
        air_void_fraction: 0.02,
        admissibility_rel_margin: 0.15,
    }
}

impl Default for GateManifest {
    fn default() -> Self {
        default_gate_manifest()
    }
}

impl From<&UmstManifest> for GateManifest {
    fn from(_: &UmstManifest) -> Self {
        default_gate_manifest()
    }
}

/// Registry-first HTTP bulk gate (`catalog_id` [`HTTP_SHIM_CATALOG_ID`]).
///
/// Supersedes deprecated slug `umst.gate.prediction_vs_physics` (see **`docs/GateUnificationSpec.md`**
/// migration notes; constant [`crate::runtime::catalog::traceability::PREDICTION_VS_PHYSICS_CATALOG_ID_DEPRECATED`]).
#[derive(Debug)]
pub struct HttpTransitionEvaluator {
    pub manifest: GateManifest,
    mix_evaluator: TransitionEvaluator,
}

impl HttpTransitionEvaluator {
    #[must_use]
    pub fn new(manifest: GateManifest) -> Self {
        Self {
            manifest,
            mix_evaluator: TransitionEvaluator::new(TransitionFilter::new()),
        }
    }

    #[must_use]
    pub fn from_umst_manifest(manifest: &UmstManifest) -> Self {
        Self::new(GateManifest::from(manifest))
    }

    #[deprecated(
        note = "use HttpTransitionEvaluator::from_umst_manifest — injection-only; see docs/RUNTIME_TOPOLOGY.md § HTTP egress retirement"
    )]
    #[must_use]
    pub fn from_domain_policy_defaults() -> Self {
        Self::new(default_gate_manifest())
    }
}

impl GateEvaluator for HttpTransitionEvaluator {
    fn catalog_id(&self) -> &'static str {
        "umst.gate.http_shim"
    }

    fn gate_family(&self) -> &'static str {
        MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY
    }
}

/// Shared runtime for `gate_server` / [`crate::gate_server_router`].
#[derive(Debug)]
pub struct GateHttpRuntime {
    pub evaluator: HttpTransitionEvaluator,
}

impl GateHttpRuntime {
    /// Injection-only constructor — callers supply a configured [`HttpTransitionEvaluator`].
    #[must_use]
    pub fn new(evaluator: HttpTransitionEvaluator) -> Self {
        Self { evaluator }
    }

    #[deprecated(
        note = "use GateHttpRuntime::from_umst_manifest — injection-only; see docs/RUNTIME_TOPOLOGY.md § HTTP egress retirement"
    )]
    #[must_use]
    pub fn from_defaults() -> Self {
        Self::new(HttpTransitionEvaluator::from_umst_manifest(
            &UmstManifest::default(),
        ))
    }

    #[must_use]
    pub fn from_umst_manifest(manifest: &UmstManifest) -> Self {
        Self::new(HttpTransitionEvaluator::from_umst_manifest(manifest))
    }

    #[must_use]
    pub fn manifest(&self) -> &GateManifest {
        &self.evaluator.manifest
    }

    #[must_use]
    pub fn evaluate_transition(&self, proposal: &MixProposal) -> GateResponse {
        evaluate(proposal, &self.evaluator.manifest)
    }

    #[deprecated(note = "renamed to evaluate_transition")]
    #[must_use]
    pub fn evaluate_mix(&self, proposal: &MixProposal) -> GateResponse {
        self.evaluate_transition(proposal)
    }
}

impl HttpTransitionEvaluator {
    #[must_use]
    pub fn evaluate_transition(&self, proposal: &MixProposal) -> GateResponse {
        evaluate(proposal, &self.manifest)
    }

    #[deprecated(note = "renamed to evaluate_transition")]
    #[must_use]
    pub fn evaluate_mix(&self, proposal: &MixProposal) -> GateResponse {
        self.evaluate_transition(proposal)
    }

    /// Optional transition witness: idle → proposal reaction extent (host [`TransitionEvaluator`]).
    pub fn evaluate_transition_witness(
        &mut self,
        proposal: &MixProposal,
        dt_seconds: f64,
    ) -> Option<super::verdict::AdmissibilityVerdict> {
        let total = proposal.constituent_primary_kg
            + proposal.constituent_secondary_kg
            + proposal.constituent_tertiary_kg;
        if total <= 1.0e-9 || proposal.water <= 0.0 {
            return None;
        }
        let w_c = proposal.water / total;
        let supplementary_ratio =
            (proposal.constituent_secondary_kg + proposal.constituent_tertiary_kg) / total;
        let alpha = reaction_extent_from_age(
            proposal.age_days,
            proposal.temperature_c,
            supplementary_ratio,
        );
        let old = ThermodynamicStateSnapshot::new_idle();
        let new = ThermodynamicStateSnapshot::from_mix_calibrated(
            w_c,
            alpha,
            proposal.temperature_c + 273.15,
            self.manifest.strength_intrinsic_mpa,
        );
        let ctx = ThermodynamicTransitionContext {
            old_state: &old,
            new_state: &new,
            dt_seconds,
        };
        Some(self.mix_evaluator.evaluate_thermo_transition(ctx))
    }

    #[deprecated(note = "renamed to evaluate_transition_witness")]
    pub fn evaluate_mix_transition(
        &mut self,
        proposal: &MixProposal,
        dt_seconds: f64,
    ) -> Option<super::verdict::AdmissibilityVerdict> {
        self.evaluate_transition_witness(proposal, dt_seconds)
    }
}

#[inline]
pub fn pinned_catalog_bundle_sha256_hex() -> String {
    catalog_lock_bundle_sha256_hex().to_string()
}

/// Proposal JSON for `/gate` (prototype `GateRequest`–compatible serde field names).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MixProposal {
    #[serde(rename = "cement")]
    pub constituent_primary_kg: f64,
    #[serde(default, rename = "slag")]
    pub constituent_secondary_kg: f64,
    #[serde(default, rename = "fly_ash")]
    pub constituent_tertiary_kg: f64,
    pub water: f64,
    #[serde(rename = "age_days", alias = "age")]
    pub age_days: f64,
    #[serde(rename = "predicted_strength_mpa", alias = "predicted_strength")]
    pub predicted_strength_mpa: f64,
    #[serde(default = "default_temperature_c")]
    pub temperature_c: f64,
}

fn default_temperature_c() -> f64 {
    20.0
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GateResponse {
    pub admissible: bool,
    pub codes: Vec<String>,
    /// Gate evaluator slug when `admissible == false` (telemetry / ROS alignment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    pub catalog_hash_hex: String,
}

#[must_use]
pub fn gate_json_parse_response() -> GateResponse {
    GateResponse {
        admissible: false,
        codes: vec!["GATE_JSON_PARSE_ERROR".to_string()],
        catalog_id: Some(HTTP_SHIM_CATALOG_ID.to_string()),
        catalog_hash_hex: pinned_catalog_bundle_sha256_hex(),
    }
}

/// Parrott-style kinetics — `PhysicsKernel::compute_reaction_extent` (`umst-core` prototype).
#[must_use]
pub fn reaction_extent_from_age(age_days: f64, temp_c: f64, supplementary_ratio: f64) -> f64 {
    let supplementary_ratio = supplementary_ratio.clamp(0.0, 1.0) as f32;
    let alpha_max = 0.95 - supplementary_ratio * 0.15;
    let k_ref = 0.55_f32;
    let t_ref_k = 293.15_f32;
    let t_k = (temp_c as f32) + 273.15;
    let e_over_r = 5000.0_f32;
    let temp_factor = (e_over_r * (1.0 / t_ref_k - 1.0 / t_k)).exp();
    let supplementary_factor = 1.0 - supplementary_ratio * 0.4;
    let k = k_ref * temp_factor * supplementary_factor;
    let age_days = age_days as f32;
    let alpha = alpha_max * (1.0 - (-k * age_days.sqrt()).exp());
    f64::from(alpha.clamp(0.0, 1.0))
}

/// Compressive strength closure (MPa) — `StrengthEngine::compute_strength`.
#[must_use]
pub fn physics_compressive_strength_mpa(
    w_c_ratio: f64,
    degree_reaction_extent: f64,
    air: f64,
    intrinsic_strength: f64,
) -> f64 {
    let wc_ratio = w_c_ratio as f32;
    if wc_ratio > 100.0 {
        return 0.0;
    }
    let degree_reaction_extent = degree_reaction_extent as f32;
    let air_content = air as f32;
    let intrinsic_strength = intrinsic_strength as f32;
    let vg_volume_phase = 0.68 * degree_reaction_extent;
    let vc_volume_capillary = wc_ratio - 0.36 * degree_reaction_extent;
    let space = vg_volume_phase + vc_volume_capillary + air_content;
    if space <= 0.001 {
        return 0.0;
    }
    let x = vg_volume_phase / space;
    let fc = intrinsic_strength * x.powi(3);
    f64::from(fc.max(0.0))
}

#[must_use]
pub fn evaluate(proposal: &MixProposal, manifest: &GateManifest) -> GateResponse {
    let mut codes = Vec::new();

    let total_binder = proposal.constituent_primary_kg
        + proposal.constituent_secondary_kg
        + proposal.constituent_tertiary_kg;
    if total_binder <= 1.0e-9 {
        codes.push("MIX_EMPTY_BINDER".to_string());
        return finalize(false, codes);
    }
    if proposal.water <= 0.0 {
        codes.push("MIX_NONPOSITIVE_WATER".to_string());
        return finalize(false, codes);
    }
    if proposal.predicted_strength_mpa.is_nan() || proposal.predicted_strength_mpa < 0.0 {
        codes.push("STRENGTH_PREDICTION_INVALID".to_string());
        return finalize(false, codes);
    }

    let w_c = proposal.water / total_binder;
    let supplementary_ratio =
        (proposal.constituent_secondary_kg + proposal.constituent_tertiary_kg) / total_binder;
    let alpha = reaction_extent_from_age(
        proposal.age_days,
        proposal.temperature_c,
        supplementary_ratio,
    );
    let fc = physics_compressive_strength_mpa(
        w_c,
        alpha,
        manifest.air_void_fraction,
        manifest.strength_intrinsic_mpa,
    );

    let bound = fc * (1.0 + manifest.admissibility_rel_margin);
    if proposal.predicted_strength_mpa > bound {
        codes.push("CLAUSIUS_GATE_STRENGTH_EXCESS".to_string());
    }

    finalize(codes.is_empty(), codes)
}

fn finalize(admissible: bool, mut codes: Vec<String>) -> GateResponse {
    codes.sort_unstable();
    codes.dedup();
    GateResponse {
        admissible,
        codes,
        catalog_id: if admissible {
            None
        } else {
            Some(HTTP_SHIM_CATALOG_ID.to_string())
        },
        catalog_hash_hex: pinned_catalog_bundle_sha256_hex(),
    }
}

#[deprecated(note = "renamed to HttpTransitionEvaluator")]
pub type HttpMixGateEvaluator = HttpTransitionEvaluator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitting_vs_reject_example() {
        let m = GateManifest::from(&UmstManifest::default());
        let admit = MixProposal {
            constituent_primary_kg: 400.0,
            constituent_secondary_kg: 0.0,
            constituent_tertiary_kg: 0.0,
            water: 200.0,
            age_days: 28.0,
            predicted_strength_mpa: 25.0,
            temperature_c: 20.0,
        };
        let r = evaluate(&admit, &m);
        assert!(r.admissible);
        assert!(r.codes.is_empty());
        assert!(r.catalog_id.is_none());
        assert_eq!(r.catalog_hash_hex.len(), 64);

        let reject = MixProposal {
            predicted_strength_mpa: 1.0e9,
            ..admit
        };
        let r2 = evaluate(&reject, &m);
        assert!(!r2.admissible);
        assert!(r2
            .codes
            .contains(&"CLAUSIUS_GATE_STRENGTH_EXCESS".to_string()));
        assert_eq!(r2.catalog_id.as_deref(), Some(HTTP_SHIM_CATALOG_ID));
        assert_eq!(r.catalog_hash_hex, r2.catalog_hash_hex);
    }

    #[test]
    fn gate_json_parse_reject_carries_http_shim_catalog_id() {
        let r = gate_json_parse_response();
        assert!(!r.admissible);
        assert_eq!(r.catalog_id.as_deref(), Some(HTTP_SHIM_CATALOG_ID));
    }
}

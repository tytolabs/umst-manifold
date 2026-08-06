// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! AGAP-2350-MANIFOLD — night residual deepen for semantic lane / WEB-005 conjuncts.
//!
//! Does **not** flip production gates or claim new GREEN beyond prior AGAP-2033 deepen.

use crate::core::semantic_lane_schema::{
    validate_v1_layout_invariants, SEMANTIC_LANE_BASE, SEMANTIC_LANE_SCHEMA_V1,
};
use crate::runtime::catalog::{
    catalog_pin_manifold_wired, catalog_pin_production_wired, manifold_catalog_pin_ceremony_closed,
};
use crate::web_constitutive::{
    web_semantic_lane_overlap_valid, SEMANTIC_RESIDUAL_HOOK_V1,
};

/// AGAP-2350 night slot id.
pub const JOB_ID: &str = "AGAP-2350-MANIFOLD";

/// Completion receipt cross-ref (this wave).
pub const RECEIPT_PATH: &str = "archived/residuals/migration-2026-07-20/COMPLETION_AGAP_AGENT_MANIFOLD_2350.md";

/// Prior manifold semantic deepen receipt.
pub const PRIOR_RECEIPT_PATH: &str =
    "archived/residuals/migration-2026-07-20/COMPLETION_AGAP_AGENT_MANIFOLD-SEM_2033.md";

/// Honest adoption tier — prep wired; production flip blocked.
pub const POSTURE_TAG: &str = "semantic-lane-prep-not-production";

/// Night deepen probe — semantic lane bridge residual census.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifoldNight2350DeepenProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub prior_receipt_path: &'static str,
    pub semantic_lane_schema_v1: u32,
    pub semantic_lane_base: usize,
    pub semantic_residual_hook: &'static str,
    pub semantic_lane_schema_wired: bool,
    pub web_constitutive_wired: bool,
    pub catalog_pin_adjacent: bool,
    pub catalog_pin_ceremony_closed: bool,
    pub production_wired: bool,
    pub flip_authorized: bool,
    pub green_claim_blocked: bool,
}

/// Whether catalog_pin is manifold-adjacent but production flip stays open.
#[must_use]
pub fn night_2350_catalog_pin_adjacent() -> bool {
    catalog_pin_manifold_wired() && !catalog_pin_production_wired()
}

/// Honest night deepen — prep wired; production flip blocked.
#[must_use]
pub fn manifold_night_2350_deepen_probe() -> ManifoldNight2350DeepenProbe {
    ManifoldNight2350DeepenProbe {
        job_id: JOB_ID,
        receipt_path: RECEIPT_PATH,
        prior_receipt_path: PRIOR_RECEIPT_PATH,
        semantic_lane_schema_v1: SEMANTIC_LANE_SCHEMA_V1,
        semantic_lane_base: SEMANTIC_LANE_BASE,
        semantic_residual_hook: SEMANTIC_RESIDUAL_HOOK_V1,
        semantic_lane_schema_wired: validate_v1_layout_invariants(),
        web_constitutive_wired: web_semantic_lane_overlap_valid(),
        catalog_pin_adjacent: night_2350_catalog_pin_adjacent(),
        catalog_pin_ceremony_closed: manifold_catalog_pin_ceremony_closed(),
        production_wired: false,
        flip_authorized: false,
        green_claim_blocked: true,
    }
}

/// Honesty gate for operator receipts.
#[must_use]
pub fn manifold_night_2350_deepen_honest(probe: &ManifoldNight2350DeepenProbe) -> bool {
    probe.job_id == JOB_ID
        && probe.receipt_path.contains("MANIFOLD_2350")
        && probe.prior_receipt_path.contains("MANIFOLD-SEM_2033")
        && probe.semantic_lane_schema_v1 == SEMANTIC_LANE_SCHEMA_V1
        && probe.semantic_lane_base == SEMANTIC_LANE_BASE
        && probe.semantic_residual_hook == SEMANTIC_RESIDUAL_HOOK_V1
        && probe.semantic_lane_schema_wired
        && probe.web_constitutive_wired
        && probe.catalog_pin_adjacent
        && probe.catalog_pin_ceremony_closed
        && !probe.production_wired
        && !probe.flip_authorized
        && probe.green_claim_blocked
        && !catalog_pin_production_wired()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agap_2350_manifold_night_metadata() {
        assert_eq!(JOB_ID, "AGAP-2350-MANIFOLD");
        assert!(PRIOR_RECEIPT_PATH.contains("MANIFOLD-SEM_2033"));
        assert_eq!(POSTURE_TAG, "semantic-lane-prep-not-production");
    }

    #[test]
    fn manifold_night_deepen_measured_wiring() {
        assert!(validate_v1_layout_invariants());
        assert!(web_semantic_lane_overlap_valid());
        assert!(night_2350_catalog_pin_adjacent());
        assert!(manifold_catalog_pin_ceremony_closed());
    }

    #[test]
    fn catalog_pin_production_stays_false() {
        assert!(!catalog_pin_production_wired());
    }

    #[test]
    fn manifold_night_deepen_honest_prep_not_green() {
        let probe = manifold_night_2350_deepen_probe();
        assert!(manifold_night_2350_deepen_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.flip_authorized);
        assert!(probe.green_claim_blocked);
    }
}

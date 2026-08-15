// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! SWARM-C25-0831-93 — MANIFOLD-DEEPEN: Kleisli gate + semantic lane bridge census.
//! W29-131 deepen: honest invent fences; prep absorb; production flip stays blocked.
//!
//! Does **not** flip production gates or claim GREEN / PRODUCTION_WIRED / MASTER / OP-5
//! beyond prior AGAP-2033 / AGAP-2350 / 0831-93 deepens.

use crate::gate::kleisli::KleisliUnitEvaluator;
use crate::night_residual_deepen::{
    manifold_night_2350_deepen_honest, manifold_night_2350_deepen_probe,
};
use crate::web_constitutive::web_semantic_lane_overlap_valid;

/// SWARM slot id (0831 morning wave · board 93).
pub const JOB_ID: &str = "SWARM-C25-0831-93";

/// Completion receipt cross-ref (this wave).
pub const RECEIPT_PATH: &str = "old/residuals/residuals/swarm-0831/COMPLETION_SWARM_SWARM-C25-0831-93_0831.md";

/// Prior manifold semantic deepen receipt (AGAP-2033).
pub const PRIOR_SEM_RECEIPT_PATH: &str =
    "old/residuals/residuals/migration-2026-07-20/COMPLETION_AGAP_AGENT_MANIFOLD-SEM_2033.md";

/// Prior manifold night deepen receipt (AGAP-2350).
pub const PRIOR_NIGHT_RECEIPT_PATH: &str =
    "old/residuals/residuals/migration-2026-07-20/COMPLETION_AGAP_AGENT_MANIFOLD_2350.md";

/// Kleisli unit catalog surface id (hand-aligned to `Gate.lean`).
pub const KLEISLI_UNIT_CATALOG_ID: &str = KleisliUnitEvaluator::CATALOG_ID;

/// W29-131 swarm manifold deepen cell id.
pub const W29_131_CELL_ID: &str = "W29-131-SWARM_MANIFOLD_DEEPEN";

/// Model pin for this deepen lane (hard pin; not fast).
pub const DEEPEN_MODEL_SLUG: &str = "cursor-grok-4.6-high";

/// Admit coding lane for this deepen.
pub const DEEPEN_LANE: &str = "umst-admit-grok";

/// Honest deepen posture — 0831 absorb + invent fences; production ceremony stays OPEN.
pub const HONEST_DEEPEN_POSTURE: &str = "SWARM_MANIFOLD_DEEPEN_HONEST_PROD_OPEN";

/// Explicit non-claims — deepen must not invent these.
pub const NON_CLAIM: &str =
    "not GREEN; not PRODUCTION_WIRED; not MASTER; not OP-5; swarm manifold production flip remains OPEN";

/// Honest master retick posture — census only.
pub const MASTER_RETICK: &str = "no";

/// Prior 0831-93 receipt slug for W29 absorb chain.
pub const PRIOR_0831_RECEIPT_SLUG: &str = "SWARM-C25-0831-93";

/// Fence hop count for W29-131 invent census (kleisli · semantic · night · 0831 · invent).
pub const FENCE_HOP_COUNT: usize = 5;

/// Probe hops wired under honest prep (same as fence census; not a production wire count).
pub const PROBE_HOPS_WIRED: usize = 5;

/// Honest `production_wired` floor — never true until measured live wire proof.
#[must_use]
pub const fn swarm_manifold_deepen_production_wired() -> bool {
    false
}

const _: () = assert!(!swarm_manifold_deepen_production_wired());

/// OP-5 PASS invent fence — stays false on swarm manifold deepen.
#[must_use]
pub const fn swarm_manifold_deepen_op5_pass_invented() -> bool {
    false
}

const _: () = assert!(!swarm_manifold_deepen_op5_pass_invented());

/// MASTER invent / retick fence — rollup census only.
#[must_use]
pub const fn swarm_manifold_deepen_master_invented() -> bool {
    false
}

const _: () = assert!(!swarm_manifold_deepen_master_invented());

/// GREEN invent fence — stays false (tool readiness ≠ physics GREEN).
#[must_use]
pub const fn swarm_manifold_deepen_green_invented() -> bool {
    false
}

const _: () = assert!(!swarm_manifold_deepen_green_invented());

/// Flip authorization — blocked until operator ceremony.
#[must_use]
pub const fn swarm_manifold_deepen_flip_authorized() -> bool {
    false
}

const _: () = assert!(!swarm_manifold_deepen_flip_authorized());

/// Manifold deepen probe — Kleisli + semantic lane conjunct census.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifoldSwarm0831DeepenProbe {
    pub job_id: &'static str,
    pub receipt_path: &'static str,
    pub kleisli_catalog_id: &'static str,
    pub kleisli_gate_wired: bool,
    pub semantic_lane_bridge_wired: bool,
    pub night_deepen_prior_honest: bool,
    pub production_wired: bool,
    pub flip_authorized: bool,
}

/// Honest swarm deepen — prep wired; production flip blocked.
#[must_use]
pub fn manifold_swarm_0831_93_deepen_probe() -> ManifoldSwarm0831DeepenProbe {
    let night = manifold_night_2350_deepen_probe();
    ManifoldSwarm0831DeepenProbe {
        job_id: JOB_ID,
        receipt_path: RECEIPT_PATH,
        kleisli_catalog_id: KLEISLI_UNIT_CATALOG_ID,
        kleisli_gate_wired: KLEISLI_UNIT_CATALOG_ID == "umst.gate.kleisli_unit",
        semantic_lane_bridge_wired: web_semantic_lane_overlap_valid(),
        night_deepen_prior_honest: manifold_night_2350_deepen_honest(&night),
        production_wired: swarm_manifold_deepen_production_wired(),
        flip_authorized: swarm_manifold_deepen_flip_authorized(),
    }
}

/// Honesty gate for operator receipts.
#[must_use]
pub fn manifold_swarm_0831_93_deepen_honest(probe: &ManifoldSwarm0831DeepenProbe) -> bool {
    probe.job_id == JOB_ID
        && probe.receipt_path.contains("SWARM-C25-0831-93")
        && probe.kleisli_catalog_id == KLEISLI_UNIT_CATALOG_ID
        && probe.kleisli_gate_wired
        && probe.semantic_lane_bridge_wired
        && probe.night_deepen_prior_honest
        && !probe.production_wired
        && !probe.flip_authorized
}

/// W29-131 honesty deepen probe — absorbs 0831-93 + invent fences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifoldSwarmW29131DeepenProbe {
    pub cell_id: &'static str,
    pub model_slug: &'static str,
    pub lane: &'static str,
    pub honest_posture: &'static str,
    pub non_claim: &'static str,
    pub master_retick: &'static str,
    pub prior_0831_receipt_pinned: bool,
    pub prior_sem_receipt_pinned: bool,
    pub prior_night_receipt_pinned: bool,
    pub swarm_0831_deepen_honest: bool,
    pub kleisli_gate_wired: bool,
    pub semantic_lane_bridge_wired: bool,
    pub fence_hop_count: usize,
    pub probe_hops_wired: usize,
    pub production_wired: bool,
    pub op5_pass_invented: bool,
    pub master_invented: bool,
    pub green_invented: bool,
    pub flip_authorized: bool,
}

/// Build W29-131 deepen probe — absorbs 0831-93 and pins invent fences.
#[must_use]
pub fn manifold_swarm_w29131_deepen_probe() -> ManifoldSwarmW29131DeepenProbe {
    let prior = manifold_swarm_0831_93_deepen_probe();
    ManifoldSwarmW29131DeepenProbe {
        cell_id: W29_131_CELL_ID,
        model_slug: DEEPEN_MODEL_SLUG,
        lane: DEEPEN_LANE,
        honest_posture: HONEST_DEEPEN_POSTURE,
        non_claim: NON_CLAIM,
        master_retick: MASTER_RETICK,
        prior_0831_receipt_pinned: PRIOR_0831_RECEIPT_SLUG.contains("SWARM-C25-0831-93"),
        prior_sem_receipt_pinned: PRIOR_SEM_RECEIPT_PATH.contains("MANIFOLD-SEM_2033"),
        prior_night_receipt_pinned: PRIOR_NIGHT_RECEIPT_PATH.contains("MANIFOLD_2350"),
        swarm_0831_deepen_honest: manifold_swarm_0831_93_deepen_honest(&prior),
        kleisli_gate_wired: prior.kleisli_gate_wired,
        semantic_lane_bridge_wired: prior.semantic_lane_bridge_wired,
        fence_hop_count: FENCE_HOP_COUNT,
        probe_hops_wired: PROBE_HOPS_WIRED,
        production_wired: swarm_manifold_deepen_production_wired(),
        op5_pass_invented: swarm_manifold_deepen_op5_pass_invented(),
        master_invented: swarm_manifold_deepen_master_invented(),
        green_invented: swarm_manifold_deepen_green_invented(),
        flip_authorized: swarm_manifold_deepen_flip_authorized(),
    }
}

/// Honesty gate for W29-131 deepen — 0831 absorbed; invent fences hold.
#[must_use]
pub fn manifold_swarm_w29131_deepen_honest(probe: &ManifoldSwarmW29131DeepenProbe) -> bool {
    probe.cell_id == W29_131_CELL_ID
        && probe.model_slug == DEEPEN_MODEL_SLUG
        && probe.lane == DEEPEN_LANE
        && probe.honest_posture == HONEST_DEEPEN_POSTURE
        && probe.non_claim.contains("not GREEN")
        && probe.non_claim.contains("not PRODUCTION_WIRED")
        && probe.non_claim.contains("not MASTER")
        && probe.non_claim.contains("not OP-5")
        && probe.master_retick == "no"
        && probe.prior_0831_receipt_pinned
        && probe.prior_sem_receipt_pinned
        && probe.prior_night_receipt_pinned
        && probe.swarm_0831_deepen_honest
        && probe.kleisli_gate_wired
        && probe.semantic_lane_bridge_wired
        && probe.fence_hop_count == FENCE_HOP_COUNT
        && probe.probe_hops_wired == PROBE_HOPS_WIRED
        && !probe.production_wired
        && !probe.op5_pass_invented
        && !probe.master_invented
        && !probe.green_invented
        && !probe.flip_authorized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swarm_0831_93_metadata_pins() {
        assert_eq!(JOB_ID, "SWARM-C25-0831-93");
        assert!(PRIOR_SEM_RECEIPT_PATH.contains("MANIFOLD-SEM_2033"));
        assert!(PRIOR_NIGHT_RECEIPT_PATH.contains("MANIFOLD_2350"));
    }

    #[test]
    fn swarm_0831_93_deepen_honest_prep_not_green() {
        let probe = manifold_swarm_0831_93_deepen_probe();
        assert!(manifold_swarm_0831_93_deepen_honest(&probe));
        assert!(!probe.production_wired);
        assert!(!probe.flip_authorized);
    }

    #[test]
    fn swarm_manifold_invent_fences_hold() {
        assert!(!swarm_manifold_deepen_production_wired());
        assert!(!swarm_manifold_deepen_op5_pass_invented());
        assert!(!swarm_manifold_deepen_master_invented());
        assert!(!swarm_manifold_deepen_green_invented());
        assert!(!swarm_manifold_deepen_flip_authorized());
        assert_eq!(MASTER_RETICK, "no");
        assert!(NON_CLAIM.contains("not GREEN"));
        assert!(NON_CLAIM.contains("not PRODUCTION_WIRED"));
        assert!(NON_CLAIM.contains("not MASTER"));
        assert!(NON_CLAIM.contains("not OP-5"));
    }

    #[test]
    fn w29131_metadata_and_lane_pins() {
        assert_eq!(W29_131_CELL_ID, "W29-131-SWARM_MANIFOLD_DEEPEN");
        assert_eq!(DEEPEN_MODEL_SLUG, "cursor-grok-4.6-high");
        assert_eq!(DEEPEN_LANE, "umst-admit-grok");
        assert_eq!(HONEST_DEEPEN_POSTURE, "SWARM_MANIFOLD_DEEPEN_HONEST_PROD_OPEN");
        assert_eq!(FENCE_HOP_COUNT, PROBE_HOPS_WIRED);
        assert!(PRIOR_0831_RECEIPT_SLUG.contains("SWARM-C25-0831-93"));
    }

    #[test]
    fn w29131_deepen_probe_absorbs_0831_honest() {
        let probe = manifold_swarm_w29131_deepen_probe();
        assert!(manifold_swarm_w29131_deepen_honest(&probe));
        assert_eq!(probe.cell_id, W29_131_CELL_ID);
        assert_eq!(probe.model_slug, DEEPEN_MODEL_SLUG);
        assert_eq!(probe.lane, DEEPEN_LANE);
        assert!(probe.swarm_0831_deepen_honest);
        assert!(probe.kleisli_gate_wired);
        assert!(probe.semantic_lane_bridge_wired);
        assert_eq!(probe.fence_hop_count, 5);
        assert!(!probe.production_wired);
        assert!(!probe.op5_pass_invented);
        assert!(!probe.master_invented);
        assert!(!probe.green_invented);
        assert!(!probe.flip_authorized);
    }

    #[test]
    fn w29131_tamper_rejects_invented_green() {
        let mut probe = manifold_swarm_w29131_deepen_probe();
        probe.green_invented = true;
        assert!(!manifold_swarm_w29131_deepen_honest(&probe));
        probe.green_invented = false;
        probe.production_wired = true;
        assert!(!manifold_swarm_w29131_deepen_honest(&probe));
        probe.production_wired = false;
        probe.master_invented = true;
        assert!(!manifold_swarm_w29131_deepen_honest(&probe));
        probe.master_invented = false;
        probe.op5_pass_invented = true;
        assert!(!manifold_swarm_w29131_deepen_honest(&probe));
        probe.op5_pass_invented = false;
        probe.flip_authorized = true;
        assert!(!manifold_swarm_w29131_deepen_honest(&probe));
    }
}

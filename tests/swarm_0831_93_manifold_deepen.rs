// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//
// SWARM-C25-0831-93 — MANIFOLD-DEEPEN integration witnesses.

use umst_manifold::core::{LANE_RELATION_GRAPH, SEMANTIC_LANE_BASE};
use umst_manifold::gate::{AdmissibilityVerdict, GateEvaluatorRegistry, KleisliUnitEvaluator};
use umst_manifold::night_residual_deepen::{
    manifold_night_2350_deepen_honest, manifold_night_2350_deepen_probe,
    PRIOR_RECEIPT_PATH as NIGHT_PRIOR,
};
use umst_manifold::swarm_manifold_deepen::{
    manifold_swarm_0831_93_deepen_honest, manifold_swarm_0831_93_deepen_probe, JOB_ID,
    KLEISLI_UNIT_CATALOG_ID, PRIOR_NIGHT_RECEIPT_PATH, PRIOR_SEM_RECEIPT_PATH, RECEIPT_PATH,
};
use umst_manifold::web_constitutive::{slice_layout, web_semantic_lane_overlap_valid};

#[test]
fn swarm_0831_93_job_metadata() {
    assert_eq!(JOB_ID, "SWARM-C25-0831-93");
    assert!(RECEIPT_PATH.contains("SWARM-C25-0831-93"));
    assert!(PRIOR_SEM_RECEIPT_PATH.contains("MANIFOLD-SEM_2033"));
    assert!(PRIOR_NIGHT_RECEIPT_PATH.contains("MANIFOLD_2350"));
    assert!(NIGHT_PRIOR.contains("MANIFOLD-SEM_2033"));
}

#[test]
fn swarm_0831_93_kleisli_gate_registry_surface() {
    let mut reg = GateEvaluatorRegistry::default();
    reg.register_kleisli(KleisliUnitEvaluator::new());
    let verdict = reg
        .evaluate_kleisli_unit(KLEISLI_UNIT_CATALOG_ID)
        .expect("kleisli unit registered on manifold gate registry");
    assert_eq!(verdict, AdmissibilityVerdict::Accepted);
    assert_eq!(KLEISLI_UNIT_CATALOG_ID, "umst.gate.kleisli_unit");
}

#[test]
fn swarm_0831_93_semantic_lane_bridge_live() {
    assert!(web_semantic_lane_overlap_valid());
    assert_eq!(slice_layout::BEHAVIOR_UCRS.start, 56);
    assert_eq!(SEMANTIC_LANE_BASE, 57);
    assert_eq!(LANE_RELATION_GRAPH, SEMANTIC_LANE_BASE + 1);
}

#[test]
fn swarm_0831_93_deepen_probe_honest_partial() {
    let probe = manifold_swarm_0831_93_deepen_probe();
    assert!(manifold_swarm_0831_93_deepen_honest(&probe));
    assert!(!probe.production_wired);
    assert!(!probe.flip_authorized);

    let night = manifold_night_2350_deepen_probe();
    assert!(manifold_night_2350_deepen_honest(&night));
    assert!(!night.production_wired);
}

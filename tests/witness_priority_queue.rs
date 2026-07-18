// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Adaptive catalog module priority (tests only; feeds `docs/ADAPTIVE_WITNESS_COVERAGE.md`).
//! TCB: `physicalSecondLaw` only.

use umst_manifold::manifest::{UmstManifestBuilder, WitnessPriorityQueue};
use umst_manifold::runtime::catalog::{
    tcb_axiom_token_allowed, traceability::LANDAUER_CBF_CATALOG_ID, WitnessLearningSignal,
    WitnessTcbAxiom, LANDAUER_LAW_LEAN_MODULE, PHYSICAL_SECOND_LAW_AXIOM,
};

#[test]
fn runtime_queue_ranks_landauer_law_after_rejects_and_learning() {
    let mut q = WitnessPriorityQueue::for_adaptive_coverage();
    assert_eq!(q.tcb_axiom(), WitnessTcbAxiom::PhysicalSecondLaw);
    assert_eq!(q.tcb_axiom().as_str(), PHYSICAL_SECOND_LAW_AXIOM);

    q.record_reject(LANDAUER_CBF_CATALOG_ID);
    q.record_reject("umst.formal.catalog_lock");
    q.apply_learning_signals(&[WitnessLearningSignal {
        catalog_id: LANDAUER_CBF_CATALOG_ID,
        weight: 8,
    }]);

    let top = q.ordered_modules();
    assert_eq!(top[0].0, LANDAUER_LAW_LEAN_MODULE);
    assert!(
        top[0].1
            > top
                .iter()
                .find(|(m, _)| *m == "EpistemicRuntimeContract")
                .expect("EpistemicRuntimeContract module must rank in adaptive queue after rejects")
                .1
    );
}

#[test]
fn manifest_optional_witness_priority_queue() {
    let mut q = WitnessPriorityQueue::for_adaptive_coverage();
    q.record_reject(LANDAUER_CBF_CATALOG_ID);

    let manifest = UmstManifestBuilder::default()
        .witness_priority_queue(q)
        .build();

    let attached = manifest
        .witness_priority_queue
        .as_ref()
        .expect("UmstManifestBuilder witness_priority_queue attachment witness (FP §6 Track G formal proof harness)");
    assert!(attached.is_enabled());
    assert_eq!(attached.priority_of_module(LANDAUER_LAW_LEAN_MODULE), 15);
}

#[test]
fn default_manifest_has_no_priority_queue() {
    let manifest = UmstManifestBuilder::default().build();
    assert!(manifest.witness_priority_queue.is_none());
}

#[test]
fn tcb_allowlist_rejects_unknown_axiom_tokens() {
    assert!(tcb_axiom_token_allowed("NONE"));
    assert!(tcb_axiom_token_allowed(PHYSICAL_SECOND_LAW_AXIOM));
    assert!(!tcb_axiom_token_allowed("clausiusDuhemAxiom"));
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Minimal smoke target for `--features formal-witness` CI matrices.
//! Adaptive module priority: `tests/witness_priority_queue.rs` + `WitnessPriorityQueue`.

use umst_manifold::ai::formal::FormalReject;
use umst_manifold::runtime::catalog::{
    traceability::LANDAUER_CBF_CATALOG_ID, WitnessLearningSignal, WitnessPriorityQueue,
    LANDAUER_LAW_LEAN_MODULE,
};

#[test]
fn formal_witness_smoke_compiles() {
    assert!(true);
}

#[test]
fn formal_reject_feeds_witness_priority_queue() {
    let mut q = WitnessPriorityQueue::for_adaptive_coverage();
    let rej = FormalReject::ThermodynamicControlBarrier {
        catalog_id: LANDAUER_CBF_CATALOG_ID,
        detail: "insufficient dissipation".into(),
    };
    q.record_formal_reject(&rej);
    q.apply_learning_signals(&[WitnessLearningSignal {
        catalog_id: LANDAUER_CBF_CATALOG_ID,
        weight: 2,
    }]);
    assert_eq!(q.reject_count(LANDAUER_CBF_CATALOG_ID), 1);
    assert_eq!(q.ordered_modules()[0].0, LANDAUER_LAW_LEAN_MODULE);
}

#[cfg(feature = "formal-witness")]
#[test]
fn manifold_gateway_new_pins_lock_digest() {
    use burn_ndarray::NdArray;
    use umst_manifold::ai::ppo::ManifoldGateway;
    use umst_manifold::core::traits::IScienceCartridge;
    use umst_manifold::runtime::catalog::lock_upstream_catalog_digest_bytes;

    struct Stub;
    impl<B: burn::tensor::backend::Backend<FloatElem = f32>> IScienceCartridge<B> for Stub {
        fn compute_all(
            &self,
            _mix: &umst_manifold::core::tensors::MixTensor<B>,
        ) -> umst_manifold::core::traits::PhysicalResult<B> {
            unimplemented!()
        }
        fn compute_topology(
            &self,
            _m: &umst_manifold::core::tensors::UnifiedMaterialStateTensor<B>,
        ) -> umst_manifold::core::traits::PhysicalResult<B> {
            unimplemented!()
        }
    }

    let g: ManifoldGateway<NdArray<f32>, Stub> =
        ManifoldGateway::new(Stub, 300.0, 1.0e-12);
    assert_eq!(
        g.expected_catalog_schema_digest,
        Some(lock_upstream_catalog_digest_bytes())
    );
}

#[cfg(feature = "formal-witness")]
#[test]
fn catalog_digest_mismatch_bumps_catalog_lock_module() {
    use umst_manifold::ai::formal::FormalReject;

    let mut q = WitnessPriorityQueue::for_adaptive_coverage();
    let rej = FormalReject::CatalogSchemaDigestMismatch {
        expected: [1u8; 32],
        observed: [2u8; 32],
    };
    q.record_formal_reject(&rej);
    assert!(q.priority_of_module("EpistemicRuntimeContract") > 0);
    assert!(q.priority_of_module("FormalFoundations") > 0);
}

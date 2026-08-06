// SPDX-License-Identifier: MIT
// HCOM-006 + IDEA-003 @ 19:15 IST — additive 64-lane semantic schema v1 migration.
// IDEA-003 @ 20:33 IST deepen — error paths, batch downgrade, reserved-band invariants.
//
// Done-when: v0→v1 migration preserves pinned physical lanes (0..7); semantic band 57..64
// zero-fills on upgrade; `SEMANTIC_LANE_SCHEMA_V1` documented in `docs/SEMANTIC_LANE_SCHEMA_V1.md`.

use umst_manifold::core::{
    consistency_defect_from_dec_stub, migrate_carrier_batch, migrate_carrier_row,
    stub_dec_graph_consistency, validate_v1_layout_invariants, CarrierSchemaVersion,
    SemanticLaneBundleV1, SemanticLaneId, SemanticLaneMigrationError, UMST_CARRIER_LANE_COUNT,
    UMST_SCALAR_CHANNEL_COUNT, LANE_RELATION_GRAPH, LANE_TOPOLOGY_SIGNATURE, RESERVED_LANE_BASE,
    RESERVED_LANE_COUNT, SEMANTIC_LANE_BASE, SEMANTIC_LANE_SCHEMA_V1,
};

#[test]
fn idea_003_schema_v1_revision_and_physical_pin_invariants() {
    assert_eq!(SEMANTIC_LANE_SCHEMA_V1, 1);
    assert_eq!(
        CarrierSchemaVersion::V1SemanticExtended.semantic_schema_revision(),
        SEMANTIC_LANE_SCHEMA_V1
    );
    assert_eq!(CarrierSchemaVersion::V0PhysicalOnly.lane_count(), UMST_SCALAR_CHANNEL_COUNT);
    assert_eq!(
        CarrierSchemaVersion::V1SemanticExtended.lane_count(),
        UMST_CARRIER_LANE_COUNT
    );
    assert_eq!(SEMANTIC_LANE_BASE, 57);
    assert_eq!(SEMANTIC_LANE_BASE + SemanticLaneId::ALL_V1.len(), UMST_CARRIER_LANE_COUNT);
    assert!(validate_v1_layout_invariants());
}

#[test]
fn reserved_lane_band_dimensions_match_blueprint() {
    assert_eq!(RESERVED_LANE_BASE, UMST_SCALAR_CHANNEL_COUNT);
    assert_eq!(RESERVED_LANE_COUNT, SEMANTIC_LANE_BASE - RESERVED_LANE_BASE);
    assert_eq!(RESERVED_LANE_BASE + RESERVED_LANE_COUNT, SEMANTIC_LANE_BASE);
}

#[test]
fn v1_to_v0_downgrade_preserves_physical_only_prefix() {
    let physical: Vec<f64> = (1..=UMST_SCALAR_CHANNEL_COUNT).map(|i| i as f64).collect();
    let mut v1 = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &physical,
    )
    .expect("seed v1");
    v1[LANE_RELATION_GRAPH] = 0.77;

    let v0 = migrate_carrier_row(
        CarrierSchemaVersion::V1SemanticExtended,
        CarrierSchemaVersion::V0PhysicalOnly,
        &v1,
    )
    .expect("v1→v0");
    assert_eq!(v0, physical);
}

#[test]
fn v1_to_v0_batch_downgrade_preserves_physical_only_prefix() {
    let nodes = 4_usize;
    let physical: Vec<f64> = (0..nodes * UMST_SCALAR_CHANNEL_COUNT)
        .map(|i| i as f64 * 0.1)
        .collect();

    let v1 = migrate_carrier_batch(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        nodes,
        &physical,
    )
    .expect("batch v0→v1");

    let v0 = migrate_carrier_batch(
        CarrierSchemaVersion::V1SemanticExtended,
        CarrierSchemaVersion::V0PhysicalOnly,
        nodes,
        &v1,
    )
    .expect("batch v1→v0");

    assert_eq!(v0, physical);
}

#[test]
fn v0_to_v1_batch_migration_preserves_physical_prefix() {
    let nodes = 3_usize;
    let physical: Vec<f64> = (0..nodes * UMST_SCALAR_CHANNEL_COUNT)
        .map(|i| i as f64 * 0.01)
        .collect();

    let migrated = migrate_carrier_batch(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        nodes,
        &physical,
    )
    .expect("batch v0→v1");

    assert_eq!(migrated.len(), nodes * UMST_CARRIER_LANE_COUNT);
    for node in 0..nodes {
        let phys = &physical[node * UMST_SCALAR_CHANNEL_COUNT..(node + 1) * UMST_SCALAR_CHANNEL_COUNT];
        let row = &migrated[node * UMST_CARRIER_LANE_COUNT..(node + 1) * UMST_CARRIER_LANE_COUNT];
        assert_eq!(&row[..UMST_SCALAR_CHANNEL_COUNT], phys);
        assert!(row[UMST_SCALAR_CHANNEL_COUNT..SEMANTIC_LANE_BASE]
            .iter()
            .all(|v| *v == 0.0));
        assert!(row[SEMANTIC_LANE_BASE..].iter().all(|v| *v == 0.0));
    }
}

#[test]
fn v1_reupgrade_after_downgrade_zeroes_semantic_band() {
    let physical: Vec<f64> = (1..=UMST_SCALAR_CHANNEL_COUNT).map(|i| i as f64).collect();
    let mut v1 = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &physical,
    )
    .expect("seed v1");
    v1[LANE_RELATION_GRAPH] = 0.88;
    v1[LANE_TOPOLOGY_SIGNATURE] = 0.99;

    let v0 = migrate_carrier_row(
        CarrierSchemaVersion::V1SemanticExtended,
        CarrierSchemaVersion::V0PhysicalOnly,
        &v1,
    )
    .expect("v1→v0");
    let reup = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &v0,
    )
    .expect("v0→v1 re-upgrade");

    assert_eq!(&reup[..UMST_SCALAR_CHANNEL_COUNT], physical.as_slice());
    assert!(reup[SEMANTIC_LANE_BASE..].iter().all(|v| *v == 0.0));
}

#[test]
fn v1_to_v1_idempotent_on_physical_and_semantic_bands() {
    let mut row = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
    )
    .expect("seed v1 row");

    row[LANE_RELATION_GRAPH] = 0.42;
    row[LANE_TOPOLOGY_SIGNATURE] = 0.99;

    let roundtrip = migrate_carrier_row(
        CarrierSchemaVersion::V1SemanticExtended,
        CarrierSchemaVersion::V1SemanticExtended,
        &row,
    )
    .expect("v1→v1");
    assert_eq!(roundtrip, row);
}

#[test]
fn v1_to_v1_batch_preserves_semantic_bundle() {
    let nodes = 2_usize;
    let physical: Vec<f64> = (0..nodes * UMST_SCALAR_CHANNEL_COUNT)
        .map(|i| (i + 1) as f64)
        .collect();
    let mut v1 = migrate_carrier_batch(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        nodes,
        &physical,
    )
    .expect("batch v0→v1");

    for node in 0..nodes {
        let offset = node * UMST_CARRIER_LANE_COUNT;
        v1[offset + LANE_RELATION_GRAPH] = (node + 1) as f64 * 0.1;
    }

    let roundtrip = migrate_carrier_batch(
        CarrierSchemaVersion::V1SemanticExtended,
        CarrierSchemaVersion::V1SemanticExtended,
        nodes,
        &v1,
    )
    .expect("batch v1→v1");
    assert_eq!(roundtrip, v1);
}

#[test]
fn migration_rejects_source_width_mismatch_on_row() {
    let err = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &[1.0, 2.0],
    )
    .expect_err("short row");
    assert_eq!(
        err,
        SemanticLaneMigrationError::SourceWidthMismatch {
            expected: UMST_SCALAR_CHANNEL_COUNT,
            found: 2,
        }
    );
}

#[test]
fn migration_rejects_source_width_mismatch_on_batch() {
    let err = migrate_carrier_batch(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        2,
        &[1.0, 2.0, 3.0],
    )
    .expect_err("short batch");
    assert_eq!(
        err,
        SemanticLaneMigrationError::SourceWidthMismatch {
            expected: 2 * UMST_SCALAR_CHANNEL_COUNT,
            found: 3,
        }
    );
}

#[test]
fn semantic_lane_indices_are_monotonic_and_within_carrier() {
    let mut prev = 0_usize;
    for lane in SemanticLaneId::ALL_V1 {
        let idx = lane.carrier_index();
        assert!(idx < UMST_CARRIER_LANE_COUNT);
        assert!(idx > prev);
        prev = idx;
    }
    assert_eq!(prev, UMST_CARRIER_LANE_COUNT - 1);
}

#[test]
fn dec_graph_stub_flags_relation_without_topology() {
    let mut row = vec![0.0; UMST_CARRIER_LANE_COUNT];
    row[LANE_RELATION_GRAPH] = 0.5;
    let report = stub_dec_graph_consistency(&row);
    assert_eq!(report.relation_graph_drift, 0.5);
    assert_eq!(report.boundary_of_boundary_defect, 0.0);
    let defect = consistency_defect_from_dec_stub(&row);
    assert!((defect - 0.5).abs() < f64::EPSILON);
}

#[test]
fn semantic_lane_bundle_roundtrip_on_row() {
    let mut row = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &vec![0.0; UMST_SCALAR_CHANNEL_COUNT],
    )
    .expect("empty v1 row");

    let bundle = SemanticLaneBundleV1 {
        concept_id: 11.0,
        relation_graph: 22.0,
        context_vector: 33.0,
        timestamp: 1_711_000.0,
        speaker_id: 7.0,
        mi_value: 0.85,
        topology_signature: 99.0,
    };
    bundle.write_into_row(&mut row);
    assert_eq!(SemanticLaneBundleV1::read_from_row(&row), bundle);
}

#[test]
fn bundle_write_preserves_physical_prefix() {
    let physical: Vec<f64> = (1..=UMST_SCALAR_CHANNEL_COUNT).map(|i| i as f64).collect();
    let mut row = migrate_carrier_row(
        CarrierSchemaVersion::V0PhysicalOnly,
        CarrierSchemaVersion::V1SemanticExtended,
        &physical,
    )
    .expect("seed v1");

    let bundle = SemanticLaneBundleV1 {
        relation_graph: 0.5,
        ..SemanticLaneBundleV1::default()
    };
    bundle.write_into_row(&mut row);
    assert_eq!(&row[..UMST_SCALAR_CHANNEL_COUNT], physical.as_slice());
    assert!(row[UMST_SCALAR_CHANNEL_COUNT..SEMANTIC_LANE_BASE]
        .iter()
        .all(|v| *v == 0.0));
}

#[test]
fn all_v1_lane_names_match_blueprint() {
    assert_eq!(SemanticLaneId::ConceptId.lane_name(), "ConceptID");
    assert_eq!(SemanticLaneId::TopologySignature.lane_name(), "TopologySignature");
    assert_eq!(SemanticLaneId::ALL_V1.len(), 7);
}

# Semantic Lane Schema v1 (HCOM-006)

Additive semantic lanes on the 64-lane UMST carrier (blueprint §3.1).

## Version

| Constant | Value |
|----------|------:|
| `SEMANTIC_LANE_SCHEMA_V1` | `1` |
| `UMST_CARRIER_LANE_COUNT` | `64` |
| `UMST_SCALAR_CHANNEL_COUNT` | `7` (pinned by `artifacts/scalar_layout.lock.json`) |

## Lane bands

| Band | Indices | Role |
|------|---------|------|
| Physical (pinned) | `0..7` | Nodal scalars from `scalar_layout.lock.json` |
| Reserved growth | `7..57` | Additive physical extension |
| Semantic v1 | `57..64` | Blueprint §3.1 meaning fields |

## Semantic v1 lane map

| Index | Lane | `SemanticLaneId` |
|------:|------|------------------|
| 57 | ConceptID | `ConceptId` |
| 58 | RelationGraph | `RelationGraph` |
| 59 | ContextVector | `ContextVector` |
| 60 | Timestamp | `Timestamp` |
| 61 | SpeakerID | `SpeakerId` |
| 62 | MIValue | `MiValue` |
| 63 | TopologySignature | `TopologySignature` |

## Migration (`CarrierSchemaVersion`)

- **V0 → V1**: copy physical prefix; zero-fill reserved + semantic bands.
- **V1 → V1**: idempotent copy of full row.
- **V1 → V0**: copy physical prefix only (semantic lanes dropped).
- **V0 → V1 → V0 → V1**: re-upgrade zero-fills semantic band (honest additive contract).

### Error paths

`SemanticLaneMigrationError::SourceWidthMismatch` when source slice width ≠ `from.lane_count()` (row) or `nodes * from.lane_count()` (batch).

SSOT: `src/core/semantic_lane_schema.rs` · mirror: `umst-semantics/src/semantic_lane.rs`

Tests: `tests/semantic_lane_schema_migration.rs` (manifold · **15** tests) · `umst-semantics/tests/semantic_lane_migration.rs` (**15** tests)

IDEA-003 deepen @ 20:33 IST — batch downgrade, width-mismatch rejection, reserved-band invariants, re-upgrade honesty.

## WEB-005 overlap (informational tensor)

The WEB-004 `WebStateTensor` and UMST carrier share index space on the high band:

| Index | WEB-004 slice | UMST carrier |
|------:|---------------|--------------|
| 56 | `BEHAVIOR_UCRS` head (UCRS anchor) | reserved growth band |
| 57..64 | behavior tail | semantic v1 (`SemanticLaneBundleV1`) |

SSOT bridge: `src/web_constitutive/semantic_residual.rs` · tests: `tests/web_semantic_lane_bridge.rs`

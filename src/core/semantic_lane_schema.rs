// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
//
// HCOM-006 @ 17:11 IST — additive semantic lanes on the 64-lane UMST carrier (blueprint §3.1).
// IDEA-003 @ 19:15 IST — migration test + version bump doc (`docs/SEMANTIC_LANE_SCHEMA_V1.md`).
// IDEA-003 @ 20:33 IST deepen — reserved-band constants + `validate_v1_layout_invariants` mirror.
// W29-029 deepen — honest fences (no invent GREEN / PRODUCTION_WIRED / MASTER / OP-5); fallible row IO.
// W29-029 grok lane — fallible DEC stub + semantic-band zero probe + fence refused count.
//
// Physical nodal scalars remain pinned by `artifacts/scalar_layout.lock.json` (`umst_schema`).
// Semantic lanes v1 occupy the high carrier band without reinterpreting physical indices.

use super::umst_schema::UMST_SCALAR_CHANNEL_COUNT;

/// W29 deepen cell — semantic lane schema honesty.
pub const W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL: &str = "W29-029-SEMANTIC_LANE_SCHEMA";

/// Fixed-width UMST carrier lane count (deployment contract — README §2.1).
pub const UMST_CARRIER_LANE_COUNT: usize = 64;

/// Additive semantic lane schema revision (blueprint §3.1 versioned lanes).
pub const SEMANTIC_LANE_SCHEMA_V1: u32 = 1;

/// First semantic lane index in the 64-lane carrier (physical + reserved band below).
pub const SEMANTIC_LANE_BASE: usize = UMST_CARRIER_LANE_COUNT - SEMANTIC_LANE_V1_COUNT;

/// Number of semantic lanes introduced in schema v1.
pub const SEMANTIC_LANE_V1_COUNT: usize = 7;

/// Reserved growth band between pinned physical scalars and semantic v1 lanes.
pub const RESERVED_LANE_BASE: usize = UMST_SCALAR_CHANNEL_COUNT;
pub const RESERVED_LANE_COUNT: usize = SEMANTIC_LANE_BASE - RESERVED_LANE_BASE;

// --- Semantic lane indices (additive v1) ------------------------------------

/// Stable concept / shape digest lane (content-addressed id projection).
pub const LANE_CONCEPT_ID: usize = SEMANTIC_LANE_BASE;
/// Relation-graph incidence digest lane (DEC/graph analog anchor).
pub const LANE_RELATION_GRAPH: usize = SEMANTIC_LANE_BASE + 1;
/// Shared-context embedding lane (speaker/time/history vector scalar projection).
pub const LANE_CONTEXT_VECTOR: usize = SEMANTIC_LANE_BASE + 2;
/// Communicative act timestamp lane (monotonic versioning scalar).
pub const LANE_TIMESTAMP: usize = SEMANTIC_LANE_BASE + 3;
/// Speaker / agent identity lane.
pub const LANE_SPEAKER_ID: usize = SEMANTIC_LANE_BASE + 4;
/// Mutual-information observation lane (MI-governed refinement hook).
pub const LANE_MI_VALUE: usize = SEMANTIC_LANE_BASE + 5;
/// Topology signature lane (structural invariance witness scalar).
pub const LANE_TOPOLOGY_SIGNATURE: usize = SEMANTIC_LANE_BASE + 6;

/// Honest posture tag — schema surface landed; HCOM-008 DEC hook remains stub.
pub const SEMANTIC_LANE_SCHEMA_POSTURE_TAG: &str = "honest-hcom-006-schema-v1-stub-dec";

/// Schema revision doc cross-ref (`docs/SEMANTIC_LANE_SCHEMA_V1.md` when published).
pub const SEMANTIC_LANE_SCHEMA_DOC_REF: &str = "docs/SEMANTIC_LANE_SCHEMA_V1.md";

/// Operator-visible honesty string — does **not** authorize production flip, physics GREEN, MASTER, or OP-5.
pub const SEMANTIC_LANE_SCHEMA_HONEST_FENCE: &str =
    "schema_surface_wired=true|layout_invariants=true|dec_hook_stub=true|hcom008_open=true|production_wired=false|physics_green=false|master=false|flip_authorized=false|op5_claimed=false";

/// Schema surface landed (typed indices + migration + manifest) — not production fleet claim.
pub const SEMANTIC_LANE_SCHEMA_SURFACE_WIRED: bool = true;

/// Honest refusal — additive schema is staging surface, not production-wired fleet claim.
pub const SEMANTIC_LANE_SCHEMA_PRODUCTION_WIRED: bool = false;

/// Honest refusal — schema pin is not a physics GREEN / oracle certification.
pub const SEMANTIC_LANE_SCHEMA_PHYSICS_GREEN: bool = false;

/// Honest refusal — no MASTER / fleet-complete posture at semantic lane seam.
pub const SEMANTIC_LANE_SCHEMA_MASTER: bool = false;

/// Honest refusal — production flip not authorized (HCOM-008 DEC composition still open).
pub const SEMANTIC_LANE_SCHEMA_FLIP_AUTHORIZED: bool = false;

/// Honest refusal — no OP-5 / fleet-master claim at semantic lane schema seam.
pub const SEMANTIC_LANE_SCHEMA_OP5_CLAIMED: bool = false;

/// Fence facet inventory size (wired + deferred).
pub const SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT: usize = 9;

/// Fence facets measured/wired today (schema surface + layout + stub DEC + open HCOM-008).
pub const SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT: usize = 4;

/// Deferred / refused fence facets (facet_count − wired_count).
pub const SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT: usize =
    SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT - SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT;

/// One facet of the semantic-lane production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticLaneSchemaFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// Semantic-lane production fence facet inventory (honest posture SSOT).
pub const SEMANTIC_LANE_SCHEMA_FENCE_FACETS: &[SemanticLaneSchemaFenceFacet] = &[
    SemanticLaneSchemaFenceFacet {
        facet: "schema_surface_wired",
        wired: true,
        owning_slice: W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL,
    },
    SemanticLaneSchemaFenceFacet {
        facet: "layout_invariants",
        wired: true,
        owning_slice: W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL,
    },
    SemanticLaneSchemaFenceFacet {
        facet: "dec_hook_stub",
        wired: true,
        owning_slice: W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL,
    },
    SemanticLaneSchemaFenceFacet {
        facet: "hcom008_composition_open",
        wired: true,
        owning_slice: "hcom-008-deferred",
    },
    SemanticLaneSchemaFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    SemanticLaneSchemaFenceFacet {
        facet: "physics_green",
        wired: false,
        owning_slice: "deferred-physics-oracle",
    },
    SemanticLaneSchemaFenceFacet {
        facet: "master",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    SemanticLaneSchemaFenceFacet {
        facet: "flip_authorized",
        wired: false,
        owning_slice: "deferred-hcom-008-signoff",
    },
    SemanticLaneSchemaFenceFacet {
        facet: "op5_claimed",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

/// Honest production wiring — **false** until HCOM-008 + fleet measured eval.
#[must_use]
pub const fn semantic_lane_schema_production_wired() -> bool {
    SEMANTIC_LANE_SCHEMA_PRODUCTION_WIRED
}

/// Honest physics GREEN claim — **false** (schema ≠ oracle).
#[must_use]
pub const fn semantic_lane_schema_physics_green() -> bool {
    SEMANTIC_LANE_SCHEMA_PHYSICS_GREEN
}

/// Honest master-tier claim — **false** until orchestrator pin.
#[must_use]
pub const fn semantic_lane_schema_master() -> bool {
    SEMANTIC_LANE_SCHEMA_MASTER
}

/// Honest flip authorization — **false** while DEC hook is stub.
#[must_use]
pub const fn semantic_lane_schema_flip_authorized() -> bool {
    SEMANTIC_LANE_SCHEMA_FLIP_AUTHORIZED
}

/// Honest OP-5 claim — **false** (schema surface ≠ OP-5 fleet pin).
#[must_use]
pub const fn semantic_lane_schema_op5_claimed() -> bool {
    SEMANTIC_LANE_SCHEMA_OP5_CLAIMED
}

const _: () = assert!(!SEMANTIC_LANE_SCHEMA_PRODUCTION_WIRED);
const _: () = assert!(!SEMANTIC_LANE_SCHEMA_PHYSICS_GREEN);
const _: () = assert!(!SEMANTIC_LANE_SCHEMA_MASTER);
const _: () = assert!(!SEMANTIC_LANE_SCHEMA_FLIP_AUTHORIZED);
const _: () = assert!(!SEMANTIC_LANE_SCHEMA_OP5_CLAIMED);
const _: () = assert!(SEMANTIC_LANE_SCHEMA_SURFACE_WIRED);
const _: () = assert!(
    SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT
        == SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT - SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT
);

/// Carrier lane band within the 64-lane UMST tensor row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CarrierLaneBand {
    /// Pinned physical scalars (`0 .. UMST_SCALAR_CHANNEL_COUNT`).
    Physical,
    /// Growth band between physical and semantic v1 lanes.
    Reserved,
    /// HCOM-006 semantic v1 lanes (`SEMANTIC_LANE_BASE .. UMST_CARRIER_LANE_COUNT`).
    SemanticV1,
}

/// One manifest row for audit / census tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticLaneManifestRow {
    pub carrier_index: usize,
    pub lane_name: &'static str,
    pub band: CarrierLaneBand,
    pub schema_revision: u32,
}

/// Typed posture probe for meta / fleet honesty gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticLaneSchemaPostureProbe {
    pub deepen_cell: &'static str,
    pub posture_tag: &'static str,
    pub schema_revision: u32,
    pub carrier_lane_count: usize,
    pub semantic_lane_count: usize,
    pub reserved_lane_count: usize,
    pub dec_hook_is_stub: bool,
    pub layout_invariants_ok: bool,
    pub schema_surface_wired: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub fence_refused_count: usize,
    pub production_wired: bool,
    pub physics_green: bool,
    pub master: bool,
    pub flip_authorized: bool,
    pub op5_claimed: bool,
    pub honest_fence: &'static str,
}

/// Runtime validation errors on v1 carrier rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticLaneSchemaError {
    /// Row width does not match the declared schema generation.
    RowWidthMismatch { expected: usize, found: usize },
    /// Carrier index is outside the semantic v1 band.
    NotSemanticLaneIndex { index: usize },
}

/// Carrier schema generation — physical-only vs semantic-extended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CarrierSchemaVersion {
    /// Physical lanes only (`F_scalars == UMST_SCALAR_CHANNEL_COUNT`).
    V0PhysicalOnly,
    /// Physical + reserved + semantic v1 lanes (`F_scalars == UMST_CARRIER_LANE_COUNT`).
    V1SemanticExtended,
}

impl CarrierSchemaVersion {
    /// Lane width for this schema generation.
    #[must_use]
    pub const fn lane_count(self) -> usize {
        match self {
            Self::V0PhysicalOnly => UMST_SCALAR_CHANNEL_COUNT,
            Self::V1SemanticExtended => UMST_CARRIER_LANE_COUNT,
        }
    }

    /// Semantic schema revision (`0` before semantic lanes).
    #[must_use]
    pub const fn semantic_schema_revision(self) -> u32 {
        match self {
            Self::V0PhysicalOnly => 0,
            Self::V1SemanticExtended => SEMANTIC_LANE_SCHEMA_V1,
        }
    }
}

/// Typed semantic lane selector (compile-time index witness).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SemanticLaneId {
    ConceptId,
    RelationGraph,
    ContextVector,
    Timestamp,
    SpeakerId,
    MiValue,
    TopologySignature,
}

impl SemanticLaneId {
    /// All v1 semantic lanes in carrier index order.
    pub const ALL_V1: [Self; SEMANTIC_LANE_V1_COUNT] = [
        Self::ConceptId,
        Self::RelationGraph,
        Self::ContextVector,
        Self::Timestamp,
        Self::SpeakerId,
        Self::MiValue,
        Self::TopologySignature,
    ];

    /// Carrier index for this semantic lane.
    #[must_use]
    pub const fn carrier_index(self) -> usize {
        match self {
            Self::ConceptId => LANE_CONCEPT_ID,
            Self::RelationGraph => LANE_RELATION_GRAPH,
            Self::ContextVector => LANE_CONTEXT_VECTOR,
            Self::Timestamp => LANE_TIMESTAMP,
            Self::SpeakerId => LANE_SPEAKER_ID,
            Self::MiValue => LANE_MI_VALUE,
            Self::TopologySignature => LANE_TOPOLOGY_SIGNATURE,
        }
    }

    /// Stable string id for audit / manifest rows.
    #[must_use]
    pub const fn lane_name(self) -> &'static str {
        match self {
            Self::ConceptId => "ConceptID",
            Self::RelationGraph => "RelationGraph",
            Self::ContextVector => "ContextVector",
            Self::Timestamp => "Timestamp",
            Self::SpeakerId => "SpeakerID",
            Self::MiValue => "MIValue",
            Self::TopologySignature => "TopologySignature",
        }
    }

    /// Reverse map from carrier index to semantic lane id.
    #[must_use]
    pub const fn try_from_carrier_index(index: usize) -> Option<Self> {
        match index {
            LANE_CONCEPT_ID => Some(Self::ConceptId),
            LANE_RELATION_GRAPH => Some(Self::RelationGraph),
            LANE_CONTEXT_VECTOR => Some(Self::ContextVector),
            LANE_TIMESTAMP => Some(Self::Timestamp),
            LANE_SPEAKER_ID => Some(Self::SpeakerId),
            LANE_MI_VALUE => Some(Self::MiValue),
            LANE_TOPOLOGY_SIGNATURE => Some(Self::TopologySignature),
            _ => None,
        }
    }
}

/// Semantic lane bundle for one communicative act (scalar carrier projection).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SemanticLaneBundleV1 {
    pub concept_id: f64,
    pub relation_graph: f64,
    pub context_vector: f64,
    pub timestamp: f64,
    pub speaker_id: f64,
    pub mi_value: f64,
    pub topology_signature: f64,
}

impl SemanticLaneBundleV1 {
    /// Scalar for one semantic lane selector.
    #[must_use]
    pub fn field(self, lane: SemanticLaneId) -> f64 {
        match lane {
            SemanticLaneId::ConceptId => self.concept_id,
            SemanticLaneId::RelationGraph => self.relation_graph,
            SemanticLaneId::ContextVector => self.context_vector,
            SemanticLaneId::Timestamp => self.timestamp,
            SemanticLaneId::SpeakerId => self.speaker_id,
            SemanticLaneId::MiValue => self.mi_value,
            SemanticLaneId::TopologySignature => self.topology_signature,
        }
    }

    /// Write one lane scalar into the bundle.
    pub fn set_field(&mut self, lane: SemanticLaneId, value: f64) {
        match lane {
            SemanticLaneId::ConceptId => self.concept_id = value,
            SemanticLaneId::RelationGraph => self.relation_graph = value,
            SemanticLaneId::ContextVector => self.context_vector = value,
            SemanticLaneId::Timestamp => self.timestamp = value,
            SemanticLaneId::SpeakerId => self.speaker_id = value,
            SemanticLaneId::MiValue => self.mi_value = value,
            SemanticLaneId::TopologySignature => self.topology_signature = value,
        }
    }

    /// Write bundle fields into a v1 carrier row (physical prefix untouched).
    ///
    /// Silent no-op when `row` is shorter than [`UMST_CARRIER_LANE_COUNT`]. Prefer
    /// [`Self::try_write_into_row`] at honesty-gated call sites.
    pub fn write_into_row(&self, row: &mut [f64]) {
        let _ = self.try_write_into_row(row);
    }

    /// Fallible write — refuses short carrier rows.
    pub fn try_write_into_row(&self, row: &mut [f64]) -> Result<(), SemanticLaneSchemaError> {
        validate_v1_carrier_row(row)?;
        row[LANE_CONCEPT_ID] = self.concept_id;
        row[LANE_RELATION_GRAPH] = self.relation_graph;
        row[LANE_CONTEXT_VECTOR] = self.context_vector;
        row[LANE_TIMESTAMP] = self.timestamp;
        row[LANE_SPEAKER_ID] = self.speaker_id;
        row[LANE_MI_VALUE] = self.mi_value;
        row[LANE_TOPOLOGY_SIGNATURE] = self.topology_signature;
        Ok(())
    }

    /// Read bundle fields from a v1 carrier row (missing indices → `0.0`).
    ///
    /// Prefer [`Self::try_read_from_row`] when row width must be enforced.
    #[must_use]
    pub fn read_from_row(row: &[f64]) -> Self {
        Self::try_read_from_row(row).unwrap_or_default()
    }

    /// Fallible read — refuses short carrier rows.
    pub fn try_read_from_row(row: &[f64]) -> Result<Self, SemanticLaneSchemaError> {
        validate_v1_carrier_row(row)?;
        Ok(Self {
            concept_id: row[LANE_CONCEPT_ID],
            relation_graph: row[LANE_RELATION_GRAPH],
            context_vector: row[LANE_CONTEXT_VECTOR],
            timestamp: row[LANE_TIMESTAMP],
            speaker_id: row[LANE_SPEAKER_ID],
            mi_value: row[LANE_MI_VALUE],
            topology_signature: row[LANE_TOPOLOGY_SIGNATURE],
        })
    }
}

/// Errors during additive carrier migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticLaneMigrationError {
    /// Source width does not match the declared `from` schema generation.
    SourceWidthMismatch { expected: usize, found: usize },
}

/// Migrate one nodal carrier row from `from` → `to` (additive, non-destructive on physical lanes).
///
/// - `V0 → V1`: copy physical lanes, zero reserved + semantic bands.
/// - `V1 → V1`: copy overlapping prefix up to `min(from,to)` lane counts.
/// - `V1 → V0`: copy physical prefix only (semantic lanes dropped).
pub fn migrate_carrier_row(
    from: CarrierSchemaVersion,
    to: CarrierSchemaVersion,
    source: &[f64],
) -> Result<Vec<f64>, SemanticLaneMigrationError> {
    let expected = from.lane_count();
    if source.len() != expected {
        return Err(SemanticLaneMigrationError::SourceWidthMismatch {
            expected,
            found: source.len(),
        });
    }

    let mut out = vec![0.0_f64; to.lane_count()];
    let copy_n = from.lane_count().min(to.lane_count());
    out[..copy_n].copy_from_slice(&source[..copy_n]);
    Ok(out)
}

/// Migrate a full `[N, F]` carrier (row-major `N * F` slice) between schema generations.
pub fn migrate_carrier_batch(
    from: CarrierSchemaVersion,
    to: CarrierSchemaVersion,
    nodes: usize,
    source: &[f64],
) -> Result<Vec<f64>, SemanticLaneMigrationError> {
    let from_w = from.lane_count();
    if source.len() != nodes * from_w {
        return Err(SemanticLaneMigrationError::SourceWidthMismatch {
            expected: nodes * from_w,
            found: source.len(),
        });
    }

    let to_w = to.lane_count();
    let mut out = vec![0.0_f64; nodes * to_w];
    for node in 0..nodes {
        let row = migrate_carrier_row(from, to, &source[node * from_w..(node + 1) * from_w])?;
        out[node * to_w..(node + 1) * to_w].copy_from_slice(&row);
    }
    Ok(out)
}

/// DEC / graph consistency hook stub (HCOM-008 deepens; blueprint §3.1 structure lane).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecGraphConsistencyReport {
    /// Discrete exterior calculus boundary-of-boundary defect (stub: `0.0` when unset).
    pub boundary_of_boundary_defect: f64,
    /// Relation-graph drift metric (stub: `0.0` when unset).
    pub relation_graph_drift: f64,
    /// Hook revision label for audit trails.
    pub hook_revision: &'static str,
}

/// Stub hook id — honest placeholder until HCOM-008 composition suite lands.
pub const DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB: &str = "hcom-006-dec-graph-stub-v1";

/// Stub DEC/graph consistency check on a v1 carrier row.
///
/// Returns zero defect when semantic lanes are unset (all zeros). Non-zero relation graph
/// without topology signature yields a positive drift placeholder so downstream gates can wire.
///
/// Silent short-row path uses `.get` defaults — prefer [`try_stub_dec_graph_consistency`] at
/// honesty-gated call sites.
#[must_use]
pub fn stub_dec_graph_consistency(row: &[f64]) -> DecGraphConsistencyReport {
    try_stub_dec_graph_consistency(row).unwrap_or_else(|_| DecGraphConsistencyReport {
        boundary_of_boundary_defect: 0.0,
        relation_graph_drift: 0.0,
        hook_revision: DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB,
    })
}

/// Fallible DEC stub — refuses short carrier rows (honest width gate).
pub fn try_stub_dec_graph_consistency(
    row: &[f64],
) -> Result<DecGraphConsistencyReport, SemanticLaneSchemaError> {
    validate_v1_carrier_row(row)?;
    let relation = row[LANE_RELATION_GRAPH].abs();
    let topology = row[LANE_TOPOLOGY_SIGNATURE].abs();

    let relation_graph_drift = if relation > 0.0 && topology == 0.0 {
        relation
    } else {
        0.0
    };

    Ok(DecGraphConsistencyReport {
        boundary_of_boundary_defect: 0.0,
        relation_graph_drift,
        hook_revision: DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB,
    })
}

/// Map stub defect into semantic admissibility scalar (preview for HCOM-008).
#[must_use]
pub fn consistency_defect_from_dec_stub(row: &[f64]) -> f64 {
    let report = stub_dec_graph_consistency(row);
    report.boundary_of_boundary_defect + report.relation_graph_drift
}

/// Fallible consistency defect — refuses short carrier rows.
pub fn try_consistency_defect_from_dec_stub(row: &[f64]) -> Result<f64, SemanticLaneSchemaError> {
    let report = try_stub_dec_graph_consistency(row)?;
    Ok(report.boundary_of_boundary_defect + report.relation_graph_drift)
}

/// Classify a carrier index into physical / reserved / semantic bands.
#[must_use]
pub const fn carrier_lane_band(index: usize) -> Option<CarrierLaneBand> {
    if index < UMST_SCALAR_CHANNEL_COUNT {
        Some(CarrierLaneBand::Physical)
    } else if index < SEMANTIC_LANE_BASE {
        Some(CarrierLaneBand::Reserved)
    } else if index < UMST_CARRIER_LANE_COUNT {
        Some(CarrierLaneBand::SemanticV1)
    } else {
        None
    }
}

/// Whether `index` is a semantic v1 lane (not physical or reserved).
#[must_use]
pub const fn is_semantic_lane_index(index: usize) -> bool {
    matches!(carrier_lane_band(index), Some(CarrierLaneBand::SemanticV1))
}

/// Require a semantic v1 carrier index — refuse physical / reserved / OOB.
pub fn require_semantic_lane_index(index: usize) -> Result<usize, SemanticLaneSchemaError> {
    if is_semantic_lane_index(index) {
        Ok(index)
    } else {
        Err(SemanticLaneSchemaError::NotSemanticLaneIndex { index })
    }
}

/// Whether the reserved growth band is all zeros on a v1 carrier row.
#[must_use]
pub fn reserved_band_is_zero(row: &[f64]) -> bool {
    if row.len() < SEMANTIC_LANE_BASE {
        return false;
    }
    row[RESERVED_LANE_BASE..SEMANTIC_LANE_BASE]
        .iter()
        .all(|v| *v == 0.0)
}

/// Whether the semantic v1 band is all zeros on a v1 carrier row.
#[must_use]
pub fn semantic_band_is_zero(row: &[f64]) -> bool {
    if row.len() < UMST_CARRIER_LANE_COUNT {
        return false;
    }
    row[SEMANTIC_LANE_BASE..UMST_CARRIER_LANE_COUNT]
        .iter()
        .all(|v| *v == 0.0)
}

/// Count wired semantic-lane fence facets (must match [`SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn semantic_lane_schema_fence_wired_count() -> usize {
    SEMANTIC_LANE_SCHEMA_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count refused / deferred fence facets (must match [`SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT`]).
#[must_use]
pub fn semantic_lane_schema_fence_refused_count() -> usize {
    SEMANTIC_LANE_SCHEMA_FENCE_FACETS
        .iter()
        .filter(|f| !f.wired)
        .count()
}

/// Manifest rows for semantic v1 lanes (audit / census).
#[must_use]
pub const fn semantic_lane_manifest_v1() -> [SemanticLaneManifestRow; SEMANTIC_LANE_V1_COUNT] {
    [
        SemanticLaneManifestRow {
            carrier_index: LANE_CONCEPT_ID,
            lane_name: "ConceptID",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_RELATION_GRAPH,
            lane_name: "RelationGraph",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_CONTEXT_VECTOR,
            lane_name: "ContextVector",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_TIMESTAMP,
            lane_name: "Timestamp",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_SPEAKER_ID,
            lane_name: "SpeakerID",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_MI_VALUE,
            lane_name: "MIValue",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
        SemanticLaneManifestRow {
            carrier_index: LANE_TOPOLOGY_SIGNATURE,
            lane_name: "TopologySignature",
            band: CarrierLaneBand::SemanticV1,
            schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        },
    ]
}

/// Honest posture probe — schema surface wired; production / GREEN / MASTER / OP-5 flip blocked.
#[must_use]
pub fn semantic_lane_schema_posture_probe() -> SemanticLaneSchemaPostureProbe {
    SemanticLaneSchemaPostureProbe {
        deepen_cell: W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL,
        posture_tag: SEMANTIC_LANE_SCHEMA_POSTURE_TAG,
        schema_revision: SEMANTIC_LANE_SCHEMA_V1,
        carrier_lane_count: UMST_CARRIER_LANE_COUNT,
        semantic_lane_count: SEMANTIC_LANE_V1_COUNT,
        reserved_lane_count: RESERVED_LANE_COUNT,
        dec_hook_is_stub: DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB.contains("stub"),
        layout_invariants_ok: validate_v1_layout_invariants(),
        schema_surface_wired: SEMANTIC_LANE_SCHEMA_SURFACE_WIRED,
        fence_facet_count: SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT,
        fence_wired_count: SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT,
        fence_refused_count: SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT,
        production_wired: semantic_lane_schema_production_wired(),
        physics_green: semantic_lane_schema_physics_green(),
        master: semantic_lane_schema_master(),
        flip_authorized: semantic_lane_schema_flip_authorized(),
        op5_claimed: semantic_lane_schema_op5_claimed(),
        honest_fence: SEMANTIC_LANE_SCHEMA_HONEST_FENCE,
    }
}

/// Honesty gate — refuse fake production / GREEN / MASTER / OP-5 claims.
#[must_use]
pub fn semantic_lane_schema_posture_honest(probe: &SemanticLaneSchemaPostureProbe) -> bool {
    probe.deepen_cell == W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL
        && probe.posture_tag == SEMANTIC_LANE_SCHEMA_POSTURE_TAG
        && probe.schema_revision == SEMANTIC_LANE_SCHEMA_V1
        && probe.carrier_lane_count == UMST_CARRIER_LANE_COUNT
        && probe.semantic_lane_count == SEMANTIC_LANE_V1_COUNT
        && probe.reserved_lane_count == RESERVED_LANE_COUNT
        && probe.dec_hook_is_stub
        && probe.layout_invariants_ok
        && probe.schema_surface_wired
        && probe.fence_facet_count == SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT
        && probe.fence_wired_count == SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT
        && probe.fence_refused_count == SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT
        && !probe.production_wired
        && !probe.physics_green
        && !probe.master
        && !probe.flip_authorized
        && !probe.op5_claimed
        && probe.honest_fence.contains("schema_surface_wired=true")
        && probe.honest_fence.contains("dec_hook_stub=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
        && probe.honest_fence.contains("flip_authorized=false")
        && probe.honest_fence.contains("op5_claimed=false")
}

/// Validate semantic-lane posture honesty — fail closed on fake GREEN / production / MASTER / OP-5.
pub fn validate_semantic_lane_schema_honesty() -> Result<(), &'static str> {
    let probe = semantic_lane_schema_posture_probe();
    if probe.production_wired || semantic_lane_schema_production_wired() {
        return Err(
            "SEMANTIC_LANE_SCHEMA_PRODUCTION_WIRED must stay false until HCOM-008 measured",
        );
    }
    if probe.physics_green || semantic_lane_schema_physics_green() {
        return Err("SEMANTIC_LANE_SCHEMA_PHYSICS_GREEN must stay false — schema ≠ oracle");
    }
    if probe.master || semantic_lane_schema_master() {
        return Err("SEMANTIC_LANE_SCHEMA_MASTER must stay false until orchestrator pin");
    }
    if probe.flip_authorized || semantic_lane_schema_flip_authorized() {
        return Err("SEMANTIC_LANE_SCHEMA_FLIP_AUTHORIZED must stay false while DEC hook is stub");
    }
    if probe.op5_claimed || semantic_lane_schema_op5_claimed() {
        return Err("SEMANTIC_LANE_SCHEMA_OP5_CLAIMED must stay false — schema ≠ OP-5 fleet pin");
    }
    if !probe.dec_hook_is_stub {
        return Err("DEC graph consistency hook must remain stub until HCOM-008");
    }
    if !probe.layout_invariants_ok {
        return Err("validate_v1_layout_invariants failed");
    }
    if semantic_lane_schema_fence_wired_count() != SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT {
        return Err("semantic_lane_schema_fence_wired_count drifted from SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT");
    }
    if semantic_lane_schema_fence_refused_count() != SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT {
        return Err("semantic_lane_schema_fence_refused_count drifted from SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT");
    }
    if SEMANTIC_LANE_SCHEMA_FENCE_FACETS.len() != SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT {
        return Err("SEMANTIC_LANE_SCHEMA_FENCE_FACETS length drifted from facet count");
    }
    if !semantic_lane_schema_posture_honest(&probe) {
        return Err("semantic_lane_schema_posture_honest failed");
    }
    Ok(())
}

/// Validate row width for a schema generation.
pub fn validate_carrier_row_width(
    version: CarrierSchemaVersion,
    row: &[f64],
) -> Result<(), SemanticLaneSchemaError> {
    let expected = version.lane_count();
    if row.len() != expected {
        return Err(SemanticLaneSchemaError::RowWidthMismatch {
            expected,
            found: row.len(),
        });
    }
    Ok(())
}

/// Require semantic v1 carrier width (`UMST_CARRIER_LANE_COUNT`).
pub fn validate_v1_carrier_row(row: &[f64]) -> Result<(), SemanticLaneSchemaError> {
    validate_carrier_row_width(CarrierSchemaVersion::V1SemanticExtended, row)
}

/// Layout invariants for schema v1 (const-evaluable; used in migration tests).
#[must_use]
pub const fn validate_v1_layout_invariants() -> bool {
    SEMANTIC_LANE_BASE + SEMANTIC_LANE_V1_COUNT == UMST_CARRIER_LANE_COUNT
        && LANE_TOPOLOGY_SIGNATURE == UMST_CARRIER_LANE_COUNT - 1
        && RESERVED_LANE_BASE == UMST_SCALAR_CHANNEL_COUNT
        && LANE_CONCEPT_ID < LANE_RELATION_GRAPH
        && LANE_RELATION_GRAPH < LANE_CONTEXT_VECTOR
        && LANE_CONTEXT_VECTOR < LANE_TIMESTAMP
        && LANE_TIMESTAMP < LANE_SPEAKER_ID
        && LANE_SPEAKER_ID < LANE_MI_VALUE
        && LANE_MI_VALUE < LANE_TOPOLOGY_SIGNATURE
}

const _: () = assert!(validate_v1_layout_invariants());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_lane_map_matches_blueprint_fields() {
        let names: Vec<_> = SemanticLaneId::ALL_V1
            .iter()
            .map(|lane| lane.lane_name())
            .collect();
        assert_eq!(
            names,
            vec![
                "ConceptID",
                "RelationGraph",
                "ContextVector",
                "Timestamp",
                "SpeakerID",
                "MIValue",
                "TopologySignature",
            ]
        );
    }

    #[test]
    fn migrate_v0_to_v1_preserves_physical_lanes() {
        let physical: Vec<f64> = (0..UMST_SCALAR_CHANNEL_COUNT)
            .map(|i| (i + 1) as f64 * 0.1)
            .collect();
        let migrated = migrate_carrier_row(
            CarrierSchemaVersion::V0PhysicalOnly,
            CarrierSchemaVersion::V1SemanticExtended,
            &physical,
        )
        .expect("v0→v1 migration");

        assert_eq!(migrated.len(), UMST_CARRIER_LANE_COUNT);
        assert_eq!(&migrated[..UMST_SCALAR_CHANNEL_COUNT], physical.as_slice());
        assert!(migrated[RESERVED_LANE_BASE..SEMANTIC_LANE_BASE]
            .iter()
            .all(|v| *v == 0.0));
        assert!(migrated[SEMANTIC_LANE_BASE..].iter().all(|v| *v == 0.0));
    }

    #[test]
    fn dec_graph_stub_zero_when_semantic_lanes_unset() {
        let row = vec![0.0; UMST_CARRIER_LANE_COUNT];
        let report = stub_dec_graph_consistency(&row);
        assert_eq!(report.boundary_of_boundary_defect, 0.0);
        assert_eq!(report.relation_graph_drift, 0.0);
    }

    #[test]
    fn semantic_lane_schema_posture_honest_not_green() {
        validate_semantic_lane_schema_honesty().expect("honest fence");
        let probe = semantic_lane_schema_posture_probe();
        assert!(semantic_lane_schema_posture_honest(&probe));
        assert_eq!(probe.deepen_cell, W29_SEMANTIC_LANE_SCHEMA_DEEPEN_CELL);
        assert!(probe.posture_tag.contains("honest"));
        assert!(probe.dec_hook_is_stub);
        assert!(!probe.production_wired);
        assert!(!probe.physics_green);
        assert!(!probe.master);
        assert!(!probe.flip_authorized);
        assert!(!probe.op5_claimed);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("master=false"));
        assert!(probe.honest_fence.contains("op5_claimed=false"));
        assert_eq!(
            semantic_lane_schema_fence_wired_count(),
            SEMANTIC_LANE_SCHEMA_FENCE_WIRED_COUNT
        );
        assert_eq!(
            semantic_lane_schema_fence_refused_count(),
            SEMANTIC_LANE_SCHEMA_FENCE_REFUSED_COUNT
        );
        assert_eq!(
            probe.fence_wired_count + probe.fence_refused_count,
            probe.fence_facet_count
        );
        assert!(!semantic_lane_schema_production_wired());
        assert!(!semantic_lane_schema_physics_green());
        assert!(!semantic_lane_schema_master());
        assert!(!semantic_lane_schema_flip_authorized());
        assert!(!semantic_lane_schema_op5_claimed());
    }

    #[test]
    fn semantic_lane_schema_honest_fence_no_green_invent() {
        assert!(SEMANTIC_LANE_SCHEMA_HONEST_FENCE.contains("production_wired=false"));
        assert!(SEMANTIC_LANE_SCHEMA_HONEST_FENCE.contains("physics_green=false"));
        assert!(SEMANTIC_LANE_SCHEMA_HONEST_FENCE.contains("master=false"));
        assert!(SEMANTIC_LANE_SCHEMA_HONEST_FENCE.contains("dec_hook_stub=true"));
        assert!(SEMANTIC_LANE_SCHEMA_HONEST_FENCE.contains("op5_claimed=false"));
        assert!(!SEMANTIC_LANE_SCHEMA_PRODUCTION_WIRED);
        assert!(!SEMANTIC_LANE_SCHEMA_PHYSICS_GREEN);
        assert!(!SEMANTIC_LANE_SCHEMA_MASTER);
        assert!(!SEMANTIC_LANE_SCHEMA_FLIP_AUTHORIZED);
        assert!(!SEMANTIC_LANE_SCHEMA_OP5_CLAIMED);
        assert_eq!(
            SEMANTIC_LANE_SCHEMA_FENCE_FACETS.len(),
            SEMANTIC_LANE_SCHEMA_FENCE_FACET_COUNT
        );
    }

    #[test]
    fn require_semantic_lane_index_refuses_physical_and_reserved() {
        assert_eq!(
            require_semantic_lane_index(0),
            Err(SemanticLaneSchemaError::NotSemanticLaneIndex { index: 0 })
        );
        assert_eq!(
            require_semantic_lane_index(RESERVED_LANE_BASE),
            Err(SemanticLaneSchemaError::NotSemanticLaneIndex {
                index: RESERVED_LANE_BASE
            })
        );
        assert_eq!(
            require_semantic_lane_index(LANE_MI_VALUE),
            Ok(LANE_MI_VALUE)
        );
        assert_eq!(
            require_semantic_lane_index(UMST_CARRIER_LANE_COUNT),
            Err(SemanticLaneSchemaError::NotSemanticLaneIndex {
                index: UMST_CARRIER_LANE_COUNT
            })
        );
    }

    #[test]
    fn try_write_read_refuse_short_rows_and_preserve_physical() {
        let mut short = vec![0.0; UMST_SCALAR_CHANNEL_COUNT];
        let bundle = SemanticLaneBundleV1 {
            concept_id: 1.0,
            ..SemanticLaneBundleV1::default()
        };
        assert_eq!(
            bundle.try_write_into_row(&mut short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT,
            })
        );
        assert_eq!(
            SemanticLaneBundleV1::try_read_from_row(&short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT,
            })
        );

        let mut row = vec![0.0; UMST_CARRIER_LANE_COUNT];
        row[0] = 42.0;
        bundle.try_write_into_row(&mut row).expect("write v1");
        assert_eq!(row[0], 42.0);
        assert!(reserved_band_is_zero(&row));
        let read = SemanticLaneBundleV1::try_read_from_row(&row).expect("read v1");
        assert_eq!(read.concept_id, 1.0);
    }

    #[test]
    fn carrier_lane_bands_partition_64_lane_row() {
        assert_eq!(carrier_lane_band(0), Some(CarrierLaneBand::Physical));
        assert_eq!(
            carrier_lane_band(UMST_SCALAR_CHANNEL_COUNT - 1),
            Some(CarrierLaneBand::Physical)
        );
        assert_eq!(
            carrier_lane_band(RESERVED_LANE_BASE),
            Some(CarrierLaneBand::Reserved)
        );
        assert_eq!(
            carrier_lane_band(SEMANTIC_LANE_BASE - 1),
            Some(CarrierLaneBand::Reserved)
        );
        assert_eq!(
            carrier_lane_band(SEMANTIC_LANE_BASE),
            Some(CarrierLaneBand::SemanticV1)
        );
        assert_eq!(
            carrier_lane_band(LANE_TOPOLOGY_SIGNATURE),
            Some(CarrierLaneBand::SemanticV1)
        );
        assert_eq!(carrier_lane_band(UMST_CARRIER_LANE_COUNT), None);
        assert!(is_semantic_lane_index(LANE_MI_VALUE));
        assert!(!is_semantic_lane_index(RESERVED_LANE_BASE));
    }

    #[test]
    fn semantic_lane_id_roundtrip_carrier_index() {
        for lane in SemanticLaneId::ALL_V1 {
            let idx = lane.carrier_index();
            assert_eq!(SemanticLaneId::try_from_carrier_index(idx), Some(lane));
            assert_eq!(
                lane.lane_name(),
                semantic_lane_manifest_v1()
                    .iter()
                    .find(|row| row.carrier_index == idx)
                    .map(|row| row.lane_name)
                    .expect("manifest row")
            );
        }
    }

    #[test]
    fn bundle_write_read_roundtrip_preserves_semantic_fields() {
        let mut row = vec![0.0; UMST_CARRIER_LANE_COUNT];
        row[0] = 42.0;
        let bundle = SemanticLaneBundleV1 {
            concept_id: 1.1,
            relation_graph: 2.2,
            context_vector: 3.3,
            timestamp: 4.4,
            speaker_id: 5.5,
            mi_value: 6.6,
            topology_signature: 7.7,
        };
        bundle.write_into_row(&mut row);
        assert_eq!(row[0], 42.0);
        let read = SemanticLaneBundleV1::read_from_row(&row);
        assert_eq!(read, bundle);
        assert_eq!(read.field(SemanticLaneId::MiValue), 6.6);
    }

    #[test]
    fn migrate_v1_to_v0_drops_semantic_lanes() {
        let mut v1 = vec![0.0; UMST_CARRIER_LANE_COUNT];
        v1[LANE_RELATION_GRAPH] = 9.0;
        let v0 = migrate_carrier_row(
            CarrierSchemaVersion::V1SemanticExtended,
            CarrierSchemaVersion::V0PhysicalOnly,
            &v1,
        )
        .expect("v1→v0 migration");
        assert_eq!(v0.len(), UMST_SCALAR_CHANNEL_COUNT);
        assert!(v0.iter().all(|v| *v == 0.0));
    }

    #[test]
    fn validate_v1_carrier_row_rejects_short_rows() {
        let short = vec![0.0; UMST_SCALAR_CHANNEL_COUNT];
        let err = validate_v1_carrier_row(&short).unwrap_err();
        assert_eq!(
            err,
            SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
    }

    #[test]
    fn migrate_carrier_batch_v0_to_v1_two_nodes() {
        let nodes = 2;
        let from_w = UMST_SCALAR_CHANNEL_COUNT;
        let source: Vec<f64> = (0..nodes * from_w).map(|i| (i + 1) as f64).collect();
        let migrated = migrate_carrier_batch(
            CarrierSchemaVersion::V0PhysicalOnly,
            CarrierSchemaVersion::V1SemanticExtended,
            nodes,
            &source,
        )
        .expect("batch v0→v1");
        assert_eq!(migrated.len(), nodes * UMST_CARRIER_LANE_COUNT);
        for node in 0..nodes {
            let row =
                &migrated[node * UMST_CARRIER_LANE_COUNT..(node + 1) * UMST_CARRIER_LANE_COUNT];
            assert_eq!(&row[..from_w], &source[node * from_w..(node + 1) * from_w]);
        }
    }

    #[test]
    fn try_stub_dec_refuses_short_rows_and_flags_relation_drift() {
        let short = vec![0.0; UMST_SCALAR_CHANNEL_COUNT];
        assert_eq!(
            try_stub_dec_graph_consistency(&short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT,
            })
        );
        assert_eq!(
            try_consistency_defect_from_dec_stub(&short),
            Err(SemanticLaneSchemaError::RowWidthMismatch {
                expected: UMST_CARRIER_LANE_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT,
            })
        );

        let mut row = vec![0.0; UMST_CARRIER_LANE_COUNT];
        assert!(semantic_band_is_zero(&row));
        row[LANE_RELATION_GRAPH] = 3.5;
        let report = try_stub_dec_graph_consistency(&row).expect("v1 dec stub");
        assert_eq!(report.relation_graph_drift, 3.5);
        assert_eq!(report.hook_revision, DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB);
        assert_eq!(
            try_consistency_defect_from_dec_stub(&row).expect("defect"),
            3.5
        );
        assert!(!semantic_band_is_zero(&row));

        row[LANE_TOPOLOGY_SIGNATURE] = 1.0;
        let cleared = try_stub_dec_graph_consistency(&row).expect("topology set");
        assert_eq!(cleared.relation_graph_drift, 0.0);
    }

    #[test]
    fn migrate_v1_to_v1_is_idempotent_on_semantic_band() {
        let mut v1 = vec![0.0; UMST_CARRIER_LANE_COUNT];
        v1[0] = 1.0;
        v1[LANE_MI_VALUE] = 8.0;
        let again = migrate_carrier_row(
            CarrierSchemaVersion::V1SemanticExtended,
            CarrierSchemaVersion::V1SemanticExtended,
            &v1,
        )
        .expect("v1→v1");
        assert_eq!(again, v1);
        assert!(reserved_band_is_zero(&again));
    }
}

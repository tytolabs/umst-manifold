// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

pub mod apply_physics;
pub mod dec_typestate;
pub mod error_boundary;
pub mod field;
pub mod field_algebra;
pub mod solver_unwrap_inventory;
pub mod material_phase;
pub mod emergence;
pub mod iterate_until;
pub mod material_transition;
pub mod semantic_lane_schema;
pub mod tensors;
pub mod traits;
pub mod umst_schema;

pub use apply_physics::apply_physics_to_umst;
pub use error_boundary::{ApplyPhysicsError, CatalogIoError, CbfReject};
pub use dec_typestate::{
    B1Incidence, DecTypestateError, ScalarChannel, ScalarChannelIdx, ScalarChannelSelector,
};
pub use field::{
    BodyForce, BodyForceField, BoundaryMask, BoundaryMaskField, CauchyStress, CauchyStressField,
    Damage, DamageField, Displacement, DisplacementField, Field, FractureEnergy,
    FractureEnergyField, Humidity, HumidityField, ReactionExtent, ReactionExtentField, SmallStrain,
    SmallStrainField, StepEntryDamageMask, Stiffness, StiffnessField, Temperature, TemperatureField,
};
pub use material_phase::{
    MaterialPhase, MaterialPhaseKind, MechanicsState, RheologyState, SettingState, ThmcEnvelope,
};
pub use material_transition::{
    MaterialTransitionParams, ReactionExtentKineticsSpec, SubstrateMaterialParams,
};
pub use semantic_lane_schema::{
    consistency_defect_from_dec_stub, migrate_carrier_batch, migrate_carrier_row,
    stub_dec_graph_consistency, validate_v1_layout_invariants, CarrierSchemaVersion,
    DecGraphConsistencyReport, SemanticLaneBundleV1, SemanticLaneId, SemanticLaneMigrationError,
    DEC_GRAPH_CONSISTENCY_HOOK_V1_STUB, LANE_CONCEPT_ID, LANE_CONTEXT_VECTOR, LANE_MI_VALUE,
    LANE_RELATION_GRAPH, LANE_SPEAKER_ID, LANE_TIMESTAMP, LANE_TOPOLOGY_SIGNATURE,
    RESERVED_LANE_BASE, RESERVED_LANE_COUNT, SEMANTIC_LANE_BASE, SEMANTIC_LANE_SCHEMA_V1,
    SEMANTIC_LANE_V1_COUNT, UMST_CARRIER_LANE_COUNT,
};
pub use tensors::*;
pub use traits::*;
pub use umst_schema::*;

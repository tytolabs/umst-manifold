// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Column indices for [`crate::core::tensors::UnifiedMaterialStateTensor::scalar_features`]
//! (`[N_active_nodes, F_scalars]`).
//!
//! ## Spatial units
//!
//! - [`crate::core::tensors::UnifiedMaterialStateTensor::node_positions`], when present, uses **SI
//!   metres** per axis (`[N, 3]`). This is independent of
//!   [`crate::core::tensors::UnifiedMaterialStateTensor::resolution_mm`], which remains a **millimetre**
//!   voxel/grid spacing hint for cartridges and visualization.
//!
//! These values are the **layout contract** for topology and THMC-style passes that read nodal
//! scalars from the manifold. Downstream domain cartridges bind these column indices in their
//! own crates; the kernel keeps only the shared layout contract.
//!
//! Channel `0` is reserved for material-specific bulk scalars (not yet fixed in the shared
//! contract); standard physics channels bind from [`SCALAR_HUMIDITY`] through
//! [`SCALAR_DAMAGE`], with optional [`SCALAR_FRACTURE_ENERGY_GC`] when `F_scalars > 5`.
//!
//! The pinned channel map is `artifacts/scalar_layout.lock.json` (Phase 1 §1B sidecar).
//! [`UMST_SCALAR_CHANNEL_COUNT`] and [`SCALAR_*`] indices are **generated schema surface**:
//! `build.rs` → `umst-layout-codegen` writes `OUT_DIR/scalar_layout_indices.rs` from
//! `artifacts/scalar_layout.lock.json`; the include below is the compile-time channel map.
//! Compile-time drift guard panics on lock mismatch.

include!(concat!(env!("OUT_DIR"), "/scalar_layout_indices.rs"));

include!(concat!(env!("OUT_DIR"), "/scalar_layout_guard.rs"));

const _: [(); UMST_SCALAR_CHANNEL_COUNT] = [(); UMST_SCALAR_CHANNEL_COUNT_LOCK];

// --- Honest posture fences (W29-032 deepen) ---------------------------------

/// Pinned scalar layout sidecar path (Phase 1 §1B SSOT).
pub const SCALAR_LAYOUT_LOCK_PATH: &str = "artifacts/scalar_layout.lock.json";

/// JSON schema id inside the scalar layout lock.
pub const SCALAR_LAYOUT_SCHEMA_ID: &str = "umst_scalar_layout_v1";

/// Layout contract is compile-time pinned — not a runtime migration surface yet.
pub const SCALAR_LAYOUT_MIGRATION_OPEN: bool = true;

/// Honest refusal — nodal scalar layout is staging SSOT, not production-wired fleet claim.
pub const PRODUCTION_WIRED: bool = false;

/// Honest refusal — layout pin is not a physics GREEN / oracle certification.
pub const PHYSICS_GREEN_CLAIMED: bool = false;

/// Honest refusal — no MASTER / fleet-complete posture at kernel schema seam.
pub const MASTER_POSTURE_CLAIMED: bool = false;

/// Vector feature slot count contract (`vector_features` dim-1 width when mechanics enabled).
pub const UMST_VECTOR_FEATURE_COUNT: usize = 1;

/// Nodal mechanical displacement **u** (SI metres), vector slot `0` in [`crate::core::tensors::UnifiedMaterialStateTensor::vector_features`]
/// (`[N, F_vectors, 3]`). When `F_vectors == 0`, THMC / mechanics adapters use zero displacement.
pub const VECTOR_MECHANICAL_DISPLACEMENT: usize = 0;

/// Errors from runtime scalar-layout probes (total public API).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarSchemaError {
    /// Column index is outside `0 .. UMST_SCALAR_CHANNEL_COUNT`.
    ChannelOutOfRange { index: usize, channel_count: usize },
    /// `scalar_features` width does not match the compile-time layout witness.
    WidthMismatch { expected: usize, found: usize },
}

/// Typed selector for pinned nodal scalar channels (generated `SCALAR_*` indices).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ScalarChannelId {
    /// Material-specific bulk scalar (channel 0 — cartridge-defined semantics).
    Channel0,
    Humidity,
    InternalVariable0,
    Temperature,
    Damage,
    FractureEnergyGc,
    EpistemicUncertainty,
}

impl ScalarChannelId {
    /// All pinned channels in lock-file column order.
    pub const ALL: [Self; UMST_SCALAR_CHANNEL_COUNT] = [
        Self::Channel0,
        Self::Humidity,
        Self::InternalVariable0,
        Self::Temperature,
        Self::Damage,
        Self::FractureEnergyGc,
        Self::EpistemicUncertainty,
    ];

    /// Column index into `scalar_features`.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Channel0 => SCALAR_CHANNEL0,
            Self::Humidity => SCALAR_HUMIDITY,
            Self::InternalVariable0 => SCALAR_INTERNAL_VARIABLE_0,
            Self::Temperature => SCALAR_TEMPERATURE,
            Self::Damage => SCALAR_DAMAGE,
            Self::FractureEnergyGc => SCALAR_FRACTURE_ENERGY_GC,
            Self::EpistemicUncertainty => SCALAR_EPISTEMIC_UNCERTAINTY,
        }
    }

    /// Stable lock-file channel id string (audit / manifest rows).
    #[must_use]
    pub const fn lock_id(self) -> &'static str {
        match self {
            Self::Channel0 => "SCALAR_CHANNEL0",
            Self::Humidity => "SCALAR_HUMIDITY",
            Self::InternalVariable0 => "SCALAR_INTERNAL_VARIABLE_0",
            Self::Temperature => "SCALAR_TEMPERATURE",
            Self::Damage => "SCALAR_DAMAGE",
            Self::FractureEnergyGc => "SCALAR_FRACTURE_ENERGY_GC",
            Self::EpistemicUncertainty => "SCALAR_EPISTEMIC_UNCERTAINTY",
        }
    }

    /// Resolve a lock-file id to the typed channel selector.
    #[must_use]
    pub fn from_lock_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|ch| ch.lock_id() == id)
    }
}

/// Reject raw column indices outside the pinned layout contract.
#[must_use]
pub fn try_scalar_channel_index(index: usize) -> Result<usize, ScalarSchemaError> {
    if index >= UMST_SCALAR_CHANNEL_COUNT {
        return Err(ScalarSchemaError::ChannelOutOfRange {
            index,
            channel_count: UMST_SCALAR_CHANNEL_COUNT,
        });
    }
    Ok(index)
}

/// Reject nodal scalar tensor widths that diverge from the compile-time witness.
#[must_use]
pub fn try_scalar_width(width: usize) -> Result<usize, ScalarSchemaError> {
    if width != UMST_SCALAR_CHANNEL_COUNT {
        return Err(ScalarSchemaError::WidthMismatch {
            expected: UMST_SCALAR_CHANNEL_COUNT,
            found: width,
        });
    }
    Ok(width)
}

/// Map a validated column index to its lock-file id (audit hook).
#[must_use]
pub fn scalar_channel_lock_id(index: usize) -> Option<&'static str> {
    try_scalar_channel_index(index).ok()?;
    ScalarChannelId::ALL
        .into_iter()
        .find(|ch| ch.index() == index)
        .map(ScalarChannelId::lock_id)
}

/// THMC core trio column indices (humidity, temperature, damage).
#[must_use]
pub const fn thmc_core_channels() -> [usize; 3] {
    [SCALAR_HUMIDITY, SCALAR_TEMPERATURE, SCALAR_DAMAGE]
}

/// Const-evaluable layout invariants (compile-time drift witness).
#[must_use]
pub const fn validate_layout_invariants() -> bool {
    UMST_SCALAR_CHANNEL_COUNT == UMST_SCALAR_CHANNEL_COUNT_LOCK
        && SCALAR_CHANNEL0 == 0
        && SCALAR_HUMIDITY < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_INTERNAL_VARIABLE_0 < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_TEMPERATURE < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_DAMAGE < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_FRACTURE_ENERGY_GC < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_EPISTEMIC_UNCERTAINTY < UMST_SCALAR_CHANNEL_COUNT
        && SCALAR_HUMIDITY < SCALAR_TEMPERATURE
        && SCALAR_TEMPERATURE < SCALAR_DAMAGE
        && VECTOR_MECHANICAL_DISPLACEMENT < UMST_VECTOR_FEATURE_COUNT
}

const _: () = assert!(validate_layout_invariants());

/// Honest kernel schema posture — pinned layout SSOT without GREEN / MASTER / production-wired.
#[must_use]
pub const fn schema_posture_is_honest_staging() -> bool {
    !PRODUCTION_WIRED && !PHYSICS_GREEN_CLAIMED && !MASTER_POSTURE_CLAIMED
}

/// Whether vector mechanics slot is within the declared vector feature count contract.
#[must_use]
pub const fn vector_slot_in_contract(slot: usize) -> bool {
    slot < UMST_VECTOR_FEATURE_COUNT
}

#[cfg(test)]
mod umst_schema_tests {
    use super::*;

    #[test]
    fn umst_schema_layout_invariants_hold() {
        assert!(validate_layout_invariants());
        assert_eq!(UMST_SCALAR_CHANNEL_COUNT, 7);
        assert_eq!(
            thmc_core_channels(),
            [SCALAR_HUMIDITY, SCALAR_TEMPERATURE, SCALAR_DAMAGE]
        );
    }

    #[test]
    fn umst_schema_channel_ids_match_lock_order() {
        let ids: Vec<_> = ScalarChannelId::ALL.iter().map(|ch| ch.lock_id()).collect();
        assert_eq!(
            ids,
            vec![
                "SCALAR_CHANNEL0",
                "SCALAR_HUMIDITY",
                "SCALAR_INTERNAL_VARIABLE_0",
                "SCALAR_TEMPERATURE",
                "SCALAR_DAMAGE",
                "SCALAR_FRACTURE_ENERGY_GC",
                "SCALAR_EPISTEMIC_UNCERTAINTY",
            ]
        );
        for ch in ScalarChannelId::ALL {
            assert_eq!(ScalarChannelId::from_lock_id(ch.lock_id()), Some(ch));
            assert_eq!(scalar_channel_lock_id(ch.index()), Some(ch.lock_id()));
        }
    }

    #[test]
    fn umst_schema_rejects_out_of_range_channel() {
        let err = try_scalar_channel_index(UMST_SCALAR_CHANNEL_COUNT).unwrap_err();
        assert_eq!(
            err,
            ScalarSchemaError::ChannelOutOfRange {
                index: UMST_SCALAR_CHANNEL_COUNT,
                channel_count: UMST_SCALAR_CHANNEL_COUNT,
            }
        );
        assert!(try_scalar_channel_index(UMST_SCALAR_CHANNEL_COUNT + 1).is_err());
        assert_eq!(try_scalar_channel_index(0).unwrap(), 0);
    }

    #[test]
    fn umst_schema_rejects_width_mismatch() {
        let err = try_scalar_width(UMST_SCALAR_CHANNEL_COUNT - 1).unwrap_err();
        assert_eq!(
            err,
            ScalarSchemaError::WidthMismatch {
                expected: UMST_SCALAR_CHANNEL_COUNT,
                found: UMST_SCALAR_CHANNEL_COUNT - 1,
            }
        );
        assert_eq!(
            try_scalar_width(UMST_SCALAR_CHANNEL_COUNT).unwrap(),
            UMST_SCALAR_CHANNEL_COUNT
        );
    }

    #[test]
    fn umst_schema_honest_fences_refuse_green() {
        assert!(schema_posture_is_honest_staging());
        assert!(!PRODUCTION_WIRED);
        assert!(!PHYSICS_GREEN_CLAIMED);
        assert!(!MASTER_POSTURE_CLAIMED);
        assert_eq!(SCALAR_LAYOUT_SCHEMA_ID, "umst_scalar_layout_v1");
        assert!(SCALAR_LAYOUT_LOCK_PATH.contains("scalar_layout.lock.json"));
    }

    #[test]
    fn umst_schema_vector_slot_contract() {
        assert!(vector_slot_in_contract(VECTOR_MECHANICAL_DISPLACEMENT));
        assert!(!vector_slot_in_contract(UMST_VECTOR_FEATURE_COUNT));
    }
}

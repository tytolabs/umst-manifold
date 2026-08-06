// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Structured rejection reasons at the manifold policy gateway.
//!
//! CBF rejections carry [`LANDAUER_CBF_CATALOG_ID`] for telemetry alignment with
//! `docs/GateUnificationSpec.md`.
//!
//! The legacy [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step`] surface keeps
//! [`String`] errors for backward compatibility; use [`FormalReject`] via
//! [`crate::ai::ppo::ManifoldGateway::evaluate_topology_step_formal`] when you need
//! machine-readable witnesses.
//!
//! # Honest boundary (W29-009)
//!
//! [`FormalReject`] is a **cold-edge witness vocabulary** — typed refusal slugs and
//! Display strings for gateway telemetry. Not physics GREEN, not `PRODUCTION_WIRED`,
//! not `MASTER`. Lean export live ceremony and default-profile digest pin remain deferred.

pub use crate::runtime::catalog::traceability::LANDAUER_CBF_CATALOG_ID;

/// W29 deepen cell — formal reject honest fence bundle.
pub const W29_FORMAL_DEEPEN_CELL: &str = "W29-009-FORMAL";

/// Honest posture tag — enum + catalog slugs landed; production/master refused.
pub const FORMAL_POSTURE_TAG: &str = "honest-formal-reject-ssot-only";

/// Lean export live ceremony deferred beyond in-repo digest pin.
pub const FORMAL_LEAN_EXPORT_DEFERRED_STEP: &str = "umst-formal/artifacts/catalog.json:live-export";

/// Gateway production orchestration pin deferred beyond formal reject vocabulary.
pub const FORMAL_PRODUCTION_ORCH_DEFERRED_STEP: &str = "W4-JG-6-loop-close";

/// Honest physics posture — reject taxonomy does not certify continuum physics.
pub const FORMAL_PHYSICS_GREEN: bool = false;

/// Production deployment wiring — not claimed by formal reject module alone.
pub const FORMAL_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by formal reject module.
pub const FORMAL_MASTER: bool = false;

/// Whether [`FormalReject`] enum + catalog_id slugs are landed.
pub const FORMAL_REJECT_ENUM_LANDED: bool = true;

/// Whether gateway formal API is landed (see `ppo::ManifoldGateway::evaluate_topology_step_formal`).
pub const FORMAL_GATEWAY_API_LANDED: bool = true;

/// Whether legacy String bridge is landed.
pub const FORMAL_LEGACY_STRING_BRIDGE_LANDED: bool = true;

/// Whether witness-priority ladder feed is landed.
pub const FORMAL_WITNESS_PRIORITY_LANDED: bool = true;

/// Honest deepen fence for meta / fleet probes.
pub const FORMAL_HONEST_FENCE: &str =
    "reject_enum_landed=true gateway_formal_api_landed=true production_wired=false master_composition_wired=false";

/// Formal fence facet count (honest census).
pub const FORMAL_FENCE_FACET_COUNT: usize = 8;

/// Formal fence facets wired today (5/8 measured; formal-witness feature-gated).
pub const FORMAL_FENCE_WIRED_COUNT: usize = 5;

/// Stable catalog slug for DEC typestate staging rejects.
pub const DEC_TYPESTATE_CATALOG_ID: &str = "umst.gate.dec_typestate";

/// Stable catalog slug for catalog digest mismatch rejects (`formal-witness` feature).
pub const CATALOG_LOCK_CATALOG_ID: &str = "umst.formal.catalog_lock";

/// Stable facet ids for formal production fence census.
pub const FORMAL_FENCE_FACET_IDS: &[&str] = &[
    "reject_enum_catalog_ids",
    "gateway_formal_api",
    "legacy_string_bridge",
    "witness_priority_ladder",
    "apply_physics_mapping",
    "formal_witness_digest",
    "lean_live_export",
    "production_wired",
];

/// One facet of the formal production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormalProductionFenceFacet {
    /// Facet under census.
    pub facet: &'static str,
    /// Whether this facet is wired today.
    pub wired: bool,
    /// Owning slice when residue.
    pub owning_slice: &'static str,
}

/// Formal production fence facet inventory (honest posture SSOT).
pub const FORMAL_PRODUCTION_FENCE_FACETS: &[FormalProductionFenceFacet] = &[
    FormalProductionFenceFacet {
        facet: "reject_enum_catalog_ids",
        wired: true,
        owning_slice: W29_FORMAL_DEEPEN_CELL,
    },
    FormalProductionFenceFacet {
        facet: "gateway_formal_api",
        wired: true,
        owning_slice: W29_FORMAL_DEEPEN_CELL,
    },
    FormalProductionFenceFacet {
        facet: "legacy_string_bridge",
        wired: true,
        owning_slice: W29_FORMAL_DEEPEN_CELL,
    },
    FormalProductionFenceFacet {
        facet: "witness_priority_ladder",
        wired: true,
        owning_slice: W29_FORMAL_DEEPEN_CELL,
    },
    FormalProductionFenceFacet {
        facet: "apply_physics_mapping",
        wired: true,
        owning_slice: W29_FORMAL_DEEPEN_CELL,
    },
    FormalProductionFenceFacet {
        facet: "formal_witness_digest",
        wired: false,
        owning_slice: "formal-witness-feature",
    },
    FormalProductionFenceFacet {
        facet: "lean_live_export",
        wired: false,
        owning_slice: FORMAL_LEAN_EXPORT_DEFERRED_STEP,
    },
    FormalProductionFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: FORMAL_PRODUCTION_ORCH_DEFERRED_STEP,
    },
];

/// One hop in the formal gateway reject wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormalGateWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the admit chain.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Formal gateway reject wire map (cold-edge witness vocabulary).
pub const FORMAL_GATE_WIRE_HOPS: &[FormalGateWireHop] = &[
    FormalGateWireHop {
        ordinal: 1,
        surface: "umst-manifold::ai::ppo::ManifoldGateway::evaluate_topology_step_formal",
        role: "Structured reject at PPO gateway boundary",
        wired: true,
    },
    FormalGateWireHop {
        ordinal: 2,
        surface: "umst-manifold::ai::formal::FormalReject::catalog_id",
        role: "Stable telemetry slug per variant",
        wired: true,
    },
    FormalGateWireHop {
        ordinal: 3,
        surface: "umst-manifold::runtime::catalog::WitnessPriorityQueue::record_formal_reject",
        role: "Adaptive witness ladder feed",
        wired: true,
    },
    FormalGateWireHop {
        ordinal: 4,
        surface: "umst-manifold::ai::ppo::ManifoldGateway::evaluate_topology_step",
        role: "Legacy String bridge via Display",
        wired: true,
    },
    FormalGateWireHop {
        ordinal: 5,
        surface: "umst-manifold::ai::ppo::formal_reject_from_apply_physics",
        role: "ApplyPhysicsError → DecTypestateStaging mapping",
        wired: true,
    },
    FormalGateWireHop {
        ordinal: 6,
        surface: "umst-manifold::ai::formal::FormalReject::CatalogSchemaDigestMismatch",
        role: "Catalog digest pin (`formal-witness` feature)",
        wired: false,
    },
    FormalGateWireHop {
        ordinal: 7,
        surface: "umst-formal::artifacts::catalog.json:live-export",
        role: "Lean export live ceremony (formal-owned)",
        wired: false,
    },
];

/// Count wired formal fence facets (must match [`FORMAL_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn formal_fence_wired_count() -> usize {
    FORMAL_PRODUCTION_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Count wired formal gateway hops.
#[must_use]
pub fn formal_gate_wire_hops_closed() -> usize {
    FORMAL_GATE_WIRE_HOPS.iter().filter(|h| h.wired).count()
}

/// Honest production wiring — **false** until fleet orch + live export measured.
#[must_use]
pub const fn formal_production_wired() -> bool {
    false
}

/// Master composition wiring — **false** until W4-JG-6 loop-close.
#[must_use]
pub const fn formal_master_composition_wired() -> bool {
    false
}

/// Compile-time fence — production flip not authorized at posture tier.
const _: () = assert!(!formal_production_wired());

/// Measured honest-posture snapshot for formal reject (cold edge only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormalHonestPosture {
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub reject_enum_landed: bool,
    pub gateway_api_landed: bool,
    pub legacy_bridge_landed: bool,
    pub witness_priority_landed: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub wire_hops_closed: usize,
    pub honest_fence: &'static str,
    pub deferred_lean_export: &'static str,
    pub deferred_production_orch: &'static str,
}

/// Honest posture bundle for orchestrator / census probes — no invented GREEN.
#[must_use]
pub fn formal_honest_posture_bundle() -> FormalHonestPosture {
    FormalHonestPosture {
        physics_green: FORMAL_PHYSICS_GREEN,
        production_wired: FORMAL_PRODUCTION_WIRED,
        master: FORMAL_MASTER,
        reject_enum_landed: FORMAL_REJECT_ENUM_LANDED,
        gateway_api_landed: FORMAL_GATEWAY_API_LANDED,
        legacy_bridge_landed: FORMAL_LEGACY_STRING_BRIDGE_LANDED,
        witness_priority_landed: FORMAL_WITNESS_PRIORITY_LANDED,
        fence_facet_count: FORMAL_FENCE_FACET_COUNT,
        fence_wired_count: formal_fence_wired_count(),
        wire_hops_closed: formal_gate_wire_hops_closed(),
        honest_fence: FORMAL_HONEST_FENCE,
        deferred_lean_export: FORMAL_LEAN_EXPORT_DEFERRED_STEP,
        deferred_production_orch: FORMAL_PRODUCTION_ORCH_DEFERRED_STEP,
    }
}

/// Typed probe for formal posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormalPostureProbe {
    pub cell_id: &'static str,
    pub posture_tag: &'static str,
    pub reject_enum_landed: bool,
    pub gateway_api_landed: bool,
    pub legacy_bridge_landed: bool,
    pub witness_priority_landed: bool,
    pub production_wired: bool,
    pub master_composition_wired: bool,
    pub physics_green: bool,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub wire_hops_closed: usize,
    pub honest_fence: &'static str,
}

/// Build introspection probe for formal done-when / fleet checks.
#[must_use]
pub const fn formal_posture_probe() -> FormalPostureProbe {
    FormalPostureProbe {
        cell_id: W29_FORMAL_DEEPEN_CELL,
        posture_tag: FORMAL_POSTURE_TAG,
        reject_enum_landed: FORMAL_REJECT_ENUM_LANDED,
        gateway_api_landed: FORMAL_GATEWAY_API_LANDED,
        legacy_bridge_landed: FORMAL_LEGACY_STRING_BRIDGE_LANDED,
        witness_priority_landed: FORMAL_WITNESS_PRIORITY_LANDED,
        production_wired: formal_production_wired(),
        master_composition_wired: formal_master_composition_wired(),
        physics_green: FORMAL_PHYSICS_GREEN,
        fence_facet_count: FORMAL_FENCE_FACET_COUNT,
        fence_wired_count: FORMAL_FENCE_WIRED_COUNT,
        wire_hops_closed: 5,
        honest_fence: FORMAL_HONEST_FENCE,
    }
}

/// Formal SSOT landed with production/master composition honestly open.
#[must_use]
pub fn formal_posture_honest(probe: &FormalPostureProbe) -> bool {
    probe.cell_id == W29_FORMAL_DEEPEN_CELL
        && probe.posture_tag == FORMAL_POSTURE_TAG
        && probe.reject_enum_landed
        && probe.gateway_api_landed
        && probe.legacy_bridge_landed
        && probe.witness_priority_landed
        && !probe.physics_green
        && !probe.production_wired
        && !probe.master_composition_wired
        && probe.fence_facet_count == FORMAL_FENCE_FACET_COUNT
        && probe.fence_wired_count == FORMAL_FENCE_WIRED_COUNT
        && probe.wire_hops_closed == formal_gate_wire_hops_closed()
        && probe.honest_fence.contains("reject_enum_landed=true")
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("master_composition_wired=false")
}

/// Validate formal posture honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_formal_posture_honesty() -> Result<(), &'static str> {
    let probe = formal_posture_probe();
    if probe.physics_green {
        return Err("FORMAL_PHYSICS_GREEN must stay false — reject taxonomy is SSOT only");
    }
    if probe.production_wired {
        return Err("formal_production_wired must stay false until fleet orch lands");
    }
    if probe.master_composition_wired {
        return Err("formal_master_composition_wired must stay false until W4-JG-6");
    }
    if !formal_posture_honest(&probe) {
        return Err("formal_posture_probe failed honest fence census");
    }
    Ok(())
}

/// Stable reject-family slug for telemetry rollups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormalRejectKind {
    /// DEC typestate staging before physics.
    DecTypestateStaging,
    /// Clausius–Duhem / Landauer CBF barrier.
    ThermodynamicControlBarrier,
    /// Catalog schema digest mismatch (`formal-witness` feature).
    #[cfg(feature = "formal-witness")]
    CatalogSchemaDigestMismatch,
}

#[cfg(feature = "formal-witness")]
fn format_catalog_digest_hex(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    use std::fmt::Write as _;
    for b in bytes {
        write!(&mut s, "{b:02x}").expect("write to pre-allocated String");
    }
    s
}

/// Machine-readable rejection at the manifold gateway boundary.
///
/// The thermodynamic barrier variant preserves the wording expected from the legacy
/// `Err(String)` path; optional catalog hashing is gated by the **`formal-witness`**
/// crate feature.
#[derive(Clone, Eq, PartialEq)]
pub enum FormalReject {
    /// DEC typestate staging bundle rejected the proposed UMST layout before physics.
    DecTypestateStaging { detail: String },
    /// Clausius–Duhem / Landauer bookkeeping rejected the proposed transition (`ThermodynamicCBF`).
    ThermodynamicControlBarrier {
        catalog_id: &'static str,
        detail: String,
    },
    /// Runtime material catalog/schema digest disagree between gateway expectation and UMST carrier.
    #[cfg(feature = "formal-witness")]
    CatalogSchemaDigestMismatch {
        expected: [u8; 32],
        observed: [u8; 32],
    },
}

impl FormalReject {
    /// Stable gate / witness slug for telemetry (see `GateUnificationSpec.md`).
    pub fn catalog_id(&self) -> &'static str {
        match self {
            Self::DecTypestateStaging { .. } => DEC_TYPESTATE_CATALOG_ID,
            Self::ThermodynamicControlBarrier { catalog_id, .. } => catalog_id,
            #[cfg(feature = "formal-witness")]
            Self::CatalogSchemaDigestMismatch { .. } => CATALOG_LOCK_CATALOG_ID,
        }
    }

    /// Classify reject into a stable family for census rollups.
    pub fn reject_kind(&self) -> FormalRejectKind {
        match self {
            Self::DecTypestateStaging { .. } => FormalRejectKind::DecTypestateStaging,
            Self::ThermodynamicControlBarrier { .. } => {
                FormalRejectKind::ThermodynamicControlBarrier
            }
            #[cfg(feature = "formal-witness")]
            Self::CatalogSchemaDigestMismatch { .. } => {
                FormalRejectKind::CatalogSchemaDigestMismatch
            }
        }
    }
}

impl core::fmt::Debug for FormalReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FormalReject::DecTypestateStaging { detail } => f
                .debug_struct("DecTypestateStaging")
                .field("detail", detail)
                .finish(),
            FormalReject::ThermodynamicControlBarrier { catalog_id, detail } => f
                .debug_struct("ThermodynamicControlBarrier")
                .field("catalog_id", catalog_id)
                .field("detail", detail)
                .finish(),
            #[cfg(feature = "formal-witness")]
            FormalReject::CatalogSchemaDigestMismatch { expected, observed } => f
                .debug_struct("CatalogSchemaDigestMismatch")
                .field("expected", &format_catalog_digest_hex(expected))
                .field("observed", &format_catalog_digest_hex(observed))
                .finish(),
        }
    }
}

impl core::fmt::Display for FormalReject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FormalReject::DecTypestateStaging { detail } => {
                write!(
                    f,
                    "DEC typestate staging reject [{DEC_TYPESTATE_CATALOG_ID}]: {detail}"
                )
            }
            FormalReject::ThermodynamicControlBarrier { catalog_id, detail } => {
                write!(f, "Transition Rejected by CBF [{catalog_id}]: {detail}")
            }
            #[cfg(feature = "formal-witness")]
            FormalReject::CatalogSchemaDigestMismatch { expected, observed } => write!(
                f,
                "Catalog schema digest mismatch [{CATALOG_LOCK_CATALOG_ID}]: expected {}, observed {}",
                format_catalog_digest_hex(expected),
                format_catalog_digest_hex(observed),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formal_honest_fence_posture_probe() {
        let probe = formal_posture_probe();
        assert!(formal_posture_honest(&probe));
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master_composition_wired);
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("master_composition_wired=false"));
        validate_formal_posture_honesty().expect("validate_formal_posture_honesty");
    }

    #[test]
    fn formal_production_and_master_stay_false() {
        assert!(!FORMAL_PHYSICS_GREEN);
        assert!(!FORMAL_PRODUCTION_WIRED);
        assert!(!FORMAL_MASTER);
        assert!(!formal_production_wired());
        assert!(!formal_master_composition_wired());
    }

    #[test]
    fn formal_fence_facet_census_matches_constants() {
        assert_eq!(FORMAL_FENCE_FACET_IDS.len(), FORMAL_FENCE_FACET_COUNT);
        assert_eq!(
            FORMAL_PRODUCTION_FENCE_FACETS.len(),
            FORMAL_FENCE_FACET_COUNT
        );
        assert_eq!(formal_fence_wired_count(), FORMAL_FENCE_WIRED_COUNT);
        let bundle = formal_honest_posture_bundle();
        assert_eq!(bundle.fence_facet_count, FORMAL_FENCE_FACET_COUNT);
        assert_eq!(bundle.fence_wired_count, FORMAL_FENCE_WIRED_COUNT);
        assert!(!bundle.physics_green);
        assert!(!bundle.production_wired);
        assert!(!bundle.master);
        assert_eq!(bundle.wire_hops_closed, formal_gate_wire_hops_closed());
    }

    #[test]
    fn formal_deferred_facets_stay_unwired() {
        let deferred: Vec<_> = FORMAL_PRODUCTION_FENCE_FACETS
            .iter()
            .filter(|f| !f.wired)
            .map(|f| f.facet)
            .collect();
        assert!(deferred.contains(&"formal_witness_digest"));
        assert!(deferred.contains(&"lean_live_export"));
        assert!(deferred.contains(&"production_wired"));
        assert_eq!(deferred.len(), 3);
    }

    #[test]
    fn formal_gate_wire_hops_closed_honest() {
        assert_eq!(formal_gate_wire_hops_closed(), 5);
        assert_eq!(FORMAL_GATE_WIRE_HOPS.len(), 7);
        let open: Vec<_> = FORMAL_GATE_WIRE_HOPS
            .iter()
            .filter(|h| !h.wired)
            .map(|h| h.ordinal)
            .collect();
        assert_eq!(open, vec![6, 7]);
    }

    #[test]
    fn dec_typestate_reject_carries_dec_typestate_slug() {
        let rej = FormalReject::DecTypestateStaging {
            detail: "invalid channel".into(),
        };
        assert_eq!(rej.catalog_id(), DEC_TYPESTATE_CATALOG_ID);
        assert_eq!(
            rej.reject_kind(),
            FormalRejectKind::DecTypestateStaging
        );
        assert!(rej.to_string().contains(DEC_TYPESTATE_CATALOG_ID));
    }

    #[cfg(feature = "formal-witness")]
    #[test]
    fn catalog_digest_mismatch_carries_catalog_lock_slug() {
        let rej = FormalReject::CatalogSchemaDigestMismatch {
            expected: [1u8; 32],
            observed: [2u8; 32],
        };
        assert_eq!(rej.catalog_id(), CATALOG_LOCK_CATALOG_ID);
        assert_eq!(
            rej.reject_kind(),
            FormalRejectKind::CatalogSchemaDigestMismatch
        );
        assert!(rej.to_string().contains(CATALOG_LOCK_CATALOG_ID));
    }

    #[test]
    fn cbf_reject_carries_landauer_catalog_id() {
        let rej = FormalReject::ThermodynamicControlBarrier {
            catalog_id: LANDAUER_CBF_CATALOG_ID,
            detail: "insufficient dissipation".into(),
        };
        assert_eq!(rej.catalog_id(), "umst.gate.landauer_cbf");
        assert_eq!(
            rej.reject_kind(),
            FormalRejectKind::ThermodynamicControlBarrier
        );
        assert!(
            rej.to_string().contains("umst.gate.landauer_cbf"),
            "Display must embed catalog_id for telemetry parsers"
        );
    }
}

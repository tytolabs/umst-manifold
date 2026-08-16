// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Lean `catalog.json` entry coverage vs gate spec / witness registry.
//!
//! See **`docs/CATALOG_TRACEABILITY.md`** and classical anchors in **`docs/claims-vs-proofs.md`** /
//! **`docs/DUAL_PIN_ARCHITECTURE.md`**. CI: `tests/catalog_all_ids_registered.rs`.
//!
//! **W29-109-TRACEABILITY** — deepen + honest fences (no invent GREEN/PRODUCTION_WIRED/MASTER/OP-5).

use serde::Serialize;

use crate::core::error_boundary::CatalogIoError;

/// Lean modules with **no** dedicated Rust gate wiring yet (formal-only or scaffold).
///
/// Every `catalog.json` `module` must appear here **or** in [`CATALOG_MODULE_WIRED`].
pub const ALLOW_UNUSED_CATALOG_IDS: &[&str] = &[
    "Concrete.Activation",
    "Adjoint",
    "Behavior.SDFCanonical",
    // umst-formal fiber rename; runtime Landauer CBF wiring uses double-slit `MeasurementCost`.
    "ClassicalMeasurementCost",
    "Complementarity",
    "Concrete.Convergence",
    "CreditGreedyOptimal",
    "Crypto.Collision",
    "Crypto.Composability",
    "Crypto.EUF_CMA",
    "Crypto.LWE",
    "Crypto.SanitizePatternCoverage",
    "Crypto.SideChannel",
    "DataProcessingInequality",
    "DensityState",
    "Dignity",
    "DoubleSlitCore",
    "Economic.BurdenRecursionIsAdmissible",
    "Economic.CollectiveCoherenceCost",
    "Economic.CreativeExplorationTolerance",
    "Economic.CreativityBudget",
    "Economic.DynamicEpsilonCalibration",
    "Economic.EconomicDomain",
    "Economic.EconomicTemperature",
    "Economic.EpistemicSensingModule",
    "Economic.HallucinationDetector",
    "Economic.HorizonAwareGrounding",
    "Economic.LowEntropyLieDetector",
    "Economic.NPVIsSpecialCaseOfThermodynamicBurden",
    "Economic.NuanceIsolator",
    "Economic.SelfReferentialEconomicTensor",
    "Economic.StochasticBurdenExpectation",
    "Economic.ThermodynamicUncertaintyCertificate",
    "Concrete.EndConditions",
    "Concrete.EnrichedAdmissibility",
    "EpistemicDynamics",
    "EpistemicGalois",
    "EpistemicNumericsContract",
    "EpistemicPerStepNumerics",
    "EpistemicPolicy",
    "EpistemicRuntimeSchemaContract",
    "EpistemicTelemetryApproximation",
    "EpistemicTelemetryBridge",
    "EpistemicTelemetryQuantitativeUtility",
    "EpistemicTelemetrySolverCalibration",
    "EpistemicTraceDerivedEpsilonCertificate",
    "EpistemicTraceDrivenCalibrationWitness",
    "EtaCog",
    "ExamplesQubit",
    "FiberedActivation",
    "FlashMoERuntimeScaffold",
    "Concrete.GaloisGate",
    "GeneralDimension",
    "GeneralResidualCoherence",
    "GeneralVisibility",
    "Concrete.GraphProperties",
    "Concrete.Helmholtz",
    "Concrete.Gate",
    "Concrete.State",
    "Core.Constitutional",
    "Core.Gate",
    "Core.Scalar",
    "Core.State",
    "DualLedger",
    "Real.Gate",
    "Real.State",
    "InfoEntropy",
    "InfoTheory",
    "JenningsGelSpace",
    "KleinInequality",
    "KroneckerEigen",
    "LandauerEinsteinBridge",
    "LindbladDynamics",
    "LindbladStreamD",
    "LogSum",
    "MatrixLog",
    "MeasurementChannel",
    "MedianConvergence",
    "Memory.MergeSafe",
    "Memory.TierDisjoint",
    "OrderStatisticsBand",
    "PMICEntropyInterior",
    "PMICVisibility",
    "PrimeSpectralCategory",
    "PrimeSpectralGuidance",
    "PrototypeSolverCalibration",
    "QuantumClassicalBridge",
    "QuantumMutualInfo",
    "RegimeSoundness",
    "RhoEstimator",
    "SchrodingerDynamics",
    "SeparationBound",
    "SimLeanBridge",
    "TensorPartialTrace",
    "Test3",
    "Test4",
    "TestEntropy",
    "TestFixes",
    "TestMixed",
    "VonNeumannEntropy",
    "WhichPathMeasurementUpdate",
    "_check_ext",
    "experiments.AutoExperimenterPlaceholder",
    "lakefile",
    "scripts.print_axioms",
    "test_tensor_eigen",
];

/// Lean modules explicitly mapped to runtime `catalog_id` slugs (see `docs/claims-vs-proofs.md`).
pub const CATALOG_MODULE_WIRED: &[(&str, &[&str])] = &[
    ("DoubleSlit", &["umst.gate.cd_transition"]),
    ("EpistemicMI", &["umst.gate.landauer_cbf"]),
    ("EpistemicRuntimeContract", &["umst.formal.catalog_lock"]),
    ("EpistemicSensing", &["umst.gate.landauer_cbf"]),
    ("EpistemicTrajectoryMI", &["umst.gate.landauer_cbf"]),
    ("ErasureChannel", &["umst.gate.landauer_cbf"]),
    ("FormalFoundations", &["umst.formal.catalog_lock"]),
    (
        "Compat.Gate",
        &["umst.gate.cd_transition", "umst.gate.kleisli_unit"],
    ),
    ("GateCompat", &["umst.gate.cd_transition"]),
    ("InformationCostIdentity", &["umst.gate.landauer_cbf"]),
    ("LandauerBound", &["umst.gate.landauer_cbf"]),
    ("LandauerExtension", &["umst.gate.landauer_cbf"]),
    ("LandauerLaw", &["umst.gate.landauer_cbf"]),
    // `SemanticSecondLaw` is absent from the composed Lean export (129-module lock);
    // cold proof + `umst.gate.semantic_cbf` wiring lives in `catalog.lock.json`
    // `semantic_witnesses` (HCOM-004), not this partition table.
    ("MeasurementCost", &["umst.gate.landauer_cbf"]),
    ("MonoidalState", &["umst.gate.cd_transition"]),
    ("Naturality", &["umst.gate.cd_transition"]),
    ("PhysicsConstrainedAI", &["umst.gate.landauer_cbf"]),
    ("ProbeOptimization", &["umst.gate.kleisli_unit"]),
    ("Compat.Constitutional", &["umst.gate.kleisli_unit"]),
    (
        "Economic.KleisliAdmissibilityComposition",
        &["umst.gate.kleisli_unit"],
    ),
    ("DIBKleisli", &["umst.gate.kleisli_unit"]),
    ("DEC", &["umst.gate.cd_transition"]),
    (
        "Concrete.Powers",
        &["thermodynamic_mix", "umst.gate.http_shim"],
    ),
];

/// `catalog_id` values listed in **`docs/GateUnificationSpec.md`** mapping table (SSOT).
pub const GATE_UNIFICATION_SPEC_CATALOG_IDS: &[&str] = &[
    "umst.formal.catalog_lock",
    "umst.gate.cd_transition",
    "umst.gate.http_shim",
    "umst.gate.kleisli_unit",
    "umst.gate.landauer_cbf",
];

/// Stable slug for [`crate::gate::ThermodynamicTransitionEvaluator`] / host CD transition rejects.
pub const CD_TRANSITION_CATALOG_ID: &str = "umst.gate.cd_transition";

/// Stable slug for [`crate::gate::TransitionEvaluator`] / mix registry host rejects.
pub const THERMODYNAMIC_MIX_CATALOG_ID: &str = "thermodynamic_mix";

/// Stable slug for [`crate::gate::http_manifest::HttpTransitionEvaluator`] HTTP shim rejects.
pub const HTTP_SHIM_CATALOG_ID: &str = "umst.gate.http_shim";

/// Deprecated `catalog_id` slug — superseded by [`HTTP_SHIM_CATALOG_ID`].
///
/// Domain-specific HTTP `gate_family` telemetry lives in the cartridge registry
/// (`umst-concrete-cartridge::cartridge_registry`), not kernel traceability.
pub const PREDICTION_VS_PHYSICS_CATALOG_ID_DEPRECATED: &str = "umst.gate.prediction_vs_physics";

/// Stable slug for [`crate::ai::cbf::ThermodynamicCBF`] / [`FormalReject::ThermodynamicControlBarrier`].
pub const LANDAUER_CBF_CATALOG_ID: &str = "umst.gate.landauer_cbf";

/// Stable slug for [`crate::gate::semantic_cbf::SemanticCBF`] hot-path rejects (HCOM-004).
pub const SEMANTIC_CBF_CATALOG_ID: &str = "umst.gate.semantic_cbf";

/// Gate slugs implemented in Rust but not yet in the spec table (see `claims-vs-proofs.md` note).
pub const RUNTIME_EXTRA_GATE_CATALOG_IDS: &[&str] = &["thermodynamic_mix"];

/// Cartridge-owned gate slugs (not in kernel universal registry after W9 Phase A).
pub const CARTRIDGE_GATE_REGISTRY_CATALOG_IDS: &[&str] = &["umst.cartridge.domain.policy"];

/// [`GateEvaluator::catalog_id`] values implemented in kernel `src/gate/` (universal registry).
pub const GATE_REGISTRY_CATALOG_IDS: &[&str] = &[
    "umst.gate.cd_transition",
    "umst.gate.http_shim",
    "umst.gate.kleisli_unit",
    "thermodynamic_mix",
];

/// Runtime gate `catalog_id`s with **no** Lean `catalog.json` backing row (legacy allowlist).
pub const ALLOW_UNUSED_GATE_CATALOG_IDS: &[&str] = &["umst.cartridge.domain.policy"];

/// Default relative path from `umst-manifold` to the Lean exporter catalog (sibling checkout layout).
pub const DEFAULT_UPSTREAM_CATALOG_JSON: &str = "../umst-formal-double-slit/artifacts/catalog.json";

/// Pinned export committed in-repo for CI when the formal sibling checkout is absent.
pub const PINNED_UPSTREAM_CATALOG_JSON: &str = "artifacts/upstream_catalog.json";

/// Resolve Lean `catalog.json` for partition tests: env override → sibling (if lock-aligned) → pinned snapshot.
pub fn resolve_upstream_catalog_json_path(manifest_dir: &std::path::Path) -> std::path::PathBuf {
    if let Ok(p) = std::env::var("UMST_LEAN_CATALOG_JSON") {
        return std::path::PathBuf::from(p);
    }
    let pinned = manifest_dir.join(PINNED_UPSTREAM_CATALOG_JSON);
    let sibling = manifest_dir.join(DEFAULT_UPSTREAM_CATALOG_JSON);
    if sibling.is_file() {
        if let Some(lock_count) = read_lock_module_count(manifest_dir) {
            if let Ok(sibling_count) = count_catalog_modules(&sibling) {
                if sibling_count == lock_count as usize {
                    return sibling;
                }
            }
        } else {
            return sibling;
        }
    }
    pinned
}

fn read_lock_module_count(manifest_dir: &std::path::Path) -> Option<u64> {
    let lock_path = manifest_dir.join("artifacts/catalog.lock.json");
    let lock_raw = std::fs::read_to_string(lock_path).ok()?;
    let lock: serde_json::Value = serde_json::from_str(&lock_raw).ok()?;
    lock.get("module_count")?.as_u64()
}

fn count_catalog_modules(path: &std::path::Path) -> Result<usize, CatalogIoError> {
    let raw = std::fs::read_to_string(path)?;
    let v: serde_json::Value = serde_json::from_str(&raw)?;
    let modules = v
        .get("modules")
        .and_then(|m| m.as_array())
        .ok_or(CatalogIoError::MissingModulesArray)?;
    Ok(modules.len())
}

// ── W29-109-TRACEABILITY · deepen + honest fence ─────────────────────────────

/// Swarm cell id for this traceability deepen.
pub const W29_109_CELL_ID: &str = "W29-109-TRACEABILITY";

/// Honest posture — deepen measured Lean↔gate partition only; no invent claims.
pub const W29_109_HONEST_POSTURE: &str = "TRACEABILITY_DEEPEN_ONLY";

/// Explicit non-claims (gate text).
pub const W29_109_NON_CLAIM: &str =
    "not GREEN; not OP-5 PASS; not production_wired; not MASTER_RETICK";

/// Deepen schema version for W29-109.
pub const W29_109_DEEPEN_SCHEMA_VERSION: &str = "traceability_w29_109_deepen_v1";

/// Honest posture tag — registration partition, not production flip.
pub const TRACEABILITY_POSTURE_TAG: &str = "catalog-partition-wired-not-production";

/// R0 lock `module_count` pin matching `catalog_all_ids_registered` / dual-pin lock.
pub const TRACEABILITY_R0_MODULE_COUNT: usize = 129;

/// Expected wired hop count on manifold partition side (production hop stays open).
pub const W29_109_WIRE_HOP_WIRED_COUNT: u8 = 4;

/// Expected total wire hops (4 closed partition + 1 production open).
pub const W29_109_WIRE_HOP_TOTAL: usize = 5;

/// Expected unit-test count pin for W29-109 deepen (keep in sync with `#[test]` fns).
pub const W29_109_UNIT_TESTS_IN_MODULE: u8 = 10;

/// One hop in the catalog traceability wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TraceabilityWireHop {
    /// Ordinal (1-based).
    pub ordinal: u8,
    /// Module or symbol surface.
    pub surface: &'static str,
    /// Role in the Lean↔gate partition.
    pub role: &'static str,
    /// Whether this hop is wired today.
    pub wired: bool,
}

/// Manifold traceability wire map (partition tables → open production invent).
pub const TRACEABILITY_WIRE_HOPS: &[TraceabilityWireHop] = &[
    TraceabilityWireHop {
        ordinal: 1,
        surface: "CATALOG_MODULE_WIRED",
        role: "Lean module → runtime catalog_id map",
        wired: true,
    },
    TraceabilityWireHop {
        ordinal: 2,
        surface: "ALLOW_UNUSED_CATALOG_IDS",
        role: "Formal-only / scaffold allowlist partition",
        wired: true,
    },
    TraceabilityWireHop {
        ordinal: 3,
        surface: "GATE_UNIFICATION_SPEC_CATALOG_IDS ∪ RUNTIME_EXTRA",
        role: "Spec + runtime-extra catalog_id SSOT",
        wired: true,
    },
    TraceabilityWireHop {
        ordinal: 4,
        surface: "GATE_REGISTRY_CATALOG_IDS ∪ ALLOW_UNUSED_GATE",
        role: "Kernel GateEvaluator registry partition",
        wired: true,
    },
    TraceabilityWireHop {
        ordinal: 5,
        surface: "production_wired / physics_GREEN invent",
        role: "Live production / GREEN invent fence (honest open)",
        wired: false,
    },
];

/// Whether live production / GREEN invent is plumbed on this surface (honest `false`).
#[must_use]
pub const fn traceability_production_wired() -> bool {
    false
}

/// Measured Lean-module partition counts (wired ∪ allowlist).
#[must_use]
pub fn traceability_partition_counts() -> (usize, usize, usize) {
    let wired = CATALOG_MODULE_WIRED.len();
    let allow = ALLOW_UNUSED_CATALOG_IDS.len();
    (wired, allow, wired + allow)
}

/// Structural quickcheck: partition tables non-empty, disjoint module names, R0 sum.
#[must_use]
pub fn traceability_partition_quickcheck() -> bool {
    let (wired, allow, total) = traceability_partition_counts();
    if wired == 0 || allow == 0 || total != TRACEABILITY_R0_MODULE_COUNT {
        return false;
    }
    if GATE_UNIFICATION_SPEC_CATALOG_IDS.is_empty() || GATE_REGISTRY_CATALOG_IDS.is_empty() {
        return false;
    }
    // Disjoint wired vs allowlist module names (O(n²) ok — static tables).
    for (module, _) in CATALOG_MODULE_WIRED {
        if ALLOW_UNUSED_CATALOG_IDS.iter().any(|m| m == module) {
            return false;
        }
    }
    // Wired catalog_ids ⊆ GATE_UNIFICATION_SPEC ∪ RUNTIME_EXTRA.
    for (_, ids) in CATALOG_MODULE_WIRED {
        for id in *ids {
            let in_spec = GATE_UNIFICATION_SPEC_CATALOG_IDS.contains(id);
            let in_extra = RUNTIME_EXTRA_GATE_CATALOG_IDS.contains(id);
            if !in_spec && !in_extra {
                return false;
            }
        }
    }
    // Gate registry rows must be Lean-wired or gate-allowlisted.
    for id in GATE_REGISTRY_CATALOG_IDS {
        let lean_wired = CATALOG_MODULE_WIRED.iter().any(|(_, ids)| ids.contains(id));
        let gate_allow = ALLOW_UNUSED_GATE_CATALOG_IDS.contains(id);
        if !lean_wired && !gate_allow {
            return false;
        }
    }
    // Cartridge registry stays disjoint from kernel gate registry.
    for id in CARTRIDGE_GATE_REGISTRY_CATALOG_IDS {
        if GATE_REGISTRY_CATALOG_IDS.contains(id) {
            return false;
        }
    }
    true
}

/// Close predicate for the **partition** surface (not production invent).
#[must_use]
pub fn traceability_partition_ceremony_closed() -> bool {
    traceability_partition_quickcheck()
        && TRACEABILITY_WIRE_HOPS.iter().filter(|h| h.wired).count()
            == W29_109_WIRE_HOP_WIRED_COUNT as usize
        && TRACEABILITY_WIRE_HOPS.len() == W29_109_WIRE_HOP_TOTAL
        && !traceability_production_wired()
}

/// Typed probe for catalog traceability partition honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraceabilityPartitionProbe {
    /// Wired Lean module row count.
    pub wired_module_count: usize,
    /// Allowlist Lean module row count.
    pub allow_unused_count: usize,
    /// Partition total (wired + allow).
    pub partition_total: usize,
    /// R0 lock module_count pin.
    pub r0_module_count_pin: usize,
    /// Partition structural quickcheck.
    pub partition_quickcheck_ok: bool,
    /// Partition ceremony close predicate.
    pub ceremony_closed: bool,
    /// Production invent claim — always false.
    pub production_wired: bool,
    /// Wire hop wired count.
    pub wire_hop_wired_count: u8,
    /// Total wire hops in map.
    pub wire_hop_total: usize,
    /// Spec catalog_id count.
    pub spec_catalog_id_count: usize,
    /// Kernel gate registry count.
    pub gate_registry_count: usize,
    /// Honest posture tag.
    pub posture_tag: &'static str,
}

/// Build introspection probe for traceability done-when checks.
#[must_use]
pub fn traceability_partition_probe() -> TraceabilityPartitionProbe {
    let (wired_module_count, allow_unused_count, partition_total) = traceability_partition_counts();
    TraceabilityPartitionProbe {
        wired_module_count,
        allow_unused_count,
        partition_total,
        r0_module_count_pin: TRACEABILITY_R0_MODULE_COUNT,
        partition_quickcheck_ok: traceability_partition_quickcheck(),
        ceremony_closed: traceability_partition_ceremony_closed(),
        production_wired: traceability_production_wired(),
        wire_hop_wired_count: TRACEABILITY_WIRE_HOPS.iter().filter(|h| h.wired).count() as u8,
        wire_hop_total: TRACEABILITY_WIRE_HOPS.len(),
        spec_catalog_id_count: GATE_UNIFICATION_SPEC_CATALOG_IDS.len(),
        gate_registry_count: GATE_REGISTRY_CATALOG_IDS.len(),
        posture_tag: TRACEABILITY_POSTURE_TAG,
    }
}

/// Honest fence flags for traceability deepen (W29-109).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TraceabilityW29109DeepenProbe {
    /// Deepen schema pin.
    pub schema_version: &'static str,
    /// Swarm cell id pin.
    pub cell_id: &'static str,
    /// Honest posture label.
    pub honest_posture: &'static str,
    /// Explicit non-claim string.
    pub non_claim: &'static str,
    /// Live partition ceremony close predicate.
    pub ceremony_closed: bool,
    /// Partition structural quickcheck.
    pub partition_quickcheck_ok: bool,
    /// Wire hops wired (partition side).
    pub wire_hop_wired_count: u8,
    /// Total wire hops in map.
    pub wire_hop_total: usize,
    /// Partition total vs R0 pin.
    pub partition_total: usize,
    /// Production invent claim — always false.
    pub production_wired_claimed: bool,
    /// Physics/fleet GREEN invent claim — always false.
    pub green_claimed: bool,
    /// OP-5 PASS invent claim — always false.
    pub op5_pass_claimed: bool,
    /// MASTER_RETICK invent claim — always false.
    pub master_retick_claimed: bool,
    /// Aggregate honesty of this deepen probe.
    pub deepen_honest: bool,
}

/// Build the W29-109 traceability deepen honesty probe from live measurements.
#[must_use]
pub fn traceability_w29_109_deepen_probe() -> TraceabilityW29109DeepenProbe {
    let production_wired_claimed = traceability_production_wired();
    let green_claimed = false;
    let op5_pass_claimed = false;
    let master_retick_claimed = false;
    let partition = traceability_partition_probe();
    let ceremony_closed = partition.ceremony_closed;
    let wire_hop_wired_count = partition.wire_hop_wired_count;
    let wire_hop_total = partition.wire_hop_total;
    let deepen_honest = W29_109_CELL_ID == "W29-109-TRACEABILITY"
        && W29_109_DEEPEN_SCHEMA_VERSION == "traceability_w29_109_deepen_v1"
        && W29_109_HONEST_POSTURE == "TRACEABILITY_DEEPEN_ONLY"
        && ceremony_closed
        && partition.partition_quickcheck_ok
        && partition.partition_total == TRACEABILITY_R0_MODULE_COUNT
        && wire_hop_wired_count == W29_109_WIRE_HOP_WIRED_COUNT
        && wire_hop_total == W29_109_WIRE_HOP_TOTAL
        && !production_wired_claimed
        && !partition.production_wired
        && !green_claimed
        && !op5_pass_claimed
        && !master_retick_claimed
        && W29_109_NON_CLAIM.contains("not GREEN")
        && W29_109_NON_CLAIM.contains("not OP-5 PASS")
        && W29_109_NON_CLAIM.contains("not production_wired")
        && W29_109_NON_CLAIM.contains("not MASTER_RETICK")
        && TRACEABILITY_POSTURE_TAG.contains("not-production");
    TraceabilityW29109DeepenProbe {
        schema_version: W29_109_DEEPEN_SCHEMA_VERSION,
        cell_id: W29_109_CELL_ID,
        honest_posture: W29_109_HONEST_POSTURE,
        non_claim: W29_109_NON_CLAIM,
        ceremony_closed,
        partition_quickcheck_ok: partition.partition_quickcheck_ok,
        wire_hop_wired_count,
        wire_hop_total,
        partition_total: partition.partition_total,
        production_wired_claimed,
        green_claimed,
        op5_pass_claimed,
        master_retick_claimed,
        deepen_honest,
    }
}

/// Whether the W29-109 traceability deepen honesty probe passes.
#[must_use]
pub fn traceability_w29_109_deepen_honest() -> bool {
    traceability_w29_109_deepen_probe().deepen_honest
}

/// Fence: refuse inventing GREEN / PRODUCTION_WIRED / MASTER / OP-5.
#[must_use]
pub fn traceability_w29_109_honest_fence_holds() -> bool {
    let p = traceability_w29_109_deepen_probe();
    p.deepen_honest
        && !p.green_claimed
        && !p.production_wired_claimed
        && !p.op5_pass_claimed
        && !p.master_retick_claimed
}

/// Deepen census — measured counts for gate_deltas (no invent flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TraceabilityW29109Census {
    /// Wired Lean modules.
    pub wired_module_count: usize,
    /// Allowlist Lean modules.
    pub allow_unused_count: usize,
    /// Partition total.
    pub partition_total: usize,
    /// Spec catalog_id count.
    pub spec_catalog_id_count: usize,
    /// Kernel gate registry count.
    pub gate_registry_count: usize,
    /// Wired hops.
    pub wire_hop_wired: u8,
    /// Total hops.
    pub wire_hop_total: usize,
    /// Unit tests in this module (compile-time pin).
    pub unit_tests_in_module: u8,
}

/// Build deepen census from live partition tables + wire map.
#[must_use]
pub fn traceability_w29_109_census() -> TraceabilityW29109Census {
    let (wired_module_count, allow_unused_count, partition_total) = traceability_partition_counts();
    TraceabilityW29109Census {
        wired_module_count,
        allow_unused_count,
        partition_total,
        spec_catalog_id_count: GATE_UNIFICATION_SPEC_CATALOG_IDS.len(),
        gate_registry_count: GATE_REGISTRY_CATALOG_IDS.len(),
        wire_hop_wired: TRACEABILITY_WIRE_HOPS.iter().filter(|h| h.wired).count() as u8,
        wire_hop_total: TRACEABILITY_WIRE_HOPS.len(),
        unit_tests_in_module: W29_109_UNIT_TESTS_IN_MODULE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_counts_sum_to_r0_module_count() {
        let (wired, allow, total) = traceability_partition_counts();
        assert!(wired > 0, "expected CATALOG_MODULE_WIRED non-empty");
        assert!(allow > 0, "expected ALLOW_UNUSED_CATALOG_IDS non-empty");
        assert_eq!(total, TRACEABILITY_R0_MODULE_COUNT);
        assert_eq!(wired + allow, 129);
    }

    #[test]
    fn partition_quickcheck_and_ceremony_closed() {
        assert!(traceability_partition_quickcheck());
        assert!(traceability_partition_ceremony_closed());
        assert!(!traceability_production_wired());
    }

    #[test]
    fn wired_modules_disjoint_from_allowlist() {
        for (module, _) in CATALOG_MODULE_WIRED {
            assert!(
                !ALLOW_UNUSED_CATALOG_IDS.contains(module),
                "overlap: {module}"
            );
        }
    }

    #[test]
    fn wired_catalog_ids_in_spec_or_runtime_extra() {
        for (module, ids) in CATALOG_MODULE_WIRED {
            for id in *ids {
                let ok = GATE_UNIFICATION_SPEC_CATALOG_IDS.contains(id)
                    || RUNTIME_EXTRA_GATE_CATALOG_IDS.contains(id);
                assert!(ok, "module {module} maps unknown catalog_id {id}");
            }
        }
    }

    #[test]
    fn gate_registry_backed_by_lean_or_allowlist() {
        for id in GATE_REGISTRY_CATALOG_IDS {
            let lean = CATALOG_MODULE_WIRED.iter().any(|(_, ids)| ids.contains(id));
            let allow = ALLOW_UNUSED_GATE_CATALOG_IDS.contains(id);
            assert!(lean || allow, "gate registry orphan: {id}");
        }
    }

    #[test]
    fn cartridge_registry_disjoint_from_kernel_gate() {
        for id in CARTRIDGE_GATE_REGISTRY_CATALOG_IDS {
            assert!(
                !GATE_REGISTRY_CATALOG_IDS.contains(id),
                "cartridge id in kernel registry: {id}"
            );
        }
    }

    #[test]
    fn wire_hops_four_of_five_wired() {
        assert_eq!(TRACEABILITY_WIRE_HOPS.len(), W29_109_WIRE_HOP_TOTAL);
        let wired = TRACEABILITY_WIRE_HOPS.iter().filter(|h| h.wired).count();
        assert_eq!(wired as u8, W29_109_WIRE_HOP_WIRED_COUNT);
        assert!(!TRACEABILITY_WIRE_HOPS[4].wired);
    }

    #[test]
    fn partition_probe_honest_surface() {
        let probe = traceability_partition_probe();
        assert!(probe.partition_quickcheck_ok);
        assert!(probe.ceremony_closed);
        assert!(!probe.production_wired);
        assert_eq!(probe.partition_total, TRACEABILITY_R0_MODULE_COUNT);
        assert_eq!(probe.wire_hop_wired_count, W29_109_WIRE_HOP_WIRED_COUNT);
        assert_eq!(probe.wire_hop_total, W29_109_WIRE_HOP_TOTAL);
        assert_eq!(probe.posture_tag, TRACEABILITY_POSTURE_TAG);
        assert!(probe.spec_catalog_id_count >= 5);
        assert!(probe.gate_registry_count >= 4);
    }

    #[test]
    fn w29_109_deepen_honest_fence_holds() {
        assert_eq!(W29_109_CELL_ID, "W29-109-TRACEABILITY");
        assert_eq!(
            W29_109_DEEPEN_SCHEMA_VERSION,
            "traceability_w29_109_deepen_v1"
        );
        let probe = traceability_w29_109_deepen_probe();
        assert!(probe.deepen_honest);
        assert!(traceability_w29_109_deepen_honest());
        assert!(traceability_w29_109_honest_fence_holds());
        assert!(!probe.green_claimed);
        assert!(!probe.production_wired_claimed);
        assert!(!probe.op5_pass_claimed);
        assert!(!probe.master_retick_claimed);
        assert!(probe.non_claim.contains("not GREEN"));
        assert!(probe.non_claim.contains("not production_wired"));
        assert!(probe.non_claim.contains("not OP-5 PASS"));
        assert!(probe.non_claim.contains("not MASTER_RETICK"));
    }

    #[test]
    fn w29_109_census_matches_partition_tables() {
        let census = traceability_w29_109_census();
        let (wired, allow, total) = traceability_partition_counts();
        assert_eq!(census.wired_module_count, wired);
        assert_eq!(census.allow_unused_count, allow);
        assert_eq!(census.partition_total, total);
        assert_eq!(census.partition_total, TRACEABILITY_R0_MODULE_COUNT);
        assert_eq!(census.wire_hop_wired, W29_109_WIRE_HOP_WIRED_COUNT);
        assert_eq!(census.wire_hop_total, W29_109_WIRE_HOP_TOTAL);
        assert_eq!(census.unit_tests_in_module, W29_109_UNIT_TESTS_IN_MODULE);
        assert_eq!(
            census.spec_catalog_id_count,
            GATE_UNIFICATION_SPEC_CATALOG_IDS.len()
        );
        assert_eq!(census.gate_registry_count, GATE_REGISTRY_CATALOG_IDS.len());
    }
}

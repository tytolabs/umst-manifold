// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Lean `catalog.json` entry coverage vs gate spec / witness registry.
//!
//! See **`docs/CATALOG_TRACEABILITY.md`** and classical anchors in **`docs/claims-vs-proofs.md`** /
//! **`docs/DUAL_PIN_ARCHITECTURE.md`**. CI: `tests/catalog_all_ids_registered.rs`.
//!
//! Catalog partition tables and structural checks (no invent GREEN / production_wired).

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
    "VonNeumannEntropy",
    "WhichPathMeasurementUpdate",
    "experiments.AutoExperimenterPlaceholder",
    "lakefile",
    "scripts.print_axioms",
    // Formal modules added by the 2026-10-02 catalog bump (umst-formal 0204d84); no Rust gate wiring yet.
    "Chem.Conservation",
    "Chem.Constants.ConstantsSheaf",
    "Chem.Constants.TemperatureGraph",
    "Chem.ConvexHull",
    "Chem.Element",
    "Chem.Environment.EnvGibbsDuhem",
    "Chem.Environment.EnvSampleSections",
    "Chem.Environment.EnvironmentContinuum",
    "Chem.GroundStates",
    "Chem.KleisliInteract",
    // Chemistry as a functor into the process family; consumer is umst-chem, which no runtime gate catalog_id names
    // (follow-up cell CATALOG-ID-CHEM-PROCESS-FUNCTOR).
    "Chem.ProcessFunctor",
    "Chem.OreAssemblage",
    "Chem.PatternNuance",
    "Chem.RefineCost",
    "Chem.SecondLaw",
    "CoarseGraining",
    // Composition law of GSM atoms (commutative monoids, convex cones): the cartridge composites it admits are built
    // in umst-cartridges, which no runtime gate catalog_id names (follow-up cell CATALOG-ID-GSM-COMPOSITION).
    "Composition.GsmMonoid",
    "CompressionCost",
    "Concrete.MicroMechanics",
    "Concrete.PoromechanicsB3",
    // Closed-form Powers capillary porosity; its rounded closure (0.36, 0.32) is carried by umst-cartridge-poromechanics
    // `capillary_porosity`, which no runtime gate catalog_id names (follow-up cell CATALOG-ID-POROMECHANICS-CAPILLARY).
    "Concrete.PowersCapillary",
    "Concrete.PowersVolume",
    "Concrete.ShrinkageB4",
    "Concrete.StiffnessTransition",
    // Second-law bounds of the constants table (generated from constants.json) and the electromagnetic, poro-continuum
    // and solid-inelastic bounds: consumers are the umst-constants bound rows and umst-cartridge-grounded
    // SecondLawBoundRow, which no runtime gate catalog_id names (follow-up cell CATALOG-ID-CONSTANTS-BOUNDS).
    "Constants.Bounds",
    "Constants.SI",
    "Constants.SIBridge",
    // Second-law bounds on dissipation coefficients and elastic moduli: the umst-constants table carries them as
    // bounds (constants.json), and no runtime gate cites the Lean module.
    "Constants.SecondLawDissipation",
    "Constants.SecondLawElastic",
    "Constants.SecondLawElectromagnetic",
    "Constants.SecondLawPoroContinuum",
    "Constants.SecondLawSolidInelastic",
    // Convex dissipation channels, Prony arms and passive restitution through `.transition`: formal-only, no
    // runtime gate cites it yet.
    "ConvexPhiChannels",
    "ConvexPhiDissipation",
    // Grammar of convex dissipation potentials; its Rust form is the umst-cartridges continuum and solid-inelastic
    // dissipation, which no runtime gate catalog_id names (follow-up cell CATALOG-ID-GSM-COMPOSITION).
    "Convex.PhiGrammar",
    "CoordinationContract",
    "CoordinationCost",
    "CoordinationCostP6",
    "CostOfInformation",
    "Crypto.CryptoHypothesis",
    "Economic.PhysicsConstrainedAI",
    "ExamplesQubitEpistemic",
    // GENERIC entropy production; consumer is umst-chem `fluctuation_theorem`, which no runtime gate catalog_id names
    // (follow-up cell CATALOG-ID-CHEM-PROCESS-FUNCTOR).
    "Generic.EntropyProduction",
    "Excitement",
    // `selectBy` over any linear order; its Rust twin is umst-algebra `select_by` (SELECT_BY_LEAN_ANCHOR), which is
    // no runtime gate `catalog_id`, so the module stays formal-only like `Excitement`.
    "ExcitementBy",
    "ExcitementByProofs",
    "ExcitementProofs",
    "InterpretationFunctor",
    "KnowingFibreInstance",
    "MaterialClass",
    "MeaningState",
    "OneInequalitySecondLaw",
    "Process",
    "SemanticEconomicModules",
    "SemanticFormal",
    "SemanticSecondLaw",
    "Solver.CombinatorLaws",
    "Solver.ProblemSize",
    "Urge.AdmitKleisli",
    "Urge.AntichainIndependent",
    "Urge.AppendOnly",
    "Urge.BackupRecovery",
    "Urge.CarrierProduct",
    "Urge.ClosedLoopWitness",
    "Urge.CobSocialGraph",
    "Urge.CollaborativeObject",
    "Urge.CompactionAsArrow",
    "Urge.CompactionComposite",
    "Urge.ContentAddressed",
    "Urge.CountedConsume",
    "Urge.CryptoCompose",
    "Urge.DropProvenance",
    "Urge.EntityRemote",
    "Urge.ExactAlgCoeffs",
    "Urge.ExactRatFreeEnergy",
    "Urge.ExcitementImport",
    "Urge.FabricNodes",
    "Urge.FastPathAdmit",
    "Urge.FrugalMi",
    "Urge.GateBeforeSync",
    "Urge.GeometricMemory",
    "Urge.GossipTick",
    "Urge.HilbertPersist",
    "Urge.HistoryFunctor",
    "Urge.HomologNotCopy",
    "Urge.InvariantWitness",
    "Urge.KleisliClone",
    "Urge.KleisliFetch",
    "Urge.KleisliInherit",
    "Urge.KleisliMerge",
    "Urge.KleisliPush",
    "Urge.KleisliRecover",
    "Urge.KleisliStatus",
    "Urge.LatencyBudget",
    "Urge.MergeSafe",
    "Urge.MetaReflexive",
    "Urge.NoWorkLost",
    "Urge.OccupancyNotForge",
    "Urge.OriginRefuse",
    "Urge.OverlayAdmit",
    "Urge.PadmaMembraneBacking",
    "Urge.ProvenancePreserve",
    "Urge.Reconcile",
    "Urge.ReplicaClass",
    "Urge.ReplicaCoalgebra",
    "Urge.ResidueReturn",
    "Urge.SdfCanonical",
    "Urge.SignedPropagate",
    "Urge.Supervision",
    "Urge.ThreeDagSeparate",
    "Urge.Transport",
    "Urge.UmstTomlRemotes",
    "Urge.UnmeasuredMeasured",
    "Urge.WorktreeExclusive",
    "WaveShape",
    "Web",
    "Web.WebMat",
    "experiments.CementThermo.cement_thermo_minimiser_unique_strict",
    "tests.AxiomProbe",
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
    // `landauerFactor_isLeast`: the least work per kelvin the `.erase` case admits for a uniform bit is ln 2, the
    // factor `ThermodynamicCBF::calculate_landauer_cost` charges per resolved bit.
    (
        "Constants.SecondLawLandauerFactor",
        &["umst.gate.landauer_cbf"],
    ),
    // `whichPath_measureFeedback_secondLaw`: a readout of I nats costs k_B·T·I as a `measureFeedback` case; the
    // gate debits that cost for the mutual information an update resolves.
    ("KnowingFibreLaw", &["umst.gate.landauer_cbf"]),
    ("LandauerExtension", &["umst.gate.landauer_cbf"]),
    ("LandauerLaw", &["umst.gate.landauer_cbf"]),
    // `SemanticSecondLaw` is absent from the composed Lean export (129-module lock);
    // cold proof + `umst.gate.semantic_cbf` wiring lives in `catalog.lock.json`
    // `semantic_witnesses` (HCOM-004), not this partition table.
    ("MeasurementCost", &["umst.gate.landauer_cbf"]),
    ("MonoidalState", &["umst.gate.cd_transition"]),
    // The runtime gate check implies the one predicate: `Concrete.SecondLaw.gateCheck_secondLaw`.
    ("Concrete.SecondLaw", &["umst.gate.cd_transition"]),
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

// ── Partition wire map · structural checks ────────────────────────────────────

/// R0 module count the Lean-module partition must cover: the module rows of the pinned formal export
/// (`FORMAL_CATALOG_MODULE_COUNT`, computed by build.rs from `artifacts/upstream_catalog.json`).
pub const TRACEABILITY_R0_MODULE_COUNT: usize = super::FORMAL_CATALOG_MODULE_COUNT as usize;

/// One hop in the catalog traceability wire map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct TraceabilityWireHop {
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
pub(crate) const TRACEABILITY_WIRE_HOPS: &[TraceabilityWireHop] = &[
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
pub(crate) const fn traceability_production_wired() -> bool {
    false
}

/// Measured Lean-module partition counts (wired ∪ allowlist).
#[must_use]
pub(crate) fn traceability_partition_counts() -> (usize, usize, usize) {
    let wired = CATALOG_MODULE_WIRED.len();
    let allow = ALLOW_UNUSED_CATALOG_IDS.len();
    (wired, allow, wired + allow)
}

/// Structural quickcheck: partition tables non-empty, disjoint module names, R0 sum.
#[must_use]
pub(crate) fn traceability_partition_quickcheck() -> bool {
    let (wired, allow, total) = traceability_partition_counts();
    if wired == 0 || allow == 0 || total != TRACEABILITY_R0_MODULE_COUNT {
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

/// Whether the wire map leaves exactly one open invent hop at the tail.
#[must_use]
fn traceability_wire_map_partition_closed() -> bool {
    let hops = TRACEABILITY_WIRE_HOPS;
    if hops.len() < 2 {
        return false;
    }
    let tail = hops.len() - 1;
    hops[..tail].iter().all(|h| h.wired) && !hops[tail].wired
}

/// Close predicate for the **partition** surface (not production invent).
#[must_use]
pub(crate) fn traceability_partition_ceremony_closed() -> bool {
    traceability_partition_quickcheck()
        && traceability_wire_map_partition_closed()
        && !traceability_production_wired()
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
        let lock = super::super::CatalogLock::from_bundled().expect("bundled lock");
        assert_eq!(lock.module_count.map(|n| n as usize), Some(wired + allow));
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
    fn wire_hops_tail_invent_fence_open() {
        assert!(traceability_wire_map_partition_closed());
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Lean `catalog.json` entry coverage vs gate spec / witness registry.
//!
//! See **`docs/CATALOG_TRACEABILITY.md`** and classical anchors in **`docs/claims-vs-proofs.md`** /
//! **`docs/DUAL_PIN_ARCHITECTURE.md`**. CI: `tests/catalog_all_ids_registered.rs`.

/// Lean modules with **no** dedicated Rust gate wiring yet (formal-only or scaffold).
///
/// Every `catalog.json` `module` must appear here **or** in [`CATALOG_MODULE_WIRED`].
pub const ALLOW_UNUSED_CATALOG_IDS: &[&str] = &[
    "Activation",
    "Adjoint",
    "Behavior.SDFCanonical",
    // umst-formal fiber rename; runtime Landauer CBF wiring uses double-slit `MeasurementCost`.
    "ClassicalMeasurementCost",
    "Complementarity",
    "Convergence",
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
    "EndConditions",
    "EnrichedAdmissibility",
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
    "GaloisGate",
    "GeneralDimension",
    "GeneralResidualCoherence",
    "GeneralVisibility",
    "GraphProperties",
    "Helmholtz",
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
        "Gate",
        &["umst.gate.cd_transition", "umst.gate.kleisli_unit"],
    ),
    ("GateCompat", &["umst.gate.cd_transition"]),
    ("InformationCostIdentity", &["umst.gate.landauer_cbf"]),
    ("LandauerBound", &["umst.gate.landauer_cbf"]),
    ("LandauerExtension", &["umst.gate.landauer_cbf"]),
    ("LandauerLaw", &["umst.gate.landauer_cbf"]),
    ("MeasurementCost", &["umst.gate.landauer_cbf"]),
    ("MonoidalState", &["umst.gate.cd_transition"]),
    ("Naturality", &["umst.gate.cd_transition"]),
    ("PhysicsConstrainedAI", &["umst.gate.landauer_cbf"]),
    ("ProbeOptimization", &["umst.gate.kleisli_unit"]),
    ("QRBridge", &["umst.gate.cd_transition"]),
    ("UMSTCore", &["umst.gate.cd_transition"]),
    ("Constitutional", &["umst.gate.kleisli_unit"]),
    (
        "Economic.KleisliAdmissibilityComposition",
        &["umst.gate.kleisli_unit"],
    ),
    ("DIBKleisli", &["umst.gate.kleisli_unit"]),
    ("DEC", &["umst.gate.cd_transition"]),
    ("Powers", &["thermodynamic_mix", "umst.gate.http_shim"]),
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
/// Telemetry only: [`crate::gate::http_manifest::HttpMixGateEvaluator::gate_family`] uses
/// [`MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY`]. Not in [`GATE_REGISTRY_CATALOG_IDS`] or
/// [`ALLOW_UNUSED_GATE_CATALOG_IDS`].
pub const PREDICTION_VS_PHYSICS_CATALOG_ID_DEPRECATED: &str = "umst.gate.prediction_vs_physics";

/// `gate_family` for HTTP bulk strength closure + Parrott kinetics (not a `catalog_id`).
pub const MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY: &str = "mix_prediction_vs_physics";

/// Stable slug for [`crate::ai::cbf::ThermodynamicCBF`] / [`FormalReject::ThermodynamicControlBarrier`].
pub const LANDAUER_CBF_CATALOG_ID: &str = "umst.gate.landauer_cbf";

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

fn count_catalog_modules(path: &std::path::Path) -> Result<usize, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let modules = v
        .get("modules")
        .and_then(|m| m.as_array())
        .ok_or_else(|| "catalog.json missing modules array".to_string())?;
    Ok(modules.len())
}

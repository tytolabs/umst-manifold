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
    (
        "Powers",
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

/// Stable slug for [`crate::gate::ThermodynamicMixEvaluator`] / mix registry host rejects.
pub const THERMODYNAMIC_MIX_CATALOG_ID: &str = "thermodynamic_mix";

/// Stable slug for [`crate::gate::http_manifest::HttpMixGateEvaluator`] HTTP shim rejects.
pub const HTTP_SHIM_CATALOG_ID: &str = "umst.gate.http_shim";

/// Deprecated `catalog_id` slug — superseded by [`HTTP_SHIM_CATALOG_ID`].
///
/// Telemetry only: [`crate::gate::http_manifest::HttpMixGateEvaluator::gate_family`] uses
/// [`MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY`]. Not in [`GATE_REGISTRY_CATALOG_IDS`] or
/// [`ALLOW_UNUSED_GATE_CATALOG_IDS`].
pub const PREDICTION_VS_PHYSICS_CATALOG_ID_DEPRECATED: &str = "umst.gate.prediction_vs_physics";

/// `gate_family` for HTTP mix Powers closure + Parrott hydration (not a `catalog_id`).
pub const MIX_PREDICTION_VS_PHYSICS_GATE_FAMILY: &str = "mix_prediction_vs_physics";

/// Stable slug for [`crate::ai::cbf::ThermodynamicCBF`] / [`FormalReject::ThermodynamicControlBarrier`].
pub const LANDAUER_CBF_CATALOG_ID: &str = "umst.gate.landauer_cbf";

/// Gate slugs implemented in Rust but not yet in the spec table (see `claims-vs-proofs.md` note).
pub const RUNTIME_EXTRA_GATE_CATALOG_IDS: &[&str] = &["thermodynamic_mix"];

/// [`GateEvaluator::catalog_id`] values implemented in `src/gate/` (registry SSOT for CI).
pub const GATE_REGISTRY_CATALOG_IDS: &[&str] = &[
    "umst.cartridge.concrete.policy",
    "umst.gate.cd_transition",
    "umst.gate.http_shim",
    "umst.gate.kleisli_unit",
    "thermodynamic_mix",
];

/// Runtime gate `catalog_id`s with **no** Lean `catalog.json` backing row (HTTP shim, mix filter, cartridge).
pub const ALLOW_UNUSED_GATE_CATALOG_IDS: &[&str] = &["umst.cartridge.concrete.policy"];

/// Default relative path from `umst-manifold` to the Lean exporter catalog.
pub const DEFAULT_UPSTREAM_CATALOG_JSON: &str = "../umst-formal-double-slit/artifacts/catalog.json";

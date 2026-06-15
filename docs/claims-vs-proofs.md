# Claims vs proofs mapping (UMSSSOT)

**Foundational split** (normative): **proofs = versioned library** (pinned Lean export + digest lock), **gates = law** (mandatory witness rejects on transitions). The hot path never replays Lean tactics or runs per-step `lake build`; rows below map **hand-aligned** Rust witnesses to theorems in the pinned library revision, not a per-step proof runner. Witness evaluation order and failure priority: [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) — [§ Proof library · gate law · MI envelope · no Rust axioms](GOD_GRADE_WITNESS_LADDER.md#proof-library--gate-law--mi-envelope--no-rust-axioms).

Traceability ledger: **Lean module / theorem family** → stable **`catalog_id`** → **Rust SSOT** → **status**. Canonical Lean inventory is `umst-formal-double-slit/artifacts/catalog.json` (**122 modules** — double-slit **69** + `umst-formal` **53** non-overlapping after W9 prime-spectral mirror; export via `tools/lean_export/export_catalog.py` with `--also-lean-root ../umst-formal/Lean` and `APPROVE_CROSS_REPO_MERGE=1`); manifold pins digest in `artifacts/catalog.lock.json` (`build.rs` → `UMST_CATALOG_LOCK_SHA256_HEX`). Modular per-fiber digests (recommended audit model): [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md). Gate routing: `docs/GateUnificationSpec.md`, `docs/PROTOTYPE_GATE_MAP.md`, `docs/TCB.md`.

**Row counts:** this table has **48 engineering-facing traceability rows** (bundles / gate surfaces), not one row per catalog module. The **73 catalog modules** without a dedicated row are listed in [Appendix A](#appendix-a--catalog-modules-not-in-traceability-table). See `docs/CATALOG_ROW_COUNT.md`.

**Runtime honesty:** **proved** means Lean-mechanized only — the hot path does **not** replay tactics or treat the theorem as enforced at runtime. Rows naming `src/` use **hand-aligned** (host mirrors obligations; parity/tests pin intent).

## Status legend

| Status | Meaning |
|--------|---------|
| **proved** | Theorem family mechanized in Lean (module in `catalog.json` with `theorem`/`axiom` entries); **no** Rust hand-alignment in this row; runtime does not enforce. Do not use for rows that name a `src/` artefact. |
| **hand-aligned** | Rust mirrors Lean/prototype obligations on host types; parity/manifest/digest pins intent. |
| **TCB** | Runtime trusted boundary (`docs/TCB.md`) or build-time digest/HTTP enforcement. |

## Lean ↔ catalog_id ↔ Rust

| Lean module / theorem family | `catalog_id` | Rust artefact | Status |
|-----------------------------|--------------|---------------|--------|
| `Gate` — `gateCheckSound` / `gateCheckComplete`, `clausiusDuhemFwd`, `forwardHydrationAdmissible` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs`, `src/gate/evaluator.rs` | hand-aligned |
| `Gate` — `kleisliAdmissibility`, `admissibleN_compose` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs` | hand-aligned |
| `UMSTCore` — `Admissible`, `MassCond` / `DissipCond` / `HydratCond` / `StrengthCond` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | hand-aligned |
| `Naturality` — `gateMaterialAgnostic`, `naturalitySquare`, `initialStateMassConserved` | `umst.gate.cd_transition` | `src/manifest/umst_manifest.rs` | hand-aligned |
| `MonoidalState` — `combine_*`, `combine_freeEnergy_le` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` (host transition only; combine lemmas Lean-only) | hand-aligned |
| `GateCompat` — `admissible_thermoCalibrated*`, `calibratedHydration_*` | `umst.gate.cd_transition` | `src/gate/mix_proposal.rs` | hand-aligned |
| `DoubleSlit` — `measurementUpdateWhichPath_gateEnforcement` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | hand-aligned |
| `QRBridge` — `admissible_thermodynamicStateToReal` | `umst.gate.cd_transition` | — (no QR bridge crate yet) | proved |
| `Activation` / `FiberedActivation` — engine activation lemmas | — | — (formal-only; no manifold engine enum) | proved |
| `LandauerBound` — `pathEntropyBits_*`, `landauerCostDiagonal_*`, `residualCoherenceCapacity_*` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs`, `src/gate/cbf_bridge.rs` | hand-aligned |
| `LandauerLaw` — `landauerBound`, `physicalSecondLaw` (**axiom**) | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | TCB |
| `LandauerExtension` — `landauerBound_nBit`, `landauerEnergy_mono` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | hand-aligned |
| `LandauerEinsteinBridge` — `massEquivalent_*` SI bounds | — | — | proved |
| `EpistemicMI` — `epistemicMI_*`, `epistemicLandauerCost_*` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` (`ManifoldGateway`) | hand-aligned |
| `EpistemicSensing` — `whichPathProbe_*`, `LandauerCostFromProbeStrength_*` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | hand-aligned |
| `EpistemicTrajectoryMI` — `cumulativeEpistemicMI_*` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | hand-aligned |
| `MeasurementCost` — `measurementCost_le_landauerBitEnergy` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | hand-aligned |
| `PhysicsConstrainedAI` — `gating_landauer_le_one_bit` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | hand-aligned |
| `InformationCostIdentity` — `residualCoherence_*`, `landauer_ratio_*` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | hand-aligned |
| `ErasureChannel` — `resetChannel_landauerCost_zero`, `idealResetErasure_saturates` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | hand-aligned |
| `ManifoldGateway` + `ThermodynamicCBF` (tensor topology step) | `umst.gate.landauer_cbf` | `src/ai/ppo.rs`, `src/ai/cbf.rs` | TCB |
| `ProbeOptimization` — `ProbeSelectionAdmissible_*`, `exists_constrainedOptimalAt` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs`, `src/gate/mix_eval_registry.rs` | hand-aligned |
| Mix transition filter (prototype `ThermodynamicMixFilter`) | `thermodynamic_mix` | `src/gate/mix_proposal.rs`, `src/gate/mix_eval_registry.rs` | hand-aligned |
| HTTP `POST /gate` + Powers/hydration closure | `umst.gate.http_shim` | `src/gate/http_manifest.rs`, `src/bin/gate_server.rs` | TCB |
| `EpistemicRuntimeContract` — `rollout_satisfies_traceContractMI` / `traceContractLandauer` | `umst.formal.catalog_lock` | `src/ai/formal.rs` (feature `formal-witness`), `src/ai/ppo.rs` | hand-aligned |
| `EpistemicRuntimeSchemaContract` — `EmittedTraceSchema`, `EmittedStepRecord`, `emittedRolloutConsistent_toPerStepConsistent` | — | `src/ros/epistemic_trace.rs` (features `ros2-contract`, `serde`); `tests/epistemic_trace_schema.rs` | hand-aligned |
| Full Lean export bundle (119 modules, unified catalog digest) | `umst.formal.catalog_lock` | `build.rs`, `src/runtime/catalog/mod.rs`, `artifacts/catalog.lock.json` | TCB |
| `Complementarity` / `QuantumClassicalBridge` — Englert / fringe–path | — | — | proved |
| `WhichPathMeasurementUpdate` — collapse / Landauer on which-path | — | — | proved |
| `DensityState` / `MeasurementChannel` — PSD, trace, `whichPath_map` | — | — | proved |
| `VonNeumannEntropy` / `KroneckerEigen` / `QuantumMutualInfo` — spectral entropy, tensor additivity | — | — | proved |
| `DataProcessingInequality` — qubit / unitary-Kraus DPI instances | — | — | proved |
| `KleinInequality` — `spectralRelativeEntropy_nonneg` | — | — | proved |
| `GeneralResidualCoherence` / `GeneralVisibility` / `PMICVisibility` | — | — | proved |
| `EpistemicPolicy` / `EpistemicDynamics` / `EpistemicGalois` | — | — | proved |
| `EpistemicNumericsContract` → `EpistemicTelemetryBridge` (trace/schema coherence) | — | — | proved |
| `PrototypeSolverCalibration` / `EpistemicTraceDrivenCalibrationWitness` | — | `trace-calibration`: `calibrate_eta_bound_from_trace` + `ManifoldGateway::calibrate_eta_from_trace` (catalog η); `check_prototype_calibration_bounds` (aggregate ε); **not** `NumericTraceApproxConsistent` without rollout | proved (Lean); Rust host checks only |
| `ExamplesQubit` — `rhoPlus_*`, `rhoZero_*` policy/probe witnesses | — | — | proved |
| `SimLeanBridge` — `SimComplementarityWitness`, `SimLandauerWitness` | — | — | proved |
| `FormalFoundations` — `umst_double_slit_formal_complete` | `umst.formal.catalog_lock` | digest pin only | proved |
| `umst-formal` `Gate.lean` (parallel cement gate, ℚ) | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | hand-aligned |
| `Constitutional` — `WellTyped`/`WellTypedN`, `kleisliComposeWellTypedN`, `sequentialCompositionSafe`, `kleisliComposeAssoc` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs`, `src/gate/evaluator.rs`, `src/gate/mix_eval_registry.rs` | hand-aligned |
| `DIBKleisli` — DIB monad `leftUnit`/`rightUnit`/`assocM`, `kleisliAssoc`, `dib_semantic_step_admissible` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs`, `src/gate/mix_eval_registry.rs` | hand-aligned |
| `Economic.KleisliAdmissibilityComposition` — re-exports / composes `Constitutional` Kleisli lemmas | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs`, `src/gate/mix_eval_registry.rs` | hand-aligned |
| `Powers` — `powers_monotone`, `powersStateFcMonotone`, `powersStateAdmissible`, gel-space ratio lemmas | `thermodynamic_mix`, `umst.gate.http_shim` | `src/gate/mix_proposal.rs`, `src/gate/http_manifest.rs`, `src/gate/concrete_cartridge.rs` | hand-aligned |
| `DEC` — `boundary_squared_zero`, `hodge_laplacian_symmetric`, `discrete_stokes`, `laplacian_row_sum_zero` | `umst.gate.cd_transition` | `src/physics/dec_primal.rs`, `src/physics/dec_operators.rs`; `tests/dec_identities.rs` | hand-aligned |
| `RegimeSoundness` — `warnings_empty_iff_in_regime`, `warning_dimension_violated`, `in_regime_decidable` | `umst.cartridge.concrete.regime` *(traceability; no manifold `GateEvaluator`)* | `umst-concrete-cartridge` `Profile::regime_check_scalars` (`calibration.rs`); formal anchors on `WaterCementRatio` / `TemperatureK` / `safety_margin` — **not** replayed on manifold hot path | hand-aligned |
| ROS gate ack / catalog hash on buses | `umst.gate.cd_transition` (telemetry) | `src/ros/contract.rs` (feature `ros2-contract`) | hand-aligned |

## Appendix A — Catalog modules not in traceability table

The following **73** modules appear in `umst-formal-double-slit/artifacts/catalog.json` (119 total) but are not given a dedicated row above. They remain **Lean-mechanized** (export witnesses) unless noted; status here is **inventory-only** (no manifold `catalog_id` / Rust mapping claimed). Classical `umst-formal` rows (`repo: umst-formal`) are included after unified export — not a separate digest fiber.

| Module | Role (from export / naming) |
|--------|----------------------------|
| `DoubleSlitCore` | Core double-slit definitions upstream of `DoubleSlit` |
| `InfoEntropy` | Shannon / von Neumann entropy utilities (imported by DPI, etc.) |
| `GeneralDimension` | Dimension-generic residual / visibility scaffolding |
| `PMICEntropyInterior` | PMIC entropy interior bounds (sibling to `PMICVisibility`) |
| `SchrodingerDynamics` | Schrödinger evolution formalism |
| `LindbladDynamics` | Lindblad master-equation layer |
| `LindbladStreamD` | Stream-D Lindblad variant |
| `TensorPartialTrace` | Partial-trace / tensor reduction lemmas |
| `MatrixLog` | Matrix logarithm / spectral helpers |
| `LogSum` | Log-sum-exp / numerical stability lemmas |
| `EpistemicPerStepNumerics` | Per-step epistemic numeric contracts |
| `EpistemicTelemetryApproximation` | Telemetry approximation bounds |
| `EpistemicTelemetryQuantitativeUtility` | Quantitative utility of telemetry probes |
| `EpistemicTelemetrySolverCalibration` | Solver calibration via telemetry |
| `EpistemicTraceDerivedEpsilonCertificate` | Trace-derived ε certificates |
| `FlashMoERuntimeScaffold` | Flash-MoE runtime scaffold (formal stub) |
| `Test3`, `Test4`, `TestEntropy`, `TestFixes`, `TestMixed` | Lean test / regression modules |
| `test_tensor_eigen` | Tensor eigenvalue test harness |
| `lakefile` | Build metadata captured by export (not a proof obligation) |

To add a row: pick a `catalog_id` from `GateUnificationSpec.md` (or extend spec), then map Rust in `src/gate/` / `src/ai/` per existing hand-aligned rows.

## Appendix B — `umst-formal` Economic layer (unified export)

Seventeen `Economic.*` modules (+ `EconomicDomain` definitions) ship in the **119-module** pin. They are **classical meso-layer** lemmas (burden, Shannon–Landauer temperature, exploration budgets) — not runtime oracles ([`../umst-formal/SAFETY-LIMITS.md`](../umst-formal/SAFETY-LIMITS.md)).

| Module | `catalog_id` | Manifold status |
|--------|--------------|-----------------|
| `Economic.KleisliAdmissibilityComposition` | `umst.gate.kleisli_unit` | hand-aligned — main table |
| `Economic.BurdenRecursionIsAdmissible`, `Economic.StochasticBurdenExpectation`, … | — | proved / inventory-only (Appendix A allowlist) |
| `Economic.PhysicsConstrainedAI` | — | **overlap:** primary `PhysicsConstrainedAI` wins in export; see main table row |

Default manifest telemetry lanes for classical anchors: `GateRegistry::default_for_unified_catalog()` in `src/manifest/umst_manifest.rs`.

## Engineering claim surface (summary)

| Claim surface | Operational proof witness | Formal / academic anchor |
|---------------|---------------------------|--------------------------|
| Thermodynamic hydration transitions obey CD inequality (host `f64`) | `gate::thermo_transition`, `GateEvaluator` (`umst.gate.cd_transition`); `tests/gate_cbf_parity.rs` | `Gate`, `GateCompat`; catalog digest in `catalog.lock.json` |
| Differentiable topology steps respect Landauer / CD scalar barrier | `ManifoldGateway` + `ThermodynamicCBF` (`umst.gate.landauer_cbf`) | `LandauerBound`, `EpistemicMI`, `PhysicsConstrainedAI` |
| Mix / ML gate vs physics strength envelope | `thermodynamic_mix`, `http_manifest`, `gate_server` | `GateCompat`, prototype Powers closure |
| Kleisli admissibility + registry routing | `gate::kleisli`, `mix_eval_registry`; `tests/gate_kleisli.rs`, `gate_parity_fixture.rs` | `Gate.kleisliAdmissibility`, `Constitutional`, `DIBKleisli`, `Economic.KleisliAdmissibilityComposition`, `ProbeOptimization` |
| Powers gel-space / strength envelope | `mix_proposal`, `http_manifest`, `concrete_cartridge` | `Powers` (`powersStateAdmissible`, monotonicity lemmas) |
| DEC topology identities (Burn tests) | `dec_primal`, `dec_operators`; `tests/dec_identities.rs` | `DEC` (`boundary_squared_zero`, Stokes, Laplacian symmetry) |
| Cross-language embodied buses | `ros::contract` (`gate_catalog_id`, `catalog_hash`) | Epistemic telemetry/schema contracts (no ROS runtime in manifold) |
| Epistemic v2 emitted trace (serde witness envelope) | `ros::epistemic_trace`, `tests/epistemic_trace_schema.rs` | `EpistemicRuntimeSchemaContract` (rollout consistency / bounds: Track G.2) |
| Hyperbox regime warnings (cartridge `f32` scalars) | `umst-concrete-cartridge` `regime_check_scalars` → CLI stderr / `result.v2` warnings (operational) | `RegimeSoundness` on ℚ hyperbox — **Lean proved**; **no** host evaluator; see § RegimeSoundness below |

## RegimeSoundness (honest split)

| Layer | Status |
|-------|--------|
| **Lean (`umst-formal/Lean/RegimeSoundness.lean`)** | **Mechanised** — `warnings_empty_iff_in_regime`, `warning_dimension_violated`, `in_regime_decidable` (export module in 119-pin digest). |
| **Manifold host** | **No** `GateEvaluator` and **no** runtime proof replay — warnings are not “proved at runtime” on the inference hot path. |
| **Cartridge mirror** | **Hand-aligned** — `Profile::regime_check_scalars` emits axis-aligned box violations on host `f32`; anchored to the Lean lemma family via `formal_anchor` comments (`FORMAL_GROUNDING_AUDIT.md`). |
| **Honest gaps** | ℚ↔`f32` interval semantics, profile TOML bounds vs Lean parameters, and warning **policy** (stderr vs hard reject) are engineering alignment — not extracted proof terms. |

Track **J.3** documents CLI/trace policy; Track **G.2** (epistemic per-step numerics bounds) is **orthogonal** — do not count RegimeSoundness doc as G.2 closure.

## Notes

- **Counts:** 48 traceability rows in § Lean ↔ catalog_id ↔ Rust (bundles / gate surfaces); **73** catalog modules without a dedicated row (Appendix A); **119** total in unified export. Pin: `docs/CATALOG_ROW_COUNT.md`; dual-pin policy: `docs/DUAL_PIN_ARCHITECTURE.md`.
- **`catalog.json` path**: lives under **`umst-formal-double-slit/artifacts/`**, not `umst-manifold/artifacts/` (manifold only ships the lock stub).
- **`thermodynamic_mix`** is wired in code but not namespaced in `GateUnificationSpec.md` (consider `umst.gate.thermodynamic_mix`).
- **`umst.gate.kleisli_unit`**: [`KleisliUnitEvaluator`](../../umst-manifold/src/gate/kleisli.rs) implements [`GateEvaluator`](../../umst-manifold/src/gate/evaluator.rs); hand-aligned to double-slit `Gate` and `umst-formal` `Constitutional` / `DIBKleisli`.
- **`RegimeSoundness`**: Lean theorem family is **proved in Mathlib**; runtime status is **hand-aligned** only via the concrete cartridge — see § RegimeSoundness. CI allowlist: `tests/regime_soundness_claims_allowlist.rs`.
- Re-export after Lean edits: `APPROVE_CROSS_REPO_MERGE=1` + `export_catalog.py --lean-root Lean --also-lean-root ../umst-formal/Lean` (or `verify_umst_stack.sh` / `bidirectional_catalog_check.sh` when `umst-formal` is present), then refresh `upstream_catalog_digest_hex` in manifold `catalog.lock.json`.

## Process

Stack verification (full flags and CI map: [`VERIFY.md`](VERIFY.md) §5):

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=../umst-formal-double-slit   # sibling layout; see VERIFY.md §5.0
bash scripts/verify_umst_stack.sh
bash scripts/bidirectional_catalog_check.sh
```

`verify_umst_stack.sh` runs digest check (when formal present), full `cargo test` gate matrix (`gate_dual_run_parity`, `formal-witness`, `gate_server_http`, …), and invokes `bidirectional_catalog_check.sh` when found. The bidirectional script may also run as a separate CI step after the stack script.

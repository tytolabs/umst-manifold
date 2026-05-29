# Catalog coverage audit

**Generated:** 2026-05-21  
**Scope:** `umst-manifold/src/runtime/catalog/`, `umst-manifold/src/gate/`, `src/ai/formal.rs`, `tests/`  
**Canonical Lean inventory:** `umst-formal-double-slit/artifacts/catalog.json` (**119** modules, digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` in `umst-manifold/artifacts/catalog.lock.json`; primary fiber **69** @ `c1d9ba2…` — rollback only in [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md))

**Automation:** CI partition enforced by `tests/catalog_all_ids_registered.rs` and `src/runtime/catalog/traceability.rs` — see [`CATALOG_TRACEABILITY.md`](CATALOG_TRACEABILITY.md).

**God-grade:** Witness priority and second formal fiber — [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md). Pipeline — [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md).

---

## `proof_ir` note

`catalog.json` exports `path`, `module`, `content_sha256`, `declarations`, `import_lines` only — **no `proof_ir` field**. Coverage classes:

| Class | Meaning |
|-------|---------|
| **runtime-wired** | `catalog_id` string used in `GateEvaluator` or `EmbodiedOrchestrator` routing |
| **claims-rust** | `catalog_id` in `claims-vs-proofs.md` with Rust SSOT outside `catalog_id()` (e.g. CBF / Kleisli) |
| **digest-only** | Module only attested via full-bundle pin (`umst.formal.catalog_lock`) |
| **catalog-only** | In `catalog.json`, no dedicated `catalog_id` / Rust row (see Appendix A in `claims-vs-proofs.md`) |

---

## Summary counts (semantic / runtime alignment)

| Status | Count | Meaning |
|--------|------:|---------|
| **used (Y)** | 13 | Hand-aligned or enforced runtime path |
| **partial** | 9 | Digest/witness only, doc traceability, or incomplete `catalog_id` wiring |
| **unused (N)** | 47 | Mechanized in Lean only |
| **Total (primary fiber)** | 69 | Per-module table below scopes **umst-formal-double-slit** primary export; unified pin is **119** modules |

**CI partition (traceability.rs, verified 2026-05-21):** **25** modules in `CATALOG_MODULE_WIRED`, **94** in `ALLOW_UNUSED_CATALOG_IDS` — **119** total, no gaps (`cargo test --test catalog_all_ids_registered`).

---

## Static ~26% vs operational \(U(t)\) (do not conflate)

| Metric | Formula / source | Value (grep / CI, 2026-05-21) |
|--------|------------------|--------------------------------|
| **Pin coverage** \(U_{\mathrm{pin}}\) | All modules in unified `catalog.lock.json` digest | **119 / 119** (= 1) |
| **Hot-path static (~26%)** | Hand-aligned modules on topology gate path | **18 / 69** primary fiber ≈ **26.1%** — list in [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md) § “Hot path (18 modules)”; **not** \(U_{\mathrm{op}}\) |
| **Semantic audit “used (Y)”** | Manual Y rows in the per-module table below | **13 / 69** primary ≈ **18.8%**; **13 / 119** unified ≈ **10.9%** |
| **CI wired registry** | `CATALOG_MODULE_WIRED` slugs (offline PR target) | **25 / 119** ≈ **21.0%** have explicit `catalog_id` map rows |
| **Runtime gate slugs** | `GATE_REGISTRY_CATALOG_IDS` in `traceability.rs` | **5** evaluators: `cd_transition`, `http_shim`, `kleisli_unit`, `thermodynamic_mix`, `umst.cartridge.concrete.policy` |
| **Claims-rust slug** | CBF / gateway (`umst.gate.landauer_cbf`) | **1** slug — `src/ai/cbf.rs`, `src/ai/ppo.rs`; not in `GateEvaluator::catalog_id()` |
| **Operational \(U_{\mathrm{op}}(t)\)** | [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) §5 — modules with law exercised or labeled reject in \((t-T,t]\) | **Dynamic**; at \(t=0\) only slugs in `GATE_REGISTRY` + `landauer_cbf` paths count — **do not** substitute **26%** or **13 used** |
| **Adaptive priority (tests)** | [`WitnessPriorityQueue`](../src/runtime/catalog/witness_priority.rs) — rejects + `WitnessLearningSignal` | **Not hot path**; ranks Lean modules for next wiring (`tests/witness_priority_queue.rs`, `formal-witness`) |

**Anti-inflation:** Product copy may cite **~26%** only for the **intentional v1 hot-path scope** (18/69). Checklist / god-grade **~84%** and **\(U_{\mathrm{pin}}=1\)** are orthogonal — see [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md).

### Category theory (one paragraph)

Evidence windows form a category \(\mathbf{Ev}\) (objects = multisets of rejects, domain tags, MI aggregates; morphisms = refinement). Activation plans form \(\mathbf{Act}\) (objects = finite enable/wire/trace decisions; morphisms = subplan inclusion). The prioritization map \(\alpha : \mathbf{Ev} \to \mathbf{Act}\) is implemented in tests by [`WitnessPriorityQueue`](../src/runtime/catalog/witness_priority.rs) (`record_reject`, `apply_learning_signals`); it must not alter the fixed witness composite \(W_4 \circ W_3 \circ W_2 \circ W_1\). Full definitions: [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) §§6–8.

---

## Per-module table

| Lean module | `catalog_id` | used_in_rust | gate/runtime path | gap recommendation |
|-------------|--------------|--------------|-------------------|-------------------|
| Activation | — | N | — | Add `Engine` enum + activation witness if cement engines enter manifold |
| Complementarity | — | N | — | Optional sim bridge (`SimLeanBridge` contracts) |
| DataProcessingInequality | — | N | — | Formal-only; no Kraus DPI in Rust |
| DensityState | — | N | — | Formal-only density-matrix layer |
| DoubleSlit | `umst.gate.cd_transition` | partial | `src/gate/thermo_transition.rs` | Name which-path enforcement explicitly in gate tests |
| DoubleSlitCore | `umst.gate.landauer_cbf` | partial | `src/ai/cbf.rs` (via Landauer chain) | Document observation-state mapping |
| EpistemicDynamics | — | N | — | Rollout policies not in runtime |
| EpistemicGalois | — | N | — | Probe-budget Galois connection not exported |
| EpistemicMI | `umst.gate.landauer_cbf` | Y | `src/ai/ppo.rs`, `tests/gateway_info_gain.rs` | Emit `catalog_id` on gateway reject telemetry |
| EpistemicNumericsContract | — | N | — | Wire numeric trace records to telemetry schema |
| EpistemicPerStepNumerics | — | N | — | Per-step MI/cost fields not in Rust traces |
| EpistemicPolicy | — | N | — | Policy optimality not in host registry |
| EpistemicRuntimeContract | `umst.formal.catalog_lock` | partial | `src/ai/formal.rs`, `src/ai/ppo.rs` (`formal-witness`) | Implement trace MI/Landauer contracts, not digest-only |
| EpistemicRuntimeSchemaContract | — | partial | `src/ros/epistemic_trace.rs`, `tests/epistemic_trace_schema.rs` | Serde roundtrip (G.1); per-step `EmittedTraceWellFormed` bounds in CI (G.2 partial); rollout consistency / aggregate calibration open |
| EpistemicSensing | `umst.gate.landauer_cbf` | Y | `src/ai/ppo.rs`, `src/ai/info_gain.rs` | Tie probe strength to typed probe enum |
| EpistemicTelemetryApproximation | — | N | — | Approximation bounds not in runtime |
| EpistemicTelemetryBridge | — | N | — | ROS/telemetry schema bridge missing |
| EpistemicTelemetryQuantitativeUtility | — | N | — | Utility ε-bounds not checked at runtime |
| EpistemicTelemetrySolverCalibration | — | N | — | `SolverCalibration` not in Rust |
| EpistemicTraceDerivedEpsilonCertificate | — | N | — | Certificate type not exported |
| EpistemicTraceDrivenCalibrationWitness | — | N | — | Witness-at-trace not in CI |
| EpistemicTrajectoryMI | `umst.gate.landauer_cbf` | Y | `src/ai/ppo.rs` | Cumulative MI budget in gateway traces |
| ErasureChannel | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs`, `tests/cbf.rs` | Document reset-channel vs CBF erasure floor |
| ExamplesQubit | — | N | — | Golden ρ₊/ρ₀ examples for parity fixtures |
| FiberedActivation | — | N | — | Engine fiber not in Rust |
| FlashMoERuntimeScaffold | — | partial | — (imports `Gate` in Lean only) | Map to `umst.gate.http_shim` or drop from export |
| FormalFoundations | `umst.formal.catalog_lock` | partial | `build.rs`, `src/runtime/catalog/mod.rs` | Digest pin only; no completeness witness in Rust |
| Gate | `umst.gate.cd_transition` | Y | `src/gate/thermo_transition.rs`, `src/gate/evaluator.rs`, `tests/gate_cbf_parity.rs` | Add parity tests for `kleisliAdmissibility` |
| GateCompat | `umst.gate.cd_transition` | Y | `src/gate/mix_proposal.rs`, `tests/gate_parity_fixture.rs` | Calibrated hydration/strength bounds in mix filter |
| GeneralDimension | — | N | — | n-ary entropy bounds not used |
| GeneralResidualCoherence | — | N | — | Purity/coherence capacity not in CBF |
| GeneralVisibility | — | N | — | Fringe visibility not in tensor gate |
| InfoEntropy | — | N | — | Shannon diagonal bridge not in Rust |
| InformationCostIdentity | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs` | Residual-coherence vs bit budget identity tests |
| KleinInequality | — | N | — | Spectral relative entropy not in Rust |
| KroneckerEigen | — | N | — | Tensor entropy additivity not in Rust |
| LandauerBound | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs`, `src/gate/cbf_bridge.rs` | Register `umst.gate.landauer_cbf` on `GateEvaluator` |
| LandauerEinsteinBridge | — | N | — | SI mass-equivalent bounds not in host |
| LandauerExtension | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs` | n-bit / mono extension tests |
| LandauerLaw | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs` (TCB axiom) | Document `physicalSecondLaw` as TCB, not extracted |
| LindbladDynamics | — | N | — | Dephasing limit not in solver |
| LindbladStreamD | — | N | — | Stream-D limit not in runtime |
| LogSum | — | N | — | Mathlib helper only |
| MatrixLog | — | N | — | Empty export / placeholder |
| MeasurementChannel | — | N | — | Kraus/which-path map not in Rust |
| MeasurementCost | `umst.gate.landauer_cbf` | Y | `src/ai/cbf.rs` | Align `measurementCost` with CBF bit energy |
| MonoidalState | `umst.gate.cd_transition` | partial | `src/gate/thermo_transition.rs` | `combine_*` not implemented for host states |
| Naturality | `umst.gate.cd_transition` | partial | `src/manifest/umst_manifest.rs` (docs/registry) | Material-agnostic `stateFor` not typed |
| PMICEntropyInterior | — | N | — | PMIC calculus not in Rust |
| PMICVisibility | — | N | — | Visibility² bound not enforced |
| PhysicsConstrainedAI | `umst.gate.landauer_cbf` | Y | `src/ai/ppo.rs` | ≤1-bit gating test in `gateway_info_gain` |
| ProbeOptimization | `umst.gate.kleisli_unit` | partial | `src/gate/kleisli.rs`, `tests/gate_kleisli.rs` | Implement `GateEvaluator` for `umst.gate.kleisli_unit` |
| PrototypeSolverCalibration | — | N | — | Prototype ε witness not in Rust |
| QRBridge | `umst.gate.cd_transition` | N | — | No QR→ℝ bridge crate (claims: proved only) |
| QuantumClassicalBridge | — | N | — | Fringe/path observables not in UMST tensors |
| QuantumMutualInfo | — | N | — | QMI not in info_gain surrogate |
| SchrodingerDynamics | — | N | — | Unitary channel not in runtime |
| SimLeanBridge | — | N | — | `Sim*Witness` structs not in Rust FFI |
| TensorPartialTrace | — | N | — | Partial trace not in tensor ops |
| Test3 | — | N | — | Exclude from production catalog export |
| Test4 | — | N | — | Exclude from production catalog export |
| TestEntropy | — | N | — | Exclude from production catalog export |
| TestFixes | — | N | — | Exclude from production catalog export |
| TestMixed | — | N | — | Exclude from production catalog export |
| UMSTCore | `umst.gate.cd_transition` | Y | `src/gate/thermo_transition.rs` | Align `Admissible` with `ThermodynamicState` fields |
| VonNeumannEntropy | — | N | — | Spectral entropy not in CBF |
| WhichPathMeasurementUpdate | — | N | — | Collapse update not in gateway |
| lakefile | — | N | — | Build metadata; filter from `catalog.json` export |
| test_tensor_eigen | — | N | — | Lean test module; filter from export |

---

## `catalog_id` literals in audited trees

| `catalog_id` | Rust definition | Runtime-wired? |
|--------------|-----------------|----------------|
| `umst.gate.cd_transition` | `src/gate/evaluator.rs` | Yes (`ThermodynamicTransitionEvaluator`) |
| `umst.gate.http_shim` | `src/gate/http_manifest.rs` | Yes (`HttpMixGateEvaluator`) |
| `thermodynamic_mix` | `src/gate/mix_eval_registry.rs` | Yes (registry + `orchestrator.rs` match) |
| `umst.cartridge.concrete.policy` | `src/gate/concrete_cartridge.rs` | Yes (`GateEvaluator` only; not in claims table) |
| `umst.gate.landauer_cbf` | `src/ai/cbf.rs`, `src/ai/ppo.rs`, `src/gate/cbf_bridge.rs` | claims-rust (no `GateEvaluator`) |
| `umst.gate.kleisli_unit` | `src/gate/kleisli.rs` | claims-rust (no `GateEvaluator`) |
| `umst.formal.catalog_lock` | `src/runtime/catalog/mod.rs`, `build.rs` | digest-only (whole bundle) |
| `umst.gate.prediction_vs_physics` | *(deprecated)* | Superseded by `umst.gate.http_shim` + `gate_family` `mix_prediction_vs_physics` |

`src/runtime/catalog/mod.rs` exposes **lock digest** and **witness envelope** only — no per-module `catalog_id`.

---

## Bidirectional table (Lean ↔ `catalog_id` ↔ Rust ↔ tests)

Lean path prefix: `umst-formal-double-slit/Lean/{Module}.lean`

### A — Runtime-wired (`catalog_id` in `src/gate/`)

| Lean module | Lean path | `catalog_id` | Rust path | Test(s) |
|-------------|-----------|--------------|-----------|---------|
| `Gate` | `Lean/Gate.lean` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs`, `src/gate/evaluator.rs` | `gate_evaluator_catalog_surface_stable`, `gate_evaluator_wires_catalog_id`, `gate_evaluator_golden_*` (`tests/gate_parity_fixture.rs`); `gate_cbf_parity::gate_evaluator_wires_catalog_id` |
| `Gate` (Kleisli lemmas) | `Lean/Gate.lean` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs` | `kleisli_compose_preserves_admissibility_chain` (`tests/gate_kleisli.rs`) — **no `catalog_id` assert** |
| `UMSTCore` | `Lean/UMSTCore.lean` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | (shared with `Gate` golden vectors) |
| `Naturality` | `Lean/Naturality.lean` | `umst.gate.cd_transition` | `src/manifest/umst_manifest.rs` | — |
| `MonoidalState` | `Lean/MonoidalState.lean` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | — |
| `GateCompat` | `Lean/GateCompat.lean` | `umst.gate.cd_transition`, `thermodynamic_mix` | `src/gate/mix_proposal.rs`, `src/gate/mix_eval_registry.rs` | `mix_gate_evaluator_catalog_surface_stable`, `registry_routes_mix_evaluator`; `embodied_orchestrator` mix step |
| `DoubleSlit` | `Lean/DoubleSlit.lean` | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | — |
| `GateCompat` (HTTP / Powers) | `Lean/GateCompat.lean` | `umst.gate.http_shim` | `src/gate/http_manifest.rs`, `src/bin/gate_server.rs` | `post_gate_json_roundtrip_localhost` (`tests/gate_server_http.rs`, feature `gate-server-bin`) |
| *(prototype mix)* | — | `thermodynamic_mix` | `src/gate/mix_eval_registry.rs`, `src/manifest/orchestrator.rs` | `mix_gate_evaluator_*`, `registry_routes_mix_evaluator`, `embodied_orchestrator` |
| *(cartridge policy)* | — | `umst.cartridge.concrete.policy` | `src/gate/concrete_cartridge.rs` | — |

### B — Claims-Rust (`catalog_id` in spec/claims, not `GateEvaluator` in `gate/`)

| Lean module | Lean path | `catalog_id` | Rust path | Test(s) |
|-------------|-----------|--------------|-----------|---------|
| `LandauerBound` | `Lean/LandauerBound.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs`, `src/gate/cbf_bridge.rs` | `gate_cbf_delegates_verify_tensor_update` (`tests/gate_kleisli.rs`); `tests/golden_path_physics_cbf.rs` |
| `LandauerLaw` | `Lean/LandauerLaw.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | — |
| `LandauerExtension` | `Lean/LandauerExtension.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | — |
| `EpistemicMI` | `Lean/EpistemicMI.lean` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | gateway / topology tests |
| `EpistemicSensing` | `Lean/EpistemicSensing.lean` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | — |
| `EpistemicTrajectoryMI` | `Lean/EpistemicTrajectoryMI.lean` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | — |
| `MeasurementCost` | `Lean/MeasurementCost.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | — |
| `PhysicsConstrainedAI` | `Lean/PhysicsConstrainedAI.lean` | `umst.gate.landauer_cbf` | `src/ai/ppo.rs` | — |
| `InformationCostIdentity` | `Lean/InformationCostIdentity.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | — |
| `ErasureChannel` | `Lean/ErasureChannel.lean` | `umst.gate.landauer_cbf` | `src/ai/cbf.rs` | — |
| `ProbeOptimization` | `Lean/ProbeOptimization.lean` | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs`, `src/gate/mix_eval_registry.rs` | `kleisli_compose_preserves_admissibility_chain` |
| `QRBridge` | `Lean/QRBridge.lean` | `umst.gate.cd_transition` | — (no QR bridge) | — |
| `EpistemicRuntimeContract` | `Lean/EpistemicRuntimeContract.lean` | `umst.formal.catalog_lock` | `src/ai/formal.rs` (`formal-witness`) | `formal_witness_smoke_compiles` (`tests/formal_witness.rs`) |
| `FormalFoundations` | `Lean/FormalFoundations.lean` | `umst.formal.catalog_lock` | digest pin only | — |
| ROS telemetry | — | `umst.gate.cd_transition` | `src/ros/contract.rs` | `ros_contract_serde_roundtrip` |
| `umst-formal` `Gate.lean` | external repo | `umst.gate.cd_transition` | `src/gate/thermo_transition.rs` | — |
| `umst-formal` `DIBKleisli.lean` | external repo | `umst.gate.kleisli_unit` | `src/gate/kleisli.rs` | — |

### C — Digest-only (all **119** modules via unified lock)

| Lean scope | `catalog_id` | Rust path | Test(s) |
|------------|--------------|-----------|---------|
| Full export (**119** modules) | `umst.formal.catalog_lock` | `src/runtime/catalog/mod.rs`, `build.rs`, `artifacts/catalog.lock.json` | `embedded_witness_catalog_parses`, `bundled_lock_matches_build_digest_semantics`, `witness_quickcheck_reports_coherent_bundle`, `catalog_witness_quickcheck` |
| HTTP response hash | *(digest via lock)* | `src/gate/http_manifest.rs` (`catalog_hash_hex`) | `post_gate_json_roundtrip_localhost` checks 64-char hex |

### D — Catalog-only (no `catalog_id`)

**24 modules** — full list in `docs/claims-vs-proofs.md` Appendix A. Additional catalog-only rows from the main traceability table: `Activation`, `FiberedActivation`, `LandauerEinsteinBridge`, `Complementarity`, `WhichPathMeasurementUpdate`, `DensityState`, `MeasurementChannel`, `VonNeumannEntropy`, `KroneckerEigen`, `QuantumMutualInfo`, `DataProcessingInequality`, `KleinInequality`, `GeneralResidualCoherence`, `GeneralVisibility`, `PMICVisibility`, `EpistemicPolicy`, `EpistemicDynamics`, `EpistemicGalois`, epistemic telemetry/schema modules, `PrototypeSolverCalibration`, `EpistemicTraceDrivenCalibrationWitness`, `ExamplesQubit`, `SimLeanBridge`, `QuantumClassicalBridge`.

---

## Reverse check: `catalog_id` → Lean modules

| `catalog_id` | Lean modules | Wired in `gate/`? |
|--------------|--------------|-------------------|
| `umst.gate.cd_transition` | `Gate`, `UMSTCore`, `Naturality`, `MonoidalState`, `GateCompat`, `DoubleSlit`, `QRBridge` | Yes |
| `umst.gate.http_shim` | `GateCompat` (HTTP closure) | Yes |
| `thermodynamic_mix` | `GateCompat` / prototype mix filter | Yes |
| `umst.gate.landauer_cbf` | Landauer + epistemic MI stack | No (`ai/` only) |
| `umst.gate.kleisli_unit` | `Gate` (Kleisli), `ProbeOptimization`, `umst-formal` `DIBKleisli` | Partial (`kleisli.rs`, no `catalog_id()`) |
| `umst.formal.catalog_lock` | All **119** + `EpistemicRuntimeContract`, `FormalFoundations` | Digest only |
| `umst.cartridge.concrete.policy` | *(none — host cartridge defaults)* | Yes (undocumented in claims) |
| `umst.gate.prediction_vs_physics` | *(deprecated — use `umst.gate.http_shim`)* | Via `HttpMixGateEvaluator` `gate_family` only |

---

## Gaps / recommended follow-ups

1. Add `umst.cartridge.concrete.policy` to `claims-vs-proofs.md` or fold into `umst.gate.http_shim`.
2. Implement `GateEvaluator` for `umst.gate.kleisli_unit` or stop advertising it as a gate slug.
3. Namespace `thermodynamic_mix` → `umst.gate.thermodynamic_mix` per `GateUnificationSpec.md`.
4. ~~Either implement `umst.gate.prediction_vs_physics` as `catalog_id` or remove from spec.~~ **Resolved (2026-05-21):** deprecated; `umst.gate.http_shim` + `mix_prediction_vs_physics` `gate_family`.
5. Export hygiene: drop `Test*`, `lakefile`, `test_tensor_eigen`, empty `MatrixLog` from production catalog (see [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md)).
6. Extend `tests/formal_witness.rs` beyond compile smoke (see [`COMPOSITIONAL_INFERENCE_AUDIT.md`](COMPOSITIONAL_INFERENCE_AUDIT.md)).

---

## Related docs

| Doc | Role |
|-----|------|
| [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) | \(U_{\mathrm{op}}(t)\), `WitnessPriorityQueue`, vs static ~26% |
| [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) | Failure priority, MI surrogate, v1/v2 witnesses |
| [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) | Lean → catalog → manifold → cartridge |
| [`CATALOG_TRACEABILITY.md`](CATALOG_TRACEABILITY.md) | CI partition (`catalog_all_ids_registered`) |
| [`claims-vs-proofs.md`](claims-vs-proofs.md) | Lean ↔ `catalog_id` ↔ Rust ledger |
| [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md) | Exporter scope (69 vs 59) |
| [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md) | `umst-formal` vs double-slit (second fiber) |

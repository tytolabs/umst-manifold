# Formal integration status (plain English)

**As of:** 2026-05-21  
**Pipeline / drift / automation (companion):** [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md)  
**Witness ladder (god-grade order):** [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md)  
**Sources:** `umst-formal-double-slit/artifacts/catalog.json` (**119** Lean modules unified, 582+ theorem/lemma/axiom names on primary fiber export), `umst-manifold/docs/claims-vs-proofs.md`, `artifacts/catalog.lock.json`, Rust `src/` grep, `docs/END_CONDITION_REPORT.md`. Truth pass: [`TRUTH_AUDIT_LOG.md`](TRUTH_AUDIT_LOG.md).  
**Evidence appendices (on disk):** [`CATALOG_COVERAGE_AUDIT.md`](CATALOG_COVERAGE_AUDIT.md), [`COMPOSITIONAL_INFERENCE_AUDIT.md`](COMPOSITIONAL_INFERENCE_AUDIT.md), [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md), [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md), [`../umst-supercap-cartridge/docs/FORMAL_SCALING.md`](../umst-supercap-cartridge/docs/FORMAL_SCALING.md).

**Narrative:** Start with [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) (forward/backward pipeline), then this file (module buckets), then [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) (enforcement order). Executive rollup: [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md).

---

## Process & verification

**Progress date:** 2026-05-21 · **Lock:** `0697014f…` · **119 modules** (`cross_repo_merge: true`) · `catalog_all_ids_registered` **4/4**

| Metric | Value |
|--------|-------|
| Plan infra | **100%** ([`TODO_COMPLETION.md`](TODO_COMPLETION.md)) |
| God-grade (weighted) | **~84%** — R0–R4 + adversarial CI; strict default, W8 git pin, v2 traces open |
| Hot path | **18 / 69 (~26%)** hand-aligned on **primary** fiber; **119 / 119** digest-pinned |

### Learnings

- **Proofs as a versioned library** — All **119** modules affect the unified lock fingerprint; only **~26%** of the **primary** fiber (18/69) are **enforced** on the topology hot path. The remainder justify the proof graph and drift detection until wired or allowlisted.
- **Second-law TCB unchanged** — `physicalSecondLaw` (`LandauerLaw.lean`) is the sole project axiom in the primary export; Rust CD/Landauer code is TCB implementation, not a new axiom ([`TCB.md`](TCB.md)).
- **Gate law vs catalog inventory** — `catalog_id` slugs route rejects; `umst.gate.kleisli_unit` routes via `KleisliUnitEvaluator` on embodied host steps ([§ Proof library · gate law · MI](GOD_GRADE_WITNESS_LADDER.md#proof-library--gate-law--mi-envelope--no-rust-axioms)).

### Impact

- Reviewers see a single **module bucket** table (hot / catalog-only / support / test) without conflating digest coverage with runtime enforcement.
- `FormalReject` + `umst.gate.landauer_cbf` on CBF path ties telemetry to the Landauer witness family.
- Remaining automation gaps are **registry and product defaults**, not missing Lean export tooling.

> **Design lens** — Objects = thermodynamic states; morphisms = gated transitions; the export functor maps Lean modules to a **pinned fiber** consumed by Rust as law (witnesses), not as live proof terms.

---

## What “119 modules / 69 primary” means here

The build pins **119 Lean modules** in the unified formal catalog (dual-pin: **69** primary + **62** `umst-formal`, composed digest). That is not 119 separate runtime checks. Each module holds many small proved facts (582 theorem/lemma/axiom names on a primary-only export scan). At runtime, manifold mostly checks **hand-written Rust** aligned to **18** primary modules—not the Lean prover itself.

---

## Summary percentages (primary fiber — 69 modules)

| Bucket | Modules | Share | What it means in practice |
|--------|---------|-------|---------------------------|
| **Hot path** | 18 | **~26%** | Rust gate / AI barrier code runs on real topology steps and is traced to these Lean modules. |
| **Catalog-only (proved, not enforced)** | 26 | **~38%** | In Lean and in the catalog digest; no matching runtime gate. |
| **Support / dependency (catalog only)** | 18 | **~26%** | Lemmas other proofs import; still in the digest, not wired to Rust. |
| **Test + tooling entries in catalog** | 7 | **~10%** | Lean test modules, `lakefile`, eigen smoke—CI/formal hygiene, not production policy. |

**Build-time pin:** All **119** modules affect `artifacts/catalog.lock.json` → `UMST_CATALOG_LOCK_SHA256_HEX` in `build.rs`. Hot-path tables below use **primary-fiber** (69) buckets. That is a **fingerprint**, not proof that each module is enforced when the solver runs.

---

## Hot path (18 modules — ~26%)

These are the modules with a documented **hand-aligned** or **trusted (TCB)** link to running Rust:

| Lean module | Runtime role (short) |
|-------------|----------------------|
| `Gate`, `UMSTCore`, `Naturality`, `GateCompat`, `MonoidalState` | Host thermodynamic transition (`umst.gate.cd_transition`) |
| `DoubleSlit` | One gate-enforcement bridge into the same transition path |
| `LandauerBound`, `LandauerLaw`, `LandauerExtension` | Landauer / dissipation scalar barrier (`umst.gate.landauer_cbf` via `ThermodynamicCBF`) |
| `EpistemicMI`, `EpistemicSensing`, `EpistemicTrajectoryMI` | Topology-step gateway accounting in `ManifoldGateway` |
| `MeasurementCost`, `PhysicsConstrainedAI`, `InformationCostIdentity`, `ErasureChannel` | CBF / cost identities on the AI path |
| `ProbeOptimization` | Kleisli admissibility + mix registry (spec id `umst.gate.kleisli_unit`) |
| `EpistemicRuntimeContract` | Optional digest witness (`formal-witness` feature) |

**Active `catalog_id` strings in Rust today:** `umst.gate.cd_transition`, `thermodynamic_mix`, `umst.gate.http_shim`, `umst.cartridge.concrete.policy`, `umst.gate.landauer_cbf` on CBF reject via `FormalReject` (plus ROS ack schema `umst.gate_ack.v1`). **`umst.gate.kleisli_unit`** — [`KleisliUnitEvaluator`](../src/gate/kleisli.rs) implements [`GateEvaluator`](../src/gate/evaluator.rs); [`EmbodiedOrchestrator::check_host_transition`](../src/manifest/orchestrator.rs) routes R4 after R1–R3 (registry default includes η unit).

**End-condition smoke (2026-05-21):** Gate dual-run parity 8/8, formal witness, ROS contract round-trip, HTTP gate server—documented in `END_CONDITION_REPORT.md`.

---

## Catalog-only (~44 modules — ~64%)

### Proved in Lean, no runtime mirror (26 modules — ~38%)

Examples: full double-slit / density-matrix stack (`DensityState`, `VonNeumannEntropy`, `WhichPathMeasurementUpdate`, `DataProcessingInequality`, …), activation engine lemmas (`Activation`, `FiberedActivation`), complementarity, QR bridge, formal completion flag (`FormalFoundations`), simulation witnesses (`SimLeanBridge`, `ExamplesQubit`), and calibration witnesses without manifold hooks.

### Support lemmas in the export (18 modules — ~26%)

Examples: `SchrodingerDynamics`, `TensorPartialTrace`, `QuantumClassicalBridge`, `InfoEntropy`, Lindblad scaffolding, epistemic telemetry subcontracts, `DoubleSlitCore`. They justify the proof graph and the digest; they do not appear on the gate hot path.

### Test / infra rows (7 modules — ~10%)

`Test3`, `Test4`, `TestEntropy`, `TestFixes`, `TestMixed`, `lakefile`, `test_tensor_eigen`.

---

## Appendix B traceability (unified export)

`claims-vs-proofs.md` Appendix B still narrates hand-aligned rows for **`umst-formal`** (cement `Gate.lean`, `DIBKleisli.lean`, `DEC.lean`, etc.). Those modules are in the **119**-module unified export since `formal-fiber-merge` ✅; Appendix B is ledger narrative, not a second production pin.

---

## Plan todo vs god-grade criteria

Synthesized from [`TODO_COMPLETION.md`](TODO_COMPLETION.md) and [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (2026-05-21).

| Plan todo | God-grade criterion | Verdict | Notes |
|-----------|---------------------|---------|-------|
| `repo-layout-ssot` | Composition layers documented | ✅ | `REPO_LAYOUT_SSOT.md` |
| `prototype-audit` | Parity fixture list | ✅ | `PROTOTYPE_GATE_MAP.md` |
| `gate-unification-spec` | `catalog_id` registry SSOT | ✅ | `GateUnificationSpec.md` |
| `manifold-runtime-catalog` | Lock + witness embed | ✅ | `runtime/catalog/`, `build.rs` |
| `manifold-gate-evaluator` | Host gates + parity tests | ✅ | `src/gate/` |
| `formal-witness-integration` | Structured reject + digest | ✅ | `FormalReject`; feature off by default |
| `manifold-manifest` | Grounding + orchestrator | ✅ | `src/manifest/` |
| `ros2-in-manifold` | Wire contracts, no ROS runtime | ✅ | `ros2-contract` feature |
| `embodied-orchestrator` | Layer-6 composition | ✅ | `EmbodiedOrchestrator` |
| `claims-vs-proofs` | Lean ↔ `catalog_id` ledger | ✅ | 37+ rows |
| *(new)* `catalog_all_ids_registered` | **119**-module partition CI | ✅ | `tests/catalog_all_ids_registered.rs` — **4/4** (Lean partition, gate registry, spec↔constants, wired↔spec/witness); spec parser strips `` `catalog_id` `` backticks |
| *(new)* CBF `catalog_id` on reject | Telemetry slug on hot path | ✅ | `FormalReject::ThermodynamicControlBarrier { catalog_id: umst.gate.landauer_cbf }` in `ppo.rs`; unit test `ai::formal::tests::cbf_reject_carries_landauer_catalog_id` |
| `lean-export-lake` | Export canonical in CI | ⚠️ | Python export; not `lake exe` |
| `concrete-cartridge-wire` | Cartridge consumes manifest | ⚠️ | Git pin; optional features |
| `parity-ci` | Full gate matrix in verify | ⚠️ | Drift workflow runs `gate_adversarial` (Rust golden FNR=0); prototype Python E6 still optional in `verify_umst_stack.sh` |
| `thin-prototypes` | Delete duplicate gate math | ⚠️ | Filter bodies remain |
| Kleisli `GateEvaluator` | Registry routes `kleisli_unit` | ✅ | `KleisliUnitEvaluator` + embodied host routing; `gate_kleisli` / `embodied_orchestrator` tests |
| Strict catalog match default | Production god-grade | ❌ | Advisory mode still allowed |
| Catalog / witness attestation (long horizon) | Extracted witnesses beyond digest pin | ❌ | Long horizon — **not** a physics-engine merge |

**Automation score (plan items):** 12 ✅ · 4 ⚠️ · 2 ❌ (of 18 tracked rows) → **67%** strict ✅-only · **~72%** weighted (½ credit on ⚠️ rows + R0–R4 witness closure).

---

## What “god-grade” automation still needs

“God-grade” here means: **automation that can reject bad states without relying on human parity reviews**, with the formal catalog as the single source of truth.

| Gap | Why it matters |
|-----|----------------|
| **No Lean → Rust extraction** | Proofs live in Lean; runtime uses hand-aligned `f64` checks. Drift is possible until parity tests or extractors exist. |
| **`formal-witness` off by default** | Catalog digest mismatch only fails when the feature is enabled in CI/product. |
| **`AdvisoryCatalogOnly` vs `StrictCatalogMatch`** | Manifest enum allows advisory mode; production god-grade needs strict hash match on every proposal. |
| **QR / quantum bridges absent in Rust** | `QRBridge` and most qubit modules are catalog-only. |
| **W8 `manifest-bridge` blocked on git pin** | Cartridge cannot consume published manifest API until `tytolabs/umst-manifold` `main` catches up (`AGENT_STATUS.md`). |
| **W10 CI split** | `rust.yml` optional `verify-umst-stack-optional` (subset or full `verify_umst_stack.sh`); drift workflow remains SSOT for export. |
| **Lean churn → lock promotion** | Manual `make lean-catalog-export` + `upstream_catalog_digest_hex` update; no bot yet. |
| **Optional: extracted witnesses (long horizon)** | Beyond R0 digest / `formal-witness` attestation; not on the inference hot path today. |

**Minimum automation ladder (practical order):**

1. CI always runs: `cargo test` gate parity + `formal-witness` + catalog drift (`VERIFY.md` §2.2).  
2. Bot or checklist on every Lean PR: re-export catalog → bump `catalog.lock.json` → run manifold tests.  
3. Implement `GateEvaluator` for `umst.gate.kleisli_unit`; register in `mix_eval_registry`.  
4. Turn on `StrictCatalogMatch` + `formal-witness` for release manifests.  
5. Longer term: extracted witnesses or FFI for high-value lemmas (gate soundness, Landauer bound)—not required for current end-condition PASS, required for full formal–runtime equivalence.

---

## Doc update checklist

When formal integration moves forward, update these in one pass:

| Document | Action |
|----------|--------|
| **`FORMAL_INTEGRATION_STATUS.md` (this file)** | Refresh module counts, hot-path table, and percentages after catalog re-export. |
| **`claims-vs-proofs.md`** | Add/remove rows for any new Lean↔Rust mapping; fix `kleisli_unit` / `thermodynamic_mix` spec gaps. |
| **`artifacts/catalog.lock.json`** | Set `upstream_catalog_digest_hex` + `module_count` after `make lean-catalog-export`. |
| **`VERIFY.md`** | Catalog digest line; gate test commands if new features land. |
| **`AGENT_STATUS.md` / `PARALLEL_HANDOFFS.md`** | Close W8/W10 items; note CI ownership. |
| **`GateUnificationSpec.md`** | New `catalog_id` rows; evaluator registry behavior. |
| **`GOD_GRADE_CHECKLIST.md`** | Composition layers, performance budget, CI matrix, criteria ticks. |
| **`TCB.md`** | If trust boundary moves (e.g. FFI prover, new axiom in Rust). |
| **`END_CONDITION_REPORT.md`** | Re-run matrix; attach date and PASS/FAIL. |
| **`PROOF-STATUS.md` / `Solver-Status.md`** | Only if solver claims cite new formal witnesses. |
| **`PROTOTYPE_GATE_MAP.md`** | If prototype paths change vs manifold SSOT. |
| **`REPO_LAYOUT_SSOT.md`** | New crates (`qr-bridge`, extract lib, etc.). |

| **`CATALOG_COVERAGE_AUDIT.md`** | Per-module `catalog_id` ↔ Rust wiring matrix. |
| **`COMPOSITIONAL_INFERENCE_AUDIT.md`** | PPO / gateway / orchestrator layer stack. |
| **`GOD_GRADE_WITNESS_LADDER.md`** | Witness rung order, failure priority, v1/v2 trace contracts. |
| **`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`** | Exporter scope (69 vs 59 roots). |
| **`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`** | `umst-formal` vs double-slit fiber policy. |
| **`../umst-supercap-cartridge/docs/FORMAL_SCALING.md`** | Supercap cartridge manifest / catalog pin scaling. |

---

## Quick reference

| Item | Value |
|------|--------|
| Catalog digest (lock) | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` |
| Modules in lock | **119** (`cross_repo_merge: true`) |
| Primary-only rollback | `c1d9ba2…` / **69** — [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) |
| Theorem / lemma / axiom names in export | 582 |
| Hot-path modules | 18 (~26%) |
| Catalog-only + support + test/infra | 51 (~74%) |
| Re-export command | `make lean-catalog-export` in `umst-formal-double-slit` |

For commands and feature flags, see [`VERIFY.md`](VERIFY.md). For row-level Lean↔Rust mapping, see [`claims-vs-proofs.md`](claims-vs-proofs.md).

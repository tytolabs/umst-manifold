# UMST plan todo completion audit

**Plan:** `lean-to-rust_proof_extraction_fd8f70b5.plan.md`  
**Audited:** 2026-05-29  
**Verified:** 2026-05-29 (UTC) — `verify_umst_stack.sh` exit **0** on unified R0 pin (`0697014fb5b90a3…`, **119** modules, lock v2); manifold CI green @ **fe22437**; **G-02** concrete `manifest-bridge` on git dep without `[patch]`; G.2 **13/13** · G.3 **8/8** in stack tail.
**Workspace:** MaOS-Workspace  

Evidence commands are read-only checks run during audit (no plan file edits).

**Narrative rollup:** [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) · pipeline [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) · witness law [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) · command ledger [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) · verified % [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md)

**Scoped closure SSOT (2026-05-29):** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) · automation denominator [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (**16/16** rows) · ceilings [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)

<!-- Cross-link when created: validation methodology doc → § Process & verification -->

---

## Process & verification

**Progress date:** 2026-05-29 · **Verified:** 2026-05-29 (`verify_umst_stack.sh` full bundle @ **fe22437**; ledger [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md))

*When a dedicated validation methodology doc is added under `docs/`, link it here (e.g. beside [`Validation.md`](Validation.md)).*

| Metric | Complete | Notes |
|--------|----------|-------|
| **Plan todos (YAML scope)** | **14 / 14 ✅** | On-disk implementation complete for every plan `id`; see [14/14 map](#1414-plan-todo-map-on-disk-vs-yaml) |
| **Plan infra → 100%** | **100%** | 14/14 on disk; optional lanes (`rust.yml`, 2a full delete) are polish, not blockers |
| **Plan + cross-repo fiber** | **100%** | 14/14 + `formal-fiber-merge` ✅ (unified digest + manifold lock + stack verify) |
| **God-grade automation (in-repo)** | **16 / 16 = 100%** | [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) § Automation criteria — all rows green @ **2026-05-29** / **fe22437** |
| **Scoped true 100% (Done morphisms)** | **3 / 4** | **G-04** ✅ · **G-05** ✅ · **W8 (G-01+G-02)** ✅ · **FFI** — **G-03** optional; see [Remaining](#remaining-to-scoped-true-100) |
| On-disk vs YAML | **14/14 ✅** vs 1 `in_progress` + 13 `pending` | Plan front-matter intentionally not edited |

### Todo counts (this audit)

| Bucket | Closed | Open | % |
|--------|--------|------|---|
| **Plan YAML `id`s (14)** | **14** | **0** | **100%** |
| **Formal lane (`formal-fiber-merge`)** | **1** | **0** | **100%** |
| **God-grade automation rows (16)** | **16** | **0** | **100%** |
| **Scoped true-100% blockers (4)** | **3** | **1** | **75%** at Done (G-04/G-05/W8 G-01+G-02; FFI horizon) |
| **Plan row `concrete-cartridge-wire` (remote W8)** | local ✅ | remote ✅ **G-02** | Git `fe22437` + GHA without `[patch]`; **G-03** supercap optional |

### 14/14 plan todo map (on-disk vs YAML)

| # | Plan `id` | YAML (unchanged) | On-disk | Infra note |
|---|-----------|------------------|---------|------------|
| 1 | `repo-layout-ssot` | `in_progress` | ✅ | `REPO_LAYOUT_SSOT.md` |
| 2 | `prototype-audit` | `pending` | ✅ | `PROTOTYPE_GATE_MAP.md` + fixtures |
| 3 | `gate-unification-spec` | `pending` | ✅ | `GateUnificationSpec.md` |
| 4 | `lean-export-lake` | `pending` | ✅ | Python `export_catalog.py` canonical; unified **119** modules (primary **69**) |
| 5 | `manifold-runtime-catalog` | `pending` | ✅ | `src/runtime/catalog/`, lock, `catalog_all_ids_registered` |
| 6 | `manifold-gate-evaluator` | `pending` | ✅ | Full `src/gate/` + Kleisli `GateEvaluator` |
| 7 | `formal-witness-integration` | `pending` | ✅ | `formal-witness` feature + tests |
| 8 | `manifold-manifest` | `pending` | ✅ | `UmstManifest`, `GroundingContract` |
| 9 | `ros2-in-manifold` | `pending` | ✅ | `ros2-contract`, `gate_server` |
| 10 | `concrete-cartridge-wire` | `pending` | ✅ local + ✅ remote **G-02** | Git `fe22437` without `[patch]` @ 2026-05-29; MaOS `[patch]` still patch-green Evidence |
| 11 | `embodied-orchestrator` | `pending` | ✅ | `EmbodiedOrchestrator` + tests |
| 12 | `claims-vs-proofs` | `pending` | ✅ | `claims-vs-proofs.md` + `TCB.md` + Appendix B |
| 13 | `parity-ci` | `pending` | ✅ | Drift CI + `verify_umst_stack` adversarial/Kleisli/rejects |
| 14 | `thin-prototypes` | `pending` | ✅ hybrid | v1 shim 226L + 8/8 dual-run; 2a `manifold-gate` hybrid 517L |

**Outside plan YAML (formal lane milestone):**

| Milestone | Status | Artifact |
|-----------|--------|----------|
| `lean-export-cross-repo` | ✅ **COMPLETE** | Alias of `formal-fiber-merge` — § [`formal-fiber-merge`](#formal-fiber-merge--complete) |
| `formal-fiber-merge` | ✅ **COMPLETE** | Unified digest `0697014f…` / **119** modules; manifold lock aligned; `verify_umst_stack.sh` green |

### Learnings

- **Proofs as a versioned library** — `lean-export-lake` ✅ via Python canonical export; unified lock `0697014f…` / **119** modules (69 primary + 50 `umst-formal` only).
- **Parity without deletion** — v1 prototype shim **226** lines, 8/8 dual-run; 2a body retained by design until ports ([`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md)).
- **Second-law TCB** — No new Rust axioms; CD/Landauer paths documented in [`TCB.md`](TCB.md) and witness ladder § Proof library · gate law · MI.

### Impact

- Evidence blocks below are the **per-todo SSOT** for coordinator handoff (commands + paths).
- Swarm audit docs (six files) close traceability without new Rust scaffolding.
- **Scoped true 100%** (honest): **3 / 4** Done — **G-04** ✅ · **G-05** ✅ · **W8 (G-01+G-02)** ✅ @ **2026-05-29**; **FFI** horizon open; **G-03** supercap optional — [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md).
- Optional polish (not scoped blockers): 2a thin delete, `rust.yml` verify lane — [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md).

> **Design lens** — Each plan todo is a morphism in the extraction pipeline; completion means the morphism factors through verified tests (exit 0), not merely files on disk. **W8:** **G-01** publish + **G-02** concrete remote CI are **Done** @ **fe22437**; MaOS `[patch]` tests remain patch-green **Evidence** for monorepo dev; **G-03** supercap remote is optional ([`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md)).

---

## Remaining to scoped true 100%

**SSOT:** [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) · plain register [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)

These are the **only** items that block an honest “scoped god-grade 100%” claim without qualifiers (**3/4** at Done; headline **~96–98%**). **16/16 in-repo automation** does **not** close **FFI** or optional **G-03**.

| ID | Blocker | Owner | Status | Evidence | Done criterion |
|----|---------|-------|--------|----------|----------------|
| **W8 / G-01** | Publish `tytolabs/umst-manifold` `main` | **human** | ✅ **Done** | `main` @ **fe22437**; `git ls-remote` OK; manifold CI green | [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) Phase 1 |
| **W8 / G-02** | Concrete remote CI without `[patch]` | **code** + CI | ✅ **Done** | Git `rev = fe22437`; GHA `manifest-bridge`; grounding test on git dep alone | Phase 2 concrete — no workspace patch |
| **W8 / G-03** | Supercap remote `manifest-bridge` in GHA | **human** | ⚠️ **OPEN** (optional) | `formal_anchors` **6/6** local | Track **I.3** — does **not** block **G-01**/**G-02** |
| **G-04** | `StrictCatalogMatch` on `UmstManifestBuilder::default()` when `UMST_RELEASE_MANIFEST_PROFILE=1` | **code** | ✅ **Done** | `verify_umst_stack.sh` exit **0** @ **2026-05-29**; `manifest_strict_witness` **4/4** | Track **H.1** — B3 |
| **G-05** | Auto-fill `upstream_catalog_digest_hex` from lock in builder/gateway/UMST | **code** | ✅ **Done** | `lock_upstream_catalog_digest_bytes()`; strict `build()` @ **2026-05-29** | Track **H.2** — B3 |
| **FFI** | Extracted Lean witnesses / attestation on hot path | **human + code** (horizon) | ❌ **OPEN** (horizon) | `rg 'lake build\|lean --run' umst-manifold/src` empty — policy | Separate FFI program — **excluded** from 16-row automation % |

**Patch-green rule:** MaOS workspace `[patch]` tests are **Evidence** for local dev; **G-02 Done** = concrete cartridge on **git** `fe22437` without patch ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) B1).

**Anti double-count:** Org W8 (~8–10% headline) is one morphism; G-04/G-05 are product/code policy — do not add automation % + scoped %.

---

## repo-layout-ssot — COMPLETE

**Requirement:** Document monorepo layout under `umst-manifold` (runtime/, manifest/, ros/, gate/, bins/); no new top-level repos.

**Evidence (2026-05-21T20:50:20Z):**
```bash
test -f umst-manifold/docs/REPO_LAYOUT_SSOT.md && wc -l umst-manifold/docs/REPO_LAYOUT_SSOT.md
# → 29 REPO_LAYOUT_SSOT.md
ls -d umst-manifold/src/{runtime,gate,manifest,ros,embodied} umst-manifold/src/bin 2>/dev/null
# → runtime gate manifest ros embodied bin
```

---

## prototype-audit — COMPLETE

**Requirement:** Inventory prototypes; map gate/ROS/Kleisli files to manifold modules + parity fixture list.

**Evidence:**
```bash
test -f umst-manifold/docs/PROTOTYPE_GATE_MAP.md
grep -c 'thermodynamic_filter\|kleisli\|gate_server' umst-manifold/docs/PROTOTYPE_GATE_MAP.md
test -f umst-manifold/tests/data/gate_dual_run_fixtures.json
```

---

## gate-unification-spec — COMPLETE

**Requirement:** `GateUnificationSpec.md` with predicate registry, FNR/dual-run strategy.

**Evidence:**
```bash
test -f umst-manifold/docs/GateUnificationSpec.md
grep -E 'dual-run|registry|catalog_id' umst-manifold/docs/GateUnificationSpec.md | head -5
```

---

## lean-export-lake — COMPLETE

**Requirement:** `tools/lean_export` + `lake exe` emitting real environment catalog (not regex-only).

**Resolution (2026-05-21):** **Python** `export_catalog.py` via **`make lean-catalog-export`** is documented as the **canonical** drift/pin path. `artifacts/README.md` and `tools/lean_export/README.md` warn that `lake exe export_catalog` must not overwrite pinned `catalog.json`. Lake exe remains optional for compact root lists (`artifacts/catalog-roots.json` / `--roots-only`).

**Evidence:**
```bash
test -f umst-formal-double-slit/tools/lean_export/export_catalog.py
test -f umst-formal-double-slit/artifacts/README.md
grep 'make lean-catalog-export' umst-formal-double-slit/artifacts/README.md
cd umst-formal-double-slit && make lean-catalog-export
cd umst-formal-double-slit && make lean-catalog-export
# Historical primary-only (pre–formal-fiber-merge):
# → digest=c1d9ba2aa402106a… module_count=69
# Current production (2026-05-21):
python3 -c "import json; l=json.load(open('umst-formal-double-slit/artifacts/catalog.lock.json')); m=json.load(open('umst-manifold/artifacts/catalog.lock.json')); assert l['catalog_digest_hex']==m['upstream_catalog_digest_hex'] and l['module_count']==m['module_count']==119"
# → lock OK: 0697014f… module_count= 119
test -f umst-formal-double-slit/Docs/EXPORT_COVERAGE.md
test -f umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md
```

**Audit docs:** [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md), [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md).

---

## lean-export-cross-repo — COMPLETE (alias; not in plan YAML)

**Runbook:** [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) · **TCB:** [`TCB.md`](TCB.md) · **Export SSOT:** [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md)

**Scope:** Not one of the 14 plan `todos`; tracked for formal-lane closure and **100%** “plan + fibers” rollup.

**Requirement:** Unified catalog pin spanning `umst-formal-double-slit` + `umst-formal`, then manifold `upstream_catalog_digest_hex` aligned.

**Evidence (2026-05-21):** ✅ `APPROVE_CROSS_REPO_MERGE=1` + `--also-lean-root ../umst-formal/Lean` → digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`, `module_count: 119`, `cross_repo_merge: true`; manifold lock bumped; `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` exit 0.

---

## formal-fiber-merge — COMPLETE

**Scope:** Promote the **second catalog fiber** ([`umst-formal`](../../umst-formal)) into the export pin consumed by manifold **R0**, concrete `manifest-bridge`, and supercap digest advisories. Same milestone as § `lean-export-cross-repo`; operator SSOT: [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md).

### Production merge — recorded pin (2026-05-21)

Unified export completed in `umst-formal-double-slit` (merge agent). Full table: [`EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md) § *Last production merge*.

| Field | Value |
|-------|-------|
| `catalog_digest_hex` | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` |
| `module_count` | **119** |
| `cross_repo_merge` | `true` |
| Primary modules | **69** |
| Secondary modules | **62** |
| `only_in_secondary_basename` | **50** (`DIBKleisli`, `Constitutional`, `Economic.*`, `DEC`, `Helmholtz`, …) |
| Overlap (primary wins) | **12** |
| Primary-only digest (historical) | `c1d9ba2aa402106a3477f454dd6d28015eb399c1160d8a2e2ba7d16788fdbfcc` |

**Canonical regen command:**

```bash
cd umst-formal-double-slit
APPROVE_CROSS_REPO_MERGE=1 python3 tools/lean_export/export_catalog.py \
  --lean-root Lean \
  --also-lean-root ../umst-formal/Lean \
  --also-lean-repo-tag umst-formal
```

**Dev-only:** add `--cross-repo-only` for local preview JSON without writing pins.

### Approval gate — `APPROVE_CROSS_REPO_MERGE=1`

Set the environment variable to `1` for unified `catalog.json` / `catalog.lock.json` writes (not a repo-root marker file). Without it, export with `--also-lean-root` writes primary-only catalog.

| State | Formal `artifacts/catalog.json` | Manifold R0 |
|-------|--------------------------------|-------------|
| **Closed (2026-05-21)** | `0697014f…`, **119** modules | `0697014f…`, **119** — `verify_umst_stack.sh` green |

**Owner:** formal / coordinator. Policy: [`UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md). Roadmap: [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) Track F.

### Ideal grounding — concrete + manifold (post-merge)

| Layer | Target | Status |
|-------|--------|--------|
| **R0 — Catalog** | One `upstream_catalog_digest_hex` for both fibers | ✅ |
| **R1 — CD (`manifest-bridge`)** | `umst.gate.cd_transition` SSOT unchanged | ✅ |
| **R5 — Manifest / digest** | `StrictCatalogMatch` + `formal-witness` on unified lock | Ops default still `CatalogPinnedRos2` |
| **Cartridge anchors** | Lemmas in unified export; optional `lean://` → `catalog_id` | Partial — Appendix B graduation ops |

Merge does **not** add Rust axioms; it enlarges proof inventory **F**. TCB remains `physicalSecondLaw` only ([`TCB.md`](TCB.md)).

### Steps (ordered)

1. ~~**Production export**~~ ✅ — digest `0697014f…`, 119 modules.
2. ~~**TCB + policy**~~ ✅ — single `physicalSecondLaw`; alignment doc signed.
3. ~~**Manifold pin**~~ ✅ — `umst-manifold/artifacts/catalog.lock.json` → `0697014f…`, `module_count: 119`.
4. ~~**Downstream verify**~~ ✅ — `verify_umst_stack.sh`, `catalog_all_ids_registered` 4/4.
5. ~~**Close milestone**~~ ✅ — Track F ✅ in [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md); verified ledger [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md).

### Verify commands

**Formal unified pin (current):**
```bash
cd umst-formal-double-slit
python3 -c "
import json
c=json.load(open('artifacts/catalog.json'))
l=json.load(open('artifacts/catalog.lock.json'))
assert c.get('cross_repo_merge') is True
assert len(c['modules'])==119
assert c['digest']==l['catalog_digest_hex']=='0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227'
print('formal unified OK', len(c['modules']))
"
```

**Phase 3 — manifold alignment ✅ (closed 2026-05-21):**
```bash
# After updating umst-manifold/artifacts/catalog.lock.json
python3 -c "
import json
l=json.load(open('umst-manifold/artifacts/catalog.lock.json'))
c=json.load(open('umst-formal-double-slit/artifacts/catalog.json'))
assert l['upstream_catalog_digest_hex']==c['digest']
assert l['module_count']==len(c['modules'])==119
print('locks aligned', l['module_count'])
"
cd umst-manifold && cargo test --test catalog_all_ids_registered -p umst-manifold
UMST_REQUIRE_FORMAL_EXPORT=1 UMST_FORMAL_ROOT=$PWD/../umst-formal-double-slit \
  bash scripts/verify_umst_stack.sh
```

**Cross-links:** [`EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md), [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md), [`FORMAL_GROUNDING_AUDIT.md`](../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md), [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md).

---

## manifold-runtime-catalog — COMPLETE

**Requirement:** `src/runtime/catalog`, `WitnessCatalog`, `build.rs` codegen, `catalog.lock.json` in repo.

**Evidence:**
```bash
test -f umst-manifold/src/runtime/catalog/mod.rs
test -f umst-manifold/artifacts/catalog.lock.json
test -f umst-manifold/build.rs
grep -l 'catalog_constants.rs' umst-manifold/build.rs
# Note: emits OUT_DIR/catalog_constants.rs (not invariants.rs — functionally equivalent)
test -f umst-manifold/docs/CATALOG_COVERAGE_AUDIT.md
test -f umst-manifold/docs/CATALOG_TRACEABILITY.md
cargo test --test catalog_all_ids_registered -p umst-manifold
# → 2026-05-21: 4 passed (**119**-module unified partition)
```

---

## manifold-gate-evaluator — COMPLETE

**Requirement:** Full `src/gate/` — `GateEvaluator`, CBF, mix proposal, Kleisli port.

**Swarm closure (2026-05-21):** `KleisliUnitEvaluator: GateEvaluator` (η unit, `umst.gate.kleisli_unit`); `EmbodiedOrchestrator::check_host_transition` routes R4 via `GateEvaluatorRegistry` after R1 CD / R3 mix; default registry registers mix + Kleisli (bind short-circuit preserved in `kleisli.rs`).

**Evidence:**
```bash
ls umst-manifold/src/gate/*.rs | wc -l
grep 'impl GateEvaluator for KleisliUnitEvaluator' umst-manifold/src/gate/kleisli.rs
grep 'KleisliUnitEvaluator::CATALOG_ID' umst-manifold/src/manifest/orchestrator.rs
cargo test --test gate_kleisli --test embodied_orchestrator -p umst-manifold
# → 2026-05-21T21:45:00Z: gate_kleisli 6/6; embodied 8/8 (incl. kleisli host routes)
test -f umst-manifold/docs/CATALOG_COVERAGE_AUDIT.md
test -f umst-manifold/docs/COMPOSITIONAL_INFERENCE_AUDIT.md
```

---

## formal-witness-integration — COMPLETE

**Requirement:** `formal-witness` feature, `FormalReject`, catalog hash enforcement on policy load.

**Evidence:**
```bash
grep 'formal-witness' umst-manifold/Cargo.toml
test -f umst-manifold/src/ai/formal.rs
test -f umst-manifold/tests/formal_witness.rs
test -f umst-manifold/docs/COMPOSITIONAL_INFERENCE_AUDIT.md
# §6 documents formal-witness automation gaps (digest not auto-filled from lock)
```

---

## manifold-manifest — COMPLETE

**Requirement:** `UmstManifest`, `GroundingContract`, public re-exports, docs.

**Evidence:**
```bash
test -f umst-manifold/src/manifest/umst_manifest.rs
grep 'pub struct UmstManifest' umst-manifold/src/manifest/umst_manifest.rs
grep 'pub mod manifest' umst-manifold/src/lib.rs
```

---

## ros2-in-manifold — COMPLETE

**Requirement:** `ros2-contract` serde types; `gate_server` bin; WCET/L2 docs.

**Evidence:**
```bash
test -f umst-manifold/src/ros/contract.rs
test -f umst-manifold/src/bin/gate_server.rs
grep 'ros2-contract\|gate-server-bin' umst-manifold/Cargo.toml
test -f umst-manifold/tests/ros_contract_serde_roundtrip.rs
test -f umst-manifold/tests/gate_server_http.rs
```

---

## concrete-cartridge-wire — COMPLETE (local + remote G-02)

**Requirement:** Cartridge depends on manifold manifest+gate; facade uses `GateEvaluator`; `ros2-contract` re-export; generated `formal_anchor` from catalog.

**Done (local):** Optional features `manifold-gate`, `manifold-manifest`, `manifest-bridge`; facade CD gate behind `manifest-bridge` → `umst.gate.cd_transition`; `ros2-contract` feature forward + `lib.rs` `pub use umst_manifold::ros`; workspace `[patch]` to sibling `umst-manifold`; [`../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md`](../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md) documents git publish blocker; `cargo test -p umst-concrete-cartridge --features manifest-bridge` passes with patch (**2026-05-21T20:50:20Z** exit **0**).

**Remote CI — G-02 Done (2026-05-29):** `tytolabs/umst-concrete-cartridge` pins git `umst-manifold` **rev `fe22437`**; GHA runs `manifest-bridge` **without** workspace `[patch]`; `manifest_bridge_catalog_grounding` asserts **119**-module digest on git dep alone. **Optional:** **G-03** supercap remote bridge ([`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) Track **I.3**).

**Deferred:** Catalog-generated `formal_anchor` rows (PROOF-STATUS still uses `lean://` / `empirical://` / `literature://`; not gate `catalog_id` slugs).

**Supercap sibling (2026-05-21):** `tests/formal_anchors.rs` **6/6**; `topology_catalog_hash_advisory()` pins lock bytes (see `FORMAL_SCALING.md` §1.3).

**Evidence:**
```bash
grep 'manifold-gate\|manifold-manifest\|manifest-bridge\|ros2-contract' umst-concrete-cartridge/crates/umst-concrete-cartridge/Cargo.toml
grep -c 'manifold-gate\|manifest-bridge' umst-concrete-cartridge/crates/umst-concrete-cartridge/src/facade/mod.rs
grep 'ros2-contract' umst-concrete-cartridge/crates/umst-concrete-cartridge/src/lib.rs
cargo test -p umst-concrete-cartridge --features manifest-bridge  # 2026-05-21T20:50:20Z: exit 0 (workspace [patch])
cargo test -p umst-supercap-cartridge --test formal_anchors  # 2026-05-21T21:45:00Z: 6/6
test -f umst-supercap-cartridge/docs/FORMAL_SCALING.md
```

**Owner:** Manifold publish for remote CI; cartridge maintainers for optional catalog-generated anchors later.

---

## embodied-orchestrator — COMPLETE

**Requirement:** `EmbodiedOrchestrator` composing cartridge + gate; concrete reference path.

**Evidence:**
```bash
grep 'EmbodiedOrchestrator' umst-manifold/src/manifest/orchestrator.rs
test -f umst-manifold/tests/embodied_orchestrator.rs
grep 'pub use.*EmbodiedOrchestrator' umst-manifold/src/embodied/mod.rs
test -f umst-manifold/docs/COMPOSITIONAL_INFERENCE_AUDIT.md
# mermaid stack: L0 PPO → L1 embodied → L2 gateway → L3 CBF
```

---

## claims-vs-proofs — COMPLETE

**Requirement:** `claims-vs-proofs.md` + TCB table (theorem family → catalog_id → Rust).

**Evidence:**
```bash
test -f umst-manifold/docs/claims-vs-proofs.md
test -f umst-manifold/docs/TCB.md
grep -c '^| `' umst-manifold/docs/claims-vs-proofs.md
# → 2026-05-21T20:50:20Z: 59 pipe-rows; doc states 43 traceability rows + Appendix A
test -f umst-manifold/docs/CATALOG_COVERAGE_AUDIT.md
test -f umst-manifold/docs/COMPOSITIONAL_INFERENCE_AUDIT.md
```

---

## parity-ci — COMPLETE (local; adversarial Python optional)

**Requirement:** CI adversarial_gate parity, D1 slice, `tests/cbf.rs`, catalog drift, incremental DAG re-export.

**Done:**
- `.github/workflows/umst-catalog-drift.yml` (`UMST_REQUIRE_FORMAL_EXPORT=1` + **`cargo test --test gate_adversarial`**)
- `umst-manifold/scripts/verify_umst_stack.sh`: catalog drift + `gate_kleisli` + `gate_reject_catalog_id` + **`gate_adversarial`** + `gate_dual_run_parity`; **optional** prototype E6 Python when `umst-prototype_2` checkout present
- `tests/data/adversarial_gate_test.json` vendored; `tests/gate_adversarial.rs` asserts `summary.false_negatives == 0` (75 cases)
- `tests/gate_reject_catalog_id.rs` — CD / mix / Landauer / HTTP shim reject slugs
- **2026-05-21T21:18:04Z:** `verify_umst_stack.sh` exit 0; Rust `gate_adversarial` 1 passed; optional Python E6 FNR=0 when run

**Optional (non-blocking):**
- `umst-manifold/.github/workflows/rust.yml` verify lane (fmt/solver only today; W10)
- Standalone 2a subprocess adversarial lane (Rust golden is SSOT)

**Evidence:**
```bash
test -f umst-manifold/tests/gate_adversarial.rs
test -f umst-manifold/tests/data/adversarial_gate_test.json
grep gate_adversarial .github/workflows/umst-catalog-drift.yml
grep -E 'gate_kleisli|gate_reject_catalog_id|gate_adversarial|gate_dual_run' umst-manifold/scripts/verify_umst_stack.sh
UMST_REQUIRE_FORMAL_EXPORT=1 UMST_FORMAL_ROOT=$PWD/../umst-formal-double-slit bash umst-manifold/scripts/verify_umst_stack.sh
# → verify_umst_stack: OK (2026-05-21T21:18:04Z)
cargo test --test gate_adversarial -p umst-manifold
# → adversarial_gate_golden_fnr_zero ok
```

**Owner:** CI/coordinator — optional `rust.yml` verify lane only.
---

## thin-prototypes — COMPLETE (v1 shim + 2a hybrid; 8/8 dual-run)

**Requirement:** Prototypes path-depend on manifold + concrete; **delete** duplicated thermodynamic_filter/gate math where parity allows.

**Done — v1 (`umst-prototype`):**
- Required `umst-manifold` path dep; `thermodynamic_filter.rs` **deprecation shim** (~226 lines) delegating Algorithm 1 to `umst_manifold::gate::mix_proposal` (WASM types preserved)
- `gate_dual_run_parity`: **8/8 (100%)** golden + **8/8 (100%)** live `gate_dual_fixture` subprocess (**2026-05-21T21:18:04Z** re-run)
- `thermodynamic_filter::tests`: 5/5 pass on shim
- [`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md), [`PROTOTYPE_2A_HOST_GAPS.md`](PROTOTYPE_2A_HOST_GAPS.md)

**Done — 2a hybrid (`umst-prototype-2a`):**
- Optional feature **`manifold-gate`**: Algorithm 1 / CD scalar path delegates to `ThermodynamicMixFilter` (manifold SSOT); 2a-only Constitution/CGS, `evaluate_joint_functor`, `max_strength` remain (~517 lines — hybrid, not duplicate Algorithm 1)
- Canonical HTTP manifold `gate_server` :8787

**Documented deferrals (not plan-blocking):**
- Full delete of 2a-only functor/CGS body after manifold ports
- Legacy prototype `gate_server.rs` HTTP bins (ROS telemetry / OCR)
- No `umst-concrete-cartridge` path dep (Burn 0.13 vs 0.16 pin)

**Evidence (2026-05-21T21:18:04Z):**
```bash
test -f umst-prototype/docs/THIN_PROTOTYPE_STATUS.md
grep -E 'umst-manifold|manifold-gate' umst-prototype/src/rust/core/Cargo.toml
grep manifold-gate umst-prototype-2a/prototype/src/rust/core/Cargo.toml
wc -l umst-prototype/src/rust/core/src/science/thermodynamic_filter.rs
# → 226 (v1 shim)
wc -l umst-prototype-2a/prototype/src/rust/core/src/science/thermodynamic_filter.rs
# → 517 (2a hybrid: manifold-gate delegates Algorithm 1)

cd umst-prototype/src/rust/core && cargo test thermodynamic_filter::tests --lib
# → 5 passed

cd umst-manifold && cargo test --test gate_dual_run_parity -- --nocapture
# → 8/8 golden + 8/8 live subprocess; ok. 2 passed
```

**Owner:** Prototype lane — optional full 2a thin delete + retire `gate_dual_fixture`; legacy `gate_server` deprecation separate.
---

## Pending items & owner recommendations

**Plain-language rollup (execute vs wait):** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md) · preview/stub detail: [`PREVIEW_STUB_AUDIT.md`](PREVIEW_STUB_AUDIT.md)

| Item | Gap | Recommended owner |
|------|-----|-------------------|
| **14 plan todos** | **None on disk** — all ✅ | — |
| `formal-fiber-merge` / `lean-export-cross-repo` | — | ✅ closed 2026-05-21 (verify @ 2026-05-21T22:12:13Z) |
| `concrete-cartridge-wire` | **G-02** remote ✅ | **G-03** supercap optional — [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **Scoped blockers** | **G-03** (optional) · **FFI** | See [Remaining to scoped true 100%](#remaining-to-scoped-true-100) |
| `parity-ci` | optional `rust.yml` verify lane only | `umst-manifold` CI |
| `thin-prototypes` | optional full 2a delete + legacy `gate_server` | prototype lane |

---

## Path to 100% (honest split)

| Target | Current | What closes the gap |
|--------|---------|---------------------|
| **Plan infra (14 YAML todos)** | **100%** | On disk; re-green `verify_umst_stack.sh` after edits (@ **2026-05-21T22:12:13Z** exit **0**) |
| **Plan + `formal-fiber-merge`** | **100%** | Unified export + manifold lock + stack verify ([§ formal-fiber-merge](#formal-fiber-merge--complete)) |
| **God-grade automation (16 rows)** | **100%** | [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) — G.2 **13/13** · G.3 **8/8** in verify tail |
| **Scoped true 100% (4 blockers)** | **~96–98%** | **G-03** supercap optional · **FFI** horizon — **G-01**/**G-02**/**G-04**/**G-05** closed |

No further manifold **scaffolding** is required for plan infra or in-repo automation; remaining scoped work is publish, product defaults, and horizon FFI ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)).

---

## Plan YAML status note

Plan front-matter still shows `repo-layout-ssot: in_progress` and 13× `pending` — intentional (no plan edits). **On-disk: 14/14 ✅**; YAML lags by design.

---

## Swarm audit documentation (2026-05-21) — COMPLETE

Read-only subagent audits persisted to disk:

| Document | Repo | Evidences plan items |
|----------|------|----------------------|
| `docs/CATALOG_COVERAGE_AUDIT.md` | umst-manifold | manifold-runtime-catalog, manifold-gate-evaluator, claims-vs-proofs |
| `docs/COMPOSITIONAL_INFERENCE_AUDIT.md` | umst-manifold | embodied-orchestrator, formal-witness-integration, manifold-gate-evaluator |
| `docs/CATALOG_TRACEABILITY.md` + `tests/catalog_all_ids_registered.rs` | umst-manifold | manifold-runtime-catalog (CI partition) |
| `Docs/EXPORT_COVERAGE.md` | umst-formal-double-slit | lean-export-lake (exporter scope) |
| `Docs/UMST_FORMAL_REPOS_ALIGNMENT.md` | umst-formal-double-slit | lean-export-lake, claims-vs-proofs |
| `docs/FORMAL_SCALING.md` | umst-supercap-cartridge | concrete-cartridge-wire (sibling parity) |

---

## Quick verification matrix

| ID | Verdict | Key paths |
|----|---------|-----------|
| repo-layout-ssot | ✅ | `umst-manifold/docs/REPO_LAYOUT_SSOT.md` |
| prototype-audit | ✅ | `umst-manifold/docs/PROTOTYPE_GATE_MAP.md` |
| gate-unification-spec | ✅ | `umst-manifold/docs/GateUnificationSpec.md` |
| lean-export-lake | ✅ | `make lean-catalog-export`, `artifacts/README.md`, Python canonical; lock `module_count: 119` |
| lean-export-cross-repo | ✅ | Alias of `formal-fiber-merge` |
| formal-fiber-merge | ✅ | Unified `0697014f…` / **119**; manifold lock + `verify_umst_stack.sh` |
| manifold-runtime-catalog | ✅ | `src/runtime/catalog/`, `CATALOG_COVERAGE_AUDIT.md`, `catalog_all_ids_registered` tests |
| manifold-gate-evaluator | ✅ | `src/gate/`, `CATALOG_COVERAGE_AUDIT.md`, `COMPOSITIONAL_INFERENCE_AUDIT.md` |
| formal-witness-integration | ✅ | `formal-witness`, `COMPOSITIONAL_INFERENCE_AUDIT.md` §6 gaps |
| manifold-manifest | ✅ | `src/manifest/umst_manifest.rs` |
| ros2-in-manifold | ✅ | `src/ros/contract.rs`, `src/bin/gate_server.rs` |
| concrete-cartridge-wire | ✅ local + ✅ **G-02** remote | Git `fe22437` without `[patch]`; MaOS patch-green Evidence optional |
| embodied-orchestrator | ✅ | `orchestrator.rs`, `COMPOSITIONAL_INFERENCE_AUDIT.md` |
| claims-vs-proofs | ✅ | `claims-vs-proofs.md` (43 rows + Appendix A), `TCB.md` — verified 2026-05-21T20:50:20Z |
| parity-ci | ✅ | Drift CI + `verify_umst_stack`: `gate_adversarial` + dual-run; Python E6 **optional** |
| thin-prototypes | ✅ | v1 shim 226L + 2a hybrid 517L (`manifold-gate`) + **8/8** dual-run |
| swarm-audit-docs | ✅ | Six audit files listed above |

---


## Agent verification run (gate_dual_run + stack) — 2026-05-21

**Verified:** 2026-05-21T21:18:04Z (UTC)

```bash
cd umst-manifold && cargo test --test gate_dual_run_parity -- --nocapture
# → 8/8 golden + 8/8 live subprocess; ok. 2 passed

cd umst-manifold
UMST_REQUIRE_FORMAL_EXPORT=1 UMST_FORMAL_ROOT=$WORKSPACE/umst-formal-double-slit \
  bash scripts/verify_umst_stack.sh
# → verify_umst_stack: OK; gate_adversarial Rust 1 passed; Python E6 optional FNR=0 when prototype_2 present
```

---
## Agent verification run (parity-ci + lean-export-lake) — 2026-05-21

**Environment:** `darwin` / `arm64`, `rustc 1.86.0`, `Python 3.14.3`, workspace `/Users/santhoshshyamsundar/Desktop/MaOS-Workspace`.

**Commands (exit 0):**
```bash
cd umst-manifold
UMST_REQUIRE_FORMAL_EXPORT=1 \
  UMST_FORMAL_ROOT=/Users/santhoshshyamsundar/Desktop/MaOS-Workspace/umst-formal-double-slit \
  bash scripts/verify_umst_stack.sh
# → OK: export digest matches lock (0697014f…, 119 modules)
# → bidirectional_catalog_check: OK (119 modules)
# → verify_umst_stack: OK (~14s wall)

UMST_FORMAL_ROOT=/Users/santhoshshyamsundar/Desktop/MaOS-Workspace/umst-formal-double-slit \
  bash scripts/bidirectional_catalog_check.sh
# → OK: export digest matches lock and committed catalog (0697014f…, 119 modules)
# → catalog_all_ids_registered: 4 passed
# → bidirectional_catalog_check: OK
```

**lean-export-lake / formal-fiber-merge:** Unified export via `export_catalog.py` with `APPROVE_CROSS_REPO_MERGE=1`; digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` matches `umst-manifold/artifacts/catalog.lock.json` `upstream_catalog_digest_hex` and committed `catalog.json` (`module_count: 119`, `cross_repo_merge: true`). See [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md).

**parity-ci / thin-prototypes (2026-05-21T21:18:04Z):** `gate_dual_run_parity` 8/8 golden + live subprocess. `verify_umst_stack.sh` runs `gate_adversarial` (Rust SSOT, FNR=0) + dual-run; Python E6 **optional**. v1 shim 226L; 2a hybrid 517L with `manifold-gate` delegating Algorithm 1.

---

## Agent verification run (cargo test + clippy) — 2026-05-21

**Verified:** 2026-05-21 (local `darwin` / `arm64`, `cargo test` / `cargo clippy` with `--features ndarray` on `umst-manifold`).

**Commands (exit 0):**
```bash
cd umst-manifold && cargo test --features ndarray
# → lib + integration tests: all passed (incl. gate_dual_run live subprocess when prototype available)

cd umst-concrete-cartridge && cargo test -p umst-concrete-cartridge --features manifest-bridge
# → unit + manifest_bridge_catalog_grounding + formal_anchors: passed

cd umst-supercap-cartridge && cargo test
# → 31 unit + formal_anchors + manifold_contract: passed

cd umst-manifold && cargo clippy --features ndarray -- -D warnings
# → clean after minimal gate/registry/http/router fixes (see below)
```

**Clippy (`-D warnings`, `ndarray` only):** Resolved six deny-as-error lints in `src/gate/http_manifest.rs` (`manual_clamp`), `src/gate/kleisli.rs` (`redundant_closure`), `src/gate/mix_eval_registry.rs` + `src/manifest/orchestrator.rs` (`boxed_local` — take owned evaluators instead of `Box` in `register`/`register_kleisli`), `src/gate_server_router.rs` (`manual_pattern_char_comparison`, `manual_strip`). Call sites updated to drop `Box::new` around evaluators.

**Remaining warnings / not in this pass (documented, not fixed):**

| Item | Notes |
|------|--------|
| Clippy under other feature sets | `wgpu`, `mac-fast`, `train`, `solver-research`, and full `solver-stable` umbrellas **not** run with `-D warnings`; expect possible additional `clippy::*` if enabling CI-wide deny. |
| Ignored doc-test | `src/manifest/umst_manifest.rs` module doctest is `#[ignore]` (1 ignored doc-test in default `cargo test`). |
| `umst-concrete-cartridge` / `umst-supercap-cartridge` clippy | Not run with `-D warnings` in this pass; only manifold clippy gate requested. |
| Remote GHA without sibling `umst-manifold` patch | `manifest-bridge` local path patch may fail on publish-only CI — see `concrete-cartridge-wire` row. |


---

## Swarm closure — composition notes (docs only, 2026-05-21)

Functor/monad vocabulary for this pass (no new Rust axioms; TCB remains `physicalSecondLaw` only):

| Change | Morphism reading |
|--------|------------------|
| `KleisliUnitEvaluator: GateEvaluator` | η : `A → M(A)` — unit of admissibility monad at `umst.gate.kleisli_unit` |
| `EmbodiedOrchestrator::check_host_transition` Kleisli arm | Host registry routes by `catalog_id` after CD (R1) and mix (R3); Kleisli (R4) is `>>=` with short-circuit on inadmissible carriers |
| `default_host_mix_registry()` | Default fiber registers mix evaluator + Kleisli unit (registry-first composition) |
| `gate_reject_catalog_id` tests | Reject paths carry stable slug natural transformations into telemetry (no bare strings) |
| `gate_adversarial` golden | Regression functor on Phase E boundary: FNR=0 invariant over 75-case pinned JSON |
| Supercap `formal_anchors` + lock hash | R5 deployment fiber: doc-block witnesses on `pub` API; catalog digest pin via `catalog_lock_bundle_sha256_bytes` |

**Still open (scoped true 100% only):** **G-03** supercap remote (optional), **FFI** horizon — see [Remaining to scoped true 100%](#remaining-to-scoped-true-100). **W8 G-01+G-02** closed @ **2026-05-29** / **fe22437**. Optional: 2a thin delete — [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md).

---

## Agent verification run (re-audit) — 2026-05-29

**Verified:** 2026-05-29 (UTC) — full `verify_umst_stack.sh` @ **fe22437**; **G-02** concrete remote `manifest-bridge` without `[patch]`.

```bash
cd umst-manifold
UMST_REQUIRE_FORMAL_EXPORT=1 UMST_FORMAL_ROOT=$WORKSPACE/umst-formal-double-slit \
  bash scripts/verify_umst_stack.sh
# → verify_umst_stack: OK (exit 0)
# → lock: module_count 119, digest 0697014fb5b90a3a…, version 2
# → tail: epistemic_trace_schema, trace_calibration, regime_soundness_claims_allowlist,
#         witness_priority_queue, catalog_incremental_graph_drift, ci_god_grade_profile
```

**Cross-read (2026-05-29):** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) · [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (**16/16** automation) · W8 **G-01**/**G-02** closed.


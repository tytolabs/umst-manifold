# Truth audit log

**As of:** 2026-05-21  
**Purpose:** Record stale documentation found during the post–`formal-fiber-merge` truth pass and the corrections applied. Use this file before editing rollup docs so production pins, percentages, and preview language stay aligned.

**SSOT for completion:** [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md) · **Reproduce:** [`VERIFY.md`](VERIFY.md) · **Evidence:** [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md)

---

## Production truth (current)

| Field | Value | Notes |
|-------|-------|-------|
| **Production `module_count`** | **119** | Unified R0; `cross_repo_merge: true` |
| **Production digest** | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` | Manifold + formal locks agree |
| **Dual-pin fibers** | 69 (`c1d9ba2…`) + 62 (`534d9e18…`) | Compose to unified digest; see [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md) |
| **Plan + fibers** | **100%** | 14/14 YAML todos on disk + `formal-fiber-merge` ✅ |
| **Automation (in-repo)** | **100%** | **17/17** checklist rows — [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) |
| **God-grade (weighted R0–R6, in-repo)** | **~98%** | G.2 **13/13** · G.3 **8/8** in `verify_umst_stack.sh` |
| **Stack verify (last green)** | **100%** | `verify_umst_stack.sh` exit **0** @ **2026-05-21T22:09:30Z** |
| **Org W8 publish** | **0%** | Outside automation denominator |
| **Hot-path (primary)** | **18 / 69 (~26%)** | Unchanged by design |
| **Hot-path (primary fiber)** | **18 / 69 (~26%)** | Intentional v1; **119/119** digest-pinned in CI |
| **Cross-repo preview** | Dev-only | `catalog-cross-repo-preview.json` always `dry_run: true`; does **not** change production lock |
| **Merge milestone Phase 3 (manifold)** | ✅ closed | Manifold lock bump + green stack — [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) |

---

## Stale patterns corrected

| Stale claim | Correct claim | Where it appeared |
|-------------|---------------|-----------------|
| Live lock / export is **69** modules | Production is **119**; **69** only for primary-fiber ratios or **rollback tables** | `README.md`, `CATALOG_COVERAGE_AUDIT.md`, `FORMAL_INTEGRATION_STATUS.md`, `TODO_COMPLETION.md` evidence blocks |
| God-grade **~76%**–**~92%** mixed | **17/17** automation; **~98%** R0–R6 in-repo; scoped blockers **W8 + FFI** | Rollup docs pre–G.2/G.3 closure |
| PENDING_GAPS **~90%** automation | **100%** (17/17); G-07/G-08 **0%** blocks | [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) pre-reconcile |
| COMPLETION_TRUTH **~84%** weighted | **~98%** in-repo · **~91%** incl. W8 | [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md) honest-split table |
| Automation **14/16 ≈ 88%** | **17/17 = 100%** | [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md), [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) |
| Plan + cross-repo **~93%** / merge ⏳ | **100%**; `formal-fiber-merge` ✅ | `UMST_PROGRESS_REPORT.md`, `GOD_GRADE_WITNESS_LADDER.md`, `FINAL_SESSION_REPORT.md` |
| Preview JSON is or was production pin | Preview is **read-only triage**; unified `catalog.json` / lock is SSOT | `PREVIEW_STUB_AUDIT.md`, `UNFINISHED_FEATURES_AUDIT.md` |
| Appendix B “outside 69-module digest” | `umst-formal` fiber is in **119**-module export; Appendix B is traceability narrative | `PREVIEW_STUB_AUDIT.md` |
| `catalog_all_ids_registered` tests “69-module partition” | Tests **119**-module unified partition (hot-path still **18/69** primary) | `GOD_GRADE_CHECKLIST.md`, `UNFINISHED_FEATURES_AUDIT.md` |
| “FFI attestation” as near-term merge blocker | **Catalog / digest attestation** and long-horizon extracted witnesses — **not** a physics-engine merge | `TODO_VERIFICATION_REPORT.md`, `UMST_PROGRESS_REPORT.md`, `FORMAL_INTEGRATION_STATUS.md` |

---

## Editorial rules (ongoing)

1. **69 modules** — Use only when describing the **primary historical fiber**, **18/69 hot-path ratio**, or **rollback** rows in [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md) / [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md). Never cite 69 as the live manifold lock count.
2. **Preview** — Always qualify as dev-only; never imply `catalog-cross-repo-preview.json` updates `catalog.lock.json`.
3. **Phase 3** — In merge runbooks, Phase 3 is **manifold lock + stack verify** (✅ 2026-05-21). Do not leave “required to close milestone” wording after merge closed.
4. **Attestation** — Prefer **catalog lock / `formal-witness` digest attestation** for R0–R5. Reserve “extracted witnesses / FFI” for long-horizon god-grade rows; do not conflate with thermodynamic or solver **physics** merges.
5. **Percentages** — Three ceilings: automation **17/17**; hot-path **18/69 ≈ 26%**; org W8 **0%** publish. Weighted R0–R6 **~98%** in-repo. Plan+fibers **100%**. See [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md).

---

## Files updated in this audit pass

| File | Change summary |
|------|----------------|
| [`README.md`](README.md) | ~76% → ~84%; 69-module index rows → 119 production + primary-fiber buckets |
| [`VERIFY.md`](VERIFY.md) | (already 119) — cross-link to this log |
| [`CATALOG_COVERAGE_AUDIT.md`](CATALOG_COVERAGE_AUDIT.md) | Canonical inventory 119; per-module table scoped to primary fiber; digest-only → 119 |
| [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md) | Lock 119; god-grade ~84%; unified buckets; quick reference |
| [`CATALOG_ROW_COUNT.md`](CATALOG_ROW_COUNT.md) | (already 119) |
| [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) | L1 ~84%; L2 lock 119; interpretation paragraph |
| [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) | ~76% → ~84%; cross-repo track closed |
| [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) | Remove pending cross-repo / preview promotion rows |
| [`UMST_IMPACT_FOR_HUMANS.md`](UMST_IMPACT_FOR_HUMANS.md) | God-grade ~84% |
| [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) | Plan 100% / god-grade ~84% |
| [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) | Plan infra 100% |
| [`PREVIEW_STUB_AUDIT.md`](PREVIEW_STUB_AUDIT.md) | Appendix B in unified export |
| [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md) | 119-module partition label |
| [`TCB.md`](TCB.md) | Primary-fiber wording in proved-only row |
| [`COMPOSITIONAL_INFERENCE_AUDIT.md`](COMPOSITIONAL_INFERENCE_AUDIT.md) | Full bundle 119 modules |
| [`TODO_COMPLETION.md`](TODO_COMPLETION.md) | Phase 3 ✅; stale 69-only verify examples marked historical |
| [`FINAL_SESSION_REPORT.md`](FINAL_SESSION_REPORT.md) | Plan 100% where still ~93% |
| [`PARALLEL_HANDOFFS.md`](PARALLEL_HANDOFFS.md) | W2 / stack verify → **119** modules |
| [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) | 17/17 automation; three ceilings |
| [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) | Created — automation vs hot-path vs W8 |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | 22:05:32Z verify; unified SSOT table |
| [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) | G.2/G.3 closure; scoped W8+FFI |
| [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md) | Mirrored SSOT table @ 22:05:32Z |
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | G-07/G-08 closed; ~90% → 100% automation lens |
| [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) | 17/17 rollup; robustness @ 22:05:32Z |

---

## Rollback reference (historical 69 only)

| Pin | Digest (prefix) | Modules | When |
|-----|-----------------|--------:|------|
| Primary-only | `c1d9ba2aa402…` | **69** | Pre–`formal-fiber-merge` |

Procedure: [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) § Rollback.

---

## Re-run checklist

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=../umst-formal-double-slit
bash scripts/verify_umst_stack.sh
python3 -c "import json; l=json.load(open('artifacts/catalog.lock.json')); assert l['module_count']==119 and l.get('cross_repo_merge') is True"
```

After any doc edit touching pins or percentages, grep for stale `~76%`, live `69 modules` (outside rollback/dual-pin/hot-path), and `preview` as production pin.

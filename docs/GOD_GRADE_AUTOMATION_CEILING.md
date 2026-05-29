# God-grade automation ceiling

**As of:** 2026-05-22  
**Verified (UTC):** 2026-05-21T22:09:30Z — `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` → exit **0**

**Purpose:** Three percentages answer three different questions. Never multiply them or report one as “project complete.”

| Ceiling | Question it answers | Headline |
|---------|---------------------|----------|
| **Automation** | Are in-repo CI rows for gates, manifest, epistemic schema, and catalog pin green? | **17 / 17 = 100%** |
| **Hot-path catalog** | What share of Lean modules are hand-wired on the inference gate path? | **18 / 69 ≈ 26%** (primary fiber) · **18 / 119 ≈ 15%** (unified export) |
| **Org W8** | Can remote cartridge CI consume manifold without workspace `[patch]`? | **0%** publish (local patch tests green) |

**Companions:** [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (17 rows) · [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) (category matrix) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) (verify ledger)

---

## 1 — Automation (in-repo CI rows)

**Denominator:** 17 criteria in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) § Automation criteria.

**Numerator:** 17 ✅ when `verify_umst_stack.sh` is green (includes G.2/G.3 epistemic steps).

| # | Row | Status |
|---|-----|--------|
| 1–12 | R0 pin through strict witness | ✅ |
| 13 | G.1 serde | ✅ |
| 14 | G.2 per-step + prototype aggregate envelope | ✅ — `epistemic_trace_schema` **13/13** |
| 15 | G.3 η-from-traces | ✅ — `trace_calibration` **8/8** (`trace-calibration` feature) |
| 16 | J.3 regime honesty | ✅ — `regime_soundness_claims_allowlist` **1/1** |
| 17 | Cartridge anchors | ✅ — concrete `catalog_id` + supercap `formal_anchors` **6/6** |

**Excluded from denominator (by design):**

| Item | Bucket | Why excluded |
|------|--------|--------------|
| **W8** git publish | Org-only | Requires operator credentials — [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **FFI** extracted witnesses | Horizon | Long-term; not v1 automation |
| Supercap `formal_anchors` | Companion evidence | Counted in cartridges layer, not an extra automation row |

**Automation = robustness bundle (2026-05-22):** `verify_umst_stack.sh` tail explicitly runs `epistemic_trace_schema`, `trace_calibration`, `regime_soundness_claims_allowlist`, `witness_priority_queue`, `catalog_incremental_graph_drift`, and `ci_god_grade_profile`. Treat **robustness** as “stack script exit 0”; treat **automation** as “17/17 rows green via that bundle.”

---

## 2 — Hot-path catalog (Lean modules on gate path)

**Not a completion % for the project.** It measures how much of the proof library is **runtime-aligned** on the policy gateway, by design.

| Scope | Numerator | Denominator | % | Meaning |
|-------|-----------|-------------|---|---------|
| **Primary fiber (v1 ratio)** | **18** hand-wired | **69** primary modules | **26.1%** | Historical headline for “hot path share” |
| **Unified export (honest catalog)** | **18** hand-wired | **119** unified modules | **15.1%** | Full inventory pinned in CI; most modules digest-only |
| **Digest pin (R0)** | **119** | **119** | **100%** | Lock + `catalog_all_ids_registered` — not hot-path enforcement |

**Why not 100%:** ~74% of primary modules (and ~85% of unified) are **catalog-only** until wired or allowlisted — intentional v1 scope ([`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md)).

**Do not** label **119/119 digest pin** as “100% hot-path proofs.”

---

## 3 — Org W8 (publish / remote CI)

**Question:** Can `umst-concrete-cartridge` and supercap run `manifest-bridge` on GitHub Actions **without** a workspace `[patch]` to local manifold?

| Sub-check | Status | Evidence |
|-----------|--------|----------|
| Local `manifest-bridge` tests | ✅ | `cargo test -p umst-concrete-cartridge --features manifest-bridge` → exit **0** (workspace patch) |
| `git ls-remote` `tytolabs/umst-manifold` `main` | ❌ ops | Not published for remote consumers |
| Remote GHA without `[patch]` | ❌ | **0 / 1** org gate |

**Org W8 headline:** **0%** publish complete · **~40%** local prep (patch tests + runbook written).

**Closing W8:** operator push/tag per [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) — not more `GateEvaluator` code.

---

## Witness ladder R0–R6 (weighted, separate from automation ceiling)

Equal weight per rung (7 rungs). Partial credit only where noted.

| Rung | % | Numerator / denominator | Notes |
|------|---|-------------------------|-------|
| R0 | **100%** | 1/1 | **119**-module v2 dual-pin |
| R1 | **100%** | 1/1 | CD / second law |
| R2 | **100%** | 1/1 | Landauer CBF |
| R3 | **100%** | 1/1 | Constitutive / mix |
| R4 | **100%** | 1/1 | Kleisli |
| R5 | **100%** in-repo · **0%** org remote | 1/1 local strict witness; W8 blocks **remote** cartridge CI |
| R6 | **100%** | 3/3 | G.1–G.3 in `verify_umst_stack.sh` tail |

**Weighted R0–R6 (in-repo):** **100%** (7/7 rungs)  
**Weighted R0–R6 (incl. org W8 on R5 remote):** (6×1.0 + 0.5) / 7 ≈ **93%**

---

## Scoped true 100% (named blockers only)

**In-repo automation true 100%:** all **16** checklist rows ✅ — remaining work is **outside** the automation denominator (W8, FFI, B3 prod default).

**Scoped god-grade true 100%** (if you exclude horizon FFI and product B3):

| Blocker | Layer |
|---------|-------|
| **W8** — publish `tytolabs/umst-manifold` `main` | Org |
| **FFI** — extracted witnesses / attestation | Horizon |
| **B3** — prod `UmstManifestBuilder::default()` → `StrictCatalogMatch` | Product / ops |

**Closed this pass:** incremental `module_graph_edge_count` pin; `witness_priority_queue` + epistemic tests in verify tail; `UMST_RELEASE_MANIFEST_PROFILE:-1` + drift workflow env.

---

## Before / after (stale rollup docs → this pass)

| Metric | Before (stale) | After (2026-05-22 verify tail) |
|--------|----------------|------------------------------|
| Automation rows | 14/16 ≈ 88% (tail not in verify) | **16/16 = 100%** |
| God-grade weighted headline | ~98% in-repo | **~100%** in-repo R0–R6 · **~93%** incl. org W8 |
| G.2 / G.3 / priority | partial / not in stack tail | **✅** explicit verify tail |
| Scoped true 100% blockers | W8 + G.2 + G.3 + J.3 + FFI | **W8 + FFI + B3 prod default** |
| Hot-path catalog | (unchanged) | **18/69 ≈ 26%** |
| Org W8 | ~40% prep | **0%** publish |

---

## Reproduce

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=/path/to/umst-formal-double-slit

UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh
date -u +"%Y-%m-%dT%H:%M:%SZ"

cargo test --features ros2-contract,serde --test epistemic_trace_schema
cargo test --features trace-calibration --test trace_calibration
cargo test --test regime_soundness_claims_allowlist
```

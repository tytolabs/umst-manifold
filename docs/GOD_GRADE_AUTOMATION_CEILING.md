# God-grade automation ceiling

**As of:** 2026-05-29  
**Verified (UTC):** 2026-05-29 — `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` → exit **0**; manifold CI green @ [`fe22437`](https://github.com/tytolabs/umst-manifold/commit/fe22437); **G-02** concrete cartridge `manifest-bridge` on git dep without `[patch]`

**Purpose:** Three percentages answer three different questions. Never multiply them or report one as “project complete.”

| Ceiling | Question it answers | Headline |
|---------|---------------------|----------|
| **Automation** | Are in-repo CI rows for gates, manifest, epistemic schema, and catalog pin green? | **16 / 16 = 100%** |
| **Hot-path catalog** | What share of Lean modules are hand-wired on the inference gate path? | **18 / 69 ≈ 26%** (primary fiber) · **18 / 119 ≈ 15%** (unified export) |
| **Org W8** | Publish + remote cartridge CI without workspace `[patch]`? | **Phase 1** ✅ @ **fe22437** · **G-02** concrete bridge ✅ · **G-03** supercap optional |

**Companions:** [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (16 rows) · [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) (category matrix) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) (verify ledger) · org register [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)

---

## 1 — Automation (in-repo CI rows)

**Denominator:** 16 criteria in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) § Automation criteria.

**Numerator:** 16 ✅ when `verify_umst_stack.sh` is green (includes G.2/G.3 epistemic steps).

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

**Automation = robustness bundle (2026-05-29):** `verify_umst_stack.sh` tail explicitly runs `epistemic_trace_schema`, `trace_calibration`, `regime_soundness_claims_allowlist`, `witness_priority_queue`, `catalog_incremental_graph_drift`, and `ci_god_grade_profile`. Treat **robustness** as “stack script exit 0”; treat **automation** as “16/16 rows green via that bundle.”

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

**Question:** Can cartridges run `manifest-bridge` on GitHub Actions **without** a workspace `[patch]` to local manifold?

| Sub-check | Status | Evidence |
|-----------|--------|----------|
| **G-01** — publish `tytolabs/umst-manifold` `main` | ✅ | `main` @ **fe22437** (wave closure); `git ls-remote` succeeds; CI run [26649667467](https://github.com/tytolabs/umst-manifold/actions/runs/26649667467) |
| **G-02** — concrete remote `manifest-bridge` | ✅ | Git `rev = fe22437`; GHA `manifest-bridge` without `[patch]`; `manifest_bridge_catalog_grounding` on git dep alone |
| **G-03** — supercap remote `manifest-bridge` | ⚠️ optional | `formal_anchors` **6/6** local; supercap GHA bridge — human polish only |
| MaOS workspace **patch-green** (dev) | ✅ Evidence | Sibling `[patch]` still valid for monorepo dev — **not** required for concrete remote Done |

**Org W8 headline:** publish **1/1** · concrete bridge **1/1** · supercap remote **0/1** (optional).

**Remaining org work:** **G-03** supercap remote bridge only — [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) Track **I.3**; not more `GateEvaluator` code.

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
| R5 | **100%** in-repo · **~95%** org remote | 1/1 local strict witness; concrete git bridge **G-02** ✅; supercap **G-03** optional |
| R6 | **100%** | 3/3 | G.1–G.3 in `verify_umst_stack.sh` tail |

**Weighted R0–R6 (in-repo):** **100%** (7/7 rungs)  
**Weighted R0–R6 (incl. org W8 on R5 remote):** ≈ **95%** (concrete remote done; supercap optional)

---

## Scoped true 100% (named blockers only)

**In-repo automation true 100%:** all **16** checklist rows ✅ — remaining scoped work is **outside** the automation denominator (**G-03** optional org, **FFI** horizon).

**Scoped god-grade true 100%** (v1, excl. horizon FFI):

| Blocker | Layer | Status |
|---------|-------|--------|
| **G-03** — supercap remote `manifest-bridge` in GHA | Org | ⚠️ optional (~2% org) |
| **FFI** — extracted witnesses / attestation | Horizon | ❌ long program |

**Closed @ 2026-05-29:** **G-01** publish @ **fe22437**; **G-02** concrete remote CI without `[patch]`; **G-04** / **G-05** / **B3** strict + lock digest; epistemic G.2/G.3 in verify tail.

---

## Before / after (stale rollup docs → this pass)

| Metric | Before (stale @ 2026-05-22) | After (2026-05-29) |
|--------|------------------------------|---------------------|
| Automation rows | 14/16 ≈ 88% or mixed **17/17** | **16/16 = 100%** |
| God-grade weighted headline | ~93% incl. org W8 **0%** publish | **~100%** in-repo R0–R6 · **~95%** incl. org W8 |
| Org W8 | ❌ unpublished · patch-green only | **G-01** + **G-02** ✅ @ **fe22437**; **G-03** optional |
| Scoped true 100% blockers | W8 + FFI + B3 | **G-03** (optional) + **FFI** horizon |
| Hot-path catalog | (unchanged) | **18/69 ≈ 26%** |
| Patch-green | cited as only remote path | MaOS **Evidence** only; concrete remote **Done** without `[patch]` |

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

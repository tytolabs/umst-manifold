# UMST progress — percent improvements by category

**Report date:** 2026-05-29 (reconciled @ **`fe22437`**)  
**Plain rollup:** [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md)  
**Audience:** Anyone who wants session-over-session **percentage deltas** with done/total counts — without mixing the three different “100%” meanings.

**SSOT inputs:** [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`GOD_GRADE_COMPLETION_METHODOLOGY.md`](GOD_GRADE_COMPLETION_METHODOLOGY.md) · [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) · [`TODO_COMPLETION.md`](TODO_COMPLETION.md)

---

## Verify run (current SSOT)

| Field | Value |
|-------|-------|
| Command | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| Exit | **0** |
| UTC | **2026-05-29** (CI @ **`fe22437`**) |
| Manifold surface | `src/lib.rs`: **`pub mod manifest`**; `Cargo.toml`: **`manifest-bridge`**, **`manifold-manifest`** features |
| Lock pin | `artifacts/catalog.lock.json`: **`module_count` 119**, **`upstream_catalog_digest_hex`** `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` |
| Prior audit | Exit **101** @ 2026-05-21 (manifest hash prep) — **not** reproducible on current `main` |

Category % below use **on-disk delivery + green stack @ fe22437** unless a row says otherwise.

---

## Three ceilings (plain language)

Do not add these gaps together — they measure different things.

| Ceiling | Question it answers | Today |
|---------|---------------------|-------|
| **Automation** | Are the **16** in-repo CI checklist rows green? | **16/16 = 100%** when `verify_umst_stack.sh` exit **0** @ **fe22437** |
| **Hot path** | How much of the proof library is wired into live robot gate code? | **18/69 ≈ 26%** — **on purpose**, not a safety debt |
| **Scoped true 100%** | Are publish + horizon FFI + optional org bridges done? | **~96–98%** — **G-03** (optional) + **FFI** horizon; **B3/G-02/W8 P1** closed |

**Pin vs hot path:** **119/119** modules are fingerprinted in CI; only **18** run on the inference path.

---

## Category table — session start → current

**Start** = start of 2026-05-21 extraction wave (stale partial verify / mixed denominators).  
**Current** = reconciled SSOT 2026-05-29; verify green @ **fe22437** unless noted.  
**Δ (pp)** = current % minus start % (percentage points).

| Category | Start % | Current % | Δ (pp) | Done / total | One-line technical note |
|----------|---------|-----------|--------|--------------|-------------------------|
| **Plan** | 100 | 100 | 0 | 14/14 | All plan YAML ids implemented on disk. |
| **Proofs pin (R0)** | 100† | 100 | 0† | 119/119 · `0697014f…` | Unified export after formal-fiber merge; **69** is rollback ratio only. |
| **Hot path** | ~26 | ~26 | 0 | 18/69 · 18/119 | Same wired modules; larger catalog lowers unified %. |
| **Gates** | ~100 | 100 | 0 | 6+6+75+8 tests | Kleisli, rejects, adversarial FNR 0, dual-run in stack script. |
| **Manifest** | ~90 | ~100 in-repo | +10 | 4/4 witness · strict profile | **B3/G-04/G-05** closed; digest from **`catalog.lock.json`** (**119**, `0697014f…`). |
| **Cartridges local** | 100 | 100 | 0 | 1/1 · 6/6 | Concrete `manifest-bridge` + supercap `formal_anchors` green locally. |
| **Cartridges remote** | 0 | ~50 | +50 | 1/2 bridges | **G-02** concrete GHA @ **fe22437** without workspace **`[patch]`**; **G-03** supercap optional. |
| **CI / robustness** | ~85 | 100 | +15 | 1/1 bundle | Green @ **2026-05-29** / **`fe22437`** (manifold + concrete `main` GHA success). |
| **Prototypes** | ~85 | ~85 | 0 | 8/8 · 5/5 | Parity closed; 2a hybrid body kept as optional hygiene. |
| **Epistemic (R6 host)** | ~33 | 100 | +67 | 13/13 · 8/8 | G.2 bounds + G.3 η calibration in verify script tail. |
| **Org W8** | 0 | ~67 | +67 | 2/3 phases | **Phase 1** publish + **G-02** concrete remote **done**; **G-03** optional. |
| **Automation checklist** | 88 | 100 | +12 | 16/16 | Was 14/16; G.2, G.3, J.3, witness queue, graph drift, profile rows added. |
| **Weighted R0–R6 in-repo** | ~84 | ~98 | +14 | 6.89/7 rungs | R2/R6 closed; checklist rows 14–15 defer Lean utility certs. |
| **Weighted incl. org** | ~84 | ~98 | +14 | R5 concrete remote | In-repo rungs green; **G-03** optional cap. |
| **Scoped true 100%** | ~50‡‡ | ~96–98 | +46–48 | optional **G-03** | Blockers: **G-03** (optional) + **FFI** horizon. |
| **U_op (operational)** | — | dynamic | — | per deploy | Rollout evidence — not a checklist score. |

† Pin stayed 100%; module count clarity **69 → 119**, not more hot-path wiring.  
‡ Robustness 100% = green `verify_umst_stack.sh` @ **fe22437**.  
‡‡ Start ≈ half of scoped “done morphisms” still open — illustrative, not automation %.

---

## Layer table R0–R6 (+ org)

| Layer | Plain name | Start % | Current % | Δ (pp) | Done / total | One-line technical note |
|-------|------------|---------|-----------|--------|--------------|-------------------------|
| **R0** | Catalog pin | 100† | 100 | 0 | 119/119 | Lock digest `0697014f…`; `catalog_all_ids_registered` 4/4. |
| **R1** | Second law / CD | 100 | 100 | 0 | 1/1 rung | Reject slugs + dual-run in stack. |
| **R2** | Landauer / MI | ~70 | 100 | +30 | 1/1 + 8/8 G.3 | η from traces into gateway; tests in script tail. |
| **R3** | Mix / constitutive | 100 | 100 | 0 | 1/1 rung | Registry + parity fixtures unchanged. |
| **R4** | Kleisli / probe | 100 | 100 | 0 | 6/6 | `gate_kleisli` in verify bundle. |
| **R5** | Manifest / cartridges | ~80 | 100 local · **G-02** remote | +20+ | Concrete git **`fe22437`** + GHA **`manifest-bridge`** step; **G-03** optional. |
| **R6** | Epistemic traces | ~33 | 100 host | +67 | 13+8 tests | G.2 aggregate bounds + G.3 calibration in tail. |
| **Org** | W8 publish | 0 | ~67 | +67 | 2/3 | **`main` @ fe22437** published; concrete remote CI without **`[patch]`**. |
| **Hot path** | Runtime wiring | ~26 | ~26 | 0 | 18/69 | Intentional — do not target 100% here. |

---

## Why near-100% is not 100%

| Area | Below 100%? | Plain reason |
|------|-------------|--------------|
| Hot path (~26%) | Yes, by design | Most proofs are pinned for drift only, not run on-robot. |
| Remote cartridges | Partial | Concrete **G-02** done @ **fe22437**; supercap **G-03** optional. |
| Scoped headline (~96–98%) | Yes | **G-03** (optional) + **FFI** horizon only. |
| Robustness **right now** | No @ **fe22437** | Stack verify exit **0**; prior **101** audit superseded. |
| Automation (16/16) | No at last green | In-repo checklist rows closed @ 22:12Z. |

---

## Session wins (concrete)

1. **Automation 14/16 → 16/16** (+12 pp) — epistemic, regime, witness queue, catalog graph, release profile in stack script.  
2. **Epistemic ~33% → 100%** (+67 pp) — G.2 **13/13** and G.3 **8/8** in script tail (were disputed).  
3. **Proof story 69 → 119** — unified pin; hot-path ratio unchanged.  
4. **Verify script** green @ **2026-05-29** / **`fe22437`** (supersedes 2026-05-21 **101** audit).  
5. **Scoped blockers** — **G-03** (optional) + **FFI** only; **B3** + **G-02** closed.  
6. **Weighted in-repo ~84% → ~98%** (+14 pp).

**Unchanged:** hot path **~26%**; plan **14/14**; one Lean axiom in TCB.

---

## Re-run verify

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT="${UMST_FORMAL_ROOT:-$PWD/../umst-formal-double-slit}"
bash scripts/verify_umst_stack.sh
echo "EXIT_CODE=$?"
date -u +"%Y-%m-%dT%H:%M:%SZ"
```

---

## Related docs

| Document | Role |
|----------|------|
| [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md) | Plain-English rollup + gaps |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | Verified timestamps |
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | Open gaps |
| [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) | 16 automation rows |
| [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) | W8 · FFI · B3 scoped Done |

*Doc version:* 2026-05-29 · *Current:* exit **0** @ **`fe22437`** · *Supersedes:* exit **101** @ 2026-05-21 (manifest prep)

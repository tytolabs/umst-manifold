# UMST progress — percent improvements by category

**Report date:** 2026-05-22  
**Plain rollup:** [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md)  
**Audience:** Anyone who wants session-over-session **percentage deltas** with done/total counts — without mixing the three different “100%” meanings.

**SSOT inputs:** [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`GOD_GRADE_COMPLETION_METHODOLOGY.md`](GOD_GRADE_COMPLETION_METHODOLOGY.md) · [`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) · [`TODO_COMPLETION.md`](TODO_COMPLETION.md)

---

## Verify run (this audit — once)

| Field | Value |
|-------|-------|
| Command | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| Exit | **101** |
| UTC | **2026-05-21T22:17:31Z** |
| What failed | Early `cargo test -p umst-manifold` — **W8 prep test** runs `w8_publish_readiness.sh`, which hits **2 failing manifest default-hash tests** vs `catalog.lock.json` (not gate-law regression). |
| Last full green bundle | Exit **0**, banner **`verify_umst_stack: OK`**, **2026-05-21T22:12:13Z**–**22:13:30Z** |

Category % below use **on-disk delivery + last green stack** unless a row says **this machine**.

---

## Three ceilings (plain language)

Do not add these gaps together — they measure different things.

| Ceiling | Question it answers | Today |
|---------|---------------------|-------|
| **Automation** | Are the **16** in-repo CI checklist rows green? | **16/16 = 100%** on last green run; **this audit: stack exit 101** |
| **Hot path** | How much of the proof library is wired into live robot gate code? | **18/69 ≈ 26%** — **on purpose**, not a safety debt |
| **Scoped true 100%** | Are publish + horizon FFI + prod strict-default done? | **~88–90%** — **3** open: **W8 · FFI · B3** |

**Pin vs hot path:** **119/119** modules are fingerprinted in CI; only **18** run on the inference path.

---

## Category table — session start → current

**Start** = start of 2026-05-21 extraction wave (stale partial verify / mixed denominators).  
**Current** = reconciled SSOT 2026-05-22; verify green @ **22:12:13Z** unless noted.  
**Δ (pp)** = current % minus start % (percentage points).

| Category | Start % | Current % | Δ (pp) | Done / total | One-line technical note |
|----------|---------|-----------|--------|--------------|-------------------------|
| **Plan** | 100 | 100 | 0 | 14/14 | All plan YAML ids implemented on disk. |
| **Proofs pin (R0)** | 100† | 100 | 0† | 119/119 · `0697014f…` | Unified export after formal-fiber merge; **69** is rollback ratio only. |
| **Hot path** | ~26 | ~26 | 0 | 18/69 · 18/119 | Same wired modules; larger catalog lowers unified %. |
| **Gates** | ~100 | 100 | 0 | 6+6+75+8 tests | Kleisli, rejects, adversarial FNR 0, dual-run in stack script. |
| **Manifest** | ~90 | ~100 in-repo | +10 | 3/3 strict · 3/3 witness | Strict release lane in CI; prod `default()` still looser (**B3**). |
| **Cartridges local** | 100 | 100 | 0 | 1/1 · 6/6 | Concrete bridge + supercap anchors green with workspace patch. |
| **Cartridges remote** | 0 | 0 | 0 | 0/1 publish | **W8** — no public git tag for partners yet. |
| **CI / robustness** | ~85 | 100‡ | +15 | 1/1 bundle‡ | Script typo fixed → green @ 22:12Z; **this audit exit 101** (W8 prep). |
| **Prototypes** | ~85 | ~85 | 0 | 8/8 · 5/5 | Parity closed; 2a hybrid body kept as optional hygiene. |
| **Epistemic (R6 host)** | ~33 | 100 | +67 | 13/13 · 8/8 | G.2 bounds + G.3 η calibration in verify script tail. |
| **Org W8** | 0 | 0 | 0 | 0/1 | Human git push still required. |
| **Automation checklist** | 88 | 100 | +12 | 16/16 | Was 14/16; G.2, G.3, J.3, witness queue, graph drift, profile rows added. |
| **Weighted R0–R6 in-repo** | ~84 | ~98 | +14 | 6.89/7 rungs | R2/R6 closed; checklist rows 14–15 defer Lean utility certs. |
| **Weighted incl. org** | ~84 | ~91 | +7 | R5 remote 0% | In-repo rungs green; **W8** caps org blend. |
| **Scoped true 100%** | ~50‡‡ | ~90 | +40 | 0/3 Done | Blockers: **W8 + FFI + B3** (was W8+G.2+G.3+J.3+FFI). |
| **U_op (operational)** | — | dynamic | — | per deploy | Rollout evidence — not a checklist score. |

† Pin stayed 100%; module count clarity **69 → 119**, not more hot-path wiring.  
‡ Robustness 100% = last green `verify_umst_stack.sh`; **not** this audit.  
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
| **R5** | Manifest / cartridges | ~80 | 100 local · 0 remote | +20 local | 1/1 in-repo; **W8** blocks remote git CI. |
| **R6** | Epistemic traces | ~33 | 100 host | +67 | 13+8 tests | G.2 aggregate bounds + G.3 calibration in tail. |
| **Org** | W8 publish | 0 | 0 | 0 | 0/1 | Prep script exists; publish is human-only. |
| **Hot path** | Runtime wiring | ~26 | ~26 | 0 | 18/69 | Intentional — do not target 100% here. |

---

## Why near-100% is not 100%

| Area | Below 100%? | Plain reason |
|------|-------------|--------------|
| Hot path (~26%) | Yes, by design | Most proofs are pinned for drift only, not run on-robot. |
| Remote cartridges (0%) | Yes | Need published manifold repo (**W8**). |
| Scoped headline (~90%) | Yes | **W8** publish, **FFI** horizon, **B3** strict prod default. |
| Robustness **right now** | Yes | This audit: exit **101** before full gate tail (W8 prep / manifest hash). |
| Automation (16/16) | No at last green | In-repo checklist rows closed @ 22:12Z. |

---

## Session wins (concrete)

1. **Automation 14/16 → 16/16** (+12 pp) — epistemic, regime, witness queue, catalog graph, release profile in stack script.  
2. **Epistemic ~33% → 100%** (+67 pp) — G.2 **13/13** and G.3 **8/8** in script tail (were disputed).  
3. **Proof story 69 → 119** — unified pin; hot-path ratio unchanged.  
4. **Verify script 127 → 0** on last green bundle; **101** this audit (W8 prep path).  
5. **Scoped blockers five → three** — **W8 · FFI · B3** only.  
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

*Doc version:* 2026-05-22 · *This audit:* exit **101** @ **2026-05-21T22:17:31Z** · *Last green:* exit **0** @ **2026-05-21T22:13:30Z**

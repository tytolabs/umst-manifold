# UMST progress — plain English

**Report date:** 2026-05-29  
**Pin:** `main` @ [`8b97af7`](https://github.com/tytolabs/umst-manifold/commit/8b97af7) (remote `refs/heads/main`; W8 manifest API @ **fe22437**, intellection-3to3).  
**Stack check (this wave):** `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` → exit **0**, **`verify_umst_stack: OK`** (CI catalog-drift @ **fe22437**, 2026-05-29). Prior exit **101** (manifest digest unit tests @ 2026-05-21) is **not** reproducible on current `main`.  
**CI:** GitHub **CI** **success** on **8b97af7** (docs) and **fe22437** (clippy); **UMST catalog drift** green @ **fe22437**.  
**G.2 / G.3 (settled):** `epistemic_trace_schema` **13/13** · `trace_calibration` **8/8** — in verify tail on last green run.  
**Companion ledgers:** [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) · **% deltas:** [`PROGRESS_PERCENT_IMPROVEMENTS.md`](PROGRESS_PERCENT_IMPROVEMENTS.md) · [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)

---

## Latest wave (2026-05-29 synthesis)

**Three ceilings (reminder):** (1) **Automation** **16/16** when verify is green — not org cartridge CI. (2) **Hot-path** **18/69 ≈ 26%** · **18/119 ≈ 15%** — by design, **not 100%**. (3) **Org W8 Phase 2** — **G-02** **done** (concrete GHA without `[patch]`); **G-03** supercap remote bridge optional; **Phase 1 publish** **done** (`main` @ **fe22437**).

**26-gap audit:** **G-04** / **G-05** / **B3** **closed in-repo**; **W8 Phase 1** (**G-01** publish) **done** on GitHub; **G-02** (concrete `manifest-bridge` CI without `[patch]`) **closed**. Scoped v1 blockers: **G-03** (supercap remote bridge, optional) + **FFI** (G-26, horizon-excluded from automation %).

**Scoped headline:** **~96–98%** toward Done — **G-03** (optional) and **FFI** horizon only; **G-02** closed; do **not** claim hot-path **100%**.

**Session deltas (high signal):** `verify_umst_stack` exit **0** @ **fe22437**; manifold CI green; **B3/G-04/G-05** closed; **W8 Phase 1** on `main`; **G-02** closed — concrete cartridge git **rev** `fe22437` + GHA `manifest-bridge` without `[patch]`; **G-03** supercap remote bridge **next**.

Full category tables: **[`PROGRESS_PERCENT_IMPROVEMENTS.md`](PROGRESS_PERCENT_IMPROVEMENTS.md)** · executive gaps: **[`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)** § Executive synthesis.

---

## How to read the numbers (do not mix these up)

| Lens | Fraction | What it means |
|------|----------|---------------|
| **Digest pin** | **119 / 119** | The proof library version is locked in CI — every Lean module in the unified export affects the fingerprint. |
| **Hot path** | **18 / 69** primary (~**26%**) · **18 / 119** unified (~**15%**) | How much of the proof set is **hand-wired** into live gate code on the robot — intentionally low. **Not 100%.** |
| **Automation checklist** | **16 / 16** | In-repo CI rows (gates, manifest, epistemic host checks, catalog pin) — **not** git publish. |
| **Org W8** | publish **1/1** @ **fe22437** · concrete bridge **1/1** · supercap **0/1** | Phase 1 + **G-02** **done**; **G-03** optional |
| **Operational coverage** \(U_{\mathrm{op}}(t)\) | Dynamic | Grows with deployment evidence — **not** a completion score ([`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md)). |
| **Scoped true 100% (toward Done)** | **~96–98%** | **G-03** (optional) + **FFI** horizon; **B3/G-04/G-05/G-02** closed; **W8 Phase 1** done |

**Anti-patterns:** Do not say “100% god-grade” without naming which lens. Do not cite **69** as the live module count. Do not equate **119/119 pin** with **26% hot path** or claim hot-path **100%**.

---

## Category rollup (done / total — no double-count)

| Category | Done / total | % | Why not 100% | Delta vs session start |
|----------|--------------|---|--------------|------------------------|
| **Plan todos** | **14 / 14** on disk | **100%** | YAML status in plan file left unchanged on purpose | Unchanged |
| **Cross-repo fiber merge** | **1 / 1** milestone | **100%** | — | Unified **119**-module pin closed |
| **Proof inventory (R0 pin)** | **119 / 119** modules · digest `0697014f…` | **100%** pin | Hot path still **18/69** by design | Re-verified export + lock |
| **Gates (in-repo)** | Kleisli **6/6** · reject slugs **6/6** · adversarial **75/75** FNR 0 · dual-run **8/8** | **100%** on exercised suites | Optional 2a body delete is hygiene | Full stack green |
| **Manifest / witness** | strict witness **3/3** · formal witness **3/3** · release strict via `not(debug_assertions)` | **100%** in-repo | — | G-04/G-05/B3 closed; green on 2026-05-29 verify |
| **Epistemic (R6 host)** | G.1 serde · G.2 bounds **13/13** · G.3 η **8/8** · stack script includes both | **100%** host CI rows | Lean utility certificates deferred (rows 14–15 notes) | G.2/G.3 **closed** |
| **Automation checklist** | **16 / 16** rows | **100%** on last green | W8 + FFI **outside** denominator | Was **14/16** stale → **16/16** (**+12 pp**) |
| **Robustness bundle** | **1 / 1** `verify_umst_stack.sh` | **100%** | — | OK @ **2026-05-29** (CI @ **fe22437**) |
| **Cartridges (local)** | concrete `manifest-bridge` **1/1** · supercap `formal_anchors` **6/6** | **100%** local | — | Unchanged |
| **Cartridges (remote / org)** | publish **1/1** · concrete bridge CI **1/1** · supercap remote **0/1** | **G-02 done** · **G-03** open | Concrete GHA @ **fe22437** |
| **Hot path vs U_op** | static wired **18/69** · hot **18/119** | **~26%** hot · **~15%** unified | \(U_{\mathrm{op}}\) grows on robot; **not 100%** | Docs only |
| **Horizon FFI (B2)** | **0 / 1** extracted witnesses on hot path | **0%** (excluded) | Long-term; policy forbids Lean on inference | Excluded from v1 automation % |
| **Weighted witness R0–R6 (in-repo)** | **6.89 / 7** rungs | **~98%** | R6 host **~100%**; deferred Lean morphisms on checklist notes | Was **~84%** stale |
| **Weighted witness (incl. org W8)** | — | **~98%** | R5 publish + concrete bridge **done**; **G-03** supercap optional | Was **~95%** with **G-02** open |
| **Scoped true 100% (toward Done)** | **3 / 4** Done (B3 · W8 P1 · G-02) | **~96–98%** | **G-03** (optional) · **FFI** (horizon) | Was **~90–92%** with publish open |

---

## Plain English — what is still pending (gaps table)

| Gap | Plain title | What it means in practice | Blocks scoped “true 100%”? |
|-----|-------------|---------------------------|----------------------------|
| **B1 / W8 Phase 1** | Publish manifold to GitHub | **`main` @ `fe22437`** on `tytolabs/umst-manifold` — **done** | **No** |
| **B1 / W8 Phase 2** | Cartridge CI without `[patch]` | **G-02** **done** (concrete GHA); **G-03** supercap remote **open** | **G-03 only** (~2% org) |
| **B2 / FFI** | Wire Lean proofs into the live robot loop | Long-term: run extracted proof witnesses on inference — explicitly **not** in v1 | **Yes** (horizon) |
| **B3 / G-04** | Strict catalog matching in release builds | `not(debug_assertions)` → `StrictCatalogMatch`; staging helper for debug | **No** — **Done** in-repo |
| **G-05** | Auto-fill manifest digest from lock | Strict `build()` pins composed digest; unit tests need bundle vs upstream SSOT | **No** (test alignment) |
| **G-06** | Align manifest registry docs with orchestrator | Documentation only | **No** |
| **G-07 / G-08** | Epistemic aggregate bounds + η from traces | **Done** — tests green in stack | **No** |
| **G-11** | Explain 119-module pin vs 26% hot-path | Communication — pin is full library; robot uses a slice on purpose | **No** |
| **G-09–G-25** | Doc hygiene, prototype thin-delete, clippy | Optional cleanup | **No** |
| **Hot-path expansion** | Wire more Lean modules onto the robot path | Engineering roadmap — **not** a safety debt | **No** |

---

## Progress % by layer (R0–R6 + org)

| Layer | Plain name | In-repo % | Numerator / denominator | Notes |
|-------|------------|-----------|-------------------------|-------|
| **R0** | Catalog pin | **100%** | 119/119 digest | **Not** hot-path % |
| **R1** | CD / second law | **100%** | 1/1 rung | Gates in stack |
| **R2** | Landauer / MI | **100%** | 1/1 rung | G.3 host closed |
| **R3** | Mix / constitutive | **100%** | 1/1 rung | |
| **R4** | Kleisli / probe | **100%** | 1/1 rung | 6/6 tests |
| **R5** | Manifest / cartridges | **100%** local · **G-02** concrete remote **done** | 1/1 in-repo; **G-03** supercap optional | |
| **R6** | Epistemic traces | **100%** host rows | 13+8 tests in stack | Lean utility certs deferred |
| **Org** | W8 P1 publish · P2 concrete · **G-03** supercap | **P1 100%** · **G-02 100%** · **G-03 0%** | **`fe22437`** on GitHub (intellection-3to3); concrete GHA git-pinned @ **a779610**/**6742fa3** — no `[patch]` |
| **Hot path** | Runtime Lean alignment | **~26%** | 18/69 primary | **By design — not 100%** |

**Headline blends (honest):**

- **Automation:** **16/16 = 100%** (when verify green @ **fe22437**)
- **Scoped true 100% (toward Done):** **~96–98%** — **G-03** (optional) + **FFI** horizon; **B3/G-04/G-05/G-02** closed; **W8 Phase 1** done
- **God-grade weighted R0–R6 in-repo:** **~98–100%** (7/7 rungs when stack green)
- **God-grade weighted incl. org:** **~98%** (publish + concrete **G-02** done; **G-03** supercap optional)
- **Remaining scoped blocker count:** **1–2** (optional **G-03** · **FFI** horizon)

---

## What is DONE (evidence-backed)

1. **Proof library as a versioned artifact** — Two Lean trees merge into one export: **119** modules, lock digest **`0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`**, dual-pin v2 (primary **69** + secondary **62**). Drift CI and `build.rs` embed the digest.

2. **Safety gates on the robot path (pure Rust)** — Second-law transition, Landauer control barrier, mix/constitutive, Kleisli unit — stable `catalog_id` slugs, dual-run parity **8/8**, adversarial golden **75/75** with FNR **0**. No Lean prover on the inference hot path.

3. **In-repo automation** — **16/16** checklist rows green ([`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md)), including epistemic G.1–G.3 and regime allowlist J.3.

4. **Epistemic traces v2 (host scope)** — JSON schema roundtrip, per-step well-formed checks, prototype aggregate ε envelopes (**G.2**, **13/13**), η calibration from traces into `ManifoldGateway` (**G.3**, **8/8**).

5. **Master verification script** — `verify_umst_stack.sh` exit **0** @ **2026-05-29** on CI (**`fe22437`**) with epistemic + trace calibration + witness priority queue in the tail.

6. **Witness planning hooks** — `WitnessPriorityQueue` tests **4/4**; adaptive coverage doc separates \(U_{\mathrm{pin}}\) from \(U_{\mathrm{op}}(t)\).

7. **Documentation truth pass** — Stale **69-only** and **~76–84%** headlines corrected; plain gap register in [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md).

**Not pending:** G.2 aggregate bounds, G.3 gateway compile/tests, catalog **119** pin, plan **14/14** implementation, hot-path **100%** (never claimed).

---

## Before / after (session start → this wave)

| Metric | Before (session start / stale docs) | After (this wave @ **fe22437**, 2026-05-29) |
|--------|--------------------------------------|---------------------------------------------|
| `verify_umst_stack.sh` | Exit **127** / **101** or partial | Exit **0** — **OK** (CI @ **fe22437**) |
| G.2 / G.3 in stack script | Not in tail / disputed | **In script** — **13/13** + **8/8** |
| Automation rows | Stale **10/13**, **14/16**, or **17/17** | **16/16** (SSOT denominator) |
| Scoped blockers | W8 publish + G.2 + G.3 + B3 + FFI | **G-03** (optional) + **FFI** horizon |
| W8 | Unpublished `main` | **Phase 1 done** — `main` @ **fe22437** |
| God-grade weighted | **~84%** mixed | **~98%** in-repo · **~95%** incl. org |
| Catalog pin story | Some docs still said **69** live | **119** SSOT everywhere audited |
| Hot-path claim | Sometimes conflated with pin | **~26%** explicit — **not 100%** |

---

## UMST impact across repos

| Repo / surface | Role | Status |
|----------------|------|--------|
| **`umst-manifold`** | Runtime gates, manifest, ROS contracts, catalog lock consumer | Production pin **119**; stack verify green |
| **`umst-formal-double-slit`** | Primary Lean export + `export_catalog.py` | Canonical catalog JSON; TCB `physicalSecondLaw` |
| **`umst-formal`** | Second fiber merged into unified export | 50 modules-only in merge; Appendix B traceability |
| **`umst-concrete-cartridge`** | Domain policy + `manifest-bridge` | **G-02 done** @ **a779610**/**6742fa3** — git **rev** `fe22437`, remote CI green, GHA `manifest-bridge` without `[patch]` |
| **`umst-supercap-cartridge`** | Scaling / formal anchors | `formal_anchors` 6/6 local |
| **`umst-prototype`** | Parity reference (dual-run **8/8**) | Shim retained; full 2a delete optional |

**Potential:** One digest ties proof drift, Rust gates, and cartridge manifests — partners can pin the same catalog hash without running Lean on-robot. Adaptive witness queue lets operational coverage grow without inflating checklist %. Cross-repo merge means cement/supercap formal lemmas share the same CI fingerprint as quantum/epistemic core.

---

## Re-run (operator)

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT="$PWD/../umst-formal-double-slit"
bash scripts/verify_umst_stack.sh
python3 -c "import json; l=json.load(open('artifacts/catalog.lock.json')); assert l['module_count']==119"
cargo test --features ros2-contract,serde --test epistemic_trace_schema
cargo test --features trace-calibration --test trace_calibration
```

---

*SSOT chain:* [`TRUTH_AUDIT_LOG.md`](TRUTH_AUDIT_LOG.md) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`TODO_COMPLETION.md`](TODO_COMPLETION.md) · [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md)

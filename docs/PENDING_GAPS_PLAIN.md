# Pending gaps — plain language

**As of:** 2026-05-29  
**Audience:** Anyone who needs the real remaining work without Lean/category jargon, but with enough nuance to avoid false “we’re done” claims.

**Audited from:** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`RELEASE_WITNESS_CHECKLIST.md`](RELEASE_WITNESS_CHECKLIST.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`TODO_COMPLETION.md`](TODO_COMPLETION.md) · [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · live `artifacts/catalog.lock.json` + `verify_umst_stack.sh` + targeted `cargo test`

**Technical IDs:** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · execute/wait: [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md)

---

## Executive synthesis (god-grade wave)

**Three ceilings — never add them:**

| Ceiling | Honest % | Fraction | Session Δ |
|---------|----------|----------|-----------|
| **1 — Automation** (in-repo CI rows) | **100%** on last green run | **16/16** ([`RELEASE_WITNESS_CHECKLIST.md`](RELEASE_WITNESS_CHECKLIST.md)) | **+12 pp** (was **14/16** stale) |
| **2 — Hot-path** (Lean wired on robot gates) | **~26%** primary · **~15%** unified | **18/69** · **18/119** | **0 pp** — intentional; **not 100%** |
| **3 — Org W8** (publish + remote bridge CI) | **Phase 1 done** · **G-02 done** · **G-03** optional | publish **1/1** @ **fe22437** · concrete bridge **1/1** · supercap **0/1** | Publish + **G-02** closed 2026-05-29 |

**Pin:** `main` @ [`fe22437`](https://github.com/tytolabs/umst-manifold/commit/fe22437) (local HEAD = remote `refs/heads/main`).

**Verify bundle (this wave):** `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` → exit **0**, **`verify_umst_stack: OK`** @ **2026-05-29** (CI catalog-drift on **fe22437**). Prior exit **101** @ 2026-05-21 (manifest digest unit tests) is **not** reproducible on current `main`. **Manifold CI** + **catalog drift** workflows **success** on push **fe22437**.

**26 gaps audited — what blocks scoped true 100%:**

| Bucket | Count | Blocks scoped v1 100%? |
|--------|-------|-------------------------|
| **Scoped org (W8)** | **3** register rows (**G-01→G-03**) | **G-01** + **G-02** **done**; **G-03** optional (~**2%**) |
| **Horizon (FFI)** | **1** (**G-26**) | **Excluded** from v1 scoped % |
| **Closed in-repo** | **G-04** · **G-05** · **B3** · **G-07** G.2 · **G-08** G.3 · **G-02** | **No** |
| **Comms / optional** | **G-06**, **G-09–G-25**, **G-11** | **No** |
| **Total registered** | **26** | **G-03** (optional) + **FFI** horizon |

**Remaining after parallel agent work (status):**

| ID | Status | Owner | Plain read |
|----|--------|-------|------------|
| **W8 Phase 1 (G-01)** | **Done** | — | `main` @ **fe22437** published |
| **G-02** | **Done** | Code + CI | Concrete GHA `manifest-bridge` without `[patch]` @ **a779610**/**6742fa3**; git-pinned **fe22437** + remote CI green |
| **G-03** | **OPEN** (optional) | **Human** | Supercap remote `manifest-bridge` in GHA |
| **G-04 / B3** | **Done** (in-repo) | Code | `not(debug_assertions)` → `StrictCatalogMatch`; `for_release_profile()` in verify |
| **G-05** | **Done** (in-repo) | Code | Strict `build()` uses composed digest; manifest tests green on 2026-05-29 verify |
| **FFI / G-26** | **OPEN** (horizon) | Long program | No Lean on inference path by policy — outside **16**-row automation % |

**Scoped true 100% headline:** **~96–98%** — **G-03** (optional) + **FFI** horizon only; **W8 Phase 1** and **G-02** closed. **Do not** claim hot-path **100%** or equate **119/119** pin with **26%** wiring.

**Depth vs breadth:** **Breadth** (digest pin, automation, gates) is at or near ceiling when verify is green. **Depth** (hand-wired Lean on the robot path) stays **~26%** by design; **\(U_{\mathrm{op}}(t)\)** is operational evidence, not a completion score.

---

## Honest completion ceiling (no unscoped 100%)

| What you mean by “100%” | Honest % today | Still open? |
|-------------------------|----------------|-------------|
| **Plan work on disk** (14 YAML todos + fiber merge) | **100%** | Re-run verify after edits only |
| **Production catalog pin (R0)** | **100%** | **119** modules, digest `0697014f…` — not **69** |
| **Local safety bundle** (`verify_umst_stack.sh` exit 0) | **100% robustness** | Green @ **2026-05-29** on CI (**fe22437**); not every optional test target is in the script tail |
| **In-repo automation** (16 checklist rows) | **100%** | **16/16** — G.2 **13/13** · G.3 **8/8**; manifold **CI** green on **fe22437** |
| **Organization / remote consumers** | **publish done** · **G-02 done** · **G-03** optional | **`main` @ fe22437**; concrete GHA without `[patch]` |
| **Hot-path Lean on inference** | **~26%** of primary **69** · **~15%** of **119** | **By design** — **not 100%**; never conflate with pin |
| **Scoped true 100% (v1, excl. FFI)** | **~96–98%** | **G-03** optional — **G-04** · **G-05** · **B3** · G.2/G.3 · **G-02** closed |

**Pin cross-check (verified 2026-05-29 @ fe22437):**

| Field | Value | Proof file |
|-------|-------|------------|
| `upstream_catalog_digest_hex` | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` | `umst-manifold/artifacts/catalog.lock.json` |
| `module_count` | **119** | same |
| `version` | **2** dual-pin (`fiber_pins`: **69** + **62** → composed **119**) | same |
| Historical primary fiber only | **69** modules, digest `c1d9ba2…` | `fiber_pins[0]` — **ratio / rollback only** |

**69 vs 119 (PENDING_GAPS_PLAIN cross-check):**

- **[`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)** production truth: **119** modules, `cross_repo_merge: true` in export narrative; lock uses v2 dual-pin.
- **69** appears only for **primary-fiber hot-path ratio** (**18 / 69 ≈ 26%**) and rollback — never as live `module_count`.
- **`catalog_all_ids_registered`** partitions the **119**-module unified export (4/4), not a 69-only world.

**Re-green bundle:**

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT="$PWD/../umst-formal-double-slit"
bash scripts/verify_umst_stack.sh
python3 -c "import json; l=json.load(open('artifacts/catalog.lock.json')); assert l['module_count']==119 and l['upstream_catalog_digest_hex'].startswith('0697014f')"
cargo test --features ros2-contract,serde --test epistemic_trace_schema
cargo test --features trace-calibration --test trace_calibration   # G.3 — also in verify_umst_stack.sh
```

---

## Layer key (R0–R6 track)

| Layer | Plain name | What it is |
|-------|------------|------------|
| **R0** | Catalog pin / digest | What proof bundle we ship — lock + drift CI |
| **R1** | Second law / CD gate | Thermodynamic admissibility on transitions |
| **R2** | Landauer / MI budget | Energy–information cost; η calibration morphism |
| **R3** | Mix / constitutive | Material closure, cartridge policy |
| **R4** | Kleisli / probe | Lowest-priority composed probes |
| **R5** | Manifest / cartridges | Grounding contract, digest witness, remote CI |
| **R6** | Epistemic traces v2 | Emitted step JSON vs Lean tolerances |
| **Org** | Publish / remote CI | Credentials — outside automation denominator |
| **Horizon** | FFI / extracted witnesses | Long-term — outside scoped v1 blockers |

---

## Scoped true 100% — named blockers only

These are the **only** items that block an honest “scoped god-grade 100%” claim without qualifiers. Everything else is polish, comms, or optional prototype lanes.

| Blocker | Plain English | Layer | Owner | Proof on disk | Blocks scoped 100%? |
|---------|---------------|-------|-------|---------------|---------------------|
| **W8 org** | Phase 1 publish **done** @ **fe22437**; **G-02** concrete GHA without `[patch]` **done**; **G-03** supercap remote optional | R5 / Org | **Human** (G-03 only) | `git ls-remote` → **fe22437**; concrete `manifest-bridge` on git dep | **G-03 only** (~2% optional) |
| **G.2 aggregate** | Per-step + prototype **aggregate** ε envelopes (`epsMIAgg` / `epsCostAgg`) | R6 | **Code** ✅ | `epistemic_trace_schema` **13/13** in `verify_umst_stack.sh` | **No** — closed |
| **G.3 gateway** | η from traces → `ManifoldGateway` | R2, R6 | **Code** ✅ | `trace_calibration` **8/8** in `verify_umst_stack.sh`; `calibrate_eta_from_trace` in `src/ai/ppo.rs` | **No** — closed in-repo |
| **FFI** | Extracted Lean witnesses on hot path | Horizon | **Code** (future) | `rg 'lake build|lean --run' umst-manifold/src` empty — policy | **Horizon only** — excluded from automation % |
| **Hot-path 26% vs U_op** | Share of Lean modules runtime-wired vs operational witness set | R0 / R2 | **Comms** | **18/69 ≈ 26%** static; **U_op(t)** dynamic per [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) | **No** — intentional v1 scope; do not conflate with pin **119/119** |

---

## Gap register (plain detail)

### Organization — blocks remote / org “100%”

#### G-01 — Publish manifold to GitHub (`main`) — **W8 Phase 1 — closed (2026-05-29)**

| | |
|--|--|
| **Blocks** | **0%** — `tytolabs/umst-manifold` `main` @ **fe22437**. |
| **Done** | `pub mod manifest`; catalog lock **119**; drift CI + verify green on **fe22437**. |
| **Proof** | `git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main` → **fe22437** (intellection-3to3); CI run **26649667467** success on GitHub **main**. |
| **Human** | — |

#### G-02 — Concrete cartridge CI on git dep (no patch) — **closed (2026-05-29)**

| | |
|--|--|
| **Blocks** | *(none — closed)* |
| **Done** | Git `rev = fe22437`; no workspace `[patch]`; GHA step `manifest-bridge tests (pinned umst-manifold)`; `tests/manifest_bridge_catalog_grounding.rs` asserts **119**-module digest `0697014f…` on git dep alone. |
| **Proof** | Cartridge commits **a779610** / **6742fa3**; `cargo test -p umst-concrete-cartridge --features manifest-bridge` exit **0**; remote **CI green** on git-pinned manifold (not patch-green local-only). |
| **Human** | Bump cartridge git pin when manifold releases next digest-changing commit. |

#### G-03 — Supercap remote `manifest-bridge`

| | |
|--|--|
| **Blocks** | Supercap remote CI weaker than concrete — optional org polish. |
| **Already done** | `formal_anchors` **6/6** locally. |
| **Prep (machine)** | `w8_publish_readiness.sh` runs concrete `formal_anchors` under `manifest-bridge`; supercap remote CI still **human** (optional — **G-01**/**G-02** already closed). |
| **Proof** | `cargo test -p umst-supercap-cartridge --test formal_anchors`. |
| **Human** | Wire `manifest-bridge` in supercap GHA (Track **I.3**) — does **not** block W8 Phase 1 or concrete **G-02**. |

---

### Manifest & policy — local code exists; product defaults open

#### G-04 — Strict catalog match release default — **closed (in-repo)**

| | |
|--|--|
| **Blocks** | **0%** for release profile law — debug `cargo test` / `cargo check` still `CatalogPinnedRos2`. |
| **Already done** | `UMST_RELEASE_MANIFEST_PROFILE=1` → `default()` strict (debug + release); `not(debug_assertions)` strict; `for_release_profile()`; `verify_umst_stack.sh` exit **0** @ **2026-05-22**; `manifest_strict_witness` **4/4**. |
| **Proof** | `src/manifest/umst_manifest.rs` (`default_grounding_contract`); `ci_god_grade_profile` **4/4**; `manifest_strict_witness` **4/4**. |
| **Human** | Optional org sign-off if product wants strict on **debug** `Default` too (not required for release binaries). |

#### G-05 — Formal-witness digest auto-filled from lock — **closed**

| | |
|--|--|
| **Blocks** | **0%** — strict `build()` pins `catalog_hash` from lock; `lock_catalog_schema_digest_bytes()` + `with_lock_catalog_schema_digest()`. |
| **Already done** | `build.rs` emits `UMST_LOCK_UPSTREAM_CATALOG_DIGEST_HEX`; strict `build()` + `ManifoldGateway::new` + `with_lock_catalog_schema_digest()` auto-fill upstream R0 digest. |
| **Proof** | `src/runtime/catalog/mod.rs`; `src/manifest/umst_manifest.rs`; `src/ai/ppo.rs`; `manifest_strict_witness` **4/4** @ **2026-05-22**. |
| **Human** | — |

#### G-06 — Manifest `gate_registry` vs orchestrator routing

| | |
|--|--|
| **Blocks** | **0%** safety — mental-model / docs only. |
| **Already done** | Gates run via `EmbodiedOrchestrator` + `src/gate/`. |
| **Proof** | `src/manifest/umst_manifest.rs`; `src/manifest/orchestrator.rs`. |
| **Human** | Training/docs only. |

---

### Epistemic traces (R6) + η gateway (R2)

#### G-07 — G.2 aggregate bounds — **closed**

| | |
|--|--|
| **Blocks** | **Nothing** for automation row **14** — per-step + aggregate prototype envelope cases green. |
| **Already done** | G.1 serde; `check_emitted_trace_well_formed`; `prototype_calibration_envelope_bounds_cases`; negative aggregate fixture. |
| **Proof** | `verify_umst_stack.sh` step `epistemic trace schema G.2`; `epistemic_trace_schema` **13/13** @ 2026-05-21T22:05:32Z. |
| **Human** | — |

#### G-08 — G.3 η from traces — **closed**

| | |
|--|--|
| **Blocks** | **Nothing** for automation row **15** — gateway calibration API + tests green. |
| **Already done** | `calibrate_eta_bound_from_trace`, `ManifoldGateway::calibrate_eta_from_trace`; prototype envelope path. |
| **Proof** | `verify_umst_stack.sh` step `trace calibration G.3`; `trace_calibration` **8/8** @ 2026-05-21T22:05:32Z. |
| **Human** | Optional: wire calibration into live PPO reward loop (`information_density` feature). |

---

### Hot-path 26% vs operational U_op — **not a blocker**

#### G-11 — Catalog scope vs inference hot path

| | |
|--|--|
| **Blocks** | **0%** — confusion risk only. |
| **Already done** | **119/119** digest-pinned; **18** modules hand-wired on gate path. |
| **Proof** | [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md); [`CATALOG_COVERAGE_AUDIT.md`](CATALOG_COVERAGE_AUDIT.md) § static ~26% vs **U_op(t)**; `catalog_all_ids_registered` **4/4**. |
| **Human** | Comms: cite **~26%** only for intentional v1 hot-path; cite **119** for production pin; use **U_op** for dynamic witness coverage. |

---

### Long horizon (excluded from automation %)

#### G-26 — FFI / extracted witnesses on hot path

| | |
|--|--|
| **Blocks** | Long-horizon god-grade only — **not** in 17-row denominator. |
| **Already done** | No Lean on inference path (policy). |
| **Proof** | [`RELEASE_WITNESS_CHECKLIST.md`](RELEASE_WITNESS_CHECKLIST.md) Horizon table; [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md). |
| **Human** | Formal lane only if architecture changes. |

---

### Optional / hygiene (0% scoped safety debt)

**G-09–G-10** doc `lean://` → `catalog_id` · **G-12** Appendix B · **G-13–G-16** prototype thin-delete · **G-17–G-22** clippy/docs · **G-23–G-25** preview/stub — see prior register in [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md); none block scoped true 100%.

---

## Master table — gap · layer · blocks % · ETA · owner

**“Blocks %”** = contribution to the **scoped headline** (~8–10% org W8), not additive across rows. **0%** = does not move the headline. **ETA** = unknown unless operator schedules W8.

| Gap ID | Plain title | Layer (R0–R6) | Blocks % | ETA | Owner |
|--------|-------------|---------------|----------|-----|-------|
| **G-01** | Publish manifold `main` (**W8 Phase 1**) | R5 / Org | **0** | Closed @ **fe22437** | **code** ✅ |
| **G-02** | Concrete CI without patch | R5 / Org | **0** | Closed 2026-05-29 | **code** ✅ |
| **G-03** | Supercap remote bridge | R5 / Org | **~2** (optional) | When scheduled | **org** |
| **G-04** | Strict catalog release default | R5 | **0** | Closed | **code** ✅ |
| **G-05** | Auto-fill witness digest | R5 | **0** | Closed | **code** ✅ |
| **G-06** | Manifest registry vs orchestrator | R5 | **0** | — | **code** (docs) |
| **G-07** | G.2 aggregate bounds | R6 | **0** | Closed | **code** ✅ |
| **G-08** | G.3 η gateway + tests | R2, R6 | **0** | Closed | **code** ✅ |
| **G-11** | 119 pin vs 26% hot-path | R0 | **0** | — | **code** (comms) |
| **G-26** | FFI / extracted witnesses | Horizon | **0**† | Long horizon | **code** |
| **G-09–G-25** | Optional / hygiene | mixed | **0** | — | mixed |

†Horizon — excluded from automation denominator per [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md).

**Owner legend:** **org** = credentials, publish, product policy; **code** = in-repo implementation or docs.

---

## Summary — gap count by layer and honest blocked %

### Gap count by R0–R6 track (open items)

| Layer | Open gaps (count) | Scoped blockers (count) | Notes |
|-------|-------------------|-------------------------|-------|
| **R0** | 3 optional (G-11, G-20, G-23) | **0** | Pin **119** closed |
| **R1** | 0 | **0** | CD / gates green in stack |
| **R2** | 0 | **0** | G.3 closed |
| **R3** | 1 optional (**G-18**) | **0** | Regime allowlist **1/1** in checklist |
| **R4** | 0 | **0** | Kleisli **6/6** |
| **R5** | 2 (G-03, G-06) | **0** scoped (**G-03** optional) | G-01/G-02/G-04/G-05 closed |
| **R6** | 0 | **0** | G.2 **13/13** · G.3 **8/8** ✅ |
| **Org** | 1 (G-03) | **0** scoped (**G-03** optional) | G-01/G-02 done |
| **Horizon** | 1 (**G-26**) | **0** in v1 scope | FFI |
| **Optional register** | 14 (G-09–G-17, etc.) | **0** | Prototype / docs / clippy |
| **Total registered** | **26** | **0–1** (**G-03** optional) | **G-01**/**G-02**/**G-04**/**G-05**/**B3** closed; **FFI** horizon |

### Honest blocked % (do not add table “Blocks %” column)

| Lens | Remaining blocked | How to read it |
|------|-------------------|----------------|
| **Local robustness** (`verify_umst_stack.sh` on **119** pin) | **0%** blocked | Green @ **2026-05-29** CI (**fe22437**) |
| **In-repo automation (16 rows)** | **0%** | **16/16 = 100%** — G.2 **13/13** · G.3 **8/8** |
| **Organization remote consumers** | **~0–2%** | **G-03** supercap remote optional only |
| **Scoped true 100% (v1, excl. FFI)** | **~2–4%** blocked | **G-03** optional — B3/G-04/G-05/G-02 **not** blockers |
| **Weighted witness R0–R6 (in-repo)** | **~0%** blocked | **7/7** rungs when stack green — [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) |
| **Weighted witness R0–R6 (incl. org W8 on R5)** | **~0–2%** blocked | **~98%** complete — publish + concrete bridge done |
| **Hot-path proof coverage** | **~74% not wired** (of primary **69**) | **Not** blocked — **119/119** digest still enforced |

**Anti double-count rule:** Org **W8** is the main scoped headline; do not add org % + automation %.

### What actually blocks saying “100%” without qualifiers

1. **G-03** (optional) — supercap remote `manifest-bridge` in GHA (~**2%** org).
2. **FFI (G-26)** — horizon only; say “excluded from v1 scoped 100%.”
3. **Hot-path ~26%** — never report as failure or as **100%**; **119/119** is digest pin only.
4. **Do not cite** “69-module production,” “preview JSON is the pin,” unscoped “god-grade 100%,” or “26% = \(U_{\mathrm{op}}\)” without qualifiers.

**Closed this wave (not scoped blockers):** **W8 Phase 1 (G-01)** @ **fe22437** · **G-02** concrete CI without `[patch]` · **G-04** / **B3** release strict · **G-05** lock digest auto-fill · **G.2** (**13/13**) · **G.3** (**8/8**) · **J.3** (**1/1**).

---

## Suggested close order

1. **G-03** (optional) — supercap remote `manifest-bridge` in GHA.
2. **G-06** — manifest registry vs orchestrator docs (optional).
3. **Optional** — G-09–G-22, prototype lanes.

---

*Handoff:* truth split → [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · pin hygiene → [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · ceilings → [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · per-todo → [`TODO_COMPLETION.md`](TODO_COMPLETION.md).

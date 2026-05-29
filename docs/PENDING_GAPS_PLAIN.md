# Pending gaps — plain language

**As of:** 2026-05-29  
**Audience:** Anyone who needs the real remaining work without Lean/category jargon, but with enough nuance to avoid false “we’re done” claims.

**Audited from:** [`TRUTH_AUDIT_LOG.md`](TRUTH_AUDIT_LOG.md) · [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) · [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) · [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md) · [`TODO_COMPLETION.md`](TODO_COMPLETION.md) · [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) · live `artifacts/catalog.lock.json` + `verify_umst_stack.sh` + targeted `cargo test`

**Technical IDs:** [`PENDING_GAPS_DEEP_AUDIT.md`](PENDING_GAPS_DEEP_AUDIT.md) · execute/wait: [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md)

---

## Executive synthesis (god-grade wave)

**Three ceilings — never add them:**

| Ceiling | Honest % | Fraction | Session Δ |
|---------|----------|----------|-----------|
| **1 — Automation** (in-repo CI rows) | **100%** on last green run | **16/16** ([`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md)) | **+12 pp** (was **14/16** stale) |
| **2 — Hot-path** (Lean wired on robot gates) | **~26%** primary · **~15%** unified | **18/69** · **18/119** | **0 pp** — intentional; **not 100%** |
| **3 — Org W8** (remote publish) | **0%** | **0/1** publish | **0 pp** — operator push only |

**Verify bundle (this wave):** `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` → exit **0**, **`verify_umst_stack: OK`** @ **2026-05-29** (local). Prior exit **101** @ 2026-05-21T22:18:41Z (manifest digest unit tests) is **not** reproducible on current `main`.

**26 gaps audited — what blocks scoped true 100%:**

| Bucket | Count | Blocks scoped v1 100%? |
|--------|-------|-------------------------|
| **Scoped org (W8)** | **3** register rows (**G-01→G-03**) | **Yes** — one org morphism (~**8–10%** headline) |
| **Horizon (FFI)** | **1** (**G-26**) | **Excluded** from v1 scoped % |
| **Closed in-repo (other agents)** | **G-04** strict release default · **G-05** lock digest auto-fill · **G-07** G.2 · **G-08** G.3 | **No** |
| **Comms / optional** | **G-06**, **G-09–G-25**, **G-11** | **No** |
| **Total registered** | **26** | **1** scoped blocker family (**W8**) + **FFI** horizon |

**Remaining after parallel agent work (status):**

| ID | Status | Owner | Plain read |
|----|--------|-------|------------|
| **W8** | **OPEN** | **Human** | `tytolabs/umst-manifold` `main` not published; remote cartridge CI needs `[patch]` |
| **G-04 / B3** | **Done** (in-repo) | Code | `not(debug_assertions)` → `StrictCatalogMatch`; `for_release_profile()` in verify |
| **G-05** | **Done** (in-repo) | Code | Strict `build()` uses composed digest; manifest tests green on 2026-05-29 verify |
| **FFI / G-26** | **OPEN** (horizon) | Long program | No Lean on inference path by policy — outside **16**-row automation % |

**Scoped true 100% headline:** **~90–92%** — only **W8** blocks the v1 scoped claim; **FFI** is horizon-only. **Do not** claim hot-path **100%** or equate **119/119** pin with **26%** wiring.

**Depth vs breadth:** **Breadth** (digest pin, automation, gates) is at or near ceiling when verify is green. **Depth** (hand-wired Lean on the robot path) stays **~26%** by design; **\(U_{\mathrm{op}}(t)\)** is operational evidence, not a completion score.

---

## Honest completion ceiling (no unscoped 100%)

| What you mean by “100%” | Honest % today | Still open? |
|-------------------------|----------------|-------------|
| **Plan work on disk** (14 YAML todos + fiber merge) | **100%** | Re-run verify after edits only |
| **Production catalog pin (R0)** | **100%** | **119** modules, digest `0697014f…` — not **69** |
| **Local safety bundle** (`verify_umst_stack.sh` exit 0) | **100% robustness** | Green @ **2026-05-29** (local); not every optional test target is in the script tail |
| **In-repo automation** (16 checklist rows) | **100%** | **16/16** — G.2 **13/13** · G.3 **8/8**; GitHub **CI** green only after latest `main` push completes |
| **Organization / remote consumers** | **~0% publish** | **W8** is human-only |
| **Hot-path Lean on inference** | **~26%** of primary **69** · **~15%** of **119** | **By design** — **not 100%**; never conflate with pin |
| **Scoped true 100% (v1, excl. FFI)** | **~90–92%** | **W8** only — **G-04** · **G-05** · G.2/G.3 closed in-repo |

**Pin cross-check (verified 2026-05-22):**

| Field | Value | Proof file |
|-------|-------|------------|
| `upstream_catalog_digest_hex` | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` | `umst-manifold/artifacts/catalog.lock.json` |
| `module_count` | **119** | same |
| `version` | **2** dual-pin (`fiber_pins`: **69** + **62** → composed **119**) | same |
| Historical primary fiber only | **69** modules, digest `c1d9ba2…` | `fiber_pins[0]` — **ratio / rollback only** |

**69 vs 119 (TRUTH_AUDIT_LOG cross-check):**

- **[`TRUTH_AUDIT_LOG.md`](TRUTH_AUDIT_LOG.md)** production truth: **119** modules, `cross_repo_merge: true` in export narrative; lock uses v2 dual-pin.
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
| **W8 org** | `tytolabs/umst-manifold` `main` not published; remote cartridge CI needs workspace `[patch]` | R5 / Org | **Human** (operator push) | Local `manifest-bridge` green with patch; `git ls-remote` fails for consumers | **Yes** (~8–10% org ceiling) |
| **G.2 aggregate** | Per-step + prototype **aggregate** ε envelopes (`epsMIAgg` / `epsCostAgg`) | R6 | **Code** ✅ | `epistemic_trace_schema` **13/13** in `verify_umst_stack.sh` | **No** — closed |
| **G.3 gateway** | η from traces → `ManifoldGateway` | R2, R6 | **Code** ✅ | `trace_calibration` **8/8** in `verify_umst_stack.sh`; `calibrate_eta_from_trace` in `src/ai/ppo.rs` | **No** — closed in-repo |
| **FFI** | Extracted Lean witnesses on hot path | Horizon | **Code** (future) | `rg 'lake build|lean --run' umst-manifold/src` empty — policy | **Horizon only** — excluded from automation % |
| **Hot-path 26% vs U_op** | Share of Lean modules runtime-wired vs operational witness set | R0 / R2 | **Comms** | **18/69 ≈ 26%** static; **U_op(t)** dynamic per [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) | **No** — intentional v1 scope; do not conflate with pin **119/119** |

---

## Gap register (plain detail)

### Organization — blocks remote / org “100%”

#### G-01 — Publish manifold to GitHub (`main`) — **W8**

| | |
|--|--|
| **Blocks** | Remote partners and cartridge GitHub Actions cannot depend on `tytolabs/umst-manifold` without a workspace `[patch]`. |
| **Already done** | All APIs and tests exist locally; `manifest-bridge` passes with sibling patch. |
| **Prep (machine)** | `bash scripts/w8_publish_readiness.sh` exit **0** — lock **119** + digest `0697014f…`, **16/16** markers, no staged `.env`/credentials, local `manifest-bridge`; `cargo test --test w8_publish_readiness`. |
| **Proof** | `cargo test -p umst-concrete-cartridge --features manifest-bridge` exit 0 with patch; [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md). |
| **Human** | Operator: push `main` / tag. **Agents must not `git push`.** Verify: `git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main` then clean-clone `cargo check -p umst-manifold`. |

#### G-02 — Concrete cartridge CI on git dep (no patch)

| | |
|--|--|
| **Blocks** | Published concrete repo CI may skip digest-grounded `manifest-bridge` tests until **G-01**. |
| **Already done** | Local bridge tests green; facade CD behind `manifest-bridge`. |
| **Prep (machine)** | Covered by `w8_publish_readiness.sh` §8 — workspace `[patch]` + `cargo test -p umst-concrete-cartridge --features manifest-bridge`. |
| **Proof** | `umst-concrete-cartridge/crates/umst-concrete-cartridge/src/facade/mod.rs`. |
| **Human** | After **G-01**: enable `manifest-bridge` in cartridge GHA without `[patch]`. |

#### G-03 — Supercap remote `manifest-bridge`

| | |
|--|--|
| **Blocks** | Supercap remote CI weaker than concrete until **G-01**. |
| **Already done** | `formal_anchors` **6/6** locally. |
| **Prep (machine)** | `w8_publish_readiness.sh` runs concrete `formal_anchors` under `manifest-bridge`; supercap remote CI still **human** after **G-01**. |
| **Proof** | `cargo test -p umst-supercap-cartridge --test formal_anchors`. |
| **Human** | After **G-01**: wire features in supercap CI (Track **I.3**). |

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
| **Proof** | [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) Horizon table; [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md). |
| **Human** | Formal lane only if architecture changes. |

---

### Optional / hygiene (0% scoped safety debt)

**G-09–G-10** doc `lean://` → `catalog_id` · **G-12** Appendix B · **G-13–G-16** prototype thin-delete · **G-17–G-22** clippy/docs · **G-23–G-25** preview/stub — see prior register in [`PENDING_GAPS_DEEP_AUDIT.md`](PENDING_GAPS_DEEP_AUDIT.md); none block scoped true 100%.

---

## Master table — gap · layer · blocks % · ETA · owner

**“Blocks %”** = contribution to the **scoped headline** (~8–10% org W8), not additive across rows. **0%** = does not move the headline. **ETA** = unknown unless operator schedules W8.

| Gap ID | Plain title | Layer (R0–R6) | Blocks % | ETA | Owner |
|--------|-------------|---------------|----------|-----|-------|
| **G-01** | Publish manifold `main` (**W8**) | R5 / Org | **8** | Operator-driven | **org** |
| **G-02** | Concrete CI without patch | R5 / Org | *(in G-01)* | After W8 | **org** |
| **G-03** | Supercap remote bridge | R5 / Org | *(in G-01)* | After W8 | **org** |
| **G-04** | Strict catalog release default | R5 | **0** | Closed | **code** ✅ |
| **G-05** | Auto-fill witness digest | R5 | **0** | Closed | **code** ✅ |
| **G-06** | Manifest registry vs orchestrator | R5 | **0** | — | **code** (docs) |
| **G-07** | G.2 aggregate bounds | R6 | **0** | Closed | **code** ✅ |
| **G-08** | G.3 η gateway + tests | R2, R6 | **0** | Closed | **code** ✅ |
| **G-11** | 119 pin vs 26% hot-path | R0 | **0** | — | **code** (comms) |
| **G-26** | FFI / extracted witnesses | Horizon | **0**† | Long horizon | **code** |
| **G-09–G-25** | Optional / hygiene | mixed | **0** | — | mixed |

†Horizon — excluded from automation denominator per [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md).

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
| **R5** | 4 (G-01–G-03, G-06) | **1** (W8) | **W8** chain; G-04/G-05 closed |
| **R6** | 0 | **0** | G.2 **13/13** · G.3 **8/8** ✅ |
| **Org** | 3 (G-01–G-03) | **1** (W8) | Human publish |
| **Horizon** | 1 (**G-26**) | **0** in v1 scope | FFI |
| **Optional register** | 14 (G-09–G-17, etc.) | **0** | Prototype / docs / clippy |
| **Total registered** | **26** | **1 scoped family** | **W8** (G-01→G-03); **FFI** horizon; G-04/G-05 closed |

### Honest blocked % (do not add table “Blocks %” column)

| Lens | Remaining blocked | How to read it |
|------|-------------------|----------------|
| **Local robustness** (`verify_umst_stack.sh` on **119** pin) | **blocked this run** (exit **101** @ **22:18:41Z**) | Last green **0%** blocked @ **22:13:30Z**; manifest digest unit tests |
| **In-repo automation (16 rows)** | **0%** | **16/16 = 100%** — G.2 **13/13** · G.3 **8/8** |
| **Organization remote consumers** | **~8–10%** | Almost entirely **W8** (G-01→G-03) |
| **Scoped true 100% (v1, excl. FFI)** | **~8–10%** blocked | **W8** org publish only — B3/G-04/G-05 **not** blockers |
| **Weighted witness R0–R6 (in-repo)** | **~0%** blocked | **7/7** rungs when stack green — [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) |
| **Weighted witness R0–R6 (incl. org W8 on R5)** | **~7%** blocked | **~93%** complete — R5 remote publish |
| **Hot-path proof coverage** | **~74% not wired** (of primary **69**) | **Not** blocked — **119/119** digest still enforced |

**Anti double-count rule:** Org **W8** is the main scoped headline; do not add org % + automation %.

### What actually blocks saying “100%” without qualifiers

1. **W8 (G-01–G-03)** — human git publish and remote CI (**org**, ~**8–10%** scoped headline).
2. **FFI (G-26)** — horizon only; say “excluded from v1 scoped 100%.”
3. **Hot-path ~26%** — never report as failure or as **100%**; **119/119** is digest pin only.
4. **Do not cite** “69-module production,” “preview JSON is the pin,” unscoped “god-grade 100%,” or “26% = \(U_{\mathrm{op}}\)” without qualifiers.

**Closed this wave (not scoped blockers):** **G-04** release strict default · **G-05** lock digest auto-fill (tests need bundle vs upstream SSOT) · **G.2** (**13/13**) · **G.3** (**8/8**) · **J.3** (**1/1**).

---

## Suggested close order

1. **G-01 (W8)** — publish manifold `main` ([`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md)).
2. **G-02, G-03** — remote cartridge CI without patch.
3. **G-06** — manifest registry vs orchestrator docs (optional).
4. **Optional** — G-09–G-22, prototype lanes.

---

*Handoff:* truth split → [`COMPLETION_TRUTH.md`](COMPLETION_TRUTH.md) · pin hygiene → [`TRUTH_AUDIT_LOG.md`](TRUTH_AUDIT_LOG.md) · ceilings → [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) · per-todo → [`TODO_COMPLETION.md`](TODO_COMPLETION.md).

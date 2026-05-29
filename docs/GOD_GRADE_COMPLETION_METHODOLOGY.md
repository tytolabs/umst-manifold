# God-grade completion methodology

**As of:** 2026-05-22  
**Audience:** Agents and coordinators closing UMST god-grade work without inflating percentages.  
**Cites:** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) · [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) · [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)

**Companions:** [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) · [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md) · [`scripts/verify_umst_stack.sh`](../scripts/verify_umst_stack.sh)

---

## 1. Why categories sit at ~90–98% (and must not be rounded to 100%)

A single headline percentage is only honest when its **numerator, denominator, and question** are named. UMST deliberately uses **three ceilings** ([`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)) so that “almost done” layers do not collapse into one false **100%**.

| Symptom | Root cause | Honest label |
|---------|------------|--------------|
| **Manifest ~90%**, **Cartridges ~80%**, **CI ~92%**, **Prototypes ~85%** | Category % blends **on-disk code**, **exit-0 tests**, and **ops gaps** (W8, thin-delete, optional workflows) in one number | Illustrative rollup — not automation |
| **God-grade weighted ~91–98%** | R5 rung splits: **100% in-repo** vs **0% org remote**; R6 had optional tail / horizon items | Use row from headline table — do not average categories |
| **Hot-path ~26%** beside **automation 100%** | **18/69** modules hand-wired on gate path; **119/119** digest pinned in CI | **26%** = operational wiring; **100%** = R0 pin — different questions |
| Stale docs still say G.2/G.3 open | Evidence moved; narrative lagged | Re-run verify; update SSOT — do not re-open closed rows from memory |

**Anti-patterns (lying to 100%):**

- Reporting **119/119** as “100% hot-path proofs” (pin ≠ runtime law).
- Reporting **17/17 automation** as “scoped god-grade 100%” (W8, B3, FFI remain outside denominator per [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)).
- Equating **U_op(t)** with checklist % or **U_pin** (see §5).
- Citing **69** as production `module_count` (rollback ratio only — live pin is **119** per [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)).

**Rule:** A category at **~95%** means “implementation and local tests are strong; named blocker or optional lane remains.” **Done** for that category requires the **closure criterion** in §4 — not rounding.

---

## 2. Methodology — Blocker → Evidence → Done

Each scoped item is one **morphism** in the completion category. Partial evidence does not compose to Done.

```
Blocker ──evidence──▶ Done
         (exit 0 tests, files on disk, operator sign-off)
```

| Stage | Meaning | Agent rule |
|-------|---------|------------|
| **Blocker** | Named gap that blocks a specific ceiling claim | Quote ID (B1 W8, B2 FFI, B3 strict default, or category gap) |
| **Evidence** | Reproducible proof today | Command + exit code + artifact path; timestamp in UTC |
| **Done** | Criterion in §4 or [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) satisfied | Human steps (git push, product default) need operator — **no `git push` by agents** |

**Scoped true 100%** ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)): **3/4** Done morphisms — **B1** W8 (**G-01** + **G-02**) ✅ · **B3** strict ✅ · **B2** FFI horizon; **G-03** supercap optional. **17/17** automation rows are **Done** and **outside** scoped blockers.

---

## 3. Three ceilings (never multiply)

From [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) and [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md):

| Ceiling | Question | Honest headline (2026-05-22) |
|---------|----------|------------------------------|
| **Automation** | Are in-repo CI rows green? | **17/17 = 100%** (16-row checklist + companion cartridges; all invoked in `verify_umst_stack.sh` tail) |
| **Hot-path catalog** | What share of Lean modules are runtime-wired on the gate path? | **18/69 ≈ 26%** (primary ratio) · **18/119 ≈ 15%** (unified) — **by design** |
| **Scoped true 100%** | Are B1–B3 Done morphisms closed? | **~96–98%** — **G-03** (optional) + **FFI** only |

**Org W8** is a fourth lens (**2/3** @ **fe22437** + **G-02**) — same bucket as **B1**, not added to automation %.

---

## 4. Test-backed closure only

**Closure law:** A category or checklist row is **Done** only when a named command returns **exit 0** (or a human Done table in [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) is signed off).

**Master bundle (recursive root):**

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT="${UMST_FORMAL_ROOT:-$PWD/../umst-formal-double-slit}"
bash scripts/verify_umst_stack.sh
```

**Recursive verify_umst_stack:** The script is the **composition** of child verifications — order matters for failure localization:

1. `cargo check` (default features)
2. Lean export regen vs `artifacts/catalog.lock.json` (digest **0697014f…**, **119** modules)
3. `bidirectional_catalog_check.sh` (embedded)
4. Default `cargo test -p umst-manifold`
5. Gate parity, Kleisli, dual-run, formal-witness, ROS
6. Epistemic G.2 / G.3, regime allowlist, witness priority, incremental graph, `ci_god_grade_profile`
7. Adversarial golden, catalog partition, lock-119 test
8. `verify_umst_stack: OK`

Re-running after any edit **re-validates the whole stack** — do not claim closure from a single sub-test in isolation unless the SSOT row explicitly allows it.

**Pin assert (R0 quick check):**

```bash
python3 -c "import json; l=json.load(open('umst-manifold/artifacts/catalog.lock.json')); assert l['module_count']==119 and l['upstream_catalog_digest_hex'].startswith('0697014f')"
```

---

## 5. Math — U_op vs U_pin (one section)

Let \(\mathcal{C}\) be the **119** Lean modules in the pinned export. Full detail: [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md).

**Pin coverage (static, CI):**

\[
U_{\mathrm{pin}} = \frac{|\{ m \in \mathcal{C} : \text{in lock digest}\}|}{|\mathcal{C}|} = 1
\]

After `export_catalog.py` + drift CI, every module is fingerprinted — this is **R0**, not “the robot proved every lemma.”

**Operational coverage (dynamic, deployment):**

\[
U_{\mathrm{op}}(t) = \frac{\sum_{m \in \mathcal{C}} w_m \cdot \mathbf{1}[\text{operational cover}(m,t)]}{\sum_{m \in \mathcal{C}} w_m}
\]

Operational cover means a slug in \(\mathrm{id}(m)\) fired on the hot path (accept/reject) in window \((t-T,t]\), or reject telemetry attributed — **not** digest-only registration.

**Static v1 headline (not \(U_{\mathrm{op}}(t)\)):** **18** hand-wired modules / **69** primary fiber ≈ **26%**; same **18** / **119** unified ≈ **15%**.

| Symbol | Typical value | Use in comms |
|--------|---------------|--------------|
| \(U_{\mathrm{pin}}\) | **1** | “Production catalog pin closed” |
| \(U_{\mathrm{op}}(t)\) | grows with rollout | “Which witness to wire next” |
| **18/69** | **~26%** | Intentional v1 hot-path scope — **not** a safety blocker |

**Never** set \(U_{\mathrm{op}} := U_{\mathrm{pin}}\) or multiply \(U_{\mathrm{op}}\) by god-grade **~91%** weighted scores.

---

## 6. Near-100% categories → exact closure criterion

Map each illustrative **~90–98%** row to **file / test / command**. Until the “Done when” column is satisfied, keep the honest % — do not round.

### Plan categories ([`GOD_GRADE_LAYER_MATRIX.md`](GOD_GRADE_LAYER_MATRIX.md))

| Category | ~% | Done when (all exit 0 unless noted) | Primary artifact / test |
|----------|-----|-------------------------------------|-------------------------|
| **Proofs** | 100 | Cross-repo export matches lock; claims doc present | `export_catalog.py` + `artifacts/catalog.lock.json`; `docs/claims-vs-proofs.md`, `docs/TCB.md` |
| **Gates** | 100 | Kleisli, reject slugs, adversarial, dual-run in stack | `tests/gate_kleisli.rs` **6/6**; `tests/gate_reject_catalog_id.rs` **6/6**; `tests/gate_adversarial.rs` FNR=0; `tests/gate_dual_run_parity.rs` **8/8** |
| **Manifest** | ~90→100 in-repo | Strict witness + embodied orchestrator green | `cargo test --features formal-witness --test manifest_strict_witness`; `tests/embodied_orchestrator.rs`; `src/manifest/umst_manifest.rs` |
| **Manifest** | scoped gap | **B1 W8** + **B3** prod default | [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md); `UmstManifestBuilder::default()` → `StrictCatalogMatch` (human) |
| **Cartridges** | ~80 | Local bridge + anchors | `cargo test -p umst-concrete-cartridge --features manifest-bridge`; `cargo test -p umst-supercap-cartridge --test formal_anchors` **6/6** |
| **Cartridges** | →100 org | Remote GHA without `[patch]` | `git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main` + cartridge `rust.yml` green |
| **CI** | ~92→100 automation | Full stack verify | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh`; MaOS `umst-catalog-drift.yml` |
| **CI** | optional | Standalone `rust.yml` gate lane | `.github/workflows/rust.yml` — polish only |
| **Prototypes** | ~85 | Dual-run parity + v1 tests | `gate_dual_run_parity` **8/8**; `umst-prototype/.../thermodynamic_filter.rs` tests **5/5** |
| **Prototypes** | →100 thin | 2a hybrid delete (Track B) | Line count / functor port per [`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md) — **not** gate-law |
| **Formal fibers** | 100 | v2 dual-pin **119** composed digest | `artifacts/catalog.lock.json` `version: 2`, `fiber_pins`; stack export step |

### Witness rungs R0–R6

| Rung | ~% | Done when | Command / file |
|------|-----|-----------|----------------|
| **R0** | 100 | Lock **119**, partition **4/4**, incremental graph | `catalog_all_ids_registered` **4/4**; `catalog_incremental_graph_drift`; `catalog_lock_119` |
| **R1** | 100 | CD / second law parity | `gate_dual_run_parity`, `gate_reject_catalog_id` |
| **R2** | 100 | Landauer CBF + G.3 calibration | `gate_cbf_parity` / `formal_witness`; `cargo test --features trace-calibration --test trace_calibration` **8/8** |
| **R3** | 100 | Mix / constitutive | `gate_parity_fixture`, mix registry tests |
| **R4** | 100 | Kleisli | `gate_kleisli` **6/6** |
| **R5** | 100 in-repo · **G-02** remote ✅ | Strict witness + concrete git pin | `manifest_strict_witness` **3/3**; **B1** G-01/G-02 done @ **fe22437** |
| **R6** | ~98→100 | G.1–G.3 in verify tail | `epistemic_trace_schema` **13/13**; `trace_calibration` **8/8**; `regime_soundness_claims_allowlist` **1/1** |

### Checklist automation rows (17/17)

Each row in [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) § Automation criteria maps 1:1 to a step in `verify_umst_stack.sh` or the quick-verify block in [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md). Row **Done** = that test passed in the **last** full stack run (record UTC in [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md)).

### Scoped blockers B1–B3 ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md))

| ID | Blocker | Done criterion (human/code) |
|----|---------|----------------------------|
| **B1** | W8 publish | **G-01** + **G-02** ✅ @ **fe22437** / **6742fa3**; **G-03** optional per [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **B2** | FFI extracted witnesses | Separate program + attestation API — **excluded** from v1 automation |
| **B3** | Strict prod default | `UmstManifestBuilder::default()` → `StrictCatalogMatch` + lock digest; Track **H.1** |

---

## 7. Plain-English decision tree

1. **Which question are you answering?** Pick automation, hot-path, scoped 100%, or org W8 — one row from §3.
2. **Is `verify_umst_stack.sh` exit 0?** If no, stop — fix stack first ([`VERIFY.md`](VERIFY.md)).
3. **Is the gap in B1–B3?** If yes, Evidence ≠ Done until runbook/human sign-off.
4. **Is the gap optional (prototype thin-delete, clippy, `rust.yml`)?** Label **0% scoped blocker** per [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md).
5. **Reporting coverage?** State **119** for pin, **~26%** for v1 hot-path, **U_op(t)** only for rollout planning.

---

## 8. Principles for other agents (5 bullets)

1. **Name the ceiling before the percent** — automation (17/17), hot-path (18/69), scoped Done (B1–B3), or org W8; never blend ([`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)).
2. **Blocker → Evidence → Done** — partial local tests are Evidence only; W8 and prod strict default require human Done ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)).
3. **Test-backed closure only** — cite `bash scripts/verify_umst_stack.sh` exit **0** and the specific `cargo test --test …` row; re-run recursively after edits.
4. **Do not conflate pin with hot-path** — \(U_{\mathrm{pin}}=1\) on **119** modules does not imply \(U_{\mathrm{op}}=1\); cite **26%** only for intentional v1 wiring ([`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md), [`ADAPTIVE_WITNESS_COVERAGE.md`](ADAPTIVE_WITNESS_COVERAGE.md)).
5. **Anti double-count** — org W8 (~8–10% scoped headline) is not additive with automation 100%; optional gaps (G-09–G-25) are **0%** scoped safety debt ([`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)).

---

## Cross-links

| Document | Role |
|----------|------|
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | Gap register, honest ceilings, G-01–G-26 |
| [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) | B1–B3 morphisms, W8 steps |
| [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) | Three ceilings SSOT |
| [`TODO_COMPLETION.md`](TODO_COMPLETION.md) | Per–plan-todo evidence |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | Verified timestamps |

*Methodology version:* 2026-05-22 · *Stack reference:* `verify_umst_stack.sh` @ 2026-05-21T22:09:30Z exit **0**

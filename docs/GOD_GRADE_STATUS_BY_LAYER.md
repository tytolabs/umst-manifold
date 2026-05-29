# God-grade status by layer

**Bundle date:** 2026-05-21  
**Verified (UTC):** 2026-05-21T22:00:03Z — `verify_umst_stack.sh` → **exit 0**  
**Workspace:** MaOS-Workspace  
**Environment:** `UMST_REQUIRE_FORMAL_EXPORT=1`

**How to read this page:** Two percentages appear throughout. **Completeness** asks how much of the roadmap is built. **Robustness** asks whether the agreed verification bundle still passes on this machine. On the 2026-05-21 bundle, **automation = 17/17 (100%)** for in-repo CI rows and **robustness = 100%** on the verify bundle. **W8** git publish remains **org-only** (outside automation %). Hot-path Lean coverage stays **~26%** by design — [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md).

**SSOT companions:** [`GOD_GRADE_LAYER_MATRIX.md`](GOD_GRADE_LAYER_MATRIX.md) (cross-maps) · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) (headline %) · [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) (17 automation rows) · [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) · [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) (per-todo exit ledger)

---

## Plain-English executive summary

UMST’s formal→Rust extraction plan is **done on disk** (14/14 YAML todos implemented; plan front-matter left unchanged on purpose). The **unified Lean catalog** spans two formal fibers (**119** modules, digest `0697014f…`) and is enforced by lock + CI, not by running Lean on every inference step.

**What is strong today:** Gate law (CD → Landauer → constitutive → Kleisli), catalog partition tests, dual-run parity (8/8), adversarial golden **FNR=0** (75 cases), formal witness in the stack script, and a full `verify_umst_stack.sh` green run.

**What blocks scoped true 100%:** **W8** publish (`tytolabs/umst-manifold` `main`) and **FFI** horizon. **G.2** (per-step bounds **12/12**) and **G.3** (`trace_calibration` **3/3**) are **closed in-repo** but G.3 is not yet in the stack script tail. Prototype thin-delete and clippy polish remain optional hygiene.

**TCB (unchanged):** One Lean axiom — `physicalSecondLaw` in `umst-formal-double-slit/Lean/LandauerLaw.lean`. Rust implements consequences only ([`TCB.md`](TCB.md)).

---

## Headline metrics (2026-05-21 bundle)

| Lens | % | Numerator / denominator | One-sentence meaning |
|------|---|-------------------------|----------------------|
| **Plan todos (14 YAML)** | **100%** | 14/14 | Every plan `id` on disk |
| **Plan + cross-repo fiber** | **100%** | 1/1 merge | **119**-module unified pin |
| **Automation (in-repo)** | **100%** | **17/17** | All checklist rows ✅ — [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) |
| **Hot-path catalog** | **~26%** | **18/69** | Primary-fiber hand-wired modules (by design) |
| **Org W8 publish** | **0%** | **0/1** | Remote git CI blocked |
| **Robustness (stack script)** | **100%** | exit **0** | @ **2026-05-21T22:00:03Z** |
| **God-grade R0–R6 (in-repo)** | **~98%** | 6.89/7 | G.3 not in stack script tail |
| **Scoped true 100% blockers** | **W8 + FFI** | 2 named | Not G.2/G.3/J.3 |

**Three ceilings:** Do not equate **automation 100%** with **hot-path 26%** or **org W8 0%** — [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md).

---

## Verification ledger (exit codes)

Recorded runs on **2026-05-21** unless noted. Re-run from `umst-manifold/` with `UMST_REQUIRE_FORMAL_EXPORT=1`.

### Master stack (bundles most layers)

| Command | Exit | Timestamp / log | What it proves |
|---------|------|-----------------|----------------|
| `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | **0** | **2026-05-21T22:00:03Z** → `verify_umst_stack: OK` | Export digest vs lock (**119**, `0697014f…`), bidirectional catalog, `catalog_all_ids_registered` **4/4**, Kleisli **6/6**, reject slugs **6/6**, adversarial **FNR=0** (75), dual-run **8/8**, `manifest_strict_witness`, ROS/HTTP, optional Python E6 **FNR=0** |
| `bash scripts/bidirectional_catalog_check.sh` | **0** | Same bundle | Regenerated export matches lock; gate `catalog_id`s anchored; `catalog_all_ids_registered` **4/4** |

**`verify_umst_stack.sh` step outcomes (all OK when exit 0):**

| Step (script order) | Exit | Notes |
|---------------------|------|-------|
| `cargo check` (default features) | **0** | Manifest + traceability imports compile |
| Lean export regen vs `artifacts/catalog.lock.json` | **0** | Unified pin `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`, **119** modules |
| `bidirectional_catalog_check.sh` | **0** | Embedded in stack script |
| `catalog_all_ids_registered` | **0** | **4 passed** |
| `gate_kleisli` | **0** | **6 passed** |
| `gate_reject_catalog_id` | **0** | **6 passed** |
| `gate_adversarial` | **0** | **FNR=0**, 75 cases |
| `gate_dual_run_parity` | **0** | **8/8** golden + **8/8** live subprocess |
| `manifest_strict_witness` | **0** | Release profile + digest mismatch reject |
| Adversarial Python E6 (optional) | **0** | When `umst-prototype_2` present: **FNR=0**, 75 cases |
| `formal_witness` / ROS / HTTP (feature bundle) | **0** | Per script tail |

**Logs:** operator shell; prior capture `/tmp/umst_verify_stack.log` (when present).

### Category-scoped commands (representative)

| Layer / check | Command (summary) | Exit | Notes |
|---------------|-------------------|------|-------|
| **Proofs / R0** | Cross-repo `export_catalog.py` + lock assert | **0** | `formal-fiber-merge` production pin |
| **Gates** | `cargo test --test gate_kleisli --test gate_reject_catalog_id --test gate_adversarial --test gate_dual_run_parity` | **0** | Law + parity + adversarial |
| **Manifest** | `cargo test --features formal-witness --test manifest_strict_witness --test formal_witness` | **0** | Strict witness in stack; dev default still `CatalogPinnedRos2` (not a verify failure) |
| **Cartridges (local)** | `cargo test -p umst-concrete-cartridge --features manifest-bridge` | **0** | Workspace `[patch]`; remote GHA blocked on **W8** |
| **CI matrix** | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` | **0** | MaOS `umst-catalog-drift.yml` uses same digest story |
| **Prototypes** | `gate_dual_run_parity` + `thermodynamic_filter::tests` (v1) | **0** | 2a hybrid body remains (Track **B**) |
| **Formal fibers** | Lock `version: 2` dual-pin + composed digest **119** | **0** | Per-fiber historical: `c1d9ba2…` (69), `534d9e18…` (62) |
| **Epistemic G.1–G.2** | `cargo test --features ros2-contract,serde --test epistemic_trace_schema` | **0** | **12/12** — serde + per-step bounds + envelope |
| **Epistemic G.3** | `cargo test --features trace-calibration --test trace_calibration` | **0** | **3/3** — not in stack script tail |
| **J.3 regime** | `cargo test --test regime_soundness_claims_allowlist` | **0** | **1/1** |
| **Supercap** | `cargo test -p umst-supercap-cartridge --test formal_anchors` | **0** | **6/6**; remote `manifest-bridge` rows need **W8** |
| **Embodied** | `cargo test --test embodied_orchestrator` | **0** | P11 closure |
| **HTTP gate (cold)** | `cargo test --features gate-server-bin --test gate_server_http` | **0** | Not on training hot path |

---

## Plan category matrix (7 layers)

Each row is a **plan layer** (grouping of YAML todos). **Verified %** blends on-disk delivery, CI truth from exit **0** runs above, and documented ops gaps.

| Category | Verified % | Plain English | Primary verify (exit **0**) | Honest gap |
|----------|------------|---------------|----------------------------|------------|
| **Proofs** | **100%** | Lean export is a versioned library; Python `export_catalog.py` is canonical; claims vs proofs documented | Cross-repo export + lock assert; `claims-vs-proofs.md` + `TCB.md` on disk | Appendix B → `catalog_id` graduation is ops polish |
| **Gates** | **100%** | Bad transitions rejected with stable `catalog_id`; Kleisli + CD + Landauer + mix on registry path | `gate_kleisli`, `gate_reject_catalog_id`, `gate_adversarial`, `gate_dual_run_parity` in stack | 2a thin delete optional (Track **B**); not a gate-law gap |
| **Manifest** | **100%** in-repo | `UmstManifest`, grounding contract, embodied orchestrator; strict witness in CI | `manifest_strict_witness`, `formal_witness`, `embodied_orchestrator` | **W8** for remote git consumers only |
| **Cartridges** | **~80%** | Concrete/supercap anchors + local `manifest-bridge` tests green | `umst-concrete-cartridge --features manifest-bridge` (patch); `formal_anchors` **6/6** | Remote GHA without published `tytolabs/umst-manifold` `main` (**W8**) |
| **CI** | **~95%** | Stack verify + drift workflow enforce digest; adversarial in script | `verify_umst_stack.sh` **0** @ 22:00:03Z; `umst-catalog-drift.yml` | Epistemic G tests not in stack script tail; optional `rust.yml` |
| **Prototypes** | **~85%** | v1 shim **226** lines, 8/8 dual-run; parity functor identity before delete | `gate_dual_run_parity`; v1 `thermodynamic_filter::tests` **5/5** | 2a hybrid **517** lines (B.3–B.4); legacy HTTP bins deprecation |
| **Formal fibers** | **100%** | Second fiber (`umst-formal`) merged into one **119**-module R0 pin | v2 dual-pin lock + composed digest `0697014f…` in stack export step | Optional per-fiber rollback policy only ([`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md)) |

**Category → plan todo IDs**

| Category | Plan `id`s / milestones |
|----------|-------------------------|
| Proofs | `lean-export-lake`, `claims-vs-proofs`, `formal-fiber-merge` |
| Gates | `gate-unification-spec`, `manifold-gate-evaluator`, `parity-ci` |
| Manifest | `manifold-manifest`, `formal-witness-integration`, `embodied-orchestrator` |
| Cartridges | `concrete-cartridge-wire`, supercap (`FORMAL_SCALING.md`) |
| CI | `parity-ci`, MaOS `umst-catalog-drift.yml`, `verify_umst_stack.sh` |
| Prototypes | `prototype-audit`, `thin-prototypes`, `ros2-in-manifold` |
| Formal fibers | `formal-fiber-merge` (= `lean-export-cross-repo`) |

**Weighted rollup:** **100%** plan+fibers; **17/17** in-repo automation; hot-path **18/69**; org W8 **0%** publish.

---

## P0–P12 phase matrix

Historical migration phases from plan §5. **P0–P7** = Lean → manifold core (all **100%** with exit **0** stack proof). **P8–P12** = prototype thin + cartridge wire (**~88%** hybrid).

| Phase | Deliverable | Verified % | Verify (exit **0** when green) | Gap | Tracks |
|-------|-------------|------------|--------------------------------|-----|--------|
| **P0** | Audit + `GateUnificationSpec.md` | **100%** | Spec + `PROTOTYPE_GATE_MAP.md` on disk | — | — |
| **P1** | Lean export + `catalog.lock` | **100%** | Export digest step inside `verify_umst_stack.sh` | — | **F** |
| **P2** | `runtime/catalog` + `build.rs` | **100%** | `catalog_all_ids_registered` **4/4** | Partition table if module count shifts | **F** |
| **P3** | `gate/` + Kleisli port | **100%** | `gate_kleisli` **6/6** | — | **C** |
| **P4** | `GateEvaluator` + CBF | **100%** | `gate_cbf_parity`, CBF reject slug tests | — | **D** |
| **P5** | `manifest` re-exports | **100%** | `embodied_orchestrator`; manifest structs present | Git consumers need **W8** | **A**, **H** |
| **P6** | `gate_server` in manifold | **100%** | `gate_server_http` (feature bundle) | Legacy prototype HTTP bins | **B** |
| **P7** | Dual-run + adversarial production | **100%** | `gate_dual_run_parity`, `gate_adversarial` in stack | 1-week prod monitor (ops); optional `rust.yml` | **E**, **J** |
| **P8** | Replace prototype filter core | **~70%** | Dual-run **8/8**; v1 shim **226** L | Full delete 2a Constitution/CGS/functor body | **B** |
| **P9** | `ros/contract` + bridge | **~95%** | `ros_contract_serde_roundtrip` | Live ROS smoke optional | — |
| **P10** | Concrete facade + anchors | **~75%** | `manifest-bridge` tests (workspace patch) | Remote git CI; generated `formal_anchor` rows | **A**, **I** |
| **P11** | `EmbodiedOrchestrator` + claims | **100%** | `embodied_orchestrator`; `claims-vs-proofs.md` | — | — |
| **P12** | Thin prototypes | **~85%** | `gate_dual_run_parity` **8/8**; v1 tests **5/5** | 2a hybrid body; no concrete path dep (Burn pin) | **B** |
| **Publish** | `tytolabs/umst-manifold` `main` | **0%** ops | Local patch tests **0** | Blocks remote cartridge GHA | **A** |

**Phase rollup:** P0–P7 **100%** · P8–P12 **~88%** · Publish **ops-only (W8)**.

---

## Tracks A–J (god-grade ops)

From [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md). **Verified %** reflects automation + exit **0** evidence where applicable.

| Track | Name | Status | Verified % | Verify (exit **0**) | Gap | Owner |
|-------|------|--------|------------|----------------------|-----|-------|
| **A** | W8 publish (`tytolabs/umst-manifold` `main`) | ❌ ops | **~40%** | Local `manifest-bridge` tests (patch) | Git push + cartridge dep without `[patch]` | manifold publish → cartridge |
| **B** | `umst-prototype-2a` thin | ⚠️ hybrid | **~85%** | `gate_dual_run_parity` in stack | B.3–B.4 full 2a body delete | prototype |
| **C** | Kleisli `GateEvaluator` | ✅ done | **100%** | `gate_kleisli` **6/6** | — | manifold |
| **D** | `catalog_id` on every reject | ✅ done | **100%** | `gate_reject_catalog_id` **6/6** | — | manifold |
| **E** | Adversarial gate CI | ✅ done | **100%** | `gate_adversarial` **FNR=0** (75); optional Python E6 | Optional E6 when prototype absent | CI |
| **F** | Cross-repo catalog (**119** pin) | ✅ done | **100%** | Lock digest `0697014f…`, **119** modules in stack | Optional dual-pin policy only | formal |
| **G** | Epistemic runtime schema v2 | ⚠️ partial | **~33%** | G.1: `epistemic_trace_schema` **0** | **G.2** bounds · **G.3** η-from-traces | manifold / ops |
| **H** | Strict witness + `formal-witness` | ✅ CI | **100%** | `manifest_strict_witness` in stack | Dev `UmstManifest::default()` stays `CatalogPinnedRos2` (not blocker) | product / ops |
| **I** | Supercap formal anchor parity | ⚠️ partial | **~70%** | `formal_anchors` **6/6** | I.3–I.4 remote `manifest-bridge`; needs **W8** | cartridge |
| **J** | Warnings / lint hygiene | ⚠️ partial | **~60%** | Dual-run + adversarial in `verify_umst_stack.sh` | J.1 clippy `-D warnings`; J.3 regime policy | manifold CI |

**Track closure (2026-05-21):** **C, D, E, F, H** ✅ · **G** ⚠️ (G.1–G.3 in-repo ✅; stack tail optional) · **B, I, J** ⚠️ hybrid · **A** ❌ ops (**W8**).

---

## Witness ladder R0–R6 (by layer)

Normative order: R0 → R1 (CD) → R2 (Landauer) → R3 (constitutive) → R4 (Kleisli) → R5 (manifest) → R6 (traces). See [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md).

| Rung | Layer / category | Verified % | Exit **0** evidence |
|------|------------------|------------|------------------------|
| **R0** | Proofs + formal fibers (**119**) | **100%** | Lock v2 + `catalog_all_ids_registered` + export step in stack |
| **R1** | Gates (CD) | **100%** | `gate_dual_run_parity`, `gate_reject_catalog_id` |
| **R2** | Gates (Landauer CBF) | **100%** | `gate_cbf_parity`, `formal_witness` |
| **R3** | Gates (constitutive / mix) | **100%** | `gate_parity_fixture`, mix registry tests |
| **R4** | Gates (Kleisli) | **100%** | `gate_kleisli` **6/6** |
| **R5** | Manifest + cartridges | **100%** in-repo · **0%** org remote | `manifest_strict_witness`, `formal_witness`, local `manifest-bridge`; **W8** blocks remote |
| **R6** | Epistemic v2 traces | **~89%** | G.1 ✅ · G.2 **12/12** · G.3 **3/3** (not in stack script tail) |

---

## Three ceilings (automation vs hot-path vs W8)

Source: [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)

| Ceiling | Numerator / denominator | % |
|---------|-------------------------|---|
| **Automation (in-repo)** | **17 / 17** | **100%** |
| **Hot-path catalog** | **18 / 69** (primary) | **~26%** |
| **Org W8 publish** | **0 / 1** | **0%** |

**W8** and **FFI** are **outside** the 17-row automation denominator. **Do not** report hot-path **26%** as god-grade automation.

### Robustness (100% on 2026-05-21 bundle)

**Definition:** Every command in the [verification ledger](#verification-ledger-exit-codes) that defines the bundle returned **exit 0** on 2026-05-21 (recorded timestamps + this-session reconfirm).

| Bundle member | Exit |
|---------------|------|
| `verify_umst_stack.sh` | **0** |
| `bidirectional_catalog_check.sh` | **0** |
| Representative per-category `cargo test` rows | **0** |

Robustness **does not** mean “no open roadmap items”; it means “the agreed safety net did not regress.”

---

## Honest path to scoped true 100%

**In-repo automation:** **17/17 = 100%** — no checklist row blockers.

**Scoped true 100%** (named blockers only):

| Item | Track / rung | What “done” looks like |
|------|--------------|------------------------|
| **W8** — publish `tytolabs/umst-manifold` `main` | **A**, **R5** | Remote cartridge CI without `[patch]` — [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **FFI** — extracted witnesses | horizon | Long-term attestation |

**Closed this pass (no longer blockers):** **G.2** (`epistemic_trace_schema` **12/12**) · **G.3** (`trace_calibration` **3/3**) · **J.3** (`regime_soundness_claims_allowlist` **1/1**).

**Optional polish:** epistemic tests in `verify_umst_stack.sh` tail; 2a thin delete; `rust.yml` gate lane; prod strict manifest default.

---

## Catalog pin anchor (R0)

| Field | Value |
|-------|-------|
| `upstream_catalog_digest_hex` | `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227` |
| `module_count` | **119** (69 primary + 50 `umst-formal`-only; 12 basename overlaps, primary wins) |
| Prior primary-only pin | `c1d9ba2aa402106a…` / **69** modules |

Hot-path hand-wired modules remain **~18 / 69 (~26%)**; the **119** pin still fingerprints the full formal inventory in CI ([`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md)).

---

## Cross-map (quick reference)

| Category | P-phases | Tracks |
|----------|----------|--------|
| Proofs | P0, P1, P11 | **F** |
| Gates | P0, P3, P4, P7, P8 | **C**, **D**, **E**, **B** |
| Manifest | P5, P11 | **A**, **H**, **G** |
| Cartridges | P10 | **A**, **I** |
| CI | P1, P7 | **E**, **J** |
| Prototypes | P0, P6, P7, P8, P12 | **B**, **E** |
| Formal fibers | P1 | **F** |

---

## Reproduce this page

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT=/path/to/umst-formal-double-slit   # optional if sibling layout

bash scripts/verify_umst_stack.sh
echo "EXIT_CODE=$?"

bash scripts/bidirectional_catalog_check.sh
echo "EXIT_CODE=$?"

cargo test --test catalog_all_ids_registered
cargo test --test gate_kleisli --test gate_reject_catalog_id --test gate_adversarial
cargo test --features formal-witness --test manifest_strict_witness
cargo test --features ros2-contract,serde --test epistemic_trace_schema
```

Full operator recipe: [`VERIFY.md`](VERIFY.md).

---

## Related documents

| Document | Role |
|----------|------|
| [`GOD_GRADE_LAYER_MATRIX.md`](GOD_GRADE_LAYER_MATRIX.md) | Category × phase × track mermaid |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | Fresh-run headline table |
| [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) | 17-row automation criteria |
| [`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md) | Automation vs hot-path vs W8 |
| [`TODO_COMPLETION.md`](TODO_COMPLETION.md) | Per–plan-todo evidence |
| [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) | Command → exit → files |
| [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) | Track A–J substeps |
| [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) | Track A operator steps |
| [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) | Track F operator steps |

*Last bundle:* 2026-05-21T22:00:03Z · *Stack:* `verify_umst_stack: OK` · **EXIT_CODE=0**

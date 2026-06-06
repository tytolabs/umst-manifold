# Unfinished features audit

**As of:** 2026-05-21  
**Audience:** Anyone deciding what to do next vs what to leave alone  
**Preview/stub detail:** [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)  
**Evidence ledgers:** [`TODO_COMPLETION.md`](TODO_COMPLETION.md), [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md), [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md), [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md)

---

## How to read this document

| Column | Meaning |
|--------|---------|
| **Owner** | Who should drive the work (lane or role) |
| **Execute** | Safe and useful to do **now** without blocking other lanes |
| **Wait** | Blocked on human sign-off, publish credentials, or an upstream merge |

**Plain-language status:**

- **Done on disk** — The 14 plan YAML todos are implemented locally; `verify_umst_stack.sh` was green @ 2026-05-21T21:18Z.
- **Not done** — Mostly **ops** (git publish, policy defaults) and **one formal promotion** (second Lean repo into the catalog pin). No new gate crate scaffolding is required for v1.

---

## Executive summary

| Bucket | Count (approx.) | Headline |
|--------|-----------------|----------|
| Preview-only | 0 major (merge closed) | Unified digest **0697014f…** / **119** modules — preview workflow demoted to dev-only |
| Ops-only | 2–4 | **G-03** supercap remote (optional), prototype thin-delete, optional `rust.yml` |
| Hybrid / optional | 2 | 2a thin delete, Python E6 adversarial (Rust `gate_adversarial` required in `rust.yml`) |
| Long horizon | 1 | Extracted witnesses / FFI |

**Recommendation (preview SSOT):** Cross-repo merge is closed; preview is dev-only triage. **2026-05-21 fix:** exporter always emits `dry_run: true` on preview JSON; stale duplicates removed; regen at `umst-formal-double-slit/artifacts/catalog-cross-repo-preview.json`. Details: [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) § 2026-05-21 fix.

---

## Preview and formal-lane items

These exist as **scaffolds** or **dry-runs**. They do not change the production catalog pin until a human approves merge policy.

| Item | What it is (plain English) | Owner | Execute vs wait | Notes |
|------|----------------------------|-------|-----------------|-------|
| **Cross-repo catalog merge** | Unified `catalog.json` (**119** modules) | formal / coordinator | ✅ **Done** 2026-05-21 | Digest `0697014f…`; see [`TODO_COMPLETION.md`](TODO_COMPLETION.md) § `formal-fiber-merge` |
| **Formal fiber merge** | Same milestone as above: one digest for R0, manifold lock, cartridge anchors | formal / coordinator | ✅ **Done** 2026-05-21 | Track F ✅ — [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) |
| **Appendix B (`umst-formal`)** | Docs trace classical lemmas; graduation ops post-merge | docs / formal | **Execute** — doc refresh optional | [`claims-vs-proofs.md`](claims-vs-proofs.md) |
| **Lean-export-cross-repo alias** | Tracker name in [`TODO_COMPLETION.md`](TODO_COMPLETION.md) — not a plan YAML id | coordinator | ✅ **Done** | Alias of `formal-fiber-merge` |

**Dev-only (no longer operator SSOT):** `--cross-repo-only` preview JSON for local triage — production pin is unified `0697014f…` / **119** modules per [`EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md).

---

## Ops-only items (no new Rust scaffolding)

| Item | What blocks “god-grade 100%” | Owner | Execute vs wait | Unblocks |
|------|------------------------------|-------|-----------------|----------|
| **W8 — Publish manifold `main`** | — | — | **Done** 2026-05-29 @ **`fe22437`** on `tytolabs/umst-manifold` `main` | Track A — [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) |
| **W8 — Cartridge CI feature** | — | cartridge maintainers | **Done** (**G-02**) — GHA `manifest-bridge` on git pin, no `[patch]` | [`FORMAL_GROUNDING_AUDIT.md`](../umst-concrete-cartridge/docs/FORMAL_GROUNDING_AUDIT.md) |
| **Strict catalog default (H)** | — | product / ops | **Done** — `StrictCatalogMatch` release default (B3) | Track H — [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) |
| **Formal-witness auto-digest (H)** | Digest compare works but callers must set `Some` manually | manifold | **Execute** — small wiring; no new axiom | [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) §6 |
| **Epistemic v2 traces (G)** | — | manifold / ops | **Done** — G.1–G.3 in `verify_umst_stack.sh` | Track G |
| **W10 — `rust.yml` verify lane** | ✅ Required `verify-umst-stack` job (parity subset + optional full stack); drift workflow unchanged | `umst-manifold` CI | **Done** 2026-05-21 | See [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) |
| **W10-b — Lean PR → export bot** | Manual `make lean-catalog-export` + lock bump | formal / coordinator | **Execute** when ready | Reduces human slip on digest |
| **Bidirectional script hygiene** | `bidirectional_catalog_check.sh` once failed on `GATE_REGISTRY` doc-comment parse (`umst.cartridge.concrete.policy`) | manifold CI | **Execute** — script fix | Re-green full stack verify per [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) |
| **Doc hygiene** | Stale Kleisli / parity rows in `claims-vs-proofs.md` | docs | **Execute** — doc-only | Aligns checklist truth |

---

## Prototype and parity (optional / hybrid)

| Item | State | Owner | Execute vs wait |
|------|-------|-------|-----------------|
| **v1 prototype shim** | ~226 lines; 8/8 dual-run ✅ | prototype lane | **Wait** — already plan-complete |
| **2a hybrid body** | ~517 lines; Algorithm 1 delegates with `manifold-gate`; Constitution/CGS/MARL local | prototype lane | **Wait** until manifold ports or HTTP-only policy |
| **2a full thin delete (B.3–B.4)** | Optional sign-off; not plan-blocking | prototype lane | **Wait** on B.1 ports |
| **Legacy `gate_dual_fixture` subprocess** | Retire when all callers use manifold `:8787` | prototype lane | **Wait** on HTTP-only migration |
| **Python E6 adversarial** | FNR=0 when prototype checkout present; Rust is SSOT | CI / coordinator | **Execute** to wire in `rust.yml` **or** **Wait** and accept Rust-only |
| **Legacy prototype `gate_server` bins** | ROS telemetry / OCR — separate from manifold SSOT | prototype lane | **Wait** — deprecation track |

See [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md), [`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md).

---

## Cartridge and supercap (partial)

| Item | State | Owner | Execute vs wait |
|------|-------|-------|-----------------|
| **Concrete `manifest-bridge`** | ✅ git **`fe22437`**, GHA without `[patch]` | cartridge | **Done** (**G-02**) |
| **Catalog-generated `formal_anchor`** | Still `lean://` URIs | cartridge | **Wait** on cross-repo merge + optional codegen |
| **Supercap `formal_anchors`** | 6/6 tests ✅ | supercap | **G-03** optional — remote `manifest-bridge` (I.3) |
| **Supercap generated anchors (I.4)** | Optional | cartridge | **Wait** |

---

## Runtime stubs and automation gaps

Not “missing features” in the product sense — documented gaps between Lean inventory and Rust hot path. Full table: [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) § Runtime stubs.

| Gap | Plain English | Owner | Execute vs wait |
|-----|---------------|-------|-----------------|
| **51/69 catalog-only modules** | In digest; not on inference hot path by design | formal / manifold | **Wait** — wire per roadmap, not bulk |
| **Manifest `GateRegistry` inert** | Declared lanes not used for execution | manifold | **Execute** only if product needs dynamic registry |
| **MI surrogate vs `EpistemicMI`** | CBF uses MSE on `info_gain` | manifold / research | **Wait** on v2 traces + calibration (G.3) |
| **Extracted witnesses / FFI** | No Lean terms on hot path | long horizon | **Wait** |

---

## God-grade tracks still open (A–J)

Quick map to [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md). Tracks **C, D, E** (Rust adversarial) are closed.

| Track | Status | Owner | Execute vs wait |
|-------|--------|-------|-----------------|
| **A — W8 publish** | ✅ **G-01** · ✅ **G-02** · **G-03** optional | operator (G-03 only) | **G-03** supercap remote when scheduled |
| **B — 2a thin** | ⚠️ hybrid | prototype | **Wait** on ports |
| **F — cross-repo catalog** | ✅ closed | formal / coordinator | — |
| **G — epistemic v2** | ✅ closed | manifold | G.1–G.3 in `verify_umst_stack.sh` |
| **H — strict default** | ✅ closed | product / ops | `StrictCatalogMatch` release default (B3) |
| **I — supercap bridge** | ⚠️ partial | cartridge | **G-03** remote `manifest-bridge` optional |
| **J — clippy / warnings** | ⚠️ partial | manifold CI | **Execute** J.1 when CI time allows |

**Suggested order (dependencies):** **G-03** supercap remote (optional) → **B** / **I** / **J** polish as capacity allows. **A/H/G/F** closed 2026-05-29.

---

## What is finished (do not re-open without cause)

| Area | Verdict |
|------|---------|
| 14 plan YAML todos | ✅ on disk — [`TODO_COMPLETION.md`](TODO_COMPLETION.md) |
| R0 lock + **119**-module partition | ✅ |
| R1 CD, R2 Landauer CBF, R3 mix, R4 Kleisli | ✅ |
| `gate_dual_run_parity` 8/8, `gate_adversarial` FNR=0 (Rust) | ✅ |
| `gate_reject_catalog_id` 6/6 | ✅ |
| Local `manifest-bridge` + embodied orchestrator | ✅ |
| Swarm audit docs (six files) | ✅ |

---

## SSOT after merge (closed 2026-05-21)

**formal-fiber-merge** is closed (unified `catalog.json`, digest `0697014f…`, **119** modules).

| Done | Item |
|------|------|
| ✅ | Preview `dry_run` always `true` in exporter; tests in `test_export_catalog_cross_repo.py` |
| ✅ | Regenerated `umst-formal-double-slit/artifacts/catalog-cross-repo-preview.json`; removed stale copies at workspace `artifacts/` and `umst-manifold/artifacts/` |
| ✅ | [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) updated — preview section shrunk |

**Still operator hygiene (doc-only):** Demote `--cross-repo-only` in runbooks where it reads as primary workflow; canonical command remains `make lean-catalog-export`.

**Keep regression tests** — `test_export_catalog_cross_repo.py` is not the primary human checklist.

---

## Related documents

| Document | Use when |
|----------|----------|
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | Listing preview JSON, schema stubs, runtime gaps |
| [`TODO_COMPLETION.md`](TODO_COMPLETION.md) | Per-todo evidence commands |
| [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) | Track A–J substeps |
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | Ops-only blocker table |
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | Promotion phases 0–4 |
| [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) | 10/13 criteria ticks |

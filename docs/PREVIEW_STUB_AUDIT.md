# Preview and stub audit

**As of:** 2026-05-21  
**Audience:** Coordinators deciding what is safe to run vs what only previews future work  
**Companion:** [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md) (owners, execute vs wait)

This file lists **preview artifacts** and **stubs** that exist on disk but are **not** production SSOT. Nothing here should be treated as the canonical catalog pin unless explicitly promoted.

### 2026-05-21 — `rust.yml` verify-umst-stack required (parity CI)

| Action | Detail |
|--------|--------|
| **Choice** | Promoted `verify-umst-stack` in `umst-manifold/.github/workflows/rust.yml` (did **not** re-open preview `dry_run` — already `true` in exporter + SSOT JSON). |
| **Behavior** | Removed `continue-on-error`; job id `verify-umst-stack` (was `verify-umst-stack-optional`). With formal sibling / `UMST_FORMAL_ROOT`: `scripts/verify_umst_stack.sh`. Without Lean export: same Rust gate subset as stack script (includes `gate_adversarial` FNR=0, `gate_reject_catalog_id`, Kleisli, dual-run, formal-witness, gate-server). |
| **Independence** | Modular formal fibers: PRs green without checking out `umst-formal-double-slit`; full export path unchanged in `umst-catalog-drift.yml`. |
| **Local test** | `cargo test` parity subset (gate_* + adversarial) — all passed 2026-05-21. |

---

### 2026-05-21 — preview `dry_run` fix (closed)

| Action | Detail |
|--------|--------|
| **Bug** | Stale `catalog-cross-repo-preview.json` had `dry_run: false` when generated under `APPROVE_CROSS_REPO_MERGE=1` (e.g. `verify_umst_stack.sh`), contradicting “preview never pins.” |
| **Code** | `export_catalog.py` — preview JSON always sets `dry_run: true`; merge approval is only `approve_cross_repo_merge_set` / `merge_blocked`. |
| **Tests** | `test_export_catalog_cross_repo.py` — `test_preview_always_dry_run_even_when_approved`, `test_cross_repo_only_dry_run_with_approve_env`. |
| **Regen** | `umst-formal-double-slit/artifacts/catalog-cross-repo-preview.json` via `--cross-repo-only` (119 modules, `merged_digest_hex` `0697014f…`). |
| **Removed** | Duplicate stale copies at workspace `artifacts/` and `umst-manifold/artifacts/` (not SSOT locations). |

---

## Preview artifacts (dry-run / non-pinning)

| Artifact | Location | What it previews | Touches production pin? |
|----------|----------|------------------|-------------------------|
| Cross-repo catalog merge | `umst-formal-double-slit/artifacts/catalog.json` (unified) | **119** modules; digest `0697014f…` | ✅ promoted 2026-05-21 (`APPROVE_CROSS_REPO_MERGE=1` env) |
| Exporter `--cross-repo-only` | `tools/lean_export/export_catalog.py` | Writes preview JSON; skips `catalog.json` / `catalog.lock.json` | **No** — preview always `dry_run: true` (2026-05-21) |
| Secondary digest in preview | `secondary_digest_hex`, `merged_digest_hex` in preview JSON | Triage only; unified pin is `catalog.json` digest `0697014f…` | **No** |

**Regenerate (safe):**

```bash
cd umst-formal-double-slit
python3 tools/lean_export/export_catalog.py \
  --lean-root Lean \
  --also-lean-root ../umst-formal/Lean \
  --also-lean-repo-tag umst-formal \
  --cross-repo-only
```

**Production pin today:** `catalog_digest_hex` = `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`, `module_count` = **119** (`cross_repo_merge: true`) — see [`TODO_COMPLETION.md`](TODO_COMPLETION.md) § `formal-fiber-merge` · [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md).

---

## Schema and doc stubs (validators only)

| Stub | Location | Role | Production? |
|------|----------|------|-------------|
| `catalog.schema.json` | `umst-manifold/artifacts/` | JSON Schema for tooling / CI validators | Pin is **lock JSON**, not schema alone |
| `PROOF-STATUS.md` (supercap) | `umst-supercap-cartridge/docs/` | Five-status blocks on `pub` API | Doc witness; not generated from `catalog_id` rows yet |
| Appendix B rows | [`claims-vs-proofs.md`](claims-vs-proofs.md) | Hand-aligned `umst-formal` fiber — in **119**-module unified export; appendix is traceability narrative | Merge closed 2026-05-21 |

---

## Runtime / API stubs (behavioral gaps)

| Surface | What works today | Stub / preview behavior |
|---------|------------------|-------------------------|
| `formal-witness` digest | `FormalReject::CatalogSchemaDigestMismatch` when both digests set | `catalog_schema_digest` **not** auto-filled from `UMST_CATALOG_LOCK_SHA256_HEX` ([`COMPOSITIONAL_INFERENCE_AUDIT.md`](COMPOSITIONAL_INFERENCE_AUDIT.md) §6) |
| `UmstManifest::gate_registry` | `declared_lanes: Vec<String>` for telemetry | **Does not execute** gates — registry-first routing is in `EmbodiedOrchestrator` + evaluators |
| `ClausiusDuhemProof` | Marker type on `VerifiedUMST` | Empty trait — no extracted Lean proof term at runtime |
| `info_gain` in gateway | CBF scalar budget | MSE surrogate, not full `EpistemicMI` semantics |
| Cartridge `formal_anchor` | `lean://` / `empirical://` URIs in PROOF-STATUS | Not yet `catalog_id` slugs from export ([`TODO_COMPLETION.md`](TODO_COMPLETION.md) § concrete-cartridge-wire) |
| Epistemic v2 | Lean contracts proved; G.1 serde roundtrip ✅ | Per-step bounds + η-from-traces still open (G.2–G.3) |
| 2a-only gates | Constitution, CGS, MARL joint functor, `max_strength` | Stay in `umst-prototype-2a` until manifold ports ([`PROTOTYPE_2A_HOST_GAPS.md`](PROTOTYPE_2A_HOST_GAPS.md)) |

---

## Lean export scaffolds (formal repo)

| Name | In export? | Notes |
|------|------------|-------|
| `FlashMoERuntimeScaffold` | Listed in catalog | Runtime spec stub — **no** manifold hook ([`EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md)) |
| Test modules (`Test3`, `Test4`, …) | In unified **119**-module digest | CI/formal hygiene; exclude from merge policy per alignment doc |

---

## SSOT cleanup after cross-repo merge (Phase 4 ✅)

Unified catalog promotion closed 2026-05-21 (`0697014f…`, **119** modules). Remaining operator hygiene:

1. **Preview path is dev-only** — `--cross-repo-only` + `umst-formal-double-slit/artifacts/catalog-cross-repo-preview.json` for local triage; canonical export is `make lean-catalog-export` → `catalog.json` / `catalog.lock.json`.
2. **Single digest** in `umst-manifold/artifacts/catalog.lock.json` — do not duplicate preview under manifold `artifacts/`.
3. **Keep** `APPROVE_CROSS_REPO_MERGE` + exporter tests (`test_export_catalog_cross_repo.py`) as regression history.
4. **`dry_run` semantics** — preview JSON always `dry_run: true`; `approve_cross_repo_merge_set` records whether a full export *may* write unified `catalog.json` (not this file).

Treat preview JSON as **read-only triage**, not a dependency pin.

---

## Related documents

| Document | Role |
|----------|------|
| [`UNFINISHED_FEATURES_AUDIT.md`](UNFINISHED_FEATURES_AUDIT.md) | Master list: owner + execute vs wait |
| [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) | Operator steps for promotion |
| [`UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md) | Merge policy for 50 secondary-only modules |
| [`TCB.md`](TCB.md) | Preview must not change axiom count |

# Formal fiber merge runbook — unified Lean export (Track F)

**Scope:** Promote `umst-formal` classical lemmas into the **single** canonical `umst-formal-double-slit/artifacts/catalog.json` pin consumed by `umst-manifold` — after coordinator review, without breaking the **`physicalSecondLaw`-only** TCB ([`TCB.md`](TCB.md)).

**Witness ladder:** [R0 — Catalog lock](GOD_GRADE_WITNESS_LADDER.md#r0--catalog-lock-build-time-functor), [§ Second catalog fiber](GOD_GRADE_WITNESS_LADDER.md#4-umst-formal-as-second-catalog-fiber)

**Status (2026-05-21):** ✅ **CLOSED** — unified digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`, `module_count: 119`; manifold lock aligned; `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` exit 0.

**Roadmap:** Track **F** ✅ in [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md#track-f--unified-lean-export-cross-repo-catalog) · verified [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md)

**Approval:** `APPROVE_CROSS_REPO_MERGE=1` environment variable only (no repo-root marker file). `make lean-catalog-export` emits unified catalog when `../umst-formal/Lean` exists.

---

## Canonical production command

```bash
cd umst-formal-double-slit
APPROVE_CROSS_REPO_MERGE=1 python3 tools/lean_export/export_catalog.py \
  --lean-root Lean \
  --also-lean-root ../umst-formal/Lean \
  --also-lean-repo-tag umst-formal
```

Writes `artifacts/catalog.json` + `artifacts/catalog.lock.json` with `cross_repo_merge: true`. **Dev-only preview** (no pin write): add `--cross-repo-only`.

**Recorded unified pin (2026-05-21):** digest `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`, **119** modules (primary **69** + **50** secondary-only; overlap **12**, primary wins). Details: [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md) § *Last production merge*.

---

## Why merge (benefits)

### Manifold (`umst-manifold`)

- **Single R0 digest** — One `upstream_catalog_digest_hex` for drift CI, `build.rs`, and `formal-witness`; no split-brain between double-slit export and informal second-fiber knowledge.
- **Traceability closure** — Appendix B rows in [`claims-vs-proofs.md`](claims-vs-proofs.md) can move to main table with stable `catalog_id`s after wiring review.
- **Gate registry completeness** — Formal-only modules (`DIBKleisli`, constitutional/economic families) become catalog-visible before optional `GateEvaluator` ports.
- **verify_umst_stack truth** — `UMST_REQUIRE_FORMAL_EXPORT=1` exercises the promoted library revision end-to-end.

### Concrete cartridge (`umst-concrete-cartridge`)

- **Aligned `lean://` inventory** — Mechanised anchors (`Powers`, `Gate`, hydration) share the same catalog generation as manifold’s lock.
- **Manifest / strict pin** — Cartridge CI can correlate cartridge git pin with one formal digest that includes classical Powers/constitutional lemmas used in profiles.
- **Unchanged TCB token** — `formal_axioms: physicalSecondLaw` and profile `axioms = ["physicalSecondLaw"]` stay valid; merge adds modules, not axioms.

---

## Prerequisites

- [x] [`TCB.md`](TCB.md) policy reviewed; merge adds modules, not axioms.
- [x] `umst-formal-double-slit` and `umst-formal` siblings present.
- [x] Unified export run (`APPROVE_CROSS_REPO_MERGE=1` command above).
- [x] Manifold `artifacts/catalog.lock.json` bumped to unified digest + `module_count: 119`.

---

## Phase 0 — TCB gate (before merge)

- [x] `rg '^axiom ' Lean/LandauerLaw.lean` → **single** `physicalSecondLaw`
- [x] Secondary scan — same axiom name only in vendored lineage
- [x] Merge policy in [`UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md) §7–9

**Fail closed:** If grep shows >1 project axiom or a new second-law axiom in secondary scan, **stop** — fix Lean first ([`TCB.md`](TCB.md)).

---

## Phase 1 — Human review (roadmap F.1)

- [x] Inventory `only_in_secondary_basename` (**50** modules) — classical/DEC/Economic families
- [x] Basename overlap **12** — primary wins (`Gate`, `LandauerLaw`, …)
- [x] Signed merge policy in alignment doc

---

## Phase 2 — Regenerate unified catalog (roadmap F.2) ✅

- [x] Production export command (see top of runbook)
- [x] `artifacts/catalog.json` + `catalog.lock.json` at digest `0697014f…`, `module_count: 119`
- [x] **TCB:** `physicalSecondLaw` only

```bash
cd umst-formal-double-slit
python3 -c "
import json
c=json.load(open('artifacts/catalog.json'))
l=json.load(open('artifacts/catalog.lock.json'))
assert c.get('cross_repo_merge') is True
assert len(c['modules'])==119
assert l['catalog_digest_hex']==c['digest']=='0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227'
print('unified catalog OK', len(c['modules']))
"
rg '^axiom ' Lean/LandauerLaw.lean
```

---

## Phase 3 — Bump manifold lock + green stack (roadmap F.3) ✅

- [x] Update `umst-manifold/artifacts/catalog.lock.json`:
  - `upstream_catalog_digest_hex`: `0697014fb5b90a3aca4db3e5cc226896ca198802c910d5395f254e4262aa6227`
  - `module_count`: `119`
- [x] Refresh `CATALOG_MODULE_WIRED` / `ALLOW_UNUSED_CATALOG_IDS` as needed
- [x] Update [`claims-vs-proofs.md`](claims-vs-proofs.md) rows for newly wired modules
- [ ] Run master verify:

```bash
cd umst-manifold
python3 -c "import json; l=json.load(open('artifacts/catalog.lock.json')); f=json.load(open('../umst-formal-double-slit/artifacts/catalog.json')); assert l['upstream_catalog_digest_hex']==f['digest']; assert l['module_count']==len(f['modules'])"
export UMST_FORMAL_ROOT="${UMST_FORMAL_ROOT:-$(cd .. && pwd)/umst-formal-double-slit}"
export UMST_REQUIRE_FORMAL_EXPORT=1
bash scripts/verify_umst_stack.sh
cargo test --test catalog_all_ids_registered -p umst-manifold
cd ../umst-concrete-cartridge && cargo test -p umst-concrete-cartridge formal_anchors
```

- [x] Mark `lean-export-cross-repo` ✅ in [`TODO_COMPLETION.md`](TODO_COMPLETION.md)

**Done when:** Lock matches export; stack + partition tests green; cartridge axiom allowlist unchanged. ✅ 2026-05-21.

---

## Phase 4 — Close roadmap / ladder

- [x] Update [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) Track F row to ✅
- [ ] [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) §4 — note unified fiber (second fiber promoted into R0)
- [x] [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) — plan+cross-repo **~93% → 100%**
- [x] [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) — verified ledger + checklist %

---

## Rollback

If stack verify fails after lock bump:

1. Revert `catalog.lock.json` to prior `upstream_catalog_digest_hex` (`c1d9ba2…`, `module_count: 69`).
2. Revert formal `catalog.json` to primary-only export (`make lean-catalog-export` without `--also-lean-root`).
3. Re-run `verify_umst_stack.sh` on known-good digest.

Do **not** patch Rust gates to “fix” digest mismatch — fix export/lock pairing first (R0 functor).

---

## Related docs

| Doc | Role |
|-----|------|
| [`TCB.md`](TCB.md) | `physicalSecondLaw`-only policy + audit checklist |
| [`TODO_COMPLETION.md`](TODO_COMPLETION.md) § `formal-fiber-merge` | Milestone evidence |
| [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) | Verified % + reproduce commands |
| [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md) | Production merge + recorded digest |
| [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md) | Two-repo policy |
| [`DUAL_PIN_ARCHITECTURE.md`](DUAL_PIN_ARCHITECTURE.md) | Dual-pin recommendation + v2 lock schema |

# Trusted computing base (TCB) — UMST manifold layer

**Normative policy:** The project axiom closure is **`physicalSecondLaw` only** in Lean (`LandauerLaw.lean`). Rust implements consequences of that axiom on the hot path; it does **not** introduce parallel axioms, undocumented `register_axiom` hooks, or cartridge tokens outside `{NONE, physicalSecondLaw}`.

**Audit SSOT:** [`CATALOG_COVERAGE_AUDIT.md`](CATALOG_COVERAGE_AUDIT.md) (LandauerLaw row), [`COMPOSITIONAL_INFERENCE_AUDIT.md`](COMPOSITIONAL_INFERENCE_AUDIT.md) (L2–L3 CBF), [`claims-vs-proofs.md`](claims-vs-proofs.md) (traceability), [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) § [No new Lean axioms in Rust](GOD_GRADE_WITNESS_LADDER.md#no-new-lean-axioms-in-rust).

**Cross-repo fiber merge:** Operator phases and TCB gates at each promotion step — [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) (Track **F** in [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md)).

---

## Rust trusted boundary (implementation)

Components that enforce policy or materially bound physics fidelity in this lane:

| TCB boundary | Artefact | Role |
|--------------|----------|------|
| **Tensor policy gateway** | `src/ai/ppo.rs` | Separates device reductions vs scalar accounting; Landauer bit budget before MI surrogate |
| **Landauer / CD bookkeeping** | `src/ai/cbf.rs` — `ThermodynamicCBF` | Operational anchor for `physicalSecondLaw` **consequences** (inequalities, bit energy); not a duplicate Lean axiom |
| **Host thermodynamic gate** | `src/gate/thermo_transition.rs`, `mix_proposal.rs`, `http_manifest.rs` | CD / admissibility before CBF on composed paths |
| **Catalog bundle digest** | `build.rs` + `artifacts/catalog.lock.json` (`UMST_CATALOG_LOCK_SHA256_HEX`) | R0 pin — versioned proof library fingerprint |
| **JSON schema stub** | `artifacts/catalog.schema.json` | Validators / CI |

**Explicitly excluded from TCB (bridges):** ROS 2 process bridges, ONNX / WASM runtimes inside prototypes, heavyweight ML optimisers mounted above the gateway trait surface.

**Hand-aligned (not TCB):** Rows in `claims-vs-proofs.md` marked **hand-aligned** mirror theorem families; parity tests and dual-run prove alignment without enlarging the axiom closure.

**Proved-only (not TCB):** Quantum / complementarity / DPI modules in the unified export (primary fiber **69** in dual-pin) — digest pin or inventory only; no runtime enforcement claim.

---

## `physicalSecondLaw`-only policy

### Lean (versioned library)

| Rule | Detail |
|------|--------|
| **Single project axiom** | Exactly one `axiom` line in primary export: `physicalSecondLaw` in `umst-formal-double-slit/Lean/LandauerLaw.lean`. |
| **Shared with classical fiber** | Vendored `LandauerLaw.lean` in `umst-formal` uses the same axiom name; cross-repo merge must **not** add a second second-law axiom or rename the TCB token. |
| **Strengthening path** | Any new assumption → prove or axiomatize in Lean first → `make lean-catalog-export` → bump `catalog.lock.json` → hand-align Rust. Never the reverse. |
| **Fiber merge** | Unified export (Track F) may enlarge `module_count` and digest; axiom count stays **1**. See runbook Phase 0 TCB gate. |

### Rust (manifold hot path)

| Allowed | Forbidden |
|---------|-----------|
| **Witness predicates** — inequalities and admissibility checks aligned to theorem **families** | New **axioms**, `const` “we assume …” physics, or silent gate weakening |
| **TCB implementation** of Landauer/CBF bookkeeping in `cbf.rs` / `ppo.rs` | Duplicating `physicalSecondLaw` as a Rust axiom or `register_axiom` |
| **`catalog_id` + `Proof:`** citations in comments/docs | Private axioms in `src/ai/cbf.rs` or `src/gate/` without Lean + lock bump |
| **`info_gain` surrogate** only **post** `ThermodynamicCBF` (witness order R1→R2) | MI surrogate as standalone certificate bypassing CBF |

**Catalog row:** `LandauerLaw` → `umst.gate.landauer_cbf` → `src/ai/cbf.rs` — status **TCB** in `claims-vs-proofs.md`; audit note: document `physicalSecondLaw` as TCB boundary, **not** as extracted proof term at runtime.

### Cartridge / supercap anchors

| Surface | Allowlist |
|---------|-----------|
| `formal_axioms` doc lines | **`NONE`** or **`physicalSecondLaw`** only (`umst-concrete-cartridge/tests/formal_anchors.rs`) |
| Profile TOML / `result.v2` `axioms` | Schema allowlist includes `physicalSecondLaw`; no ad-hoc tokens without schema bump |
| `lean://` citations | Mechanised rows cite `umst-formal` lemmas; axiom closure still defers to Lean `LandauerLaw` |

---

## TCB audit checklist (CI / coordinator)

Run after any change touching `cbf.rs`, `ppo.rs`, gate stack, `catalog.lock.json`, Lean `LandauerLaw.lean`, or cross-repo export:

```bash
# 1 — Lean axiom count (primary fiber)
cd umst-formal-double-slit/Lean
lake build
rg '^axiom ' LandauerLaw.lean
# expect: single match physicalSecondLaw

# 2 — No Lean on manifold hot path
rg 'lake build|lean --run' umst-manifold/src
# expect: empty

# 3 — Lock digest + stack (when formal sibling present)
cd umst-manifold
export UMST_FORMAL_ROOT="${UMST_FORMAL_ROOT:-$(cd .. && pwd)/umst-formal-double-slit}"
export UMST_REQUIRE_FORMAL_EXPORT=1
bash scripts/verify_umst_stack.sh

# 4 — Cartridge axiom allowlist (if cartridge tree present)
cd umst-concrete-cartridge
cargo test -p umst-concrete-cartridge formal_anchors -- --nocapture
```

| Check | Pass criterion |
|-------|----------------|
| Lean `axiom` grep | One line: `physicalSecondLaw` |
| Manifold `src/` | No `lake` invocation on inference path |
| `verify_umst_stack.sh` | Exit 0 with `UMST_REQUIRE_FORMAL_EXPORT=1` |
| `formal_anchors` | Rejects `formal_axioms` ∉ `{NONE, physicalSecondLaw}` |
| Cross-repo preview | `dry_run: true` always on preview JSON — **no** manifold lock change (unified pin: `catalog.json`) |

**Not duplicated in manifold CI today:** formal-repo `print_axioms` / `scripts/check_lean_axioms.py` — policy is fixed here; formal lane runs Lake + grep on promotion.

---

## What changes the TCB vs what does not

| Change type | TCB impact |
|-------------|------------|
| New theorem + hand-aligned Rust witness | **No** axiom change; update `claims-vs-proofs.md` |
| Regenerated `catalog.json` / lock bump (same axiom) | **No** axiom change; R0 digest changes |
| Cross-repo module merge (Track F) | **No** axiom change if grep + merge review pass |
| New Lean `axiom` or Rust “axiom” constant | **TCB violation** — requires explicit governance + ladder review |
| `StrictCatalogMatch` / manifest-bridge | **No** new physics axiom; digest/orchestration only |
| Epistemic v2 trace schema | Witness envelope only; not a physics axiom |

---

## Related docs

| Doc | Use |
|-----|-----|
| [`FORMAL_FIBER_MERGE_RUNBOOK.md`](FORMAL_FIBER_MERGE_RUNBOOK.md) | Track F operator steps + per-phase TCB gates |
| [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) | Global invariants table; Track F |
| [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) | R5 publish; Phase 0 includes TCB grep |
| [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md) | Two-repo fiber policy |

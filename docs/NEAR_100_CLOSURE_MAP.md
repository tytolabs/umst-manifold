# Near-100% closure map

**As of:** 2026-05-29  
**SSOT for %:** [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md) category rollup  
**Methodology:** [`GOD_GRADE_COMPLETION_METHODOLOGY.md`](GOD_GRADE_COMPLETION_METHODOLOGY.md) · scoped blockers [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)

This page lists the five plan categories that sit between **85% and 99%** (not rounded to 100%). Each row names what **Done** means, how to prove it, who owns the last mile, and whether it blocks **scoped true 100%** (**G-03** optional + **FFI** horizon — see [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)).

---

## Blocker → Evidence → Done (short)

Each gap is one step: **Blocker** (what is missing) → **Evidence** (what is green today) → **Done** (criterion met).

Partial Evidence (local tests, files on disk) does **not** equal Done when a human step or product policy is still open. Full rules: [`GOD_GRADE_COMPLETION_METHODOLOGY.md`](GOD_GRADE_COMPLETION_METHODOLOGY.md) §2.

| Stage | Plain meaning |
|-------|----------------|
| **Blocker** | Named item still blocking a honest “100%” claim for this category |
| **Evidence** | Command returns exit **0** (or artifact on disk); record UTC in [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md) |
| **Done** | Criterion in the tables below — operator sign-off where noted |

**Do not** mix ceilings: automation **16/16**, hot-path **~26%**, and scoped **G-03 + FFI** are different questions ([`GOD_GRADE_AUTOMATION_CEILING.md`](GOD_GRADE_AUTOMATION_CEILING.md)).

---

## Per-category closure (85–99%)

### Manifest — **~90%** ([`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md))

| Field | Value |
|-------|--------|
| **Blocker** | ~~Release default strict catalog match (**G-04**, **B3**)~~ — **Done** @ **2026-05-22** |
| **Evidence today** | `UMST_RELEASE_MANIFEST_PROFILE=1` → `default()` strict; `lock_upstream_catalog_digest_bytes()` auto-fill (**G-05**); `manifest_strict_witness` **4/4** · `verify_umst_stack.sh` exit **0** |
| **Done criterion** | ✅ `UmstManifestBuilder::default()` strict when profile **1**; upstream digest from `catalog.lock.json` on gateway + strict `build()` |
| **Test command** | `cd umst-manifold && cargo test --features formal-witness --test manifest_strict_witness --test formal_witness` · full stack: `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| **Owner** | **Product / ops** (default flip); **manifold** for **G-05** auto-digest polish |
| **Blocks scoped 100%?** | **No** (B3/G-04/G-05 closed in-repo). W8 affects remote git consumers only |

**Blocker → Evidence → Done:** B3 policy open → strict tests exit **0** → default builder + deployment manifests strict without opt-in.

---

### Cartridges — **~95%** (local **100%** · concrete remote **100%** · supercap remote **0/1** — [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md))

| Field | Value |
|-------|--------|
| **Blocker** | **G-03** only — supercap remote `manifest-bridge` in GHA (**optional**) |
| **Evidence today** | **G-01** publish @ **fe22437**; **G-02** concrete GHA without `[patch]`; supercap `formal_anchors` **6/6** local; MaOS `[patch]` patch-green **Evidence** for dev |
| **Done criterion** | **G-01**/**G-02** ✅; optional **G-03**: wire supercap GHA per [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) Track **I.3** |
| **Test command** | Concrete: `cargo test -p umst-concrete-cartridge --features manifest-bridge` on git **fe22437** (no patch) · Supercap: `cargo test -p umst-supercap-cartridge --test formal_anchors` |
| **Owner** | **Human** (**G-03** only) |
| **Blocks scoped 100%?** | **G-03 only** (~2% optional). **G-02** does **not** block |

**Blocker → Evidence → Done:** **G-03** open → concrete git bridge green → optional supercap remote GHA.

---

### Epistemic — **~98%** weighted · **100%** host CI ([`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md))

| Field | Value |
|-------|--------|
| **Blocker (host)** | **None** — G.1–G.3 closed in stack |
| **Blocker (optional / horizon)** | Lean utility certificates (`NumericTraceApproxConsistent`, rollout-approx witness) deferred on checklist rows 14–15; optional PPO `information_density` reward wire |
| **Evidence today** | `epistemic_trace_schema` **13/13** · `trace_calibration` **8/8** · both in `verify_umst_stack.sh` tail |
| **Done criterion (in-repo)** | Last full stack run exit **0** with epistemic steps green (already met @ 2026-05-21T22:09:30Z) |
| **Done criterion (full weighted R6)** | Lean utility certs + optional live η reward — **formal lane** / optional code |
| **Test command** | `cargo test --features ros2-contract,serde --test epistemic_trace_schema` · `cargo test --features trace-calibration --test trace_calibration` · `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| **Owner** | **Manifold** (host — done); **formal lane** (Lean certs); **code** optional (PPO wire) |
| **Blocks scoped 100%?** | **No** for scoped v1 (**G-07**, **G-08** closed). Deferred Lean rows are **not** failing automation |

**Blocker → Evidence → Done:** Host rows → 13/13 + 8/8 → optional Lean morphisms (horizon, not scoped blocker).

---

### Prototypes — **~85%** ([`GOD_GRADE_STATUS_BY_LAYER.md`](GOD_GRADE_STATUS_BY_LAYER.md) · P12 — not a separate % row in [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md))

| Field | Value |
|-------|--------|
| **Blocker** | v1 shim **226** lines + 2a hybrid **517** lines remain (Track **B** B.3–B.4); legacy HTTP `gate_server` deprecation optional |
| **Evidence today** | Dual-run **8/8** in stack; v1 `thermodynamic_filter` tests **5/5** |
| **Done criterion** | Track **B** closure: 2a body deleted or ported; parity functor identity preserved; optional line-count targets in [`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md) |
| **Test command** | `cd umst-manifold && cargo test --test gate_dual_run_parity` · `cd umst-prototype/src/rust/core && cargo test thermodynamic_filter::tests --lib` |
| **Owner** | **Prototype lane** |
| **Blocks scoped 100%?** | **No** — hygiene / comms only (**G-09**–**G-25**). Gate law lives in manifold |

**Blocker → Evidence → Done:** Hybrid 2a body → 8/8 parity → thin delete (optional for v1 scoped 100%).

---

### Org — **~96–98%** weighted ([`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md) “weighted witness incl. org W8”)

| Field | Value |
|-------|--------|
| **Blocker** | **G-03** supercap remote bridge only (**optional**) |
| **Evidence today** | Publish **1/1** @ **fe22437**; concrete bridge **1/1** without `[patch]`; in-repo R5 **100%** |
| **Done criterion** | **G-01**/**G-02** ✅; optional **G-03** in supercap GHA; docs free of stale `W8 ❌` / “unpublished `main`” |
| **Test command** | `git ls-remote https://github.com/tytolabs/umst-manifold.git refs/heads/main` · `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` |
| **Owner** | **Human** (**G-03** only). **Agents must not `git push`.** |
| **Blocks scoped 100%?** | **G-03 only** (~2% optional). Do **not** add org % + automation % |

**Blocker → Evidence → Done:** **G-01**/**G-02** closed @ **fe22437** → optional **G-03** supercap remote.

---

## Summary table

| Category | Current % | One step to 100% | Blocks scoped 100%? |
|----------|-------------|------------------|---------------------|
| **Manifest** | **~100%** in-repo | ~~B3/G-04/G-05~~ closed @ **2026-05-22** (`verify_umst_stack.sh` exit **0**) | **No** |
| **Cartridges** | **~95%** | Optional **G-03** supercap remote GHA | **G-03 only** (~2% optional) |
| **Epistemic** | **~98%** weighted · **100%** host | **Already 100%** for scoped host CI — optional Lean certs / PPO wire only | **No** (v1 scoped) |
| **Prototypes** | **~85%** | Prototype lane: finish Track **B** thin delete (2a hybrid) after parity | **No** |
| **Org** | **~96–98%** weighted | Optional **G-03** supercap remote | **G-03 only** (optional) |

### Cannot reach 100% by design (do not chase in v1 scoped %)

| Category / lens | Why |
|-----------------|-----|
| **Hot path** (~26% primary) | Only **18/69** modules wired on robot path on purpose — not a near-100% category |
| **Epistemic — full Lean R6** | Utility certificates and rollout-approx witness are horizon / formal lane, not failing host rows |
| **FFI / extracted witnesses** | **B2** — long program; excluded from v1 automation ([`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md)) |
| **Prototypes — “zero lines”** | v1 keeps shim + 2a hybrid until ports complete; **100%** means parity + policy, not empty repos |

---

## Master verify (all categories)

```bash
cd umst-manifold
export UMST_REQUIRE_FORMAL_EXPORT=1
export UMST_FORMAL_ROOT="${UMST_FORMAL_ROOT:-$PWD/../umst-formal-double-slit}"
bash scripts/verify_umst_stack.sh
python3 -c "import json; l=json.load(open('artifacts/catalog.lock.json')); assert l['module_count']==119"
```

Re-run after any edit; cite exit **0** + UTC in [`GOD_GRADE_PROGRESS_VERIFIED.md`](GOD_GRADE_PROGRESS_VERIFIED.md).

---

## Cross-links

| Document | Role |
|----------|------|
| [`PROGRESS_PLAIN.md`](PROGRESS_PLAIN.md) | Category % SSOT |
| [`GOD_GRADE_COMPLETION_METHODOLOGY.md`](GOD_GRADE_COMPLETION_METHODOLOGY.md) | Blocker → Evidence → Done, ceilings, U_pin vs U_op |
| [`SCOPED_100_CLOSURE.md`](SCOPED_100_CLOSURE.md) | **B1** W8 · **B2** FFI · **B3** strict default |
| [`PENDING_GAPS_PLAIN.md`](PENDING_GAPS_PLAIN.md) | **G-01**–**G-26** gap register |
| [`W8_PUBLISH_RUNBOOK.md`](W8_PUBLISH_RUNBOOK.md) | Org / cartridge Done steps |

*Map version:* 2026-05-29 · Stack reference: `verify_umst_stack.sh` exit **0** @ **fe22437**; **G-02** concrete remote without `[patch]`

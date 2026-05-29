# What UMST extraction changed — for humans

**As of:** 2026-05-21  
**Audience:** Anyone who needs to know *why this work mattered* without reading the engineering ledgers first.

**Evidence (numbers in this doc):** [`TODO_COMPLETION.md`](TODO_COMPLETION.md), [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md), [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md)  
**Philosophy (why checks run in this order):** [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md)

---

## One-sentence summary

We turned scattered prototypes and informal “trust the math” notes into a **single, test-backed pipeline**: machine-checked proofs export to a locked inventory, the runtime enforces the same safety rules every time, and one command proves the whole stack still matches — so bad material transitions get **rejected automatically**, not caught by a human after the fact.

---

## Before and after

### Before (start of the extraction push)

| Area | What it felt like |
|------|-------------------|
| **Proofs vs runtime** | Lean proofs lived in one repo; gate code lived in prototypes. There was no stable “this is the proof set we ship today” fingerprint that CI could fail on. |
| **Gate math** | Thermodynamic filtering logic was **copied** in prototype Rust (~378 lines in the v1 filter alone). Manifold and prototype could drift apart silently. |
| **Catalog** | Export paths were unclear; regex-only or partial exports risked **missing modules** or **stale digests** without anyone noticing until behavior changed in production. |
| **Safety checks** | Rejects were inconsistent: some paths used ad hoc strings, some used scores instead of hard stops, and failure order (which rule wins when two fail) was not written down as law. |
| **Cartridges** | Concrete and supercap cartridges did not share one manifest + gate story with manifold; formal grounding was documentation, not a wired contract. |
| **Verification** | No single “green means we’re aligned” script spanning formal export, catalog IDs, gate parity, adversarial cases, ROS/HTTP contracts, and formal-witness. |
| **Plan tracking** | A 14-item plan existed on paper; most items were still marked pending even when code was already on disk — hard for coordinators to hand off. |

### After (now)

| Area | What changed |
|------|----------------|
| **Proofs as a library** | **69 Lean modules** export to `catalog.json` with a pinned digest (`c1d9ba2aa402…`). Formal and manifold locks **match**. Regenerating without updating the lock **fails CI**. |
| **One gate implementation** | Manifold `src/gate/` is the enforcement home. Prototype v1 filter is a **226-line shim** that delegates core mix math to manifold — not a second copy of the algorithm. |
| **Parity proof** | **8/8** golden vectors and **8/8** live subprocess checks agree: manifold vs prototype give the same admit/reject answers. |
| **Adversarial regression** | **75** pinned adversarial cases; **false negatives = 0** (nothing unsafe slipped through as “OK”). |
| **Witness order** | Rejects follow a fixed ladder: catalog pin → second-law (CD) → energy/information budget (Landauer) → mix/constitutive rules → probe composition. Higher-priority failures **stop** lower checks — documented in the [witness ladder](GOD_GRADE_WITNESS_LADDER.md). |
| **Traceability** | `claims-vs-proofs.md` + `TCB.md` map theorem families to stable `catalog_id` slugs and Rust locations — auditors can follow a row from claim to code. |
| **Stack verify** | `bash scripts/verify_umst_stack.sh` with formal export required → **exit 0 (green)** on the audited workspace (export digest, bidirectional catalog, gate tests, formal-witness, embodied **8/8**, adversarial golden, etc.). |
| **Plan clarity** | **12–13 of 14** plan todos are complete on disk; **2** remain honestly partial (CI polish, prototype thinning). Six swarm audit docs give per-area evidence without new scaffolding. |

The YAML plan file still lags on purpose (coordinator policy: don’t edit the plan file). **Disk and tests are ahead of the checkbox tracker** — use [`TODO_COMPLETION.md`](TODO_COMPLETION.md) as the handoff source of truth.

---

## What would have broken if we hadn’t done this

These are not hypotheticals; they are the failure modes the new pipeline was built to prevent.

1. **Silent catalog drift**  
   Someone merges Lean changes, forgets to regen export or bump the lock, and production still “works” while enforcing an **old** proof inventory. Downstream you get claims in papers or manifests that no longer match what the binary actually assumes.  
   *Now:* digest mismatch fails export check and drift CI.

2. **Gate false negatives (unsafe admitted)**  
   A refactor in prototype or manifold tweaks inequalities or ordering; unsafe mix transitions pass as admissible. In materials / robotics terms: you act on a state that violates thermodynamic or constitutive limits.  
   *Now:* dual-run **8/8** parity plus **75-case** adversarial suite with **FNR = 0** in CI.

3. **Duplicate math diverging**  
   Two copies of the thermodynamic filter evolve separately; operators see green tests in one repo and red behavior in another.  
   *Now:* v1 delegates to manifold; parity tests bind them. (v2 prototype still keeps a larger body **by design** until more ports land — see [`THIN_PROTOTYPE_STATUS.md`](../umst-prototype/docs/THIN_PROTOTYPE_STATUS.md).)

4. **Formal proofs disconnected from runtime**  
   Proofs exist in Lean but runtime invents new assumptions in Rust (“we’ll axiomatize later”). That enlarges the trusted computing base without review.  
   *Now:* policy is **no new Lean axioms in Rust**; second law stays in Lean; Rust implements witnesses aligned to the pinned catalog ([witness ladder § Proof library · gate law](GOD_GRADE_WITNESS_LADDER.md#proof-library--gate-law--mi-envelope--no-rust-axioms)).

5. **Wrong failure priority**  
   A cheap constitutive check runs before second-law check; system logs a “mix issue” when the real problem was thermodynamic inadmissibility — wrong telemetry, wrong fixes.  
   *Now:* [god-grade decision 1](GOD_GRADE_WITNESS_LADDER.md#1-failure-priority-cd--2nd-law--landauer--constitutive--probe): CD → Landauer → constitutive → probe, short-circuit.

6. **Cartridge / manifest mismatch in the wild**  
   Local dev passes with a workspace patch; published git CI still pins an old `umst-manifold` without manifest APIs — green laptop, red GitHub.  
   *Still a known gap (W8 publish):* local `manifest-bridge` is green; **remote** cartridge CI waits on publishing manifold to git. Documented, not hidden.

7. **Coordinator handoff chaos**  
   “Is lean-export done?” answered differently by every agent.  
   *Now:* per-todo evidence blocks, verification matrix, and this impact doc align on the same metrics.

---

## Who benefits

| Who | How |
|-----|-----|
| **Operators / on-call** | One green command (`verify_umst_stack.sh`) means export, catalog IDs, gates, parity, and contracts still line up — less archaeology before a release or incident. |
| **Safety & compliance reviewers** | Ordered witnesses + traceability table + TCB doc: you can see *which rule rejected* and *which proof family* it traces to, without reading all of Rust. |
| **Formal / proof engineers** | Proofs stay in Lean; export is versioned like a dependency. You promote a digest deliberately, not by accident at runtime. |
| **Runtime / gate engineers** | Single implementation in manifold; prototypes thin toward delegation; registry uses stable `catalog_id` slugs, not one-off strings. |
| **Cartridge teams (concrete, supercap)** | Manifest bridge and formal anchors tested locally; roadmap clear for git publish (W8). Supercap formal anchor tests **6/6** with lock hash advisory. |
| **CI / release coordinators** | Drift workflow + stack script; adversarial golden in drift CI; [`TODO_COMPLETION.md`](TODO_COMPLETION.md) as SSOT for the 14 plan items. |
| **Future agents / contributors** | Six audit docs + witness ladder + bidirectional alignment doc reduce re-discovery; failure priority is normative, not tribal knowledge. |

---

## Metrics you can cite (verified 2026-05-21)

| Metric | Result | Meaning in plain language |
|--------|--------|---------------------------|
| **Stack verify** | **Green (exit 0)** | `UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh` — full stack aligned on audited machine. |
| **Catalog lock** | **119 modules**, digest `0697014fb5b90a3a…` | Formal export and manifold lock agree; no drift at last audit. |
| **Plan todos (on disk)** | **12–13 / 14 complete** | ~**93%** infra; only `parity-ci` and `thin-prototypes` partial (plus future `lean-export-cross-repo`). |
| **Strict checklist rows** | **~70%** strict ✅ | “God-grade” automation headline **~84%** weighted — stricter human checklist still has open rows ([`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md)). |
| **Dual-run parity** | **8/8** golden + **8/8** live | Manifold and prototype agree on every fixture in the parity suite. |
| **Embodied orchestrator tests** | **8/8** | Host routing including Kleisli paths through the gate registry. |
| **Gate Kleisli tests** | **6/6** | Composition / unit evaluator behavior covered. |
| **Adversarial gate golden** | **75 cases, FNR = 0** | No unsafe case classified as safe in the pinned regression JSON. |
| **Catalog ID registration** | **4/4** tests pass | Every expected catalog partition registered for the **119**-module unified export. |
| **Prototype shim unit tests** | **5/5** | Delegating shim still behaves on its surface API. |
| **Supercap formal anchors** | **6/6** | Sibling cartridge parity with topology catalog hash advisory. |
| **Cartridge manifest-bridge (local)** | **Pass** with workspace patch | Remote git CI still blocked until manifold publish — not counted as “done everywhere.” |

**How to re-check (read-only):**

```bash
cd umst-manifold
UMST_REQUIRE_FORMAL_EXPORT=1 \
  UMST_FORMAL_ROOT=/path/to/umst-formal-double-slit \
  bash scripts/verify_umst_stack.sh
```

---

## Witness ladder philosophy (linked, in human terms)

The full normative doc is **[God-grade witness ladder](GOD_GRADE_WITNESS_LADDER.md)**. Here is the philosophy without the category-theory table.

### Four ideas that everything else hangs on

1. **Proofs = versioned library**  
   Lean produces an inventory + fingerprint. Runtime **imports that revision**; it does not re-run the prover on every step. Changing what is proved means: regen export → bump lock → green verify — like bumping a dependency major version.

2. **Gates = law**  
   Safety rules **reject** bad transitions. They are not hints or scores you can override in “advisory mode” if you want god-grade behavior.

3. **Information / MI only inside the energy envelope**  
   Surrogate “information gain” signals are allowed **after** the Landauer-style barrier check binds them — not as a standalone “trust this number” certificate.

4. **No new axioms in Rust**  
   If Rust needs a stronger assumption, it must appear in Lean and the catalog first. Rust does not quietly widen what we trust.

### The ladder (order matters)

Think of each step as a guard on the way out the door:

| Step | Plain name | What it guards |
|------|------------|----------------|
| **R0** | Catalog lock | “Are we still running against the proof inventory we signed?” |
| **R1** | Second law (CD) | “Is this thermodynamic transition admissible?” — **first** physics reject. |
| **R2** | Landauer / CBF | “Does this tensor update respect erasure / information cost budgets?” |
| **R3** | Constitutive / mix | “Is the mix/hydration/strength proposal physically closed?” |
| **R4** | Probe / Kleisli | “Is this composed probe policy admissible?” — lowest priority among gate-family rejects. |
| **R5** | Manifest / digest | “Does the deployed cartridge manifest match the manifold lock?” (v1 digest; v2 trace schema still open). |
| **R6** | Trace schema (future) | “Do emitted runtime traces match the proved epistemic schema?” |

**Short-circuit rule:** If R1 fires, we do not “also check” R3 for politeness — we reject and stop. That is [decision 1](GOD_GRADE_WITNESS_LADDER.md#1-failure-priority-cd--2nd-law--landauer--constitutive--probe) in the ladder doc. It matches how operators should read logs: the **first** failed witness is the root cause class.

This extraction push **implemented and tested** R0–R4 and much of R5 locally; R6 and strict default-on manifest in all CI environments remain on the [roadmap](PENDING_GOD_GRADE_ROADMAP.md).

---

## What is honestly still open

Credibility requires saying what is **not** finished:

- **Plan YAML** still shows many `pending` items — use on-disk verdicts in [`TODO_COMPLETION.md`](TODO_COMPLETION.md), not YAML alone.
- **`parity-ci`** — adversarial Rust golden is in drift CI; optional Python E6 and a dedicated `rust.yml` verify lane remain nice-to-haves.
- **`thin-prototypes`** — v1 is a shim with **8/8** parity; **umst-prototype-2a** keeps a larger filter body until Constitution/DCS ports land.
- **W8 git publish** — remote cartridge CI cannot consume `manifest-bridge` until manifold is published to git without local `[patch]`.
- **Cross-repo catalog** — preview merge of a second Lean repo exists; not promoted to the canonical lock yet.
- **God-grade checklist** — weighted **~84%**; strict “every row green” is lower — see [`GOD_GRADE_CHECKLIST.md`](GOD_GRADE_CHECKLIST.md) and roadmap tracks.

None of those open items undo the core win: **proof inventory pinned, gates unified, parity and adversarial regression green, stack verify green.**

---

## Where to read next

| Question | Document |
|----------|----------|
| Per-todo evidence and commands | [`TODO_COMPLETION.md`](TODO_COMPLETION.md) |
| Command ledger / level-3 witness checks | [`TODO_VERIFICATION_REPORT.md`](TODO_VERIFICATION_REPORT.md) |
| Day rollup and layer robustness | [`UMST_PROGRESS_REPORT.md`](UMST_PROGRESS_REPORT.md) |
| Witness order and decisions (normative) | [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) |
| Lean → catalog → manifold → cartridge pipeline | [`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md) |
| Remaining tracks and owners | [`PENDING_GOD_GRADE_ROADMAP.md`](PENDING_GOD_GRADE_ROADMAP.md) |

---

## Closing impact statement

Before this work, “formally grounded” mostly meant **we have proofs somewhere** and **we have gates somewhere**, with weak mechanical glue between them. After it, grounding means **a pinned proof library**, **law-like gates with tested parity and zero adversarial false negatives on the golden set**, and **a green stack verify** that any operator or agent can re-run. The [witness ladder](GOD_GRADE_WITNESS_LADDER.md) explains *why* that glue must stay ordered — not because aesthetics demand it, but because the first failed guard is the only story you should need when something goes wrong in the field.

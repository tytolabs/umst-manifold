# Executive directive: 100% solver realization (MaOS v04)

**Status:** Planning / execution-gated — not a claim that every matrix row or CI lane is already at **100%**. This document is the **canonical directive** (items **1–5** below) for what “solver realization” means after gap-close and categorical hardening; work proceeds only under **Honest constraints** and **Deferrals (execution-gated)** as written below.

**Related:** [`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md) (swarm phases A–E; Phase E categorical sorting precedes full execution of this directive), [`MAOS_SOLVER_100_PERCENT_CLOSEOUT_PLAN.md`](MAOS_SOLVER_100_PERCENT_CLOSEOUT_PLAN.md), [`Solver-Status.md`](Solver-Status.md).

### Directive index (1–5)

| # | Title | Primary artefact / lane |
|---|--------|-------------------------|
| **1** | PNP 3D GMRES + SG | Electrochemistry, matrix **#5**, `fp-gap-fp001` parity |
| **2** | Photonics DEC curl–curl 3D | `closeout-m6-dec`, **`exec-solver-photonics-dec`** / **`exec-solver-photonics-dec-3d`**, `solve_maxwell_curl_curl`, tensor **ε** |
| **3** | Pure monad purge | `thmc_residual`, orchestration composition, E4 classification |
| **4** | Striatus topology + JFNK + shell | `compute_topology`, cartridge gates, `optimize_shell_3d` / B8 |
| **5** | Dissipation + ThermodynamicCBF + Clausius–Duhem | CBF / second-law aligned controls and tests |

---

## 1 — PNP 3D GMRES + Scharfetter–Gummel flux

- **Target:** Poisson–Nernst–Planck in **3D** with **GMRES** (or equivalent Krylov stack) on the coupled discrete system, with **Scharfetter–Gummel (SG)** flux discretization where the physics brief requires it.
- **Acceptance:** Residual norms, flux conservation / discrete maximum principles where applicable, and **parity** against reference dense or 1-D staging paths **where tests exist** — no narrative “shipped” without a green or explicitly scoped test gate.

---

## 2 — Photonics: DEC curl–curl 3D

- **Target:** Production **2D/3D** discrete exterior calculus (DEC) **curl–curl** formulation, including tensor permittivity **ε** where required, wired through [`PhotonicsSolver::solve_maxwell_curl_curl`](../src/physics/solvers/photonics.rs) (or successor) with **matrix row [#6](VERIFICATION_COMPLETION_MATRIX.md)** and **[`Solver-Status.md`](Solver-Status.md)** (photonics lane, **50%** completion bin) kept in lockstep — see **[`PHOTONICS_DEC_3D_ROADMAP.md`](PHOTONICS_DEC_3D_ROADMAP.md)** for chain vs `faces_b2` patch vs volumetric sequencing.
- **Shipped partial (swarm `closeout-m6-dec` / solver lane):** uniform **x-monotone chain** TE \(E_y\) curl–curl equals scalar Helmholtz; optional **`PhotonicsDecFacesPatch`** vector DEC on **embedded 2-surfaces** in \(\mathbb{R}^3\) with test-authored **`faces_b2`** COO, **diagonal primal-length \(\star_1\)** on the curl leg ([`photonics_dec_patch_uses_metric_dual_edge_hodge`](../src/physics/solvers/photonics.rs) **`true`**), dense solve under a node cap and **capped CG** beyond — **not** a sparse-factorized volumetric production path.
- **Acceptance (still execution-gated):** **automatic** incidence from full mesh/manifold assembly; **volumetric 3D** complexes; **circumcentric/barycentric** dual Hodge refinements beyond the lumped \(\star_1\); **sparse** robust inner solves at production **N**; **PML** and **tensor-imaginary** Maxwell on the patch path without small-\(N\) / lossless restrictions; **BCs** beyond gauge **pin**; matrix **#6** *Exact acceptance criterion* satisfied end-to-end before claiming directive **2** or lane **100%**.

---

## 3 — Pure monad purge

- **Target:** Residual and control-flow purity in the spirit of the categorical sorting list: `evaluate_residual` and related pipelines as **pure** transformers where feasible; orchestration steps composable without hidden global mutation between solver calls.
- **Acceptance:** JFNK / THMC callers use residuals as **closures** or functors without spurious side effects between evaluations; **classification** of unavoidable host bridges per burn policy, not a pretend “zero `into_scalar` everywhere” mandate.

---

## 4 — Striatus: `compute_topology` + JFNK THMC + `optimize_shell_3d` criteria

- **Topology:** `compute_topology` (and cartridge topology gates) must reflect **real** watertight analysis and genus / complexity budgets — no boolean edits in `print_ready` JSON to fake passes.
- **THMC:** JFNK-coupled THMC residuals integrated with honest verification (finite backward chain, residual norms, documented deferrals).
- **Shell:** `optimize_shell_3d` and **`UMST_SHELL_*`** knobs meet **VF / greyness / compliance / B8** criteria as defined in cartridge scripts and [`MAOS_PENDING_TRACK1_AUDIT.md`](../../umst-concrete-cartridge/docs/MAOS_PENDING_TRACK1_AUDIT.md); `gates_track_b8_all_pass` flips **true** only from a **regenerated** artefact that passes gates.

---

## 5 — Dissipation, ThermodynamicCBF, Clausius–Duhem

- **Target:** Dissipation-aware controls and **thermodynamic control barrier function (ThermodynamicCBF)** semantics consistent with **Clausius–Duhem** / second-law style inequalities where the model stack defines them.
- **Acceptance:** Documented invariants, tests or verification logs where implemented; no claiming full thermodynamic certificate coverage without matching tests and matrix text.

---

## Honest constraints (no fake “100% shipped” without gates)

1. **[`FP_CATEGORICAL_BURN.md`](FP_CATEGORICAL_BURN.md)** — Krylov / Newton and host-bridge patterns are **classified**, not blanket-removed; inner CG reductions and PNP staging remain **ConvergenceRequired** / **HostBridge** until a fused-device redesign is scoped and reviewed.
2. **Gradient script** — `check_physics_no_gradient_break.sh` (and [`physics_gradient_escape_allowlist.txt`](../scripts/physics_gradient_escape_allowlist.txt)) must stay green; any shrink of allowlisted escapes needs dated rationale and CI evidence.
3. **Partial matrix rows** — [`Solver-Status.md`](Solver-Status.md) and [`MAOS_PLAN_ALIGNMENT_REPORT.md`](MAOS_PLAN_ALIGNMENT_REPORT.md): several rows may remain **partial (not full matrix 100%)** while engineering slices land; prose must match code.
4. **Band–LU parity** — [`FP_GAP_BACKLOG.md`](FP_GAP_BACKLOG.md) / `fp-gap-fp001`: **band LU vs dense-expand** PNP full-SG narratives and tests; **parity failures** are **open** until root-caused and verified — do not tick matrix **#5** or electrochemistry “100%” on documentation alone.

**Rule:** No marketing-style **“100% shipped”** without the **recursive verification ladder**, Solver-Status bullets, and any lane-specific scripts (`verify_striatus_coupled_gates.sh`, pytest B8 contract, etc.) all aligned.

---

## Deferrals (execution-gated)

The following are **explicitly out of scope** for “solver realization” execution until upstream closure is honest; they are **not** waived—only **sequenced after** [`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md) Phase A–E and related matrix/CI gates.

1. **Wide parallel implementation swarms** — Do not spawn overlapping implementation subagents on electrochemistry, photonics, and cartridge in one session unless the parent timeline requests it; prefer single-writer Solver-Status edits and one PR per lane.
2. **Large refactors during electrochemistry parity** — Defer tree-wide imperative→operator refactors until **Final Categorical Hardening** and task-swarm completion, especially **`fp-gap-fp001`** (band LU vs dense-expand PNP full-SG). The full `solver-experimental` union may still fail at least one test until that row closes.
3. **Directive 1 / matrix #5 “100%” ticks** — No matrix or marketing claim of full PNP production readiness without green parity tests and aligned `try_solve` / dense narrative ([`FP_GAP_BACKLOG.md`](FP_GAP_BACKLOG.md)).
4. **Directive 2 full acceptance** — Assembly-from-mesh, **volumetric 3D**, refined **dual** Hodge, **sparse** production inner solves, **PML** / full **complex ε** on the patch path, and **BCs** beyond a **pin** remain **open** per [`Solver-Status.md`](Solver-Status.md) / matrix **#6** until `closeout-m6-dec` criteria are met; **lumped metric \(\star_1\)** on the small patch and **small-\(N\)** lossy scalar **`eps_r_imag`** do **not** alone close the directive.
5. **Directive 4 B8 / topology** — `gates_track_b8_all_pass` flips **true** only from **regenerated** `print_ready` JSON and passing scripts — never from hand-edited booleans (see **closeout-m1-b8** in [`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md)).
6. **Directive 5 certificates** — No claim of full thermodynamic certificate coverage without tests and matrix text that match implementation.
7. **Planning-only todo hygiene** — Cursor todo IDs matching **`exec-solver-*`** and **`exec-striatus-*`** stay **pending** until an **execution phase** after gap-close / hardening; doc-only or planning passes must not clear them.

---

## Agent policy (this run and sequencing)

- **Do not** spawn parallel **implementation** subagents in the same session as this doc’s authoring unless the parent explicitly requests it — avoid thrashing overlapping lanes (electrochemistry, photonics, cartridge).
- **Defer** large refactors until ongoing **Final Categorical Hardening** and prior **Task swarm** transcripts report **done**, especially **electrochemistry parity** — the **full** `solver-experimental` suite may still **fail at least one test** until `fp-gap-fp001` / band–LU closure and related fixes land. Do not destabilize the tree with wide refactors while that lane is mid-flight. *(Normative list: see **Deferrals** above.)*

**Cursor todos:** IDs matching **`exec-solver-*`** and **`exec-striatus-*`** remain **pending** until the **execution phase** after gap-close / hardening — **do not** mark them completed in doc-only or planning passes. (Do not use planning-only work to clear execution-gated todos.)

---

## Recommended next agents (resume after idle / gates green)

Numbered lanes to **resume after** hardening swarms report complete and CI ladder is honest for the touched crates:

1. **Electrochemistry lane** — PNP 3D GMRES + SG, matrix **#5**, `fp-gap-fp001` band–LU vs dense parity, `try_solve` narrative vs code.
2. **Photonics DEC lane** — `closeout-m6-dec` + **`exec-solver-photonics-dec`** / **`exec-solver-photonics-dec-3d`**: volumetric 3D + assembly + refined Hodge / sparse / BC / full complex-ε + PML slices with tests aligned to matrix **#6**.
3. **Purge lane** — Coordinate with [`FP_CATEGORICAL_BURN.md`](FP_CATEGORICAL_BURN.md) and E4 classification: minimize **LeakCandidate** only with AD + numerics sign-off; no blanket purge.
4. **Cartridge Striatus lane** — `compute_topology`, `verify_striatus_coupled_gates.sh`, `optimize_shell_3d` / `UMST_SHELL_*`, B8 rollup and **`gates_track_b8_all_pass`**.
5. **CBF / dissipation lane** — ThermodynamicCBF, Clausius–Duhem alignment, tests and docs vs Solver-Status.

---

*Document version: MaOS v04 alignment. Execution handoff: parent timeline + gap-closure plan.*

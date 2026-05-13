# FP categorical burn audit — `into_scalar`, host sync, and sync reads

**Branch context:** `fp-sort-io-monad-audit` (E4 classification pass; prior burn audit: `fp-v04-6-burn-audit`)  
**Scope:** Burn [`Tensor`](https://burn.dev/) escapes that force **device → host** synchronization (`into_scalar`, `into_data`, and full-vector reads). Focus is **solver hot paths** in `umst-manifold` plus **`umst-concrete-cartridge`** integration surfaces.

This note classifies each pattern as **intentional control-flow / diagnostics**, **architectural host bridge** (CPU kernels or dense linear algebra), **test-only**, or **risk / drift** (including AD semantics and CI guard mismatches).

---

## E4 — physics `src/physics/` tier tags (`fp-sort-io-monad-audit`, 2026-05-11)

CI greps **`into_scalar` / `into_data`** per file; the allowlist is **per-file**. Below, each allowlisted source is tagged for triage (see also `scripts/physics_gradient_escape_allowlist.txt`). **Do not** blanket-remove **Krylov / Newton** per-iteration scalars without a fused-device redesign — they stay **ConvergenceRequired**.

| Tier | Meaning |
|------|---------|
| **ConvergenceRequired** | Classical CG / Newton stopping, dot products, norms — intrinsic per-iteration reads unless the algorithm is rewritten. |
| **Diagnostic** | Lower-rate logging, bisection bounds, parity checks outside inner Krylov. |
| **HostBridge** | Full-vector `into_data().value` feeding CPU assembly, dense GMRES, FDFD, or 1-D PNP staging. |
| **TestOnly** | Hits only under `#[cfg(test)]` (or feature-gated tests); still matches the grep-based script. |
| **LeakCandidate** | Previously meant “candidate to replace with tensor-native ops”; **only shrink this tier** when AD + numerics reviewers agree (e.g. adjoint compliance **`c_pad`** now uses `Tensor::from_inner(comp)` instead of `Tensor::full` from a host `f32`, removing a **graph cut** while keeping one **`into_scalar`** for the `(surrogate, c_raw)` API). |

| File | Dominant tier(s) |
|------|------------------|
| `physics/mechanics.rs` | **ConvergenceRequired** (tensor PCG: `rhs_norm`, `rz`, `β`, `‖r‖` exit); **TestOnly** / **Diagnostic** (`into_data` in unit tests and Voigt/Hooke asserts). |
| `physics/solvers/electrochemistry.rs` | **ConvergenceRequired** + **HostBridge** (CG scalars + repeated 1-D `into_data` PNP bridge). |
| `physics/solvers/rheology_flow.rs` | **No allowlisted escapes** (2026-05-12, `exec-solver-purge`) — `into_scalar` / `into_data` removed from production/tests in this module; dropped from `physics_gradient_escape_allowlist.txt`. |
| `physics/solvers/thmc.rs` | **ConvergenceRequired** — implicit thermal CG: per-iter `sqrt(⟨r,r⟩).into_scalar()` to `Vec<f32>` residual trace + tol (coefficients stay tensor); rustdoc/comment-only lines also match the CI grep. |
| `physics/solvers/thmc_residual.rs` | **ConvergenceRequired** + **HostBridge** (`‖R‖²` stacks + GMRES vector stitch). |
| `physics/solvers/photonics.rs` | **HostBridge** + **Diagnostic** (FDFD host assembly; patch guard reads). |
| `physics/solvers/acoustics.rs` | **HostBridge** (Newmark + `pack_bn3_to_flat` → host GMRES) + **TestOnly** (`into_data` in `#[cfg(test)]` asserts). |
| `physics/dec_primal.rs` | **Diagnostic** — `dec_primal_max_abs_d1_of_scalar_gradient`: one `max().into_scalar()` DEC witness (not inner Krylov). |
| `physics/solvers/fracture_field.rs` | **ConvergenceRequired** (outer-loop stopping) + **TestOnly** (`into_data` in fracture-at2 tests/diagnostics). |
| `physics/solvers/statistical_mechanics.rs` | **HostBridge** (EOS table materialization) + **TestOnly**. |
| `physics/solvers/topology_solver.rs` | **No allowlisted escapes** — unit tests use tensor [`all_close`](https://docs.rs/burn-tensor/0.13.2/burn_tensor/struct.Tensor.html#method.all_close) / mask `float` parity instead of `into_data` / host vector scans (2026-05-11). |
| `physics/extruded_plate.rs` | **HostBridge** (Q1 hex reference loops on host vectors). **LeakCandidate (deferred):** triple `into_data().value` staging stays until a device-native Q1-hex path + numerics parity — see § *exec-solver-purge* below. |
| `physics/topology_filter.rs` | **TestOnly** (Helmholtz filter asserts under `topology-density-evolution`). |
| `physics/adjoint.rs` | **Diagnostic** — one **`into_scalar`** at the `(surrogate, c_raw)` boundary; surrogate’s compliance term stays on-tape via **`from_inner`**. |
| `physics/orchestration.rs` | **TestOnly** — `into_data()` in `#[cfg(test)]` for tensor equality between `run_plan_step` and `fold_plan_step`. |
| `physics/operator.rs` | **HostBridge** — `BarMatvecOperator::apply_vec`: packed `f32` Krylov matvec; narrowed batch-row slice, `reshape([3N])`, then one `into_data().value` (not a training inner loop). |

### exec-solver-purge (2026-05-12)

Lane hygiene for [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](./EXEC_SOLVER_REALIZATION_MAOS_V04.md) directive **#3** (pure monad / classification): **no** blanket removal of **ConvergenceRequired** Krylov scalars; allowlist and this doc stay aligned with `scripts/check_physics_no_gradient_break.sh`.

1. **`physics/solvers/rheology_flow.rs`** — confirmed **zero** `into_scalar` / `into_data` matches; entry **removed** from `physics_gradient_escape_allowlist.txt` (stale allowlist noise purge).
2. **LeakCandidate — documented deferral (`physics/extruded_plate.rs`):** three `into_data().value` pulls (`rho`, body force, boundary mask) are **structural** inputs to `q1_hex_elasticity::hex_solve_pcg_masked` on host `Vec<f32>`. Shrinking this to a single sync or a fully device-resident assembly is a **LeakCandidate**-class change **deferred** until a Burn-backed Q1-hex operator (or equivalent) ships with **numerics + AD** parity review — not a drive-by graph-cut edit.

### E5 — Solver core line inventory (`into_scalar` / `into_data`)

**Audit dates:** baseline **2026-05-11**; inventory **refreshed 2026-05-12** (`exec-solver-purge`). **Path:** `umst-manifold/src/physics/solvers` (same `rg -nE 'into_scalar|into_data' …/**/*.rs` family as CI). **Scope:** line-occurrence count only (each matching line counted once per file).

**Reconciled totals (2026-05-12):** **127** matching lines across **seven** `.rs` files that contain at least one hit. Prior narrative figures (**177** / **eight** files, **`electrochemistry.rs` at 58**) reflected an older tree — **replace** those aggregates with **127** / **seven** and **`electrochemistry.rs` at 45** when citing this audit.

**`thmc.rs` nuance:** **seven** lines match the grep pattern; **two** are live `.into_scalar()` calls (L2 residual telemetry in `step_thermal_implicit`, pushed to `residual_norms`). The other **five** hits are **rustdoc / line comments** that mention `.into_scalar()` in prose (including cross-refs to `thmc_residual` rank‑1 reads).

**Canon (no blanket tensor-only rewrite):** Classical **Krylov / CG** and **Newton** loops on Burn still need per-iteration scalar reductions unless the algorithm is redesigned; see [FP_FIXED_POINT_CANONICAL.md](./FP_FIXED_POINT_CANONICAL.md) (inner CG and per-iteration `.into_scalar()`) and, in this document, the subsections *A. Krylov / CG inner loops* and *B. Newton / outer residual norms* under *Hotspots — `umst-manifold` physics / core*. **Do not** treat the “tensor predicate” column as a mandate to remove those sites without numerics + AD review.

| File (under `src/physics/solvers/`) | Line matches (2026-05-12) | Tier for zero-sync triage |
|--------------------------------------|---------------|---------------------------|
| `electrochemistry.rs` | 45 | **Mixed:** **ConvergenceRequired** (inner CG: `rz` / `p_ap` / norms / guards) plus **HostBridge** (repeated 1-D PNP `into_data().value` staging). **Tensor-predicate candidates:** only **outside** inner Krylov and host-only Newton staging — not the CG coefficient path. |
| `thmc_residual.rs` | 23 | **Mixed:** **ConvergenceRequired** (`‖R‖²`-style `.sum().into_scalar()` stacks) plus **HostBridge** (`into_data().value` vector stitch for host GMRES / dense paths; module docs on Newton early-exit). **Candidates:** only where residual assembly can stay on-device end-to-end (large refactor). |
| `photonics.rs` | 22 | **HostBridge-heavy** (FDFD / patch host assembly). **Tensor-predicate candidate:** low until sparse/direct solve moves off CPU staging. |
| `fracture_field.rs` | 16 | **Mixed:** **ConvergenceRequired** (outer damage / strain `max` / `sum` scalars) plus **TestOnly** `into_data` in fracture-at2 tests. **Candidates:** test-only paths; outer scalars stay **ConvergenceRequired** under current AT2 driver. |
| `statistical_mechanics.rs` | 9 | **HostBridge** (EOS / virial table materialization) + **TestOnly** asserts. **Candidates:** tests; production table path remains host-shaped. |
| `thmc.rs` | 7 (2 live) | **ConvergenceRequired** for **two** L2 telemetry `.into_scalar()` sites; remaining matches are documentation only. Optional monolithic Newton path **deliberately** avoids scalar early-exit on the hot path (comments cross-ref residual-on-device policy). |
| `acoustics.rs` | 5 | **HostBridge** + **TestOnly** — `pack_bn3_to_flat` host GMRES staging plus test-only `into_data` asserts (see allowlist rustdoc). |

**Zero-match solver modules (same directory):** `mod.rs`, `fixed_point.rs`, `thmc_jfnk.rs`, `lj_johnson_1993_reference.rs`, **`rheology_flow.rs`**, **`topology_solver.rs`** — no `into_scalar` / `into_data` occurrences at 2026-05-12 re-scan.

---

## Definitions

| Pattern | Typical cost | Role |
|--------|--------------|------|
| `.into_scalar()` | Sync **one** scalar; blocks until reduction completes | CG coefficients, convergence checks, scalar controls |
| `.into_data()` | Often materializes **full tensor** on host | CPU loops, custom sparse/dense solvers, assertions |
| `.into_data().value` iteration | Full tensor sync + Rust iteration | Same as `into_data`; highest “burn” for large fields |

**“Categorical burn”** here means any host materialization that **breaks a purely device-resident subgraph** — relevant for autodiff training loops (graph cuts) and for throughput in tight Krylov / Newton iterations.

---

## Hotspots — `umst-manifold` physics / core

### A. Krylov / CG inner loops (high frequency per iteration)

**Fixed-point driver context (`gap-fp-inner-loop-syncs`):** per-iteration `.into_scalar()` in inner CG is **orthogonal** to the `iterate_until` vs `repeat_controlled` choice for outer bounded iteration. When and why those scalars stay per iteration, optional batched-reduction futures, and the “docs-only ⇒ no behavior change” bar are spelled out in [`FP_FIXED_POINT_CANONICAL.md`](./FP_FIXED_POINT_CANONICAL.md) § *Inner CG and per-iteration `.into_scalar()` (Burn)*.

These use `.into_scalar()` for dot products, norms, and stability guards — **intrinsic** to classical CG on Burn unless rewritten with fully fused device ops and deferred stopping.

| Location | Pattern | Classification |
|----------|---------|------------------|
| `physics/solvers/electrochemistry.rs` | CG: `rz_old`, `p_ap`, `phi_mx`, `res_norm`, `rz_new`, `rhs_abs_max`, … | **Intentional solver math** — sync per iteration |
| `physics/solvers/rheology_flow.rs` | *(2026-05-12)* No `into_scalar` / `into_data`; allowlist entry removed after purge re-scan. | **N/A** (grep-clean). |
| `physics/solvers/thmc.rs` | `step_thermal_implicit`: per-iteration `sqrt(⟨r,r⟩).into_scalar()` pushed to `residual_norms: Vec<f32>` for tol / trace; CG coefficients (`α`, `β`) stay on tensor — see module rustdoc E4. | **ConvergenceRequired** (telemetry + stopping on host trace). |
| `physics/mechanics.rs` | Tensor CG: `rz`, `beta`, `r_norm`, … | Production path uses scalars for coefficients; **non-test** code participates — allowlisted file |

### B. Newton / outer residual norms and stopping (`into_scalar` + stacked ‖R‖)

| Location | Pattern | Classification |
|----------|---------|----------------|
| `physics/solvers/thmc_residual.rs` | Sums of `.mul().sum().into_scalar()` for ‖R‖²-style quantities; `into_data()` to stitch residual vectors for host GMRES / dense paths | **Mixed:** scalar reductions are **control-flow** for damped Newton / JFNK; **full `into_data`** stacks are **host-algorithm bridge**. Module docs acknowledge AD-noncommutative stopping — see “Follow-up (`m8-scale-ad`)” in file header |
| `physics/solvers/electrochemistry.rs` (later regions) | `max().into_scalar()` for Δφ, Δc, comparisons | **Convergence / bisection / deferred diagnostics** |
| `physics/solvers/fracture_field.rs` | `outer_stopping_should_break`: `max().into_scalar()` for damage/strain tol; `degraded_psi_mean_scalar`: `sum().into_scalar()` | **Intentional outer-loop stopping** (feature `fracture-at2`) |

### C. Host bridge — CPU staging for non-GPU algorithms

| Location | Pattern | Classification |
|----------|---------|----------------|
| `physics/solvers/electrochemistry.rs` | Repeated `into_data().value` for `phi`, `c`, ε, D along **1-D** PNP sub-problems | **Structural:** finite-volume / boundary indexing on host arrays |
| `physics/operator.rs` | `BarMatvecOperator::apply_vec`: narrowed `ku` row, `reshape([3N])`, `into_data().value` | **Structural:** host Krylov matvec return path (bar stiffness apply) |
| `physics/solvers/photonics.rs` | `into_data().value` for ε, sources, edges, coords; mutable host assembly | **Structural:** FDFD-style host sparse/direct solve path |
| `physics/extruded_plate.rs` | `rho_flat`, `f_flat`, `m_flat` → nested loops for Q1 hex cells | **Structural:** explicit cell loops + Thomas/CG reference path on CPU. **LeakCandidate (deferred):** see § *exec-solver-purge*. |
| `physics/solvers/acoustics.rs` | `pack_bn3_to_flat` → `into_data().value` for host GMRES (`krylov_host::gmres_f32_try`) | **Structural:** Newmark step Krylov bridge (narrowed batch row). |
| `physics/topology_filter.rs` | `into_data()` inside **`#[cfg(all(test, feature = "topology-density-evolution"))]`** only (`tensor_max` assertions) | **Test-only** — grep-based CI script still flags file (see Guardrails) |
| `physics/orchestration.rs` | `into_data()` in **`#[cfg(test)]`** only — equality of `ThmcState` tensors across `run_plan_step` vs `fold_plan_step` | **Test-only** — no production host reads |

### D. Diagnostics and monitors (lower frequency but still “burn”)

| Location | Pattern | Classification |
|----------|---------|----------------|
| `core/emergence.rs` | `sdf.mul_scalar(0.0).into_scalar()` as pad constant | **Tiny sync** — scalar literal trick for `pad` API |
| `physics/adjoint.rs` | `Tensor::from_inner(comp)` for `c_pad` + **`comp.into_scalar()`** once for `c_raw` | **E4:** compliance contributes to **`surrogate` on the autodiff tape**; remaining scalar is **API / logging boundary** only (not a Krylov inner-loop pattern) |
| `physics/dec_primal.rs` | `dec_primal_max_abs_d1_of_scalar_gradient`: **`max().into_scalar()`** | **Diagnostic** — DEC witness / photonics identity checks |

### E. AI / topology orchestration (session-scale, not micro-step inner loops)

| Location | Pattern | Classification |
|----------|---------|----------------|
| `ai/cbf.rs` | `sum_bits_tensor.into_scalar()` — documented **host barrier** for Landauer / accounting | **Policy logging** — deliberate once-per-step |
| `ai/topology.rs` | `compliance.into_scalar()`, `into_data().value` iteration for continuation | **Outer orchestration** — acceptable at topology-step granularity; avoid pushing into per-edge inner loops |
| `ai/ppo.rs`, `ai/liquid_ppo.rs` | Module docs: avoid scalar burn on PPO hot path | **Documentation guard** |

### F. `physics/solvers/statistical_mechanics.rs`

Host reads for EOS tables / verification comparisons — mix of **production table lookup** (`into_data` slices) and **`#[cfg(test)]`** parity asserts. File is **allowlisted** with rationale in `physics_gradient_escape_allowlist.txt`.

---

## Hotspots — `umst-concrete-cartridge`

Integration code pulls **scalar knobs** from solver-report tensors for material closures and costing — acceptable when **O(1)** per pipeline step, not per finite-element node inside a subsolver loop.

| Location | Role |
|----------|------|
| `src/core/implementation.rs` | `tensor_l1(t) > eps` → `t.into_scalar()` for `fc_use`, `tau_use`, `gwpt` scalars; bridges UMST `PhysicalResult` fields into nodal tensors |
| `src/pipeline/orchestrator.rs` | Broad host reads: mix fractions, slice scalars for ITZ / creep / shrink summaries — **pipeline diagnostics** |
| `src/facade/mod.rs` | `slice.into_scalar()` for scalar extraction API |
| `src/mix_layout.rs` | `slice(...).into_scalar()` |
| `examples/*.rs` | Training loops: `into_data().value[0]` for loss — **expected** for Python-style logging |

**Verdict:** cartridge **burn** is predominantly **outer-loop / API boundary**, not competing with manifold’s inner Krylov iterations — but **profiling** should confirm no accidental per-element scalarization in future PRs.

---

## Leak vs intentional logging

| Signal | Intentional / necessary | Likely leak or smell |
|--------|----------------------|----------------------|
| Inside CG while-loop every iteration | Classic algorithm needs scalars | N/A unless iterations can be batched/fused on device |
| Once per topology / RL step | Policy bits, compliance reporting | Doing the same **inside** nested damage or hydration substeps |
| `into_data()` + **full vector** in tight loop | Required only when host algorithm consumes full state | Replacing with Rust loops **without** shrinking tensor size first |
| Comment references “Landauer”, “barrier”, “tol”, “Newton” | Documented contract | Silent `.into_scalar()` with no comment in new solver code |

---

## Guardrails for future PRs

1. **CI script:** `umst-manifold/scripts/check_physics_no_gradient_break.sh` greps `into_scalar|into_data` under `src/physics/` and fails unless the file appears in `scripts/physics_gradient_escape_allowlist.txt` with a **human rationale**.
2. **Allowlist hygiene:** Adding a new escape requires **one-line justification** + date in the allowlist. Prefer **tensor-native** alternatives first (pure Burn reductions, masks, `sum_dim` staying on device until the last mile).
3. **Tests:** `#[cfg(test)]` modules still **match** the script’s patterns — tests in hot-path files inflate grep noise; either split test helpers to test-only files or extend allowlist entries with “tests only” notes (already done for several files historically).
4. **AD-sensitive paths:** For differentiable outer loops, avoid `.into_scalar()` **in stopping predicates** unless you accept non-reparameterized gradients — `thmc_residual.rs` documents this explicitly.
5. **Cartridge:** Keep scalar pulls at **pipeline boundaries**; avoid new `into_data()` of full nodal fields inside `ThmcSolver` / fracture substeps unless staging to CPU is unavoidable.

### Allowlist / CI drift (audit finding)

**Update 2026-05-11 (`gap-ci-physics-allowlist`):** the physics gradient-escape allowlist now covers the audited solver/host-bridge files above; `bash umst-manifold/scripts/check_physics_no_gradient_break.sh` is expected to exit **0** on a clean tree. **Deferred hotspots** remain any future `.into_scalar()` / `.into_data()` additions in non-allowlisted physics sources — those must join the allowlist with rationale or be refactored to tensor-native ops.

**Update 2026-05-11 (`fp-sort-io-monad-audit`, E4):** per-file **tier tags** (ConvergenceRequired / Diagnostic / HostBridge / TestOnly) are recorded in § *E4 — physics `src/physics/` tier tags* above; **LeakCandidate** shrink is **evidence-led** only (e.g. adjoint **`c_pad`** no longer uses a host-filled constant tensor).

**Update 2026-05-12 (`exec-solver-purge`):** `rheology_flow.rs` dropped from the allowlist (grep-clean); E5 counts refreshed (**127** / **seven** solver files); **LeakCandidate** deferral recorded for **`extruded_plate.rs`** (see § *exec-solver-purge*). `check_physics_no_gradient_break.sh` remains exit **0**.

---

## Verification performed for this document

### Ripgrep patterns used

```text
# Primary Burn escapes (run from repo root)
rg -n 'into_scalar|into_data' umst-manifold/src --glob '*.rs'
rg -n 'into_scalar|into_data' umst-concrete-cartridge --glob '*.rs'

# Solver subdirectory only
rg -n 'into_scalar|into_data' umst-manifold/src/physics/solvers --glob '*.rs'

# Same patterns as CI script
rg -nE 'into_scalar|into_data' umst-manifold/src/physics/**/*.rs

# Optional: host-sync wording / docs
rg -ni 'host.*sync|into_scalar|Landauer|gradient escape' umst-manifold/src --glob '*.rs'
```

**E5 parity (2026-05-12, `exec-solver-purge`):** the same solver-only pattern under `umst-manifold/src/physics/solvers` yields **127** line matches across **seven** `.rs` files with ≥1 hit (per-file breakdown in subsection *E5 — Solver core line inventory* above). Re-run after substantive solver edits.

### Script parity

```bash
bash umst-manifold/scripts/check_physics_no_gradient_break.sh
```

**Result:** exits **0** on a clean tree (verified **2026-05-12** after `exec-solver-purge` allowlist trim); see “Allowlist / CI drift” above.

### `cargo test` / `clippy`

Allowlist / rustdoc hygiene edits do not change solver semantics; **`cargo test` / `clippy`** should still be run after substantive physics changes. Re-run after any subsequent code or allowlist edits.

---

## References

- `umst-manifold/scripts/check_physics_no_gradient_break.sh`
- `umst-manifold/scripts/physics_gradient_escape_allowlist.txt`
- `umst-manifold/src/physics/solvers/thmc_residual.rs` (module docs — AD-safe ‖R‖ follow-up)
- `composer-plans/umst_full_coupling_integration.md` (architectural note on hot-path sync)

---
name: MaOS Solver 100% Close-out
overview: Repo-first, zero-compromise roadmap to drive every numbered verification scope (#1–#10) to matrix-defined completion (100% bins where applicable), using nested verification gates (narrow tests → feature matrix → clippy → check_solver_status) and honest deferrals only where the matrix explicitly allows artefacts or opt-in proofs.
todos:
  - id: x-cut-ci
    content: "Cross-cutting: keep solver-experimental tests + clippy -D warnings green; run check_solver_status.py after every Solver-Status/matrix edit; single-writer for Completion % merges"
  - id: x-stash
    content: Resolve umst-manifold git stash backlog safely (inspect stash@{n}, merge or drop) before large multi-file solver refactors
  - id: m1-b6
    content: "#1: Run shell_topology_rib_pattern_full_v04 40×40×4×200 outers with documented UMST_SHELL_* env; meet B6 VF/greyness/xy_var/compliance gates"
  - id: m1-b8
    content: "#1: Achieve Track B8 — striatus_shell_v0.4.print_ready.json gates_track_b8_all_pass; shell_demo_smoke + pytest UMST_REQUIRE_B8=1"
  - id: m1-l
    content: "#1: Commit Track L artefacts (GIF/STL/print_ready JSON, optional OBJ) under cartridge notebooks/_artifacts with size/genus/VF constraints"
  - id: m1-doc
    content: "#1: Sync manifold + cartridge Solver-Status; set matrix #1 / Completion % to 100 only when all acceptance bullets satisfied"
  - id: m2-sri
    content: "#2: Implement SRI/enrichment + BC alignment; add default-CI Kirchhoff within-X% test on brief brick path (§R2.1)"
  - id: m2-doc
    content: "#2: Update Solver-Status + matrix #2; ensure mechanics combined row min(#2,#10) remains honest"
  - id: m3-stagger-stop
    content: "#3: Implement energy/residual outer stopping for fracture stagger (update_damage_staggered / solve_staggered_with_mechanics)"
  - id: m3-memo7
    content: "#3: Close remaining Track 12 §7 acceptance — ψ⁺ Γ/dissipation, (l₀,h), u↔d, matrix_features/no-bar per matrix"
  - id: m4-maint
    content: "#4: Lock acoustics n=128 return-map + Ω recipe; regression tests only unless optional graph-assembled work is explicitly scoped"
  - id: m5-scale
    content: "#5: Replace dense-expand inner Newton ceiling — band LU parity+wiring OR Krylov/matrix-free; verify at target N"
  - id: m5-graph
    content: "#5: General-graph Poisson/Picard–Newton (non-chain Thomas); variable ε; |zΔφ| robustness; closed-graph mass band"
  - id: m6-dec
    content: "#6: Wire 2D/3D DEC curl-curl + tensor ε into solve_maxwell_curl_curl with integration tests"
  - id: m6-fresnel
    content: "#6: Tighten Fresnel / r_disc acceptance to matrix margin"
  - id: m7-longrun
    content: "#7: Long-run L² vs Poiseuille <15% harness (env-gated if needed); MAC/open-x BC milestone per matrix"
  - id: m7-mac
    content: "#7: Eliminate checkerboard via MAC/open boundary realism — roadmap-aligned, not surrogate-only"
  - id: m8-jfnk
    content: "#8: Implement JFNK (matrix-free + Krylov) replacing dense gauss_jordan monolith path; preconditioner (Jacobi/ILU0) with benchmarks"
  - id: m8-scale-ad
    content: "#8: AD-safe ‖R‖ exit + adaptive dt + within-step u↔d per matrix / Track 13"
  - id: m9-upscale
    content: "#9: upscale_potentials (ρ*,T*) contract + Johnson/virial inside bridge with stated tolerance tests"
  - id: m10-transient
    content: "#10: Default-CI transient vector mechanics OR explicit out-of-scope statement — pick matrix PR slice A/B consistently"
  - id: m10-contact
    content: "#10: Contact + Coulomb friction verification stack if matrix demands 100% for combined mechanics story"
  - id: int-striatus
    content: "Integration: Striatus Gate — cartridge proof + optional manifold PPO/physics chain smoke; VRAM claim only with profiling task"
isProject: false
---

# MaOS v0.4 Solver — Detailed To-Do (0 compromise → 100% matrix acceptance)

**Sources of truth:** [`umst-manifold/docs/Solver-Status.md`](Solver-Status.md), [`umst-manifold/docs/VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md), [`umst-manifold/docs/VERIFICATION_SCOPE_INDEX.md`](VERIFICATION_SCOPE_INDEX.md), [`umst-manifold/GAP_AUDIT.md`](../GAP_AUDIT.md). Missing external files (`solver_honest_audit.md`, etc.) are **out of scope** until added to the workspace.

**“100% completion” definition:** For each matrix row, **all** bullets under **Exact acceptance criterion** are satisfied on the stated surface (default CI, named `--features`, committed artefacts, or explicitly documented opt-in proof). **Completion %** moves only per [`b7dd1b9`](Solver-Status.md)-style rubric — not test count alone. Mechanics combined row uses **min(#2, #10)** where Solver-Status says so.

**Recursive gate after every substantial task:** (1) narrow `cargo test` for the lane → (2) `cargo test --features solver-experimental` (or lane union) → (3) `cargo clippy --all-targets --features solver-experimental -- -D warnings` → (4) `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` → (5) if docs touch cartridge mirror, refresh [`umst-concrete-cartridge/docs/Solver-Status.md`](../../umst-concrete-cartridge/docs/Solver-Status.md) in a follow-up PR.

---

## Cross-cutting (parallel track — blocks nothing but merges)

- Maintain green **`main`**: full `solver-experimental` test + clippy before each merge.
- **Single-writer rule** for [`Solver-Status.md`](Solver-Status.md) table **Completion (%)** column per merge window to avoid doc conflicts.
- Resolve stale **git stash** pile on [`umst-manifold`](../) before large mechanical refactors (prior audit: overlapping agent WIP).

---

## #1 Topology / shell — matrix row → **100%**

1. Reproduce **B6** numerically: [`shell_topology_rib_pattern_full_v04`](../../umst-concrete-cartridge/crates/umst-concrete-cartridge/tests/shell_topology_rib_pattern.rs) at **40×40×4**, **200** outers, seed **42**, `UMST_SHELL_RIB_PATTERN=1`, `--release` before `--`, documented env (`UMST_SHELL_*`, roof ramp vs uniform honesty).
2. Meet gates: VF **±1%** of **0.15**; **volume** greyness **mean(4ρ(1−ρ)) < 0.15**; **xy** variance **> 0.1**; compliance **< 0.6×** iter-1; no NaNs (PCG/adjoint conditioning per runbook).
3. **Track B8:** [`striatus_shell_v0.4.print_ready.json`](../../umst-concrete-cartridge/notebooks/_artifacts) — genus **≥ 1**, variance **≥ 0.1**, VF **[0.10, 0.25]**; **`gates_track_b8_all_pass`**; [`shell_demo_smoke`](../../umst-concrete-cartridge) + rib GIF backup path per matrix.
4. **Track L:** commit **`striatus_emergence.gif`**, **`striatus_shell_v0.4.stl`**, **`striatus_shell_v0.4.print_ready.json`** (optional `.obj`) under [`notebooks/_artifacts/`](../../umst-concrete-cartridge/notebooks/_artifacts) — GIF **≤ 5 MB**, **≥ 30** frames; STL **≤ 8 MB**, watertight, genus **≥ 1**, VF in band.
5. Run [`notebooks/tests/test_print_ready.py`](../../umst-concrete-cartridge/notebooks/tests/test_print_ready.py) with **`UMST_REQUIRE_B8=1`** when enforcing gates.
6. Sync manifold + cartridge [`Solver-Status.md`](Solver-Status.md) P0 blocks; advance **#1** to **100%** only when (1)–(5) are true.

---

## #2 Mechanics — plates / §R2.1 → **100%**

1. Implement **SRI or brief-approved stabilization** + **BC alignment** on the **brief’s brick path** so **centre deflection vs Kirchhoff SSSS** meets matrix **within-X%** (§R2.1) on **default CI** (named test + explicit **L/t**, mesh in test/docs).
2. Keep [`plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band`](../tests/verification/mechanics_analytic.rs) as **orthogonal regression** only — do not treat as §R2.1 closure.
3. Update [`Solver-Status.md`](Solver-Status.md) mechanics row + matrix **#2**; mechanics combined completion respects **min(#2,#10)**.

---

## #3 Fracture — AT2 / Γ / ψ⁺ / stagger → **100%**

1. Deliver memo **§7** backlog: Γ-type or dissipation with **ψ⁺ ≠ 0**; broader **(l₀,h)**; **within-step THMC u↔d** where matrix demands; [`matrix_features`](../src/physics/solvers/fracture_field.rs) / no-bar path documented + tested.
2. **Stagger certificate:** energy-norm or residual-based **outer** stopping for [`update_damage_staggered`](../src/physics/solvers/fracture_field.rs) / [`solve_staggered_with_mechanics`](../src/physics/solvers/fracture_field.rs) — not “one outer pass” unless config explicitly sets `outer_iterations=1` with documented semantics.
3. Verify with `cargo test --features fracture-at2,solver-experimental` (+ `thmc-coupled` where coupling tests apply).

---

## #4 Acoustics → **100%** (already “Done” in matrix — maintenance / regression lock)

1. Keep [`plane_wave_return_map_n128_l2_within_two_percent`](../tests/verification/acoustics_plane_wave.rs) and lumped **Ω**, **T = 2π/Ω** story; ensure no regression vs matrix wording.
2. Optional breadth (does not block **#4** row): graph-assembled acoustic stiffness — only if scoped as separate backlog.

---

## #5 Electrochemistry → **100%**

1. **Default CI λ_D gates:** [`debye_screening_256_cells_*`](../tests/verification/pnp_debye_layer.rs) **±11% / ±15%** bands stable.
2. **Cost / scale:** Remove production reliance on **O((3N)³)** dense-expand inner Newton for full-SG chain at target **N** — implement **band LU wired into `try_solve`** *after* parity **or** **matrix-free Krylov** inner correction with documented tolerance; avoid false “shipped” claims (see current [`electrochemistry.rs`](../src/physics/solvers/electrochemistry.rs) + Solver-Status).
3. **General graph:** Implicit Newton / Poisson on **arbitrary 3-skeleton** DEC graph — sparse-direct or Krylov Poisson (not chain-only Thomas).
4. Close matrix bullets: non-chain implicit Newton smoke; variable **ε** path; large **|zΔφ|** **f32** robustness; closed-graph mass band where stated.

---

## #6 Photonics → **100%**

1. **2D/3D** metric-weighted DEC **vector** curl–curl wired through [`PhotonicsSolver::solve_maxwell_curl_curl`](../src/physics/solvers/photonics.rs) with integration tests (not only 1-D TE reduction).
2. **Tensor ε** beyond scalar nodal **ε_r**.
3. Tighten Fresnel / discrete **r_disc** vs analytic margin (partial work may exist — extend until matrix acceptance met).

---

## #7 Rheology → **100%**

1. **Long-run** centreline **L²** vs analytic Poiseuille **< 15%** at stated **dt** / steps (matrix harness); env-gate if wall-time large.
2. **MAC** stagger and/or consistent **open-x** BCs — eliminate checkerboard **without** surrogateamplification ~10³ to NaN on **65×17** (see [`rheology_pressure_poisson_roadmap.md`](research/rheology_pressure_poisson_roadmap.md)).
3. Pressure RHS consistent with staggered **div u*** per matrix.

---

## #8 THMC → **100%**

1. **Small-DOF shipped story** already partially verified — retain [`THMC_DENSE_NEWTON_MAX_STACKED_DOFS`](../src/physics/solvers/thmc_residual.rs) guards.
2. **Replace dense [`gauss_jordan_solve`](../src/physics/solvers/thmc_residual.rs)** monolith path with **JFNK**: matrix-free matvec + **BiCGSTAB/GMRES** (host **`f64`** acceptable for linear solve).
3. **Topological preconditioning:** Jacobi from diagonal / ILU(0) only with explicit sparsity pattern — benchmark before claiming **N > 10⁴**.
4. **AD-safe** stacked **‖R‖** early-exit at scale (matrix blocker — host-gated today must evolve per acceptance).
5. **Within-step u↔d** stagger + **adaptive dt** per matrix / Track 13 memos.

---

## #9 Statistical mechanics → **100%**

1. Extend tensor/state API for **(ρ*,T*)** (or parallel state tensor) per matrix.
2. Route **Johnson / virial** into [`upscale_potentials`](../src/physics/solvers/statistical_mechanics.rs) with reference **K**, documented **T** / cutoff — error **< stated tolerance** vs reference.

---

## #10 Mechanics — transient & contact → **100%**

1. **Either** default-CI **transient vector** mechanics on stated graph DOFs (Newmark / generalized-α + manufactured solution) **or** explicit v0.4 scope exclusion in brief/README + matrix alignment (choose one path per matrix “PR slice A/B”).
2. **Contact / Coulomb friction** with verification tests — until present, matrix row cannot read **100%**; mechanics combined row stays capped by **min(#2,#10)**.

---

## Grand integration — “Striatus + learned control” (optional 100% story — only if product demands)

1. **Striatus Gate A:** Cartridge shell proof + artefacts (**#1**).
2. **Gate B:** Manifold integration test: finite **`backward()`** through **THMC → mechanics strain → fracture** at **small N**; then scale only after #5/#8 memory wins.
3. **VRAM / 8GB claim:** requires explicit Burn/WGPU profiling task — not assumed by unit tests.

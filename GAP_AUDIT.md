# UMST gap audit (workspace snapshot)

**Scope:** `umst-manifold` and `umst-concrete-cartridge` aligned to the nine-phase solver plan in `composer-plans/umst_bleeding_edge_solvers.md`.  
**Canonical solver verification (lanes, features, CI-backed claims):** [`docs/Solver-Status.md`](docs/Solver-Status.md) — when this audit disagrees with that file on implementation status, **trust Solver-Status** and treat stale gap bullets below as narrative debt unless marked historical.  
**Effort key:** S = small (≤~1 dev-day), M = medium (~2–5 days), L = large (multi-week / research-grade).

---

## Recently tightened (implementation scope)

These items were previously called out as inconsistent or missing; the current `umst-manifold` tree addresses them at **engineering** scope; **default `cargo test` is green** locally, and **`cargo test --features solver-tests`** (run in root CI) is **green** on the verified snapshot — see **Verification: Cargo tests** below. Research-lane solver completeness vs the bleeding-edge **plan** is summarized per module in [`docs/Solver-Status.md`](docs/Solver-Status.md).

| Item | Status |
|------|--------|
| **`edges_b1` layout** | **Resolved** — canonical Burn row-major `[2, E]` (all sources, then all targets); enforced via [`EdgeTopology`](src/physics/topology.rs) and documented in [`mechanics`](src/physics/mechanics.rs) tests. |
| **CG packed solve** | **Resolved** — equilibrium uses **packed** conjugate gradient on free DOFs in [`VectorMechanicsSolver::solve_equilibrium`](src/physics/mechanics.rs) (not tensor-masked CG on the full vector). |
| **CBF \(k_{\mathrm{phys}}\)** | **Resolved** — [`ThermodynamicCBF::k_phys_dint_to_joules`](src/ai/cbf.rs) scales dissipation to joules; covered by [`tests/cbf.rs`](tests/cbf.rs). |
| **THMC hydration (explicit Euler)** | **Resolved at scaffold level** — [`ThmcSolver::step`](src/physics/solvers/thmc.rs) advances `hydration_alpha` with explicit Euler + Arrhenius placeholder. **Refinement:** outer Newton loop on thermal/hydraulic residuals \(R_T+R_h\) vs `tol` (not monolithic implicit Jacobian). **Still NOT DONE:** coupled implicit Jacobian, adaptive \(dt\) at Phase‑5 plan scope |
| **`ai::info_gain` module** | **Resolved** — [`src/ai/info_gain.rs`](src/ai/info_gain.rs) + unit tests (MSE / nodal flattening helpers). Still **not** mutual information or Landauer‑certified bits; epistemic table below still applies for true MI and sensor fusion.
| **Mechanics unit tests** | **Green** — `chain_bar_reduced_k_matches_tridiagonal`, `voigt_strain_and_hooke_shear_free_analytic`, and `one_d_bar_tip_displacement` **pass** on the default profile and with **`--features solver-tests`** (including the free‑DOF equilibrium relative L2 residual bound); see **Verification: Cargo tests**. |
| **Manifold golden-path integration** | **Partial** — [`tests/golden_path_physics_cbf.rs`](tests/golden_path_physics_cbf.rs): mechanics row runs [`ManifoldGateway`](src/ai/ppo.rs) + [`apply_physics_to_umst`](src/core/apply_physics.rs); experimental row runs [`TopologyPhysicsOrchestrator::run_plan_step`](src/physics/orchestration.rs) + [`ThermodynamicCBF`](src/ai/cbf.rs) + merge (no cross-crate cartridge). |
---

## Verification: Cargo tests (2026-05-10, P0 snapshot)

**Solver/module truth vs plan gaps:** use [`docs/Solver-Status.md`](docs/Solver-Status.md) (v0.4 brief + verification paths); this section only records aggregate `cargo` health.

| Crate / workspace | Command | Result |
|-------------------|---------|--------|
| `umst-manifold` | `cargo test` | **PASS** — lib + integration tests (`cbf`, `conservation`, `dec_identities`, `gateway_info_gain`, etc.); 0 failures |
| `umst-manifold` | `cargo test --features solver-tests` | **PASS** — all tests including `physics::mechanics::tests::one_d_bar_tip_displacement`; 0 failures |
| `umst-manifold` | `cargo test --all-features` | **PASS** — 0 failures on current feature matrix |
| `umst-concrete-cartridge` | `cargo test` (workspace) | **PASS** — 1 ignored test (`proof_status_doc` refresh) |

**Failed tests (P0 run):** **none** for default features, `solver-tests`, or `--all-features` on `umst-manifold`.

---

## Plan phases 1–9

Plan reference: [`composer-plans/umst_bleeding_edge_solvers.md`](../composer-plans/umst_bleeding_edge_solvers.md).

**Disposition (plan vs repo):** Phase **1** mechanics + emergence unit tests are **green** (default + **`solver-tests`**). Phases **2–9** use **`solver-stable`** vs **`solver-research`** lanes and feature flags as in [`docs/Solver-Status.md`](docs/Solver-Status.md): many **research** modules have **CI-verified** partial implementations (fracture, photonics, PNP, acoustics, THMC, stat mech) while **plan-complete** coupling, 2D/3D generality, or monolithic implicit stacks remain open. The table rows below state **remaining plan gaps** (not “nothing exists”). Composer refinements — see **Refinements verified in code**.

*Supersedes earlier audit rows that claimed “no SG”, “photonics stub”, and “topology_solver scaffold-only” without lane context — **superseded by** [`docs/Solver-Status.md`](docs/Solver-Status.md).*

| Phase | Plan intent | Gap | Owner crate | Effort |
|-------|-------------|-----|-------------|--------|
| **1** | Differentiable solid mechanics (vector DEC, CG on Burn) | **Partial:** packed CG + `edges_b1` + mechanics / emergence unit tests **green** (including **`solver-tests`** / `one_d_bar_tip_displacement`). Full 3D Voigt on general skeleton still reduced scope vs plan | `umst-manifold` | **M** |
| **1** | Full 3D Voigt elasticity + edge interpolation per plan | **Research lane:** Q1 hex / plate checks and adjoint paths per Solver-Status; **plan gap:** general \( \sigma = C:\varepsilon \) on arbitrary skeleton vs bar-network / targeted verification | `umst-manifold` | **L** |
| **2** | AT2 phase-field + spectral strain split + irreversibility | **Research (`fracture-at2`):** inner AT2 + staggered / chain harnesses **verified** (see [`docs/Solver-Status.md`](docs/Solver-Status.md)); **plan gaps:** Γ-convergence / multi-scale dissipation; THMC does not auto-wire post-mechanics \(\varepsilon(\mathbf u)\) into fracture; default build may still omit fracture | `umst-manifold` | **L** |
| **3** | Bingham Navier–Stokes + Chorin + thixotropy | **Research:** Poiseuille / Chorin smokes + 64×16 edge scaffold **verified**; **plan gaps:** steady benchmark vs analytic on full lattice, surrogate pressure Poisson, inlet/outlet BCs (see Solver-Status **DEFERRAL — Rheology**) | `umst-manifold` | **L** |
| **4** | Neural-SIMP + augmented Lagrangian + sensitivity filter | `DensityNet` / `TopologyOptimizer`: differentiable forward when **`solver-experimental`**; **plan gaps:** full MBB / crisp projection / filter loop as in composer plan | `umst-manifold` | **M** |
| **4** | `topology_solver` density-on-sheaf evolution | **Stable lane:** heat-equation / SIMP-adjacent evolution with **`tests/topology_continuation.rs`**, **`tests/topology_filter.rs`** (`topology-density-evolution`) — see Solver-Status; **plan gap:** sheaf density evolution beyond current stable scope | `umst-manifold` | **M** |
| **5** | Monolithic THMC Newton + autodiff Jacobian + adaptive \(dt\) | [`ThmcSolver::step`](src/physics/solvers/thmc.rs): explicit transport + hydration + quasi-static mechanics + **outer Newton** on \(R_T+R_h\); **research partial:** implicit \(T\)–\(\alpha\) block + one Newton step **verified** on tiny chains (Solver-Status); **plan gaps:** fully coupled implicit Newton–Krylov, adaptive \(dt\) | `umst-manifold` | **L** |
| **5** | Rheology inside same tick as THMC | Orchestrator: Bingham **not** folded into `ThmcSolver::step` | `umst-manifold` | **M** |
| **6** | PNP + Poisson + Scharfetter–Gummel | **Research:** SG NP + Thomas Poisson on MVP chains; implicit backward-Euler + damped Newton **optional** chain API; explicit Picard remains default on general path — see Solver-Status **DEFERRAL — Electrochemistry**; **plan gaps:** SG-style coupling + implicit Newton on **general graphs**, variable \(\varepsilon\), large-\(|z\Delta\phi|\) robustness | `umst-manifold` | **L** |
| **7** | Frequency-domain Maxwell curl–curl on DEC | **Research (`photonics`):** scalar TE FDFD Helmholtz, Thomas+PML, chain **`solve_maxwell_curl_curl`** vs scalar Helmholtz + **`photonics_fresnel`** — verified; **plan gaps:** full **2D/3D** DEC vector curl–curl, tensor permittivity, tighter Fresnel calibration (Solver-Status **DEFERRAL — Photonics**) | `umst-manifold` | **L** |
| **8** | Newmark elastodynamics + TopOpt coupling | **Research (`acoustics-newmark`):** dense-reference Newmark + plane-wave / energy tests **verified**; **plan gaps:** TopOpt coupling; brief return-map doc vs n=128 phase slip (ignored test); default build may still no-op [`step_wave`](src/physics/solvers/acoustics.rs) without feature | `umst-manifold` | **L** |
| **9** | Atomistic → continuum upscale (virial / partition functions) | **Research:** Vinet EOS + LJ bridge contract + **Johnson (1993)** reference documenting placeholder **`upscale_potentials`** mismatch — see Solver-Status; **plan gap:** virial / coexistence route **into** `upscale_potentials` (physical bridge, not just reference API) | `umst-manifold` | **L** |
| **All** | End-to-end loop: solvers → `PhysicalResult` → CBF → verified step | **Partial:** [`tests/golden_path_physics_cbf.rs`](tests/golden_path_physics_cbf.rs) — **mechanics row:** bar-network equilibrium → `PhysicalResult` → [`ManifoldGateway`](src/ai/ppo.rs) + [`apply_physics_to_umst`](src/core/apply_physics.rs); **experimental row:** [`TopologyPhysicsOrchestrator`](src/physics/orchestration.rs) / [`ThmcSolver::step`](src/physics/solvers/thmc.rs) → functor summary → [`ThermodynamicCBF`](src/ai/cbf.rs) + merge (2-node topology; **manifold-only**; no `umst-concrete-cartridge`; not phases 1–5 jointly). | `umst-manifold` + `umst-concrete-cartridge` | **L** |

---

## Epistemic / mutual information

| Gap | Owner crate | Effort |
|-----|-------------|--------|
| With **`information_density`** feature + non-zero [`ManifoldGateway::eta`](src/ai/ppo.rs), scalar reward adds **η · mean(information_density)** ([`core::traits`](src/core/traits.rs)); covered by [`tests/gateway_info_gain.rs`](tests/gateway_info_gain.rs). Default **`eta = 0`** and builds without the feature preserve prior reward shape | `umst-manifold` | **S** (wired) |
| PPO path still uses scalar / flattened **`info_gain`** surrogate ([`ai::info_gain`](src/ai/info_gain.rs)); **no nodal MI field** (information‑theoretic) or sensor fusion in reward | `umst-manifold` | **M** |
| CBF + Landauer tests exist (`tests/cbf.rs`) — **no gap** for basic entropy-accounting semantics | — | — |
| Emergence / dissipation hotspots complementary to MI — not unified into a single epistemic objective | `umst-manifold` | **S** |

---

## CI readiness

Workspace root [`.github/workflows/rust-solvers.yml`](../.github/workflows/rust-solvers.yml) (MaOS-Workspace) triggers on changes under **`umst-manifold/**`**, **`umst-concrete-cartridge/**`**, or the workflow file. Two jobs:

1. **`umst-manifold-rust`** — `working-directory: umst-manifold`: **physics Host-tensor guard** (`scripts/check_physics_no_gradient_break.sh`), **`dtolnay/rust-toolchain@stable` with `toolchain: 1.88`**, **`cargo fmt --check`**, **`cargo clippy --all-targets -- -D warnings`**, **`cargo test`**, **`cargo test --features solver-tests`** (no separate `--all-features` step in CI; run locally per **Verification** table).
2. **`cartridge`** — `working-directory: umst-concrete-cartridge`: same toolchain pin when both `umst-manifold/Cargo.toml` and `umst-concrete-cartridge/Cargo.toml` exist (`hashFiles` guards); **`cargo fmt --check`**, **`cargo clippy`**, **`cargo test`**.

**Readiness note:** On the P0 verification snapshot, **`cargo test --features solver-tests` passes** in `umst-manifold` (see **Verification**). CI still depends on the workflow’s **1.88** pin matching **`rust-toolchain.toml`** / transitive MSRV.

The crate-local [`.github/workflows/rust.yml`](.github/workflows/rust.yml) also uses **stable** for build/test; its `rustfmt` / clippy job is marked **advisory** (`continue-on-error: true`), so **root `rust-solvers.yml` is the stricter fmt + solver-tests gate** for monorepo CI.

---

## Toolchain

| Gap | Owner crate | Effort |
|-----|-------------|--------|
| Root `rust-solvers.yml` **solver-tests** step: `cargo test --features solver-tests` is **green** on the P0 verification snapshot | `umst-manifold` | **S** |
| Per-crate workflow does **not** run `solver-tests`; **root** `rust-solvers.yml` does — align expectations when debugging CI only on `umst-manifold/.github` | `umst-manifold` | **S** |
| `wgpu` backend optional; **not exercised** in CI (Metal/Vulkan variance) | `umst-manifold` | **S** |
| `bincode = "=2.0.0-rc.3"` pin — upgrade path tied to `burn` compatibility | `umst-manifold`, `umst-concrete-cartridge` | **S** |
| Plan mentions MMS / conservation / regression suites — **not** wired as named CI steps beyond default tests | `umst-manifold` | **M** |
| **PATH / rustup:** If `which cargo` resolves to Homebrew (`/opt/homebrew/bin/cargo`) ahead of rustup, `rust-toolchain.toml` may not apply; use **`rustup run 1.88 cargo`** (or put `~/.rustup/toolchains/1.88-aarch64-apple-darwin/bin` first on `PATH`) so fmt/clippy/tests match **`rustc 1.88.0`**. | dev env | **S** |

---

## Cartridge / publish

| Gap | Owner crate | Effort |
|-----|-------------|--------|
| `ConcreteCartridge::compute_topology`: **default** path = heat Laplacian + placeholders; full THMC stack only with `solver-experimental` | `umst-concrete-cartridge` | **M** |
| With `solver-experimental`, intermediate `free_energy` / `safety_margin` / `cost` from THMC block are **overwritten** before `PhysicalResult` return — experimental physics summary unused | `umst-concrete-cartridge` | **S** |
| `thmc_state_from_umst` reads **[`VECTOR_MECHANICAL_DISPLACEMENT`](src/core/umst_schema.rs)** (`vector_features` slot 0) when `F_vectors > 0`; zeros only if no vector columns | `umst-concrete-cartridge` | **S** (done) |
| Spatial **\(G_c\)**: optional UMST column **[`SCALAR_FRACTURE_ENERGY_GC`](src/core/umst_schema.rs)** (`F_scalars > 5`); else profile scalar broadcast. No `PhysicalResult` field; no auto-fill from mesh/calibration grids yet | `umst-concrete-cartridge` | **S** (partial) |
| Workspace `[patch]` uses path `umst-manifold`; **published** `umst-concrete-cartridge` depends on **git** `main` — drift risk vs crates.io `umst-manifold` | `umst-concrete-cartridge` | **S** |
| Manifold **`README.md`** **Citing** section states **reserved** Zenodo DOI until v0.1.0 deposit; **live deposit + docs.rs** alignment for the crate remains release work | `umst-manifold` | **S** |
| Cartridge **`README.md`** documents **live** dataset Zenodo **[14921019](https://zenodo.org/records/14921019)** (calibration provenance) | `umst-concrete-cartridge` | **S** (done) |
| PyPI / bindings (`umst-py`) and MCP (`umst-mcp`) packaging versioning vs manifold semver | `umst-concrete-cartridge` workspace | **S** |

---

## Document history

- Created as part of a workspace gap audit; phases trace to `composer-plans/umst_bleeding_edge_solvers.md`.
- **2026-05-10:** Re-ran `cargo test` in `umst-manifold`; recorded resolved scaffolding (`edges_b1`, packed CG, CBF `k_phys`, THMC hydration Euler step, `info_gain` module); updated failures (mechanics residual + emergence); Phase 2–9 and end-to-end integration remain **NOT DONE** as before.
- **2026-05-10 (later):** Post-fix `cargo test` **PASS** in `umst-manifold` (default features) and `umst-concrete-cartridge` workspace; **`cargo test --features solver-tests`** still **FAIL** (`one_d_bar_tip_displacement`). Audit updated. Added **CI readiness** (root `rust-solvers.yml`, stable toolchain, fmt gate); Toolchain table adjusted for split workflow behavior.
- **2026-05-10 (P0):** Ran **`cargo fmt`**, **`cargo fmt --check`**, **`cargo clippy --all-targets -- -D warnings`**, **`cargo test`**, **`cargo test --features solver-tests`**, **`cargo test --all-features`** (and **`cargo clippy --all-features --all-targets -- -D warnings`**) under **`rustc 1.88.0`** via **`rustup run 1.88`**; all **PASS**. Restored **Deferred work** after accidental truncation; updated tables for **green** `solver-tests`; added **PATH / rustup** note.
- **2026-05-10:** Manifold-local golden path documented in [`tests/golden_path_physics_cbf.rs`](tests/golden_path_physics_cbf.rs): `golden_path_mechanics_*` (DEC equilibrium → `PhysicalResult` → `ThermodynamicCBF` + [`ManifoldGateway`](src/ai/ppo.rs) → `apply_physics_to_umst`); with **`solver-experimental`**, `golden_path_thmc_*` runs [`TopologyPhysicsOrchestrator::run_plan_step`](src/physics/orchestration.rs) then CBF + merge (finite synthetic [`PhysicalResult::damage`] for write-back where AT2 may NaN on the 2-node demo). Phase **All** row updated — cross-crate golden path still open (P1).
- **2026-05-10:** Projected bar-network CG applies **`boundary_mask`** after each `bar_matvec` on the search direction (**`Ap = P·(K·(P·p))`**); **`TopologyOptimizer::optimize_step`** uses **`masked_dot`** for compliance (aligned with **`optimize_step_simplite`**); SIMP parity test calls the current **`optimize_step`** signature. `src/physics/` production: no **`into_data`** / **`into_scalar`** (grep); those APIs remain in **`#[cfg(test)]`** paths only.
- **2026-05-10:** Synced phase rows and **Deferred work** with repo-verified composer refinements (mechanics CPU path, THMC Newton residuals, `optimize_step` behind **`solver-experimental`**, electro drift, policy mask); refreshed **`composer-plans/umst_solver_refinements.md`** phase verdicts.
- **2026-05-10:** **Verification** section rename; **CI readiness** updated for dual-job `rust-solvers.yml` + cartridge path filters; epistemic table + **Cartridge / publish** README rows aligned with **`information_density` / η** wiring and in-tree README disclosure; P1 checklist split (information density vs emergence); new P0 ticks for cartridge CI + README.
- **2026-05-11:** **Plan phases 1–9**, **Verification** intro, **Deferred work** (P3/P4), and **Sorted issue index** (P2) updated so stale claims (e.g. photonics stub, SG absent, topology_solver “scaffold only”) match CI-backed truth in [`docs/Solver-Status.md`](docs/Solver-Status.md); that doc is **canonical** for solver/module status. Earlier history entries **superseded as verification authority** by Solver-Status where they conflict on phases 2–9.

---

## Deferred work

In `Cargo.toml`, `solver-experimental` groups the leaf features `fracture-at2`, `acoustics-newmark`, `topology-density-evolution`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-voigt-cauchy`, `rheology-bingham`, and `photonics-fdfd`; `solver-tests` enables that umbrella.

### P0 — CI, format, tests

- **MSRV / pins:** `rust-toolchain.toml` pins **rustc 1.88** (full optional graph / `--all-features`; see file header). `Cargo.toml` pins **`bincode = "=2.0.0-rc.3"`** (Burn 0.13 API surface) and **`time = "=0.3.40"`** (newer `time` pulls MSRV above common toolchains). Do not relax pins without checking Burn and transitive MSRV.
- [x] Keep root `rust-solvers.yml` gate green: `cargo fmt`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo test --features solver-tests`, `cargo test --all-features` (verified `rustc 1.88.0` / `rustup run 1.88`; see **Toolchain** PATH note).
- [x] Root `rust-solvers.yml` **`cartridge`** job: `fmt` / `clippy` / `cargo test` in `umst-concrete-cartridge/` when both crate manifests exist (`hashFiles` guards); workflow `paths` include `umst-concrete-cartridge/**`.
- [x] Manifold **`README.md`** discloses reserved Zenodo DOI until v0.1.0 deposit (**Citing this work**); cartridge README documents live dataset Zenodo for calibration CSVs.
- [ ] Track crate-local `umst-manifold/.github/workflows/rust.yml` (advisory) vs root strict gate when debugging CI-only failures.
- [ ] Add or promote MMS / conservation / regression suites to explicit CI steps when harness exists (currently implicit in default tests only).

### Refinements verified in code ([`composer-plans/umst_solver_refinements.md`](../composer-plans/umst_solver_refinements.md))

- [x] **Mechanics CPU fallback removed** from production solver — packed/tensor CG + `bar_matvec`; `.into_data()` only under `#[cfg(test)]` (`src/physics/mechanics.rs`).
- [x] **THMC Newton outer loop** — iterate transport + mechanics; exit when \(R_T+R_h<\) `tol` or warn on exhaustion (`src/physics/solvers/thmc.rs`).
- [x] **TopologyOptimizer `optimize_step`** — full differentiable forward (DensityNet → SIMP → equilibrium) when **`solver-experimental`**; without it, no-arg `optimize_step(&mut self)` remains `{}` (`src/ai/topology.rs`).
- [x] **Electrochemistry drift** — Nernst–Planck drift surrogate `j_drift` using discrete \(\Delta\Phi\) (`lap_phi`) (`src/physics/solvers/electrochemistry.rs`).
- [x] **Policy mask** — `UnifiedMaterialStateTensor::apply_policy_mask` / `project_all_scalars` (`src/core/tensors.rs`).

### P1 — Integration: gateway emergence hook, publish crate

- [x] **`information_density` in gateway:** with crate feature `information_density` and non-zero [`ManifoldGateway::eta`](src/ai/ppo.rs), scalar reward adds **η · mean(information_density)** per [`core::traits`](src/core/traits.rs); [`tests/gateway_info_gain.rs`](tests/gateway_info_gain.rs).
- [ ] **Emergence in reward:** wire `core::emergence` nodal / hotspot signals into the scalar reward (beyond existing `dissipation` / `cost` / `free_energy` terms and the `info_gain` surrogate); unify toward `core::traits` epistemic story.
- [ ] Golden-path integration test spanning manifold + `umst-concrete-cartridge` — manifold-local chain exists: [`tests/golden_path_physics_cbf.rs`](tests/golden_path_physics_cbf.rs).
- [ ] Publish `umst-manifold` on crates.io with docs.rs; align `umst-concrete-cartridge` git vs path `[patch]` drift and semver story.
- [ ] Release hygiene: **live** manifold Zenodo deposit (reserved DOI already documented in README), **docs.rs**, PyPI / `umst-py` / `umst-mcp` versioning vs manifold semver.

### P2 — Phase 1 Voigt in `solve_equilibrium`

- [ ] Full 3D Voigt solid elasticity \( \sigma = C : \varepsilon \) on general skeleton inside `VectorMechanicsSolver::solve_equilibrium` (today reduced bar-network / partial scope vs plan).

### P3 — Phases 2–5 full physics

*Research-lane milestones and verification paths:* [`docs/Solver-Status.md`](docs/Solver-Status.md). Items below are **plan-complete** gaps vs `composer-plans/umst_bleeding_edge_solvers.md` (not “no implementation”).

- [ ] Phase 2: Γ-convergence / multi-\((l_0,h)\) dissipation limits; THMC-orchestrated feed of post-mechanics strain into fracture; spectral split / irreversibility to full composer spec (`fracture-at2` partial — Solver-Status).
- [ ] Phase 3: Steady Chorin vs analytic on 64×16 (or equivalent); surrogate pressure Poisson + BCs (`rheology-bingham` partial — Solver-Status).
- [ ] Phase 4: Neural-SIMP + augmented Lagrangian + sensitivity filter to plan completeness; sheaf density evolution beyond stable-lane topology continuation/filter (`topology-density-evolution` shipped — Solver-Status).
- [ ] Phase 5: Monolithic implicit THMC Newton + autodiff Jacobian + adaptive \(dt\); fold Bingham rheology into `ThmcSolver::step` (track 13 partial — Solver-Status).

### P4 — Phases 6–9

- [ ] Phase 6: Coupled implicit Newton on **general graphs**; variable \(\varepsilon\); SG robustness at large \(|z\Delta\phi|\) — MVP chain SG + optional implicit Newton **exist** (`electrochemistry-pnp` — Solver-Status **DEFERRAL — Electrochemistry**).
- [ ] Phase 7: Full **2D/3D** DEC vector curl–curl, tensor permittivity, tighter Fresnel calibration — chain TE Helmholtz + `solve_maxwell_curl_curl` + Fresnel tests **exist** (`photonics` — Solver-Status **DEFERRAL — Photonics**).
- [ ] Phase 8: TopOpt coupling with acoustics; align brief return-map acceptance — Newmark / plane-wave suite **exists** behind `acoustics-newmark` (Solver-Status).
- [ ] Phase 9: Virial / coexistence route **into** `upscale_potentials` — Johnson EOS reference + placeholder honesty tests **exist**; physical LJ→continuum bridge still open (Solver-Status **DEFERRAL — Statistical mechanics**).

### P5 — True MI, formal bridge

- [ ] Nodal mutual-information / Landauer-certified information field (replace scalar `ai::info_gain` surrogate in PPO reward).
- [ ] Formal multi-scale bridge and epistemic objective unifying MI, emergence hotspots, and CBF dissipation accounting.

---

## Sorted issue index

**As of 2026-05-11.** Per-solver verification — [`docs/Solver-Status.md`](docs/Solver-Status.md). Pointers into this document only; detail stays in the sections cited.

### P0 — Correctness / CI

1. Reconcile manifold-local [`.github/workflows/rust.yml`](.github/workflows/rust.yml) (advisory `continue-on-error`) with the stricter root [`rust-solvers.yml`](../.github/workflows/rust-solvers.yml) gate when debugging CI-only failures (**Deferred work** § P0; **CI readiness**).
2. Add or promote MMS / conservation / named regression suites to explicit CI steps once a harness exists; today the main signal is default `cargo test` (**Deferred work** § P0; **Toolchain** table).
3. Keep developer `PATH` / rustup ordering consistent with the **1.88** workflow pin so `rust-toolchain.toml` actually applies (`rustup run 1.88 cargo …` if Homebrew `cargo` shadows rustup) (**Toolchain** table).
4. `wgpu` optional backend is not exercised in CI (Metal/Vulkan variance) (**Toolchain** table).

### P1 — Integration / cartridge

1. Golden-path integration test spanning `umst-manifold` and `umst-concrete-cartridge`; manifold-local coverage only in [`tests/golden_path_physics_cbf.rs`](tests/golden_path_physics_cbf.rs) (**Deferred work** § P1; **Plan phases** row *All*).
2. `ConcreteCartridge::compute_topology`: default path remains heat Laplacian + placeholders; full THMC stack only with `solver-experimental` (**Cartridge / publish**).
3. With `solver-experimental`, intermediate `free_energy` / `safety_margin` / `cost` from the THMC block are overwritten before `PhysicalResult` return — experimental physics summary unused (**Cartridge / publish**).
4. Spatial \(G_c\): optional [`SCALAR_FRACTURE_ENERGY_GC`](src/core/umst_schema.rs) vs profile broadcast; no `PhysicalResult` field or mesh/calibration auto-fill yet (**Cartridge / publish**).
5. Workspace `[patch]` vs published cartridge depending on git `main` — drift risk vs crates.io `umst-manifold` (**Cartridge / publish**).
6. Publish `umst-manifold` on crates.io with docs.rs; align cartridge git vs semver story (**Deferred work** § P1; **Cartridge / publish**).
7. Live manifold Zenodo deposit (README currently discloses reserved DOI), docs.rs, and PyPI / `umst-py` / `umst-mcp` versioning vs manifold semver (**Deferred work** § P1; **Cartridge / publish**).
8. Wire `core::emergence` nodal / hotspot signals into the scalar PPO reward beyond `dissipation` / `cost` / `free_energy` and the `info_gain` surrogate; align with **Epistemic / mutual information** narrative (**Deferred work** § P1; **Epistemic / mutual information**).

### P2 — Research / deferred physics

1. Full 3D Voigt solid \( \sigma = C:\varepsilon \) on a general skeleton inside `VectorMechanicsSolver::solve_equilibrium` vs bar-network / targeted research verification (**Deferred work** § P2; **Plan phases** phase **1**; Solver-Status mechanics row).
2. Phase-field fracture: Γ-limit / THMC strain feed / full composer irreversibility vs **`fracture-at2`** harnesses in Solver-Status (**Deferred work** § P3; **Plan phases** phase **2**).
3. Bingham: steady 64×16 benchmark vs analytic + pressure Poisson vs Poiseuille smokes (**Deferred work** § P3; Solver-Status **DEFERRAL — Rheology**).
4. Neural-SIMP composer completeness vs `TopologyOptimizer`; topology evolution beyond stable **`topology_continuation` / `topology_filter`** (**Deferred work** § P3; **Plan phases** phase **4**).
5. Monolithic implicit THMC + adaptive \(dt\); fold Bingham into `ThmcSolver::step` vs track 13 partial (**Deferred work** § P3; **Plan phases** phase **5**).
6. Electrochemistry: general-graph implicit Newton + variable \(\varepsilon\) + large-\(|z\Delta\phi|\) robustness vs MVP SG + optional chain Newton (**Deferred work** § P4; Solver-Status phase **6** / **DEFERRAL — Electrochemistry**).
7. Photonics: full 2D/3D DEC Maxwell vs chain TE Helmholtz + **`solve_maxwell_curl_curl`** + Fresnel CI (**Deferred work** § P4; Solver-Status **DEFERRAL — Photonics**).
8. Acoustics: TopOpt coupling + return-map brief vs Newmark / plane-wave verification behind feature (**Deferred work** § P4; Solver-Status acoustics row).
9. Statistical mechanics: physical bridge into **`upscale_potentials`** vs Johnson reference + placeholder tests (**Deferred work** § P4; Solver-Status phase **9**).
10. Nodal mutual-information / Landauer-certified information field replacing scalar `ai::info_gain` in PPO reward (**Deferred work** § P5; **Epistemic / mutual information**).
11. Formal multi-scale bridge and single epistemic objective unifying MI, emergence hotspots, and CBF dissipation accounting (**Deferred work** § P5; **Epistemic / mutual information**).

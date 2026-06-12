# Solver status (v0.4)

This page is the **authoritative** mapping from solver surfaces to **Cargo feature lanes**, **integration tests** cited in CI, and **honest** completion signals. A short parallel index lives in [`PROOF-STATUS.md`](PROOF-STATUS.md) (Track J3). Formal notation and validation context: [`Mathematical-Foundations.md`](Mathematical-Foundations.md), [`Validation.md`](Validation.md). Bibliography skeleton: [`References.bib`](References.bib).

**Cartridge / Striatus work** (shell topology, rib harness, print-ready JSON) is owned by the separate **[umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge)** repository; see that repo’s `docs/Solver-Status.md` and `docs/Striatus.md` for commands and artefact paths.

---

## Completion column

**Completion (%)** is a coarse label in **{0, 25, 50, 75, 100}**, aligned with the v0.4 verification narrative when that material is available in your checkout (historically shipped beside this crate as numbered rows **#1–#10**). It is **not** “fraction of tests green.” **100** means the public acceptance story for that lane is met end-to-end on the stated CI path; lower values mean partial milestones, smokes, or documented gaps. **Do not** treat **100** as permission to claim closure unless the underlying acceptance text is satisfied.

**Compiled ≠ validated:** a green **`solver-research-check-pr`** job is **`cargo check` only** — it does **not** execute physics integration tests. Say “verified on CI” only for behaviours exercised by **`cargo test`** on the paths named in **Verification** (or the workflow’s explicit **`--release`** Phase-4 crates). See audit finding **#7** in [`SOLVER_QUALITY_AUDIT.md`](SOLVER_QUALITY_AUDIT.md).

**Smoke vs acceptance (cartridge B6):** **`UMST_SHELL_RIB_FULL_ITERS < 200`** is **smoke** — greyness, planar-variance, and **`c1`** gates are skipped. A **20-outer** or **60-outer PASS** is **not** **200-outer B6 acceptance**. The 200-outer logit-offset run is **MISTRIAL†** (sym-boundary measured wrong state); honest re-run with Voigt **p = 1** **`c0_uniform`** gate is pending — see **[umst-concrete-cartridge `docs/Solver-Status.md`](../../umst-concrete-cartridge/docs/Solver-Status.md)**. Audit finding **#8**: future readers will conflate smoke and acceptance unless this ledger forbids it.

---

## Feature lanes (`Cargo.toml`)

| Meta-feature | Includes (intent) |
| --- | --- |
| **`solver-stable`** | `topology-density-evolution`, `statistical-mechanics-vinet` — kernels and tests intended to stay green on narrow CI. |
| **`solver-research`** | Opt-in scaffolds: `fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-adjoint`, `mechanics-adjoint-q1-hex`, `rheology-bingham`, `photonics-fdfd`, `statistical-mechanics-johnson-reference`, … (full list in `[features]`). |
| **`solver-experimental`** | Union **`solver-stable` ∪ `solver-research`**; same graph as meta **`solver-tests`**. |

**Canonical vs cfg names:** e.g. **`photonics-fdfd`** forwards to **`photonics`** for `#[cfg]`; **`electrochemistry-pnp`** forwards to **`electrochemistry-mvp`**. Deprecated alias **`photonics-scaffold`** → **`photonics-fdfd`**.

**GPU:** Optional `wgpu` is not exercised in CI; portable builds and throughput work use **`ndarray`** (and **`mac-fast`** on Apple Silicon where applicable).

---

## CI (`.github/workflows/rust.yml`)

| Job | Role |
| --- | --- |
| **`solver-status`** | `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` — stable rows must cite real tests; statistical-mechanics row must list the full `statmech_*` set (see `scripts/check_solver_status.py`). |
| **`build-test`** | Default features: `cargo build` / `cargo test`. |
| **`solver-stable-pr`** (PR) | `cargo test --features solver-stable`. |
| **`solver-research-check-pr`** (PR) | **`cargo check` only** (compile graph for `solver-stable` ∪ `solver-research`) — **not** physics validation; do not read green as “solver verified.” |
| **`phase4-verification-pr`** (PR) | Release tests: `thmc_monolithic_newton_chain`; `photonics_curl_curl_{2d_patch,3d_brick}` with `photonics-fdfd`; `statmech_lj_johnson_upscale_bridge` — all with `solver-research` as in the workflow. |
| **`lint`** | `cargo fmt --check` and `cargo clippy --all-targets --features solver-experimental -- -D warnings` (toolchain pinned in workflow). |
| **`research-stack`** | `main` / `workflow_dispatch`: `cargo test --release --features solver-experimental` with one retry. |

---

## Main solver table

Use **Verification** paths as the contract for what “implemented / verified” means in prose. **Stable** lane rows must have non-empty **Verification** (enforced by `check_solver_status.py`).

| Solver | Lane | Completion (%) | Verification | Notes |
| --- | --- | --- | --- | --- |
| `solvers::topology_solver` (`TopologyOptimizer`, density evolution) | stable | **25** | `tests/topology_continuation.rs`, `tests/topology_filter.rs` | Feature **`topology-density-evolution`**. Conservative stable entry: heat-equation / SIMP-style density evolution. Shell **B6 / B8 / Track L** acceptance and cartridge commands live in **umst-concrete-cartridge** (not repeated here). |
| `mechanics::VectorMechanicsSolver`, `adjoint::AdjointCompliance`, `adjoint_q1_hex::AdjointComplianceQ1Hex` | research | **25** | `tests/verification/mechanics_analytic.rs`, `tests/verification/adjoint_compliance_analytic.rs`, `tests/verification/adjoint_q1_hex_compliance_analytic.rs`, `tests/verification/adjoint_q1_hex_matches_bar_in_limit.rs` | Features **`mechanics-voigt-cauchy`**, **`topology-density-evolution`**, **`mechanics-adjoint-q1-hex`**. Shipped quasi-static bar / plate / Q1-hex paths + discrete adjoint checks. **Coupled production paths** (THMC **`R_u`**, fracture stagger, monolithic Newton, adjoint TO, protocols) still call **bar-network** mechanics — **research-grade**, mechanism caveat documented; migration tracked (**Wave 3**). Striatus shell / cartridge TO uses **Q1 hex**. Kirchhoff split: **R2.1-A** — `plate_centre_deflection_kirchhoff_ssss_q1_hex_within_five_percent` (38²×4, lateral **`u_z`**, **≤ 5.5 %**, stable-CI); **R2.1-B** — `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate` **`#[ignore]`**, **O(1)** error on brick BCs — **open**. **Contact / friction:** not implemented. See **§ Mechanics**. |
| `solvers::fracture_field` (`PhaseFieldFractureSolver`) | research | **50** | `tests/verification/fracture_gamma_convergence.rs`, `tests/verification/staggered_fracture_mechanics_chain.rs`, `tests/verification/staggered_ud_loop_milestone.rs`, `tests/verification/thmc_drying_shrinkage.rs` | Feature **`fracture-at2`**. AT2 relaxation, length-scale and **partial** Γ-type harnesses, staggered bar milestones, THMC kinematic strain parity vs fracture helpers. **Open:** sharp-interface Γ-limit with driven **ψ⁺**, broader \((l_0,h)\), within-step THMC **u↔d** stagger. **Compressive crushing / plasticity caps** are **not** in this solver (spectral tensile **ψ⁺** drive only). See **§ Fracture**. |
| `solvers::acoustics` (`AcousticWaveSolver`, `AcousticNewmarkBar1dPeriodic`) | research | **100** | `tests/verification/acoustics_plane_wave.rs` | Feature **`acoustics-newmark`**. **100 % = 1-D periodic bar** (`AcousticNewmarkBar1dPeriodic`) only — Newmark vs dense reference; **return-map** at **n = 100** and **`plane_wave_return_map_n128_l2_within_two_percent`** (period \(T=2\pi/\Omega\), lumped \(m=\rho\Delta x\)). Runs on **`research-main`**, not default PR. **`AcousticWaveSolver`** (3-D graph + bar GMRES, f32) is **SMOKE / research** — not part of the 100 % row. See **§ Acoustics**. |
| `solvers::electrochemistry` (`ElectroChemicalSolver`) | research | **75** | `tests/verification/pnp_debye_layer.rs` | Feature **`electrochemistry-pnp`**. MVP chain: SG NP + Thomas Poisson; **explicit Picard** path in `solve_pnp_step` / `solve_pnp_step_experimental` (no **`NewtonPnpContext`** on that default path). **λ\_D** screening-length LS gates on **256**-cell chains run on the **`pnp_debye_layer`** surface when the feature is enabled (quasi-steady trajectory + `mesh_spacing = h` harness). Opt-in **`solve_pnp_step_dispatch`** + implicit Newton chain for research users. **Open:** general-graph implicit Newton, removing worst-case dense \((3N)^2\) scratch at large **N**, variable ε on non-chain topologies. See **§ Electrochemistry**. |
| `solvers::photonics` (`PhotonicsSolver`, `PhotonicsHelmholtzSolver`) | research | **50** | `tests/verification/photonics_fresnel.rs`, `tests/verification/photonics_curl_curl_2d_patch.rs`, `tests/verification/photonics_curl_curl_3d_brick.rs`, `tests/verification/photonics_curl_curl_stub_default_build.rs` | Feature **`photonics-fdfd`**. TE Helmholtz / 1-D curl–curl reductions; small embedded **DEC** patch tests (PR **phase4** path); **stub** documents default-build no-op when **`photonics`** is off. **Open:** dual Hodge / metrics, sparse Krylov, complex ε + PML on patches, BCs beyond gauge pin, production 3D assembly. See **§ Photonics**. |
| `solvers::rheology_flow` (`BinghamFlowSolver`) | research | **50** | `tests/verification/rheology_poiseuille.rs` | Feature **`rheology-bingham`**. **Bingham only** — **no Herschel–Bulkley** implementation (README §6 still says Herschel–Bulkley; see [`README.md`](../README.md) drift note). Analytic Poiseuille / Bingham references, Chorin split smokes, **Jacobi–PCG** pressure Poisson on **−𝒞**. **research-stack** short-channel smokes are **finite / bracket** guards, **not** long-run steady **L²** vs developed Poiseuille (`#[ignore]`). **Open:** MAC / open **x** BCs; steady channel acceptance. See **§ Rheology**. |
| `solvers::thmc` (`ThmcSolver`, …) | research | **75** | `tests/verification/thmc_drying_shrinkage.rs`, `tests/verification/thmc_monolithic_newton_chain.rs` | Feature **`thmc-coupled`**. **75 % = milestone label**; audit **verification axis ~50–60 %** (residual trust, dead **`ThmcSolver.tol`** on live coupled path, post-Newton diagnostics on **explicit-Euler** Laplacian mismatch vs implicit BE stacked **R** — **Wave 1**). Drying + shrinkage + implicit **(T, α)** on tiny graphs; monolithic **≤ 64** DOF dense Newton **partially closed** (`phase4-pr`). Production-scale monolith / JFNK, adaptive **dt**, **u↔d** stagger — **open**. **`R_u`** uses bar-network mechanics. See **§ THMC**. |
| `solvers::statistical_mechanics` | research | **25** | `tests/verification/statmech_vinet_eos.rs`, `tests/verification/statmech_lj_bridge_contract.rs`, `tests/verification/statmech_lj_johnson_eos_reference.rs`, `tests/verification/statmech_lj_johnson_upscale_bridge.rs`, `tests/verification/statmech_mechanics_fracture_bridge.rs` | **`statistical-mechanics-vinet`** — scalar Vinet EOS stable slice. **`[B,2]`** LJ bridge = documented placeholder. **`[B,4]` upscale = third-order virial surrogate** (Padé **B₃\***), **not** Johnson **`K`** validation in Burn — **f64** Johnson (1993) reference is the strong lane (`statmech_lj_johnson_eos_reference.rs`). **γ\_gc** physical bridge **open**. **P0:** `statmech_mechanics_fracture_bridge.rs` may need **`Cargo.toml` `[[test]]`** registration — cited by `check_solver_status.py`. See **§ Statistical mechanics**. |

**Prose rule (Track J1):** say “verified on CI” only for behaviours exercised by the **Verification** paths above (plus the workflow’s explicit **`--release`** crates for Phase-4 slices). **Exclude** **`solver-research-check-pr`** (`cargo check` only).

---

## Research tracks (12–16)

Long-form numbered memos under `docs/research/` were **removed** from this repository to reduce drift. **Scopes** (fracture coupling, THMC monolith, implicit PNP, DEC photonics, LJ/stat-mech bridge) are summarized in the **§ Per-lane** sections and the **Open themes** list below — cite **tests and `src/`** as the live sources of truth.

---

## Open themes (checklist)

| # | Theme | Completion (%) | Direction |
| --- | --- | --- | --- |
| 1 | Topology / shell / Striatus | **25** | Cartridge B6/B8, artefact budgets, `gates_track_b8_all_pass` — see sibling repo docs. |
| 2 | Mechanics — thin plate Kirchhoff accuracy | **25** | **R2.1-A** green (38² lateral **`u_z`**); **R2.1-B** brick `plate_r21_*` **`#[ignore]`** open. Align BCs for true brick-path gate. |
| 3 | Fracture — Γ limit, ψ⁺ drives, THMC stagger | **50** | Extend harnesses beyond fixed partial-Γ schedules. |
| 4 | Acoustics | **100** | Optional: graph-assembled stiffness beyond 1-D periodic bar benchmark. |
| 5 | Electrochemistry — scale & graph generality | **75** | Matrix-free / banded solvers; nonlinear SG beyond linearised Debye gates. |
| 6 | Photonics — production DEC + solvers | **50** | Metrics, sparse inner loops, BCs/PML. |
| 7 | Rheology — developed channel fidelity | **50** | MAC / open BCs; long-run **L²** acceptance. |
| 8 | THMC — large‑N monolith | **75** | Krylov–JFNK, adaptive time stepping, stagger policy. |
| 9 | Statistical mechanics — virial / coexistence in **`upscale_potentials`** | **25** | Physical **γ\_gc**; AD-safe **`K`** in Burn. |
| 10 | Transient **vector** solid dynamics & contact | **25** | No default CI **2‑DOF vector** transient stack on the mechanics graph; contact out of scope until a new verification row exists. |

---

## Per-lane notes

### Mechanics

- **Modules:** `VectorMechanicsSolver::solve_equilibrium`, `AdjointCompliance`, `AdjointComplianceQ1Hex` (see `src/physics/mechanics.rs`, `adjoint.rs`, `adjoint_q1_hex`).
- **Coupled paths (bar-network):** THMC **`R_u`**, fracture stagger, monolithic Newton **`R_u`**, adjoint TO, and protocol/AI topology callers still use **bar-network** quasi-static mechanics — **research-grade**, mechanism caveat (9×8×2 roof stall ~0.94 rel). Striatus / cartridge shell path uses **Q1 hex**. Migration **Wave 3** (THMC **`R_u`** first).
- **Kirchhoff §R2.1:** **R2.1-A** — `plate_centre_deflection_kirchhoff_ssss_q1_hex_within_five_percent` @ **38²×4**, lateral **`u_z`**, **≤ 5.5 %** (stable-CI). **R2.1-B** — `plate_r21_kirchhoff_ssss_centre_w_within_5pct_brick_path_gate` **`#[ignore]`**, brick BCs, **O(1)** error — **open**.
- **Q1 hex:** shipped operator uses a **unified** **D** on the B-bar / centroid-shear strain path; a naive dev/vol split was **rejected** because it regressed slender-column consistency.
- **Ignored slender Q1 vs bar:** z-skeleton bar limit matches closed form; the **1×1** brick cross-section remains much more compliant — documented harness limitation, not “solver wrong” without refined 1D reduction.

### Fracture

- **Driver:** spectral **ψ⁺** (tensile principal strains); **no** compressive crushing / Drucker–Prager cap in **`PhaseFieldFractureSolver`**.
- **THMC:** when SI **`[N,3]`** embedding holds, post-mechanics strain feeds fracture; **`thmc_drying_shrinkage`** includes kinematic parity tests vs **`strain_tensor_for_fracture_after_mechanics`**.

### Acoustics

- **100 % scope:** **`AcousticNewmarkBar1dPeriodic`** only — verified Newmark + return-map suite in `tests/verification/acoustics_plane_wave.rs` (**`research-main`**).
- **`AcousticWaveSolver`:** nodal graph + bar GMRES (f32) — **SMOKE / research**; not certified; optional extension beyond 1-D bar (open theme #4).

### Electrochemistry

- **Default path:** explicit Picard coupling; implicit backward–Euler Newton lives behind opt-in **`pnp_implicit_newton_chain`** / dispatch helpers (`src/physics/solvers/electrochemistry.rs` and tests in `pnp_debye_layer.rs`).
- **Performance:** some **`N=256`** Newton paths are fast in **`--release`** but heavy in unoptimised **`cargo test`** — prefer **`--release`** for local reruns.

### Photonics

- **Stub:** `photonics_curl_curl_stub_default_build` pins **`solve_maxwell_curl_curl`** as identity when **`photonics`** is off.
- **Patches:** DEC face-patch tests use test-authored **COO** incidence and **unweighted** \(d_1^\top d_1\) structure — not yet full mesh pipeline.

### Rheology

- **Scope:** **`BinghamFlowSolver` only** — no Herschel–Bulkley (**n ≠ 1**, power-law index) in `rheology_flow.rs`; README §6 marketing text is ahead of code.
- **Pressure:** Jacobi-preconditioned CG on graph Laplacian (`rheology_flow.rs`); Richardson fallback feature **`rheology_poisson_richardson_fallback`** exists in `Cargo.toml` for Chorin Poisson.

### THMC

- **Honesty flags (Wave 1):** **`ThmcSolver.tol`** is **dead** on the live coupled path (fixed outer count, no stacked-**R** exit); post-step diagnostics in **`step`** evaluate an **explicit-Euler** Laplacian mismatch, **not** the implicit BE stacked residual being solved.
- **Split path:** \((T,\alpha)\) → humidity → **bar-network** equilibrium per outer pass; **`update_damage`** once per step after outers on the SI strain path.
- **Monolith:** small-graph dense Newton behind **`ThmcMonolithicNewtonConfig`** (**≤ 64** DOFs, **`phase4-pr`** — partially closed); mutually exclusive with implicit **(T, α)** Newton when both are `Some`; drying guard **`drying_last_node_evaporation_k == 0`** among preconditions.

### Statistical mechanics

- **Tests (canonical set, all named in table):** Vinet EOS; LJ bridge scaling; **f64** Johnson EOS reference; Johnson upscale bridge (**virial surrogate** for **`[B,4]`**, not Johnson **`K`** in Burn); mechanics–fracture bridge coupling file.
- **`[B,4]` upscale:** third-order virial + Padé **B₃\*** — dilute-ρ ratio band test, **not** Johnson **`K`** validation in Burn.
- **Johnson `K` from rows \((\varepsilon,\sigma,\rho^\*,T^\*)\):** host **`f64`** reference loop — **not** AD-through-`K` today.

### Topology (manifold crate)

- Default CI here exercises **`tests/topology_*.rs`** only (heat/SIMP evolution smokes). Striatus-scale topology / **B6** acceptance lives in the **cartridge** repo.
- **Helmholtz filter (`topology_filter.rs`):** forward Richardson on \((I-sL)\tilde\rho=\rho\); **`apply_straight_through`** is an **STE stopgap** (`ρ_st + (filtered−ρ).detach()`), **not** an implicit PDE filter adjoint — B6 H2-class; true adjoint vs FD on **ρ** is **open**.

---

## Documentation honesty (Track J)

1. This file’s **main table** is the public solver ↔ lane ↔ verification contract.
2. [`README.md`](../README.md) should stay consistent with lane names here; it links one directory shallower.
3. [`PROOF-STATUS.md`](PROOF-STATUS.md) must keep **`benchmark_test`** non-empty for any **`stable`** row.
4. Extend [`References.bib`](References.bib) when rustdoc starts citing new primary sources.

---

## Local check (parity with CI)

```bash
cd umst-manifold
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set
```

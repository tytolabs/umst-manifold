# Solver status (v0.4)

This page is the **authoritative** mapping from solver surfaces to **Cargo feature lanes**, **integration tests** cited in CI, and **honest** completion signals. A short parallel index lives in [`PROOF-STATUS.md`](PROOF-STATUS.md) (Track J3). Formal notation and validation context: [`Mathematical-Foundations.md`](Mathematical-Foundations.md), [`Validation.md`](Validation.md). Bibliography skeleton: [`References.bib`](References.bib).

**Cartridge / Striatus work** (shell topology, rib harness, print-ready JSON) is owned by the separate **[umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge)** repository; see that repo’s `docs/Solver-Status.md` and `docs/Striatus.md` for commands and artefact paths.

---

## Smoke vs acceptance (Track C / Wave 0)

**Do not conflate CI green with physics acceptance.** Three horizons appear across this ledger and the cartridge mirror:

| Horizon | What runs | What it proves | What it does **not** prove |
| --- | --- | --- | --- |
| **Compile** | `cargo check` / `cargo clippy` on `solver-research` (`solver-research-compile-pr`) | Feature graph builds | Any residual, benchmark, or acceptance gate |
| **Smoke** | Default `cargo test`, quick harnesses, `UMST_SHELL_RIB_FULL_ITERS` **< 200**, subset `#[ignore]` envelopes with shortened iters | Finite metrics, regression guards, operator probes on small meshes | B6 §9 acceptance, Kirchhoff **R2.1-A**, developed-channel **L²**, never-run `#[ignore]` physics |
| **Acceptance** | Full schedules: B6 **200-outer** post-finisher export, `UMST_MECHANICS_R21_GATE=1` wide-plate gate, Wave 2 never-run ledger execution | Pre-registered §9 tables, honest pass/fail rows | Closure of open research lanes (Γ-limit, bar→Q1, THMC monolith at scale) |

**Examples (honest labels):** cartridge **20-outer PASS** (2026-06-12) is **smoke/schedule-regime**, not B6 acceptance — acceptance **FAIL on c1 only** was measured separately at **200-outer** (same date). Manifold **phase4-verification-pr** is **test** CI but still **tiny-graph** scope for THMC monolith. **`research-stack`** on `main` is optional (`continue-on-error`) — not a merge gate.

**Never-run inventory:** [`SOLVER_NEVER_RUN_LEDGER.md`](SOLVER_NEVER_RUN_LEDGER.md) — Wave 2 executes each `#[ignore]` envelope once (`--release`); Track C prep does **not** run them.

---

## Completion column

**Completion (%)** is a coarse label in **{0, 25, 50, 75, 100}**, aligned with the v0.4 verification narrative when that material is available in your checkout (historically shipped beside this crate as numbered rows **#1–#10**). It is **not** “fraction of tests green.” **100** means the public acceptance story for that lane is met end-to-end on the stated **test** CI path for the scoped benchmark (not the whole solver family). Lower values mean partial milestones, smokes, or documented gaps. **Do not** treat **100** as permission to claim closure unless the underlying acceptance text is satisfied. **Compiled ≠ validated:** `cargo check` / `cargo clippy` green does **not** discharge physics — see § CI.

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

**Signal legend:** **test** = `cargo test` (physics benches may run); **compile** = `cargo check` / `cargo clippy` only (**builds**, not validated physics); **docs** = markdown / script lint. Workflow **job ids** appear in backticks; doc labels rename misleading ids where noted.

| Doc label (workflow job id) | Signal | Role |
| --- | --- | --- |
| **`solver-status`** | docs | `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` — stable rows must cite real tests; statistical-mechanics row must list the full `statmech_*` set (see `scripts/check_solver_status.py`). |
| **`build-test`** | test | Default features: `cargo build` / `cargo test`. |
| **`solver-stable-pr`** (PR) | test | `cargo test --features solver-stable`. |
| **`solver-research-compile-pr`** (`solver-research-check-pr`) | **compile** | `cargo check --all-targets --features solver-stable,solver-research` — research graph **builds**; does **not** run physics verification. |
| **`phase4-verification-pr`** (PR) | test | Release tests: `thmc_monolithic_newton_chain`; `photonics_curl_curl_{2d_patch,3d_brick}` with `photonics-fdfd`; `statmech_lj_johnson_upscale_bridge` — all with `solver-research` as in the workflow. |
| **`hardware-perf-q1hex-pr`** (PR) | test | `cargo test --features mechanics-adjoint-q1-hex` on `hardware_perf_adversarial`, `solver_region_parity`, `grid_witness_catalog` — fast H0–H6 instrumentation witnesses (not wall-clock benchmarks). |
| **`lint`** | compile | `cargo fmt --check` and `cargo clippy --all-targets --features solver-experimental -- -D warnings` (toolchain pinned in workflow). |
| **`solver-experimental-pr-optional`** (PR) | test (optional) | `cargo test --features solver-experimental --no-fail-fast`; not a merge gate (`continue-on-error`). |
| **`research-stack`** (`main`) | test (optional) | `main` / `workflow_dispatch`: `cargo test --release --features solver-experimental` with one retry; `continue-on-error` on `main`. |
| **`verify-umst-stack`** | test | Gate parity + formal witness subset via `verify_umst_stack.sh` when formal export is present. |

---

## Main solver table

Use **Verification** paths as the contract for what “implemented / verified” means in prose. **Stable** lane rows must have non-empty **Verification** (enforced by `check_solver_status.py`).

| Solver | Lane | Completion (%) | Verification | Notes |
| --- | --- | --- | --- | --- |
| `solvers::topology_solver` (`TopologyOptimizer`, density evolution) | stable | **25** | `tests/topology_continuation.rs`, `tests/topology_filter.rs` | Feature **`topology-density-evolution`**. Conservative stable entry: heat-equation / SIMP-style density evolution. Shell **B6 / B8 / Track L** acceptance and cartridge commands live in **umst-concrete-cartridge** (not repeated here). |
| `mechanics::VectorMechanicsSolver`, `adjoint::AdjointCompliance`, `adjoint_q1_hex::AdjointComplianceQ1Hex` | research | **25** | `tests/verification/mechanics_analytic.rs`, `tests/verification/adjoint_compliance_analytic.rs`, `tests/verification/adjoint_q1_hex_compliance_analytic.rs`, `tests/verification/adjoint_q1_hex_matches_bar_in_limit.rs`, `tests/verification/hardware_perf_adversarial.rs`, `tests/verification/solver_region_parity.rs`, `tests/verification/grid_witness_catalog.rs`, `tests/verification/q1_hex_pcg_warm_start_ab.rs`, `tests/verification/q1_hex_forward_perf_instrument.rs` | Features **`mechanics-voigt-cauchy`**, **`topology-density-evolution`**, **`mechanics-adjoint-q1-hex`**. Shipped quasi-static bar / plate / Q1-hex paths + discrete adjoint checks. **Hardware perf (H0–H6):** `SolverRegion`, operator cache, `DeviceSheet`, grid witnesses, cockpit budget functor — **instrumentation + adversarial CI only**; measured Striatus-scale wall-clock **did not improve** (warm-start regression; BJ ties Jacobi; iso/semi MG fail `eq_rel`). **`acoustics-newmark`** exercises **scalar 1-D** periodic bar waves — **not** vector 3D solid dynamics on the mechanics graph. Kirchhoff **≤ 5 %** centre-deflection vs classical SSSS remains **open** for the shipped brick BCs: the **`#[ignore]`** wide-plate gate still shows **O(1)** relative error; the tighter **`w/w_K`** band test is a **regression** guard on the mismatched-BC path, not thin-plate accuracy. **Contact / friction:** not implemented. See **§ Mechanics**. |
| `solvers::fracture_field` (`PhaseFieldFractureSolver`) | research | **50** | `tests/verification/fracture_gamma_convergence.rs`, `tests/verification/staggered_fracture_mechanics_chain.rs`, `tests/verification/staggered_ud_loop_milestone.rs`, `tests/verification/thmc_drying_shrinkage.rs` | Feature **`fracture-at2`**. AT2 relaxation, length-scale and **partial** Γ-type harnesses, staggered bar milestones, THMC kinematic strain parity vs fracture helpers. **Open:** sharp-interface Γ-limit with driven **ψ⁺**, broader \((l_0,h)\), within-step THMC **u↔d** stagger. **Compressive crushing / plasticity caps** are **not** in this solver (spectral tensile **ψ⁺** drive only). See **§ Fracture**. |
| `solvers::acoustics` (`AcousticWaveSolver`, `AcousticNewmarkBar1dPeriodic`) | research | **100** | `tests/verification/acoustics_plane_wave.rs` | Feature **`acoustics-newmark`**. **100 % = 1-D periodic bar** (`AcousticNewmarkBar1dPeriodic`) only — not graph-assembled **`AcousticWaveSolver`**. Newmark vs dense reference; **return-map** at **n ∈ {64, 100, 128}** including **`plane_wave_return_map_n128_l2_within_two_percent`** with period \(T=2\pi/\Omega\) and lumped \(m=\rho\Delta x\). Inner linear solve uses **f64** Cholesky at larger `n`. See **§ Acoustics**. |
| `solvers::electrochemistry` (`ElectroChemicalSolver`) | research | **75** | `tests/verification/pnp_debye_layer.rs` | Feature **`electrochemistry-pnp`**. MVP chain: SG NP + Thomas Poisson; **explicit Picard** path in `solve_pnp_step` / `solve_pnp_step_experimental` (no **`NewtonPnpContext`** on that default path). **λ\_D** screening-length LS gates on **256**-cell chains run on the **`pnp_debye_layer`** surface when the feature is enabled (quasi-steady trajectory + `mesh_spacing = h` harness). Opt-in **`solve_pnp_step_dispatch`** + implicit Newton chain for research users. **Open:** general-graph implicit Newton, removing worst-case dense \((3N)^2\) scratch at large **N**, variable ε on non-chain topologies. See **§ Electrochemistry**. |
| `solvers::photonics` (`PhotonicsSolver`, `PhotonicsHelmholtzSolver`) | research | **50** | `tests/verification/photonics_fresnel.rs`, `tests/verification/photonics_curl_curl_2d_patch.rs`, `tests/verification/photonics_curl_curl_3d_brick.rs`, `tests/verification/photonics_curl_curl_stub_default_build.rs` | Feature **`photonics-fdfd`**. TE Helmholtz / 1-D curl–curl reductions — **STE stopgap**, not production adjoint TO; small embedded **DEC** patch tests (PR **phase4** path); **stub** documents default-build no-op when **`photonics`** is off. **Open:** dual Hodge / metrics, sparse Krylov, complex ε + PML on patches, BCs beyond gauge pin, production 3D assembly. See **§ Photonics**. |
| `solvers::rheology_flow` (`BinghamFlowSolver`) | research | **50** | `tests/verification/rheology_poiseuille.rs` | Feature **`rheology-bingham`**. **Bingham only** — no Herschel–Bulkley. Analytic Poiseuille / Bingham references, Chorin split smokes, **Jacobi–PCG** pressure Poisson on **−𝒞**. **research-stack** runs short-channel smokes without **`#[ignore]`** — they are **finite / bracket** guards, **not** long-run steady **L²** vs developed Poiseuille. **Open:** MAC-style staggering and/or consistent open **x** BCs; multi-thousand-step steady acceptance. See **§ Rheology**. |
| `solvers::thmc` (`ThmcSolver`, …) | research | **75** | `tests/verification/thmc_drying_shrinkage.rs`, `tests/verification/thmc_monolithic_newton_chain.rs` | Feature **`thmc-coupled`**. Drying + shrinkage + hydration kinetics; implicit **(T, α)** Newton block on tiny graphs; stacked **(T, h, α, u)** dense damped Newton with quasi-static **R\_u** on **≤ 64** DOFs wired into **`ThmcSolver::step` / `step_experimental`** when configured (**Phase 5** in prior memos). **Open:** production-scale monolith / JFNK, adaptive **dt**, within-step **u↔d** stagger. See **§ THMC**. |
| `solvers::statistical_mechanics` | research | **25** | `tests/verification/statmech_vinet_eos.rs`, `tests/verification/statmech_lj_bridge_contract.rs`, `tests/verification/statmech_lj_johnson_eos_reference.rs`, `tests/verification/statmech_lj_johnson_upscale_bridge.rs`, `tests/verification/statmech_mechanics_fracture_bridge.rs` | **`statistical-mechanics-vinet`** marks the **scalar Vinet EOS** stable slice. **Research:** **`[B,2]`** LJ→continuum **`upscale_potentials`** remains a **documented placeholder / virial surrogate** vs Johnson **`K`**; **`[B,4]`** Johnson **`K\_T`** rows via host **`f64`** materialisation (not AD-safe). Johnson (1993) **`f64`** reference surface + upscale bridge tests; optional **`statistical-mechanics-johnson-reference`** re-export. **γ\_gc** and virial-backed bridges remain **open**. See **§ Statistical mechanics**. |

**Prose rule (Track J1):** say “verified on CI” only for behaviours exercised by the **Verification** paths above on a **test** CI lane (§ CI). Do **not** cite **`solver-research-compile-pr`** (`solver-research-check-pr`) or **`lint`** as physics validation. Formal Lean discharge is separate from regression tests.

---

## Research tracks (12–16)

Long-form numbered memos under `docs/research/` were **removed** from this repository to reduce drift. **Scopes** (fracture coupling, THMC monolith, implicit PNP, DEC photonics, LJ/stat-mech bridge) are summarized in the **§ Per-lane** sections and the **Open themes** list below — cite **tests and `src/`** as the live sources of truth.

---

## Open themes (checklist)

| # | Theme | Completion (%) | Direction |
| --- | --- | --- | --- |
| 1 | Topology / shell / Striatus | **25** | Cartridge B6 **ACCEPTANCE FAIL (c1 only, 2026-06-12)** — vf/eq_rel/greyness/xy_var PASS on post-finisher export; see cartridge `Solver-Status.md` + `outputs/b6-acceptance-verdict.md`. B7/B8 blocked until c1 gate design resolved. |
| 2 | Mechanics — thin plate Kirchhoff accuracy | **25** | Align BCs / enrichment for true §R2.1-style gate; keep ratio-band tests as regression only. |
| 3 | Fracture — Γ limit, ψ⁺ drives, THMC stagger | **50** | Extend harnesses beyond fixed partial-Γ schedules. |
| 4 | Acoustics | **100** | **1-D periodic bar** benchmark closed; optional: graph-assembled stiffness beyond bar. |
| 5 | Electrochemistry — scale & graph generality | **75** | Matrix-free / banded solvers; nonlinear SG beyond linearised Debye gates. |
| 6 | Photonics — production DEC + solvers | **50** | Metrics, sparse inner loops, BCs/PML. |
| 7 | Rheology — developed channel fidelity | **50** | MAC / open BCs; long-run **L²** acceptance. |
| 8 | THMC — large‑N monolith | **75** | Krylov–JFNK, adaptive time stepping, stagger policy. |
| 9 | Statistical mechanics — virial / coexistence in **`upscale_potentials`** | **25** | Physical **γ\_gc**; AD-safe **`K`** in Burn. |
| 10 | Transient **vector** solid dynamics & contact | **25** | No default CI **2‑DOF vector** transient stack on the mechanics graph; contact out of scope until a new verification row exists. |
| 11 | Prime-spectral NTT exact density filter (branch-only) | **25** | **Parked** on `prime-spectral-research` — zero mod-q conservation drift validated; L∞ float parity blocked on requantization. Not on `main`; see [issue #26](https://github.com/tytolabs/umst-manifold/issues/26) and MaOS [`FINAL_FINDING.md`](https://github.com/tytolabs/MaOS-Workspace/blob/prime-spectral-research/outputs/prime-spectral-research/FINAL_FINDING.md). |

---

## Per-lane notes

### Mechanics

- **Modules:** `VectorMechanicsSolver::solve_equilibrium`, `AdjointCompliance`, `AdjointComplianceQ1Hex` (see `src/physics/mechanics.rs`, `adjoint.rs`, `adjoint_q1_hex`).
- **Kirchhoff gates:** **R2.1-A** — wide-plate centre deflection vs classical SSSS (`#[ignore]`, **open**, O(1) on shipped brick BCs). **R2.1-B** — tighter **`w/w_K`** band on the mismatched-BC path is **regression-only**, not thin-plate accuracy.
- **Q1 hex:** shipped operator uses a **unified** **D** on the B-bar / centroid-shear strain path; a naive dev/vol split was **rejected** because it regressed slender-column consistency.
- **Hardware perf (Q1 hex):** reusable PCG workspace (`solver_region.rs`), warm-start / op-cache knobs, host `DeviceSheet`, fused Krylov reductions, grid catalog witnesses, cockpit iteration budget (`solve_budget.rs`) — verified by `hardware_perf_adversarial` (5 tests) on PR CI. **Vault IO:** `UMST_COCKPIT_JSON` → cockpit budget in `umst-steerable-vault` (`cockpit_io.rs`); `UMST_VAULT_PRECOND=amg` selects algebraic semicoarsening V-cycle (Jacobi pre/post). **Not a Striatus wall-clock win** yet: warm-start regressed; block-Jacobi ties Jacobi; geometric/semicoarsening MG fail at scale. **8×8×4 parity:** `q1_hex_perf_levers_ab` includes AMG arm (eq_rel &lt; 1e-4). TNA use case: `cast_lifecycle::tna_block_lane_with_cockpit_budget`.
- **Ignored slender Q1 vs bar:** z-skeleton bar limit matches closed form; the **1×1** brick cross-section remains much more compliant — documented harness limitation, not “solver wrong” without refined 1D reduction.

### Fracture

- **Driver:** spectral **ψ⁺** (tensile principal strains); **no** compressive crushing / Drucker–Prager cap in **`PhaseFieldFractureSolver`**.
- **THMC:** when SI **`[N,3]`** embedding holds, post-mechanics strain feeds fracture; **`thmc_drying_shrinkage`** includes kinematic parity tests vs **`strain_tensor_for_fracture_after_mechanics`**.

### Acoustics

- **`AcousticNewmarkBar1dPeriodic`** hosts the verified Newmark + return-map suite in `tests/verification/acoustics_plane_wave.rs` — this is the **100 %** scope (1-D periodic bar).
- **`AcousticWaveSolver`** remains the nodal graph contraction; graph-assembled dynamics are **not** certified on CI.

### Electrochemistry

- **Default path:** explicit Picard coupling; implicit backward–Euler Newton lives behind opt-in **`pnp_implicit_newton_chain`** / dispatch helpers (`src/physics/solvers/electrochemistry.rs` and tests in `pnp_debye_layer.rs`).
- **Performance:** some **`N=256`** Newton paths are fast in **`--release`** but heavy in unoptimised **`cargo test`** — prefer **`--release`** for local reruns.

### Photonics

- **STE stopgap:** TE Helmholtz / embedded DEC patches are regression harnesses — **not** production adjoint topology optimisation.
- **Stub:** `photonics_curl_curl_stub_default_build` pins **`solve_maxwell_curl_curl`** as identity when **`photonics`** is off.
- **Patches:** DEC face-patch tests use test-authored **COO** incidence and **unweighted** \(d_1^\top d_1\) structure — not yet full mesh pipeline.

### Rheology

- **Constitutive scope:** **Bingham** only — Herschel–Bulkley and yield-stress generalisations are **out of scope** until a new verification row exists.
- **Pressure:** Jacobi-preconditioned CG on graph Laplacian (`rheology_flow.rs`); Richardson fallback feature **`rheology_poisson_richardson_fallback`** exists in `Cargo.toml` for Chorin Poisson.

### THMC

- **Split path:** \((T,\alpha)\) → humidity → bar equilibrium per outer pass; **`update_damage`** once per step after outers on the SI strain path.
- **Monolith:** small-graph dense Newton behind **`ThmcMonolithicNewtonConfig`**, mutually exclusive with implicit **(T, α)** Newton when both are `Some`; drying guard **`drying_last_node_evaporation_k == 0`** among preconditions.
- **Wave 1 prep (queued, S1 gate):** post-Newton stacked-\(\|R\|_2\) diagnostic hook on the implicit functional is **not** wired in production `ThmcSolver::step` yet — split-path early exit on **`tol`** is shipped; monolithic exit uses **`ThmcMonolithicNewtonConfig::stacked_residual_l2_tolerance`**. Wave 1 will add a feature-gated **`ThmcPostNewtonDiagnostic`** witness at Newton exit (stacked residual + brute-force oracle parity). Until then, the oracle contract lives in **`tests/verification/thmc_post_newton_oracle_fixture.rs`** (skeleton only; no hot-path edits).

### Statistical mechanics

- **Tests (canonical set, all named in table):** Vinet EOS; LJ bridge scaling; Johnson EOS reference + documented mismatch vs placeholder upscale; Johnson upscale bridge; mechanics–fracture bridge coupling file.
- **Upscale honesty:** **`upscale_potentials`** is a **virial surrogate / placeholder** vs Johnson **`K`** — not a discharged coexistence proof.
- **Johnson `K` from rows \((\varepsilon,\sigma,\rho^\*,T^\*)\):** implemented via Burn column views + host **`f64`** loop — **not** AD-through-`K` today.

### Topology (manifold crate)

- Default CI here exercises **`tests/topology_*.rs`** only. Striatus-scale topology lives in the **cartridge** repo.

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

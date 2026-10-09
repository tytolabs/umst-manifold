SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
# Engine slices — feature lanes → physics modules

**Status:** Framework map (`engine-slices-frameworks`, Wave 4 slot 10).  
**Plan:** Master reengineering **T3.6** — promote three validated POC primitives (chaos fail-closed, entropy-tax-catches-gaming, const-generic shape conservation) into [`SolveReport`](../../src/solve_report.rs) / gate-evidence machinery.  
**SSOT siblings:** [`Solver-Status.md`](Solver-Status.md) (solver completion + CI), [`PROOF-STATUS.md`](PROOF-STATUS.md) (Track J3 index).

This document names **physics slices**: compile-time domains in `src/physics/solvers/` (and adjacent modules) selected by Cargo `[features]`. Meta-features **`solver-stable`**, **`solver-research`**, and **`solver-experimental`** are the public lanes agents and CI use — not individual slice flags in isolation.

**Compiled ≠ validated:** a green `cargo check --features solver-research` only proves the graph **builds**. Say “verified on CI” only when the row’s **`benchmark_test`** path runs under a **test** job in [`Solver-Status.md`](Solver-Status.md) § CI.

---

## Meta-feature lanes (`Cargo.toml`)

| Meta-feature | Includes | CI role |
| --- | --- | --- |
| **`solver-stable`** | `topology-density-evolution`, `statistical-mechanics-vinet` | **`solver-stable-pr`** — blocking `cargo test --features solver-stable` |
| **`solver-research`** | `fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-adjoint`, `mechanics-adjoint-q1-hex`, `rheology-bingham`, `photonics-fdfd`, `statistical-mechanics-johnson-reference`, … | **`solver-research-compile-pr`** — compile-only; **`phase4-verification-pr`** — selected physics tests |
| **`solver-experimental`** | `solver-stable` ∪ `solver-research` (alias: **`solver-tests`**) | **`lint`** (clippy), optional **`solver-experimental-pr-optional`** / **`research-stack`** |

Dependency edges worth remembering:

- `thmc-coupled` → `fracture-at2`
- `mechanics-adjoint` → `mechanics-voigt-cauchy`
- `mechanics-adjoint-q1-hex` → `topology-density-evolution`
- `electrochemistry-pnp` → `electrochemistry-mvp`
- `photonics-fdfd` → `photonics` (cfg alias; deprecated `photonics-scaffold`)

---

## Physics slice map + PROOF-STATUS honesty

One row per solver surface in [`PROOF-STATUS.md`](PROOF-STATUS.md). **Completion (%)** matches [`Solver-Status.md`](Solver-Status.md) — coarse milestone label, not “fraction of tests green.”

| Physics slice | Cargo feature(s) | Meta-lane | Rust module | `benchmark_test` | `verification_status` | Completion (%) | PROOF-STATUS honesty |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **Topology / density evolution** | `topology-density-evolution` | **stable** | `solvers::topology_solver` | `tests/topology_continuation.rs`, `tests/topology_filter.rs` | mechanised | **25** | Stable lane entry; SIMP-style evolution only. Striatus B6/B8 shell acceptance lives in **umst-concrete-cartridge**, not this crate. |
| **Statistical mechanics — Vinet EOS** | `statistical-mechanics-vinet` | **stable** | `solvers::statistical_mechanics` | `tests/verification/statmech_vinet_eos.rs` | analytic-benchmark | **25** | Scalar Vinet slice is the **stable** stat-mech surface; Johnson / LJ upscale rows remain research. |
| **Mechanics — bar / plate / adjoint / Q1 hex** | `mechanics-voigt-cauchy`, `mechanics-adjoint`, `mechanics-adjoint-q1-hex` | research | `physics::mechanics`, `adjoint`, `adjoint_q1_hex` | `tests/verification/mechanics_analytic.rs`, `adjoint_compliance_analytic.rs`, `adjoint_q1_hex_compliance_analytic.rs`, `adjoint_q1_hex_matches_bar_in_limit.rs` | analytic-benchmark | **25** | Bar/plate + discrete adjoint checks ship; Kirchhoff **R2.1-A** wide-plate gate **open** (`#[ignore]`, O(1) error). **R2.1-B** `w/w_K` band is regression-only. Contact / 3D vector transient dynamics **not** certified. |
| **Fracture — phase field AT2** | `fracture-at2` | research | `solvers::fracture_field` | `fracture_gamma_convergence.rs`, `staggered_fracture_mechanics_chain.rs`, `staggered_ud_loop_milestone.rs`, `thmc_drying_shrinkage.rs` | analytic-benchmark | **50** | Partial Γ-type harnesses; spectral **ψ⁺** drive only — **no** compressive crushing cap. Within-step THMC **u↔d** stagger **open**. |
| **Acoustics — 1-D Newmark bar** | `acoustics-newmark` | research | `solvers::acoustics` | `tests/verification/acoustics_plane_wave.rs` | analytic-benchmark | **100** | **100 % scope = 1-D periodic bar** (`AcousticNewmarkBar1dPeriodic`) only. `AcousticWaveSolver` graph assembly **not** certified. |
| **Electrochemistry — PNP chain** | `electrochemistry-pnp` | research | `solvers::electrochemistry` | `tests/verification/pnp_debye_layer.rs` | analytic-benchmark | **75** | Default Picard path; implicit Newton behind opt-in dispatch. Large-**N** dense scratch and general-graph Newton **open**. |
| **Photonics — TE Helmholtz / DEC patches** | `photonics-fdfd` | research | `solvers::photonics` | `photonics_fresnel.rs`, `photonics_curl_curl_2d_patch.rs`, `photonics_curl_curl_3d_brick.rs`, `photonics_curl_curl_stub_default_build.rs` | analytic-benchmark | **50** | **STE stopgap** — not production adjoint topology optimisation. Dual Hodge / sparse Krylov / PML **open**. |
| **Rheology — Bingham flow** | `rheology-bingham` | research | `solvers::rheology_flow` | `tests/verification/rheology_poiseuille.rs` | analytic-benchmark | **50** | **Bingham only** — no Herschel–Bulkley. Developed-channel steady **L²** vs Poiseuille **open** (`#[ignore]` smokes are bracket guards only). |
| **THMC — coupled transport** | `thmc-coupled` | research | `solvers::thmc`, `thmc_residual` | `thmc_drying_shrinkage.rs`, `thmc_monolithic_newton_chain.rs` | analytic-benchmark | **75** | Split + tiny-graph monolithic Newton (≤ **64** stacked DOFs). Production JFNK / adaptive **dt** **open**. `ThmcSolver::tol` → stacked-**R** exit **open** (Wave 1). |
| **Statistical mechanics — LJ / Johnson research** | `statistical-mechanics-johnson-reference` (+ Vinet when enabled) | research | `solvers::statistical_mechanics`, `lj_johnson_1993_reference` | `statmech_lj_bridge_contract.rs`, `statmech_lj_johnson_eos_reference.rs`, `statmech_lj_johnson_upscale_bridge.rs`, `statmech_mechanics_fracture_bridge.rs` | analytic-benchmark | **25** | **`upscale_potentials`** is a **documented virial surrogate / placeholder** vs Johnson **K** — not discharged coexistence. Johnson **K** from rows uses host **`f64`** — **not** AD-safe. |
| **Experimental-only harnesses** | `solver-experimental` (union) | experimental | topology filters, Helmholtz autodiff, aug-Lagrangian, … | `tests/topology_filter.rs`, `helmholtz_striatus_autodiff.rs`, `aug_lagrangian_volume.rs`, … | analytic-benchmark / none | **25** | Extra integration tests gated on **`solver-experimental`**; optional CI (`continue-on-error`). `thmc_jfnk` module compiles only under this lane. |
| **Prime-spectral NTT filter** | — (isolated branch) | **not on main** | — | branch `prime-spectral-research` | literature | **25** | Zero mod-q conservation drift validated; L∞ float parity **blocked** on requantization. See issue #26. |

Formal Lean discharge for solver rows is **separate** from regression tests — see [`FORMAL_INTEGRATION_STATUS.md`](FORMAL_INTEGRATION_STATUS.md).

---

## T3.6 framework primitives (POC → machinery)

Master plan **T3.6** named three POC primitives on branch `engine-slices` (three commits, preserved at tag `backup/engine-slices-20260624`). The branch is not merged; each primitive was decided on 2026-10-09 (cell MANIFOLD-ENGINE-SLICES-DECIDE) against the gate and solve surfaces on `main`:

| Primitive | Branch commit | Disposition on `main` | Test that carries it |
| --- | --- | --- | --- |
| **Chaos fail-closed** | `64da191` | **Ported.** The well-formed snapshot guard landed on `main` inside the pure evaluator `gate::transition_outcome` (wrapped by `TransitionFilter` = `ThermodynamicMixFilter`) and in `thermodynamic_transition_admissible_tol`, with a `MalformedInput` reject reason. The branch tests are ported onto that surface and widened to every snapshot field × {NaN, +∞, −∞} on both sides, non-positive temperature, every bad `dt`, and every argument of the pure predicate; the filter's rejection counter must move on each case. | `tests/chaos_crucible_gate_transition.rs` (5 tests) |
| **Chaos fail-closed — solve witness hook** | `64da191` | **Superseded.** The branch added `SolveReport::gate_reject_non_converged_solve` (= `!converged()`). `main` carries the typed refusal `SolveReport::ensure_converged() -> Result<(), &str>` with distinct reasons for an inadmissible residual, a non-positive tolerance and a stall. | `solve_report::tests::solve_report_construction_and_converged`, `solve_report_refuses_nan_inf_residual` |
| **Entropy tax catches gaming** | `dadb50c` | **Not ported.** `entropy_tax_j = D − k_B T ln 2 · ΔI` took ΔI (bits) from the solver iteration count, which is not a measured mutual information, and the second test asserts only that 50 × floor − floor > 1.5 × floor − floor. The Landauer floor itself is carried on `main` by `constants::landauer_bit_energy_joules` and by `ai::constraint_loss::landauer_slack_violation` (`relu(k_B T ln 2 · bits − credit)`). An excess-dissipation witness needs a measured ΔI before it can be tested. | `constants::tests::landauer_bit_energy_*`; `tests/rejection_witness.rs`; `tests/cross_backend_gate_determinism.rs` |
| **Const-generic shape conservation** | `e840b40` | **Ported** as a rustdoc pair on `umst_math::tensor::partial_trace_second`: a passing example at rank 2 and an otherwise identical `compile_fail` example at rank 3. The branch used `trybuild`, which is not in the offline registry; stable rustdoc does not check `compile_fail` error codes, so the two examples differ only in the bound rank. | `cargo test -p umst-math --doc partial_trace_second` (2 doctests) |

Wiring these primitives into every physics slice exit and the gate HTTP payloads stays open (`integration-solve-report`, solver-audit waves 1–3).

---

## Cross-slice infrastructure (always-on vs gated)

| Surface | Feature gate | Role in slice framework |
| --- | --- | --- |
| `SolveReport`, `SOLVER_ENTRY_POINTS` | always (inventory) | Unified equilibrium witness contract (Wave 0 ledger honesty) |
| `VectorMechanicsSolver::solve_equilibrium` | default | Load-bearing bar network — no adjoint until `mechanics-adjoint` |
| `gate::mix_proposal`, `ThermodynamicMixFilter` | always | Hot-path transition gate; chaos guards land here (T3.6) |
| `umst-math::catalog_functor` | always | Fiber → scalar channel count (orthogonal to physics slices) |

---

## Local verify

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cd umst-manifold   # or this worktree root

# Lane parity with CI
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set

# Stable lane
cargo test --features solver-stable

# Research compile graph (physics not run)
cargo check --all-targets --features solver-stable,solver-research
```

---

## Related

- [`Solver-Status.md`](Solver-Status.md) — completion %, per-lane notes, CI job ids
- [`PROOF-STATUS.md`](PROOF-STATUS.md) — Track J3 short index (must stay consistent with the table above)
- [`SOLVE_CONTRACT.md`](SOLVE_CONTRACT.md) — `SolveReport` adoption ladder
- [`outputs/.plans/umst-master-reengineering.md`](../../outputs/.plans/umst-master-reengineering.md) — T3.6 + solver waves S0–S3
- [`outputs/.plans/umst-swarm-execution.md`](../../outputs/.plans/umst-swarm-execution.md) — swarm slot map

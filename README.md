<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)

**UMST Manifold** is a differentiable spatiotemporal substrate for heterogeneous materials: one unified state tensor, discrete exterior calculus (DEC) operators, graph- and lattice-based solvers, and control-barrier–style thermodynamic gating—implemented in **Rust** on the **Burn** stack with **`burn-ndarray`** as the default execution path. Domain science plugs in through the **`IScienceCartridge`** surface; the crate ships equilibrium mechanics, adjoint hooks, topology evolution, and an opt-in solver stack grouped into explicit **Cargo feature lanes**.

![UMST 64-Tensor Pipeline (Light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST 64-Tensor Pipeline (Dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

## Core idea

- **Unified state (`UMST`):** A fixed-width tensor channelization carries mechanical, thermal, chemical, and auxiliary signals across a mesh or graph so constitutive updates and PDE-style operators share one representation.
- **Differentiable evolution:** Burn autodiff wires through the tensor paths that tests exercise; adjoint and optimization-facing code paths are first-class alongside forward solvers.
- **DEC + physics kernels:** Sheaf/DEC plumbing, mechanics on free degrees of freedom, fracture and transport scaffolds, and specialized kernels (topology, acoustics, THMC, electrochemistry, photonics, rheology, statistical mechanics) compile behind granular `#[cfg(feature = "...")]` flags—see [`docs/Solver-Status.md`](docs/Solver-Status.md) for the solver ↔ lane ↔ verification mapping.

## Solver architecture (feature lanes)

Lanes are **meta-features** in [`Cargo.toml`](Cargo.toml); names and inclusion sets are authoritative there and summarized in [`docs/Solver-Status.md`](docs/Solver-Status.md).

| Lane | Role |
|------|------|
| **`solver-stable`** | `topology-density-evolution`, `statistical-mechanics-vinet` — narrow-CI kernels with declared verification tests. |
| **`solver-research`** | Opt-in kernels: fracture (`fracture-at2`), acoustics (`acoustics-newmark`), coupled THMC (`thmc-coupled`), Poisson–Nernst–Planck (`electrochemistry-pnp`), mechanics + discrete adjoint (`mechanics-adjoint`, `mechanics-adjoint-q1-hex`), Bingham flow (`rheology-bingham`), FDFD-style photonics (`photonics-fdfd`), Johnson-reference statistical mechanics (`statistical-mechanics-johnson-reference`), … |
| **`solver-experimental`** | **`solver-stable` ∪ `solver-research`** — full opt-in union (backward-compatible umbrella). |
| **`solver-tests`** | Same dependency graph as **`solver-experimental`** — used for CI solver coverage and `check-cfg` surfaces. |

Canonical feature names forward to legacy `#[cfg]` names where needed (e.g. **`photonics-fdfd`** → **`photonics`**; **`electrochemistry-pnp`** → **`electrochemistry-mvp`**). Deprecated alias **`photonics-scaffold`** resolves to **`photonics-fdfd`**.

## Cartridge ecosystem

| Cartridge | Domain | Status |
|-----------|--------|--------|
| [`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge) | Cementitious materials, RC and shell topology | **Active** |
| `umst-supercap-cartridge` | Structural batteries, ion transport | In progress |

Striatus-class shell workflows, artefact contracts, and print-ready gates live in the **concrete cartridge** repository (`docs/Striatus.md`, `docs/Solver-Status.md` there). Manifold-side solver verification remains indexed in **`docs/Solver-Status.md`** here.

## Grounded examples (mix, Pareto, sampling)

This crate hosts **solvers, DEC operators, and `IScienceCartridge` hooks** — mix JSON, calibration CSVs, notebooks, and headline metric tables ship in **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge)**. Below, manifold commands point to **in-repo** sources; mix calibration and Pareto-style **dataset metric tables** are exercised there.

- **Cartridge boundary (`IScienceCartridge` minimal host):** `cargo run --example basic_topology` — read [`examples/basic_topology.rs`](examples/basic_topology.rs) alongside [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md) for how unified tensors enter operator graphs.
- **Mechanics + adjoint lane (topology / compliance workflows used by the concrete cartridge):** enable `mechanics-adjoint` (via `solver-research` / `solver-experimental`) per [`docs/Solver-Status.md`](docs/Solver-Status.md); end-to-end RC beam and shell drivers live in [`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge) (`optimize_rc_beam`, `optimize_shell_3d`, [`notebooks/README.md`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/notebooks/README.md)), with the animated strut-and-tie artefact at [`docs/assets/beam_strut_and_tie.gif`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/docs/assets/beam_strut_and_tie.gif).
- **Mix calibration and tabular trade space (downstream repo):** `umst predict` / `umst audit` and [`results/canonical/table_per_dataset_metrics.csv`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/results/canonical/table_per_dataset_metrics.csv) document per-profile MAE/RMSE/R² on bundled CSVs; [`results/canonical/README.md`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/results/canonical/README.md) explains regeneration. Differentiable mix–cost wiring for multi-objective stepping: [`cost.rs`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/crates/umst-concrete-cartridge/src/physics/cost.rs) in the cartridge.
- **Sampling / sweep verification (this repo):** `cargo test --features solver-stable` (PR gate) and `cargo test --features solver-tests` for the full experimental matrix; keep [`docs/Solver-Status.md`](docs/Solver-Status.md) aligned with `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`.
- **64-channel pipeline figure (tensor layout teaser):** [`docs/assets/fig1_teaser.png`](docs/assets/fig1_teaser.png) (light) / [`docs/assets/fig1_teaser_dark.png`](docs/assets/fig1_teaser_dark.png) (dark) — static overview of the unified tensor stack rendered in-repo.

Design-space **Pareto counts and multi-objective exploration** for the thermodynamic-gated learner are documented in the companion draft `main_v4.tex` (*Thermodynamic Design Exploration*, Table `tab:results_design`); that TeX source is **not** vendored inside this repository (see **Research context**).

## Research context (companion manuscripts)

Paths below are **not** in the default [`umst-manifold`](https://github.com/tytolabs/umst-manifold) clone; they appear in internal multi-project checkouts under a sibling `UMST-Papers/` tree (parent directory name varies).

| Path (under `UMST-Papers/`) | Contents |
|-----------------------------|----------|
| `arxiv_2_v4/main_v4.tex` | Draft UMST manuscript: §Method (tensor, gate, complexity), §Experiments including *Thermodynamic Design Exploration* / `tab:results_design`, *Ablation Studies* (`tab:ablation`), *Robustness \& Sensitivity Analysis*. |
| `arxiv_2_v4/tables/ssot_rust_d1_d4.tex` | SSOT tables for Rust vs D1–D4 evaluation splits. |
| `iros paper 2 final files/iros2026_paper/Paper2_Final_IROS2026_anonymous.tex` | IROS-track paper: *Epistemic Proxy Selection*, *Experiments* (proxy effort / field sampling narrative). |
| `iros paper 2 final files/iros2026_paper/Paper2_Supplementary_anonymous.tex` | Supplementary derivations and extended admissibility discussion. |
| `iros paper 2 final files/iros2026_reproducibility_package/` | Reproducibility tree (prototype-era `pareto_design_benchmark.rs`, experiment harnesses). |

Historical Pareto tables in the public prototype line: **[`umst-prototype-2a`](https://github.com/tytolabs/umst-prototype-2a)**.

## Everyday solver commands

- **Default CPU stack:** `cargo build` then `cargo test` at the repo root (matches CI `build-test` on ndarray).
- **Stable solver lane:** `cargo test --features solver-stable` (topology-density-evolution + statistical-mechanics-vinet; PR gate on pull requests).
- **Docs + path contract:** `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set` (same flags as the **`solver-status`** CI job).
- **Experimental solver union (heavy):** `cargo test --features solver-tests` (same feature graph as **`solver-experimental`**).
- **Lint like merge-ready Rust:** `cargo fmt --check` and `cargo clippy --all-targets --features solver-experimental -- -D warnings` on the pinned toolchain ([`rust-toolchain.toml`](rust-toolchain.toml); CI uses Rust **1.88**).

## Surfaces & entrypoints

| Surface | Best for | Copy-paste | Prerequisites |
|--------|----------|------------|-----------------|
| **Rust library** (`umst_manifold`) | Embedding DEC/solvers, cartridge backends, custom physics | Add a path or git dependency on this crate; enable feature lanes from [`Cargo.toml`](Cargo.toml). | **Rust 1.88** for parity with CI; `rust-version` in `Cargo.toml` is the declared MSRV floor for default-feature builds. |
| **Cargo tests** | Regression, solver proofs, lane coverage | `cargo test` · `cargo test --features solver-stable` · `cargo test --features solver-tests` | Same toolchain; CPU-only (`ndarray` default). |
| **Cargo examples** | One-file integration narrative | `cargo run --example basic_topology` | Default features unless you extend the example locally. |
| **Python · MCP · Docker · end-user CLI** | Notebooks, agent tools, container deploy, calibration CLI | Not shipped here — use **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge)** (`crates/umst-py`, `crates/umst-mcp`, `crates/umst-cli`, root `Dockerfile` / `docker-compose.yml`). | That workspace pins the same **Rust 1.88** line for CI alignment. |

## Choose your path

- **Library / cartridge author:** Depend on `umst-manifold`, pick **`solver-stable`** vs **`solver-research`** / **`solver-experimental`** from [`Cargo.toml`](Cargo.toml); cross-check rows in [`docs/Solver-Status.md`](docs/Solver-Status.md) before enabling a kernel.
- **Application engineer:** Start from [`examples/basic_topology.rs`](examples/basic_topology.rs) and the default `cargo test` surface; graduate to `--features solver-stable` when you touch stable-lane solvers.
- **Researcher / repro / paper track:** Read [`docs/Solver-Status.md`](docs/Solver-Status.md) for lane ↔ verification ↔ CI mapping; run `scripts/check_solver_status.py` with the flags above before changing solver tables or memo links.
- **Integrator / product:** Consume this crate from Rust, or mount domain logic through **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge)** for Python, MCP, Docker, and the `umst` CLI — the manifold stays library-centric.

## For agents

- **Repo root:** treat the checkout directory of this repository as the working root for all `cargo` / `python3` commands unless a doc says otherwise.
- **Read first:** [`README.md`](README.md) (this file), [`Cargo.toml`](Cargo.toml) `[features]`, [`docs/Solver-Status.md`](docs/Solver-Status.md), [`.github/workflows/rust.yml`](.github/workflows/rust.yml) (job names: **README sanity**, **solver status**, **build & test (ndarray backend)**, **solver-stable tests (PR)**, …).
- **Safe, no-GPU commands:** `cargo build`, `cargo test`, `cargo test --features solver-stable`, `cargo run --example basic_topology`, `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`.
- **Before editing:** scan [`docs/Solver-Status.md`](docs/Solver-Status.md) and run `check_solver_status.py` before changing solver feature tables, `tests/verification/`, or lane-related `#[cfg(feature = "...")]` blocks.
- **No Python or MCP in-tree:** agent-facing notebooks, PyO3, and MCP stdio live only under **`umst-concrete-cartridge`**; do not assume `maturin` or MCP manifests exist here.

## Build, test, CI parity

```bash
cd umst-manifold
cargo build
cargo test
```

- **Solver integration tests:** `cargo test --features solver-tests` (same feature graph as **`solver-experimental`**).
- **GPU (`wgpu`):** The **`wgpu`** feature selects Burn’s WGPU backend; on the pinned **Burn 0.13** line this path fails to compile on current stable Rust because of upstream `burn-jit` derive defaults—CPU builds use **`ndarray`**; on Apple Silicon, **`mac-fast`** (`ndarray` + **`blas-accelerate`**) is the supported fast path until Burn is upgraded or patched.

CI (`.github/workflows/rust.yml`): **README sanity** (minimum length), **`solver-status`** (`python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`), **default `cargo build` / examples / `cargo test`**, PR **`cargo test --features solver-stable`**, PR **`cargo check --all-targets --features solver-stable,solver-research`**, PR **Phase-4 `--release`** slices (THMC monolithic Newton chain; photonics curl–curl 2D/3D; statistical-mechanics Johnson upscale bridge), **`cargo fmt`** + **`cargo clippy --all-targets --features solver-experimental -D warnings`** on Rust **1.88**, **`cargo test --release --features solver-experimental`** on `main` (single retry), plus an **optional** PR job for the full experimental test matrix and a physics host-tensor guard script.

Local parity with the docs linter:

```bash
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set
```

## Toolchain and CPU acceleration

**Rust 1.88** — [`rust-toolchain.toml`](rust-toolchain.toml). The package **`rust-version`** in [`Cargo.toml`](Cargo.toml) remains the declared MSRV floor for default-feature builds; optional dependency paths used under **`--all-features`** require the pinned toolchain in practice.

**Apple Accelerate (macOS):** `cargo build --features blas-accelerate` or the umbrella `cargo build --features mac-fast`. Cap BLAS threads to match core count, e.g. `export VECLIB_MAXIMUM_THREADS=$(sysctl -n hw.perflevel0.logicalcpu)`.

## Selected Cargo features

| Feature | Purpose |
|---------|---------|
| `ndarray` (default) | CPU tensors via `burn-ndarray`. |
| `blas-accelerate` | vecLib/Accelerate-backed matmul on macOS (forwarded to `burn-ndarray`). |
| `mac-fast` | `ndarray` + `blas-accelerate` convenience bundle. |
| `wgpu` | Burn WGPU backend (non-building on pinned Burn 0.13 + current stable; see above). |
| `train` | Burn training utilities. |
| `solver-stable`, `solver-research`, `solver-experimental`, `solver-tests` | Solver lane umbrellas (see table). |
| Granular solver flags | `fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-voigt-cauchy`, `mechanics-adjoint`, `mechanics-adjoint-q1-hex`, `rheology-bingham`, `topology-density-evolution`, `photonics-fdfd`, `statistical-mechanics-vinet`, `statistical-mechanics-johnson-reference`, … — full matrix in `[features]` in [`Cargo.toml`](Cargo.toml). |

## Quick start: `IScienceCartridge`

Domain cartridges implement **`IScienceCartridge`** to supply constitutive closures into the manifold’s operators and solvers. End-to-end wiring: [`examples/basic_topology.rs`](examples/basic_topology.rs).

## Reference

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths, CI contract:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index (Track J3):** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)
- **Gap audit:** [`GAP_AUDIT.md`](GAP_AUDIT.md)
- **Lean formalization (separate repo):** [umst-formal](https://github.com/tytolabs/umst-formal)

When `composer_prompts/` sits beside this checkout: [`../composer_prompts/v0.4_solver_completion_no_namesakes.md`](../composer_prompts/v0.4_solver_completion_no_namesakes.md).

## Citation

[`CITATION.cff`](CITATION.cff) and the repository URL carry bibliographic metadata.

## Contributing and license

[`CONTRIBUTING.md`](CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), [`SECURITY.md`](SECURITY.md).  
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

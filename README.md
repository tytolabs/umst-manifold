<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)

**Unified Material-State Tensor (UMST) manifold** — a Rust / Burn substrate for differentiable material physics on a graph 1-skeleton: discrete exterior calculus operators, thermodynamic admissibility gating, and adjoint-friendly evolution. Intended for researchers and engineers integrating domain cartridges (constitutive models) with conserved fluxes and verified state wrappers.

**Repository:** [github.com/tytolabs/umst-manifold](https://github.com/tytolabs/umst-manifold)

## Solver maturity

Lanes (`solver-stable` vs `solver-research`), verification paths, completion matrix alignment, and honest deferrals (including topology / shell and matrix **#10** scope): **[`docs/Solver-Status.md`](docs/Solver-Status.md)**. Per-solver index: **[`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)**. v0.4 brief checklist (when `composer_prompts/` sits beside this repo): **[`../composer_prompts/v0.4_solver_completion_no_namesakes.md`](../composer_prompts/v0.4_solver_completion_no_namesakes.md)**.

## Build and test

```bash
cd umst-manifold
cargo build
cargo test
```

GPU backend (local Vulkan/Metal): `cargo build --features wgpu`. Solver integration tests: `cargo test --features solver-tests`.

Shell topology and print-ready gates (cartridge artefacts, **B6**/**B8**/**L**): **[`docs/Solver-Status.md`](docs/Solver-Status.md)**; cartridge mirror: [`../umst-concrete-cartridge/docs/Solver-Status.md`](../umst-concrete-cartridge/docs/Solver-Status.md).

CI lint (`solver-status` job in [`.github/workflows/rust.yml`](.github/workflows/rust.yml)) and recommended local parity:

```bash
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set
```

## Toolchain

**Rust 1.88** — pinned in [`rust-toolchain.toml`](rust-toolchain.toml). Match CI with `rustup default 1.88` or `cargo +1.88 build` if your global default differs. The `rust-version` field in `Cargo.toml` marks a lower bound for minimal feature sets; use the pinned toolchain for `--all-features` builds.

**CPU matmul (macOS / Apple Silicon):** optional `cargo build --features blas-accelerate` enables `burn-ndarray`’s `blas-accelerate` (Accelerate via `blas-src`). Cap BLAS threads to avoid oversubscription: **`VECLIB_MAXIMUM_THREADS`** (e.g. `export VECLIB_MAXIMUM_THREADS=$(sysctl -n hw.perflevel0.logicalcpu 2>/dev/null || sysctl -n hw.logicalcpu)`), or **`OPENBLAS_NUM_THREADS`** when using an OpenBLAS-linked stack.

## Cargo features

| Feature | Purpose |
|--------|---------|
| `ndarray` (default) | CPU tensors via `burn-ndarray`. |
| `blas-accelerate` | CPU matmul via Apple Accelerate on macOS (optional). |
| `wgpu` | GPU tensors via Burn/WGPU. |
| `train` | Burn training utilities. |
| `solver-experimental` | Umbrella flag: all opt-in solver scaffolds (damage, THMC, electrochemistry, mechanics variants, etc.). |
| `solver-tests` | Same dependency graph as `solver-experimental`; used for CI solver coverage. |

Individual flags (`fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-voigt-cauchy`, `rheology-bingham`, `topology-density-evolution`, `photonics-fdfd`; legacy aliases `electrochemistry-mvp`, `photonics-scaffold`) select subsets; see `[features]` in [`Cargo.toml`](Cargo.toml).

## Scope

The default build exposes DEC / sheaf plumbing, equilibrium mechanics on free degrees of freedom, thermodynamic control-barrier gating, adjoint hooks, and the `IScienceCartridge` surface. With `solver-experimental`, additional forward and coupled solves compile and run where wired; coverage and production readiness vary by module.

## Reference

- Formal notation and cartridge interface: [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- Burn host-sync / solver hot-path audit (`into_scalar`, `into_data`): [`docs/FP_CATEGORICAL_BURN.md`](docs/FP_CATEGORICAL_BURN.md)
- Detailed gap list (spec vs. implementation): [`GAP_AUDIT.md`](GAP_AUDIT.md)

## Example

End-to-end cartridge hookup: [`examples/basic_topology.rs`](examples/basic_topology.rs)

## Related repositories

| Project | Role |
|--------|------|
| [umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge) | Cementitious constitutive cartridge |
| [umst-formal](https://github.com/tytolabs/umst-formal) | Companion formal developments |

## Cartridge ecosystem

Striatus-class shell demos and artefact contracts live in the **[umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge)** repo; shell topology, print-ready gates, and deferrals: **[`docs/Solver-Status.md`](docs/Solver-Status.md)** and [`docs/Striatus.md`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/docs/Striatus.md) (cartridge). **Track L** filenames (`striatus_emergence.gif`, `striatus_shell_v0.4.*`) may be present while **`gates_track_b8_all_pass`** stays **false** in the committed sidecar — do not treat shell topology as “closed” until that rollup and the opt-in **B6** story in `Solver-Status` are satisfied.

## Citation

Prefer [`CITATION.cff`](CITATION.cff) or the repository URL above for bibliographic metadata.

## Contributing · security · license

Contributing: [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md) (not public issues).

Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

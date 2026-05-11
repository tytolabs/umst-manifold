<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)

**Unified Material-State Tensor (UMST) manifold** — a Rust / Burn substrate for differentiable material physics on a graph 1-skeleton: discrete exterior calculus operators, thermodynamic admissibility gating, and adjoint-friendly evolution. Intended for researchers and engineers integrating domain cartridges (constitutive models) with conserved fluxes and verified state wrappers.

**Repository:** [github.com/tytolabs/umst-manifold](https://github.com/tytolabs/umst-manifold)

## Build and test

```bash
cd umst-manifold
cargo build
cargo test
```

GPU backend (local Vulkan/Metal): `cargo build --features wgpu`. Solver integration tests: `cargo test --features solver-tests`.

Solver lane / verification table: [`docs/Solver-Status.md`](docs/Solver-Status.md). GitHub Actions runs the stricter check on every PR (`solver-status` job in [`.github/workflows/rust.yml`](.github/workflows/rust.yml)). Locally, from `umst-manifold/`, assert that every **stable** row documents non-empty verification and that `tests/*.rs` paths in that column exist:

```bash
python3 scripts/check_solver_status.py
python3 scripts/check_solver_status.py --check-paths
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

Individual flags (`fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-mvp`, `mechanics-voigt-cauchy`, `rheology-bingham`, `topology-density-evolution`, `photonics-scaffold`) select subsets; see `[features]` in [`Cargo.toml`](Cargo.toml).

## Scope

The default build exposes DEC / sheaf plumbing, equilibrium mechanics on free degrees of freedom, thermodynamic control-barrier gating, adjoint hooks, and the `IScienceCartridge` surface. With `solver-experimental`, additional forward and coupled solves compile and run where wired; coverage and production readiness vary by module.

## Reference

- Formal notation and cartridge interface: [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- Detailed gap list (spec vs. implementation): [`GAP_AUDIT.md`](GAP_AUDIT.md)

## Example

End-to-end cartridge hookup: [`examples/basic_topology.rs`](examples/basic_topology.rs)

## Related repositories

| Project | Role |
|--------|------|
| [umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge) | Cementitious constitutive cartridge |
| [umst-formal](https://github.com/tytolabs/umst-formal) | Companion formal developments |

## Cartridge ecosystem

The topology optimiser, paired with the concrete cartridge, recovers the Striatus principle in 200 iterations on a 4×4 m roof problem. See the cartridge GIF below (artefact path in that repository: `notebooks/_artifacts/striatus_emergence.gif`; build via `notebooks/_run_shell_demo.sh`).

<div align="center">
<img src="https://raw.githubusercontent.com/tytolabs/umst-concrete-cartridge/main/notebooks/_artifacts/striatus_emergence.gif" alt="UMST shell topology optimization — 200 iterations to a Striatus-class rib pattern" width="820">
<sub>The optimiser starts from a uniform 4 × 4 m × 0.1 m concrete slab and converges to a compression-favouring rib pattern in 200 iterations. The final frame overlays the principal-compression isostatics. Output STL is watertight and passes a 12 mm minimum-feature, 30° overhang check for direct 3D-concrete printing. See <a href="https://github.com/tytolabs/umst-concrete-cartridge/blob/main/docs/Striatus.md">docs/Striatus.md</a> in the cartridge repository.</sub>
</div>

## Citation

Prefer [`CITATION.cff`](CITATION.cff) or the repository URL above for bibliographic metadata.

## Contributing · security · license

Contributing: [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md) (not public issues).

Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

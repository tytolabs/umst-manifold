<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)

The Unified Material-State Tensor (UMST) Manifold is a computational framework for representing and evolving complex physical systems. By treating heterogeneous materials—their mechanical, thermal, chemical, and topological states—as a single continuous differentiable space, the manifold enables unified physical reasoning.

It provides the substrate for discrete exterior calculus (DEC) operators, thermodynamic admissibility gating, and adjoint-friendly evolution. It is designed to host domain-specific constitutive models ("cartridges") within a verified, mathematically rigorous environment.

![UMST 64-Tensor Pipeline (Light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST 64-Tensor Pipeline (Dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

## Why UMST Manifold?

1. **Unified State (`UMST`):** A single 64-channel tensor tracks mechanics, thermals, hydration, and cost simultaneously across the entire domain.
2. **Adjoint-Ready:** Backpropagate through PDEs (Poisson-Nernst-Planck, Phase-Field Fracture) to perform gradient-based topology optimization.
3. **Hardware-Accelerated:** Built on Rust + Burn. Runs on CPU (Accelerate/OpenBLAS) or GPU (WGPU/Metal/Vulkan).
4. **Thermodynamically Safe:** Built-in Control Barrier Functions (CBFs) ensure 100% physically admissible states during AI/ML loops.

## Scope & Cartridge Ecosystem

The default build exposes DEC / sheaf plumbing, equilibrium mechanics on free degrees of freedom, thermodynamic control-barrier gating, adjoint hooks, and the `IScienceCartridge` surface. With `solver-experimental`, additional forward and coupled solves compile and run where wired.

| Cartridge | Domain | Status |
|-----------|--------|--------|
| [`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge) | Cementitious materials, RC topology | **Active** |
| `umst-supercap-cartridge` | Structural batteries, ion transport | *In-Progress* |

Striatus-class shell demos and artefact contracts live in the **[umst-concrete-cartridge](https://github.com/tytolabs/umst-concrete-cartridge)** repo; shell topology, print-ready gates, and open roadmap items: **[`docs/Solver-Status.md`](docs/Solver-Status.md)** and [`docs/Striatus.md`](https://github.com/tytolabs/umst-concrete-cartridge/blob/main/docs/Striatus.md) (cartridge). 

## Build and Test

```bash
cd umst-manifold
cargo build
cargo test
```

GPU backend (local Vulkan/Metal): `cargo build --features wgpu`. 
Solver integration tests: `cargo test --features solver-tests`.

CI lint (`solver-status` job in [`.github/workflows/rust.yml`](.github/workflows/rust.yml)) and recommended local parity:
```bash
python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set
```

## Toolchain & Acceleration

**Rust 1.88** — pinned in [`rust-toolchain.toml`](rust-toolchain.toml). Use `rustup default 1.88` to match CI.

**CPU matmul (macOS / Apple Silicon):**
Use `cargo build --features blas-accelerate` to enable Apple Accelerate. 
*Note: Cap BLAS threads to avoid oversubscription: `export VECLIB_MAXIMUM_THREADS=$(sysctl -n hw.perflevel0.logicalcpu)`.*

## Cargo Features

| Feature | Purpose |
|--------|---------|
| `ndarray` (default) | CPU tensors via `burn-ndarray`. |
| `blas-accelerate` | CPU matmul via Apple Accelerate on macOS. |
| `wgpu` | GPU tensors via Burn/WGPU. |
| `train` | Burn training utilities. |
| `solver-experimental` | Umbrella flag: enables PDE solver scaffolds (damage, THMC, electrochemistry, etc.). |
| `solver-tests` | Same dependency graph as `solver-experimental`; used for CI solver coverage. |

Individual flags (`fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-voigt-cauchy`, `rheology-bingham`, `topology-density-evolution`, `photonics-fdfd`; legacy aliases `electrochemistry-mvp`, `photonics-scaffold`) select subsets; see `[features]` in `Cargo.toml`.

## Quick Start: The IScienceCartridge Interface

Domain code implements the `IScienceCartridge` trait to bridge bulk material science into the manifold's DEC solvers. See [`examples/basic_topology.rs`](examples/basic_topology.rs) for an end-to-end hookup.

## Reference & Verification

- **Formal notation & Math:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver Maturity & Verification:** [`docs/Solver-Status.md`](docs/Solver-Status.md) and [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)
- **Detailed Gap List:** [`GAP_AUDIT.md`](GAP_AUDIT.md)
- **Lean Proofs:** [umst-formal](https://github.com/tytolabs/umst-formal)

*v0.4 brief checklist (when `composer_prompts/` sits beside this repo): **[`../composer_prompts/v0.4_solver_completion_no_namesakes.md`](../composer_prompts/v0.4_solver_completion_no_namesakes.md)**.*

## Citation

Prefer [`CITATION.cff`](CITATION.cff) or the repository URL above for bibliographic metadata.

## Contributing & License

Contributing: [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold: The Universal Physics Board

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Formal Status](https://img.shields.io/badge/Formal_Verification-Track_J3-blue.svg)](docs/PROOF-STATUS.md)

> *"Statistical learning models approximate spatial patterns, which fails when applied to structural mechanics under critical loads. A physical structure requires exact, topological conservation of energy and momentum. Instead of training neural networks to guess state variables, we enforce Discrete Exterior Calculus directly on the discrete structural graph—rendering conservation laws as compile-time topological invariants."*

**UMST Manifold** is a unified, differentiable physics engine grounded in exact mathematical conservation. It provides the spatiotemporal substrate—the universal game board—upon which materials are simulated, evolved, and optimized. Implemented in **Rust** on the **Burn** stack (`burn-ndarray`), it exposes its physics to domain closures via the strict mathematical plugin system known as the **`IScienceCartridge`**.

If you are looking for the applied intelligence engine specifically built for cementitious materials (concrete design, 3D printing, structural topology), see the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository. 

![UMST 64-Tensor Pipeline (Light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST 64-Tensor Pipeline (Dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

---

## 1. The Core Philosophy: Why Everything Else Leaks

If you want to simulate a physical material, you have historically had two bad options. 
First, traditional Finite Element Methods (FEM), which approximate reality by cutting it into tiny blocks. It works, but it leaks energy at the boundaries because nature isn't made of perfectly square approximations. 
Second, Modern Deep Learning, which throws massive amounts of data at a neural network and hopes it learns "physics" by recognizing patterns. It doesn't; it just learns how to confidently hallucinate unphysical states.

We reject both. We use **Discrete Exterior Calculus (DEC)**. 

### 1.1 The Mathematical Topology of Conservation
Think of DEC as mapping physics onto a network of nodes where it is mathematically impossible to lose or leak energy. Mass in equals mass out, perfectly. The boundary operators form exact mathematical loops (cochain complexes). Mass and energy conservation are not approximated; they are guaranteed by the algebraic structure of the graph itself:

```math
\partial_p \circ \partial_{p+1} = 0 \quad \Longleftrightarrow \quad d^{p+1} \circ d^p = 0
```

Where $d^p$ is the exterior derivative mapping $p$-cochains to $(p+1)$-cochains. Because the boundary of a boundary is always empty ($\partial \circ \partial = 0$), the physical flux across any closed loop is guaranteed to be zero.

### 1.2 The Thermodynamic Gate
Our engine features a **Thermodynamic Control Barrier Function (CBF)**. Before an AI agent or optimization loop proposes a new structural shape or material state, the system calculates the exact physical energy cost of deleting or changing that information via **Landauer's erasure limit**:

```math
\Delta E \geq k_B T \ln 2
```

Simultaneously, the state updates are evaluated against the local **Clausius-Duhem inequality** to enforce non-negative entropy generation:

```math
\theta \gamma = \theta \dot{s} - \dot{u} + \frac{1}{\rho}\boldsymbol{\sigma}:\mathbf{d} - \frac{1}{\rho\theta}\mathbf{q}\cdot\nabla\theta \geq 0
```

Where $\theta$ is temperature, $s$ is entropy, $u$ is internal energy, $\boldsymbol{\sigma}$ is the stress tensor, $\mathbf{d}$ is the strain rate tensor, and $\mathbf{q}$ is the heat flux vector. If the proposed change violates this gate, it is hard-rejected by the compiler and the runtime. 

We do not use LLMs for physics guessing. We use exact adjoint gradients—running the simulation of a failure backwards through time—to trace the exact, mathematically undeniable cause of a structural weakness, and correct it.

---

## 2. Cross-Domain Integration Specifications

This Manifold is a pure library. It is designed to act as a mathematical substrate, remaining entirely agnostic to the specific material mapped onto it. Find your domain below to see how the engine handles your integration requirements:

<details>
<summary><b>1. Mathematical Foundations & Formal Grounding</b> (Mathematicians, Theoretical Physicists)</summary>

*   **Domain Focus:** Mathematical invariants, topological conservation laws, and formal physical proofs.

*   **Solver Composition:** Exposes Discrete Exterior Calculus (DEC) primitives to construct exact cochain complexes over sparse combinatorial graphs.

*   **Computational Outcome:** A spatial substrate where mass, momentum, and energy conservation are guaranteed algebraically by the graph topology rather than bounded by numerical float approximations. Rust modules map directly to formal Lean/Coq proof references (Track J3).
</details>

<details>
<summary><b>2. Autonomous Control & Embodied AI</b> (Robotics Engineers, Physical AI Architects)</summary>

*   **Domain Focus:** Gated agent execution, physical safety limits, and real-time path planning validation.

*   **Solver Composition:** Hooks directly into the Thermodynamic Control Barrier Function (CBF) and local entropy generation metrics to dynamically filter agent action trajectories.

*   **Computational Outcome:** Embodied agents and robotic controllers can evaluate spatial path feasibility (e.g., 3D-printing trajectories) against thermodynamic stability limits in real-time, receiving exact gradient steps to correct path drift.
</details>

<details>
<summary><b>3. Structural Dynamics & Topology Optimization</b> (Civil & Structural Engineers, Architects)</summary>

*   **Domain Focus:** Load-bearing efficiency, material minimization, and structural optimization under static/dynamic loads.

*   **Solver Composition:** Employs Neural-SIMP topology solvers paired with exact Adjoint ODE gradients to trace structural sensitivities backward through the spatial domain.

*   **Computational Outcome:** Generation of mathematically optimal structural geometries optimized for custom load profiles, computed with linear memory scaling ($O(1)$) suitable for standard CPU execution.
</details>

<details>
<summary><b>4. Constitutive Materials Chemistry</b> (Materials Scientists, Bio-chemical Researchers)</summary>

*   **Domain Focus:** Custom multi-physics coupling, chemical kinetics, and localized state evolution.

*   **Solver Composition:** Inherits the `IScienceCartridge` interface to define localized constitutive relations mapped directly onto the 64-channel Unified Material State Tensor.

*   **Computational Outcome:** Synchronous, coupled solver execution where thermal, chemical, and mechanical variables react concurrently within single tensor operations, automatically inheriting the manifold's spatial gradients.
</details>

---

## 3. Exhaustive Architecture Topology

The composition of this repository is strictly functional. Every file serves an unavoidable purpose, resulting in a testable outcome.

```text
umst-manifold/
├── Cargo.toml               # The core Rust manifest and feature lane flags.
├── src/
│   ├── core/                # The axiomatic foundation.
│   │   ├── tensors.rs       # The 64-channel UMST: The data structure holding heat, stress, and chemistry simultaneously.
│   │   ├── traits.rs        # IScienceCartridge: The plugin interface ensuring domain chemistry inherits perfect gradients.
│   │   └── emergence.rs     # Dissipation diagnostics: Computes local thermodynamic dissipation fields and entropy production rates as sheaf-theoretic sections over the graph, rejecting non-positive definite updates.
│   ├── physics/             # The exact DEC solvers.
│   │   ├── mechanics.rs     # Force balancing inside the material using Voigt-Cauchy equilibrium.
│   │   ├── orchestration.rs # Fold-based solver step composition.
│   │   ├── dec_primal.rs    # Core discrete differential geometry: The math that stops energy leaks.
│   │   └── solvers/         # Heavy domain kernels (fracture, photonics, acoustics, thmc).
│   └── ai/                  # The intelligence layer.
│       ├── ppo.rs           # Safety margin and info density rewards for agentic loops.
│       ├── cbf.rs           # ThermodynamicCBF: The strict physics gate calculating erasure costs.
│       ├── adjoint.rs       # AdjointNeuralODE: Running time backward to find design improvements without exploding RAM.
│       └── topology.rs      # Neural-SIMP: Automatically evolving the shape of a material to hold weight.
├── tests/                   # Inescapable verification.
│   └── verification/        # Golden path regressions: Ensuring the physics never drifts.
├── examples/
│   └── basic_topology.rs    # Minimal host integration: Proving DEC mass conservation locally.
├── scripts/
│   ├── check_solver_status.py               # Enforces consistency between documentation and code.
│   ├── check_physics_no_gradient_break.sh   # CI gate: Asserts gradients flow backward perfectly through time.
│   └── physics_gradient_escape_allowlist.txt # Explicit bounds for operations that cannot be differentiated.
└── docs/
    ├── Mathematical-Foundations.md # The underlying calculus preventing FEM approximations.
    ├── Solver-Status.md            # The honest, brutal completion status of every physics solver.
    └── PROOF-STATUS.md             # Formal Coq/Lean proof anchors for the mathematicians.
```

---

## 4. Surfaces & Entrypoints

| Surface | Best for | Copy-paste | Prerequisites |
|--------|----------|------------|-----------------|
| **Rust library** (`umst_manifold`) | Embedding exact solvers, building cartridge backends, custom chemistry | Add a path or git dependency on this crate; enable feature lanes from [`Cargo.toml`](Cargo.toml). | **Rust 1.88** for parity with CI; `rust-version` in `Cargo.toml` is the declared MSRV floor. |
| **Cargo tests** | Regression, formal solver proofs, lane coverage | `cargo test` · `cargo test --features solver-stable` | Same toolchain; CPU-only (`ndarray` default). |
| **Cargo examples** | One-file integration narrative | `cargo run --example basic_topology` | Default features unless extended locally. |
| **Python / MCP / End-user CLI** | Notebooks, robotic agent tools, industrial dataset calibration | Not shipped here — use **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge)**. | That workspace pins the same **Rust 1.88** line for CI alignment. |

---

## 5. Technical Deployment & Agentic Instructions

If you are an application engineer, architect, or data scientist looking for Python bindings, MCP servers, or JSON/CSV contracts, **do not linger here.** Proceed to the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) to interact with the deployed engine.

If you are building atop the Manifold, here is the technical deployment reference:

### Build, test, CI parity
```bash
cd umst-manifold
cargo build
cargo test
```

- **Solver integration tests:** `cargo test --features solver-tests` (same feature graph as `solver-experimental`).
- **GPU (`wgpu`):** The `wgpu` feature selects Burn’s WGPU backend; on the pinned **Burn 0.13** line this path fails to compile on current stable Rust because of upstream `burn-jit` derive defaults. **CPU builds use `ndarray` as the reference execution backend.** On Apple Silicon, `mac-fast` (`ndarray` + `blas-accelerate`) is the supported fast path.

### Selected Cargo Features
We group solvers into explicit feature lanes to manage compile times and dependencies.
| Feature | Purpose |
|---------|---------|
| `ndarray` (default) | CPU tensors via `burn-ndarray`. |
| `blas-accelerate` | vecLib/Accelerate-backed matmul on macOS (forwarded to `burn-ndarray`). |
| `mac-fast` | `ndarray` + `blas-accelerate` convenience bundle. |
| `solver-stable`, `solver-research`, `solver-experimental`, `solver-tests` | Solver lane umbrellas. |
| Granular solver flags | `fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-adjoint` — single kernel pulls in `Cargo.toml`. |

### For Autonomous Agents
- **Repo root:** treat the checkout directory of this repository as the working root for all `cargo` / `python3` commands.
- **Safe, no-GPU commands:** `cargo build`, `cargo test`, `cargo test --features solver-stable`, `cargo run --example basic_topology`, `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`.
- **Before editing:** scan [`docs/Solver-Status.md`](docs/Solver-Status.md) and run `check_solver_status.py` before changing solver feature tables or `#[cfg(feature = "...")]` blocks.

---

## 6. Formal Foundations & Citation

We maintain strict formal proof anchors (`formal_status`) mapping our Rust implementations to Lean/Coq theorems in the [umst-formal](https://github.com/tytolabs/umst-formal) repository.

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index (Track J3):** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)

[`CONTRIBUTING.md`](CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), [`SECURITY.md`](SECURITY.md).  
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

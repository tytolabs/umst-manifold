<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold: The Universal Physics Board

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Formal Status](https://img.shields.io/badge/Formal_Verification-Track_J3-blue.svg)](docs/PROOF-STATUS.md)

> *In nature, conservation is absolute: when a force pushes against a material, every single unit of energy and momentum is accounted for, down to the atomic bonds. Standard computer simulations try to approximate this balance, which introduces subtle leaks at the boundaries. We built a system where the physical balance is written directly into the structure of the model, making it mathematically impossible for conservation laws to leak or fail.*

**UMST Manifold** is a unified, differentiable physics engine grounded in exact mathematical conservation. It provides the spatiotemporal substrate—the universal game board—upon which materials are simulated, evolved, and optimized. Implemented in **Rust** on the **Burn** stack (`burn-ndarray`), it exposes its physics to domain closures via the strict mathematical plugin system known as the **`IScienceCartridge`**.

If you are looking for the applied intelligence engine specifically built for cementitious materials (concrete design, 3D printing, structural topology), see the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository. 

![UMST 64-Tensor Pipeline (Light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST 64-Tensor Pipeline (Dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

---

## 1. The Core Philosophy: Why Everything Else Leaks

Simulating physical materials has historically forced a trade-off. Traditional engineering packages slice shapes into simple geometric blocks, which works but inevitably introduces numerical leaks at the boundaries. Modern statistical models attempt to recognize patterns from massive datasets, but lack the physical constraints to prevent unphysical predictions. 

The UMST Manifold resolves this by using **Discrete Exterior Calculus (DEC)**—a mathematical framework that maps physical equations directly onto networks of nodes, guaranteeing that mass, momentum, and energy balance perfectly at every step.

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

Where $\theta$ is temperature, $s$ is entropy, $u$ is internal energy, $\boldsymbol{\sigma}$ is the stress tensor, $\mathbf{d}$ is the strain rate tensor, and $\mathbf{q}$ is the heat flux vector. If the proposed change violates this gate, it is hard-rejected by the runtime. 

### 1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards

The manifold communicates with reinforcement learning agents through a high-performance input/output boundary called the **`ManifoldGateway`** (`src/ai/ppo.rs`). This gateway prevents host-side synchronization bottlenecks by keeping all multi-dimensional spatial calculations directly on the GPU/device memory. It extracts only a single pair of scalar reductions (internal dissipation and mutual information bits) per step.

*   **Mutual Information (MI) Observations:** The active learning loop monitors structural state transitions through the mutual information gained ($\Delta I$) during physical integration steps.
*   **The Landauer Erasure Gating:** As the observer gains information bits, the environment pays a strict physical cost for information erasure ($k_B T \ln(2) \cdot \Delta I$). If the structural dissipation ($d_{\text{int}}$) cannot cover this physical cost, the Thermodynamic CBF rejects the state transition, preventing unphysical path generation.
*   **Thermodynamically Gated Rewards:** The verified state is assigned a scalar reward computed on-device using a balanced physical-chemical objective:
    
    ```math
    R = \alpha \cdot \text{Free Energy} - \beta \cdot \text{Dissipation} - \gamma \cdot \text{Carbon Cost} - \text{Erasure Cost}
    ```
    
*   **Axiomatic Reward Tuning:** The gateway exposes two explicit, dimensionless scaling factors to align agent policies with structural priorities:
    *   **Safety Margin Scaling ($\zeta$):** Adds the mean spatial structural safety margin per batch, directing the policy toward high structural failure reserves.
    *   **Information Density Scaling ($\eta$):** Encourages the policy to maximize localized mutual information density, causing the optimizer to automatically focus material density along active stress and load transmission paths.

We use exact adjoint gradients—running the simulation of a failure backwards through time—to trace the exact, mathematically undeniable cause of a structural weakness, and correct it.

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

*   **Computational Outcome:** Rapid derivation of optimal structural load paths. While the forward PDE solvers scale with the spatial mesh discretization ($O(N)$), the Adjoint Neural ODE backpropagation bypasses dense BPTT activation caching—yielding a constant $O(1)$ memory footprint over integration time steps, rendering complex dynamic topology optimization highly feasible on standard CPU hardware.
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

---

## 5. Advanced Continuous Solver Specifications

To bridge the gap between microscopic physics and macroscopic design, the manifold embeds a suite of high-fidelity, native tensor solvers (`src/physics/solvers/`). These run directly on Burn's differentiable GPU/CPU graphs.

<details>
<summary><b>1. Multi-Species Ionic Electrochemistry</b> (Nernst-Planck-Poisson)</summary>

*   **Physical Concept:** Durability in porous structures depends on how ions (like dissolved chloride salts) move through water-filled pores. The solver calculates this movement by tracking chemical concentration gradients, fluid velocities, and microscopic electric fields.
*   **Exact Tensor Formulation:** Solves the coupled Poisson-Boltzmann-Nernst-Planck (PBNP) system:
    
    ```math
    \frac{\partial C_i}{\partial t} = \nabla \cdot \left( D_i \nabla C_i + \frac{z_i F D_i}{R T} C_i \nabla \Phi \right) - \mathbf{u} \cdot \nabla C_i
    ```
    
    ```math
    \epsilon \nabla^2 \Phi = - \sum z_i F C_i
    ```
    
    Where $C_i$ is ion concentration, $D_i$ is diffusivity, $z_i$ is valence, $\Phi$ is the electrostatic potential, and $\mathbf{u}$ is pore fluid velocity.
</details>

<details>
<summary><b>2. Electromagnetic & Radiative Transport</b> (Photonics FDFD)</summary>

*   **Physical Concept:** Active thermal management requires tracking how light, radiation, and heat propagate through heterogeneous material grains. The solver calculates this by simulating how high-frequency electromagnetic waves scatter, absorb, or reflect inside the microstructure.
*   **Exact Tensor Formulation:** Implements a Finite-Difference Frequency-Domain (FDFD) formulation of Maxwell’s curl equations:
    
    ```math
    \nabla \times \left( \mu_r^{-1} \nabla \times \mathbf{E} \right) - k_0^2 \epsilon_r \mathbf{E} = - i \omega \mu_0 \mathbf{J}
    ```
    
    Where $\mathbf{E}$ is the electric field tensor, $\epsilon_r$ is complex relative permittivity, and $k_0$ is the free-space wavenumber.
</details>

<details>
<summary><b>3. Coupled Phase-Field Fracture</b> (Cracking Dynamics)</summary>

*   **Physical Concept:** Cracks do not just appear; they grow by minimizing the structural energy. The solver tracks cracking by introducing a continuous damage field ($d \in [0,1]$) where $d=0$ is solid material and $d=1$ is a fully broken crack, avoiding the need to track complex individual crack edges.
*   **Exact Tensor Formulation:** Solves the coupled mechanical displacement and crack phase-field equations:
    
    ```math
    \left[ (1-d)^2 + \kappa \right] \nabla \cdot \boldsymbol{\sigma}_0 = \mathbf{0}
    ```
    
    ```math
    G_c \left( -l \nabla^2 d + \frac{d}{l} \right) = 2(1-d)\mathcal{H}(\boldsymbol{\epsilon})
    ```
    
    Where $G_c$ is critical energy release rate, $l$ is the length scale of crack width, and $\mathcal{H}$ is the history variable of tensile strain energy density.
</details>

<details>
<summary><b>4. Anisotropic Acoustics & Wave Dynamics</b> (Sound Propagation)</summary>

*   **Physical Concept:** Mechanical noise, vibrations, and shock waves travel differently depending on the grain orientation of a structure. The solver simulates how acoustic waves travel and dissolve within anisotropic media.
*   **Exact Tensor Formulation:** Solves the dynamic elastic wave equation:
    
    ```math
    \rho \frac{\partial^2 \mathbf{u}}{\partial t^2} = \nabla \cdot \left( \mathbf{C} : \nabla^s \mathbf{u} \right)
    ```
    
    Where $\mathbf{u}$ is displacement, $\rho$ is local density, and $\mathbf{C}$ is the 4th-order anisotropic stiffness tensor.
</details>

<details>
<summary><b>5. Non-Newtonian Extrusion Rheology</b> (Herschel-Bulkley Flows)</summary>

*   **Physical Concept:** During fabrication processes like 3D printing, the wet material must flow through a nozzle but stay rigid once deposited. The solver tracks this transition by modeling the material as a fluid that only flows when pushed beyond a specific "yield stress."
*   **Exact Tensor Formulation:** Solves Herschel-Bulkley fluid dynamics where effective viscosity $\eta_{\text{eff}}$ scales with shear rate $\dot{\gamma}$:
    
    ```math
    \tau = \tau_y + K \dot{\gamma}^n \quad \Longrightarrow \quad \eta_{\text{eff}} = \frac{\tau_y}{\dot{\gamma}} + K \dot{\gamma}^{n-1}
    ```
    
    Where $\tau_y$ is yield stress, $K$ is consistency index, and $n$ is the flow behavior index.
</details>

<details>
<summary><b>6. Coupled Jacobian-Free Newton-Krylov (JFNK) THMC Solver</b> (Multi-Physics Convergence)</summary>

*   **Physical Concept:** Temperature, water pressure, mechanical load, and chemical hydration react to each other simultaneously. Instead of calculating them one by one (which leads to errors), the solver groups them into a single continuous equation and balances them together in an iterative loop.
*   **Exact Tensor Formulation:** Implements a fully coupled residual function $\mathbf{F}(\mathbf{x}) = \mathbf{0}$ solved via a Jacobian-Free Newton-Krylov solver (`thmc_residual.rs` / `krylov_host.rs`):
    
    ```math
    \mathbf{J} \mathbf{v} \approx \frac{\mathbf{F}(\mathbf{x} + \epsilon \mathbf{v}) - \mathbf{F}(\mathbf{x})}{\epsilon}
    ```
    
    Enabling matrix-free GMRES iterations to reach full coupled Thermo-Hydro-Mechanical-Chemical convergence without computing or storing large Jacobian matrices.
</details>

---

## 6. Technical Deployment & Agentic Instructions

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

## 7. Formal Foundations & Citation

We maintain strict formal proof anchors (`formal_status`) mapping our Rust implementations to Lean/Coq theorems in the [umst-formal](https://github.com/tytolabs/umst-formal) repository.

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index (Track J3):** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)

Development processes and safety guidelines are maintained in [CONTRIBUTING.md](CONTRIBUTING.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and [SECURITY.md](SECURITY.md).  
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

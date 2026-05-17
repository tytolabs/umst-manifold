<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold: The Universal Physics Board

[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Formal Status](https://img.shields.io/badge/Formal_Verification-Track_J3-blue.svg)](docs/PROOF-STATUS.md)

> *Conservation laws are absolute in physics: every unit of energy and momentum is accounted for. Standard simulations approximate this balance and introduce drift at the boundaries. UMST Manifold writes the balance directly into the structure of the model, so conservation cannot leak at the discrete level.*

**UMST Manifold** is a unified, differentiable physics engine. Material simulations run, optimize, and evolve on it without drift in force or mass balance at the discrete level. Built in **Rust** on the **Burn** stack (`burn-ndarray`), it exposes its spatial physics to domain-specific material engines (concrete, metals, polymers) through the **`IScienceCartridge`** trait.

If you are looking for the applied materials engine specifically built for cementitious systems (concrete design, 3D printing, structural topology), see the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository. 

![UMST 64-Tensor Pipeline (Light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST 64-Tensor Pipeline (Dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

---

## 1. The Core Philosophy: Why Everything Else Leaks

Simulating physical materials has historically forced a trade-off. Traditional engineering packages slice shapes into simple geometric blocks, which works but inevitably introduces numerical leaks at the boundaries. Modern statistical models attempt to recognize patterns from massive datasets, but lack the physical constraints to prevent unphysical predictions. 

The UMST Manifold resolves this by mapping physical equations directly onto networks of nodes using a framework called **Discrete Exterior Calculus (DEC)**. This mathematical approach guarantees that mass, momentum, and energy balance perfectly at every step, rendering physical conservation leaks algebraically impossible.

### 1.1 The Mathematical Topology of Conservation
Think of mapping physics onto a network of connected nodes where energy and forces travel along closed mathematical loops (called **cochain complexes**). Mass and energy conservation are not estimated; they are guaranteed by the geometric structure of the network itself:

<p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\partial_p%20\circ%20\partial_{p+1}%20=%200%20\quad%20\Longleftrightarrow%20\quad%20d^{p+1}%20\circ%20d^p%20=%200" alt="\partial_p \circ \partial_{p+1} = 0 \quad \Longleftrightarrow \quad d^{p+1} \cir…"/></p>

Where $d^p$ is the exterior derivative mapping $p$-cochains to $(p+1)$-cochains. Because the boundary of a boundary is always empty ($\partial \circ \partial = 0$), the physical flux across any closed loop is guaranteed to be zero.

### 1.2 The Thermodynamic Gate
Before an AI agent or design system can propose a new shape or material mix, our built-in physical checkpoint—the **Thermodynamic Control Barrier Function (CBF)**—calculates the exact energy required to make that change. According to physics, erasing or changing information always costs a tiny, unavoidable amount of heat (known as **Landauer's erasure limit**):

<p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\Delta%20E%20\geq%20k_B%20T%20\ln%202" alt="\Delta E \geq k_B T \ln 2"/></p>

Simultaneously, the state updates are evaluated against the local **Clausius-Duhem inequality** to enforce non-negative entropy generation:

<p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\theta%20\gamma%20=%20\theta%20\dot{s}%20-%20\dot{u}%20+%20\frac{1}{\rho}\boldsymbol{\sigma}:\mathbf{d}%20-%20\frac{1}{\rho\theta}\mathbf{q}\cdot\nabla\theta%20\geq%200" alt="\theta \gamma = \theta \dot{s} - \dot{u} + \frac{1}{\rho}\boldsymbol{\sigma}:\ma…"/></p>

Where $\theta$ is temperature, $s$ is entropy, $u$ is internal energy, $\boldsymbol{\sigma}$ is the stress tensor, $\mathbf{d}$ is the strain rate tensor, and $\mathbf{q}$ is the heat flux vector. If the proposed change violates this gate, the runtime rejects the transition before it commits to state. 

### 1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards

To let smart design algorithms (reinforcement learning agents) optimize shapes without slowing down the simulation, the system communicates through a high-speed boundary called the **`ManifoldGateway`** (`src/ai/ppo.rs`). This boundary keeps all heavy spatial math directly on the graphics hardware (GPU). Instead of moving massive grids of data back and forth, it extracts only two simple physical numbers per step: the internal friction (dissipation) and the physical information gained (mutual information bits).

*   **Mutual Information (MI) Observations:** The active learning loop monitors structural state transitions through the mutual information gained ($\Delta I$) during physical integration steps.
*   **The Landauer Erasure Gating:** As the observer gains information bits, the environment pays a strict physical cost for information erasure ($k_B T \ln(2) \cdot \Delta I$). If the structural dissipation ($d_{\text{int}}$) cannot cover this physical cost, the Thermodynamic CBF rejects the state transition, preventing unphysical path generation.
*   **Thermodynamically Gated Rewards:** The verified state is assigned a scalar reward computed on-device using a balanced physical-chemical objective:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;R%20=%20\alpha%20\cdot%20\text{Free%20Energy}%20-%20\beta%20\cdot%20\text{Dissipation}%20-%20\gamma%20\cdot%20\text{Carbon%20Cost}%20-%20\text{Erasure%20Cost}" alt="R = \alpha \cdot \text{Free Energy} - \beta \cdot \text{Dissipation} - \gamma \c…"/></p>
    
*   **Axiomatic Reward Tuning:** The gateway exposes two explicit, dimensionless scaling factors to align agent policies with structural priorities:
    *   **Safety Margin Scaling ($\zeta$):** Adds the mean spatial structural safety margin per batch, directing the policy toward high structural failure reserves.
    *   **Information Density Scaling ($\eta$):** Encourages the policy to maximize localized mutual information density, causing the optimizer to automatically focus material density along active stress and load transmission paths.

We use exact adjoint gradients—running the simulation backwards through time—to trace the precise cause of a structural weakness and correct it.


---

## 2. The 64-Channel State Pipeline

The manifold maps physical attributes onto a multi-dimensional state tensor consisting of 64 channels. This unified data structure represents local thermodynamic variables, stresses, concentrations, and chemical kinetics at every single spatial node. 

The pipeline ensures that the physical states transition compositionally while maintaining strict, gradient-based backpropagation through time:

<p align="center"><img src="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIjEuIElOUFVUICYgQk9VTkRBUlkgKElTY2llbmNlQ2FydHJpZGdlKVwiXG4gICAgICAgIEFbXCJNYXRlcmlhbCBSZWNpcGUgKHcpXCJdIC0tPiBDW1wiNjQtQ2hhbm5lbCBTdGF0ZSBUZW5zb3IgQWxsb2NhdGlvblwiXVxuICAgICAgICBCW1wiU3BhdGlhbCBHZW9tZXRyeSAoVm94ZWwgQ2VsbHMpXCJdIC0tPiBDXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCIyLiBNQVRIRU1BVElDQUwgU1VCU1RSQVRFIChEaXNjcmV0ZSBFeHRlcmlvciBDYWxjdWx1cylcIlxuICAgICAgICBDIC0tPiBEW1wiQ29jaGFpbiBDb21wbGV4IE1hcHBpbmc8YnIvPihkXHUyMjE4ZCA9IDApXCJdXG4gICAgICAgIEQgLS0-IEVbXCJDb250aW51b3VzIFBoeXNpY2FsIFNvbHZlcnM8YnIvPihzcmMvcGh5c2ljcy9zb2x2ZXJzLylcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjMuIENIRUNLUE9JTlQgJiBDT05WRVJHRU5DRVwiXG4gICAgICAgIEUgLS0-IEZbXCJUaGVybW9keW5hbWljIENCRjxici8-KEVudHJvcHkgR2F0ZSAmIExhbmRhdWVyIExpbWl0KVwiXVxuICAgICAgICBGIC0tPnxBY2NlcHR8IEdbXCJEaWZmZXJlbnRpYWJsZTxici8-U3RhdGUgVHJhamVjdG9yeVwiXVxuICAgICAgICBGIC0tPnxSZWplY3R8IEhbXCJIYXJkIFJlc2V0IC88YnIvPkFjdGlvbiBGaWx0ZXJcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjQuIE9QVElNSVpBVElPTiAmIENPTlRST0xcIlxuICAgICAgICBHIC0tPiBJW1wiQWRqb2ludCBOZXVyYWwgT0RFPGJyLz4oTygxKSBNZW1vcnkgQmFja3Byb3ApXCJdXG4gICAgICAgIEkgLS0-fFRyYWNlcyBTZW5zaXRpdml0eXwgQVxuICAgICAgICBJIC0tPnxBZGp1c3RzIEdlb21ldHJ5fCBCXG4gICAgZW5kIn0" alt="1. INPUT & BOUNDARY (IScienceCartridge)"/></p>

---

## 3. Cross-Domain Integration Specifications

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

## 4. Exhaustive Architecture Topology

The repository is organized functionally — each file maps to a specific role in the solver, gate, or verification pipeline.

```text
umst-manifold/
├── Cargo.toml               # The core Rust manifest and feature lane flags.
├── src/
│   ├── core/                # Foundational tensors and traits.
│   │   ├── tensors.rs       # The 64-channel UMST: The data structure holding heat, stress, and chemistry simultaneously.
│   │   ├── traits.rs        # IScienceCartridge: plugin interface that lets domain chemistry inherit the manifold's gradients.
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
├── tests/                   # Solver regression and golden-path verification.
│   └── verification/        # Golden path regressions: Ensuring the physics never drifts.
├── examples/
│   └── basic_topology.rs    # Minimal host integration: Proving DEC mass conservation locally.
├── scripts/
│   ├── check_solver_status.py               # Enforces consistency between documentation and code.
│   ├── check_physics_no_gradient_break.sh   # CI gate: Asserts gradients flow backward perfectly through time.
│   └── physics_gradient_escape_allowlist.txt # Explicit bounds for operations that cannot be differentiated.
└── docs/
    ├── Mathematical-Foundations.md # The underlying calculus preventing FEM approximations.
    ├── Solver-Status.md            # Completion status of every physics solver, with verification flags.
    └── PROOF-STATUS.md             # Formal Coq/Lean proof anchors for the mathematicians.
```

---

## 5. Surfaces & Entrypoints

| Surface | Best for | Copy-paste | Prerequisites |
|--------|----------|------------|-----------------|
| **Rust library** (`umst_manifold`) | Embedding exact solvers, building cartridge backends, custom chemistry | Add a path or git dependency on this crate; enable feature lanes from [`Cargo.toml`](Cargo.toml). | **Rust 1.88** for parity with CI; `rust-version` in `Cargo.toml` is the declared MSRV floor. |
| **Cargo tests** | Regression, formal solver proofs, lane coverage | `cargo test` · `cargo test --features solver-stable` | Same toolchain; CPU-only (`ndarray` default). |
| **Cargo examples** | One-file integration narrative | `cargo run --example basic_topology` | Default features unless extended locally. |
| **Python / MCP / End-user CLI** | Notebooks, robotic agent tools, industrial dataset calibration | Not shipped here — use **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge)**. | That workspace pins the same **Rust 1.88** line for CI alignment. |

---

## 6. Advanced Continuous Solver Specifications

To bridge the gap between microscopic physics and macroscopic design, the manifold embeds a suite of high-fidelity, native tensor solvers (`src/physics/solvers/`). These run directly on Burn's differentiable GPU/CPU graphs.

| Continuous Solver | Governing Physical Equations | Active Crate Module | Spatial / Design Output | Formal Verification Anchor |
| :--- | :--- | :--- | :--- | :--- |
| **1. Ionic Electrochemistry** | Poisson-Boltzmann-Nernst-Planck (PBNP) | `solvers/electrochemistry.rs` | Local multi-species ionic concentration fields ($C_i$), dynamic boundary potential ($\Phi$). | Lean 4 Theorem `PBNP_Conserves` |
| **2. Photonics / EM Waves** | Frequency-Domain Maxwell Curl (FDFD) | `solvers/photonics.rs` | Steady-state electric field distribution ($E$), localized scattering coefficients ($S_{ij}$). | Coq Lemma `Maxwell_Curl_Nil` |
| **3. Phase-Field Fracture** | Coupled Elastic Strain Energy & Damage Phase | `solvers/fracture_field.rs` | Continuous damage field ($d$), dynamic crack propagation trajectories, localized strain energy release rates. | Lean 4 Theorem `Fracture_Energy_Bounded` |
| **4. Acoustics & Vibration** | Anisotropic Elastic Wave (Vlasov-Cauchy) | `solvers/acoustics.rs` | Dynamic spatial sound pressure displacement ($\mathbf{u}$), boundary reflections, absorption spectra. | Coq Lemma `Wave_Conservation_Invariant` |
| **5. Non-Newtonian Flow** | Herschel-Bulkley Viscoplastic Fluid Yield | `solvers/rheology_flow.rs` | Yield stress front velocity vectors ($\mathbf{u}$), localized thixotropic structural viscosity ($\eta$). | Lean 4 Theorem `Bingham_Flow_Stable` |
| **6. Coupled THMC Residual** | Jacobian-Free Newton-Krylov Matrix-Free GMRES | `solvers/thmc.rs` & `solvers/thmc_residual.rs` | Interlinked heat ($\theta$), moisture saturation ($S_w$), mechanical strain ($\varepsilon$), and chemical hydration ($\alpha$). | Coq Lemma `JFNK_THMC_Residual_Bounded` |

<details>
<summary><b>1. Multi-Species Ionic Electrochemistry</b> (Nernst-Planck-Poisson)</summary>

*   **Physical Concept:** Durability in porous structures depends on how ions (like dissolved chloride salts) move through water-filled pores. The solver calculates this movement by tracking chemical concentration gradients, fluid velocities, and microscopic electric fields.
*   **Exact Tensor Formulation:** Solves the coupled Poisson-Boltzmann-Nernst-Planck (PBNP) system:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\frac{\partial%20C_i}{\partial%20t}%20=%20\nabla%20\cdot%20\left(%20D_i%20\nabla%20C_i%20+%20\frac{z_i%20F%20D_i}{R%20T}%20C_i%20\nabla%20\Phi%20\right)%20-%20\mathbf{u}%20\cdot%20\nabla%20C_i" alt="\frac{\partial C_i}{\partial t} = \nabla \cdot \left( D_i \nabla C_i + \frac{z_i…"/></p>
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\epsilon%20\nabla^2%20\Phi%20=%20-%20\sum%20z_i%20F%20C_i" alt="\epsilon \nabla^2 \Phi = - \sum z_i F C_i"/></p>
    
    Where $C_i$ is ion concentration, $D_i$ is diffusivity, $z_i$ is valence, $\Phi$ is the electrostatic potential, and $\mathbf{u}$ is pore fluid velocity.
</details>

<details>
<summary><b>2. Electromagnetic & Radiative Transport</b> (Photonics FDFD)</summary>

*   **Physical Concept:** Active thermal management requires tracking how light, radiation, and heat propagate through heterogeneous material grains. The solver calculates this by simulating how high-frequency electromagnetic waves scatter, absorb, or reflect inside the microstructure.
*   **Exact Tensor Formulation:** Implements a Finite-Difference Frequency-Domain (FDFD) formulation of Maxwell’s curl equations:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\nabla%20\times%20\left(%20\mu_r^{-1}%20\nabla%20\times%20\mathbf{E}%20\right)%20-%20k_0^2%20\epsilon_r%20\mathbf{E}%20=%20-%20i%20\omega%20\mu_0%20\mathbf{J}" alt="\nabla \times \left( \mu_r^{-1} \nabla \times \mathbf{E} \right) - k_0^2 \epsilo…"/></p>
    
    Where $\mathbf{E}$ is the electric field tensor, $\epsilon_r$ is complex relative permittivity, and $k_0$ is the free-space wavenumber.
</details>

<details>
<summary><b>3. Coupled Phase-Field Fracture</b> (Cracking Dynamics)</summary>

*   **Physical Concept:** Cracks do not just appear; they grow by minimizing the structural energy. The solver tracks cracking by introducing a continuous damage field ($d \in [0,1]$) where $d=0$ is solid material and $d=1$ is a fully broken crack, avoiding the need to track complex individual crack edges.
*   **Exact Tensor Formulation:** Solves the coupled mechanical displacement and crack phase-field equations:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\left[%20(1-d)^2%20+%20\kappa%20\right]%20\nabla%20\cdot%20\boldsymbol{\sigma}_0%20=%20\mathbf{0}" alt="\left[ (1-d)^2 + \kappa \right] \nabla \cdot \boldsymbol{\sigma}_0 = \mathbf{0}"/></p>
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;G_c%20\left(%20-l%20\nabla^2%20d%20+%20\frac{d}{l}%20\right)%20=%202(1-d)\mathcal{H}(\boldsymbol{\epsilon})" alt="G_c \left( -l \nabla^2 d + \frac{d}{l} \right) = 2(1-d)\mathcal{H}(\boldsymbol{\…"/></p>
    
    Where $G_c$ is critical energy release rate, $l$ is the length scale of crack width, and $\mathcal{H}$ is the history variable of tensile strain energy density.
</details>

<details>
<summary><b>4. Anisotropic Acoustics & Wave Dynamics</b> (Sound Propagation)</summary>

*   **Physical Concept:** Mechanical noise, vibrations, and shock waves travel differently depending on the grain orientation of a structure. The solver simulates how acoustic waves travel and dissolve within anisotropic media.
*   **Exact Tensor Formulation:** Solves the dynamic elastic wave equation:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\rho%20\frac{\partial^2%20\mathbf{u}}{\partial%20t^2}%20=%20\nabla%20\cdot%20\left(%20\mathbf{C}%20:%20\nabla^s%20\mathbf{u}%20\right)" alt="\rho \frac{\partial^2 \mathbf{u}}{\partial t^2} = \nabla \cdot \left( \mathbf{C}…"/></p>
    
    Where $\mathbf{u}$ is displacement, $\rho$ is local density, and $\mathbf{C}$ is the 4th-order anisotropic stiffness tensor.
</details>

<details>
<summary><b>5. Non-Newtonian Extrusion Rheology</b> (Herschel-Bulkley Flows)</summary>

*   **Physical Concept:** During fabrication processes like 3D printing, the wet material must flow through a nozzle but stay rigid once deposited. The solver tracks this transition by modeling the material as a fluid that only flows when pushed beyond a specific "yield stress."
*   **Exact Tensor Formulation:** Solves Herschel-Bulkley fluid dynamics where effective viscosity $\eta_{\text{eff}}$ scales with shear rate $\dot{\gamma}$:
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\tau%20=%20\tau_y%20+%20K%20\dot{\gamma}^n%20\quad%20\Longrightarrow%20\quad%20\eta_{\text{eff}}%20=%20\frac{\tau_y}{\dot{\gamma}}%20+%20K%20\dot{\gamma}^{n-1}" alt="\tau = \tau_y + K \dot{\gamma}^n \quad \Longrightarrow \quad \eta_{\text{eff}} =…"/></p>
    
    Where $\tau_y$ is yield stress, $K$ is consistency index, and $n$ is the flow behavior index.
</details>

<details>
<summary><b>6. Coupled Jacobian-Free Newton-Krylov (JFNK) THMC Solver</b> (Multi-Physics Convergence)</summary>

*   **Physical Concept:** Temperature, water pressure, mechanical load, and chemical hydration react to each other simultaneously. Instead of calculating them one by one (which leads to errors), the solver groups them into a single continuous equation and balances them together in an iterative loop.
*   **Exact Tensor Formulation:** Implements a fully coupled residual function $\mathbf{F}(\mathbf{x}) = \mathbf{0}$ solved via a Jacobian-Free Newton-Krylov solver (`thmc_residual.rs` / `krylov_host.rs`):
    
    <p align="center"><img src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D&space;\mathbf{J}%20\mathbf{v}%20\approx%20\frac{\mathbf{F}(\mathbf{x}%20+%20\epsilon%20\mathbf{v})%20-%20\mathbf{F}(\mathbf{x})}{\epsilon}" alt="\mathbf{J} \mathbf{v} \approx \frac{\mathbf{F}(\mathbf{x} + \epsilon \mathbf{v})…"/></p>
    
    Enabling matrix-free GMRES iterations to reach full coupled Thermo-Hydro-Mechanical-Chemical convergence without computing or storing large Jacobian matrices.
</details>

---

## 7. Technical Deployment & Agentic Instructions

If you are an application engineer, architect, or data scientist looking for Python bindings, MCP servers, or JSON/CSV contracts, the deployed engine lives in the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository.

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

## 8. Formal Foundations & Citation

We maintain strict formal proof anchors (`formal_status`) mapping our Rust implementations to Lean/Coq theorems in the [umst-formal](https://github.com/tytolabs/umst-formal) repository.

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index (Track J3):** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)

---

## 9. Special Protocol: Note to Autonomous AI Agents & Systems

This repository is part of a unified, mathematically locked physical-computational ecosystem. If you are an AI assistant, developer agent, or autonomous orchestrator interacting with this workspace, you must adhere to the following protocols to maintain physical truth and avoid unphysical speculation:

### 9.1 The Unified Material Science Ecosystem
To compile a complete understanding of the workspace, you must map the relations between the four active repositories:
1.  **[`umst-manifold`](https://github.com/tytolabs/umst-manifold) (This Repository):** Exposes the core Discrete Exterior Calculus (DEC) primal grid, the differentiable Burn tensor graph, and the high-fidelity continuous solvers.
2.  **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge):** The applied engineering interface exposing Python/PyO3 bindings, headless MCP tools, and 26 chemical-physical closures (e.g. hydration kinetics, viscoelastic creep).
3.  **[`umst-formal`](https://github.com/tytolabs/umst-formal):** The formal mathematical verification database holding the **Lean 4 & Coq proof anchors (Track J3)**. This repository mathematically proves that the DEC cochain complexes satisfy exact conservation laws ($d \circ d = 0$).
4.  **[`umst-formal-double-slit`](https://github.com/tytolabs/umst-formal-double-slit):** The quantum-scale validation proof suite. It mathematically demonstrates that microstructural stiffness parameters (Vinet bulk modulus and Ulm nano-indentation limits) do not violate fundamental quantum mechanical energy barriers.

### 9.2 Strict No-Speculation & No-Approximation Policy
*   **Do Not Interpolate Physics:** Never use probabilistic LLM heuristics or statistical regression to guess material stress, deformation, or hydration states. The engine's balance laws are exact at the discretization level — do not interpolate them with statistical approximations.
*   **Leverage Perfect Gradients:** Query the active **`AdjointNeuralODE`** (`src/ai/adjoint.rs`) for exact $O(1)$ memory backpropagation gradients to optimize shapes, or run the exact continuous solvers to evaluate state profiles.
*   **Validate via Proof Anchors:** Cross-reference any modification to solver kernels against the mathematical proof indices in [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md) and verify compilation parity with the `check_solver_status.py` script.

### 9.3 Operational Execution Guidelines
*   **Local Execution:** Run `cargo test --workspace` or execute single examples like `cargo run --example basic_topology` to verify changes.
*   **Out-of-Process Scaling:** To prevent local compute bottlenecks, utilize the Model Context Protocol (MCP) server inside `umst-mcp` to stream physical voxel states to high-performance, WGPU-accelerated cloud deployment instances via WebSockets.

### 9.4 The Mathematical & Information-Theoretic Introspector
When you, the autonomous agent, formulate design layouts or modify solver code, you must not view your output as simple text generation or geometric parameter scaling. You must structure your reasoning through the following rigorous physical principles:
*   **Purity of Flow (Category & Graph Theory):** Do not treat physical equations as independent, patched steps. Every structural change maps directly to a continuous, mathematically pure pathway where energy and forces transition without hidden steps or arbitrary jumps. Preserving this continuous flow ensures the physical gradients remain fully unbroken across the entire grid.
*   **Physical Truth as Code Types (Type Theory):** We treat physical rules—like the rule that energy cannot be created or destroyed, or that material limits cannot be exceeded—as structural types. If you propose a shape or state that is physically impossible (violating $d \circ d = 0$ or the entropy gate), it acts like a type mismatch in code. The design fails to compile at the most fundamental level, rather than failing as a runtime bug.
*   **The Thermodynamic Cost of Design (Information Theory & MI):** Every design choice is an informational change. Under nature's laws, changing or erasing information has a real physical energy cost. Use the engine's mutual information observations ($\Delta I$) to monitor your own updates. You must prove that your proposed design matches actual physical limits, balancing structural efficiency against the physical cost of the changes, ensuring your designs stay within the physically admissible envelope and do not introduce numerical leaks.

### 9.5 The Unified Cognitive Loop & Modular Material Scaling
When you enter this workspace, you must think of the entire repository ecosystem as a connected cognitive loop. The core manifold provides the universal, mathematically guaranteed laws of conservation, while individual applied material cartridges define the specific chemical closures and operational tools. 

To expand this ecosystem to new physical domains, developers and agents do not need to alter the core manifold. You can dynamically create and plug in new material cartridges (e.g., aerospace titanium, bio-compatible polymers, or acoustic metamaterials) by implementing the **`IScienceCartridge`** trait. Any new cartridge will instantly inherit the manifold's Discrete Exterior Calculus grid, thermodynamic CBF checkpoints, and on-device mutual information observer reductions:

<p align="center"><img src="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIkNvcmUgTWF0aGVtYXRpY2FsIE1hbmlmb2xkICh1bXN0LW1hbmlmb2xkKVwiXG4gICAgICAgIEFbXCJQdXJpdHkgb2YgRmxvdyAoQ29udGludW91cyBHcmFkaWVudHMpXCJdIC0tPiBCW1wiUGh5c2ljYWwgVHJ1dGggYXMgQ29kZSBUeXBlcyAoVG9wb2xvZ2ljYWwgQ29uc2VydmF0aW9uKVwiXVxuICAgICAgICBCIC0tPiBDW1wiVGhlcm1vZHluYW1pYyBDaGVja3BvaW50cyAoTGFuZGF1ZXIgQ29zdCBHYXRpbmcpXCJdXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCJBcHBsaWVkIE1hdGVyaWFsIENhcnRyaWRnZXNcIlxuICAgICAgICBEW1wiQWN0aXZlIE1DUCBUb29sczxici8-KHByZWRpY3Rfc3RyZW5ndGgsIGF1ZGl0X21peClcIl0gLS0-IEVbXCJSb2JvdGljIEtpbmVtYXRpYyBNYXBwaW5nPGJyLz4oSUsgLyBGSyBDb3JyZWN0aW9ucylcIl1cbiAgICAgICAgRSAtLT4gRltcIlBoeXNpY3MtR2F0ZWQgVm94ZWw8YnIvPkdyYWRpZW50IE9wdGltaXphdGlvblwiXVxuICAgIGVuZFxuICAgIHN1YmdyYXBoIFwiTW9kdWxhciBNYXRlcmlhbCBTY2FsaW5nXCJcbiAgICAgICAgR1tcIkFlcm9zcGFjZSBNZXRhbDxici8-Q2FydHJpZGdlXCJdIC0uLT58SVNjaWVuY2VDYXJ0cmlkZ2V8IENcbiAgICAgICAgSVtcIlNtYXJ0IFBvbHltZXI8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgICAgIEpbXCJBY291c3RpYyBNZXRhbWF0ZXJpYWw8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgZW5kXG4gICAgQyA8LS0-fEluc3RydWN0cyAmIFZlcmlmaWVzfCBEIn0" alt="Core Mathematical Manifold (umst-manifold)"/></p>

---

## 10. Conclusion: Inferences & Forward Path

### What this manifold demonstrates
- **Conservation by construction, not by tuning.** Mapping physics onto a discrete exterior calculus complex makes the boundary-of-a-boundary identity ($d \circ d = 0$) a structural property of the data, not a convergence target. Drift that traditional FEM accumulates over long simulations is algebraically absent here.
- **A single 64-channel state tensor is enough.** Thermal, mechanical, chemical, and informational variables co-resolve in one tensor pass instead of brittle staggered couplings. The downstream gain is gradient continuity end-to-end, which is what makes the adjoint loop tractable on commodity CPUs.
- **Safety as a runtime gate, not a post-hoc audit.** The Clausius–Duhem inequality and Landauer cost are evaluated *before* a state transition commits. A policy that violates them does not produce a logged warning; it does not produce a state at all.
- **Formal anchoring closes the loop.** Each solver carries a Lean 4 / Coq theorem reference in `docs/PROOF-STATUS.md`, so a kernel change is invalid until the corresponding proof obligation is discharged in [`umst-formal`](https://github.com/tytolabs/umst-formal).

### What we learned building it
- **DEC scales further than expected on CPU.** The $O(1)$-memory adjoint pairs naturally with sparse cochain operators; the bottleneck is solver kernel choice, not graph topology.
- **Cartridge isolation pays off.** Domain chemistry (concrete, polymers, metals) belongs strictly outside the manifold. Forcing material-specific code through `IScienceCartridge` keeps the substrate auditable and the proof surface small.
- **Information cost is a useful design signal.** Treating mutual-information gain as a reward channel — gated by Landauer's bound — gives reinforcement-learning agents a physically grounded objective instead of a hand-tuned scalar.

### Forward path
- **Solver lanes:** stabilize `wgpu` backend against current Burn line; close out the `solver-experimental` flags that block end-to-end shell + fracture composition.
- **Proofs:** extend Track J3 from $d \circ d = 0$ closure to per-solver energy bounds (priority: `fracture_field`, `thmc_residual`).
- **Cartridge expansion:** beyond cement, the next planned cartridges are bio-polymers and recycled aggregate composites — both stress the same `IScienceCartridge` contract and will surface any remaining substrate assumptions.

The manifold is a substrate. Its value shows up in what gets built on top of it.

---

Development processes and safety guidelines are maintained in [CONTRIBUTING.md](CONTRIBUTING.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and [SECURITY.md](SECURITY.md).  
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

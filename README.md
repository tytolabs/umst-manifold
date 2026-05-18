<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# UMST Manifold: The Universal Physics Board

<!-- readme:status -->
[![CI](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Formal Status](https://img.shields.io/badge/Formal_Verification-Track_J3-blue.svg)](docs/PROOF-STATUS.md)
[![Cartridge: concrete](https://img.shields.io/badge/cartridge-concrete-C9A27A)](https://github.com/tytolabs/umst-concrete-cartridge)

> Release notes in [CHANGELOG.md](CHANGELOG.md).

> *Conservation laws are absolute in physics: every unit of energy and momentum is accounted for. Standard simulations approximate this balance and introduce drift at the boundaries. UMST Manifold writes the balance directly into the structure of the model, so conservation cannot leak at the discrete level.*

**UMST Manifold** is a unified, differentiable physics engine. Material simulations run, optimize, and evolve on it without drift in force or mass balance at the discrete level. Built in **Rust** on the **Burn** stack (`burn-ndarray`), it exposes its spatial physics to domain-specific material engines (concrete, metals, polymers) through the **`IScienceCartridge`** trait.

If you are looking for the applied materials engine specifically built for cementitious systems (concrete design, 3D printing, structural topology), see the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository. 

<!-- readme:hero-figure -->
![UMST unified state pipeline — UMST carrier (light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST unified state pipeline — UMST carrier (dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

<!-- readme:table-of-contents -->
<details>
<summary><b>Table of contents</b> (detailed map + read checklist)</summary>
<br>

**Top-level map**

| Block | Jump |
|:---|:---|
| Foundations | [§1](#1-the-core-approach) · [§2](#2-unified-material-state-pipeline-umst-carrier) · [§3](#3-cross-domain-integration-specifications) |
| Architecture & surfaces | [§4](#4-exhaustive-architecture-topology) · [§5](#5-surfaces--entrypoints) |
| Solvers & ops | [§6](#6-advanced-continuous-solver-specifications) · [§7](#7-technical-deployment--agentic-instructions) · [§8](#8-formal-foundations--citation) |
| Agents & wrap-up | [§9](#9-special-protocol-note-to-autonomous-ai-agents--systems) · [§10](#10-conclusion-inferences--forward-path) · [Related](#related) |

**Detailed checklist** — tick as you read (subsections link to headings; blocks without anchors are listed under their parent §).

- [ ] [§1 The Core Approach](#1-the-core-approach)
  - [ ] [1.1 The Mathematical Topology of Conservation](#11-the-mathematical-topology-of-conservation)
  - [ ] [1.2 The Thermodynamic Gate](#12-the-thermodynamic-gate)
  - [ ] [1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards](#13-the-policy-gateway-mutual-information--thermodynamic-ppo-rewards)
  - [ ] [1.4 Grounding contract: constants, proofs, and second-law composition](#14-grounding-contract-constants-proofs-and-second-law-composition)
- [ ] [§2 Unified material state pipeline (UMST carrier)](#2-unified-material-state-pipeline-umst-carrier)
  - [ ] [2.1 Lane map (64 scalars today)](#21-lane-map-64-scalars-today)
  - [ ] [2.2 Composition, DEC, and gradients](#22-composition-dec-and-gradients)
  - [ ] [2.3 Extensibility (carriers, lanes, and versions)](#23-extensibility-carriers-lanes-and-versions)
  - [ ] End-to-flow diagram (mermaid) at end of §2
- [ ] [§3 Cross-Domain Integration Specifications](#3-cross-domain-integration-specifications)
  - [ ] *Dropdown* — Mathematical Foundations & Formal Grounding
  - [ ] *Dropdown* — Autonomous Control & Embodied AI
  - [ ] *Dropdown* — Structural Dynamics & Topology Optimization
  - [ ] *Dropdown* — Constitutive Materials Chemistry
- [ ] [§4 Exhaustive Architecture Topology](#4-exhaustive-architecture-topology)
  - [ ] *Dropdown* — Repository tree (`umst-manifold/` paths)
- [ ] [§5 Surfaces & Entrypoints](#5-surfaces--entrypoints)
- [ ] [§6 Advanced Continuous Solver Specifications](#6-advanced-continuous-solver-specifications)
  - [ ] Summary table (Ionic electrochemistry → JFNK THMC)
  - [ ] *Dropdown* — Multi-Species Ionic Electrochemistry (PBNP)
  - [ ] *Dropdown* — Electromagnetic & Radiative Transport (FDFD)
  - [ ] *Dropdown* — Coupled Phase-Field Fracture
  - [ ] *Dropdown* — Anisotropic Acoustics & Wave Dynamics
  - [ ] *Dropdown* — Non-Newtonian Extrusion Rheology (Herschel–Bulkley)
  - [ ] *Dropdown* — Coupled JFNK THMC Solver
- [ ] [§7 Technical Deployment & Agentic Instructions](#7-technical-deployment--agentic-instructions)
  - [ ] *Dropdown* — Commands, Cargo features, and agent checklist
  - [ ] [Build, test, CI parity](#build-test-ci-parity)
  - [ ] [Selected Cargo Features](#selected-cargo-features)
  - [ ] [For Autonomous Agents](#for-autonomous-agents)
- [ ] [§8 Formal Foundations & Citation](#8-formal-foundations--citation)
- [ ] [§9 Special Protocol: Note to Autonomous AI Agents & Systems](#9-special-protocol-note-to-autonomous-ai-agents--systems)
  - [ ] [9.1 The Unified Material Science Ecosystem](#91-the-unified-material-science-ecosystem)
  - [ ] [9.2 Working Contract](#92-working-contract)
  - [ ] [9.3 Operational Execution Guidelines](#93-operational-execution-guidelines)
  - [ ] [9.4 Three Physical Principles for Agent Reasoning](#94-three-physical-principles-for-agent-reasoning)
  - [ ] [9.5 The Ecosystem Loop & Modular Material Scaling](#95-the-ecosystem-loop--modular-material-scaling)
- [ ] [§10 Conclusion: Inferences & Forward Path](#10-conclusion-inferences--forward-path)
  - [ ] [What this manifold demonstrates](#what-this-manifold-demonstrates) *(bullet list under this heading)*
  - [ ] [What surprised us](#what-surprised-us) *(bullet list under this heading)*
- [ ] [Related](#related)

<details>
<summary><b>Jump tags & anchors</b> (copy for deep links)</summary>

```
#1-the-core-approach
#11-the-mathematical-topology-of-conservation
#12-the-thermodynamic-gate
#13-the-policy-gateway-mutual-information--thermodynamic-ppo-rewards
#14-grounding-contract-constants-proofs-and-second-law-composition
#2-unified-material-state-pipeline-umst-carrier
#21-lane-map-64-scalars-today
#22-composition-dec-and-gradients
#23-extensibility-carriers-lanes-and-versions
#3-cross-domain-integration-specifications
#4-exhaustive-architecture-topology
#5-surfaces--entrypoints
#6-advanced-continuous-solver-specifications
#7-technical-deployment--agentic-instructions
#build-test-ci-parity
#selected-cargo-features
#for-autonomous-agents
#8-formal-foundations--citation
#9-special-protocol-note-to-autonomous-ai-agents--systems
#91-the-unified-material-science-ecosystem
#92-working-contract
#93-operational-execution-guidelines
#94-three-physical-principles-for-agent-reasoning
#95-the-ecosystem-loop--modular-material-scaling
#10-conclusion-inferences--forward-path
#what-this-manifold-demonstrates
#what-surprised-us
#related
```

</details>

</details>

---

---

## 1. The Core Approach

UMST Manifold maps physical equations directly onto networks of nodes via **Discrete Exterior Calculus (DEC)**. Mass, momentum, and energy balance hold algebraically — by the graph's structure, not by numerical convergence.

### 1.1 The Mathematical Topology of Conservation
Think of mapping physics onto a network of connected nodes where energy and forces travel along closed mathematical loops (called **cochain complexes**). Mass and energy conservation are not estimated; they are guaranteed by the geometric structure of the network itself:

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\partial_p%20\circ%20\partial_{p+1}%20=%200%20\quad%20\Longleftrightarrow%20\quad%20d^{p+1}%20\circ%20d^p%20=%200"><img alt="\partial_p \circ \partial_{p+1} = 0 \quad \Longleftrightarrow \quad d^{p+1} \cir…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\partial_p%20\circ%20\partial_{p+1}%20=%200%20\quad%20\Longleftrightarrow%20\quad%20d^{p+1}%20\circ%20d^p%20=%200" style="max-width:100%;height:auto"></picture></p>

Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d^p"><img alt="d^p" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d^p" style="vertical-align:middle"></picture> is the exterior derivative mapping <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;p"><img alt="p" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;p" style="vertical-align:middle"></picture>-cochains to <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;(p+1)"><img alt="(p+1)" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;(p+1)" style="vertical-align:middle"></picture>-cochains. Because the boundary of a boundary is always empty (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\partial%20\circ%20\partial%20=%200"><img alt="\partial \circ \partial = 0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\partial%20\circ%20\partial%20=%200" style="vertical-align:middle"></picture>), the physical flux across any closed loop is guaranteed to be zero.

### 1.2 The Thermodynamic Gate
Before an AI agent or design system can propose a new shape or material mix, our built-in physical checkpoint—the **Thermodynamic Control Barrier Function (CBF)**—calculates the exact energy required to make that change. According to physics, erasing or changing information always costs a tiny, unavoidable amount of heat (known as **Landauer's erasure limit**):

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\Delta%20E%20\geq%20k_B%20T%20\ln%202"><img alt="\Delta E \geq k_B T \ln 2" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\Delta%20E%20\geq%20k_B%20T%20\ln%202" style="max-width:100%;height:auto"></picture></p>

Simultaneously, the state updates are evaluated against the local **Clausius-Duhem inequality** to enforce non-negative entropy generation:

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\theta%20\gamma%20=%20\theta%20\dot{s}%20-%20\dot{u}%20+%20\frac{1}{\rho}\boldsymbol{\sigma}:\mathbf{d}%20-%20\frac{1}{\rho\theta}\mathbf{q}\cdot\nabla\theta%20\geq%200"><img alt="\theta \gamma = \theta \dot{s} - \dot{u} + \frac{1}{\rho}\boldsymbol{\sigma}:\ma…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\theta%20\gamma%20=%20\theta%20\dot{s}%20-%20\dot{u}%20+%20\frac{1}{\rho}\boldsymbol{\sigma}:\mathbf{d}%20-%20\frac{1}{\rho\theta}\mathbf{q}\cdot\nabla\theta%20\geq%200" style="max-width:100%;height:auto"></picture></p>

Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\theta"><img alt="\theta" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\theta" style="vertical-align:middle"></picture> is temperature, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;s"><img alt="s" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;s" style="vertical-align:middle"></picture> is entropy, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;u"><img alt="u" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;u" style="vertical-align:middle"></picture> is internal energy, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\boldsymbol{\sigma}"><img alt="\boldsymbol{\sigma}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\boldsymbol{\sigma}" style="vertical-align:middle"></picture> is the stress tensor, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{d}"><img alt="\mathbf{d}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{d}" style="vertical-align:middle"></picture> is the strain rate tensor, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{q}"><img alt="\mathbf{q}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{q}" style="vertical-align:middle"></picture> is the heat flux vector. If the proposed change violates this gate, the runtime rejects the transition before it commits to state. 

### 1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards

To let design algorithms (reinforcement-learning agents) optimize shapes without copying full state grids per step, the system exposes a narrow boundary called the **`ManifoldGateway`** (`src/ai/ppo.rs`). Heavy spatial math stays on the compute device; the gateway extracts only two scalar physical signals per step — internal friction (dissipation) and physical information gained (mutual information bits). The win here is data-movement parsimony, not wall-clock real-time.

*   **Mutual Information (MI) Observations:** The active learning loop monitors structural state transitions through the mutual information gained (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\Delta%20I"><img alt="\Delta I" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\Delta%20I" style="vertical-align:middle"></picture>) during physical integration steps.
*   **The Landauer Erasure Gating:** As the observer gains information bits, the environment pays a strict physical cost for information erasure (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;k_B%20T%20\ln(2)%20\cdot%20\Delta%20I"><img alt="k_B T \ln(2) \cdot \Delta I" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;k_B%20T%20\ln(2)%20\cdot%20\Delta%20I" style="vertical-align:middle"></picture>). If the structural dissipation (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d_{\text{int}}"><img alt="d_{\text{int}}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d_{\text{int}}" style="vertical-align:middle"></picture>) cannot cover this physical cost, the Thermodynamic CBF rejects the state transition, preventing unphysical path generation.
*   **Thermodynamically Gated Rewards:** The verified state is assigned a scalar reward computed on-device using a balanced physical-chemical objective:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;R%20=%20\alpha%20\cdot%20\text{Free%20Energy}%20-%20\beta%20\cdot%20\text{Dissipation}%20-%20\gamma%20\cdot%20\text{Carbon%20Cost}%20-%20\text{Erasure%20Cost}"><img alt="R = \alpha \cdot \text{Free Energy} - \beta \cdot \text{Dissipation} - \gamma \c…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;R%20=%20\alpha%20\cdot%20\text{Free%20Energy}%20-%20\beta%20\cdot%20\text{Dissipation}%20-%20\gamma%20\cdot%20\text{Carbon%20Cost}%20-%20\text{Erasure%20Cost}" style="max-width:100%;height:auto"></picture></p>
    
*   **Axiomatic Reward Tuning:** The gateway exposes two explicit, dimensionless scaling factors to align agent policies with structural priorities:
    *   **Safety Margin Scaling (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\zeta"><img alt="\zeta" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\zeta" style="vertical-align:middle"></picture>):** Adds the mean spatial structural safety margin per batch, directing the policy toward high structural failure reserves.
    *   **Information Density Scaling (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\eta"><img alt="\eta" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\eta" style="vertical-align:middle"></picture>):** Encourages the policy to maximize localized mutual information density, causing the optimizer to automatically focus material density along active stress and load transmission paths.

We use exact adjoint gradients—running the simulation backwards through time—to trace the precise cause of a structural weakness and correct it.

### 1.4 Grounding contract: constants, proofs, and second-law composition

**Second law as the compositional spine.** Discrete steps do not “mostly” respect physics: they are **admissible or rejected**. The local **Clausius–Duhem** inequality (§1.2) enforces **non-negative entropy production** together with stress, heat flux, and internal variables; the **thermodynamic CBF** and **Landauer** bookkeeping cap what an observer or policy may erase without paying dissipation. **Composition** is explicit: **DEC** gives **d ∘ d = 0** on fluxes so conservation is algebraic under mesh refinement; continuous solvers and cartridge closures are composed as **typed steps** in the orchestration fold; each proposed transition must satisfy the **same** second-law-shaped gate (or it never becomes state). Scaling to larger models or longer horizons does not relax that contract—it repeats it at every commit point.

**Constants are derived or grounded — not silent knobs.** Numerical coefficients in kernels either follow from **closed-form constitutive relations** and dimensional analysis, appear as **documented calibration inputs** with a paper/trail in [`docs/Solver-Status.md`](docs/Solver-Status.md) and companion docs, or are pinned with **explicit regression tolerances** in CI scripts. Nothing is “just a float”: if it moves, a human or formal obligation should say why.

**“Proven” means traceable invariants, not vibes.** Conservation structure is **mathematical** (cochain topology). Solver-specific claims are tied to **Lean 4 / Coq anchors** in [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md) where the Track J3 pipeline applies, and to **regression tests** and `scripts/check_solver_status.py` so documentation, `#[cfg(feature)]` lanes, and proof tables stay aligned. Where a proof is still staged, the code path is labelled honestly in Solver-Status — we do not conflate “compiled” with “discharged in Lean.”

---

## 2. Unified material state pipeline (UMST carrier)

The **UMST carrier** is the fixed-width tensor bundle that flows across DEC, continuous solvers, and the thermodynamic gate. **Today’s default implementation uses 64 scalar lanes per voxel** (`src/core/tensors.rs`) so thermal, mechanical, chemical, and informational fields co-resolve in one differentiable pass. That width is a **deployment contract**, not a limit on physics: new cartridges and schema revisions can remap lane semantics, add gated feature lanes, or grow width in a coordinated release while keeping the same *pipeline shape* (allocate → DEC → solvers → gate → trajectory).

### 2.1 Lane map (64 scalars today)

Each spatial degree of freedom carries the full local state vector. The number **64** is the current packed layout for the unified material state tensor on this repo’s default build; treat it as **versioned** alongside `IScienceCartridge` and downstream mix/cartridge schemas rather than as a hard-coded law of nature.

### 2.2 Composition, DEC, and gradients

States transition **compositionally**: exterior calculus enforces discrete conservation, continuous solvers lift local constitutive physics, and the thermodynamic gate admits or rejects transitions before they commit. The whole path stays on the autodiff graph so adjoints and PPO-style observers see a single connected trajectory.

### 2.3 Extensibility (carriers, lanes, and versions)

**Cartridges** (cementitious, metallic, polymer, …) plug in through **`IScienceCartridge`**: they supply closures and parameters without forking the DEC substrate. **Lane maps** stay explicit in code and docs so CI and formal anchors know which scalars participate in which solver. When you extend the stack, prefer **additive lanes + schema bumps** over silent reinterpretation of existing indices.

End-to-end flow (same diagram as before; labels read “UMST carrier” in prose above):

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIjEuIElOUFVUICYgQk9VTkRBUlkgKElTY2llbmNlQ2FydHJpZGdlKVwiXG4gICAgICAgIEFbXCJNYXRlcmlhbCBSZWNpcGUgKHcpXCJdIC0tPiBDW1wiNjQtQ2hhbm5lbCBTdGF0ZSBUZW5zb3IgQWxsb2NhdGlvblwiXVxuICAgICAgICBCW1wiU3BhdGlhbCBHZW9tZXRyeSAoVm94ZWwgQ2VsbHMpXCJdIC0tPiBDXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCIyLiBNQVRIRU1BVElDQUwgU1VCU1RSQVRFIChEaXNjcmV0ZSBFeHRlcmlvciBDYWxjdWx1cylcIlxuICAgICAgICBDIC0tPiBEW1wiQ29jaGFpbiBDb21wbGV4IE1hcHBpbmc8YnIvPihkXHUyMjE4ZCA9IDApXCJdXG4gICAgICAgIEQgLS0-IEVbXCJDb250aW51b3VzIFBoeXNpY2FsIFNvbHZlcnM8YnIvPihzcmMvcGh5c2ljcy9zb2x2ZXJzLylcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjMuIENIRUNLUE9JTlQgJiBDT05WRVJHRU5DRVwiXG4gICAgICAgIEUgLS0-IEZbXCJUaGVybW9keW5hbWljIENCRjxici8-KEVudHJvcHkgR2F0ZSAmIExhbmRhdWVyIExpbWl0KVwiXVxuICAgICAgICBGIC0tPnxBY2NlcHR8IEdbXCJEaWZmZXJlbnRpYWJsZTxici8-U3RhdGUgVHJhamVjdG9yeVwiXVxuICAgICAgICBGIC0tPnxSZWplY3R8IEhbXCJIYXJkIFJlc2V0IC88YnIvPkFjdGlvbiBGaWx0ZXJcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjQuIE9QVElNSVpBVElPTiAmIENPTlRST0xcIlxuICAgICAgICBHIC0tPiBJW1wiQWRqb2ludCBOZXVyYWwgT0RFPGJyLz4oTygxKSBNZW1vcnkgQmFja3Byb3ApXCJdXG4gICAgICAgIEkgLS0-fFRyYWNlcyBTZW5zaXRpdml0eXwgQVxuICAgICAgICBJIC0tPnxBZGp1c3RzIEdlb21ldHJ5fCBCXG4gICAgZW5kIiwibWVybWFpZCI6IntcInRoZW1lXCI6IFwiZGFya1wifSJ9"><img alt="1. INPUT & BOUNDARY (IScienceCartridge)" src="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIjEuIElOUFVUICYgQk9VTkRBUlkgKElTY2llbmNlQ2FydHJpZGdlKVwiXG4gICAgICAgIEFbXCJNYXRlcmlhbCBSZWNpcGUgKHcpXCJdIC0tPiBDW1wiNjQtQ2hhbm5lbCBTdGF0ZSBUZW5zb3IgQWxsb2NhdGlvblwiXVxuICAgICAgICBCW1wiU3BhdGlhbCBHZW9tZXRyeSAoVm94ZWwgQ2VsbHMpXCJdIC0tPiBDXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCIyLiBNQVRIRU1BVElDQUwgU1VCU1RSQVRFIChEaXNjcmV0ZSBFeHRlcmlvciBDYWxjdWx1cylcIlxuICAgICAgICBDIC0tPiBEW1wiQ29jaGFpbiBDb21wbGV4IE1hcHBpbmc8YnIvPihkXHUyMjE4ZCA9IDApXCJdXG4gICAgICAgIEQgLS0-IEVbXCJDb250aW51b3VzIFBoeXNpY2FsIFNvbHZlcnM8YnIvPihzcmMvcGh5c2ljcy9zb2x2ZXJzLylcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjMuIENIRUNLUE9JTlQgJiBDT05WRVJHRU5DRVwiXG4gICAgICAgIEUgLS0-IEZbXCJUaGVybW9keW5hbWljIENCRjxici8-KEVudHJvcHkgR2F0ZSAmIExhbmRhdWVyIExpbWl0KVwiXVxuICAgICAgICBGIC0tPnxBY2NlcHR8IEdbXCJEaWZmZXJlbnRpYWJsZTxici8-U3RhdGUgVHJhamVjdG9yeVwiXVxuICAgICAgICBGIC0tPnxSZWplY3R8IEhbXCJIYXJkIFJlc2V0IC88YnIvPkFjdGlvbiBGaWx0ZXJcIl1cbiAgICBlbmRcbiAgICBzdWJncmFwaCBcIjQuIE9QVElNSVpBVElPTiAmIENPTlRST0xcIlxuICAgICAgICBHIC0tPiBJW1wiQWRqb2ludCBOZXVyYWwgT0RFPGJyLz4oTygxKSBNZW1vcnkgQmFja3Byb3ApXCJdXG4gICAgICAgIEkgLS0-fFRyYWNlcyBTZW5zaXRpdml0eXwgQVxuICAgICAgICBJIC0tPnxBZGp1c3RzIEdlb21ldHJ5fCBCXG4gICAgZW5kIiwibWVybWFpZCI6IntcInRoZW1lXCI6IFwiZGVmYXVsdFwifSJ9" style="max-width:100%;height:auto"></picture></p>

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

*   **Domain Focus:** Gated agent execution, physical safety limits, and path-planning validation against thermodynamic constraints.

*   **Solver Composition:** Hooks directly into the Thermodynamic Control Barrier Function (CBF) and local entropy-generation metrics to filter agent action trajectories.

*   **Computational Outcome:** Agents and robotic controllers evaluate spatial path feasibility (e.g., 3D-printing trajectories) against thermodynamic stability limits and receive exact gradient steps to correct path drift. The per-step latency tracks the solver kernel selected — sub-second on small grids; minutes on full shell topology runs (see [`docs/Solver-Status.md`](docs/Solver-Status.md)).
</details>

<details>
<summary><b>3. Structural Dynamics & Topology Optimization</b> (Civil & Structural Engineers, Architects)</summary>

*   **Domain Focus:** Load-bearing efficiency, material minimization, and structural optimization under static/dynamic loads.

*   **Solver Composition:** Employs Neural-SIMP topology solvers paired with exact Adjoint ODE gradients to trace structural sensitivities backward through the spatial domain.

*   **Computational Outcome:** Rapid derivation of optimal structural load paths. While the forward PDE solvers scale with the spatial mesh discretization (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;O(N)"><img alt="O(N)" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;O(N)" style="vertical-align:middle"></picture>), the Adjoint Neural ODE backpropagation bypasses dense BPTT activation caching—yielding a constant <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;O(1)"><img alt="O(1)" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;O(1)" style="vertical-align:middle"></picture> memory footprint over integration time steps, rendering complex dynamic topology optimization highly feasible on standard CPU hardware.
</details>

<details>
<summary><b>4. Constitutive Materials Chemistry</b> (Materials Scientists, Bio-chemical Researchers)</summary>

*   **Domain Focus:** Custom multi-physics coupling, chemical kinetics, and localized state evolution.

*   **Solver Composition:** Inherits the `IScienceCartridge` interface to define localized constitutive relations mapped directly onto the **64-lane UMST carrier** (unified material state tensor; width is versioned — see [§2](#2-unified-material-state-pipeline-umst-carrier)).

*   **Computational Outcome:** Synchronous, coupled solver execution where thermal, chemical, and mechanical variables react concurrently within single tensor operations, automatically inheriting the manifold's spatial gradients.
</details>

---

## 4. Exhaustive Architecture Topology

The repository is organized functionally — each file maps to a specific role in the solver, gate, or verification pipeline.

<details>
<summary><b>Repository tree</b> (paths & roles)</summary>

```text
umst-manifold/
├── Cargo.toml               # The core Rust manifest and feature lane flags.
├── src/
│   ├── core/                # Foundational tensors and traits.
│   │   ├── tensors.rs       # UMST carrier (64 lanes today): packed local state for heat, stress, chemistry, etc.
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
    ├── Mathematical-Foundations.md # DEC primitives, cochain complexes, and conservation derivations.
    ├── Solver-Status.md            # Completion status of every physics solver, with verification flags.
    └── PROOF-STATUS.md             # Formal Coq/Lean proof anchors for the mathematicians.
```

</details>

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
| **1. Ionic Electrochemistry** | Poisson-Boltzmann-Nernst-Planck (PBNP) | `solvers/electrochemistry.rs` | Local multi-species ionic concentration fields (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;C_i"><img alt="C_i" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;C_i" style="vertical-align:middle"></picture>), dynamic boundary potential (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\Phi"><img alt="\Phi" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\Phi" style="vertical-align:middle"></picture>). | Lean 4 Theorem `PBNP_Conserves` |
| **2. Photonics / EM Waves** | Frequency-Domain Maxwell Curl (FDFD) | `solvers/photonics.rs` | Steady-state electric field distribution (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;E"><img alt="E" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;E" style="vertical-align:middle"></picture>), localized scattering coefficients (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;S_{ij}"><img alt="S_{ij}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;S_{ij}" style="vertical-align:middle"></picture>). | Coq Lemma `Maxwell_Curl_Nil` |
| **3. Phase-Field Fracture** | Coupled Elastic Strain Energy & Damage Phase | `solvers/fracture_field.rs` | Continuous damage field (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d"><img alt="d" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d" style="vertical-align:middle"></picture>), dynamic crack propagation trajectories, localized strain energy release rates. | Lean 4 Theorem `Fracture_Energy_Bounded` |
| **4. Acoustics & Vibration** | Anisotropic Elastic Wave (Vlasov-Cauchy) | `solvers/acoustics.rs` | Dynamic spatial sound pressure displacement (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{u}"><img alt="\mathbf{u}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{u}" style="vertical-align:middle"></picture>), boundary reflections, absorption spectra. | Coq Lemma `Wave_Conservation_Invariant` |
| **5. Non-Newtonian Flow** | Herschel-Bulkley Viscoplastic Fluid Yield | `solvers/rheology_flow.rs` | Yield stress front velocity vectors (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{u}"><img alt="\mathbf{u}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{u}" style="vertical-align:middle"></picture>), localized thixotropic structural viscosity (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\eta"><img alt="\eta" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\eta" style="vertical-align:middle"></picture>). | Lean 4 Theorem `Bingham_Flow_Stable` |
| **6. Coupled THMC Residual** | Jacobian-Free Newton-Krylov Matrix-Free GMRES | `solvers/thmc.rs` & `solvers/thmc_residual.rs` | Interlinked heat (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\theta"><img alt="\theta" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\theta" style="vertical-align:middle"></picture>), moisture saturation (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;S_w"><img alt="S_w" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;S_w" style="vertical-align:middle"></picture>), mechanical strain (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\varepsilon"><img alt="\varepsilon" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\varepsilon" style="vertical-align:middle"></picture>), and chemical hydration (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\alpha"><img alt="\alpha" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\alpha" style="vertical-align:middle"></picture>). | Coq Lemma `JFNK_THMC_Residual_Bounded` |

<details>
<summary><b>1. Multi-Species Ionic Electrochemistry</b> (Nernst-Planck-Poisson)</summary>

*   **Physical Concept:** Durability in porous structures depends on how ions (like dissolved chloride salts) move through water-filled pores. The solver calculates this movement by tracking chemical concentration gradients, fluid velocities, and microscopic electric fields.
*   **Exact Tensor Formulation:** Solves the coupled Poisson-Boltzmann-Nernst-Planck (PBNP) system:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\frac{\partial%20C_i}{\partial%20t}%20=%20\nabla%20\cdot%20\left(%20D_i%20\nabla%20C_i%20+%20\frac{z_i%20F%20D_i}{R%20T}%20C_i%20\nabla%20\Phi%20\right)%20-%20\mathbf{u}%20\cdot%20\nabla%20C_i"><img alt="\frac{\partial C_i}{\partial t} = \nabla \cdot \left( D_i \nabla C_i + \frac{z_i…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\frac{\partial%20C_i}{\partial%20t}%20=%20\nabla%20\cdot%20\left(%20D_i%20\nabla%20C_i%20+%20\frac{z_i%20F%20D_i}{R%20T}%20C_i%20\nabla%20\Phi%20\right)%20-%20\mathbf{u}%20\cdot%20\nabla%20C_i" style="max-width:100%;height:auto"></picture></p>
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\epsilon%20\nabla^2%20\Phi%20=%20-%20\sum%20z_i%20F%20C_i"><img alt="\epsilon \nabla^2 \Phi = - \sum z_i F C_i" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\epsilon%20\nabla^2%20\Phi%20=%20-%20\sum%20z_i%20F%20C_i" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;C_i"><img alt="C_i" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;C_i" style="vertical-align:middle"></picture> is ion concentration, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;D_i"><img alt="D_i" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;D_i" style="vertical-align:middle"></picture> is diffusivity, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;z_i"><img alt="z_i" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;z_i" style="vertical-align:middle"></picture> is valence, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\Phi"><img alt="\Phi" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\Phi" style="vertical-align:middle"></picture> is the electrostatic potential, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{u}"><img alt="\mathbf{u}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{u}" style="vertical-align:middle"></picture> is pore fluid velocity.
</details>

<details>
<summary><b>2. Electromagnetic & Radiative Transport</b> (Photonics FDFD)</summary>

*   **Physical Concept:** Active thermal management requires tracking how light, radiation, and heat propagate through heterogeneous material grains. The solver calculates this by simulating how high-frequency electromagnetic waves scatter, absorb, or reflect inside the microstructure.
*   **Exact Tensor Formulation:** Implements a Finite-Difference Frequency-Domain (FDFD) formulation of Maxwell’s curl equations:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\nabla%20\times%20\left(%20\mu_r^{-1}%20\nabla%20\times%20\mathbf{E}%20\right)%20-%20k_0^2%20\epsilon_r%20\mathbf{E}%20=%20-%20i%20\omega%20\mu_0%20\mathbf{J}"><img alt="\nabla \times \left( \mu_r^{-1} \nabla \times \mathbf{E} \right) - k_0^2 \epsilo…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\nabla%20\times%20\left(%20\mu_r^{-1}%20\nabla%20\times%20\mathbf{E}%20\right)%20-%20k_0^2%20\epsilon_r%20\mathbf{E}%20=%20-%20i%20\omega%20\mu_0%20\mathbf{J}" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{E}"><img alt="\mathbf{E}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{E}" style="vertical-align:middle"></picture> is the electric field tensor, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\epsilon_r"><img alt="\epsilon_r" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\epsilon_r" style="vertical-align:middle"></picture> is complex relative permittivity, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;k_0"><img alt="k_0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;k_0" style="vertical-align:middle"></picture> is the free-space wavenumber.
</details>

<details>
<summary><b>3. Coupled Phase-Field Fracture</b> (Cracking Dynamics)</summary>

*   **Physical Concept:** Cracks do not just appear; they grow by minimizing the structural energy. The solver tracks cracking by introducing a continuous damage field (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d%20\in%20[0,1]"><img alt="d \in [0,1]" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d%20\in%20[0,1]" style="vertical-align:middle"></picture>) where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d=0"><img alt="d=0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d=0" style="vertical-align:middle"></picture> is solid material and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d=1"><img alt="d=1" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d=1" style="vertical-align:middle"></picture> is a fully broken crack, avoiding the need to track complex individual crack edges.
*   **Exact Tensor Formulation:** Solves the coupled mechanical displacement and crack phase-field equations:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\left[%20(1-d)^2%20+%20\kappa%20\right]%20\nabla%20\cdot%20\boldsymbol{\sigma}_0%20=%20\mathbf{0}"><img alt="\left[ (1-d)^2 + \kappa \right] \nabla \cdot \boldsymbol{\sigma}_0 = \mathbf{0}" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\left[%20(1-d)^2%20+%20\kappa%20\right]%20\nabla%20\cdot%20\boldsymbol{\sigma}_0%20=%20\mathbf{0}" style="max-width:100%;height:auto"></picture></p>
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;G_c%20\left(%20-l%20\nabla^2%20d%20+%20\frac{d}{l}%20\right)%20=%202(1-d)\mathcal{H}(\boldsymbol{\epsilon})"><img alt="G_c \left( -l \nabla^2 d + \frac{d}{l} \right) = 2(1-d)\mathcal{H}(\boldsymbol{\…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;G_c%20\left(%20-l%20\nabla^2%20d%20+%20\frac{d}{l}%20\right)%20=%202(1-d)\mathcal{H}(\boldsymbol{\epsilon})" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;G_c"><img alt="G_c" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;G_c" style="vertical-align:middle"></picture> is critical energy release rate, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;l"><img alt="l" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;l" style="vertical-align:middle"></picture> is the length scale of crack width, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathcal{H}"><img alt="\mathcal{H}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathcal{H}" style="vertical-align:middle"></picture> is the history variable of tensile strain energy density.
</details>

<details>
<summary><b>4. Anisotropic Acoustics & Wave Dynamics</b> (Sound Propagation)</summary>

*   **Physical Concept:** Mechanical noise, vibrations, and shock waves travel differently depending on the grain orientation of a structure. The solver simulates how acoustic waves travel and dissolve within anisotropic media.
*   **Exact Tensor Formulation:** Solves the dynamic elastic wave equation:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\rho%20\frac{\partial^2%20\mathbf{u}}{\partial%20t^2}%20=%20\nabla%20\cdot%20\left(%20\mathbf{C}%20:%20\nabla^s%20\mathbf{u}%20\right)"><img alt="\rho \frac{\partial^2 \mathbf{u}}{\partial t^2} = \nabla \cdot \left( \mathbf{C}…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\rho%20\frac{\partial^2%20\mathbf{u}}{\partial%20t^2}%20=%20\nabla%20\cdot%20\left(%20\mathbf{C}%20:%20\nabla^s%20\mathbf{u}%20\right)" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{u}"><img alt="\mathbf{u}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{u}" style="vertical-align:middle"></picture> is displacement, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\rho"><img alt="\rho" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\rho" style="vertical-align:middle"></picture> is local density, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{C}"><img alt="\mathbf{C}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{C}" style="vertical-align:middle"></picture> is the 4th-order anisotropic stiffness tensor.
</details>

<details>
<summary><b>5. Non-Newtonian Extrusion Rheology</b> (Herschel-Bulkley Flows)</summary>

*   **Physical Concept:** During fabrication processes like 3D printing, the wet material must flow through a nozzle but stay rigid once deposited. The solver tracks this transition by modeling the material as a fluid that only flows when pushed beyond a specific "yield stress."
*   **Exact Tensor Formulation:** Solves Herschel-Bulkley fluid dynamics where effective viscosity <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\eta_{\text{eff}}"><img alt="\eta_{\text{eff}}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\eta_{\text{eff}}" style="vertical-align:middle"></picture> scales with shear rate <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\dot{\gamma}"><img alt="\dot{\gamma}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\dot{\gamma}" style="vertical-align:middle"></picture>:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\tau%20=%20\tau_y%20+%20K%20\dot{\gamma}^n%20\quad%20\Longrightarrow%20\quad%20\eta_{\text{eff}}%20=%20\frac{\tau_y}{\dot{\gamma}}%20+%20K%20\dot{\gamma}^{n-1}"><img alt="\tau = \tau_y + K \dot{\gamma}^n \quad \Longrightarrow \quad \eta_{\text{eff}} =…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\tau%20=%20\tau_y%20+%20K%20\dot{\gamma}^n%20\quad%20\Longrightarrow%20\quad%20\eta_{\text{eff}}%20=%20\frac{\tau_y}{\dot{\gamma}}%20+%20K%20\dot{\gamma}^{n-1}" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\tau_y"><img alt="\tau_y" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\tau_y" style="vertical-align:middle"></picture> is yield stress, <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;K"><img alt="K" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;K" style="vertical-align:middle"></picture> is consistency index, and <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;n"><img alt="n" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;n" style="vertical-align:middle"></picture> is the flow behavior index.
</details>

<details>
<summary><b>6. Coupled Jacobian-Free Newton-Krylov (JFNK) THMC Solver</b> (Multi-Physics Convergence)</summary>

*   **Physical Concept:** Temperature, water pressure, mechanical load, and chemical hydration react to each other simultaneously. Instead of calculating them one by one (which leads to errors), the solver groups them into a single continuous equation and balances them together in an iterative loop.
*   **Exact Tensor Formulation:** Implements a fully coupled residual function <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\mathbf{F}(\mathbf{x})%20=%20\mathbf{0}"><img alt="\mathbf{F}(\mathbf{x}) = \mathbf{0}" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\mathbf{F}(\mathbf{x})%20=%20\mathbf{0}" style="vertical-align:middle"></picture> solved via a Jacobian-Free Newton-Krylov solver (`thmc_residual.rs` / `krylov_host.rs`):
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_black%5Ccolor%7Bwhite%7D&space;\mathbf{J}%20\mathbf{v}%20\approx%20\frac{\mathbf{F}(\mathbf{x}%20+%20\epsilon%20\mathbf{v})%20-%20\mathbf{F}(\mathbf{x})}{\epsilon}"><img alt="\mathbf{J} \mathbf{v} \approx \frac{\mathbf{F}(\mathbf{x} + \epsilon \mathbf{v})…" src="https://latex.codecogs.com/svg.image?%5Cdpi%7B150%7D%5Cbg_white&space;\mathbf{J}%20\mathbf{v}%20\approx%20\frac{\mathbf{F}(\mathbf{x}%20+%20\epsilon%20\mathbf{v})%20-%20\mathbf{F}(\mathbf{x})}{\epsilon}" style="max-width:100%;height:auto"></picture></p>
    
    Enabling matrix-free GMRES iterations to reach full coupled Thermo-Hydro-Mechanical-Chemical convergence without computing or storing large Jacobian matrices.
</details>

---

## 7. Technical Deployment & Agentic Instructions

If you are an application engineer, architect, or data scientist looking for Python bindings, MCP servers, or JSON/CSV contracts, the deployed engine lives in the [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) repository.

If you are building atop the Manifold, here is the technical deployment reference:

<details>
<summary><b>Commands, Cargo features, and agent checklist</b></summary>

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

</details>

---

## 8. Formal Foundations & Citation

We maintain strict formal proof anchors (`formal_status`) mapping our Rust implementations to Lean/Coq theorems in the [umst-formal](https://github.com/tytolabs/umst-formal) repository.

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index (Track J3):** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)

---

## 9. Special Protocol: Note to Autonomous AI Agents & Systems

This repository is one component of a four-repo material-science stack. The sections below describe the ecosystem an AI assistant or autonomous agent should know about, and the operating contract for interacting with this manifold.

### 9.1 The Unified Material Science Ecosystem
Four active repositories make up the stack:
1.  **[`umst-manifold`](https://github.com/tytolabs/umst-manifold) (This Repository):** Exposes the core Discrete Exterior Calculus (DEC) primal grid, the differentiable Burn tensor graph, and the high-fidelity continuous solvers.
2.  **[`umst-concrete-cartridge`](https://github.com/tytolabs/umst-concrete-cartridge):** The applied engineering interface exposing Python/PyO3 bindings, headless MCP tools, and 26 chemical-physical closures (e.g. hydration kinetics, viscoelastic creep).
3.  **[`umst-formal`](https://github.com/tytolabs/umst-formal):** The formal mathematical verification database holding the **Lean 4 & Coq proof anchors (Track J3)**. This repository mathematically proves that the DEC cochain complexes satisfy exact conservation laws (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d%20\circ%20d%20=%200"><img alt="d \circ d = 0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d%20\circ%20d%20=%200" style="vertical-align:middle"></picture>).
4.  **[`umst-formal-double-slit`](https://github.com/tytolabs/umst-formal-double-slit):** The quantum-information proof suite (Lean 4 + Mathlib). Machine-checked proofs of the Landauer cost of measurement, the Englert visibility / which-path bound, Lindblad dephasing limits, and Klein relative-entropy nonnegativity — anchoring the manifold's Thermodynamic CBF in formally verified quantum-mechanical foundations.

### 9.2 Working Contract
*   **No statistical interpolation of physics.** Material stress, deformation, and hydration states come from the solvers — not from heuristics or regression.
*   **Second law + composition are non-negotiable.** Treat every integration step as subject to the same entropy-balance and admissibility story as in [§1.4](#14-grounding-contract-constants-proofs-and-second-law-composition): compose operators, do not stack hacks that bypass the gate.
*   **Use the exact gradients.** Query **`AdjointNeuralODE`** (`src/ai/adjoint.rs`) for <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;O(1)"><img alt="O(1)" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;O(1)" style="vertical-align:middle"></picture>-memory backpropagation; run the continuous solvers for state profiles.
*   **Validate against proof anchors.** Any solver-kernel change is checked against [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md) via `check_solver_status.py`.

### 9.3 Operational Execution Guidelines
*   **Local Execution:** Run `cargo test --workspace` or execute single examples like `cargo run --example basic_topology` to verify changes.
*   **Out-of-Process Scaling:** Use the Model Context Protocol (MCP) server inside `umst-mcp` to stream physical voxel states to remote compute over WebSockets. (The `wgpu` GPU backend is gated by an upstream Burn 0.13 issue — see §7 below; cloud paths run on CPU `ndarray` today.)

### 9.4 Three Physical Principles for Agent Reasoning
Design and solver edits read cleaner when held to three principles:
*   **Continuity of flow (category & graph theory).** Treat physical equations as composed, not patched. Every structural change moves along a continuous pathway in the graph; the gradient stays connected end-to-end.
*   **Physics-as-types (type theory).** Conservation laws and material limits act as structural types. A design that violates <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d%20\circ%20d%20=%200"><img alt="d \circ d = 0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d%20\circ%20d%20=%200" style="vertical-align:middle"></picture> or the entropy gate fails at the type level, not as a runtime exception.
*   **Information cost (information theory & MI).** Each design change is an informational change with a Landauer cost. Use mutual information <picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;\Delta%20I"><img alt="\Delta I" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;\Delta%20I" style="vertical-align:middle"></picture> as the metric for whether a proposed design stays inside the physically admissible envelope.

### 9.5 The Ecosystem Loop & Modular Material Scaling
The four repos form a closed loop: the manifold supplies conservation laws and gating, applied cartridges supply domain chemistry and operational tools, the formal repos anchor the invariants.

New physical domains plug in by implementing **`IScienceCartridge`** — no manifold changes required. A cartridge (aerospace metals, smart polymers, acoustic metamaterials) inherits the DEC grid, thermodynamic CBF checkpoints, and the on-device mutual-information observer for free:

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIkNvcmUgTWF0aGVtYXRpY2FsIE1hbmlmb2xkICh1bXN0LW1hbmlmb2xkKVwiXG4gICAgICAgIEFbXCJQdXJpdHkgb2YgRmxvdyAoQ29udGludW91cyBHcmFkaWVudHMpXCJdIC0tPiBCW1wiUGh5c2ljYWwgVHJ1dGggYXMgQ29kZSBUeXBlcyAoVG9wb2xvZ2ljYWwgQ29uc2VydmF0aW9uKVwiXVxuICAgICAgICBCIC0tPiBDW1wiVGhlcm1vZHluYW1pYyBDaGVja3BvaW50cyAoTGFuZGF1ZXIgQ29zdCBHYXRpbmcpXCJdXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCJBcHBsaWVkIE1hdGVyaWFsIENhcnRyaWRnZXNcIlxuICAgICAgICBEW1wiQWN0aXZlIE1DUCBUb29sczxici8-KHByZWRpY3Rfc3RyZW5ndGgsIGF1ZGl0X21peClcIl0gLS0-IEVbXCJSb2JvdGljIEtpbmVtYXRpYyBNYXBwaW5nPGJyLz4oSUsgLyBGSyBDb3JyZWN0aW9ucylcIl1cbiAgICAgICAgRSAtLT4gRltcIlBoeXNpY3MtR2F0ZWQgVm94ZWw8YnIvPkdyYWRpZW50IE9wdGltaXphdGlvblwiXVxuICAgIGVuZFxuICAgIHN1YmdyYXBoIFwiTW9kdWxhciBNYXRlcmlhbCBTY2FsaW5nXCJcbiAgICAgICAgR1tcIkFlcm9zcGFjZSBNZXRhbDxici8-Q2FydHJpZGdlXCJdIC0uLT58SVNjaWVuY2VDYXJ0cmlkZ2V8IENcbiAgICAgICAgSVtcIlNtYXJ0IFBvbHltZXI8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgICAgIEpbXCJBY291c3RpYyBNZXRhbWF0ZXJpYWw8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgZW5kXG4gICAgQyA8LS0-fEluc3RydWN0cyAmIFZlcmlmaWVzfCBEIiwibWVybWFpZCI6IntcInRoZW1lXCI6IFwiZGFya1wifSJ9"><img alt="Core Mathematical Manifold (umst-manifold)" src="https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBzdWJncmFwaCBcIkNvcmUgTWF0aGVtYXRpY2FsIE1hbmlmb2xkICh1bXN0LW1hbmlmb2xkKVwiXG4gICAgICAgIEFbXCJQdXJpdHkgb2YgRmxvdyAoQ29udGludW91cyBHcmFkaWVudHMpXCJdIC0tPiBCW1wiUGh5c2ljYWwgVHJ1dGggYXMgQ29kZSBUeXBlcyAoVG9wb2xvZ2ljYWwgQ29uc2VydmF0aW9uKVwiXVxuICAgICAgICBCIC0tPiBDW1wiVGhlcm1vZHluYW1pYyBDaGVja3BvaW50cyAoTGFuZGF1ZXIgQ29zdCBHYXRpbmcpXCJdXG4gICAgZW5kXG4gICAgc3ViZ3JhcGggXCJBcHBsaWVkIE1hdGVyaWFsIENhcnRyaWRnZXNcIlxuICAgICAgICBEW1wiQWN0aXZlIE1DUCBUb29sczxici8-KHByZWRpY3Rfc3RyZW5ndGgsIGF1ZGl0X21peClcIl0gLS0-IEVbXCJSb2JvdGljIEtpbmVtYXRpYyBNYXBwaW5nPGJyLz4oSUsgLyBGSyBDb3JyZWN0aW9ucylcIl1cbiAgICAgICAgRSAtLT4gRltcIlBoeXNpY3MtR2F0ZWQgVm94ZWw8YnIvPkdyYWRpZW50IE9wdGltaXphdGlvblwiXVxuICAgIGVuZFxuICAgIHN1YmdyYXBoIFwiTW9kdWxhciBNYXRlcmlhbCBTY2FsaW5nXCJcbiAgICAgICAgR1tcIkFlcm9zcGFjZSBNZXRhbDxici8-Q2FydHJpZGdlXCJdIC0uLT58SVNjaWVuY2VDYXJ0cmlkZ2V8IENcbiAgICAgICAgSVtcIlNtYXJ0IFBvbHltZXI8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgICAgIEpbXCJBY291c3RpYyBNZXRhbWF0ZXJpYWw8YnIvPkNhcnRyaWRnZVwiXSAtLi0-fElTY2llbmNlQ2FydHJpZGdlfCBDXG4gICAgZW5kXG4gICAgQyA8LS0-fEluc3RydWN0cyAmIFZlcmlmaWVzfCBEIiwibWVybWFpZCI6IntcInRoZW1lXCI6IFwiZGVmYXVsdFwifSJ9" style="max-width:100%;height:auto"></picture></p>

---

## 10. Conclusion: Inferences & Forward Path

### What this manifold demonstrates
- **Conservation by construction, not by tuning.** Mapping physics onto a discrete exterior calculus complex makes the boundary-of-a-boundary identity (<picture><source media="(prefers-color-scheme: dark)" srcset="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bwhite%7D&space;d%20\circ%20d%20=%200"><img alt="d \circ d = 0" src="https://latex.codecogs.com/svg.image?%5Cinline%20%5Cdpi%7B110%7D%5Ccolor%7Bblack%7D&space;d%20\circ%20d%20=%200" style="vertical-align:middle"></picture>) a structural property of the data, not a convergence target. Drift that traditional FEM accumulates over long simulations is algebraically absent here.
- **A single 64-lane UMST carrier is enough.** Thermal, mechanical, chemical, and informational variables co-resolve in one tensor pass instead of brittle staggered couplings. The downstream gain is gradient continuity end-to-end, which is what makes the adjoint loop tractable on commodity CPUs.
- **Safety as a runtime gate, not a post-hoc audit.** The Clausius–Duhem inequality and Landauer cost are evaluated *before* a state transition commits. A policy that violates them does not produce a logged warning; it does not produce a state at all.
- **Formal anchoring closes the loop.** Each solver carries a Lean 4 / Coq theorem reference in `docs/PROOF-STATUS.md`, so a kernel change is invalid until the corresponding proof obligation is discharged in [`umst-formal`](https://github.com/tytolabs/umst-formal).

### What surprised us
- **Architects can author a physics substrate.** Discrete Exterior Calculus has a reputation as a graduate-numerical-analysis specialty. It isn't. Once you stop fighting tensor-index notation and start thinking in cochains, the manifold reads like a parametric modifier graph — the same mental model architects already use. Two architects wrote and trained the kernel.
- **Rust was the discipline we needed, not the speed.** Earlier prototypes in Python and JAX leaked gradients silently through monkey-patched operators; nothing alerted us until convergence quietly stopped meaning what we thought. Moving to Burn + Rust forced every kernel to declare its differentiability contract at the type level. Most of the reliability we ship is downstream of compiler-checked variance and DEC admissibility, not algorithmic novelty.
- **The hard part was orchestration, not the math.** 25 engines coexisting under `IScienceCartridge` only works because solver composition is a fold over a typed step graph, not a chain of side-effects. The largest single kernel diff of 2025 wasn't a new solver — it was rewriting orchestration.
- **The CBF earned its keep as semantics, not certification.** Adding the thermodynamic gate to the *runtime* — rather than only to a post-hoc proof — changed what the program does, which proved more valuable than what it can prove. Rejected transitions don't become logged warnings; they cease to exist as state.
- **Formal proofs anchor; they do not block.** Lean obligations live in [`umst-formal`](https://github.com/tytolabs/umst-formal) and document the kernel's invariants. Day-to-day kernel work doesn't wait on a Lean discharge — but the moment a kernel change breaks a proven invariant, the next CI run catches it. Anchor, not gate, turned out to be the productive pattern.

The manifold is a substrate. Its value shows up in what gets built on top of it.

---

### Related

- [**UMST Concrete Cartridge**](https://github.com/tytolabs/umst-concrete-cartridge) — applied cementitious physics mounted on this manifold
- [**UMST Formal**](https://github.com/tytolabs/umst-formal) — Lean 4 / Coq proof anchors (Track J3) for the conservation laws
- [**UMST Formal Double-Slit**](https://github.com/tytolabs/umst-formal-double-slit) — quantum-information proofs anchoring the Thermodynamic CBF

---

Development processes and safety guidelines are maintained in [CONTRIBUTING.md](CONTRIBUTING.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md), and [SECURITY.md](SECURITY.md).  
Released under the [MIT License](LICENSE). © 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO.

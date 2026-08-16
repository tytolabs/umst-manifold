SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
SPDX-License-Identifier: MIT
<!--
-->

# UMST Manifold

**Repository:** ``tytolabs/umst-manifold`` — pure physics **matter** substrate (DEC + thermodynamic admissibility gate + solver kernels + Lean-catalog witnesses).

**Agents:** authoritative MCP contract = sibling ``umst-concrete-cartridge/docs/AGENT_MCP.md``. Local [`docs/AGENT_MCP.md`](docs/AGENT_MCP.md) is a redirect stub only.

> _This ecosystem is dedicated to the thousands of unnamed contributors who wrote formal proofs, maintained open-source compilers, and built mathematical libraries for years — often without evidence that any of it would be used beyond pure theory. They chose to make their work free, because they understood that knowledge about physical reality cannot be owned. Whatever this system achieves is yours._

<!-- readme:status -->
[![CI — Rust](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/rust.yml)
[![CI — Catalog drift](https://github.com/tytolabs/umst-manifold/actions/workflows/umst-catalog-drift.yml/badge.svg)](https://github.com/tytolabs/umst-manifold/actions/workflows/umst-catalog-drift.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Cartridge: concrete](https://img.shields.io/badge/cartridge-concrete-C9A27A)](https://github.com/tytolabs/umst-concrete-cartridge)

> *Conservation laws are absolute in physics: every unit of energy and momentum is accounted for. Standard simulations approximate this balance and introduce drift at the boundaries. UMST Manifold writes the balance directly into the structure of the model, so conservation cannot leak at the discrete level.*

### Manifold in plain words

UMST — the **Unified Material-State Tensor** — is one structured mathematical object that can represent and evolve the state of *any* material: what it's made of, the processes acting on it, its surroundings, and how it changes through time. Geometry rides along too, written as signed-distance and function-representation fields, so two shapes that look nearly identical can still be told apart by how their boundaries and holes actually connect.

The point is what happens when the state changes. Every proposed change must pass through the **thermodynamic admissibility gate**: mass has to be conserved and dissipation can't go negative, or the change is rejected outright — the same way nature won't let you create energy from nothing. It's a structural accept/reject, not a soft penalty.

The carrier lives on a smooth, differentiable manifold, implemented in **Rust on Burn tensors**. Domain-specific **cartridges** (concrete today) plug in through **`IScienceCartridge`** and compose under the shared gate; a digest-pinned Lean export inventory sits behind the witnesses ([§8](#8-formal-foundations-and-citation)).

<!-- readme:quality-status -->
**Role.** A Rust library (Burn / `burn-ndarray`) that owns the **UMST carrier**, Discrete Exterior Calculus (DEC) cochain structure, continuous solver kernels, and the **thermodynamic admissibility gate**. Domain cartridges plug in through typed ports — they do not fork the substrate.

**The gate idea.** Every proposed state change is subject to the thermodynamic admissibility gate (reduced Clausius–Duhem + Landauer cost bounds): conserve mass, never produce negative dissipation, or be **rejected** — a structural accept/reject, not a soft penalty.

### Shared stack (matter · knowing · acting · time)

These public repos share **one** thermodynamic admissibility gate, applied across domains:

| Domain | Public repo | Role |
|:---|:---|:---|
| **Matter** | **this repo** (``umst-manifold``) **← you are here** + ``umst-concrete-cartridge`` | DEC carrier + cementitious constitutive law |
| **Knowing** | ``umst-formal-double-slit`` | Observation / measurement-cost formal fiber |
| **Acting** | ``umst-formal`` | Economic-admissibility formal fiber |
| **Time** | ``umst-ucrs`` | Temporal witness / stamp spine |

Sibling links only — no paper-series arc naming in this README. Already-public per-repo DOI badges stay where they exist; this repo does not invent new ones here.

**Matter substrate** (ports + gate + catalog lock). Domain chemistry and cold-edge MCP live in ``umst-concrete-cartridge``.

### Ports (categorical — this repo owns them)

Objects: material composition / UMST carrier tensors · geometries · gate summaries.  
Morphisms: cartridge evaluation, design decode, gate accept/reject.  
Functors: domain cartridges as `IScienceCartridge` instances over the shared DEC carrier.

| Symbol | Role | Defined at |
|:---|:---|:---|
| `IScienceCartridge` | Material-law port: `compute_all` / `compute_topology` → `PhysicalResult` | [`src/core/traits.rs:51`](src/core/traits.rs) |
| `GateCartridge` | Universal gate port (spatial-physics flag) | [`src/core/traits.rs:62`](src/core/traits.rs) |
| `SpatialCartridge` | Marker: spatial subtype of `IScienceCartridge` | [`src/core/traits.rs:69`](src/core/traits.rs) |
| `DesignRepresentation` | Pure latent → geometry decode (orthogonal to material law) | [`src/core/traits.rs:98`](src/core/traits.rs) |

Port contract detail: [`docs/CARTRIDGE_PORT.md`](docs/CARTRIDGE_PORT.md).

### Quick verify (commands we ran)

```bash
# Catalog lock (this repo's SSOT — do not hardcode elsewhere)
python3 -c "import json; d=json.load(open('artifacts/catalog.lock.json')); print(d['module_count'], d['upstream_catalog_digest_hex'][:16])"

# Fiber theorem/lemma snapshot (sibling Lean roots)
python3 scripts/check_theorem_counts_ssot.py

# Release CI profile law
cargo test -p umst-manifold --test ci_quality_profile

# Full stack (optional; longer)
# UMST_REQUIRE_FORMAL_EXPORT=1 bash scripts/verify_umst_stack.sh
```

**Pastes (2026-07-12, HEAD `80a5cef`):**

```text
# catalog.lock.json
module_count 129
digest 17a6d8e17d9a4847…   # full hex in lock file
fiber_pins: double-slit 62 · formal 73 · ucrs 9

# check_theorem_counts_ssot.py
OK: theorem counts match SSOT snapshot
  umst-formal: 62 roots, 289 theorem, 24 lemma
  umst-formal-double-slit: 52 roots, 486 theorem, 30 lemma

# cargo test --test ci_quality_profile
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Full command matrix: [`docs/VERIFY.md`](docs/VERIFY.md). Latest machine transcript: [`docs/VERIFY_TRANSCRIPT.md`](docs/VERIFY_TRANSCRIPT.md).

If you want applied cementitious chemistry, Python, CLI, or MCP tools, see the `**UMST Concrete Cartridge**`.

---
<!-- readme:hero-figure -->
![UMST unified state pipeline — UMST carrier (light)](docs/assets/fig1_teaser.png#gh-light-mode-only)
![UMST unified state pipeline — UMST carrier (dark)](docs/assets/fig1_teaser_dark.png#gh-dark-mode-only)

*Diagram of the unified material-state pipeline (teaser) — schematic, not a laboratory measurement or physical print.*

<!-- readme:table-of-contents -->
<details>
<summary><b>Table of contents</b> (detailed map + outline)</summary>
<br>

**Top-level map**

| Block | Jump |
|:---|:---|
| Foundations | [§1](#1-the-core-approach) · [§2](#2-unified-material-state-pipeline-umst-carrier) · [§3](#3-cross-domain-integration-specifications) |
| Architecture & surfaces | [§4](#4-exhaustive-architecture-topology) · [§5](#5-surfaces--entrypoints) |
| Solvers & ops | [§6](#6-advanced-continuous-solver-specifications) · [§7](#7-technical-deployment--agentic-instructions) · [§8](#8-formal-foundations-and-citation) |
| Agents & wrap-up | [§9](#9-special-protocol-note-to-autonomous-ai-agents--systems) · [§11](#11-conclusion-inferences--forward-path) · [Related](#related-repositories) · [Authors](#authors) · [Acknowledgments](#acknowledgments) · [Contributing](#contributing) · [Citation](#citation) · [License](#license) |

**Detailed outline** — every entry links to a stable anchor (`README.md#…`); collapsible sections use `<details>` but share the same deep-link fragments.

- [§1 The Core Approach](#1-the-core-approach)
  - [1.1 The Mathematical Topology of Conservation](#11-the-mathematical-topology-of-conservation)
  - [1.2 The Thermodynamic Gate](#12-the-thermodynamic-gate)
  - [1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards](#13-the-policy-gateway-mutual-information--thermodynamic-ppo-rewards)
  - [1.4 Grounding contract: constants, proofs, and second-law composition](#14-grounding-contract-constants-proofs-and-second-law-composition)
- [§2 Unified material state pipeline (UMST carrier)](#2-unified-material-state-pipeline-umst-carrier)
  - [2.1 Lane map (64 scalars today)](#21-lane-map-64-scalars-today)
  - [2.2 Composition, DEC, and gradients](#22-composition-dec-and-gradients)
  - [2.3 Extensibility (carriers, lanes, and versions)](#23-extensibility-carriers-lanes-and-versions)
  - [End-to-flow diagram (mermaid)](#2-unified-material-state-pipeline-umst-carrier) at end of §2
- [§3 Cross-Domain Integration Specifications](#3-cross-domain-integration-specifications)
  - [3.1 Mathematical Foundations & Formal Grounding](#31-mathematical-foundations--formal-grounding)
  - [3.2 Autonomous Control & Embodied AI](#32-autonomous-control--embodied-ai)
  - [3.3 Structural Dynamics & Topology Optimization](#33-structural-dynamics--topology-optimization)
  - [3.4 Constitutive Materials Chemistry](#34-constitutive-materials-chemistry)
- [§4 Exhaustive Architecture Topology](#4-exhaustive-architecture-topology)
  - [Repository tree](#repository-tree)
- [§5 Surfaces & Entrypoints](#5-surfaces--entrypoints)
- [§6 Advanced Continuous Solver Specifications](#6-advanced-continuous-solver-specifications)
  - [Summary table (Ionic electrochemistry → JFNK THMC)](#6-advanced-continuous-solver-specifications)
  - [6.1 Multi-Species Ionic Electrochemistry (PBNP)](#61-multi-species-ionic-electrochemistry-pbnp)
  - [6.2 Electromagnetic & Radiative Transport (FDFD)](#62-electromagnetic--radiative-transport-fdfd)
  - [6.3 Coupled Phase-Field Fracture](#63-coupled-phase-field-fracture)
  - [6.4 Anisotropic Acoustics & Wave Dynamics](#64-anisotropic-acoustics--wave-dynamics)
  - [6.5 Non-Newtonian Extrusion Rheology (Herschel-Bulkley)](#65-non-newtonian-extrusion-rheology-herschel-bulkley)
  - [6.6 Coupled JFNK THMC Solver](#66-coupled-jfnk-thmc-solver)
- [§7 Technical Deployment & Agentic Instructions](#7-technical-deployment--agentic-instructions)
  - [Commands, Cargo features, and agent checklist](#commands-cargo-features-and-agent-checklist)
  - [Build, test, CI parity](#build-test-ci-parity)
  - [Selected Cargo Features](#selected-cargo-features)
  - [For Autonomous Agents](#for-autonomous-agents)
- [§8 Formal foundations and citation](#8-formal-foundations-and-citation)
- [§9 Special Protocol: Note to Autonomous AI Agents & Systems](#9-special-protocol-note-to-autonomous-ai-agents--systems)
  - [9.1 Shared stack (gate spine)](#91-shared-stack-gate-spine)
  - [9.2 Hot arena vs cold edge](#92-hot-arena-vs-cold-edge)
  - [9.3 Working contract (library)](#93-working-contract-library)
  - [9.4 Operational mapping](#94-operational-mapping)
  - [9.5 Proposed (not yet built)](#95-proposed-not-yet-built)
  - [9.6 Principles](#96-principles)
- [§11 Conclusion: Inferences & Forward Path](#11-conclusion-inferences--forward-path)
  - [What this manifold demonstrates](#what-this-manifold-demonstrates)
  - [What surprised us](#what-surprised-us)
- [Related repositories](#related-repositories)
- [Authors](#authors)
- [Acknowledgments](#acknowledgments)
- [Contributing](#contributing)
- [Citation](#citation)
- [License](#license)

<details>
<summary><b>Heading anchor list</b> (URL fragments for deep links)</summary>

Each `##` / `###` heading on GitHub gets a stable **anchor**: the part after `#` in `README.md#anchor-name`. Use the same fragment from issues and PRs (`tytolabs/umst-manifold/blob/main/README.md#…`). The list below is for copy-paste only.

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
#31-mathematical-foundations--formal-grounding
#32-autonomous-control--embodied-ai
#33-structural-dynamics--topology-optimization
#34-constitutive-materials-chemistry
#4-exhaustive-architecture-topology
#repository-tree
#5-surfaces--entrypoints
#6-advanced-continuous-solver-specifications
#61-multi-species-ionic-electrochemistry-pbnp
#62-electromagnetic--radiative-transport-fdfd
#63-coupled-phase-field-fracture
#64-anisotropic-acoustics--wave-dynamics
#65-non-newtonian-extrusion-rheology-herschel-bulkley
#66-coupled-jfnk-thmc-solver
#7-technical-deployment--agentic-instructions
#commands-cargo-features-and-agent-checklist
#build-test-ci-parity
#selected-cargo-features
#for-autonomous-agents
#8-formal-foundations-and-citation
#9-special-protocol-note-to-autonomous-ai-agents--systems
#91-shared-stack-gate-spine
#92-hot-arena-vs-cold-edge
#93-working-contract-library
#94-operational-mapping
#95-proposed-not-yet-built
#96-principles
#11-conclusion-inferences--forward-path
#what-this-manifold-demonstrates
#what-surprised-us
#related-repositories
#authors
#acknowledgments
#contributing
#citation
#license
#quick-verify-commands-we-ran
```
</details>

</details>

---

---

## 1. The Core Approach

UMST Manifold maps physical equations directly onto networks of nodes via **Discrete Exterior Calculus (DEC)**. Mass, momentum, and energy balance hold algebraically — by the graph's structure, not by numerical convergence.

### 1.1 The Mathematical Topology of Conservation
Think of mapping physics onto a network of connected nodes where energy and forces travel along closed mathematical loops (called **cochain complexes**). Mass and energy conservation are not estimated; they are guaranteed by the geometric structure of the network itself:

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\partial_p \circ \partial_{p+1} = 0 \quad \Longleftrightarrow \quad d^{p+1} \cir…" src=" style="max-width:100%;height:auto"></picture></p>

Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d^p" src=" style="vertical-align:middle"></picture> is the exterior derivative mapping <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="p" src=" style="vertical-align:middle"></picture>-cochains to <picture><source media="(prefers-color-scheme: dark)" srcset=")"><img alt="(p+1)" src=")" style="vertical-align:middle"></picture>-cochains. Because the boundary of a boundary is always empty (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\partial \circ \partial = 0" src=" style="vertical-align:middle"></picture>), the physical flux across any closed loop is guaranteed to be zero.

### 1.2 The Thermodynamic Gate
Before an AI agent or design system can propose a new shape or material mix, our built-in physical checkpoint—the **Thermodynamic Control Barrier Function (CBF)**—calculates the exact energy required to make that change. According to physics, erasing or changing information always costs a tiny, unavoidable amount of heat (known as **Landauer's erasure limit**):

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\Delta E \geq k_B T \ln 2" src=" style="max-width:100%;height:auto"></picture></p>

Simultaneously, the state updates are evaluated against the local **Clausius-Duhem inequality** to enforce non-negative entropy generation:

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\theta \gamma = \theta \dot{s} - \dot{u} + \frac{1}{\rho}\boldsymbol{\sigma}:\ma…" src=" style="max-width:100%;height:auto"></picture></p>

Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\theta" src=" style="vertical-align:middle"></picture> is temperature, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="s" src=" style="vertical-align:middle"></picture> is entropy, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="u" src=" style="vertical-align:middle"></picture> is internal energy, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\boldsymbol{\sigma}" src=" style="vertical-align:middle"></picture> is the stress tensor, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{d}" src=" style="vertical-align:middle"></picture> is the strain rate tensor, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{q}" src=" style="vertical-align:middle"></picture> is the heat flux vector. If the proposed change violates this gate, the runtime rejects the transition before it commits to state. 

### 1.3 The Policy Gateway: Mutual Information & Thermodynamic PPO Rewards

To let design algorithms (reinforcement-learning agents) optimize shapes without copying full state grids per step, the system exposes a narrow boundary called the **`ManifoldGateway`** (`src/ai/ppo.rs`). Heavy spatial math stays on the compute device; the gateway extracts only two scalar physical signals per step — internal friction (dissipation) and physical information gained (mutual information bits). The win here is data-movement parsimony, not wall-clock real-time.

*   **Mutual Information (MI) Observations:** The active learning loop monitors structural state transitions through the mutual information gained (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\Delta I" src=" style="vertical-align:middle"></picture>) during physical integration steps.
*   **The Landauer Erasure Gating:** As the observer gains information bits, the environment pays a strict physical cost for information erasure (<picture><source media="(prefers-color-scheme: dark)" srcset=")%20\cdot%20\Delta%20I"><img alt="k_B T \ln(2) \cdot \Delta I" src=")%20\cdot%20\Delta%20I" style="vertical-align:middle"></picture>). If the structural dissipation (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d_{\text{int}}" src=" style="vertical-align:middle"></picture>) cannot cover this physical cost, the Thermodynamic CBF rejects the state transition, preventing unphysical path generation.
*   **Thermodynamically Gated Rewards:** The verified state is assigned a scalar reward computed on-device using a balanced physical-chemical objective:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="R = \alpha \cdot \text{Free Energy} - \beta \cdot \text{Dissipation} - \gamma \c…" src=" style="max-width:100%;height:auto"></picture></p>
    
*   **Axiomatic Reward Tuning:** The gateway exposes two explicit, dimensionless scaling factors to align agent policies with structural priorities:
    *   **Safety Margin Scaling (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\zeta" src=" style="vertical-align:middle"></picture>):** Adds the mean spatial structural safety margin per batch, directing the policy toward high structural failure reserves.
    *   **Information Density Scaling (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\eta" src=" style="vertical-align:middle"></picture>):** Encourages the policy to maximize localized mutual information density, causing the optimizer to automatically focus material density along active stress and load transmission paths.

We use exact adjoint gradients—running the simulation backwards through time—to trace the precise cause of a structural weakness and correct it.

### 1.4 Grounding contract: constants, proofs, and second-law composition

**Second law as the compositional spine.** Discrete steps do not “mostly” respect physics: they are **admissible or rejected**. The local **Clausius–Duhem** inequality (§1.2) enforces **non-negative entropy production** together with stress, heat flux, and internal variables; the **thermodynamic CBF** and **Landauer** bookkeeping cap what an observer or policy may erase without paying dissipation. **Composition** is explicit: **DEC** gives **d ∘ d = 0** on fluxes so conservation is algebraic under mesh refinement; continuous solvers and cartridge closures are composed as **typed steps** in the orchestration fold; each proposed transition must satisfy the **same** second-law-shaped gate (or it never becomes state). Scaling to larger models or longer horizons does not relax that contract—it repeats it at every commit point.

**Constants are derived, measured, or grounded in truth — not silent knobs.** Every coefficient must trace to at least one obligation: **derived** from closed-form constitutive relations, limits, or dimensional analysis tied to the second-law spine (§1.2); **measured** from experiment, benchmark, or site calibration with recorded conditions (what was measured, on which material, under which schema version); or **grounded** as a documented calibration input with literature, dataset, or formal trail in [`docs/Solver-Status.md`](docs/Solver-Status.md) and companion docs, and often **pinned with explicit regression tolerances** in CI so drift is visible. Nothing is “just a float”: if it moves, a derivation, measurement record, or human or formal obligation must say why.

**“Proven” means traceable invariants, not vibes.** Conservation structure is **mathematical** (cochain topology). Solver-specific claims are tied to **Lean 4 / Coq anchors** in [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md) where the formal cement-gate fiber applies, and to **regression tests** and `scripts/check_solver_status.py` so documentation, `#[cfg(feature)]` lanes, and proof tables stay aligned. Where a proof is still staged, the code path is labelled honestly in Solver-Status — we do not conflate “compiled” with “discharged in Lean.”

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

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="1. INPUT & BOUNDARY (IScienceCartridge)" src=" style="max-width:100%;height:auto"></picture></p>

---

## 3. Cross-Domain Integration Specifications

This Manifold is a pure library — a mathematical substrate, agnostic to whichever material you map onto it. Open a domain below for focus, composition, outcome, and an honest limit on what this crate alone can claim:

<a id="31-mathematical-foundations--formal-grounding"></a>
<details>
<summary><b>1. Mathematical Foundations & Formal Grounding</b> (Mathematicians, Theoretical Physicists)</summary>

*   **Domain Focus:** Mathematical invariants, topological conservation laws, and formal physical proofs.

*   **Solver Composition:** Exposes Discrete Exterior Calculus (DEC) primitives to construct exact cochain complexes over sparse combinatorial graphs.

*   **Computational Outcome:** A spatial substrate where mass, momentum, and energy conservation are guaranteed algebraically by the graph topology rather than bounded by numerical float approximations. Rust modules map directly to formal Lean/Coq proof references in the pinned catalog.

*   **Honest limit:** Formal proofs **anchor** invariants — they do not block day-to-day kernel work; catalog witnesses link out, they are not a substitute for `cargo test` on changed solvers.
</details>

<a id="32-autonomous-control--embodied-ai"></a>
<details>
<summary><b>2. Autonomous Control & Embodied AI</b> (Robotics Engineers, Physical AI Architects)</summary>

*   **Domain Focus:** Gated agent execution, physical safety limits, and path-planning validation against thermodynamic constraints.

*   **Solver Composition:** Hooks directly into the Thermodynamic Control Barrier Function (CBF) and local entropy-generation metrics to filter agent action trajectories.

*   **Computational Outcome:** Agents and robotic controllers evaluate spatial path feasibility (e.g., 3D-printing trajectories) against thermodynamic stability limits and receive exact gradient steps to correct path drift. The per-step latency tracks the solver kernel selected — sub-second on small grids; minutes on full shell topology runs (see [`docs/Solver-Status.md`](docs/Solver-Status.md)).

*   **Honest limit:** CBF semantics change **runtime** behavior in integrated stacks — this library does not ship certified robot safety products; MCP hot path lives in concrete.
</details>

<a id="33-structural-dynamics--topology-optimization"></a>
<details>
<summary><b>3. Structural Dynamics & Topology Optimization</b> (Civil & Structural Engineers, Architects)</summary>

*   **Domain Focus:** Load-bearing efficiency, material minimization, and structural optimization under static/dynamic loads.

*   **Solver Composition:** Employs Neural-SIMP topology solvers paired with exact Adjoint ODE gradients to trace structural sensitivities backward through the spatial domain.

*   **Computational Outcome:** Rapid derivation of optimal structural load paths. While the forward PDE solvers scale with the spatial mesh discretization (<picture><source media="(prefers-color-scheme: dark)" srcset=")"><img alt="O(N)" src=")" style="vertical-align:middle"></picture>), the Adjoint Neural ODE backpropagation bypasses dense BPTT activation caching—yielding a constant <picture><source media="(prefers-color-scheme: dark)" srcset=")"><img alt="O(1)" src=")" style="vertical-align:middle"></picture> memory footprint over integration time steps, rendering complex dynamic topology optimization highly feasible on standard CPU hardware.

*   **Honest limit:** Full shell topology runs are **batch** workloads (minutes–hours) — adjoint memory claims do not imply real-time collapse analysis on production meshes without profiling.
</details>

<a id="34-constitutive-materials-chemistry"></a>
<details>
<summary><b>4. Constitutive Materials Chemistry</b> (Materials Scientists, Bio-chemical Researchers)</summary>

*   **Domain Focus:** Custom multi-physics coupling, chemical kinetics, and localized state evolution.

*   **Solver Composition:** Inherits the `IScienceCartridge` interface to define localized constitutive relations mapped directly onto the **64-lane UMST carrier** (unified material state tensor; width is versioned — see [§2](#2-unified-material-state-pipeline-umst-carrier)).

*   **Computational Outcome:** Synchronous, coupled solver execution where thermal, chemical, and mechanical variables react concurrently within single tensor operations, automatically inheriting the manifold's spatial gradients.

*   **Honest limit:** Constitutive chemistry lives in **`IScienceCartridge`** implementations (e.g. concrete) — the manifold supplies the carrier and DEC substrate, not mix recipes.
</details>

---

## 4. Exhaustive Architecture Topology

The repository is organized functionally — each file maps to a specific role in the solver, gate, or verification pipeline.

<a id="repository-tree"></a>
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
| **Python / MCP / End-user CLI** | Notebooks, robotic agent tools, industrial dataset calibration | Not shipped here — use **``umst-concrete-cartridge``**. | That workspace pins the same **Rust 1.88** line for CI alignment. |

---

## 6. Advanced Continuous Solver Specifications

To bridge the gap between microscopic physics and macroscopic design, the manifold embeds a suite of high-fidelity, native tensor solvers (`src/physics/solvers/`). These run directly on Burn's differentiable GPU/CPU graphs.

| Continuous Solver | Governing Physical Equations | Active Crate Module | Spatial / Design Output | Formal Verification Anchor |
| :--- | :--- | :--- | :--- | :--- |
| **1. Ionic Electrochemistry** | Poisson-Boltzmann-Nernst-Planck (PBNP) | `solvers/electrochemistry.rs` | Local multi-species ionic concentration fields (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="C_i" src=" style="vertical-align:middle"></picture>), dynamic boundary potential (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\Phi" src=" style="vertical-align:middle"></picture>). | Lean 4 Theorem `PBNP_Conserves` |
| **2. Photonics / EM Waves** | Frequency-Domain Maxwell Curl (FDFD) | `solvers/photonics.rs` | Steady-state electric field distribution (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="E" src=" style="vertical-align:middle"></picture>), localized scattering coefficients (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="S_{ij}" src=" style="vertical-align:middle"></picture>). | Coq Lemma `Maxwell_Curl_Nil` |
| **3. Phase-Field Fracture** | Coupled Elastic Strain Energy & Damage Phase | `solvers/fracture_field.rs` | Continuous damage field (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d" src=" style="vertical-align:middle"></picture>), dynamic crack propagation trajectories, localized strain energy release rates. | Lean 4 Theorem `Fracture_Energy_Bounded` |
| **4. Acoustics & Vibration** | Anisotropic Elastic Wave (Vlasov-Cauchy) | `solvers/acoustics.rs` | Dynamic spatial sound pressure displacement (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{u}" src=" style="vertical-align:middle"></picture>), boundary reflections, absorption spectra. | Coq Lemma `Wave_Conservation_Invariant` |
| **5. Non-Newtonian Flow** | Herschel-Bulkley Viscoplastic Fluid Yield | `solvers/rheology_flow.rs` | Yield stress front velocity vectors (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{u}" src=" style="vertical-align:middle"></picture>), localized thixotropic structural viscosity (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\eta" src=" style="vertical-align:middle"></picture>). | Lean 4 Theorem `Bingham_Flow_Stable` |
| **6. Coupled THMC Residual** | Jacobian-Free Newton-Krylov Matrix-Free GMRES | `solvers/thmc.rs` & `solvers/thmc_residual.rs` | Interlinked heat (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\theta" src=" style="vertical-align:middle"></picture>), moisture saturation (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="S_w" src=" style="vertical-align:middle"></picture>), mechanical strain (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\varepsilon" src=" style="vertical-align:middle"></picture>), and chemical hydration (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\alpha" src=" style="vertical-align:middle"></picture>). | Coq Lemma `JFNK_THMC_Residual_Bounded` |

<a id="61-multi-species-ionic-electrochemistry-pbnp"></a>
<details>
<summary><b>1. Multi-Species Ionic Electrochemistry</b> (Nernst-Planck-Poisson)</summary>

*   **Physical Concept:** Durability in porous structures depends on how ions (like dissolved chloride salts) move through water-filled pores. The solver calculates this movement by tracking chemical concentration gradients, fluid velocities, and microscopic electric fields.
*   **Exact Tensor Formulation:** Solves the coupled Poisson-Boltzmann-Nernst-Planck (PBNP) system:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")%20-%20\mathbf{u}%20\cdot%20\nabla%20C_i"><img alt="\frac{\partial C_i}{\partial t} = \nabla \cdot \left( D_i \nabla C_i + \frac{z_i…" src=")%20-%20\mathbf{u}%20\cdot%20\nabla%20C_i" style="max-width:100%;height:auto"></picture></p>
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\epsilon \nabla^2 \Phi = - \sum z_i F C_i" src=" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="C_i" src=" style="vertical-align:middle"></picture> is ion concentration, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="D_i" src=" style="vertical-align:middle"></picture> is diffusivity, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="z_i" src=" style="vertical-align:middle"></picture> is valence, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\Phi" src=" style="vertical-align:middle"></picture> is the electrostatic potential, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{u}" src=" style="vertical-align:middle"></picture> is pore fluid velocity.
</details>

<a id="62-electromagnetic--radiative-transport-fdfd"></a>
<details>
<summary><b>2. Electromagnetic & Radiative Transport</b> (Photonics FDFD)</summary>

*   **Physical Concept:** Active thermal management requires tracking how light, radiation, and heat propagate through heterogeneous material grains. The solver calculates this by simulating how high-frequency electromagnetic waves scatter, absorb, or reflect inside the microstructure.
*   **Exact Tensor Formulation:** Implements a Finite-Difference Frequency-Domain (FDFD) formulation of Maxwell’s curl equations:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")%20-%20k_0^2%20\epsilon_r%20\mathbf{E}%20=%20-%20i%20\omega%20\mu_0%20\mathbf{J}"><img alt="\nabla \times \left( \mu_r^{-1} \nabla \times \mathbf{E} \right) - k_0^2 \epsilo…" src=")%20-%20k_0^2%20\epsilon_r%20\mathbf{E}%20=%20-%20i%20\omega%20\mu_0%20\mathbf{J}" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{E}" src=" style="vertical-align:middle"></picture> is the electric field tensor, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\epsilon_r" src=" style="vertical-align:middle"></picture> is complex relative permittivity, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="k_0" src=" style="vertical-align:middle"></picture> is the free-space wavenumber.
</details>

<a id="63-coupled-phase-field-fracture"></a>
<details>
<summary><b>3. Coupled Phase-Field Fracture</b> (Cracking Dynamics)</summary>

*   **Physical Concept:** Cracks do not just appear; they grow by minimizing the structural energy. The solver tracks cracking by introducing a continuous damage field (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d \in [0,1]" src=" style="vertical-align:middle"></picture>) where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d=0" src=" style="vertical-align:middle"></picture> is solid material and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d=1" src=" style="vertical-align:middle"></picture> is a fully broken crack, avoiding the need to track complex individual crack edges.
*   **Exact Tensor Formulation:** Solves the coupled mechanical displacement and crack phase-field equations:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")^2%20+%20\kappa%20\right]%20\nabla%20\cdot%20\boldsymbol{\sigma}_0%20=%20\mathbf{0}"><img alt="\left[ (1-d)^2 + \kappa \right] \nabla \cdot \boldsymbol{\sigma}_0 = \mathbf{0}" src=")^2%20+%20\kappa%20\right]%20\nabla%20\cdot%20\boldsymbol{\sigma}_0%20=%20\mathbf{0}" style="max-width:100%;height:auto"></picture></p>
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")%20=%202(1-d)\mathcal{H}(\boldsymbol{\epsilon})"><img alt="G_c \left( -l \nabla^2 d + \frac{d}{l} \right) = 2(1-d)\mathcal{H}(\boldsymbol{\…" src=")%20=%202(1-d)\mathcal{H}(\boldsymbol{\epsilon})" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="G_c" src=" style="vertical-align:middle"></picture> is critical energy release rate, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="l" src=" style="vertical-align:middle"></picture> is the length scale of crack width, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathcal{H}" src=" style="vertical-align:middle"></picture> is the history variable of tensile strain energy density.
</details>

<a id="64-anisotropic-acoustics--wave-dynamics"></a>
<details>
<summary><b>4. Anisotropic Acoustics & Wave Dynamics</b> (Sound Propagation)</summary>

*   **Physical Concept:** Mechanical noise, vibrations, and shock waves travel differently depending on the grain orientation of a structure. The solver simulates how acoustic waves travel and dissolve within anisotropic media.
*   **Exact Tensor Formulation:** Solves the dynamic elastic wave equation:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")"><img alt="\rho \frac{\partial^2 \mathbf{u}}{\partial t^2} = \nabla \cdot \left( \mathbf{C}…" src=")" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{u}" src=" style="vertical-align:middle"></picture> is displacement, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\rho" src=" style="vertical-align:middle"></picture> is local density, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\mathbf{C}" src=" style="vertical-align:middle"></picture> is the 4th-order anisotropic stiffness tensor.
</details>

<a id="65-non-newtonian-extrusion-rheology-herschel-bulkley"></a>
<details>
<summary><b>5. Non-Newtonian Extrusion Rheology</b> (Herschel-Bulkley Flows)</summary>

*   **Physical Concept:** During fabrication processes like 3D printing, the wet material must flow through a nozzle but stay rigid once deposited. The solver tracks this transition by modeling the material as a fluid that only flows when pushed beyond a specific "yield stress."
*   **Exact Tensor Formulation:** Solves Herschel-Bulkley fluid dynamics where effective viscosity <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\eta_{\text{eff}}" src=" style="vertical-align:middle"></picture> scales with shear rate <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\dot{\gamma}" src=" style="vertical-align:middle"></picture>:
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\tau = \tau_y + K \dot{\gamma}^n \quad \Longrightarrow \quad \eta_{\text{eff}} =…" src=" style="max-width:100%;height:auto"></picture></p>
    
    Where <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="\tau_y" src=" style="vertical-align:middle"></picture> is yield stress, <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="K" src=" style="vertical-align:middle"></picture> is consistency index, and <picture><source media="(prefers-color-scheme: dark)" srcset=" alt="n" src=" style="vertical-align:middle"></picture> is the flow behavior index.
</details>

<a id="66-coupled-jfnk-thmc-solver"></a>
<details>
<summary><b>6. Coupled Jacobian-Free Newton-Krylov (JFNK) THMC Solver</b> (Multi-Physics Convergence)</summary>

*   **Physical Concept:** Temperature, water pressure, mechanical load, and chemical hydration react to each other simultaneously. Instead of calculating them one by one (which leads to errors), the solver groups them into a single continuous equation and balances them together in an iterative loop.
*   **Exact Tensor Formulation:** Implements a fully coupled residual function <picture><source media="(prefers-color-scheme: dark)" srcset=")%20=%20\mathbf{0}"><img alt="\mathbf{F}(\mathbf{x}) = \mathbf{0}" src=")%20=%20\mathbf{0}" style="vertical-align:middle"></picture> solved via a Jacobian-Free Newton-Krylov solver (`thmc_residual.rs` / `krylov_host.rs`):
    
    <p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset=")%20-%20\mathbf{F}(\mathbf{x})}{\epsilon}"><img alt="\mathbf{J} \mathbf{v} \approx \frac{\mathbf{F}(\mathbf{x} + \epsilon \mathbf{v})…" src=")%20-%20\mathbf{F}(\mathbf{x})}{\epsilon}" style="max-width:100%;height:auto"></picture></p>
    
    Enabling matrix-free GMRES iterations to reach full coupled Thermo-Hydro-Mechanical-Chemical convergence without computing or storing large Jacobian matrices.
</details>

---

## 7. Technical Deployment & Agentic Instructions

If you are an application engineer, architect, or data scientist looking for Python bindings, MCP servers, or JSON/CSV contracts, the deployed engine lives in the `**UMST Concrete Cartridge**` repository.

If you are building atop the Manifold, here is the technical deployment reference:

<a id="commands-cargo-features-and-agent-checklist"></a>
<details>
<summary><b>Commands, Cargo features, and agent checklist</b></summary>

### Build, test, CI parity
```bash
cd umst-manifold
cargo build
cargo test
```

- **Solver integration tests:** `cargo test --features solver-tests` (same feature graph as `solver-experimental`).
- **Gate HTTP shim (`gate_server`):** `cargo test -p umst-manifold --features gate-server-bin --test gate_server_http` (also builds the `--bin gate_server` target via the same flag; **`gate-server`** forwards to **`gate-server-bin`**).
- **GPU (`wgpu`):** The optional `wgpu` feature is not part of the default CI matrix; **CPU builds with `ndarray`** are the portable reference path. On Apple Silicon, `mac-fast` (`ndarray` + `blas-accelerate`) is the supported high-throughput local configuration.

### Selected Cargo Features
We group solvers into explicit feature lanes to manage compile times and dependencies.
| Feature | Purpose |
|---------|---------|
| `ndarray` (default) | CPU tensors via `burn-ndarray`. |
| `blas-accelerate` | vecLib/Accelerate-backed matmul on macOS (forwarded to `burn-ndarray`). |
| `mac-fast` | `ndarray` + `blas-accelerate` convenience bundle. |
| `solver-stable`, `solver-research`, `solver-experimental`, `solver-tests` | Solver lane umbrellas. |
| Granular solver flags | `fracture-at2`, `acoustics-newmark`, `thmc-coupled`, `electrochemistry-pnp`, `mechanics-adjoint` — single kernel pulls in `Cargo.toml`. |
| `gate-server-bin` (+ alias `gate-server`) | Enables `cargo run … --bin gate_server` (`POST /gate` JSON); Powers/Parrott mix gate lives in **`crate::gate::http_manifest`**. |
| `manifest-bridge` | Downstream hook for cartridges re-exporting `umst_manifold::manifest::*` (declare-only; verify on sibling cartridge). |
| `manifold-manifest` | Typed manifest façade (`UmstManifest`, catalog hash advisory). |
| `manifold-gate` | Transition gate trait surface for host/cartridge parity (pairs with `manifest-bridge` on concrete). |

### Stack verification (`verify_umst_stack.sh`)

From the crate root, gate parity and optional Lean export digest checks:

```bash
bash scripts/verify_umst_stack.sh
export UMST_REQUIRE_FORMAL_EXPORT=1   # fail if umst-formal-double-slit export missing
bash scripts/verify_umst_stack.sh
```

Monorepo layout: sibling `../umst-formal-double-slit` or `UMST_FORMAL_ROOT`. Full command matrix: [`docs/VERIFY.md`](docs/VERIFY.md). Workspace root shortcut: [`VERIFY.md`](../VERIFY.md) (multi-repo workspace).

### For Autonomous Agents
- **Repo root:** treat the checkout directory of this repository as the working root for all `cargo` / `python3` commands.
- **Safe, no-GPU commands:** `cargo build`, `cargo test`, `cargo test --features solver-stable`, `cargo run --example basic_topology`, `python3 scripts/check_solver_status.py --check-paths --check-memo-links --check-statmech-verification-set`.
- **Before editing:** scan [`docs/Solver-Status.md`](docs/Solver-Status.md) and run `check_solver_status.py` before changing solver feature tables or `#[cfg(feature = "...")]` blocks.

</details>

---

## 8. Formal foundations and citation

We maintain strict formal proof anchors (`formal_status`) mapping our Rust implementations to Lean/Coq theorems in the `umst-formal` repository.

- **Notation and foundations:** [`docs/Mathematical-Foundations.md`](docs/Mathematical-Foundations.md)
- **Solver lanes, verification paths:** [`docs/Solver-Status.md`](docs/Solver-Status.md)
- **Formal proof index:** [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md)
- **Developer verify matrix (CI parity):** [`docs/VERIFY.md`](docs/VERIFY.md)
- **Claims vs proofs ledger (Lean ↔ `catalog_id` ↔ Rust):** [`docs/claims-vs-proofs.md`](docs/claims-vs-proofs.md)
- **Formal integration status (module buckets, release witness gaps):** [`docs/FORMAL_INTEGRATION_STATUS.md`](docs/FORMAL_INTEGRATION_STATUS.md)
- **Catalog ↔ Rust coverage audit:** [`docs/CATALOG_COVERAGE_AUDIT.md`](docs/CATALOG_COVERAGE_AUDIT.md)
- **Compositional inference / gateway audit:** [`docs/ADAPTIVE_WITNESS_COVERAGE.md`](docs/ADAPTIVE_WITNESS_COVERAGE.md) (gateway witness priority; static inventory in [`docs/CATALOG_COVERAGE_AUDIT.md`](docs/CATALOG_COVERAGE_AUDIT.md))
- **Release witness ladder:** [`docs/RELEASE_WITNESS_LADDER.md`](docs/RELEASE_WITNESS_LADDER.md) — philosophy [§ Proof library · gate law · MI envelope · no Rust axioms](docs/RELEASE_WITNESS_LADDER.md#proof-library--gate-law--mi-envelope--no-rust-axioms)
- **Formal export scope (sibling):** [`../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md`](../umst-formal-double-slit/Docs/EXPORT_COVERAGE.md)
- **Two-repo formal alignment (sibling):** [`../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md`](../umst-formal-double-slit/Docs/UMST_FORMAL_REPOS_ALIGNMENT.md)
- **Supercap formal scaling (sibling):** [`../umst-supercap-cartridge/docs/FORMAL_SCALING.md`](../umst-supercap-cartridge/docs/FORMAL_SCALING.md)

### Lean catalog lock (SSOT — this repo owns the composed count)

**Authoritative file:** [`artifacts/catalog.lock.json`](artifacts/catalog.lock.json)  
**Do not hardcode module counts in sibling READMEs** — link this lock (concrete A1/F1).

**Live values (pasted 2026-07-12 @ `80a5cef`):**

| Field | Value |
|:---|:---|
| `module_count` | **129** |
| `upstream_catalog_digest_hex` | `17a6d8e17d9a4847231a255ffb1214db0319a7a2727ecd80708cb7f08045da1e` |
| Fiber pins | double-slit **62** · formal **73** · ucrs **9** |

**Sibling Lean declaration snapshots** (via `python3 scripts/check_theorem_counts_ssot.py` → [`scripts/theorem_counts_snapshot.json`](scripts/theorem_counts_snapshot.json); script wins on mismatch):

| Fiber | Lake roots | theorem + lemma (roots-only) |
|:---|:---|:---|
| `umst-formal` | 62 | 289 + 24 |
| `umst-formal-double-slit` | 52 | 486 + 30 |

The manifold pins the exported Lean inventory; digest is enforced at build via `build.rs` → `UMST_CATALOG_LOCK_SHA256_HEX`. Canonical export path: sibling `umst-formal-double-slit/artifacts/catalog.json` (`make lean-catalog-export`). Override at build: `UMST_CATALOG=/path/to/lock.json`. Historical dual-pin / rollback notes: [`docs/DUAL_PIN_ARCHITECTURE.md`](docs/DUAL_PIN_ARCHITECTURE.md) · [`docs/CATALOG_UPDATE_PROTOCOL.md`](docs/CATALOG_UPDATE_PROTOCOL.md) — **re-open the lock file for live numbers**; older prose citing 119/122 is superseded by the lock above.

Handoffs: [`docs/AGENT_STATUS.md`](docs/AGENT_STATUS.md).
---

## 9. Special Protocol: Note to Autonomous AI Agents & Systems

**This repo is the hot physics substrate**, not the MCP host. Full agent MCP contract: sibling ``umst-concrete-cartridge/docs/AGENT_MCP.md``. Local stub [`docs/AGENT_MCP.md`](docs/AGENT_MCP.md) is **not** the SSOT (drifted; reconcile tracked as Work-order A7/B9).

### 9.1 Shared stack (gate spine)

Same block as the front door: **matter** (this repo + concrete) · **knowing** (formal-double-slit) · **acting** (formal) · **time** (ucrs) — one thermodynamic admissibility gate. Sibling links in [Related repositories](#related-repositories).

### 9.2 Hot arena vs cold edge

| Path | Use | Where |
|:---|:---|:---|
| **Hot library** | In-process DEC / gate / solvers | this crate |
| **Warm arena** | Parse-once batch gate loops | [`umst-runtime-arena`](umst-runtime-arena/) |
| **Cold MCP** | Contribute / memory / explain over stdio | concrete `umst-mcp` only |

Never conflate library calls with MCP. Benchmarks: [`docs/benchmarks/arena_vs_mcp.md`](docs/benchmarks/arena_vs_mcp.md). Topology: [`docs/RUNTIME_TOPOLOGY.md`](docs/RUNTIME_TOPOLOGY.md).

### 9.3 Working contract (library)

* **No statistical interpolation of physics.** Solver / gate answers come from kernels — not guessed.
* **Second law + composition are non-negotiable.** See [§1.4](#14-grounding-contract-constants-proofs-and-second-law-composition).
* **Use exact gradients** where exposed (`AdjointNeuralODE` in `src/ai/adjoint.rs`).
* **Validate against proof anchors.** Kernel changes: [`docs/PROOF-STATUS.md`](docs/PROOF-STATUS.md) via `check_solver_status.py`.
* **Gate semantics for agents** (when using sibling MCP): success = admissible PASS; REJECT = structured remediation — never silent failure. Enforcing details live in the concrete cartridge agent layer, not duplicated here.

### 9.4 Operational mapping

| Goal | Action |
|:---|:---|
| Local library check | `cargo test --workspace` / `cargo test --test ci_quality_profile` |
| Catalog / digest | read `artifacts/catalog.lock.json`; `verify_umst_stack.sh` |
| Agent MCP / contribute | sibling concrete `umst-mcp` (**stdio**) — not WebSocket streaming in this repo |
| Cartridge mount | implement `IScienceCartridge` ([`docs/CARTRIDGE_PORT.md`](docs/CARTRIDGE_PORT.md)) |

### 9.5 Proposed (not yet built)

Documented for agents under Proposed in the concrete MCP contract — **not** available as manifold tools:

* `umst_dry_run`, `umst_promote_contribution` (MCP), `umst_arena_session` fused tool
* WebSocket voxel streaming from this repo (MCP transport verified elsewhere is **stdio**)

### 9.6 Principles

* **Continuity of flow.** DEC cochain structure; `d ∘ d = 0` is algebraic.
* **Admissibility is runtime gate law**, not a marketing metaphor for rustc errors — illegal transitions fail witnesses / reject, they do not become “compile-time type errors” in this README’s sense.
* **Information cost.** Landauer / MI observers bound informational updates on the gated path.

## 10. Honesty and limits

**Honest is / isn't.** **Is:** in-repo DEC/gate/solvers, catalog lock SSOT, witness ladder automation, library APIs. **Isn't:** an end-user app, MCP server, or cement chemistry — those live in sibling cartridges. Do **not** blend “CI green”, “catalog modules wired on hot path”, and “org publish” into one completion %.

### Hot arena vs cold edge (performance honesty)

| Path | Where | Character |
|:---|:---|:---|
| **Hot / warm (this repo)** | Library kernels + [`umst-runtime-arena`](umst-runtime-arena/) | Pure tensor / native in-process; DEC, gate witnesses, solver steps, parse-once arena |
| **Cold edge (not here)** | Sibling ``umst-concrete-cartridge` `umst-mcp`` | Effectful stdio MCP: contribute / memory / explain |

Authoritative agent MCP surface = concrete `umst-mcp` — **not** this README’s crate tree. See [`docs/RUNTIME_TOPOLOGY.md`](docs/RUNTIME_TOPOLOGY.md) · [`docs/benchmarks/arena_vs_mcp.md`](docs/benchmarks/arena_vs_mcp.md).
### Honesty ledger (one status pointer)

Proven vs aspirational accounting lives in **[`docs/PENDING_GAPS_PLAIN.md`](docs/PENDING_GAPS_PLAIN.md)** (verified rollup: [`docs/RELEASE_WITNESS_PROGRESS_VERIFIED.md`](docs/RELEASE_WITNESS_PROGRESS_VERIFIED.md); redirect aliases [`docs/QUALITY_PROGRESS_VERIFIED.md`](docs/QUALITY_PROGRESS_VERIFIED.md), [`docs/QUALITY_WITNESS_LADDER.md`](docs/QUALITY_WITNESS_LADDER.md) → release witness docs). Checklist / roadmap redirects: [`docs/QUALITY_CHECKLIST.md`](docs/QUALITY_CHECKLIST.md), [`docs/PENDING_QUALITY_ROADMAP.md`](docs/PENDING_QUALITY_ROADMAP.md). Strengthen every disclaimer below; soften none.

### Proven scope and limits (honest version)

We don't pretend everything is proven. Conservation structure is mathematical, and the thermodynamic gate is enforced in code on every step — but only part of the Lean/Coq/Agda library is hand-wired onto the runtime gate path, **by design**: at inference time the robot runs fast Rust witnesses, not a theorem prover. There are three different things people mean by "done" here — in-repo automation, how much of the proof library is wired on the hot path, and organization-level publishing — and they should **never** be blended into one "completion %". The honest, current accounting of each lives in one place: **[`docs/PENDING_GAPS_PLAIN.md`](docs/PENDING_GAPS_PLAIN.md)**.
## 11. Conclusion: Inferences & Forward Path

### This repository demonstrates
- **Conservation by construction, not by tuning.** Mapping physics onto a discrete exterior calculus complex makes the boundary-of-a-boundary identity (<picture><source media="(prefers-color-scheme: dark)" srcset=" alt="d \circ d = 0" src=" style="vertical-align:middle"></picture>) a structural property of the data, not a convergence target. Drift that traditional FEM accumulates over long simulations is algebraically absent here.
- **A single 64-lane UMST carrier is enough.** Thermal, mechanical, chemical, and informational variables co-resolve in one tensor pass instead of brittle staggered couplings. The downstream gain is gradient continuity end-to-end, which is what makes the adjoint loop tractable on commodity CPUs.
- **Safety as a runtime gate, not a post-hoc audit.** The Clausius–Duhem inequality and Landauer cost are evaluated *before* a state transition commits. A policy that violates them does not produce a logged warning; it does not produce a state at all.
- **Formal anchoring closes the loop.** Each solver carries a Lean 4 / Coq theorem reference in `docs/PROOF-STATUS.md`, so a kernel change is invalid until the corresponding proof obligation is discharged in ``umst-formal``.

### Inferences from the work
- **Architects can author a physics substrate.** Discrete Exterior Calculus has a reputation as a graduate-numerical-analysis specialty. It isn't. Once you stop fighting tensor-index notation and start thinking in cochains, the manifold reads like a parametric modifier graph — the same mental model architects already use. Two architects wrote and trained the kernel.
- **Rust was the discipline we needed, not the speed.** Earlier prototypes in Python and JAX leaked gradients silently through monkey-patched operators; nothing alerted us until convergence quietly stopped meaning what we thought. Moving to Burn + Rust forced every kernel to declare its differentiability contract at the type level. Most of the reliability we ship is downstream of compiler-checked variance and DEC admissibility, not algorithmic novelty.
- **The hard part was orchestration, not the math.** 25 engines coexisting under `IScienceCartridge` only works because solver composition is a fold over a typed step graph, not a chain of side-effects. The largest single kernel diff of 2025 wasn't a new solver — it was rewriting orchestration.
- **The CBF earned its keep as semantics, not certification.** Adding the thermodynamic gate to the *runtime* — rather than only to a post-hoc proof — changed what the program does, which proved more valuable than what it can prove. Rejected transitions don't become logged warnings; they cease to exist as state.
- **Formal proofs anchor; they do not block.** Lean obligations live in ``umst-formal`` and document the kernel's invariants. Day-to-day kernel work doesn't wait on a Lean discharge — but the moment a kernel change breaks a proven invariant, the next CI run catches it. Anchor, not gate, turned out to be the productive pattern.

The manifold is a substrate. Its value shows up in what gets built on top of it.

---

### Related repositories

Shared gate spine — **matter** (this repo + concrete) · **knowing** · **acting** · **time**. Each sibling below is listed for how it composes **with this manifold**, not as a generic link dump. Private / out-of-scope cartridges are omitted.

| Repository | Spine role | Relation to this manifold |
|:---|:---|:---|
| ``umst-concrete-cartridge`` | **Matter** cartridge | Mounts `IScienceCartridge` on this DEC carrier and UMST lanes. Owns cementitious closures, CLI / Python / **stdio MCP**, and research-memory ingest. Hot physics stays here; cold agent tools live there (``AGENT_MCP.md``). |
| ``umst-formal-double-slit`` | **Knowing** | Machine-checked observation / Landauer / Englert fiber. This repo **consumes** catalog witnesses (R0) from the lock — it does not re-prove which-path cost mid-solve. |
| ``umst-formal`` | **Acting** | Economic / Kleisli admissibility predicates. Catalog anchors document kernel obligations; runtime gate rejection still happens in this crate (and concrete), not by `lake build`. |
| ``umst-ucrs`` | **Time** | Temporal witness / stamp spine. Stamps *when* a gate-admitted commit lands; does not validate constitutive law or replace DEC solvers here. |

---

## Release & agent path

> Release notes in [CHANGELOG.md](CHANGELOG.md). Material-agnostic cartridge port: [`docs/CARTRIDGE_PORT.md`](docs/CARTRIDGE_PORT.md). **Catalog SSOT:** [`artifacts/catalog.lock.json`](artifacts/catalog.lock.json) (**129** modules @ digest `17a6d8e1…` — re-open the file; do not trust this prose if they diverge). **v2.0.0 tags/releases withdrawn** pending sign-off (no Zenodo software DOI minted). **Stack verify:** see [Quick verify](#quick-verify-commands-we-ran) · [`docs/VERIFY_TRANSCRIPT.md`](docs/VERIFY_TRANSCRIPT.md).
### Fast Path for Agents

| Goal | Start here |
|------|------------|
| Batch sweeps, optimization, many gate checks | [`umst-runtime-arena`](umst-runtime-arena/) — [`load_arena`](umst-runtime-arena/src/load.rs), optional [`mmap`](umst-runtime-arena/src/mmap.rs); cartridge ``06_arena_batch.py`` |
| Prototyping, IDE agents, single-shot | Sibling ``umst-concrete-cartridge` Agent MCP` |
| Throughput vs MCP | [`docs/benchmarks/arena_vs_mcp.md`](docs/benchmarks/arena_vs_mcp.md) — CI enforces arena ≥**5×** stdio MCP |

Hot/warm/cold boundaries: [`docs/RUNTIME_TOPOLOGY.md`](docs/RUNTIME_TOPOLOGY.md).

### For Agents & Researchers

| Path | When to use |
|------|-------------|
| **Library (hot)** | Batch sweeps, training, CI physics — `cargo test`, cartridge `IScienceCartridge` in-process |
| **Arena (warm)** | Parse-once loops — [`umst-runtime-arena`](umst-runtime-arena/) `load_arena()` + optional `mmap` |
| **MCP (cold)** | IDE agents, discovery, single-shot gate/predict — sibling ``umst-concrete-cartridge` Agent MCP` |
| **Formal** | Catalog witnesses, digest pins — [`docs/FORMAL_INTEGRATION_STATUS.md`](docs/FORMAL_INTEGRATION_STATUS.md) |

Prefer **library/arena calls over Docker MCP** for performance-sensitive proposal loops. Hot/warm/cold boundaries: [`docs/RUNTIME_TOPOLOGY.md`](docs/RUNTIME_TOPOLOGY.md). **Gaps / pending (in-repo):** [`docs/PENDING_GAPS_PLAIN.md`](docs/PENDING_GAPS_PLAIN.md).

---

## Authors

**Santhosh Shyamsundar** —  · [santhoshshyamsundar@tyto.studio](mailto:santhoshshyamsundar@tyto.studio)

**Santosh Prabhu Shenbagamoorthy** —  · [santosh@tyto.studio](mailto:santosh@tyto.studio)

---

## Acknowledgments

Portions of this work were developed in collaboration with advanced large-language-model tools, across multiple model iterations.
Claude Opus and Sonnet (Anthropic) provided surgical precision during drafting and refinement.
Gemini (Google) offered exceptional large-context planning and file management.
Grok (xAI) and its collaborative reasoning team contributed core mathematical and scientific reasoning.
The Cursor code editor, Composer, Claude Code, and Antigravity supported seamless implementation and agentic file management.

The large-language models assisted with exploration, drafting, and code scaffolding — never with the validity of formal proofs or physics regression tests. `cargo test` and catalog witnesses are authoritative for kernel behavior; Lean obligations **anchor** invariants, they do not replace runtime gate checks.

We gratefully acknowledge the open-source ecosystems that make this work possible: **Rust** and **Burn**; DEC/gate kernels; **Lean**-catalog witnesses; and **Python** verification scripts.

---

## Contributing

Development processes and safety guidelines are maintained in [`CONTRIBUTING.md`](CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), and [`SECURITY.md`](SECURITY.md). Corrections welcome via PR. Prefer `cargo test` / stack verify when touching solvers or gate paths.

---

## Citation

Bibliographic metadata is maintained in [`CITATION.cff`](CITATION.cff).

---

## License

Released under the [MIT License](LICENSE). © 2026 .

<!-- AUTO-LATTICE:BEGIN -->
## Lattice position

**What it is:** `tytolabs/umst-manifold` — Future rename target: umst-runtime (A3).

**One-line role:** `runtime` on layer `runtime` (status `wip`, stability `evolving`, semver `0.1.0`).

**Composes into:** `self`

**Composed into by:** —(none declared)

**Honest tier:** structural/reorg standing only — not physics GREEN · not `production_wired` · INV4 flip unauthorized.

_Generated by `scripts/gen-lattice-readme.sh` from `umst.toml`. Do not hand-edit inside markers._
<!-- AUTO-LATTICE:END -->

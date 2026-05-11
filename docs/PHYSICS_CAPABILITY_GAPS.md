<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Physics capability gaps (structural / materials modelling)

This page summarizes **user-facing structural and materials scope** that sits **outside** the solver lanes and verification contracts in [`Solver-Status.md`](Solver-Status.md). It is a pointer index, not a substitute for that table or the v0.4 brief.

1. **Transient \(M_\mu\) (time-evolving stiffness / viscoelastic branch in solid mechanics)**  
   **Current shipped capability:** Quasi-static **bar-network equilibrium** with per-step elastic parameters (Young’s modulus reduced to edges, optional damage coupling on other paths) and THMC transport with **split** time stepping plus **research-partial** implicit Newton on the **thermal + hydration \(\alpha\)** block only (`ThmcImplicitTAlphaNewtonConfig`).  
   **Limitation:** There is **no** first-class **Maxwell / Kelvin–Voigt-style** evolution of a relaxation shear modulus \(M_\mu\) (or equivalent viscoelastic state) **inside** the mechanics equilibrium kernel coupled to the same clock as THMC; modulus updates follow the **instantaneous** elastic bar model, not a documented hereditary viscoelastic tangent on every substep.  
   **Why it matters:** Creep, relaxation, and hydration-ageing of stiffness need **history-dependent** tangents; treating elasticity as a snapshot underestimates delayed deformation and stress redistribution in concrete-type workflows.  
   **Pointers:** [`src/physics/mechanics.rs`](../src/physics/mechanics.rs) (module **Capability gaps** + `VectorMechanicsSolver::solve_equilibrium`), [`src/physics/solvers/thmc.rs`](../src/physics/solvers/thmc.rs), [`tests/verification/thmc_drying_shrinkage.rs`](../tests/verification/thmc_drying_shrinkage.rs), [`docs/research/v0.4_track13_monolithic_newton_thmc.md`](research/v0.4_track13_monolithic_newton_thmc.md).

2. **Geometric nonlinearity (large displacements / finite strain on the skeleton)**  
   **Current shipped capability:** **Small-strain** bar kinematics and graph-fixed edge directions for equilibrium and verified **linear** mechanics / adjoint checks on chains and Q1-hex plate bands.  
   **Limitation:** No **updated Lagrangian** or **finite-strain** redefinition of edge stretch and direction from the evolving displacement field in the shipped equilibrium path; geometric stiffness and follower loads are out of scope.  
   **Why it matters:** Slender structures, buckling, and large deflections change load paths; a geometrically linear kernel can mis-predict equilibrium and sensitivities used in topology and shell demos.  
   **Pointers:** [`src/physics/mechanics.rs`](../src/physics/mechanics.rs), [`tests/verification/mechanics_analytic.rs`](../tests/verification/mechanics_analytic.rs), [`tests/verification/adjoint_compliance_analytic.rs`](../tests/verification/adjoint_compliance_analytic.rs), **Solver lanes — Mechanics** notes in [`Solver-Status.md`](Solver-Status.md).

3. **Compressive plasticity (crush / cap-type solid irreversibility)**  
   **Current shipped capability:** **Linear elastic** bar response with optional **scalar damage** on fracture / THMC-adjacent paths (AT2-style relaxation and spectral **tensile** split in verified fracture harnesses — see Solver-Status fracture row).  
   **Limitation:** No **pressure-dependent compressive plasticity** (e.g. cap, crush, pore-collapse) as a distinct constitutive branch with hardening laws and verified return-map tests on the bar or continuum reduction.  
   **Why it matters:** Concrete and geomaterials often fail in **compression** through microcrushing and pore collapse; tensile damage and elastic bars do not bound that failure mode for structural rating.  
   **Pointers:** [`src/physics/solvers/fracture_field.rs`](../src/physics/solvers/fracture_field.rs), [`tests/verification/fracture_gamma_convergence.rs`](../tests/verification/fracture_gamma_convergence.rs), [`tests/verification/staggered_fracture_mechanics_chain.rs`](../tests/verification/staggered_fracture_mechanics_chain.rs), [`docs/research/v0.4_track12_staggered_fracture_mechanics.md`](research/v0.4_track12_staggered_fracture_mechanics.md).

4. **Contact (gap closure, friction, impact on the 1-skeleton)**  
   **Current shipped capability:** **Fixed** graph incidence (`edges_b1`); mechanics and transport operate on a **prescribed** connectivity with Dirichlet-style masks — no dynamic **contact set** discovery.  
   **Limitation:** No **Signorini / gap–friction** formulation, no active-set updates on `edges_b1`/`faces_b2` for closing gaps or sliding interfaces, and no CI tests for contact kinematics.  
   **Why it matters:** Structural assemblies, bedded shells, and post-fracture rub rely on **changing** interface constraints; without contact, multi-body realism and load transfer across interfaces remain manual graph edits only.  
   **Pointers:** [`docs/Mathematical-Foundations.md`](Mathematical-Foundations.md) (§1 — topology change *including contact* as intended sparse combinatorics, not yet a solver), [`src/physics/mechanics.rs`](../src/physics/mechanics.rs) (boundary projection only), [`Solver-Status.md`](Solver-Status.md) **Topology / shell** / rib-pattern context for **fixed** topology demos.

## 5. Kirchhoff SSSS plate gate (R2.1 vs current CI) {#r21-plate-gaps-index}

**Checklist (strict gate vs `plate_centre_deflection_kirchhoff_ratio_q1_hex_locked_band`):** [`mechanics_geometric_nonlinearity_scope.md` §5](research/mechanics_geometric_nonlinearity_scope.md#r21-plate-checklist). **Solver contract:** [`Solver-Status.md`](Solver-Status.md) mechanics row. **Completion brief:** [§R2.1](../../composer_prompts/v0.4_phase_3_followup_for_composer.md#r21--q1-hex-sri-for-kirchhoff-plate-5-gate).

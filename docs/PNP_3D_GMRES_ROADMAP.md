# PNP 3D: Jacobi-PCG vs matrix-free GMRES — roadmap

**Scope:** Engineering estimate and test plan for extending Poisson–Nernst–Planck (PNP) toward **3D** discrete problems with a **Krylov outer/inner** story, while keeping **Scharfetter–Gummel (SG)** flux consistent with the existing **`edges_b1`** 1-skeleton contract.

**Execution tracking:** Aligns with Cursor exec todo **`exec-solver-pnp-3d-gmres`** and executive section **§1** in [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md). Do not mark that todo complete on documentation-only work.

**Related code (baseline):**

- PNP scaffold, Poisson split (chain **Thomas** vs **Jacobi-preconditioned CG** on the graph Laplacian), SG NP, and implicit chain Newton: [`src/physics/solvers/electrochemistry.rs`](../src/physics/solvers/electrochemistry.rs).
- Host **`f32` GMRES** (matrix-free `matvec`, no restart in current implementation): [`src/physics/solvers/thmc_jfnk.rs`](../src/physics/solvers/thmc_jfnk.rs) — **`thmc_jfnk`** documents the preferred closure shape (`gmres_f32_try` / `gmres_f32`) and fallible matvec semantics for production JFNK slices.

---

## 1. Problem statement

Today’s electrochemistry lane is **DEC 1-skeleton / graph-first**: topology enters as **`edges_b1: [2, E]`** (source row, target row). SG flux is assembled on those edges and passed through **`primal_divergence_from_edge_flux_topo`** (see module rustdoc in `electrochemistry.rs`). Poisson on **non-chain** graphs uses **`poisson_graph_uniform_laplacian_jacobi_pcg`** (Jacobi-preconditioned CG on the scalar graph Laplacian). Implicit backward–Euler **Newton** on **MVP path chains** uses band/dense host linear algebra, not GMRES.

**3D target (per exec brief):** a coupled discrete PNP system in **3D** (volumetric mesh or equivalent DEC complex) where a **matrix-free GMRES** (or restarted GMRES / FGMRES if scope expands) can replace or complement explicit Jacobian factorizations for large **`N`**, while **SG flux remains conservative on the oriented edge list** carried by **`edges_b1`** (or its 3D successor with the same logical contract).

---

## 2. Jacobi-PCG vs matrix-free GMRES hook-up

| Aspect | Jacobi-PCG (current graph Poisson) | Matrix-free GMRES (target pattern) |
|--------|-----------------------------------|-----------------------------------|
| Operator | Symmetric **graph Laplacian** (SPD structure in typical ε-weighting) | General **non-symmetric** or indefinite coupled **PNP Jacobian** / Schur complements |
| Application | `TopologicalLaplacian` matvec via tensor ops + Jacobi diagonal | `FnMut(&[f32]) -> Result<Vec<f32>, String>` matvec as in **`thmc_jfnk`** |
| Preconditioning | Jacobi (cheap, parallel-friendly) | **Open:** ILU / multigrid / block-Jacobi / physics-based — must be estimated per mesh class |
| Cost model | Iterations × cheap matvec + reductions | Arnoldi + matvec; memory for Krylov basis if **no restart** (current `gmres_f32_try` has **no restart** — likely insufficient at large **`n`** without extension) |

**Engineering decisions (to lock before coding spike):**

1. **Which linear system?** Options: (a) monolithic Newton step on full **`(φ, c⁺, c⁻)`** unknowns; (b) **Schur** or **block** solves with GMRES on the reduced operator; (c) **Picard** outer with GMRES only on Poisson or only on NP blocks. Choice drives matvec implementation and preconditioner.
2. **Precision:** Host Newton in PNP today uses **`f64`** in places; **`thmc_jfnk`** is **`f32`**. Document whether PNP GMRES stays **`f64`** for stability or adapts the **`f32`** helper with explicit error budgets.
3. **Restarted GMRES:** If **`n`** or condition number grows, extend **`thmc_jfnk`** with restart + optional flexible GMRES rather than growing full Krylov basis — budget as a separate slice from “first matvec hook-up”.
4. **Burn / categorical constraints:** Per [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md) and [`FP_CATEGORICAL_BURN.md`](FP_CATEGORICAL_BURN.md), host bridges and Krylov loops remain **classified**; avoid claiming device-fused parity until designed.

---

## 3. SG flux on **`edges_b1`**

**Invariant to preserve in 3D staging:**

- **Orientation:** SG edge flux must match **`edges_b1`** endpoint ordering when mapping **`Δφ`** and concentrations **`c_a`, `c_b`** to the Bernoulli stabilisation (see rustdoc: \(J_e \propto (D/h)[c_a B(\cdot) - c_b B(\cdot)]\)).
- **Divergence:** Mass update uses **`primal_divergence_from_edge_flux_topo`** — any 3D generalisation must keep **discrete conservation** testable (fluxes sum to divergence on nodes).
- **Mesh spacing:** Uniform **`mesh_spacing` `h`** scales **`J ∝ D/h`**; non-uniform **`h_e` per edge** is already noted as deferred in-module — 3D work should either **carry per-edge `h_e`** in the same tensor layout or document a single **`h`** equivalence class for acceptance tests.

**Risk:** Branching “2D surface edges vs 3D volumetric adjacency” without a single **`edges_b1`** convention will break parity tests; add a **topology fixture** early.

---

## 4. Citation: **`thmc_jfnk`**

Use **`crate::physics::solvers::thmc_jfnk`** as the **authoritative in-tree** GMRES implementation for:

- **Closure-based matvec** (`gmres_f32_try`) with **fallible** assembly (aligns with JFNK residual evaluation patterns).
- **Unit tests** bundled in the same module (verify behaviour before PNP-specific wiring).

PNP GMRES should **reuse or generalise** this module rather than duplicating Arnoldi logic, unless **`f64`** or restart requirements force a thin wrapper crate/module.

---

## 5. Engineering estimate (order-of-magnitude)

Assumes one senior Rust/numerics engineer familiar with `umst-manifold`, feature flags (`electrochemistry-mvp`, `solver-experimental`), and existing PNP tests.

| Phase | Work | Estimate |
|-------|------|----------|
| A | Spec: coupled unknown ordering, `h`/`edges_b1` for 3D fixture, choice of GMRES system (monolithic vs block) | **3–5 days** |
| B | Matvec: matrix-free application of PNP Jacobian or Schur operator; host **`f64`/`f32`** decision + wrappers | **1–2 weeks** |
| C | Preconditioner MVP (e.g. block-Jacobi or Poisson-only approximate inverse) | **1–2 weeks** |
| D | Restarted/flexible GMRES if required (extend **`thmc_jfnk`**) | **3–7 days** |
| E | CI integration, docs, Solver-Status row updates | **2–4 days** |

**Total (sequential):** roughly **4–7 engineer-weeks** to a defensible MVP with tests; **8+ weeks** if production-grade preconditioning and 3D mesh classes are broad.

**Dependencies / gates:**

- [`FP_GAP_BACKLOG.md`](FP_GAP_BACKLOG.md) / **`fp-gap-fp001`**: band–LU vs dense-expand narrative for full-SG — **orthogonal** to GMRES but same electrochemistry lane; coordinate so docs and matrix rows stay honest.
- Gradient escape scripts / allowlist must remain green ([`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md) constraints).

---

## 6. Test plan

1. **Unit: GMRES driver** — Existing **`thmc_jfnk`** tests remain mandatory green; add PNP-specific **small **`n`**** cases with **known Jacobian** (diagonal or 2×2 block) to verify matvec + GMRES solution vs dense solve.
2. **Parity: 1D chain** — Where the implicit Newton dense path exists, add **GMRES correction** parity on **fixed Jacobian** (or one Newton step) for **`N ∈ {17, 33}`**: same **`δ`** within tolerance as dense-expand (or document scaling why not).
3. **SG + topology** — Extend or mirror **`pnp_debye_layer`**-style checks: flux conservation on **`edges_b1`**, **`J ∝ 1/h`**, and divergence consistency after one step.
4. **Non-chain graph** — Small tree (existing PCG tests in `electrochemistry.rs` tests module) as **stress** for any graph Laplacian preconditioner coupling.
5. **3D fixture (minimal)** — One **tetra/hex** patch or synthetic **E** edges with known **φ**, **c** manufactured solution (or Method of Manufactured Solutions) — **regression** for orientation and **`h`**.
6. **Performance smoke** — Optional **`#[ignore]`** test: Arnoldi iterations vs **`N`** for a single matvec budget (no CI wall-time regression on default lane).

**Feature flags:** Gate new paths behind **`solver-experimental`** (and electrochemistry features as today) until default CI is stable.

---

## 7. Out of scope (this document)

- No implementation scaffold in this change set (roadmap only).
- No **`.cursor/plans/*`** artefacts.
- Photonics DEC 3D, Striatus JFNK, and monad purge are separate exec lanes; cross-link only when shared **`thmc_jfnk`** changes land.

---

*Last updated: aligns with MaOS v04 exec structure and in-tree PNP/`thmc_jfnk` as of authoring.*

# Photonics DEC — chain stub vs `faces_b2` → 3D roadmap

**Scope:** Track how today’s **1-D uniform-chain** TE path and **feature-off** stub relate to the **`faces_b2`** small-patch DEC surface, how that connects to **[`tests/dec_identities.rs`](../tests/dec_identities.rs)** and the **[`photonics_dec_patch_uses_metric_dual_edge_hodge`](../src/physics/solvers/photonics.rs)** gate, and what **execution-gated** work remains for **volumetric 3D** DEC Maxwell alignment with matrix **#6**.

**Authority:** [`Solver-Status.md`](Solver-Status.md) (photonics lane / DEFERRAL), [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6**, [`MAOS_PENDING_M6_DEC_AUDIT.md`](MAOS_PENDING_M6_DEC_AUDIT.md). For executive sequencing, [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md) §**2** — Photonics DEC **2D/3D**.

---

## 1 — Two entry shapes (do not conflate)

| Track | What ships today | Role |
| --- | --- | --- |
| **Uniform x-monotone chain** | [`PhotonicsSolver::solve_maxwell_curl_curl`](../src/physics/solvers/photonics.rs) with **`photonics`**: TE \(E_y\) via the same tridiagonal story as scalar Helmholtz; primal-chain DEC assembly on the **path graph** only. | Production-shaped **1-D** regression anchor; **not** general simplicial DEC. |
| **`faces_b2` patch (`PhotonicsDecFacesPatch`)** | Optional **`dec_patch`** → small-**N** host **dense** (Gauss–Jordan when \(N\le 64\)) or **capped CG** beyond, on **\(3N\)** nodal vector DoFs (stacked **\(2\cdot 3N\)** real system when **`eps_r_imag`** is lossy at small **\(N\)** only), with **`faces_b2`** COO + **`face_column_ranges`** (same contract as [`UnifiedMaterialStateTensor::faces_b2`](../src/core/tensors.rs)). | **2-surface** (embedded simplicial **2-cells** in \(\mathbb{R}^3\)) milestone toward matrix **#6** — **not** a volumetric **3D** tet/hex solver. |

**Chain stub (`photonics` off):** default builds document [`solve_maxwell_curl_curl`](../src/physics/solvers/photonics.rs) as a **no-op** / Phase-7 stub (returns `e_field` unchanged); [`tests/verification/photonics_curl_curl_stub_default_build.rs`](../tests/verification/photonics_curl_curl_stub_default_build.rs) pins that behaviour. That stub is **orthogonal** to proving **`faces_b2`** DEC correctness — it only keeps non-photonics CI stable.

**Non-chain without patch:** when the graph is **not** a recognized uniform chain and **`dec_patch`** is missing or invalid, the solver **warns and passes through** `e_field` unchanged — the **real** DEC Maxwell matvec/solve for arbitrary topology is explicitly **`faces_b2` + patch** (or future sparse/volumetric successors), not the chain reduction.

---

## 2 — `dec_identities` vs photonics solve path

[`tests/dec_identities.rs`](../tests/dec_identities.rs) proves **primal DEC morphism** facts on fixed **`edges_b1` / `faces_b2`** tensors: \(d_1 \circ d_0 = 0\) (Burn **`dec_curl_d1_annihilates_gradient_*`** family) and **unweighted** \(d_1^\top\) adjoint identities (**`dec_primal_d1_adjoint_identity_*`**), using [`dec_primal`](../src/physics/dec_primal.rs) — **without** calling [`PhotonicsSolver`](../src/physics/solvers/photonics.rs).

**Linkage:** the photonics module rustdoc and [`tests/verification/photonics_fresnel.rs`](../tests/verification/photonics_fresnel.rs) intentionally reuse **the same** quad-split / two-quad **`faces_b2`** incidence as `dec_identities` so patch solves and DEC identities **witness the same COO contract**. Passing **`dec_identities`** is **necessary** topology evidence; it is **not** sufficient to claim matrix **#6** or “3D DEC shipped” — see the audit table in [`MAOS_PENDING_M6_DEC_AUDIT.md`](MAOS_PENDING_M6_DEC_AUDIT.md).

**Suggested narrow gate (existing):**

```bash
cargo test --features solver-experimental --test photonics_fresnel --test dec_identities
```

---

## 3 — [`photonics_dec_patch_uses_metric_dual_edge_hodge`](../src/physics/solvers/photonics.rs)

**Purpose:** compile-time **honesty predicate** for the **`faces_b2`** patch curl leg — it returns **`true`** today: the patch stack applies a **diagonal primal-length \(\star_1\)** (symmetric \(\sqrt{\star_1}\) sandwich from SI **`coords_n3`** edge lengths) on **\(d_1^\top d_1\)** edge tangential projections — **not** circumcentric/barycentric dual-cell masses.

**Regression rule (documented in rustdoc):** if the implementation **regresses** to unweighted \(d_1^\top d_1\) on the patch path, set the predicate back to **`false`** **only** with a dated rationale and aligned matrix **#6** / [`Solver-Status.md`](Solver-Status.md) prose in the same change set. **Row #6 / lane 100%** still require assembly, volumetric 3D, refined dual Hodge, sparse production solves, PML, and BCs beyond a pin — flipping this predicate does **not** close matrix acceptance.

**Precursor / coverage tests:** e.g. **`dec_patch_primal_edge_lengths_si_*`** and the honesty family in [`photonics_fresnel.rs`](../tests/verification/photonics_fresnel.rs) / [`photonics.rs`](../src/physics/solvers/photonics.rs) (`dec_patch_dense_node_cap_is_stable_contract`, `dec_patch_dual_edge_hodge_diagonal_primal_length_wired`).

---

## 4 — Target: volumetric **3D** (beyond embedded `faces_b2` sheets)

Today’s **`PhotonicsDecFacesPatch`** path is explicitly **2-cell / surface** DEC embedded in \(\mathbb{R}^3\) (see [`MAOS_PENDING_M6_DEC_AUDIT.md`](MAOS_PENDING_M6_DEC_AUDIT.md) — “**3D volume**” gap). A **3D roadmap** therefore means, at minimum:

1. **Complexes:** primal **3-cells** (tets/hexes) and **2-faces** as **boundary** or **interior** facets, with orientation-consistent **`faces_b2`-style** incidence **lifted from manifold/mesh state**, not only hand-authored COO in tests. **Incremental (2026-05-12):** [`canonical_tetrahedron_boundary_dec_coo`](../src/physics/dec_primal.rs) + **`dec_curl_d1_annihilates_gradient_tetrahedron_boundary_burn`** / **`dec_primal_d1_adjoint_identity_tetrahedron_boundary_burn`** in [`tests/dec_identities.rs`](../tests/dec_identities.rs) pin the **closed boundary** COO of one canonical tet — **skin only**; interior facet enumeration remains open.
2. **Operators:** curl–curl (or equivalent 1-form/edge-based Maxwell weak form) with **Hodge stars** at the right grades (\(\star_1\), \(\star_2\) as needed), aligned with categorical notes in [`FP_CATEGORICAL_DEC.md`](FP_CATEGORICAL_DEC.md).
3. **Solvers:** **sparse** Krylov (or similar) inner solves at production **N** — replacing the **`PHOTONICS_DEC_PATCH_MAX_NODES_DIRECT`** dense cap for large problems.
4. **Materials / BCs:** **complex** \(\varepsilon\), **PML** or absorbing boundaries on the **same** API path as production claims; BCs beyond a single **gauge pin**.

Sequence should extend [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md) §**2** acceptance bullets in lockstep with matrix row **#6** text.

---

## 5 — Execution todos: **`exec-solver-photonics-dec`** / **`exec-solver-photonics-dec-3d`**

**IDs:** **`exec-solver-photonics-dec`** (parent / umbrella for photonics DEC acceptance slices) · **`exec-solver-photonics-dec-3d`** (volumetric **3D** body)

**Intent (`exec-solver-photonics-dec-3d`):** **Execution-phase** (not doc-only) closure of photonics **volumetric 3D** DEC + tensor \(\varepsilon\) + sparse/BC/complex-\(\varepsilon\) slices through [`solve_maxwell_curl_curl`](../src/physics/solvers/photonics.rs) (or a named successor), until [`VERIFICATION_COMPLETION_MATRIX.md`](VERIFICATION_COMPLETION_MATRIX.md) row **#6** *Exact acceptance criterion* and [`Solver-Status.md`](Solver-Status.md) photonics lane match shipped code.

**Policy:** Per [`EXEC_SOLVER_REALIZATION_MAOS_V04.md`](EXEC_SOLVER_REALIZATION_MAOS_V04.md) — **`exec-solver-*`** todos stay **pending** until real implementation + CI gates land; **do not** clear these ids from documentation-only passes.

**Plan alignment:** Substantive scope overlaps swarm item **`closeout-m6-dec`** in [`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md) (`#6` Wire production 2D/3D DEC…). Treat **`exec-solver-photonics-dec`** / **`exec-solver-photonics-dec-3d`** as the **solver-lane execution slices** for that row’s photonics/3D body of work; keep matrix prose, [`photonics_dec_patch_uses_metric_dual_edge_hodge`](../src/physics/solvers/photonics.rs), and verification logs **consistent** when milestones complete.

---

## 6 — Quick reference links

| Artifact | Path |
| --- | --- |
| Photonics solver + patch struct + Hodge predicate | [`../src/physics/solvers/photonics.rs`](../src/physics/solvers/photonics.rs) |
| Primal \(d_1\) / \(d_1^\top\) helpers | [`../src/physics/dec_primal.rs`](../src/physics/dec_primal.rs) |
| DEC topology tests | [`../tests/dec_identities.rs`](../tests/dec_identities.rs) |
| Photonics integration (chain, Fresnel, patch) | [`../tests/verification/photonics_fresnel.rs`](../tests/verification/photonics_fresnel.rs) |
| Default-build stub | [`../tests/verification/photonics_curl_curl_stub_default_build.rs`](../tests/verification/photonics_curl_curl_stub_default_build.rs) |

**`closeout-m6-dec` — quick test gate index (sorted by file, then name):** run with `--features photonics` / `solver-experimental` as in [`Solver-Status.md`](Solver-Status.md).

- [`tests/dec_identities.rs`](../tests/dec_identities.rs) — `dec_curl_d1_annihilates_gradient_on_triangle_faces_b2_burn`, `dec_curl_d1_annihilates_gradient_quad_split_two_faces_burn`, `dec_curl_d1_annihilates_gradient_two_quads_shared_edge_burn`, `dec_curl_d1_annihilates_gradient_tetrahedron_boundary_burn`, `dec_primal_d1_adjoint_identity_single_triangle_burn`, `dec_primal_d1_adjoint_identity_quad_split_two_faces_burn`, `dec_primal_d1_adjoint_identity_tetrahedron_boundary_burn`, `dec_primal_d1_adjoint_identity_two_quads_shared_edge_burn` (primal \(d_1\) / \(d_1^\top\) on fixed `faces_b2`; no `PhotonicsSolver`).
- [`tests/verification/photonics_fresnel.rs`](../tests/verification/photonics_fresnel.rs) — chain vs Helmholtz / TE stencil / Fresnel / patch residual family: `curl_curl_y_mode_matches_scalar_helmholtz*`, `dec_te_primal_tensor_matches_chain_stencil`, `dec_maxwell_assembly_quad_split_*`, `solve_maxwell_dec_patch_*` (incl. **`solve_maxwell_dec_patch_quad_split_lossless_auto_csr_matches_dense_csr_inner_off`** — default **`auto`** CSR-first vs **`CSR_INNER=off`** dense on quad split; **`solve_maxwell_dec_patch_quad_split_scalar_eps_imag_stacked_residual`** for nodal **`eps_r_imag`** + stacked \(2\cdot 3N\) dense), `solve_maxwell_curl_curl_pass_through_quad_split_not_chain`, `solve_maxwell_curl_curl_dec_patch_csr_inner_matches_dense_quad_split`, `two_half_spaces_fresnel_te_no_pml_matches_analytic`, `photonics_matrix_six_honesty_tests` in [`photonics.rs`](../src/physics/solvers/photonics.rs) (`dec_patch_dense_node_cap_is_stable_contract`, `dec_patch_dual_edge_hodge_diagonal_primal_length_wired`).
- Swarm mapping: [`MULTI_AGENT_GAP_CLOSURE_PLAN.md`](MULTI_AGENT_GAP_CLOSURE_PLAN.md) (`closeout-m6-dec`).

---

*Roadmap doc only — no operator implementation in this pass.*

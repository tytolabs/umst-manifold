<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Rheology: discrete pressure Poisson on `TopologicalLaplacian` (design roadmap)

**Status:** design + partial ship — **Jacobi-preconditioned CG** on \(-\mathcal{L}\) for \(\mathcal{L}\phi=b_h(u^\*)\) is implemented in [`rheology_flow.rs`](../../src/physics/solvers/rheology_flow.rs) (mean-free RHS, relative residual exit); this memo still records **MAC / open-\(x\) BC** follow-ons. **Motivation:** align developed-channel Chorin with plane Poiseuille once staggered divergence and inlet/outlet data are consistent with the split. **Related:** [`Solver-Status.md`](../Solver-Status.md) Rheology lane; harness notes in [`tests/verification/rheology_poiseuille.rs`](../../tests/verification/rheology_poiseuille.rs). **Capability index:** [`../PHYSICS_CAPABILITY_GAPS.md`](../PHYSICS_CAPABILITY_GAPS.md).

---

## 1. Operator: same discrete Laplacian as today

**Anchor — assembled operator:** [`TopologicalLaplacian::scalar_laplacian`](../../src/physics/laplacian.rs) implements \(\mathcal{L}x = B_1\bigl((1-\bar d)\odot B_1^\top x\bigr)\) on the primal 1-skeleton (`edges_b1`), with per-edge flow coefficients from nodal `damage` (today rheology passes zeros ⇒ fully connected edges).

**Pressure Poisson (target):** seek \(\phi\) on nodes such that

\[
\mathcal{L}\phi = \frac{1}{\Delta t}\, b_h(u^\*)
\]

where \(b_h(u^\*)\) is a **consistent discrete divergence** of the predictor velocity field (see §2). The **same** sparse pattern and sign structure as `scalar_laplacian` applies; the upgrade is (a) RHS construction, (b) boundary rows / Lagrange pinning for the pressure null space, (c) a convergent linear solve replacing fixed-count Richardson ([`POISSON_ITERS` / `POISSON_OMEGA`](../../src/physics/solvers/rheology_flow.rs)).

**Non-goal for this memo:** changing the Bingham / thixotropy closure in [`step_experimental`](../../src/physics/solvers/rheology_flow.rs) beyond feeding a better \(\phi\).

---

## 2. RHS: from surrogate triple-Laplacian to discrete divergence

**Today (surrogate):** [`step_experimental`](../../src/physics/solvers/rheology_flow.rs) sets `rhs = L(u*_x)+L(u*_y)+L(u*_z)` (lines ~326–334). The module rustdoc states this is **not** \(\nabla_h\!\cdot u^\*\) for a staggered incompressible discretization ([`rheology_flow.rs`](../../src/physics/solvers/rheology_flow.rs) — “Chorin-style split” §2 and “MAC + Poisson” bullets).

**Target RHS:** form an edge-based flux \(F^\*_e\) from \(u^\*\) (nodal `[B,N,3]` gathered to edges), then

\[
b_h(u^\*) = \texttt{primal\_divergence\_from\_edge\_flux\_topo}(F^\*, \ldots)
\]

**Anchor:** [`primal_divergence_from_edge_flux_topo`](../../src/physics/dec_primal.rs) (weak \(B_1^\top\) scatter-sum). The MAC note in `rheology_flow.rs` already names this as the insertion point for a consistent Poisson source.

**Design choice (document before coding):** On an unstructured nodal graph, \(F^\*_e\) may be **oriented edge velocity jump** (current viscous flux uses `du` from [`primal_scalar_edge_increment`](../../src/physics/dec_primal.rs)), a **density-weighted average normal flux** once face topology exists, or a **MAC face flux** if the predictor is refactored to commit face-normal unknowns. The roadmap requires one explicit contract per lane: **graph-only v0** = divergence of a chosen edge flux tensor field dimension-by-dimension or a signed scalar flux derived from \(u^\*\cdot\hat{t}\); **MAC v1** = divergence of face-normal provisional fluxes (new DOFs), still reducible to a pressure Poisson with a possibly different Laplacian stencil — out of scope for “same `TopologicalLaplacian`” unless proven equivalent on the channel scaffold.

---

## 3. BC matrix: channel walls, inlet, outlet

**Context:** Walls are currently enforced **outside** the solver via nodal masks in tests ([`rheology_flow.rs`](../../src/physics/solvers/rheology_flow.rs) audit memo — “wall velocity … outside the solver”). Open boundaries (pressure drop / developed flow) are not embedded in \(\mathcal{L}\) or the projection step.

**Null space:** Pure Neumann data on a connected graph leaves \(\phi\) determined only up to an additive constant. **Pinning:** one Dirichlet row (or a rank-1 penalty / Lagrange multiplier) fixes the gauge.

**BC matrix concept (design):** Build a sparse selector \(B\!\in\!\mathbb{R}^{n_b\times N}\) and optional constraint vector \(g\) such that the reduced system is

\[
\begin{bmatrix}\mathcal{L} \\ B\end{bmatrix}\phi \stackrel{?}{=} \begin{bmatrix}\tilde b \\ g\end{bmatrix}
\]

or, equivalently, augment \(\mathcal{L}\leftarrow \mathcal{L} + B^\top W B\) for penalty form. Rows types to plan for the **65×17 / 64×16** channel harnesses:

| Region | Intended row | Notes |
|--------|--------------|--------|
| Wall / no-slip support nodes | Omit or Neumann on \(\phi\)**?** | Classic Chorin often uses homogeneous Neumann on \(\phi\) at walls with no-slip \(u\); **compatibility** with discrete \(\nabla\!\cdot\) must be checked for the chosen RHS. Alternative: Dirichlet \(\phi\) from hydrostatic head on wall — only if consistent with pressure increment interpretation. |
| Inlet / outlet | Dirichlet \(\phi\) or flux BC on \(u\) | Pressure drop driving flow may appear as body force today; open boundary **I/O** needs either prescribed \(p\) (Dirichlet on pressure correction) or specified normal velocity flux consistency. |
| Gauge | Single node pin | \(B\) = one row with identity on one interior or boundary node. |

**Deliverable before implementation:** a short “compatibility table” (Neumann/Dirichlet on \(\phi\) vs \(u\) BCs) keyed to the chosen RHS in §2, referencing the same channel test layout as [`rheology_poiseuille.rs`](../../tests/verification/rheology_poiseuille.rs).

---

## 4. Chorin steps (aligned with current code path)

The following maps the **target** split onto the existing [`step_experimental`](../../src/physics/solvers/rheology_flow.rs) structure (predictor → Poisson → projection → pressure update).

1. **Predictor \(u^\*\)** — unchanged in intent: `u_star = velocity + dt * (g + viscous_accel + pressure_accel)` (same file). Viscous term already uses [`primal_divergence_from_edge_flux_topo`](../../src/physics/dec_primal.rs) on edge viscous flux; pressure gradient uses the same divergence of edge pressure flux.

2. **Pressure increment \(\phi\)** — replace surrogate Richardson block with:
   - assemble \(\mathcal{L}\) (same semantics as `scalar_laplacian`, optionally extracted as explicit CSR for host solves);
   - form RHS \(\tilde b = \frac{1}{\Delta t} b_h(u^\*)\) per §2;
   - apply BC / gauge (§3);
   - solve with CG / SSOR / Jacobi with tolerance **or** fast path: [`electrochemistry`](../../src/physics/solvers/electrochemistry.rs) chain **Thomas** Poisson when topology is a 1-D path (analogy only — channel is 2D quad graph).

3. **Projection** — keep the **shape** of the correction: edge increments [`primal_scalar_edge_increment`](../../src/physics/dec_primal.rs) on \(\phi\), tangent \(\hat t\) from `u_star`, `proj_flux = dphi * t_hat * flow_coeff`, then [`primal_divergence_from_edge_flux_topo`](../../src/physics/dec_primal.rs) → subtract from `u_star` (current lines ~343–363). Once \(\phi\) solves the consistent Poisson, this step remains the natural discrete Helmholtz–Hodge projection **provided** BCs are compatible.

4. **Pressure update** — `pressure_new = pressure + phi` (unchanged contract).

**Regression policy:** When a true Poisson ships, revisit [`chorin_surrogate_poisson_amplification_regression_guard`](../../tests/verification/rheology_poiseuille.rs) — either retire or replace with a stability band on the **new** solve.

---

## 5. MAC path (optional second lane)

The rustdoc “MAC + Poisson” section ([`rheology_flow.rs`](../../src/physics/solvers/rheology_flow.rs) lines ~79–100) explicitly scopes **2D channel MAC + consistent divergence BCs** as larger than a sub-hundred-line swap. If MAC is chosen:

- **Predictor** commits face-normal fluxes; **Poisson** uses a cell-centred or staggered Laplacian that may **not** coincide with [`TopologicalLaplacian`](../../src/physics/laplacian.rs) without reduction lemmas.
- This memo’s “same `TopologicalLaplacian`” lane stays the **default incremental** upgrade; MAC is a **parallel** design fork with its own operator assembly file anchors (future `*_mac.rs` or extension of `EdgeTopology`).

---

## 6. Acceptance touchpoints (existing tests)

| Test / doc | Role |
|------------|------|
| [`chorin_steady_channel_64x16_vs_regularized_reference`](../../tests/verification/rheology_poiseuille.rs) | **`f02120d`:** **`research-stack`** smoke (**no `#[ignore]`**) — 100 substeps on **65×17**, finite ‖u‖/‖p‖ + ‖u‖∞ band; full multi-thousand-step steady **L²** vs regularized reference still deferred until MAC + open **x** BCs. |
| [`chorin_developed_channel_centreline_vs_regularized_reference`](../../tests/verification/rheology_poiseuille.rs) | **`f02120d`:** **`research-stack`** smoke (**no `#[ignore]`**) — 80 steps, centreline finite and **≤** developed regularized Bingham reference (+ sanity on **p**); tight profile **L²** still deferred. |
| [`chorin_surrogate_poisson_amplification_regression_guard`](../../tests/verification/rheology_poiseuille.rs) | CI guard (historical name): two-step ‖u‖∞ bracket on **65×17** catches regression toward legacy surrogate-scale amplification; shipped RHS is weak primal divergence + momentum-consistent projection. **`solver-experimental`:** [`chorin_poisson_rhs_surrogate_vs_weak_divergence_tiny_channel`](../../src/physics/solvers/rheology_flow.rs) compares legacy \(\sum_c\mathcal{L}u^\*_c\) vs shipped divergence RHS on **5×5**. |
| [`Solver-Status.md`](../Solver-Status.md) §Verification scope item 7 + **Solver lanes — Rheology** | Public ship criteria and R2.2 follow-up pointer. |

---

## 7. File anchor index

| Topic | Path |
|-------|------|
| Chorin MVP + MAC hook bullets | [`src/physics/solvers/rheology_flow.rs`](../../src/physics/solvers/rheology_flow.rs) |
| \(\mathcal{L}\) matvec | [`src/physics/laplacian.rs`](../../src/physics/laplacian.rs) |
| Divergence / edge increment | [`src/physics/dec_primal.rs`](../../src/physics/dec_primal.rs) |
| Channel tests + amplification guard | [`tests/verification/rheology_poiseuille.rs`](../../tests/verification/rheology_poiseuille.rs) |
| Chain Poisson precedent (Thomas) | [`src/physics/solvers/electrochemistry.rs`](../../src/physics/solvers/electrochemistry.rs) |
| Solver / CI contract | [`docs/Solver-Status.md`](../Solver-Status.md) |

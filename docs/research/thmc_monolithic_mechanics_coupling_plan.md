# Monolithic THMC with mechanics — implementation plan (research)

**Workspace:** `umst-manifold` · **Feature lane:** `thmc-coupled` / `solvers::thmc`.  
**Related:** Executive roadmap + phased acceptance — [`v0.4_track13_monolithic_thmc_mechanics_coupling_plan.md`](v0.4_track13_monolithic_thmc_mechanics_coupling_plan.md); Track 13 Newton / JFNK memo [`v0.4_track13_monolithic_newton_thmc.md`](v0.4_track13_monolithic_newton_thmc.md); [`Solver-Status.md`](../Solver-Status.md) §Solver lanes — THMC; matrix row **#8** in [`VERIFICATION_COMPLETION_MATRIX.md`](../VERIFICATION_COMPLETION_MATRIX.md).

This document is a **step-by-step researched plan** for **full monolithic coupling** of thermal ($T$), humidity ($h$), hydration degree ($\alpha$), and **quasi-static bar-network displacement** $\mathbf u\in\mathbb{R}^{3N}$ in one implicit root $R(U^{n+1})=0$. It does **not** assert that the full stack is already shipped; claims below are tied to **verified** paths in the cited files.

---

## 1. Problem statement

### 1.1 What “full monolithic THMC coupling” means here

**Target:** At each time step, find a single stacked unknown
\[
U^{n+1} = \bigl(T^{n+1},\,h^{n+1},\,\alpha^{n+1},\,\mathbf u^{n+1}\bigr)
\]
such that **all** semi-discrete residuals vanish simultaneously:
\[
R(U^{n+1}) = \begin{bmatrix} R_T \\ R_h \\ R_\alpha \\ R_u \end{bmatrix} = 0,
\]
with backward Euler on the **evolving** scalar fields $(T,h,\alpha)$ and **quasi-static equilibrium** encoded in $R_u$ (no inertia term in the shipped bar-network contract — see [`Solver-Status.md`](../Solver-Status.md) row **#10** on transient solids).

**Coupling loops that must appear in $J=\partial R/\partial U$ (at least conceptually):**

- $T \leftrightarrow \alpha$: Arrhenius $\dot\alpha(\alpha,T)$ and exothermic $\dot\alpha$ source in $R_T$ (already in [`ThmcImplicitEulerThermalHydrationResidual`](../../src/physics/solvers/thmc_residual.rs)).
- $\alpha \leftrightarrow \mathbf u$: stiffness scales with $\alpha$ in [`ThmcSolver::step_experimental`](../../src/physics/solvers/thmc.rs) ($E_{\text{nodal}} = \alpha \cdot \texttt{stiffness\_e\_scale\_pa}$ after clamp) → $\partial R_u/\partial\alpha \neq 0$.
- Optional later: $h \to$ shrinkage / effective strain → $\mathbf f_{\mathrm{int}}$ or equivalent load in $R_u$ (today shrinkage helpers [`mc2010_style_notional_shrink_strain`](../../src/physics/solvers/thmc.rs), [`shrink_strain_from_saturation_loss`](../../src/physics/solvers/thmc.rs) are **verification / notional hooks**, not assembled into `VectorMechanicsSolver::solve_equilibrium` body force in the shipped step).

### 1.2 What is **already** in-tree vs **not**

| Capability | Status | Evidence |
|------------|--------|----------|
| Implicit BE $(T,\alpha)$ residual + dense damped Newton | **Shipped** (small graphs) | [`ThmcImplicitEulerThermalHydrationResidual`](../../src/physics/solvers/thmc_residual.rs); wired from [`ThmcSolver::step_experimental`](../../src/physics/solvers/thmc.rs) when `implicit_t_alpha_newton: Some(_)`; tests `thmc_implicit_euler_t_alpha_*`, `thmc_step_implicit_t_alpha_newton_*` in [`tests/verification/thmc_drying_shrinkage.rs`](../../tests/verification/thmc_drying_shrinkage.rs). |
| Implicit BE $(T,h,\alpha)$ residual + stacked Newton **without** $R_u$ | **In-tree assembler + tests** | [`ThmcImplicitEulerThermalHumidityHydrationResidual`](../../src/physics/solvers/thmc_residual.rs); tests `thmc_implicit_euler_t_h_alpha_residual_humidity_matches_brute_force_two_nodes`, `thmc_implicit_euler_t_h_alpha_multi_newton_monotone_stacked_residual_norm` — module comment and test name explicitly state **no** $R_u$ / no `ThmcSolver` wiring. |
| Layout const $M = NF_T+NF_h+NF_\alpha+3N$ | **Const anchor only** | [`ThmcMonolithicImplicitUnknownLayout::field_major_stacked_dof_count`](../../src/physics/solvers/thmc_residual.rs) includes $3N$; prefix count [`field_major_scalar_transport_hydration_dof_count`](../../src/physics/solvers/thmc_residual.rs) matches leading $(T,h,\alpha)$ block. |
| Operator-split step: $(T,\alpha)$ → $h$ → `solve_equilibrium` | **Shipped** | [`thmc.rs`](../../src/physics/solvers/thmc.rs) `step_experimental` loop; humidity **not** in implicit Newton branch (CI: `thmc_step_implicit_t_alpha_newton_same_humidity_as_explicit_split`). |
| Monolithic $R_u$ in same Newton stack as $(T,h,\alpha)$ | **Not shipped** | No struct assembling $R_u$ next to transport; `VectorMechanicsSolver::solve_equilibrium` solves $K\mathbf u=\mathbf f$ **to completion** per substep rather than exposing a finite $R_u$ for joint Newton. |
| `ThmcTripleResidual` / `step_implicit_euler` as named APIs | **Not found** | Repository grep: no symbol `ThmcTripleResidual`; no `step_implicit_euler` — closest is [`ThmcSolver::step_thermal_implicit`](../../src/physics/solvers/thmc.rs) for **thermal-only** implicit CG. |

**Slice `28d9c11` (Track 13) alignment:** The delivered milestone matches **$(T,\alpha)$** (and the extended **$(T,h,\alpha)$** residual type) transport chemistry — **not** a combined residual that includes bar equilibrium $R_u$.

---

## 2. Governing residuals (symbolic)

Discrete graph Laplacian on scalars: use the same operator as today,
\[
\mathcal{L}_\phi(\phi) = \bigl(\Delta_0 \phi\bigr)_i
\]
implemented as [`TopologicalLaplacian::scalar_laplacian`](../../src/physics/laplacian.rs) with edge set `edges_b1` and nodal damage mask `damage_m` (see [`Mathematical-Foundations.md`](../Mathematical-Foundations.md) §7 for $\Delta_0 = d^{*}d$ on the 1-skeleton).

### 2.1 $R_T$ — thermal (backward Euler)

\[
R_T = T^{n+1} - T^n - \Delta t\,\Bigl(\mathcal{L}_T(T^{n+1}; d^n) + q_{\mathrm{exo}}\,\dot\alpha(\alpha^{n+1}, T^{n+1})\Bigr),
\]
with $\dot\alpha$ from [`full_hydration_alpha_rate_tensor`](../../src/physics/solvers/thmc.rs) / [`ThmcHydrationKinetics`](../../src/physics/solvers/thmc.rs). Same structure as [`ThmcImplicitEulerThermalHydrationResidual::assemble`](../../src/physics/solvers/thmc_residual.rs).

**Discrete reference:** `lap_t = TopologicalLaplacian::scalar_laplacian(t, edges_b1, damage_m)`; `exo` from `d_alpha` slice and `exothermic_k_per_alpha_rate`.

### 2.2 $R_h$ — humidity (backward Euler baseline)

**Pure implicit diffusion (current building block):**
\[
R_h = h^{n+1} - h^n - \Delta t\,\mathcal{L}_h(h^{n+1}; d^n).
\]
Implemented in [`ThmcImplicitEulerThermalHumidityHydrationResidual::assemble`](../../src/physics/solvers/thmc_residual.rs).

**Split solver closure (shipped `ThmcSolver`):** After Laplacian, a **facet evaporation** algebraic update applies to the last node when `drying_last_node_evaporation_k > 0` ([`thmc.rs`](../../src/physics/solvers/thmc.rs)). For monolithic consistency, that closure must either:

- be rewritten as part of $R_h$ (or a slack variable), or  
- be deferred to a **post-Newton** correction with documented splitting error (worse for theory, easier for migration).

### 2.3 $R_\alpha$ — hydration degree (backward Euler)

\[
R_\alpha = \alpha^{n+1} - \alpha^n - \Delta t\,\dot\alpha(\alpha^{n+1}, T^{n+1}),
\]
with optional channel broadcast from $T$ channel 0 when $F_\alpha>1$ (same rule as [`ThmcImplicitEulerThermalHydrationResidual::assemble`](../../src/physics/solvers/thmc_residual.rs)).

**Bounds:** Explicit path uses `.clamp(0,1)` after update; monolithic Newton should document whether bounds are **active-set**, **smooth penalty**, or **line-search projection** only (Track 13 memo appendix §B).

### 2.4 $R_u$ — quasi-static bar-network equilibrium

At fixed $(\alpha^{n+1}, d^n)$, **discrete equilibrium**:
\[
R_u(\mathbf u, \alpha^{n+1}) = P\bigl(\mathbf f_{\mathrm{ext}} - \mathbf f_{\mathrm{int}}(\mathbf u; E(\alpha^{n+1}), d^n)\bigr) = 0,
\]
where $P$ is the **Dirichlet projector** from `displacement_bc_mask` (same semantics as [`VectorMechanicsSolver::packed_bar_network_equilibrium`](../../src/physics/mechanics.rs) / [`solve_equilibrium`](../../src/physics/mechanics.rs)): free rows satisfy force balance; clamped rows have prescribed increment zero.

**Not** a time-difference residual: no $(\mathbf u^{n+1}-\mathbf u^n)/\Delta t$ term in the quasi-static shipped model.

**Internal force:** [`bar_matvec`](../../src/physics/mechanics.rs) on axial stiffness $k_e \propto (EA/L)\cdot g(d_{\mathrm{edge}})$ with Young’s modulus averaged to edges from nodal `stiffness` (first channel $E$, second $\nu$ — see `packed_bar_network_equilibrium` in [`mechanics.rs`](../../src/physics/mechanics.rs)).

**Coupling to $\alpha$:** Match [`thmc.rs`](../../src/physics/solvers/thmc.rs): $E \propto \alpha$ at nodes (with lower clamp $10^{-6}$ today) before edge averaging.

### 2.5 Chemistry beyond $\alpha$ (scope note)

**In scope for this plan:** single hydration scalar $\alpha$ per node (or multi-channel $\alpha$ with documented broadcast). **Out of v0.4 monolith unless explicitly added:** pore solution speciation, ionic diffusion, sorption isotherms coupling $h$–$T$–$\alpha$. If added later, extend $U$ and $R$ with additional blocks; Jacobian sparsity gains extra nodal-local bands.

---

## 3. Unknown ordering, layout, Jacobian sparsity

### 3.1 Field-major stack (recommended — matches existing Newton flatten)

Per batch slice $b$, align with Track 13 appendix and [`ThmcImplicitEulerThermalHydrationResidual::one_damped_newton_step`](../../src/physics/solvers/thmc_residual.rs) / [`ThmcImplicitEulerThermalHumidityHydrationResidual::one_damped_newton_step`](../../src/physics/solvers/thmc_residual.rs):

\[
U_b = \bigl[\mathrm{vec}(T_{b,:,:})\ \|\ \mathrm{vec}(h_{b,:,:})\ \|\ \mathrm{vec}(\alpha_{b,:,:})\ \|\ \mathrm{vec}(\mathbf u_{b,:,:})\bigr].
\]

**Total DOFs per batch** (already in code):
\[
M = N F_T + N F_h + N F_\alpha + 3N
= \texttt{ThmcMonolithicImplicitUnknownLayout::field\_major\_stacked\_dof\_count}(N,F_T,F_h,F_\alpha).
\]

**Scalar transport + hydration prefix** (used to size $(T,h,\alpha)$ Newton today):
\[
M_{\mathrm{th}\alpha} = N(F_T+F_h+F_\alpha)
= \texttt{field\_major\_scalar\_transport\_hydration\_dof\_count}(\ldots).
\]

**Extension for mechanics:** New dense-FD or JFNK packing must flatten $\mathbf u$ as the **tail** $3N$ entries (row-major over nodes, $x,y,z$ per node), consistent with [`MechanicalPlan`](../../src/physics/solvers/thmc.rs) shape `[B,N,3]`.

### 3.2 Block Jacobian sketch ($4\times 4$)

|  | $\partial/\partial T$ | $\partial/\partial h$ | $\partial/\partial \alpha$ | $\partial/\partial \mathbf u$ |
|--|:---:|:---:|:---:|:---:|
| $R_T$ | graph Laplacian + local $\partial/\partial T$ of exothermic term | $0$* | $\partial R_T/\partial\alpha$ via $\dot\alpha$ | $0$ |
| $R_h$ | $0$* | graph Laplacian + sink derivatives | $0$* | $0$* |
| $R_\alpha$ | Arrhenius sensitivity | $0$* | diagonal / small dense per node | $0$* |
| $R_u$ | $0$* | $\partial \mathbf f_{\mathrm{int}}/\partial h$* | $\partial \mathbf f_{\mathrm{int}}/\partial \alpha$ (stiffness path) | sparse stiffness $K_T$ (bar network) |

\*Unless extended with sorption, shrinkage eigenstrain from $h$, or strain-dependent kinetics.

**Sparsity:**

- $J_{TT}, J_{hh}$: same edge-supported pattern as graph Laplacian.
- $J_{\alpha T}, J_{T\alpha}$: nodal-local (dense in small channel dimension).
- $J_{uu}$: bar-network pattern (couples endpoints of each edge).
- $J_{u\alpha}$: generally **dense in $\alpha$ per node** affecting incident edges after $E(\alpha)$ lift → sparse **node-star** pattern (nonzero for $(i,j)$ if nodes share an edge and $\alpha$ enters $E$ on that stencil).

---

## 4. Phased roadmap (dependency order)

Phases are ordered so each step has a **testable residual** before the next expands DOFs or solvers.

### Phase 0 — Contract freeze (docs + types)

**Goals:** Freeze unknown order, naming, and what “converged step” means (stacked $\|R\|_2$, block weights, BC handling on $R_u$).

**Files:** [`thmc_residual.rs`](../../src/physics/solvers/thmc_residual.rs) (rustdoc on `ResidualThmc`, `ThmcMonolithicImplicitUnknownLayout`), [`thmc.rs`](../../src/physics/solvers/thmc.rs) module docs, optionally [`Solver-Status.md`](../Solver-Status.md) when a CI test lands.

**Acceptance:** Doc-only PR; no behaviour change.

**Risks:** None technical; avoids parallel PRs disagreeing on vector layout.

---

### Phase 1 — $R_u$ evaluation (non-solving) + parity with `solve_equilibrium`

**Goals:** Implement a function `evaluate_r_u(trial_state) -> Tensor<[B,N,3]>` that computes **projected residual** $P(\mathbf f_{\mathrm{ext}} - K\mathbf u)$ using the **same** stiffness assembly and `bar_matvec` as [`packed_bar_network_equilibrium`](../../src/physics/mechanics.rs), **without** replacing the inner PCG loop yet.

**Dependency order:** stiffness from $\alpha$ → bar assembly → matvec $K\mathbf u$ → mask → subtract $\mathbf f_{\mathrm{ext}}$.

**Files:** [`mechanics.rs`](../../src/physics/mechanics.rs) (factor shared “$Ku$” path if needed), new helper in [`thmc_residual.rs`](../../src/physics/solvers/thmc_residual.rs) or small `thmc_mechanics_residual.rs`.

**Acceptance tests:**

- `thmc_r_u_zero_at_solved_equilibrium_two_node_chain` — compute $\mathbf u^\star$ with existing `solve_equilibrium`; assert $\|R_u(\mathbf u^\star)\|$ below tolerance vs same tolerances as PCG exit.
- Reuse SI chain fixtures from [`thmc_drying_shrinkage.rs`](../../tests/verification/thmc_drying_shrinkage.rs).

**Risks:** Duplicate assembly vs drift from `solve_equilibrium`; mitigate with **single** kernel for $K\mathbf u$ or golden-value comparison.

---

### Phase 2 — Stacked residual assembly $(T,h,\alpha,\mathbf u)$ **without** joint Newton

**Goals:** `ThmcMonolithicImplicitResidual::assemble` returning $(R_T,R_h,R_\alpha,R_u)$ at a trial `ThmcState`, reusing Phase 1 for $R_u$ and [`ThmcImplicitEulerThermalHumidityHydrationResidual`](../../src/physics/solvers/thmc_residual.rs) for the first three blocks (or shared internal fn to avoid drift).

**Files:** [`thmc_residual.rs`](../../src/physics/solvers/thmc_residual.rs), exports in [`mod.rs`](../../src/physics/solvers/mod.rs).

**Acceptance:**

- `thmc_monolithic_residual_l2_matches_sum_of_blocks_two_nodes` — numeric consistency.
- At trial $\mathbf u=\mathbf 0$, $R_u$ should equal projected external load (sign convention documented).

**Risks:** Humidity drying tail vs pure $R_h$ mismatch — flag as `MonolithicHumidityMode::{PureDiffusion, WithFacetSink}`.

---

### Phase 3 — Dense damped Newton on full $M\le 64$ (verification only)

**Goals:** Extend the pattern of `one_damped_newton_step` / `flatten_*` / `trial_from_packed_*` to **four** fields; Jacobian by column FD as today; Gauss–Jordan cap unchanged.

**Dependency order:** Phase 2 assembler → flatten $M$ → FD → solve $\delta$ → line search / damping.

**Files:** [`thmc_residual.rs`](../../src/physics/solvers/thmc_residual.rs).

**Acceptance (aligns with matrix #8 “next PR slice”):**

- `thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes` — one or multi-step monotone decrease of $\sqrt{\|R_T\|^2+\|R_h\|^2+\|R_\alpha\|^2+\|R_u\|^2}$.
- **Prerequisite graph sizing:** for $N=2$, $F_T=F_h=F_\alpha=1$: $M=12\le 64$ (see existing layout asserts in `thmc_implicit_euler_t_h_alpha_residual_humidity_matches_brute_force_two_nodes`).

**Risks:** Ill-conditioning when $E(\alpha)\to 0$; use smallest $\alpha$ clamp consistent with mechanics path; may need smaller `fd_eps` for $u$ block scale differences vs $T$.

---

### Phase 4 — Optional: shrinkage / eigenstrain in $R_u$

**Goals:** Map [`shrink_strain_from_saturation_loss`](../../src/physics/solvers/thmc.rs) (or a tensor variant) to an **equivalent nodal load** or **inelastic strain** channel consistent with bar kinematics.

**Dependency order:** constitutive choice (eigenstrain vs body force) → weak-form discrete derivation → assembly → tests vs notional band [`thmc_drying_shrinkage_within_mc2010_notional_band`](../../tests/verification/thmc_drying_shrinkage.rs).

**Risks:** Physical calibration is **notional** today; tests should stay **band / regression**, not claim MC2010 certification.

---

### Phase 5 — Wire opt-in path on `ThmcSolver` (still small-DOF)

**Goals:** `ThmcSolver` config flag (e.g. `monolithic_thmc_newton: Option<ThmcMonolithicNewtonConfig>`) calling Phase 3 from `step_experimental` **instead of** split sequence when graph caps allow; otherwise `Err` with clear message (mirror `implicit_t_alpha_newton` batch/DOF guards in [`thmc.rs`](../../src/physics/solvers/thmc.rs)).

**Acceptance:**

- `thmc_step_monolithic_newton_preserves_damage_mask_contract` (damage frozen at step entry, same as monolith memo).
- Split vs monolithic **difference** test on tiny graph where coupling is nontrivial (pattern: `thmc_step_implicit_t_alpha_newton_differs_from_explicit_split`).

**Risks:** API surface explosion; keep behind `Option` and feature `thmc-coupled`.

---

### Phase 6 — JFNK / block preconditioner (production scale)

**Goals:** GMRES/FGMRES on $J\delta=-R$ with matrix-free $Jv$; block Jacobi: Laplacian approximations for $T,h$; nodal scaling for $\alpha$; **one** linearized mechanics solve as approximate $J_{uu}^{-1}$ ([`VectorMechanicsSolver`](../../src/physics/mechanics.rs) PCG).

**Files:** new module e.g. `thmc_jfnk.rs`, integration with Burn autodiff only if AD path is opt-in ([`_implicit_step`](../../src/physics/solvers/thmc.rs) placeholder today).

**Acceptance:** Performance / smoke tests gated `#[ignore]` or small-$N$ first; matrix #8 “tol early exit” once host reads of $\|R\|$ are acceptable for the solver contract.

**Risks:** PCG early exit as **preconditioner** vs inconsistent $R_u$; document frozen-iteration linearization if needed.

---

### Phase 7 — Fracture / damage stagger (optional, cross-lane)

**Goals:** Within-step $u\!\leftrightarrow\!d$ per Track 12 / [`Solver-Status.md`](../Solver-Status.md) fracture row — **explicitly separate** from Phase 3–6; damage remains **frozen inside $R$** at $d^n$ until a later memo changes the contract ([`v0.4_track13_monolithic_newton_thmc.md`](v0.4_track13_monolithic_newton_thmc.md) appendix §A).

---

## 5. Verification and regression

### 5.1 Reuse patterns from [`thmc_drying_shrinkage.rs`](../../tests/verification/thmc_drying_shrinkage.rs)

- **Chain manifold + `edges_b1`:** `chain_manifold`, fixed `dt`, `ThmcHydrationKinetics::default()`.
- **Brute-force hand residual checks:** mirror `thmc_implicit_euler_t_alpha_residual_matches_brute_force_two_nodes` and `thmc_implicit_euler_t_h_alpha_residual_humidity_matches_brute_force_two_nodes`.
- **Newton monotone norms:** mirror `thmc_implicit_euler_t_h_alpha_multi_newton_monotone_stacked_residual_norm`.
- **Fracture kinematic parity** (post-$\mathbf u$): keep `bar_network_strain_matches_strain_tensor_for_fracture_after_mechanics` when SI `[N,3]` embedding is used.

### 5.2 Proposed new test names (additive)

| Test name | Intent |
|-----------|--------|
| `thmc_r_u_zero_at_solved_equilibrium_two_node_chain` | Phase 1 |
| `thmc_monolithic_residual_blocks_consistent_two_nodes` | Phase 2 |
| `thmc_monolithic_t_h_alpha_u_newton_lowers_stacked_norm_two_nodes` | Phase 3 (matrix #8 style) |
| `thmc_monolithic_newton_preserves_frozen_damage_snapshot` | Phase 5 |
| `thmc_step_monolithic_vs_split_coupling_difference_smoke` | Phase 5 integration |

Run under `--features thmc-coupled` (or `solver-experimental`) per existing lane.

---

## 6. Open research questions

1. **Staggered vs monolithic for fracture-THMC:** When phase field $d$ evolves on $\varepsilon(\mathbf u)$, is a **monolithic** $(T,h,\alpha,u,d)$ root justified, or is a **staggered** inner loop (R3.1 in composer follow-up) strictly better for stability / AT2 regularization?
2. **Humidity–mechanics closure:** Should drying shrinkage enter as **eigenstrain** (fixed stress-free strain) or **effective pore pressure** load? Implications for symmetry of $J_{hu}$ when sorption couples $h$ to $\alpha$.
3. **Scale-separated quasi-static $\mathbf u$:** If $\Delta t$ is small for transport but mechanics equilibrates fast, does **Schur reduction** (eliminate $\mathbf u$ approximately) yield a cheaper preconditioner than full GMRES on $M$?
4. **$f_{32}$ conditioning:** Bar PCG in [`packed_bar_network_equilibrium`](../../src/physics/mechanics.rs) uses relative tolerances; Newton outer loop may need **f64** accumulation or scaled unknowns for $J$ columns spanning $T\sim 300$ K and $u\sim 10^{-6}$ m.
5. **Damage inside monolith:** Freezing $d^n$ preserves today’s contract; **partial derivative $\partial R_T/\partial d$** is zero by design — confirm for transport coefficients masked per edge.

---

## 7. Notation cross-check

- Graph $G=(V,E)$, incidence, $\Delta_0$: [`Mathematical-Foundations.md`](../Mathematical-Foundations.md) §7.
- Manifold SI embedding: **`node_positions`** `[N,3]`, **`displacement_bc_mask`**: [`Mathematical-Foundations.md`](../Mathematical-Foundations.md) §6 (cartridge / `UnifiedMaterialStateTensor`).

---

## 8. References (implementation anchors)

- [`src/physics/solvers/thmc_residual.rs`](../../src/physics/solvers/thmc_residual.rs) — residuals, layout, dense Newton helpers.
- [`src/physics/solvers/thmc.rs`](../../src/physics/solvers/thmc.rs) — `ThmcSolver::step_experimental`, kinetics, shrinkage hooks, implicit $(T,\alpha)$ wiring.
- [`src/physics/mechanics.rs`](../../src/physics/mechanics.rs) — `packed_bar_network_equilibrium`, `bar_matvec`, `solve_equilibrium`.
- [`src/physics/laplacian.rs`](../../src/physics/laplacian.rs) — `TopologicalLaplacian`.
- [`tests/verification/thmc_drying_shrinkage.rs`](../../tests/verification/thmc_drying_shrinkage.rs) — verification patterns and explicit “no $R_u$” comments on $(T,h,\alpha)$ Newton tests.

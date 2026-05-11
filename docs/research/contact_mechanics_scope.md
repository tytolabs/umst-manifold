<!--
SPDX-License-Identifier: MIT
Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO
-->

# Contact mechanics — scope note (research)

## What the codebase does today

- **`physics::dec_primal`** ([`src/physics/dec_primal.rs`](../../src/physics/dec_primal.rs)) implements **primal-chain discrete exterior calculus (DEC) primitives**: oriented edge increments (\(d_0\)), weak divergence (\(B_1^\top\)), face curl (\(d_1\)) and transpose, from graph incidence and optional `faces_b2` COO data. Module rustdoc states explicitly that **metric/Hodge weights and material laws are not applied here** — there is **no** contact force, gap law, or collision response in this layer.
- The **cellular sheaf** picture in [`Mathematical-Foundations.md`](../Mathematical-Foundations.md) explains that working on the **1-skeleton** makes *topology change* (fracture, **contact**, severing) a **sparse combinatorial** update on incidence — that sentence describes a **data-model / roadmap** affordance, **not** a shipped contact or friction algorithm.
- Repository search for **contact**, **collision**, and **friction** in `umst-manifold` does not surface a constraint-based or penalty contact solver, rigid-body collision, or inter-surface friction law tied to mechanics time-stepping. Unrelated uses include **SIMP-style mass penalty** in mechanics (void–solid, not contact) and **no-slip wall** language in Bingham analytic references.

## DEC / sheaf “continuous mesh” vs collision–friction

- **DEC on the primal chain** = discrete differential operators consistent with the **fixed** (or editor-updated) graph and face COO; continuity is in the **discrete cochain** sense on that complex, not a penalty or Lagrange multiplier that **enforces** non-penetration between **independently meshed** bodies.
- **Collision / friction** typically requires **detection** (proximity, signed distance, or event capture), **constraints or impulses** (non-penetration, tangential stick–slip), and often **active set** or smooth-regularized laws — none of that is part of `dec_primal` or the documented solver lanes in [`Solver-Status.md`](../Solver-Status.md).

## Scope boundaries (examples)

| Scenario | Relation to current stack |
| -------- | --------------------------- |
| **Formwork removal** | Interface opening between concrete and rigid formwork is a **contact release** problem (changing contact set, possible impact). Requires explicit contact/impact modeling — **out of scope** unless added as a dedicated track. |
| **Soil–structure interaction (SSI)** | Relative motion, sliding, and separation at the soil–foundation interface need **interface elements**, Winkler/Pasternak variants with gap, or full contact — **not** covered by incidence-only DEC helpers or the current lane table. |
| **Rebar slip / bond–slip** | Bond–slip laws along bars are **constitutive interface** or embedded-bar formulations, distinct from graph DEC operators; **not** implemented as a named solver row here. |

## Takeaway

Treat **contact, collision, and inter-body friction** as **explicitly absent** from the v0.4 solver surface unless a future memo and verification row add them. DEC primal and sheaf topology docs support **consistent fluxes and discrete complexes** on a given graph; they do **not** substitute for contact mechanics.

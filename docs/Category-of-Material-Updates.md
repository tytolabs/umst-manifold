# Category of material updates (design sketch)

**Epic:** `maos-fp-categorical-v04` · **Task:** `fp-v04-traits-category`

This note fixes vocabulary for how UMST-related traits compose. It is *not* a formal category-theory proof layer; it guides API boundaries and future functor laws.

## Objects

- **`UnifiedMaterialStateTensor`** (and related verified/tensor bundles): a *state object* on the topology—coordinates, DEC incidence, and feature channels that solvers read and write under documented invariants.
- **`MixTensor`**: a *homogeneous* state object (bulk recipe / collapsed batch) used when topology is not in play.

## Morphisms (informal)

- **`IScienceCartridge`**: domain-specific evaluation **MixTensor → PhysicalResult** and **UMST → PhysicalResult** (bulk vs topology pass). Think of it as the *material law functor* from layout/state to thermodynamic summaries the rest of the stack consumes.
- **Solver façade types** (`PhysicsSolverZst` and concrete ZSTs such as `VectorMechanicsSolver`): *typed identities* for kernel families—markers for which morphism family is active, not trait objects over Burn kernels.
- **`TopologyPhysicsOrchestrator` / `ThmcSolver::step`**: *sequential composition* of transport → chemistry → mechanics → fracture (and optional rheology *outside* the default tick when documented). Composition order is the integration contract in `physics/orchestration.rs`.

## Composition & second law (interface contract)

- **Composition:** orchestration chains morphisms with explicit data dependencies (e.g. mechanics needs embeddings; fracture reads strain/energy proxies). Callers must not duplicate loops that violate that order.
- **Second law (thermodynamic interface):** `PhysicalResult` carries `free_energy`, `dissipation`, and related fields so CBF / RL and merge paths can treat **non-negative dissipation** and consistent energy accounting as *policy-level invariants*—implementations are responsible for numerical closure; the type is the hook for audits.

## Trait boundaries (this round)

| Surface | Role |
|--------|------|
| `IScienceCartridge` | Stable cartridge port; no solver internals. |
| `PhysicsSolverZst` | Burn-safe solver tagging; extend with new ZSTs, not dyn traits. |
| `TopologyPhysicsOrchestrator` | Single delegation chokepoint for THMC plan steps. |

Concrete cartridge pipelines (`umst-concrete-cartridge`) implement `compute_all` via staged tensor engines; topology coupling stays on the manifold + `ConcreteCartridge::compute_topology` path.

# FP categorical hardening audit

**Epic:** `maos-fp-categorical-v04` · **Task:** `fp-v04-1-analysis` (this deliverable)

This note inventories the core categorical surfaces in `umst-manifold`, relates them to an informal ontology (objects / morphisms / composition), records mutability versus pure-transform patterns, and lists trait-as-typeclass extension points. It does **not** duplicate sibling docs reserved for other agents (`FP_CATEGORICAL_DEC.md`, `FP_CATEGORICAL_IO.md`, `FP_CATEGORICAL_BURN.md`).

**DEC morphisms (primal↔dual, module map):** [FP_CATEGORICAL_DEC.md](FP_CATEGORICAL_DEC.md).

---

## § Ontology (UMST objects, solver morphisms, composition, second law)

### Objects

| Concept | Role | Primary types |
|--------|------|----------------|
| **Homogeneous bulk state** | Collapsed recipe / batch without full topology coupling | `MixTensor` — `umst-manifold/src/core/tensors.rs` |
| **Topological material state** | Cellular-sheaf DEC carrier: coordinates, incidence (`edges_b1`, `faces_b2`), equivariant feature stacks, optional SI embeddings | `UnifiedMaterialStateTensor` — `umst-manifold/src/core/tensors.rs` |
| **Proof-carrying wrapper** | Gate verified UMST for downstream consumers | `VerifiedUMST<B, P>` with `Proof` witness — `umst-manifold/src/core/tensors.rs` |
| **Thermodynamic observation** | Sparse nodal summaries consumed by RL / CBF / merge paths | `PhysicalResult<B>` — `umst-manifold/src/core/traits.rs` |
| **Coupled physics bundle** | Transport + chemistry + mechanics + fracture state carried across THMC ticks | `ThmcState<B>`, plans (`ThermalPlan`, …) — `umst-manifold/src/physics/solvers/thmc.rs` |

### Morphisms (informal)

- **`IScienceCartridge<B>`** — constitutive **evaluator**: `MixTensor → PhysicalResult` and `UnifiedMaterialStateTensor → PhysicalResult`. Parameterized by Burn `Backend` `B`; implementations supply bulk and topology passes without exposing solver internals (`umst-manifold/src/core/traits.rs`).
- **Solver configuration / step** — **`ThmcSolver`** holds numerical controls (`dt`, Newton counts, optional implicit blocks). **`ThmcSolver::step`** maps `(cartridge, ThmcState, UMST) → Result<ThmcState, …>` with **`&self`** on the solver: configuration is immutable for the call; state advances via returned `ThmcState`. Implementation: `umst-manifold/src/physics/solvers/thmc.rs`.
- **Orchestration wrapper** — **`TopologyPhysicsOrchestrator`** exposes a single chokepoint `run_plan_step` → delegates to `ThmcSolver::step` only (`umst-manifold/src/physics/orchestration.rs`).
- **Solver façade tags** — There is **no** trait named `PhysicsSolver` in this crate. The Burn-safe marker is **`PhysicsSolverZst`** plus concrete zero-sized types (e.g. **`VectorMechanicsSolver`** in `umst-manifold/src/physics/mechanics.rs`). Definitions: `umst-manifold/src/physics/framework.rs`.
- **State merge / policy** — `apply_physics_to_umst` applies `PhysicalResult` into a **mutable** `UnifiedMaterialStateTensor` (damage, optional temperature), respecting `policy_editable_mask` — `umst-manifold/src/core/apply_physics.rs`.

### Composition (integration order)

Canonical **plan-step** ordering is documented in module docs for `TopologyPhysicsOrchestrator` and implemented inside `ThmcSolver::step` (transport hints → chemistry placeholders → mechanics → fracture; optional rheology **outside** the default tick unless explicitly composed). Callers should not duplicate loops that violate that contract (`umst-manifold/src/physics/orchestration.rs`, `umst-manifold/src/physics/solvers/thmc.rs`).

### Second law (interface framing)

`PhysicalResult` exposes `free_energy`, `dissipation`, `safety_margin`, `cost`, and related channels so policy layers (`ManifoldGateway`, `ThermodynamicCBF`) can treat **non-negative dissipation** and consistent thermodynamic accounting as **policy-level** invariants. Numerical closure remains the responsibility of each cartridge / solver path; the type is the audit hook (`umst-manifold/src/core/traits.rs`, `umst-manifold/src/ai/ppo.rs`, `umst-manifold/src/ai/cbf.rs`).

---

## Inventory: `IScienceCartridge`, solver surfaces, orchestrator

### `IScienceCartridge<B: Backend>`

| Item | Path |
|------|------|
| Trait definition (`compute_all`, `compute_topology`; both **`&self`**) | `umst-manifold/src/core/traits.rs` |
| Re-export | `umst-manifold/src/core/mod.rs` → `pub use traits::*` |
| Production cartridge (concrete domain) | `umst-concrete-cartridge/crates/umst-concrete-cartridge/src/core/implementation.rs` (`impl … IScienceCartridge<B> for ConcreteCartridge<B>`) |
| RL / gateway stubs | `umst-manifold/src/ai/liquid_ppo.rs` (`PpoChainStubCartridge`) |
| Integration tests | `umst-manifold/tests/golden_path_physics_cbf.rs`, `gateway_info_gain.rs`, `apply_physics_writeback.rs`, `thmc_step_node_positions.rs`, `verification/thmc_drying_shrinkage.rs` |

### Physics solver naming (`PhysicsSolver` vs `PhysicsSolverZst`)

| Item | Path |
|------|------|
| Marker trait **`PhysicsSolverZst`** (in-tree name; not `PhysicsSolver`) | `umst-manifold/src/physics/framework.rs` |
| **`PhysicsBackend`** bound (`Backend<FloatElem = f32>`) | `umst-manifold/src/physics/framework.rs` |
| Example ZST implementor **`VectorMechanicsSolver`** | `umst-manifold/src/physics/mechanics.rs` |
| Coupled stepper **`ThmcSolver`** (cloneable config; **`step(&self, …)`**) | `umst-manifold/src/physics/solvers/thmc.rs` |
| Solver module index | `umst-manifold/src/physics/solvers/mod.rs` |

**Note:** External prototypes (`MaOS-Core`, `umst-prototype*`, WASM cartridges) may define a **different** `IScienceCartridge` shape (RL/data-provider stacks). This audit scopes **`umst-manifold`** and **`umst-concrete-cartridge`** as the FP categorical spine.

### `TopologyPhysicsOrchestrator`

| Item | Path |
|------|------|
| Struct + `run_plan_step` / `run_full_integration_step` (**`&self`**) | `umst-manifold/src/physics/orchestration.rs` |
| `thmc_solver` / `thmc_solver_mut` (**`&mut self`**, behind `feature = "thmc-coupled"`) | `umst-manifold/src/physics/orchestration.rs` |

---

## `&mut self` versus pure-transform opportunities

| Surface | Receiver | Notes |
|---------|----------|--------|
| `IScienceCartridge::compute_all` / `compute_topology` | `&self` | **Pure at the trait boundary** if implementations avoid interior mutation; enables cartridge sharing across stepping calls. |
| `ThmcSolver::step` | `&self` | Returns new `ThmcState`; inner `step_experimental` uses **`mut state`** locally — functional update style at the API level. |
| `TopologyPhysicsOrchestrator::run_plan_step` | `&self` | Thin delegate; no orchestrator mutation on the happy path. |
| `TopologyPhysicsOrchestrator::thmc_solver_mut` | `&mut self` | Optional tuning escape hatch (`thmc-coupled`). |
| `apply_physics_to_umst` | `umst: &mut UnifiedMaterialStateTensor` | **Explicit morphism into live UMST** for damage / temperature merge — inherently mutating. |
| `UnifiedMaterialStateTensor::write_scalar_channel` | `&mut self` | Low-level channel write used by merge paths. |

**Opportunities:** Keep cartridges **`&self`**-pure for functor-style reasoning; confine **`&mut UMST`** to merge utilities (`apply_physics`) and planners that intentionally rewrite policy-masked columns. Prefer returning updated **`ThmcState`** over hidden solver interior mutation (already the pattern for `ThmcSolver::step`).

---

## Trait-as-typeclass extension points

Rust traits here act like **typeclasses** indexed by Burn `Backend` `B`:

- **`IScienceCartridge<B>`** — New materials (steel, supercap, …) implement the trait for their `B`; generics `ManifoldGateway<B, C>`, `ThmcSolver::step`, and `TopologyPhysicsOrchestrator::run_plan_step` stay **monomorphized**, avoiding `dyn` over Burn kernels (`umst-manifold/src/ai/ppo.rs`, `umst-manifold/src/physics/solvers/thmc.rs`, `umst-manifold/src/physics/orchestration.rs`).
- **`PhysicsSolverZst`** — Extend with **new zero-sized solver tokens**, not trait objects; aligns with `Category-of-Material-Updates.md` Burn-safe tagging (`umst-manifold/src/physics/framework.rs`).
- **`PhysicsBackend`** — Blanket impl for `B: Backend<FloatElem = f32>` centralizes f32 stacks (`umst-manifold/src/physics/framework.rs`).

---

## Verification checklist (sibling tasks `fp-v04-2` … `fp-v04-6`)

Use this table to sign off cross-cutting invariants after sibling deliverables land. **Scope titles** align with reserved sibling filenames where noted; adjust titles to match your issue tracker if they differ.

| Task | Scope (expected focus) | Verification checklist |
|------|-------------------------|-------------------------|
| **fp-v04-2** | DEC / discrete exterior calculus morphisms (see `FP_CATEGORICAL_DEC.md`) | Operator layouts (`dec_primal`, `dec_operators`, topology incidence) consistent with UMST `edges_b1` / `faces_b2` docs; no duplicate incidence loops that contradict `TopologyPhysicsOrchestrator` ordering for coupled physics. |
| **fp-v04-3** | IO barrier / host–device contract (see `FP_CATEGORICAL_IO.md`) | `ManifoldGateway` docs in `ppo.rs`: tensor reductions on-device; deliberate `.into_scalar()` only at CBF “bits → host” boundary; solvers remain free of `std::fs` / ad hoc file I/O. |
| **fp-v04-4** | Burn / autodiff boundaries (see `FP_CATEGORICAL_BURN.md`) | `cargo clippy -p umst-manifold --features solver-experimental -- -D warnings` when touching Rust; implicit-step hooks (`ThmcSolver::_implicit_step`) remain opt-in behind `thmc-coupled`; no `dyn` Burn traits on hot paths per `PhysicsSolverZst` policy. |
| **fp-v04-5** | Orchestration invariants | Single delegation: full plan ticks go through `TopologyPhysicsOrchestrator::run_plan_step` or acknowledged exceptions documented next to call sites; `ThmcSolver::step` errors forwarded (no silent swallow when `thmc-coupled` off). |
| **fp-v04-6** | Golden-path / regression | `cargo test -p umst-manifold --features solver-experimental` (or project-standard test subset): `tests/golden_path_physics_cbf.rs`, `tests/verification/thmc_drying_shrinkage.rs` as relevant; cartridge smoke in `umst-concrete-cartridge` when pipeline changes. |

**Doc-only changes:** Clippy not required. **Rust changes:** use `--features solver-experimental` (and enable `thmc-coupled` when exercising `ThmcSolver::step` bodies).

---

## Status

| Item | State |
|------|--------|
| **`fp-v04-1-analysis`** (this file) | **Complete** — inventory and audit delivered. |
| **`maos-fp-categorical-v04` (epic)** | **Pending** until sibling tasks `fp-v04-2` … `fp-v04-6` and related docs ship. |

---

## Suggested commit message

```
docs(umst-manifold): add FP categorical hardening audit (fp-v04-1-analysis)

Add FP_CATEGORICAL_HARDENING_AUDIT.md: ontology, IScienceCartridge /
PhysicsSolverZst / TopologyPhysicsOrchestrator inventory with paths,
mutability vs pure-transform notes, typeclass extension points, and
fp-v04-2..6 verification checklist.
```

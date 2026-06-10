# Category of material updates (manifold SSOT)

**Epic:** `fp-categorical-v04` · **Task:** `fp-v04-traits-category`

Canonical categorical vocabulary for UMST manifold + cartridges. Cartridge-specific pipeline notes: [`umst-concrete-cartridge/docs/Category-of-Material-Updates.md`](../../umst-concrete-cartridge/docs/Category-of-Material-Updates.md).

## Objects

| Object | Rust type | Role |
|--------|-----------|------|
| Bulk mix state | [`MixTensor`](../src/core/tensors.rs) | Homogeneous cartridge input |
| Topology-carrying state | [`UnifiedMaterialStateTensor`](../src/core/tensors.rs) | UMST carrier (DEC + lanes) |
| THMC inner state | [`ThmcState`](../src/physics/solvers/thmc.rs) | Coupled tick state |

## Morphisms

| Morphism | Location | Composition |
|----------|----------|-------------|
| Cartridge law port | [`IScienceCartridge`](../src/core/traits.rs) | `compute_all` / `compute_topology` → [`PhysicalResult`](../src/core/traits.rs) |
| Topology plan step | [`TopologyPhysicsOrchestrator`](../src/physics/orchestration.rs) | Fold over [`TopologyPlanIntent`](../src/physics/orchestration.rs) |
| Gateway / PPO step | [`ManifoldGateway`](../src/ai/ppo.rs) | Policy → physics → CBF witness chain |
| Witness ladder | [`GOD_GRADE_WITNESS_LADDER.md`](GOD_GRADE_WITNESS_LADDER.md) | R1 CD → R2 Landauer → R3 constitutive → R4 Kleisli (short-circuit) |

## Sequential composition (THMC contract)

Documented order in [`orchestration.rs`](../src/physics/orchestration.rs):

1. Laplacian transport hints  
2. Chemistry (cartridge closures)  
3. Mechanics  
4. Fracture  
5. Rheology (optional, outside default tick when explicit)

## Second law hook

[`PhysicalResult`](../src/core/traits.rs) exposes `dissipation`, `free_energy`, `cost` for CBF and merge paths — constitutive closures must populate consistently with their numerical scheme.

## Functor / export (formal fiber)

Lean export → catalog digest → Rust `catalog_id` registry ([`FORMAL_BIDIRECTIONAL_ALIGNMENT.md`](FORMAL_BIDIRECTIONAL_ALIGNMENT.md)). Runtime does not replay tactics; witnesses are hand-aligned implementations.

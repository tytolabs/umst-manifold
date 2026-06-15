# Cartridge port (W9 Phase B)

## Hierarchy

```text
GateCartridge<B>           — universal admissibility / policy (no DEC required)
    ↑
IScienceCartridge<B>       — spatial physics: compute_all, compute_topology
    ↑ (alias)
SpatialCartridge<B>        — documentation subtyping marker
```

## Injection invariant

The kernel **never** constructs a domain cartridge by default. Callers pass `C: IScienceCartridge<B>` (or `SpatialCartridge`) into `ManifoldGateway`, orchestration, and THMC steps.

Gate HTTP paths (`transition_proposal`, `HttpTransitionEvaluator`) use host `f64` transition math today — they do **not** call the cartridge for scalar admissibility evidence. Tensor evidence flows through `compute_topology` → CBF only.

## Gate-only cartridges

Types implementing only [`GateCartridge`](../src/core/traits.rs) (see `tests/gate_cartridge_only_stub.rs`) may mount policy without claiming spatial physics.

## Cross-repo

Domain implementations live in cartridge repos (`umst-concrete-cartridge`, `umst-supercap-cartridge`). Manifold pins semver tags `v2.0.0-rc1` (Phase A) / `v2.0.0` (Phase B).

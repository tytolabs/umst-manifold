---
status: live
owner: tytolabs
last_verified: 2026-08-08
tags: [c2, tensor-algebra, burn, design-ssot]
---

# C2 Tensor Algebra Design (SSOT)

**Anchor:** `umst-cartridge-api/src/algebra.rs` (`TensorAlgebra` trait)  
**Adapter crate:** `crates/umst-algebra-burn/` (workspace path under `umst-manifold`; legacy alias `umst-runtime/crates/umst-algebra-burn/`)  
**Cited by:** `umst-manifold/src/runtime/atoms_tensor_lift_ops.rs:DESIGN_DOC_PATH`

## 1. Purpose

Cartridge atoms express constitutive laws **once**, generic over `A: TensorAlgebra`. The runtime edge supplies concrete algebra instances (`ScalarAlgebra`, `BurnAlgebra`). Atoms never import `burn` — they monomorphize only through the trait boundary.

This document is the **design SSOT** for how `BurnAlgebra` implements the six `TensorAlgebra` members and how rank-0 commutation with `ScalarAlgebra` is enforced.

## 2. The six trait members

| Member | `ScalarAlgebra` semantics | `BurnAlgebra` mapping |
|--------|-------------------------|----------------------|
| `type Field` | `f64` | Rank-0: `BurnRank0Field` (`#[repr(transparent)]` over `f64`, exact). Rank-1+: `burn::Tensor<B, D>` |
| `zero()` | `0.0` | Rank-0: `BurnRank0Field(0.0)`. Rank-1+: `Tensor::zeros(shape, device)` |
| `add(lhs, rhs)` | `lhs + rhs` | Rank-0: exact `f64` add. Rank-1+: elementwise tensor add |
| `mul(lhs, rhs)` | `lhs * rhs` | Rank-0: exact `f64` mul. Rank-1+: elementwise tensor mul |
| `contract(lhs, rhs)` | `lhs * rhs` (scalar) | Rank-0: exact `f64` product. Rank-1+: inner product per op-spec row |
| `grad(field)` | identity on scalar | Rank-0: identity. Rank-1+: spatial gradient per op-spec row |

**Law:** implementations must be **pure, total, referentially transparent** — no IO, no ambient state, no allocation-order dependence on the shipping surface.

## 3. ScalarAlgebra reference semantics

`ScalarAlgebra` (`umst-cartridge-api/src/algebra.rs:198`) is the **sole `Field = f64` reference** (R-ATOMS-SC-01):

- `field_to_f64` / `f64_to_field` are identity (`Some`)
- All ops are host `f64` arithmetic
- Used for: tests, fixtures, parity goldens, gate snapshots

Every `BurnAlgebra` rank-0 path must **commute** with this reference exactly.

## 4. Rank-0 commutation law

For every atom cartridge `C` and host probe inputs `x`:

```
∀ op ∈ {zero, add, mul, contract, grad}:
  project(BurnAlgebra::op(...)) == ScalarAlgebra::op(...)
```

Where `project` is `field_to_f64` for rank-0 fields.

**Exactness:** rank-0 equality is **exact** (`f64` bit-equality or exact rational). No `eps` at rank-0.

**Perturbation witness:** changing one input must change output — tests that cannot fail are tautological and rejected.

**Failure policy:** if rank-0 does not commute, **stop** — do not proceed to rank-1+ closures or Burn surface flips.

## 5. Rank-1+ lift and `eps`

Rank-1+ fields use `burn::Tensor<B, D>` per slice-3b THMC ledger and slice-3d op-spec rows (`atoms_tensor_lift_ops.rs`).

Comparisons against CON Burn goldens may use a declared `eps` with provenance (f32 cold-boundary cast, mesh tolerance, etc.) — never an unexplained inline literal.

`RANK1_PLUS_IMPL_LANDED` flips **only** when rank-1+ paths are measured against goldens with documented `eps`.

## 6. Where monomorphization happens

```
┌─────────────────────┐     generic      ┌──────────────────────┐
│  umst-cartridge-*   │  over A: Tensor  │  umst-cartridge-api  │
│  (atoms, materials) │ ───────────────► │  TensorAlgebra trait │
│  NO burn dep        │     Algebra      │  ScalarAlgebra ref   │
└─────────────────────┘                  └──────────────────────┘
                                                    │
                    runtime edge monomorphization   │
                                                    ▼
                              ┌─────────────────────────────────┐
                              │  umst-algebra-burn              │
                              │  impl TensorAlgebra for Burn    │
                              │  (ONLY lattice crate naming     │
                              │   burn in cartridge reorg)      │
                              └─────────────────────────────────┘
                                                    │
                                                    ▼
                              ┌─────────────────────────────────┐
                              │  umst-manifold / umst-runtime   │
                              │  executor, solvers, arena         │
                              │  (legacy burn home — pre-reorg) │
                              └─────────────────────────────────┘
```

- **Inside cartridges:** `cartridge.free_energy::<A>(state)` — `A` is a type parameter.
- **At runtime edge:** `umst-executor` (future) or integration tests monomorphize `A = BurnAlgebra`.
- **Never inside atoms:** `use burn::...` or `burn` in atom `Cargo.toml`.

## 7. Why no cartridge may name `burn`

Blueprint §4.1 / §6: atoms are backend-free. Direct `burn` deps would:

1. Break the tagless-final functor law (atoms would not be functors over algebras)
2. Prevent scalar-first development and gate parity
3. Duplicate the single adapter crate responsibility
4. Block honest `*_BURN_BACKEND_RUNTIME_DEFERRED` census (surfaces would invent partial closes)

Burn surfaces close by **monomorphization** after `umst-algebra-burn` exists — not by per-atom `burn` deps.

## 8. Slice ladder (honest posture)

| Slice | Location | Status |
|-------|----------|--------|
| slice-1 | `ScalarAlgebra` in `umst-cartridge-api` | **landed** |
| slice-2 | `BurnScalar` 0D prototype in `umst-cartridge-continuum/tensor_lift` | **landed** (prototype) |
| slice-3 | `BurnAtomAlgebra` 0D in `atoms_tensor_lift.rs` | **landed** (f32 tensor; superseded by algebra-burn rank-0) |
| slice-3b | THMC field ledger | **landed** (ledger only) |
| slice-3c | Adapter contract | **landed** (scaffold) |
| slice-3d | Tensor op spec | **landed** (spec only) |
| **adapter** | `umst-algebra-burn` | **R12-1 target** |

Parent residue: `R-faithful-decomp-B1` / `R-atoms-scalar` until production tensor path measured.

## 9. Adapter crate layout

```
umst-manifold/crates/umst-algebra-burn/
  Cargo.toml          # burn =0.13.2, burn-ndarray, umst-cartridge-api
  src/lib.rs          # pub mod rank0; pub mod tensor;
  src/rank0.rs        # BurnRank0Field, BurnRank0Algebra — exact f64
  src/tensor.rs       # BurnAlgebra<B>, rank-1+ Tensor ops
  tests/
    rank0_commutation.rs   # R12-2 law witness
    rank0_perturbation.rs  # non-tautological divergence
```

## 10. Non-claims

This design doc does **not** claim:

- `physics_green: true`
- `PRODUCTION_WIRED: true`
- `ADAPTER_CRATE_LANDED: true` (until crate compiles on disk)
- `RANK1_PLUS_IMPL_LANDED: true` (until measured)
- Any `*_BURN_BACKEND_RUNTIME_DEFERRED` flip without golden comparison

---

_One crate, six functions, one commutation law — then eighteen closures that mean something._

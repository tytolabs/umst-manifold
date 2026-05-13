# Fixed-point iteration drivers: which combinator?

This crate ships two small **bounded iteration** helpers that both use [`std::ops::ControlFlow`](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html) for early exit. They are **not** interchangeable at every call site: the choice is driven by **what state the step closure must hold and mutate**.

| Driver | Path | Step shape | Returns | Prefer when |
|--------|------|-------------|---------|-------------|
| **`iterate_until`** | `crate::core::iterate_until::iterate_until` | `FnMut(&mut S) -> ControlFlow<_, _>` | Count of **completed** iterations (`usize`) | The loop body needs a **single mutable carrier** `S` passed each time — e.g. a PCG workspace, scratch struct, or any state you want **explicitly threaded** through `step`. Natural fit for **tensor-heavy** inner solves where the body is already written as “update this `&mut` bundle” and you want parity with a `for` + `break` (see `src/physics/q1_hex_elasticity.rs`). |
| **`repeat_controlled`** | `crate::physics::solvers::fixed_point::repeat_controlled` | `FnMut() -> ControlFlow<_, _>` (no `state` argument) | `()` | **Scalar / small `Copy` / host `f64` math**, or when state is **closed over** by the closure (counters, a few locals). Handy for reference Newton / Picard-style snippets **without** threading a struct. **Avoid** as a blanket wrapper around Burn [`Tensor`](https://burn.dev/docs/tensor) inner loops where each step would need `clone()` or awkward moves — keep those as open `for` bodies or use `iterate_until` with a mutable workspace (see comment in `src/physics/solvers/electrochemistry.rs`). |

## Semantics (both)

- **`Continue`** — run another iteration if the cap is not reached.
- **`Break`** — stop **after** the current iteration (same “finish this iteration, then exit” feel as `break` at the end of a `for` body).

`iterate_until` additionally returns how many iterations completed, which is useful for logging, tests, and outer-loop logic.

## Inner CG and per-iteration `.into_scalar()` (Burn)

**Branch note (`gap-fp-inner-loop-syncs`):** choosing `iterate_until` vs `repeat_controlled` for an outer driver does **not** remove the need for **scalar reductions** inside a classical conjugate-gradient (CG) inner solve written on [Burn](https://burn.dev/) tensors. Textbook CG reads dot products, norms, and stability checks each iteration; the current production paths materialize those as **per-iteration** `.into_scalar()` (or equivalent) calls — that matches **classical CG on Burn** as implemented today, not a defect in the fixed-point combinator choice.

Optional future work (batched reductions, more fused device ops, deferred stopping) could lower host-sync frequency. Treat that as a **separate** design + review: profiling, allowlist/CI rationale updates if patterns move, and explicit numerical parity against the reference inner loop. **Documentation-only** edits under `gap-fp-inner-loop-syncs` assert **no intended behavior or convergence change**; any code change remains out of scope unless reviewed on its own.

For a hotspot-oriented audit of `into_scalar` / `into_data` in solver paths, see [`FP_CATEGORICAL_BURN.md`](./FP_CATEGORICAL_BURN.md).

## Related docs

- Narrow combinator note (API + tests pointer): [`FP_FIXED_POINT_COMBINATOR.md`](./FP_FIXED_POINT_COMBINATOR.md).
- Orchestrated material-update step (functor / fold language): [`Category-of-Material-Updates.md`](./Category-of-Material-Updates.md) and `src/physics/orchestration.rs`.
- Device sync / allowlist context for inner Krylov loops: [`FP_CATEGORICAL_BURN.md`](./FP_CATEGORICAL_BURN.md) (cross-links to this section).

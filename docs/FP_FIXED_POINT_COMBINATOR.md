# Fixed-point / iterate-until combinator

**When to use this vs `repeat_controlled`:** see [`FP_FIXED_POINT_CANONICAL.md`](./FP_FIXED_POINT_CANONICAL.md) (tensor / `&mut` state vs `Copy` / closed-over host state).

## Location

- **API:** `umst_manifold::core::iterate_until::iterate_until`
- **Source:** `src/core/iterate_until.rs`

## Role

`iterate_until` runs at most `max_iters` steps on a mutable `state`. Each step returns `std::ops::ControlFlow`: `Continue` runs another iteration (if any remain); `Break` stops after the current step, matching a `for` loop with an inner `break`.

## Use in this crate

The masked Q1-hex PCG driver in `src/physics/q1_hex_elasticity.rs` uses this combinator for the inner iteration (`hex_masked_pcg_one_iteration`), preserving the same tolerances and iteration cap as the previous explicit `for` loop.

## Tests

- `core::iterate_until::tests` — control-flow smoke checks.
- `physics::q1_hex_elasticity::hex_masked_pcg_iterate_parity_tests::hex_masked_pcg_iterate_until_matches_for_break_parity_deterministic` — bitwise `u` parity vs `for` + `break` on a tiny brick.

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! Bounded **iterate-until** / fixed-point style driver: run at most `max_iters` steps, stopping
//! early when the step closure returns [`ControlFlow::Break`].
//!
//! **Vs `physics::solvers::fixed_point::repeat_controlled`:** see `docs/FP_FIXED_POINT_CANONICAL.md`
//! (tensor / explicit `&mut` state vs scalar / `Copy` / closed-over host loops).

use core::ops::ControlFlow;

/// Run at most `max_iters` iterations of `step` on `state`.
///
/// Semantics match a plain `for _ in 0..max_iters { ... }` loop whose body ends with
/// `if should_stop { break; }` after completing the current iteration’s work:
/// - [`ControlFlow::Continue`] — proceed to the next iteration (if any remain).
/// - [`ControlFlow::Break`] — stop immediately **after** the current iteration (no further
///   iterations run).
///
/// Returns the number of **completed** iterations (from `1` to `max_iters` inclusive when
/// `max_iters > 0`, or `0` when `max_iters == 0`).
#[inline]
pub fn iterate_until<S>(
    max_iters: usize,
    state: &mut S,
    mut step: impl FnMut(&mut S) -> ControlFlow<(), ()>,
) -> usize {
    for i in 0..max_iters {
        if let ControlFlow::Break(()) = step(state) {
            return i + 1;
        }
    }
    max_iters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_until_zero_max_completes_zero_iterations() {
        let mut n = 0;
        let k = iterate_until(0, &mut n, |s| {
            *s += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(k, 0);
        assert_eq!(n, 0);
    }

    #[test]
    fn iterate_until_break_exits_after_current_iteration_matches_for_loop() {
        let max = 20;
        let mut x_for = 0_i32;
        for _ in 0..max {
            x_for += 1;
            if x_for >= 4 {
                break;
            }
        }
        let mut x_it = 0_i32;
        let k = iterate_until(max, &mut x_it, |v| {
            *v += 1;
            if *v >= 4 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        assert_eq!(x_for, x_it);
        assert_eq!(k, 4);
    }
}

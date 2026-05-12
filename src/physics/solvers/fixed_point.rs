// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! FP v0.4 pilot: **controlled iteration** helpers for solver loops (Picard, damped Newton on `Copy` state, etc.).
//!
//! **Canonical choice vs `core::iterate_until`:** see `docs/FP_FIXED_POINT_CANONICAL.md` — use this module for
//! scalar / small `Copy` / host closures; prefer [`crate::core::iterate_until::iterate_until`] when the step
//! should take `&mut S` (e.g. tensor workspaces, PCG state).
//!
//! [`repeat_controlled`] wraps a `for` with early exits via [`core::ops::ControlFlow`]. It adds **no**
//! per-iteration allocations beyond what the body already performs.
//!
//! **Burn tensors:** do not wrap inner Krylov / PCG loops that repeatedly move `Tensor` values unless
//! you accept extra `clone()`s each step — Rust `FnMut` closures cannot reassign owned tensor state the
//! same way an open `for` body can. Use this module for **scalar / small `Copy` structs / `&mut` buffers**
//! (host Newton corrections, Picard counters, reference solvers).

use core::ops::ControlFlow;

/// Run `body` at most `max_iters` times. Stop when `body` returns [`ControlFlow::Break`].
#[inline]
pub fn repeat_controlled(max_iters: usize, mut body: impl FnMut() -> ControlFlow<(), ()>) {
    for _ in 0..max_iters {
        if let ControlFlow::Break(()) = body() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Babylonian / Heron step for \(\sqrt{2}\): \(x_{k+1}=\tfrac12(x_k + 2/x_k)\) (quadratic convergence).
    #[test]
    fn repeat_controlled_heron_sqrt2() {
        let mut x = 1.0_f64;
        repeat_controlled(40, || {
            let nx = 0.5 * (x + 2.0 / x);
            if (nx - x).abs() < 1e-14 {
                return ControlFlow::Break(());
            }
            x = nx;
            ControlFlow::Continue(())
        });
        assert!((x - std::f64::consts::SQRT_2).abs() < 1e-12);
    }
}

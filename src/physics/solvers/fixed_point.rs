// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
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
//!
//! ## Honest fences (W29-074)
//!
//! - [`repeat_controlled`] is a **control-flow primitive** — not a solver convergence witness.
//! - [`FIXED_POINT_PHYSICS_GREEN`], [`FIXED_POINT_PRODUCTION_WIRED`], and [`FIXED_POINT_MASTER`] stay
//!   **false**; callers own physics / production posture.
//! - Early exit is [`ControlFlow::Break`] only; no implicit residual or tolerance checks here.

use core::ops::ControlFlow;

/// W29 deepen cell — fixed_point honest fence bundle.
pub const W29_FIXED_POINT_DEEPEN_CELL: &str = "W29-074-FIXED_POINT";

/// Honest physics posture — driver primitive; does not certify solver convergence.
pub const FIXED_POINT_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not claimed by the bounded driver alone.
pub const FIXED_POINT_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by the bounded driver alone.
pub const FIXED_POINT_MASTER: bool = false;

/// Operator-visible honesty string — does **not** authorize production flip or MASTER retick.
pub const FIXED_POINT_HONEST_FENCE: &str =
    "repeat_controlled_driver=true|control_flow_break=true|completed_count=true|copy_host_closures=true|convergence_witness=false|production_wired=false|physics_green=false|master=false";

/// Fence facet count for honest census.
pub const FIXED_POINT_FENCE_FACET_COUNT: usize = 8;

/// Fence facets wired today (4/8 measured).
pub const FIXED_POINT_FENCE_WIRED_COUNT: usize = 4;

/// One facet of the fixed_point production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// fixed_point production fence facet inventory (honest posture SSOT).
pub const FIXED_POINT_FENCE_FACETS: &[FixedPointFenceFacet] = &[
    FixedPointFenceFacet {
        facet: "bounded_repeat_controlled",
        wired: true,
        owning_slice: W29_FIXED_POINT_DEEPEN_CELL,
    },
    FixedPointFenceFacet {
        facet: "control_flow_break_semantics",
        wired: true,
        owning_slice: W29_FIXED_POINT_DEEPEN_CELL,
    },
    FixedPointFenceFacet {
        facet: "completed_iteration_count",
        wired: true,
        owning_slice: W29_FIXED_POINT_DEEPEN_CELL,
    },
    FixedPointFenceFacet {
        facet: "copy_host_closure_driver",
        wired: true,
        owning_slice: W29_FIXED_POINT_DEEPEN_CELL,
    },
    FixedPointFenceFacet {
        facet: "solver_convergence_witness",
        wired: false,
        owning_slice: "caller-owned (picard/newton residual)",
    },
    FixedPointFenceFacet {
        facet: "tensor_inner_loop_wrapper",
        wired: false,
        owning_slice: "refused — prefer iterate_until or open for",
    },
    FixedPointFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    FixedPointFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

const _: () = assert!(!FIXED_POINT_PHYSICS_GREEN);
const _: () = assert!(!FIXED_POINT_PRODUCTION_WIRED);
const _: () = assert!(!FIXED_POINT_MASTER);

/// Count wired fixed_point fence facets (must match [`FIXED_POINT_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn fixed_point_fence_wired_count() -> usize {
    FIXED_POINT_FENCE_FACETS.iter().filter(|f| f.wired).count()
}

/// Typed probe for fixed_point posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub bounded_driver: bool,
    pub control_flow_break: bool,
    pub completed_count: bool,
    pub copy_host_closures: bool,
    pub convergence_witness: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for fixed_point done-when checks.
#[must_use]
pub const fn fixed_point_probe() -> FixedPointProbe {
    FixedPointProbe {
        deepen_cell: W29_FIXED_POINT_DEEPEN_CELL,
        fence_facet_count: FIXED_POINT_FENCE_FACET_COUNT,
        fence_wired_count: FIXED_POINT_FENCE_WIRED_COUNT,
        bounded_driver: true,
        control_flow_break: true,
        completed_count: true,
        copy_host_closures: true,
        convergence_witness: false,
        production_wired: FIXED_POINT_PRODUCTION_WIRED,
        master: FIXED_POINT_MASTER,
        physics_green: FIXED_POINT_PHYSICS_GREEN,
        honest_fence: FIXED_POINT_HONEST_FENCE,
    }
}

/// fixed_point landed with production/master composition honestly open.
#[must_use]
pub fn fixed_point_honest(probe: &FixedPointProbe) -> bool {
    probe.deepen_cell == W29_FIXED_POINT_DEEPEN_CELL
        && probe.fence_facet_count == FIXED_POINT_FENCE_FACET_COUNT
        && probe.fence_wired_count == FIXED_POINT_FENCE_WIRED_COUNT
        && probe.bounded_driver
        && probe.control_flow_break
        && probe.completed_count
        && probe.copy_host_closures
        && !probe.convergence_witness
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate fixed_point honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_fixed_point_honesty() -> Result<(), &'static str> {
    let probe = fixed_point_probe();
    if probe.production_wired {
        return Err("FIXED_POINT_PRODUCTION_WIRED must stay false — driver primitive only");
    }
    if probe.master {
        return Err("FIXED_POINT_MASTER must stay false until orchestrator pin lands");
    }
    if probe.physics_green {
        return Err("FIXED_POINT_PHYSICS_GREEN must stay false — no convergence witness");
    }
    if probe.convergence_witness {
        return Err("fixed_point must not claim solver convergence witness");
    }
    if fixed_point_fence_wired_count() != FIXED_POINT_FENCE_WIRED_COUNT {
        return Err("fixed_point_fence_wired_count drifted from FIXED_POINT_FENCE_WIRED_COUNT");
    }
    if !fixed_point_honest(&probe) {
        return Err("fixed_point_probe failed fixed_point_honest gate");
    }
    Ok(())
}

/// Run `body` at most `max_iters` times. Stop when `body` returns [`ControlFlow::Break`].
///
/// Semantics match a plain `for _ in 0..max_iters { ... }` loop whose body ends with
/// `if should_stop { break; }` after completing the current iteration's work:
/// - [`ControlFlow::Continue`] — proceed to the next iteration (if any remain).
/// - [`ControlFlow::Break`] — stop immediately **after** the current iteration (no further
///   iterations run).
///
/// Returns the number of **completed** iterations (from `1` to `max_iters` inclusive when
/// `max_iters > 0`, or `0` when `max_iters == 0`).
#[inline]
pub fn repeat_controlled(max_iters: usize, mut body: impl FnMut() -> ControlFlow<(), ()>) -> usize {
    for i in 0..max_iters {
        if let ControlFlow::Break(()) = body() {
            return i + 1;
        }
    }
    max_iters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_honest_fence_bundle() {
        validate_fixed_point_honesty().expect("honest fence");
        let probe = fixed_point_probe();
        assert!(fixed_point_honest(&probe));
        assert_eq!(
            fixed_point_fence_wired_count(),
            FIXED_POINT_FENCE_WIRED_COUNT
        );
        assert!(!FIXED_POINT_PHYSICS_GREEN);
        assert!(!FIXED_POINT_PRODUCTION_WIRED);
        assert!(!FIXED_POINT_MASTER);
        assert_eq!(W29_FIXED_POINT_DEEPEN_CELL, "W29-074-FIXED_POINT");
        assert_eq!(FIXED_POINT_FENCE_FACET_COUNT, 8);
        assert_eq!(FIXED_POINT_FENCE_WIRED_COUNT, 4);
        assert!(FIXED_POINT_HONEST_FENCE.contains("convergence_witness=false"));
    }

    /// Babylonian / Heron step for \(\sqrt{2}\): \(x_{k+1}=\tfrac12(x_k + 2/x_k)\) (quadratic convergence).
    #[test]
    fn repeat_controlled_heron_sqrt2() {
        let mut x = 1.0_f64;
        let k = repeat_controlled(40, || {
            let nx = 0.5 * (x + 2.0 / x);
            if (nx - x).abs() < 1e-14 {
                return ControlFlow::Break(());
            }
            x = nx;
            ControlFlow::Continue(())
        });
        assert!((x - std::f64::consts::SQRT_2).abs() < 1e-12);
        assert!(k >= 1 && k <= 40);
    }

    #[test]
    fn repeat_controlled_zero_max_completes_zero_iterations() {
        let mut n = 0;
        let k = repeat_controlled(0, || {
            n += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(k, 0);
        assert_eq!(n, 0);
    }

    #[test]
    fn repeat_controlled_break_exits_after_current_iteration_matches_for_loop() {
        let max = 20;
        let mut x_for = 0_i32;
        for _ in 0..max {
            x_for += 1;
            if x_for >= 4 {
                break;
            }
        }
        let mut x_rc = 0_i32;
        let k = repeat_controlled(max, || {
            x_rc += 1;
            if x_rc >= 4 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        assert_eq!(x_for, x_rc);
        assert_eq!(k, 4);
    }

    #[test]
    fn repeat_controlled_continue_runs_all_iterations() {
        let max = 5;
        let mut acc = 0_u32;
        let k = repeat_controlled(max, || {
            acc += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(k, max);
        assert_eq!(acc, max as u32);
    }

    #[test]
    fn repeat_controlled_break_on_first_iteration_returns_one() {
        let mut n = 0;
        let k = repeat_controlled(10, || {
            n += 1;
            ControlFlow::Break(())
        });
        assert_eq!(k, 1);
        assert_eq!(n, 1);
    }

    #[test]
    fn fixed_point_refuses_fake_green_production_master() {
        let probe = fixed_point_probe();
        assert!(!probe.physics_green);
        assert!(!probe.production_wired);
        assert!(!probe.master);
        assert!(!probe.convergence_witness);
        assert!(probe.honest_fence.contains("physics_green=false"));
        assert!(probe.honest_fence.contains("production_wired=false"));
        assert!(probe.honest_fence.contains("master=false"));
    }
}

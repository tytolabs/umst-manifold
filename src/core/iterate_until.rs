// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Bounded **iterate-until** / fixed-point style driver: run at most `max_iters` steps, stopping
//! early when the step closure returns [`ControlFlow::Break`].
//!
//! **Vs `physics::solvers::fixed_point::repeat_controlled`:** see `docs/FP_FIXED_POINT_CANONICAL.md`
//! (tensor / explicit `&mut` state vs scalar / `Copy` / closed-over host loops).
//!
//! ## Honest fences (W29-026)
//!
//! - [`iterate_until`] is a **control-flow primitive** — not a solver convergence witness.
//! - [`ITERATE_UNTIL_PHYSICS_GREEN`], [`ITERATE_UNTIL_PRODUCTION_WIRED`], and
//!   [`ITERATE_UNTIL_MASTER`] stay **false**; callers own physics / production posture.
//! - Early exit is [`ControlFlow::Break`] only; no implicit residual or tolerance checks here.

use core::ops::ControlFlow;

/// W29 deepen cell — iterate_until honest fence bundle.
pub const W29_ITERATE_UNTIL_DEEPEN_CELL: &str = "W29-026-ITERATE_UNTIL";

/// Honest physics posture — driver primitive; does not certify solver convergence.
pub const ITERATE_UNTIL_PHYSICS_GREEN: bool = false;

/// Production orchestration pin — not claimed by the bounded driver alone.
pub const ITERATE_UNTIL_PRODUCTION_WIRED: bool = false;

/// Master composition pin — not claimed by the bounded driver alone.
pub const ITERATE_UNTIL_MASTER: bool = false;

/// Operator-visible honesty string — does **not** authorize production flip or MASTER retick.
pub const ITERATE_UNTIL_HONEST_FENCE: &str =
    "iterate_until_driver=true|control_flow_break=true|completed_count=true|convergence_witness=false|production_wired=false|physics_green=false|master=false";

/// Fence facet count for honest census.
pub const ITERATE_UNTIL_FENCE_FACET_COUNT: usize = 7;

/// Fence facets wired today (4/7 measured).
pub const ITERATE_UNTIL_FENCE_WIRED_COUNT: usize = 4;

/// One facet of the iterate_until production fence matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IterateUntilFenceFacet {
    pub facet: &'static str,
    pub wired: bool,
    pub owning_slice: &'static str,
}

/// iterate_until production fence facet inventory (honest posture SSOT).
pub const ITERATE_UNTIL_FENCE_FACETS: &[IterateUntilFenceFacet] = &[
    IterateUntilFenceFacet {
        facet: "bounded_driver",
        wired: true,
        owning_slice: W29_ITERATE_UNTIL_DEEPEN_CELL,
    },
    IterateUntilFenceFacet {
        facet: "control_flow_break_semantics",
        wired: true,
        owning_slice: W29_ITERATE_UNTIL_DEEPEN_CELL,
    },
    IterateUntilFenceFacet {
        facet: "completed_iteration_count",
        wired: true,
        owning_slice: W29_ITERATE_UNTIL_DEEPEN_CELL,
    },
    IterateUntilFenceFacet {
        facet: "mut_state_step_closure",
        wired: true,
        owning_slice: W29_ITERATE_UNTIL_DEEPEN_CELL,
    },
    IterateUntilFenceFacet {
        facet: "solver_convergence_witness",
        wired: false,
        owning_slice: "caller-owned (acoustics/fracture/rheology)",
    },
    IterateUntilFenceFacet {
        facet: "production_wired",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
    IterateUntilFenceFacet {
        facet: "master_orchestrator_pin",
        wired: false,
        owning_slice: "deferred-orchestrator-pin",
    },
];

const _: () = assert!(!ITERATE_UNTIL_PHYSICS_GREEN);
const _: () = assert!(!ITERATE_UNTIL_PRODUCTION_WIRED);
const _: () = assert!(!ITERATE_UNTIL_MASTER);

/// Count wired iterate_until fence facets (must match [`ITERATE_UNTIL_FENCE_WIRED_COUNT`]).
#[must_use]
pub fn iterate_until_fence_wired_count() -> usize {
    ITERATE_UNTIL_FENCE_FACETS
        .iter()
        .filter(|f| f.wired)
        .count()
}

/// Typed probe for iterate_until posture honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IterateUntilProbe {
    pub deepen_cell: &'static str,
    pub fence_facet_count: usize,
    pub fence_wired_count: usize,
    pub bounded_driver: bool,
    pub control_flow_break: bool,
    pub completed_count: bool,
    pub convergence_witness: bool,
    pub production_wired: bool,
    pub master: bool,
    pub physics_green: bool,
    pub honest_fence: &'static str,
}

/// Build introspection probe for iterate_until done-when checks.
#[must_use]
pub const fn iterate_until_probe() -> IterateUntilProbe {
    IterateUntilProbe {
        deepen_cell: W29_ITERATE_UNTIL_DEEPEN_CELL,
        fence_facet_count: ITERATE_UNTIL_FENCE_FACET_COUNT,
        fence_wired_count: ITERATE_UNTIL_FENCE_WIRED_COUNT,
        bounded_driver: true,
        control_flow_break: true,
        completed_count: true,
        convergence_witness: false,
        production_wired: ITERATE_UNTIL_PRODUCTION_WIRED,
        master: ITERATE_UNTIL_MASTER,
        physics_green: ITERATE_UNTIL_PHYSICS_GREEN,
        honest_fence: ITERATE_UNTIL_HONEST_FENCE,
    }
}

/// iterate_until landed with production/master composition honestly open.
#[must_use]
pub fn iterate_until_honest(probe: &IterateUntilProbe) -> bool {
    probe.deepen_cell == W29_ITERATE_UNTIL_DEEPEN_CELL
        && probe.fence_facet_count == ITERATE_UNTIL_FENCE_FACET_COUNT
        && probe.fence_wired_count == ITERATE_UNTIL_FENCE_WIRED_COUNT
        && probe.bounded_driver
        && probe.control_flow_break
        && probe.completed_count
        && !probe.convergence_witness
        && !probe.production_wired
        && !probe.master
        && !probe.physics_green
        && probe.honest_fence.contains("production_wired=false")
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("master=false")
}

/// Validate iterate_until honesty — fail closed on fake production/master/GREEN claims.
pub fn validate_iterate_until_honesty() -> Result<(), &'static str> {
    let probe = iterate_until_probe();
    if probe.production_wired {
        return Err("ITERATE_UNTIL_PRODUCTION_WIRED must stay false — driver primitive only");
    }
    if probe.master {
        return Err("ITERATE_UNTIL_MASTER must stay false until orchestrator pin lands");
    }
    if probe.physics_green {
        return Err("ITERATE_UNTIL_PHYSICS_GREEN must stay false — no convergence witness");
    }
    if probe.convergence_witness {
        return Err("iterate_until must not claim solver convergence witness");
    }
    if iterate_until_fence_wired_count() != ITERATE_UNTIL_FENCE_WIRED_COUNT {
        return Err("iterate_until_fence_wired_count drifted from ITERATE_UNTIL_FENCE_WIRED_COUNT");
    }
    if !iterate_until_honest(&probe) {
        return Err("iterate_until_probe failed iterate_until_honest gate");
    }
    Ok(())
}

/// Run at most `max_iters` iterations of `step` on `state`.
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
    fn iterate_until_honest_fence_bundle() {
        validate_iterate_until_honesty().expect("honest fence");
        let probe = iterate_until_probe();
        assert!(iterate_until_honest(&probe));
        assert_eq!(
            iterate_until_fence_wired_count(),
            ITERATE_UNTIL_FENCE_WIRED_COUNT
        );
        assert!(!ITERATE_UNTIL_PHYSICS_GREEN);
        assert!(!ITERATE_UNTIL_PRODUCTION_WIRED);
        assert!(!ITERATE_UNTIL_MASTER);
    }

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

    #[test]
    fn iterate_until_continue_runs_all_iterations() {
        let max = 5;
        let mut acc = 0_u32;
        let k = iterate_until(max, &mut acc, |s| {
            *s += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(k, max);
        assert_eq!(acc, max as u32);
    }

    #[test]
    fn iterate_until_break_on_first_iteration_returns_one() {
        let mut n = 0;
        let k = iterate_until(10, &mut n, |s| {
            *s += 1;
            ControlFlow::Break(())
        });
        assert_eq!(k, 1);
        assert_eq!(n, 1);
    }
}

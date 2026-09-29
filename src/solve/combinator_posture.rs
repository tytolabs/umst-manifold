// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Honest posture for manifold Track C solve combinator integration.

/// Deepen cell — manifold `src/solve` combinator family.
pub const MANIFOLD_SOLVE_COMBINATOR_DEEPEN_CELL: &str = "TRACK-C-MANIFOLD-SOLVE-COMBINATOR";

/// Does not certify fleet physics GREEN.
pub const MANIFOLD_SOLVE_COMBINATOR_PHYSICS_GREEN: bool = false;

/// Production wiring — physics sites not fully adopted.
pub const MANIFOLD_SOLVE_COMBINATOR_PRODUCTION_WIRED: bool = false;

/// Master / OP-5 — refused at this integration layer.
pub const MANIFOLD_SOLVE_COMBINATOR_MASTER: bool = false;

/// Honest fence for meta probes.
pub const MANIFOLD_SOLVE_COMBINATOR_HONEST_FENCE: &str =
    "unfold_callers_landed=true|physics_sites_adopted=false|production_wired=false|physics_green=false|master=false";

const _: () = assert!(!MANIFOLD_SOLVE_COMBINATOR_PHYSICS_GREEN);
const _: () = assert!(!MANIFOLD_SOLVE_COMBINATOR_PRODUCTION_WIRED);
const _: () = assert!(!MANIFOLD_SOLVE_COMBINATOR_MASTER);

/// Typed probe for manifold solve combinator posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifoldSolveCombinatorProbe {
    pub deepen_cell: &'static str,
    pub physics_green: bool,
    pub production_wired: bool,
    pub master: bool,
    pub unfold_callers_landed: bool,
    pub physics_sites_adopted: bool,
    pub honest_fence: &'static str,
}

#[must_use]
pub const fn manifold_solve_combinator_probe() -> ManifoldSolveCombinatorProbe {
    ManifoldSolveCombinatorProbe {
        deepen_cell: MANIFOLD_SOLVE_COMBINATOR_DEEPEN_CELL,
        physics_green: MANIFOLD_SOLVE_COMBINATOR_PHYSICS_GREEN,
        production_wired: MANIFOLD_SOLVE_COMBINATOR_PRODUCTION_WIRED,
        master: MANIFOLD_SOLVE_COMBINATOR_MASTER,
        unfold_callers_landed: true,
        physics_sites_adopted: false,
        honest_fence: MANIFOLD_SOLVE_COMBINATOR_HONEST_FENCE,
    }
}

#[must_use]
pub fn manifold_solve_combinator_honest(probe: &ManifoldSolveCombinatorProbe) -> bool {
    !probe.physics_green
        && !probe.production_wired
        && !probe.master
        && probe.unfold_callers_landed
        && !probe.physics_sites_adopted
        && probe.deepen_cell == MANIFOLD_SOLVE_COMBINATOR_DEEPEN_CELL
        && probe.honest_fence.contains("physics_green=false")
        && probe.honest_fence.contains("production_wired=false")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifold_solve_combinator_honest_fence() {
        let probe = manifold_solve_combinator_probe();
        assert!(manifold_solve_combinator_honest(&probe));
        assert!(!MANIFOLD_SOLVE_COMBINATOR_PHYSICS_GREEN);
    }
}

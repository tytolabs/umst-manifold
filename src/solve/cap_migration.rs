// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Caps → unfold migration ledger (manifold Track C). One site per cell until census hits zero.

/// How a legacy compiled iteration cap is dispositioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapMigrationDisposition {
    /// Manifold `src/solve` exposes an unfold caller; physics call site still open.
    UnfoldCallerLanded {
        /// Public wrapper in this crate.
        symbol: &'static str,
    },
    /// Cannot route through unfold without a meter / budget witness (typed absence).
    HonestTypedAbsence {
        /// Why unfold is refused on this path.
        reason: &'static str,
    },
}

/// One capped loop site in the migration antichain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapMigrationSite {
    /// Legacy module path (audit only).
    pub legacy_module: &'static str,
    /// Legacy cap token (audit only — not a `max_iter: N` literal in this row).
    pub legacy_token: &'static str,
    pub disposition: CapMigrationDisposition,
}

/// Track C migration inventory (monotone: only grows or flips disposition).
pub const CAP_MIGRATION_SITES: &[CapMigrationSite] = &[
    CapMigrationSite {
        legacy_module: "physics/solvers/fixed_point.rs",
        legacy_token: "repeat_controlled(max_iters)",
        disposition: CapMigrationDisposition::UnfoldCallerLanded {
            symbol: "scalar_residual_unfold",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/krylov_host.rs",
        legacy_token: "gmres_f32(max_iter as restart)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "host GMRES needs tensor workspace + live package-power meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/acoustics.rs",
        legacy_token: "AcousticGmresConfig.max_iter (registry budget)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "implicit Newmark bar-network GMRES needs device workspace meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/thmc.rs",
        legacy_token: "ThmcNewtonConfig.max_iterations (registry budget)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "coupled THMC monolithic Newton needs operator-split energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/electrochemistry.rs",
        legacy_token: "gmres_f32_try(dim + 120).min(512)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "electrochemistry GMRES cap is coupled to sparse dim; needs basis_budget meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/photonics.rs",
        legacy_token: "inner.max_cg_iterations (Helmholtz / CG loops)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "photonics CG caps need SPD operator energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/rheology_flow.rs",
        legacy_token: "chorin_poisson_max_iters_from_budget (Jacobi-PCG byte budget)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "Chorin Poisson PCG iteration cap is byte-budgeted on lazy tensor graph; needs basis_budget meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/fracture_field.rs",
        legacy_token: "JACOBI_SWEEPS (spectral eigenvalue clamp on strain tensor)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "AT2 spectral Jacobi sweeps need fracture energy unfold meter before debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/thmc_jfnk.rs",
        legacy_token: "gmres_f32_try (restart bounded by problem dim for JFNK inner)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "matrix-free THMC JFNK inner GMRES needs coupled residual energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/mechanics.rs",
        legacy_token: "bar_network_pcg_f64_single_batch (pcg_iters vs n_unknowns=3*n_v)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "bar-network projected PCG hard-stops at n_unknowns sweep budget; needs mechanics strain-energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/adjoint_q1_hex.rs",
        legacy_token: "opts.pcg_max_iter / cg.iteration_budget(n_dof) masked hex PCG",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "Q1 hex adjoint PCG hard-stops at iteration_budget; needs topology-gradient energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/extruded_plate.rs",
        legacy_token: "cg_config.iteration_budget(n * 3) extruded hex PCG",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "extruded plate masked hex PCG hard-stops at iteration_budget; needs plate strain-energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/topology_filter.rs",
        legacy_token: "HelmholtzTopologyFilter.max_cg_iterations (Richardson stationary)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "topology-density Helmholtz Richardson cap is historical CG naming; needs filter energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/hex_elasticity.rs",
        legacy_token: "hex_solve_pcg_masked(..., max_iter, ...) masked structured PCG",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "hex masked structured PCG hard-stops at max_iter; needs hex elasticity strain-energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/time_orchestration.rs",
        legacy_token: "SimulationClocks.max_mech_sub_iters_per_chem (chem→mech substep cap)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "coupled chem–mech clock substep cap binds before staggered energy meter; needs unfold debit witness before cap removal",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/thmc_residual.rs",
        legacy_token: "gmres_f32_try(m_a.saturating_add(12)) quasi-static stacked residual inner",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "THMC reduced quasi-static inner GMRES cap is matrix-free FD sized; needs coupled stack residual energy meter before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "core/iterate_until.rs",
        legacy_token: "iterate_until(max_iters, &mut state, step_closure)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "shared bounded driver backs acoustics Newmark, rheology Jacobi, fracture outer loops; needs per-physics unfold debit meter before cap removal",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solve_budget.rs",
        legacy_token: "pcg_max_iter / max_cg_iterations overlay (cockpit budget lane)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "cockpit PCG cap overlays registry max_cg_iterations before hex/adjoint solves; needs unified energy-budget witness before unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/mechanics_solve_port.rs",
        legacy_token: "MechanicsInnerLoopConfig.max_cg_iterations (bar port integration fixture)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "mechanics solve port bar-network fixture hard-codes max_cg_iterations before port-level energy meter; needs unfold debit witness before cap removal",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/adjoint.rs",
        legacy_token: "MechanicsInnerLoopConfig.max_cg_iterations: 500 (topology adjoint PCG lane)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "topology adjoint masked PCG hard-stops at max_cg_iterations before adjoint energy meter; needs unfold debit witness before cap removal",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/acoustics.rs",
        legacy_token: "AcousticNewmarkBar1dPeriodic.run_to_energy_tol(max_steps, energy_tol, …)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "Newmark energy-relaxation loop caps at max_steps before acoustic mechanical-energy meter ties to unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/fracture_field.rs",
        legacy_token: "iterate_until(outer.max_outer_iterations, …) AT2 staggered outer loop",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "AT2 phase-field outer stagger caps at max_outer_iterations before fracture release-energy meter ties to unfold debit",
        },
    },
    CapMigrationSite {
        legacy_module: "physics/solvers/fracture_field.rs",
        legacy_token: "iterate_until(config.outer_iters, …) staggered phase-field coupled mechanics",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "staggered phase-field coupled mechanics outer loop caps at outer_iters before coupled fracture–mechanics energy meter ties to unfold debit",
        },
    },
];

#[must_use]
pub const fn cap_migration_site_count() -> usize {
    CAP_MIGRATION_SITES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_migration_inventory_honest() {
        assert_eq!(cap_migration_site_count(), 23);
        let fixed = &CAP_MIGRATION_SITES[0];
        assert_eq!(fixed.legacy_module, "physics/solvers/fixed_point.rs");
        assert!(matches!(
            fixed.disposition,
            CapMigrationDisposition::UnfoldCallerLanded { .. }
        ));
        let gmres = &CAP_MIGRATION_SITES[1];
        assert!(matches!(
            gmres.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let thmc = &CAP_MIGRATION_SITES[3];
        assert_eq!(thmc.legacy_module, "physics/solvers/thmc.rs");
        assert!(matches!(
            thmc.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let echem = &CAP_MIGRATION_SITES[4];
        assert_eq!(echem.legacy_module, "physics/solvers/electrochemistry.rs");
        let photonics = &CAP_MIGRATION_SITES[5];
        assert_eq!(photonics.legacy_module, "physics/solvers/photonics.rs");
        let rheology = &CAP_MIGRATION_SITES[6];
        assert_eq!(rheology.legacy_module, "physics/solvers/rheology_flow.rs");
        let fracture = &CAP_MIGRATION_SITES[7];
        assert_eq!(fracture.legacy_module, "physics/solvers/fracture_field.rs");
        let jfnk = &CAP_MIGRATION_SITES[8];
        assert_eq!(jfnk.legacy_module, "physics/solvers/thmc_jfnk.rs");
        let mechanics = &CAP_MIGRATION_SITES[9];
        assert_eq!(mechanics.legacy_module, "physics/mechanics.rs");
        assert!(matches!(
            mechanics.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let adjoint_hex = &CAP_MIGRATION_SITES[10];
        assert_eq!(adjoint_hex.legacy_module, "physics/adjoint_q1_hex.rs");
        assert!(matches!(
            adjoint_hex.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let extruded = &CAP_MIGRATION_SITES[11];
        assert_eq!(extruded.legacy_module, "physics/extruded_plate.rs");
        assert!(matches!(
            extruded.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let topo_filter = &CAP_MIGRATION_SITES[12];
        assert_eq!(topo_filter.legacy_module, "physics/topology_filter.rs");
        assert!(matches!(
            topo_filter.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let hex_pcg = &CAP_MIGRATION_SITES[13];
        assert_eq!(hex_pcg.legacy_module, "physics/hex_elasticity.rs");
        assert!(matches!(
            hex_pcg.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let clocks = &CAP_MIGRATION_SITES[14];
        assert_eq!(clocks.legacy_module, "physics/time_orchestration.rs");
        assert!(matches!(
            clocks.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let thmc_res = &CAP_MIGRATION_SITES[15];
        assert_eq!(thmc_res.legacy_module, "physics/solvers/thmc_residual.rs");
        assert!(matches!(
            thmc_res.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let iterate_until = &CAP_MIGRATION_SITES[16];
        assert_eq!(iterate_until.legacy_module, "core/iterate_until.rs");
        assert!(matches!(
            iterate_until.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let solve_budget = &CAP_MIGRATION_SITES[17];
        assert_eq!(solve_budget.legacy_module, "physics/solve_budget.rs");
        assert!(matches!(
            solve_budget.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let mechanics_port = &CAP_MIGRATION_SITES[18];
        assert_eq!(mechanics_port.legacy_module, "physics/mechanics_solve_port.rs");
        assert!(matches!(
            mechanics_port.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let adjoint = &CAP_MIGRATION_SITES[19];
        assert_eq!(adjoint.legacy_module, "physics/adjoint.rs");
        assert!(matches!(
            adjoint.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let acoustics = &CAP_MIGRATION_SITES[20];
        assert_eq!(acoustics.legacy_module, "physics/solvers/acoustics.rs");
        assert!(matches!(
            acoustics.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let fracture_outer = &CAP_MIGRATION_SITES[21];
        assert_eq!(fracture_outer.legacy_module, "physics/solvers/fracture_field.rs");
        assert!(matches!(
            fracture_outer.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
        let fracture_coupled = &CAP_MIGRATION_SITES[22];
        assert_eq!(fracture_coupled.legacy_module, "physics/solvers/fracture_field.rs");
        assert!(matches!(
            fracture_coupled.disposition,
            CapMigrationDisposition::HonestTypedAbsence { .. }
        ));
    }
}

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
        assert_eq!(cap_migration_site_count(), 10);
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
    }
}

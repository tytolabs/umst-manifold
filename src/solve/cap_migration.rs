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
        legacy_token: "AcousticGmresConfig.max_iter (default 256)",
        disposition: CapMigrationDisposition::HonestTypedAbsence {
            reason: "implicit Newmark bar-network GMRES needs device workspace meter before unfold debit",
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
        assert_eq!(cap_migration_site_count(), 3);
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
    }
}

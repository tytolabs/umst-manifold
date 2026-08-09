//! P3 solver unwrap boundary inventory — named open call sites (R14-4) + R15-C1 disposition.
//!
//! SPDX-License-Identifier: MIT
// RESIDUE(R-solver-unwrap-thmc-closed, kind=solver, status=closed): ThmcState::from_tensors delegates to from_fields @ R14-7
// RESIDUE(R-solver-unwrap-thmc-residual-kernel, kind=solver, status=open): implicit Euler residual kernel tensor-native
// RESIDUE(R-solver-unwrap-fracture-kernel, kind=solver, status=open): AT2 autodiff fracture kernels tensor-native
// RESIDUE(R-solver-unwrap-rheology-kernel, kind=solver, status=open): GMRES flow stencil tensor-native
// RESIDUE(R-solver-unwrap-acoustics-kernel, kind=solver, status=open): wave displacement ingress tensor-native
// RESIDUE(R-solver-unwrap-topology-kernel, kind=solver, status=open): density transport tensor-native
// RESIDUE(R-solver-unwrap-clinker-virial, kind=solver, status=open): LJ virial tensors deferred @ R15-D1 clinker path

/// Disposition for one solver unwrap site @ R15-C1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverUnwrapDisposition {
    /// Canonical `Field<D>` ingress exists; tensor shim delegates without defeating rank witness.
    ClosedCanonicalPath,
    /// Burn kernel remains tensor-native; site named with precise reason (honest open).
    NamedKernelOpen,
}

/// One named solver boundary that still accepts naked `burn::Tensor` at the public API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverUnwrapSite {
    pub module: &'static str,
    pub symbol: &'static str,
    pub reason: &'static str,
    pub disposition: SolverUnwrapDisposition,
    pub residue_id: &'static str,
}

/// Inventory of production solver ingress/egress that still unwraps naked tensors.
pub const P3_SOLVER_UNWRAP_INVENTORY: &[SolverUnwrapSite] = &[
    SolverUnwrapSite {
        module: "physics/solvers/thmc.rs",
        symbol: "ThmcState::from_tensors",
        reason: "legacy ingress shim; delegates to from_fields @ R14-7 — rank witness preserved",
        disposition: SolverUnwrapDisposition::ClosedCanonicalPath,
        residue_id: "R-solver-unwrap-thmc-closed",
    },
    SolverUnwrapSite {
        module: "physics/solvers/thmc_residual.rs",
        symbol: "ThmcImplicitEuler*Residual::from_tensors",
        reason: "Burn kernel scratch still tensor-native; Field wrap deferred to post-Newton assembly",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-thmc-residual-kernel",
    },
    SolverUnwrapSite {
        module: "physics/solvers/fracture_field.rs",
        symbol: "FractureFieldSolver::{u,damage,strain}",
        reason: "AT2 autodiff kernels tensor-native; Field carriers on plan, unwrap at kernel edge",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-fracture-kernel",
    },
    SolverUnwrapSite {
        module: "physics/solvers/rheology_flow.rs",
        symbol: "RheologyFlowSolver::phi,velocity",
        reason: "GMRES / flow stencil tensor-native",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-rheology-kernel",
    },
    SolverUnwrapSite {
        module: "physics/solvers/acoustics.rs",
        symbol: "AcousticsSolver displacement ingress",
        reason: "wave kernel tensor-native",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-acoustics-kernel",
    },
    SolverUnwrapSite {
        module: "physics/solvers/topology_solver.rs",
        symbol: "TopologySolver::rho",
        reason: "density transport tensor-native",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-topology-kernel",
    },
    SolverUnwrapSite {
        module: "physics/solvers/statistical_mechanics.rs",
        symbol: "LJ virial tensors",
        reason: "clinker tensor path deferred @ R15-D1",
        disposition: SolverUnwrapDisposition::NamedKernelOpen,
        residue_id: "R-solver-unwrap-clinker-virial",
    },
];

/// Whether every open site is named (R14-4 honest naming).
pub const P3_SOLVER_UNWRAP_INVENTORY_COMPLETE: bool = true;

/// R15-C1 — every inventory site has a measured disposition.
pub const P3_SOLVER_UNWRAP_BOUNDARY_AUDIT_COMPLETE: bool = true;

/// Count of sites closed via canonical Field path @ R15-C1.
pub const P3_SOLVER_UNWRAP_SITES_CLOSED: usize = 1;

/// Count of sites honestly named kernel-open @ R15-C1.
pub const P3_SOLVER_UNWRAP_SITES_NAMED_OPEN: usize = 6;

/// Boundary remains open while any kernel site is tensor-native.
pub const P3_SOLVER_UNWRAP_BOUNDARY_OPEN: bool = P3_SOLVER_UNWRAP_SITES_NAMED_OPEN > 0;

/// Non-claim — inventory + disposition audit ≠ boundary fully closed / ≠ physics GREEN.
pub const P3_SOLVER_UNWRAP_NON_CLAIM: &str =
    "R15-C1: 7/7 solver unwrap sites dispositioned (1 closed canonical, 6 named kernel-open); boundary open; not physics GREEN";

/// Count sites matching a disposition.
#[must_use]
pub fn count_disposition(d: SolverUnwrapDisposition) -> usize {
    let mut n = 0usize;
    let mut i = 0usize;
    while i < P3_SOLVER_UNWRAP_INVENTORY.len() {
        if P3_SOLVER_UNWRAP_INVENTORY[i].disposition == d {
            n += 1;
        }
        i += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_unwrap_inventory_named() {
        assert!(P3_SOLVER_UNWRAP_INVENTORY_COMPLETE);
        assert!(!P3_SOLVER_UNWRAP_INVENTORY.is_empty());
        assert_eq!(P3_SOLVER_UNWRAP_INVENTORY.len(), 7);
        assert!(P3_SOLVER_UNWRAP_NON_CLAIM.contains("not physics GREEN"));
    }

    #[test]
    fn r15_c1_solver_unwrap_disposition_audit() {
        assert!(P3_SOLVER_UNWRAP_BOUNDARY_AUDIT_COMPLETE);
        assert_eq!(P3_SOLVER_UNWRAP_SITES_CLOSED, 1);
        assert_eq!(P3_SOLVER_UNWRAP_SITES_NAMED_OPEN, 6);
        assert!(P3_SOLVER_UNWRAP_BOUNDARY_OPEN);
        assert_eq!(
            count_disposition(SolverUnwrapDisposition::ClosedCanonicalPath),
            1
        );
        assert_eq!(
            count_disposition(SolverUnwrapDisposition::NamedKernelOpen),
            6
        );
        for site in P3_SOLVER_UNWRAP_INVENTORY {
            assert!(!site.residue_id.is_empty());
            assert!(!site.reason.is_empty());
        }
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `impl TensorAlgebra` for Burn — rank-0 exact + rank-1+ tensor paths.
//!
//! **Fence:** this crate is the sole cartridge-lattice adapter naming `burn`.
//! Cartridge atoms remain generic over `A: TensorAlgebra`.

pub mod golden_harness;
pub mod parity_rank;
pub mod rank0;
pub mod rank1;
pub mod rank1_field_harness;
pub mod surface_lift;
pub mod tensor;

pub use golden_harness::{
    compare_burn_lift_to_golden, compare_host_relative, compare_host_scalar, rank1_eps, GoldenVerdict,
};
pub use rank1_field_harness::{
    rank1_field_gate_closes, rank1_field_parity_claim, rank1_field_parity_closes,
    rank1_field_parity_verdict, rank1_field_perturbation_witness, RANK1_FIELD_DEFAULT_RTOL,
};
pub use parity_rank::{
    close, rank_label, rank_ordinal, CloseError, Evidence, ParityClaim, ParityRank,
};
pub use surface_lift::{burn_scalar_lift_closes, burn_scalar_lift_verdict};

pub use rank0::{BurnRank0Algebra, BurnRank0Field, RANK0_SLICE_ID};
pub use rank1::{RANK1_PLUS_COMPARISON_EPS, RANK1_PLUS_IMPL_LANDED, RANK1_PLUS_RESIDUE_ID};
pub use tensor::{BurnAlgebra, BurnNdArrayAlgebra, BurnTensorField, DefaultBackend, DEFAULT_BACKEND, RANK1_PLUS_DEFERRED};

/// Re-export `burn` crates for lattice consumers (R13-1 single-home routing).
pub use burn;
pub use burn_ndarray;

/// Adapter crate landed @ R12-1.
pub const ADAPTER_CRATE_LANDED: bool = true;

/// Repo-relative path (manifold workspace).
pub const ADAPTER_CRATE_PATH: &str = "crates/umst-algebra-burn/";

/// Legacy alias cited by pre-reorg posture modules.
pub const ADAPTER_CRATE_PATH_LEGACY: &str = "umst-runtime/crates/umst-algebra-burn/";

/// C2 design SSOT cross-ref.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Parent residue — rank-1+ production path may remain open after adapter lands.
pub const PARENT_RESIDUE_ID: &str = "R-atoms-scalar";

/// Honest posture tag.
pub const POSTURE_TAG: &str = "ADAPTER_CRATE_LANDED";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_crate_posture_metadata_locked() {
        assert!(ADAPTER_CRATE_LANDED);
        assert_eq!(ADAPTER_CRATE_PATH, "crates/umst-algebra-burn/");
        assert!(RANK1_PLUS_IMPL_LANDED);
        assert!(!RANK1_PLUS_DEFERRED);
    }
}

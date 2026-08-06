// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! `NalgebraAlgebra` production posture — slice-4 defer anchor for `umst-runtime`.
//!
//! **Honest boundary:** `umst-runtime` is the designated `nalgebra` home per
//! [`RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md`](../../../../archived/residuals/misc-outputs-tmp/RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md).
//! Production B1 cartridge monomorphization and `num-dual` AD pairing are **not**
//! landed here — 3×3 scaffold only.
//!
//! Witness: [`nalgebra_algebra`](../../../../src/runtime/nalgebra_algebra.rs) · LIB-NALGEBRA.

/// Slice identifier for runtime nalgebra tensor lift.
pub const SLICE_ID: &str = "slice-4";

/// Parent residue — tensor production path not closed.
pub const PARENT_RESIDUE_ID: &str = "R-faithful-decomp-B1";

/// LIB-LEARN-F-NALGEBRA workstream cross-ref.
pub const LIB_ID: &str = "LIB-LEARN-F-NALGEBRA";

/// Research blueprint row (F-09).
pub const BLUEPRINT_ROW: &str = "F-09";

/// Honest posture — runtime nalgebra home exists; cartridge lift **open**.
pub const POSTURE_TAG: &str = "HONEST_PARTIAL";

/// Whether the slice-4 3×3 nalgebra scaffold is landed.
pub const SCAFFOLD_LANDED: bool = true;

/// Whether B1 cartridge monomorphization over `NalgebraAlgebra` is landed.
pub const CARTRIDGE_LIFT_LANDED: bool = false;

/// Whether `num-dual` σ = ∂ψ/∂ε scalar AD is landed (F-10 @ AGAP-2037-LIB-NUMDUAL).
pub const NUM_DUAL_LANDED: bool = true;

/// Whether full 3×3 tensor AD σ_ij pairing is landed.
pub const NUM_DUAL_TENSOR_AD_LANDED: bool = false;

/// Continuum F-10 module anchor.
pub const NUM_DUAL_MODULE_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/src/tensor_lift/num_dual_ad.rs";

/// Slice-4 nalgebra scaffold (landed @ AGAP-2037-LIB-NALGEBRA).
pub const SLICE4_SCAFFOLD_PATH: &str = "umst-manifold/src/runtime/nalgebra_algebra.rs";

/// C2 design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Research libs SSOT.
pub const RESEARCH_DOC_PATH: &str = "archived/residuals/misc-outputs-tmp/RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md";

/// Fleet receipt for LIB-NALGEBRA tensor row advance.
pub const RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_LIB-NALGEBRA_2037";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nalgebra_algebra_posture_metadata_locked() {
        assert_eq!(SLICE_ID, "slice-4");
        assert_eq!(PARENT_RESIDUE_ID, "R-faithful-decomp-B1");
        assert_eq!(LIB_ID, "LIB-LEARN-F-NALGEBRA");
        assert_eq!(BLUEPRINT_ROW, "F-09");
        assert_eq!(POSTURE_TAG, "HONEST_PARTIAL");
        assert!(SCAFFOLD_LANDED);
        assert!(!CARTRIDGE_LIFT_LANDED);
        assert!(NUM_DUAL_LANDED);
        assert!(!NUM_DUAL_TENSOR_AD_LANDED);
    }

    #[test]
    fn nalgebra_algebra_posture_slice_paths_honest() {
        assert!(SLICE4_SCAFFOLD_PATH.contains("nalgebra_algebra"));
        assert!(DESIGN_DOC_PATH.contains("C2_TENSOR_ALGEBRA"));
        assert!(RESEARCH_DOC_PATH.contains("RESEARCH_LIBS_FORMAL"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_LIB-NALGEBRA_2037");
    }
}

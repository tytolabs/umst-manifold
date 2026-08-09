// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

//! F-10 `num-dual` AD σ = ∂ψ/∂ε runtime posture anchor.
//!
//! **Honest boundary:** scalar B1 σ landed in `umst-cartridge-continuum` behind feature `num-dual`;
//! full 3×3 Cauchy tensor awaits F-09 `nalgebra` lift in `umst-runtime`.
//!
//! Witness: PBM-009 · [`RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md`](../../../../old/residuals/residuals/misc-outputs-tmp/RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026.md) §3.3.

/// Slice identifier for F-10 num-dual AD.
pub const SLICE_ID: &str = "F-10-num-dual";

/// Parent residue — rank-1+ tensor path open.
pub const PARENT_RESIDUE_ID: &str = "R-faithful-decomp-B1";

/// PBM-009 workstream cross-ref (faithful ψ row).
pub const PBM_ID: &str = "PBM-009";

/// F-09 nalgebra companion (not landed).
pub const F09_NALGEBRA_ID: &str = "F-09-nalgebra";

/// Honest posture — scalar AD landed; tensor lift deferred.
pub const POSTURE_TAG: &str = "HONEST_PARTIAL";

/// Whether scalar AD σ = ∂ψ/∂ε is on disk in continuum fence.
pub const SCALAR_AD_LANDED: bool = true;

/// Whether 3×3 `nalgebra` tensor lift is landed.
pub const TENSOR_LIFT_LANDED: bool = false;

/// Continuum fence module path.
pub const CONTINUUM_MODULE_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/src/tensor_lift/num_dual_ad.rs";

/// Integration witness path.
pub const INTEGRATION_WITNESS_PATH: &str =
    "umst-cartridges/crates/umst-cartridge-continuum/tests/num_dual_ad_spike.rs";

/// Feature gate name on continuum crate.
pub const FEATURE_GATE: &str = "num-dual";

/// `num-dual` crate pin (rustc 1.88 compatible).
pub const NUM_DUAL_CRATE_PIN: &str = "=0.13.7";

/// Research authority slug.
pub const RESEARCH_RECEIPT: &str = "RESEARCH_LIBS_FORMAL_FIELD_CHEM_JUL2026";

/// Fleet receipt for this wave.
pub const RECEIPT_SLUG: &str = "COMPLETION_AGAP_AGENT_LIB-NUMDUAL_2037";

/// F-09 blueprint row — tensor lift companion (not closed).
pub const F09_BLUEPRINT_ROW: &str = "F-09";

/// Scalar AD carrier rank — B1 σ = ∂ψ/∂ε on continuum fence only.
pub const SCALAR_AD_RANK: &str = "rank-0";

/// Tensor lift carrier rank — 3×3 Cauchy σ_ij awaits F-09 `nalgebra`.
pub const TENSOR_LIFT_RANK: &str = "rank-2-3x3";

/// Whether full 3×3 tensor AD σ_ij = ∂ψ/∂ε_ij is landed.
pub const TENSOR_AD_LANDED: bool = false;

/// Runtime F-09 nalgebra scaffold (slice-4; tensor home, AD pairing open).
pub const NALGEBRA_SCAFFOLD_PATH: &str = "umst-manifold/src/runtime/nalgebra_algebra.rs";

/// C2 tensor algebra design SSOT.
pub const DESIGN_DOC_PATH: &str = "docs/C2_TENSOR_ALGEBRA_DESIGN.md";

/// Honest blocker — tensor AD requires F-09 lift before runtime closure.
pub const TENSOR_LIFT_BLOCKER: &str = "F-09-nalgebra-slice-4-open";

/// Fleet census row for F-10 scalar AD vs F-09 tensor lift honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumDualAdPostureDepthSummary {
    pub slice_id: &'static str,
    pub parent_residue_id: &'static str,
    pub posture_tag: &'static str,
    pub scalar_ad_landed: bool,
    pub tensor_lift_landed: bool,
    pub tensor_ad_landed: bool,
    pub f09_blueprint_row: &'static str,
    pub tensor_lift_blocker: &'static str,
}

/// Frozen depth summary — scalar AD landed; tensor lift deferred.
#[must_use]
pub const fn num_dual_ad_posture_depth_summary() -> NumDualAdPostureDepthSummary {
    NumDualAdPostureDepthSummary {
        slice_id: SLICE_ID,
        parent_residue_id: PARENT_RESIDUE_ID,
        posture_tag: POSTURE_TAG,
        scalar_ad_landed: SCALAR_AD_LANDED,
        tensor_lift_landed: TENSOR_LIFT_LANDED,
        tensor_ad_landed: TENSOR_AD_LANDED,
        f09_blueprint_row: F09_BLUEPRINT_ROW,
        tensor_lift_blocker: TENSOR_LIFT_BLOCKER,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_dual_ad_posture_metadata_locked() {
        assert_eq!(SLICE_ID, "F-10-num-dual");
        assert_eq!(PARENT_RESIDUE_ID, "R-faithful-decomp-B1");
        assert_eq!(PBM_ID, "PBM-009");
        assert_eq!(F09_NALGEBRA_ID, "F-09-nalgebra");
        assert_eq!(POSTURE_TAG, "HONEST_PARTIAL");
        assert!(SCALAR_AD_LANDED);
        assert!(!TENSOR_LIFT_LANDED);
        assert!(!TENSOR_AD_LANDED);
        assert_eq!(FEATURE_GATE, "num-dual");
        assert_eq!(NUM_DUAL_CRATE_PIN, "=0.13.7");
        assert_eq!(SCALAR_AD_RANK, "rank-0");
        assert_eq!(TENSOR_LIFT_RANK, "rank-2-3x3");
        assert_eq!(F09_BLUEPRINT_ROW, "F-09");
        assert_eq!(TENSOR_LIFT_BLOCKER, "F-09-nalgebra-slice-4-open");
    }

    #[test]
    fn num_dual_ad_posture_paths_honest() {
        assert!(CONTINUUM_MODULE_PATH.contains("num_dual_ad"));
        assert!(INTEGRATION_WITNESS_PATH.contains("num_dual_ad_spike"));
        assert!(NALGEBRA_SCAFFOLD_PATH.contains("nalgebra_algebra"));
        assert!(DESIGN_DOC_PATH.contains("C2_TENSOR_ALGEBRA"));
        assert_eq!(RECEIPT_SLUG, "COMPLETION_AGAP_AGENT_LIB-NUMDUAL_2037");
    }

    #[test]
    fn num_dual_ad_posture_scalar_ad_vs_tensor_lift_fence() {
        assert!(SCALAR_AD_LANDED, "B1 scalar σ = ∂ψ/∂ε landed in continuum");
        assert!(!TENSOR_LIFT_LANDED, "3×3 nalgebra lift not closed");
        assert!(!TENSOR_AD_LANDED, "tensor AD σ_ij not landed");
        assert_ne!(SCALAR_AD_RANK, TENSOR_LIFT_RANK);
        assert!(CONTINUUM_MODULE_PATH.contains("continuum"));
        assert!(NALGEBRA_SCAFFOLD_PATH.contains("umst-manifold"));
    }

    #[test]
    fn num_dual_ad_posture_depth_summary_locked() {
        let summary = num_dual_ad_posture_depth_summary();
        assert_eq!(summary.slice_id, "F-10-num-dual");
        assert_eq!(summary.parent_residue_id, "R-faithful-decomp-B1");
        assert_eq!(summary.posture_tag, "HONEST_PARTIAL");
        assert!(summary.scalar_ad_landed);
        assert!(!summary.tensor_lift_landed);
        assert!(!summary.tensor_ad_landed);
        assert_eq!(summary.f09_blueprint_row, F09_BLUEPRINT_ROW);
        assert!(summary.tensor_lift_blocker.contains("F-09"));
    }

    #[test]
    fn num_dual_ad_posture_f09_companion_cross_ref() {
        assert_eq!(F09_NALGEBRA_ID, "F-09-nalgebra");
        assert_eq!(F09_BLUEPRINT_ROW, "F-09");
        assert!(NALGEBRA_SCAFFOLD_PATH.ends_with("nalgebra_algebra.rs"));
        assert!(!TENSOR_LIFT_LANDED);
        assert_eq!(TENSOR_LIFT_BLOCKER, "F-09-nalgebra-slice-4-open");
    }

    #[test]
    fn num_dual_ad_posture_feature_gate_continuum_only() {
        assert_eq!(FEATURE_GATE, "num-dual");
        assert!(CONTINUUM_MODULE_PATH.contains("umst-cartridge-continuum"));
        assert!(!NALGEBRA_SCAFFOLD_PATH.contains("continuum"));
    }
}

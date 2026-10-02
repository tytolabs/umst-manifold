// SPDX-FileCopyrightText: 2026 Santosh Prabhu Shenbagamoorthy and Santhosh Shyamsundar
// SPDX-License-Identifier: MIT
//! Compile-time registry of (theorem hint, DOI) pairs for cross-checks and telemetry.
//!
//! Canonical Zenodo family for quantum bridge rows: **10_5281/zenodo.19159660** (`umst-formal-double-slit`).
//!
//! **N3-FPD-c:** every `module::theorem` hint uses **double-colon** (`::`) — no slash (`/`) form.

/// `(module path hint, Zenodo DOI)` — human-facing; Lean names are canonical in source docs.
/// THEOREM-BOUND: `UMST.FormalDoubleSlit.QuantumClassicalBridge::complementarity_fringe_path` (§14bis.l W-3 G8)
pub const THEOREM_REGISTRY: &[(&str, &str)] = &[
    ("UMST.FormalDoubleSlit.QuantumClassicalBridge::complementarity_fringe_path", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.LandauerBound::principle_of_maximal_information_collapse", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.KleinInequality::spectralRelativeEntropynonneg", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.QuantumMutualInfo::I(A:B)_formula", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.TensorPartialTrace::tensorDensity", "10_5281/zenodo.19159660"),
    // Parity extension (Appendix J.5a) — mirrors `/// Proof:` citations across `umst-math` modules.
    ("UMST.FormalDoubleSlit.VonNeumannEntropy::vN_entropy_nonneg", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.DataProcessingInequality::vonNeumannEntropy_nondecreasing_unital_CPTP_n", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.ErasureChannel::idealResetErasure_saturates", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicSensing::QuantumProbe", "10_5281/zenodo.19159660"),
    ("UMST.Formal.LandauerLaw::landauerBound", "10_5281/zenodo.19159660"),
    ("UMST.Formal.InfoTheory::product_joint_mass", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.LindbladDynamics::dephasingSolution_tendsto_diagonal", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.WhichPathMeasurementUpdate::measurementUpdateWhichPath", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.SchrodingerDynamics::unitary_channel", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.PMICVisibility::path_entropy_visibility", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.ExamplesQubit::qubit_zero", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_nonneg", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.GeneralResidualCoherence::residual_coherence_capacity", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.InformationCostIdentity::residualCoherence_eq_one_minus_epistemic_bits", "10_5281/zenodo.19159660"),
    // Phase K1 — epistemic proxy selector (bind-only)
    ("UMST.Formal.Gate::gate_check", "10_5281/zenodo.19159660"),
    ("UMST.Formal.Gate::transitionTolerance", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicMI::epistemicMIBits_le_one", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicProxySelector::MI_weighted_rank", "10_5281/zenodo.19159660"),
    ("UMST.FormalDoubleSlit.EpistemicPolicy::runtime_specialisation_hook", "10_5281/zenodo.19159660"),
    // Phase K2 — hypergraph + monoidal functor (bind-only)
    ("UMST.Formal.GraphProperties::hypergraph_incidence", "10_5281/zenodo.19159660"),
    ("UMST.Formal.GraphProperties::finite_edge_union", "10_5281/zenodo.19159660"),
    ("UMST.Formal.Naturality::functor_compose_vertex", "10_5281/zenodo.19159660"),
    ("UMST.Formal.Naturality::relabeling_coherent", "10_5281/zenodo.19159660"),
    ("UMST.Formal.MonoidalState::tensor_product_monoid", "10_5281/zenodo.19159660"),
    // Phase M4 — cockpit consumer: `cockpit credit module` (`record_contribution` / influence mass)
    ("UMST.Formal.CreditGreedy::credit_greedy_optimal", "10_5281/zenodo.19159660"),
    ("UMST.Formal.Dignity::dignity_monotone_under_mi_gain", "10_5281/zenodo.19159660"),
    ("UMST.Formal.EtaCog::eta_cog_nonneg", "10_5281/zenodo.19159660"),
    ("UMST.Formal.RhoEstimator::rho_based_mi_formula", "10_5281/zenodo.19159660"),
    ("UMST.Formal.MedianConvergence::median_convergence_sample_size", "10_5281/zenodo.19159660"),
    ("UMST.Formal.MedianConvergence::sqrt_window_warmup_is_admissible", "10_5281/zenodo.19159660"),
    ("UMST.Formal.OrderStatisticsBand::order_statistic_concentration", "10_5281/zenodo.19159660"),
    ("UMST.Formal.OrderStatisticsBand::p25_p75_admissibility", "10_5281/zenodo.19159660"),
];

/// The registry row a Lean declaration fixes: a function of `constants::registry::REGISTRY`, whose
/// `Derivation::Theorem` rows name their declarations, so no second table of theorem–constant pairs exists.
#[must_use]
pub fn constant_for_theorem(decl: crate::constants::derivation::LeanDecl) -> Option<&'static str> {
    use crate::constants::derivation::Derivation;
    crate::constants::registry::REGISTRY
        .iter()
        .find(|e| matches!(e.derivation, Derivation::Theorem { decl: d, .. } if d == decl))
        .map(|e| e.name)
}

/// The Lean declaration that fixes registry row `constant_name`, when a theorem fixes it.
#[must_use]
pub fn theorem_for_constant(constant_name: &str) -> Option<crate::constants::derivation::LeanDecl> {
    use crate::constants::derivation::Derivation;
    crate::constants::registry::REGISTRY
        .iter()
        .find(|e| e.name == constant_name)
        .and_then(|e| match e.derivation {
            Derivation::Theorem { decl, .. } => Some(decl),
            _ => None,
        })
}

/// Theorem–constant crosswalk coverage, computed from REGISTRY: every row a theorem fixes names its declaration,
/// so the crosswalk covers every derived row by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrosswalkStats {
    /// Theorem–constant pairs (one per theorem row).
    pub map_rows: usize,
    /// Distinct constants a theorem fixes.
    pub mapped_constants: usize,
    /// Rows whose derivation is a theorem.
    pub derived_constant_rows: usize,
    /// Derived rows the crosswalk reaches.
    pub covered_derived_rows: usize,
}

/// Coverage of the crosswalk over the registry's theorem rows.
#[must_use]
pub fn crosswalk_stats() -> CrosswalkStats {
    use crate::constants::derivation::Derivation;
    let derived = crate::constants::registry::REGISTRY
        .iter()
        .filter(|e| matches!(e.derivation, Derivation::Theorem { .. }))
        .count();
    let covered = crate::constants::registry::REGISTRY
        .iter()
        .filter(|e| theorem_for_constant(e.name).is_some_and(|d| constant_for_theorem(d) == Some(e.name)))
        .count();
    CrosswalkStats { map_rows: derived, mapped_constants: derived, derived_constant_rows: derived, covered_derived_rows: covered }
}

/// The Lean declarations that fix registry row `constant_name` (empty when no theorem fixes it).
#[must_use]
pub fn theorems_for_constant(constant_name: &str) -> Vec<crate::constants::derivation::LeanDecl> {
    theorem_for_constant(constant_name).into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{constant_for_theorem, crosswalk_stats, theorem_for_constant, THEOREM_REGISTRY};

    #[test]
    fn theorem_and_constant_maps_are_inverse() {
        let decl = theorem_for_constant("gate_mass_tolerance_kg_m3").expect("a theorem fixes the gate tolerance");
        assert_eq!(constant_for_theorem(decl), Some("gate_mass_tolerance_kg_m3"));
        assert_eq!(theorem_for_constant("transition_tolerance"), None);
        let s = crosswalk_stats();
        assert!(s.derived_constant_rows > 0);
        assert_eq!(s.covered_derived_rows, s.derived_constant_rows);
    }

    #[test]
    fn registry_parity_minimum_rows() {
        assert!(
            THEOREM_REGISTRY.len() >= 36,
            "expected ≥36 registry rows (Phase FPD-OrderStatisticsBand+), got {}",
            THEOREM_REGISTRY.len()
        );
    }

    #[test]
    fn registry_hints_use_double_colon_form() {
        for (hint, _) in THEOREM_REGISTRY {
            assert!(
                hint.contains("::"),
                "registry hint must use `::` form, got {hint}"
            );
            assert!(
                !hint.contains('/'),
                "registry hint must not contain `/` path separator, got {hint}"
            );
        }
    }
}
